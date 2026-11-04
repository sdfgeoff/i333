use crate::commands_parser::CommandParserState;
use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, resolve};
use crate::config::barconfigs;
use crate::domains::{ContainerType, FullscreenMode};
use crate::ipc::ipc_send_barconfig_update_event;
use crate::randr::outputs;
use crate::types::BarHiddenState;
use crate::types::BarMode;
use crate::types::BorderStyle;
use crate::types::ConfigLoad;
use crate::types::Direction;
use crate::types::JsonContent;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::MarkMode;
use crate::types::Orientation;
use crate::types::Position;
use crate::types::ScratchpadState;
use crate::types::ShutdownReason;
use std::os::unix::ffi::OsStrExt;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * commands.c: all command functions (see commands_parser.c)
 *
 */
// Mechanical file-by-file Rust translation of src/commands.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use crate::ipc::{emit_command_reply, CommandReply};

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::Barconfig;

pub use crate::application::ipc_client;

pub use crate::parser_util::ParserStack as stack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindow {
    pub con: *mut Con,
    pub owindows: CommandWindowLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandWindowLinks {
    pub tqe_next: *mut owindow,
    pub tqe_prev: *mut *mut owindow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResultIR {
    pub ctx: *mut cmd_parser_ctx,
    pub client: *mut ipc_client,
    pub next_state: CommandParserState,
    pub needs_tree_render: bool,
}
impl CommandResultIR {
    /// The parser owns the output for the complete dispatch; this is a short
    /// read through its existing context handle, never a stored mutable borrow.
    unsafe fn has_reply(&self) -> bool {
        !self.ctx.is_null() && unsafe { (*self.ctx).json_output.is_some() }
    }
}
#[repr(C)]
pub struct cmd_parser_ctx {
    pub json_output: Option<crate::ipc::CommandReplies>,
    pub state: CommandParserState,
    pub current_match: Match,
    pub stack: stack,
    pub owindows: owindows_head,
    pub subcommand_output: CommandResultIR,
    pub command_output: CommandResultIR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindows_head {
    pub tqh_first: *mut owindow,
    pub tqh_last: *mut *mut owindow,
}
type OutputNames = std::sync::Mutex<Vec<Vec<u8>>>;
static names: OutputNames = std::sync::Mutex::new(Vec::new());
static names_global: OutputNames = std::sync::Mutex::new(Vec::new());
pub type gap_accessor = Option<unsafe fn(*mut gaps_t) -> *mut i32>;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const F_GETFD: i32 = 1_i32;
pub const F_SETFD: i32 = 2_i32;
pub const FD_CLOEXEC: i32 = 1_i32;
unsafe fn maybe_back_and_forth(cmd_output: *mut CommandResultIR, name: &[u8]) -> bool {
    let ws: *const Con = con_get_workspace(focused);
    if (*ws).name.as_deref() != Some(name) {
        return false;
    }
    ::log::debug!(
        "../src/commands.c:maybe_back_and_forth:91 - This workspace is already focused.\n"
    );
    if config.workspace_auto_back_and_forth {
        workspace_back_and_forth();
        (*cmd_output).needs_tree_render = true;
    }
    true
}
unsafe fn maybe_auto_back_and_forth_workspace(workspace: *mut Con) -> *mut Con {
    if !config.workspace_auto_back_and_forth {
        return workspace;
    }
    let current: *const Con = con_get_workspace(focused);
    if std::ptr::eq(current, workspace) {
        let baf: *mut Con = workspace_back_and_forth_get();
        if !baf.is_null() {
            ::log::debug!("../src/commands.c:maybe_auto_back_and_forth_workspace:112 - Substituting workspace with back_and_forth, as it is focused.\n");
            return baf;
        }
    }
    workspace
}

pub unsafe fn cmd_criteria_init(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::debug!(
        "../src/commands.c:cmd_criteria_init:133 - Initializing criteria, current_match = {}\n",
        i3_common::logging::pointer(current_match)
    );
    *current_match = Match::empty();
}

pub unsafe fn cmd_criteria_match_windows(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    ::log::debug!("../src/commands.c:cmd_criteria_match_windows:144 - match specification finished, matching...\n");
    while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
        if !(*ow).owindows.tqe_next.is_null() {
            (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
        } else {
            (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
        }
        *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
        free(ow as *mut ::core::ffi::c_void);
    }
    (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
    (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
    let mut current_block_41: u64;
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(con);
            let log_name_owner_2 = (*con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!("../src/commands.c:cmd_criteria_match_windows:157 - checking if con {} / {} matches\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_criteria_match_windows:157 - checking if con {} / ", log_value_0).as_bytes(), &log_value_1, " matches\n".as_bytes() ]);
            }
        };
        let mut accept_match: bool = false;
        if !(*current_match).con_id.is_null() {
            accept_match = true;
            if (*current_match).con_id == con {
                ::log::debug!(
                    "../src/commands.c:cmd_criteria_match_windows:167 - con_id matched.\n"
                );
                current_block_41 = 15089075282327824602;
            } else {
                ::log::debug!(
                    "../src/commands.c:cmd_criteria_match_windows:169 - con_id does not match.\n"
                );
                current_block_41 = 3512920355445576850;
            }
        } else {
            current_block_41 = 15089075282327824602;
        }
        if current_block_41 == 15089075282327824602 {
            if (*current_match).mark.is_some() && !(*con).marks_head.tqh_first.is_null() {
                accept_match = true;
                let mut matched_by_mark: bool = false;
                let mut mark: *mut mark_t = (*con).marks_head.tqh_first;

                while !mark.is_null() {
                    let mark_name =
                        i3_common::logging::string_bytes(Some(&(*mark).name), None).to_vec();
                    if !regex_matches_bytes((*current_match).mark.as_ref().unwrap(), &mark_name) {
                        mark = (*mark).marks.tqe_next;
                    } else {
                        ::log::debug!(
                            "../src/commands.c:cmd_criteria_match_windows:184 - match by mark\n"
                        );
                        matched_by_mark = true;
                        break;
                    }
                }
                if !matched_by_mark {
                    ::log::debug!(
                        "../src/commands.c:cmd_criteria_match_windows:190 - mark does not match.\n"
                    );
                    current_block_41 = 3512920355445576850;
                } else {
                    current_block_41 = 5494826135382683477;
                }
            } else {
                current_block_41 = 5494826135382683477;
            }
            match current_block_41 {
                3512920355445576850 => {}
                _ => {
                    if !(*con).window.is_null() {
                        if match_matches_window(current_match, (*con).window as *mut i3Window) {
                            ::log::debug!("../src/commands.c:cmd_criteria_match_windows:197 - matches window!\n");
                            accept_match = true;
                            current_block_41 = 8845338526596852646;
                        } else {
                            ::log::debug!("../src/commands.c:cmd_criteria_match_windows:200 - doesn't match\n");
                            current_block_41 = 3512920355445576850;
                        }
                    } else {
                        current_block_41 = 8845338526596852646;
                    }
                    match current_block_41 {
                        3512920355445576850 => {}
                        _ => {
                            if accept_match {
                                {
                                    // Snapshot native bytes before the logger can dispatch a callback.
                                    let log_value_0 = i3_common::logging::pointer(con);
                                    let log_name_owner_2 = (*con).name.clone();
                                    let log_value_1 = i3_common::logging::string_bytes(
                                        log_name_owner_2.as_deref(),
                                        None,
                                    )
                                    .to_vec();
                                    if let (Ok(log_value_1_text),) =
                                        (std::str::from_utf8(&log_value_1),)
                                    {
                                        ::log::debug!("../src/commands.c:cmd_criteria_match_windows:206 - matching: {} / {}\n", log_value_0, log_value_1_text);
                                    } else {
                                        i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_criteria_match_windows:206 - matching: {} / ", log_value_0).as_bytes(), &log_value_1, "\n".as_bytes() ]);
                                    }
                                };
                                let ow_0: *mut owindow =
                                    smalloc(::core::mem::size_of::<owindow>() as usize)
                                        as *mut owindow;
                                (*ow_0).con = con;
                                (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
                                (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
                                *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
                                (*(*cmd_output).ctx).owindows.tqh_last =
                                    &raw mut (*ow_0).owindows.tqe_next;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub unsafe fn cmd_criteria_add(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    match_parse_property(current_match, ctype, cvalue);
}
unsafe fn move_matches_to_workspace(owindows: *const owindows_head, ws: *mut Con) {
    let mut current: *mut owindow = (*owindows).tqh_first;

    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:move_matches_to_workspace:226 - matching: {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/commands.c:move_matches_to_workspace:226 - matching: {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        con_move_to_workspace((*current).con, ws, true, false, false);
        current = (*current).owindows.tqe_next;
    }
}

pub unsafe fn cmd_move_con_to_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = which;
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
            ::log::debug!(
                "../src/commands.c:cmd_move_con_to_workspace:264 - which={}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_move_con_to_workspace:264 - which=".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Nothing to move: specified criteria don't match any window"),
            );
        }
        return;
    } else {
        let mut found: bool = false;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next;
            if ((*(*current).con).type_0) == ContainerType::Workspace
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(b"Nothing to move: workspace empty"),
                );
            }
            return;
        }
    }
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(which, c"next".as_ptr()) == 0_i32 {
        ws = workspace_next();
    } else if strcmp(which, c"prev".as_ptr()) == 0_i32 {
        ws = workspace_prev();
    } else if strcmp(which, c"next_on_output".as_ptr()) == 0_i32 {
        ws = workspace_next_on_output();
    } else if strcmp(which, c"prev_on_output".as_ptr()) == 0_i32 {
        ws = workspace_prev_on_output();
    } else if strcmp(which, c"current".as_ptr()) == 0_i32 {
        ws = con_get_workspace(focused);
    } else {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = which;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message_2 = [b"BUG: called with which=".as_slice(), error_bytes_0].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_2));
        }
        return;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_con_to_workspace_back_and_forth(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    let ws: *mut Con = workspace_back_and_forth_get();
    if ws.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"No workspace was previously active."),
            );
        }
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message_0 = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_con_to_workspace_name(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
    no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if strncasecmp(name, c"__".as_ptr(), c"__".count_bytes()) == 0_i32 {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = name;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message = [
                b"You cannot move containers to i3-internal workspaces (\"".as_slice(),
                error_bytes_0,
                b"\").".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message_0 = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Nothing to move: specified criteria don't match any window"),
            );
        }
        return;
    } else {
        let mut found: bool = false;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next;
            if ((*(*current).con).type_0) == ContainerType::Workspace
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(b"Nothing to move: workspace empty"),
                );
            }
            return;
        }
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = name;
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
            ::log::info!("should move window to workspace {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "should move window to workspace ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut ws: *mut Con = workspace_get(&std::ffi::CStr::from_ptr(name).to_bytes().to_vec());
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_con_to_workspace_number(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
    no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Nothing to move: specified criteria don't match any window"),
            );
        }
        return;
    } else {
        let mut found: bool = false;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next;
            if ((*(*current).con).type_0) == ContainerType::Workspace
                && !con_has_children((*current).con)
            {
                if !(*current).owindows.tqe_next.is_null() {
                    (*(*current).owindows.tqe_next).owindows.tqe_prev =
                        (*current).owindows.tqe_prev;
                } else {
                    (*(*cmd_output).ctx).owindows.tqh_last = (*current).owindows.tqe_prev;
                }
                *(*current).owindows.tqe_prev = (*current).owindows.tqe_next;
            } else {
                found = true;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(b"Nothing to move: workspace empty"),
                );
            }
            return;
        }
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = which;
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
            ::log::info!("should move window to workspace {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "should move window to workspace ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let parsed_num: i64 =
        ws_name_to_number(std::ffi::CStr::from_ptr(which).to_bytes()).unwrap_or(-1) as i64;
    if parsed_num == -1_i32 as i64 {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = which;
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
                ::log::info!(
                    "Could not parse initial part of \"{}\" as a number.\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "Could not parse initial part of \"".as_bytes(),
                        &log_value_0,
                        "\" as a number.\n".as_bytes(),
                    ],
                );
            }
        };
        if (*cmd_output).has_reply() {
            let error_pointer_0 = which;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message_2 = [
                b"Could not parse number \"".as_slice(),
                error_bytes_0,
                b"\"".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_2));
        }
        return;
    }
    let mut ws: *mut Con = get_existing_workspace_by_num(parsed_num as i32);
    if ws.is_null() {
        ws = workspace_get(&std::ffi::CStr::from_ptr(which).to_bytes().to_vec());
    }
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
fn direction_argument(value: &[u8]) -> Option<Direction> {
    match value {
        b"left" => Some(Direction::Left),
        b"right" => Some(Direction::Right),
        b"up" => Some(Direction::Up),
        b"down" => Some(Direction::Down),
        _ => None,
    }
}

