use crate::commands::cmd_append_layout;
use crate::commands::cmd_bar_hidden_state;
use crate::commands::cmd_bar_mode;
use crate::commands::cmd_border;
use crate::commands::cmd_criteria_add;
use crate::commands::cmd_criteria_init;
use crate::commands::cmd_criteria_match_windows;
use crate::commands::cmd_debuglog;
use crate::commands::cmd_exec;
use crate::commands::cmd_exit;
use crate::commands::cmd_floating;
use crate::commands::cmd_focus;
use crate::commands::cmd_focus_direction;
use crate::commands::cmd_focus_level;
use crate::commands::cmd_focus_output;
use crate::commands::cmd_focus_sibling;
use crate::commands::cmd_focus_window_mode;
use crate::commands::cmd_fullscreen;
use crate::commands::cmd_gaps;
use crate::commands::cmd_kill;
use crate::commands::cmd_layout;
use crate::commands::cmd_layout_toggle;
use crate::commands::cmd_mark;
use crate::commands::cmd_mode;
use crate::commands::cmd_move_con_to_mark;
use crate::commands::cmd_move_con_to_output;
use crate::commands::cmd_move_con_to_workspace;
use crate::commands::cmd_move_con_to_workspace_back_and_forth;
use crate::commands::cmd_move_con_to_workspace_name;
use crate::commands::cmd_move_con_to_workspace_number;
use crate::commands::cmd_move_direction;
use crate::commands::cmd_move_scratchpad;
use crate::commands::cmd_move_window_to_center;
use crate::commands::cmd_move_window_to_mouse;
use crate::commands::cmd_move_window_to_position;
use crate::commands::cmd_nop;
use crate::commands::cmd_open;
use crate::commands::cmd_reload;
use crate::commands::cmd_rename_workspace;
use crate::commands::cmd_resize;
use crate::commands::cmd_resize_set;
use crate::commands::cmd_restart;
use crate::commands::cmd_scratchpad_show;
use crate::commands::cmd_shmlog;
use crate::commands::cmd_split;
use crate::commands::cmd_sticky;
use crate::commands::cmd_swap;
use crate::commands::cmd_title_format;
use crate::commands::cmd_title_window_icon;
use crate::commands::cmd_unmark;
use crate::commands::cmd_workspace;
use crate::commands::cmd_workspace_back_and_forth;
use crate::commands::cmd_workspace_name;
use crate::commands::cmd_workspace_number;
use crate::domains::{DockMatch, MatchInsertWhere};
use crate::types::UrgencyMatch;
use crate::types::WindowMode;
use std::os::unix::ffi::OsStrExt;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * commands_parser.c: hand-written parser to parse commands (commands are what
 * you bind on keys and what you can send to i3 using the IPC interface, like
 * 'move left' or 'workspace 4').
 *
 * We use a hand-written parser instead of lex/yacc because our commands are
 * easy for humans, not for computers. Thus, it’s quite hard to specify a
 * context-free grammar for the commands. A PEG grammar would be easier, but
 * there’s downsides to every PEG parser generator I have come across so far.
 *
 * This parser is basically a state machine which looks for literals or strings
 * and can push either on a stack. After identifying a literal or string, it
 * will either transition to the current state, to a different state, or call a
 * function (like cmd_move()).
 *
 * Special care has been taken that error messages are useful and the code is
 * well testable (when compiled with -DTEST_PARSER it will output to stdout
 * instead of actually calling any function).
 *
 */
// Mechanical file-by-file Rust translation of src/commands_parser.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use crate::ipc::{emit_command_reply, CommandReply};

pub use crate::application::ipc_client;

pub use crate::assignments::CommandResult;
pub use crate::commands::cmd_parser_ctx;
pub use crate::commands::owindow;
pub use crate::commands::owindows_head;
pub use crate::commands::stack;
pub use crate::commands::CommandResultIR;
pub type cmdp_token = token;
#[derive(Copy, Clone)]
pub struct token {
    pub name: &'static [u8],
    pub identifier: Option<&'static [u8]>,
    pub next_state: CommandParserState,
    pub extra: ParserTokenData,
}
#[derive(Copy, Clone)]
pub struct ParserTokenData {
    pub call_identifier: u16,
}

