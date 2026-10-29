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
use crate::parser_util::parser_clear_stack;
use crate::parser_util::parser_get_long;
use crate::parser_util::parser_get_string;
use crate::parser_util::parser_push_long;
use crate::parser_util::parser_push_string;
use crate::types::UrgencyMatch;
use crate::types::WindowMode;
use i3_common::safewrappers::sasprintf;
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
pub use i3_json::compat::{yajl_gen, yajl_gen_t};
use i3_json::compat::{
    yajl_gen_array_close, yajl_gen_array_open, yajl_gen_bool, yajl_gen_map_close,
    yajl_gen_map_open, yajl_gen_string,
};

pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_NODOCK: ::core::ffi::c_int = 0;

pub use crate::regex::Regex as regex;

pub use crate::application::ipc_client;

pub use crate::assignments::CommandResult;
pub use crate::commands::cmd_parser_ctx;
pub use crate::commands::owindow;
pub use crate::commands::owindows_head;
pub use crate::commands::stack;
pub use crate::commands::CommandResultIR;
pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: CommandParserState,
    pub extra: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub call_identifier: u16,
}

pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 82] = unsafe {
    ::core::mem::transmute::<[u8; 82], [::core::ffi::c_char; 82]>(
        *b"void GENERATED_call(Match *, struct stack *, const int, struct CommandResultIR *)\0",
    )
};
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME: [cmdp_token; 2] = [
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"new_name".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 1 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 2 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 3 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RENAME_WORKSPACE_TO_NEW_NAME: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"new_name".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 4 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RENAME_WORKSPACE_LIKELY_TO: [cmdp_token; 2] = [
    token {
        name: c"'to ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"new_name".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 5 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_ABSOLUTE_POSITION: [cmdp_token; 1] = [token {
    name: c"'position".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::MoveToPosition,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_TITLE_WINDOW_ICON_PADDING: [cmdp_token; 3] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 6 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 7 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"padding".as_ptr().cast_mut(),
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::CriterionFromStr,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 8 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 9 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_TO_OUTPUT: [cmdp_token; 1] = [token {
    name: c"'output".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::MoveWorkspaceToOutputWord,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RESIZE_HEIGHT_GET_NUMBER: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: CommandParserState::ResizeHeight,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 10 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MOVE_DIRECTION_NUMBER: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 11 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 12 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 13 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 14 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_HIDDEN_STATE_ID: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"bar_id".as_ptr().cast_mut(),
        next_state: CommandParserState::BarHiddenStateId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 15 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RENAME_WORKSPACE_TO: [cmdp_token; 1] = [token {
    name: c"'to".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::RenameWorkspaceToNewName,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RESIZE_TILING_FINAL: [cmdp_token; 2] = [
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 16 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 17 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CommandParserState::CriterionFromStrEnd,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CommandParserState::CriterionFromStrEnd,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_POSITION_X: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: c"mode_x".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToPositionX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: c"mode_x".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToPositionX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"coord_y".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToPositionY,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_POSITION_Y: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: c"mode_y".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 18 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: c"mode_y".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 19 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 20 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FULLSCREEN_COMPAT: [cmdp_token; 2] = [
    token {
        name: c"'global".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 21 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 22 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_TITLE_WINDOW_ICON: [cmdp_token; 14] = [
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::TitleWindowIconPadding,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 23 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 24 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 25 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 26 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 27 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 28 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 29 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 30 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 31 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 32 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 33 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enable".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 34 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 3] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarHiddenStateId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarHiddenStateId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarHiddenStateId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_POSITION: [cmdp_token; 5] = [
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 35 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mouse".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 36 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'cursor".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 37 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'pointer".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 38 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"coord_x".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToPositionX,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RENAME_WORKSPACE: [cmdp_token; 2] = [
    token {
        name: c"'to".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::RenameWorkspaceLikelyTo,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"old_name".as_ptr().cast_mut(),
        next_state: CommandParserState::RenameWorkspaceTo,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESIZE_DIRECTION: [cmdp_token; 6] = [
    token {
        name: c"'up".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'down".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'width".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizePx,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESIZE_TILING_OR: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"resize_ppt".as_ptr().cast_mut(),
    next_state: CommandParserState::ResizeTilingFinal,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 39 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_FULLSCREEN_MODE: [cmdp_token; 2] = [
    token {
        name: c"'global".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 40 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 41 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 4] = [
    token {
        name: c"'plus".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'minus".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::CriterionFromStrStart,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS_WITH_MODE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: CommandParserState::GapsEnd,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_GAPS_WITH_TYPE: [cmdp_token; 2] = [
    token {
        name: c"'current".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithScope,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithScope,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_DIRECTION: [cmdp_token; 2] = [
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"amount".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveDirectionNumber,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 42 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_OUTPUT: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 43 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 44 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_WORKSPACE: [cmdp_token; 9] = [
    token {
        name: c"'to ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::MoveWorkspaceToOutput,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'next_on_output".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 45 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'prev_on_output".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 46 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'next".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 47 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'prev".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 48 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'current".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 49 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 50 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::MoveWorkspaceNumber,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 51 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_APPEND_LAYOUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 52 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 53 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_LAYOUT_TOGGLE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 54 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"toggle_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 55 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESIZE_HEIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: c"mode_height".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeHeight,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: c"mode_height".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeHeight,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 56 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESIZE_TILING: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::ResizeTiling,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'or".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::ResizeTilingOr,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 57 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_SWAP_ARGUMENT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"arg".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 58 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BORDER_WIDTH: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 59 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"border_width".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 60 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_OUTPUT: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 61 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 62 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MOVE_TO_MARK: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"mark".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 63 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RESIZE_WIDTH: [cmdp_token; 5] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: c"mode_width".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'ppt".as_ptr().cast_mut(),
        identifier: c"mode_width".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 64 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::ResizeHeightGetNumber,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"height".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeHeight,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_TITLE_FORMAT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"format".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 65 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_BAR_MODE_ID: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"bar_id".as_ptr().cast_mut(),
        next_state: CommandParserState::BarModeId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 66 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS_AUTO: [cmdp_token; 2] = [
    token {
        name: c"'sibling".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 67 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 68 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 69 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::FullscreenMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::FullscreenMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::FullscreenCompat,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RESIZE_SET: [cmdp_token; 3] = [
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::ResizeHeightGetNumber,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'width".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::ResizeSet,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_SCRATCHPAD: [cmdp_token; 1] = [token {
    name: c"'show".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 70 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::CriterionStr,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RESIZE_PX: [cmdp_token; 2] = [
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"resize_px".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeTiling,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 71 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 8] = [
    token {
        name: c"'--no-auto-back-and-forth".as_ptr().cast_mut(),
        identifier: c"no_auto_back_and_forth".as_ptr().cast_mut(),
        next_state: CommandParserState::Workspace,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'next_on_output".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 72 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'prev_on_output".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 73 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'next".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 74 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'prev".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 75 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 76 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::WorkspaceNumber,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 77 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR_MODE: [cmdp_token; 4] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarModeId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarModeId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarModeId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"bar_value".as_ptr().cast_mut(),
        next_state: CommandParserState::BarModeId,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Criterion,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::CriterionFrom,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::CriterionFrom,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 78 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 79 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 80 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 81 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_DEBUGLOG: [cmdp_token; 3] = [
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"argument".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 82 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"argument".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 83 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"argument".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 84 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FLOATING: [cmdp_token; 3] = [
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"floating".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 85 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"floating".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 86 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"floating".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 87 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::GapsEnd,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 88 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_INITIAL: [cmdp_token; 32] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Initial,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'[".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 89 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'move".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Move,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Exec,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'exit".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 90 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'restart".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 91 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'reload".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 92 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'shmlog".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Shmlog,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'debuglog".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Debuglog,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'border".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Border,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Layout,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'append_layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::AppendLayout,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Workspace,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'focus".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Focus,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'kill".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Kill,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'open".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 93 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'fullscreen".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Fullscreen,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'sticky".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Sticky,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'split".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Split,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Floating,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mark".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Mark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'unmark".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Unmark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'resize".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Resize,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'rename".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Rename,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'nop".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Nop,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'scratchpad".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Scratchpad,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'swap".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Swap,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title_format".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::TitleFormat,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'title_window_icon".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::TitleWindowIcon,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Mode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Bar,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Gaps,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BORDER: [cmdp_token; 5] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border_style".as_ptr().cast_mut(),
        next_state: CommandParserState::BorderWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border_style".as_ptr().cast_mut(),
        next_state: CommandParserState::BorderWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"border_style".as_ptr().cast_mut(),
        next_state: CommandParserState::BorderWidth,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border_style".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 94 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 95 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_LAYOUT: [cmdp_token; 7] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 96 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 97 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 98 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 99 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'splitv".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 100 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'splith".as_ptr().cast_mut(),
        identifier: c"layout_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 101 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::LayoutToggle,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_RENAME: [cmdp_token; 1] = [token {
    name: c"'workspace".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CommandParserState::RenameWorkspace,
    extra: C2RustUnnamed_20 {
        call_identifier: 0 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_RESIZE: [cmdp_token; 3] = [
    token {
        name: c"'grow".as_ptr().cast_mut(),
        identifier: c"way".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'shrink".as_ptr().cast_mut(),
        identifier: c"way".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: c"set".as_ptr().cast_mut(),
        next_state: CommandParserState::ResizeSet,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_SHMLOG: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"argument".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 102 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_STICKY: [cmdp_token; 3] = [
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 103 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 104 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"action".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 105 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_UNMARK: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 106 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"mark".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 107 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_FOCUS: [cmdp_token; 14] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 108 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 109 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'up".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 110 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'down".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 111 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'prev".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::FocusAuto,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'next".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::FocusAuto,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::FocusOutput,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"window_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 112 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"window_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 113 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode_toggle".as_ptr().cast_mut(),
        identifier: c"window_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 114 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'parent".as_ptr().cast_mut(),
        identifier: c"level".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 115 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'child".as_ptr().cast_mut(),
        identifier: c"level".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 116 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 117 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 118 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_SPLIT: [cmdp_token; 6] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 119 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 120 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'toggle".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 121 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'v".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 122 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'h".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 123 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'t".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 124 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"nosn".as_ptr().cast_mut(),
        next_state: CommandParserState::Exec,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 125 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"type".as_ptr().cast_mut(),
        next_state: CommandParserState::GapsWithType,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_KILL: [cmdp_token; 3] = [
    token {
        name: c"'window".as_ptr().cast_mut(),
        identifier: c"kill_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 126 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'client".as_ptr().cast_mut(),
        identifier: c"kill_mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 127 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 128 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MARK: [cmdp_token; 4] = [
    token {
        name: c"'--add".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Mark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--replace".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::Mark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--toggle".as_ptr().cast_mut(),
        identifier: c"toggle".as_ptr().cast_mut(),
        next_state: CommandParserState::Mark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"mark".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 129 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: CommandParserState::Call,
    extra: C2RustUnnamed_20 {
        call_identifier: 130 as ::core::ffi::c_int as u16,
    },
}];
static mut tokens_MOVE: [cmdp_token; 14] = [
    token {
        name: c"'window".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Move,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'container".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Move,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'to".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Move,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'--no-auto-back-and-forth".as_ptr().cast_mut(),
        identifier: c"no_auto_back_and_forth".as_ptr().cast_mut(),
        next_state: CommandParserState::Move,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::MoveWorkspace,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::MoveToOutput,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mark".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::MoveToMark,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'scratchpad".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 131 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'up".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'down".as_ptr().cast_mut(),
        identifier: c"direction".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveDirection,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: c"method".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToPosition,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'absolute".as_ptr().cast_mut(),
        identifier: c"method".as_ptr().cast_mut(),
        next_state: CommandParserState::MoveToAbsolutePosition,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_SWAP: [cmdp_token; 5] = [
    token {
        name: c"'container".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Swap,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'with".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Swap,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::SwapArgument,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::SwapArgument,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mark".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: CommandParserState::SwapArgument,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 2] = [
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::BarHiddenState,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::BarMode,
        extra: C2RustUnnamed_20 {
            call_identifier: 0 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens_NOP: [cmdp_token; 2] = [
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"comment".as_ptr().cast_mut(),
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 132 as ::core::ffi::c_int as u16,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CommandParserState::Call,
        extra: C2RustUnnamed_20 {
            call_identifier: 133 as ::core::ffi::c_int as u16,
        },
    },
];
static mut tokens: [cmdp_token_ptr; 77] = unsafe {
    [
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_LIKELY_TO_NEW_NAME as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_TO_OUTPUT_WORD as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_TO_NEW_NAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_LIKELY_TO as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_ABSOLUTE_POSITION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_WINDOW_ICON_PADDING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_START as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_TO_OUTPUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_HEIGHT_GET_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR_END as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_DIRECTION_NUMBER as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE_ID as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE_TO as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING_FINAL as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM_STR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION_X as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION_Y as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN_COMPAT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_WINDOW_ICON as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_HIDDEN_STATE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_POSITION as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME_WORKSPACE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_DIRECTION as *mut cmdp_token,
            n: 6 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING_OR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE_NUMBER as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN_MODE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_SCOPE as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_FROM as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_MODE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_WITH_TYPE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_DIRECTION as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_OUTPUT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_WORKSPACE as *mut cmdp_token,
            n: 9 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_APPEND_LAYOUT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION_STR as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_LAYOUT_TOGGLE as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_HEIGHT as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_TILING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SWAP_ARGUMENT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BORDER_WIDTH as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_OUTPUT as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE_TO_MARK as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_WIDTH as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_TITLE_FORMAT as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE_ID as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS_AUTO as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FULLSCREEN as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_SET as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SCRATCHPAD as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERION as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE_PX as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_WORKSPACE as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR_MODE as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_CRITERIA as *mut cmdp_token,
            n: 17 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_DEBUGLOG as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FLOATING as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS_END as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_INITIAL as *mut cmdp_token,
            n: 32 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BORDER as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_LAYOUT as *mut cmdp_token,
            n: 7 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RENAME as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_RESIZE as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SHMLOG as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_STICKY as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_UNMARK as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_FOCUS as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SPLIT as *mut cmdp_token,
            n: 6 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_EXEC as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_GAPS as *mut cmdp_token,
            n: 8 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_KILL as *mut cmdp_token,
            n: 3 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MARK as *mut cmdp_token,
            n: 4 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MODE as *mut cmdp_token,
            n: 1 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_MOVE as *mut cmdp_token,
            n: 14 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_SWAP as *mut cmdp_token,
            n: 5 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_BAR as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
        tokenptr {
            array: &raw const tokens_NOP as *mut cmdp_token,
            n: 2 as ::core::ffi::c_int,
        },
    ]
};
unsafe fn GENERATED_call(
    current_match: *mut Match,
    stack: *mut stack,
    call_identifier: ::core::ffi::c_int,
    result: *mut CommandResultIR,
) {
    match call_identifier {
        0 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_rename_workspace(
                current_match,
                result,
                c"to".as_ptr(),
                parser_get_string(stack, c"new_name".as_ptr()),
            );
        }
        1 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_rename_workspace(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                c"to".as_ptr(),
            );
        }
        2 => {
            (*result).next_state = CommandParserState::MoveWorkspaceToOutputWord;
            cmd_move_con_to_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
                1 as ::core::ffi::c_int != 0,
            );
        }
        3 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                1 as ::core::ffi::c_int != 0,
            );
        }
        4 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_rename_workspace(
                current_match,
                result,
                parser_get_string(stack, c"old_name".as_ptr()),
                parser_get_string(stack, c"new_name".as_ptr()),
            );
        }
        5 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_rename_workspace(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                parser_get_string(stack, c"new_name".as_ptr()),
            );
        }
        6 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                parser_get_long(stack, c"padding".as_ptr()) as ::core::ffi::c_int,
            );
        }
        7 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                parser_get_long(stack, c"padding".as_ptr()) as ::core::ffi::c_int,
            );
        }
        8 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        9 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        10 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        11 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"amount".as_ptr()),
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        12 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"amount".as_ptr()),
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        13 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"amount".as_ptr()),
                c"px".as_ptr(),
            );
        }
        14 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace_number(
                current_match,
                result,
                parser_get_string(stack, c"number".as_ptr()),
                parser_get_string(stack, c"no_auto_back_and_forth".as_ptr()),
            );
        }
        15 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_bar_hidden_state(
                current_match,
                result,
                parser_get_string(stack, c"bar_value".as_ptr()),
                parser_get_string(stack, c"bar_id".as_ptr()),
            );
        }
        16 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, c"way".as_ptr()),
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"resize_px".as_ptr()),
                parser_get_long(stack, c"resize_ppt".as_ptr()),
            );
        }
        17 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, c"way".as_ptr()),
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"resize_px".as_ptr()),
                parser_get_long(stack, c"resize_ppt".as_ptr()),
            );
        }
        18 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(stack, c"coord_x".as_ptr()),
                parser_get_string(stack, c"mode_x".as_ptr()),
                parser_get_long(stack, c"coord_y".as_ptr()),
                parser_get_string(stack, c"mode_y".as_ptr()),
            );
        }
        19 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(stack, c"coord_x".as_ptr()),
                parser_get_string(stack, c"mode_x".as_ptr()),
                parser_get_long(stack, c"coord_y".as_ptr()),
                parser_get_string(stack, c"mode_y".as_ptr()),
            );
        }
        20 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_position(
                current_match,
                result,
                parser_get_long(stack, c"coord_x".as_ptr()),
                parser_get_string(stack, c"mode_x".as_ptr()),
                parser_get_long(stack, c"coord_y".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        21 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                c"toggle".as_ptr(),
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        22 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                c"toggle".as_ptr(),
                c"output".as_ptr(),
            );
        }
        23 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        24 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        25 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        26 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        27 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        28 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        29 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        30 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        31 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        32 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        33 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        34 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_window_icon(
                current_match,
                result,
                parser_get_string(stack, c"enable".as_ptr()),
                0 as ::core::ffi::c_int,
            );
        }
        35 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_window_to_center(
                current_match,
                result,
                parser_get_string(stack, c"method".as_ptr()),
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
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"no_auto_back_and_forth".as_ptr()),
            );
        }
        40 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        41 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
                c"output".as_ptr(),
            );
        }
        42 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
                10 as ::core::ffi::c_long,
                c"px".as_ptr(),
            );
        }
        43 => {
            (*result).next_state = CommandParserState::MoveToOutput;
            cmd_move_con_to_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
                0 as ::core::ffi::c_int != 0,
            );
        }
        44 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_int != 0,
            );
        }
        45 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
            );
        }
        46 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
            );
        }
        47 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
            );
        }
        48 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
            );
        }
        49 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
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
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"no_auto_back_and_forth".as_ptr()),
            );
        }
        52 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_append_layout(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        53 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"cvalue".as_ptr()),
            );
        }
        54 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout_toggle(
                current_match,
                result,
                parser_get_string(stack, c"toggle_mode".as_ptr()),
            );
        }
        55 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout_toggle(
                current_match,
                result,
                parser_get_string(stack, c"toggle_mode".as_ptr()),
            );
        }
        56 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize_set(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_string(stack, c"mode_width".as_ptr()),
                parser_get_long(stack, c"height".as_ptr()),
                parser_get_string(stack, c"mode_height".as_ptr()),
            );
        }
        57 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize(
                current_match,
                result,
                parser_get_string(stack, c"way".as_ptr()),
                parser_get_string(stack, c"direction".as_ptr()),
                parser_get_long(stack, c"resize_px".as_ptr()),
                0 as ::core::ffi::c_long,
            );
        }
        58 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_swap(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
                parser_get_string(stack, c"arg".as_ptr()),
            );
        }
        59 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                parser_get_string(stack, c"border_style".as_ptr()),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        60 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                parser_get_string(stack, c"border_style".as_ptr()),
                parser_get_long(stack, c"border_width".as_ptr()),
            );
        }
        61 => {
            (*result).next_state = CommandParserState::FocusOutput;
            cmd_focus_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        62 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_output(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        63 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_con_to_mark(
                current_match,
                result,
                parser_get_string(stack, c"mark".as_ptr()),
            );
        }
        64 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_resize_set(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_string(stack, c"mode_width".as_ptr()),
                0 as ::core::ffi::c_long,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        65 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_title_format(
                current_match,
                result,
                parser_get_string(stack, c"format".as_ptr()),
            );
        }
        66 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"bar_value".as_ptr()),
                parser_get_string(stack, c"bar_id".as_ptr()),
            );
        }
        67 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_sibling(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        68 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        69 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
                c"output".as_ptr(),
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
                parser_get_string(stack, c"way".as_ptr()),
                parser_get_string(stack, c"direction".as_ptr()),
                10 as ::core::ffi::c_long,
                10 as ::core::ffi::c_long,
            );
        }
        72 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        73 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        74 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        75 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_workspace(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
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
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"no_auto_back_and_forth".as_ptr()),
            );
        }
        78 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        79 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        80 => {
            (*result).next_state = CommandParserState::Criteria;
            cmd_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
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
                parser_get_string(stack, c"argument".as_ptr()),
            );
        }
        83 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_debuglog(
                current_match,
                result,
                parser_get_string(stack, c"argument".as_ptr()),
            );
        }
        84 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_debuglog(
                current_match,
                result,
                parser_get_string(stack, c"argument".as_ptr()),
            );
        }
        85 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                parser_get_string(stack, c"floating".as_ptr()),
            );
        }
        86 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                parser_get_string(stack, c"floating".as_ptr()),
            );
        }
        87 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_floating(
                current_match,
                result,
                parser_get_string(stack, c"floating".as_ptr()),
            );
        }
        88 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_gaps(
                current_match,
                result,
                parser_get_string(stack, c"type".as_ptr()),
                parser_get_string(stack, c"scope".as_ptr()),
                parser_get_string(stack, c"mode".as_ptr()),
                parser_get_string(stack, c"value".as_ptr()),
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
                parser_get_string(stack, c"border_style".as_ptr()),
                0 as ::core::ffi::c_long,
            );
        }
        95 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_border(
                current_match,
                result,
                c"pixel".as_ptr(),
                1 as ::core::ffi::c_long,
            );
        }
        96 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        97 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        98 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        99 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        100 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        101 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout_mode".as_ptr()),
            );
        }
        102 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_shmlog(
                current_match,
                result,
                parser_get_string(stack, c"argument".as_ptr()),
            );
        }
        103 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
            );
        }
        104 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
            );
        }
        105 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_sticky(
                current_match,
                result,
                parser_get_string(stack, c"action".as_ptr()),
            );
        }
        106 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_unmark(
                current_match,
                result,
                parser_get_string(stack, c"mark".as_ptr()),
            );
        }
        107 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_unmark(
                current_match,
                result,
                parser_get_string(stack, c"mark".as_ptr()),
            );
        }
        108 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        109 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        110 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        111 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_direction(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        112 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(stack, c"window_mode".as_ptr()),
            );
        }
        113 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(stack, c"window_mode".as_ptr()),
            );
        }
        114 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_window_mode(
                current_match,
                result,
                parser_get_string(stack, c"window_mode".as_ptr()),
            );
        }
        115 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_level(
                current_match,
                result,
                parser_get_string(stack, c"level".as_ptr()),
            );
        }
        116 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus_level(
                current_match,
                result,
                parser_get_string(stack, c"level".as_ptr()),
            );
        }
        117 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus(current_match, result, 1 as ::core::ffi::c_int != 0);
        }
        118 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_focus(current_match, result, 0 as ::core::ffi::c_int != 0);
        }
        119 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        120 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        121 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        122 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        123 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        124 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_split(
                current_match,
                result,
                parser_get_string(stack, c"direction".as_ptr()),
            );
        }
        125 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_exec(
                current_match,
                result,
                parser_get_string(stack, c"nosn".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        126 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(
                current_match,
                result,
                parser_get_string(stack, c"kill_mode".as_ptr()),
            );
        }
        127 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(
                current_match,
                result,
                parser_get_string(stack, c"kill_mode".as_ptr()),
            );
        }
        128 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_kill(
                current_match,
                result,
                parser_get_string(stack, c"kill_mode".as_ptr()),
            );
        }
        129 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_mark(
                current_match,
                result,
                parser_get_string(stack, c"mark".as_ptr()),
                parser_get_string(stack, c"mode".as_ptr()),
                parser_get_string(stack, c"toggle".as_ptr()),
            );
        }
        130 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        131 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_move_scratchpad(current_match, result);
        }
        132 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_nop(
                current_match,
                result,
                parser_get_string(stack, c"comment".as_ptr()),
            );
        }
        133 => {
            (*result).next_state = CommandParserState::Initial;
            cmd_nop(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        _ => {
            printf(c"BUG in the parser. state = %d\n".as_ptr(), call_identifier);
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"./GENERATED_command_call.h".as_ptr(),
                    1078 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        }
    };
}
// These private generated arrays are initialized once and never mutated. Only
// the legacy metadata representation uses raw pointers; callers receive a
// checked immutable view, and no user-supplied pointer enters this boundary.
fn state_tokens(state: CommandParserState) -> &'static [cmdp_token] {
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

