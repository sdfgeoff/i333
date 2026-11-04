use crate::application::assignments;
use crate::application::bindings;
use crate::application::ws_assignments;
use crate::bindings::binding_free;
use crate::con::arena::{containers, resolve};
use crate::config_parser::free_variables;
use crate::config_parser::parse_file;
use crate::config_parser::ConfigParserState;
use crate::domains::{DockMatch, HideEdgeBordersMode, MatchInsertWhere};
use crate::flags::TilingDrag;
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
use std::ffi::CStr;
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

pub use crate::application::bindings_head;
pub use crate::application::Workspace_Assignment;
/// # Safety
/// The initialized assignment queue is exclusively owned by this event thread.
/// No concurrent or reentrant queue mutation may occur while linking the Box.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn append_workspace_assignment(
    value: Workspace_Assignment,
) -> *mut Workspace_Assignment {
    let node = Box::into_raw(Box::new(value));
    // SAFETY: the queue's predecessor slot remains live during this append.
    unsafe {
        (*node).ws_assignments.tqe_next = std::ptr::null_mut();
        (*node).ws_assignments.tqe_prev = ws_assignments.tqh_last;
        *ws_assignments.tqh_last = node;
        ws_assignments.tqh_last = &raw mut (*node).ws_assignments.tqe_next;
    }
    node
}
/// # Safety
/// node is a live, uniquely owned linked Box; queue access is event-thread
/// exclusive with no concurrent/reentrant mutation. Unlink precedes owner Drop.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn remove_workspace_assignment(node: *mut Workspace_Assignment) {
    // SAFETY: neighboring link slots remain live until the unlink completes.
    unsafe {
        if !(*node).ws_assignments.tqe_next.is_null() {
            (*(*node).ws_assignments.tqe_next).ws_assignments.tqe_prev =
                (*node).ws_assignments.tqe_prev;
        } else {
            ws_assignments.tqh_last = (*node).ws_assignments.tqe_prev;
        }
        *(*node).ws_assignments.tqe_prev = (*node).ws_assignments.tqe_next;
        drop(Box::from_raw(node));
    }
}

pub type tiling_drag_t = TilingDrag;
pub struct IncludedFile {
    pub path: std::ffi::CString,
    /// None means reading has not started; Some retains the zeroed stat-sized
    /// buffer and trailing NUL even when fread returns a partial prefix.
    pub raw_contents: Option<Vec<u8>>,
    pub variable_replaced_contents: Option<std::ffi::CString>,
    pub files: IncludedFileLinks,
}
impl IncludedFile {
    pub fn new(path: std::ffi::CString) -> Self {
        Self {
            path,
            raw_contents: None,
            variable_replaced_contents: None,
            files: IncludedFileLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        }
    }
    /// IPC historically observes only the first C-string prefix; retain the
    /// complete file bytes in owned storage until this explicit projection.
    pub fn raw_contents_prefix(&self) -> Option<&[u8]> {
        self.raw_contents.as_deref().map(|bytes| {
            let end = bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len());
            &bytes[..end]
        })
    }
}

