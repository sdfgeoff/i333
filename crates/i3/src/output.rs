/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * output.c: Output (monitor) related functions.
 *
 */
// Mechanical file-by-file Rust translation of src/output.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
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

pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_0 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_0 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_5 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_5 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_5 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_5 = 0;
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

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_7 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_7 = 1;
pub const M_HERE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_8 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_8 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_8 = 4;
pub const WM_TILING: C2RustUnnamed_8 = 3;
pub const WM_TILING_USER: C2RustUnnamed_8 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_8 = 1;
pub const WM_ANY: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_9 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_9 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_9 = 1;
pub const M_NODOCK: C2RustUnnamed_9 = 0;
pub const M_DONTCHECK: C2RustUnnamed_9 = -1;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_10 = 1;
pub const U_LATEST: C2RustUnnamed_10 = 0;
pub const U_DONTCHECK: C2RustUnnamed_10 = -1;

pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_11 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_11 = 1;
pub const W_NODOCK: C2RustUnnamed_11 = 0;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_14 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_14 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_14 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_14 = 2;
pub const A_COMMAND: C2RustUnnamed_14 = 1;
pub const A_ANY: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_16 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_16 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_16 = 3;
pub const CT_CON: C2RustUnnamed_16 = 2;
pub const CT_OUTPUT: C2RustUnnamed_16 = 1;
pub const CT_ROOT: C2RustUnnamed_16 = 0;

pub type Output = xoutput;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b"Output *get_output_for_con(Con *)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn output_get_content(output: *mut Con) -> *mut Con {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*output).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return child;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn get_output_from_string(
    current_output: *mut Output,
    output_str: *const ::core::ffi::c_char,
) -> *mut Output {
    if strcasecmp(output_str, c"current".as_ptr()) == 0 as ::core::ffi::c_int {
        return get_output_for_con(focused);
    }
    if strcasecmp(output_str, c"left".as_ptr()) == 0 as ::core::ffi::c_int {
        return get_output_next_wrap(D_LEFT, current_output);
    }
    if strcasecmp(output_str, c"right".as_ptr()) == 0 as ::core::ffi::c_int {
        return get_output_next_wrap(D_RIGHT, current_output);
    }
    if strcasecmp(output_str, c"up".as_ptr()) == 0 as ::core::ffi::c_int {
        return get_output_next_wrap(D_UP, current_output);
    }
    if strcasecmp(output_str, c"down".as_ptr()) == 0 as ::core::ffi::c_int {
        return get_output_next_wrap(D_DOWN, current_output);
    }
    get_output_by_name(output_str, true_0 != 0)
}

pub unsafe fn output_primary_name(output: *mut Output) -> *mut ::core::ffi::c_char {
    (*(*output).names_head.slh_first).name
}

pub unsafe fn get_output_for_con(con: *mut Con) -> *mut Output {
    let output_con: *mut Con = con_get_output(con);
    let output: *mut Output = get_output_by_name((*output_con).name, true_0 != 0);
    if output != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Output {
    } else {
        __assert_fail(
            c"output != NULL".as_ptr(),
            c"../src/output.c".as_ptr(),
            70 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    output
}

pub unsafe fn output_push_sticky_windows(old_focus: *mut Con) {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).focus_head.tqh_first as *mut Con;
    while !output.is_null() {
        let mut visible_ws: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !workspace_is_visible(child) {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    visible_ws = child;
                    break;
                }
            }
        }
        let mut workspace: *mut Con =
            (*output_get_content(output)).focus_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            let current_ws: *mut Con = workspace;
            workspace = (*workspace).focused.tqe_next as *mut Con;
            let mut child_0: *mut Con = (*current_ws).focus_head.tqh_first as *mut Con;
            while !child_0.is_null() {
                let current: *mut Con = child_0;
                child_0 = (*child_0).focused.tqe_next as *mut Con;
                if (*current).type_0 as ::core::ffi::c_uint
                    != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                    || !con_is_sticky(current)
                {
                    continue;
                }
                let ignore_focus: bool = old_focus.is_null() || current != (*old_focus).parent;
                con_move_to_workspace(current, visible_ws, true_0 != 0, false_0 != 0, ignore_focus);
                if !ignore_focus {
                    let current_ws_0: *mut Con = con_get_workspace(focused);
                    con_activate(con_descend_focused(current));
                    con_activate(con_descend_focused(current_ws_0));
                }
            }
        }
        output = (*output).focused.tqe_next as *mut Con;
    }
}

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::strcasecmp;

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_activate;
use crate::con::con_descend_focused;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_is_sticky;
use crate::con::con_move_to_workspace;
use crate::randr::get_output_by_name;
use crate::randr::get_output_next_wrap;
use crate::tree::croot;
use crate::tree::focused;
use crate::workspace::workspace_is_visible;
