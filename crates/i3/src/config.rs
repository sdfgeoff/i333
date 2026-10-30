use crate::application::assignments;
use crate::application::bindings;
use crate::application::ws_assignments;
use crate::bindings::binding_free;
use crate::con::arena::{containers, resolve};
use crate::config_parser::free_variables;
use crate::config_parser::parse_file;
use crate::config_parser::ConfigParserState;
use crate::types::BorderStyle;
use crate::types::ConfigLoad;
use crate::types::FocusActivation;
use crate::types::FocusWrapping;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::ParseFileResult;
use crate::types::PopupDuringFullscreen;
use crate::types::SmartGaps;
use crate::types::TitleAlign;
use crate::types::UrgencyMatch;
use crate::types::Warping;
use crate::types::WindowMode;
use i3_common::font::FontType;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * config.c: Configuration file (calling the parser (src/config_parser.c) with
 *           the correct path, switching key bindings mode).
 *
 */
// Mechanical file-by-file Rust translation of src/config.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub use crate::application::Binding;

pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_NODOCK: ::core::ffi::c_int = 0;

pub use crate::regex::Regex as regex;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub use crate::application::bindings_head;
pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncludedFile {
    pub path: *mut ::core::ffi::c_char,
    pub raw_contents: *mut ::core::ffi::c_char,
    pub variable_replaced_contents: *mut ::core::ffi::c_char,
    pub files: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut IncludedFile,
    pub tqe_prev: *mut *mut IncludedFile,
}

pub use crate::application::Barconfig;

pub use crate::application::Barbinding;

pub use crate::application::barconfig_head;
pub use crate::application::tray_output_t;
pub use crate::bindings::modes_head;
pub use crate::bindings::Mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct includedfiles_head {
    pub tqh_first: *mut IncludedFile,
    pub tqh_last: *mut *mut IncludedFile,
}
/// Owned parser variables; these never reside in the zeroed global Config POD.
#[derive(Debug)]
pub struct Variable {
    pub key: std::ffi::CString,
    pub value: std::ffi::CString,
}
pub type variables_head = Vec<Variable>;

pub struct parser_ctx {
    pub use_nagbar: bool,
    pub state: ConfigParserState,
    pub current_match: Match,
    pub statelist: Vec<ConfigParserState>,
    pub statelist_idx: ::core::ffi::c_int,
    pub stack: *mut stack,
    pub variables: variables_head,
    pub has_errors: bool,
}

pub use crate::commands::stack;

pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"void free_configuration(void)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut current_configpath: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub static mut config: Config = Config {
    terminal: ::core::ptr::null::<::core::ffi::c_char>(),
    font: Font {
        type_0: FontType::None,
        height: 0,
        pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        specific: C2RustUnnamed {
            xcb: C2RustUnnamed_0 {
                id: 0,
                info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
                table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
            },
        },
    },
    ipc_socket_path: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    restart_state_path: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    default_layout: Layout::Default,
    container_stack_limit: 0,
    container_stack_limit_value: 0,
    default_border_width: 0,
    default_floating_border_width: 0,
    default_orientation: Orientation::None,
    disable_focus_follows_mouse: false,
    mouse_warping: Warping::Output,
    hide_edge_borders: HEBM_NONE,
    disable_workspace_bar: false,
    focus_wrapping: FocusWrapping::Off,
    force_xinerama: false,
    disable_randr15: false,
    fake_outputs: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    workspace_auto_back_and_forth: false,
    workspace_urgency_timer: 0.,
    focus_on_window_activation: FocusActivation::Smart,
    show_marks: false,
    title_align: TitleAlign::Left,
    default_border: BorderStyle::None,
    default_floating_border: BorderStyle::None,
    floating_modifier: 0,
    swap_modifier: 0,
    floating_maximum_width: 0,
    floating_maximum_height: 0,
    floating_minimum_width: 0,
    floating_minimum_height: 0,
    client: config_client {
        background: color_t {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.,
            colorpixel: 0,
        },
        focused: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        focused_inactive: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        focused_tab_title: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        unfocused: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        urgent: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        placeholder: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        got_focused_tab_title: false,
    },
    bar: config_bar {
        focused: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        unfocused: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
        urgent: Colortriple {
            border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            background: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            text: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            indicator: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
            child_border: color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            },
        },
    },
    popup_during_fullscreen: PopupDuringFullscreen::Smart,
    number_barconfigs: 0,
    tiling_drag: TILING_DRAG_OFF,
    gaps: gaps_t {
        inner: 0,
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    },
    smart_gaps: SmartGaps::Off,
};

