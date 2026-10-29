use crate::application::assignments;
use crate::application::autostarts;
use crate::application::autostarts_always;
use crate::application::ws_assignments;
use crate::bindings::configure_binding;
use crate::config::barconfigs;
use crate::config::included_files;
use crate::config::modes;
use crate::config_parser::parse_file;
use crate::config_parser::ConfigParserState;
use crate::parser_util::StackValueKind;
use crate::types::BarHiddenState;
use crate::types::BarMode;
use crate::types::BarPosition;
use crate::types::BorderStyle;
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
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * config_directives.c: all config storing functions (see config_parser.c)
 *
 */
// Mechanical file-by-file Rust translation of src/config_directives.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::ev_tstamp;

pub type i3String = _i3String;

pub type i3Font = Font;

pub type i3_event_state_mask_t = u32;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub use crate::regex::Regex as regex;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub const I3_XKB_GROUP_MASK_4: ::core::ffi::c_uint = 8;
pub const I3_XKB_GROUP_MASK_3: ::core::ffi::c_uint = 4;
pub const I3_XKB_GROUP_MASK_2: ::core::ffi::c_uint = 2;
pub const I3_XKB_GROUP_MASK_1: ::core::ffi::c_uint = 1;
pub const I3_XKB_GROUP_MASK_ANY: ::core::ffi::c_uint = 0;

pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;

pub use crate::application::Autostart;
pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
pub use crate::config::IncludedFile;

pub use crate::application::Barconfig;

pub use crate::application::Barbinding;

pub use crate::application::tray_output_t;
pub use crate::bindings::Mode;
pub use crate::commands::stack_entry;
pub use crate::commands::C2RustUnnamed_31 as C2RustUnnamed_39;