fn parse_direction(value: &[u8]) -> Direction {
    let value = value.split(|&byte| byte == 0).next().unwrap();
    if let Some(direction) = direction_argument(value) {
        return direction;
    }
    ::log::error!("ERROR: Invalid direction. This is a parser bug.\n");
    // Preserve the native parser-bug abort and diagnostic rather than unwinding.
    unsafe {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/commands.c".as_ptr(),
            389 as ::core::ffi::c_uint,
            c"direction_t parse_direction(const char *)".as_ptr(),
        )
    }
}
unsafe fn cmd_resize_floating(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
    floating_con: *mut Con,
    mut px: i32,
) {
    let old_rect: Rect = (*floating_con).rect as Rect;
    let focused_con: *const Con = con_descend_focused(floating_con);
    let mut direction: Direction = Direction::Left;
    if strcmp(direction_str, c"height".as_ptr()) == 0_i32 {
        direction = Direction::Down;
    } else if strcmp(direction_str, c"width".as_ptr()) == 0_i32 {
        direction = Direction::Right;
    } else {
        direction = parse_direction(std::ffi::CStr::from_ptr(direction_str).to_bytes());
    }
    let orientation: Orientation = orientation_from_direction(direction);
    let window: *const i3Window = (*focused_con).window;
    if !window.is_null() {
        if orientation == Orientation::Vertical {
            if px < 0_i32 {
                px = if -px < (*window).height_increment {
                    -(*window).height_increment
                } else {
                    px
                };
            } else {
                px = if px < (*window).height_increment {
                    (*window).height_increment
                } else {
                    px
                };
            }
        } else if px < 0_i32 {
            px = if -px < (*window).width_increment {
                -(*window).width_increment
            } else {
                px
            };
        } else {
            px = if px < (*window).width_increment {
                (*window).width_increment
            } else {
                px
            };
        }
    }
    if orientation == Orientation::Vertical {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(px as u32);
    } else {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(px as u32);
    }
    floating_check_size(floating_con, orientation == Orientation::Vertical);
    if rect_equals(old_rect, (*floating_con).rect as Rect) {
        return;
    }
    if direction == Direction::Up {
        (*floating_con).rect.y = (*floating_con)
            .rect
            .y
            .wrapping_sub((*floating_con).rect.height.wrapping_sub(old_rect.height));
    } else if direction == Direction::Left {
        (*floating_con).rect.x = (*floating_con)
            .rect
            .x
            .wrapping_sub((*floating_con).rect.width.wrapping_sub(old_rect.width));
    }
    if (*floating_con).scratchpad_state == ScratchpadState::Fresh {
        (*floating_con).scratchpad_state = ScratchpadState::Changed;
    }
}
unsafe fn cmd_resize_tiling_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    current: *mut Con,
    direction: *const ::core::ffi::c_char,
    mut px: i32,
    ppt: i32,
) -> bool {
    let search_direction: Direction =
        parse_direction(std::ffi::CStr::from_ptr(direction).to_bytes());
    let Ok((first, second)) = resize_find_tiling_participants(current, search_direction, false)
    else {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"No second container found in this direction."),
            );
        }
        return false;
    };
    if ppt != 0 {
        px = 0_i32;
    }
    resize_neighboring_cons(first, second, px, ppt)
}
unsafe fn cmd_resize_tiling_width_height(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    current: *mut Con,
    direction: *const ::core::ffi::c_char,
    px: i32,
    mut ppt: f64,
) -> bool {
    ::log::info!("width/height resize\n");
    let search_direction: Direction = if strcmp(direction, c"width".as_ptr()) == 0_i32 {
        Direction::Left
    } else {
        Direction::Down
    };
    let Ok((current, _)) = resize_find_tiling_participants(current, search_direction, true) else {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(
                    b"Failed to find appropriate tiling containers for resize operation",
                ),
            );
        }
        return false;
    };
    let children: i32 = con_num_children(parent_ptr(current));
    ::log::info!("ins. {} children\n", { (children) });
    let percentage: f64 = 1.0f64 / children as f64;
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::fixed_decimal(
            (percentage),
            i3_common::logging::FixedPrecision::Six,
        );
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("default percentage = {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "default percentage = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut child: *mut Con = nodes::first_ptr(parent_ptr(current));

    while !child.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::fixed_decimal(
                ((*child).percent),
                i3_common::logging::FixedPrecision::Six,
            );
            let log_value_1 = i3_common::logging::pointer(child);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::info!(
                    "child->percent = {} (child {})\n",
                    log_value_0_text,
                    log_value_1
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "child->percent = ".as_bytes(),
                        &log_value_0,
                        format!(" (child {})\n", log_value_1).as_bytes(),
                    ],
                );
            }
        };
        if (*child).percent == 0.0f64 {
            (*child).percent = percentage;
        }
        child = nodes::next_ptr(child);
    }
    let mut new_current_percent: f64 = 0.;
    if ppt != 0.0f64 {
        new_current_percent = (*current).percent + ppt;
    } else {
        ppt = px as f64 / con_rect_size_in_orientation(parent_ptr(current)) as f64;
        new_current_percent = (*current).percent + ppt;
    }
    let subtract_percent: f64 = ppt / (children - 1_i32) as f64;
    if ppt < 0.0f64 && new_current_percent < percent_for_1px(current) {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Not resizing, container would end with less than 1px"),
            );
        }
        return false;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::fixed_decimal(
            (new_current_percent),
            i3_common::logging::FixedPrecision::Six,
        );
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("new_current_percent = {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "new_current_percent = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::fixed_decimal(
            (subtract_percent),
            i3_common::logging::FixedPrecision::Six,
        );
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("subtract_percent = {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "subtract_percent = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if subtract_percent >= 0.0f64 {
        child = nodes::first_ptr(parent_ptr(current));
        while !child.is_null() {
            if !(child == current) && (*child).percent - subtract_percent < percent_for_1px(child) {
                if (*cmd_output).has_reply() {
                    let pointer = i3_common::logging::pointer(child);
                    let prefix = format!("Not resizing, already at minimum size (child {pointer} would end up with a size of ");
                    let percentage = i3_common::native_stdio::fixed_decimal(
                        (*child).percent - subtract_percent,
                        i3_common::native_stdio::FixedPrecision::Zero,
                    );
                    let message_1 = [prefix.as_bytes(), &percentage].concat();
                    emit_command_reply(cmd_output, CommandReply::Error(&message_1));
                }
                return false;
            }
            child = nodes::next_ptr(child);
        }
    }
    (*current).percent = new_current_percent;
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::fixed_decimal(
            ((*current).percent),
            i3_common::logging::FixedPrecision::Six,
        );
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("current->percent after = {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "current->percent after = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    child = nodes::first_ptr(parent_ptr(current));
    while !child.is_null() {
        if !(child == current) {
            (*child).percent -= subtract_percent;
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer(child);
                let log_value_1 = i3_common::logging::fixed_decimal(
                    ((*child).percent),
                    i3_common::logging::FixedPrecision::Six,
                );
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::info!(
                        "child->percent after ({}) = {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Verbose,
                        &[
                            format!("child->percent after ({}) = ", log_value_0).as_bytes(),
                            &log_value_1,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
        }
        child = nodes::next_ptr(child);
    }
    true
}

pub unsafe fn cmd_resize(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    way: *const ::core::ffi::c_char,
    direction: *const ::core::ffi::c_char,
    mut resize_px: i64,
    mut resize_ppt: i64,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = way;
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
        let log_pointer_2 = direction;
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
        let log_value_2 = (resize_px);
        let log_value_3 = (resize_ppt);
        if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
            std::str::from_utf8(&log_value_0),
            std::str::from_utf8(&log_value_1),
        ) {
            ::log::debug!("../src/commands.c:cmd_resize:545 - resizing in way {}, direction {}, px {} or ppt {}\n", log_value_0_text, log_value_1_text, log_value_2, log_value_3);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_resize:545 - resizing in way ".as_bytes(),
                    &log_value_0,
                    ", direction ".as_bytes(),
                    &log_value_1,
                    format!(", px {} or ppt {}\n", log_value_2, log_value_3).as_bytes(),
                ],
            );
        }
    };
    if strcmp(way, c"shrink".as_ptr()) == 0_i32 {
        resize_px *= -1_i32 as i64;
        resize_ppt *= -1_i32 as i64;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        if !(*(*current).con).window.is_null() && (*(*(*current).con).window).dock as u32 != 0 {
            ::log::debug!("../src/commands.c:cmd_resize:557 - This is a dock window. Not resizing (con = {})\n)", i3_common::logging::pointer((*current).con));
        } else {
            let mut floating_con: *mut Con = con_inside_floating((*current).con);

            if !floating_con.is_null() {
                cmd_resize_floating(
                    current_match,
                    cmd_output,
                    direction,
                    floating_con,
                    resize_px as i32,
                );
            } else if strcmp(direction, c"width".as_ptr()) == 0_i32
                || strcmp(direction, c"height".as_ptr()) == 0_i32
            {
                let ppt: f64 = resize_ppt as f64 / 100.0f64;
                if !cmd_resize_tiling_width_height(
                    current_match,
                    cmd_output,
                    (*current).con,
                    direction,
                    resize_px as i32,
                    ppt,
                ) {
                    if (*cmd_output).has_reply() {
                        emit_command_reply(cmd_output, CommandReply::Error(b"Cannot resize."));
                    }
                    return;
                }
            } else if !cmd_resize_tiling_direction(
                current_match,
                cmd_output,
                (*current).con,
                direction,
                resize_px as i32,
                resize_ppt as i32,
            ) {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Error(b"Cannot resize."));
                }
                return;
            }
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
unsafe fn resize_set_tiling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    target: *mut Con,
    resize_orientation: Orientation,
    is_ppt: bool,
    target_size: i64,
) -> bool {
    let mut search_direction: Direction = Direction::Left;
    let mut mode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if resize_orientation == Orientation::Horizontal {
        search_direction = Direction::Left;
        mode = c"width".as_ptr().cast_mut();
    } else {
        search_direction = Direction::Down;
        mode = c"height".as_ptr().cast_mut();
    }
    let target = match resize_find_tiling_participants(target, search_direction, true) {
        Ok((first, _)) | Err(first) => first,
    };
    let mut ppt: f64 = 0.0f64;
    let mut px: i32 = 0_i32;
    if is_ppt {
        ppt = target_size as f64 / 100.0f64 - (*target).percent;
    } else {
        px = (target_size
            - (if resize_orientation == Orientation::Horizontal {
                (*target).rect.width
            } else {
                (*target).rect.height
            }) as i64) as i32;
    }
    cmd_resize_tiling_width_height(current_match, cmd_output, target, mode, px, ppt)
}

pub unsafe fn cmd_resize_set(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut cwidth: i64,
    mode_width: *const ::core::ffi::c_char,
    mut cheight: i64,
    mode_height: *const ::core::ffi::c_char,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = (cwidth);
        let log_pointer_2 = mode_width;
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
        let log_value_2 = (cheight);
        let log_pointer_4 = mode_height;
        let log_value_3 = i3_common::logging::string_bytes(
            (if log_pointer_4.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_4))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        if let (Ok(log_value_1_text), Ok(log_value_3_text)) = (
            std::str::from_utf8(&log_value_1),
            std::str::from_utf8(&log_value_3),
        ) {
            ::log::debug!(
                "../src/commands.c:cmd_resize_set:624 - resizing to {} {} x {} {}\n",
                log_value_0,
                log_value_1_text,
                log_value_2,
                log_value_3_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/commands.c:cmd_resize_set:624 - resizing to {} ",
                        log_value_0
                    )
                    .as_bytes(),
                    &log_value_1,
                    format!(" x {} ", log_value_2).as_bytes(),
                    &log_value_3,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if cwidth < 0_i64 || cheight < 0_i64 {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Dimensions cannot be negative."),
            );
        }
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message_0 = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    let mut success: bool = true;
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        let mut floating_con: *mut Con = con_inside_floating((*current).con);

        if !floating_con.is_null() {
            let output: *const Con = con_get_output(floating_con);
            if cwidth == 0_i64 {
                cwidth = (*floating_con).rect.width as i64;
            } else if !mode_width.is_null() && strcmp(mode_width, c"ppt".as_ptr()) == 0_i32 {
                cwidth = ((*output).rect.width as f64 * (cwidth as f64 / 100.0f64)) as i64;
            }
            if cheight == 0_i64 {
                cheight = (*floating_con).rect.height as i64;
            } else if !mode_height.is_null() && strcmp(mode_height, c"ppt".as_ptr()) == 0_i32 {
                cheight = ((*output).rect.height as f64 * (cheight as f64 / 100.0f64)) as i64;
            }
            floating_resize(floating_con, cwidth as u32, cheight as u32);
        } else if !(*(*current).con).window.is_null()
            && (*(*(*current).con).window).dock as u32 != 0
        {
            ::log::debug!("../src/commands.c:cmd_resize_set:651 - This is a dock window. Not resizing (con = {})\n)", i3_common::logging::pointer((*current).con));
        } else {
            if cwidth > 0_i64 {
                let is_ppt: bool =
                    !mode_width.is_null() && strcmp(mode_width, c"ppt".as_ptr()) == 0_i32;
                success = (success as i32
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        Orientation::Horizontal,
                        is_ppt,
                        cwidth,
                    ) as i32)
                    != 0;
            }
            if cheight > 0_i64 {
                let is_ppt_0: bool =
                    !mode_height.is_null() && strcmp(mode_height, c"ppt".as_ptr()) == 0_i32;
                success = (success as i32
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        Orientation::Vertical,
                        is_ppt_0,
                        cheight,
                    ) as i32)
                    != 0;
            }
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(success));
    }
}
unsafe fn border_width_from_style(
    border_style: BorderStyle,
    border_width: i64,
    con: *mut Con,
) -> i32 {
    if border_style == BorderStyle::None {
        return 0_i32;
    }
    if border_width >= 0_i64 {
        return logical_px(border_width as i32);
    }
    let is_floating: bool = !con_inside_floating(con).is_null();
    if is_floating && border_style == config.default_floating_border {
        config.default_floating_border_width
    } else if !is_floating && border_style == config.default_border {
        config.default_border_width
    } else {
        logical_px(if border_style == BorderStyle::Normal {
            2_i32
        } else {
            1_i32
        })
    }
}