pub const LONG_MAX: i64 = __LONG_MAX__;
pub const LONG_MIN: i64 = -__LONG_MAX__ - 1_i64;
static tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME: [cmdp_token; 2] = [
    token {
        name: b"string",
        identifier: Some(b"new_name"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 1_i32 as u16,
        },
    },
];
static tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"output"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 2_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 3_i32 as u16,
        },
    },
];
static tokens_RENAME_WORKSPACE_TO_NEW_NAME: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"new_name"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 4_i32 as u16,
    },
}];
static tokens_RENAME_WORKSPACE_LIKELY_TO: [cmdp_token; 2] = [
    token {
        name: b"'to ",
        identifier: None,
        next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"new_name"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 5_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_ABSOLUTE_POSITION: [cmdp_token; 1] = [token {
    name: b"'position",
    identifier: None,
    next_state: CommandParserState::MoveToPosition,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_TITLE_WINDOW_ICON_PADDING: [cmdp_token; 3] = [
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 6_i32 as u16,
        },
    },
    token {
        name: b"'px",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 7_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"padding"),
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: b"'\"",
        identifier: None,
        next_state: CommandParserState::CriterionFromStr,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 8_i32 as u16,
        },
    },
    token {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 9_i32 as u16,
        },
    },
];
static tokens_MOVE_WORKSPACE_TO_OUTPUT: [cmdp_token; 1] = [token {
    name: b"'output",
    identifier: None,
    next_state: CommandParserState::MoveWorkspaceToOutputWord,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_RESIZE_HEIGHT_GET_NUMBER: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"height"),
    next_state: CommandParserState::ResizeHeight,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: b"'\"",
    identifier: None,
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 10_i32 as u16,
    },
}];
static tokens_MOVE_DIRECTION_NUMBER: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 11_i32 as u16,
        },
    },
    token {
        name: b"'ppt",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 12_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 13_i32 as u16,
        },
    },
];
static tokens_MOVE_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"number"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 14_i32 as u16,
    },
}];
static tokens_BAR_HIDDEN_STATE_ID: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"bar_id"),
        next_state: CommandParserState::BarHiddenStateId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 15_i32 as u16,
        },
    },
];
static tokens_RENAME_WORKSPACE_TO: [cmdp_token; 1] = [token {
    name: b"'to",
    identifier: None,
    next_state: CommandParserState::RenameWorkspaceToNewName,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_RESIZE_TILING_FINAL: [cmdp_token; 2] = [
    token {
        name: b"'ppt",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 16_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 17_i32 as u16,
        },
    },
];
static tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: b"'auto",
        identifier: Some(b"kind"),
        next_state: CommandParserState::CriterionFromStrEnd,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'user",
        identifier: Some(b"kind"),
        next_state: CommandParserState::CriterionFromStrEnd,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_POSITION_X: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: Some(b"mode_x"),
        next_state: CommandParserState::MoveToPositionX,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ppt",
        identifier: Some(b"mode_x"),
        next_state: CommandParserState::MoveToPositionX,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"coord_y"),
        next_state: CommandParserState::MoveToPositionY,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_POSITION_Y: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: Some(b"mode_y"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 18_i32 as u16,
        },
    },
    token {
        name: b"'ppt",
        identifier: Some(b"mode_y"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 19_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 20_i32 as u16,
        },
    },
];
static tokens_FULLSCREEN_COMPAT: [cmdp_token; 2] = [
    token {
        name: b"'global",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 21_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 22_i32 as u16,
        },
    },
];
static tokens_TITLE_WINDOW_ICON: [cmdp_token; 14] = [
    token {
        name: b"'padding",
        identifier: None,
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"enable"),
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'1",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 23_i32 as u16,
        },
    },
    token {
        name: b"'yes",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 24_i32 as u16,
        },
    },
    token {
        name: b"'true",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 25_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 26_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 27_i32 as u16,
        },
    },
    token {
        name: b"'active",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 28_i32 as u16,
        },
    },
    token {
        name: b"'0",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 29_i32 as u16,
        },
    },
    token {
        name: b"'no",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 30_i32 as u16,
        },
    },
    token {
        name: b"'false",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 31_i32 as u16,
        },
    },
    token {
        name: b"'off",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 32_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 33_i32 as u16,
        },
    },
    token {
        name: b"'inactive",
        identifier: Some(b"enable"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 34_i32 as u16,
        },
    },
];
static tokens_BAR_HIDDEN_STATE: [cmdp_token; 3] = [
    token {
        name: b"'hide",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarHiddenStateId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'show",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarHiddenStateId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarHiddenStateId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_POSITION: [cmdp_token; 5] = [
    token {
        name: b"'center",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 35_i32 as u16,
        },
    },
    token {
        name: b"'mouse",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 36_i32 as u16,
        },
    },
    token {
        name: b"'cursor",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 37_i32 as u16,
        },
    },
    token {
        name: b"'pointer",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 38_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"coord_x"),
        next_state: CommandParserState::MoveToPositionX,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_RENAME_WORKSPACE: [cmdp_token; 2] = [
    token {
        name: b"'to",
        identifier: None,
        next_state: CommandParserState::RenameWorkspaceLikelyTo,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"word",
        identifier: Some(b"old_name"),
        next_state: CommandParserState::RenameWorkspaceTo,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_RESIZE_DIRECTION: [cmdp_token; 6] = [
    token {
        name: b"'up",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'down",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'left",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'width",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'height",
        identifier: Some(b"direction"),
        next_state: CommandParserState::ResizePx,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_RESIZE_TILING_OR: [cmdp_token; 1] = [token {
    name: b"number",
    identifier: Some(b"resize_ppt"),
    next_state: CommandParserState::ResizeTilingFinal,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"workspace"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 39_i32 as u16,
    },
}];
static tokens_FULLSCREEN_MODE: [cmdp_token; 2] = [
    token {
        name: b"'global",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 40_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 41_i32 as u16,
        },
    },
];
static tokens_GAPS_WITH_SCOPE: [cmdp_token; 4] = [
    token {
        name: b"'plus",
        identifier: Some(b"mode"),
        next_state: CommandParserState::GapsWithMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'minus",
        identifier: Some(b"mode"),
        next_state: CommandParserState::GapsWithMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set",
        identifier: Some(b"mode"),
        next_state: CommandParserState::GapsWithMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"mode"),
        next_state: CommandParserState::GapsWithMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: b"'=",
    identifier: None,
    next_state: CommandParserState::CriterionFromStrStart,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_GAPS_WITH_MODE: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"value"),
    next_state: CommandParserState::GapsEnd,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_GAPS_WITH_TYPE: [cmdp_token; 2] = [
    token {
        name: b"'current",
        identifier: Some(b"scope"),
        next_state: CommandParserState::GapsWithScope,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'all",
        identifier: Some(b"scope"),
        next_state: CommandParserState::GapsWithScope,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_MOVE_DIRECTION: [cmdp_token; 2] = [
    token {
        name: b"number",
        identifier: Some(b"amount"),
        next_state: CommandParserState::MoveDirectionNumber,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 42_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_OUTPUT: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"output"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 43_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 44_i32 as u16,
        },
    },
];
static tokens_MOVE_WORKSPACE: [cmdp_token; 9] = [
    token {
        name: b"'to ",
        identifier: None,
        next_state: CommandParserState::MoveWorkspaceToOutput,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'next_on_output",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 45_i32 as u16,
        },
    },
    token {
        name: b"'prev_on_output",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 46_i32 as u16,
        },
    },
    token {
        name: b"'next",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 47_i32 as u16,
        },
    },
    token {
        name: b"'prev",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 48_i32 as u16,
        },
    },
    token {
        name: b"'current",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 49_i32 as u16,
        },
    },
    token {
        name: b"'back_and_forth",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 50_i32 as u16,
        },
    },
    token {
        name: b"'number",
        identifier: None,
        next_state: CommandParserState::MoveWorkspaceNumber,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 51_i32 as u16,
        },
    },
];
static tokens_APPEND_LAYOUT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"path"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 52_i32 as u16,
    },
}];
static tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: b"word",
    identifier: Some(b"cvalue"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 53_i32 as u16,
    },
}];
static tokens_LAYOUT_TOGGLE: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 54_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"toggle_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 55_i32 as u16,
        },
    },
];
static tokens_RESIZE_HEIGHT: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: Some(b"mode_height"),
        next_state: CommandParserState::ResizeHeight,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ppt",
        identifier: Some(b"mode_height"),
        next_state: CommandParserState::ResizeHeight,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 56_i32 as u16,
        },
    },
];
static tokens_RESIZE_TILING: [cmdp_token; 3] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: CommandParserState::ResizeTiling,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'or",
        identifier: None,
        next_state: CommandParserState::ResizeTilingOr,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 57_i32 as u16,
        },
    },
];
static tokens_SWAP_ARGUMENT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"arg"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 58_i32 as u16,
    },
}];
static tokens_BORDER_WIDTH: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 59_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"border_width"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 60_i32 as u16,
        },
    },
];
static tokens_FOCUS_OUTPUT: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"output"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 61_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 62_i32 as u16,
        },
    },
];
static tokens_MOVE_TO_MARK: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"mark"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 63_i32 as u16,
    },
}];
static tokens_RESIZE_WIDTH: [cmdp_token; 5] = [
    token {
        name: b"'px",
        identifier: Some(b"mode_width"),
        next_state: CommandParserState::ResizeWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'ppt",
        identifier: Some(b"mode_width"),
        next_state: CommandParserState::ResizeWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 64_i32 as u16,
        },
    },
    token {
        name: b"'height",
        identifier: None,
        next_state: CommandParserState::ResizeHeightGetNumber,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"height"),
        next_state: CommandParserState::ResizeHeight,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_TITLE_FORMAT: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"format"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 65_i32 as u16,
    },
}];
static tokens_BAR_MODE_ID: [cmdp_token; 2] = [
    token {
        name: b"word",
        identifier: Some(b"bar_id"),
        next_state: CommandParserState::BarModeId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 66_i32 as u16,
        },
    },
];
static tokens_FOCUS_AUTO: [cmdp_token; 2] = [
    token {
        name: b"'sibling",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 67_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 68_i32 as u16,
        },
    },
];
static tokens_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: b"'disable",
        identifier: Some(b"action"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 69_i32 as u16,
        },
    },
    token {
        name: b"'enable",
        identifier: Some(b"action"),
        next_state: CommandParserState::FullscreenMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"action"),
        next_state: CommandParserState::FullscreenMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'",
        identifier: Some(b"action"),
        next_state: CommandParserState::FullscreenCompat,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_RESIZE_SET: [cmdp_token; 3] = [
    token {
        name: b"'height",
        identifier: None,
        next_state: CommandParserState::ResizeHeightGetNumber,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'width",
        identifier: None,
        next_state: CommandParserState::ResizeSet,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"number",
        identifier: Some(b"width"),
        next_state: CommandParserState::ResizeWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_SCRATCHPAD: [cmdp_token; 1] = [token {
    name: b"'show",
    identifier: None,
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 70_i32 as u16,
    },
}];
static tokens_CRITERION: [cmdp_token; 1] = [token {
    name: b"'=",
    identifier: None,
    next_state: CommandParserState::CriterionStr,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_RESIZE_PX: [cmdp_token; 2] = [
    token {
        name: b"number",
        identifier: Some(b"resize_px"),
        next_state: CommandParserState::ResizeTiling,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 71_i32 as u16,
        },
    },
];
static tokens_WORKSPACE: [cmdp_token; 8] = [
    token {
        name: b"'--no-auto-back-and-forth",
        identifier: Some(b"no_auto_back_and_forth"),
        next_state: CommandParserState::Workspace,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'next_on_output",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 72_i32 as u16,
        },
    },
    token {
        name: b"'prev_on_output",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 73_i32 as u16,
        },
    },
    token {
        name: b"'next",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 74_i32 as u16,
        },
    },
    token {
        name: b"'prev",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 75_i32 as u16,
        },
    },
    token {
        name: b"'back_and_forth",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 76_i32 as u16,
        },
    },
    token {
        name: b"'number",
        identifier: None,
        next_state: CommandParserState::WorkspaceNumber,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 77_i32 as u16,
        },
    },
];
static tokens_BAR_MODE: [cmdp_token; 4] = [
    token {
        name: b"'dock",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarModeId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'hide",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarModeId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'invisible",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarModeId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"bar_value"),
        next_state: CommandParserState::BarModeId,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: b"'class",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'instance",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'window_role",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'con_id",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'id",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'window_type",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'con_mark",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'title",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'urgent",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'machine",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Criterion,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating_from",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::CriterionFrom,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling_from",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::CriterionFrom,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 78_i32 as u16,
        },
    },
    token {
        name: b"'floating",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 79_i32 as u16,
        },
    },
    token {
        name: b"'all",
        identifier: Some(b"ctype"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 80_i32 as u16,
        },
    },
    token {
        name: b"']",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 81_i32 as u16,
        },
    },
];
static tokens_DEBUGLOG: [cmdp_token; 3] = [
    token {
        name: b"'toggle",
        identifier: Some(b"argument"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 82_i32 as u16,
        },
    },
    token {
        name: b"'on",
        identifier: Some(b"argument"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 83_i32 as u16,
        },
    },
    token {
        name: b"'off",
        identifier: Some(b"argument"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 84_i32 as u16,
        },
    },
];
static tokens_FLOATING: [cmdp_token; 3] = [
    token {
        name: b"'enable",
        identifier: Some(b"floating"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 85_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"floating"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 86_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"floating"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 87_i32 as u16,
        },
    },
];
static tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: b"'px",
        identifier: None,
        next_state: CommandParserState::GapsEnd,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 88_i32 as u16,
        },
    },
];
static tokens_INITIAL: [cmdp_token; 32] = [
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Initial,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'[",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 89_i32 as u16,
        },
    },
    token {
        name: b"'move",
        identifier: None,
        next_state: CommandParserState::Move,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'exec",
        identifier: None,
        next_state: CommandParserState::Exec,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'exit",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 90_i32 as u16,
        },
    },
    token {
        name: b"'restart",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 91_i32 as u16,
        },
    },
    token {
        name: b"'reload",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 92_i32 as u16,
        },
    },
    token {
        name: b"'shmlog",
        identifier: None,
        next_state: CommandParserState::Shmlog,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'debuglog",
        identifier: None,
        next_state: CommandParserState::Debuglog,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'border",
        identifier: None,
        next_state: CommandParserState::Border,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'layout",
        identifier: None,
        next_state: CommandParserState::Layout,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'append_layout",
        identifier: None,
        next_state: CommandParserState::AppendLayout,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: None,
        next_state: CommandParserState::Workspace,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'focus",
        identifier: None,
        next_state: CommandParserState::Focus,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'kill",
        identifier: None,
        next_state: CommandParserState::Kill,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'open",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 93_i32 as u16,
        },
    },
    token {
        name: b"'fullscreen",
        identifier: None,
        next_state: CommandParserState::Fullscreen,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'sticky",
        identifier: None,
        next_state: CommandParserState::Sticky,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'split",
        identifier: None,
        next_state: CommandParserState::Split,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'floating",
        identifier: None,
        next_state: CommandParserState::Floating,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mark",
        identifier: None,
        next_state: CommandParserState::Mark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'unmark",
        identifier: None,
        next_state: CommandParserState::Unmark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'resize",
        identifier: None,
        next_state: CommandParserState::Resize,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'rename",
        identifier: None,
        next_state: CommandParserState::Rename,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'nop",
        identifier: None,
        next_state: CommandParserState::Nop,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'scratchpad",
        identifier: None,
        next_state: CommandParserState::Scratchpad,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'swap",
        identifier: None,
        next_state: CommandParserState::Swap,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'title_format",
        identifier: None,
        next_state: CommandParserState::TitleFormat,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'title_window_icon",
        identifier: None,
        next_state: CommandParserState::TitleWindowIcon,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mode",
        identifier: None,
        next_state: CommandParserState::Mode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bar",
        identifier: None,
        next_state: CommandParserState::Bar,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'gaps",
        identifier: None,
        next_state: CommandParserState::Gaps,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BORDER: [cmdp_token; 5] = [
    token {
        name: b"'normal",
        identifier: Some(b"border_style"),
        next_state: CommandParserState::BorderWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'pixel",
        identifier: Some(b"border_style"),
        next_state: CommandParserState::BorderWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"border_style"),
        next_state: CommandParserState::BorderWidth,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'none",
        identifier: Some(b"border_style"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 94_i32 as u16,
        },
    },
    token {
        name: b"'1pixel",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 95_i32 as u16,
        },
    },
];
static tokens_LAYOUT: [cmdp_token; 7] = [
    token {
        name: b"'default",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 96_i32 as u16,
        },
    },
    token {
        name: b"'stacked",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 97_i32 as u16,
        },
    },
    token {
        name: b"'stacking",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 98_i32 as u16,
        },
    },
    token {
        name: b"'tabbed",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 99_i32 as u16,
        },
    },
    token {
        name: b"'splitv",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 100_i32 as u16,
        },
    },
    token {
        name: b"'splith",
        identifier: Some(b"layout_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 101_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: None,
        next_state: CommandParserState::LayoutToggle,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_RENAME: [cmdp_token; 1] = [token {
    name: b"'workspace",
    identifier: None,
    next_state: CommandParserState::RenameWorkspace,
    extra: ParserTokenData {
        call_identifier: 0_i32 as u16,
    },
}];
static tokens_RESIZE: [cmdp_token; 3] = [
    token {
        name: b"'grow",
        identifier: Some(b"way"),
        next_state: CommandParserState::ResizeDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'shrink",
        identifier: Some(b"way"),
        next_state: CommandParserState::ResizeDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'set",
        identifier: Some(b"set"),
        next_state: CommandParserState::ResizeSet,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_SHMLOG: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"argument"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 102_i32 as u16,
    },
}];
static tokens_STICKY: [cmdp_token; 3] = [
    token {
        name: b"'enable",
        identifier: Some(b"action"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 103_i32 as u16,
        },
    },
    token {
        name: b"'disable",
        identifier: Some(b"action"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 104_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"action"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 105_i32 as u16,
        },
    },
];
static tokens_UNMARK: [cmdp_token; 2] = [
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 106_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"mark"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 107_i32 as u16,
        },
    },
];
static tokens_FOCUS: [cmdp_token; 14] = [
    token {
        name: b"'left",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 108_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 109_i32 as u16,
        },
    },
    token {
        name: b"'up",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 110_i32 as u16,
        },
    },
    token {
        name: b"'down",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 111_i32 as u16,
        },
    },
    token {
        name: b"'prev",
        identifier: Some(b"direction"),
        next_state: CommandParserState::FocusAuto,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'next",
        identifier: Some(b"direction"),
        next_state: CommandParserState::FocusAuto,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'output",
        identifier: None,
        next_state: CommandParserState::FocusOutput,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'tiling",
        identifier: Some(b"window_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 112_i32 as u16,
        },
    },
    token {
        name: b"'floating",
        identifier: Some(b"window_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 113_i32 as u16,
        },
    },
    token {
        name: b"'mode_toggle",
        identifier: Some(b"window_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 114_i32 as u16,
        },
    },
    token {
        name: b"'parent",
        identifier: Some(b"level"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 115_i32 as u16,
        },
    },
    token {
        name: b"'child",
        identifier: Some(b"level"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 116_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: Some(b"workspace"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 117_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 118_i32 as u16,
        },
    },
];
static tokens_SPLIT: [cmdp_token; 6] = [
    token {
        name: b"'horizontal",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 119_i32 as u16,
        },
    },
    token {
        name: b"'vertical",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 120_i32 as u16,
        },
    },
    token {
        name: b"'toggle",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 121_i32 as u16,
        },
    },
    token {
        name: b"'v",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 122_i32 as u16,
        },
    },
    token {
        name: b"'h",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 123_i32 as u16,
        },
    },
    token {
        name: b"'t",
        identifier: Some(b"direction"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 124_i32 as u16,
        },
    },
];
static tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: b"'--no-startup-id",
        identifier: Some(b"nosn"),
        next_state: CommandParserState::Exec,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"command"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 125_i32 as u16,
        },
    },
];
static tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: b"'inner",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'outer",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'horizontal",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'vertical",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'top",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'bottom",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'left",
        identifier: Some(b"type"),
        next_state: CommandParserState::GapsWithType,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_KILL: [cmdp_token; 3] = [
    token {
        name: b"'window",
        identifier: Some(b"kill_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 126_i32 as u16,
        },
    },
    token {
        name: b"'client",
        identifier: Some(b"kill_mode"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 127_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 128_i32 as u16,
        },
    },
];
static tokens_MARK: [cmdp_token; 4] = [
    token {
        name: b"'--add",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Mark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--replace",
        identifier: Some(b"mode"),
        next_state: CommandParserState::Mark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--toggle",
        identifier: Some(b"toggle"),
        next_state: CommandParserState::Mark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"string",
        identifier: Some(b"mark"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 129_i32 as u16,
        },
    },
];
static tokens_MODE: [cmdp_token; 1] = [token {
    name: b"string",
    identifier: Some(b"mode"),
    next_state: CommandParserState::Call,
    extra: ParserTokenData {
        call_identifier: 130_i32 as u16,
    },
}];
static tokens_MOVE: [cmdp_token; 14] = [
    token {
        name: b"'window",
        identifier: None,
        next_state: CommandParserState::Move,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'container",
        identifier: None,
        next_state: CommandParserState::Move,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'to",
        identifier: None,
        next_state: CommandParserState::Move,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'--no-auto-back-and-forth",
        identifier: Some(b"no_auto_back_and_forth"),
        next_state: CommandParserState::Move,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'workspace",
        identifier: None,
        next_state: CommandParserState::MoveWorkspace,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'output",
        identifier: None,
        next_state: CommandParserState::MoveToOutput,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mark",
        identifier: None,
        next_state: CommandParserState::MoveToMark,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'scratchpad",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 131_i32 as u16,
        },
    },
    token {
        name: b"'left",
        identifier: Some(b"direction"),
        next_state: CommandParserState::MoveDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'right",
        identifier: Some(b"direction"),
        next_state: CommandParserState::MoveDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'up",
        identifier: Some(b"direction"),
        next_state: CommandParserState::MoveDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'down",
        identifier: Some(b"direction"),
        next_state: CommandParserState::MoveDirection,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'position",
        identifier: Some(b"method"),
        next_state: CommandParserState::MoveToPosition,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'absolute",
        identifier: Some(b"method"),
        next_state: CommandParserState::MoveToAbsolutePosition,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_SWAP: [cmdp_token; 5] = [
    token {
        name: b"'container",
        identifier: None,
        next_state: CommandParserState::Swap,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'with",
        identifier: None,
        next_state: CommandParserState::Swap,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'id",
        identifier: Some(b"mode"),
        next_state: CommandParserState::SwapArgument,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'con_id",
        identifier: Some(b"mode"),
        next_state: CommandParserState::SwapArgument,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mark",
        identifier: Some(b"mode"),
        next_state: CommandParserState::SwapArgument,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_BAR: [cmdp_token; 2] = [
    token {
        name: b"'hidden_state",
        identifier: None,
        next_state: CommandParserState::BarHiddenState,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
    token {
        name: b"'mode",
        identifier: None,
        next_state: CommandParserState::BarMode,
        extra: ParserTokenData {
            call_identifier: 0_i32 as u16,
        },
    },
];
static tokens_NOP: [cmdp_token; 2] = [
    token {
        name: b"string",
        identifier: Some(b"comment"),
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 132_i32 as u16,
        },
    },
    token {
        name: b"end",
        identifier: None,
        next_state: CommandParserState::Call,
        extra: ParserTokenData {
            call_identifier: 133_i32 as u16,
        },
    },
];
static tokens: [&[cmdp_token]; 77] = [
    &tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME,
    &tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD,
    &tokens_RENAME_WORKSPACE_TO_NEW_NAME,
    &tokens_RENAME_WORKSPACE_LIKELY_TO,
    &tokens_MOVE_TO_ABSOLUTE_POSITION,
    &tokens_TITLE_WINDOW_ICON_PADDING,
    &tokens_CRITERION_FROM_STR_START,
    &tokens_MOVE_WORKSPACE_TO_OUTPUT,
    &tokens_RESIZE_HEIGHT_GET_NUMBER,
    &tokens_CRITERION_FROM_STR_END,
    &tokens_MOVE_DIRECTION_NUMBER,
    &tokens_MOVE_WORKSPACE_NUMBER,
    &tokens_BAR_HIDDEN_STATE_ID,
    &tokens_RENAME_WORKSPACE_TO,
    &tokens_RESIZE_TILING_FINAL,
    &tokens_CRITERION_FROM_STR,
    &tokens_MOVE_TO_POSITION_X,
    &tokens_MOVE_TO_POSITION_Y,
    &tokens_FULLSCREEN_COMPAT,
    &tokens_TITLE_WINDOW_ICON,
    &tokens_BAR_HIDDEN_STATE,
    &tokens_MOVE_TO_POSITION,
    &tokens_RENAME_WORKSPACE,
    &tokens_RESIZE_DIRECTION,
    &tokens_RESIZE_TILING_OR,
    &tokens_WORKSPACE_NUMBER,
    &tokens_FULLSCREEN_MODE,
    &tokens_GAPS_WITH_SCOPE,
    &tokens_CRITERION_FROM,
    &tokens_GAPS_WITH_MODE,
    &tokens_GAPS_WITH_TYPE,
    &tokens_MOVE_DIRECTION,
    &tokens_MOVE_TO_OUTPUT,
    &tokens_MOVE_WORKSPACE,
    &tokens_APPEND_LAYOUT,
    &tokens_CRITERION_STR,
    &tokens_LAYOUT_TOGGLE,
    &tokens_RESIZE_HEIGHT,
    &tokens_RESIZE_TILING,
    &tokens_SWAP_ARGUMENT,
    &tokens_BORDER_WIDTH,
    &tokens_FOCUS_OUTPUT,
    &tokens_MOVE_TO_MARK,
    &tokens_RESIZE_WIDTH,
    &tokens_TITLE_FORMAT,
    &tokens_BAR_MODE_ID,
    &tokens_FOCUS_AUTO,
    &tokens_FULLSCREEN,
    &tokens_RESIZE_SET,
    &tokens_SCRATCHPAD,
    &tokens_CRITERION,
    &tokens_RESIZE_PX,
    &tokens_WORKSPACE,
    &tokens_BAR_MODE,
    &tokens_CRITERIA,
    &tokens_DEBUGLOG,
    &tokens_FLOATING,
    &tokens_GAPS_END,
    &tokens_INITIAL,
    &tokens_BORDER,
    &tokens_LAYOUT,
    &tokens_RENAME,
    &tokens_RESIZE,
    &tokens_SHMLOG,
    &tokens_STICKY,
    &tokens_UNMARK,
    &tokens_FOCUS,
    &tokens_SPLIT,
    &tokens_EXEC,
    &tokens_GAPS,
    &tokens_KILL,
    &tokens_MARK,
    &tokens_MODE,
    &tokens_MOVE,
    &tokens_SWAP,
    &tokens_BAR,
    &tokens_NOP,
];
unsafe fn GENERATED_call(
    current_match: *mut Match,
    stack: *mut stack,
    call_identifier: i32,
    result: *mut CommandResultIR,
) {
    let captures = (&*stack).clone();
    match call_identifier {
        0 => {
            (*result).next_state = CommandParserState::Initial;
            let new_name = captures
                .get_string(b"new_name")
                .expect("rename grammar supplies new_name")
                .to_vec();
            cmd_rename_workspace(current_match, result, Some(b"to"), &new_name);
        }
        1 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_rename_workspace(current_match, result, None, b"to");
        }
        2 => {
            (*result).next_state = CommandParserState::MoveWorkspaceToOutputWord;
            cmd_move_con_to_output(current_match, result, captures.get_string(b"output"), true);
        }
        3 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_output(current_match, result, None, true);
        }
        4 => {
            (*result).next_state = CommandParserState::Initial;
            let old_name = captures.get_string(b"old_name").map(|name| name.to_vec());
            let new_name = captures
                .get_string(b"new_name")
                .expect("rename grammar supplies new_name")
                .to_vec();
            cmd_rename_workspace(current_match, result, old_name.as_deref(), &new_name);
        }
        5 => {
            (*result).next_state = CommandParserState::Initial;
            let new_name = captures
                .get_string(b"new_name")
                .expect("rename grammar supplies new_name")
                .to_vec();
            cmd_rename_workspace(current_match, result, None, &new_name);
        }
        6 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                captures.get_string(b"enable"),
                captures.get_long(b"padding") as i32,
            );
        }
        7 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                captures.get_string(b"enable"),
                captures.get_long(b"padding") as i32,
            );
        }
        8 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"kind"),
            );
        }
        9 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"kind"),
            );
        }
        10 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"kind"),
            );
        }
        11 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"amount"),
                captures.get_string(b"mode"),
            );
        }
        12 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"amount"),
                captures.get_string(b"mode"),
            );
        }
        13 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"amount"),
                Some(b"px".as_slice()),
            );
        }
        14 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace_number(
                current_match,
                result,
                captures
                    .get_string(b"number")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"no_auto_back_and_forth"),
            );
        }
        15 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_bar_hidden_state(
                current_match,
                result,
                captures
                    .get_string(b"bar_value")
                    .expect("bar grammar supplies value"),
                captures.get_string(b"bar_id"),
            );
        }
        16 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                captures
                    .get_string(b"way")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"resize_px"),
                captures.get_long(b"resize_ppt"),
            );
        }
        17 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                captures
                    .get_string(b"way")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"resize_px"),
                captures.get_long(b"resize_ppt"),
            );
        }
        18 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                captures.get_long(b"coord_x"),
                captures.get_string(b"mode_x"),
                captures.get_long(b"coord_y"),
                captures.get_string(b"mode_y"),
            );
        }
        19 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                captures.get_long(b"coord_x"),
                captures.get_string(b"mode_x"),
                captures.get_long(b"coord_y"),
                captures.get_string(b"mode_y"),
            );
        }
        20 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                captures.get_long(b"coord_x"),
                captures.get_string(b"mode_x"),
                captures.get_long(b"coord_y"),
                None,
            );
        }
        21 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                b"toggle",
                captures
                    .get_string(b"mode")
                    .expect("command grammar supplies required text"),
            );
        }
        22 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(current_match, result, b"toggle", b"output");
        }
        23 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        24 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        25 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        26 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        27 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        28 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        29 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        30 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        31 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        32 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        33 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        34 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(current_match, result, captures.get_string(b"enable"), 0_i32);
        }
        35 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_center(
                current_match,
                result,
                captures
                    .get_string(b"method")
                    .expect("command grammar supplies required text"),
            );
        }
        36 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_mouse(current_match, result);
        }
        37 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_mouse(current_match, result);
        }
        38 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_mouse(current_match, result);
        }
        39 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace_number(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"no_auto_back_and_forth"),
            );
        }
        40 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"mode")
                    .expect("command grammar supplies required text"),
            );
        }
        41 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
                b"output",
            );
        }
        42 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                10_i64,
                Some(b"px".as_slice()),
            );
        }
        43 => {
            (*result).next_state = CommandParserState::MoveToOutput;
            cmd_move_con_to_output(current_match, result, captures.get_string(b"output"), false);
        }
        44 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_output(current_match, result, None, false);
        }
        45 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
            );
        }
        46 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
            );
        }
        47 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
            );
        }
        48 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
            );
        }
        49 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
            );
        }
        50 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace_back_and_forth(current_match, result);
        }
        51 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace_name(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"no_auto_back_and_forth"),
            );
        }
        52 => {
            (*result).next_state = CommandParserState::Initial;
            let path = std::path::Path::new(std::ffi::OsStr::from_bytes(
                captures
                    .get_string(b"path")
                    .expect("append grammar supplies path"),
            ));
            cmd_append_layout(current_match, result, path);
        }
        53 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"cvalue"),
            );
        }
        54 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout_toggle(current_match, result, captures.get_string(b"toggle_mode"));
        }
        55 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout_toggle(current_match, result, captures.get_string(b"toggle_mode"));
        }
        56 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize_set(
                current_match,
                result,
                captures.get_long(b"width"),
                captures.get_string(b"mode_width"),
                captures.get_long(b"height"),
                captures.get_string(b"mode_height"),
            );
        }
        57 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                captures
                    .get_string(b"way")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"resize_px"),
                0_i64,
            );
        }
        58 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_swap(
                current_match,
                result,
                captures
                    .get_string(b"mode")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"arg")
                    .expect("command grammar supplies required text"),
            );
        }
        59 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                captures
                    .get_string(b"border_style")
                    .expect("command grammar supplies required text"),
                -1_i32 as i64,
            );
        }
        60 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                captures
                    .get_string(b"border_style")
                    .expect("command grammar supplies required text"),
                captures.get_long(b"border_width"),
            );
        }
        61 => {
            (*result).next_state = CommandParserState::FocusOutput;
            cmd_focus_output(current_match, result, captures.get_string(b"output"));
        }
        62 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_output(current_match, result, None);
        }
        63 => {
            (*result).next_state = CommandParserState::Initial;
            let mark = captures
                .get_string(b"mark")
                .expect("command grammar supplies mark")
                .to_vec();
            cmd_move_con_to_mark(current_match, result, &mark);
        }
        64 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize_set(
                current_match,
                result,
                captures.get_long(b"width"),
                captures.get_string(b"mode_width"),
                0_i64,
                None,
            );
        }
        65 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_format(
                current_match,
                result,
                captures
                    .get_string(b"format")
                    .expect("title grammar supplies format"),
            );
        }
        66 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_bar_mode(
                current_match,
                result,
                captures
                    .get_string(b"bar_value")
                    .expect("bar grammar supplies value"),
                captures.get_string(b"bar_id"),
            );
        }
        67 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_sibling(current_match, result, &command_text);
        }
        68 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_direction(current_match, result, &command_text);
        }
        69 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
                b"output",
            );
        }
        70 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_scratchpad_show(current_match, result);
        }
        71 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                captures
                    .get_string(b"way")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
                10_i64,
                10_i64,
            );
        }
        72 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
            );
        }
        73 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
            );
        }
        74 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
            );
        }
        75 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                captures
                    .get_string(b"direction")
                    .expect("command grammar supplies required text"),
            );
        }
        76 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace_back_and_forth(current_match, result);
        }
        77 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace_name(
                current_match,
                result,
                captures
                    .get_string(b"workspace")
                    .expect("command grammar supplies required text"),
                captures.get_string(b"no_auto_back_and_forth"),
            );
        }
        78 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                None,
            );
        }
        79 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                None,
            );
        }
        80 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                captures
                    .get_string(b"ctype")
                    .expect("command grammar supplies required text"),
                None,
            );
        }
        81 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_criteria_match_windows(current_match, result);
        }
        82 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_debuglog(
                current_match,
                result,
                captures
                    .get_string(b"argument")
                    .expect("command grammar supplies required text"),
            );
        }
        83 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_debuglog(
                current_match,
                result,
                captures
                    .get_string(b"argument")
                    .expect("command grammar supplies required text"),
            );
        }
        84 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_debuglog(
                current_match,
                result,
                captures
                    .get_string(b"argument")
                    .expect("command grammar supplies required text"),
            );
        }
        85 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                captures
                    .get_string(b"floating")
                    .expect("command grammar supplies required text"),
            );
        }
        86 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                captures
                    .get_string(b"floating")
                    .expect("command grammar supplies required text"),
            );
        }
        87 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                captures
                    .get_string(b"floating")
                    .expect("command grammar supplies required text"),
            );
        }
        88 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_gaps(
                current_match,
                result,
                captures
                    .get_string(b"type")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"scope")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"mode")
                    .expect("command grammar supplies required text"),
                captures
                    .get_string(b"value")
                    .expect("command grammar supplies required text"),
            );
        }
        89 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_init(current_match, result);
        }
        90 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_exit(current_match, result);
        }
        91 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_restart(current_match, result);
        }
        92 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_reload(current_match, result);
        }
        93 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_open(current_match, result);
        }
        94 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                captures
                    .get_string(b"border_style")
                    .expect("command grammar supplies required text"),
                0_i64,
            );
        }
        95 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(current_match, result, b"pixel", 1_i64);
        }
        96 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        97 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        98 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        99 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        100 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        101 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                captures
                    .get_string(b"layout_mode")
                    .expect("command grammar supplies required text"),
            );
        }
        102 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_shmlog(
                current_match,
                result,
                captures
                    .get_string(b"argument")
                    .expect("command grammar supplies required text"),
            );
        }
        103 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
            );
        }
        104 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
            );
        }
        105 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                captures
                    .get_string(b"action")
                    .expect("command grammar supplies required text"),
            );
        }
        106 => {
            (*result).next_state = CommandParserState::Initial;
            let mark = captures.get_string(b"mark").map(|value| value.to_vec());
            cmd_unmark(current_match, result, mark.as_deref());
        }
        107 => {
            (*result).next_state = CommandParserState::Initial;
            let mark = captures.get_string(b"mark").map(|value| value.to_vec());
            cmd_unmark(current_match, result, mark.as_deref());
        }
        108 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_direction(current_match, result, &command_text);
        }
        109 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_direction(current_match, result, &command_text);
        }
        110 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_direction(current_match, result, &command_text);
        }
        111 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_focus_direction(current_match, result, &command_text);
        }
        112 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"window_mode")
                .expect("command grammar supplies window_mode")
                .to_vec();
            cmd_focus_window_mode(current_match, result, &command_text);
        }
        113 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"window_mode")
                .expect("command grammar supplies window_mode")
                .to_vec();
            cmd_focus_window_mode(current_match, result, &command_text);
        }
        114 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"window_mode")
                .expect("command grammar supplies window_mode")
                .to_vec();
            cmd_focus_window_mode(current_match, result, &command_text);
        }
        115 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"level")
                .expect("command grammar supplies level")
                .to_vec();
            cmd_focus_level(current_match, result, &command_text);
        }
        116 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"level")
                .expect("command grammar supplies level")
                .to_vec();
            cmd_focus_level(current_match, result, &command_text);
        }
        117 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus(current_match, result, true);
        }
        118 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus(current_match, result, false);
        }
        119 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        120 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        121 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        122 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        123 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        124 => {
            (*result).next_state = CommandParserState::Initial;
            let command_text = captures
                .get_string(b"direction")
                .expect("command grammar supplies direction")
                .to_vec();
            cmd_split(current_match, result, &command_text);
        }
        125 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_exec(
                current_match,
                result,
                captures.get_string(b"nosn"),
                captures
                    .get_string(b"command")
                    .expect("command grammar supplies required text"),
            );
        }
        126 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(current_match, result, captures.get_string(b"kill_mode"));
        }
        127 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(current_match, result, captures.get_string(b"kill_mode"));
        }
        128 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(current_match, result, captures.get_string(b"kill_mode"));
        }
        129 => {
            (*result).next_state = CommandParserState::Initial;
            let mark = captures
                .get_string(b"mark")
                .expect("command grammar supplies mark")
                .to_vec();
            let mode = captures.get_string(b"mode").map(|value| value.to_vec());
            let toggle = captures.get_string(b"toggle").map(|value| value.to_vec());
            cmd_mark(
                current_match,
                result,
                &mark,
                mode.as_deref(),
                toggle.as_deref(),
            );
        }
        130 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_mode(
                current_match,
                result,
                captures
                    .get_string(b"mode")
                    .expect("command grammar supplies required text"),
            );
        }
        131 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_scratchpad(current_match, result);
        }
        132 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_nop(current_match, result, captures.get_string(b"comment"));
        }
        133 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_nop(current_match, result, None);
        }
        _ => {
            printf(c"BUG in the parser. state = %d\n".as_ptr(), call_identifier);
            __assert_fail(
                c"false".as_ptr(),
                c"./GENERATED_command_call.h".as_ptr(),
                1078 as ::core::ffi::c_uint,
                c"void GENERATED_call(Match *, struct stack *, const int, struct CommandResultIR *)".as_ptr(),
            );
        }
    };
}
fn state_tokens(state: CommandParserState) -> &'static [cmdp_token] {
    tokens
        .get(state as usize)
        .copied()
        .expect("grammar state has a token table")
}