pub use crate::commands::stack;
pub use crate::config::parser_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConfigResultIR {
    pub ctx: *mut parser_ctx,
    pub next_state: ConfigParserState,
    pub has_errors: bool,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn cfg_include(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    pattern: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - include %s\n".as_ptr().cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_include".as_ptr(),
        19 as ::core::ffi::c_int,
        pattern,
    );
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    let ret: ::core::ffi::c_int =
        wordexp(pattern, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if ret != 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: wordexp(%s): error %d\n".as_ptr().cast_mut(),
            pattern,
            ret,
        );
        (*result).has_errors = true_0 != 0;
        return;
    }
    let w: *mut *mut ::core::ffi::c_char = p.we_wordv;
    let mut i: usize = 0 as usize;
    while i < p.we_wordc {
        let mut resolved_path: [::core::ffi::c_char; 4096] = [
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
        if realpath(
            *w.offset(i as isize),
            &raw mut resolved_path as *mut ::core::ffi::c_char,
        )
        .is_null()
        {
            verboselog(
                c"Skipping %s: %s\n".as_ptr().cast_mut(),
                *w.offset(i as isize),
                strerror(*__errno_location()),
            );
        } else {
            let mut skip: bool = false_0 != 0;
            let mut file: *mut IncludedFile = ::core::ptr::null_mut::<IncludedFile>();
            file = included_files.tqh_first as *mut IncludedFile;
            while !file.is_null() {
                if strcmp(
                    (*file).path,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    skip = true_0 != 0;
                    break;
                } else {
                    file = (*file).files.tqe_next as *mut IncludedFile;
                }
            }
            if skip {
                verboselog(
                    c"Skipping file %s (already included)\n".as_ptr().cast_mut(),
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                );
            } else {
                verboselog(
                    c"Including config file %s\n".as_ptr().cast_mut(),
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                );
                file = scalloc(1 as usize, ::core::mem::size_of::<IncludedFile>() as usize)
                    as *mut IncludedFile;
                (*file).path = sstrdup(&raw mut resolved_path as *mut ::core::ffi::c_char);
                (*file).files.tqe_next = ::core::ptr::null_mut::<IncludedFile>();
                (*file).files.tqe_prev = included_files.tqh_last;
                *included_files.tqh_last = file as *mut IncludedFile;
                included_files.tqh_last = &raw mut (*file).files.tqe_next;
                let mut stack: stack = stack {
                    stack: [
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                        stack_entry {
                            identifier: ::core::ptr::null::<::core::ffi::c_char>(),
                            type_0: StackValueKind::String,
                            val: C2RustUnnamed_39 {
                                str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            },
                        },
                    ],
                };
                let mut ctx: parser_ctx = parser_ctx {
                    use_nagbar: (*(*result).ctx).use_nagbar,
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
                    statelist: [ConfigParserState::ForceDisplayUrgencyHintMs; 10],
                    statelist_idx: 0,
                    stack: &raw mut stack,
                    variables: (*(*result).ctx).variables,
                    has_errors: false,
                };
                let current_block_46: u64;
                match parse_file(
                    &raw mut ctx,
                    &raw mut resolved_path as *mut ::core::ffi::c_char,
                    file,
                ) {
                    ParseFileResult::Success => {
                        current_block_46 = 10150597327160359210;
                    }
                    ParseFileResult::Failed => {
                        errorlog(
                            c"ERROR: including config file %s: %s\n".as_ptr().cast_mut(),
                            &raw mut resolved_path as *mut ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        current_block_46 = 1383695296304875401;
                    }
                    ParseFileResult::ConfigErrors => {
                        current_block_46 = 1383695296304875401;
                    }
                }
                match current_block_46 {
                    1383695296304875401 => {
                        (*result).has_errors = true_0 != 0;
                        if !(*file).files.tqe_next.is_null() {
                            (*(*file).files.tqe_next).files.tqe_prev = (*file).files.tqe_prev;
                        } else {
                            included_files.tqh_last = (*file).files.tqe_prev;
                        }
                        *(*file).files.tqe_prev = (*file).files.tqe_next;
                        free((*file).path as *mut ::core::ffi::c_void);
                        (*file).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free((*file).raw_contents as *mut ::core::ffi::c_void);
                        (*file).raw_contents = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free((*file).variable_replaced_contents as *mut ::core::ffi::c_void);
                        (*file).variable_replaced_contents =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        free(file as *mut ::core::ffi::c_void);
                        file = ::core::ptr::null_mut::<IncludedFile>();
                    }
                    _ => {}
                }
                (*(*result).ctx).variables = ctx.variables;
            }
        }
        i = i.wrapping_add(1);
    }
    wordfree(&raw mut p);
}
static mut criteria_next_state: ConfigParserState = ConfigParserState::ForceDisplayUrgencyHintMs;

pub unsafe fn cfg_criteria_init(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    _state: ConfigParserState,
) {
    criteria_next_state = _state;
    debuglog(
        c"%s:%s:%d - Initializing criteria, current_match = %p, state = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_criteria_init".as_ptr(),
        101 as ::core::ffi::c_int,
        current_match,
        _state as u32,
    );
    match_free(current_match);
    match_init(current_match);
}

pub unsafe fn cfg_criteria_pop_state(current_match: *mut Match, result: *mut ConfigResultIR) {
    (*result).next_state = criteria_next_state;
}

pub unsafe fn cfg_criteria_add(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    match_parse_property(current_match, ctype, cvalue);
}

pub unsafe fn event_state_from_str(str: *const ::core::ffi::c_char) -> i3_event_state_mask_t {
    let mut result: i3_event_state_mask_t = 0 as i3_event_state_mask_t;
    if str.is_null() {
        return result;
    }
    if !strstr(str, c"Mod1".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_1 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Mod2".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_2 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Mod3".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_3 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Mod4".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_4 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Mod5".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_MOD_5 as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Control".as_ptr()).is_null() || !strstr(str, c"Ctrl".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_CONTROL as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Shift".as_ptr()).is_null() {
        result |= XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as i3_event_state_mask_t;
    }
    if !strstr(str, c"Group1".as_ptr()).is_null() {
        result |= ((I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, c"Group2".as_ptr()).is_null() || !strstr(str, c"Mode_switch".as_ptr()).is_null()
    {
        result |= ((I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, c"Group3".as_ptr()).is_null() {
        result |= ((I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    if !strstr(str, c"Group4".as_ptr()).is_null() {
        result |= ((I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            as i3_event_state_mask_t;
    }
    result
}

pub unsafe fn cfg_font(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    font: *const ::core::ffi::c_char,
) {
    config.font = load_font(font, true_0 != 0);
    set_font(&raw mut config.font);
}

pub unsafe fn cfg_binding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    bindtype: *const ::core::ffi::c_char,
    modifiers: *const ::core::ffi::c_char,
    key: *const ::core::ffi::c_char,
    release: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    whole_window: *const ::core::ffi::c_char,
    exclude_titlebar: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    configure_binding(
        bindtype,
        modifiers,
        key,
        release,
        border,
        whole_window,
        exclude_titlebar,
        command,
        DEFAULT_BINDING_MODE,
        false_0 != 0,
    );
}
static mut current_mode: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut current_mode_pango_markup: bool = false;

pub unsafe fn cfg_mode_binding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    bindtype: *const ::core::ffi::c_char,
    modifiers: *const ::core::ffi::c_char,
    key: *const ::core::ffi::c_char,
    release: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    whole_window: *const ::core::ffi::c_char,
    exclude_titlebar: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    if current_mode.is_null() {
        return;
    }
    configure_binding(
        bindtype,
        modifiers,
        key,
        release,
        border,
        whole_window,
        exclude_titlebar,
        command,
        current_mode,
        current_mode_pango_markup,
    );
}

pub unsafe fn cfg_enter_mode(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    pango_markup: *const ::core::ffi::c_char,
    modename: *const ::core::ffi::c_char,
) {
    if strcmp(modename, DEFAULT_BINDING_MODE) == 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: You cannot use the name %s for your mode\n"
                .as_ptr()
                .cast_mut(),
            DEFAULT_BINDING_MODE,
        );
        return;
    }
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, modename) == 0 as ::core::ffi::c_int {
            errorlog(
                c"ERROR: The binding mode with name \"%s\" is defined at least twice.\n"
                    .as_ptr()
                    .cast_mut(),
                modename,
            );
        }
        mode = (*mode).modes.sle_next;
    }
    debuglog(
        c"%s:%s:%d - \t now in mode %s\n".as_ptr().cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_enter_mode".as_ptr(),
        211 as ::core::ffi::c_int,
        modename,
    );
    free(current_mode as *mut ::core::ffi::c_void);
    current_mode = ::core::ptr::null_mut::<::core::ffi::c_char>();
    current_mode = sstrdup(modename);
    current_mode_pango_markup = !pango_markup.is_null();
}

pub unsafe fn cfg_exec(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    exectype: *const ::core::ffi::c_char,
    no_startup_id: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    let new: *mut Autostart =
        smalloc(::core::mem::size_of::<Autostart>() as usize) as *mut Autostart;
    (*new).command = sstrdup(command);
    (*new).no_startup_id = !no_startup_id.is_null();
    if strcmp(exectype, c"exec".as_ptr()) == 0 as ::core::ffi::c_int {
        (*new).autostarts.tqe_next = ::core::ptr::null_mut::<Autostart>();
        (*new).autostarts.tqe_prev = autostarts.tqh_last;
        *autostarts.tqh_last = new;
        autostarts.tqh_last = &raw mut (*new).autostarts.tqe_next;
    } else {
        (*new).autostarts_always.tqe_next = ::core::ptr::null_mut::<Autostart>();
        (*new).autostarts_always.tqe_prev = autostarts_always.tqh_last;
        *autostarts_always.tqh_last = new;
        autostarts_always.tqh_last = &raw mut (*new).autostarts_always.tqe_next;
    };
}

pub unsafe fn cfg_for_window(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            c"ERROR: Match is empty, ignoring this for_window statement\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - \t should execute command %s for the criteria mentioned above\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_for_window".as_ptr(),
        237 as ::core::ffi::c_int,
        command,
    );
    let assignment: *mut Assignment =
        scalloc(1 as usize, ::core::mem::size_of::<Assignment>() as usize) as *mut Assignment;
    (*assignment).type_0 = A_COMMAND;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).dest.command = sstrdup(command);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
unsafe fn apply_gaps(gaps: *mut gaps_t, mask: gaps_mask_t, value: ::core::ffi::c_int) {
    if gaps.is_null() {
        return;
    }
    if mask as ::core::ffi::c_uint & GAPS_INNER as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).inner = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).top = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).right = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).bottom = value;
    }
    if mask as ::core::ffi::c_uint & GAPS_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*gaps).left = value;
    }
}
unsafe fn create_gaps_assignment(
    workspace: *const ::core::ffi::c_char,
    mask: gaps_mask_t,
    pixels: ::core::ffi::c_int,
) {
    if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        return;
    }
    debuglog(
        c"%s:%s:%d - Setting gaps for workspace %s"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"create_gaps_assignment".as_ptr(),
        271 as ::core::ffi::c_int,
        workspace,
    );
    let mut found: bool = false_0 != 0;
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    assignment = ws_assignments.tqh_first;
    while !assignment.is_null() {
        if strcasecmp((*assignment).name, workspace) == 0 as ::core::ffi::c_int {
            found = true_0 != 0;
            break;
        } else {
            assignment = (*assignment).ws_assignments.tqe_next;
        }
    }
    if !found {
        assignment = scalloc(
            1 as usize,
            ::core::mem::size_of::<Workspace_Assignment>() as usize,
        ) as *mut Workspace_Assignment;
        (*assignment).name = sstrdup(workspace);
        (*assignment).output = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*assignment).ws_assignments.tqe_next = ::core::ptr::null_mut::<Workspace_Assignment>();
        (*assignment).ws_assignments.tqe_prev = ws_assignments.tqh_last;
        *ws_assignments.tqh_last = assignment;
        ws_assignments.tqh_last = &raw mut (*assignment).ws_assignments.tqe_next;
    }
    (*assignment).gaps_mask = ::core::mem::transmute::<::core::ffi::c_uint, gaps_mask_t>(
        (*assignment).gaps_mask as ::core::ffi::c_uint | mask as ::core::ffi::c_uint,
    );
    apply_gaps(&raw mut (*assignment).gaps, mask, pixels);
}
unsafe fn gaps_scope_to_mask(scope: *const ::core::ffi::c_char) -> gaps_mask_t {
    if strcmp(scope, c"inner".as_ptr()) == 0 {
        return GAPS_INNER;
    }
    if strcmp(scope, c"outer".as_ptr()) == 0 {
        return GAPS_OUTER;
    }
    if strcmp(scope, c"vertical".as_ptr()) == 0 {
        return GAPS_VERTICAL;
    }
    if strcmp(scope, c"horizontal".as_ptr()) == 0 {
        return GAPS_HORIZONTAL;
    }
    if strcmp(scope, c"top".as_ptr()) == 0 {
        return GAPS_TOP;
    }
    if strcmp(scope, c"right".as_ptr()) == 0 {
        return GAPS_RIGHT;
    }
    if strcmp(scope, c"bottom".as_ptr()) == 0 {
        return GAPS_BOTTOM;
    }
    if strcmp(scope, c"left".as_ptr()) == 0 {
        return GAPS_LEFT;
    }
    errorlog(
        c"ERROR: Invalid command, cannot process scope %s"
            .as_ptr()
            .cast_mut(),
        scope,
    );
    0 as gaps_mask_t
}