pub unsafe fn cmd_border(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    border_style_str: *const ::core::ffi::c_char,
    border_width: i64,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = border_style_str;
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
        let log_value_1 = (border_width);
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!("../src/commands.c:cmd_border:697 - border style should be changed to {} with border width {}\n", log_value_0_text, log_value_1);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_border:697 - border style should be changed to "
                        .as_bytes(),
                    &log_value_0,
                    format!(" with border width {}\n", log_value_1).as_bytes(),
                ],
            );
        }
    };
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:cmd_border:703 - matching: {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/commands.c:cmd_border:703 - matching: {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        let mut border_style: BorderStyle = BorderStyle::None;
        if strcmp(border_style_str, c"toggle".as_ptr()) == 0_i32 {
            border_style = (*(*current).con).border_style.next();
        } else if strcmp(border_style_str, c"normal".as_ptr()) == 0_i32 {
            border_style = BorderStyle::Normal;
        } else if strcmp(border_style_str, c"pixel".as_ptr()) == 0_i32 {
            border_style = BorderStyle::Pixel;
        } else if strcmp(border_style_str, c"none".as_ptr()) == 0_i32 {
            border_style = BorderStyle::None;
        } else {
            if (*cmd_output).has_reply() {
                let error_pointer_0 = border_style_str;
                let error_bytes_0 = if error_pointer_0.is_null() {
                    b"(null)".as_slice()
                } else {
                    std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
                };
                let message_0 =
                    [b"BUG: called with border_style=".as_slice(), error_bytes_0].concat();
                emit_command_reply(cmd_output, CommandReply::Error(&message_0));
            }
            return;
        }
        (*(*current).con).max_user_border_style = border_style;
        let con_border_width: i32 =
            border_width_from_style(border_style, border_width, (*current).con) as i32;
        con_set_border_style((*current).con, border_style, con_border_width);
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_nop(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    comment: *const ::core::ffi::c_char,
) {
    ::log::info!("-------------------------------------------------\n");
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = comment;
        let log_value_0 = i3_common::logging::string_bytes(
            (if log_pointer_1.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_1))
            })
            .map(std::ffi::CStr::to_bytes),
            Some(4000),
        )
        .to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("  NOP: {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &["  NOP: ".as_bytes(), &log_value_0, "\n".as_bytes()],
            );
        }
    };
    ::log::info!("-------------------------------------------------\n");
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_append_layout(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    cpath: &std::path::Path,
) {
    let cpath = cpath.to_owned();
    let mut content: JsonContent = JsonContent::Unknown;
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut errormsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = cpath.as_os_str().as_bytes().to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("Appending layout \"{}\"\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "Appending layout \"".as_bytes(),
                    &log_value_0,
                    "\"\n".as_bytes(),
                ],
            );
        }
    };
    let path_owner = resolve_or_exit(&cpath);
    let mut bytes = match slurp(&path_owner) {
        Ok(bytes) => bytes,
        Err(_) => {
            if (*cmd_output).has_reply() {
                let mut message = b"Could not slurp \"".to_vec();
                message.extend_from_slice(path_owner.as_os_str().as_bytes());
                message.extend_from_slice(b"\".");
                emit_command_reply(cmd_output, CommandReply::Error(&message));
            }
            return;
        }
    };
    let len = bytes.len();
    // Parser input retains its original byte length, including any embedded NUL.
    bytes.push(0);
    let buf = bytes.as_ptr().cast();
    if !json_validate(buf, len) {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = path_owner.as_os_str().as_bytes().to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!(
                    "ERROR: Could not parse \"{}\" as JSON, not loading.\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Could not parse \"".as_bytes(),
                        &log_value_0,
                        "\" as JSON, not loading.\n".as_bytes(),
                    ],
                );
            }
        };
        if (*cmd_output).has_reply() {
            let mut message_0 = b"Could not parse \"".to_vec();
            message_0.extend_from_slice(path_owner.as_os_str().as_bytes());
            message_0.extend_from_slice(b"\" as JSON.");
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
    } else {
        content = json_determine_content(buf, len);
        ::log::info!("JSON content = {}\n", (content as u32) as i32);
        if content == JsonContent::Unknown {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = path_owner.as_os_str().as_bytes().to_vec();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!(
                        "ERROR: Could not determine the contents of \"{}\", not loading.\n",
                        log_value_0_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Error,
                        &[
                            "ERROR: Could not determine the contents of \"".as_bytes(),
                            &log_value_0,
                            "\", not loading.\n".as_bytes(),
                        ],
                    );
                }
            };
            if (*cmd_output).has_reply() {
                let mut message_1 = b"Could not determine the contents of \"".to_vec();
                message_1.extend_from_slice(path_owner.as_os_str().as_bytes());
                message_1.extend_from_slice(b"\".");
                emit_command_reply(cmd_output, CommandReply::Error(&message_1));
            }
        } else {
            parent = focused;
            if content == JsonContent::Workspace {
                parent = output_get_content(con_get_output(parent));
            } else {
                while ((*parent).type_0) != ContainerType::Workspace && !con_accepts_window(parent)
                {
                    parent = parent_ptr(parent);
                }
            }
            ::log::debug!("../src/commands.c:cmd_append_layout:784 - Appending to parent={} instead of focused={}\n", i3_common::logging::pointer(parent), i3_common::logging::pointer(focused));
            errormsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            tree_append_json(parent, buf, len, &raw mut errormsg);
            if !errormsg.is_null() {
                if (*cmd_output).has_reply() {
                    let error_pointer_0 = errormsg;
                    let error_bytes_0 = if error_pointer_0.is_null() {
                        b"(null)".as_slice()
                    } else {
                        std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
                    };
                    let message_2 = [error_bytes_0].concat();
                    emit_command_reply(cmd_output, CommandReply::Error(&message_2));
                }
                free(errormsg as *mut ::core::ffi::c_void);
            } else if (*cmd_output).has_reply() {
                emit_command_reply(cmd_output, CommandReply::Success(true));
            }
            render_con(croot);
            restore_open_placeholder_windows(parent);
            if content == JsonContent::Workspace {
                ipc_send_workspace_event(
                    c"restored".as_ptr(),
                    parent,
                    ::core::ptr::null_mut::<Con>(),
                );
            }
            (*cmd_output).needs_tree_render = true;
        }
    }
}
unsafe fn disable_global_fullscreen() {
    let fs: *mut Con = con_get_fullscreen_con(croot, FullscreenMode::Global);
    if !fs.is_null() {
        con_disable_fullscreen(fs);
    }
}

pub unsafe fn cmd_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
) {
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = which;
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
            ::log::debug!(
                "../src/commands.c:cmd_workspace:832 - which={}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_workspace:832 - which=".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    disable_global_fullscreen();
    if strcmp(which, c"next".as_ptr()) == 0_i32 {
        ws = workspace_next();
    } else if strcmp(which, c"prev".as_ptr()) == 0_i32 {
        ws = workspace_prev();
    } else if strcmp(which, c"next_on_output".as_ptr()) == 0_i32 {
        ws = workspace_next_on_output();
    } else if strcmp(which, c"prev_on_output".as_ptr()) == 0_i32 {
        ws = workspace_prev_on_output();
    } else {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = which;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message = [b"BUG: called with which=".as_slice(), error_bytes_0].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    workspace_show(ws);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_workspace_number(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
    no_auto_back_and_forth_str: *const ::core::ffi::c_char,
) {
    let no_auto_back_and_forth: bool = !no_auto_back_and_forth_str.is_null();
    disable_global_fullscreen();
    let parsed_num: i64 =
        ws_name_to_number(std::ffi::CStr::from_ptr(which).to_bytes()).unwrap_or(-1) as i64;
    if parsed_num == -1_i32 as i64 {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = which;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message = [
                b"Could not parse initial part of \"".as_slice(),
                error_bytes_0,
                b"\" as a number.".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    let workspace: *mut Con = get_existing_workspace_by_num(parsed_num as i32);
    if workspace.is_null() {
        ::log::info!(
            "There is no workspace with number {}, creating a new one.\n",
            { (parsed_num) }
        );
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        workspace_show_by_name(&std::ffi::CStr::from_ptr(which).to_bytes().to_vec());
        (*cmd_output).needs_tree_render = true;
        return;
    }
    let workspace_name = (*workspace).name.clone().expect("workspace has a name");
    if !no_auto_back_and_forth && maybe_back_and_forth(cmd_output, &workspace_name) {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        return;
    }
    workspace_show(workspace);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_workspace_back_and_forth(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    disable_global_fullscreen();
    workspace_back_and_forth();
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_workspace_name(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
    no_auto_back_and_forth_str: *const ::core::ffi::c_char,
) {
    let no_auto_back_and_forth: bool = !no_auto_back_and_forth_str.is_null();
    if strncasecmp(name, c"__".as_ptr(), c"__".count_bytes()) == 0_i32 {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = name;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message = [
                b"You cannot switch to the i3-internal workspaces (\"".as_slice(),
                error_bytes_0,
                b"\").".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    disable_global_fullscreen();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = name;
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
            ::log::debug!(
                "../src/commands.c:cmd_workspace_name:918 - should switch to workspace {}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_workspace_name:918 - should switch to workspace "
                        .as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if !no_auto_back_and_forth
        && maybe_back_and_forth(
            cmd_output,
            &std::ffi::CStr::from_ptr(name).to_bytes().to_vec(),
        )
    {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        return;
    }
    workspace_show_by_name(&std::ffi::CStr::from_ptr(name).to_bytes().to_vec());
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_mark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: &[u8],
    mode: Option<&[u8]>,
    toggle: Option<&[u8]>,
) {
    let mark = mark.split(|&byte| byte == 0).next().unwrap();
    let mode = mode.map(|value| value.split(|&byte| byte == 0).next().unwrap());
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let current: *const owindow = (*(*cmd_output).ctx).owindows.tqh_first;
    if current.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"Given criteria don't match a window"),
            );
        }
        return;
    }
    if !std::ptr::eq(
        current,
        *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last,
    ) {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"A mark must not be put onto more than one window"),
            );
        }
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer((*current).con);
        let log_name_owner_2 = (*(*current).con).name.clone();
        let log_value_1 =
            i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
            ::log::debug!(
                "../src/commands.c:cmd_mark:949 - matching: {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/commands.c:cmd_mark:949 - matching: {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    &log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mark_mode: MarkMode = if mode.is_none() || mode == Some(b"--replace".as_slice()) {
        MarkMode::Replace
    } else {
        MarkMode::Add
    };
    if toggle.is_some() {
        con_mark_toggle((*current).con, mark, mark_mode);
    } else {
        con_mark((*current).con, mark, mark_mode);
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_unmark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: Option<&[u8]>,
) {
    let mark = mark.map(|value| value.split(|&byte| byte == 0).next().unwrap());
    if match_is_empty(current_match) {
        con_unmark(::core::ptr::null_mut::<Con>(), mark);
    } else {
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

        while !current.is_null() {
            con_unmark((*current).con, mark);
            current = (*current).owindows.tqe_next;
        }
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mode: *const ::core::ffi::c_char,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = mode;
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
            ::log::debug!(
                "../src/commands.c:cmd_mode:987 - mode={}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_mode:987 - mode=".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    switch_mode(&std::ffi::CStr::from_ptr(mode).to_bytes().to_vec());
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
unsafe fn user_output_names_add(list: &OutputNames, name: *const libc::c_char) {
    let get_non_primary = strcasecmp(c"nonprimary".as_ptr(), name) == 0;
    let mut added = Vec::new();
    if get_non_primary || strcmp(name, c"next".as_ptr()) == 0 {
        let mut output = outputs.tqh_first;
        while !output.is_null() {
            if !(get_non_primary && (*output).primary) {
                added.push(output_primary_name(output));
            }
            output = (*output).outputs.tqe_next;
        }
    } else {
        added.push(std::ffi::CStr::from_ptr(name).to_bytes().to_vec());
    }
    // No lock is held while querying native graphs or invoking callbacks.
    list.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .extend(added);
}

unsafe fn user_output_names_find_next(
    list: &OutputNames,
    current_output: *mut Output,
) -> *mut Output {
    let snapshot = list
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    let mut target: *mut Output = std::ptr::null_mut();
    for (index, name) in snapshot.iter().enumerate() {
        if target.is_null() {
            target = get_output_from_string(current_output, name.as_slice());
        }
        if crate::util::text_eq_ignore_case(&output_primary_name(current_output), name) {
            for next in &snapshot[index + 1..] {
                let output = get_output_from_string(current_output, next.as_slice());
                if !output.is_null() {
                    return output;
                }
            }
            break;
        }
    }
    target
}

fn user_output_names_free(list: &OutputNames) {
    list.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
}

pub unsafe fn cmd_move_con_to_output(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
    move_workspace: bool,
) {
    if !name.is_null() {
        user_output_names_add(&names, name);
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if names
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .is_empty()
    {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"At least one output must be specified"),
            );
        }
        return;
    }
    let mut success: bool = false;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        let ws: *mut Con = con_get_workspace((*current).con);
        if !con_is_internal(ws) {
            let current_output: *mut Output = get_output_for_con(ws);
            let target_output: *mut Output = user_output_names_find_next(&names, current_output);
            if !target_output.is_null() {
                if move_workspace {
                    workspace_move_to_output(ws, target_output);
                } else {
                    con_move_to_output((*current).con, target_output, true);
                }
                success = true;
            }
        }
        current = (*current).owindows.tqe_next;
    }
    user_output_names_free(&names);
    (*cmd_output).needs_tree_render = success;
    if success {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    } else if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Error(b"No output matched"));
    }
}

pub unsafe fn cmd_move_con_to_mark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: &[u8],
) {
    let mark = mark.split(|&byte| byte == 0).next().unwrap();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = mark.to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands.c:cmd_move_con_to_mark:1123 - moving window to mark \"{}\"\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_move_con_to_mark:1123 - moving window to mark \""
                        .as_bytes(),
                    &log_value_0,
                    "\"\n".as_bytes(),
                ],
            );
        }
    };
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut result: bool = true;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            let log_value_2 = mark.to_vec();
            if let (Ok(log_value_1_text), Ok(log_value_2_text)) = (
                std::str::from_utf8(&log_value_1),
                std::str::from_utf8(&log_value_2),
            ) {
                ::log::debug!("../src/commands.c:cmd_move_con_to_mark:1130 - moving matched window {} / {} to mark \"{}\"\n", log_value_0, log_value_1_text, log_value_2_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_move_con_to_mark:1130 - moving matched window {} / ", log_value_0).as_bytes(), &log_value_1, " to mark \"".as_bytes(), &log_value_2, "\"\n".as_bytes() ]);
            }
        };
        result = (result as i32 & con_move_to_mark((*current).con, mark) as i32) != 0;
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(result));
    }
}

