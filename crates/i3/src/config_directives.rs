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
use crate::domains::{DockMatch, HideEdgeBordersMode, MatchInsertWhere};
use crate::flags::{GapMask, KeyboardGroups, TilingDrag};
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
use std::ffi::CStr;
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

pub type i3_event_state_mask_t = u32;

pub type gaps_mask_t = GapMask;

pub use crate::application::Autostart;
pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = TilingDrag;
pub use crate::config::IncludedFile;

pub use crate::application::Barconfig;

pub use crate::application::Barbinding;

pub use crate::application::tray_output_t;
pub use crate::bindings::Mode;

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

struct ExpandedWords(wordexp_t);
impl Drop for ExpandedWords {
    fn drop(&mut self) {
        unsafe { wordfree(&mut self.0) };
    }
}

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
        // GNU wordexp may leave partial output on WRDE_NOSPACE (value1 in the
        // supported native ABI); other failed expansions transfer no ownership.
        if ret == 1 {
            wordfree(&mut p);
        }
        errorlog(
            c"ERROR: wordexp(%s): error %d\n".as_ptr().cast_mut(),
            pattern,
            ret,
        );
        (*result).has_errors = true_0 != 0;
        return;
    }
    let mut expanded_words = ExpandedWords(p);
    let p = &mut expanded_words.0;
    let w: *mut *mut ::core::ffi::c_char = p.we_wordv;
    let mut i: usize = 0 as usize;
    while i < p.we_wordc {
        let mut resolved_path: [::core::ffi::c_char; 4096] = [0; 4096];
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
                let mut stack: stack = stack::default();
                let mut ctx: parser_ctx = parser_ctx {
                    use_nagbar: (*(*result).ctx).use_nagbar,
                    state: ConfigParserState::ForceDisplayUrgencyHintMs,
                    current_match: Match::empty(),
                    statelist: Vec::new(),
                    statelist_idx: 0,
                    stack: &raw mut stack,
                    variables: std::mem::take(&mut (*(*result).ctx).variables),
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
}
static CRITERIA_NEXT_STATE: std::sync::Mutex<ConfigParserState> =
    std::sync::Mutex::new(ConfigParserState::ForceDisplayUrgencyHintMs);

pub unsafe fn cfg_criteria_init(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    _state: ConfigParserState,
) {
    *CRITERIA_NEXT_STATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = _state;
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
    *current_match = Match::empty();
}

pub unsafe fn cfg_criteria_pop_state(current_match: *mut Match, result: *mut ConfigResultIR) {
    (*result).next_state = *CRITERIA_NEXT_STATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
}

pub unsafe fn cfg_criteria_add(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    match_parse_property(current_match, ctype, cvalue);
}

fn event_state_bytes(bytes: &[u8]) -> u32 {
    let bytes = &bytes[..bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len())];
    let flags: &[(&[u8], u32)] = &[
        (b"Mod1", (::xcb::x::KeyButMask::MOD1.bits())),
        (b"Mod2", (::xcb::x::KeyButMask::MOD2.bits())),
        (b"Mod3", (::xcb::x::KeyButMask::MOD3.bits())),
        (b"Mod4", (::xcb::x::KeyButMask::MOD4.bits())),
        (b"Mod5", (::xcb::x::KeyButMask::MOD5.bits())),
        (b"Control", (::xcb::x::KeyButMask::CONTROL.bits())),
        (b"Ctrl", (::xcb::x::KeyButMask::CONTROL.bits())),
        (b"Shift", (::xcb::x::KeyButMask::SHIFT.bits())),
        (b"Group1", KeyboardGroups::GROUP1.state_mask()),
        (b"Group2", KeyboardGroups::GROUP2.state_mask()),
        (b"Mode_switch", KeyboardGroups::GROUP2.state_mask()),
        (b"Group3", KeyboardGroups::GROUP3.state_mask()),
        (b"Group4", KeyboardGroups::GROUP4.state_mask()),
    ];
    flags
        .iter()
        .filter(|(name, _)| bytes.windows(name.len()).any(|window| window == *name))
        .fold(0, |mask, (_, flag)| mask | flag)
}