unsafe fn next_state(token: *const cmdp_token, cmd_ctx: *mut cmd_parser_ctx) {
    if (*token).next_state == CommandParserState::Call {
        (*cmd_ctx).subcommand_output.ctx = cmd_ctx as *mut cmd_parser_ctx;
        (*cmd_ctx).subcommand_output.json_gen = (*cmd_ctx).command_output.json_gen;
        (*cmd_ctx).subcommand_output.client = (*cmd_ctx).command_output.client;
        (*cmd_ctx).subcommand_output.needs_tree_render = false_0 != 0;
        GENERATED_call(
            &raw mut (*cmd_ctx).current_match,
            &raw mut (*cmd_ctx).stack,
            (*token).extra.call_identifier as ::core::ffi::c_int,
            &raw mut (*cmd_ctx).subcommand_output,
        );
        (*cmd_ctx).state = (*cmd_ctx).subcommand_output.next_state;
        if (*cmd_ctx).subcommand_output.needs_tree_render {
            (*cmd_ctx).command_output.needs_tree_render = true_0 != 0;
        }
        parser_clear_stack(&raw mut (*cmd_ctx).stack);
        return;
    }
    (*cmd_ctx).state = (*token).next_state;
    if (*cmd_ctx).state == CommandParserState::Initial {
        parser_clear_stack(&raw mut (*cmd_ctx).stack);
    }
}