pub unsafe fn cmd_floating(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    floating_mode: *const ::core::ffi::c_char,
) {
    if !floating_mode.is_null() {
    } else {
        __assert_fail(
            c"floating_mode != NULL".as_ptr(),
            c"../src/commands.c".as_ptr(),
            1143 as ::core::ffi::c_uint,
            c"void cmd_floating(Match *, struct CommandResultIR *, const char *)".as_ptr(),
        );
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = floating_mode;
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
            ::log::debug!(
                "../src/commands.c:cmd_floating:1144 - floating_mode={}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_floating:1144 - floating_mode=".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:cmd_floating:1150 - matching: {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/commands.c:cmd_floating:1150 - matching: {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if strcmp(floating_mode, c"toggle".as_ptr()) == 0_i32 {
            ::log::debug!("../src/commands.c:cmd_floating:1152 - should toggle mode\n");
            toggle_floating_mode((*current).con, false);
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = floating_mode;
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
                    ::log::debug!(
                        "../src/commands.c:cmd_floating:1155 - should switch mode to {}\n",
                        log_value_0_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            "../src/commands.c:cmd_floating:1155 - should switch mode to "
                                .as_bytes(),
                            &log_value_0,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            if strcmp(floating_mode, c"enable".as_ptr()) == 0_i32 {
                floating_enable((*current).con, false);
            } else {
                floating_disable((*current).con);
            }
        }
        run_assignments((*(*current).con).window as *mut i3Window);
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_split(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction: &[u8],
) {
    let direction = direction.split(|&byte| byte == 0).next().unwrap();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = vec![direction.first().copied().unwrap_or(0)];
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::info!("splitting in direction {}\n", log_value_0_text);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[
                    "splitting in direction ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        if con_is_docked((*current).con) {
            ::log::error!("ERROR: Cannot split a docked container, skipping.\n");
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/commands.c:cmd_split:1185 - matching: {} / {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            format!(
                                "../src/commands.c:cmd_split:1185 - matching: {} / ",
                                log_value_0
                            )
                            .as_bytes(),
                            &log_value_1,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            if direction.first() == Some(&b't') {
                let mut current_layout: Layout = Layout::Default;
                if ((*(*current).con).type_0) == ContainerType::Workspace {
                    current_layout = (*(*current).con).layout;
                } else {
                    current_layout = (*parent_ptr((*current).con)).layout;
                }
                if current_layout == Layout::SplitH {
                    tree_split((*current).con, Orientation::Vertical);
                } else {
                    tree_split((*current).con, Orientation::Horizontal);
                }
            } else {
                tree_split(
                    (*current).con,
                    if direction.first() == Some(&b'v') {
                        Orientation::Vertical
                    } else {
                        Orientation::Horizontal
                    },
                );
            }
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_kill(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut kill_mode_str: *const ::core::ffi::c_char,
) {
    if kill_mode_str.is_null() {
        kill_mode_str = c"window".as_ptr();
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = kill_mode_str;
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
            ::log::debug!(
                "../src/commands.c:cmd_kill:1218 - kill_mode={}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_kill:1218 - kill_mode=".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut kill_mode: KillWindow = KillWindow::DontKill;
    if strcmp(kill_mode_str, c"window".as_ptr()) == 0_i32 {
        kill_mode = KillWindow::Window;
    } else if strcmp(kill_mode_str, c"client".as_ptr()) == 0_i32 {
        kill_mode = KillWindow::Client;
    } else {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = kill_mode_str;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message = [b"BUG: called with kill_mode=".as_slice(), error_bytes_0].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message_0 = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        con_close((*current).con, kill_mode);
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_exec(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    nosn: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
) {
    let no_startup_id: bool = !nosn.is_null();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: i32 = 0_i32;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next;
    }
    if count > 1_i32 {
        ::log::info!("WARNING: Your criteria for the exec command match {} containers, so the command will execute this many times.\n", { (count) });
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = command;
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
            let log_value_1 = (no_startup_id as i32);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!(
                    "../src/commands.c:cmd_exec:1264 - should execute {}, no_startup_id = {}\n",
                    log_value_0_text,
                    log_value_1
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        "../src/commands.c:cmd_exec:1264 - should execute ".as_bytes(),
                        &log_value_0,
                        format!(", no_startup_id = {}\n", log_value_1).as_bytes(),
                    ],
                );
            }
        };
        start_application(
            &std::ffi::CStr::from_ptr(command).to_bytes().to_vec(),
            no_startup_id,
            (if (*(*current).con).window.is_null() {
                0
            } else {
                (*(*(*current).con).window).id as i64
            }) as u32,
        );
        current = (*current).owindows.tqe_next;
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: &[u8],
) {
    let direction_str = direction_str.split(|&byte| byte == 0).next().unwrap();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: i32 = 0_i32;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next;
    }
    if count > 1_i32 {
        ::log::info!("WARNING: Your criteria for the focus command matches {} containers, while only exactly one container can be focused at a time.\n", { (count) });
    }
    let mut direction: Direction = Direction::Left;
    let mut position: Position = Position::Before;
    let mut auto_direction: bool = true;
    if direction_str == b"prev" {
        position = Position::Before;
    } else if direction_str == b"next" {
        position = Position::After;
    } else {
        auto_direction = false;
        direction = parse_direction(direction_str);
    }
    let mut current_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws)) {
            if auto_direction {
                let o: Orientation = con_orientation(parent_ptr((*current_0).con));
                direction = direction_from_orientation_position(o, position);
            }
            tree_next((*current_0).con, direction);
        }
        current_0 = (*current_0).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_sibling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: &[u8],
) {
    let direction_str = direction_str.split(|&byte| byte == 0).next().unwrap();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: i32 = 0_i32;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next;
    }
    if count > 1_i32 {
        ::log::info!("WARNING: Your criteria for the focus command matches {} containers, while only exactly one container can be focused at a time.\n", { (count) });
    }
    let direction: Position = if i3_common::safewrappers::strcasecmp_nullable(
        Some(&direction_str[..direction_str.len().min(4)]),
        Some(b"prev"),
    ) == 0
    {
        Position::Before
    } else {
        Position::After
    };
    let mut current_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws)) {
            let next: *mut Con = get_tree_next_sibling((*current_0).con, direction);
            if !next.is_null() {
                if ((*next).type_0) == ContainerType::Workspace {
                    workspace_show(next);
                    con_focus(next);
                } else {
                    con_activate(next);
                }
            }
        }
        current_0 = (*current_0).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_window_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    window_mode: &[u8],
) {
    let window_mode = window_mode.split(|&byte| byte == 0).next().unwrap();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = window_mode.to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands.c:cmd_focus_window_mode:1366 - window_mode = {}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_focus_window_mode:1366 - window_mode = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut to_floating: bool = false;
    if window_mode == b"mode_toggle" {
        to_floating = con_inside_floating(focused).is_null();
    } else if window_mode == b"floating" {
        to_floating = true;
    } else if window_mode == b"tiling" {
        to_floating = false;
    }
    let ws: *const Con = con_get_workspace(focused);
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut success: bool = false;
    current = focus::first_ptr(ws.cast_mut());
    while !current.is_null() {
        if to_floating && ((*current).type_0) != ContainerType::Floating
            || !to_floating && ((*current).type_0) == ContainerType::Floating
        {
            current = focus::next_ptr(current);
        } else {
            con_activate_unblock(con_descend_focused(current));
            success = true;
            break;
        }
    }
    if success {
        (*cmd_output).needs_tree_render = true;
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    } else if (*cmd_output).has_reply() {
        let error_pointer_0 = if to_floating {
            c"floating".as_ptr()
        } else {
            c"tiling".as_ptr()
        };
        let error_bytes_0 = if error_pointer_0.is_null() {
            b"(null)".as_slice()
        } else {
            std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
        };
        let message = [
            b"Failed to find a ".as_slice(),
            error_bytes_0,
            b" container in workspace.".as_slice(),
        ]
        .concat();
        emit_command_reply(cmd_output, CommandReply::Error(&message));
    }
}

pub unsafe fn cmd_focus_level(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    level: &[u8],
) {
    let level = level.split(|&byte| byte == 0).next().unwrap();
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = level.to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands.c:cmd_focus_level:1404 - level = {}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_focus_level:1404 - level = ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut success: bool = false;
    if level == b"parent" {
        if !focused.is_null() && !parent_ptr(focused).is_null() {
            if con_fullscreen_permits_focusing(parent_ptr(focused)) {
                success = level_up();
            } else {
                ::log::error!("ERROR: 'focus parent': Currently in fullscreen, not going up\n");
            }
        }
    } else {
        success = level_down();
    }
    (*cmd_output).needs_tree_render = success;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(success));
    }
}

pub unsafe fn cmd_focus(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    focus_workspace: bool,
) {
    ::log::debug!(
        "../src/commands.c:cmd_focus:1434 - current_match = {}\n",
        i3_common::logging::pointer(current_match)
    );
    if match_is_empty(current_match) {
        ::log::error!("ERROR: You have to specify which window/container should be focused.\n");
        ::log::error!("ERROR: Example: [class=\"urxvt\" title=\"irssi\"] focus\n");
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(
                    b"You have to specify which window/container should be focused",
                ),
            );
        }
        return;
    } else if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"No window matches given criteria"),
            );
        }
        return;
    }
    let mut count: i32 = 0_i32;
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next;
    }
    if count > 1_i32 {
        ::log::info!("WARNING: Your criteria for the focus command matches {} containers, while only exactly one container can be focused at a time.\n", { (count) });
    }
    let mut __i3_scratch: *const Con = workspace_get(b"__i3_scratch");
    let mut current_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !ws.is_null() {
            if std::ptr::eq(ws, __i3_scratch) && !focus_workspace {
                scratchpad_show((*current_0).con);
                break;
            } else if focus_workspace {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = i3_common::logging::pointer((*current_0).con);
                    let log_name_owner_2 = (*(*current_0).con).name.clone();
                    let log_value_1 =
                        i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None)
                            .to_vec();
                    let log_value_2 = i3_common::logging::pointer(ws);
                    let log_name_owner_4 = (*ws).name.clone();
                    let log_value_3 =
                        i3_common::logging::string_bytes(log_name_owner_4.as_deref(), None)
                            .to_vec();
                    if let (Ok(log_value_1_text), Ok(log_value_3_text)) = (
                        std::str::from_utf8(&log_value_1),
                        std::str::from_utf8(&log_value_3),
                    ) {
                        ::log::info!(
                            "focusing workspace {} / {} - {} / {}\n",
                            log_value_0,
                            log_value_1_text,
                            log_value_2,
                            log_value_3_text
                        );
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                format!("focusing workspace {} / ", log_value_0).as_bytes(),
                                &log_value_1,
                                format!(" - {} / ", log_value_2).as_bytes(),
                                &log_value_3,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
                workspace_show(maybe_auto_back_and_forth_workspace(ws));
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = i3_common::logging::pointer((*current_0).con);
                    let log_name_owner_2 = (*(*current_0).con).name.clone();
                    let log_value_1 =
                        i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None)
                            .to_vec();
                    if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                        ::log::info!("focusing {} / {}\n", log_value_0, log_value_1_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                format!("focusing {} / ", log_value_0).as_bytes(),
                                &log_value_1,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
                con_activate_unblock((*current_0).con);
            }
        }
        current_0 = (*current_0).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_fullscreen(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    action: *const ::core::ffi::c_char,
    fullscreen_mode: *const ::core::ffi::c_char,
) {
    let mode: FullscreenMode = (if strcmp(fullscreen_mode, c"global".as_ptr()) == 0_i32 {
        FullscreenMode::Global
    } else {
        FullscreenMode::Output
    });
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = action;
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
        let log_pointer_2 = fullscreen_mode;
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
            ::log::debug!(
                "../src/commands.c:cmd_fullscreen:1490 - {} fullscreen, mode = {}\n",
                log_value_0_text,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_fullscreen:1490 - ".as_bytes(),
                    &log_value_0,
                    " fullscreen, mode = ".as_bytes(),
                    &log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:cmd_fullscreen:1496 - matching: {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/commands.c:cmd_fullscreen:1496 - matching: {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if strcmp(action, c"toggle".as_ptr()) == 0_i32 {
            con_toggle_fullscreen((*current).con, mode);
        } else if strcmp(action, c"enable".as_ptr()) == 0_i32 {
            con_enable_fullscreen((*current).con, mode);
        } else if strcmp(action, c"disable".as_ptr()) == 0_i32 {
            con_disable_fullscreen((*current).con);
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_sticky(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    action: *const ::core::ffi::c_char,
) {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = action;
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
            ::log::debug!(
                "../src/commands.c:cmd_sticky:1516 - {} sticky on window\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_sticky:1516 - ".as_bytes(),
                    &log_value_0,
                    " sticky on window\n".as_bytes(),
                ],
            );
        }
    };
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        if (*(*current).con).window.is_null() {
            ::log::error!(
                "ERROR: only containers holding a window can be made sticky, skipping con = {}\n",
                i3_common::logging::pointer((*current).con)
            );
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!("../src/commands.c:cmd_sticky:1525 - setting sticky for container = {} / {}\n", log_value_0, log_value_1_text);
                } else {
                    i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_sticky:1525 - setting sticky for container = {} / ", log_value_0).as_bytes(), &log_value_1, "\n".as_bytes() ]);
                }
            };
            let mut sticky: bool = false;
            if strcmp(action, c"enable".as_ptr()) == 0_i32 {
                sticky = true;
            } else if strcmp(action, c"disable".as_ptr()) == 0_i32 {
                sticky = false;
            } else if strcmp(action, c"toggle".as_ptr()) == 0_i32 {
                sticky = !(*(*current).con).sticky;
            }
            (*(*current).con).sticky = sticky;
            ewmh_update_sticky((*(*(*current).con).window).id, sticky);
        }
        current = (*current).owindows.tqe_next;
    }
    output_push_sticky_windows(focused);
    ewmh_update_wm_desktop();
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
    amount: i64,
    mode: *const ::core::ffi::c_char,
) {
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let direction: Direction = parse_direction(std::ffi::CStr::from_ptr(direction_str).to_bytes());
    let is_ppt: bool = !mode.is_null() && strcmp(mode, c"ppt".as_ptr()) == 0_i32;
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = direction_str;
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
        let log_value_1 = (amount);
        let log_pointer_3 = mode;
        let log_value_2 = i3_common::logging::string_bytes(
            (if log_pointer_3.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_3))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        if let (Ok(log_value_0_text), Ok(log_value_2_text)) = (
            std::str::from_utf8(&log_value_0),
            std::str::from_utf8(&log_value_2),
        ) {
            ::log::debug!(
                "../src/commands.c:cmd_move_direction:1562 - moving in direction {}, {} {}\n",
                log_value_0_text,
                log_value_1,
                log_value_2_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_move_direction:1562 - moving in direction ".as_bytes(),
                    &log_value_0,
                    format!(", {} ", log_value_1).as_bytes(),
                    &log_value_2,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        if con_is_floating((*current).con) {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (amount);
                let log_pointer_2 = mode;
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
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/commands.c:cmd_move_direction:1565 - floating move with {} {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_move_direction:1565 - floating move with {} ", log_value_0).as_bytes(), &log_value_1, "\n".as_bytes() ]);
                }
            };
            let mut newrect: Rect = (*parent_ptr((*current).con)).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            match direction {
                Direction::Left => {
                    newrect.x = (newrect.x as f64
                        - if is_ppt {
                            (*output).rect.width as f64 * (amount as f64 / 100.0f64)
                        } else {
                            amount as f64
                        }) as u32;
                }
                Direction::Right => {
                    newrect.x = (newrect.x as f64
                        + if is_ppt {
                            (*output).rect.width as f64 * (amount as f64 / 100.0f64)
                        } else {
                            amount as f64
                        }) as u32;
                }
                Direction::Up => {
                    newrect.y = (newrect.y as f64
                        - if is_ppt {
                            (*output).rect.height as f64 * (amount as f64 / 100.0f64)
                        } else {
                            amount as f64
                        }) as u32;
                }
                Direction::Down => {
                    newrect.y = (newrect.y as f64
                        + if is_ppt {
                            (*output).rect.height as f64 * (amount as f64 / 100.0f64)
                        } else {
                            amount as f64
                        }) as u32;
                }
            }
            (*cmd_output).needs_tree_render =
                floating_reposition(parent_ptr((*current).con), newrect);
        } else {
            tree_move((*current).con, direction);
            (*cmd_output).needs_tree_render = true;
        }
        current = (*current).owindows.tqe_next;
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_layout(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    layout_str: *const ::core::ffi::c_char,
) {
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let Some(layout) = layout_from_name(std::ffi::CStr::from_ptr(layout_str).to_bytes()) else {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = layout_str;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message_0 = [
                b"Unknown layout \"".as_slice(),
                error_bytes_0,
                b"\", this is a mismatch between code and parser spec.".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = layout_str;
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
        let log_value_1 = (layout as u32) as i32;
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands.c:cmd_layout:1608 - changing layout to {} ({})\n",
                log_value_0_text,
                log_value_1
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_layout:1608 - changing layout to ".as_bytes(),
                    &log_value_0,
                    format!(" ({})\n", log_value_1).as_bytes(),
                ],
            );
        }
    };
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        if con_is_docked((*current).con) {
            ::log::error!("ERROR: cannot change layout of a docked container, skipping it.\n");
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/commands.c:cmd_layout:1617 - matching: {} / {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            format!(
                                "../src/commands.c:cmd_layout:1617 - matching: {} / ",
                                log_value_0
                            )
                            .as_bytes(),
                            &log_value_1,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            con_set_layout((*current).con, layout);
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_layout_toggle(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut toggle_mode: *const ::core::ffi::c_char,
) {
    if toggle_mode.is_null() {
        toggle_mode = c"default".as_ptr();
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = toggle_mode;
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
            ::log::debug!(
                "../src/commands.c:cmd_layout_toggle:1635 - toggling layout (mode = {})\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_layout_toggle:1635 - toggling layout (mode = "
                        .as_bytes(),
                    &log_value_0,
                    ")\n".as_bytes(),
                ],
            );
        }
    };
    if match_is_empty(current_match) {
        con_toggle_layout(focused, toggle_mode);
    } else {
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

        while !current.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/commands.c:cmd_layout_toggle:1643 - matching: {} / {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            format!(
                                "../src/commands.c:cmd_layout_toggle:1643 - matching: {} / ",
                                log_value_0
                            )
                            .as_bytes(),
                            &log_value_1,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            con_toggle_layout((*current).con, toggle_mode);
            current = (*current).owindows.tqe_next;
        }
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_exit(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::info!("Exiting due to user command.\n");
    exit(EXIT_SUCCESS);
}

pub unsafe fn cmd_reload(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::info!("reloading\n");
    kill_nagbar(config_error_nagbar_pid, false);
    kill_nagbar(command_error_nagbar_pid, false);
    command_error_nagbar_pid = -1_i32 as libc::pid_t;
    config_error_nagbar_pid = command_error_nagbar_pid;
    load_configuration(None, ConfigLoad::Reload);
    x_set_i3_atoms();
    ipc_send_workspace_event(
        c"reload".as_ptr(),
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<Con>(),
    );
    let mut current: *mut Barconfig = barconfigs.tqh_first;

    while !current.is_null() {
        ipc_send_barconfig_update_event(current);
        current = (*current).configs.tqe_next;
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_restart(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::info!("restarting i3\n");
    let mut exempt_fd: i32 = -1_i32;
    if !(*cmd_output).client.is_null() {
        exempt_fd = (*(*cmd_output).client).fd;
        ::log::info!("Carrying file descriptor {} across restart\n", {
            (exempt_fd)
        });
        let mut flags: i32 = 0;
        flags = fcntl(exempt_fd, F_GETFD);
        if flags < 0_i32 || fcntl(exempt_fd, F_SETFD, flags & !FD_CLOEXEC) < 0_i32 {
            ::log::error!("ERROR: Could not disable FD_CLOEXEC on fd {}\n", {
                (exempt_fd)
            });
        }
        let fdstr = std::ffi::CString::new(exempt_fd.to_string())
            .expect("a decimal file descriptor contains no NUL");
        setenv(
            c"_I3_RESTART_FD".as_ptr(),
            fdstr.as_ptr(),
            1 as ::core::ffi::c_int,
        );
    }
    ipc_shutdown(ShutdownReason::Restart, exempt_fd);
    if let Some(path) = config.ipc_socket_path.as_deref() {
        let _ = std::fs::remove_file(path);
    }
    if let Some(path) = current_log_stream_socket_path.as_deref() {
        let _ = std::fs::remove_file(path);
    }
    purge_zerobyte_logfile();
    i3_restart(false);
    __assert_fail(
        c"false".as_ptr(),
        c"../src/commands.c".as_ptr(),
        1723 as ::core::ffi::c_uint,
        c"void cmd_restart(Match *, struct CommandResultIR *)".as_ptr(),
    );
}

pub unsafe fn cmd_open(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::info!("opening new container\n");
    let con: *mut Con = tree_open_con(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*con).layout = Layout::SplitH;
    con_activate(con);
    emit_command_reply(cmd_output, CommandReply::WithId(con as usize as i64));
    (*cmd_output).needs_tree_render = true;
}

pub unsafe fn cmd_focus_output(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
) {
    if !name.is_null() {
        user_output_names_add(&names_global, name);
        return;
    }
    if names_global
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .is_empty()
    {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"At least one output must be specified"),
            );
        }
        return;
    }
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message_0 = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        return;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    let mut con: *mut Con = focused;
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        if !con_is_internal(con_get_workspace((*current).con)) {
            con = (*current).con;
            break;
        } else {
            current = (*current).owindows.tqe_next;
        }
    }
    let current_output: *mut Output = get_output_for_con(con);
    let target_output: *const Output = user_output_names_find_next(&names_global, current_output);
    user_output_names_free(&names_global);
    let mut success: bool = false;
    if !target_output.is_null() {
        success = true;
        let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -1_i32 as *mut Con;
        while child == -1_i32 as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = nodes::first_ptr(output_get_content((*target_output).con));
            while !child.is_null() {
                if !workspace_is_visible(child) {
                    child = nodes::next_ptr(child);
                } else {
                    ws = child;
                    break;
                }
            }
        }
        if ws.is_null() {
            if (*cmd_output).has_reply() {
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(b"BUG: No workspace found on output."),
                );
            }
            return;
        }
        workspace_show(ws);
    }
    (*cmd_output).needs_tree_render = success;
    if success {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    } else if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Error(b"No output matched"));
    }
}

