use crate::config::barconfigs;
use crate::ipc::ipc_send_barconfig_update_event;
use crate::randr::outputs;
use crate::tree::all_cons;
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
pub use i3_json::compat::yajl_gen;
use i3_json::compat::{
    yajl_gen_bool, yajl_gen_integer, yajl_gen_map_close, yajl_gen_map_open, yajl_gen_string,
};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_3 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_3 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_3 = 0;

pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_8 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_8 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_8 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_8 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_10 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_10 = 1;
pub const M_HERE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_11 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_11 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_11 = 4;
pub const WM_TILING: C2RustUnnamed_11 = 3;
pub const WM_TILING_USER: C2RustUnnamed_11 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_11 = 1;
pub const WM_ANY: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_12 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_12 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_12 = 1;
pub const M_NODOCK: C2RustUnnamed_12 = 0;
pub const M_DONTCHECK: C2RustUnnamed_12 = -1;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_13 = 1;
pub const U_LATEST: C2RustUnnamed_13 = 0;
pub const U_DONTCHECK: C2RustUnnamed_13 = -1;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_14 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_14 = 1;
pub const W_NODOCK: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_17 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_17 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_17 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_17 = 2;
pub const A_COMMAND: C2RustUnnamed_17 = 1;
pub const A_ANY: C2RustUnnamed_17 = 0;

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_19 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_19 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_19 = 3;
pub const CT_CON: C2RustUnnamed_19 = 2;
pub const CT_OUTPUT: C2RustUnnamed_19 = 1;
pub const CT_ROOT: C2RustUnnamed_19 = 0;

pub type Output = xoutput;
pub type i3Window = Window;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type position_t = ::core::ffi::c_uint;
pub const AFTER: position_t = 1;
pub const BEFORE: position_t = 0;
pub type kill_window_t = ::core::ffi::c_uint;
pub const KILL_CLIENT: kill_window_t = 2;
pub const KILL_WINDOW: kill_window_t = 1;
pub const DONT_KILL_WINDOW: kill_window_t = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type mark_mode_t = ::core::ffi::c_uint;
pub const MM_ADD: mark_mode_t = 1;
pub const MM_REPLACE: mark_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_21 = 3;
pub const PDF_IGNORE: C2RustUnnamed_21 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_21 = 1;
pub const PDF_SMART: C2RustUnnamed_21 = 0;

pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_22 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_22 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_23 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_23 = 2;
pub const FOWA_URGENT: C2RustUnnamed_23 = 1;
pub const FOWA_SMART: C2RustUnnamed_23 = 0;
pub use crate::application::Barconfig;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const P_TOP: C2RustUnnamed_25 = 1;
pub const P_BOTTOM: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub const S_SHOW: C2RustUnnamed_27 = 1;
pub const S_HIDE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const M_INVISIBLE: C2RustUnnamed_28 = 2;
pub const M_HIDE: C2RustUnnamed_28 = 1;
pub const M_DOCK: C2RustUnnamed_28 = 0;
pub type config_load_t = ::core::ffi::c_uint;
pub const C_RELOAD: config_load_t = 2;
pub const C_LOAD: config_load_t = 1;
pub const C_VALIDATE: config_load_t = 0;
pub use crate::application::ipc_client;
pub type shutdown_reason_t = ::core::ffi::c_uint;
pub const SHUTDOWN_REASON_EXIT: shutdown_reason_t = 1;
pub const SHUTDOWN_REASON_RESTART: shutdown_reason_t = 0;
pub type json_content_t = ::core::ffi::c_uint;
pub const JSON_CONTENT_WORKSPACE: json_content_t = 2;
pub const JSON_CONTENT_CON: json_content_t = 1;
pub const JSON_CONTENT_UNKNOWN: json_content_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_32,
    pub val: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_32 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_32 = 1;
