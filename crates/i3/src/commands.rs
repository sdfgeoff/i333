use crate::commands_parser::CommandParserState;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, resolve};
use crate::config::barconfigs;
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
use i3_common::safewrappers::sasprintf;
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

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;

pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::Barconfig;

pub use crate::application::ipc_client;

pub use crate::parser_util::ParserStack as stack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct owindow {
    pub con: *mut Con,
    pub owindows: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
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
    pub json_output: Option<i3_json::Generator>,
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
type OutputNames = std::sync::Mutex<Vec<std::ffi::CString>>;
static names: OutputNames = std::sync::Mutex::new(Vec::new());
static names_global: OutputNames = std::sync::Mutex::new(Vec::new());
pub type gap_accessor = Option<unsafe fn(*mut gaps_t) -> *mut libc::c_int>;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe fn maybe_back_and_forth(
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
) -> bool {
    let ws: *const Con = con_get_workspace(focused);
    if strcmp((*ws).name, name) != 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    debuglog(
        c"%s:%s:%d - This workspace is already focused.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"maybe_back_and_forth".as_ptr(),
        91 as ::core::ffi::c_int,
    );
    if config.workspace_auto_back_and_forth {
        workspace_back_and_forth();
        (*cmd_output).needs_tree_render = true_0 != 0;
    }
    true_0 != 0
}
unsafe fn maybe_auto_back_and_forth_workspace(workspace: *mut Con) -> *mut Con {
    if !config.workspace_auto_back_and_forth {
        return workspace;
    }
    let current: *const Con = con_get_workspace(focused);
    if current == workspace as *const Con {
        let baf: *mut Con = workspace_back_and_forth_get();
        if !baf.is_null() {
            debuglog(
                c"%s:%s:%d - Substituting workspace with back_and_forth, as it is focused.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"maybe_auto_back_and_forth_workspace".as_ptr(),
                112 as ::core::ffi::c_int,
            );
            return baf;
        }
    }
    workspace
}

pub unsafe fn cmd_criteria_init(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    debuglog(
        c"%s:%s:%d - Initializing criteria, current_match = %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_criteria_init".as_ptr(),
        133 as ::core::ffi::c_int,
        current_match,
    );
    match_free(current_match);
    match_init(current_match);
}

pub unsafe fn cmd_criteria_match_windows(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    debuglog(
        c"%s:%s:%d - match specification finished, matching...\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_criteria_match_windows".as_ptr(),
        144 as ::core::ffi::c_int,
    );
    while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        debuglog(
            c"%s:%s:%d - checking if con %p / %s matches\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_criteria_match_windows".as_ptr(),
            157 as ::core::ffi::c_int,
            con,
            (*con).name,
        );
        let mut accept_match: bool = false_0 != 0;
        if !(*current_match).con_id.is_null() {
            accept_match = true_0 != 0;
            if (*current_match).con_id == con {
                debuglog(
                    c"%s:%s:%d - con_id matched.\n".as_ptr().cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"cmd_criteria_match_windows".as_ptr(),
                    167 as ::core::ffi::c_int,
                );
                current_block_41 = 15089075282327824602;
            } else {
                debuglog(
                    c"%s:%s:%d - con_id does not match.\n".as_ptr().cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"cmd_criteria_match_windows".as_ptr(),
                    169 as ::core::ffi::c_int,
                );
                current_block_41 = 3512920355445576850;
            }
        } else {
            current_block_41 = 15089075282327824602;
        }
        match current_block_41 {
            15089075282327824602 => {
                if !(*current_match).mark.is_null() && !(*con).marks_head.tqh_first.is_null() {
                    accept_match = true_0 != 0;
                    let mut matched_by_mark: bool = false_0 != 0;
                    let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
                    mark = (*con).marks_head.tqh_first as *mut mark_t;
                    while !mark.is_null() {
                        if !regex_matches((*current_match).mark, (*mark).name) {
                            mark = (*mark).marks.tqe_next as *mut mark_t;
                        } else {
                            debuglog(
                                c"%s:%s:%d - match by mark\n".as_ptr().cast_mut(),
                                c"../src/commands.c".as_ptr(),
                                c"cmd_criteria_match_windows".as_ptr(),
                                184 as ::core::ffi::c_int,
                            );
                            matched_by_mark = true_0 != 0;
                            break;
                        }
                    }
                    if !matched_by_mark {
                        debuglog(
                            c"%s:%s:%d - mark does not match.\n".as_ptr().cast_mut(),
                            c"../src/commands.c".as_ptr(),
                            c"cmd_criteria_match_windows".as_ptr(),
                            190 as ::core::ffi::c_int,
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
                                debuglog(
                                    c"%s:%s:%d - matches window!\n".as_ptr().cast_mut(),
                                    c"../src/commands.c".as_ptr(),
                                    c"cmd_criteria_match_windows".as_ptr(),
                                    197 as ::core::ffi::c_int,
                                );
                                accept_match = true_0 != 0;
                                current_block_41 = 8845338526596852646;
                            } else {
                                debuglog(
                                    c"%s:%s:%d - doesn't match\n".as_ptr().cast_mut(),
                                    c"../src/commands.c".as_ptr(),
                                    c"cmd_criteria_match_windows".as_ptr(),
                                    200 as ::core::ffi::c_int,
                                );
                                current_block_41 = 3512920355445576850;
                            }
                        } else {
                            current_block_41 = 8845338526596852646;
                        }
                        match current_block_41 {
                            3512920355445576850 => {}
                            _ => {
                                if accept_match {
                                    debuglog(
                                        c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
                                        c"../src/commands.c".as_ptr(),
                                        c"cmd_criteria_match_windows".as_ptr(),
                                        206 as ::core::ffi::c_int,
                                        con,
                                        (*con).name,
                                    );
                                    let ow_0: *mut owindow =
                                        smalloc(::core::mem::size_of::<owindow>() as usize)
                                            as *mut owindow;
                                    (*ow_0).con = con;
                                    (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
                                    (*ow_0).owindows.tqe_prev =
                                        (*(*cmd_output).ctx).owindows.tqh_last;
                                    *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
                                    (*(*cmd_output).ctx).owindows.tqh_last =
                                        &raw mut (*ow_0).owindows.tqe_next;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
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
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*owindows).tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"move_matches_to_workspace".as_ptr(),
            226 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        con_move_to_workspace((*current).con, ws, true_0 != 0, false_0 != 0, false_0 != 0);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
}

pub unsafe fn cmd_move_con_to_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - which=%s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_move_con_to_workspace".as_ptr(),
        264 as ::core::ffi::c_int,
        which,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                );
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(which, c"next".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_next();
    } else if strcmp(which, c"prev".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_prev();
    } else if strcmp(which, c"next_on_output".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_next_on_output();
    } else if strcmp(which, c"prev_on_output".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_prev_on_output();
    } else if strcmp(which, c"current".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = con_get_workspace(focused);
    } else {
        if (*cmd_output).has_reply() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                c"BUG: called with which=%s".as_ptr(),
                which,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
            );
            free(message_2 as *mut ::core::ffi::c_void);
        }
        return;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
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
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"No workspace was previously active.".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    if strncasecmp(name, c"__".as_ptr(), strlen(c"__".as_ptr())) == 0 as ::core::ffi::c_int {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You cannot move containers to i3-internal workspaces (\"%s\").".as_ptr(),
                name,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
            );
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
                );
                free(message_2 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    verboselog(
        c"should move window to workspace %s\n".as_ptr().cast_mut(),
        name,
    );
    let mut ws: *mut Con = workspace_get(name);
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    } else {
        let mut found: bool = false_0 != 0;
        let mut current: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            let next: *mut owindow = (*current).owindows.tqe_next as *mut owindow;
            if (*(*current).con).type_0 as ::core::ffi::c_uint
                == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                found = true_0 != 0;
            }
            current = next;
        }
        if !found {
            if (*cmd_output).has_reply() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                );
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    verboselog(
        c"should move window to workspace %s\n".as_ptr().cast_mut(),
        which,
    );
    let parsed_num: ::core::ffi::c_long = ws_name_to_number(which) as ::core::ffi::c_long;
    if parsed_num == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        verboselog(
            c"Could not parse initial part of \"%s\" as a number.\n"
                .as_ptr()
                .cast_mut(),
            which,
        );
        if (*cmd_output).has_reply() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                c"Could not parse number \"%s\"".as_ptr(),
                which,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
            );
            free(message_2 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut ws: *mut Con = get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
    if ws.is_null() {
        ws = workspace_get(which);
    }
    if no_auto_back_and_forth.is_null() {
        ws = maybe_auto_back_and_forth_workspace(ws);
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
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

unsafe fn parse_direction(value: *const libc::c_char) -> Direction {
    if let Some(direction) = direction_argument(std::ffi::CStr::from_ptr(value).to_bytes()) {
        return direction;
    }
    errorlog(
        c"ERROR: Invalid direction. This is a parser bug.\n"
            .as_ptr()
            .cast_mut(),
    );
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/commands.c".as_ptr(),
            389 as ::core::ffi::c_uint,
            c"direction_t parse_direction(const char *)".as_ptr(),
        );
    };
    panic!("Reached end of non-void function without returning");
}
unsafe fn cmd_resize_floating(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
    floating_con: *mut Con,
    mut px: ::core::ffi::c_int,
) {
    let old_rect: Rect = (*floating_con).rect as Rect;
    let focused_con: *const Con = con_descend_focused(floating_con);
    let mut direction: Direction = Direction::Left;
    if strcmp(direction_str, c"height".as_ptr()) == 0 as ::core::ffi::c_int {
        direction = Direction::Down;
    } else if strcmp(direction_str, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
        direction = Direction::Right;
    } else {
        direction = parse_direction(direction_str);
    }
    let orientation: Orientation = orientation_from_direction(direction);
    let window: *const i3Window = (*focused_con).window;
    if !window.is_null() {
        if orientation == Orientation::Vertical {
            if px < 0 as ::core::ffi::c_int {
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
        } else if px < 0 as ::core::ffi::c_int {
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
    mut px: ::core::ffi::c_int,
    ppt: ::core::ffi::c_int,
) -> bool {
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = current;
    let search_direction: Direction = parse_direction(direction);
    let res: bool = resize_find_tiling_participants(
        &raw mut first,
        &raw mut second,
        search_direction,
        false_0 != 0,
    ) as bool;
    if !res {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"No second container found in this direction.".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    if ppt != 0 {
        px = 0 as ::core::ffi::c_int;
    }
    resize_neighboring_cons(first, second, px, ppt)
}
unsafe fn cmd_resize_tiling_width_height(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut current: *mut Con,
    direction: *const ::core::ffi::c_char,
    px: ::core::ffi::c_int,
    mut ppt: ::core::ffi::c_double,
) -> bool {
    verboselog(c"width/height resize\n".as_ptr().cast_mut());
    let mut dummy: *mut Con = ::core::ptr::null_mut::<Con>();
    let search_direction: Direction =
        if strcmp(direction, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
            Direction::Left
        } else {
            Direction::Down
        };
    let search_result: bool = resize_find_tiling_participants(
        &raw mut current,
        &raw mut dummy,
        search_direction,
        true_0 != 0,
    ) as bool;
    if search_result as ::core::ffi::c_int == false_0 {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Failed to find appropriate tiling containers for resize operation".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    let children: ::core::ffi::c_int = con_num_children(parent_ptr(current)) as ::core::ffi::c_int;
    verboselog(c"ins. %d children\n".as_ptr().cast_mut(), children);
    let percentage: ::core::ffi::c_double = 1.0f64 / children as ::core::ffi::c_double;
    verboselog(c"default percentage = %f\n".as_ptr().cast_mut(), percentage);
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*parent_ptr(current)).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        verboselog(
            c"child->percent = %f (child %p)\n".as_ptr().cast_mut(),
            (*child).percent,
            child,
        );
        if (*child).percent == 0.0f64 {
            (*child).percent = percentage;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    let mut new_current_percent: ::core::ffi::c_double = 0.;
    if ppt != 0.0f64 {
        new_current_percent = (*current).percent + ppt;
    } else {
        ppt = px as ::core::ffi::c_double
            / con_rect_size_in_orientation(parent_ptr(current)) as ::core::ffi::c_double;
        new_current_percent = (*current).percent + ppt;
    }
    let subtract_percent: ::core::ffi::c_double =
        ppt / (children - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if ppt < 0.0f64 && new_current_percent < percent_for_1px(current) {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Not resizing, container would end with less than 1px".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    verboselog(
        c"new_current_percent = %f\n".as_ptr().cast_mut(),
        new_current_percent,
    );
    verboselog(
        c"subtract_percent = %f\n".as_ptr().cast_mut(),
        subtract_percent,
    );
    if subtract_percent >= 0.0f64 {
        child = (*parent_ptr(current)).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if !(child == current) {
                if (*child).percent - subtract_percent < percent_for_1px(child) {
                    if (*cmd_output).has_reply() {
                        let mut message_1: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut message_1,
                            c"Not resizing, already at minimum size (child %p would end up with a size of %.f".as_ptr(),
                            child,
                            (*child).percent - subtract_percent,
                        );
                        emit_command_reply(
                            cmd_output,
                            CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                        );
                        free(message_1 as *mut ::core::ffi::c_void);
                    }
                    return false_0 != 0;
                }
            }
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    (*current).percent = new_current_percent;
    verboselog(
        c"current->percent after = %f\n".as_ptr().cast_mut(),
        (*current).percent,
    );
    child = (*parent_ptr(current)).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !(child == current) {
            (*child).percent -= subtract_percent;
            verboselog(
                c"child->percent after (%p) = %f\n".as_ptr().cast_mut(),
                child,
                (*child).percent,
            );
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    true_0 != 0
}

pub unsafe fn cmd_resize(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    way: *const ::core::ffi::c_char,
    direction: *const ::core::ffi::c_char,
    mut resize_px: ::core::ffi::c_long,
    mut resize_ppt: ::core::ffi::c_long,
) {
    debuglog(
        c"%s:%s:%d - resizing in way %s, direction %s, px %ld or ppt %ld\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_resize".as_ptr(),
        545 as ::core::ffi::c_int,
        way,
        direction,
        resize_px,
        resize_ppt,
    );
    if strcmp(way, c"shrink".as_ptr()) == 0 as ::core::ffi::c_int {
        resize_px *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        resize_ppt *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !(*(*current).con).window.is_null()
            && (*(*(*current).con).window).dock as ::core::ffi::c_uint != 0
        {
            debuglog(
                c"%s:%s:%d - This is a dock window. Not resizing (con = %p)\n)"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_resize".as_ptr(),
                557 as ::core::ffi::c_int,
                (*current).con,
            );
        } else {
            let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
            floating_con = con_inside_floating((*current).con);
            if !floating_con.is_null() {
                cmd_resize_floating(
                    current_match,
                    cmd_output,
                    direction,
                    floating_con,
                    resize_px as ::core::ffi::c_int,
                );
            } else if strcmp(direction, c"width".as_ptr()) == 0 as ::core::ffi::c_int
                || strcmp(direction, c"height".as_ptr()) == 0 as ::core::ffi::c_int
            {
                let ppt: ::core::ffi::c_double = resize_ppt as ::core::ffi::c_double / 100.0f64;
                if !cmd_resize_tiling_width_height(
                    current_match,
                    cmd_output,
                    (*current).con,
                    direction,
                    resize_px as ::core::ffi::c_int,
                    ppt,
                ) {
                    if (*cmd_output).has_reply() {
                        let mut message_0: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(&raw mut message_0, c"Cannot resize.".as_ptr());
                        emit_command_reply(
                            cmd_output,
                            CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
                        );
                        free(message_0 as *mut ::core::ffi::c_void);
                    }
                    return;
                }
            } else if !cmd_resize_tiling_direction(
                current_match,
                cmd_output,
                (*current).con,
                direction,
                resize_px as ::core::ffi::c_int,
                resize_ppt as ::core::ffi::c_int,
            ) {
                if (*cmd_output).has_reply() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(&raw mut message_1, c"Cannot resize.".as_ptr());
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                    );
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                return;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
unsafe fn resize_set_tiling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut target: *mut Con,
    resize_orientation: Orientation,
    is_ppt: bool,
    target_size: ::core::ffi::c_long,
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
    let mut dummy: *mut Con = ::core::ptr::null_mut::<Con>();
    resize_find_tiling_participants(
        &raw mut target,
        &raw mut dummy,
        search_direction,
        true_0 != 0,
    );
    let mut ppt: ::core::ffi::c_double = 0.0f64;
    let mut px: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_ppt {
        ppt = target_size as ::core::ffi::c_double / 100.0f64 - (*target).percent;
    } else {
        px = (target_size
            - (if resize_orientation == Orientation::Horizontal {
                (*target).rect.width
            } else {
                (*target).rect.height
            }) as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
    cmd_resize_tiling_width_height(current_match, cmd_output, target, mode, px, ppt)
}

pub unsafe fn cmd_resize_set(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut cwidth: ::core::ffi::c_long,
    mode_width: *const ::core::ffi::c_char,
    mut cheight: ::core::ffi::c_long,
    mode_height: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - resizing to %ld %s x %ld %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_resize_set".as_ptr(),
        624 as ::core::ffi::c_int,
        cwidth,
        mode_width,
        cheight,
        mode_height,
    );
    if cwidth < 0 as ::core::ffi::c_long || cheight < 0 as ::core::ffi::c_long {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"Dimensions cannot be negative.".as_ptr());
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    let mut success: bool = true_0 != 0;
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
        floating_con = con_inside_floating((*current).con);
        if !floating_con.is_null() {
            let output: *const Con = con_get_output(floating_con);
            if cwidth == 0 as ::core::ffi::c_long {
                cwidth = (*floating_con).rect.width as ::core::ffi::c_long;
            } else if !mode_width.is_null()
                && strcmp(mode_width, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                cwidth = ((*output).rect.width as ::core::ffi::c_double
                    * (cwidth as ::core::ffi::c_double / 100.0f64))
                    as ::core::ffi::c_long;
            }
            if cheight == 0 as ::core::ffi::c_long {
                cheight = (*floating_con).rect.height as ::core::ffi::c_long;
            } else if !mode_height.is_null()
                && strcmp(mode_height, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                cheight = ((*output).rect.height as ::core::ffi::c_double
                    * (cheight as ::core::ffi::c_double / 100.0f64))
                    as ::core::ffi::c_long;
            }
            floating_resize(floating_con, cwidth as u32, cheight as u32);
        } else if !(*(*current).con).window.is_null()
            && (*(*(*current).con).window).dock as ::core::ffi::c_uint != 0
        {
            debuglog(
                c"%s:%s:%d - This is a dock window. Not resizing (con = %p)\n)"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_resize_set".as_ptr(),
                651 as ::core::ffi::c_int,
                (*current).con,
            );
        } else {
            if cwidth > 0 as ::core::ffi::c_long {
                let is_ppt: bool = !mode_width.is_null()
                    && strcmp(mode_width, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int;
                success = (success as ::core::ffi::c_int
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        Orientation::Horizontal,
                        is_ppt,
                        cwidth,
                    ) as ::core::ffi::c_int)
                    != 0;
            }
            if cheight > 0 as ::core::ffi::c_long {
                let is_ppt_0: bool = !mode_height.is_null()
                    && strcmp(mode_height, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int;
                success = (success as ::core::ffi::c_int
                    & resize_set_tiling(
                        current_match,
                        cmd_output,
                        (*current).con,
                        Orientation::Vertical,
                        is_ppt_0,
                        cheight,
                    ) as ::core::ffi::c_int)
                    != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(success));
    }
}
unsafe fn border_width_from_style(
    border_style: BorderStyle,
    border_width: ::core::ffi::c_long,
    con: *mut Con,
) -> ::core::ffi::c_int {
    if border_style == BorderStyle::None {
        return 0 as ::core::ffi::c_int;
    }
    if border_width >= 0 as ::core::ffi::c_long {
        return logical_px(border_width as ::core::ffi::c_int);
    }
    let is_floating: bool = !con_inside_floating(con).is_null();
    if is_floating as ::core::ffi::c_int != 0 && border_style == config.default_floating_border {
        config.default_floating_border_width
    } else if !is_floating && border_style == config.default_border {
        config.default_border_width
    } else {
        logical_px(if border_style == BorderStyle::Normal {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        })
    }
}

pub unsafe fn cmd_border(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    border_style_str: *const ::core::ffi::c_char,
    border_width: ::core::ffi::c_long,
) {
    debuglog(
        c"%s:%s:%d - border style should be changed to %s with border width %ld\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_border".as_ptr(),
        697 as ::core::ffi::c_int,
        border_style_str,
        border_width,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_border".as_ptr(),
            703 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        let mut border_style: BorderStyle = BorderStyle::None;
        if strcmp(border_style_str, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = (*(*current).con).border_style.next();
        } else if strcmp(border_style_str, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BorderStyle::Normal;
        } else if strcmp(border_style_str, c"pixel".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BorderStyle::Pixel;
        } else if strcmp(border_style_str, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BorderStyle::None;
        } else {
            if (*cmd_output).has_reply() {
                let mut message_0: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_0,
                    c"BUG: called with border_style=%s".as_ptr(),
                    border_style_str,
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
                );
                free(message_0 as *mut ::core::ffi::c_void);
            }
            return;
        }
        (*(*current).con).max_user_border_style = border_style;
        let con_border_width: ::core::ffi::c_int =
            border_width_from_style(border_style, border_width, (*current).con)
                as ::core::ffi::c_int;
        con_set_border_style((*current).con, border_style, con_border_width);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_nop(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    comment: *const ::core::ffi::c_char,
) {
    verboselog(
        c"-------------------------------------------------\n"
            .as_ptr()
            .cast_mut(),
    );
    verboselog(c"  NOP: %.4000s\n".as_ptr().cast_mut(), comment);
    verboselog(
        c"-------------------------------------------------\n"
            .as_ptr()
            .cast_mut(),
    );
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_append_layout(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    cpath: *const ::core::ffi::c_char,
) {
    let mut content: JsonContent = JsonContent::Unknown;
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut errormsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    verboselog(c"Appending layout \"%s\"\n".as_ptr().cast_mut(), cpath);
    let path: *mut ::core::ffi::c_char = resolve_tilde(cpath);
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: isize = 0;
    len = slurp(path, &raw mut buf);
    if len < 0 as isize {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"Could not slurp \"%s\".".as_ptr(), path);
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
    } else if !json_validate(buf, len as usize) {
        errorlog(
            c"ERROR: Could not parse \"%s\" as JSON, not loading.\n"
                .as_ptr()
                .cast_mut(),
            path,
        );
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Could not parse \"%s\" as JSON.".as_ptr(),
                path,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else {
        content = json_determine_content(buf, len as usize);
        verboselog(
            c"JSON content = %d\n".as_ptr().cast_mut(),
            content as ::core::ffi::c_uint,
        );
        if content == JsonContent::Unknown {
            errorlog(
                c"ERROR: Could not determine the contents of \"%s\", not loading.\n"
                    .as_ptr()
                    .cast_mut(),
                path,
            );
            if (*cmd_output).has_reply() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Could not determine the contents of \"%s\".".as_ptr(),
                    path,
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                );
                free(message_1 as *mut ::core::ffi::c_void);
            }
        } else {
            parent = focused;
            if content == JsonContent::Workspace {
                parent = output_get_content(con_get_output(parent));
            } else {
                while (*parent).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !con_accepts_window(parent)
                {
                    parent = parent_ptr(parent);
                }
            }
            debuglog(
                c"%s:%s:%d - Appending to parent=%p instead of focused=%p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_append_layout".as_ptr(),
                784 as ::core::ffi::c_int,
                parent,
                focused,
            );
            errormsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            tree_append_json(parent, buf, len as usize, &raw mut errormsg);
            if !errormsg.is_null() {
                if (*cmd_output).has_reply() {
                    let mut message_2: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(&raw mut message_2, c"%s".as_ptr(), errormsg);
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
                    );
                    free(message_2 as *mut ::core::ffi::c_void);
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
            (*cmd_output).needs_tree_render = true_0 != 0;
        }
    }
    free(path as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
}
unsafe fn disable_global_fullscreen() {
    let fs: *mut Con = con_get_fullscreen_con(croot, CF_GLOBAL);
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
    debuglog(
        c"%s:%s:%d - which=%s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_workspace".as_ptr(),
        832 as ::core::ffi::c_int,
        which,
    );
    disable_global_fullscreen();
    if strcmp(which, c"next".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_next();
    } else if strcmp(which, c"prev".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_prev();
    } else if strcmp(which, c"next_on_output".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_next_on_output();
    } else if strcmp(which, c"prev_on_output".as_ptr()) == 0 as ::core::ffi::c_int {
        ws = workspace_prev_on_output();
    } else {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"BUG: called with which=%s".as_ptr(),
                which,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    workspace_show(ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    let parsed_num: ::core::ffi::c_long = ws_name_to_number(which) as ::core::ffi::c_long;
    if parsed_num == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Could not parse initial part of \"%s\" as a number.".as_ptr(),
                which,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let workspace: *mut Con = get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
    if workspace.is_null() {
        verboselog(
            c"There is no workspace with number %ld, creating a new one.\n"
                .as_ptr()
                .cast_mut(),
            parsed_num,
        );
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        workspace_show_by_name(which);
        (*cmd_output).needs_tree_render = true_0 != 0;
        return;
    }
    if !no_auto_back_and_forth
        && maybe_back_and_forth(cmd_output, (*workspace).name) as ::core::ffi::c_int != 0
    {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        return;
    }
    workspace_show(workspace);
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    if strncasecmp(name, c"__".as_ptr(), strlen(c"__".as_ptr())) == 0 as ::core::ffi::c_int {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You cannot switch to the i3-internal workspaces (\"%s\").".as_ptr(),
                name,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    disable_global_fullscreen();
    debuglog(
        c"%s:%s:%d - should switch to workspace %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_workspace_name".as_ptr(),
        918 as ::core::ffi::c_int,
        name,
    );
    if !no_auto_back_and_forth && maybe_back_and_forth(cmd_output, name) as ::core::ffi::c_int != 0
    {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
        return;
    }
    workspace_show_by_name(name);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_mark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
    toggle: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let current: *const owindow = (*(*cmd_output).ctx).owindows.tqh_first;
    if current.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Given criteria don't match a window".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if current
        != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last
            as *const owindow
    {
        if (*cmd_output).has_reply() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"A mark must not be put onto more than one window".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
            );
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    debuglog(
        c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_mark".as_ptr(),
        949 as ::core::ffi::c_int,
        (*current).con,
        (*(*current).con).name,
    );
    let mark_mode: MarkMode =
        if mode.is_null() || strcmp(mode, c"--replace".as_ptr()) == 0 as ::core::ffi::c_int {
            MarkMode::Replace
        } else {
            MarkMode::Add
        };
    if !toggle.is_null() {
        con_mark_toggle((*current).con, mark, mark_mode);
    } else {
        con_mark((*current).con, mark, mark_mode);
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_unmark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: *const ::core::ffi::c_char,
) {
    if match_is_empty(current_match) {
        con_unmark(::core::ptr::null_mut::<Con>(), mark);
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            con_unmark((*current).con, mark);
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mode: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - mode=%s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_mode".as_ptr(),
        987 as ::core::ffi::c_int,
        mode,
    );
    switch_mode(mode);
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
                added.push(std::ffi::CStr::from_ptr(output_primary_name(output)).to_owned());
            }
            output = (*output).outputs.tqe_next;
        }
    } else {
        added.push(std::ffi::CStr::from_ptr(name).to_owned());
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
            target = get_output_from_string(current_output, name.as_ptr());
        }
        if strcasecmp(output_primary_name(current_output), name.as_ptr()) == 0 {
            for next in &snapshot[index + 1..] {
                let output = get_output_from_string(current_output, next.as_ptr());
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
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if names
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .is_empty()
    {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"At least one output must be specified".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut success: bool = false_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let ws: *mut Con = con_get_workspace((*current).con);
        if !con_is_internal(ws) {
            let current_output: *mut Output = get_output_for_con(ws);
            let target_output: *mut Output = user_output_names_find_next(&names, current_output);
            if !target_output.is_null() {
                if move_workspace {
                    workspace_move_to_output(ws, target_output);
                } else {
                    con_move_to_output((*current).con, target_output, true_0 != 0);
                }
                success = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    user_output_names_free(&names);
    (*cmd_output).needs_tree_render = success;
    if success {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    } else if (*cmd_output).has_reply() {
        let mut message_1: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut message_1, c"No output matched".as_ptr());
        emit_command_reply(
            cmd_output,
            CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
        );
        free(message_1 as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn cmd_move_con_to_mark(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mark: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - moving window to mark \"%s\"\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_move_con_to_mark".as_ptr(),
        1123 as ::core::ffi::c_int,
        mark,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut result: bool = true_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - moving matched window %p / %s to mark \"%s\"\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_move_con_to_mark".as_ptr(),
            1130 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
            mark,
        );
        result = (result as ::core::ffi::c_int
            & con_move_to_mark((*current).con, mark) as ::core::ffi::c_int)
            != 0;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success((result)));
    }
}

pub unsafe fn cmd_floating(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    floating_mode: *const ::core::ffi::c_char,
) {
    if floating_mode != ::core::ptr::null_mut::<::core::ffi::c_void>() as *const ::core::ffi::c_char
    {
    } else {
        __assert_fail(
            c"floating_mode != NULL".as_ptr(),
            c"../src/commands.c".as_ptr(),
            1143 as ::core::ffi::c_uint,
            c"void cmd_floating(Match *, struct CommandResultIR *, const char *)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - floating_mode=%s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_floating".as_ptr(),
        1144 as ::core::ffi::c_int,
        floating_mode,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_floating".as_ptr(),
            1150 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        if strcmp(floating_mode, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
            debuglog(
                c"%s:%s:%d - should toggle mode\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_floating".as_ptr(),
                1152 as ::core::ffi::c_int,
            );
            toggle_floating_mode((*current).con, false_0 != 0);
        } else {
            debuglog(
                c"%s:%s:%d - should switch mode to %s\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_floating".as_ptr(),
                1155 as ::core::ffi::c_int,
                floating_mode,
            );
            if strcmp(floating_mode, c"enable".as_ptr()) == 0 as ::core::ffi::c_int {
                floating_enable((*current).con, false_0 != 0);
            } else {
                floating_disable((*current).con);
            }
        }
        run_assignments((*(*current).con).window as *mut i3Window);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_split(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    verboselog(
        c"splitting in direction %c\n".as_ptr().cast_mut(),
        *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    );
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_docked((*current).con) {
            errorlog(
                c"ERROR: Cannot split a docked container, skipping.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            debuglog(
                c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_split".as_ptr(),
                1185 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            if *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 't' as i32
            {
                let mut current_layout: Layout = Layout::Default;
                if (*(*current).con).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
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
                    if *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'v' as i32
                    {
                        Orientation::Vertical
                    } else {
                        Orientation::Horizontal
                    },
                );
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    debuglog(
        c"%s:%s:%d - kill_mode=%s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_kill".as_ptr(),
        1218 as ::core::ffi::c_int,
        kill_mode_str,
    );
    let mut kill_mode: KillWindow = KillWindow::DontKill;
    if strcmp(kill_mode_str, c"window".as_ptr()) == 0 as ::core::ffi::c_int {
        kill_mode = KillWindow::Window;
    } else if strcmp(kill_mode_str, c"client".as_ptr()) == 0 as ::core::ffi::c_int {
        kill_mode = KillWindow::Client;
    } else {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"BUG: called with kill_mode=%s".as_ptr(),
                kill_mode_str,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        con_close((*current).con, kill_mode);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            c"WARNING: Your criteria for the exec command match %d containers, so the command will execute this many times.\n".as_ptr().cast_mut(),
            count,
        );
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - should execute %s, no_startup_id = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_exec".as_ptr(),
            1264 as ::core::ffi::c_int,
            command,
            no_startup_id as ::core::ffi::c_int,
        );
        start_application(
            command,
            no_startup_id,
            (if (*(*current).con).window.is_null() {
                XCB_NONE
            } else {
                (*(*(*current).con).window).id as ::core::ffi::c_long
            }) as xcb_window_t,
        );
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            c"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n".as_ptr().cast_mut(),
            count,
        );
    }
    let mut direction: Direction = Direction::Left;
    let mut position: Position = Position::Before;
    let mut auto_direction: bool = true_0 != 0;
    if strcmp(direction_str, c"prev".as_ptr()) == 0 as ::core::ffi::c_int {
        position = Position::Before;
    } else if strcmp(direction_str, c"next".as_ptr()) == 0 as ::core::ffi::c_int {
        position = Position::After;
    } else {
        auto_direction = false_0 != 0;
        direction = parse_direction(direction_str);
    }
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws) as ::core::ffi::c_int != 0) {
            if auto_direction {
                let o: Orientation = con_orientation(parent_ptr((*current_0).con));
                direction = direction_from_orientation_position(o, position);
            }
            tree_next((*current_0).con, direction);
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_sibling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            c"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n".as_ptr().cast_mut(),
            count,
        );
    }
    let direction: Position =
        if strncasecmp(direction_str, c"prev".as_ptr(), strlen(c"prev".as_ptr()))
            == 0 as ::core::ffi::c_int
        {
            Position::Before
        } else {
            Position::After
        };
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !(ws.is_null() || con_is_internal(ws) as ::core::ffi::c_int != 0) {
            let next: *mut Con = get_tree_next_sibling((*current_0).con, direction);
            if !next.is_null() {
                if (*next).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    workspace_show(next);
                    con_focus(next);
                } else {
                    con_activate(next);
                }
            }
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_focus_window_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    window_mode: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - window_mode = %s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_focus_window_mode".as_ptr(),
        1366 as ::core::ffi::c_int,
        window_mode,
    );
    let mut to_floating: bool = false_0 != 0;
    if strcmp(window_mode, c"mode_toggle".as_ptr()) == 0 as ::core::ffi::c_int {
        to_floating = con_inside_floating(focused).is_null();
    } else if strcmp(window_mode, c"floating".as_ptr()) == 0 as ::core::ffi::c_int {
        to_floating = true_0 != 0;
    } else if strcmp(window_mode, c"tiling".as_ptr()) == 0 as ::core::ffi::c_int {
        to_floating = false_0 != 0;
    }
    let ws: *const Con = con_get_workspace(focused);
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut success: bool = false_0 != 0;
    current = (*ws).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        if to_floating as ::core::ffi::c_int != 0
            && (*current).type_0 as ::core::ffi::c_uint
                != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
            || !to_floating
                && (*current).type_0 as ::core::ffi::c_uint
                    == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current = (*current).focused.tqe_next as *mut Con;
        } else {
            con_activate_unblock(con_descend_focused(current));
            success = true_0 != 0;
            break;
        }
    }
    if success {
        (*cmd_output).needs_tree_render = true_0 != 0;
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    } else if (*cmd_output).has_reply() {
        let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut message,
            c"Failed to find a %s container in workspace.".as_ptr(),
            if to_floating as ::core::ffi::c_int != 0 {
                c"floating".as_ptr()
            } else {
                c"tiling".as_ptr()
            },
        );
        emit_command_reply(
            cmd_output,
            CommandReply::Error(std::ffi::CStr::from_ptr(message)),
        );
        free(message as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn cmd_focus_level(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    level: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - level = %s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_focus_level".as_ptr(),
        1404 as ::core::ffi::c_int,
        level,
    );
    let mut success: bool = false_0 != 0;
    if strcmp(level, c"parent".as_ptr()) == 0 as ::core::ffi::c_int {
        if !focused.is_null() && !parent_ptr(focused).is_null() {
            if con_fullscreen_permits_focusing(parent_ptr(focused)) {
                success = level_up();
            } else {
                errorlog(
                    c"ERROR: 'focus parent': Currently in fullscreen, not going up\n"
                        .as_ptr()
                        .cast_mut(),
                );
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
    debuglog(
        c"%s:%s:%d - current_match = %p\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_focus".as_ptr(),
        1434 as ::core::ffi::c_int,
        current_match,
    );
    if match_is_empty(current_match) {
        errorlog(
            c"ERROR: You have to specify which window/container should be focused.\n"
                .as_ptr()
                .cast_mut(),
        );
        errorlog(
            c"ERROR: Example: [class=\"urxvt\" title=\"irssi\"] focus\n"
                .as_ptr()
                .cast_mut(),
        );
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You have to specify which window/container should be focused".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    } else if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"No window matches given criteria".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        count += 1;
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if count > 1 as ::core::ffi::c_int {
        verboselog(
            c"WARNING: Your criteria for the focus command matches %d containers, while only exactly one container can be focused at a time.\n".as_ptr().cast_mut(),
            count,
        );
    }
    let mut __i3_scratch: *const Con = workspace_get(c"__i3_scratch".as_ptr());
    let mut current_0: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current_0 = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current_0.is_null() {
        let ws: *mut Con = con_get_workspace((*current_0).con);
        if !ws.is_null() {
            if ws == __i3_scratch as *mut Con && !focus_workspace {
                scratchpad_show((*current_0).con);
                break;
            } else if focus_workspace {
                verboselog(
                    c"focusing workspace %p / %s - %p / %s\n"
                        .as_ptr()
                        .cast_mut(),
                    (*current_0).con,
                    (*(*current_0).con).name,
                    ws,
                    (*ws).name,
                );
                workspace_show(maybe_auto_back_and_forth_workspace(ws));
            } else {
                verboselog(
                    c"focusing %p / %s\n".as_ptr().cast_mut(),
                    (*current_0).con,
                    (*(*current_0).con).name,
                );
                con_activate_unblock((*current_0).con);
            }
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    let mode: fullscreen_mode_t =
        (if strcmp(fullscreen_mode, c"global".as_ptr()) == 0 as ::core::ffi::c_int {
            CF_GLOBAL as ::core::ffi::c_int
        } else {
            CF_OUTPUT as ::core::ffi::c_int
        }) as fullscreen_mode_t;
    debuglog(
        c"%s:%s:%d - %s fullscreen, mode = %s\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_fullscreen".as_ptr(),
        1490 as ::core::ffi::c_int,
        action,
        fullscreen_mode,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_fullscreen".as_ptr(),
            1496 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        if strcmp(action, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
            con_toggle_fullscreen((*current).con, mode as ::core::ffi::c_int);
        } else if strcmp(action, c"enable".as_ptr()) == 0 as ::core::ffi::c_int {
            con_enable_fullscreen((*current).con, mode);
        } else if strcmp(action, c"disable".as_ptr()) == 0 as ::core::ffi::c_int {
            con_disable_fullscreen((*current).con);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_sticky(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    action: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - %s sticky on window\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_sticky".as_ptr(),
        1516 as ::core::ffi::c_int,
        action,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if (*(*current).con).window.is_null() {
            errorlog(
                c"ERROR: only containers holding a window can be made sticky, skipping con = %p\n"
                    .as_ptr()
                    .cast_mut(),
                (*current).con,
            );
        } else {
            debuglog(
                c"%s:%s:%d - setting sticky for container = %p / %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_sticky".as_ptr(),
                1525 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            let mut sticky: bool = false_0 != 0;
            if strcmp(action, c"enable".as_ptr()) == 0 as ::core::ffi::c_int {
                sticky = true_0 != 0;
            } else if strcmp(action, c"disable".as_ptr()) == 0 as ::core::ffi::c_int {
                sticky = false_0 != 0;
            } else if strcmp(action, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
                sticky = !(*(*current).con).sticky;
            }
            (*(*current).con).sticky = sticky;
            ewmh_update_sticky((*(*(*current).con).window).id, sticky);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    output_push_sticky_windows(focused);
    ewmh_update_wm_desktop();
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
    amount: ::core::ffi::c_long,
    mode: *const ::core::ffi::c_char,
) {
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let direction: Direction = parse_direction(direction_str);
    let is_ppt: bool = !mode.is_null() && strcmp(mode, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int;
    debuglog(
        c"%s:%s:%d - moving in direction %s, %ld %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_move_direction".as_ptr(),
        1562 as ::core::ffi::c_int,
        direction_str,
        amount,
        mode,
    );
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_floating((*current).con) {
            debuglog(
                c"%s:%s:%d - floating move with %ld %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_move_direction".as_ptr(),
                1565 as ::core::ffi::c_int,
                amount,
                mode,
            );
            let mut newrect: Rect = (*parent_ptr((*current).con)).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            match direction {
                Direction::Left => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as u32;
                }
                Direction::Right => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as u32;
                }
                Direction::Up => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as u32;
                }
                Direction::Down => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as u32;
                }
            }
            (*cmd_output).needs_tree_render =
                floating_reposition(parent_ptr((*current).con), newrect);
        } else {
            tree_move((*current).con, direction);
            (*cmd_output).needs_tree_render = true_0 != 0;
        }
        current = (*current).owindows.tqe_next as *mut owindow;
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
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut layout: Layout = Layout::Default;
    if !layout_from_name(layout_str, &raw mut layout) {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Unknown layout \"%s\", this is a mismatch between code and parser spec.".as_ptr(),
                layout_str,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    debuglog(
        c"%s:%s:%d - changing layout to %s (%d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_layout".as_ptr(),
        1608 as ::core::ffi::c_int,
        layout_str,
        layout as ::core::ffi::c_uint,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if con_is_docked((*current).con) {
            errorlog(
                c"ERROR: cannot change layout of a docked container, skipping it.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            debuglog(
                c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_layout".as_ptr(),
                1617 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            con_set_layout((*current).con, layout);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
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
    debuglog(
        c"%s:%s:%d - toggling layout (mode = %s)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_layout_toggle".as_ptr(),
        1635 as ::core::ffi::c_int,
        toggle_mode,
    );
    if match_is_empty(current_match) {
        con_toggle_layout(focused, toggle_mode);
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            debuglog(
                c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_layout_toggle".as_ptr(),
                1643 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            con_toggle_layout((*current).con, toggle_mode);
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_exit(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    verboselog(c"Exiting due to user command.\n".as_ptr().cast_mut());
    exit(EXIT_SUCCESS);
}

pub unsafe fn cmd_reload(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    verboselog(c"reloading\n".as_ptr().cast_mut());
    kill_nagbar(config_error_nagbar_pid, false_0 != 0);
    kill_nagbar(command_error_nagbar_pid, false_0 != 0);
    command_error_nagbar_pid = -(1 as ::core::ffi::c_int) as libc::pid_t;
    config_error_nagbar_pid = command_error_nagbar_pid;
    load_configuration(
        ::core::ptr::null::<::core::ffi::c_char>(),
        ConfigLoad::Reload,
    );
    x_set_i3_atoms();
    ipc_send_workspace_event(
        c"reload".as_ptr(),
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<Con>(),
    );
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        ipc_send_barconfig_update_event(current);
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_restart(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    verboselog(c"restarting i3\n".as_ptr().cast_mut());
    let mut exempt_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !(*cmd_output).client.is_null() {
        exempt_fd = (*(*cmd_output).client).fd;
        verboselog(
            c"Carrying file descriptor %d across restart\n"
                .as_ptr()
                .cast_mut(),
            exempt_fd,
        );
        let mut flags: ::core::ffi::c_int = 0;
        flags = fcntl(exempt_fd, F_GETFD);
        if flags < 0 as ::core::ffi::c_int
            || fcntl(exempt_fd, F_SETFD, flags & !FD_CLOEXEC) < 0 as ::core::ffi::c_int
        {
            errorlog(
                c"ERROR: Could not disable FD_CLOEXEC on fd %d\n"
                    .as_ptr()
                    .cast_mut(),
                exempt_fd,
            );
        }
        let mut fdstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut fdstr, c"%d".as_ptr(), exempt_fd);
        setenv(c"_I3_RESTART_FD".as_ptr(), fdstr, 1 as ::core::ffi::c_int);
    }
    ipc_shutdown(ShutdownReason::Restart, exempt_fd);
    unlink(config.ipc_socket_path);
    if !current_log_stream_socket_path.is_null() {
        unlink(current_log_stream_socket_path);
    }
    purge_zerobyte_logfile();
    i3_restart(false_0 != 0);
    if 0 as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/commands.c".as_ptr(),
            1723 as ::core::ffi::c_uint,
            c"void cmd_restart(Match *, struct CommandResultIR *)".as_ptr(),
        );
    };
}

pub unsafe fn cmd_open(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    verboselog(c"opening new container\n".as_ptr().cast_mut());
    let con: *mut Con = tree_open_con(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*con).layout = Layout::SplitH;
    con_activate(con);
    emit_command_reply(
        cmd_output,
        CommandReply::WithId(con as usize as ::core::ffi::c_longlong),
    );
    (*cmd_output).needs_tree_render = true_0 != 0;
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
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"At least one output must be specified".as_ptr(),
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
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
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !con_is_internal(con_get_workspace((*current).con)) {
            con = (*current).con;
            break;
        } else {
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    let current_output: *mut Output = get_output_for_con(con);
    let target_output: *const Output = user_output_names_find_next(&names_global, current_output);
    user_output_names_free(&names_global);
    let mut success: bool = false_0 != 0;
    if !target_output.is_null() {
        success = true_0 != 0;
        let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*output_get_content((*target_output).con))
                .nodes_head
                .tqh_first as *mut Con;
            while !child.is_null() {
                if !workspace_is_visible(child) {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    ws = child;
                    break;
                }
            }
        }
        if ws.is_null() {
            if (*cmd_output).has_reply() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"BUG: No workspace found on output.".as_ptr(),
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                );
                free(message_1 as *mut ::core::ffi::c_void);
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
        let mut message_2: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut message_2, c"No output matched".as_ptr());
        emit_command_reply(
            cmd_output,
            CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
        );
        free(message_2 as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn cmd_move_window_to_position(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    x: ::core::ffi::c_long,
    mode_x: *const ::core::ffi::c_char,
    y: ::core::ffi::c_long,
    mode_y: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false_0 != 0;
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if !con_is_floating((*current).con) {
            errorlog(
                c"ERROR: Cannot change position. The window/container is not floating\n"
                    .as_ptr()
                    .cast_mut(),
            );
            if !has_error {
                if (*cmd_output).has_reply() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        c"Cannot change position of a window/container because it is not floating."
                            .as_ptr(),
                    );
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
                    );
                    free(message_0 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        } else {
            let mut newrect: Rect = (*parent_ptr((*current).con)).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            newrect.x = (if !mode_x.is_null()
                && strcmp(mode_x, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*output).rect.width as ::core::ffi::c_double
                    * (x as ::core::ffi::c_double / 100.0f64)
            } else {
                x as ::core::ffi::c_double
            }) as u32;
            newrect.y = (if !mode_y.is_null()
                && strcmp(mode_y, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*output).rect.height as ::core::ffi::c_double
                    * (y as ::core::ffi::c_double / 100.0f64)
            } else {
                y as ::core::ffi::c_double
            }) as u32;
            debuglog(
                c"%s:%s:%d - moving to position %d %s %d %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_move_window_to_position".as_ptr(),
                1835 as ::core::ffi::c_int,
                newrect.x,
                mode_x,
                newrect.y,
                mode_y,
            );
            if !floating_reposition(parent_ptr((*current).con), newrect) {
                if (*cmd_output).has_reply() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_1,
                        c"Cannot move window/container out of bounds.".as_ptr(),
                    );
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
                    );
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !has_error {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    }
}

pub unsafe fn cmd_move_window_to_center(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    method: *const ::core::ffi::c_char,
) {
    let mut has_error: bool = false_0 != 0;
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            errorlog(
                c"ERROR: con %p / %s is not floating, cannot move it to the center.\n"
                    .as_ptr()
                    .cast_mut(),
                (*current).con,
                (*(*current).con).name,
            );
            if !has_error {
                if (*cmd_output).has_reply() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        c"Cannot change position of a window/container because it is not floating."
                            .as_ptr(),
                    );
                    emit_command_reply(
                        cmd_output,
                        CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
                    );
                    free(message_0 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        } else {
            if strcmp(method, c"absolute".as_ptr()) == 0 as ::core::ffi::c_int {
                debuglog(
                    c"%s:%s:%d - moving to absolute center\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"cmd_move_window_to_center".as_ptr(),
                    1872 as ::core::ffi::c_int,
                );
                floating_center(floating_con, (*croot).rect as Rect);
                floating_maybe_reassign_ws(floating_con);
                (*cmd_output).needs_tree_render = true_0 != 0;
            }
            if strcmp(method, c"position".as_ptr()) == 0 as ::core::ffi::c_int {
                debuglog(
                    c"%s:%s:%d - moving to center\n".as_ptr().cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"cmd_move_window_to_center".as_ptr(),
                    1880 as ::core::ffi::c_int,
                );
                floating_center(
                    floating_con,
                    (*con_get_workspace(floating_con)).rect as Rect,
                );
                (*cmd_output).needs_tree_render = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !has_error {
        if (*cmd_output).has_reply() {
            emit_command_reply(cmd_output, CommandReply::Success(true));
        }
    }
}

pub unsafe fn cmd_move_window_to_mouse(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        let floating_con: *mut Con = con_inside_floating((*current).con);
        if floating_con.is_null() {
            debuglog(
                c"%s:%s:%d - con %p / %s is not floating, cannot move it to the mouse position.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_move_window_to_mouse".as_ptr(),
                1905 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
        } else {
            debuglog(
                c"%s:%s:%d - moving floating container %p / %s to cursor position\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_move_window_to_mouse".as_ptr(),
                1909 as ::core::ffi::c_int,
                floating_con,
                (*floating_con).name,
            );
            floating_move_to_pointer(floating_con);
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_move_scratchpad(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    debuglog(
        c"%s:%s:%d - should move window to scratchpad\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_move_scratchpad".as_ptr(),
        1922 as ::core::ffi::c_int,
    );
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_move_scratchpad".as_ptr(),
            1928 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        scratchpad_move((*current).con);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_scratchpad_show(current_match: *mut Match, cmd_output: *mut CommandResultIR) {
    debuglog(
        c"%s:%s:%d - should show scratchpad window\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_scratchpad_show".as_ptr(),
        1942 as ::core::ffi::c_int,
    );
    let mut result: bool = false_0 != 0;
    if match_is_empty(current_match) {
        result = scratchpad_show(::core::ptr::null_mut::<Con>());
    } else {
        let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
        current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
        while !current.is_null() {
            debuglog(
                c"%s:%s:%d - matching: %p / %s\n".as_ptr().cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_scratchpad_show".as_ptr(),
                1950 as ::core::ffi::c_int,
                (*current).con,
                (*(*current).con).name,
            );
            result = (result as ::core::ffi::c_int
                | scratchpad_show((*current).con) as ::core::ffi::c_int)
                != 0;
            current = (*current).owindows.tqe_next as *mut owindow;
        }
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success((result)));
    }
}

pub unsafe fn cmd_swap(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mode: *const ::core::ffi::c_char,
    arg: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let match_0: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    if match_0.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message_0, c"No match found for swapping.".as_ptr());
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if (*match_0).con.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"Match %p has no container.".as_ptr(),
                match_0,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
            );
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(mode, c"id".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut target: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target, 0 as ::core::ffi::c_int) {
            if (*cmd_output).has_reply() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    c"Failed to parse %s into a window id.".as_ptr(),
                    arg,
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_2)),
                );
                free(message_2 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_window_id(target as xcb_window_t);
    } else if strcmp(mode, c"con_id".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut target_0: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target_0, 0 as ::core::ffi::c_int) {
            if (*cmd_output).has_reply() {
                let mut message_3: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_3,
                    c"Failed to parse %s into a container id.".as_ptr(),
                    arg,
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message_3)),
                );
                free(message_3 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_con_id(target_0);
    } else if strcmp(mode, c"mark".as_ptr()) == 0 as ::core::ffi::c_int {
        con = con_by_mark(arg);
    } else {
        if (*cmd_output).has_reply() {
            let mut message_4: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_4,
                c"Unhandled swap mode \"%s\". This is a bug.".as_ptr(),
                mode,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_4)),
            );
            free(message_4 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if con.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_5: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_5,
                c"Could not find container for %s = %s".as_ptr(),
                mode,
                arg,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_5)),
            );
            free(message_5 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_0 != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last {
        verboselog(
            c"More than one container matched the swap command, only using the first one."
                .as_ptr()
                .cast_mut(),
        );
    }
    debuglog(
        c"%s:%s:%d - Swapping %p with %p.\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_swap".as_ptr(),
        2010 as ::core::ffi::c_int,
        (*match_0).con,
        con,
    );
    let result: bool = con_swap((*match_0).con, con) as bool;
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success((result)));
    }
}

pub unsafe fn cmd_title_format(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    format: *const ::core::ffi::c_char,
) {
    debuglog(
        c"%s:%s:%d - setting title_format to \"%s\"\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_title_format".as_ptr(),
        2023 as ::core::ffi::c_int,
        format,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        debuglog(
            c"%s:%s:%d - setting title_format for %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_title_format".as_ptr(),
            2028 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
        );
        free((*(*current).con).title_format as *mut ::core::ffi::c_void);
        (*(*current).con).title_format = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if strcasecmp(format, c"%title".as_ptr()) != 0 as ::core::ffi::c_int {
            (*(*current).con).title_format = sstrdup(format);
            if !(*(*current).con).window.is_null() {
                let formatted_title = con_parse_title_format((*current).con);
                ewmh_update_visible_name(
                    (*(*(*current).con).window).id,
                    formatted_title
                        .cstr_or_report()
                        .map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
                );
            }
        } else if !(*(*current).con).window.is_null() {
            ewmh_update_visible_name(
                (*(*(*current).con).window).id,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true_0 != 0;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_title_window_icon(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    enable: *const ::core::ffi::c_char,
    mut padding: ::core::ffi::c_int,
) {
    let mut is_toggle: bool = false_0 != 0;
    if !enable.is_null() {
        if strcmp(enable, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
            is_toggle = true_0 != 0;
        } else if !boolstr(enable) {
            padding = -(1 as ::core::ffi::c_int);
        }
    }
    debuglog(
        c"%s:%s:%d - setting window_icon=%d\n".as_ptr().cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"cmd_title_window_icon".as_ptr(),
        2074 as ::core::ffi::c_int,
        padding,
    );
    if !(*current_match).error.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if match_is_empty(current_match) {
        while !(*(*cmd_output).ctx).owindows.tqh_first.is_null() {
            let ow: *mut owindow = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
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
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut current: *mut owindow = ::core::ptr::null_mut::<owindow>();
    current = (*(*cmd_output).ctx).owindows.tqh_first as *mut owindow;
    while !current.is_null() {
        if is_toggle {
            let current_padding: ::core::ffi::c_int = (*(*current).con).window_icon_padding;
            if padding > 0 as ::core::ffi::c_int {
                if current_padding < 0 as ::core::ffi::c_int {
                    (*(*current).con).window_icon_padding = padding;
                } else {
                    (*(*current).con).window_icon_padding = -(padding + 1 as ::core::ffi::c_int);
                }
            } else {
                (*(*current).con).window_icon_padding =
                    -(current_padding + 1 as ::core::ffi::c_int);
            }
        } else {
            (*(*current).con).window_icon_padding = padding;
        }
        debuglog(
            c"%s:%s:%d - Set window_icon for %p / %s to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"cmd_title_window_icon".as_ptr(),
            2095 as ::core::ffi::c_int,
            (*current).con,
            (*(*current).con).name,
            (*(*current).con).window_icon_padding,
        );
        if !(*(*current).con).window.is_null() {
            (*(*(*current).con).window).name_x_changed = true_0 != 0;
        } else {
            free((*(*current).con).deco_render_params as *mut ::core::ffi::c_void);
            (*(*current).con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_rename_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut old_name: *const ::core::ffi::c_char,
    new_name: *const ::core::ffi::c_char,
) {
    if strncasecmp(new_name, c"__".as_ptr(), strlen(c"__".as_ptr())) == 0 as ::core::ffi::c_int {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Cannot rename workspace to \"%s\": names starting with __ are i3-internal."
                    .as_ptr(),
                new_name,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !old_name.is_null() {
        verboselog(
            c"Renaming workspace \"%s\" to \"%s\"\n".as_ptr().cast_mut(),
            old_name,
            new_name,
        );
    } else {
        verboselog(
            c"Renaming current workspace to \"%s\"\n"
                .as_ptr()
                .cast_mut(),
            new_name,
        );
    }
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    if !old_name.is_null() {
        workspace = get_existing_workspace_by_name(old_name);
    } else {
        workspace = con_get_workspace(focused);
        old_name = (*workspace).name;
    }
    if workspace.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Old workspace \"%s\" not found".as_ptr(),
                old_name,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let check_dest: *const Con = get_existing_workspace_by_name(new_name);
    if !check_dest.is_null() && check_dest != workspace as *const Con {
        if (*cmd_output).has_reply() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"New workspace \"%s\" already exists".as_ptr(),
                new_name,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_1)),
            );
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let old_name_copy: *mut ::core::ffi::c_char = sstrdup(old_name);
    free((*workspace).name as *mut ::core::ffi::c_void);
    (*workspace).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*workspace).name = sstrdup(new_name);
    (*workspace).num = ws_name_to_number(new_name);
    verboselog(c"num = %d\n".as_ptr().cast_mut(), (*workspace).num);
    let previously_focused: *mut Con = focused;
    let previously_focused_content: *const Con = if (*focused).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        parent_ptr(focused)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    let parent: *mut Con = parent_ptr(workspace);
    con_detach(workspace);
    con_attach(workspace, parent, false_0 != 0);
    ipc_send_workspace_event(
        c"rename".as_ptr(),
        workspace,
        ::core::ptr::null_mut::<Con>(),
    );
    let assigned: *mut Con =
        get_assigned_output((*workspace).name, (*workspace).num as ::core::ffi::c_long);
    if !assigned.is_null() {
        workspace_move_to_output(workspace, get_output_for_con(assigned));
    }
    let mut can_restore_focus: bool = !previously_focused.is_null();
    if !previously_focused_content.is_null() {
        let mut ws: *const Con = ::core::ptr::null::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*previously_focused_content).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !(child == previously_focused) {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    ws = child;
                    break;
                }
            }
        }
        can_restore_focus = (can_restore_focus as ::core::ffi::c_int
            & (ws != NULL as *const Con) as ::core::ffi::c_int)
            != 0;
    }
    if can_restore_focus {
        workspace_show(con_get_workspace(previously_focused));
        con_focus(previously_focused);
    }
    if !previous_workspace_name.is_null() && strcmp(previous_workspace_name, old_name_copy) == 0 {
        free(previous_workspace_name as *mut ::core::ffi::c_void);
        previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        previous_workspace_name = sstrdup(new_name);
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
    ewmh_update_desktop_properties();
    startup_sequence_rename_workspace(old_name_copy, new_name);
    free(old_name_copy as *mut ::core::ffi::c_void);
}

pub unsafe fn cmd_bar_mode(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    bar_mode: *const ::core::ffi::c_char,
    bar_id: *const ::core::ffi::c_char,
) {
    let mut mode: BarMode = BarMode::Dock;
    let mut toggle: bool = false_0 != 0;
    if strcmp(bar_mode, c"dock".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = BarMode::Dock;
    } else if strcmp(bar_mode, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = BarMode::Hide;
    } else if strcmp(bar_mode, c"invisible".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = BarMode::Invisible;
    } else if strcmp(bar_mode, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
        toggle = true_0 != 0;
    } else {
        errorlog(
            c"ERROR: Unknown bar mode \"%s\", this is a mismatch between code and parser spec.\n"
                .as_ptr()
                .cast_mut(),
            bar_mode,
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                c"false".as_ptr(),
                c"../src/commands.c".as_ptr(),
                2220 as ::core::ffi::c_uint,
                c"void cmd_bar_mode(Match *, struct CommandResultIR *, const char *, const char *)"
                    .as_ptr(),
            );
        };
    }
    if barconfigs.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"No bars found\n".as_ptr());
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                mode = (*current).mode.toggled();
            }
            debuglog(
                c"%s:%s:%d - Changing bar mode of bar_id '%s' from '%d' to '%s (%d)'\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_bar_mode".as_ptr(),
                2239 as ::core::ffi::c_int,
                (*current).id,
                (*current).mode as ::core::ffi::c_uint,
                bar_mode,
                (mode) as u32,
            );
            if (*current).mode != mode {
                (*current).mode = mode;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Success(true));
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Changing bar mode of bar_id %s failed, bar_id not found.\n".as_ptr(),
                bar_id,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}

pub unsafe fn cmd_bar_hidden_state(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    bar_hidden_state: *const ::core::ffi::c_char,
    bar_id: *const ::core::ffi::c_char,
) {
    let mut hidden_state: BarHiddenState = BarHiddenState::Show;
    let mut toggle: bool = false_0 != 0;
    if strcmp(bar_hidden_state, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
        hidden_state = BarHiddenState::Hide;
    } else if strcmp(bar_hidden_state, c"show".as_ptr()) == 0 as ::core::ffi::c_int {
        hidden_state = BarHiddenState::Show;
    } else if strcmp(bar_hidden_state, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
        toggle = true_0 != 0;
    } else {
        errorlog(
            c"ERROR: Unknown bar state \"%s\", this is a mismatch between code and parser spec.\n"
                .as_ptr()
                .cast_mut(),
            bar_hidden_state,
        );
        if 0 as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                c"false".as_ptr(),
                c"../src/commands.c".as_ptr(),
                2277 as ::core::ffi::c_uint,
                c"void cmd_bar_hidden_state(Match *, struct CommandResultIR *, const char *, const char *)".as_ptr(),
            );
        };
    }
    if barconfigs.tqh_first.is_null() {
        if (*cmd_output).has_reply() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"No bars found\n".as_ptr());
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message)),
            );
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                hidden_state = (*current).hidden_state.toggled();
            }
            debuglog(
                c"%s:%s:%d - Changing bar hidden_state of bar_id '%s' from '%d' to '%s (%d)'\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/commands.c".as_ptr(),
                c"cmd_bar_hidden_state".as_ptr(),
                2296 as ::core::ffi::c_int,
                (*current).id,
                (*current).hidden_state as ::core::ffi::c_uint,
                bar_hidden_state,
                (hidden_state) as u32,
            );
            if (*current).hidden_state != hidden_state {
                (*current).hidden_state = hidden_state;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if (*cmd_output).has_reply() {
                    emit_command_reply(cmd_output, CommandReply::Success(true));
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if (*cmd_output).has_reply() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Changing bar hidden_state of bar_id %s failed, bar_id not found.\n".as_ptr(),
                bar_id,
            );
            emit_command_reply(
                cmd_output,
                CommandReply::Error(std::ffi::CStr::from_ptr(message_0)),
            );
            free(message_0 as *mut ::core::ffi::c_void);
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
        shmlog_size = 0 as ::core::ffi::c_int;
    } else {
        let mut new_size: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        if !parse_long(argument, &raw mut new_size, 0 as ::core::ffi::c_int) {
            if (*cmd_output).has_reply() {
                let mut message: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message,
                    c"Failed to parse %s into a shmlog size.".as_ptr(),
                    argument,
                );
                emit_command_reply(
                    cmd_output,
                    CommandReply::Error(std::ffi::CStr::from_ptr(message)),
                );
                free(message as *mut ::core::ffi::c_void);
            }
            return;
        }
        if shmlog_size > 0 as ::core::ffi::c_int {
            shmlog_size = 0 as ::core::ffi::c_int;
            verboselog(c"Restarting shm logging...\n".as_ptr().cast_mut());
            init_logging();
        }
        shmlog_size = new_size as ::core::ffi::c_int;
    }
    verboselog(
        c"%s shm logging\n".as_ptr().cast_mut(),
        if shmlog_size > 0 as ::core::ffi::c_int {
            c"Enabling".as_ptr()
        } else {
            c"Disabling".as_ptr()
        },
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
        verboselog(
            c"%s debug logging\n".as_ptr().cast_mut(),
            if logging as ::core::ffi::c_int != 0 {
                c"Disabling".as_ptr()
            } else {
                c"Enabling".as_ptr()
            },
        );
        set_debug_logging(!logging);
    } else if strcmp(argument, c"on".as_ptr()) == 0 && !logging {
        verboselog(c"Enabling debug logging\n".as_ptr().cast_mut());
        set_debug_logging(true_0 != 0);
    } else if strcmp(argument, c"off".as_ptr()) == 0 && logging as ::core::ffi::c_int != 0 {
        verboselog(c"Disabling debug logging\n".as_ptr().cast_mut());
        set_debug_logging(false_0 != 0);
    }
    if (*cmd_output).has_reply() {
        emit_command_reply(cmd_output, CommandReply::Success(true));
    }
}
unsafe fn gaps_inner(gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    &raw mut (*gaps).inner
}
unsafe fn gaps_top(gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    &raw mut (*gaps).top
}
unsafe fn gaps_left(gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    &raw mut (*gaps).left
}
unsafe fn gaps_bottom(gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    &raw mut (*gaps).bottom
}
unsafe fn gaps_right(gaps: *mut gaps_t) -> *mut ::core::ffi::c_int {
    &raw mut (*gaps).right
}
unsafe fn gaps_update(
    get: gap_accessor,
    scope: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
    pixels: ::core::ffi::c_int,
) -> bool {
    debuglog(
        c"%s:%s:%d - gaps_update(scope=%s, mode=%s, pixels=%d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"gaps_update".as_ptr(),
        2394 as ::core::ffi::c_int,
        scope,
        mode,
        pixels,
    );
    let workspace: *mut Con = con_get_workspace(focused);
    let global_gap_size: ::core::ffi::c_int =
        *get.expect("non-null function pointer")(&raw mut config.gaps);
    let mut current_value: ::core::ffi::c_int = global_gap_size;
    if strcmp(scope, c"current".as_ptr()) == 0 as ::core::ffi::c_int {
        current_value += *get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
    }
    debuglog(
        c"%s:%s:%d - global_gap_size=%d, current_value=%d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/commands.c".as_ptr(),
        c"gaps_update".as_ptr(),
        2402 as ::core::ffi::c_int,
        global_gap_size,
        current_value,
    );
    let mut reset: bool = false_0 != 0;
    if strcmp(mode, c"plus".as_ptr()) == 0 as ::core::ffi::c_int {
        current_value += pixels;
    } else if strcmp(mode, c"minus".as_ptr()) == 0 as ::core::ffi::c_int {
        current_value -= pixels;
    } else if strcmp(mode, c"set".as_ptr()) == 0 as ::core::ffi::c_int {
        current_value = pixels;
        reset = true_0 != 0;
    } else if strcmp(mode, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
        current_value = (current_value == 0) as ::core::ffi::c_int * pixels;
        reset = true_0 != 0;
    } else {
        errorlog(
            c"ERROR: Invalid mode %s when changing gaps"
                .as_ptr()
                .cast_mut(),
            mode,
        );
        return false_0 != 0;
    }
    let mut min_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let is_outer: bool = get.expect("non-null function pointer")(&raw mut config.gaps)
        != gaps_inner(&raw mut config.gaps);
    if is_outer {
        if strcmp(scope, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
            min_value = -config.gaps.inner;
        } else {
            min_value = -config.gaps.inner - (*workspace).gaps.inner;
        }
    }
    if current_value < min_value {
        current_value = min_value;
    }
    if strcmp(scope, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
        output = (*croot).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            let mut cur_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let content: *const Con = output_get_content(output);
            cur_ws = (*content).nodes_head.tqh_first as *mut Con;
            while !cur_ws.is_null() {
                let gaps_value: *mut ::core::ffi::c_int =
                    get.expect("non-null function pointer")(&raw mut (*cur_ws).gaps);
                debuglog(
                    c"%s:%s:%d - current gaps_value = %d\n".as_ptr().cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"gaps_update".as_ptr(),
                    2443 as ::core::ffi::c_int,
                    *gaps_value,
                );
                if reset {
                    *gaps_value = 0 as ::core::ffi::c_int;
                } else {
                    let mut max_compensate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if is_outer {
                        max_compensate = config.gaps.inner;
                    }
                    if *gaps_value + current_value + max_compensate < 0 as ::core::ffi::c_int {
                        *gaps_value = -current_value - max_compensate;
                    }
                }
                debuglog(
                    c"%s:%s:%d - gaps_value after fix = %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/commands.c".as_ptr(),
                    c"gaps_update".as_ptr(),
                    2459 as ::core::ffi::c_int,
                    *gaps_value,
                );
                cur_ws = (*cur_ws).nodes.tqe_next as *mut Con;
            }
            output = (*output).nodes.tqe_next as *mut Con;
        }
        *get.expect("non-null function pointer")(&raw mut config.gaps) = current_value;
        debuglog(
            c"%s:%s:%d - global gaps value after fix = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/commands.c".as_ptr(),
            c"gaps_update".as_ptr(),
            2464 as ::core::ffi::c_int,
            *get.expect("non-null function pointer")(&raw mut config.gaps),
        );
    } else {
        let gaps_value_0: *mut ::core::ffi::c_int =
            get.expect("non-null function pointer")(&raw mut (*workspace).gaps);
        *gaps_value_0 = current_value - global_gap_size;
    }
    true_0 != 0
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
    let pixels: ::core::ffi::c_int = logical_px(atoi(value)) as ::core::ffi::c_int;
    if strcmp(type_0, c"inner".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_inner as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else if !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0 as ::core::ffi::c_int,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            c"all".as_ptr(),
            c"plus".as_ptr(),
            0 as ::core::ffi::c_int,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else if strcmp(type_0, c"outer".as_ptr()) == 0 {
        if !gaps_update(
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) || !gaps_update(
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_top as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_bottom as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_right as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
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
            Some(gaps_left as unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int),
            scope,
            mode,
            pixels,
        ) {
            current_block = 1063758793336646565;
        } else {
            current_block = 11057878835866523405;
        }
    } else {
        errorlog(
            c"ERROR: Invalid type %s when changing gaps"
                .as_ptr()
                .cast_mut(),
            type_0,
        );
        current_block = 1063758793336646565;
    }
    match current_block {
        1063758793336646565 => {
            if (*cmd_output).has_reply() {
                emit_command_reply(cmd_output, CommandReply::Success(false));
            }
        }
        _ => {
            (*cmd_output).needs_tree_render = true_0 != 0;
            if (*cmd_output).has_reply() {
                emit_command_reply(cmd_output, CommandReply::Success(true));
            }
        }
    }
}
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

use crate::regex::regex_matches;

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
use i3_common::boolstr::boolstr;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{xcb_window_t, XCB_NONE};

pub use libc::{atoi, exit, fcntl, free, setenv, strcasecmp, strcmp, strlen, strncasecmp, unlink};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

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
use crate::r#match::match_free;
use crate::r#match::match_init;
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
use i3_common::resolve_tilde::resolve_tilde;

#[cfg(test)]
mod argument_tests {
    use super::*;
    #[test]
    fn direction_arguments_remain_exact_case_sensitive_keywords() {
        assert!(matches!(direction_argument(b"left"), Some(Direction::Left)));
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
        assert_eq!(snapshot[0].as_bytes(), b"HDMI-\xff");
        user_output_names_free(&accumulator);
        user_output_names_free(&accumulator);
        assert!(accumulator.lock().unwrap().is_empty());
        assert_eq!(snapshot[0].as_bytes(), b"HDMI-\xff");
    }
}