pub unsafe fn cfg_gaps(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: *const ::core::ffi::c_char,
    scope: *const ::core::ffi::c_char,
    value: ::core::ffi::c_long,
) {
    let pixels: ::core::ffi::c_int = logical_px(value as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mask: gaps_mask_t = gaps_scope_to_mask(scope) as gaps_mask_t;
    if workspace.is_null() {
        apply_gaps(&raw mut config.gaps, mask, pixels);
    } else {
        create_gaps_assignment(workspace, mask, pixels);
    };
}

pub unsafe fn cfg_smart_borders(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    enable: *const ::core::ffi::c_char,
) {
    if strcmp(enable, c"no_gaps".as_ptr()) == 0 {
        config.hide_edge_borders = HEBM_SMART_NO_GAPS;
    } else if boolstr(enable) {
        if config.hide_edge_borders as ::core::ffi::c_uint
            == HEBM_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            config.hide_edge_borders = HEBM_SMART;
        } else {
            errorlog(
                c"ERROR: Both hide_edge_borders and smart_borders was used. Ignoring smart_borders as it is deprecated.\n".as_ptr().cast_mut(),
            );
        }
    }
}

pub unsafe fn cfg_smart_gaps(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    enable: *const ::core::ffi::c_char,
) {
    if strcmp(enable, c"inverse_outer".as_ptr()) == 0 {
        config.smart_gaps = SmartGaps::InverseOuter;
    } else {
        config.smart_gaps = if boolstr(enable) as ::core::ffi::c_int != 0 {
            SmartGaps::On
        } else {
            SmartGaps::Off
        };
    };
}

pub unsafe fn cfg_floating_minimum_size(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
    height: ::core::ffi::c_long,
) {
    config.floating_minimum_width = width as i32;
    config.floating_minimum_height = height as i32;
}

pub unsafe fn cfg_floating_maximum_size(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
    height: ::core::ffi::c_long,
) {
    config.floating_maximum_width = width as i32;
    config.floating_maximum_height = height as i32;
}

pub unsafe fn cfg_floating_modifier(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    modifiers: *const ::core::ffi::c_char,
) {
    config.floating_modifier = event_state_from_str(modifiers) as u32;
}

pub unsafe fn cfg_tiling_drag_swap_modifier(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    modifiers: *const ::core::ffi::c_char,
) {
    config.swap_modifier = event_state_from_str(modifiers) as u32;
}

pub unsafe fn cfg_default_orientation(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    orientation: *const ::core::ffi::c_char,
) {
    if strcmp(orientation, c"horizontal".as_ptr()) == 0 as ::core::ffi::c_int {
        config.default_orientation = Orientation::Horizontal;
    } else if strcmp(orientation, c"vertical".as_ptr()) == 0 as ::core::ffi::c_int {
        config.default_orientation = Orientation::Vertical;
    } else {
        config.default_orientation = Orientation::None;
    };
}

