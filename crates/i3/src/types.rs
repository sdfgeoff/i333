// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Shared event-thread model records and native layout-compatible values.
//!
//! Containers and windows have unique Rust owners and must not be copied or
//! zero-initialized. Remaining `Copy` records contain borrowed links/native
//! handles; copying those values does not transfer resource ownership. Graph
//! mutation/freeing belongs to the event-thread owner modules. No whole-record
//! Rust borrow may span reentrant callbacks. Legacy zero-filled POD constructors
//! require valid enum discriminants; their current defaults are zero. Open
//! integer domains remain integers until all producers migrate together.

/// The layout of a container or the default layout for a workspace.
///
/// Zero is a valid default because legacy model allocation still zeroes storage.
/// Keep the existing representation while these models use C-compatible layouts.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Layout {
    #[default]
    Default = 0,
    Stacked = 1,
    Tabbed = 2,
    DockArea = 3,
    Output = 4,
    SplitV = 5,
    SplitH = 6,
}

// Closed application domains retain their original storage while legacy models are zero-allocated.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum BorderStyle {
    #[default]
    None = 0,
    Pixel = 1,
    Normal = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Orientation {
    #[default]
    None = 0,
    Horizontal = 1,
    Vertical = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Direction {
    #[default]
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Position {
    #[default]
    Before = 0,
    After = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum InputType {
    #[default]
    Keyboard = 0,
    Mouse = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ConfigLoad {
    #[default]
    Validate = 0,
    Load = 1,
    Reload = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ShutdownReason {
    #[default]
    Restart = 0,
    Exit = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum KillWindow {
    #[default]
    DontKill = 0,
    Window = 1,
    Client = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum MarkMode {
    #[default]
    Replace = 0,
    Add = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ClickDestination {
    #[default]
    Border = 0,
    Decoration = 1,
    Inside = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum OutputSelection {
    #[default]
    Closest = 0,
    Farthest = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum DragResult {
    #[default]
    Dragging = 0,
    Success = 1,
    Revert = 2,
    Abort = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum DropType {
    #[default]
    Sibling = 0,
    Center = 1,
    Parent = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum JsonContent {
    #[default]
    Unknown = 0,
    Container = 1,
    Workspace = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(i32)]
pub enum ParseFileResult {
    Failed = -1,
    #[default]
    Success = 0,
    ConfigErrors = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum SmartGaps {
    #[default]
    Off = 0,
    On = 1,
    InverseOuter = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum Warping {
    #[default]
    Output = 0,
    None = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum FocusWrapping {
    #[default]
    Off = 0,
    On = 1,
    Force = 2,
    Workspace = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum BindingRelease {
    #[default]
    KeyPress = 0,
    KeyRelease = 1,
    KeyReleaseIgnoreMods = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum ScratchpadState {
    #[default]
    None = 0,
    Fresh = 1,
    Changed = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum FloatingState {
    #[default]
    AutoOff = 0,
    UserOff = 1,
    AutoOn = 2,
    UserOn = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum WindowMode {
    #[default]
    Any = 0,
    TilingAuto = 1,
    TilingUser = 2,
    Tiling = 3,
    FloatingAuto = 4,
    FloatingUser = 5,
    Floating = 6,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(i32)]
pub enum UrgencyMatch {
    DontCheck = -1,
    #[default]
    Latest = 0,
    Oldest = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum WindowDock {
    #[default]
    None = 0,
    Top = 1,
    Bottom = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum PopupDuringFullscreen {
    #[default]
    Smart = 0,
    LeaveFullscreen = 1,
    Ignore = 2,
    All = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum TitleAlign {
    #[default]
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum FocusActivation {
    #[default]
    Smart = 0,
    Urgent = 1,
    Focus = 2,
    None = 3,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum BarMode {
    #[default]
    Dock = 0,
    Hide = 1,
    Invisible = 2,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum BarHiddenState {
    #[default]
    Hide = 0,
    Show = 1,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum BarPosition {
    #[default]
    Bottom = 0,
    Top = 1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Assignment {
    pub type_0: ::core::ffi::c_uint,
    pub match_0: crate::types::Match,
    pub dest: crate::types::AssignmentDest,
    pub assignments: crate::types::AssignmentAssignments,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AssignmentAssignments {
    pub tqe_next: *mut crate::types::Assignment,
    pub tqe_prev: *mut *mut crate::types::Assignment,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union AssignmentDest {
    pub command: *mut ::core::ffi::c_char,
    pub workspace: *mut ::core::ffi::c_char,
    pub output: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colortriple {
    pub border: i3_common::draw_util::color_t,
    pub background: i3_common::draw_util::color_t,
    pub text: i3_common::draw_util::color_t,
    pub indicator: i3_common::draw_util::color_t,
    pub child_border: i3_common::draw_util::color_t,
}

/// Event-thread container, owned by the stable container arena.
/// Links are borrowed identities; copying an entire container is invalid.
#[repr(C)]
pub struct Con {
    pub mapped: bool,
    pub urgent: bool,
    pub ignore_unmap: u8,
    pub frame: i3_common::draw_util::surface_t,
    pub frame_buffer: i3_common::draw_util::surface_t,
    pub pixmap_recreated: bool,
    pub type_0: ::core::ffi::c_uint,
    pub num: ::core::ffi::c_int,
    pub gaps: crate::types::gaps_t,
    pub parent: *mut crate::types::Con,
    pub rect: crate::types::Rect,
    pub window_rect: crate::types::Rect,
    pub deco_rect: crate::types::Rect,
    pub geometry: crate::types::Rect,
    pub name: *mut ::core::ffi::c_char,
    pub title_format: *mut ::core::ffi::c_char,
    pub window_icon_padding: ::core::ffi::c_int,
    pub sticky_group: *mut ::core::ffi::c_char,
    pub marks_head: crate::types::marks_head,
    pub mark_changed: bool,
    pub percent: ::core::ffi::c_double,
    pub border_width: ::core::ffi::c_int,
    pub current_border_width: ::core::ffi::c_int,
    pub window: *mut crate::types::Window,
    pub urgency_timer: *mut libev_sys::ev_timer,
    pub deco_render_params: *mut crate::types::deco_render_params,
    pub floating_head: crate::types::floating_head,
    pub nodes_head: crate::types::nodes_head,
    pub focus_head: crate::types::focus_head,
    pub swallow_head: crate::types::swallow_head,
    pub fullscreen_mode: ::core::ffi::c_uint,
    pub sticky: bool,
    pub layout: Layout,
    pub last_split_layout: Layout,
    pub workspace_layout: Layout,
    pub border_style: BorderStyle,
    pub max_user_border_style: BorderStyle,
    pub floating: FloatingState,
    pub nodes: crate::types::ConNodes,
    pub focused: crate::types::ConFocused,
    pub all_cons: crate::types::ConAllCons,
    pub floating_windows: crate::types::ConFloatingWindows,
    pub on_remove_child: Option<unsafe fn(*mut crate::types::Con) -> ()>,
    pub scratchpad_state: ScratchpadState,
    pub old_id: ::core::ffi::c_int,
    pub depth: u16,
    pub colormap: xcb_sys::xcb_colormap_t,
}

impl Con {
    /// Construct detached storage without assuming that Rust owners are zeroable.
    /// Self-referential queue tails are installed only after stable allocation.
    pub fn empty() -> Self {
        use std::ptr::null_mut;
        let rect = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let surface = i3_common::draw_util::surface_t {
            id: 0,
            gc: 0,
            owns_gc: false,
            width: 0,
            height: 0,
            surface: null_mut(),
            cr: null_mut(),
        };
        Self {
            mapped: false,
            urgent: false,
            ignore_unmap: 0,
            frame: surface,
            frame_buffer: surface,
            pixmap_recreated: false,
            type_0: 0,
            num: 0,
            gaps: gaps_t {
                inner: 0,
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            },
            parent: null_mut(),
            rect,
            window_rect: rect,
            deco_rect: rect,
            geometry: rect,
            name: null_mut(),
            title_format: null_mut(),
            window_icon_padding: 0,
            sticky_group: null_mut(),
            marks_head: marks_head {
                tqh_first: null_mut(),
                tqh_last: null_mut(),
            },
            mark_changed: false,
            percent: 0.0,
            border_width: 0,
            current_border_width: 0,
            window: null_mut(),
            urgency_timer: null_mut(),
            deco_render_params: null_mut(),
            floating_head: floating_head {
                tqh_first: null_mut(),
                tqh_last: null_mut(),
            },
            nodes_head: nodes_head {
                tqh_first: null_mut(),
                tqh_last: null_mut(),
            },
            focus_head: focus_head {
                tqh_first: null_mut(),
                tqh_last: null_mut(),
            },
            swallow_head: swallow_head {
                tqh_first: null_mut(),
                tqh_last: null_mut(),
            },
            fullscreen_mode: 0,
            sticky: false,
            layout: Layout::Default,
            last_split_layout: Layout::Default,
            workspace_layout: Layout::Default,
            border_style: BorderStyle::None,
            max_user_border_style: BorderStyle::None,
            floating: FloatingState::default(),
            nodes: ConNodes {
                tqe_next: null_mut(),
                tqe_prev: null_mut(),
            },
            focused: ConFocused {
                tqe_next: null_mut(),
                tqe_prev: null_mut(),
            },
            all_cons: ConAllCons {
                tqe_next: null_mut(),
                tqe_prev: null_mut(),
            },
            floating_windows: ConFloatingWindows {
                tqe_next: null_mut(),
                tqe_prev: null_mut(),
            },
            on_remove_child: None,
            scratchpad_state: ScratchpadState::default(),
            old_id: 0,
            depth: 0,
            colormap: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConAllCons {
    pub tqe_next: *mut crate::types::Con,
    pub tqe_prev: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConFloatingWindows {
    pub tqe_next: *mut crate::types::Con,
    pub tqe_prev: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConFocused {
    pub tqe_next: *mut crate::types::Con,
    pub tqe_prev: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConNodes {
    pub tqe_next: *mut crate::types::Con,
    pub tqe_prev: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Config {
    pub terminal: *const ::core::ffi::c_char,
    pub font: i3_common::font::Font,
    pub ipc_socket_path: *mut ::core::ffi::c_char,
    pub restart_state_path: *mut ::core::ffi::c_char,
    pub default_layout: Layout,
    pub container_stack_limit: ::core::ffi::c_int,
    pub container_stack_limit_value: ::core::ffi::c_int,
    pub default_border_width: ::core::ffi::c_int,
    pub default_floating_border_width: ::core::ffi::c_int,
    pub default_orientation: Orientation,
    pub disable_focus_follows_mouse: bool,
    pub mouse_warping: Warping,
    pub hide_edge_borders: ::core::ffi::c_uint,
    pub disable_workspace_bar: bool,
    pub focus_wrapping: FocusWrapping,
    pub force_xinerama: bool,
    pub disable_randr15: bool,
    pub fake_outputs: *mut ::core::ffi::c_char,
    pub workspace_auto_back_and_forth: bool,
    pub workspace_urgency_timer: ::core::ffi::c_float,
    pub focus_on_window_activation: FocusActivation,
    pub show_marks: bool,
    pub title_align: TitleAlign,
    pub default_border: BorderStyle,
    pub default_floating_border: BorderStyle,
    pub floating_modifier: u32,
    pub swap_modifier: u32,
    pub floating_maximum_width: i32,
    pub floating_maximum_height: i32,
    pub floating_minimum_width: i32,
    pub floating_minimum_height: i32,
    pub client: crate::types::config_client,
    pub bar: crate::types::config_bar,
    pub popup_during_fullscreen: PopupDuringFullscreen,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: ::core::ffi::c_uint,
    pub gaps: crate::types::gaps_t,
    pub smart_gaps: SmartGaps,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkTMarks {
    pub tqe_next: *mut crate::types::mark_t,
    pub tqe_prev: *mut *mut crate::types::mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match {
    pub error: *mut ::core::ffi::c_char,
    pub title: *mut crate::regex::Regex,
    pub application: *mut crate::regex::Regex,
    pub class: *mut crate::regex::Regex,
    pub instance: *mut crate::regex::Regex,
    pub mark: *mut crate::regex::Regex,
    pub window_role: *mut crate::regex::Regex,
    pub workspace: *mut crate::regex::Regex,
    pub machine: *mut crate::regex::Regex,
    pub window_type: xcb_sys::xcb_atom_t,
    pub urgent: UrgencyMatch,
    pub dock: ::core::ffi::c_int,
    pub id: xcb_sys::xcb_window_t,
    pub window_mode: WindowMode,
    pub con_id: *mut crate::types::Con,
    pub match_all_windows: bool,
    pub insert_where: ::core::ffi::c_uint,
    pub matches: crate::types::MatchMatches,
    pub restart_mode: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchMatches {
    pub tqe_next: *mut crate::types::Match,
    pub tqe_prev: *mut *mut crate::types::Match,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OutputNameNames {
    pub sle_next: *mut crate::types::output_name,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct Window {
    pub id: xcb_sys::xcb_window_t,
    pub leader: xcb_sys::xcb_window_t,
    pub transient_for: xcb_sys::xcb_window_t,
    pub nr_assignments: u32,
    pub ran_assignments: *mut *mut crate::types::Assignment,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub name: Option<i3_common::string::I3String>,
    pub role: *mut ::core::ffi::c_char,
    pub machine: *mut ::core::ffi::c_char,
    pub name_x_changed: bool,
    pub uses_net_wm_name: bool,
    pub needs_take_focus: bool,
    pub doesnt_accept_focus: bool,
    pub window_type: xcb_sys::xcb_atom_t,
    pub wm_desktop: u32,
    pub dock: WindowDock,
    pub urgent: libc::timeval,
    pub reserved: crate::types::reservedpx,
    pub depth: u16,
    pub base_width: ::core::ffi::c_int,
    pub base_height: ::core::ffi::c_int,
    pub width_increment: ::core::ffi::c_int,
    pub height_increment: ::core::ffi::c_int,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
    pub min_aspect_ratio: ::core::ffi::c_double,
    pub max_aspect_ratio: ::core::ffi::c_double,
    pub icon: *mut cairo_sys::cairo_surface_t,
    pub shaped: bool,
    pub input_shaped: bool,
    pub managed_since: ::core::ffi::c_long,
    pub swallowed: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: Default::default(),
            leader: Default::default(),
            transient_for: Default::default(),
            nr_assignments: Default::default(),
            ran_assignments: std::ptr::null_mut(),
            class_class: std::ptr::null_mut(),
            class_instance: std::ptr::null_mut(),
            name: None,
            role: std::ptr::null_mut(),
            machine: std::ptr::null_mut(),
            name_x_changed: Default::default(),
            uses_net_wm_name: Default::default(),
            needs_take_focus: Default::default(),
            doesnt_accept_focus: Default::default(),
            window_type: Default::default(),
            wm_desktop: Default::default(),
            dock: Default::default(),
            urgent: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            reserved: reservedpx {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
            depth: Default::default(),
            base_width: Default::default(),
            base_height: Default::default(),
            width_increment: Default::default(),
            height_increment: Default::default(),
            min_width: Default::default(),
            min_height: Default::default(),
            max_width: Default::default(),
            max_height: Default::default(),
            min_aspect_ratio: Default::default(),
            max_aspect_ratio: Default::default(),
            icon: std::ptr::null_mut(),
            shaped: Default::default(),
            input_shaped: Default::default(),
            managed_since: Default::default(),
            swallowed: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XoutputOutputs {
    pub tqe_next: *mut crate::types::xoutput,
    pub tqe_prev: *mut *mut crate::types::xoutput,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_bar {
    pub focused: crate::types::Colortriple,
    pub unfocused: crate::types::Colortriple,
    pub urgent: crate::types::Colortriple,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_client {
    pub background: i3_common::draw_util::color_t,
    pub focused: crate::types::Colortriple,
    pub focused_inactive: crate::types::Colortriple,
    pub focused_tab_title: crate::types::Colortriple,
    pub unfocused: crate::types::Colortriple,
    pub urgent: crate::types::Colortriple,
    pub placeholder: crate::types::Colortriple,
    pub got_focused_tab_title: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_render_params {
    pub color: *mut crate::types::Colortriple,
    pub border_style: BorderStyle,
    pub con_rect: crate::types::width_height,
    pub con_window_rect: crate::types::width_height,
    pub con_deco_rect: crate::types::Rect,
    pub background: i3_common::draw_util::color_t,
    pub parent_layout: Layout,
    pub con_is_leaf: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct floating_head {
    pub tqh_first: *mut crate::types::Con,
    pub tqh_last: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_head {
    pub tqh_first: *mut crate::types::Con,
    pub tqh_last: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaps_t {
    pub inner: ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub right: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub left: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mark_t {
    pub name: *mut ::core::ffi::c_char,
    pub marks: crate::types::MarkTMarks,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct marks_head {
    pub tqh_first: *mut crate::types::mark_t,
    pub tqh_last: *mut *mut crate::types::mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct names_head {
    pub slh_first: *mut crate::types::output_name,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodes_head {
    pub tqh_first: *mut crate::types::Con,
    pub tqh_last: *mut *mut crate::types::Con,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_name {
    pub name: *mut ::core::ffi::c_char,
    pub names: crate::types::OutputNameNames,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct reservedpx {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct swallow_head {
    pub tqh_first: *mut crate::types::Match,
    pub tqh_last: *mut *mut crate::types::Match,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct width_height {
    pub w: u32,
    pub h: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xoutput {
    pub id: xcb_sys::xcb_randr_output_t,
    pub active: bool,
    pub changed: bool,
    pub to_be_disabled: bool,
    pub primary: bool,
    pub names_head: crate::types::names_head,
    pub con: *mut crate::types::Con,
    pub rect: crate::types::Rect,
    pub outputs: crate::types::XoutputOutputs,
}

impl BorderStyle {
    pub fn next(self) -> Self {
        match self {
            Self::None => Self::Pixel,
            Self::Pixel => Self::Normal,
            Self::Normal => Self::None,
        }
    }
}
impl FloatingState {
    pub fn is_floating(self) -> bool {
        matches!(self, Self::AutoOn | Self::UserOn)
    }
}
impl BarMode {
    pub fn toggled(self) -> Self {
        match self {
            Self::Dock | Self::Invisible => Self::Hide,
            Self::Hide => Self::Dock,
        }
    }
}
impl BarHiddenState {
    pub fn toggled(self) -> Self {
        match self {
            Self::Hide => Self::Show,
            Self::Show => Self::Hide,
        }
    }
}