unsafe fn next_state(token: &cmdp_token, cmd_ctx: *mut cmd_parser_ctx) {
    if token.next_state == CommandParserState::Call {
        (*cmd_ctx).subcommand_output.ctx = cmd_ctx;
        (*cmd_ctx).subcommand_output.client = (*cmd_ctx).command_output.client;
        (*cmd_ctx).subcommand_output.needs_tree_render = false;
        GENERATED_call(
            &raw mut (*cmd_ctx).current_match,
            &raw mut (*cmd_ctx).stack,
            token.extra.call_identifier as i32,
            &raw mut (*cmd_ctx).subcommand_output,
        );
        (*cmd_ctx).state = (*cmd_ctx).subcommand_output.next_state;
        if (*cmd_ctx).subcommand_output.needs_tree_render {
            (*cmd_ctx).command_output.needs_tree_render = true;
        }
        (*cmd_ctx).stack.clear();
        return;
    }
    (*cmd_ctx).state = token.next_state;
    if (*cmd_ctx).state == CommandParserState::Initial {
        (*cmd_ctx).stack.clear();
    }
}

/// Parse and execute commands, returning an owned diagnostic and render flag.
///
/// # Safety
/// Command handlers require exclusive event-thread access to the initialized
/// window graph and display state. A non-null client must identify a live IPC
/// client at entry; handlers can reload or replace that state during dispatch.
/// No borrowed graph data may survive command callbacks.
pub unsafe fn parse_command(
    input: &[u8],
    output: Option<&mut Vec<u8>>,
    client: *mut ipc_client,
) -> CommandResult {
    // Own bytes before callbacks. The existing private scanner currently uses
    // pointer cursors, so its bounded backing Vec includes one trailing zero.
    let mut input_owner = input.split(|byte| *byte == 0).next().unwrap().to_vec();
    let log_value_0 = input_owner[..input_owner.len().min(4000)].to_vec();
    input_owner.push(0);
    let input = input_owner.as_ptr().cast::<libc::c_char>();
    {
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands_parser.c:parse_command:159 - COMMAND: *{}*\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "../src/commands_parser.c:parse_command:159 - COMMAND: *".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("*\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let mut output = output;
    let mut cmd_ctx: cmd_parser_ctx = cmd_parser_ctx {
        json_output: output
            .as_ref()
            .map(|_| crate::ipc::CommandReplies::default()),
        state: CommandParserState::RenameWorkspaceLikelyToNewName,
        current_match: Match::empty(),
        stack: stack::default(),
        owindows: owindows_head {
            tqh_first: ::core::ptr::null_mut::<owindow>(),
            tqh_last: ::core::ptr::null_mut::<*mut owindow>(),
        },
        subcommand_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
            needs_tree_render: false,
        },
        command_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
            needs_tree_render: false,
        },
    };
    cmd_ctx.state = CommandParserState::Initial;
    let mut result = CommandResult::default();
    cmd_ctx.command_output.ctx = &raw mut cmd_ctx;
    cmd_ctx.command_output.client = client;
    cmd_ctx.command_output.needs_tree_render = false;
    let mut walk: *const ::core::ffi::c_char = input;
    let len = input_owner.len() - 1;
    cmd_criteria_init(
        &raw mut cmd_ctx.current_match,
        &raw mut cmd_ctx.subcommand_output,
    );
    while walk.offset_from(input) as i64 as usize <= len {
        while (*walk as i32 == ' ' as i32
            || *walk as i32 == '\t' as i32
            || *walk as i32 == '\r' as i32
            || *walk as i32 == '\n' as i32)
            && *walk as i32 != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let grammar = state_tokens(cmd_ctx.state);
        let mut token_handled = false;
        for token in grammar {
            if let Some(literal) = token.name.strip_prefix(b"'") {
                let remaining = &input_owner[walk.offset_from(input) as usize..len];
                if remaining.len() >= literal.len()
                    && crate::util::text_eq_ignore_case(&remaining[..literal.len()], literal)
                {
                    if let Some(identifier) = token.identifier {
                        cmd_ctx.stack.push_string(identifier, literal);
                    }
                    walk = walk.add(literal.len());
                    next_state(token, &raw mut cmd_ctx);
                    token_handled = true;
                    break;
                }
            } else if token.name == b"number" {
                let mut end = std::ptr::null_mut();
                *__errno_location() = 0;
                let num = strtol(walk, &raw mut end, 10);
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0 && num == 0)
                    && !std::ptr::eq(end, walk)
                {
                    if let Some(identifier) = token.identifier {
                        cmd_ctx.stack.push_long(identifier, num);
                    }
                    walk = end;
                    next_state(token, &raw mut cmd_ctx);
                    token_handled = true;
                    break;
                }
            } else {
                if token.name == b"string" || token.name == b"word" {
                    let capture = crate::parser_util::lex_string(
                        &input_owner[walk.offset_from(input) as usize..len],
                        token.name.first() != Some(&b's'),
                        crate::parser_util::StringGrammar::Command,
                    );
                    walk = walk.add(capture.consumed);
                    if let Some(bytes) = capture.bytes {
                        if let Some(identifier) = token.identifier {
                            cmd_ctx.stack.push_string(identifier, &bytes);
                        }
                        if *walk as i32 == '"' as i32 {
                            walk = walk.add(1);
                        }
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true;
                        break;
                    }
                }
                if token.name == b"end"
                    && (*walk as i32 == '\0' as i32
                        || *walk as i32 == ',' as i32
                        || *walk as i32 == ';' as i32)
                {
                    next_state(token, &raw mut cmd_ctx);
                    token_handled = true;
                    if *walk as i32 == '\0' as i32 || *walk as i32 == ';' as i32 {
                        cmd_criteria_init(
                            &raw mut cmd_ctx.current_match,
                            &raw mut cmd_ctx.subcommand_output,
                        );
                    }
                    walk = walk.add(1);
                    break;
                }
            }
        }
        if token_handled {
            continue;
        }
        // Grammar names borrow immutable bounded byte literals.
        let names: Vec<_> = grammar.iter().map(|token| token.name).collect();
        let mut message = b"Expected one of these tokens: ".to_vec();
        message.extend_from_slice(&crate::parser_util::expected_tokens(&names, false));
        let offset = (walk.offset_from(input) as usize).min(len);
        let mut caret = vec![b' '; len];
        caret[offset..].fill(b'^');
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = message.to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = input_owner[..len].to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: Your command: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: Your command: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = caret.to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR:               {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR:               ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        result.parse_error = true;
        emit_command_reply(
            &raw mut cmd_ctx.command_output,
            CommandReply::ParseError {
                error: &message,
                input: &input_owner[..len],
                position: &caret,
            },
        );
        result.error_message = Some(message);
        cmd_ctx.stack.clear();
        break;
    }
    if let Some(output) = output {
        *output = cmd_ctx
            .json_output
            .take()
            .expect("requested command output")
            .finish();
    }
    result.needs_tree_render = cmd_ctx.command_output.needs_tree_render;
    while !cmd_ctx.owindows.tqh_first.is_null() {
        let ow: *mut owindow = cmd_ctx.owindows.tqh_first;
        if !(*ow).owindows.tqe_next.is_null() {
            (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
        } else {
            cmd_ctx.owindows.tqh_last = (*ow).owindows.tqe_prev;
        }
        *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
        free(ow as *mut ::core::ffi::c_void);
    }
    result
}

pub const ERANGE: i32 = 34_i32;
pub const __LONG_MAX__: i64 = 9223372036854775807_i64;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{__errno_location, free, printf, strcmp, strcpy, strlen, strncasecmp, strtol};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;

/// A valid state in the generated command parser table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CommandParserState {
    Call = 77,
    Nop = 76,
    Bar = 75,
    Swap = 74,
    Move = 73,
    Mode = 72,
    Mark = 71,
    Kill = 70,
    Gaps = 69,
    Exec = 68,
    Split = 67,
    Focus = 66,
    Unmark = 65,
    Sticky = 64,
    Shmlog = 63,
    Resize = 62,
    Rename = 61,
    Layout = 60,
    Border = 59,
    Initial = 58,
    GapsEnd = 57,
    Floating = 56,
    Debuglog = 55,
    Criteria = 54,
    BarMode = 53,
    Workspace = 52,
    ResizePx = 51,
    Criterion = 50,
    Scratchpad = 49,
    ResizeSet = 48,
    Fullscreen = 47,
    FocusAuto = 46,
    BarModeId = 45,
    TitleFormat = 44,
    ResizeWidth = 43,
    MoveToMark = 42,
    FocusOutput = 41,
    BorderWidth = 40,
    SwapArgument = 39,
    ResizeTiling = 38,
    ResizeHeight = 37,
    LayoutToggle = 36,
    CriterionStr = 35,
    AppendLayout = 34,
    MoveWorkspace = 33,
    MoveToOutput = 32,
    MoveDirection = 31,
    GapsWithType = 30,
    GapsWithMode = 29,
    CriterionFrom = 28,
    GapsWithScope = 27,
    FullscreenMode = 26,
    WorkspaceNumber = 25,
    ResizeTilingOr = 24,
    ResizeDirection = 23,
    RenameWorkspace = 22,
    MoveToPosition = 21,
    BarHiddenState = 20,
    TitleWindowIcon = 19,
    FullscreenCompat = 18,
    MoveToPositionY = 17,
    MoveToPositionX = 16,
    CriterionFromStr = 15,
    ResizeTilingFinal = 14,
    RenameWorkspaceTo = 13,
    BarHiddenStateId = 12,
    MoveWorkspaceNumber = 11,
    MoveDirectionNumber = 10,
    CriterionFromStrEnd = 9,
    ResizeHeightGetNumber = 8,
    MoveWorkspaceToOutput = 7,
    CriterionFromStrStart = 6,
    TitleWindowIconPadding = 5,
    MoveToAbsolutePosition = 4,
    RenameWorkspaceLikelyTo = 3,
    RenameWorkspaceToNewName = 2,
    MoveWorkspaceToOutputWord = 1,
    RenameWorkspaceLikelyToNewName = 0,
}