/// # Safety
/// value is null or a readable terminated string, borrowed only for this call.
pub unsafe fn event_state_from_str(value: *const libc::c_char) -> i3_event_state_mask_t {
    if value.is_null() {
        0
    } else {
        event_state_bytes(std::ffi::CStr::from_ptr(value).to_bytes())
    }
}

pub unsafe fn cfg_font(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    font: *const ::core::ffi::c_char,
) {
    crate::config::set_config_font(CStr::from_ptr(font));
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
static CURRENT_MODE: std::sync::Mutex<Option<(std::ffi::CString, bool)>> =
    std::sync::Mutex::new(None);

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
    let snapshot = CURRENT_MODE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let Some((mode, markup)) = snapshot else {
        return;
    };
    // The owned snapshot survives callbacks without retaining the registry lock.
    configure_binding(
        bindtype,
        modifiers,
        key,
        release,
        border,
        whole_window,
        exclude_titlebar,
        command,
        mode.as_ptr(),
        markup,
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
    let mode = std::ffi::CStr::from_ptr(modename).to_owned();
    *CURRENT_MODE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some((mode, !pango_markup.is_null()));
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
    if (*current_match).error.is_some() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error_ptr(),
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
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::Command(CStr::from_ptr(command).to_owned()),
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
unsafe fn apply_gaps(gaps: *mut gaps_t, mask: gaps_mask_t, value: ::core::ffi::c_int) {
    if gaps.is_null() {
        return;
    }
    if mask.contains(GapMask::INNER) {
        (*gaps).inner = value;
    }
    if mask.contains(GapMask::TOP) {
        (*gaps).top = value;
    }
    if mask.contains(GapMask::RIGHT) {
        (*gaps).right = value;
    }
    if mask.contains(GapMask::BOTTOM) {
        (*gaps).bottom = value;
    }
    if mask.contains(GapMask::LEFT) {
        (*gaps).left = value;
    }
}
unsafe fn create_gaps_assignment(
    workspace: *const ::core::ffi::c_char,
    mask: gaps_mask_t,
    pixels: ::core::ffi::c_int,
) {
    if mask.is_empty() {
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
    (*assignment).gaps_mask |= mask;
    apply_gaps(&raw mut (*assignment).gaps, mask, pixels);
}
unsafe fn gaps_scope_to_mask(scope: *const ::core::ffi::c_char) -> gaps_mask_t {
    if strcmp(scope, c"inner".as_ptr()) == 0 {
        return GapMask::INNER;
    }
    if strcmp(scope, c"outer".as_ptr()) == 0 {
        return GapMask::OUTER;
    }
    if strcmp(scope, c"vertical".as_ptr()) == 0 {
        return GapMask::VERTICAL;
    }
    if strcmp(scope, c"horizontal".as_ptr()) == 0 {
        return GapMask::HORIZONTAL;
    }
    if strcmp(scope, c"top".as_ptr()) == 0 {
        return GapMask::TOP;
    }
    if strcmp(scope, c"right".as_ptr()) == 0 {
        return GapMask::RIGHT;
    }
    if strcmp(scope, c"bottom".as_ptr()) == 0 {
        return GapMask::BOTTOM;
    }
    if strcmp(scope, c"left".as_ptr()) == 0 {
        return GapMask::LEFT;
    }
    errorlog(
        c"ERROR: Invalid command, cannot process scope %s"
            .as_ptr()
            .cast_mut(),
        scope,
    );
    GapMask::empty()
}

pub unsafe fn cfg_gaps(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: *const ::core::ffi::c_char,
    scope: *const ::core::ffi::c_char,
    value: ::core::ffi::c_long,
) {
    let pixels: ::core::ffi::c_int = logical_px(value as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mask: gaps_mask_t = gaps_scope_to_mask(scope);
    if workspace.is_null() {
        apply_gaps(&raw mut config.gaps, mask, pixels);
    } else {
        create_gaps_assignment(workspace, mask, pixels);
    };
}

pub unsafe fn cfg_smart_borders(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    enable: &CStr,
) {
    if enable == c"no_gaps" {
        config.hide_edge_borders = HideEdgeBordersMode::SmartNoGaps;
    } else if is_enabled(enable.to_bytes()) {
        if config.hide_edge_borders == HideEdgeBordersMode::None {
            config.hide_edge_borders = HideEdgeBordersMode::Smart;
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
    enable: &CStr,
) {
    if enable == c"inverse_outer" {
        config.smart_gaps = SmartGaps::InverseOuter;
    } else {
        config.smart_gaps = if is_enabled(enable.to_bytes()) {
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
    borders: &CStr,
) {
    if borders == c"smart_no_gaps" {
        config.hide_edge_borders = HideEdgeBordersMode::SmartNoGaps;
    } else if borders == c"smart" {
        config.hide_edge_borders = HideEdgeBordersMode::Smart;
    } else if borders == c"vertical" {
        config.hide_edge_borders = HideEdgeBordersMode::Vertical;
    } else if borders == c"horizontal" {
        config.hide_edge_borders = HideEdgeBordersMode::Horizontal;
    } else if borders == c"both" {
        config.hide_edge_borders = HideEdgeBordersMode::Both;
    } else if borders == c"none" {
        config.hide_edge_borders = HideEdgeBordersMode::None;
    } else if is_enabled(borders.to_bytes()) {
        config.hide_edge_borders = HideEdgeBordersMode::Vertical;
    } else {
        config.hide_edge_borders = HideEdgeBordersMode::None;
    };
}

pub unsafe fn cfg_focus_follows_mouse(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    config.disable_focus_follows_mouse = !value;
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
    value: bool,
) {
    config.force_xinerama = value;
}

pub unsafe fn cfg_disable_randr15(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    config.disable_randr15 = value;
}

pub unsafe fn cfg_focus_wrapping(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: &CStr,
) {
    if value == c"force" {
        config.focus_wrapping = FocusWrapping::Force;
    } else if value == c"workspace" {
        config.focus_wrapping = FocusWrapping::Workspace;
    } else if is_enabled(value.to_bytes()) {
        config.focus_wrapping = FocusWrapping::On;
    } else {
        config.focus_wrapping = FocusWrapping::Off;
    };
}

pub unsafe fn cfg_force_focus_wrapping(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    if value {
        config.focus_wrapping = FocusWrapping::Force;
    } else if config.focus_wrapping == FocusWrapping::Force {
        config.focus_wrapping = FocusWrapping::On;
    }
}

pub unsafe fn cfg_workspace_back_and_forth(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    config.workspace_auto_back_and_forth = value;
}

pub unsafe fn cfg_fake_outputs(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    outputs: *const ::core::ffi::c_char,
) {
    config.fake_outputs = Some(std::ffi::CStr::from_ptr(outputs).to_owned());
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

pub unsafe fn cfg_show_marks(current_match: *mut Match, result: *mut ConfigResultIR, value: bool) {
    config.show_marks = value;
}
static CURRENT_WORKSPACE: std::sync::Mutex<Option<std::ffi::CString>> = std::sync::Mutex::new(None);

pub unsafe fn cfg_workspace(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    mut workspace: *const ::core::ffi::c_char,
    output: *const ::core::ffi::c_char,
) {
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    let cached_workspace = CURRENT_WORKSPACE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    if !workspace.is_null() {
        *CURRENT_WORKSPACE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
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
        *CURRENT_WORKSPACE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) =
            Some(std::ffi::CStr::from_ptr(workspace).to_owned());
    } else {
        if cached_workspace.is_none() {
            debuglog(
                c"%s:%s:%d - Both workspace and current_workspace are NULL, assuming we had an error before\n".as_ptr().cast_mut(),
                c"../src/config_directives.c".as_ptr(),
                c"cfg_workspace".as_ptr(),
                563 as ::core::ffi::c_int,
            );
            return;
        }
        workspace = cached_workspace.as_ref().unwrap().as_ptr();
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
    config.ipc_socket_path = Some(std::ffi::CStr::from_ptr(path).to_owned());
}

pub unsafe fn cfg_restart_state(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    path: *const ::core::ffi::c_char,
) {
    config.restart_state_path = Some(std::ffi::CStr::from_ptr(path).to_owned());
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
    if (*current_match).error.is_some() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error_ptr(),
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
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::Output(CStr::from_ptr(output).to_owned()),
        (*current_match).clone(),
    )));
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
    if (*current_match).error.is_some() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error_ptr(),
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
        && ws_name_to_number(std::ffi::CStr::from_ptr(workspace)).unwrap_or(-1)
            == -(1 as ::core::ffi::c_int)
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
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        if is_number {
            crate::types::AssignmentDest::WorkspaceNumber(CStr::from_ptr(workspace).to_owned())
        } else {
            crate::types::AssignmentDest::Workspace(CStr::from_ptr(workspace).to_owned())
        },
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment as *mut Assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_no_focus(current_match: *mut Match, result: *mut ConfigResultIR) {
    if (*current_match).error.is_some() {
        errorlog(
            c"ERROR: match has error: %s\n".as_ptr().cast_mut(),
            (*current_match).error_ptr(),
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
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::NoFocus,
        (*current_match).clone(),
    )));
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
        config.tiling_drag = TilingDrag::MODIFIER;
    } else if strcmp(value, c"titlebar".as_ptr()) == 0 as ::core::ffi::c_int {
        config.tiling_drag = TilingDrag::TITLEBAR;
    } else if strcmp(value, c"modifier,titlebar".as_ptr()) == 0 as ::core::ffi::c_int
        || strcmp(value, c"titlebar,modifier".as_ptr()) == 0 as ::core::ffi::c_int
    {
        config.tiling_drag = TilingDrag::MODIFIER_OR_TITLEBAR;
    } else {
        config.tiling_drag = TilingDrag::empty();
    };
}
static mut current_bar: *mut Barconfig = ::core::ptr::null::<Barconfig>() as *mut Barconfig;

pub unsafe fn cfg_bar_font(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    font: *const ::core::ffi::c_char,
) {
    (*current_bar).font = Some(std::ffi::CStr::from_ptr(font).to_owned());
}

pub unsafe fn cfg_bar_separator_symbol(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    separator: *const ::core::ffi::c_char,
) {
    (*current_bar).separator_symbol = Some(std::ffi::CStr::from_ptr(separator).to_owned());
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
    (*current_bar).id = Some(std::ffi::CStr::from_ptr(bar_id).to_owned());
}

pub unsafe fn cfg_bar_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: *const ::core::ffi::c_char,
) {
    (*current_bar)
        .outputs
        .push(std::ffi::CStr::from_ptr(output).to_owned());
}

pub unsafe fn cfg_bar_verbose(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    verbose: bool,
) {
    (*current_bar).verbose = verbose;
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
        0
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
    (*current_bar).i3bar_command = Some(std::ffi::CStr::from_ptr(i3bar_command).to_owned());
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
            (*current_bar).colors.focused_workspace_border =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
            (*current_bar).colors.focused_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.focused_workspace_text =
                Some(std::ffi::CStr::from_ptr(text).to_owned());
        } else {
            (*current_bar).colors.focused_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.focused_workspace_text =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
        }
    }
    if strcmp(colorclass, c"active_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.active_workspace_border =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
            (*current_bar).colors.active_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.active_workspace_text =
                Some(std::ffi::CStr::from_ptr(text).to_owned());
        } else {
            (*current_bar).colors.active_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.active_workspace_text =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
        }
    }
    if strcmp(colorclass, c"inactive_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.inactive_workspace_border =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
            (*current_bar).colors.inactive_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.inactive_workspace_text =
                Some(std::ffi::CStr::from_ptr(text).to_owned());
        } else {
            (*current_bar).colors.inactive_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.inactive_workspace_text =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
        }
    }
    if strcmp(colorclass, c"urgent_workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.urgent_workspace_border =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
            (*current_bar).colors.urgent_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.urgent_workspace_text =
                Some(std::ffi::CStr::from_ptr(text).to_owned());
        } else {
            (*current_bar).colors.urgent_workspace_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.urgent_workspace_text =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
        }
    }
    if strcmp(colorclass, c"binding_mode".as_ptr()) == 0 as ::core::ffi::c_int {
        if !text.is_null() {
            (*current_bar).colors.binding_mode_border =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
            (*current_bar).colors.binding_mode_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.binding_mode_text =
                Some(std::ffi::CStr::from_ptr(text).to_owned());
        } else {
            (*current_bar).colors.binding_mode_bg =
                Some(std::ffi::CStr::from_ptr(background).to_owned());
            (*current_bar).colors.binding_mode_text =
                Some(std::ffi::CStr::from_ptr(border).to_owned());
        }
    }
}

