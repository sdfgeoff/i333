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
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;

pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
pub use crate::config::IncludedFile;

pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_20 = 3;
pub const PDF_IGNORE: C2RustUnnamed_20 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_20 = 1;
pub const PDF_SMART: C2RustUnnamed_20 = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_21 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_21 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_22 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_22 = 2;
pub const FOWA_URGENT: C2RustUnnamed_22 = 1;
pub const FOWA_SMART: C2RustUnnamed_22 = 0;
pub use crate::bindings::context;
pub use crate::config::Variable;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_25 = 1;
pub const STACK_STR: C2RustUnnamed_25 = 0;
pub use crate::commands::stack;
pub use crate::config::parser_ctx;
pub use crate::config::variables_head;
pub use crate::config_directives::ConfigResultIR;
pub type parse_file_result_t = ::core::ffi::c_int;
pub const PARSE_FILE_CONFIG_ERRORS: parse_file_result_t = 1;
pub const PARSE_FILE_SUCCESS: parse_file_result_t = 0;
pub const PARSE_FILE_FAILED: parse_file_result_t = -1;
pub type cmdp_token = token;
pub use crate::commands_parser::token;
pub use crate::commands_parser::C2RustUnnamed_20 as C2RustUnnamed_26;
pub type cmdp_state = ::core::ffi::c_uint;
pub const __CALL: cmdp_state = 117;
pub const BAR: cmdp_state = 116;
pub const MODE: cmdp_state = 115;
pub const GAPS: cmdp_state = 114;
pub const FONT: cmdp_state = 113;
pub const EXEC: cmdp_state = 112;
pub const BAR_ID: cmdp_state = 111;
pub const ASSIGN: cmdp_state = 110;
pub const INITIAL: cmdp_state = 109;
pub const INCLUDE: cmdp_state = 108;
pub const BINDING: cmdp_state = 107;
pub const NO_FOCUS: cmdp_state = 106;
pub const MODENAME: cmdp_state = 105;
pub const GAPS_END: cmdp_state = 104;
pub const CRITERIA: cmdp_state = 103;
pub const BAR_MODE: cmdp_state = 102;
pub const BAR_FONT: cmdp_state = 101;
pub const BARBRACE: cmdp_state = 100;
pub const WORKSPACE: cmdp_state = 99;
pub const MODEBRACE: cmdp_state = 98;
pub const CRITERION: cmdp_state = 97;
pub const SMART_GAPS: cmdp_state = 96;
pub const SHOW_MARKS: cmdp_state = 95;
pub const IPC_SOCKET: cmdp_state = 94;
pub const FOR_WINDOW: cmdp_state = 93;
pub const COLOR_TEXT: cmdp_state = 92;
pub const BAR_OUTPUT: cmdp_state = 91;
pub const BAR_HEIGHT: cmdp_state = 90;
pub const BAR_COLORS: cmdp_state = 89;
pub const TITLE_ALIGN: cmdp_state = 88;
pub const TILING_DRAG: cmdp_state = 87;
pub const IGNORE_LINE: cmdp_state = 86;
pub const BINDCOMMAND: cmdp_state = 85;
pub const BAR_VERBOSE: cmdp_state = 84;
pub const BAR_PADDING: cmdp_state = 83;
pub const BAR_BINDSYM: cmdp_state = 82;
pub const NO_FOCUS_END: cmdp_state = 81;
pub const MODE_BINDING: cmdp_state = 80;
pub const FAKE_OUTPUTS: cmdp_state = 79;
pub const COLOR_SINGLE: cmdp_state = 78;
pub const COLOR_BORDER: cmdp_state = 77;
pub const BAR_POSITION: cmdp_state = 76;
pub const BAR_MODIFIER: cmdp_state = 75;
pub const SMART_BORDERS: cmdp_state = 74;
pub const RESTART_STATE: cmdp_state = 73;
pub const MOUSE_WARPING: cmdp_state = 72;
pub const CRITERION_STR: cmdp_state = 71;
pub const ASSIGN_OUTPUT: cmdp_state = 70;
pub const FORCE_XINERAMA: cmdp_state = 69;
pub const FOCUS_WRAPPING: cmdp_state = 68;
pub const DEFAULT_BORDER: cmdp_state = 67;
pub const CRITERION_FROM: cmdp_state = 66;
pub const GAPS_WITH_SCOPE: cmdp_state = 65;
pub const DISABLE_RANDR15: cmdp_state = 64;
pub const COLOR_INDICATOR: cmdp_state = 63;
pub const BAR_TRAY_OUTPUT: cmdp_state = 62;
pub const BAR_SOCKET_PATH: cmdp_state = 61;
pub const BAR_PADDING_TOP: cmdp_state = 60;
pub const BAR_IGNORE_LINE: cmdp_state = 59;
pub const BAR_COLORS_TEXT: cmdp_state = 58;
pub const BAR_BAR_COMMAND: cmdp_state = 57;
pub const WORKSPACE_LAYOUT: cmdp_state = 56;
pub const TILING_DRAG_MODE: cmdp_state = 55;
pub const MODE_IGNORE_LINE: cmdp_state = 54;
pub const MODE_BINDCOMMAND: cmdp_state = 53;
pub const IPC_KILL_TIMEOUT: cmdp_state = 52;
pub const COLOR_BACKGROUND: cmdp_state = 51;
pub const BAR_WHEEL_UP_CMD: cmdp_state = 50;
pub const BAR_TRAY_PADDING: cmdp_state = 49;
pub const BAR_PADDING_LEFT: cmdp_state = 48;
pub const BAR_HIDDEN_STATE: cmdp_state = 47;
pub const BAR_COLORS_BRACE: cmdp_state = 46;
pub const ASSIGN_WORKSPACE: cmdp_state = 45;
pub const WORKSPACE_COMMAND: cmdp_state = 44;
pub const HIDE_EDGE_BORDERS: cmdp_state = 43;
pub const FLOATING_MODIFIER: cmdp_state = 42;
pub const BAR_PADDING_RIGHT: cmdp_state = 41;
pub const BAR_COLORS_SINGLE: cmdp_state = 40;
pub const BAR_COLORS_BORDER: cmdp_state = 39;
pub const FOR_WINDOW_COMMAND: cmdp_state = 38;
pub const CRITERION_FROM_STR: cmdp_state = 37;
pub const COLOR_CHILD_BORDER: cmdp_state = 36;
pub const BAR_WHEEL_DOWN_CMD: cmdp_state = 35;
pub const BAR_STATUS_COMMAND: cmdp_state = 34;
pub const BAR_PADDING_BOTTOM: cmdp_state = 33;
pub const FOCUS_FOLLOWS_MOUSE: cmdp_state = 32;
pub const DEFAULT_ORIENTATION: cmdp_state = 31;
pub const BAR_TRAY_PADDING_PX: cmdp_state = 30;
pub const BAR_BINDSYM_COMMAND: cmdp_state = 29;
pub const FORCE_FOCUS_WRAPPING: cmdp_state = 28;
pub const BAR_SEPARATOR_SYMBOL: cmdp_state = 27;
pub const WORKSPACE_OUTPUT_WORD: cmdp_state = 26;
pub const DEFAULT_BORDER_PIXELS: cmdp_state = 25;
pub const BAR_WORKSPACE_COMMAND: cmdp_state = 24;
pub const BAR_WORKSPACE_BUTTONS: cmdp_state = 23;
pub const BAR_COLORS_BACKGROUND: cmdp_state = 22;
pub const CRITERION_FROM_STR_END: cmdp_state = 21;
pub const BAR_COLORS_IGNORE_LINE: cmdp_state = 20;
pub const POPUP_DURING_FULLSCREEN: cmdp_state = 19;
pub const FLOATING_MINIMUM_SIZE_X: cmdp_state = 18;
pub const FLOATING_MAXIMUM_SIZE_X: cmdp_state = 17;
pub const BAR_WORKSPACE_MIN_WIDTH: cmdp_state = 16;
pub const ASSIGN_WORKSPACE_NUMBER: cmdp_state = 15;
pub const WORKSPACE_BACK_AND_FORTH: cmdp_state = 14;
pub const DEFAULT_BORDER_PIXELS_PX: cmdp_state = 13;
pub const CRITERION_FROM_STR_START: cmdp_state = 12;
pub const BAR_STRIP_WORKSPACE_NAME: cmdp_state = 11;
pub const TILING_DRAG_SWAP_MODIFIER: cmdp_state = 10;
pub const FORCE_DISPLAY_URGENCY_HINT: cmdp_state = 9;
pub const FOCUS_ON_WINDOW_ACTIVATION: cmdp_state = 8;
pub const BAR_WORKSPACE_MIN_WIDTH_PX: cmdp_state = 7;
pub const BAR_BINDING_MODE_INDICATOR: cmdp_state = 6;
pub const FLOATING_MINIMUM_SIZE_WIDTH: cmdp_state = 5;
pub const FLOATING_MAXIMUM_SIZE_WIDTH: cmdp_state = 4;
pub const BAR_STRIP_WORKSPACE_NUMBERS: cmdp_state = 3;
pub const FLOATING_MINIMUM_SIZE_HEIGHT: cmdp_state = 2;
pub const FLOATING_MAXIMUM_SIZE_HEIGHT: cmdp_state = 1;
pub const FORCE_DISPLAY_URGENCY_HINT_MS: cmdp_state = 0;
pub type cmdp_token_ptr = tokenptr;
pub use crate::commands_parser::tokenptr;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub static mut database: *mut xcb_xrm_database_t =
    ::core::ptr::null::<xcb_xrm_database_t>() as *mut xcb_xrm_database_t;