pub unsafe fn cfg_workspace_layout(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    layout: *const ::core::ffi::c_char,
) {
    if strcmp(layout, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
        config.default_layout = Layout::Default;
    } else if strcmp(layout, c"stacking".as_ptr()) == 0 as ::core::ffi::c_int
        || strcmp(layout, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int
    {
        config.default_layout = Layout::Stacked;
    } else {
        config.default_layout = Layout::Tabbed;
    };
}

pub unsafe fn cfg_default_border(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    windowtype: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    width: ::core::ffi::c_long,
) {
    let mut border_style: BorderStyle = BorderStyle::None;
    let mut border_width: ::core::ffi::c_int = 0;
    if strcmp(border, c"1pixel".as_ptr()) == 0 as ::core::ffi::c_int {
        border_style = BorderStyle::Pixel;
        border_width = 1 as ::core::ffi::c_int;
    } else if strcmp(border, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
        border_style = BorderStyle::None;
        border_width = 0 as ::core::ffi::c_int;
    } else if strcmp(border, c"pixel".as_ptr()) == 0 as ::core::ffi::c_int {
        border_style = BorderStyle::Pixel;
        border_width = width as ::core::ffi::c_int;
    } else {
        border_style = BorderStyle::Normal;
        border_width = width as ::core::ffi::c_int;
    }
    if strcmp(windowtype, c"default_border".as_ptr()) == 0 as ::core::ffi::c_int
        || strcmp(windowtype, c"new_window".as_ptr()) == 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - default tiled border style = %d and border width = %d (%d physical px)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/config_directives.c".as_ptr(),
            c"cfg_default_border".as_ptr(),
            416 as ::core::ffi::c_int,
            (border_style) as u32,
            border_width,
            logical_px(border_width),
        );
        config.default_border = border_style;
        config.default_border_width = logical_px(border_width);
    } else {
        debuglog(
            c"%s:%s:%d - default floating border style = %d and border width = %d (%d physical px)\n".as_ptr().cast_mut(),
            c"../src/config_directives.c".as_ptr(),
            c"cfg_default_border".as_ptr(),
            421 as ::core::ffi::c_int,
            (border_style) as u32,
            border_width,
            logical_px(border_width),
        );
        config.default_floating_border = border_style;
        config.default_floating_border_width = logical_px(border_width);
    };
}

pub unsafe fn cfg_hide_edge_borders(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    borders: *const ::core::ffi::c_char,
) {
    if strcmp(borders, c"smart_no_gaps".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_SMART_NO_GAPS;
    } else if strcmp(borders, c"smart".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_SMART;
    } else if strcmp(borders, c"vertical".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_VERTICAL;
    } else if strcmp(borders, c"horizontal".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_HORIZONTAL;
    } else if strcmp(borders, c"both".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_BOTH;
    } else if strcmp(borders, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
        config.hide_edge_borders = HEBM_NONE;
    } else if boolstr(borders) {
        config.hide_edge_borders = HEBM_VERTICAL;
    } else {
        config.hide_edge_borders = HEBM_NONE;
    };
}

pub unsafe fn cfg_focus_follows_mouse(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    config.disable_focus_follows_mouse = !boolstr(value);
}

pub unsafe fn cfg_mouse_warping(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    if strcmp(value, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
        config.mouse_warping = Warping::None;
    } else if strcmp(value, c"output".as_ptr()) == 0 as ::core::ffi::c_int {
        config.mouse_warping = Warping::Output;
    }
}

pub unsafe fn cfg_force_xinerama(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    config.force_xinerama = boolstr(value);
}

pub unsafe fn cfg_disable_randr15(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    config.disable_randr15 = boolstr(value);
}

pub unsafe fn cfg_focus_wrapping(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    if strcmp(value, c"force".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_wrapping = FocusWrapping::Force;
    } else if strcmp(value, c"workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_wrapping = FocusWrapping::Workspace;
    } else if boolstr(value) {
        config.focus_wrapping = FocusWrapping::On;
    } else {
        config.focus_wrapping = FocusWrapping::Off;
    };
}

pub unsafe fn cfg_force_focus_wrapping(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    if boolstr(value) {
        config.focus_wrapping = FocusWrapping::Force;
    } else if config.focus_wrapping == FocusWrapping::Force {
        config.focus_wrapping = FocusWrapping::On;
    }
}

pub unsafe fn cfg_workspace_back_and_forth(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    config.workspace_auto_back_and_forth = boolstr(value);
}

pub unsafe fn cfg_fake_outputs(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    outputs: *const ::core::ffi::c_char,
) {
    free(config.fake_outputs as *mut ::core::ffi::c_void);
    config.fake_outputs = sstrdup(outputs);
}

pub unsafe fn cfg_force_display_urgency_hint(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    duration_ms: ::core::ffi::c_long,
) {
    config.workspace_urgency_timer =
        (duration_ms as ::core::ffi::c_double / 1000.0f64) as ::core::ffi::c_float;
}

pub unsafe fn cfg_focus_on_window_activation(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    mode: *const ::core::ffi::c_char,
) {
    if strcmp(mode, c"smart".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_on_window_activation = FocusActivation::Smart;
    } else if strcmp(mode, c"urgent".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_on_window_activation = FocusActivation::Urgent;
    } else if strcmp(mode, c"focus".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_on_window_activation = FocusActivation::Focus;
    } else if strcmp(mode, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
        config.focus_on_window_activation = FocusActivation::None;
    } else {
        errorlog(
            c"ERROR: Unknown focus_on_window_activation mode \"%s\", ignoring it.\n"
                .as_ptr()
                .cast_mut(),
            mode,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - Set new focus_on_window_activation mode = %i.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_focus_on_window_activation".as_ptr(),
        519 as ::core::ffi::c_int,
        config.focus_on_window_activation as ::core::ffi::c_uint,
    );
}

pub unsafe fn cfg_title_align(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    alignment: *const ::core::ffi::c_char,
) {
    if strcmp(alignment, c"left".as_ptr()) == 0 as ::core::ffi::c_int {
        config.title_align = TitleAlign::Left;
    } else if strcmp(alignment, c"center".as_ptr()) == 0 as ::core::ffi::c_int {
        config.title_align = TitleAlign::Center;
    } else if strcmp(alignment, c"right".as_ptr()) == 0 as ::core::ffi::c_int {
        config.title_align = TitleAlign::Right;
    } else {
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                c"false".as_ptr(),
                c"../src/config_directives.c".as_ptr(),
                530 as ::core::ffi::c_uint,
                c"void cfg_title_align(Match *, struct ConfigResultIR *, const char *)".as_ptr(),
            );
        };
    };
}

pub unsafe fn cfg_show_marks(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    config.show_marks = boolstr(value);
}
static mut current_workspace: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;

