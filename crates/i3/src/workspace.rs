use crate::application::bindings;
use crate::application::ws_assignments;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * workspace.c: Modifying workspaces, accessing them, moving containers to
 *              workspaces.
 *
 */
// Mechanical file-by-file Rust translation of src/workspace.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{
    ev_loop, ev_timer, ev_timer_again, ev_timer_start, ev_timer_stop, ev_tstamp, ev_watcher,
    ev_watcher_time,
};

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::Workspace_Assignment;

pub type ylength = usize;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_WORKSPACE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 0 as ::core::ffi::c_ulong;

pub static mut previous_workspace_name: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static BINDING_WORKSPACE_NAMES: std::sync::Mutex<Vec<std::ffi::CString>> =
    std::sync::Mutex::new(Vec::new());

pub unsafe fn get_existing_workspace_by_name(name: *const ::core::ffi::c_char) -> *mut Con {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if strcasecmp((*child).name, name) != 0 {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    workspace = child;
                    break;
                }
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    workspace
}

pub unsafe fn get_existing_workspace_by_num(num: ::core::ffi::c_int) -> *mut Con {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !((*child).num == num) {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    workspace = child;
                    break;
                }
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    workspace
}
unsafe fn _workspace_apply_default_orientation(ws: *mut Con) {
    if config.default_orientation == Orientation::None {
        let output: *mut Con = con_get_output(ws);
        (*ws).layout = if (*output).rect.height > (*output).rect.width {
            Layout::SplitV
        } else {
            Layout::SplitH
        };
        (*ws).rect = (*output).rect;
        debuglog(
            c"%s:%s:%d - Auto orientation. Workspace size set to (%d,%d), setting layout to %d.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"_workspace_apply_default_orientation".as_ptr(),
            67 as ::core::ffi::c_int,
            (*output).rect.width,
            (*output).rect.height,
            (*ws).layout as ::core::ffi::c_uint,
        );
    } else {
        (*ws).layout = if config.default_orientation == Orientation::Horizontal {
            Layout::SplitH
        } else {
            Layout::SplitV
        };
    };
}

pub unsafe fn get_assigned_output(
    name: *const ::core::ffi::c_char,
    parsed_num: ::core::ffi::c_long,
) -> *mut Con {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    assignment = ws_assignments.tqh_first;
    while !assignment.is_null() {
        if !(*assignment).output.is_null() {
            if !name.is_null() && strcmp((*assignment).name, name) == 0 as ::core::ffi::c_int {
                debuglog(
                    c"%s:%s:%d - Found workspace name=\"%s\" assignment to output \"%s\"\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/workspace.c".as_ptr(),
                    c"get_assigned_output".as_ptr(),
                    94 as ::core::ffi::c_int,
                    name,
                    (*assignment).output,
                );
                let assigned_by_name: *mut Output = get_output_by_name((*assignment).output, true);
                if !assigned_by_name.is_null() {
                    return (*assigned_by_name).con;
                }
            } else if output.is_null()
                && parsed_num != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                && name_is_digits((*assignment).name) as ::core::ffi::c_int != 0
                && ws_name_to_number((*assignment).name) as ::core::ffi::c_long == parsed_num
            {
                debuglog(
                    c"%s:%s:%d - Found workspace number=%ld assignment to output \"%s\"\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/workspace.c".as_ptr(),
                    c"get_assigned_output".as_ptr(),
                    105 as ::core::ffi::c_int,
                    parsed_num,
                    (*assignment).output,
                );
                let assigned_by_num: *mut Output = get_output_by_name((*assignment).output, true);
                if !assigned_by_num.is_null() {
                    output = (*assigned_by_num).con;
                }
            }
        }
        assignment = (*assignment).ws_assignments.tqe_next;
    }
    output
}

pub unsafe fn output_triggers_assignment(
    output: *mut Output,
    assignment: *mut Workspace_Assignment,
) -> bool {
    let assigned: *mut Con = get_assigned_output(
        (*assignment).name,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    !assigned.is_null() && assigned == (*output).con
}

pub unsafe fn workspace_get(num: *const ::core::ffi::c_char) -> *mut Con {
    let mut workspace: *mut Con = get_existing_workspace_by_name(num);
    if !workspace.is_null() {
        return workspace;
    }
    verboselog(c"Creating new workspace \"%s\"\n".as_ptr().cast_mut(), num);
    let parsed_num: ::core::ffi::c_int = ws_name_to_number(num) as ::core::ffi::c_int;
    let mut output: *mut Con = get_assigned_output(num, parsed_num as ::core::ffi::c_long);
    if output.is_null() {
        output = con_get_output(focused);
    }
    workspace = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(&raw mut name, c"[i3 con] workspace %s".as_ptr(), num);
    x_set_name(workspace, name);
    free(name as *mut ::core::ffi::c_void);
    free((*workspace).name as *mut ::core::ffi::c_void);
    (*workspace).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*workspace).name = sstrdup(num);
    (*workspace).workspace_layout = config.default_layout;
    (*workspace).num = parsed_num;
    (*workspace).type_0 = CT_WORKSPACE;
    (*workspace).gaps = gaps_for_workspace(workspace);
    con_attach(workspace, output_get_content(output), false);
    _workspace_apply_default_orientation(workspace);
    ipc_send_workspace_event(c"init".as_ptr(), workspace, ::core::ptr::null_mut::<Con>());
    ewmh_update_desktop_properties();
    workspace
}

