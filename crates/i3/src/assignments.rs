use crate::application::assignments;
use crate::commands_parser::command_result_free;
use crate::commands_parser::parse_command;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * assignments.c: Assignments for specific windows (for_window).
 *
 */
// Mechanical file-by-file Rust translation of src/assignments.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use i3_json::compat::yajl_gen_t;

pub type i3String = _i3String;

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
pub type i3Window = Window;

pub use crate::application::ipc_client;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: *mut ::core::ffi::c_char,
    pub needs_tree_render: bool,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn run_assignments(window: *mut i3Window) {
    if window.is_null() {
        return;
    }
    debuglog(
        c"%s:%s:%d - Checking if any assignments match this window\n"
            .as_ptr()
            .cast_mut(),
        c"../src/assignments.c".as_ptr(),
        c"run_assignments".as_ptr(),
        22 as ::core::ffi::c_int,
    );
    let mut needs_tree_render: bool = false_0 != 0;
    let old_first_assignment: *const Assignment = assignments.tqh_first;
    let mut current: *mut Assignment = ::core::ptr::null_mut::<Assignment>();
    current = assignments.tqh_first as *mut Assignment;
    while !current.is_null() {
        if !((*current).type_0 as ::core::ffi::c_uint
            != A_COMMAND as ::core::ffi::c_int as ::core::ffi::c_uint
            || !match_matches_window(&raw mut (*current).match_0, window))
        {
            let mut skip: bool = false_0 != 0;
            let mut c: u32 = 0 as u32;
            while c < (*window).nr_assignments {
                if *(*window).ran_assignments.offset(c as isize) != current {
                    c = c.wrapping_add(1);
                } else {
                    debuglog(
                        c"%s:%s:%d - This assignment already ran for the given window, not executing it again.\n".as_ptr().cast_mut(),
                        c"../src/assignments.c".as_ptr(),
                        c"run_assignments".as_ptr(),
                        40 as ::core::ffi::c_int,
                    );
                    skip = true_0 != 0;
                    break;
                }
            }
            if !skip {
                (*window).nr_assignments = (*window).nr_assignments.wrapping_add(1);
                (*window).ran_assignments = srealloc(
                    (*window).ran_assignments as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<*mut Assignment>() as usize)
                        .wrapping_mul((*window).nr_assignments as usize),
                ) as *mut *mut Assignment;
                let ref mut fresh0 = *(*window)
                    .ran_assignments
                    .offset((*window).nr_assignments.wrapping_sub(1 as u32) as isize);
                *fresh0 = current;
                debuglog(
                    c"%s:%s:%d - matching assignment, execute command %s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/assignments.c".as_ptr(),
                    c"run_assignments".as_ptr(),
                    56 as ::core::ffi::c_int,
                    (*current).dest.command,
                );
                let mut full_command: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut full_command,
                    c"[id=\"%d\"] %s".as_ptr(),
                    (*window).id,
                    (*current).dest.command,
                );
                let result: *mut CommandResult = parse_command(
                    full_command,
                    ::core::ptr::null_mut::<yajl_gen_t>(),
                    ::core::ptr::null_mut::<ipc_client>(),
                );
                free(full_command as *mut ::core::ffi::c_void);
                if (*result).needs_tree_render {
                    needs_tree_render = true_0 != 0;
                }
                command_result_free(result);
                if old_first_assignment != assignments.tqh_first as *const Assignment {
                    break;
                }
            }
        }
        current = (*current).assignments.tqe_next as *mut Assignment;
    }
    if needs_tree_render {
        tree_render();
    }
}

pub unsafe fn assignment_for(window: *mut i3Window, type_0: ::core::ffi::c_int) -> *mut Assignment {
    let mut assignment: *mut Assignment = ::core::ptr::null_mut::<Assignment>();
    assignment = assignments.tqh_first as *mut Assignment;
    while !assignment.is_null() {
        if type_0 != A_ANY as ::core::ffi::c_int
            && (*assignment).type_0 as ::core::ffi::c_uint & type_0 as ::core::ffi::c_uint
                == 0 as ::core::ffi::c_uint
            || !match_matches_window(&raw mut (*assignment).match_0, window)
        {
            assignment = (*assignment).assignments.tqe_next as *mut Assignment;
        } else {
            debuglog(
                c"%s:%s:%d - got a matching assignment\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/assignments.c".as_ptr(),
                c"assignment_for".as_ptr(),
                95 as ::core::ffi::c_int,
            );
            return assignment;
        }
    }
    ::core::ptr::null_mut::<Assignment>()
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tree::tree_render;
use i3_common::safewrappers::srealloc;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::free;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Assignment;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::r#match::match_matches_window;