pub unsafe fn cfg_workspace(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    mut workspace: *const ::core::ffi::c_char,
    output: *const ::core::ffi::c_char,
) {
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    if !workspace.is_null() {
        free(current_workspace as *mut ::core::ffi::c_void);
        current_workspace = ::core::ptr::null_mut::<::core::ffi::c_char>();
        assignment = ws_assignments.tqh_first;
        while !assignment.is_null() {
            if strcasecmp((*assignment).name, workspace) == 0 as ::core::ffi::c_int {
                if !(*assignment).output.is_null() {
                    errorlog(
                        c"ERROR: You have a duplicate workspace assignment for workspace \"%s\"\n"
                            .as_ptr()
                            .cast_mut(),
                        workspace,
                    );
                    return;
                }
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
        current_workspace = sstrdup(workspace);
    } else {
        if current_workspace.is_null() {
            debuglog(
                c"%s:%s:%d - Both workspace and current_workspace are NULL, assuming we had an error before\n".as_ptr().cast_mut(),
                c"../src/config_directives.c".as_ptr(),
                c"cfg_workspace".as_ptr(),
                563 as ::core::ffi::c_int,
            );
            return;
        }
        workspace = current_workspace;
    }
    debuglog(
        c"%s:%s:%d - Assigning workspace \"%s\" to output \"%s\"\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_workspace".as_ptr(),
        569 as ::core::ffi::c_int,
        workspace,
        output,
    );
    assignment = scalloc(
        1 as usize,
        ::core::mem::size_of::<Workspace_Assignment>() as usize,
    ) as *mut Workspace_Assignment;
    (*assignment).name = sstrdup(workspace);
    (*assignment).output = sstrdup(output);
    (*assignment).ws_assignments.tqe_next = ::core::ptr::null_mut::<Workspace_Assignment>();
    (*assignment).ws_assignments.tqe_prev = ws_assignments.tqh_last;
    *ws_assignments.tqh_last = assignment;
    ws_assignments.tqh_last = &raw mut (*assignment).ws_assignments.tqe_next;
}

pub unsafe fn cfg_ipc_socket(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    path: *const ::core::ffi::c_char,
) {
    free(config.ipc_socket_path as *mut ::core::ffi::c_void);
    config.ipc_socket_path = sstrdup(path);
}

pub unsafe fn cfg_restart_state(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    path: *const ::core::ffi::c_char,
) {
    free(config.restart_state_path as *mut ::core::ffi::c_void);
    config.restart_state_path = sstrdup(path);
}

pub unsafe fn cfg_popup_during_fullscreen(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    if strcmp(value, c"ignore".as_ptr()) == 0 as ::core::ffi::c_int {
        config.popup_during_fullscreen = PopupDuringFullscreen::Ignore;
    } else if strcmp(value, c"leave_fullscreen".as_ptr()) == 0 as ::core::ffi::c_int {
        config.popup_during_fullscreen = PopupDuringFullscreen::LeaveFullscreen;
    } else if strcmp(value, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
        config.popup_during_fullscreen = PopupDuringFullscreen::All;
    } else {
        config.popup_during_fullscreen = PopupDuringFullscreen::Smart;
    };
}

pub unsafe fn cfg_color_single(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: *const ::core::ffi::c_char,
    color: *const ::core::ffi::c_char,
) {
    config.client.background = draw_util_hex_to_color(color);
}

pub unsafe fn cfg_color(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    background: *const ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
    indicator: *const ::core::ffi::c_char,
    child_border: *const ::core::ffi::c_char,
) {
    if strcmp(colorclass, c"client.focused_inactive".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"focused_inactive".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.focused_inactive.border = draw_util_hex_to_color(border);
        config.client.focused_inactive.background = draw_util_hex_to_color(background);
        config.client.focused_inactive.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused_inactive.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused_inactive.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused_inactive.child_border = config.client.focused_inactive.background;
        }
        return;
    }
    if strcmp(colorclass, c"client.focused_tab_title".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"focused_tab_title".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.focused_tab_title.border = draw_util_hex_to_color(border);
        config.client.focused_tab_title.background = draw_util_hex_to_color(background);
        config.client.focused_tab_title.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused_tab_title.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused_tab_title.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused_tab_title.child_border =
                config.client.focused_tab_title.background;
        }
        return;
    }
    if strcmp(colorclass, c"client.focused".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"focused".as_ptr()) == 0 as ::core::ffi::c_int {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.focused.border = draw_util_hex_to_color(border);
        config.client.focused.background = draw_util_hex_to_color(background);
        config.client.focused.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.focused.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.focused.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.focused.child_border = config.client.focused.background;
        }
        return;
    }
    if strcmp(colorclass, c"client.unfocused".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"unfocused".as_ptr()) == 0 as ::core::ffi::c_int {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.unfocused.border = draw_util_hex_to_color(border);
        config.client.unfocused.background = draw_util_hex_to_color(background);
        config.client.unfocused.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.unfocused.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.unfocused.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.unfocused.child_border = config.client.unfocused.background;
        }
        return;
    }
    if strcmp(colorclass, c"client.urgent".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"urgent".as_ptr()) == 0 as ::core::ffi::c_int {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.urgent.border = draw_util_hex_to_color(border);
        config.client.urgent.background = draw_util_hex_to_color(background);
        config.client.urgent.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.urgent.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.urgent.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.urgent.child_border = config.client.urgent.background;
        }
        return;
    }
    if strcmp(colorclass, c"client.placeholder".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(c"focused_tab_title".as_ptr(), c"placeholder".as_ptr()) == 0 as ::core::ffi::c_int
        {
            config.client.got_focused_tab_title = true_0 != 0;
            if !indicator.is_null() || !child_border.is_null() {
                errorlog(
                    c"ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n".as_ptr().cast_mut(),
                );
            }
        }
        config.client.placeholder.border = draw_util_hex_to_color(border);
        config.client.placeholder.background = draw_util_hex_to_color(background);
        config.client.placeholder.text = draw_util_hex_to_color(text);
        if !indicator.is_null() {
            config.client.placeholder.indicator = draw_util_hex_to_color(indicator);
        }
        if !child_border.is_null() {
            config.client.placeholder.child_border = draw_util_hex_to_color(child_border);
        } else {
            config.client.placeholder.child_border = config.client.placeholder.background;
        }
    }
}