pub unsafe fn cfg_bar_socket_path(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    socket_path: *const ::core::ffi::c_char,
) {
    (*current_bar).socket_path = Some(std::ffi::CStr::from_ptr(socket_path).to_owned());
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
        (*current_bar).colors.background = Some(std::ffi::CStr::from_ptr(color).to_owned());
    } else if strcmp(colorclass, c"separator".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.separator = Some(std::ffi::CStr::from_ptr(color).to_owned());
    } else if strcmp(colorclass, c"statusline".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.statusline = Some(std::ffi::CStr::from_ptr(color).to_owned());
    } else if strcmp(colorclass, c"focused_background".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.focused_background = Some(std::ffi::CStr::from_ptr(color).to_owned());
    } else if strcmp(colorclass, c"focused_separator".as_ptr()) == 0 as ::core::ffi::c_int {
        (*current_bar).colors.focused_separator = Some(std::ffi::CStr::from_ptr(color).to_owned());
    } else {
        (*current_bar).colors.focused_statusline = Some(std::ffi::CStr::from_ptr(color).to_owned());
    };
}

pub unsafe fn cfg_bar_status_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    (*current_bar).status_command = Some(std::ffi::CStr::from_ptr(command).to_owned());
}

pub unsafe fn cfg_bar_workspace_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: *const ::core::ffi::c_char,
) {
    (*current_bar).workspace_command = Some(std::ffi::CStr::from_ptr(command).to_owned());
}

