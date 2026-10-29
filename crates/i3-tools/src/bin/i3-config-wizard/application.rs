use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::mkdirp::mkdirp;
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
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
use i3_common::safewrappers::{scalloc, sstrdup};

pub type __builtin_va_list = [__va_list_tag; 1];
pub use i3_common::safewrappers::__va_list_tag;
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
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = isize;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;
pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;

pub type XID = ::core::ffi::c_ulong;
pub type Window = XID;
pub type KeySym = XID;

pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const STEP_GENERATE: C2RustUnnamed_3 = 1;
pub const STEP_WELCOME: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const MOD_Mod4: C2RustUnnamed_4 = 1;
pub const MOD_Mod1: C2RustUnnamed_4 = 0;
pub type cmdp_token = token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub name: *mut ::core::ffi::c_char,
    pub identifier: *mut ::core::ffi::c_char,
    pub next_state: cmdp_state,
    pub extra: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub call_identifier: uint16_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_7,
    pub val: C2RustUnnamed_6,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_7 = 1;
pub const STACK_STR: C2RustUnnamed_7 = 0;
pub type cmdp_token_ptr = tokenptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenptr {
    pub array: *mut cmdp_token,
    pub n: ::core::ffi::c_int,
}
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;

pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const DEFAULT_DIR_MODE: ::core::ffi::c_int = S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IXGRP: ::core::ffi::c_int = S_IXUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IXOTH: ::core::ffi::c_int = S_IXGRP >> 3 as ::core::ffi::c_int;

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

pub static mut current_step: C2RustUnnamed_3 = STEP_WELCOME;

pub static mut modifier: C2RustUnnamed_4 = MOD_Mod4;
static mut config_path: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut xcb_numlock_mask: uint32_t = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut keysyms: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;
static mut modmap_reply: *mut xcb_get_modifier_mapping_reply_t =
    ::core::ptr::null::<xcb_get_modifier_mapping_reply_t>()
        as *mut xcb_get_modifier_mapping_reply_t;