pub unsafe fn cfg_assign_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            c"ERROR: Match is empty, ignoring this assignment\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    if (*current_match).window_mode != WindowMode::Any {
        errorlog(
            c"ERROR: Assignments using window mode (floating/tiling) is not supported\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - New assignment, using above criteria, to output \"%s\".\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_assign_output".as_ptr(),
        654 as ::core::ffi::c_int,
        output,
    );
    let assignment: *mut Assignment =
        scalloc(1 as usize, ::core::mem::size_of::<Assignment>() as usize) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = A_TO_OUTPUT;
    (*assignment).dest.output = sstrdup(output);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_assign(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: *const ::core::ffi::c_char,
    is_number: bool,
) {
    if !(*current_match).error.is_null() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            c"ERROR: Match is empty, ignoring this assignment\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    if (*current_match).window_mode != WindowMode::Any {
        errorlog(
            c"ERROR: Assignments using window mode (floating/tiling) is not supported\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    if is_number as ::core::ffi::c_int != 0
        && ws_name_to_number(workspace) == -(1 as ::core::ffi::c_int)
    {
        errorlog(
            c"ERROR: Could not parse initial part of \"%s\" as a number.\n"
                .as_ptr()
                .cast_mut(),
            workspace,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - New assignment, using above criteria, to workspace \"%s\".\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_assign".as_ptr(),
        682 as ::core::ffi::c_int,
        workspace,
    );
    let assignment: *mut Assignment =
        scalloc(1 as usize, ::core::mem::size_of::<Assignment>() as usize) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = (if is_number as ::core::ffi::c_int != 0 {
        A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int
    } else {
        A_TO_WORKSPACE as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    (*assignment).dest.workspace = sstrdup(workspace);
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_no_focus(current_match: *mut Match, result: *mut ConfigResultIR) {
    if !(*current_match).error.is_null() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error,
        );
        return;
    }
    if match_is_empty(current_match) {
        errorlog(
            c"ERROR: Match is empty, ignoring this assignment\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - New assignment, using above criteria, to ignore focus on manage.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_no_focus".as_ptr(),
        700 as ::core::ffi::c_int,
    );
    let assignment: *mut Assignment =
        scalloc(1 as usize, ::core::mem::size_of::<Assignment>() as usize) as *mut Assignment;
    match_copy(&raw mut (*assignment).match_0, current_match);
    (*assignment).type_0 = A_NO_FOCUS;
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_ipc_kill_timeout(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    timeout_ms: ::core::ffi::c_long,
) {
    ipc_set_kill_timeout(timeout_ms as ev_tstamp / 1000.0f64);
}

pub unsafe fn cfg_tiling_drag(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    if strcmp(value, c"modifier".as_ptr()) == 0 as ::core::ffi::c_int {
        config.tiling_drag = TILING_DRAG_MODIFIER;
    } else if strcmp(value, c"titlebar".as_ptr()) == 0 as ::core::ffi::c_int {
        config.tiling_drag = TILING_DRAG_TITLEBAR;
    } else if strcmp(value, c"modifier,titlebar".as_ptr()) == 0 as ::core::ffi::c_int
        || strcmp(value, c"titlebar,modifier".as_ptr()) == 0 as ::core::ffi::c_int
    {
        config.tiling_drag = TILING_DRAG_MODIFIER_OR_TITLEBAR;
    } else {
        config.tiling_drag = TILING_DRAG_OFF;
    };
}
static mut current_bar: *mut Barconfig = ::core::ptr::null::<Barconfig>() as *mut Barconfig;

pub unsafe fn cfg_bar_font(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    font: *const ::core::ffi::c_char,
) {
    free((*current_bar).font as *mut ::core::ffi::c_void);
    (*current_bar).font = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).font = sstrdup(font);
}

pub unsafe fn cfg_bar_separator_symbol(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    separator: *const ::core::ffi::c_char,
) {
    free((*current_bar).separator_symbol as *mut ::core::ffi::c_void);
    (*current_bar).separator_symbol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).separator_symbol = sstrdup(separator);
}

pub unsafe fn cfg_bar_mode(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    mode: *const ::core::ffi::c_char,
) {
    (*current_bar).mode = if strcmp(mode, c"dock".as_ptr()) == 0 as ::core::ffi::c_int {
        BarMode::Dock
    } else {
        if strcmp(mode, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
            BarMode::Hide
        } else {
            BarMode::Invisible
        }
    };
}

pub unsafe fn cfg_bar_hidden_state(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    hidden_state: *const ::core::ffi::c_char,
) {
    (*current_bar).hidden_state =
        if strcmp(hidden_state, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
            BarHiddenState::Hide
        } else {
            BarHiddenState::Show
        };
}

pub unsafe fn cfg_bar_id(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    bar_id: *const ::core::ffi::c_char,
) {
    (*current_bar).id = sstrdup(bar_id);
}

pub unsafe fn cfg_bar_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: *const ::core::ffi::c_char,
) {
    let new_outputs: ::core::ffi::c_int = (*current_bar).num_outputs + 1 as ::core::ffi::c_int;
    (*current_bar).outputs = srealloc(
        (*current_bar).outputs as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            .wrapping_mul(new_outputs as usize),
    ) as *mut *mut ::core::ffi::c_char;
    let ref mut fresh0 = *(*current_bar)
        .outputs
        .offset((*current_bar).num_outputs as isize);
    *fresh0 = sstrdup(output);
    (*current_bar).num_outputs = new_outputs;
}

pub unsafe fn cfg_bar_verbose(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    verbose: *const ::core::ffi::c_char,
) {
    (*current_bar).verbose = boolstr(verbose);
}

pub unsafe fn cfg_bar_height(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    height: ::core::ffi::c_long,
) {
    (*current_bar).bar_height = height as u32;
}
unsafe fn dlog_padding() {
    debuglog(
        c"%s:%s:%d - padding now: x=%d, y=%d, w=%d, h=%d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"dlog_padding".as_ptr(),
        773 as ::core::ffi::c_int,
        (*current_bar).padding.x,
        (*current_bar).padding.y,
        (*current_bar).padding.width,
        (*current_bar).padding.height,
    );
}

pub unsafe fn cfg_bar_padding_one(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    all: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = all as u32;
    (*current_bar).padding.y = all as u32;
    (*current_bar).padding.width = all as u32;
    (*current_bar).padding.height = all as u32;
    dlog_padding();
}

pub unsafe fn cfg_bar_padding_two(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    top_and_bottom: ::core::ffi::c_long,
    right_and_left: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = right_and_left as u32;
    (*current_bar).padding.y = top_and_bottom as u32;
    (*current_bar).padding.width = right_and_left as u32;
    (*current_bar).padding.height = top_and_bottom as u32;
    dlog_padding();
}

pub unsafe fn cfg_bar_padding_three(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    top: ::core::ffi::c_long,
    right_and_left: ::core::ffi::c_long,
    bottom: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = right_and_left as u32;
    (*current_bar).padding.y = top as u32;
    (*current_bar).padding.width = right_and_left as u32;
    (*current_bar).padding.height = bottom as u32;
    dlog_padding();
}

pub unsafe fn cfg_bar_padding_four(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    top: ::core::ffi::c_long,
    right: ::core::ffi::c_long,
    bottom: ::core::ffi::c_long,
    left: ::core::ffi::c_long,
) {
    (*current_bar).padding.x = left as u32;
    (*current_bar).padding.y = top as u32;
    (*current_bar).padding.width = right as u32;
    (*current_bar).padding.height = bottom as u32;
    dlog_padding();
}

pub unsafe fn cfg_bar_modifier(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    modifiers: *const ::core::ffi::c_char,
) {
    (*current_bar).modifier = (if !modifiers.is_null() {
        event_state_from_str(modifiers) as ::core::ffi::c_long
    } else {
        XCB_NONE
    }) as u32;
}
unsafe fn bar_configure_binding(
    button: *const ::core::ffi::c_char,
    release: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    if strncasecmp(button, c"button".as_ptr(), strlen(c"button".as_ptr()))
        != 0 as ::core::ffi::c_int
    {
        errorlog(
            c"ERROR: Bindings for a bar can only be mouse bindings, not \"%s\", ignoring.\n"
                .as_ptr()
                .cast_mut(),
            button,
        );
        return;
    }
    let input_code: ::core::ffi::c_int =
        atoi(button.offset(strlen(c"button".as_ptr()) as isize)) as ::core::ffi::c_int;
    if input_code < 1 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Button \"%s\" does not seem to be in format 'buttonX'.\n"
                .as_ptr()
                .cast_mut(),
            button,
        );
        return;
    }
    let release_bool: bool = !release.is_null();
    let mut current: *mut Barbinding = ::core::ptr::null_mut::<Barbinding>();
    current = (*current_bar).bar_bindings.tqh_first as *mut Barbinding;
    while !current.is_null() {
        if (*current).input_code == input_code
            && (*current).release as ::core::ffi::c_int == release_bool as ::core::ffi::c_int
        {
            errorlog(
                c"ERROR: command for button %s was already specified, ignoring.\n"
                    .as_ptr()
                    .cast_mut(),
                button,
            );
            return;
        }
        current = (*current).bindings.tqe_next;
    }
    let new_binding: *mut Barbinding =
        scalloc(1 as usize, ::core::mem::size_of::<Barbinding>() as usize) as *mut Barbinding;
    (*new_binding).release = release_bool;
    (*new_binding).input_code = input_code;
    (*new_binding).command = sstrdup(command);
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Barbinding>();
    (*new_binding).bindings.tqe_prev = (*current_bar).bar_bindings.tqh_last;
    *(*current_bar).bar_bindings.tqh_last = new_binding;
    (*current_bar).bar_bindings.tqh_last = &raw mut (*new_binding).bindings.tqe_next;
}

