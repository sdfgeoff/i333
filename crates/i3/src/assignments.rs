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
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;

pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
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

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;
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
            let mut c: uint32_t = 0 as uint32_t;
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
                    (::core::mem::size_of::<*mut Assignment>() as size_t)
                        .wrapping_mul((*window).nr_assignments as size_t),
                ) as *mut *mut Assignment;
                let ref mut fresh0 = *(*window)
                    .ran_assignments
                    .offset((*window).nr_assignments.wrapping_sub(1 as uint32_t) as isize);
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