pub unsafe fn cmd_move_window_to_position(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    x: i64,
    mode_x: *const ::core::ffi::c_char,
    y: i64,
    mode_y: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        if !con_is_floating((*current).con) {
            ::log::error!("ERROR: Cannot change position. The window/container is not floating\n");
            if !has_error {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Error(b"Cannot change position of a window/container because it is not floating."));
                }
                has_error = true;
            }
        } else {
            let mut newrect: Rect = (*parent_ptr((*current).con)).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            newrect.x = (if !mode_x.is_null() && strcmp(mode_x, c"ppt".as_ptr()) == 0_i32 {
                (*output).rect.width as f64 * (x as f64 / 100.0f64)
            } else {
                x as f64
            }) as u32;
            newrect.y = (if !mode_y.is_null() && strcmp(mode_y, c"ppt".as_ptr()) == 0_i32 {
                (*output).rect.height as f64 * (y as f64 / 100.0f64)
            } else {
                y as f64
            }) as u32;
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (newrect.x) as i32;
                let log_pointer_2 = mode_x;
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
                let log_value_2 = (newrect.y) as i32;
                let log_pointer_4 = mode_y;
                let log_value_3 = i3_common::logging::string_bytes(
                    (if log_pointer_4.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_4))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_1_text), Ok(log_value_3_text)) = (
                    std::str::from_utf8(&log_value_1),
                    std::str::from_utf8(&log_value_3),
                ) {
                    ::log::debug!("../src/commands.c:cmd_move_window_to_position:1835 - moving to position {} {} {} {}\n", log_value_0, log_value_1_text, log_value_2, log_value_3_text);
                } else {
                    i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_move_window_to_position:1835 - moving to position {} ", log_value_0).as_bytes(), &log_value_1, format!(" {} ", log_value_2).as_bytes(), &log_value_3, "\n".as_bytes() ]);
                }
            };
            if !floating_reposition(parent_ptr((*current).con), newrect) {
                if (*cmd_output).has_reply() {
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(b"Cannot move window/container out of bounds."),
                    );
                }
                has_error = true;
            }
        }
        current = (*current).owindows.tqe_next;
    }
    if !has_error && (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_window_to_center(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    method: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false;
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        let floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::error!(
                        "ERROR: con {} / {} is not floating, cannot move it to the center.\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Error,
                        &[
                            format!("ERROR: con {} / ", log_value_0).as_bytes(),
                            &log_value_1,
                            " is not floating, cannot move it to the center.\n".as_bytes(),
                        ],
                    );
                }
            };
            if !has_error {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Error(b"Cannot change position of a window/container because it is not floating."));
                }
                has_error = true;
            }
        } else {
            if strcmp(method, c"absolute".as_ptr()) == 0_i32 {
                ::log::debug!("../src/commands.c:cmd_move_window_to_center:1872 - moving to absolute center\n");
                floating_center(floating_con, (*croot).rect as Rect);
                floating_maybe_reassign_ws(floating_con);
                (*cmd_output).needs_tree_render = true;
            }
            if strcmp(method, c"position".as_ptr()) == 0_i32 {
                ::log::debug!(
                    "../src/commands.c:cmd_move_window_to_center:1880 - moving to center\n"
                );
                floating_center(
                    floating_con,
                    (*con_get_workspace(floating_con)).rect as Rect,
                );
                (*cmd_output).needs_tree_render = true;
            }
        }
        current = (*current).owindows.tqe_next;
    }
    if !has_error && (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_window_to_mouse(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        let floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!("../src/commands.c:cmd_move_window_to_mouse:1905 - con {} / {} is not floating, cannot move it to the mouse position.\n", log_value_0, log_value_1_text);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            format!(
                                "../src/commands.c:cmd_move_window_to_mouse:1905 - con {} / ",
                                log_value_0
                            )
                            .as_bytes(),
                            &log_value_1,
                            " is not floating, cannot move it to the mouse position.\n".as_bytes(),
                        ],
                    );
                }
            };
        } else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer(floating_con);
                let log_name_owner_2 = (*floating_con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!("../src/commands.c:cmd_move_window_to_mouse:1909 - moving floating container {} / {} to cursor position\n", log_value_0, log_value_1_text);
                } else {
                    i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_move_window_to_mouse:1909 - moving floating container {} / ", log_value_0).as_bytes(), &log_value_1, " to cursor position\n".as_bytes() ]);
                }
            };
            floating_move_to_pointer(floating_con);
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_scratchpad(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::debug!(
        "../src/commands.c:cmd_move_scratchpad:1922 - should move window to scratchpad\n"
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first;
    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:cmd_move_scratchpad:1928 - matching: {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/commands.c:cmd_move_scratchpad:1928 - matching: {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        scratchpad_move((*current).con);
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_scratchpad_show(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    ::log::debug!("../src/commands.c:cmd_scratchpad_show:1942 - should show scratchpad window\n");
    let mut result: bool = false;
    if match_is_empty(current_match) {
        result = scratchpad_show(::core::ptr::null_mut::<Con>());
    } else {
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

        while !current.is_null() {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer((*current).con);
                let log_name_owner_2 = (*(*current).con).name.clone();
                let log_value_1 =
                    i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                    ::log::debug!(
                        "../src/commands.c:cmd_scratchpad_show:1950 - matching: {} / {}\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            format!(
                                "../src/commands.c:cmd_scratchpad_show:1950 - matching: {} / ",
                                log_value_0
                            )
                            .as_bytes(),
                            &log_value_1,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            result = (result as i32 | scratchpad_show((*current).con) as i32) != 0;
            current = (*current).owindows.tqe_next;
        }
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(result));
    }
}

pub unsafe fn cmd_swap(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mode: *const ::core::ffi::c_char,
    arg: *const ::core::ffi::c_char,
) {
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let match_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
    if match_0.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(
                cmd_output,
                CommandReply::Error(b"No match found for swapping."),
            );
        }
        return;
    }
    if (*match_0).con.is_null() {
        if (*cmd_output).has_reply() {
            let message_1 = format!(
                "Match {} has no container.",
                i3_common::logging::pointer(match_0)
            );
            emit_command_reply(cmd_output, CommandReply::Error(message_1.as_bytes()));
        }
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(mode, c"id".as_ptr()) == 0_i32 {
        let Ok(target) = parse_long(std::ffi::CStr::from_ptr(arg).to_bytes(), 0) else {
            if (*cmd_output).has_reply() {
                let error_pointer_0 = arg;
                let error_bytes_0 = if error_pointer_0.is_null() {
                    b"(null)".as_slice()
                } else {
                    std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
                };
                let message_2 = [
                    b"Failed to parse ".as_slice(),
                    error_bytes_0,
                    b" into a window id.".as_slice(),
                ]
                .concat();
                emit_command_reply(cmd_output, CommandReply::Error(&message_2));
            }
            return;
        };
        con = con_by_window_id(target as u32);
    } else if strcmp(mode, c"con_id".as_ptr()) == 0_i32 {
        let Ok(target_0) = parse_long(std::ffi::CStr::from_ptr(arg).to_bytes(), 0) else {
            if (*cmd_output).has_reply() {
                let error_pointer_0 = arg;
                let error_bytes_0 = if error_pointer_0.is_null() {
                    b"(null)".as_slice()
                } else {
                    std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
                };
                let message_3 = [
                    b"Failed to parse ".as_slice(),
                    error_bytes_0,
                    b" into a container id.".as_slice(),
                ]
                .concat();
                emit_command_reply(cmd_output, CommandReply::Error(&message_3));
            }
            return;
        };
        con = con_by_con_id(target_0);
    } else if strcmp(mode, c"mark".as_ptr()) == 0_i32 {
        con = con_by_mark(std::ffi::CStr::from_ptr(arg).to_bytes());
    } else {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = mode;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let message_4 = [
                b"Unhandled swap mode \"".as_slice(),
                error_bytes_0,
                b"\". This is a bug.".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_4));
        }
        return;
    }
    if con.is_null() {
        if (*cmd_output).has_reply() {
            let error_pointer_0 = mode;
            let error_bytes_0 = if error_pointer_0.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
            };
            let error_pointer_1 = arg;
            let error_bytes_1 = if error_pointer_1.is_null() {
                b"(null)".as_slice()
            } else {
                std::ffi::CStr::from_ptr(error_pointer_1).to_bytes()
            };
            let message_5 = [
                b"Could not find container for ".as_slice(),
                error_bytes_0,
                b" = ".as_slice(),
                error_bytes_1,
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_5));
        }
        return;
    }
    if match_0 != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last {
        ::log::info!("More than one container matched the swap command, only using the first one.");
    }
    ::log::debug!(
        "../src/commands.c:cmd_swap:2010 - Swapping {} with {}.\n",
        i3_common::logging::pointer((*match_0).con),
        i3_common::logging::pointer(con)
    );
    let result: bool = con_swap((*match_0).con, con);
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(result));
    }
}