pub unsafe fn cfg_bar_wheel_up_cmd(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    errorlog(
        c"ERROR: 'wheel_up_cmd' is deprecated. Please use 'bindsym button4 %s' instead.\n"
            .as_ptr()
            .cast_mut(),
        command,
    );
    bar_configure_binding(
        c"button4".as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        command,
    );
}

pub unsafe fn cfg_bar_wheel_down_cmd(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    errorlog(
        c"ERROR: 'wheel_down_cmd' is deprecated. Please use 'bindsym button5 %s' instead.\n"
            .as_ptr()
            .cast_mut(),
        command,
    );
    bar_configure_binding(
        c"button5".as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        command,
    );
}

pub unsafe fn cfg_bar_bindsym(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    button: *const ::core::ffi::c_char,
    release: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    bar_configure_binding(button, release, command);
}

pub unsafe fn cfg_bar_position(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    position: *const ::core::ffi::c_char,
) {
    (*current_bar).position = if strcmp(position, c"top".as_ptr()) == 0 as ::core::ffi::c_int {
        BarPosition::Top
    } else {
        BarPosition::Bottom
    };
}

pub unsafe fn cfg_bar_i3bar_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    i3bar_command: *const ::core::ffi::c_char,
) {
    free((*current_bar).i3bar_command as *mut ::core::ffi::c_void);
    (*current_bar).i3bar_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).i3bar_command = sstrdup(i3bar_command);
}

pub unsafe fn cfg_bar_color(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    background: *const ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
) {
    if strcmp(colorclass, c"focused_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.focused_workspace_border = sstrdup(border);
            (*current_bar).colors.focused_workspace_bg = sstrdup(background);
            (*current_bar).colors.focused_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.focused_workspace_bg = sstrdup(background);
            (*current_bar).colors.focused_workspace_text = sstrdup(border);
        }
    }
    if strcmp(colorclass, c"active_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.active_workspace_border = sstrdup(border);
            (*current_bar).colors.active_workspace_bg = sstrdup(background);
            (*current_bar).colors.active_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.active_workspace_bg = sstrdup(background);
            (*current_bar).colors.active_workspace_text = sstrdup(border);
        }
    }
    if strcmp(colorclass, c"inactive_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.inactive_workspace_border = sstrdup(border);
            (*current_bar).colors.inactive_workspace_bg = sstrdup(background);
            (*current_bar).colors.inactive_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.inactive_workspace_bg = sstrdup(background);
            (*current_bar).colors.inactive_workspace_text = sstrdup(border);
        }
    }
    if strcmp(colorclass, c"urgent_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.urgent_workspace_border = sstrdup(border);
            (*current_bar).colors.urgent_workspace_bg = sstrdup(background);
            (*current_bar).colors.urgent_workspace_text = sstrdup(text);
        } else {
            (*current_bar).colors.urgent_workspace_bg = sstrdup(background);
            (*current_bar).colors.urgent_workspace_text = sstrdup(border);
        }
    }
    if strcmp(colorclass, c"binding_mode".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.binding_mode_border = sstrdup(border);
            (*current_bar).colors.binding_mode_bg = sstrdup(background);
            (*current_bar).colors.binding_mode_text = sstrdup(text);
        } else {
            (*current_bar).colors.binding_mode_bg = sstrdup(background);
            (*current_bar).colors.binding_mode_text = sstrdup(border);
        }
    }
}

