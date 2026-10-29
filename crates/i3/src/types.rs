// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Shared model representations extracted from con.rs; implementations stay in their files.
// Anonymous C2Rust names are mapped through their owning field paths, not numeric suffixes.

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

#[derive(Copy, Clone)]
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
    pub layout: ::core::ffi::c_uint,
    pub last_split_layout: ::core::ffi::c_uint,
    pub workspace_layout: ::core::ffi::c_uint,
    pub border_style: ::core::ffi::c_uint,
    pub max_user_border_style: ::core::ffi::c_uint,
    pub floating: ::core::ffi::c_uint,
    pub nodes: crate::types::ConNodes,
    pub focused: crate::types::ConFocused,
    pub all_cons: crate::types::ConAllCons,
    pub floating_windows: crate::types::ConFloatingWindows,
    pub on_remove_child: Option<unsafe fn(*mut crate::types::Con) -> ()>,
    pub scratchpad_state: ::core::ffi::c_uint,
    pub old_id: ::core::ffi::c_int,
    pub depth: u16,
    pub colormap: xcb_sys::xcb_colormap_t,
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
    pub default_layout: ::core::ffi::c_uint,
    pub container_stack_limit: ::core::ffi::c_int,
    pub container_stack_limit_value: ::core::ffi::c_int,
    pub default_border_width: ::core::ffi::c_int,
    pub default_floating_border_width: ::core::ffi::c_int,
    pub default_orientation: ::core::ffi::c_int,
    pub disable_focus_follows_mouse: bool,
    pub mouse_warping: ::core::ffi::c_uint,
    pub hide_edge_borders: ::core::ffi::c_uint,
    pub disable_workspace_bar: bool,
    pub focus_wrapping: ::core::ffi::c_uint,
    pub force_xinerama: bool,
    pub disable_randr15: bool,
    pub fake_outputs: *mut ::core::ffi::c_char,
    pub workspace_auto_back_and_forth: bool,
    pub workspace_urgency_timer: ::core::ffi::c_float,
    pub focus_on_window_activation: ::core::ffi::c_uint,
    pub show_marks: bool,
    pub title_align: ::core::ffi::c_uint,
    pub default_border: ::core::ffi::c_uint,
    pub default_floating_border: ::core::ffi::c_uint,
    pub floating_modifier: u32,
    pub swap_modifier: u32,
    pub floating_maximum_width: i32,
    pub floating_maximum_height: i32,
    pub floating_minimum_width: i32,
    pub floating_minimum_height: i32,
    pub client: crate::types::config_client,
    pub bar: crate::types::config_bar,
    pub popup_during_fullscreen: ::core::ffi::c_uint,
    pub number_barconfigs: ::core::ffi::c_int,
    pub tiling_drag: ::core::ffi::c_uint,
    pub gaps: crate::types::gaps_t,
    pub smart_gaps: ::core::ffi::c_uint,
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
    pub urgent: ::core::ffi::c_int,
    pub dock: ::core::ffi::c_int,
    pub id: xcb_sys::xcb_window_t,
    pub window_mode: ::core::ffi::c_uint,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub id: xcb_sys::xcb_window_t,
    pub leader: xcb_sys::xcb_window_t,
    pub transient_for: xcb_sys::xcb_window_t,
    pub nr_assignments: u32,
    pub ran_assignments: *mut *mut crate::types::Assignment,
    pub class_class: *mut ::core::ffi::c_char,
    pub class_instance: *mut ::core::ffi::c_char,
    pub name: *mut i3_common::string::I3String,
    pub role: *mut ::core::ffi::c_char,
    pub machine: *mut ::core::ffi::c_char,
    pub name_x_changed: bool,
    pub uses_net_wm_name: bool,
    pub needs_take_focus: bool,
    pub doesnt_accept_focus: bool,
    pub window_type: xcb_sys::xcb_atom_t,
    pub wm_desktop: u32,
    pub dock: ::core::ffi::c_uint,
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
    pub border_style: ::core::ffi::c_int,
    pub con_rect: crate::types::width_height,
    pub con_window_rect: crate::types::width_height,
    pub con_deco_rect: crate::types::Rect,
    pub background: i3_common::draw_util::color_t,
    pub parent_layout: ::core::ffi::c_uint,
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