#[cfg(test)]
mod owned_parser_tests {
    use super::*;
    #[test]
    fn command_results_own_errors_after_input_and_subsequent_parses_are_dropped() {
        let mut output = Vec::new();
        let result = {
            let input = b"not-a-command\0ignored".to_vec();
            // SAFETY: an unknown initial token only touches the local parser
            // context and immutable grammar; it dispatches no window commands.
            unsafe { parse_command(&input, Some(&mut output), std::ptr::null_mut()) }
        };
        assert!(result.parse_error);
        assert!(!result.needs_tree_render);
        let error = result.error_message.as_ref().unwrap().clone();
        assert!(error.starts_with(b"Expected one of these tokens: "));
        let reply: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(reply[0]["input"], "not-a-command");
        assert_eq!(reply[0]["error"].as_str().unwrap().as_bytes(), error);
        // The second result and its input die first; the earlier message must
        // remain independently owned without a native free function.
        drop(unsafe { parse_command(b"another-unknown-command", None, std::ptr::null_mut()) });
        assert_eq!(result.error_message.as_ref(), Some(&error));
    }

    #[test]
    fn command_lexer_owns_captures_and_reports_consumed_offsets() {
        let capture = crate::parser_util::lex_string(
            b"\"a\\\"b\"rest",
            false,
            crate::parser_util::StringGrammar::Command,
        );
        assert_eq!(capture.bytes.as_deref(), Some(b"a\"b".as_slice()));
        assert_eq!(capture.consumed, 5);
        let empty = crate::parser_util::lex_string(
            b"\"\"",
            false,
            crate::parser_util::StringGrammar::Command,
        );
        assert!(empty.bytes.is_none());
        assert_eq!(empty.consumed, 1);
    }
    #[test]
    fn generated_command_tables_keep_complete_states_and_dispatch_ids() {
        let states = &tokens;
        assert_eq!(states.len(), 77);
        let mut calls = std::collections::BTreeSet::new();
        for table in states {
            assert!(!table.is_empty());
            let entries = *table;
            for entry in entries {
                assert!(!entry.name.is_empty());
                if entry.next_state == CommandParserState::Call {
                    calls.insert(entry.extra.call_identifier);
                } else {
                    assert!((entry.next_state as usize) < states.len());
                }
            }
        }
        assert_eq!(
            calls.into_iter().collect::<Vec<_>>(),
            (0..134).collect::<Vec<u16>>()
        );
    }
}