pub unsafe fn cfg_bar_socket_path(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    socket_path: *const ::core::ffi::c_char,
) {
    free((*current_bar).socket_path as *mut ::core::ffi::c_void);
    (*current_bar).socket_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).socket_path = sstrdup(socket_path);
}

pub unsafe fn cfg_bar_tray_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: *const ::core::ffi::c_char,
) {
    let tray_output: *mut tray_output_t =
        scalloc(1 as usize, ::core::mem::size_of::<tray_output_t>() as usize) as *mut tray_output_t;
    (*tray_output).output = sstrdup(output);
    (*tray_output).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
    (*tray_output).tray_outputs.tqe_prev = (*current_bar).tray_outputs.tqh_last;
    *(*current_bar).tray_outputs.tqh_last = tray_output;
    (*current_bar).tray_outputs.tqh_last = &raw mut (*tray_output).tray_outputs.tqe_next;
}

pub unsafe fn cfg_bar_tray_padding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    padding_px: ::core::ffi::c_long,
) {
    (*current_bar).tray_padding = padding_px as ::core::ffi::c_int;
}

pub unsafe fn cfg_bar_color_single(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: *const ::core::ffi::c_char,
    color: *const ::core::ffi::c_char,
) {
    if strcmp(colorclass, c"background".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.background = sstrdup(color);
    } else if strcmp(colorclass, c"separator".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.separator = sstrdup(color);
    } else if strcmp(colorclass, c"statusline".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.statusline = sstrdup(color);
    } else if strcmp(colorclass, c"focused_background".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.focused_background = sstrdup(color);
    } else if strcmp(colorclass, c"focused_separator".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.focused_separator = sstrdup(color);
    } else {
        (*current_bar).colors.focused_statusline = sstrdup(color);
    };
}

pub unsafe fn cfg_bar_status_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    free((*current_bar).status_command as *mut ::core::ffi::c_void);
    (*current_bar).status_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).status_command = sstrdup(command);
}

pub unsafe fn cfg_bar_workspace_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    free((*current_bar).workspace_command as *mut ::core::ffi::c_void);
    (*current_bar).workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*current_bar).workspace_command = sstrdup(command);
}

pub unsafe fn cfg_bar_binding_mode_indicator(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    (*current_bar).hide_binding_mode_indicator = !boolstr(value);
}

pub unsafe fn cfg_bar_workspace_buttons(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    (*current_bar).hide_workspace_buttons = !boolstr(value);
}

pub unsafe fn cfg_bar_workspace_min_width(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    width: ::core::ffi::c_long,
) {
    (*current_bar).workspace_min_width = width as ::core::ffi::c_int;
}

pub unsafe fn cfg_bar_strip_workspace_numbers(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    (*current_bar).strip_workspace_numbers = boolstr(value);
}

pub unsafe fn cfg_bar_strip_workspace_name(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: *const ::core::ffi::c_char,
) {
    (*current_bar).strip_workspace_name = boolstr(value);
}

pub unsafe fn cfg_bar_start(current_match: *mut Match, result: *mut ConfigResultIR) {
    current_bar =
        scalloc(1 as usize, ::core::mem::size_of::<Barconfig>() as usize) as *mut Barconfig;
    (*current_bar).bar_bindings.tqh_first = ::core::ptr::null_mut::<Barbinding>();
    (*current_bar).bar_bindings.tqh_last =
        &raw mut (*current_bar).bar_bindings.tqh_first as *mut *mut Barbinding;
    (*current_bar).tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    (*current_bar).tray_outputs.tqh_last =
        &raw mut (*current_bar).tray_outputs.tqh_first as *mut *mut tray_output_t;
    (*current_bar).tray_padding = 2 as ::core::ffi::c_int;
    (*current_bar).modifier = XCB_KEY_BUT_MASK_MOD_4 as ::core::ffi::c_int as u32;
}

pub unsafe fn cfg_bar_finish(current_match: *mut Match, result: *mut ConfigResultIR) {
    debuglog(
        c"%s:%s:%d - \t new bar configuration finished, saving.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/config_directives.c".as_ptr(),
        c"cfg_bar_finish".as_ptr(),
        959 as ::core::ffi::c_int,
    );
    if (*current_bar).id.is_null() {
        sasprintf(
            &raw mut (*current_bar).id,
            c"bar-%d".as_ptr(),
            config.number_barconfigs,
        );
    }
    config.number_barconfigs += 1;
    (*current_bar).configs.tqe_next = ::core::ptr::null_mut::<Barconfig>();
    (*current_bar).configs.tqe_prev = barconfigs.tqh_last;
    *barconfigs.tqh_last = current_bar as *mut Barconfig;
    barconfigs.tqh_last = &raw mut (*current_bar).configs.tqe_next;
    current_bar = ::core::ptr::null_mut::<Barconfig>();
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ipc::ipc_set_kill_timeout;
use crate::util::ws_name_to_number;
use i3_common::boolstr::boolstr;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    XCB_KEY_BUT_MASK_CONTROL, XCB_KEY_BUT_MASK_MOD_1, XCB_KEY_BUT_MASK_MOD_2,
    XCB_KEY_BUT_MASK_MOD_3, XCB_KEY_BUT_MASK_MOD_4, XCB_KEY_BUT_MASK_MOD_5, XCB_KEY_BUT_MASK_SHIFT,
    XCB_NONE,
};

pub use libc::{
    __errno_location, atoi, free, realpath, strcasecmp, strcmp, strerror, strlen, strncasecmp,
    strstr,
};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

use i3_common::native_stdio::{wordexp, wordexp_t, wordfree};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::MatchMatches as C2RustUnnamed_11;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::bindings::DEFAULT_BINDING_MODE;
use crate::config::config;
use crate::r#match::match_copy;
use crate::r#match::match_free;
use crate::r#match::match_init;
use crate::r#match::match_is_empty;
use crate::r#match::match_parse_property;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::font::load_font;
use i3_common::font::set_font;