/// Link a uniquely owned file node into the existing intrusive queue.
/// # Safety
/// The event-thread queue is initialized and exclusively accessed. No concurrent
/// or reentrant queue mutation may occur; nodes stay live throughout parsing.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn append_included_file(path: std::ffi::CString) -> *mut IncludedFile {
    let file = Box::into_raw(Box::new(IncludedFile::new(path)));
    // SAFETY: exclusive initialized queue access keeps its link slots live.
    unsafe {
        (*file).files.tqe_prev = included_files.tqh_last;
        *included_files.tqh_last = file;
        included_files.tqh_last = &raw mut (*file).files.tqe_next;
    }
    file
}
/// Unlink the live Box before dropping its owned path and content buffers.
/// # Safety
/// `file` is a live, uniquely owned linked Box created by append_included_file.
/// Queue access is event-thread exclusive, without concurrent/reentrant mutation.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn remove_included_file(file: *mut IncludedFile) {
    // SAFETY: all neighboring link slots remain live until this unlink finishes.
    unsafe {
        if !(*file).files.tqe_next.is_null() {
            (*(*file).files.tqe_next).files.tqe_prev = (*file).files.tqe_prev;
        } else {
            included_files.tqh_last = (*file).files.tqe_prev;
        }
        *(*file).files.tqe_prev = (*file).files.tqe_next;
        drop(Box::from_raw(file));
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IncludedFileLinks {
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut current_configpath: *mut ::core::ffi::c_char =
    std::ptr::null_mut::<::core::ffi::c_char>();

const fn empty_config() -> Config {
    Config {
        terminal: None,
        font: Font::empty(),
        ipc_socket_path: None,
        restart_state_path: None,
        default_layout: Layout::Default,
        container_stack_limit: 0,
        container_stack_limit_value: 0,
        default_border_width: 0,
        default_floating_border_width: 0,
        default_orientation: Orientation::None,
        disable_focus_follows_mouse: false,
        mouse_warping: Warping::Output,
        hide_edge_borders: HideEdgeBordersMode::None,
        disable_workspace_bar: false,
        focus_wrapping: FocusWrapping::Off,
        force_xinerama: false,
        disable_randr15: false,
        fake_outputs: None,
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
        tiling_drag: TilingDrag::empty(),
        gaps: gaps_t {
            inner: 0,
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        },
        smart_gaps: SmartGaps::Off,
    }
}
pub static mut config: Config = empty_config();

pub static mut modes: modes_head = modes_head {
    slh_first: std::ptr::null_mut::<Mode>(),
};

pub static mut barconfigs: barconfig_head = barconfig_head {
    tqh_first: std::ptr::null_mut(),
    tqh_last: unsafe { &raw mut barconfigs.tqh_first },
};

pub static mut included_files: includedfiles_head = includedfiles_head {
    tqh_first: std::ptr::null_mut(),
    tqh_last: unsafe { &raw mut included_files.tqh_first },
};

pub unsafe fn ungrab_all_keys() {
    ::log::debug!("../src/config.c:ungrab_all_keys:28 - Ungrabbing all keys\n");
    i3_common::display::connection()
        .expect("main X11 connection is registered")
        .send_request(&xcb::x::UngrabKey {
            key: 0,
            grab_window: <xcb::x::Window as xcb::XidNew>::new(root),
            modifiers: xcb::x::ModMask::ANY,
        });
}
unsafe fn free_configuration() {
    if !conn.is_null() {
    } else {
        __assert_fail(
            c"conn != NULL".as_ptr(),
            c"../src/config.c".as_ptr(),
            33 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    switch_mode(b"default");
    ungrab_all_keys();
    while !modes.slh_first.is_null() {
        crate::bindings::remove_first_mode();
    }
    while !assignments.tqh_first.is_null() {
        let mut assign: *mut Assignment = assignments.tqh_first;
        if !(*assign).assignments.tqe_next.is_null() {
            (*(*assign).assignments.tqe_next).assignments.tqe_prev = (*assign).assignments.tqe_prev;
        } else {
            assignments.tqh_last = (*assign).assignments.tqe_prev;
        }
        *(*assign).assignments.tqe_prev = (*assign).assignments.tqe_next;
        drop(Box::from_raw(assign));
        assign = ::core::ptr::null_mut::<Assignment>();
    }
    while !ws_assignments.tqh_first.is_null() {
        remove_workspace_assignment(ws_assignments.tqh_first);
    }
    while !barconfigs.tqh_first.is_null() {
        let mut barconfig: *mut Barconfig = barconfigs.tqh_first;
        while !(*barconfig).bar_bindings.tqh_first.is_null() {
            let mut binding: *mut Barbinding = (*barconfig).bar_bindings.tqh_first;
            if !(*binding).bindings.tqe_next.is_null() {
                (*(*binding).bindings.tqe_next).bindings.tqe_prev = (*binding).bindings.tqe_prev;
            } else {
                (*barconfig).bar_bindings.tqh_last = (*binding).bindings.tqe_prev;
            }
            *(*binding).bindings.tqe_prev = (*binding).bindings.tqe_next;
            drop(Box::from_raw(binding));
            binding = ::core::ptr::null_mut::<Barbinding>();
        }
        while !(*barconfig).tray_outputs.tqh_first.is_null() {
            let mut tray_output: *mut tray_output_t = (*barconfig).tray_outputs.tqh_first;
            if !(*tray_output).tray_outputs.tqe_next.is_null() {
                (*(*tray_output).tray_outputs.tqe_next)
                    .tray_outputs
                    .tqe_prev = (*tray_output).tray_outputs.tqe_prev;
            } else {
                (*barconfig).tray_outputs.tqh_last = (*tray_output).tray_outputs.tqe_prev;
            }
            *(*tray_output).tray_outputs.tqe_prev = (*tray_output).tray_outputs.tqe_next;
            drop(Box::from_raw(tray_output));
            tray_output = ::core::ptr::null_mut::<tray_output_t>();
        }
        if !(*barconfig).configs.tqe_next.is_null() {
            (*(*barconfig).configs.tqe_next).configs.tqe_prev = (*barconfig).configs.tqe_prev;
        } else {
            barconfigs.tqh_last = (*barconfig).configs.tqe_prev;
        }
        *(*barconfig).configs.tqe_prev = (*barconfig).configs.tqe_next;
        drop(Box::from_raw(barconfig));
        barconfig = ::core::ptr::null_mut::<Barconfig>();
    }
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        if !(*con).window.is_null() {
            (*(*con).window).ran_assignments.clear();
        }
        free((*con).deco_render_params as *mut ::core::ffi::c_void);
        (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    }
    config.font = Font::empty();
    config.terminal = None;
    config.ipc_socket_path = None;
    config.restart_state_path = None;
    config.fake_outputs = None;
}

/// Configuration owns its font; validation without X11 keeps an empty value.
/// # Safety
/// Requires exclusive event-thread access to configuration and its live display.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn set_config_font(pattern: &CStr) {
    // SAFETY: caller serializes the display/configuration globals.
    if unsafe { crate::application::conn.is_null() } {
        unsafe { config.font = Font::empty() };
        return;
    }
    let context =
        i3_common::font::FontContext::for_screen(unsafe { crate::application::root_display() });
    unsafe { i3_common::font::replace_font(&mut config.font, &context, pattern, true) };
}

pub unsafe fn load_configuration(
    override_configpath: *const ::core::ffi::c_char,
    load_type: ConfigLoad,
) -> bool {
    if load_type == ConfigLoad::Reload {
        free_configuration();
    }
    modes.slh_first = ::core::ptr::null_mut::<Mode>();
    let default_mode = Box::into_raw(Box::new(Mode::new(b"default".to_vec(), false)));
    (*default_mode).modes.sle_next = modes.slh_first;
    modes.slh_first = default_mode;
    bindings = (*default_mode).bindings_ptr();
    current_binding_mode = Some((*default_mode).name.clone());
    config = empty_config();
    {
        let screen = crate::application::root_display_opt();
        config.client.background = draw_util_hex_to_color(screen, "#000000");
        config.client.focused.border = draw_util_hex_to_color(screen, "#4c7899");
        config.client.focused.background = draw_util_hex_to_color(screen, "#285577");
        config.client.focused.text = draw_util_hex_to_color(screen, "#ffffff");
        config.client.focused.indicator = draw_util_hex_to_color(screen, "#2e9ef4");
        config.client.focused.child_border = draw_util_hex_to_color(screen, "#285577");
        config.client.focused_inactive.border = draw_util_hex_to_color(screen, "#333333");
        config.client.focused_inactive.background = draw_util_hex_to_color(screen, "#5f676a");
        config.client.focused_inactive.text = draw_util_hex_to_color(screen, "#ffffff");
        config.client.focused_inactive.indicator = draw_util_hex_to_color(screen, "#484e50");
        config.client.focused_inactive.child_border = draw_util_hex_to_color(screen, "#5f676a");
        config.client.unfocused.border = draw_util_hex_to_color(screen, "#333333");
        config.client.unfocused.background = draw_util_hex_to_color(screen, "#222222");
        config.client.unfocused.text = draw_util_hex_to_color(screen, "#888888");
        config.client.unfocused.indicator = draw_util_hex_to_color(screen, "#292d2e");
        config.client.unfocused.child_border = draw_util_hex_to_color(screen, "#222222");
        config.client.urgent.border = draw_util_hex_to_color(screen, "#2f343a");
        config.client.urgent.background = draw_util_hex_to_color(screen, "#900000");
        config.client.urgent.text = draw_util_hex_to_color(screen, "#ffffff");
        config.client.urgent.indicator = draw_util_hex_to_color(screen, "#900000");
        config.client.urgent.child_border = draw_util_hex_to_color(screen, "#900000");
        config.client.got_focused_tab_title = false;
        config.client.placeholder.border = draw_util_hex_to_color(screen, "#000000");
        config.client.placeholder.background = draw_util_hex_to_color(screen, "#0c0c0c");
        config.client.placeholder.text = draw_util_hex_to_color(screen, "#ffffff");
        config.client.placeholder.indicator = draw_util_hex_to_color(screen, "#000000");
        config.client.placeholder.child_border = draw_util_hex_to_color(screen, "#0c0c0c");
        config.bar.focused.border = draw_util_hex_to_color(screen, "#4c7899");
        config.bar.focused.background = draw_util_hex_to_color(screen, "#285577");
        config.bar.focused.text = draw_util_hex_to_color(screen, "#ffffff");
        config.bar.focused.indicator = draw_util_hex_to_color(screen, "#000000");
        config.bar.focused.child_border = draw_util_hex_to_color(screen, "#285577");
        config.bar.unfocused.border = draw_util_hex_to_color(screen, "#333333");
        config.bar.unfocused.background = draw_util_hex_to_color(screen, "#222222");
        config.bar.unfocused.text = draw_util_hex_to_color(screen, "#888888");
        config.bar.unfocused.indicator = draw_util_hex_to_color(screen, "#000000");
        config.bar.unfocused.child_border = draw_util_hex_to_color(screen, "#222222");
        config.bar.urgent.border = draw_util_hex_to_color(screen, "#2f343a");
        config.bar.urgent.background = draw_util_hex_to_color(screen, "#900000");
        config.bar.urgent.text = draw_util_hex_to_color(screen, "#ffffff");
        config.bar.urgent.indicator = draw_util_hex_to_color(screen, "#000000");
        config.bar.urgent.child_border = draw_util_hex_to_color(screen, "#900000");
    }
    config.show_marks = true;
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
    config.tiling_drag = TilingDrag::MODIFIER;
    config.swap_modifier = (::xcb::x::KeyButMask::SHIFT.bits()) as ::core::ffi::c_int as u32;
    free(current_configpath as *mut ::core::ffi::c_void);
    current_configpath = ::core::ptr::null_mut::<::core::ffi::c_char>();
    current_configpath = get_config_path(override_configpath, true);
    if current_configpath.is_null() {
        errx(
            EXIT_FAILURE,
            c"Unable to find the configuration file (looked at $XDG_CONFIG_HOME/i3/config, ~/.i3/config, $XDG_CONFIG_DIRS/i3/config and /usr/local/etc/i3/config)".as_ptr(),
        );
    }
    while !included_files.tqh_first.is_null() {
        remove_included_file(included_files.tqh_first);
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
    let file = append_included_file(CStr::from_ptr(resolved_path.as_ptr()).to_owned());
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = &raw mut resolved_path as *mut ::core::ffi::c_char;
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
            ::log::info!("Parsing configfile {}\n", log_value_0_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice("Parsing configfile ".as_bytes());
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Verbose, &message);
        }
    };
    let mut stack: stack = stack::default();
    let mut ctx: parser_ctx = parser_ctx {
        use_nagbar: load_type != ConfigLoad::Validate,
        state: ConfigParserState::ForceDisplayUrgencyHintMs,
        current_match: Match::empty(),
        statelist: Vec::new(),
        statelist_idx: 0,
        stack: &raw mut stack,
        variables: Vec::new(),
        has_errors: false,
    };
    let result: ParseFileResult =
        parse_file(&raw mut ctx, CStr::from_ptr(resolved_path.as_ptr()), file);
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
    if !config.font.is_loaded() && load_type != ConfigLoad::Validate {
        ::log::error!("ERROR: You did not specify required configuration option \"font\"\n");
        set_config_font(c"fixed");
    }
    if load_type != ConfigLoad::Validate {
        let mut current: *mut Barconfig = barconfigs.tqh_first;

        while !current.is_null() {
            if (*current).font.is_none() {
                (*current).font = config.font.pattern().map(CStr::to_owned);
            }
            current = (*current).configs.tqe_next;
        }
    }
    if load_type == ConfigLoad::Reload {
        translate_keysyms();
        grab_all_keys();
        regrab_all_buttons();
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
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb::ffi::xcb_connection_t;

pub use libc::{__errno_location, free, realpath, strerror};

use i3_common::native_stdio::{__assert_fail, errx};

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
pub use i3_common::draw_util::color_t;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::current_binding_mode;
use crate::application::root;
use crate::bindings::grab_all_keys;
use crate::bindings::regrab_all_buttons;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::get_config_path::get_config_path;

#[cfg(test)]
mod included_file_tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt;
    #[test]
    fn failed_include_unlinks_owned_box_without_losing_other_nodes_or_path_snapshots() {
        if std::env::var_os("I333_INCLUDED_QUEUE_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "config::included_file_tests::failed_include_unlinks_owned_box_without_losing_other_nodes_or_path_snapshots", "--nocapture"])
                .env("I333_INCLUDED_QUEUE_PROBE", "1")
                .env_remove("DISPLAY").env_remove("I3SOCK").env_remove("WAYLAND_DISPLAY")
                .output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../.work/included-file-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        std::fs::create_dir_all(&directory).unwrap();
        struct Remove(std::path::PathBuf);
        impl Drop for Remove {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let _directory = Remove(directory.clone());
        let mut bytes = directory.as_os_str().as_bytes().to_vec();
        bytes.extend_from_slice(b"/missing-\xff");
        let missing_path = std::ffi::CString::new(bytes).unwrap();
        unsafe {
            assert!(included_files.tqh_first.is_null());
            let first = append_included_file(c"/first".to_owned());
            let failed = append_included_file(missing_path.clone());
            let last = append_included_file(c"/last".to_owned());
            let filename_snapshot = (*failed).path.clone();
            let mut stack = crate::config_parser::stack::default();
            let mut ctx = parser_ctx {
                use_nagbar: false,
                state: ConfigParserState::Initial,
                current_match: Match::empty(),
                statelist: Vec::new(),
                statelist_idx: 0,
                stack: &mut stack,
                variables: Vec::new(),
                has_errors: false,
            };
            assert_eq!(
                parse_file(&mut ctx, &filename_snapshot, failed),
                ParseFileResult::Failed
            );
            assert!((*failed).raw_contents.is_none());
            assert!((*failed).variable_replaced_contents.is_none());
            remove_included_file(failed);
            assert_eq!(filename_snapshot, missing_path);
            assert_eq!(included_files.tqh_first, first);
            assert_eq!((*first).files.tqe_next, last);
            assert_eq!((*last).files.tqe_prev, &raw mut (*first).files.tqe_next);
            assert_eq!(included_files.tqh_last, &raw mut (*last).files.tqe_next);
            remove_included_file(first);
            assert_eq!(included_files.tqh_first, last);
            assert_eq!((*last).files.tqe_prev, &raw mut included_files.tqh_first);
            remove_included_file(last);
            assert!(included_files.tqh_first.is_null());
            assert_eq!(included_files.tqh_last, &raw mut included_files.tqh_first);
        }
    }
}

#[cfg(test)]
mod workspace_assignment_tests {
    use super::*;
    use crate::flags::GapMask;
    use crate::types::{
        names_head, output_name, xoutput, Con, OutputNameNames, Rect, XoutputOutputs,
    };
    #[test]
    fn owned_assignments_preserve_routing_null_empty_and_gap_precedence() {
        if std::env::var_os("I333_WORKSPACE_ASSIGNMENT_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "config::workspace_assignment_tests::owned_assignments_preserve_routing_null_empty_and_gap_precedence", "--nocapture"])
                .env("I333_WORKSPACE_ASSIGNMENT_PROBE", "1")
                .env_remove("DISPLAY").env_remove("I3SOCK").env_remove("WAYLAND_DISPLAY")
                .output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        fn output(id: u32, name: &mut output_name, con: *mut Con) -> Box<xoutput> {
            Box::new(xoutput {
                id,
                active: true,
                changed: false,
                to_be_disabled: false,
                primary: id == 1,
                names_head: names_head { slh_first: name },
                con,
                rect: Rect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1,
                },
                outputs: XoutputOutputs {
                    tqe_next: std::ptr::null_mut(),
                    tqe_prev: std::ptr::null_mut(),
                },
            })
        }
        let gaps = |value| gaps_t {
            inner: value,
            top: value,
            right: value,
            bottom: value,
            left: value,
        };
        let label_a = c"OUT-A".to_owned();
        let label_b = c"out-\xff".to_owned();
        let mut name_a = Box::new(output_name {
            name: label_a.as_ptr().cast_mut(),
            names: OutputNameNames {
                sle_next: std::ptr::null_mut(),
            },
        });
        let mut name_b = Box::new(output_name {
            name: label_b.as_ptr().cast_mut(),
            names: OutputNameNames {
                sle_next: std::ptr::null_mut(),
            },
        });
        let mut con_a = Box::new(Con::empty());
        let mut con_b = Box::new(Con::empty());
        let mut a = output(1, &mut name_a, &mut *con_a);
        let mut b = output(2, &mut name_b, &mut *con_b);
        i3_common::logging::set_sink(|_, _| {});
        unsafe {
            assert!(ws_assignments.tqh_first.is_null());
            ws_assignments.tqh_last = &raw mut ws_assignments.tqh_first;
            // Error recovery returns before dereferencing missing output values.
            crate::config_directives::cfg_workspace(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert!(ws_assignments.tqh_first.is_null());
            crate::randr::outputs.tqh_first = &mut *a;
            crate::randr::outputs.tqh_last = &raw mut b.outputs.tqe_next;
            a.outputs.tqe_prev = &raw mut crate::randr::outputs.tqh_first;
            a.outputs.tqe_next = &mut *b;
            b.outputs.tqe_prev = &raw mut a.outputs.tqe_next;
            let gaps_only =
                append_workspace_assignment(Workspace_Assignment::new(c"1".to_owned(), None));
            let first = append_workspace_assignment(Workspace_Assignment::new(
                c"1".to_owned(),
                Some(c"out-a".to_owned()),
            ));
            append_workspace_assignment(Workspace_Assignment::new(
                c"1:dev".to_owned(),
                Some(label_b.clone()),
            ));
            append_workspace_assignment(Workspace_Assignment::new(
                c"01".to_owned(),
                Some(label_b.clone()),
            ));
            let empty_output = append_workspace_assignment(Workspace_Assignment::new(
                c"empty".to_owned(),
                Some(c"".to_owned()),
            ));
            let invalid = append_workspace_assignment(Workspace_Assignment::new(
                c"work-\xff".to_owned(),
                Some(label_b.clone()),
            ));
            assert!((*gaps_only).output.is_none());
            assert_eq!((*empty_output).output.as_deref(), Some(c""));
            let saved_name = (*invalid).name.clone();
            let saved_output = (*invalid).output.clone();
            assert_eq!(
                crate::workspace::get_assigned_output(c"1:dev".as_ptr(), 1),
                &raw mut *con_b
            );
            assert_eq!(
                crate::workspace::get_assigned_output(c"1:Dev".as_ptr(), 1),
                &raw mut *con_a
            );
            assert_eq!(
                crate::workspace::get_assigned_output(std::ptr::null(), 1),
                &raw mut *con_a
            );
            assert_eq!(
                crate::workspace::get_assigned_output(c"work-\xff".as_ptr(), -1),
                &raw mut *con_b
            );
            assert!(crate::workspace::get_assigned_output(c"empty".as_ptr(), -1).is_null());
            assert!(crate::workspace::output_triggers_assignment(
                &mut *a, gaps_only
            ));
            // Some(empty) still counts as a duplicate output assignment.
            let tail = ws_assignments.tqh_last;
            crate::config_directives::cfg_workspace(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                c"empty".as_ptr(),
                std::ptr::null(),
            );
            assert_eq!(ws_assignments.tqh_last, tail);
            let successor = (*first).ws_assignments.tqe_next;
            remove_workspace_assignment(first);
            assert_eq!((*gaps_only).ws_assignments.tqe_next, successor);
            assert_eq!(
                (*successor).ws_assignments.tqe_prev,
                &raw mut (*gaps_only).ws_assignments.tqe_next
            );
            while !ws_assignments.tqh_first.is_null() {
                remove_workspace_assignment(ws_assignments.tqh_first);
            }
            assert_eq!(ws_assignments.tqh_last, &raw mut ws_assignments.tqh_first);
            assert_eq!(saved_name.as_bytes(), b"work-\xff");
            assert_eq!(saved_output.as_deref(), Some(label_b.as_c_str()));
            // Exercise the actual queue adapter, not only the pure gap reducer.
            config.gaps = gaps_t {
                inner: 1,
                top: 2,
                right: 3,
                bottom: 4,
                left: 5,
            };
            let numeric =
                append_workspace_assignment(Workspace_Assignment::new(c"1".to_owned(), None));
            (*numeric).gaps = gaps(10);
            (*numeric).gaps_mask = GapMask::TOP;
            let numeric =
                append_workspace_assignment(Workspace_Assignment::new(c"01".to_owned(), None));
            (*numeric).gaps = gaps(20);
            (*numeric).gaps_mask = GapMask::LEFT;
            let mut workspace = Con::empty();
            workspace.name = Some(c"1:dev".to_owned());
            workspace.num = 1;
            let actual = crate::gaps::gaps_for_workspace(&mut workspace);
            assert_eq!(
                [
                    actual.inner,
                    actual.top,
                    actual.right,
                    actual.bottom,
                    actual.left
                ],
                [20, 20, 20, 20, 15]
            );
            let exact =
                append_workspace_assignment(Workspace_Assignment::new(c"1:dev".to_owned(), None));
            (*exact).gaps = gaps(30);
            (*exact).gaps_mask = GapMask::INNER | GapMask::BOTTOM;
            let actual = crate::gaps::gaps_for_workspace(&mut workspace);
            assert_eq!(
                [
                    actual.inner,
                    actual.top,
                    actual.right,
                    actual.bottom,
                    actual.left
                ],
                [29, 30, 30, 26, 30]
            );
            let later =
                append_workspace_assignment(Workspace_Assignment::new(c"1:dev".to_owned(), None));
            (*later).gaps = gaps(99);
            let actual = crate::gaps::gaps_for_workspace(&mut workspace);
            assert_eq!(actual.inner, 29);
            while !ws_assignments.tqh_first.is_null() {
                remove_workspace_assignment(ws_assignments.tqh_first);
            }
            crate::randr::outputs.tqh_first = std::ptr::null_mut();
            crate::randr::outputs.tqh_last = &raw mut crate::randr::outputs.tqh_first;
        }
    }
}