/// # Safety
/// walk is writable and points to a live NUL-terminated input cursor. Returned
/// storage is malloc-owned; the updated cursor remains inside the original input.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn parse_string(walk: *mut *const libc::c_char, as_word: bool) -> *mut libc::c_char {
    // SAFETY: caller supplies readable terminated input and writable cursor storage.
    let input = unsafe { std::ffi::CStr::from_ptr(*walk) };
    let capture = crate::parser_util::lex_string(
        input.to_bytes(),
        as_word,
        crate::parser_util::StringGrammar::Command,
    );
    // SAFETY: consumed is bounded by the original C string, including its terminator.
    unsafe { *walk = (*walk).add(capture.consumed) };
    capture.bytes.map_or(std::ptr::null_mut(), |bytes| {
        // SAFETY: initialized bytes are readable for their bounded length; the
        // native duplicate transfers independent malloc-compatible ownership.
        unsafe { i3_common::safewrappers::sstrndup(bytes.as_ptr().cast(), bytes.len()) }
    })
}

pub unsafe fn parse_command(
    input: *const ::core::ffi::c_char,
    gen: yajl_gen,
    client: *mut ipc_client,
) -> *mut CommandResult {
    debuglog(
        c"%s:%s:%d - COMMAND: *%.4000s*\n".as_ptr().cast_mut(),
        c"../src/commands_parser.c".as_ptr(),
        c"parse_command".as_ptr(),
        159 as ::core::ffi::c_int,
        input,
    );
    let mut cmd_ctx: cmd_parser_ctx = cmd_parser_ctx {
        state: CommandParserState::RenameWorkspaceLikelyToNewName,
        current_match: Match {
            error: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            title: ::core::ptr::null_mut::<regex>(),
            application: ::core::ptr::null_mut::<regex>(),
            class: ::core::ptr::null_mut::<regex>(),
            instance: ::core::ptr::null_mut::<regex>(),
            mark: ::core::ptr::null_mut::<regex>(),
            window_role: ::core::ptr::null_mut::<regex>(),
            workspace: ::core::ptr::null_mut::<regex>(),
            machine: ::core::ptr::null_mut::<regex>(),
            window_type: 0,
            urgent: UrgencyMatch::Latest,
            dock: M_NODOCK,
            id: 0,
            window_mode: WindowMode::Any,
            con_id: ::core::ptr::null_mut::<Con>(),
            match_all_windows: false,
            insert_where: M_HERE,
            matches: C2RustUnnamed_5 {
                tqe_next: ::core::ptr::null_mut::<Match>(),
                tqe_prev: ::core::ptr::null_mut::<*mut Match>(),
            },
            restart_mode: false,
        },
        stack: stack::default(),
        owindows: owindows_head {
            tqh_first: ::core::ptr::null_mut::<owindow>(),
            tqh_last: ::core::ptr::null_mut::<*mut owindow>(),
        },
        subcommand_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            json_gen: ::core::ptr::null_mut::<yajl_gen_t>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
            needs_tree_render: false,
        },
        command_output: CommandResultIR {
            ctx: ::core::ptr::null_mut::<cmd_parser_ctx>(),
            json_gen: ::core::ptr::null_mut::<yajl_gen_t>(),
            client: ::core::ptr::null_mut::<ipc_client>(),
            next_state: CommandParserState::RenameWorkspaceLikelyToNewName,
            needs_tree_render: false,
        },
    };
    cmd_ctx.state = CommandParserState::Initial;
    let result: *mut CommandResult =
        scalloc(1 as usize, ::core::mem::size_of::<CommandResult>() as usize) as *mut CommandResult;
    cmd_ctx.command_output.ctx = &raw mut cmd_ctx as *mut cmd_parser_ctx;
    cmd_ctx.command_output.client = client;
    cmd_ctx.command_output.json_gen = gen;
    if !cmd_ctx.command_output.json_gen.is_null() {
        yajl_gen_array_open(cmd_ctx.command_output.json_gen);
    } else {
    };
    cmd_ctx.command_output.needs_tree_render = false_0 != 0;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: usize = strlen(input) as usize;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut token_handled: bool = false;
    cmd_criteria_init(
        &raw mut cmd_ctx.current_match,
        &raw mut cmd_ctx.subcommand_output,
    );
    while walk.offset_from(input) as ::core::ffi::c_long as usize <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32
            || *walk as ::core::ffi::c_int == '\r' as i32
            || *walk as ::core::ffi::c_int == '\n' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let grammar = state_tokens(cmd_ctx.state);
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
                    strlen((*token).name).wrapping_sub(1 as usize),
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*token).identifier.is_null() {
                        parser_push_string(
                            &raw mut cmd_ctx.stack,
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as usize) as isize);
                    next_state(token, &raw mut cmd_ctx);
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
                {
                    if !(end == walk as *mut ::core::ffi::c_char) {
                        if !(*token).identifier.is_null() {
                            parser_push_long(&raw mut cmd_ctx.stack, (*token).identifier, num);
                        }
                        walk = end;
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
            } else {
                if strcmp((*token).name, c"string".as_ptr()) == 0 as ::core::ffi::c_int
                    || strcmp((*token).name, c"word".as_ptr()) == 0 as ::core::ffi::c_int
                {
                    let str: *mut ::core::ffi::c_char = parse_string(
                        &raw mut walk,
                        *(*token).name.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            != 's' as i32,
                    );
                    if !str.is_null() {
                        if !(*token).identifier.is_null() {
                            parser_push_string(&raw mut cmd_ctx.stack, (*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
                if strcmp((*token).name, c"end".as_ptr()) == 0 as ::core::ffi::c_int {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == ',' as i32
                        || *walk as ::core::ffi::c_int == ';' as i32
                    {
                        next_state(token, &raw mut cmd_ctx);
                        token_handled = true_0 != 0;
                        if *walk as ::core::ffi::c_int == '\0' as i32
                            || *walk as ::core::ffi::c_int == ';' as i32
                        {
                            cmd_criteria_init(
                                &raw mut cmd_ctx.current_match,
                                &raw mut cmd_ctx.subcommand_output,
                            );
                        }
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
        if token_handled {
            continue;
        }
        // Names are borrowed from immutable generated C-string literals.
        let names: Vec<_> = grammar
            .iter()
            .map(|token| std::ffi::CStr::from_ptr(token.name))
            .collect();
        let mut message = b"Expected one of these tokens: ".to_vec();
        message.extend_from_slice(&crate::parser_util::expected_tokens(&names, false));
        let message = std::ffi::CString::new(message).expect("grammar names contain no NUL");
        let errormessage =
            i3_common::safewrappers::sstrndup(message.as_ptr(), message.as_bytes().len());
        let offset = (walk.offset_from(input) as usize).min(len);
        let mut caret = vec![b' '; len];
        caret[offset..].fill(b'^');
        let caret = std::ffi::CString::new(caret).expect("caret bytes contain no NUL");
        let position = caret.as_ptr();
        errorlog(c"ERROR: %s\n".as_ptr().cast_mut(), errormessage);
        errorlog(c"ERROR: Your command: %s\n".as_ptr().cast_mut(), input);
        errorlog(c"ERROR:               %s\n".as_ptr().cast_mut(), position);
        (*result).parse_error = true_0 != 0;
        (*result).error_message = errormessage;
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_map_open(cmd_ctx.command_output.json_gen);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_bool(cmd_ctx.command_output.json_gen, false_0);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                c"parse_error".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"parse_error".as_ptr()),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_bool(cmd_ctx.command_output.json_gen, true_0);
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"error".as_ptr()),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                errormessage as *mut ::core::ffi::c_uchar,
                strlen(errormessage),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                c"input".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"input".as_ptr()),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                input as *mut ::core::ffi::c_uchar,
                strlen(input),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                c"errorposition".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"errorposition".as_ptr()),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_string(
                cmd_ctx.command_output.json_gen,
                position as *mut ::core::ffi::c_uchar,
                strlen(position),
            );
        } else {
        };
        if !cmd_ctx.command_output.json_gen.is_null() {
            yajl_gen_map_close(cmd_ctx.command_output.json_gen);
        } else {
        };
        parser_clear_stack(&raw mut cmd_ctx.stack);
        break;
    }
    if !cmd_ctx.command_output.json_gen.is_null() {
        yajl_gen_array_close(cmd_ctx.command_output.json_gen);
    } else {
    };
    (*result).needs_tree_render = cmd_ctx.command_output.needs_tree_render;
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

pub unsafe fn command_result_free(mut result: *mut CommandResult) {
    if result.is_null() {
        return;
    }
    free((*result).error_message as *mut ::core::ffi::c_void);
    (*result).error_message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(result as *mut ::core::ffi::c_void);
    result = ::core::ptr::null_mut::<CommandResult>();
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{__errno_location, free, printf, strcmp, strcpy, strlen, strncasecmp, strtol};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::MatchMatches as C2RustUnnamed_5;
pub use i3_common::string::I3String as _i3String;

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
    fn legacy_string_entry_updates_cursor_and_keeps_malloc_ownership() {
        let input = c"\"a\\\"b\"rest";
        let mut cursor = input.as_ptr();
        // SAFETY: cursor is writable, and input remains a valid native string.
        unsafe {
            let value = parse_string(&mut cursor, false);
            assert_eq!(std::ffi::CStr::from_ptr(value), c"a\"b");
            assert_eq!(cursor.offset_from(input.as_ptr()), 5);
            libc::free(value.cast());
            let mut empty = c"\"\"".as_ptr();
            assert!(parse_string(&mut empty, false).is_null());
            assert_eq!(*empty, b'"' as libc::c_char);
        }
    }
    #[test]
    fn generated_command_tables_keep_complete_states_and_dispatch_ids() {
        // SAFETY: private immutable generated arrays remain initialized for this process.
        let states = unsafe { &*(&raw const tokens) };
        assert_eq!(states.len(), 77);
        let mut calls = std::collections::BTreeSet::new();
        for table in states {
            assert!(table.n > 0);
            let entries = unsafe { std::slice::from_raw_parts(table.array, table.n as usize) };
            for entry in entries {
                assert!(!unsafe { std::ffi::CStr::from_ptr(entry.name) }
                    .to_bytes()
                    .is_empty());
                if entry.next_state == CommandParserState::Call {
                    calls.insert(unsafe { entry.extra.call_identifier });
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