pub static mut config_error_nagbar_pid: pid_t = -(1 as pid_t);
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: c"'ms".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 1 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 2 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 3 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: FLOATING_MAXIMUM_SIZE_X,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: FLOATING_MINIMUM_SIZE_X,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 4 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 5 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 6 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"duration_ms".as_ptr().cast_mut(),
    next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 7 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 8 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 9 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 10 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 11 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 12 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 13 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'ignore".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 14 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'leave_fullscreen".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 15 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 16 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 17 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 18 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: BAR_COLORS_TEXT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 19 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 20 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 21 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 22 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"separator".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 23 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 24 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 25 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING_PX,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 26 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 27 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 28 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 29 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 30 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"left".as_ptr().cast_mut(),
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 31 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 32 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 33 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"child_border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 34 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 35 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 36 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: BAR_COLORS_BACKGROUND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 37 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"bottom".as_ptr().cast_mut(),
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 38 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 39 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 40 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 41 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 42 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'both".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 43 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_no_gaps".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 44 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 45 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 46 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 47 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 48 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 49 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 50 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 51 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_OUTPUT_WORD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: c"'→".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE_NUMBER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 52 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 53 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 54 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 55 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"padding_px".as_ptr().cast_mut(),
    next_state: BAR_TRAY_PADDING_PX,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 56 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: COLOR_TEXT,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"timeout".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 57 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 58 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MODE,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 59 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 60 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 61 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 62 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 63 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 64 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 65 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"text".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 66 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_TOP,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"right_or_right_and_left".as_ptr().cast_mut(),
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 67 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 68 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 69 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"indicator".as_ptr().cast_mut(),
        next_state: COLOR_CHILD_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 70 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 71 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: GAPS_END,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_FROM_STR_START,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 72 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 73 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 74 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 75 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 76 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 77 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 78 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 79 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 80 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 81 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 82 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 83 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 84 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 85 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 86 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 87 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 88 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 89 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 90 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 91 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 92 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 93 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 94 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 95 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 96 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 97 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 98 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 99 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no_gaps".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 100 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 101 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 102 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 103 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 104 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 105 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: COLOR_BACKGROUND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 106 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"outputs".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 107 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: c"end".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 108 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"button".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"top_or_all".as_ptr().cast_mut(),
    next_state: BAR_PADDING_TOP,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 109 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 110 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: INITIAL,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 111 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 112 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 113 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 114 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 115 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 116 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'swap_modifier".as_ptr().cast_mut(),
        identifier: c"swap_modifier".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 117 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 118 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 119 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'urgent_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'binding_mode".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 120 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 121 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"text".as_ptr().cast_mut(),
    next_state: COLOR_INDICATOR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 122 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 123 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 124 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 125 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 126 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 127 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 128 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 129 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 130 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 131 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 132 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 133 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 134 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 135 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 136 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inverse_outer".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 137 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_STR,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODEBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: WORKSPACE_COMMAND,
    extra: C2RustUnnamed_26 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 138 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 139 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 140 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 141 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 142 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 143 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 144 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 145 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 146 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS_END,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 147 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: c"'--pango_markup".as_ptr().cast_mut(),
        identifier: c"pango_markup".as_ptr().cast_mut(),
        next_state: MODENAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"modename".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 148 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 149 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"pattern".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 150 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set\t".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set_from_resource".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'include".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INCLUDE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FONT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_BORDERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_GAPS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_minimum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MINIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_maximum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MAXIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_orientation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_ORIENTATION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_LAYOUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'new_window".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_floating_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'new_float".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hide_edge_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: HIDE_EDGE_BORDERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'for_window".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOR_WINDOW,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'assign".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no_focus".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: NO_FOCUS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_follows_mouse".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_FOLLOWS_MOUSE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mouse_warping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOUSE_WARPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_WRAPPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_FOCUS_WRAPPING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force-xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable_randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable-randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_auto_back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_BACK_AND_FORTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'fake_outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'fake-outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_display_urgency_hint".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_on_window_activation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_ON_WINDOW_ACTIVATION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'title_align".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_ALIGN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'show_marks".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SHOW_MARKS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc_socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc-socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc_kill_timeout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_KILL_TIMEOUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'restart_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESTART_STATE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'popup_during_fullscreen".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: POPUP_DURING_FULLSCREEN,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling_drag".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'exec_always".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_SINGLE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused_inactive".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused_tab_title".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.unfocused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.urgent".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.placeholder".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 151 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"bar_id".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 152 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"no_startup_id".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 153 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_26 {
        call_identifier: 154 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'i3bar_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BAR_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'status_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STATUS_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_COMMAND,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'socket_path".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SOCKET_PATH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_ID,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'wheel_up_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_UP_CMD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'wheel_down_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_DOWN_CMD,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_POSITION,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tray_output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_OUTPUT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tray_padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_FONT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'separator_symbol".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SEPARATOR_SYMBOL,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'binding_mode_indicator".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDING_MODE_INDICATOR,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_buttons".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_BUTTONS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_min_width".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'strip_workspace_numbers".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NUMBERS,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'strip_workspace_name".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NAME,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'verbose".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_VERBOSE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HEIGHT,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'colors".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_26 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_26 {
            call_identifier: 155 as ::core::ffi::c_int as uint16_t,
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
unsafe fn GENERATED_call(
    current_match: *mut Match,
    stack: *mut stack,
    call_identifier: ::core::ffi::c_int,
    result: *mut ConfigResultIR,
) {
    match call_identifier {
        0 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_display_urgency_hint(
                current_match,
                result,
                parser_get_long(stack, c"duration_ms".as_ptr()),
            );
        }
        1 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_maximum_size(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_long(stack, c"height".as_ptr()),
            );
        }
        2 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_minimum_size(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
                parser_get_long(stack, c"height".as_ptr()),
            );
        }
        3 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_strip_workspace_numbers(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        4 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_binding_mode_indicator(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        5 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_min_width(
                current_match,
                result,
                parser_get_long(stack, c"width".as_ptr()),
            );
        }
        6 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_on_window_activation(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        7 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag_swap_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        8 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_strip_workspace_name(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        9 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        10 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        11 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                parser_get_long(stack, c"width".as_ptr()),
            );
        }
        12 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_back_and_forth(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        13 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign(
                current_match,
                result,
                parser_get_string(stack, c"number".as_ptr()),
                1 as ::core::ffi::c_int != 0,
            );
        }
        14 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        15 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        16 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        17 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_popup_during_fullscreen(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        18 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"kind".as_ptr()),
            );
        }
        19 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_buttons(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        20 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_workspace_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        21 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                2 as ::core::ffi::c_long,
            );
        }
        22 => {
            (*result).next_state = WORKSPACE_OUTPUT_WORD as ::core::ffi::c_int;
            cfg_workspace(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        23 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_separator_symbol(
                current_match,
                result,
                parser_get_string(stack, c"separator".as_ptr()),
            );
        }
        24 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        25 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_bindsym(
                current_match,
                result,
                parser_get_string(stack, c"button".as_ptr()),
                parser_get_string(stack, c"release".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        26 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_tray_padding(
                current_match,
                result,
                parser_get_long(stack, c"padding_px".as_ptr()),
            );
        }
        27 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        28 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        29 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_orientation(
                current_match,
                result,
                parser_get_string(stack, c"orientation".as_ptr()),
            );
        }
        30 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_follows_mouse(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        31 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_three(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
                parser_get_long(stack, c"right_or_right_and_left".as_ptr()),
                parser_get_long(stack, c"bottom".as_ptr()),
            );
        }
        32 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_status_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        33 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_wheel_down_cmd(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        34 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
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
            (*result).next_state = INITIAL as ::core::ffi::c_int;
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
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_for_window(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        37 => {
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
            cfg_bar_color_single(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"color".as_ptr()),
            );
        }
        38 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_two(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
                parser_get_long(stack, c"right_or_right_and_left".as_ptr()),
            );
        }
        39 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_floating_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        40 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        41 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        42 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        43 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        44 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        45 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        46 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        47 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        48 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        49 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        50 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        51 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_hide_edge_borders(
                current_match,
                result,
                parser_get_string(stack, c"hide_borders".as_ptr()),
            );
        }
        52 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
                0 as ::core::ffi::c_int != 0,
            );
        }
        53 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(stack, c"hidden_state".as_ptr()),
            );
        }
        54 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_hidden_state(
                current_match,
                result,
                parser_get_string(stack, c"hidden_state".as_ptr()),
            );
        }
        55 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
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
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_wheel_up_cmd(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        57 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_ipc_kill_timeout(
                current_match,
                result,
                parser_get_long(stack, c"timeout".as_ptr()),
            );
        }
        58 => {
            (*result).next_state = MODE as ::core::ffi::c_int;
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
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        60 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        61 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        62 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        63 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_workspace_layout(
                current_match,
                result,
                parser_get_string(stack, c"layout".as_ptr()),
            );
        }
        64 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_i3bar_command(
                current_match,
                result,
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        65 => {
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
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
            (*result).next_state = BAR_COLORS as ::core::ffi::c_int;
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
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_padding_one(
                current_match,
                result,
                parser_get_long(stack, c"top_or_all".as_ptr()),
            );
        }
        68 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_socket_path(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        69 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_tray_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        70 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
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
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_disable_randr15(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        72 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        73 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_default_border(
                current_match,
                result,
                parser_get_string(stack, c"windowtype".as_ptr()),
                parser_get_string(stack, c"border".as_ptr()),
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
            );
        }
        74 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        75 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        76 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        77 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        78 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        79 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        80 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        81 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        82 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        83 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        84 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        85 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        86 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        87 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_focus_wrapping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        88 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_force_xinerama(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        89 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_assign_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        90 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                parser_get_string(stack, c"cvalue".as_ptr()),
            );
        }
        91 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        92 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_mouse_warping(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        93 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_restart_state(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        94 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        95 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        96 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        97 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        98 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        99 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        100 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_borders(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        101 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        102 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        103 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_modifier(
                current_match,
                result,
                parser_get_string(stack, c"modifiers".as_ptr()),
            );
        }
        104 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(stack, c"position".as_ptr()),
            );
        }
        105 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_position(
                current_match,
                result,
                parser_get_string(stack, c"position".as_ptr()),
            );
        }
        106 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_color_single(
                current_match,
                result,
                parser_get_string(stack, c"colorclass".as_ptr()),
                parser_get_string(stack, c"color".as_ptr()),
            );
        }
        107 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_fake_outputs(
                current_match,
                result,
                parser_get_string(stack, c"outputs".as_ptr()),
            );
        }
        108 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_no_focus(current_match, result);
        }
        109 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_verbose(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        110 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
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
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        112 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        113 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        114 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        115 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        116 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_tiling_drag(
                current_match,
                result,
                parser_get_string(stack, c"off".as_ptr()),
            );
        }
        117 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        118 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        119 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_title_align(
                current_match,
                result,
                parser_get_string(stack, c"alignment".as_ptr()),
            );
        }
        120 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_height(
                current_match,
                result,
                parser_get_long(stack, c"value".as_ptr()),
            );
        }
        121 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_output(
                current_match,
                result,
                parser_get_string(stack, c"output".as_ptr()),
            );
        }
        122 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 38 as ::core::ffi::c_int);
        }
        123 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_ipc_socket(
                current_match,
                result,
                parser_get_string(stack, c"path".as_ptr()),
            );
        }
        124 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_show_marks(
                current_match,
                result,
                parser_get_string(stack, c"value".as_ptr()),
            );
        }
        125 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        126 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        127 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        128 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        129 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        130 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        131 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        132 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        133 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        134 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        135 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        136 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        137 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_smart_gaps(
                current_match,
                result,
                parser_get_string(stack, c"enabled".as_ptr()),
            );
        }
        138 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_start(current_match, result);
        }
        139 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_font(
                current_match,
                result,
                parser_get_string(stack, c"font".as_ptr()),
            );
        }
        140 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        141 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        142 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_mode(
                current_match,
                result,
                parser_get_string(stack, c"mode".as_ptr()),
            );
        }
        143 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        144 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        145 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_add(
                current_match,
                result,
                parser_get_string(stack, c"ctype".as_ptr()),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        146 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_criteria_pop_state(current_match, result);
        }
        147 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_gaps(
                current_match,
                result,
                parser_get_string(stack, c"workspace".as_ptr()),
                parser_get_string(stack, c"scope".as_ptr()),
                parser_get_long(stack, c"value".as_ptr()),
            );
        }
        148 => {
            (*result).next_state = MODEBRACE as ::core::ffi::c_int;
            cfg_enter_mode(
                current_match,
                result,
                parser_get_string(stack, c"pango_markup".as_ptr()),
                parser_get_string(stack, c"modename".as_ptr()),
            );
        }
        149 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 81 as ::core::ffi::c_int);
        }
        150 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_include(
                current_match,
                result,
                parser_get_string(stack, c"pattern".as_ptr()),
            );
        }
        151 => {
            (*result).next_state = CRITERIA as ::core::ffi::c_int;
            cfg_criteria_init(current_match, result, 45 as ::core::ffi::c_int);
        }
        152 => {
            (*result).next_state = BAR as ::core::ffi::c_int;
            cfg_bar_id(
                current_match,
                result,
                parser_get_string(stack, c"bar_id".as_ptr()),
            );
        }
        153 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_exec(
                current_match,
                result,
                parser_get_string(stack, c"exectype".as_ptr()),
                parser_get_string(stack, c"no_startup_id".as_ptr()),
                parser_get_string(stack, c"command".as_ptr()),
            );
        }
        154 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
            cfg_font(
                current_match,
                result,
                parser_get_string(stack, c"font".as_ptr()),
            );
        }
        155 => {
            (*result).next_state = INITIAL as ::core::ffi::c_int;
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
unsafe fn next_state(token: *const cmdp_token, ctx: *mut parser_ctx) {
    let mut _next_state: cmdp_state = (*token).next_state;
    if (*token).next_state as ::core::ffi::c_uint
        == __CALL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut subcommand_output: ConfigResultIR = ConfigResultIR {
            ctx,
            next_state: 0,
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
        _next_state = subcommand_output.next_state as cmdp_state;
        parser_clear_stack((*ctx).stack);
    }
    (*ctx).state = _next_state as ::core::ffi::c_int;
    if (*ctx).state == INITIAL as ::core::ffi::c_int {
        parser_clear_stack((*ctx).stack);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).statelist_idx {
        if (*ctx).statelist[i as usize] as cmdp_state as ::core::ffi::c_uint
            != _next_state as ::core::ffi::c_uint
        {
            i += 1;
        } else {
            (*ctx).statelist_idx = i + 1 as ::core::ffi::c_int;
            return;
        }
    }
    let fresh7 = (*ctx).statelist_idx;
    (*ctx).statelist_idx = (*ctx).statelist_idx + 1;
    (*ctx).statelist[fresh7 as usize] = _next_state as ::core::ffi::c_int;
}
unsafe fn start_of_line(
    mut walk: *const ::core::ffi::c_char,
    beginning: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    while walk >= beginning
        && *walk as ::core::ffi::c_int != '\n' as i32
        && *walk as ::core::ffi::c_int != '\r' as i32
    {
        walk = walk.offset(-1);
    }
    walk.offset(1 as ::core::ffi::c_int as isize)
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
            debuglog(
                c"%s:%s:%d - CONFIG(line %3d): %.*s\n".as_ptr().cast_mut(),
                c"../src/config_parser.c".as_ptr(),
                c"parse_config".as_ptr(),
                151 as ::core::ffi::c_int,
                linecnt,
                next_nl.offset_from(dumpwalk) as ::core::ffi::c_long as ::core::ffi::c_int,
                dumpwalk,
            );
            dumpwalk = next_nl.offset(1 as ::core::ffi::c_int as isize);
            linecnt += 1;
        } else {
            debuglog(
                c"%s:%s:%d - CONFIG(line %3d): %s\n".as_ptr().cast_mut(),
                c"../src/config_parser.c".as_ptr(),
                c"parse_config".as_ptr(),
                154 as ::core::ffi::c_int,
                linecnt,
                dumpwalk,
            );
            break;
        }
    }
    (*ctx).state = INITIAL as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        (*ctx).statelist[i as usize] = INITIAL as ::core::ffi::c_int;
        i += 1;
    }
    (*ctx).statelist_idx = 1 as ::core::ffi::c_int;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: size_t = strlen(input) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut token_handled: bool = false;
    linecnt = 1 as ::core::ffi::c_int;
    let mut subcommand_output: ConfigResultIR = ConfigResultIR {
        ctx,
        next_state: 0,
        has_errors: false,
    };
    cfg_criteria_init(
        &raw mut (*ctx).current_match,
        &raw mut subcommand_output,
        INITIAL as ::core::ffi::c_int,
    );
    while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let ptr: *const cmdp_token_ptr = (&raw mut tokens as *mut cmdp_token_ptr)
            .offset((*ctx).state as isize)
            as *mut cmdp_token_ptr;
        token_handled = false_0 != 0;
        c = 0 as ::core::ffi::c_int;
        while c < (*ptr).n {
            token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
            if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
            {
                if strncasecmp(
                    walk,
                    (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    strlen((*token).name).wrapping_sub(1 as size_t),
                ) == 0 as ::core::ffi::c_int
                {
                    if !(*token).identifier.is_null() {
                        parser_push_string(
                            (*ctx).stack,
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as size_t) as isize);
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
                {
                    if !(end == walk as *mut ::core::ffi::c_char) {
                        if !(*token).identifier.is_null() {
                            parser_push_long((*ctx).stack, (*token).identifier, num);
                        }
                        walk = end;
                        next_state(token, ctx);
                        token_handled = true_0 != 0;
                        break;
                    }
                }
            } else {
                if strcmp((*token).name, c"string".as_ptr()) == 0 as ::core::ffi::c_int
                    || strcmp((*token).name, c"word".as_ptr()) == 0 as ::core::ffi::c_int
                {
                    let mut beginning: *const ::core::ffi::c_char = walk;
                    if *walk as ::core::ffi::c_int == '"' as i32 {
                        beginning = beginning.offset(1);
                        walk = walk.offset(1);
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && (*walk as ::core::ffi::c_int != '"' as i32
                                || *walk.offset(-(1 as ::core::ffi::c_int as isize))
                                    as ::core::ffi::c_int
                                    == '\\' as i32)
                        {
                            walk = walk.offset(1);
                        }
                    } else if *(*token).name.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 's' as i32
                    {
                        while *walk as ::core::ffi::c_int != '\0' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                        {
                            walk = walk.offset(1);
                        }
                    } else {
                        while *walk as ::core::ffi::c_int != ' ' as i32
                            && *walk as ::core::ffi::c_int != '\t' as i32
                            && *walk as ::core::ffi::c_int != ']' as i32
                            && *walk as ::core::ffi::c_int != ',' as i32
                            && *walk as ::core::ffi::c_int != ';' as i32
                            && *walk as ::core::ffi::c_int != '\r' as i32
                            && *walk as ::core::ffi::c_int != '\n' as i32
                            && *walk as ::core::ffi::c_int != '\0' as i32
                        {
                            walk = walk.offset(1);
                        }
                    }
                    if walk != beginning {
                        let str: *mut ::core::ffi::c_char = scalloc(
                            (walk.offset_from(beginning) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as size_t,
                            1 as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        let mut inpos: ::core::ffi::c_int = 0;
                        let mut outpos: ::core::ffi::c_int = 0;
                        inpos = 0 as ::core::ffi::c_int;
                        outpos = 0 as ::core::ffi::c_int;
                        while (inpos as ::core::ffi::c_long)
                            < walk.offset_from(beginning) as ::core::ffi::c_long
                        {
                            if *beginning.offset(inpos as isize) as ::core::ffi::c_int
                                == '\\' as i32
                                && *beginning.offset((inpos + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '"' as i32
                            {
                                inpos += 1;
                            }
                            *str.offset(outpos as isize) = *beginning.offset(inpos as isize);
                            inpos += 1;
                            outpos += 1;
                        }
                        if !(*token).identifier.is_null() {
                            parser_push_string((*ctx).stack, (*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        next_state(token, ctx);
                        token_handled = true_0 != 0;
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
                } else if strcmp((*token).name, c"end".as_ptr()) == 0 as ::core::ffi::c_int {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == '\n' as i32
                        || *walk as ::core::ffi::c_int == '\r' as i32
                    {
                        next_state(token, ctx);
                        token_handled = true_0 != 0;
                        cfg_criteria_init(
                            &raw mut (*ctx).current_match,
                            &raw mut subcommand_output,
                            INITIAL as ::core::ffi::c_int,
                        );
                        linecnt += 1;
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
        if !token_handled {
            let mut tokenlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            c = 0 as ::core::ffi::c_int;
            while c < (*ptr).n {
                tokenlen = (tokenlen as size_t).wrapping_add(
                    strlen((*(*ptr).array.offset(c as isize)).name)
                        .wrapping_add(strlen(c"'', ".as_ptr())),
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
                c += 1;
            }
            let mut errormessage: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let possible_tokens: *mut ::core::ffi::c_char =
                smalloc((tokenlen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
            let mut tokenwalk: *mut ::core::ffi::c_char = possible_tokens;
            let mut current_block_96: u64;
            c = 0 as ::core::ffi::c_int;
            while c < (*ptr).n {
                token = (*ptr).array.offset(c as isize) as *mut cmdp_token;
                if *(*token).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    let fresh1 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh1 = '\'' as i32 as ::core::ffi::c_char;
                    strcpy(
                        tokenwalk,
                        (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    );
                    tokenwalk = tokenwalk.offset(strlen(
                        (*token).name.offset(1 as ::core::ffi::c_int as isize),
                    ) as isize);
                    let fresh2 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh2 = '\'' as i32 as ::core::ffi::c_char;
                    current_block_96 = 796174441944384681;
                } else if strcmp((*token).name, c"error".as_ptr()) == 0 as ::core::ffi::c_int {
                    current_block_96 = 14541395414537699361;
                } else {
                    let fresh3 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh3 = '<' as i32 as ::core::ffi::c_char;
                    strcpy(tokenwalk, (*token).name);
                    tokenwalk = tokenwalk.offset(strlen((*token).name) as isize);
                    let fresh4 = tokenwalk;
                    tokenwalk = tokenwalk.offset(1);
                    *fresh4 = '>' as i32 as ::core::ffi::c_char;
                    current_block_96 = 796174441944384681;
                }
                match current_block_96 {
                    796174441944384681 => {
                        if c < (*ptr).n - 1 as ::core::ffi::c_int {
                            let fresh5 = tokenwalk;
                            tokenwalk = tokenwalk.offset(1);
                            *fresh5 = ',' as i32 as ::core::ffi::c_char;
                            let fresh6 = tokenwalk;
                            tokenwalk = tokenwalk.offset(1);
                            *fresh6 = ' ' as i32 as ::core::ffi::c_char;
                        }
                    }
                    _ => {}
                }
                c += 1;
            }
            *tokenwalk = '\0' as i32 as ::core::ffi::c_char;
            sasprintf(
                &raw mut errormessage,
                c"Expected one of these tokens: %s".as_ptr(),
                possible_tokens,
            );
            free(possible_tokens as *mut ::core::ffi::c_void);
            let mut error_line: *const ::core::ffi::c_char = start_of_line(walk, input);
            let position: *mut ::core::ffi::c_char =
                scalloc(strlen(error_line).wrapping_add(1 as size_t), 1 as size_t)
                    as *mut ::core::ffi::c_char;
            let mut copywalk: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            copywalk = error_line;
            while *copywalk as ::core::ffi::c_int != '\n' as i32
                && *copywalk as ::core::ffi::c_int != '\r' as i32
                && *copywalk as ::core::ffi::c_int != '\0' as i32
            {
                *position
                    .offset(copywalk.offset_from(error_line) as ::core::ffi::c_long as isize) =
                    (if copywalk >= walk {
                        '^' as i32
                    } else if *copywalk as ::core::ffi::c_int == '\t' as i32 {
                        '\t' as i32
                    } else {
                        ' ' as i32
                    }) as ::core::ffi::c_char;
                copywalk = copywalk.offset(1);
            }
            *position.offset(copywalk.offset_from(error_line) as ::core::ffi::c_long as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            errorlog(c"ERROR: CONFIG: %s\n".as_ptr().cast_mut(), errormessage);
            errorlog(
                c"ERROR: CONFIG: (in file %s)\n".as_ptr().cast_mut(),
                (*context).filename,
            );
            let mut error_copy: *mut ::core::ffi::c_char = single_line(error_line);
            if linecnt > 1 as ::core::ffi::c_int {
                let context_p1_start: *const ::core::ffi::c_char = start_of_line(
                    error_line.offset(-(2 as ::core::ffi::c_int as isize)),
                    input,
                );
                let context_p1_line: *mut ::core::ffi::c_char = single_line(context_p1_start);
                if linecnt > 2 as ::core::ffi::c_int {
                    let context_p2_start: *const ::core::ffi::c_char = start_of_line(
                        context_p1_start.offset(-(2 as ::core::ffi::c_int as isize)),
                        input,
                    );
                    let context_p2_line: *mut ::core::ffi::c_char = single_line(context_p2_start);
                    errorlog(
                        c"ERROR: CONFIG: Line %3d: %s\n".as_ptr().cast_mut(),
                        linecnt - 2 as ::core::ffi::c_int,
                        context_p2_line,
                    );
                    free(context_p2_line as *mut ::core::ffi::c_void);
                }
                errorlog(
                    c"ERROR: CONFIG: Line %3d: %s\n".as_ptr().cast_mut(),
                    linecnt - 1 as ::core::ffi::c_int,
                    context_p1_line,
                );
                free(context_p1_line as *mut ::core::ffi::c_void);
            }
            errorlog(
                c"ERROR: CONFIG: Line %3d: %s\n".as_ptr().cast_mut(),
                linecnt,
                error_copy,
            );
            errorlog(
                c"ERROR: CONFIG:           %s\n".as_ptr().cast_mut(),
                position,
            );
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
                    errorlog(
                        c"ERROR: CONFIG: Line %3d: %s\n".as_ptr().cast_mut(),
                        linecnt + i_0 + 1 as ::core::ffi::c_int,
                        error_copy,
                    );
                    free(error_copy as *mut ::core::ffi::c_void);
                }
                i_0 += 1;
            }
            (*context).has_errors = true_0 != 0;
            while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len
                && *walk as ::core::ffi::c_int != '\n' as i32
            {
                walk = walk.offset(1);
            }
            free(position as *mut ::core::ffi::c_void);
            free(errormessage as *mut ::core::ffi::c_void);
            parser_clear_stack((*ctx).stack);
            let mut error_token_found: bool = false_0 != 0;
            let mut i_1: ::core::ffi::c_int = (*ctx).statelist_idx - 1 as ::core::ffi::c_int;
            while i_1 >= 0 as ::core::ffi::c_int && !error_token_found {
                let errptr: *const cmdp_token_ptr = (&raw mut tokens as *mut cmdp_token_ptr).offset(
                    *(&raw mut (*ctx).statelist as *mut ::core::ffi::c_int).offset(i_1 as isize)
                        as isize,
                ) as *mut cmdp_token_ptr;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < (*errptr).n {
                    if strcmp(
                        (*(*errptr).array.offset(j as isize)).name,
                        c"error".as_ptr(),
                    ) != 0 as ::core::ffi::c_int
                    {
                        j += 1;
                    } else {
                        next_state((*errptr).array.offset(j as isize) as *mut cmdp_token, ctx);
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
        (if !config.font.pattern.is_null() {
            config.font.pattern as *const ::core::ffi::c_char
        } else {
            c"fixed".as_ptr()
        }) as *mut ::core::ffi::c_char,
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
unsafe fn upsert_variable(
    variables: *mut variables_head,
    key: *mut ::core::ffi::c_char,
    value: *mut ::core::ffi::c_char,
) {
    let mut current: *mut Variable = ::core::ptr::null_mut::<Variable>();
    current = (*variables).slh_first;
    while !current.is_null() {
        if strcmp((*current).key, key) != 0 as ::core::ffi::c_int {
            current = (*current).variables.sle_next;
        } else {
            debuglog(
                c"%s:%s:%d - Updated variable: %s = %s -> %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/config_parser.c".as_ptr(),
                c"upsert_variable".as_ptr(),
                543 as ::core::ffi::c_int,
                key,
                (*current).value,
                value,
            );
            free((*current).value as *mut ::core::ffi::c_void);
            (*current).value = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*current).value = sstrdup(value);
            return;
        }
    }
    debuglog(
        c"%s:%s:%d - Defined new variable: %s = %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_parser.c".as_ptr(),
        c"upsert_variable".as_ptr(),
        549 as ::core::ffi::c_int,
        key,
        value,
    );
    let new: *mut Variable =
        scalloc(1 as size_t, ::core::mem::size_of::<Variable>() as size_t) as *mut Variable;
    let mut test: *mut Variable = ::core::ptr::null_mut::<Variable>();
    let mut loc: *mut Variable = ::core::ptr::null_mut::<Variable>();
    (*new).key = sstrdup(key);
    (*new).value = sstrdup(value);
    test = (*variables).slh_first;
    while !test.is_null() {
        if strlen((*new).key) >= strlen((*test).key) {
            break;
        }
        loc = test;
        test = (*test).variables.sle_next;
    }
    if loc.is_null() {
        (*new).variables.sle_next = (*variables).slh_first;
        (*variables).slh_first = new;
    } else {
        (*new).variables.sle_next = (*loc).variables.sle_next;
        (*loc).variables.sle_next = new;
    };
}
unsafe fn get_resource(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if conn.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if database.is_null() {
        database = xcb_xrm_database_from_default(conn);
        if database.is_null() {
            errorlog(
                c"ERROR: Failed to open the resource database.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            database = xcb_xrm_database_from_string(c"".as_ptr());
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    let mut resource: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    xcb_xrm_resource_get_string(
        database,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut resource,
    );
    resource
}

pub unsafe fn free_variables(ctx: *mut parser_ctx) {
    while !(*ctx).variables.slh_first.is_null() {
        let mut current: *mut Variable = (*ctx).variables.slh_first;
        free((*current).key as *mut ::core::ffi::c_void);
        (*current).key = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*current).value as *mut ::core::ffi::c_void);
        (*current).value = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*ctx).variables.slh_first = (*(*ctx).variables.slh_first).variables.sle_next;
        free(current as *mut ::core::ffi::c_void);
        current = ::core::ptr::null_mut::<Variable>();
    }
}
unsafe fn try_chdir(path: *const ::core::ffi::c_char) -> bool {
    let mut result: bool = true_0 != 0;
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dirbuf: *mut ::core::ffi::c_char = sstrdup(path);
    dir = dirname(dirbuf);
    if !dir.is_null() {
        verboselog(
            c"Changing working directory to config file directory %s\n"
                .as_ptr()
                .cast_mut(),
            dir,
        );
        if chdir(dir) == -(1 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: chdir(%s) failed: %s\n".as_ptr().cast_mut(),
                dir,
                strerror(*__errno_location()),
            );
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
) -> parse_file_result_t {
    if !try_chdir(f) {
        return PARSE_FILE_FAILED;
    }
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(f, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        return PARSE_FILE_FAILED;
    }
    let mut stbuf: stat = ::core::mem::zeroed();
    if fstat(fd, &raw mut stbuf) == -(1 as ::core::ffi::c_int) {
        close(fd);
        return PARSE_FILE_FAILED;
    }
    let mut fstr: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstr = fdopen(fd, c"r".as_ptr());
    if fstr.is_null() {
        return PARSE_FILE_FAILED;
    }
    (*included_file).raw_contents =
        scalloc((stbuf.st_size + 1 as __off_t) as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
    if fread(
        (*included_file).raw_contents as *mut ::core::ffi::c_void,
        1 as size_t,
        stbuf.st_size as size_t,
        fstr,
    ) as ssize_t
        != stbuf.st_size as ssize_t
    {
        fclose(fstr);
        return PARSE_FILE_FAILED;
    }
    rewind(fstr);
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut key: [::core::ffi::c_char; 512] = [0; 512];
    let mut value: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut continuation: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut invalid_sets: bool = false_0 != 0;
    let buf: *mut ::core::ffi::c_char =
        scalloc((stbuf.st_size + 1 as __off_t) as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
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
            fclose(fstr);
            return PARSE_FILE_FAILED;
        } else {
            if buffer[strlen(&raw mut buffer as *mut ::core::ffi::c_char).wrapping_sub(1 as size_t)
                as usize] as ::core::ffi::c_int
                != '\n' as i32
                && feof(fstr) == 0
            {
                errorlog(
                    c"ERROR: Your line continuation is too long, it exceeds %zd bytes\n"
                        .as_ptr()
                        .cast_mut(),
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                );
            }
            value[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            let skip_line: bool = sscanf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                c"%511s %4095[^\n]".as_ptr(),
                &raw mut key as *mut ::core::ffi::c_char,
                &raw mut value as *mut ::core::ffi::c_char,
            ) < 1 as ::core::ffi::c_int
                || strlen(&raw mut key as *mut ::core::ffi::c_char) < 3 as size_t;
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
                debuglog(
                    c"%s:%s:%d - line continuation in comment is ignored: \"%.*s\"\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/config_parser.c".as_ptr(),
                    c"parse_file_inner".as_ptr(),
                    682 as ::core::ffi::c_int,
                    strlen(&raw mut buffer as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                );
                continuation = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            strcpy(
                buf.offset(strlen(buf) as isize),
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
            if skip_line as ::core::ffi::c_int != 0 || comment as ::core::ffi::c_int != 0 {
                continue;
            }
            if strcasecmp(&raw mut key as *mut ::core::ffi::c_char, c"set".as_ptr())
                == 0 as ::core::ffi::c_int
                && *(&raw mut value as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    != '\0' as i32
            {
                let mut v_key: [::core::ffi::c_char; 512] = [0; 512];
                let mut v_value: [::core::ffi::c_char; 4096] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    c"%511s %4095[^\n]".as_ptr(),
                    &raw mut v_key as *mut ::core::ffi::c_char,
                    &raw mut v_value as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    errorlog(
                        c"ERROR: Failed to parse variable specification '%s', skipping it.\n"
                            .as_ptr()
                            .cast_mut(),
                        &raw mut value as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else if v_key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    errorlog(
                        c"ERROR: Malformed variable assignment, name has to start with $\n"
                            .as_ptr()
                            .cast_mut(),
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
                let mut res_name: [::core::ffi::c_char; 512] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                let mut v_key_0: [::core::ffi::c_char; 512] = [0; 512];
                let mut fallback: [::core::ffi::c_char; 4096] = [
                    '\0' as i32 as ::core::ffi::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ];
                fallback[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                if sscanf(
                    &raw mut value as *mut ::core::ffi::c_char,
                    c"%511s %511s %4095[^\n]".as_ptr(),
                    &raw mut v_key_0 as *mut ::core::ffi::c_char,
                    &raw mut res_name as *mut ::core::ffi::c_char,
                    &raw mut fallback as *mut ::core::ffi::c_char,
                ) < 1 as ::core::ffi::c_int
                {
                    errorlog(
                        c"ERROR: Failed to parse resource specification '%s', skipping it.\n"
                            .as_ptr()
                            .cast_mut(),
                        &raw mut value as *mut ::core::ffi::c_char,
                    );
                    invalid_sets = true_0 != 0;
                } else if v_key_0[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '$' as i32
                {
                    errorlog(
                        c"ERROR: Malformed variable assignment, name has to start with $\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                    invalid_sets = true_0 != 0;
                } else {
                    let mut res_value: *mut ::core::ffi::c_char =
                        get_resource(&raw mut res_name as *mut ::core::ffi::c_char);
                    if res_value.is_null() {
                        debuglog(
                            c"%s:%s:%d - Could not get resource '%s', using fallback '%s'.\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/config_parser.c".as_ptr(),
                            c"parse_file_inner".as_ptr(),
                            738 as ::core::ffi::c_int,
                            &raw mut res_name as *mut ::core::ffi::c_char,
                            &raw mut fallback as *mut ::core::ffi::c_char,
                        );
                        res_value = sstrdup(&raw mut fallback as *mut ::core::ffi::c_char);
                    }
                    upsert_variable(
                        &raw mut (*ctx).variables,
                        &raw mut v_key_0 as *mut ::core::ffi::c_char,
                        res_value,
                    );
                    free(res_value as *mut ::core::ffi::c_void);
                    res_value = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
        }
    }
    fclose(fstr);
    if !database.is_null() {
        xcb_xrm_database_free(database);
        database = ::core::ptr::null_mut::<xcb_xrm_database_t>();
    }
    let mut current: *mut Variable = ::core::ptr::null_mut::<Variable>();
    let mut extra_bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bufcopy: *mut ::core::ffi::c_char = sstrdup(buf);
    current = (*ctx).variables.slh_first;
    while !current.is_null() {
        let extra: ::core::ffi::c_int =
            strlen((*current).value).wrapping_sub(strlen((*current).key)) as ::core::ffi::c_int;
        let mut next: *mut ::core::ffi::c_char = bufcopy;
        while next < bufcopy.offset(stbuf.st_size as isize) && {
            next = strcasestr(next, (*current).key);
            !next.is_null()
        } {
            let end: *const ::core::ffi::c_char = next.offset(strlen((*current).key) as isize);
            while next < end as *mut ::core::ffi::c_char {
                let fresh0 = next;
                next = next.offset(1);
                *fresh0 = ' ' as i32 as ::core::ffi::c_char;
            }
            extra_bytes += extra;
        }
        current = (*current).variables.sle_next;
    }
    free(bufcopy as *mut ::core::ffi::c_void);
    bufcopy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut walk: *const ::core::ffi::c_char = buf;
    let new: *mut ::core::ffi::c_char = scalloc(
        (stbuf.st_size + extra_bytes as __off_t + 1 as __off_t) as size_t,
        1 as size_t,
    ) as *mut ::core::ffi::c_char;
    let mut destwalk: *mut ::core::ffi::c_char = new;
    while walk < buf.offset(stbuf.st_size as isize) as *const ::core::ffi::c_char {
        current = (*ctx).variables.slh_first;
        while !current.is_null() {
            (*current).next_match = strcasestr(walk, (*current).key);
            current = (*current).variables.sle_next;
        }
        let mut nearest: *const Variable = ::core::ptr::null::<Variable>();
        let mut distance: ::core::ffi::c_int = stbuf.st_size as ::core::ffi::c_int;
        current = (*ctx).variables.slh_first;
        while !current.is_null() {
            if !(*current).next_match.is_null() {
                if ((*current).next_match.offset_from(walk) as ::core::ffi::c_long)
                    < distance as ::core::ffi::c_long
                {
                    distance = (*current).next_match.offset_from(walk) as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    nearest = current;
                }
            }
            current = (*current).variables.sle_next;
        }
        if nearest.is_null() {
            strncpy(
                destwalk,
                walk,
                buf.offset(stbuf.st_size as isize).offset_from(walk) as ::core::ffi::c_long
                    as size_t,
            );
            destwalk = destwalk.offset(buf.offset(stbuf.st_size as isize).offset_from(walk)
                as ::core::ffi::c_long as isize);
            *destwalk = '\0' as i32 as ::core::ffi::c_char;
            break;
        } else {
            strncpy(destwalk, walk, distance as size_t);
            strcpy(destwalk.offset(distance as isize), (*nearest).value);
            walk = walk.offset((distance as size_t).wrapping_add(strlen((*nearest).key)) as isize);
            destwalk = destwalk
                .offset((distance as size_t).wrapping_add(strlen((*nearest).value)) as isize);
        }
    }
    (*included_file).variable_replaced_contents = sstrdup(new);
    let context: *mut context =
        scalloc(1 as size_t, ::core::mem::size_of::<context>() as size_t) as *mut context;
    (*context).filename = f;
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
        errorlog(
            c"ERROR: FYI: You are using i3 version %s\n"
                .as_ptr()
                .cast_mut(),
            i3_version,
        );
        start_config_error_nagbar(
            f,
            (*context).has_errors as ::core::ffi::c_int != 0
                || invalid_sets as ::core::ffi::c_int != 0,
        );
    }
    let has_errors: bool = (*context).has_errors;
    free((*context).line_copy as *mut ::core::ffi::c_void);
    (*context).line_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(context as *mut ::core::ffi::c_void);
    free(new as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
    if has_errors {
        return PARSE_FILE_CONFIG_ERRORS;
    }
    PARSE_FILE_SUCCESS
}

pub unsafe fn parse_file(
    ctx: *mut parser_ctx,
    f: *const ::core::ffi::c_char,
    included_file: *mut IncludedFile,
) -> parse_file_result_t {
    let old_dir: *mut ::core::ffi::c_char =
        getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
    let mut result: parse_file_result_t = parse_file_inner(ctx, f, included_file);
    if chdir(old_dir) == -(1 as ::core::ffi::c_int) {
        errorlog(
            c"ERROR: chdir(%s) failed: %s\n".as_ptr().cast_mut(),
            old_dir,
            strerror(*__errno_location()),
        );
        result = PARSE_FILE_FAILED;
    }
    free(old_dir as *mut ::core::ffi::c_void);
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

use xcb_sys::{
    xcb_xrm_database_free, xcb_xrm_database_from_default, xcb_xrm_database_from_string,
    xcb_xrm_database_t, xcb_xrm_resource_get_string,
};

pub use libc::{
    __errno_location, chdir, close, dirname, fclose, fdopen, feof, fgets, fread, free, fstat,
    getcwd, open, printf, rewind, sscanf, stat, strcasecmp, strcasestr, strchr, strcmp, strcpy,
    strerror, strlen, strncasecmp, strncpy, strstr, strtol, FILE,
};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Match;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::config::config;
use crate::log::errorfilename;
use crate::version::i3_version;