fn command_prefix(input: &[u8], prefix: &[u8]) -> bool {
    input.len() >= prefix.len()
        && unsafe {
            // Native locale case matching, bounded by both borrowed byte buffers.
            libc::strncasecmp(input.as_ptr().cast(), prefix.as_ptr().cast(), prefix.len()) == 0
        }
}
fn workspace_binding_name(command: &[u8]) -> Option<std::ffi::CString> {
    if command.len() < b"workspace ".len() || !command_prefix(command, b"workspace") {
        return None;
    }
    let trim = |bytes: &[u8]| {
        bytes
            .iter()
            .position(|&byte| byte != b' ' && byte != b'\t')
            .unwrap_or(bytes.len())
    };
    let mut target = &command[b"workspace ".len()..];
    target = &target[trim(target)..];
    if [
        b"next".as_slice(),
        b"prev",
        b"next_on_output",
        b"prev_on_output",
        b"back_and_forth",
        b"current",
    ]
    .iter()
    .any(|prefix| command_prefix(target, prefix))
    {
        return None;
    }
    for prefix in [b"--no-auto-back-and-forth".as_slice(), b"number"] {
        if command_prefix(target, prefix) {
            target = &target[prefix.len()..];
            target = &target[trim(target)..];
        }
    }
    let name =
        crate::parser_util::lex_string(target, false, crate::parser_util::StringGrammar::Command)
            .bytes?;
    Some(std::ffi::CString::new(name).expect("command lexer projects first NUL"))
}

/// Bindings and command strings are borrowed from the live event-thread config.
/// Build owned names before publishing, retaining declaration order/duplicates.
pub unsafe fn extract_workspace_names_from_bindings() {
    let mut names = Vec::new();
    let mut binding = (*bindings).tqh_first;
    while !binding.is_null() {
        let command = std::ffi::CStr::from_ptr((*binding).command);
        if let Some(name) = workspace_binding_name(command.to_bytes()) {
            if name.as_bytes().starts_with(b"__") {
                verboselog(
                    c"Cannot create workspace \"%s\". Names starting with __ are i3-internal.\n"
                        .as_ptr()
                        .cast_mut(),
                    name.as_ptr(),
                );
            } else {
                names.push(name);
            }
        }
        binding = (*binding).bindings.tqe_next;
    }
    *BINDING_WORKSPACE_NAMES.lock().unwrap() = names;
}

pub unsafe fn create_workspace_on_output(output: *mut Output, content: *mut Con) -> *mut Con {
    let mut exists: bool = true;
    let ws: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*ws).type_0 = CT_WORKSPACE;
    // Release the cache lock before querying/creating graph state or callbacks.
    let binding_names = BINDING_WORKSPACE_NAMES.lock().unwrap().clone();
    for name in &binding_names {
        let target_name = name.as_ptr();
        let assigned: *mut Con = get_assigned_output(
            target_name,
            -(1 as ::core::ffi::c_int) as ::core::ffi::c_long,
        );
        if !(!assigned.is_null() && assigned != (*output).con) {
            let num: ::core::ffi::c_int = ws_name_to_number(target_name) as ::core::ffi::c_int;
            exists = !if num == -(1 as ::core::ffi::c_int) {
                get_existing_workspace_by_name(target_name)
            } else {
                get_existing_workspace_by_num(num)
            }
            .is_null();
            if !exists {
                (*ws).name = sstrdup(target_name);
                (*ws).num = num;
                debuglog(
                    c"%s:%s:%d - Used number %d for workspace with name %s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/workspace.c".as_ptr(),
                    c"create_workspace_on_output".as_ptr(),
                    276 as ::core::ffi::c_int,
                    (*ws).num,
                    (*ws).name,
                );
                break;
            }
        }
    }
    if exists {
        debuglog(
            c"%s:%s:%d - Getting next unused workspace by number\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"create_workspace_on_output".as_ptr(),
            284 as ::core::ffi::c_int,
        );
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while exists {
            c += 1;
            let assigned_0: *mut Con = get_assigned_output(
                ::core::ptr::null::<::core::ffi::c_char>(),
                c as ::core::ffi::c_long,
            );
            exists = !get_existing_workspace_by_num(c).is_null()
                || !assigned_0.is_null() && assigned_0 != (*output).con;
            debuglog(
                c"%s:%s:%d - result for ws %d: exists = %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/workspace.c".as_ptr(),
                c"create_workspace_on_output".as_ptr(),
                290 as ::core::ffi::c_int,
                c,
                exists as ::core::ffi::c_int,
            );
        }
        (*ws).num = c;
        sasprintf(&raw mut (*ws).name, c"%d".as_ptr(), c);
    }
    con_attach(ws, content, false);
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(&raw mut name, c"[i3 con] workspace %s".as_ptr(), (*ws).name);
    x_set_name(ws, name);
    free(name as *mut ::core::ffi::c_void);
    (*ws).gaps = gaps_for_workspace(ws);
    (*ws).fullscreen_mode = CF_OUTPUT;
    (*ws).workspace_layout = config.default_layout;
    _workspace_apply_default_orientation(ws);
    ipc_send_workspace_event(c"init".as_ptr(), ws, ::core::ptr::null_mut::<Con>());
    ws
}