pub const STACK_STR: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
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
    pub json_gen: yajl_gen,
    pub client: *mut ipc_client,
    pub next_state: ::core::ffi::c_int,
    pub needs_tree_render: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parser_ctx {
    pub state: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_output_names_head {
    pub tqh_first: *mut user_output_name,
    pub tqh_last: *mut *mut user_output_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_output_name {
    pub name: *mut ::core::ffi::c_char,
    pub user_output_names: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut user_output_name,
    pub tqe_prev: *mut *mut user_output_name,
}
pub type gap_accessor = Option<unsafe fn(*mut gaps_t) -> *mut ::core::ffi::c_int>;
static mut names: user_output_names_head = user_output_names_head {
    tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
    tqh_last: ::core::ptr::null::<*mut user_output_name>() as *mut *mut user_output_name,
};
static mut names_global: user_output_names_head = user_output_names_head {
    tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
    tqh_last: ::core::ptr::null::<*mut user_output_name>() as *mut *mut user_output_name,
};
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
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current_block_41: u64;
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
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
                                        smalloc(::core::mem::size_of::<owindow>() as size_t)
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
        con = (*con).all_cons.tqe_next as *mut Con;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                c"BUG: called with which=%s".as_ptr(),
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_2 as *mut ::core::ffi::c_uchar,
                    strlen(message_2),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_2 as *mut ::core::ffi::c_void);
        }
        return;
    }
    move_matches_to_workspace(&raw mut (*(*cmd_output).ctx).owindows, ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_move_con_to_workspace_back_and_forth(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    let ws: *mut Con = workspace_back_and_forth_get();
    if ws.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"No workspace was previously active.".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_move_con_to_workspace_name(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
    no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if strncasecmp(name, c"__".as_ptr(), strlen(c"__".as_ptr())) == 0 as ::core::ffi::c_int {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You cannot move containers to i3-internal workspaces (\"%s\").".as_ptr(),
                name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_2 as *mut ::core::ffi::c_uchar,
                        strlen(message_2),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_move_con_to_workspace_number(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    which: *const ::core::ffi::c_char,
    no_auto_back_and_forth: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Nothing to move: specified criteria don't match any window".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Nothing to move: workspace empty".as_ptr(),
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_2,
                c"Could not parse number \"%s\"".as_ptr(),
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_2 as *mut ::core::ffi::c_uchar,
                    strlen(message_2),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe fn parse_direction(str: *const ::core::ffi::c_char) -> direction_t {
    if strcmp(str, c"left".as_ptr()) == 0 as ::core::ffi::c_int {
        return D_LEFT;
    }
    if strcmp(str, c"right".as_ptr()) == 0 as ::core::ffi::c_int {
        return D_RIGHT;
    }
    if strcmp(str, c"up".as_ptr()) == 0 as ::core::ffi::c_int {
        return D_UP;
    }
    if strcmp(str, c"down".as_ptr()) == 0 as ::core::ffi::c_int {
        return D_DOWN;
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
    let mut direction: direction_t = D_LEFT;
    if strcmp(direction_str, c"height".as_ptr()) == 0 as ::core::ffi::c_int {
        direction = D_DOWN;
    } else if strcmp(direction_str, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
        direction = D_RIGHT;
    } else {
        direction = parse_direction(direction_str);
    }
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let window: *const i3Window = (*focused_con).window;
    if !window.is_null() {
        if orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint {
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
    if orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(px as uint32_t);
    } else {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(px as uint32_t);
    }
    floating_check_size(
        floating_con,
        orientation as ::core::ffi::c_uint == VERT as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if rect_equals(old_rect, (*floating_con).rect as Rect) {
        return;
    }
    if direction as ::core::ffi::c_uint == D_UP as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*floating_con).rect.y = (*floating_con)
            .rect
            .y
            .wrapping_sub((*floating_con).rect.height.wrapping_sub(old_rect.height));
    } else if direction as ::core::ffi::c_uint
        == D_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*floating_con).rect.x = (*floating_con)
            .rect
            .x
            .wrapping_sub((*floating_con).rect.width.wrapping_sub(old_rect.width));
    }
    if (*floating_con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*floating_con).scratchpad_state = SCRATCHPAD_CHANGED;
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
    let search_direction: direction_t = parse_direction(direction) as direction_t;
    let res: bool = resize_find_tiling_participants(
        &raw mut first,
        &raw mut second,
        search_direction,
        false_0 != 0,
    ) as bool;
    if !res {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"No second container found in this direction.".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    let search_direction: direction_t =
        (if strcmp(direction, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
            D_LEFT as ::core::ffi::c_int
        } else {
            D_DOWN as ::core::ffi::c_int
        }) as direction_t;
    let search_result: bool = resize_find_tiling_participants(
        &raw mut current,
        &raw mut dummy,
        search_direction,
        true_0 != 0,
    ) as bool;
    if search_result as ::core::ffi::c_int == false_0 {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Failed to find appropriate tiling containers for resize operation".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return false_0 != 0;
    }
    let children: ::core::ffi::c_int =
        con_num_children((*current).parent as *mut Con) as ::core::ffi::c_int;
    verboselog(c"ins. %d children\n".as_ptr().cast_mut(), children);
    let percentage: ::core::ffi::c_double = 1.0f64 / children as ::core::ffi::c_double;
    verboselog(c"default percentage = %f\n".as_ptr().cast_mut(), percentage);
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
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
            / con_rect_size_in_orientation((*current).parent as *mut Con) as ::core::ffi::c_double;
        new_current_percent = (*current).percent + ppt;
    }
    let subtract_percent: ::core::ffi::c_double =
        ppt / (children - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    if ppt < 0.0f64 && new_current_percent < percent_for_1px(current) {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Not resizing, container would end with less than 1px".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if !(child == current) {
                if (*child).percent - subtract_percent < percent_for_1px(child) {
                    if !(*cmd_output).json_gen.is_null() {
                        let mut message_1: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut message_1,
                            c"Not resizing, already at minimum size (child %p would end up with a size of %.f".as_ptr(),
                            child,
                            (*child).percent - subtract_percent,
                        );
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_open((*cmd_output).json_gen);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                                strlen(c"success".as_ptr()),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_bool((*cmd_output).json_gen, false_0);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                                strlen(c"error".as_ptr()),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                message_1 as *mut ::core::ffi::c_uchar,
                                strlen(message_1),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_close((*cmd_output).json_gen);
                        } else {
                        };
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
    child = (*(*current).parent).nodes_head.tqh_first as *mut Con;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
                    if !(*cmd_output).json_gen.is_null() {
                        let mut message_0: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(&raw mut message_0, c"Cannot resize.".as_ptr());
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_open((*cmd_output).json_gen);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                                strlen(c"success".as_ptr()),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_bool((*cmd_output).json_gen, false_0);
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                                strlen(c"error".as_ptr()),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_string(
                                (*cmd_output).json_gen,
                                message_0 as *mut ::core::ffi::c_uchar,
                                strlen(message_0),
                            );
                        } else {
                        };
                        if !(*cmd_output).json_gen.is_null() {
                            yajl_gen_map_close((*cmd_output).json_gen);
                        } else {
                        };
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
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(&raw mut message_1, c"Cannot resize.".as_ptr());
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"error".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_1 as *mut ::core::ffi::c_uchar,
                            strlen(message_1),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                return;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe fn resize_set_tiling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut target: *mut Con,
    resize_orientation: orientation_t,
    is_ppt: bool,
    target_size: ::core::ffi::c_long,
) -> bool {
    let mut search_direction: direction_t = D_LEFT;
    let mut mode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if resize_orientation as ::core::ffi::c_uint
        == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        search_direction = D_LEFT;
        mode = c"width".as_ptr().cast_mut();
    } else {
        search_direction = D_DOWN;
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
            - (if resize_orientation as ::core::ffi::c_uint
                == HORIZ as ::core::ffi::c_int as ::core::ffi::c_uint
            {
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"Dimensions cannot be negative.".as_ptr());
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
            floating_resize(floating_con, cwidth as uint32_t, cheight as uint32_t);
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
                        HORIZ,
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
                        VERT,
                        is_ppt_0,
                        cheight,
                    ) as ::core::ffi::c_int)
                    != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, success as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe fn border_width_from_style(
    border_style: border_style_t,
    border_width: ::core::ffi::c_long,
    con: *mut Con,
) -> ::core::ffi::c_int {
    if border_style as ::core::ffi::c_uint == BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    if border_width >= 0 as ::core::ffi::c_long {
        return logical_px(border_width as ::core::ffi::c_int);
    }
    let is_floating: bool = !con_inside_floating(con).is_null();
    if is_floating as ::core::ffi::c_int != 0
        && border_style as ::core::ffi::c_uint
            == config.default_floating_border as ::core::ffi::c_uint
    {
        config.default_floating_border_width
    } else if !is_floating
        && border_style as ::core::ffi::c_uint == config.default_border as ::core::ffi::c_uint
    {
        config.default_border_width
    } else {
        logical_px(
            if border_style as ::core::ffi::c_uint
                == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
        )
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
        let mut border_style: border_style_t = BS_NONE;
        if strcmp(border_style_str, c"toggle".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = ((*(*current).con).border_style as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_rem(3 as ::core::ffi::c_uint)
                as border_style_t;
        } else if strcmp(border_style_str, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BS_NORMAL;
        } else if strcmp(border_style_str, c"pixel".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BS_PIXEL;
        } else if strcmp(border_style_str, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
            border_style = BS_NONE;
        } else {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_0: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_0,
                    c"BUG: called with border_style=%s".as_ptr(),
                    border_style_str,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_0 as *mut ::core::ffi::c_uchar,
                        strlen(message_0),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_append_layout(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    cpath: *const ::core::ffi::c_char,
) {
    let mut content: json_content_t = JSON_CONTENT_UNKNOWN;
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut errormsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    verboselog(c"Appending layout \"%s\"\n".as_ptr().cast_mut(), cpath);
    let path: *mut ::core::ffi::c_char = resolve_tilde(cpath);
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ssize_t = 0;
    len = slurp(path, &raw mut buf);
    if len < 0 as ssize_t {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"Could not slurp \"%s\".".as_ptr(), path);
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
    } else if !json_validate(buf, len as size_t) {
        errorlog(
            c"ERROR: Could not parse \"%s\" as JSON, not loading.\n"
                .as_ptr()
                .cast_mut(),
            path,
        );
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Could not parse \"%s\" as JSON.".as_ptr(),
                path,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else {
        content = json_determine_content(buf, len as size_t) as json_content_t;
        verboselog(
            c"JSON content = %d\n".as_ptr().cast_mut(),
            content as ::core::ffi::c_uint,
        );
        if content as ::core::ffi::c_uint
            == JSON_CONTENT_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            errorlog(
                c"ERROR: Could not determine the contents of \"%s\", not loading.\n"
                    .as_ptr()
                    .cast_mut(),
                path,
            );
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"Could not determine the contents of \"%s\".".as_ptr(),
                    path,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
        } else {
            parent = focused;
            if content as ::core::ffi::c_uint
                == JSON_CONTENT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                parent = output_get_content(con_get_output(parent));
            } else {
                while (*parent).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !con_accepts_window(parent)
                {
                    parent = (*parent).parent as *mut Con;
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
            tree_append_json(parent, buf, len as size_t, &raw mut errormsg);
            if !errormsg.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_2: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(&raw mut message_2, c"%s".as_ptr(), errormsg);
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"error".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_2 as *mut ::core::ffi::c_uchar,
                            strlen(message_2),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_2 as *mut ::core::ffi::c_void);
                }
                free(errormsg as *mut ::core::ffi::c_void);
            } else if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
            render_con(croot);
            restore_open_placeholder_windows(parent);
            if content as ::core::ffi::c_uint
                == JSON_CONTENT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"BUG: called with which=%s".as_ptr(),
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    workspace_show(ws);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Could not parse initial part of \"%s\" as a number.".as_ptr(),
                which,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        workspace_show_by_name(which);
        (*cmd_output).needs_tree_render = true_0 != 0;
        return;
    }
    if !no_auto_back_and_forth
        && maybe_back_and_forth(cmd_output, (*workspace).name) as ::core::ffi::c_int != 0
    {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        return;
    }
    workspace_show(workspace);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_workspace_back_and_forth(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    disable_global_fullscreen();
    workspace_back_and_forth();
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You cannot switch to the i3-internal workspaces (\"%s\").".as_ptr(),
                name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
        return;
    }
    workspace_show_by_name(name);
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Given criteria don't match a window".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if current
        != *(*((*(*cmd_output).ctx).owindows.tqh_last as *mut owindows_head)).tqh_last
            as *const owindow
    {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"A mark must not be put onto more than one window".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    let mark_mode: mark_mode_t =
        (if mode.is_null() || strcmp(mode, c"--replace".as_ptr()) == 0 as ::core::ffi::c_int {
            MM_REPLACE as ::core::ffi::c_int
        } else {
            MM_ADD as ::core::ffi::c_int
        }) as mark_mode_t;
    if !toggle.is_null() {
        con_mark_toggle((*current).con, mark, mark_mode);
    } else {
        con_mark((*current).con, mark, mark_mode);
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}
unsafe fn user_output_names_add(
    list: *mut user_output_names_head,
    name: *const ::core::ffi::c_char,
) {
    let get_non_primary: bool = strcasecmp(c"nonprimary".as_ptr(), name) == 0 as ::core::ffi::c_int;
    if get_non_primary as ::core::ffi::c_int != 0
        || strcmp(name, c"next".as_ptr()) == 0 as ::core::ffi::c_int
    {
        let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
        output = outputs.tqh_first as *mut Output;
        while !output.is_null() {
            if !(get_non_primary as ::core::ffi::c_int != 0
                && (*output).primary as ::core::ffi::c_int != 0)
            {
                let co: *mut user_output_name = scalloc(
                    ::core::mem::size_of::<user_output_name>() as size_t,
                    1 as size_t,
                ) as *mut user_output_name;
                (*co).name = sstrdup(output_primary_name(output));
                (*co).user_output_names.tqe_next = ::core::ptr::null_mut::<user_output_name>();
                (*co).user_output_names.tqe_prev = (*list).tqh_last;
                *(*list).tqh_last = co as *mut user_output_name;
                (*list).tqh_last = &raw mut (*co).user_output_names.tqe_next;
            }
            output = (*output).outputs.tqe_next as *mut Output;
        }
        return;
    }
    let co_0: *mut user_output_name = scalloc(
        ::core::mem::size_of::<user_output_name>() as size_t,
        1 as size_t,
    ) as *mut user_output_name;
    (*co_0).name = sstrdup(name);
    (*co_0).user_output_names.tqe_next = ::core::ptr::null_mut::<user_output_name>();
    (*co_0).user_output_names.tqe_prev = (*list).tqh_last;
    *(*list).tqh_last = co_0 as *mut user_output_name;
    (*list).tqh_last = &raw mut (*co_0).user_output_names.tqe_next;
}
unsafe fn user_output_names_find_next(
    names_0: *const user_output_names_head,
    current_output: *mut Output,
) -> *mut Output {
    let mut target_output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut uo: *mut user_output_name = ::core::ptr::null_mut::<user_output_name>();
    uo = (*names_0).tqh_first as *mut user_output_name;
    while !uo.is_null() {
        if target_output.is_null() {
            target_output = get_output_from_string(current_output, (*uo).name);
        }
        if strcasecmp(output_primary_name(current_output), (*uo).name) == 0 as ::core::ffi::c_int {
            loop {
                uo = (*uo).user_output_names.tqe_next as *mut user_output_name;
                if uo.is_null() {
                    break;
                }
                let out: *mut Output = get_output_from_string(current_output, (*uo).name);
                if !out.is_null() {
                    return out;
                }
            }
            break;
        } else {
            uo = (*uo).user_output_names.tqe_next as *mut user_output_name;
        }
    }
    target_output
}
unsafe fn user_output_names_free(names_0: *mut user_output_names_head) {
    while !(*names_0).tqh_first.is_null() {
        let uo: *mut user_output_name = (*names_0).tqh_first as *mut user_output_name;
        free((*uo).name as *mut ::core::ffi::c_void);
        if !(*uo).user_output_names.tqe_next.is_null() {
            (*(*uo).user_output_names.tqe_next)
                .user_output_names
                .tqe_prev = (*uo).user_output_names.tqe_prev;
        } else {
            (*names_0).tqh_last = (*uo).user_output_names.tqe_prev;
        }
        *(*uo).user_output_names.tqe_prev = (*uo).user_output_names.tqe_next;
        free(uo as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn cmd_move_con_to_output(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
    move_workspace: bool,
) {
    if !name.is_null() {
        user_output_names_add(&raw mut names, name);
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if names.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"At least one output must be specified".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            let target_output: *mut Output =
                user_output_names_find_next(&raw mut names, current_output);
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
    user_output_names_free(&raw mut names);
    (*cmd_output).needs_tree_render = success;
    if success {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
        let mut message_1: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut message_1, c"No output matched".as_ptr());
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"error".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message_1 as *mut ::core::ffi::c_uchar,
                strlen(message_1),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_split(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
                let mut current_layout: layout_t = L_DEFAULT;
                if (*(*current).con).type_0 as ::core::ffi::c_uint
                    == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    current_layout = (*(*current).con).layout;
                } else {
                    current_layout = (*(*(*current).con).parent).layout;
                }
                if current_layout as ::core::ffi::c_uint
                    == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    tree_split((*current).con, VERT);
                } else {
                    tree_split((*current).con, HORIZ);
                }
            } else {
                tree_split(
                    (*current).con,
                    (if *direction.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'v' as i32
                    {
                        VERT as ::core::ffi::c_int
                    } else {
                        HORIZ as ::core::ffi::c_int
                    }) as orientation_t,
                );
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    let mut kill_mode: ::core::ffi::c_int = 0;
    if strcmp(kill_mode_str, c"window".as_ptr()) == 0 as ::core::ffi::c_int {
        kill_mode = KILL_WINDOW as ::core::ffi::c_int;
    } else if strcmp(kill_mode_str, c"client".as_ptr()) == 0 as ::core::ffi::c_int {
        kill_mode = KILL_CLIENT as ::core::ffi::c_int;
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"BUG: called with kill_mode=%s".as_ptr(),
                kill_mode_str,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
        con_close((*current).con, kill_mode as kill_window_t);
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_focus_direction(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    let mut direction: direction_t = D_LEFT;
    let mut position: position_t = BEFORE;
    let mut auto_direction: bool = true_0 != 0;
    if strcmp(direction_str, c"prev".as_ptr()) == 0 as ::core::ffi::c_int {
        position = BEFORE;
    } else if strcmp(direction_str, c"next".as_ptr()) == 0 as ::core::ffi::c_int {
        position = AFTER;
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
                let o: orientation_t =
                    con_orientation((*(*current_0).con).parent as *mut Con) as orientation_t;
                direction = direction_from_orientation_position(o, position);
            }
            tree_next((*current_0).con, direction);
        }
        current_0 = (*current_0).owindows.tqe_next as *mut owindow;
    }
    (*cmd_output).needs_tree_render = true_0 != 0;
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_focus_sibling(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    direction_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    let direction: position_t =
        (if strncasecmp(direction_str, c"prev".as_ptr(), strlen(c"prev".as_ptr()))
            == 0 as ::core::ffi::c_int
        {
            BEFORE as ::core::ffi::c_int
        } else {
            AFTER as ::core::ffi::c_int
        }) as position_t;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
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
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"error".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message as *mut ::core::ffi::c_uchar,
                strlen(message),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !focused.is_null() && !(*focused).parent.is_null() {
            if con_fullscreen_permits_focusing((*focused).parent as *mut Con) {
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, success as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"You have to specify which window/container should be focused".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    } else if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"No window matches given criteria".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let direction: direction_t = parse_direction(direction_str) as direction_t;
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
            let mut newrect: Rect = (*(*(*current).con).parent).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            match direction as ::core::ffi::c_uint {
                0 => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                1 => {
                    newrect.x = (newrect.x as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.width as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                2 => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        - if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                3 => {
                    newrect.y = (newrect.y as ::core::ffi::c_double
                        + if is_ppt as ::core::ffi::c_int != 0 {
                            (*output).rect.height as ::core::ffi::c_double
                                * (amount as ::core::ffi::c_double / 100.0f64)
                        } else {
                            amount as ::core::ffi::c_double
                        }) as uint32_t;
                }
                _ => {}
            }
            (*cmd_output).needs_tree_render =
                floating_reposition((*(*current).con).parent as *mut Con, newrect);
        } else {
            tree_move((*current).con, direction);
            (*cmd_output).needs_tree_render = true_0 != 0;
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_layout(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    layout_str: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    let mut layout: layout_t = L_DEFAULT;
    if !layout_from_name(layout_str, &raw mut layout) {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Unknown layout \"%s\", this is a mismatch between code and parser spec.".as_ptr(),
                layout_str,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    command_error_nagbar_pid = -(1 as ::core::ffi::c_int) as pid_t;
    config_error_nagbar_pid = command_error_nagbar_pid;
    load_configuration(::core::ptr::null::<::core::ffi::c_char>(), C_RELOAD);
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    ipc_shutdown(SHUTDOWN_REASON_RESTART, exempt_fd);
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
    (*con).layout = L_SPLITH;
    con_activate(con);
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_map_open((*cmd_output).json_gen);
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_string(
            (*cmd_output).json_gen,
            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"success".as_ptr()),
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_bool((*cmd_output).json_gen, true_0);
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_string(
            (*cmd_output).json_gen,
            c"id".as_ptr() as *mut ::core::ffi::c_uchar,
            strlen(c"id".as_ptr()),
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_integer(
            (*cmd_output).json_gen,
            con as uintptr_t as ::core::ffi::c_longlong,
        );
    } else {
    };
    if !(*cmd_output).json_gen.is_null() {
        yajl_gen_map_close((*cmd_output).json_gen);
    } else {
    };
    (*cmd_output).needs_tree_render = true_0 != 0;
}

pub unsafe fn cmd_focus_output(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    name: *const ::core::ffi::c_char,
) {
    if !name.is_null() {
        user_output_names_add(&raw mut names_global, name);
        return;
    }
    if names_global.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"At least one output must be specified".as_ptr(),
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
        (*ow_0).con = focused;
        (*(*cmd_output).ctx).owindows.tqh_first = ::core::ptr::null_mut::<owindow>();
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*(*cmd_output).ctx).owindows.tqh_first;
        (*ow_0).owindows.tqe_next = ::core::ptr::null_mut::<owindow>();
        (*ow_0).owindows.tqe_prev = (*(*cmd_output).ctx).owindows.tqh_last;
        *(*(*cmd_output).ctx).owindows.tqh_last = ow_0 as *mut owindow;
        (*(*cmd_output).ctx).owindows.tqh_last = &raw mut (*ow_0).owindows.tqe_next;
    }
    if (*(*cmd_output).ctx).owindows.tqh_first.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    let target_output: *const Output =
        user_output_names_find_next(&raw mut names_global, current_output);
    user_output_names_free(&raw mut names_global);
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_1,
                    c"BUG: No workspace found on output.".as_ptr(),
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_1 as *mut ::core::ffi::c_uchar,
                        strlen(message_1),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_1 as *mut ::core::ffi::c_void);
            }
            return;
        }
        workspace_show(ws);
    }
    (*cmd_output).needs_tree_render = success;
    if success {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    } else if !(*cmd_output).json_gen.is_null() {
        let mut message_2: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut message_2, c"No output matched".as_ptr());
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, false_0);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"error".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                message_2 as *mut ::core::ffi::c_uchar,
                strlen(message_2),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        c"Cannot change position of a window/container because it is not floating."
                            .as_ptr(),
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"error".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_0 as *mut ::core::ffi::c_uchar,
                            strlen(message_0),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_0 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        } else {
            let mut newrect: Rect = (*(*(*current).con).parent).rect as Rect;
            let output: *const Con = con_get_output((*current).con);
            newrect.x = (if !mode_x.is_null()
                && strcmp(mode_x, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*output).rect.width as ::core::ffi::c_double
                    * (x as ::core::ffi::c_double / 100.0f64)
            } else {
                x as ::core::ffi::c_double
            }) as uint32_t;
            newrect.y = (if !mode_y.is_null()
                && strcmp(mode_y, c"ppt".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*output).rect.height as ::core::ffi::c_double
                    * (y as ::core::ffi::c_double / 100.0f64)
            } else {
                y as ::core::ffi::c_double
            }) as uint32_t;
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
            if !floating_reposition((*(*current).con).parent as *mut Con, newrect) {
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_1,
                        c"Cannot move window/container out of bounds.".as_ptr(),
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"error".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_1 as *mut ::core::ffi::c_uchar,
                            strlen(message_1),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                    free(message_1 as *mut ::core::ffi::c_void);
                }
                has_error = true_0 != 0;
            }
        }
        current = (*current).owindows.tqe_next as *mut owindow;
    }
    if !has_error {
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
                if !(*cmd_output).json_gen.is_null() {
                    let mut message_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sasprintf(
                        &raw mut message_0,
                        c"Cannot change position of a window/container because it is not floating."
                            .as_ptr(),
                    );
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, false_0);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"error".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            message_0 as *mut ::core::ffi::c_uchar,
                            strlen(message_0),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
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
        if !(*cmd_output).json_gen.is_null() {
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
        }
    }
}

pub unsafe fn cmd_move_window_to_mouse(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_swap(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mode: *const ::core::ffi::c_char,
    arg: *const ::core::ffi::c_char,
) {
    if !(*current_match).error.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message_0, c"No match found for swapping.".as_ptr());
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if (*match_0).con.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"Match %p has no container.".as_ptr(),
                match_0,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_1 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if strcmp(mode, c"id".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut target: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target, 0 as ::core::ffi::c_int) {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_2,
                    c"Failed to parse %s into a window id.".as_ptr(),
                    arg,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_2 as *mut ::core::ffi::c_uchar,
                        strlen(message_2),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_2 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_window_id(target as xcb_window_t);
    } else if strcmp(mode, c"con_id".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut target_0: ::core::ffi::c_long = 0;
        if !parse_long(arg, &raw mut target_0, 0 as ::core::ffi::c_int) {
            if !(*cmd_output).json_gen.is_null() {
                let mut message_3: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message_3,
                    c"Failed to parse %s into a container id.".as_ptr(),
                    arg,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message_3 as *mut ::core::ffi::c_uchar,
                        strlen(message_3),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
                free(message_3 as *mut ::core::ffi::c_void);
            }
            return;
        }
        con = con_by_con_id(target_0);
    } else if strcmp(mode, c"mark".as_ptr()) == 0 as ::core::ffi::c_int {
        con = con_by_mark(arg);
    } else {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_4: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_4,
                c"Unhandled swap mode \"%s\". This is a bug.".as_ptr(),
                mode,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_4 as *mut ::core::ffi::c_uchar,
                    strlen(message_4),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_4 as *mut ::core::ffi::c_void);
        }
        return;
    }
    if con.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_5: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_5,
                c"Could not find container for %s = %s".as_ptr(),
                mode,
                arg,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_5 as *mut ::core::ffi::c_uchar,
                    strlen(message_5),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, result as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
                let mut formatted_title: *mut i3String = con_parse_title_format((*current).con);
                ewmh_update_visible_name(
                    (*(*(*current).con).window).id,
                    i3string_as_utf8(formatted_title),
                );
                i3string_free(formatted_title);
                formatted_title = ::core::ptr::null_mut::<i3String>();
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Invalid match: %s".as_ptr(),
                (*current_match).error,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
            smalloc(::core::mem::size_of::<owindow>() as size_t) as *mut owindow;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_rename_workspace(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    mut old_name: *const ::core::ffi::c_char,
    new_name: *const ::core::ffi::c_char,
) {
    if strncasecmp(new_name, c"__".as_ptr(), strlen(c"__".as_ptr())) == 0 as ::core::ffi::c_int {
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message,
                c"Cannot rename workspace to \"%s\": names starting with __ are i3-internal."
                    .as_ptr(),
                new_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Old workspace \"%s\" not found".as_ptr(),
                old_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
        return;
    }
    let check_dest: *const Con = get_existing_workspace_by_name(new_name);
    if !check_dest.is_null() && check_dest != workspace as *const Con {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_1,
                c"New workspace \"%s\" already exists".as_ptr(),
                new_name,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_1 as *mut ::core::ffi::c_uchar,
                    strlen(message_1),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
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
        (*focused).parent
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    let parent: *mut Con = (*workspace).parent as *mut Con;
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    let mut mode: ::core::ffi::c_int = M_DOCK as ::core::ffi::c_int;
    let mut toggle: bool = false_0 != 0;
    if strcmp(bar_mode, c"dock".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = M_DOCK as ::core::ffi::c_int;
    } else if strcmp(bar_mode, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = M_HIDE as ::core::ffi::c_int;
    } else if strcmp(bar_mode, c"invisible".as_ptr()) == 0 as ::core::ffi::c_int {
        mode = M_INVISIBLE as ::core::ffi::c_int;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"No bars found\n".as_ptr());
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                mode = ((*current).mode as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_rem(2 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
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
                mode,
            );
            if (*current).mode as ::core::ffi::c_int != mode {
                (*current).mode = mode as C2RustUnnamed_28;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Changing bar mode of bar_id %s failed, bar_id not found.\n".as_ptr(),
                bar_id,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
    }
}

pub unsafe fn cmd_bar_hidden_state(
    current_match: *mut Match,
    cmd_output: *mut CommandResultIR,
    bar_hidden_state: *const ::core::ffi::c_char,
    bar_id: *const ::core::ffi::c_char,
) {
    let mut hidden_state: ::core::ffi::c_int = S_SHOW as ::core::ffi::c_int;
    let mut toggle: bool = false_0 != 0;
    if strcmp(bar_hidden_state, c"hide".as_ptr()) == 0 as ::core::ffi::c_int {
        hidden_state = S_HIDE as ::core::ffi::c_int;
    } else if strcmp(bar_hidden_state, c"show".as_ptr()) == 0 as ::core::ffi::c_int {
        hidden_state = S_SHOW as ::core::ffi::c_int;
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
        if !(*cmd_output).json_gen.is_null() {
            let mut message: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(&raw mut message, c"No bars found\n".as_ptr());
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message as *mut ::core::ffi::c_uchar,
                    strlen(message),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message as *mut ::core::ffi::c_void);
        }
        return;
    }
    let mut current: *mut Barconfig = ::core::ptr::null_mut::<Barconfig>();
    current = barconfigs.tqh_first as *mut Barconfig;
    while !current.is_null() {
        if !(!bar_id.is_null() && strcmp((*current).id, bar_id) != 0 as ::core::ffi::c_int) {
            if toggle {
                hidden_state = ((*current).hidden_state as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_rem(2 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
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
                hidden_state,
            );
            if (*current).hidden_state as ::core::ffi::c_int != hidden_state {
                (*current).hidden_state = hidden_state as C2RustUnnamed_27;
                ipc_send_barconfig_update_event(current);
            }
            if !bar_id.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_open((*cmd_output).json_gen);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_string(
                            (*cmd_output).json_gen,
                            c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                            strlen(c"success".as_ptr()),
                        );
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                    } else {
                    };
                    if !(*cmd_output).json_gen.is_null() {
                        yajl_gen_map_close((*cmd_output).json_gen);
                    } else {
                    };
                }
                return;
            }
        }
        current = (*current).configs.tqe_next as *mut Barconfig;
    }
    if !bar_id.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            let mut message_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut message_0,
                c"Changing bar hidden_state of bar_id %s failed, bar_id not found.\n".as_ptr(),
                bar_id,
            );
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_open((*cmd_output).json_gen);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"success".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_bool((*cmd_output).json_gen, false_0);
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                    strlen(c"error".as_ptr()),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_string(
                    (*cmd_output).json_gen,
                    message_0 as *mut ::core::ffi::c_uchar,
                    strlen(message_0),
                );
            } else {
            };
            if !(*cmd_output).json_gen.is_null() {
                yajl_gen_map_close((*cmd_output).json_gen);
            } else {
            };
            free(message_0 as *mut ::core::ffi::c_void);
        }
    } else if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
            if !(*cmd_output).json_gen.is_null() {
                let mut message: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut message,
                    c"Failed to parse %s into a shmlog size.".as_ptr(),
                    argument,
                );
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, false_0);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"error".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"error".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        message as *mut ::core::ffi::c_uchar,
                        strlen(message),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
    if !(*cmd_output).json_gen.is_null() {
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_open((*cmd_output).json_gen);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_string(
                (*cmd_output).json_gen,
                c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                strlen(c"success".as_ptr()),
            );
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
        } else {
        };
        if !(*cmd_output).json_gen.is_null() {
            yajl_gen_map_close((*cmd_output).json_gen);
        } else {
        };
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
            if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 0 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
        }
        _ => {
            (*cmd_output).needs_tree_render = true_0 != 0;
            if !(*cmd_output).json_gen.is_null() {
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_open((*cmd_output).json_gen);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_string(
                        (*cmd_output).json_gen,
                        c"success".as_ptr() as *mut ::core::ffi::c_uchar,
                        strlen(c"success".as_ptr()),
                    );
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_bool((*cmd_output).json_gen, 1 as ::core::ffi::c_int);
                } else {
                };
                if !(*cmd_output).json_gen.is_null() {
                    yajl_gen_map_close((*cmd_output).json_gen);
                } else {
                };
            }
        }
    }
}
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    names = user_output_names_head {
        tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
        tqh_last: &raw const names.tqh_first as *mut *mut user_output_name,
    };
    names_global = user_output_names_head {
        tqh_first: ::core::ptr::null::<user_output_name>() as *mut user_output_name,
        tqh_last: &raw const names_global.tqh_first as *mut *mut user_output_name,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

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
pub use i3_common::string::I3String as _i3String;

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
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