pub unsafe fn cfg_bar_binding_mode_indicator(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    (*current_bar).hide_binding_mode_indicator = !value;
}

pub unsafe fn cfg_bar_workspace_buttons(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    (*current_bar).hide_workspace_buttons = !value;
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
    value: bool,
) {
    (*current_bar).strip_workspace_numbers = value;
}

pub unsafe fn cfg_bar_strip_workspace_name(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: bool,
) {
    (*current_bar).strip_workspace_name = value;
}

pub unsafe fn cfg_bar_start(current_match: *mut Match, result: *mut ConfigResultIR) {
    current_bar = Box::into_raw(Box::new(Barconfig::default()));
    (*current_bar).bar_bindings.tqh_first = ::core::ptr::null_mut::<Barbinding>();
    (*current_bar).bar_bindings.tqh_last =
        &raw mut (*current_bar).bar_bindings.tqh_first as *mut *mut Barbinding;
    (*current_bar).tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    (*current_bar).tray_outputs.tqh_last =
        &raw mut (*current_bar).tray_outputs.tqh_first as *mut *mut tray_output_t;
    (*current_bar).tray_padding = 2 as ::core::ffi::c_int;
    (*current_bar).modifier = (::xcb::x::KeyButMask::MOD4.bits()) as ::core::ffi::c_int as u32;
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
    if (*current_bar).id.is_none() {
        (*current_bar).id =
            Some(std::ffi::CString::new(format!("bar-{}", config.number_barconfigs)).unwrap());
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
use i3_common::boolstr::is_enabled;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

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
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::bindings::DEFAULT_BINDING_MODE;
use crate::config::config;
use crate::r#match::match_is_empty;
use crate::r#match::match_parse_property;
use i3_common::draw_util::draw_util_hex_to_color;

#[cfg(test)]
mod argument_tests {
    use super::*;
    #[test]
    fn modifier_decisions_keep_native_substring_case_and_first_nul_rules() {
        assert_eq!(
            event_state_bytes(b"prefixMod1suffix+Ctrl+Shift"),
            (::xcb::x::KeyButMask::MOD1.bits())
                | (::xcb::x::KeyButMask::CONTROL.bits())
                | (::xcb::x::KeyButMask::SHIFT.bits())
        );
        assert_eq!(event_state_bytes(b"mod1+ctrl+shift"), 0);
        assert_eq!(
            event_state_bytes(b"Mode_switch+Group4"),
            (KeyboardGroups::GROUP2.bits() | KeyboardGroups::GROUP4.bits()) << 16
        );
        assert_eq!(
            event_state_bytes(b"Mod1\0Mod4"),
            (::xcb::x::KeyButMask::MOD1.bits())
        );
        unsafe {
            assert_eq!(event_state_from_str(std::ptr::null()), 0);
        }
    }
}
