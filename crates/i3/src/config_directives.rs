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
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
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

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

struct ExpandedWords(wordexp_t);
impl Drop for ExpandedWords {
    fn drop(&mut self) {
        unsafe { wordfree(&mut self.0) };
    }
}

pub unsafe fn cfg_include(current_match: *mut Match, result: *mut ConfigResultIR, pattern: &[u8]) {
    let pattern = text_prefix(pattern).to_vec();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = pattern.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/config_directives.c:cfg_include:19 - include {}\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "../src/config_directives.c:cfg_include:19 - include ".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let mut p: wordexp_t = wordexp_t {
        we_wordc: 0,
        we_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        we_offs: 0,
    };
    let native_pattern = std::ffi::CString::new(pattern.as_slice()).unwrap();
    let ret: i32 = wordexp(native_pattern.as_ptr(), &raw mut p, 0_i32) as i32;
    if ret != 0_i32 {
        // GNU wordexp may leave partial output on WRDE_NOSPACE (value1 in the
        // supported native ABI); other failed expansions transfer no ownership.
        if ret == 1 {
            wordfree(&mut p);
        }
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = pattern.clone();
            let log_value_1 = (ret) as i32;
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: wordexp({}): error {}\n",
                    log_value_0_text,
                    log_value_1
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: wordexp(".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice(format!("): error {}\n", log_value_1).as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        (*result).has_errors = true;
        return;
    }
    let mut expanded_words = ExpandedWords(p);
    let p = &mut expanded_words.0;
    let w: *mut *mut ::core::ffi::c_char = p.we_wordv;
    let mut i: usize = 0_usize;
    while i < p.we_wordc {
        let mut resolved_path: [::core::ffi::c_char; 4096] = [0; 4096];
        if realpath(
            *w.add(i),
            &raw mut resolved_path as *mut ::core::ffi::c_char,
        )
        .is_null()
        {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = *w.add(i);
                let log_value_0 = i3_common::logging::string_bytes(
                    (if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                let log_pointer_2 = strerror(*__errno_location());
                let log_value_1 = i3_common::logging::string_bytes(
                    (if log_pointer_2.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_2))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                    std::str::from_utf8(&log_value_0),
                    std::str::from_utf8(&log_value_1),
                ) {
                    ::log::info!("Skipping {}: {}\n", log_value_0_text, log_value_1_text);
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("Skipping ".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice(": ".as_bytes());
                    message.extend_from_slice(&log_value_1);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Verbose, &message);
                }
            };
        } else {
            let mut skip: bool = false;
            let mut file: *mut IncludedFile = included_files.tqh_first;

            while !file.is_null() {
                if (*file).path.as_os_str().as_bytes()
                    == CStr::from_ptr(resolved_path.as_ptr()).to_bytes()
                {
                    skip = true;
                    break;
                } else {
                    file = (*file).files.tqe_next;
                }
            }
            if skip {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_pointer_1 = &raw mut resolved_path as *mut ::core::ffi::c_char;
                    let log_value_0 = i3_common::logging::string_bytes(
                        (if log_pointer_1.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(log_pointer_1))
                        })
                        .map(std::ffi::CStr::to_bytes),
                        None,
                    )
                    .to_vec();
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Skipping file {} (already included)\n", log_value_0_text);
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice("Skipping file ".as_bytes());
                        message.extend_from_slice(&log_value_0);
                        message.extend_from_slice(" (already included)\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Verbose, &message);
                    }
                };
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_pointer_1 = &raw mut resolved_path as *mut ::core::ffi::c_char;
                    let log_value_0 = i3_common::logging::string_bytes(
                        (if log_pointer_1.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(log_pointer_1))
                        })
                        .map(std::ffi::CStr::to_bytes),
                        None,
                    )
                    .to_vec();
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Including config file {}\n", log_value_0_text);
                    } else {
                        let mut message = Vec::new();
                        message.extend_from_slice("Including config file ".as_bytes());
                        message.extend_from_slice(&log_value_0);
                        message.extend_from_slice("\n".as_bytes());
                        i3_common::logging::emit(i3_common::logging::Level::Verbose, &message);
                    }
                };
                file = crate::config::append_included_file(std::path::PathBuf::from(
                    std::ffi::OsStr::from_bytes(CStr::from_ptr(resolved_path.as_ptr()).to_bytes()),
                ));
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

                let parse_result = parse_file(
                    &raw mut ctx,
                    std::path::Path::new(std::ffi::OsStr::from_bytes(
                        CStr::from_ptr(resolved_path.as_ptr()).to_bytes(),
                    )),
                    file,
                );
                let current_block_46: u64 = match parse_result {
                    ParseFileResult::Success => 10150597327160359210,
                    ParseFileResult::Failed => {
                        {
                            // Snapshot native bytes before the logger can dispatch a callback.
                            let log_pointer_1 = &raw mut resolved_path as *mut ::core::ffi::c_char;
                            let log_value_0 = i3_common::logging::string_bytes(
                                (if log_pointer_1.is_null() {
                                    None
                                } else {
                                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                                })
                                .map(std::ffi::CStr::to_bytes),
                                None,
                            )
                            .to_vec();
                            let log_pointer_2 = strerror(*__errno_location());
                            let log_value_1 = i3_common::logging::string_bytes(
                                (if log_pointer_2.is_null() {
                                    None
                                } else {
                                    Some(std::ffi::CStr::from_ptr(log_pointer_2))
                                })
                                .map(std::ffi::CStr::to_bytes),
                                None,
                            )
                            .to_vec();
                            if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                                std::str::from_utf8(&log_value_0),
                                std::str::from_utf8(&log_value_1),
                            ) {
                                ::log::error!(
                                    "ERROR: including config file {}: {}\n",
                                    log_value_0_text,
                                    log_value_1_text
                                );
                            } else {
                                let mut message = Vec::new();
                                message
                                    .extend_from_slice("ERROR: including config file ".as_bytes());
                                message.extend_from_slice(&log_value_0);
                                message.extend_from_slice(": ".as_bytes());
                                message.extend_from_slice(&log_value_1);
                                message.extend_from_slice("\n".as_bytes());
                                i3_common::logging::emit(
                                    i3_common::logging::Level::Error,
                                    &message,
                                );
                            }
                        };
                        1383695296304875401
                    }
                    ParseFileResult::ConfigErrors => 1383695296304875401,
                };
                if current_block_46 == 1383695296304875401 {
                    (*result).has_errors = true;
                    crate::config::remove_included_file(file);
                    file = ::core::ptr::null_mut::<IncludedFile>();
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
    {
        let log_value_0 = i3_common::logging::pointer(current_match);
        let log_value_1 = (_state as u32) as i32;
        ::log::debug!("../src/config_directives.c:cfg_criteria_init:101 - Initializing criteria, current_match = {}, state = {}\n", log_value_0, log_value_1);
    };
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
    ctype: &[u8],
    cvalue: Option<&[u8]>,
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

/// Native modifier-name substring policy over bounded application bytes.
pub fn event_state_from_str(value: Option<&[u8]>) -> i3_event_state_mask_t {
    value.map_or(0, event_state_bytes)
}

pub unsafe fn cfg_font(current_match: *mut Match, result: *mut ConfigResultIR, font: &[u8]) {
    let font = text_prefix(font).to_vec();
    crate::config::set_config_font(&font.clone());
}

pub unsafe fn cfg_binding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    bindtype: &[u8],
    modifiers: Option<&[u8]>,
    key: &[u8],
    release: Option<&[u8]>,
    border: Option<&[u8]>,
    whole_window: Option<&[u8]>,
    exclude_titlebar: Option<&[u8]>,
    command: &[u8],
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
        false,
    );
}
static CURRENT_MODE: std::sync::Mutex<Option<(Vec<u8>, bool)>> = std::sync::Mutex::new(None);