pub unsafe fn workspace_is_visible(ws: *mut Con) -> bool {
    let output: *mut Con = con_get_output(ws);
    if output.is_null() {
        return false;
    }
    let fs: *mut Con = con_get_fullscreen_con(output, CF_OUTPUT);
    fs == ws
}
unsafe fn _get_sticky(
    con: *mut Con,
    sticky_group: *const ::core::ffi::c_char,
    exclude: *mut Con,
) -> *mut Con {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if current != exclude
            && !(*current).sticky_group.is_null()
            && !(*current).window.is_null()
            && strcmp((*current).sticky_group, sticky_group) == 0 as ::core::ffi::c_int
        {
            return current;
        }
        let recurse: *mut Con = _get_sticky(current, sticky_group, exclude);
        if !recurse.is_null() {
            return recurse;
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        if current != exclude
            && !(*current).sticky_group.is_null()
            && !(*current).window.is_null()
            && strcmp((*current).sticky_group, sticky_group) == 0 as ::core::ffi::c_int
        {
            return current;
        }
        let recurse_0: *mut Con = _get_sticky(current, sticky_group, exclude);
        if !recurse_0.is_null() {
            return recurse_0;
        }
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}
unsafe fn workspace_reassign_sticky(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if (*current).sticky_group.is_null() {
            workspace_reassign_sticky(current);
        } else {
            verboselog(
                c"Ah, this one is sticky: %s / %p\n".as_ptr().cast_mut(),
                (*current).name,
                current,
            );
            let output: *mut Con = con_get_output(current);
            let src: *mut Con = _get_sticky(output, (*current).sticky_group, current);
            if src.is_null() {
                verboselog(
                    c"No window found for this sticky group\n"
                        .as_ptr()
                        .cast_mut(),
                );
                workspace_reassign_sticky(current);
            } else {
                x_move_win(src, current);
                (*current).window = (*src).window;
                (*current).mapped = true;
                (*src).window = ::core::ptr::null_mut::<Window>();
                (*src).mapped = false;
                x_reparent_child(current, src);
                verboselog(
                    c"re-assigned window from src %p to dest %p\n"
                        .as_ptr()
                        .cast_mut(),
                    src,
                    current,
                );
            }
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        workspace_reassign_sticky(current);
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
}
unsafe extern "C" fn workspace_defer_update_urgent_hint_cb(
    loop_0: *mut ev_loop,
    w: *mut ev_timer,
    revents: ::core::ffi::c_int,
) {
    let con: *mut Con = (*w).data as *mut Con;
    ev_timer_stop(main_loop, (*con).urgency_timer as *mut ev_timer);
    free((*con).urgency_timer as *mut ::core::ffi::c_void);
    (*con).urgency_timer = ::core::ptr::null_mut::<ev_timer>();
    if (*con).urgent {
        debuglog(
            c"%s:%s:%d - Resetting urgency flag of con %p by timer\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_defer_update_urgent_hint_cb".as_ptr(),
            425 as ::core::ffi::c_int,
            con,
        );
        con_set_urgency(con, false);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
        ipc_send_window_event(c"urgent".as_ptr(), con);
        tree_render();
    }
}

pub unsafe fn workspace_show(workspace: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut old: *mut Con = ::core::ptr::null_mut::<Con>();
    if con_is_internal(workspace) {
        return;
    }
    current = (*(*workspace).parent).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if (*current).fullscreen_mode as ::core::ffi::c_uint
            == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            old = current;
        }
        (*current).fullscreen_mode = CF_NONE;
        current = (*current).nodes.tqe_next as *mut Con;
    }
    (*workspace).fullscreen_mode = CF_OUTPUT;
    current = con_get_workspace(focused);
    if workspace == current {
        debuglog(
            c"%s:%s:%d - Not switching, already there.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_show".as_ptr(),
            460 as ::core::ffi::c_int,
        );
        return;
    }
    let mut old_focus: *mut Con = if !old.is_null() {
        con_descend_focused(old)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    if !current.is_null() && !con_is_internal(current) {
        free(previous_workspace_name as *mut ::core::ffi::c_void);
        previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        previous_workspace_name = sstrdup((*current).name);
        debuglog(
            c"%s:%s:%d - Setting previous_workspace_name = %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_show".as_ptr(),
            480 as ::core::ffi::c_int,
            previous_workspace_name,
        );
    }
    workspace_reassign_sticky(workspace);
    debuglog(
        c"%s:%s:%d - switching to %p / %s\n".as_ptr().cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_show".as_ptr(),
        485 as ::core::ffi::c_int,
        workspace,
        (*workspace).name,
    );
    let next: *mut Con = con_descend_focused(workspace);
    let old_output: *mut Con = con_get_output(focused);
    if (*next).urgent as ::core::ffi::c_int != 0
        && (config.workspace_urgency_timer * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
            as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
    {
        (*next).urgent = false;
        con_focus(next);
        (*focused).urgent = true;
        (*workspace).urgent = true;
        if (*focused).urgency_timer.is_null() {
            debuglog(
                c"%s:%s:%d - Deferring reset of urgency flag of con %p on newly shown workspace %p\n".as_ptr().cast_mut(),
                c"../src/workspace.c".as_ptr(),
                c"workspace_show".as_ptr(),
                506 as ::core::ffi::c_int,
                focused,
                workspace,
            );
            (*focused).urgency_timer =
                scalloc(1 as usize, ::core::mem::size_of::<ev_timer>() as usize) as *mut ev_timer;
            let ref mut fresh2 = (*((*focused).urgency_timer as *mut ::core::ffi::c_void
                as *mut ev_watcher))
                .pending;
            *fresh2 = 0 as ::core::ffi::c_int;
            (*((*focused).urgency_timer as *mut ::core::ffi::c_void as *mut ev_watcher)).active =
                *fresh2;
            (*((*focused).urgency_timer as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
                0 as ::core::ffi::c_int;
            (*(*focused).urgency_timer).cb = Some(
                workspace_defer_update_urgent_hint_cb
                    as unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> (),
            )
                as Option<
                    unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> (),
                >;
            memmove(
                &raw mut (*((*focused).urgency_timer as *mut ev_watcher)).cb
                    as *mut ::core::ffi::c_void,
                &raw mut (*(*focused).urgency_timer).cb as *const ::core::ffi::c_void,
                ::core::mem::size_of::<
                    Option<
                        unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> (),
                    >,
                >() as usize,
            );
            (*((*focused).urgency_timer as *mut ev_watcher_time)).at =
                config.workspace_urgency_timer as ev_tstamp;
            (*(*focused).urgency_timer).repeat = config.workspace_urgency_timer as ev_tstamp;
            (*(*focused).urgency_timer).data = focused as *mut ::core::ffi::c_void;
            ev_timer_start(main_loop, (*focused).urgency_timer as *mut ev_timer);
        } else {
            debuglog(
                c"%s:%s:%d - Resetting urgency timer of con %p on workspace %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/workspace.c".as_ptr(),
                c"workspace_show".as_ptr(),
                515 as ::core::ffi::c_int,
                focused,
                workspace,
            );
            ev_timer_again(main_loop, (*focused).urgency_timer as *mut ev_timer);
        }
    } else {
        con_focus(next);
    }
    ipc_send_workspace_event(c"focus".as_ptr(), workspace, current);
    debuglog(
        c"%s:%s:%d - old = %p / %s\n".as_ptr().cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_show".as_ptr(),
        524 as ::core::ffi::c_int,
        old,
        if !old.is_null() {
            (*old).name as *const ::core::ffi::c_char
        } else {
            c"(null)".as_ptr()
        },
    );
    if !old.is_null()
        && (*old).nodes_head.tqh_first.is_null()
        && (*old).floating_head.tqh_first.is_null()
    {
        if !workspace_is_visible(old) {
            verboselog(
                c"Closing old workspace (%p / %s), it is empty\n"
                    .as_ptr()
                    .cast_mut(),
                old,
                (*old).name,
            );
            let payload =
                ipc_marshal_workspace_event(c"empty".as_ptr(), old, ::core::ptr::null_mut::<Con>());
            tree_close_internal(old, KillWindow::DontKill, false);
            ipc_send_event(
                c"workspace".as_ptr(),
                I3_IPC_EVENT_WORKSPACE as u32,
                payload.as_ptr(),
            );
            if old == old_focus {
                old_focus = ::core::ptr::null_mut::<Con>();
            }
            ewmh_update_desktop_properties();
        }
    }
    (*workspace).fullscreen_mode = CF_OUTPUT;
    verboselog(
        c"focused now = %p / %s\n".as_ptr().cast_mut(),
        focused,
        (*focused).name,
    );
    let new_output: *mut Con = con_get_output(focused);
    if old_output != new_output {
        x_set_warp_to(&raw mut (*next).rect);
    }
    ewmh_update_current_desktop();
    output_push_sticky_windows(old_focus);
}

pub unsafe fn workspace_show_by_name(num: *const ::core::ffi::c_char) {
    workspace_show(workspace_get(num));
}

pub unsafe fn workspace_next() -> *mut Con {
    let current: *mut Con = con_get_workspace(focused);
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first_opposite: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*current).num == -(1 as ::core::ffi::c_int) {
        next = (*current).nodes.tqe_next as *mut Con;
        if !next.is_null() {
            return next;
        }
        let mut found_current: bool = false;
        output = (*croot).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            if !con_is_internal(output) {
                let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                while child == -(1 as ::core::ffi::c_int) as *mut Con && {
                    child = ::core::ptr::null_mut::<Con>();
                    true
                } {
                    child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
                    while !child.is_null() {
                        if !((*child).type_0 as ::core::ffi::c_uint
                            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            if first.is_null() {
                                first = child;
                            }
                            if first_opposite.is_null()
                                || (*child).num != -(1 as ::core::ffi::c_int)
                                    && (*child).num < (*first_opposite).num
                            {
                                first_opposite = child;
                            }
                            if child == current {
                                found_current = true;
                            } else if (*child).num == -(1 as ::core::ffi::c_int)
                                && found_current as ::core::ffi::c_int != 0
                            {
                                next = child;
                                return next;
                            }
                        }
                        child = (*child).nodes.tqe_next as *mut Con;
                    }
                }
            }
            output = (*output).nodes.tqe_next as *mut Con;
        }
    } else {
        let mut found_current_0: bool = false;
        output = (*croot).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            if !con_is_internal(output) {
                let mut child_0: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                while child_0 == -(1 as ::core::ffi::c_int) as *mut Con && {
                    child_0 = ::core::ptr::null_mut::<Con>();
                    true
                } {
                    child_0 = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
                    while !child_0.is_null() {
                        if !((*child_0).type_0 as ::core::ffi::c_uint
                            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            if first.is_null()
                                || (*child_0).num != -(1 as ::core::ffi::c_int)
                                    && (*child_0).num < (*first).num
                            {
                                first = child_0;
                            }
                            if first_opposite.is_null()
                                && (*child_0).num == -(1 as ::core::ffi::c_int)
                            {
                                first_opposite = child_0;
                            }
                            if (*child_0).num == -(1 as ::core::ffi::c_int) {
                                break;
                            }
                            if (*current).num < (*child_0).num
                                && (next.is_null() || (*child_0).num < (*next).num)
                            {
                                next = child_0;
                            }
                            if child_0 == current {
                                found_current_0 = true;
                            } else if found_current_0 as ::core::ffi::c_int != 0
                                && (*current).num == (*child_0).num
                            {
                                return child_0;
                            }
                        }
                        child_0 = (*child_0).nodes.tqe_next as *mut Con;
                    }
                }
            }
            output = (*output).nodes.tqe_next as *mut Con;
        }
    }
    if next.is_null() {
        next = if !first_opposite.is_null() {
            first_opposite
        } else {
            first
        };
    }
    next
}

pub unsafe fn workspace_prev() -> *mut Con {
    let current: *mut Con = con_get_workspace(focused);
    let mut prev: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first_opposite: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut last: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*current).num == -(1 as ::core::ffi::c_int) {
        prev = *(*((*current).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
        if !prev.is_null() && (*prev).num != -(1 as ::core::ffi::c_int) {
            prev = ::core::ptr::null_mut::<Con>();
        }
        if prev.is_null() {
            let mut found_current: bool = false;
            output = *(*((*croot).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
            while !output.is_null() {
                if !con_is_internal(output) {
                    let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                    while child == -(1 as ::core::ffi::c_int) as *mut Con && {
                        child = ::core::ptr::null_mut::<Con>();
                        true
                    } {
                        child = *(*((*output_get_content(output)).nodes_head.tqh_last
                            as *mut nodes_head))
                            .tqh_last as *mut Con;
                        while !child.is_null() {
                            if !((*child).type_0 as ::core::ffi::c_uint
                                != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                            {
                                if last.is_null() {
                                    last = child;
                                }
                                if first_opposite.is_null()
                                    || (*child).num != -(1 as ::core::ffi::c_int)
                                        && (*child).num > (*first_opposite).num
                                {
                                    first_opposite = child;
                                }
                                if child == current {
                                    found_current = true;
                                } else if (*child).num == -(1 as ::core::ffi::c_int)
                                    && found_current as ::core::ffi::c_int != 0
                                {
                                    prev = child;
                                    return prev;
                                }
                            }
                            child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last
                                as *mut Con;
                        }
                    }
                }
                output = *(*((*output).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            }
        }
    } else {
        let mut found_current_0: bool = false;
        output = *(*((*croot).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !output.is_null() {
            if !con_is_internal(output) {
                let mut child_0: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                while child_0 == -(1 as ::core::ffi::c_int) as *mut Con && {
                    child_0 = ::core::ptr::null_mut::<Con>();
                    true
                } {
                    child_0 = *(*((*output_get_content(output)).nodes_head.tqh_last
                        as *mut nodes_head))
                        .tqh_last as *mut Con;
                    while !child_0.is_null() {
                        if !((*child_0).type_0 as ::core::ffi::c_uint
                            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            if last.is_null()
                                || (*child_0).num != -(1 as ::core::ffi::c_int)
                                    && (*last).num < (*child_0).num
                            {
                                last = child_0;
                            }
                            if first_opposite.is_null()
                                && (*child_0).num == -(1 as ::core::ffi::c_int)
                            {
                                first_opposite = child_0;
                            }
                            if !((*child_0).num == -(1 as ::core::ffi::c_int)) {
                                if (*current).num > (*child_0).num
                                    && (prev.is_null() || (*child_0).num > (*prev).num)
                                {
                                    prev = child_0;
                                }
                                if child_0 == current {
                                    found_current_0 = true;
                                } else if found_current_0 as ::core::ffi::c_int != 0
                                    && (*current).num == (*child_0).num
                                {
                                    return child_0;
                                }
                            }
                        }
                        child_0 =
                            *(*((*child_0).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                    }
                }
            }
            output = *(*((*output).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
        }
    }
    if prev.is_null() {
        prev = if !first_opposite.is_null() {
            first_opposite
        } else {
            last
        };
    }
    prev
}

pub unsafe fn workspace_next_on_output() -> *mut Con {
    let current_block: u64;
    let current: *mut Con = con_get_workspace(focused);
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let output: *mut Con = con_get_output(focused);
    if (*current).num == -(1 as ::core::ffi::c_int) {
        next = (*current).nodes.tqe_next as *mut Con;
    } else {
        let mut found_current: bool = false;
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !((*child).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if (*child).num == -(1 as ::core::ffi::c_int) {
                        break;
                    }
                    if (*current).num < (*child).num
                        && (next.is_null() || (*child).num < (*next).num)
                    {
                        next = child;
                    }
                    if child == current {
                        found_current = true;
                    } else if found_current as ::core::ffi::c_int != 0
                        && (*current).num == (*child).num
                    {
                        return child;
                    }
                }
                child = (*child).nodes.tqe_next as *mut Con;
            }
        }
    }
    if next.is_null() {
        let mut found_current_0: bool = false;
        let mut child_0: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        's_105: loop {
            if !(child_0 == -(1 as ::core::ffi::c_int) as *mut Con && {
                child_0 = ::core::ptr::null_mut::<Con>();
                true
            }) {
                current_block = 17281240262373992796;
                break;
            }
            child_0 = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !child_0.is_null() {
                if !((*child_0).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if child_0 == current {
                        found_current_0 = true;
                    } else if (*child_0).num == -(1 as ::core::ffi::c_int)
                        && ((*current).num != -(1 as ::core::ffi::c_int)
                            || found_current_0 as ::core::ffi::c_int != 0)
                    {
                        next = child_0;
                        current_block = 13918395486153594028;
                        break 's_105;
                    }
                }
                child_0 = (*child_0).nodes.tqe_next as *mut Con;
            }
        }
    } else {
        current_block = 17281240262373992796;
    }
    match current_block {
        17281240262373992796 => {
            if next.is_null() {
                let mut child_1: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                while child_1 == -(1 as ::core::ffi::c_int) as *mut Con && {
                    child_1 = ::core::ptr::null_mut::<Con>();
                    true
                } {
                    child_1 = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
                    while !child_1.is_null() {
                        if !((*child_1).type_0 as ::core::ffi::c_uint
                            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            if next.is_null()
                                || (*child_1).num != -(1 as ::core::ffi::c_int)
                                    && (*child_1).num < (*next).num
                            {
                                next = child_1;
                            }
                        }
                        child_1 = (*child_1).nodes.tqe_next as *mut Con;
                    }
                }
            }
        }
        _ => {}
    }
    next
}

pub unsafe fn workspace_prev_on_output() -> *mut Con {
    let current_block: u64;
    let current: *mut Con = con_get_workspace(focused);
    let mut prev: *mut Con = ::core::ptr::null_mut::<Con>();
    let output: *mut Con = con_get_output(focused);
    debuglog(
        c"%s:%s:%d - output = %s\n".as_ptr().cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_prev_on_output".as_ptr(),
        826 as ::core::ffi::c_int,
        (*output).name,
    );
    if (*current).num == -(1 as ::core::ffi::c_int) {
        prev = *(*((*current).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
        if !prev.is_null() && (*prev).num != -(1 as ::core::ffi::c_int) {
            prev = ::core::ptr::null_mut::<Con>();
        }
    } else {
        let mut found_current: bool = false;
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true
        } {
            child = *(*((*output_get_content(output)).nodes_head.tqh_last as *mut nodes_head))
                .tqh_last as *mut Con;
            while !child.is_null() {
                if !((*child).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*child).num == -(1 as ::core::ffi::c_int))
                {
                    if (*current).num > (*child).num
                        && (prev.is_null() || (*child).num > (*prev).num)
                    {
                        prev = child;
                    }
                    if child == current {
                        found_current = true;
                    } else if found_current as ::core::ffi::c_int != 0
                        && (*current).num == (*child).num
                    {
                        return child;
                    }
                }
                child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            }
        }
    }
    if prev.is_null() {
        let mut found_current_0: bool = false;
        let mut child_0: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        's_113: loop {
            if !(child_0 == -(1 as ::core::ffi::c_int) as *mut Con && {
                child_0 = ::core::ptr::null_mut::<Con>();
                true
            }) {
                current_block = 11913429853522160501;
                break;
            }
            child_0 = *(*((*output_get_content(output)).nodes_head.tqh_last as *mut nodes_head))
                .tqh_last as *mut Con;
            while !child_0.is_null() {
                if !((*child_0).type_0 as ::core::ffi::c_uint
                    != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if child_0 == current {
                        found_current_0 = true;
                    } else if (*child_0).num == -(1 as ::core::ffi::c_int)
                        && ((*current).num != -(1 as ::core::ffi::c_int)
                            || found_current_0 as ::core::ffi::c_int != 0)
                    {
                        prev = child_0;
                        current_block = 12235783886841305969;
                        break 's_113;
                    }
                }
                child_0 = *(*((*child_0).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            }
        }
    } else {
        current_block = 11913429853522160501;
    }
    match current_block {
        11913429853522160501 => {
            if prev.is_null() {
                let mut child_1: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
                while child_1 == -(1 as ::core::ffi::c_int) as *mut Con && {
                    child_1 = ::core::ptr::null_mut::<Con>();
                    true
                } {
                    child_1 = *(*((*output_get_content(output)).nodes_head.tqh_last
                        as *mut nodes_head))
                        .tqh_last as *mut Con;
                    while !child_1.is_null() {
                        if !((*child_1).type_0 as ::core::ffi::c_uint
                            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            if prev.is_null() || (*child_1).num > (*prev).num {
                                prev = child_1;
                            }
                        }
                        child_1 =
                            *(*((*child_1).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                    }
                }
            }
        }
        _ => {}
    }
    prev
}

pub unsafe fn workspace_back_and_forth() {
    if previous_workspace_name.is_null() {
        debuglog(
            c"%s:%s:%d - No previous workspace name set. Not switching.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_back_and_forth".as_ptr(),
            896 as ::core::ffi::c_int,
        );
        return;
    }
    workspace_show_by_name(previous_workspace_name);
}

pub unsafe fn workspace_back_and_forth_get() -> *mut Con {
    if previous_workspace_name.is_null() {
        debuglog(
            c"%s:%s:%d - No previous workspace name set.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_back_and_forth_get".as_ptr(),
            909 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<Con>();
    }
    workspace_get(previous_workspace_name)
}
unsafe fn get_urgency_flag(con: *mut Con) -> bool {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).urgent as ::core::ffi::c_int != 0
            || get_urgency_flag(child) as ::core::ffi::c_int != 0
        {
            return true;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).urgent as ::core::ffi::c_int != 0
            || get_urgency_flag(child) as ::core::ffi::c_int != 0
        {
            return true;
        }
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    false
}

pub unsafe fn workspace_update_urgent_flag(ws: *mut Con) {
    let old_flag: bool = (*ws).urgent;
    (*ws).urgent = get_urgency_flag(ws);
    debuglog(
        c"%s:%s:%d - Workspace urgency flag changed from %d to %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_update_urgent_flag".as_ptr(),
        941 as ::core::ffi::c_int,
        old_flag as ::core::ffi::c_int,
        (*ws).urgent as ::core::ffi::c_int,
    );
    if old_flag as ::core::ffi::c_int != (*ws).urgent as ::core::ffi::c_int {
        ipc_send_workspace_event(c"urgent".as_ptr(), ws, ::core::ptr::null_mut::<Con>());
    }
}

pub unsafe fn ws_force_orientation(ws: *mut Con, orientation: Orientation) {
    let split: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*split).parent = ws as *mut Con;
    (*split).layout = (*ws).layout;
    let focus_order = get_focus_order(ws);
    debuglog(
        c"%s:%s:%d - Moving cons\n".as_ptr().cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"ws_force_orientation".as_ptr(),
        964 as ::core::ffi::c_int,
    );
    while !(*ws).nodes_head.tqh_first.is_null() {
        let child: *mut Con = (*ws).nodes_head.tqh_first as *mut Con;
        con_detach(child);
        con_attach(child, split, true);
    }
    set_focus_order(split, &focus_order);
    (*ws).layout = if orientation == Orientation::Horizontal {
        Layout::SplitH
    } else {
        Layout::SplitV
    };
    debuglog(
        c"%s:%s:%d - split->layout = %d, ws->layout = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"ws_force_orientation".as_ptr(),
        976 as ::core::ffi::c_int,
        (*split).layout as ::core::ffi::c_uint,
        (*ws).layout as ::core::ffi::c_uint,
    );
    debuglog(
        c"%s:%s:%d - Attaching new split (%p) to ws (%p)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"ws_force_orientation".as_ptr(),
        979 as ::core::ffi::c_int,
        split,
        ws,
    );
    con_attach(split, ws, false);
    con_fix_percent(ws);
}

pub unsafe fn workspace_attach_to(ws: *mut Con) -> *mut Con {
    debuglog(
        c"%s:%s:%d - Attaching a window to workspace %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_attach_to".as_ptr(),
        997 as ::core::ffi::c_int,
        ws,
        (*ws).name,
    );
    if (*ws).workspace_layout == Layout::Default {
        debuglog(
            c"%s:%s:%d - Default layout, just attaching it to the workspace itself.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_attach_to".as_ptr(),
            1000 as ::core::ffi::c_int,
        );
        return ws;
    }
    debuglog(
        c"%s:%s:%d - Non-default layout, creating a new split container\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_attach_to".as_ptr(),
        1004 as ::core::ffi::c_int,
    );
    let new: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*new).parent = ws as *mut Con;
    (*new).layout = (*ws).workspace_layout;
    debuglog(
        c"%s:%s:%d - Attaching new split %p to workspace %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_attach_to".as_ptr(),
        1013 as ::core::ffi::c_int,
        new,
        ws,
    );
    con_attach(new, ws, false);
    con_fix_percent(ws);
    new
}

pub unsafe fn workspace_encapsulate(ws: *mut Con) -> *mut Con {
    if (*ws).nodes_head.tqh_first.is_null() {
        errorlog(
            c"ERROR: Workspace %p / %s has no children to encapsulate\n"
                .as_ptr()
                .cast_mut(),
            ws,
            (*ws).name,
        );
        return ::core::ptr::null_mut::<Con>();
    }
    let new: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*new).parent = ws as *mut Con;
    (*new).layout = (*ws).layout;
    let focus_order = get_focus_order(ws);
    debuglog(
        c"%s:%s:%d - Moving children of workspace %p / %s into container %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_encapsulate".as_ptr(),
        1041 as ::core::ffi::c_int,
        ws,
        (*ws).name,
        new,
    );
    while !(*ws).nodes_head.tqh_first.is_null() {
        let child: *mut Con = (*ws).nodes_head.tqh_first as *mut Con;
        con_detach(child);
        con_attach(child, new, true);
    }
    set_focus_order(new, &focus_order);
    con_attach(new, ws, true);
    new
}

pub unsafe fn workspace_move_to_output(mut ws: *mut Con, output: *mut Output) {
    debuglog(
        c"%s:%s:%d - Moving workspace %p / %s to output %p / \"%s\".\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_move_to_output".as_ptr(),
        1060 as ::core::ffi::c_int,
        ws,
        (*ws).name,
        output,
        output_primary_name(output),
    );
    let current_output: *mut Output = get_output_for_con(ws);
    let content: *mut Con = output_get_content((*output).con);
    debuglog(
        c"%s:%s:%d - got output %p with content %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_move_to_output".as_ptr(),
        1064 as ::core::ffi::c_int,
        output,
        content,
    );
    if (*ws).parent == content {
        debuglog(
            c"%s:%s:%d - Nothing to do, workspace already there\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_move_to_output".as_ptr(),
            1067 as ::core::ffi::c_int,
        );
        return;
    }
    let previously_visible_ws: *mut Con = (*content).focus_head.tqh_first as *mut Con;
    if !previously_visible_ws.is_null() {
        debuglog(
            c"%s:%s:%d - Previously visible workspace = %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_move_to_output".as_ptr(),
            1073 as ::core::ffi::c_int,
            previously_visible_ws,
            (*previously_visible_ws).name,
        );
    } else {
        debuglog(
            c"%s:%s:%d - No previously visible workspace on output.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_move_to_output".as_ptr(),
            1075 as ::core::ffi::c_int,
        );
    }
    let workspace_was_visible: bool = workspace_is_visible(ws);
    if con_num_children((*ws).parent as *mut Con) == 1 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - Creating a new workspace to replace \"%s\" (last on its output).\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_move_to_output".as_ptr(),
            1080 as ::core::ffi::c_int,
            (*ws).name,
        );
        let mut used_assignment: bool = false;
        let mut assignment: *mut Workspace_Assignment =
            ::core::ptr::null_mut::<Workspace_Assignment>();
        assignment = ws_assignments.tqh_first;
        while !assignment.is_null() {
            if output_triggers_assignment(current_output, assignment) {
                let num: ::core::ffi::c_int =
                    ws_name_to_number((*assignment).name) as ::core::ffi::c_int;
                let attached: bool = !if num == -(1 as ::core::ffi::c_int) {
                    get_existing_workspace_by_name((*assignment).name)
                } else {
                    get_existing_workspace_by_num(num)
                }
                .is_null();
                if !attached {
                    debuglog(
                        c"%s:%s:%d - Creating workspace from assignment %s.\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/workspace.c".as_ptr(),
                        c"workspace_move_to_output".as_ptr(),
                        1099 as ::core::ffi::c_int,
                        (*assignment).name,
                    );
                    workspace_get((*assignment).name);
                    used_assignment = true;
                    break;
                }
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
        if !used_assignment {
            create_workspace_on_output(current_output, (*ws).parent as *mut Con);
        }
    }
    debuglog(
        c"%s:%s:%d - Detaching\n".as_ptr().cast_mut(),
        c"../src/workspace.c".as_ptr(),
        c"workspace_move_to_output".as_ptr(),
        1112 as ::core::ffi::c_int,
    );
    let old_content: *mut Con = (*ws).parent as *mut Con;
    con_detach(ws);
    if workspace_was_visible {
        let focus_ws: *mut Con = (*old_content).focus_head.tqh_first as *mut Con;
        debuglog(
            c"%s:%s:%d - workspace was visible, focusing %p / %s now\n"
                .as_ptr()
                .cast_mut(),
            c"../src/workspace.c".as_ptr(),
            c"workspace_move_to_output".as_ptr(),
            1121 as ::core::ffi::c_int,
            focus_ws,
            (*focus_ws).name,
        );
        workspace_show(focus_ws);
    }
    con_attach(ws, content, false);
    let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
    floating_con = (*ws).floating_head.tqh_first as *mut Con;
    while !floating_con.is_null() {
        floating_fix_coordinates(
            floating_con,
            &raw mut (*old_content).rect,
            &raw mut (*content).rect,
        );
        floating_con = (*floating_con).floating_windows.tqe_next as *mut Con;
    }
    ipc_send_workspace_event(c"move".as_ptr(), ws, ::core::ptr::null_mut::<Con>());
    if workspace_was_visible {
        workspace_show(ws);
    }
    ewmh_update_desktop_properties();
    if previously_visible_ws.is_null() {
        return;
    }
    ws = (*content).nodes_head.tqh_first as *mut Con;
    while !ws.is_null() {
        if ws != previously_visible_ws {
            ws = (*ws).nodes.tqe_next as *mut Con;
        } else {
            (*previously_visible_ws)
                .on_remove_child
                .expect("non-null function pointer")(previously_visible_ws);
            break;
        }
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_current_desktop;
use crate::ewmh::ewmh_update_desktop_properties;
use crate::ipc::ipc_send_event;
use crate::tree::tree_render;
use crate::util::name_is_digits;
use crate::util::ws_name_to_number;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{free, memmove, strcasecmp, strcmp, strncasecmp};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::con::con_attach;
use crate::con::con_descend_focused;
use crate::con::con_detach;
use crate::con::con_fix_percent;
use crate::con::con_focus;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_is_internal;
use crate::con::con_new;
use crate::con::con_num_children;
use crate::con::con_set_urgency;
use crate::con::con_update_parents_urgency;
use crate::con::get_focus_order;
use crate::con::set_focus_order;
use crate::config::config;
use crate::floating::floating_fix_coordinates;
use crate::gaps::gaps_for_workspace;
use crate::ipc::ipc_marshal_workspace_event;
use crate::ipc::ipc_send_window_event;
use crate::ipc::ipc_send_workspace_event;
use crate::output::get_output_for_con;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::output::output_push_sticky_windows;
use crate::randr::get_output_by_name;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::x::x_move_win;
use crate::x::x_reparent_child;
use crate::x::x_set_name;
use crate::x::x_set_warp_to;

#[cfg(test)]
mod owned_binding_tests {
    use super::*;
    #[test]
    fn workspace_commands_preserve_prefix_modifiers_and_command_quoting() {
        for (command, name) in [
            (&b"workspace  4: www"[..], &b"4: www"[..]),
            (
                b"WORKSPACE\t--no-auto-back-and-forth number \"3: web\"; focus left",
                b"3: web",
            ),
            (b"workspace number7", b"7"),
            (b"workspace \"a\\\"b\"", b"a\"b"),
            (b"workspace bad\xff\0ignored", b"bad\xff"),
        ] {
            assert_eq!(workspace_binding_name(command).unwrap().as_bytes(), name);
        }
        for command in [
            b"workspace next".as_slice(),
            b"workspace prev-name",
            b"workspace current",
            b"workspace \"\"",
            b"workspace",
            b"exec workspace one",
        ] {
            assert!(workspace_binding_name(command).is_none());
        }
    }
    #[test]
    fn binding_names_remain_owned_ordered_and_duplicate_preserving() {
        let names: Vec<_> = [
            b"workspace one".as_slice(),
            b"workspace two",
            b"workspace one",
        ]
        .into_iter()
        .filter_map(workspace_binding_name)
        .collect();
        let snapshot = names.clone();
        drop(names);
        assert_eq!(
            snapshot
                .iter()
                .map(|name| name.as_bytes())
                .collect::<Vec<_>>(),
            [b"one".as_slice(), b"two", b"one"]
        );
    }
}