pub static mut modes: modes_head = modes_head {
    slh_first: ::core::ptr::null::<Mode>() as *mut Mode,
};

pub static mut barconfigs: barconfig_head = barconfig_head {
    tqh_first: std::ptr::null_mut(),
    tqh_last: unsafe { &raw mut barconfigs.tqh_first },
};

pub static mut included_files: includedfiles_head = includedfiles_head {
    tqh_first: std::ptr::null_mut(),
    tqh_last: unsafe { &raw mut included_files.tqh_first },
};

pub unsafe fn ungrab_all_keys(conn_0: *mut xcb_connection_t) {
    debuglog(
        c"%s:%s:%d - Ungrabbing all keys\n".as_ptr().cast_mut(),
        c"../src/config.c".as_ptr(),
        c"ungrab_all_keys".as_ptr(),
        28 as ::core::ffi::c_int,
    );
    xcb_ungrab_key(
        conn_0,
        XCB_GRAB_ANY as ::core::ffi::c_int as xcb_keycode_t,
        root,
        XCB_BUTTON_MASK_ANY as ::core::ffi::c_int as u16,
    );
}
unsafe fn free_configuration() {
    if conn != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut xcb_connection_t {
    } else {
        __assert_fail(
            c"conn != NULL".as_ptr(),
            c"../src/config.c".as_ptr(),
            33 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    switch_mode(c"default".as_ptr());
    ungrab_all_keys(conn);
    while !modes.slh_first.is_null() {
        let mut mode: *mut Mode = modes.slh_first as *mut Mode;
        free((*mode).name as *mut ::core::ffi::c_void);
        (*mode).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        while !(*(*mode).bindings).tqh_first.is_null() {
            let bind: *mut Binding = (*(*mode).bindings).tqh_first as *mut Binding;
            if !(*bind).bindings.tqe_next.is_null() {
                (*(*bind).bindings.tqe_next).bindings.tqe_prev = (*bind).bindings.tqe_prev;
            } else {
                (*(*mode).bindings).tqh_last = (*bind).bindings.tqe_prev;
            }
            *(*bind).bindings.tqe_prev = (*bind).bindings.tqe_next;
            binding_free(bind);
        }
        free((*mode).bindings as *mut ::core::ffi::c_void);
        (*mode).bindings = ::core::ptr::null_mut::<bindings_head>();
        if modes.slh_first == mode {
            modes.slh_first = (*modes.slh_first).modes.sle_next as *mut Mode;
        } else {
            let mut curelm: *mut Mode = modes.slh_first as *mut Mode;
            while (*curelm).modes.sle_next != mode {
                curelm = (*curelm).modes.sle_next;
            }
            (*curelm).modes.sle_next = (*(*curelm).modes.sle_next).modes.sle_next;
        }
        free(mode as *mut ::core::ffi::c_void);
        mode = ::core::ptr::null_mut::<Mode>();
    }
    while !assignments.tqh_first.is_null() {
        let mut assign: *mut Assignment = assignments.tqh_first as *mut Assignment;
        if (*assign).type_0 as ::core::ffi::c_uint
            == A_TO_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*assign).type_0 as ::core::ffi::c_uint
                == A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free((*assign).dest.workspace as *mut ::core::ffi::c_void);
            (*assign).dest.workspace = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else if (*assign).type_0 as ::core::ffi::c_uint
            == A_COMMAND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free((*assign).dest.command as *mut ::core::ffi::c_void);
            (*assign).dest.command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else if (*assign).type_0 as ::core::ffi::c_uint
            == A_TO_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free((*assign).dest.output as *mut ::core::ffi::c_void);
            (*assign).dest.output = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        match_free(&raw mut (*assign).match_0);
        if !(*assign).assignments.tqe_next.is_null() {
            (*(*assign).assignments.tqe_next).assignments.tqe_prev = (*assign).assignments.tqe_prev;
        } else {
            assignments.tqh_last = (*assign).assignments.tqe_prev;
        }
        *(*assign).assignments.tqe_prev = (*assign).assignments.tqe_next;
        free(assign as *mut ::core::ffi::c_void);
        assign = ::core::ptr::null_mut::<Assignment>();
    }
    while !ws_assignments.tqh_first.is_null() {
        let mut assign_0: *mut Workspace_Assignment = ws_assignments.tqh_first;
        free((*assign_0).name as *mut ::core::ffi::c_void);
        (*assign_0).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*assign_0).output as *mut ::core::ffi::c_void);
        (*assign_0).output = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*assign_0).ws_assignments.tqe_next.is_null() {
            (*(*assign_0).ws_assignments.tqe_next)
                .ws_assignments
                .tqe_prev = (*assign_0).ws_assignments.tqe_prev;
        } else {
            ws_assignments.tqh_last = (*assign_0).ws_assignments.tqe_prev;
        }
        *(*assign_0).ws_assignments.tqe_prev = (*assign_0).ws_assignments.tqe_next;
        free(assign_0 as *mut ::core::ffi::c_void);
        assign_0 = ::core::ptr::null_mut::<Workspace_Assignment>();
    }
    while !barconfigs.tqh_first.is_null() {
        let mut barconfig: *mut Barconfig = barconfigs.tqh_first as *mut Barconfig;
        free((*barconfig).id as *mut ::core::ffi::c_void);
        (*barconfig).id = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < (*barconfig).num_outputs {
            free(*(*barconfig).outputs.offset(c as isize) as *mut ::core::ffi::c_void);
            c += 1;
        }
        while !(*barconfig).bar_bindings.tqh_first.is_null() {
            let mut binding: *mut Barbinding =
                (*barconfig).bar_bindings.tqh_first as *mut Barbinding;
            free((*binding).command as *mut ::core::ffi::c_void);
            (*binding).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(*binding).bindings.tqe_next.is_null() {
                (*(*binding).bindings.tqe_next).bindings.tqe_prev = (*binding).bindings.tqe_prev;
            } else {
                (*barconfig).bar_bindings.tqh_last = (*binding).bindings.tqe_prev;
            }
            *(*binding).bindings.tqe_prev = (*binding).bindings.tqe_next;
            free(binding as *mut ::core::ffi::c_void);
            binding = ::core::ptr::null_mut::<Barbinding>();
        }
        while !(*barconfig).tray_outputs.tqh_first.is_null() {
            let mut tray_output: *mut tray_output_t =
                (*barconfig).tray_outputs.tqh_first as *mut tray_output_t;
            free((*tray_output).output as *mut ::core::ffi::c_void);
            (*tray_output).output = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !(*tray_output).tray_outputs.tqe_next.is_null() {
                (*(*tray_output).tray_outputs.tqe_next)
                    .tray_outputs
                    .tqe_prev = (*tray_output).tray_outputs.tqe_prev;
            } else {
                (*barconfig).tray_outputs.tqh_last = (*tray_output).tray_outputs.tqe_prev;
            }
            *(*tray_output).tray_outputs.tqe_prev = (*tray_output).tray_outputs.tqe_next;
            free(tray_output as *mut ::core::ffi::c_void);
            tray_output = ::core::ptr::null_mut::<tray_output_t>();
        }
        free((*barconfig).outputs as *mut ::core::ffi::c_void);
        (*barconfig).outputs = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        free((*barconfig).socket_path as *mut ::core::ffi::c_void);
        (*barconfig).socket_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).status_command as *mut ::core::ffi::c_void);
        (*barconfig).status_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).workspace_command as *mut ::core::ffi::c_void);
        (*barconfig).workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).i3bar_command as *mut ::core::ffi::c_void);
        (*barconfig).i3bar_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).font as *mut ::core::ffi::c_void);
        (*barconfig).font = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.background as *mut ::core::ffi::c_void);
        (*barconfig).colors.background = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.statusline as *mut ::core::ffi::c_void);
        (*barconfig).colors.statusline = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.separator as *mut ::core::ffi::c_void);
        (*barconfig).colors.separator = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_background as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_background = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_statusline as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_statusline = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_separator as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_separator = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_workspace_border as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_workspace_border =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_workspace_bg as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_workspace_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.focused_workspace_text as *mut ::core::ffi::c_void);
        (*barconfig).colors.focused_workspace_text = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.active_workspace_border as *mut ::core::ffi::c_void);
        (*barconfig).colors.active_workspace_border =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.active_workspace_bg as *mut ::core::ffi::c_void);
        (*barconfig).colors.active_workspace_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.active_workspace_text as *mut ::core::ffi::c_void);
        (*barconfig).colors.active_workspace_text = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.inactive_workspace_border as *mut ::core::ffi::c_void);
        (*barconfig).colors.inactive_workspace_border =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.inactive_workspace_bg as *mut ::core::ffi::c_void);
        (*barconfig).colors.inactive_workspace_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.inactive_workspace_text as *mut ::core::ffi::c_void);
        (*barconfig).colors.inactive_workspace_text =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.urgent_workspace_border as *mut ::core::ffi::c_void);
        (*barconfig).colors.urgent_workspace_border =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.urgent_workspace_bg as *mut ::core::ffi::c_void);
        (*barconfig).colors.urgent_workspace_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.urgent_workspace_text as *mut ::core::ffi::c_void);
        (*barconfig).colors.urgent_workspace_text = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.binding_mode_border as *mut ::core::ffi::c_void);
        (*barconfig).colors.binding_mode_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.binding_mode_bg as *mut ::core::ffi::c_void);
        (*barconfig).colors.binding_mode_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*barconfig).colors.binding_mode_text as *mut ::core::ffi::c_void);
        (*barconfig).colors.binding_mode_text = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*barconfig).configs.tqe_next.is_null() {
            (*(*barconfig).configs.tqe_next).configs.tqe_prev = (*barconfig).configs.tqe_prev;
        } else {
            barconfigs.tqh_last = (*barconfig).configs.tqe_prev;
        }
        *(*barconfig).configs.tqe_prev = (*barconfig).configs.tqe_next;
        free(barconfig as *mut ::core::ffi::c_void);
        barconfig = ::core::ptr::null_mut::<Barconfig>();
    }
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        if !(*con).window.is_null() {
            (*(*con).window).nr_assignments = 0 as u32;
            free((*(*con).window).ran_assignments as *mut ::core::ffi::c_void);
            (*(*con).window).ran_assignments = ::core::ptr::null_mut::<*mut Assignment>();
        }
        free((*con).deco_render_params as *mut ::core::ffi::c_void);
        (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    }
    free_font();
    free(config.ipc_socket_path as *mut ::core::ffi::c_void);
    free(config.restart_state_path as *mut ::core::ffi::c_void);
    free(config.fake_outputs as *mut ::core::ffi::c_void);
}