pub unsafe fn cfg_mode_binding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    bindtype: &[u8],
    modifiers: Option<&[u8]>,
    key: &[u8],
    release: Option<&[u8]>,
    border: Option<&[u8]>,
    whole_window: Option<&[u8]>,
    exclude_titlebar: Option<&[u8]>,
    command: &[u8],
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
        &mode,
        markup,
    );
}

pub unsafe fn cfg_enter_mode(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    pango_markup: bool,
    modename: &[u8],
) {
    let modename = modename.split(|byte| *byte == 0).next().unwrap().to_vec();
    if modename == DEFAULT_BINDING_MODE {
        {
            let log_value_0 = DEFAULT_BINDING_MODE;
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                ::log::error!(
                    "ERROR: You cannot use the name {} for your mode\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: You cannot use the name ".as_bytes());
                message.extend_from_slice(log_value_0);
                message.extend_from_slice(" for your mode\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    let mut mode: *mut Mode = modes.slh_first;

    while !mode.is_null() {
        if (*mode).name == modename {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = modename.clone();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!(
                        "ERROR: The binding mode with name \"{}\" is defined at least twice.\n",
                        log_value_0_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: The binding mode with name \"".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice("\" is defined at least twice.\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
        }
        mode = (*mode).modes.sle_next;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = modename.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/config_directives.c:cfg_enter_mode:211 - \t now in mode {}\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "../src/config_directives.c:cfg_enter_mode:211 - \t now in mode ".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let mode = modename;
    *CURRENT_MODE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some((mode, pango_markup));
}

pub unsafe fn cfg_exec(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    exectype: &[u8],
    no_startup_id: bool,
    command: &[u8],
) {
    let new = Box::into_raw(Box::new(Autostart::new(command.to_vec(), no_startup_id)));
    if exectype.split(|byte| *byte == 0).next().unwrap() == b"exec" {
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
    command: &[u8],
) {
    let command = text_prefix(command).to_vec();
    if (*current_match).error.is_some() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: match has error: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: match has error: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    if match_is_empty(current_match) {
        ::log::error!("ERROR: Match is empty, ignoring this for_window statement\n");
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = command.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!("../src/config_directives.c:cfg_for_window:237 - \t should execute command {} for the criteria mentioned above\n", log_value_0_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "../src/config_directives.c:cfg_for_window:237 - \t should execute command "
                    .as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice(" for the criteria mentioned above\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::Command(command.clone()),
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}
unsafe fn apply_gaps(gaps: *mut gaps_t, mask: gaps_mask_t, value: i32) {
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
unsafe fn create_gaps_assignment(workspace: &[u8], mask: gaps_mask_t, pixels: i32) {
    if mask.is_empty() {
        return;
    }
    let workspace_owner = workspace.split(|byte| *byte == 0).next().unwrap().to_vec();
    let workspace = workspace_owner.as_slice();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = workspace.to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!("../src/config_directives.c:create_gaps_assignment:271 - Setting gaps for workspace {}", log_value_0_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice("../src/config_directives.c:create_gaps_assignment:271 - Setting gaps for workspace ".as_bytes());
            message.extend_from_slice(&log_value_0);
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let mut found: bool = false;
    let mut assignment: *mut Workspace_Assignment = ws_assignments.tqh_first;

    while !assignment.is_null() {
        if crate::util::text_eq_ignore_case(&(*assignment).name, workspace) {
            found = true;
            break;
        } else {
            assignment = (*assignment).ws_assignments.tqe_next;
        }
    }
    if !found {
        assignment = crate::config::append_workspace_assignment(Workspace_Assignment::new(
            workspace_owner,
            None,
        ));
    }
    (*assignment).gaps_mask |= mask;
    apply_gaps(&raw mut (*assignment).gaps, mask, pixels);
}
unsafe fn gaps_scope_to_mask(scope: &[u8]) -> gaps_mask_t {
    let scope = text_prefix(scope).to_vec();
    if scope == b"inner" {
        return GapMask::INNER;
    }
    if scope == b"outer" {
        return GapMask::OUTER;
    }
    if scope == b"vertical" {
        return GapMask::VERTICAL;
    }
    if scope == b"horizontal" {
        return GapMask::HORIZONTAL;
    }
    if scope == b"top" {
        return GapMask::TOP;
    }
    if scope == b"right" {
        return GapMask::RIGHT;
    }
    if scope == b"bottom" {
        return GapMask::BOTTOM;
    }
    if scope == b"left" {
        return GapMask::LEFT;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = scope.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::error!(
                "ERROR: Invalid command, cannot process scope {}",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice("ERROR: Invalid command, cannot process scope ".as_bytes());
            message.extend_from_slice(&log_value_0);
            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
        }
    };
    GapMask::empty()
}

pub unsafe fn cfg_gaps(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: Option<&[u8]>,
    scope: &[u8],
    value: i64,
) {
    let scope = text_prefix(scope).to_vec();
    let workspace = workspace.map(|bytes| text_prefix(bytes).to_vec());
    let pixels: i32 = logical_px(value as i32) as i32;
    let mask: gaps_mask_t = gaps_scope_to_mask(&scope);
    if let Some(workspace) = workspace.as_deref() {
        create_gaps_assignment(workspace, mask, pixels);
    } else {
        apply_gaps(&raw mut config.gaps, mask, pixels);
    };
}

pub unsafe fn cfg_smart_borders(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    enable: &[u8],
) {
    let enable = text_prefix(enable).to_vec();
    if enable == b"no_gaps" {
        config.hide_edge_borders = HideEdgeBordersMode::SmartNoGaps;
    } else if is_enabled(enable.as_slice()) {
        if config.hide_edge_borders == HideEdgeBordersMode::None {
            config.hide_edge_borders = HideEdgeBordersMode::Smart;
        } else {
            ::log::error!("ERROR: Both hide_edge_borders and smart_borders was used. Ignoring smart_borders as it is deprecated.\n");
        }
    }
}

pub unsafe fn cfg_smart_gaps(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    enable: &[u8],
) {
    let enable = text_prefix(enable).to_vec();
    if enable == b"inverse_outer" {
        config.smart_gaps = SmartGaps::InverseOuter;
    } else {
        config.smart_gaps = if is_enabled(enable.as_slice()) {
            SmartGaps::On
        } else {
            SmartGaps::Off
        };
    };
}

pub unsafe fn cfg_floating_minimum_size(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    width: i64,
    height: i64,
) {
    config.floating_minimum_width = width as i32;
    config.floating_minimum_height = height as i32;
}

pub unsafe fn cfg_floating_maximum_size(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    width: i64,
    height: i64,
) {
    config.floating_maximum_width = width as i32;
    config.floating_maximum_height = height as i32;
}

pub unsafe fn cfg_floating_modifier(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    modifiers: Option<&[u8]>,
) {
    let modifiers = modifiers.map(|bytes| text_prefix(bytes).to_vec());
    config.floating_modifier = event_state_from_str(modifiers.as_deref()) as u32;
}

pub unsafe fn cfg_tiling_drag_swap_modifier(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    modifiers: Option<&[u8]>,
) {
    let modifiers = modifiers.map(|bytes| text_prefix(bytes).to_vec());
    config.swap_modifier = event_state_from_str(modifiers.as_deref()) as u32;
}

pub unsafe fn cfg_default_orientation(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    orientation: &[u8],
) {
    let orientation = text_prefix(orientation).to_vec();
    if orientation == b"horizontal" {
        config.default_orientation = Orientation::Horizontal;
    } else if orientation == b"vertical" {
        config.default_orientation = Orientation::Vertical;
    } else {
        config.default_orientation = Orientation::None;
    };
}

pub unsafe fn cfg_workspace_layout(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    layout: &[u8],
) {
    let layout = text_prefix(layout).to_vec();
    if layout == b"default" {
        config.default_layout = Layout::Default;
    } else if layout == b"stacking" || layout == b"stacked" {
        config.default_layout = Layout::Stacked;
    } else {
        config.default_layout = Layout::Tabbed;
    };
}

pub unsafe fn cfg_default_border(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    windowtype: &[u8],
    border: &[u8],
    width: i64,
) {
    let windowtype = text_prefix(windowtype).to_vec();
    let border = text_prefix(border).to_vec();
    let mut border_style: BorderStyle = BorderStyle::None;
    let mut border_width: i32 = 0;
    if border == b"1pixel" {
        border_style = BorderStyle::Pixel;
        border_width = 1_i32;
    } else if border == b"none" {
        border_style = BorderStyle::None;
        border_width = 0_i32;
    } else if border == b"pixel" {
        border_style = BorderStyle::Pixel;
        border_width = width as i32;
    } else {
        border_style = BorderStyle::Normal;
        border_width = width as i32;
    }
    if windowtype == b"default_border" || windowtype == b"new_window" {
        {
            let log_value_0 = ((border_style) as u32) as i32;
            let log_value_1 = (border_width);
            let log_value_2 = (logical_px(border_width)) as i32;
            ::log::debug!("../src/config_directives.c:cfg_default_border:416 - default tiled border style = {} and border width = {} ({} physical px)\n", log_value_0, log_value_1, log_value_2);
        };
        config.default_border = border_style;
        config.default_border_width = logical_px(border_width);
    } else {
        {
            let log_value_0 = ((border_style) as u32) as i32;
            let log_value_1 = (border_width);
            let log_value_2 = (logical_px(border_width)) as i32;
            ::log::debug!("../src/config_directives.c:cfg_default_border:421 - default floating border style = {} and border width = {} ({} physical px)\n", log_value_0, log_value_1, log_value_2);
        };
        config.default_floating_border = border_style;
        config.default_floating_border_width = logical_px(border_width);
    };
}

pub unsafe fn cfg_hide_edge_borders(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    borders: &[u8],
) {
    let borders = text_prefix(borders).to_vec();
    if borders == b"smart_no_gaps" {
        config.hide_edge_borders = HideEdgeBordersMode::SmartNoGaps;
    } else if borders == b"smart" {
        config.hide_edge_borders = HideEdgeBordersMode::Smart;
    } else if borders == b"vertical" {
        config.hide_edge_borders = HideEdgeBordersMode::Vertical;
    } else if borders == b"horizontal" {
        config.hide_edge_borders = HideEdgeBordersMode::Horizontal;
    } else if borders == b"both" {
        config.hide_edge_borders = HideEdgeBordersMode::Both;
    } else if borders == b"none" {
        config.hide_edge_borders = HideEdgeBordersMode::None;
    } else if is_enabled(borders.as_slice()) {
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
    value: &[u8],
) {
    let value = text_prefix(value).to_vec();
    if value == b"none" {
        config.mouse_warping = Warping::None;
    } else if value == b"output" {
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
    value: &[u8],
) {
    let value = text_prefix(value).to_vec();
    if value == b"force" {
        config.focus_wrapping = FocusWrapping::Force;
    } else if value == b"workspace" {
        config.focus_wrapping = FocusWrapping::Workspace;
    } else if is_enabled(value.as_slice()) {
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
    outputs: &[u8],
) {
    config.fake_outputs = Some(outputs.split(|byte| *byte == 0).next().unwrap().to_vec());
}

pub unsafe fn cfg_force_display_urgency_hint(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    duration_ms: i64,
) {
    config.workspace_urgency_timer = (duration_ms as f64 / 1000.0f64) as f32;
}

pub unsafe fn cfg_focus_on_window_activation(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    mode: &[u8],
) {
    let mode = text_prefix(mode).to_vec();
    if mode == b"smart" {
        config.focus_on_window_activation = FocusActivation::Smart;
    } else if mode == b"urgent" {
        config.focus_on_window_activation = FocusActivation::Urgent;
    } else if mode == b"focus" {
        config.focus_on_window_activation = FocusActivation::Focus;
    } else if mode == b"none" {
        config.focus_on_window_activation = FocusActivation::None;
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = mode.clone();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: Unknown focus_on_window_activation mode \"{}\", ignoring it.\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice(
                    "ERROR: Unknown focus_on_window_activation mode \"".as_bytes(),
                );
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\", ignoring it.\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    {
        let log_value_0 = (config.focus_on_window_activation as u32) as i32;
        ::log::debug!("../src/config_directives.c:cfg_focus_on_window_activation:519 - Set new focus_on_window_activation mode = {}.\n", log_value_0);
    };
}

pub unsafe fn cfg_title_align(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    alignment: &[u8],
) {
    let alignment = text_prefix(alignment).to_vec();
    if alignment == b"left" {
        config.title_align = TitleAlign::Left;
    } else if alignment == b"center" {
        config.title_align = TitleAlign::Center;
    } else if alignment == b"right" {
        config.title_align = TitleAlign::Right;
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/config_directives.c".as_ptr(),
            530_u32,
            c"void cfg_title_align(Match *, struct ConfigResultIR *, const char *)".as_ptr(),
        );
    };
}

fn parse_titlebar_buttons(value: &[u8]) -> Result<crate::types::TitlebarButtons, &'static str> {
    let mut buttons = crate::types::TitlebarButtons::default();
    let mut none = false;
    let mut count = 0;
    for token in text_prefix(value)
        .split(|byte| byte.is_ascii_whitespace())
        .filter(|token| !token.is_empty())
    {
        count += 1;
        if token.eq_ignore_ascii_case(b"none") {
            if none {
                return Err("Duplicate titlebar button option 'none'.");
            }
            if buttons.fullscreen || buttons.close {
                return Err("titlebar_buttons 'none' cannot be combined with buttons.");
            }
            none = true;
        } else if token.eq_ignore_ascii_case(b"fullscreen") {
            if none {
                return Err("titlebar_buttons 'none' cannot be combined with buttons.");
            }
            if buttons.fullscreen {
                return Err("Duplicate titlebar button 'fullscreen'.");
            }
            buttons.fullscreen = true;
        } else if token.eq_ignore_ascii_case(b"close") {
            if none {
                return Err("titlebar_buttons 'none' cannot be combined with buttons.");
            }
            if buttons.close {
                return Err("Duplicate titlebar button 'close'.");
            }
            buttons.close = true;
        } else {
            return Err("Invalid titlebar button; expected fullscreen, close, or none.");
        }
    }
    if count == 0 {
        return Err("titlebar_buttons requires fullscreen, close, or none.");
    }
    Ok(buttons)
}

pub unsafe fn cfg_titlebar_buttons(
    _current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: &[u8],
) {
    match parse_titlebar_buttons(value) {
        Ok(buttons) => config.titlebar_buttons = buttons,
        Err(error) => {
            (*result).has_errors = true;
            ::log::error!("ERROR: CONFIG: {error}\n");
        }
    }
}

pub unsafe fn cfg_show_marks(current_match: *mut Match, result: *mut ConfigResultIR, value: bool) {
    config.show_marks = value;
}
static CURRENT_WORKSPACE: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);

pub unsafe fn cfg_workspace(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: Option<&[u8]>,
    output: Option<&[u8]>,
) {
    let supplied_workspace =
        workspace.map(|name| name.split(|byte| *byte == 0).next().unwrap().to_vec());
    let output_owner = output.map(|name| name.split(|byte| *byte == 0).next().unwrap().to_vec());
    let cached_workspace = CURRENT_WORKSPACE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let workspace = if let Some(workspace) = supplied_workspace {
        *CURRENT_WORKSPACE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        let mut assignment = ws_assignments.tqh_first;
        while !assignment.is_null() {
            if crate::util::text_eq_ignore_case(&(*assignment).name, &workspace)
                && (*assignment).output.is_some()
            {
                if let Ok(name) = std::str::from_utf8(&workspace) {
                    ::log::error!(
                        "ERROR: You have a duplicate workspace assignment for workspace \"{}\"\n",
                        name
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Error,
                        &[
                            b"ERROR: You have a duplicate workspace assignment for workspace \"",
                            &workspace,
                            b"\"\n",
                        ],
                    );
                }
                return;
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
        *CURRENT_WORKSPACE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(workspace.clone());
        workspace
    } else if let Some(workspace) = cached_workspace {
        workspace
    } else {
        ::log::debug!("../src/config_directives.c:cfg_workspace:563 - Both workspace and current_workspace are NULL, assuming we had an error before\n");
        return;
    };
    let output_text = i3_common::logging::string_bytes(output_owner.as_deref(), None);
    if let (Ok(name), Ok(output)) = (
        std::str::from_utf8(&workspace),
        std::str::from_utf8(output_text),
    ) {
        ::log::debug!("../src/config_directives.c:cfg_workspace:569 - Assigning workspace \"{}\" to output \"{}\"\n", name, output);
    } else {
        i3_common::logging::emit_parts(
            i3_common::logging::Level::Debug,
            &[
                b"../src/config_directives.c:cfg_workspace:569 - Assigning workspace \"",
                &workspace,
                b"\" to output \"",
                output_text,
                b"\"\n",
            ],
        );
    }
    crate::config::append_workspace_assignment(Workspace_Assignment::new(
        workspace,
        Some(
            output_owner.expect("workspace output is required when creating a routing assignment"),
        ),
    ));
}

pub unsafe fn cfg_ipc_socket(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    path: &std::path::Path,
) {
    config.ipc_socket_path = Some(path.to_path_buf());
}

pub unsafe fn cfg_restart_state(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    path: &std::path::Path,
) {
    config.restart_state_path = Some(path.to_path_buf());
}

pub unsafe fn cfg_popup_during_fullscreen(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: &[u8],
) {
    let value = text_prefix(value).to_vec();
    if value == b"ignore" {
        config.popup_during_fullscreen = PopupDuringFullscreen::Ignore;
    } else if value == b"leave_fullscreen" {
        config.popup_during_fullscreen = PopupDuringFullscreen::LeaveFullscreen;
    } else if value == b"all" {
        config.popup_during_fullscreen = PopupDuringFullscreen::All;
    } else {
        config.popup_during_fullscreen = PopupDuringFullscreen::Smart;
    };
}

pub unsafe fn cfg_color_single(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: &[u8],
    color: &[u8],
) {
    let colorclass = text_prefix(colorclass).to_vec();
    let color = text_prefix(color).to_vec();
    config.client.background =
        draw_util_hex_to_color(crate::application::root_display_opt(), &color);
}

pub unsafe fn cfg_color(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: &[u8],
    border: &[u8],
    background: &[u8],
    text: &[u8],
    indicator: Option<&[u8]>,
    child_border: Option<&[u8]>,
) {
    let colorclass = text_prefix(colorclass).to_vec();
    let border = text_prefix(border).to_vec();
    let background = text_prefix(background).to_vec();
    let text = text_prefix(text).to_vec();
    let indicator = indicator.map(|bytes| text_prefix(bytes).to_vec());
    let child_border = child_border.map(|bytes| text_prefix(bytes).to_vec());
    // Snapshot every parser argument before any color-error logger can call out.
    let screen = crate::application::root_display_opt();
    if colorclass == b"client.focused_inactive" {
        if false {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.focused_inactive.border = draw_util_hex_to_color(screen, &border);
        config.client.focused_inactive.background = draw_util_hex_to_color(screen, &background);
        config.client.focused_inactive.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.focused_inactive.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.focused_inactive.child_border =
                draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.focused_inactive.child_border = config.client.focused_inactive.background;
        }
        return;
    }
    if colorclass == b"client.focused_tab_title" {
        if true {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.focused_tab_title.border = draw_util_hex_to_color(screen, &border);
        config.client.focused_tab_title.background = draw_util_hex_to_color(screen, &background);
        config.client.focused_tab_title.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.focused_tab_title.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.focused_tab_title.child_border =
                draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.focused_tab_title.child_border =
                config.client.focused_tab_title.background;
        }
        return;
    }
    if colorclass == b"client.focused" {
        if false {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.focused.border = draw_util_hex_to_color(screen, &border);
        config.client.focused.background = draw_util_hex_to_color(screen, &background);
        config.client.focused.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.focused.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.focused.child_border = draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.focused.child_border = config.client.focused.background;
        }
        return;
    }
    if colorclass == b"client.unfocused" {
        if false {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.unfocused.border = draw_util_hex_to_color(screen, &border);
        config.client.unfocused.background = draw_util_hex_to_color(screen, &background);
        config.client.unfocused.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.unfocused.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.unfocused.child_border = draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.unfocused.child_border = config.client.unfocused.background;
        }
        return;
    }
    if colorclass == b"client.urgent" {
        if false {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.urgent.border = draw_util_hex_to_color(screen, &border);
        config.client.urgent.background = draw_util_hex_to_color(screen, &background);
        config.client.urgent.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.urgent.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.urgent.child_border = draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.urgent.child_border = config.client.urgent.background;
        }
        return;
    }
    if colorclass == b"client.placeholder" {
        if false {
            config.client.got_focused_tab_title = true;
            if indicator.is_some() || child_border.is_some() {
                ::log::error!("ERROR: indicator and child_border colors have no effect for client.focused_tab_title\n");
            }
        }
        config.client.placeholder.border = draw_util_hex_to_color(screen, &border);
        config.client.placeholder.background = draw_util_hex_to_color(screen, &background);
        config.client.placeholder.text = draw_util_hex_to_color(screen, &text);
        if let Some(indicator) = indicator.as_deref() {
            config.client.placeholder.indicator = draw_util_hex_to_color(screen, indicator);
        }
        if let Some(child_border) = child_border.as_deref() {
            config.client.placeholder.child_border = draw_util_hex_to_color(screen, child_border);
        } else {
            config.client.placeholder.child_border = config.client.placeholder.background;
        }
    }
}

pub unsafe fn cfg_assign_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: &[u8],
) {
    let output = text_prefix(output).to_vec();
    if (*current_match).error.is_some() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: match has error: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: match has error: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    if match_is_empty(current_match) {
        ::log::error!("ERROR: Match is empty, ignoring this assignment\n");
        return;
    }
    if (*current_match).window_mode != WindowMode::Any {
        ::log::error!("ERROR: Assignments using window mode (floating/tiling) is not supported\n");
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = output.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!("../src/config_directives.c:cfg_assign_output:654 - New assignment, using above criteria, to output \"{}\".\n", log_value_0_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice("../src/config_directives.c:cfg_assign_output:654 - New assignment, using above criteria, to output \"".as_bytes());
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\".\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::Output(output.clone()),
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_assign(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    workspace: &[u8],
    is_number: bool,
) {
    let workspace = text_prefix(workspace).to_vec();
    if (*current_match).error.is_some() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: match has error: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: match has error: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    if match_is_empty(current_match) {
        ::log::error!("ERROR: Match is empty, ignoring this assignment\n");
        return;
    }
    if (*current_match).window_mode != WindowMode::Any {
        ::log::error!("ERROR: Assignments using window mode (floating/tiling) is not supported\n");
        return;
    }
    if is_number && ws_name_to_number(workspace.as_slice()).unwrap_or(-1) == -1_i32 {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = workspace.clone();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: Could not parse initial part of \"{}\" as a number.\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: Could not parse initial part of \"".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\" as a number.\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = workspace.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!("../src/config_directives.c:cfg_assign:682 - New assignment, using above criteria, to workspace \"{}\".\n", log_value_0_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice("../src/config_directives.c:cfg_assign:682 - New assignment, using above criteria, to workspace \"".as_bytes());
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\".\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        if is_number {
            crate::types::AssignmentDest::WorkspaceNumber(workspace.clone())
        } else {
            crate::types::AssignmentDest::Workspace(workspace.clone())
        },
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_no_focus(current_match: *mut Match, result: *mut ConfigResultIR) {
    if (*current_match).error.is_some() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: match has error: {}\n", log_value_0_text);
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: match has error: ".as_bytes());
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    if match_is_empty(current_match) {
        ::log::error!("ERROR: Match is empty, ignoring this assignment\n");
        return;
    }
    ::log::debug!("../src/config_directives.c:cfg_no_focus:700 - New assignment, using above criteria, to ignore focus on manage.\n");
    let assignment: *mut Assignment = Box::into_raw(Box::new(Assignment::new(
        crate::types::AssignmentDest::NoFocus,
        (*current_match).clone(),
    )));
    (*assignment).assignments.tqe_next = ::core::ptr::null_mut::<Assignment>();
    (*assignment).assignments.tqe_prev = assignments.tqh_last;
    *assignments.tqh_last = assignment;
    assignments.tqh_last = &raw mut (*assignment).assignments.tqe_next;
}

pub unsafe fn cfg_ipc_kill_timeout(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    timeout_ms: i64,
) {
    ipc_set_kill_timeout(timeout_ms as ev_tstamp / 1000.0f64);
}

pub unsafe fn cfg_tiling_drag(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    value: &[u8],
) {
    let value = text_prefix(value).to_vec();
    if value == b"modifier" {
        config.tiling_drag = TilingDrag::MODIFIER;
    } else if value == b"titlebar" {
        config.tiling_drag = TilingDrag::TITLEBAR;
    } else if value == b"modifier,titlebar" || value == b"titlebar,modifier" {
        config.tiling_drag = TilingDrag::MODIFIER_OR_TITLEBAR;
    } else {
        config.tiling_drag = TilingDrag::empty();
    };
}
static mut current_bar: *mut Barconfig = std::ptr::null_mut::<Barconfig>();

fn text_prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|&byte| byte == 0).next().unwrap_or_default()
}

pub unsafe fn cfg_bar_font(current_match: *mut Match, result: *mut ConfigResultIR, font: &[u8]) {
    (*current_bar).font = Some(text_prefix(font).to_vec());
}

pub unsafe fn cfg_bar_separator_symbol(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    separator: &[u8],
) {
    (*current_bar).separator_symbol = Some(text_prefix(separator).to_vec());
}

pub unsafe fn cfg_bar_mode(current_match: *mut Match, result: *mut ConfigResultIR, mode: &[u8]) {
    (*current_bar).mode = if text_prefix(mode) == b"dock" {
        BarMode::Dock
    } else {
        if text_prefix(mode) == b"hide" {
            BarMode::Hide
        } else {
            BarMode::Invisible
        }
    };
}

pub unsafe fn cfg_bar_hidden_state(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    hidden_state: &[u8],
) {
    (*current_bar).hidden_state = if text_prefix(hidden_state) == b"hide" {
        BarHiddenState::Hide
    } else {
        BarHiddenState::Show
    };
}

pub unsafe fn cfg_bar_id(current_match: *mut Match, result: *mut ConfigResultIR, bar_id: &[u8]) {
    (*current_bar).id = Some(text_prefix(bar_id).to_vec());
}

pub unsafe fn cfg_bar_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: &[u8],
) {
    (*current_bar).outputs.push(text_prefix(output).to_vec());
}

pub unsafe fn cfg_bar_verbose(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    verbose: bool,
) {
    (*current_bar).verbose = verbose;
}

pub unsafe fn cfg_bar_height(current_match: *mut Match, result: *mut ConfigResultIR, height: i64) {
    (*current_bar).bar_height = height as u32;
}
unsafe fn dlog_padding() {
    {
        let log_value_0 = ((*current_bar).padding.x) as i32;
        let log_value_1 = ((*current_bar).padding.y) as i32;
        let log_value_2 = ((*current_bar).padding.width) as i32;
        let log_value_3 = ((*current_bar).padding.height) as i32;
        ::log::debug!(
            "../src/config_directives.c:dlog_padding:773 - padding now: x={}, y={}, w={}, h={}\n",
            log_value_0,
            log_value_1,
            log_value_2,
            log_value_3
        );
    };
}

pub unsafe fn cfg_bar_padding_one(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    all: i64,
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
    top_and_bottom: i64,
    right_and_left: i64,
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
    top: i64,
    right_and_left: i64,
    bottom: i64,
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
    top: i64,
    right: i64,
    bottom: i64,
    left: i64,
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
    modifiers: Option<&[u8]>,
) {
    (*current_bar).modifier = event_state_from_str(modifiers);
}
unsafe fn bar_configure_binding(button: &[u8], release: bool, command: &[u8]) {
    // Actual libc locale/atoi validation keeps the legacy button syntax.
    let native_button = std::ffi::CString::new(text_prefix(button)).unwrap();
    if strncasecmp(
        native_button.as_ptr(),
        c"button".as_ptr(),
        c"button".count_bytes(),
    ) != 0
    {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = text_prefix(button).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: Bindings for a bar can only be mouse bindings, not \"{}\", ignoring.\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice(
                    "ERROR: Bindings for a bar can only be mouse bindings, not \"".as_bytes(),
                );
                message.extend_from_slice(&log_value_0);
                message.extend_from_slice("\", ignoring.\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    let input_code: i32 = atoi(native_button.as_ptr().add(c"button".count_bytes()));
    if input_code < 1_i32 {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = text_prefix(button).to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: Button \"{}\" does not seem to be in format 'buttonX'.\n",
                    log_value_0_text
                );
            } else {
                let mut message = Vec::new();
                message.extend_from_slice("ERROR: Button \"".as_bytes());
                message.extend_from_slice(&log_value_0);
                message
                    .extend_from_slice("\" does not seem to be in format 'buttonX'.\n".as_bytes());
                i3_common::logging::emit(i3_common::logging::Level::Error, &message);
            }
        };
        return;
    }
    let release_bool = release;
    let mut current: *mut Barbinding = (*current_bar).bar_bindings.tqh_first;

    while !current.is_null() {
        if (*current).input_code == input_code && (*current).release == release_bool {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = text_prefix(button).to_vec();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!(
                        "ERROR: command for button {} was already specified, ignoring.\n",
                        log_value_0_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: command for button ".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice(" was already specified, ignoring.\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            return;
        }
        current = (*current).bindings.tqe_next;
    }
    let new_binding = Box::into_raw(Box::new(Barbinding {
        release: release_bool,
        input_code,
        command: text_prefix(command).to_vec(),
        bindings: crate::application::BarBindingLinks {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: std::ptr::null_mut(),
        },
    }));
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Barbinding>();
    (*new_binding).bindings.tqe_prev = (*current_bar).bar_bindings.tqh_last;
    *(*current_bar).bar_bindings.tqh_last = new_binding;
    (*current_bar).bar_bindings.tqh_last = &raw mut (*new_binding).bindings.tqe_next;
}

pub unsafe fn cfg_bar_wheel_up_cmd(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: &[u8],
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = text_prefix(command).to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::error!(
                "ERROR: 'wheel_up_cmd' is deprecated. Please use 'bindsym button4 {}' instead.\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "ERROR: 'wheel_up_cmd' is deprecated. Please use 'bindsym button4 ".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("' instead.\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
        }
    };
    bar_configure_binding(b"button4", false, command);
}

pub unsafe fn cfg_bar_wheel_down_cmd(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: &[u8],
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = text_prefix(command).to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::error!(
                "ERROR: 'wheel_down_cmd' is deprecated. Please use 'bindsym button5 {}' instead.\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "ERROR: 'wheel_down_cmd' is deprecated. Please use 'bindsym button5 ".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("' instead.\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
        }
    };
    bar_configure_binding(b"button5", false, command);
}

pub unsafe fn cfg_bar_bindsym(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    button: &[u8],
    release: bool,
    command: &[u8],
) {
    bar_configure_binding(button, release, command);
}

pub unsafe fn cfg_bar_position(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    position: &[u8],
) {
    (*current_bar).position = if text_prefix(position) == b"top" {
        BarPosition::Top
    } else {
        BarPosition::Bottom
    };
}

pub unsafe fn cfg_bar_i3bar_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    i3bar_command: &[u8],
) {
    (*current_bar).i3bar_command = Some(text_prefix(i3bar_command).to_vec());
}

pub unsafe fn cfg_bar_color(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: &[u8],
    border: &[u8],
    background: &[u8],
    text: Option<&[u8]>,
) {
    if text_prefix(colorclass) == b"focused_workspace" {
        if let Some(text) = text {
            (*current_bar).colors.focused_workspace_border = Some(text_prefix(border).to_vec());
            (*current_bar).colors.focused_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.focused_workspace_text = Some(text_prefix(text).to_vec());
        } else {
            (*current_bar).colors.focused_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.focused_workspace_text = Some(text_prefix(border).to_vec());
        }
    }
    if text_prefix(colorclass) == b"active_workspace" {
        if let Some(text) = text {
            (*current_bar).colors.active_workspace_border = Some(text_prefix(border).to_vec());
            (*current_bar).colors.active_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.active_workspace_text = Some(text_prefix(text).to_vec());
        } else {
            (*current_bar).colors.active_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.active_workspace_text = Some(text_prefix(border).to_vec());
        }
    }
    if text_prefix(colorclass) == b"inactive_workspace" {
        if let Some(text) = text {
            (*current_bar).colors.inactive_workspace_border = Some(text_prefix(border).to_vec());
            (*current_bar).colors.inactive_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.inactive_workspace_text = Some(text_prefix(text).to_vec());
        } else {
            (*current_bar).colors.inactive_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.inactive_workspace_text = Some(text_prefix(border).to_vec());
        }
    }
    if text_prefix(colorclass) == b"urgent_workspace" {
        if let Some(text) = text {
            (*current_bar).colors.urgent_workspace_border = Some(text_prefix(border).to_vec());
            (*current_bar).colors.urgent_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.urgent_workspace_text = Some(text_prefix(text).to_vec());
        } else {
            (*current_bar).colors.urgent_workspace_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.urgent_workspace_text = Some(text_prefix(border).to_vec());
        }
    }
    if text_prefix(colorclass) == b"binding_mode" {
        if let Some(text) = text {
            (*current_bar).colors.binding_mode_border = Some(text_prefix(border).to_vec());
            (*current_bar).colors.binding_mode_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.binding_mode_text = Some(text_prefix(text).to_vec());
        } else {
            (*current_bar).colors.binding_mode_bg = Some(text_prefix(background).to_vec());
            (*current_bar).colors.binding_mode_text = Some(text_prefix(border).to_vec());
        }
    }
}

pub unsafe fn cfg_bar_socket_path(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    socket_path: &std::path::Path,
) {
    (*current_bar).socket_path = Some(socket_path.to_owned());
}

pub unsafe fn cfg_bar_tray_output(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    output: &[u8],
) {
    let tray_output = Box::into_raw(Box::new(tray_output_t {
        output: text_prefix(output).to_vec(),
        tray_outputs: crate::application::TrayOutputLinks {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: std::ptr::null_mut(),
        },
    }));
    (*tray_output).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
    (*tray_output).tray_outputs.tqe_prev = (*current_bar).tray_outputs.tqh_last;
    *(*current_bar).tray_outputs.tqh_last = tray_output;
    (*current_bar).tray_outputs.tqh_last = &raw mut (*tray_output).tray_outputs.tqe_next;
}

pub unsafe fn cfg_bar_tray_padding(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    padding_px: i64,
) {
    (*current_bar).tray_padding = padding_px as i32;
}

pub unsafe fn cfg_bar_color_single(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    colorclass: &[u8],
    color: &[u8],
) {
    if text_prefix(colorclass) == b"background" {
        (*current_bar).colors.background = Some(text_prefix(color).to_vec());
    } else if text_prefix(colorclass) == b"separator" {
        (*current_bar).colors.separator = Some(text_prefix(color).to_vec());
    } else if text_prefix(colorclass) == b"statusline" {
        (*current_bar).colors.statusline = Some(text_prefix(color).to_vec());
    } else if text_prefix(colorclass) == b"focused_background" {
        (*current_bar).colors.focused_background = Some(text_prefix(color).to_vec());
    } else if text_prefix(colorclass) == b"focused_separator" {
        (*current_bar).colors.focused_separator = Some(text_prefix(color).to_vec());
    } else {
        (*current_bar).colors.focused_statusline = Some(text_prefix(color).to_vec());
    };
}

pub unsafe fn cfg_bar_status_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: &[u8],
) {
    (*current_bar).status_command = Some(text_prefix(command).to_vec());
}

pub unsafe fn cfg_bar_workspace_command(
    current_match: *mut Match,
    result: *mut ConfigResultIR,
    command: &[u8],
) {
    (*current_bar).workspace_command = Some(text_prefix(command).to_vec());
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
    width: i64,
) {
    (*current_bar).workspace_min_width = width as i32;
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
    (*current_bar).bar_bindings.tqh_last = &raw mut (*current_bar).bar_bindings.tqh_first;
    (*current_bar).tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    (*current_bar).tray_outputs.tqh_last = &raw mut (*current_bar).tray_outputs.tqh_first;
    (*current_bar).tray_padding = 2_i32;
    (*current_bar).modifier = (::xcb::x::KeyButMask::MOD4.bits()) as i32 as u32;
}

pub unsafe fn cfg_bar_finish(current_match: *mut Match, result: *mut ConfigResultIR) {
    ::log::debug!("../src/config_directives.c:cfg_bar_finish:959 - \t new bar configuration finished, saving.\n");
    if (*current_bar).id.is_none() {
        (*current_bar).id = Some(format!("bar-{}", config.number_barconfigs).into_bytes());
    }
    config.number_barconfigs += 1;
    (*current_bar).configs.tqe_next = ::core::ptr::null_mut::<Barconfig>();
    (*current_bar).configs.tqe_prev = barconfigs.tqh_last;
    *barconfigs.tqh_last = current_bar;
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

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{
    __errno_location, atoi, free, realpath, strcasecmp, strcmp, strerror, strlen, strncasecmp,
    strstr,
};

use i3_common::native_stdio::__assert_fail;

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
mod titlebar_button_tests {
    use super::parse_titlebar_buttons;
    use crate::types::TitlebarButtons;
    #[test]
    fn button_selection_accepts_individuals_and_rejects_ambiguous_options() {
        assert_eq!(
            parse_titlebar_buttons(b"none"),
            Ok(TitlebarButtons::default())
        );
        assert_eq!(
            parse_titlebar_buttons(b"fullscreen close"),
            Ok(TitlebarButtons {
                fullscreen: true,
                close: true
            })
        );
        assert_eq!(
            parse_titlebar_buttons(b"CLOSE\tfullscreen"),
            Ok(TitlebarButtons {
                fullscreen: true,
                close: true
            })
        );
        assert_eq!(
            parse_titlebar_buttons(b"fullscreen"),
            Ok(TitlebarButtons {
                fullscreen: true,
                close: false
            })
        );
        assert_eq!(
            parse_titlebar_buttons(b"close\0ignored"),
            Ok(TitlebarButtons {
                fullscreen: false,
                close: true
            })
        );
        for input in [
            b"".as_slice(),
            b"unknown",
            b"\xff",
            b"fullscreen fullscreen",
            b"close CLOSE",
            b"none none",
            b"none close",
            b"fullscreen none",
        ] {
            assert!(parse_titlebar_buttons(input).is_err(), "{input:?}");
        }
    }
}

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
        assert_eq!(event_state_from_str(None), 0);
        assert_eq!(event_state_from_str(Some(b"")), 0);
        assert_eq!(
            event_state_from_str(Some(b"Mod1\0Mod4")),
            xcb::x::KeyButMask::MOD1.bits()
        );
    }
}

#[cfg(test)]
mod color_snapshot_tests {
    use super::*;
    #[test]
    fn color_arguments_remain_owned_when_error_logging_drops_the_parser_inputs() {
        if std::env::var_os("I333_CONFIG_COLOR_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "config_directives::color_snapshot_tests::color_arguments_remain_owned_when_error_logging_drops_the_parser_inputs", "--nocapture"])
                .env("I333_CONFIG_COLOR_PROBE", "1")
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
        static INPUTS: std::sync::Mutex<Option<Vec<Vec<u8>>>> = std::sync::Mutex::new(None);
        fn release_inputs(level: i3_common::logging::Level, _: &[u8]) {
            assert_eq!(level, i3_common::logging::Level::Error);
            drop(INPUTS.lock().unwrap().take());
        }
        let values: &[&[u8]] = &[
            b"client.focused",
            b"invalid-\xff",
            b"#112233",
            b"#445566",
            b"#778899",
            b"#abcdef",
        ];
        *INPUTS.lock().unwrap() = Some(values.iter().map(|value| value.to_vec()).collect());
        // The dispatcher owns an independent capture snapshot across callbacks.
        let captures = INPUTS.lock().unwrap().as_ref().unwrap().clone();
        i3_common::logging::set_sink(release_inputs);
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
        let mut result = ConfigResultIR {
            ctx: &mut ctx,
            next_state: ConfigParserState::Initial,
            has_errors: false,
        };
        let mut current = Match::empty();
        unsafe {
            assert!(crate::application::root_display_opt().is_none());
            cfg_color(
                &mut current,
                &mut result,
                &captures[0],
                &captures[1],
                &captures[2],
                &captures[3],
                Some(&captures[4]),
                Some(&captures[5]),
            );
            assert!(INPUTS.lock().unwrap().is_none());
            assert_eq!(config.client.focused.border.colorpixel, 0xffa9a9a9);
            assert_eq!(config.client.focused.background.colorpixel, 0xff112233);
            assert_eq!(config.client.focused.text.colorpixel, 0xff445566);
            assert_eq!(config.client.focused.indicator.colorpixel, 0xff778899);
            assert_eq!(config.client.focused.child_border.colorpixel, 0xffabcdef);
        }
    }
}