static mut font: i3Font = Font {
    type_0: FONT_TYPE_NONE,
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
static mut bold_font: i3Font = Font {
    type_0: FONT_TYPE_NONE,
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
static mut socket_path: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
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
static mut xkb_base_event: uint8_t = 0;
static mut xkb_base_error: uint8_t = 0;
static mut tokens_FORCE_DISPLAY_URGENCY_HINT_MS: [cmdp_token; 2] = [
    token {
        name: c"'ms".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MAXIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 1 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"height".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 2 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_STRIP_WORKSPACE_NUMBERS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 3 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: FLOATING_MAXIMUM_SIZE_X,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: FLOATING_MINIMUM_SIZE_X,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDING_MODE_INDICATOR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 4 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 5 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_ON_WINDOW_ACTIVATION: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"mode".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 6 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_DISPLAY_URGENCY_HINT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"duration_ms".as_ptr().cast_mut(),
    next_state: FORCE_DISPLAY_URGENCY_HINT_MS,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_SWAP_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 7 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STRIP_WORKSPACE_NAME: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 8 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_START: [cmdp_token; 3] = [
    token {
        name: c"'\"".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: CRITERION_FROM_STR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 9 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 10 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_BORDER_PIXELS_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 11 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_BACK_AND_FORTH: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 12 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_WORKSPACE_NUMBER: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"number".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 13 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_MIN_WIDTH: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"width".as_ptr().cast_mut(),
    next_state: BAR_WORKSPACE_MIN_WIDTH_PX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MAXIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MAXIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FLOATING_MINIMUM_SIZE_X: [cmdp_token; 1] = [token {
    name: c"'x".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: FLOATING_MINIMUM_SIZE_HEIGHT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_POPUP_DURING_FULLSCREEN: [cmdp_token; 4] = [
    token {
        name: c"'ignore".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 14 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'leave_fullscreen".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 15 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 16 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 17 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR_COLORS,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM_STR_END: [cmdp_token; 1] = [token {
    name: c"'\"".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 18 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: BAR_COLORS_TEXT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_BUTTONS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 19 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WORKSPACE_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 20 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER_PIXELS: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 21 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"width".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_OUTPUT_WORD: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"output".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 22 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SEPARATOR_SYMBOL: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"separator".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 23 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FORCE_FOCUS_WRAPPING: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 24 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 25 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING_PX: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING_PX,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 26 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DEFAULT_ORIENTATION: [cmdp_token; 3] = [
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 27 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 28 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"orientation".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 29 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_FOLLOWS_MOUSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 30 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_BOTTOM: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"left".as_ptr().cast_mut(),
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 31 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_STATUS_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 32 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_DOWN_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 33 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_CHILD_BORDER: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"child_border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 34 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 35 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION_FROM_STR: [cmdp_token; 2] = [
    token {
        name: c"'auto".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'user".as_ptr().cast_mut(),
        identifier: c"kind".as_ptr().cast_mut(),
        next_state: CRITERION_FROM_STR_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOR_WINDOW_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 36 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: BAR_COLORS_BACKGROUND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 37 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_RIGHT: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"bottom".as_ptr().cast_mut(),
        next_state: BAR_PADDING_BOTTOM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 38 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FLOATING_MODIFIER: [cmdp_token; 10] = [
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 39 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_HIDE_EDGE_BORDERS: [cmdp_token; 12] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 40 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 41 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 42 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'both".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 43 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_no_gaps".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 44 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 45 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 46 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 47 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 48 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 49 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 50 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"hide_borders".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 51 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_COMMAND: [cmdp_token; 2] = [
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_OUTPUT_WORD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN_WORKSPACE: [cmdp_token; 5] = [
    token {
        name: c"'→".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'number".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN_WORKSPACE_NUMBER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"workspace".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 52 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS_BRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HIDDEN_STATE: [cmdp_token; 2] = [
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 53 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'show".as_ptr().cast_mut(),
        identifier: c"hidden_state".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 54 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING_LEFT: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_LEFT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 55 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_TRAY_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"padding_px".as_ptr().cast_mut(),
    next_state: BAR_TRAY_PADDING_PX,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_WHEEL_UP_CMD: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 56 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_BACKGROUND: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"background".as_ptr().cast_mut(),
    next_state: COLOR_TEXT,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_KILL_TIMEOUT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"timeout".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 57 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 58 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: MODE,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG_MODE: [cmdp_token; 3] = [
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 59 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE_LAYOUT: [cmdp_token; 4] = [
    token {
        name: c"'default".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 60 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'stacking".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 61 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'stacked".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 62 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tabbed".as_ptr().cast_mut(),
        identifier: c"layout".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 63 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_BAR_COMMAND: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"command".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 64 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_COLORS_TEXT: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 65 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"text".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 66 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: BAR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_PADDING_TOP: [cmdp_token; 3] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING_TOP,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"number".as_ptr().cast_mut(),
        identifier: c"right_or_right_and_left".as_ptr().cast_mut(),
        next_state: BAR_PADDING_RIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 67 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_SOCKET_PATH: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 68 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_TRAY_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 69 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_INDICATOR: [cmdp_token; 2] = [
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"indicator".as_ptr().cast_mut(),
        next_state: COLOR_CHILD_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 70 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_DISABLE_RANDR15: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 71 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS_WITH_SCOPE: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: GAPS_END,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_FROM: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_FROM_STR_START,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_DEFAULT_BORDER: [cmdp_token; 4] = [
    token {
        name: c"'normal".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER_PIXELS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'1pixel".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 72 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 73 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FOCUS_WRAPPING: [cmdp_token; 14] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 74 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 75 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 76 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 77 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 78 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 79 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 80 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 81 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 82 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 83 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 84 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 85 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 86 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 87 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FORCE_XINERAMA: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 88 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_ASSIGN_OUTPUT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 89 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_CRITERION_STR: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"cvalue".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 90 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MOUSE_WARPING: [cmdp_token; 2] = [
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 91 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 92 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_RESTART_STATE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 93 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_BORDERS: [cmdp_token; 7] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 94 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 95 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 96 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 97 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 98 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 99 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no_gaps".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 100 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_MODIFIER: [cmdp_token; 12] = [
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 101 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'none".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 102 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 103 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_POSITION: [cmdp_token; 2] = [
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 104 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"position".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 105 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_COLOR_BORDER: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"border".as_ptr().cast_mut(),
    next_state: COLOR_BACKGROUND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_SINGLE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"color".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 106 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FAKE_OUTPUTS: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"outputs".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 107 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODE_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: MODE_BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS_END: [cmdp_token; 1] = [token {
    name: c"end".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 108 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_BINDSYM: [cmdp_token; 2] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"button".as_ptr().cast_mut(),
        next_state: BAR_BINDSYM_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_PADDING: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"top_or_all".as_ptr().cast_mut(),
    next_state: BAR_PADDING_TOP,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_VERBOSE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 109 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDCOMMAND: [cmdp_token; 5] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 110 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_IGNORE_LINE: [cmdp_token; 1] = [token {
    name: c"line".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: INITIAL,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_TILING_DRAG: [cmdp_token; 9] = [
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 111 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 112 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 113 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 114 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 115 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"off".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 116 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'swap_modifier".as_ptr().cast_mut(),
        identifier: c"swap_modifier".as_ptr().cast_mut(),
        next_state: TILING_DRAG_SWAP_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'titlebar".as_ptr().cast_mut(),
        identifier: c"value".as_ptr().cast_mut(),
        next_state: TILING_DRAG_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_TITLE_ALIGN: [cmdp_token; 3] = [
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 117 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'center".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 118 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"alignment".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 119 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_COLORS: [cmdp_token; 15] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_statusline".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_separator".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focused_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'urgent_workspace".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'binding_mode".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: BAR_COLORS_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_HEIGHT: [cmdp_token; 1] = [token {
    name: c"number".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 120 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_OUTPUT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"output".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 121 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_COLOR_TEXT: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"text".as_ptr().cast_mut(),
    next_state: COLOR_INDICATOR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_FOR_WINDOW: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 122 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_IPC_SOCKET: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"path".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 123 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SHOW_MARKS: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"value".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 124 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_SMART_GAPS: [cmdp_token; 13] = [
    token {
        name: c"'1".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 125 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'yes".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 126 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'true".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 127 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'on".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 128 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'enable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 129 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'active".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 130 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'0".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 131 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 132 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'false".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 133 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'off".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 134 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 135 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inactive".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 136 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'inverse_outer".as_ptr().cast_mut(),
        identifier: c"enabled".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 137 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERION: [cmdp_token; 1] = [token {
    name: c"'=".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: CRITERION_STR,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_MODEBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODEBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_WORKSPACE: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"workspace".as_ptr().cast_mut(),
    next_state: WORKSPACE_COMMAND,
    extra: C2RustUnnamed_5 {
        call_identifier: 0 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BARBRACE: [cmdp_token; 2] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'{".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 138 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 139 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_MODE: [cmdp_token; 3] = [
    token {
        name: c"'dock".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 140 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hide".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 141 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'invisible".as_ptr().cast_mut(),
        identifier: c"mode".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 142 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_CRITERIA: [cmdp_token; 17] = [
    token {
        name: c"'class".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'instance".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'window_role".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'con_id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'window_type".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'con_mark".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'title".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'urgent".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'machine".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling_from".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: CRITERION_FROM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 143 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 144 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'all".as_ptr().cast_mut(),
        identifier: c"ctype".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 145 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"']".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 146 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_GAPS_END: [cmdp_token; 2] = [
    token {
        name: c"'px".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS_END,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 147 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODENAME: [cmdp_token; 2] = [
    token {
        name: c"'--pango_markup".as_ptr().cast_mut(),
        identifier: c"pango_markup".as_ptr().cast_mut(),
        next_state: MODENAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"modename".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 148 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_NO_FOCUS: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 149 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BINDING: [cmdp_token; 20] = [
    token {
        name: c"'--release".as_ptr().cast_mut(),
        identifier: c"release".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--border".as_ptr().cast_mut(),
        identifier: c"border".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--whole-window".as_ptr().cast_mut(),
        identifier: c"whole_window".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'--exclude-titlebar".as_ptr().cast_mut(),
        identifier: c"exclude_titlebar".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mod5".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Shift".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Control".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Ctrl".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Mode_switch".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group1".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group2".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group3".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'Group4".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'$mod".as_ptr().cast_mut(),
        identifier: c"modifiers".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'+".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"word".as_ptr().cast_mut(),
        identifier: c"key".as_ptr().cast_mut(),
        next_state: BINDCOMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_INCLUDE: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"pattern".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 150 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_INITIAL: [cmdp_token; 60] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set ".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set\t".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set_from_resource".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'include".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INCLUDE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bar".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BARBRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FONT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODENAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_BORDERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'smart_gaps".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SMART_GAPS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_minimum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MINIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_maximum_size".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MAXIMUM_SIZE_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'floating_modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FLOATING_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_orientation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DEFAULT_ORIENTATION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_layout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_LAYOUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'new_window".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'default_floating_border".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'new_float".as_ptr().cast_mut(),
        identifier: c"windowtype".as_ptr().cast_mut(),
        next_state: DEFAULT_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hide_edge_borders".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: HIDE_EDGE_BORDERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'for_window".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOR_WINDOW,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'assign".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: ASSIGN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'no_focus".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: NO_FOCUS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_follows_mouse".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_FOLLOWS_MOUSE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mouse_warping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MOUSE_WARPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_WRAPPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_focus_wrapping".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_FOCUS_WRAPPING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force-xinerama".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_XINERAMA,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable_randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'disable-randr15".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: DISABLE_RANDR15,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_auto_back_and_forth".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE_BACK_AND_FORTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'fake_outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'fake-outputs".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FAKE_OUTPUTS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'force_display_urgency_hint".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FORCE_DISPLAY_URGENCY_HINT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'focus_on_window_activation".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: FOCUS_ON_WINDOW_ACTIVATION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'title_align".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TITLE_ALIGN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'show_marks".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: SHOW_MARKS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: WORKSPACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc_socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc-socket".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_SOCKET,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'ipc_kill_timeout".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: IPC_KILL_TIMEOUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'restart_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: RESTART_STATE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'popup_during_fullscreen".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: POPUP_DURING_FULLSCREEN,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tiling_drag".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: TILING_DRAG,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'exec_always".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'exec".as_ptr().cast_mut(),
        identifier: c"exectype".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.background".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_SINGLE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused_inactive".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused_tab_title".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.focused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.unfocused".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.urgent".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'client.placeholder".as_ptr().cast_mut(),
        identifier: c"colorclass".as_ptr().cast_mut(),
        next_state: COLOR_BORDER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_ASSIGN: [cmdp_token; 1] = [token {
    name: c"'[".as_ptr().cast_mut(),
    identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 151 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_BAR_ID: [cmdp_token; 1] = [token {
    name: c"word".as_ptr().cast_mut(),
    identifier: c"bar_id".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 152 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_EXEC: [cmdp_token; 2] = [
    token {
        name: c"'--no-startup-id".as_ptr().cast_mut(),
        identifier: c"no_startup_id".as_ptr().cast_mut(),
        next_state: EXEC,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"string".as_ptr().cast_mut(),
        identifier: c"command".as_ptr().cast_mut(),
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
            call_identifier: 153 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_FONT: [cmdp_token; 1] = [token {
    name: c"string".as_ptr().cast_mut(),
    identifier: c"font".as_ptr().cast_mut(),
    next_state: __CALL,
    extra: C2RustUnnamed_5 {
        call_identifier: 154 as ::core::ffi::c_int as uint16_t,
    },
}];
static mut tokens_GAPS: [cmdp_token; 8] = [
    token {
        name: c"'inner".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'outer".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'horizontal".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'vertical".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'top".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'right".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bottom".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'left".as_ptr().cast_mut(),
        identifier: c"scope".as_ptr().cast_mut(),
        next_state: GAPS_WITH_SCOPE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_MODE: [cmdp_token; 8] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: MODE_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindcode".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bind".as_ptr().cast_mut(),
        identifier: c"bindtype".as_ptr().cast_mut(),
        next_state: MODE_BINDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: INITIAL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
];
static mut tokens_BAR: [cmdp_token; 31] = [
    token {
        name: c"end".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"error".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'#".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'set".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_IGNORE_LINE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'i3bar_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BAR_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'status_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STATUS_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_command".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_COMMAND,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'socket_path".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SOCKET_PATH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'mode".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'hidden_state".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HIDDEN_STATE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'id".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_ID,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'modifier".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_MODIFIER,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'wheel_up_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_UP_CMD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'wheel_down_cmd".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WHEEL_DOWN_CMD,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'bindsym".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDSYM,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'position".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_POSITION,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tray_output".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_OUTPUT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'tray_padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_TRAY_PADDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'font".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_FONT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'separator_symbol".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_SEPARATOR_SYMBOL,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'binding_mode_indicator".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_BINDING_MODE_INDICATOR,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_buttons".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_BUTTONS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'workspace_min_width".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_WORKSPACE_MIN_WIDTH,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'strip_workspace_numbers".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NUMBERS,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'strip_workspace_name".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_STRIP_WORKSPACE_NAME,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'verbose".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_VERBOSE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'height".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_HEIGHT,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'padding".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_PADDING,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'colors".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: BAR_COLORS_BRACE,
        extra: C2RustUnnamed_5 {
            call_identifier: 0 as ::core::ffi::c_int as uint16_t,
        },
    },
    token {
        name: c"'}".as_ptr().cast_mut(),
        identifier: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        next_state: __CALL,
        extra: C2RustUnnamed_5 {
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
static mut state: cmdp_state = FORCE_DISPLAY_URGENCY_HINT_MS;
static mut statelist: [cmdp_state; 10] = [
    INITIAL,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
    FORCE_DISPLAY_URGENCY_HINT_MS,
];
static mut statelist_idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut stack: [stack_entry; 10] = [stack_entry {
    identifier: ::core::ptr::null::<::core::ffi::c_char>(),
    type_0: STACK_STR,
    val: C2RustUnnamed_6 {
        str_0: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    },
}; 10];
unsafe fn push_string(identifier: *const ::core::ffi::c_char, str: *const ::core::ffi::c_char) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !stack[c as usize].identifier.is_null()
            && strcmp(stack[c as usize].identifier, identifier) != 0 as ::core::ffi::c_int
        {
            c += 1;
        } else {
            if stack[c as usize].identifier.is_null() {
                stack[c as usize].identifier = identifier;
                stack[c as usize].val.str_0 = sstrdup(str);
                stack[c as usize].type_0 = STACK_STR;
            } else {
                let prev: *mut ::core::ffi::c_char = stack[c as usize].val.str_0;
                sasprintf(
                    &raw mut (*(&raw mut stack as *mut stack_entry).offset(c as isize))
                        .val
                        .str_0,
                    c"%s,%s".as_ptr(),
                    prev,
                    str,
                );
                free(prev as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    fprintf(
        stderr,
        c"BUG: commands_parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr(),
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe fn push_long(identifier: *const ::core::ffi::c_char, num: ::core::ffi::c_long) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !stack[c as usize].identifier.is_null() {
            c += 1;
        } else {
            stack[c as usize].identifier = identifier;
            stack[c as usize].val.num = num;
            stack[c as usize].type_0 = STACK_LONG;
            return;
        }
    }
    fprintf(
        stderr,
        c"BUG: commands_parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr(),
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe fn get_string(identifier: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if stack[c as usize].identifier.is_null() {
            break;
        }
        if strcmp(identifier, stack[c as usize].identifier) == 0 as ::core::ffi::c_int {
            return stack[c as usize].val.str_0;
        }
        c += 1;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
unsafe fn clear_stack() {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if stack[c as usize].type_0 as ::core::ffi::c_uint
            == STACK_STR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free(stack[c as usize].val.str_0 as *mut ::core::ffi::c_void);
        }
        stack[c as usize].identifier = ::core::ptr::null::<::core::ffi::c_char>();
        stack[c as usize].val.str_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        stack[c as usize].val.num = 0 as ::core::ffi::c_long;
        c += 1;
    }
}
unsafe fn keysym_used_on_other_key(sym: KeySym, except_keycode: xcb_keycode_t) -> bool {
    let mut i: xcb_keycode_t = 0;
    let min_keycode: xcb_keycode_t = (*xcb_get_setup(conn)).min_keycode;
    let max_keycode: xcb_keycode_t = (*xcb_get_setup(conn)).max_keycode;
    i = min_keycode;
    while i as ::core::ffi::c_int != 0
        && i as ::core::ffi::c_int <= max_keycode as ::core::ffi::c_int
    {
        if !(i as ::core::ffi::c_int == except_keycode as ::core::ffi::c_int) {
            let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while level < 4 as ::core::ffi::c_int {
                if xcb_key_symbols_get_keysym(keysyms, i, level) as KeySym != sym {
                    level += 1;
                } else {
                    return true_0 != 0;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    false_0 != 0
}
unsafe fn next_state(token: *const cmdp_token) -> *mut ::core::ffi::c_char {
    let mut _next_state: cmdp_state = (*token).next_state;
    if (*token).next_state as ::core::ffi::c_uint
        == __CALL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let modifiers: *const ::core::ffi::c_char = get_string(c"modifiers".as_ptr());
        let keycode: ::core::ffi::c_int = atoi(get_string(c"key".as_ptr()));
        let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !modifiers.is_null() && !strstr(modifiers, c"Shift".as_ptr()).is_null() {
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
                *syms.offset(0 as ::core::ffi::c_int as isize) as KeySym,
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
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            errx(
                EXIT_FAILURE,
                c"xkb_keysym_get_name(%u) failed".as_ptr(),
                *syms_0.offset(0 as ::core::ffi::c_int as isize),
            );
        }
        let release: *const ::core::ffi::c_char = get_string(c"release".as_ptr());
        let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let modrep: *mut ::core::ffi::c_char = if modifiers.is_null() {
            sstrdup(c"".as_ptr())
        } else {
            sstrdup(modifiers)
        };
        let mut comma: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        loop {
            comma = strchr(modrep, ',' as i32);
            if comma.is_null() {
                break;
            }
            *comma = '+' as i32 as ::core::ffi::c_char;
        }
        sasprintf(
            &raw mut res,
            c"bindsym %s%s%s %s%s\n".as_ptr(),
            if modifiers.is_null() {
                c"".as_ptr()
            } else {
                modrep as *const ::core::ffi::c_char
            },
            if modifiers.is_null() {
                c"".as_ptr()
            } else {
                c"+".as_ptr()
            },
            &raw mut str as *mut ::core::ffi::c_char,
            if release.is_null() {
                c"".as_ptr()
            } else {
                release
            },
            get_string(c"command".as_ptr()),
        );
        clear_stack();
        free(modrep as *mut ::core::ffi::c_void);
        return res;
    }
    state = _next_state;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < statelist_idx {
        if statelist[i as usize] as ::core::ffi::c_uint != _next_state as ::core::ffi::c_uint {
            i += 1;
        } else {
            statelist_idx = i + 1 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    let fresh0 = statelist_idx;
    statelist_idx = statelist_idx + 1;
    statelist[fresh0 as usize] = _next_state;
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
unsafe fn rewrite_binding(input: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    state = INITIAL;
    statelist_idx = 1 as ::core::ffi::c_int;
    let mut walk: *const ::core::ffi::c_char = input;
    let len: size_t = strlen(input) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut token: *const cmdp_token = ::core::ptr::null::<cmdp_token>();
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while walk.offset_from(input) as ::core::ffi::c_long as size_t <= len {
        while (*walk as ::core::ffi::c_int == ' ' as i32
            || *walk as ::core::ffi::c_int == '\t' as i32)
            && *walk as ::core::ffi::c_int != '\0' as i32
        {
            walk = walk.offset(1);
        }
        let ptr: *mut cmdp_token_ptr =
            (&raw mut tokens as *mut cmdp_token_ptr).offset(state as isize) as *mut cmdp_token_ptr;
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
                        push_string(
                            (*token).identifier,
                            (*token).name.offset(1 as ::core::ffi::c_int as isize),
                        );
                    }
                    walk = walk.offset(strlen((*token).name).wrapping_sub(1 as size_t) as isize);
                    result = next_state(token);
                    if !result.is_null() {
                        return result;
                    }
                    break;
                }
            } else if strcmp((*token).name, c"number".as_ptr()) == 0 as ::core::ffi::c_int {
                let mut end: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *__errno_location() = 0 as ::core::ffi::c_int;
                let num: ::core::ffi::c_long = strtol(walk, &raw mut end, 10 as ::core::ffi::c_int);
                if !(*__errno_location() == ERANGE && (num == LONG_MIN || num == LONG_MAX)
                    || *__errno_location() != 0 as ::core::ffi::c_int
                        && num == 0 as ::core::ffi::c_long)
                {
                    if !(end == walk as *mut ::core::ffi::c_char) {
                        if !(*token).identifier.is_null() {
                            push_long((*token).identifier, num);
                        }
                        walk = end;
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
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
                            push_string((*token).identifier, str);
                        }
                        free(str as *mut ::core::ffi::c_void);
                        if *walk as ::core::ffi::c_int == '"' as i32 {
                            walk = walk.offset(1);
                        }
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
                        break;
                    }
                }
                if strcmp((*token).name, c"end".as_ptr()) == 0 as ::core::ffi::c_int {
                    if *walk as ::core::ffi::c_int == '\0' as i32
                        || *walk as ::core::ffi::c_int == '\n' as i32
                        || *walk as ::core::ffi::c_int == '\r' as i32
                    {
                        result = next_state(token);
                        if !result.is_null() {
                            return result;
                        }
                        walk = walk.offset(1);
                        break;
                    }
                }
            }
            c += 1;
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
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
    let string: *mut i3String = i3string_from_utf8(text);
    draw_util_text(
        string,
        &raw mut surface,
        fg,
        bg,
        x,
        y,
        logical_px(300 as ::core::ffi::c_int) - x - logical_px(4 as ::core::ffi::c_int),
    );
    i3string_free(string);
}
unsafe fn handle_expose() -> ::core::ffi::c_int {
    let black: color_t = draw_util_hex_to_color(c"#000000".as_ptr()) as color_t;
    let white: color_t = draw_util_hex_to_color(c"#FFFFFF".as_ptr()) as color_t;
    let green: color_t = draw_util_hex_to_color(c"#00FF00".as_ptr()) as color_t;
    let red: color_t = draw_util_hex_to_color(c"#FF0000".as_ptr()) as color_t;
    draw_util_clear_surface(&raw mut surface, black);
    set_font(&raw mut font);
    if current_step as ::core::ffi::c_uint
        == STEP_WELCOME as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
        let mut msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut msg, c"%s?".as_ptr(), config_path);
        txt(
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            msg,
            white,
            black,
        );
        free(msg as *mut ::core::ffi::c_void);
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
    if current_step as ::core::ffi::c_uint
        == STEP_GENERATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
        if modifier as ::core::ffi::c_uint == MOD_Mod4 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
        if modifier as ::core::ffi::c_uint == MOD_Mod4 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
    ignored: *mut ::core::ffi::c_void,
    conn_0: *mut xcb_connection_t,
    event: *mut xcb_key_press_event_t,
) -> ::core::ffi::c_int {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let mut state_filtered: uint16_t = ((*event).state as uint32_t
        & !(xcb_numlock_mask | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as uint32_t))
        as uint16_t;
    state_filtered =
        (state_filtered as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint16_t;
    let sym: xcb_keysym_t =
        xcb_key_press_lookup_keysym(symbols, event, state_filtered as ::core::ffi::c_int);
    printf(c"sym = %c (%d)\n".as_ptr(), sym, sym);
    if sym == XK_Return as xcb_keysym_t || sym == XK_KP_Enter as xcb_keysym_t {
        if current_step as ::core::ffi::c_uint
            == STEP_WELCOME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current_step = STEP_GENERATE;
            xcb_change_property(
                conn_0,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                win,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as uint8_t,
                strlen(c"i3: generate config".as_ptr()) as uint32_t,
                c"i3: generate config".as_ptr() as *const ::core::ffi::c_void,
            );
            xcb_flush(conn_0);
        } else {
            finish();
        }
    }
    if sym == XK_Up as xcb_keysym_t || sym == XK_Down as xcb_keysym_t {
        modifier = (if modifier as ::core::ffi::c_uint
            == MOD_Mod1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            MOD_Mod4 as ::core::ffi::c_int
        } else {
            MOD_Mod1 as ::core::ffi::c_int
        }) as C2RustUnnamed_4;
        handle_expose();
    }
    if sym == XK_Escape as xcb_keysym_t {
        exit(0 as ::core::ffi::c_int);
    }
    let modmap: *mut xcb_keycode_t = xcb_get_modifier_mapping_keycodes(modmap_reply);
    let mut mask: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int {
        let code: xcb_keycode_t = *modmap.offset(
            (mask * (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int + i) as isize,
        );
        if !(code as ::core::ffi::c_long == XCB_NONE) {
            printf(
                c"Modifier keycode for Mod1: 0x%02x\n".as_ptr(),
                code as ::core::ffi::c_int,
            );
            if code as ::core::ffi::c_int == (*event).detail as ::core::ffi::c_int {
                modifier = MOD_Mod1;
                printf(c"This is Mod1!\n".as_ptr());
            }
        }
        i += 1;
    }
    mask = 6 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int {
        let code_0: xcb_keycode_t = *modmap.offset(
            (mask * (*modmap_reply).keycodes_per_modifier as ::core::ffi::c_int + i_0) as isize,
        );
        if !(code_0 as ::core::ffi::c_long == XCB_NONE) {
            printf(
                c"Modifier keycode for Mod4: 0x%02x\n".as_ptr(),
                code_0 as ::core::ffi::c_int,
            );
            if code_0 as ::core::ffi::c_int == (*event).detail as ::core::ffi::c_int {
                modifier = MOD_Mod4;
                printf(c"This is Mod4!\n".as_ptr());
            }
        }
        i_0 += 1;
    }
    handle_expose();
    1 as ::core::ffi::c_int
}
unsafe fn handle_button_press(event: *mut xcb_button_press_event_t) {
    if current_step as ::core::ffi::c_uint
        != STEP_GENERATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if ((*event).event_x as ::core::ffi::c_int)
        < (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * char_width
            + logical_px(4 as ::core::ffi::c_int)
        || (*event).event_x as ::core::ffi::c_int
            > (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * char_width
                + logical_px(4 as ::core::ffi::c_int)
    {
        return;
    }
    if (*event).event_y as ::core::ffi::c_int
        >= (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
            + logical_px(4 as ::core::ffi::c_int)
        && (*event).event_y as ::core::ffi::c_int
            <= (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
                + logical_px(4 as ::core::ffi::c_int)
                + font.height
    {
        modifier = MOD_Mod4;
        handle_expose();
    }
    if (*event).event_y as ::core::ffi::c_int
        >= (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
            + logical_px(4 as ::core::ffi::c_int)
        && (*event).event_y as ::core::ffi::c_int
            <= (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * font.height
                + logical_px(4 as ::core::ffi::c_int)
                + font.height
    {
        modifier = MOD_Mod1;
        handle_expose();
    }
}
unsafe fn finish() {
    printf(c"creating \"%s\"...\n".as_ptr(), config_path);
    let mut xkb_context: *mut xkb_context = ::core::ptr::null_mut::<xkb_context>();
    xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    if xkb_context.is_null() {
        errx(
            1 as ::core::ffi::c_int,
            c"could not create xkbcommon context".as_ptr(),
        );
    }
    let device_id: int32_t = xkb_x11_get_core_keyboard_device_id(conn);
    xkb_keymap =
        xkb_x11_keymap_new_from_device(xkb_context, conn, device_id, XKB_KEYMAP_COMPILE_NO_FLAGS);
    if xkb_keymap.is_null() {
        errx(
            1 as ::core::ffi::c_int,
            c"xkb_x11_keymap_new_from_device failed".as_ptr(),
        );
    }
    let kc_config: *mut FILE =
        fopen(c"/usr/local/etc/i3/config.keycodes".as_ptr(), c"r".as_ptr()) as *mut FILE;
    if kc_config.is_null() {
        err(
            1 as ::core::ffi::c_int,
            c"Could not open input file \"%s\"".as_ptr(),
            c"/usr/local/etc/i3/config.keycodes".as_ptr(),
        );
    }
    let ks_config: *mut FILE = fopen(config_path, c"w".as_ptr()) as *mut FILE;
    if ks_config.is_null() {
        err(
            1 as ::core::ffi::c_int,
            c"Could not open output config file \"%s\"".as_ptr(),
            config_path,
        );
    }
    free(config_path as *mut ::core::ffi::c_void);
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0 as size_t;
    let mut read: ssize_t = 0;
    let mut head_of_file: bool = true_0 != 0;
    fputs(
        c"# This file has been auto-generated by i3-config-wizard(1).\n".as_ptr(),
        ks_config,
    );
    fputs(
        c"# It will not be overwritten, so edit it as you like.\n".as_ptr(),
        ks_config,
    );
    fputs(c"#\n".as_ptr(), ks_config);
    fputs(
        c"# Should you change your keyboard layout some time, delete\n".as_ptr(),
        ks_config,
    );
    fputs(
        c"# this file and re-run i3-config-wizard(1).\n".as_ptr(),
        ks_config,
    );
    fputs(c"#\n".as_ptr(), ks_config);
    let mut linecap: size_t = 0 as size_t;
    loop {
        read = getline(&raw mut line, &raw mut linecap, kc_config) as ssize_t;
        if !(read != -(1 as ::core::ffi::c_int) as ssize_t) {
            break;
        }
        len = strlen(line);
        if head_of_file as ::core::ffi::c_int != 0
            && strncmp(c"# WARNING".as_ptr(), line, strlen(c"# WARNING".as_ptr()))
                == 0 as ::core::ffi::c_int
        {
            continue;
        }
        head_of_file = false_0 != 0;
        let mut walk: *mut ::core::ffi::c_char = line;
        while *(*__ctype_b_loc())
            .offset(*walk as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
            && walk < line.offset(len as isize)
        {
            fputc(*walk as ::core::ffi::c_int, ks_config);
            walk = walk.offset(1);
        }
        if strncmp(walk, c"set $mod ".as_ptr(), strlen(c"set $mod ".as_ptr()))
            == 0 as ::core::ffi::c_int
        {
            if modifier as ::core::ffi::c_uint
                == MOD_Mod1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fputs(c"set $mod Mod1\n".as_ptr(), ks_config);
            } else {
                fputs(c"set $mod Mod4\n".as_ptr(), ks_config);
            }
        } else if strncmp(walk, c"bindcode".as_ptr(), strlen(c"bindcode".as_ptr()))
            != 0 as ::core::ffi::c_int
        {
            fputs(walk, ks_config);
        } else {
            let result: *mut ::core::ffi::c_char = rewrite_binding(walk);
            fputs(result, ks_config);
            free(result as *mut ::core::ffi::c_void);
        }
    }
    fflush(ks_config);
    fsync(fileno(ks_config));
    free(line as *mut ::core::ffi::c_void);
    fclose(kc_config);
    fclose(ks_config);
    let sockfd: ::core::ffi::c_int = ipc_connect(socket_path);
    ipc_send_message(
        sockfd,
        strlen(c"reload".as_ptr()) as uint32_t,
        0 as uint32_t,
        c"reload".as_ptr() as *mut uint8_t,
    );
    close(sockfd);
    exit(0 as ::core::ffi::c_int);
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
    socket_path = settings
        .socket
        .as_ref()
        .map_or(std::ptr::null_mut(), |socket| socket.as_ptr().cast_mut());
    let headless_run = settings.modifier.is_some();
    if let Some(key) = settings.modifier {
        modifier = match key {
            ModifierKey::Alt => MOD_Mod1,
            ModifierKey::Win => MOD_Mod4,
        };
    }
    let path: *mut ::core::ffi::c_char =
        get_config_path(::core::ptr::null::<::core::ffi::c_char>(), false_0 != 0);
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
    xdg_config_home = resolve_tilde(xdg_config_home);
    sasprintf(
        &raw mut config_path,
        c"%s/i3/config".as_ptr(),
        xdg_config_home,
    );
    let mut config_dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stbuf: stat = ::core::mem::zeroed();
    sasprintf(&raw mut config_dir, c"%s/i3".as_ptr(), xdg_config_home);
    if stat(config_dir, &raw mut stbuf) != 0 as ::core::ffi::c_int {
        if mkdirp(config_dir, DEFAULT_DIR_MODE as mode_t) != 0 as ::core::ffi::c_int {
            err(EXIT_FAILURE, c"mkdirp(%s) failed".as_ptr(), config_dir);
        }
    }
    free(config_dir as *mut ::core::ffi::c_void);
    free(xdg_config_home as *mut ::core::ffi::c_void);
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(config_path, O_CREAT | O_RDWR, 0o644 as ::core::ffi::c_int);
    if fd == -(1 as ::core::ffi::c_int) {
        printf(
            c"Cannot open file \"%s\" for writing: %s. Exiting.\n".as_ptr(),
            config_path,
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    close(fd);
    unlink(config_path);
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(1 as ::core::ffi::c_int, c"Cannot open display".as_ptr());
    }
    if xkb_x11_setup_xkb_extension(
        conn,
        XKB_X11_MIN_MAJOR_XKB_VERSION as uint16_t,
        XKB_X11_MIN_MINOR_XKB_VERSION as uint16_t,
        xkb_x11_setup_xkb_extension_flags::NO_FLAGS,
        ::core::ptr::null_mut::<uint16_t>(),
        ::core::ptr::null_mut::<uint16_t>(),
        &raw mut xkb_base_event,
        &raw mut xkb_base_error,
    ) != 1 as ::core::ffi::c_int
    {
        errx(EXIT_FAILURE, c"Could not setup XKB extension.".as_ptr());
    }
    keysyms = xcb_key_symbols_alloc(conn);
    let mut modmap_cookie: xcb_get_modifier_mapping_cookie_t =
        xcb_get_modifier_mapping_cookie_t { sequence: 0 };
    modmap_cookie = xcb_get_modifier_mapping(conn);
    symbols = xcb_key_symbols_alloc(conn);
    if headless_run {
        finish();
        return 0 as ::core::ffi::c_int;
    }
    let mut _NET_WM_NAME_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_NAME".as_ptr()) as uint16_t,
        c"_NET_WM_NAME".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DIALOG_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr()) as uint16_t,
        c"_NET_WM_WINDOW_TYPE_DIALOG".as_ptr(),
    );
    let ATOM_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"ATOM".as_ptr()) as uint16_t,
        c"ATOM".as_ptr(),
    );
    let CARDINAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"CARDINAL".as_ptr()) as uint16_t,
        c"CARDINAL".as_ptr(),
    );
    let UTF8_STRING_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as uint8_t,
        strlen(c"UTF8_STRING".as_ptr()) as uint16_t,
        c"UTF8_STRING".as_ptr(),
    );
    let sndisplay: *mut SnDisplay = sn_xcb_display_new(conn, None, None);
    let sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screen);
    sn_display_unref(sndisplay);
    root_screen = xcb_aux_get_screen(conn, screen);
    root = (*root_screen).root;
    modmap_reply = xcb_get_modifier_mapping_reply(
        conn,
        modmap_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if modmap_reply.is_null() {
        errx(EXIT_FAILURE, c"Could not get modifier mapping".as_ptr());
    }
    xcb_numlock_mask = get_mod_mask_for(XCB_NUM_LOCK as uint32_t, symbols, modmap_reply);
    init_dpi();
    font = load_font(pattern, true_0 != 0);
    bold_font = load_font(patternbold, true_0 != 0);
    set_font(&raw mut font);
    char_width = predict_text_width(i3string_from_utf8(c"a".as_ptr()));
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [uint32_t; 2] = [
        0 as ::core::ffi::c_int as uint32_t,
        (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int) as uint32_t,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        win,
        root,
        logical_px(490 as ::core::ffi::c_int) as int16_t,
        logical_px(297 as ::core::ffi::c_int) as int16_t,
        logical_px(300 as ::core::ffi::c_int) as uint16_t,
        (15 as ::core::ffi::c_int * font.height + logical_px(4 as ::core::ffi::c_int)) as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int)
            as uint32_t,
        &raw mut fresh1 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as Window);
    }
    xcb_map_window(conn, win);
    let reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_NAME_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errx(EXIT_FAILURE, c"Could not get atom _NET_WM_NAME".as_ptr());
    }
    A__NET_WM_NAME = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    let reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errx(
            EXIT_FAILURE,
            c"Could not get atom _NET_WM_WINDOW_TYPE".as_ptr(),
        );
    }
    A__NET_WM_WINDOW_TYPE = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DIALOG_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errx(
            EXIT_FAILURE,
            c"Could not get atom _NET_WM_WINDOW_TYPE_DIALOG".as_ptr(),
        );
    }
    A__NET_WM_WINDOW_TYPE_DIALOG = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        ATOM_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errx(EXIT_FAILURE, c"Could not get atom ATOM".as_ptr());
    }
    A_ATOM = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        CARDINAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errx(EXIT_FAILURE, c"Could not get atom CARDINAL".as_ptr());
    }
    A_CARDINAL = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        UTF8_STRING_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errx(EXIT_FAILURE, c"Could not get atom UTF8_STRING".as_ptr());
    }
    A_UTF8_STRING = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut A__NET_WM_WINDOW_TYPE_DIALOG as *mut ::core::ffi::c_uchar
            as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        win,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(c"i3: first configuration".as_ptr()) as uint32_t,
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
    let mut cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard_cookie_t { sequence: 0 };
    let mut reply_5: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (reply_5.is_null()
        || (*reply_5).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int)
        && {
            let fresh2 = count;
            count = count + 1;
            fresh2 < 500 as ::core::ffi::c_int
        }
    {
        cookie = xcb_grab_keyboard(
            conn,
            false_0 as uint8_t,
            win,
            XCB_CURRENT_TIME as xcb_timestamp_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        );
        reply_5 = xcb_grab_keyboard_reply(
            conn,
            cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        usleep(1000 as __useconds_t);
    }
    if (*reply_5).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            (*reply_5).status as ::core::ffi::c_int,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
        sn_launchee_context_unref(sncontext);
    }
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
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
                    handle_key_press(NULL, conn, event as *mut xcb_key_press_event_t);
                }
                XCB_BUTTON_PRESS => {
                    handle_button_press(event as *mut xcb_button_press_event_t);
                }
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose();
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    draw_util_surface_free(conn, &raw mut surface);
    0 as ::core::ffi::c_int
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
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

pub use libc::{
    __errno_location, atoi, close, exit, fclose, fflush, fileno, fopen, fprintf, fputc, fputs,
    free, fsync, getenv, getline, open, printf, stat, strchr, strcmp, strerror, strlen,
    strncasecmp, strncmp, strstr, strtol, unlink, usleep, FILE,
};

use i3_common::native_stdio::{__ctype_b_loc, err, errx, stderr, stdout, vfprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_config_path::get_config_path;
use i3_common::get_mod_mask::get_mod_mask_for;
use i3_common::get_visualtype::get_visualtype;
use i3_common::ipc_connect::ipc_connect;
use i3_common::ipc_send_message::ipc_send_message;
use i3_common::resolve_tilde::resolve_tilde;

pub use i3_common::string::I3String as _i3String;