pub unsafe fn cmd_title_format(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    format: &[u8],
) {
    let format = format.split(|byte| *byte == 0).next().unwrap().to_vec();
    {
        let log_value_0 = format.clone();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/commands.c:cmd_title_format:2023 - setting title_format to \"{}\"\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:cmd_title_format:2023 - setting title_format to \""
                        .as_bytes(),
                    &log_value_0,
                    "\"\n".as_bytes(),
                ],
            );
        }
    };
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!(
                    "../src/commands.c:cmd_title_format:2028 - setting title_format for {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_title_format:2028 - setting title_format for {} / ", log_value_0).as_bytes(), &log_value_1, "\n".as_bytes() ]);
            }
        };
        (*(*current).con).title_format = None;
        if strcasecmp(
            std::ffi::CString::new(format.as_slice()).unwrap().as_ptr(),
            c"%title".as_ptr(),
        ) != 0_i32
        {
            (*(*current).con).title_format = Some(format.clone());
            if !(*(*current).con).window.is_null() {
                let formatted_title = con_parse_title_format((*current).con);
                ewmh_update_visible_name(
                    (*(*(*current).con).window).id,
                    formatted_title.str_or_report().map(str::as_bytes),
                );
            }
        } else if !(*(*current).con).window.is_null() {
            ewmh_update_visible_name((*(*(*current).con).window).id, None);
        }
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_title_window_icon(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    enable: Option<&std::ffi::CStr>,
    mut padding: i32,
) {
    let mut is_toggle: bool = false;
    if let Some(enable) = enable {
        if enable == c"toggle" {
            is_toggle = true;
        } else if !is_enabled(enable.to_bytes()) {
            padding = -1_i32;
        }
    }
    ::log::debug!(
        "../src/commands.c:cmd_title_window_icon:2074 - setting window_icon={}\n",
        { (padding) }
    );
    if (*current_match).error.is_some() {
        if (*cmd_output).has_reply() {
            let error =
                i3_common::logging::string_bytes((*current_match).error.as_deref(), None).to_vec();
            let message = [b"Invalid match: ".as_slice(), &error].concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;
            if !(*ow).owindows.tqe_next.is_null() {
                (*(*ow).owindows.tqe_next).owindows.tqe_prev = (*ow).owindows.tqe_prev;
            } else {
                (*(*cmd_output).ctx).owindows.tqh_last = (*ow).owindows.tqe_prev;
            }
            *(*ow).owindows.tqe_prev = (*ow).owindows.tqe_next;
            free(ow as *mut ::core::ffi::c_void);
        }
        let ow_0: *mut owindow =
            smalloc(::core::mem::size_of::<owindow>() as usize) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first;

    while !current.is_null() {
        if is_toggle {
            let current_padding: i32 = (*(*current).con).window_icon_padding;
            if padding > 0_i32 {
                if current_padding < 0_i32 {
                    (*(*current).con).window_icon_padding = padding;
                } else {
                    (*(*current).con).window_icon_padding = -(padding + 1_i32);
                }
            } else {
                (*(*current).con).window_icon_padding = -(current_padding + 1_i32);
            }
        } else {
            (*(*current).con).window_icon_padding = padding;
        }
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer((*current).con);
            let log_name_owner_2 = (*(*current).con).name.clone();
            let log_value_1 =
                i3_common::logging::string_bytes(log_name_owner_2.as_deref(), None).to_vec();
            let log_value_2 = ((*(*current).con).window_icon_padding);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::debug!("../src/commands.c:cmd_title_window_icon:2095 - Set window_icon for {} / {} to {}\n", log_value_0, log_value_1_text, log_value_2);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/commands.c:cmd_title_window_icon:2095 - Set window_icon for {} / ", log_value_0).as_bytes(), &log_value_1, format!(" to {}\n", log_value_2).as_bytes() ]);
            }
        };
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next;
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_rename_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    old_name: Option<&[u8]>,
    new_name: &[u8],
) {
    let mut old_name_owner =
        old_name.map(|name| name.split(|&byte| byte == 0).next().unwrap().to_vec());
    let new_name_owner = new_name.split(|&byte| byte == 0).next().unwrap().to_vec();
    if new_name_owner.starts_with(b"__") {
        if (*cmd_output).has_reply() {
            let message = [
                b"Cannot rename workspace to \"".as_slice(),
                new_name_owner.as_slice(),
                b"\": names starting with __ are i3-internal.".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
        return;
    }
    if let Some(old_name) = old_name_owner.as_deref() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = old_name.to_vec();
            let log_value_1 = new_name_owner.clone();
            if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                std::str::from_utf8(&log_value_0),
                std::str::from_utf8(&log_value_1),
            ) {
                ::log::info!(
                    "Renaming workspace \"{}\" to \"{}\"\n",
                    log_value_0_text,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "Renaming workspace \"".as_bytes(),
                        &log_value_0,
                        "\" to \"".as_bytes(),
                        &log_value_1,
                        "\"\n".as_bytes(),
                    ],
                );
            }
        };
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = new_name_owner.clone();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::info!("Renaming current workspace to \"{}\"\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "Renaming current workspace to \"".as_bytes(),
                        &log_value_0,
                        "\"\n".as_bytes(),
                    ],
                );
            }
        };
    }
    let workspace = if let Some(name) = old_name_owner.as_deref() {
        get_existing_workspace_by_name(name)
    } else {
        let workspace = con_get_workspace(focused);
        old_name_owner = (*workspace).name.clone();
        workspace
    };
    if workspace.is_null() {
        if (*cmd_output).has_reply() {
            let message_0 = [
                b"Old workspace \"".as_slice(),
                old_name_owner.as_deref().unwrap_or(b"(null)"),
                b"\" not found".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_0));
        }
        return;
    }
    let check_dest: *const Con = get_existing_workspace_by_name(&new_name_owner);
    if !check_dest.is_null() && !std::ptr::eq(check_dest, workspace) {
        if (*cmd_output).has_reply() {
            let message_1 = [
                b"New workspace \"".as_slice(),
                new_name_owner.as_slice(),
                b"\" already exists".as_slice(),
            ]
            .concat();
            emit_command_reply(cmd_output, CommandReply::Error(&message_1));
        }
        return;
    }
    let old_name_copy = old_name_owner
        .clone()
        .expect("renamed workspace has an old name");
    (*workspace).name = Some(new_name_owner.clone());
    (*workspace).num = ws_name_to_number(&new_name_owner).unwrap_or(-1);
    ::log::info!("num = {}\n", { ((*workspace).num) });
    let previously_focused: *mut Con = focused;
    let previously_focused_content: *const Con = if ((*focused).type_0) == ContainerType::Workspace
    {
        parent_ptr(focused)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    let parent: *mut Con = parent_ptr(workspace);
    con_detach(workspace);
    con_attach(workspace, parent, false);
    ipc_send_workspace_event(
        c"rename".as_ptr(),
        workspace,
        ::core::ptr::null_mut::<Con>(),
    );
    let assigned_name = (*workspace).name.clone().expect("workspace has a name");
    let assigned: *mut Con = get_assigned_output(Some(&assigned_name), (*workspace).num as i64);
    if !assigned.is_null() {
        workspace_move_to_output(workspace, get_output_for_con(assigned));
    }
    let mut can_restore_focus: bool = !previously_focused.is_null();
    if !previously_focused_content.is_null() {
        let mut ws: *const Con = ::core::ptr::null::<Con>();
        let mut child: *mut Con = -1_i32 as *mut Con;
        while child == -1_i32 as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = nodes::first_ptr(previously_focused_content.cast_mut());
            while !child.is_null() {
                if !(child == previously_focused) {
                    child = nodes::next_ptr(child);
                } else {
                    ws = child;
                    break;
                }
            }
        }
        can_restore_focus = (can_restore_focus as i32 & (ws != NULL as *const Con) as i32) != 0;
    }
    if can_restore_focus {
        workspace_show(con_get_workspace(previously_focused));
        con_focus(previously_focused);
    }
    if previous_workspace_name.as_deref() == Some(old_name_copy.as_slice()) {
        previous_workspace_name = Some(new_name_owner.clone());
    }
    (*cmd_output).needs_tree_render = true;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
    ewmh_update_desktop_properties();
    startup_sequence_rename_workspace(&old_name_copy, &new_name_owner);
}