pub unsafe fn load_configuration(
    override_configpath: *const ::core::ffi::c_char,
    load_type: ConfigLoad,
) -> bool {
    if load_type == ConfigLoad::Reload {
        free_configuration();
    }
    modes.slh_first = ::core::ptr::null_mut::<Mode>();
    let default_mode: *mut Mode =
        scalloc(1 as usize, ::core::mem::size_of::<Mode>() as usize) as *mut Mode;
    (*default_mode).name = sstrdup(c"default".as_ptr());
    (*default_mode).bindings =
        scalloc(1 as usize, ::core::mem::size_of::<bindings_head>() as usize) as *mut bindings_head;
    (*(*default_mode).bindings).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*(*default_mode).bindings).tqh_last = &raw mut (*(*default_mode).bindings).tqh_first;
    (*default_mode).modes.sle_next = modes.slh_first as *mut Mode;
    modes.slh_first = default_mode as *mut Mode;
    bindings = (*default_mode).bindings as *mut bindings_head;
    current_binding_mode = (*default_mode).name;
    memset(
        &raw mut config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Config>() as usize,
    );
    config.client.background = draw_util_hex_to_color(c"#000000".as_ptr());
    config.client.focused.border = draw_util_hex_to_color(c"#4c7899".as_ptr());
    config.client.focused.background = draw_util_hex_to_color(c"#285577".as_ptr());
    config.client.focused.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.client.focused.indicator = draw_util_hex_to_color(c"#2e9ef4".as_ptr());
    config.client.focused.child_border = draw_util_hex_to_color(c"#285577".as_ptr());
    config.client.focused_inactive.border = draw_util_hex_to_color(c"#333333".as_ptr());
    config.client.focused_inactive.background = draw_util_hex_to_color(c"#5f676a".as_ptr());
    config.client.focused_inactive.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.client.focused_inactive.indicator = draw_util_hex_to_color(c"#484e50".as_ptr());
    config.client.focused_inactive.child_border = draw_util_hex_to_color(c"#5f676a".as_ptr());
    config.client.unfocused.border = draw_util_hex_to_color(c"#333333".as_ptr());
    config.client.unfocused.background = draw_util_hex_to_color(c"#222222".as_ptr());
    config.client.unfocused.text = draw_util_hex_to_color(c"#888888".as_ptr());
    config.client.unfocused.indicator = draw_util_hex_to_color(c"#292d2e".as_ptr());
    config.client.unfocused.child_border = draw_util_hex_to_color(c"#222222".as_ptr());
    config.client.urgent.border = draw_util_hex_to_color(c"#2f343a".as_ptr());
    config.client.urgent.background = draw_util_hex_to_color(c"#900000".as_ptr());
    config.client.urgent.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.client.urgent.indicator = draw_util_hex_to_color(c"#900000".as_ptr());
    config.client.urgent.child_border = draw_util_hex_to_color(c"#900000".as_ptr());
    config.client.got_focused_tab_title = false_0 != 0;
    config.client.placeholder.border = draw_util_hex_to_color(c"#000000".as_ptr());
    config.client.placeholder.background = draw_util_hex_to_color(c"#0c0c0c".as_ptr());
    config.client.placeholder.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.client.placeholder.indicator = draw_util_hex_to_color(c"#000000".as_ptr());
    config.client.placeholder.child_border = draw_util_hex_to_color(c"#0c0c0c".as_ptr());
    config.bar.focused.border = draw_util_hex_to_color(c"#4c7899".as_ptr());
    config.bar.focused.background = draw_util_hex_to_color(c"#285577".as_ptr());
    config.bar.focused.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.bar.focused.indicator = draw_util_hex_to_color(c"#000000".as_ptr());
    config.bar.focused.child_border = draw_util_hex_to_color(c"#285577".as_ptr());
    config.bar.unfocused.border = draw_util_hex_to_color(c"#333333".as_ptr());
    config.bar.unfocused.background = draw_util_hex_to_color(c"#222222".as_ptr());
    config.bar.unfocused.text = draw_util_hex_to_color(c"#888888".as_ptr());
    config.bar.unfocused.indicator = draw_util_hex_to_color(c"#000000".as_ptr());
    config.bar.unfocused.child_border = draw_util_hex_to_color(c"#222222".as_ptr());
    config.bar.urgent.border = draw_util_hex_to_color(c"#2f343a".as_ptr());
    config.bar.urgent.background = draw_util_hex_to_color(c"#900000".as_ptr());
    config.bar.urgent.text = draw_util_hex_to_color(c"#ffffff".as_ptr());
    config.bar.urgent.indicator = draw_util_hex_to_color(c"#000000".as_ptr());
    config.bar.urgent.child_border = draw_util_hex_to_color(c"#900000".as_ptr());
    config.show_marks = true_0 != 0;
    config.default_border = BorderStyle::Normal;
    config.default_floating_border = BorderStyle::Normal;
    config.default_border_width = logical_px(2 as ::core::ffi::c_int);
    config.default_floating_border_width = logical_px(2 as ::core::ffi::c_int);
    config.default_orientation = Orientation::None;
    config.gaps.inner = 0 as ::core::ffi::c_int;
    config.gaps.top = 0 as ::core::ffi::c_int;
    config.gaps.right = 0 as ::core::ffi::c_int;
    config.gaps.bottom = 0 as ::core::ffi::c_int;
    config.gaps.left = 0 as ::core::ffi::c_int;
    if config.workspace_urgency_timer == 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        config.workspace_urgency_timer = 0.5f32;
    }
    config.focus_wrapping = FocusWrapping::On;
    config.tiling_drag = TILING_DRAG_MODIFIER;
    config.swap_modifier = XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as u32;
    free(current_configpath as *mut ::core::ffi::c_void);
    current_configpath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    current_configpath = get_config_path(override_configpath, true_0 != 0);
    if current_configpath.is_null() {
        errx(
            EXIT_FAILURE,
            c"Unable to find the configuration file (looked at $XDG_CONFIG_HOME/i3/config, ~/.i3/config, $XDG_CONFIG_DIRS/i3/config and /usr/local/etc/i3/config)".as_ptr(),
        );
    }
    let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
    while !included_files.tqh_first.is_null() {
        file = included_files.tqh_first as *mut IncludedFile;
        free((*file).path as *mut ::core::ffi::c_void);
        (*file).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*file).raw_contents as *mut ::core::ffi::c_void);
        (*file).raw_contents = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*file).variable_replaced_contents as *mut ::core::ffi::c_void);
        (*file).variable_replaced_contents = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*file).files.tqe_next.is_null() {
            (*(*file).files.tqe_next).files.tqe_prev = (*file).files.tqe_prev;
        } else {
            included_files.tqh_last = (*file).files.tqe_prev;
        }
        *(*file).files.tqe_prev = (*file).files.tqe_next;
        free(file as *mut ::core::ffi::c_void);
        file = ::core::ptr::null_mut::<IncludedFile>();
    }
    let mut resolved_path: [::core::ffi::c_char; 4096] = [0; 4096];
    if realpath(
        current_configpath,
        &raw mut resolved_path as *mut ::core::ffi::c_char,
    )
    .is_null()
    {
        errx(
            EXIT_FAILURE,
            c"realpath(%s): %s".as_ptr(),
            current_configpath,
            strerror(*__errno_location()),
        );
    }
    file =
        scalloc(1 as usize, ::core::mem::size_of::<IncludedFile>() as usize) as *mut IncludedFile;
    (*file).path = sstrdup(&raw mut resolved_path as *mut ::core::ffi::c_char);
    (*file).files.tqe_next = ::core::ptr::null_mut::<IncludedFile>();
    (*file).files.tqe_prev = included_files.tqh_last;
    *included_files.tqh_last = file as *mut IncludedFile;
    included_files.tqh_last = &raw mut (*file).files.tqe_next;
    verboselog(
        c"Parsing configfile %s\n".as_ptr().cast_mut(),
        &raw mut resolved_path as *mut ::core::ffi::c_char,
    );
    let mut stack: stack = stack::default();
    let mut ctx: parser_ctx = parser_ctx {
        use_nagbar: load_type != ConfigLoad::Validate,
        state: ConfigParserState::ForceDisplayUrgencyHintMs,
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
            matches: C2RustUnnamed_11 {
                tqe_next: ::core::ptr::null_mut::<Match>(),
                tqe_prev: ::core::ptr::null_mut::<*mut Match>(),
            },
            restart_mode: false,
        },
        statelist: Vec::new(),
        statelist_idx: 0,
        stack: &raw mut stack,
        variables: Vec::new(),
        has_errors: false,
    };
    let result: ParseFileResult = parse_file(
        &raw mut ctx,
        &raw mut resolved_path as *mut ::core::ffi::c_char,
        file,
    );
    free_variables(&raw mut ctx);
    if result == ParseFileResult::Failed {
        errx(
            EXIT_FAILURE,
            c"Could not open configuration file: %s\n".as_ptr(),
            strerror(*__errno_location()),
        );
    }
    extract_workspace_names_from_bindings();
    reorder_bindings();
    if config.font.type_0 == FontType::None && load_type != ConfigLoad::Validate {
        errorlog(
            c"ERROR: You did not specify required configuration option \"font\"\n"
                .as_ptr()
                .cast_mut(),
        );
        config.font = load_font(c"fixed".as_ptr(), true_0 != 0);
        set_font(&raw mut config.font);
    }
    if load_type != ConfigLoad::Validate {
        let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
        current = barconfigs.tqh_first as *mut Barconfig;
        while !current.is_null() {
            if (*current).font.is_null() {
                (*current).font = sstrdup(config.font.pattern);
            }
            current = (*current).configs.tqe_next as *mut Barconfig;
        }
    }
    if load_type == ConfigLoad::Reload {
        translate_keysyms();
        grab_all_keys(conn);
        regrab_all_buttons(conn);
        gaps_reapply_workspace_assignments();
        tree_render();
    }
    result == ParseFileResult::Success
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::reorder_bindings;
use crate::bindings::switch_mode;
use crate::bindings::translate_keysyms;
use crate::gaps::gaps_reapply_workspace_assignments;
use crate::tree::tree_render;
use crate::workspace::extract_workspace_names_from_bindings;
use i3_common::dpi::logical_px;
use i3_common::font::free_font;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_charinfo_t, xcb_connection_t, xcb_keycode_t, xcb_query_font_reply_t, xcb_ungrab_key,
    XCB_BUTTON_MASK_ANY, XCB_GRAB_ANY, XCB_KEY_BUT_MASK_SHIFT,
};

pub use libc::{__errno_location, free, memset, realpath, strerror};

use i3_common::native_stdio::{__assert_fail, errx};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::config_bar;
pub use crate::types::config_client;
pub use crate::types::deco_render_params;
pub use crate::types::gaps_t;
pub use crate::types::Assignment;
pub use crate::types::Colortriple;
pub use crate::types::Con;
pub use crate::types::Config;
pub use crate::types::Match;
pub use crate::types::MatchMatches as C2RustUnnamed_11;
pub use i3_common::draw_util::color_t;
pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::current_binding_mode;
use crate::application::root;
use crate::bindings::grab_all_keys;
use crate::bindings::regrab_all_buttons;
use crate::r#match::match_free;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::font::load_font;
use i3_common::font::set_font;
use i3_common::get_config_path::get_config_path;