pub unsafe fn cmd_bar_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    bar_mode: &[u8],
    bar_id: Option<&[u8]>,
) {
    let bar_mode = bar_mode.split(|&byte| byte == 0).next().unwrap().to_vec();
    let bar_id = bar_id.map(|bytes| bytes.split(|&byte| byte == 0).next().unwrap().to_vec());
    let mut mode: BarMode = BarMode::Dock;
    let mut toggle: bool = false;
    if bar_mode.as_slice() == b"dock" {
        mode = BarMode::Dock;
    } else if bar_mode.as_slice() == b"hide" {
        mode = BarMode::Hide;
    } else if bar_mode.as_slice() == b"invisible" {
        mode = BarMode::Invisible;
    } else if bar_mode.as_slice() == b"toggle" {
        toggle = true;
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = bar_mode.to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: Unknown bar mode \"{}\", this is a mismatch between code and parser spec.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Unknown bar mode \"".as_bytes(),
                        &log_value_0,
                        "\", this is a mismatch between code and parser spec.\n".as_bytes(),
                    ],
                );
            }
        };
        __assert_fail(
            c"false".as_ptr(),
            c"../src/commands.c".as_ptr(),
            2220 as ::core::ffi::c_uint,
            c"void cmd_bar_mode(Match *, struct CommandResultIR *, const char *, const char *)"
                .as_ptr(),
        );
    }
    if barconfigs.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Error(b"No bars found\n"));
        }
        return;
    }
    let mut current: *mut Barconfig = barconfigs.tqh_first;

    while !current.is_null() {
        if bar_id.as_deref().is_none_or(|id| (*current).id() == id) {
            if toggle {
                mode = (*current).mode.toggled();
            }
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (*current).id().to_vec();
                let log_value_1 = ((*current).mode as u32) as i32;
                let log_value_2 = bar_mode.to_vec();
                let log_value_3 = ((mode) as u32) as i32;
                if let (Ok(log_value_0_text), Ok(log_value_2_text)) = (
                    std::str::from_utf8(&log_value_0),
                    std::str::from_utf8(&log_value_2),
                ) {
                    ::log::debug!("../src/commands.c:cmd_bar_mode:2239 - Changing bar mode of bar_id '{}' from '{}' to '{} ({})'\n", log_value_0_text, log_value_1, log_value_2_text, log_value_3);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            "../src/commands.c:cmd_bar_mode:2239 - Changing bar mode of bar_id '"
                                .as_bytes(),
                            &log_value_0,
                            format!("' from '{}' to '", log_value_1).as_bytes(),
                            &log_value_2,
                            format!(" ({})'\n", log_value_3).as_bytes(),
                        ],
                    );
                }
            };
            if (*current).mode != mode {
                (*current).mode = mode;
                ipc_send_barconfig_update_event(current);
            }
            if bar_id.is_some() {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Success(true));
                }
                return;
            }
        }
        current = (*current).configs.tqe_next;
    }
    if let Some(bar_id) = bar_id.as_deref() {
        if (*cmd_output).has_reply() {
            let mut message = b"Changing bar mode of bar_id ".to_vec();
            message.extend_from_slice(bar_id);
            message.extend_from_slice(b" failed, bar_id not found.\n");
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
    } else if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_bar_hidden_state(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    bar_hidden_state: &[u8],
    bar_id: Option<&[u8]>,
) {
    let bar_hidden_state = bar_hidden_state
        .split(|&byte| byte == 0)
        .next()
        .unwrap()
        .to_vec();
    let bar_id = bar_id.map(|bytes| bytes.split(|&byte| byte == 0).next().unwrap().to_vec());
    let mut hidden_state: BarHiddenState = BarHiddenState::Show;
    let mut toggle: bool = false;
    if bar_hidden_state.as_slice() == b"hide" {
        hidden_state = BarHiddenState::Hide;
    } else if bar_hidden_state.as_slice() == b"show" {
        hidden_state = BarHiddenState::Show;
    } else if bar_hidden_state.as_slice() == b"toggle" {
        toggle = true;
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = bar_hidden_state.to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: Unknown bar state \"{}\", this is a mismatch between code and parser spec.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Unknown bar state \"".as_bytes(),
                        &log_value_0,
                        "\", this is a mismatch between code and parser spec.\n".as_bytes(),
                    ],
                );
            }
        };
        __assert_fail(
                c"false".as_ptr(),
                c"../src/commands.c".as_ptr(),
                2277 as ::core::ffi::c_uint,
                c"void cmd_bar_hidden_state(Match *, struct CommandResultIR *, const char *, const char *)".as_ptr(),
            );
    }
    if barconfigs.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Error(b"No bars found\n"));
        }
        return;
    }
    let mut current: *mut Barconfig = barconfigs.tqh_first;

    while !current.is_null() {
        if bar_id.as_deref().is_none_or(|id| (*current).id() == id) {
            if toggle {
                hidden_state = (*current).hidden_state.toggled();
            }
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = (*current).id().to_vec();
                let log_value_1 = ((*current).hidden_state as u32) as i32;
                let log_value_2 = bar_hidden_state.to_vec();
                let log_value_3 = ((hidden_state) as u32) as i32;
                if let (Ok(log_value_0_text), Ok(log_value_2_text)) = (
                    std::str::from_utf8(&log_value_0),
                    std::str::from_utf8(&log_value_2),
                ) {
                    ::log::debug!("../src/commands.c:cmd_bar_hidden_state:2296 - Changing bar hidden_state of bar_id '{}' from '{}' to '{} ({})'\n", log_value_0_text, log_value_1, log_value_2_text, log_value_3);
                } else {
                    i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/commands.c:cmd_bar_hidden_state:2296 - Changing bar hidden_state of bar_id '".as_bytes(), &log_value_0, format!("' from '{}' to '", log_value_1).as_bytes(), &log_value_2, format!(" ({})'\n", log_value_3).as_bytes() ]);
                }
            };
            if (*current).hidden_state != hidden_state {
                (*current).hidden_state = hidden_state;
                ipc_send_barconfig_update_event(current);
            }
            if bar_id.is_some() {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Success(true));
                }
                return;
            }
        }
        current = (*current).configs.tqe_next;
    }
    if let Some(bar_id) = bar_id.as_deref() {
        if (*cmd_output).has_reply() {
            let mut message = b"Changing bar hidden_state of bar_id ".to_vec();
            message.extend_from_slice(bar_id);
            message.extend_from_slice(b" failed, bar_id not found.\n");
            emit_command_reply(cmd_output, CommandReply::Error(&message));
        }
    } else if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_shmlog(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    argument: *const ::core::ffi::c_char,
) {
    if strcmp(argument, c"toggle".as_ptr()) == 0 {
        shmlog_size = if shmlog_size != 0 {
            -shmlog_size
        } else {
            default_shmlog_size
        };
    } else if strcmp(argument, c"on".as_ptr()) == 0 {
        shmlog_size = default_shmlog_size;
    } else if strcmp(argument, c"off".as_ptr()) == 0 {
        shmlog_size = 0_i32;
    } else {
        let Ok(new_size) = parse_long(std::ffi::CStr::from_ptr(argument).to_bytes(), 0) else {
            if (*cmd_output).has_reply() {
                let error_pointer_0 = argument;
                let error_bytes_0 = if error_pointer_0.is_null() {
                    b"(null)".as_slice()
                } else {
                    std::ffi::CStr::from_ptr(error_pointer_0).to_bytes()
                };
                let message = [
                    b"Failed to parse ".as_slice(),
                    error_bytes_0,
                    b" into a shmlog size.".as_slice(),
                ]
                .concat();
                emit_command_reply(cmd_output, CommandReply::Error(&message));
            }
            return;
        };
        if shmlog_size > 0_i32 {
            shmlog_size = 0_i32;
            ::log::info!("Restarting shm logging...\n");
            init_logging();
        }
        shmlog_size = new_size as i32;
    }
    ::log::info!(
        "{} shm logging\n",
        if shmlog_size > 0_i32 {
            "Enabling"
        } else {
            "Disabling"
        }
    );
    init_logging();
    update_shmlog_atom();
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_debuglog(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    argument: *const ::core::ffi::c_char,
) {
    let logging: bool = get_debug_logging();
    if strcmp(argument, c"toggle".as_ptr()) == 0 {
        ::log::info!(
            "{} debug logging\n",
            if logging as i32 != 0 {
                "Disabling"
            } else {
                "Enabling"
            }
        );
        set_debug_logging(!logging);
    } else if strcmp(argument, c"on".as_ptr()) == 0 && !logging {
        ::log::info!("Enabling debug logging\n");
        set_debug_logging(true);
    } else if strcmp(argument, c"off".as_ptr()) == 0 && logging {
        ::log::info!("Disabling debug logging\n");
        set_debug_logging(false);
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
unsafe fn gaps_inner(gaps: *mut gaps_t) -> *mut i32 {
    &raw mut (*gaps).inner
}
unsafe fn gaps_top(gaps: *mut gaps_t) -> *mut i32 {
    &raw mut (*gaps).top
}
unsafe fn gaps_left(gaps: *mut gaps_t) -> *mut i32 {
    &raw mut (*gaps).left
}
unsafe fn gaps_bottom(gaps: *mut gaps_t) -> *mut i32 {
    &raw mut (*gaps).bottom
}
unsafe fn gaps_right(gaps: *mut gaps_t) -> *mut i32 {
    &raw mut (*gaps).right
}
unsafe fn gaps_update(
    get: gap_accessor,
    scope: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
    pixels: i32,
) -> bool {
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_pointer_1 = scope;
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
        let log_pointer_2 = mode;
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
        let log_value_2 = (pixels);
        if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
            std::str::from_utf8(&log_value_0),
            std::str::from_utf8(&log_value_1),
        ) {
            ::log::debug!(
                "../src/commands.c:gaps_update:2394 - gaps_update(scope={}, mode={}, pixels={})\n",
                log_value_0_text,
                log_value_1_text,
                log_value_2
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/commands.c:gaps_update:2394 - gaps_update(scope=".as_bytes(),
                    &log_value_0,
                    ", mode=".as_bytes(),
                    &log_value_1,
                    format!(", pixels={})\n", log_value_2).as_bytes(),
                ],
            );
        }
    };
    let workspace: *mut Con = con_get_workspace(focused);
    let global_gap_size: i32 = *get.expect("non-null function pointer")(&raw mut config.gaps);
    let mut current_value: i32 = global_gap_size;
    if strcmp(scope, c"current".as_ptr()) == 0_i32 {
        current_value += *get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
    }
    ::log::debug!(
        "../src/commands.c:gaps_update:2402 - global_gap_size={}, current_value={}\n",
        { (global_gap_size) },
        { (current_value) }
    );
    let mut reset: bool = false;
    if strcmp(mode, c"plus".as_ptr()) == 0_i32 {
        current_value += pixels;
    } else if strcmp(mode, c"minus".as_ptr()) == 0_i32 {
        current_value -= pixels;
    } else if strcmp(mode, c"set".as_ptr()) == 0_i32 {
        current_value = pixels;
        reset = true;
    } else if strcmp(mode, c"toggle".as_ptr()) == 0_i32 {
        current_value = (current_value == 0) as i32 * pixels;
        reset = true;
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = mode;
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
                ::log::error!(
                    "ERROR: Invalid mode {} when changing gaps",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Invalid mode ".as_bytes(),
                        &log_value_0,
                        " when changing gaps".as_bytes(),
                    ],
                );
            }
        };
        return false;
    }
    let mut min_value: i32 = 0_i32;
    let is_outer: bool = get.expect("non-null function pointer")(&raw mut config.gaps)
        != gaps_inner(&raw mut config.gaps);
    if is_outer {
        if strcmp(scope, c"all".as_ptr()) == 0_i32 {
            min_value = -config.gaps.inner;
        } else {
            min_value = -config.gaps.inner - (*workspace).gaps.inner;
        }
    }
    if current_value < min_value {
        current_value = min_value;
    }
    if strcmp(scope, c"all".as_ptr()) == 0_i32 {
        let mut output: *mut Con = nodes::first_ptr(croot);

        while !output.is_null() {
            let mut cur_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let content: *const Con = output_get_content(output);
            cur_ws = nodes::first_ptr(content.cast_mut());
            while !cur_ws.is_null() {
                let gaps_value: *mut i32 =
                    get.expect("non-null function pointer")(&raw mut (*cur_ws).gaps);
                ::log::debug!(
                    "../src/commands.c:gaps_update:2443 - current gaps_value = {}\n",
                    (*gaps_value) as i32
                );
                if reset {
                    *gaps_value = 0_i32;
                } else {
                    let mut max_compensate: i32 = 0_i32;
                    if is_outer {
                        max_compensate = config.gaps.inner;
                    }
                    if *gaps_value + current_value + max_compensate < 0_i32 {
                        *gaps_value = -current_value - max_compensate;
                    }
                }
                ::log::debug!(
                    "../src/commands.c:gaps_update:2459 - gaps_value after fix = {}\n",
                    (*gaps_value) as i32
                );
                cur_ws = nodes::next_ptr(cur_ws);
            }
            output = nodes::next_ptr(output);
        }
        *get.expect("non-null function pointer")(&raw mut config.gaps) = current_value;
        ::log::debug!(
            "../src/commands.c:gaps_update:2464 - global gaps value after fix = {}\n",
            { (*get.expect("non-null function pointer")(&raw mut config.gaps)) }
        );
    } else {
        let gaps_value_0: *mut i32 =
            get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
        *gaps_value_0 = current_value - global_gap_size;
    }
    true
}

pub unsafe fn cmd_gaps(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    type_0: *const ::core::ffi::c_char,
    scope: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
    value: *const ::core::ffi::c_char,
) {
    let current_block: u64;
    let pixels: i32 = logical_px(atoi(value)) as i32;
    if strcmp(type_0, c"inner".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_inner as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut i32),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0_i32,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut i32),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0_i32,
        ) || !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut i32),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0_i32,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut i32),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0_i32,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"outer".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"vertical".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"horizontal".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"top".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"bottom".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"right".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"left".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut i32),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = type_0;
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
                ::log::error!(
                    "ERROR: Invalid type {} when changing gaps",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Invalid type ".as_bytes(),
                        &log_value_0,
                        " when changing gaps".as_bytes(),
                    ],
                );
            }
        };
        current_block = 1063758793336646565;
    }
    match current_block {
        1063758793336646565 => {
            if (*cmd_output).has_reply() {
                emit_command_reply(cmd_output, CommandReply::Success(false));
            }
        }
        _ => {
            (*cmd_output).needs_tree_render = true;
            if (*cmd_output).has_reply() {
                emit_command_reply(cmd_output, CommandReply::Success(true));
            }
        }
    }
}
pub const EXIT_SUCCESS: i32 = 0_i32;

use crate::r#match::regex_matches_bytes;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::switch_mode;
use crate::config::load_configuration;
use crate::ewmh::ewmh_update_desktop_properties;
use crate::ewmh::ewmh_update_sticky;
use crate::ewmh::ewmh_update_visible_name;
use crate::ewmh::ewmh_update_wm_desktop;
use crate::ipc::ipc_shutdown;
use crate::load_layout::json_determine_content;
use crate::load_layout::json_validate;
use crate::log::get_debug_logging;
use crate::log::init_logging;
use crate::log::purge_zerobyte_logfile;
use crate::log::set_debug_logging;
use crate::startup::start_application;
use crate::startup::startup_sequence_rename_workspace;
use crate::tree::level_down;
use crate::tree::level_up;
use crate::util::direction_from_orientation_position;
use crate::util::i3_restart;
use crate::util::kill_nagbar;
use crate::util::layout_from_name;
use crate::util::orientation_from_direction;
use crate::util::parse_long;
use crate::util::slurp;
use crate::util::ws_name_to_number;
use crate::workspace::workspace_back_and_forth;
use crate::workspace::workspace_show_by_name;
use crate::x::update_shmlog_atom;
use crate::x::x_set_i3_atoms;
use i3_common::boolstr::is_enabled;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{atoi, exit, fcntl, free, setenv, strcasecmp, strcmp, strlen, strncasecmp, unlink};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::deco_render_params;
pub use crate::types::gaps_t;
pub use crate::types::mark_t;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::default_shmlog_size;
use crate::assignments::run_assignments;
use crate::bindings::command_error_nagbar_pid;
use crate::con::con_accepts_window;
use crate::con::con_activate;
use crate::con::con_activate_unblock;
use crate::con::con_attach;
use crate::con::con_by_con_id;
use crate::con::con_by_mark;
use crate::con::con_by_window_id;
use crate::con::con_close;
use crate::con::con_descend_focused;
use crate::con::con_detach;
use crate::con::con_disable_fullscreen;
use crate::con::con_enable_fullscreen;
use crate::con::con_focus;
use crate::con::con_fullscreen_permits_focusing;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_has_children;
use crate::con::con_inside_floating;
use crate::con::con_is_docked;
use crate::con::con_is_floating;
use crate::con::con_is_internal;
use crate::con::con_mark;
use crate::con::con_mark_toggle;
use crate::con::con_move_to_mark;
use crate::con::con_move_to_output;
use crate::con::con_move_to_workspace;
use crate::con::con_num_children;
use crate::con::con_orientation;
use crate::con::con_parse_title_format;
use crate::con::con_rect_size_in_orientation;
use crate::con::con_set_border_style;
use crate::con::con_set_layout;
use crate::con::con_swap;
use crate::con::con_toggle_fullscreen;
use crate::con::con_toggle_layout;
use crate::con::con_unmark;
use crate::config::config;
use crate::config_parser::config_error_nagbar_pid;
use crate::floating::floating_center;
use crate::floating::floating_check_size;
use crate::floating::floating_disable;
use crate::floating::floating_enable;
use crate::floating::floating_maybe_reassign_ws;
use crate::floating::floating_move_to_pointer;
use crate::floating::floating_reposition;
use crate::floating::floating_resize;
use crate::floating::toggle_floating_mode;
use crate::ipc::ipc_send_workspace_event;
use crate::load_layout::tree_append_json;
use crate::log::current_log_stream_socket_path;
use crate::log::shmlog_size;
use crate::output::get_output_for_con;
use crate::output::get_output_from_string;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::output::output_push_sticky_windows;
use crate::r#match::match_is_empty;
use crate::r#match::match_matches_window;
use crate::r#match::match_parse_property;
use crate::r#move::tree_move;
use crate::render::render_con;
use crate::resize::percent_for_1px;
use crate::resize::resize_find_tiling_participants;
use crate::resize::resize_neighboring_cons;
use crate::restore_layout::restore_open_placeholder_windows;
use crate::scratchpad::scratchpad_move;
use crate::scratchpad::scratchpad_show;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::get_tree_next_sibling;
use crate::tree::tree_next;
use crate::tree::tree_open_con;
use crate::tree::tree_split;
use crate::util::rect_equals;
use crate::workspace::get_assigned_output;
use crate::workspace::get_existing_workspace_by_name;
use crate::workspace::get_existing_workspace_by_num;
use crate::workspace::previous_workspace_name;
use crate::workspace::workspace_back_and_forth_get;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_move_to_output;
use crate::workspace::workspace_next;
use crate::workspace::workspace_next_on_output;
use crate::workspace::workspace_prev;
use crate::workspace::workspace_prev_on_output;
use crate::workspace::workspace_show;
use i3_common::resolve_tilde::resolve_or_exit;

use crate::con::arena::nodes;

#[cfg(test)]
mod argument_tests {
    use super::*;
    #[test]
    fn command_error_byte_builder_matches_native_printf_before_serialization() {
        let nul_backing = b"nul\0tail\xff";
        let first_nul = std::ffi::CStr::from_bytes_until_nul(nul_backing).unwrap();
        for input in [c"invalid", c"quote\"\n", c"bad\xff", first_nul] {
            let mut context = cmd_parser_ctx {
                json_output: Some(crate::ipc::CommandReplies::default()),
                state: CommandParserState::Initial,
                current_match: Match::empty(),
                stack: stack::default(),
                owindows: owindows_head {
                    tqh_first: std::ptr::null_mut(),
                    tqh_last: std::ptr::null_mut(),
                },
                subcommand_output: CommandResultIR {
                    ctx: std::ptr::null_mut(),
                    client: std::ptr::null_mut(),
                    next_state: CommandParserState::Initial,
                    needs_tree_render: false,
                },
                command_output: CommandResultIR {
                    ctx: std::ptr::null_mut(),
                    client: std::ptr::null_mut(),
                    next_state: CommandParserState::Initial,
                    needs_tree_render: false,
                },
            };
            context.command_output.ctx = &raw mut context;
            // SAFETY: each input is intentionally not a shmlog keyword/number;
            // its failure branch touches only this live parser context and
            // thread-local errno, with no X11 or logging-state mutation.
            unsafe {
                cmd_shmlog(
                    &raw mut context.current_match,
                    &raw mut context.command_output,
                    input.as_ptr(),
                );
            }
            let actual = context.json_output.take().unwrap().finish();
            let mut native = [0u8; 256];
            let count = unsafe {
                libc::snprintf(
                    native.as_mut_ptr().cast(),
                    native.len(),
                    c"Failed to parse %s into a shmlog size.".as_ptr(),
                    input.as_ptr(),
                )
            };
            assert!(count >= 0 && (count as usize) < native.len());
            context.json_output = Some(crate::ipc::CommandReplies::default());
            unsafe {
                emit_command_reply(
                    &raw mut context.command_output,
                    CommandReply::Error(&native[..count as usize]),
                );
            }
            assert_eq!(actual, context.json_output.take().unwrap().finish());
        }
    }

    #[test]
    fn direction_arguments_remain_exact_case_sensitive_keywords() {
        assert!(matches!(direction_argument(b"left"), Some(Direction::Left)));
        assert!(matches!(
            parse_direction(b"left\0ignored\xff"),
            Direction::Left
        ));
        assert!(matches!(
            direction_argument(b"right"),
            Some(Direction::Right)
        ));
        assert!(matches!(direction_argument(b"up"), Some(Direction::Up)));
        assert!(matches!(direction_argument(b"down"), Some(Direction::Down)));
        for invalid in [b"LEFT".as_slice(), b"leftward", b"", b"\xff"] {
            assert!(direction_argument(invalid).is_none());
        }
    }
    #[test]
    fn output_name_accumulators_own_bytes_and_snapshot_without_holding_callback_locks() {
        let accumulator = OutputNames::new(Vec::new());
        let input = std::ffi::CString::new(b"HDMI-\xff".as_slice()).unwrap();
        unsafe { user_output_names_add(&accumulator, input.as_ptr()) };
        drop(input);
        let snapshot = accumulator.lock().unwrap().clone();
        assert_eq!(snapshot[0].as_slice(), b"HDMI-\xff");
        user_output_names_free(&accumulator);
        user_output_names_free(&accumulator);
        assert!(accumulator.lock().unwrap().is_empty());
        assert_eq!(snapshot[0].as_slice(), b"HDMI-\xff");
    }
}
