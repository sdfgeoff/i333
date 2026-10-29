use crate::tree::all_cons;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * scratchpad.c: Moving windows to the scratchpad and making them visible again.
 *
 */
// Mechanical file-by-file Rust translation of src/scratchpad.c.
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn scratchpad_move(mut con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        verboselog(
            c"'move scratchpad' used on a workspace \"%s\". Calling it recursively on all windows on this workspace.\n".as_ptr().cast_mut(),
            (*con).name,
        );
        let mut current: *mut Con = (*con).focus_head.tqh_first as *mut Con;
        while !current.is_null() {
            let next: *mut Con = (*current).focused.tqe_next as *mut Con;
            scratchpad_move(current);
            current = next;
        }
        return;
    }
    debuglog(
        c"%s:%s:%d - should move con %p to __i3_scratch\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_move".as_ptr(),
        32 as ::core::ffi::c_int,
        con,
    );
    let mut __i3_scratch: *mut Con = workspace_get(c"__i3_scratch".as_ptr());
    if con_get_workspace(con) == __i3_scratch {
        debuglog(
            c"%s:%s:%d - This window is already on __i3_scratch.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_move".as_ptr(),
            36 as ::core::ffi::c_int,
        );
        return;
    }
    con_disable_fullscreen(con);
    let maybe_floating_con: *mut Con = con_inside_floating(con);
    if maybe_floating_con.is_null() {
        floating_enable(con, false_0 != 0);
        con = (*con).parent as *mut Con;
    } else {
        con = maybe_floating_con;
    }
    con_move_to_workspace(con, __i3_scratch, true_0 != 0, true_0 != 0, false_0 != 0);
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - This window was never used as a scratchpad before.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_move".as_ptr(),
            64 as ::core::ffi::c_int,
        );
        if con == maybe_floating_con {
            debuglog(
                c"%s:%s:%d - It was in floating mode before, set scratchpad state to changed.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_move".as_ptr(),
                66 as ::core::ffi::c_int,
            );
            (*con).scratchpad_state = SCRATCHPAD_CHANGED;
        } else {
            debuglog(
                c"%s:%s:%d - It was in tiling mode before, set scratchpad state to fresh.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_move".as_ptr(),
                69 as ::core::ffi::c_int,
            );
            (*con).scratchpad_state = SCRATCHPAD_FRESH;
        }
    }
}

pub unsafe fn scratchpad_show(mut con: *mut Con) -> bool {
    debuglog(
        c"%s:%s:%d - should show scratchpad window %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_show".as_ptr(),
        85 as ::core::ffi::c_int,
        con,
    );
    let mut __i3_scratch: *mut Con = workspace_get(c"__i3_scratch".as_ptr());
    let mut floating: *mut Con = ::core::ptr::null_mut::<Con>();
    if con.is_null()
        && {
            floating = con_inside_floating(focused);
            !floating.is_null()
        }
        && (*floating).scratchpad_state as ::core::ffi::c_uint
            != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Focused window is a scratchpad window, hiding it.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            95 as ::core::ffi::c_int,
        );
        scratchpad_move(focused);
        return true_0 != 0;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null()
        && (*fs).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fs = (*fs).parent as *mut Con;
    }
    if !fs.is_null()
        && (*fs).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_toggle_fullscreen(fs, CF_OUTPUT as ::core::ffi::c_int);
    }
    let mut walk_con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut focused_ws: *mut Con = con_get_workspace(focused);
    walk_con = (*focused_ws).floating_head.tqh_first as *mut Con;
    while !walk_con.is_null() {
        if con.is_null()
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && (*floating).scratchpad_state as ::core::ffi::c_uint
                != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && floating != con_inside_floating(focused)
        {
            debuglog(
                c"%s:%s:%d - Found an unfocused scratchpad window on this workspace\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                119 as ::core::ffi::c_int,
            );
            debuglog(
                c"%s:%s:%d - Focusing it: %p\n".as_ptr().cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                120 as ::core::ffi::c_int,
                walk_con,
            );
            con_activate(con_descend_tiling_focused(walk_con));
            return true_0 != 0;
        }
        walk_con = (*walk_con).floating_windows.tqe_next as *mut Con;
    }
    focused_ws = con_get_workspace(focused);
    walk_con = all_cons.tqh_first as *mut Con;
    while !walk_con.is_null() {
        let walk_ws: *mut Con = con_get_workspace(walk_con);
        if con.is_null()
            && !walk_ws.is_null()
            && !con_is_internal(walk_ws)
            && focused_ws != walk_ws
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && (*floating).scratchpad_state as ::core::ffi::c_uint
                != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - Found a visible scratchpad window on another workspace,\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                139 as ::core::ffi::c_int,
            );
            debuglog(
                c"%s:%s:%d - moving it to this workspace: con = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                140 as ::core::ffi::c_int,
                walk_con,
            );
            con_move_to_workspace(
                floating,
                focused_ws,
                true_0 != 0,
                false_0 != 0,
                false_0 != 0,
            );
            con_activate(con_descend_focused(walk_con));
            return true_0 != 0;
        }
        walk_con = (*walk_con).all_cons.tqe_next as *mut Con;
    }
    if !con.is_null()
        && (*(*con).parent).scratchpad_state as ::core::ffi::c_uint
            == SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Window is not in the scratchpad, doing nothing.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            150 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let active: *mut Con = con_get_workspace(focused);
    let current: *mut Con = con_get_workspace(con);
    if !con.is_null()
        && {
            floating = con_inside_floating(con);
            !floating.is_null()
        }
        && (*floating).scratchpad_state as ::core::ffi::c_uint
            != SCRATCHPAD_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && current != __i3_scratch
    {
        if current == active {
            debuglog(
                c"%s:%s:%d - Window is a scratchpad window, hiding it.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                165 as ::core::ffi::c_int,
            );
            scratchpad_move(con);
            return true_0 != 0;
        }
    }
    if con.is_null() {
        con = (*__i3_scratch).floating_head.tqh_first as *mut Con;
        if con.is_null() {
            verboselog(
                c"You don't have any scratchpad windows yet.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            verboselog(
                c"Use 'move scratchpad' to move a window to the scratchpad.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return false_0 != 0;
        }
    } else {
        con = con_inside_floating(con);
    }
    con_move_to_workspace(con, active, true_0 != 0, false_0 != 0, false_0 != 0);
    if (*con).scratchpad_state as ::core::ffi::c_uint
        == SCRATCHPAD_FRESH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Adjusting size of this window.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            193 as ::core::ffi::c_int,
        );
        let output: *mut Con = con_get_output(con);
        (*con).rect.width = ((*output).rect.width as ::core::ffi::c_double * 0.5f64) as uint32_t;
        (*con).rect.height = ((*output).rect.height as ::core::ffi::c_double * 0.75f64) as uint32_t;
        floating_check_size(con, false_0 != 0);
        floating_center(con, (*con_get_workspace(con)).rect as Rect);
    }
    if current != active {
        workspace_show(active);
    }
    con_activate(con_descend_focused(con));
    true_0 != 0
}
unsafe fn _gcd(m: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if n == 0 as ::core::ffi::c_int {
        return m;
    }
    _gcd(n, m % n)
}
unsafe fn _lcm(m: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let o: ::core::ffi::c_int = _gcd(m, n) as ::core::ffi::c_int;
    m * n / o
}

pub unsafe fn scratchpad_fix_resolution() {
    let mut __i3_scratch: *mut Con = workspace_get(c"__i3_scratch".as_ptr());
    let mut __i3_output: *mut Con = con_get_output(__i3_scratch);
    debuglog(
        c"%s:%s:%d - Current resolution: (%d, %d) %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_fix_resolution".as_ptr(),
        253 as ::core::ffi::c_int,
        (*__i3_output).rect.x,
        (*__i3_output).rect.y,
        (*__i3_output).rect.width,
        (*__i3_output).rect.height,
    );
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut new_width: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut new_height: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !(output == __i3_output) {
            debuglog(
                c"%s:%s:%d - output %s's resolution: (%d, %d) %d x %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_fix_resolution".as_ptr(),
                263 as ::core::ffi::c_int,
                (*output).name,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if new_width == -(1 as ::core::ffi::c_int) {
                new_width = (*output).rect.width as ::core::ffi::c_int;
                new_height = (*output).rect.height as ::core::ffi::c_int;
            } else {
                new_width = _lcm(new_width, (*output).rect.width as ::core::ffi::c_int);
                new_height = _lcm(new_height, (*output).rect.height as ::core::ffi::c_int);
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let mut old_rect: Rect = (*__i3_output).rect as Rect;
    debuglog(
        c"%s:%s:%d - new width = %d, new height = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_fix_resolution".as_ptr(),
        276 as ::core::ffi::c_int,
        new_width,
        new_height,
    );
    (*__i3_output).rect.width = new_width as uint32_t;
    (*__i3_output).rect.height = new_height as uint32_t;
    let mut new_rect: Rect = (*__i3_output).rect as Rect;
    if rect_equals(new_rect, old_rect) {
        debuglog(
            c"%s:%s:%d - Scratchpad size unchanged.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_fix_resolution".as_ptr(),
            283 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - Fixing coordinates of scratchpad windows\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_fix_resolution".as_ptr(),
        287 as ::core::ffi::c_int,
    );
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = (*__i3_scratch).floating_head.tqh_first as *mut Con;
    while !con.is_null() {
        floating_fix_coordinates(con, &raw mut old_rect, &raw mut new_rect);
        con = (*con).floating_windows.tqe_next as *mut Con;
    }
}

// Canonical Cairo backend ABI; renderer logic remains local.

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_activate;
use crate::con::con_descend_focused;
use crate::con::con_descend_tiling_focused;
use crate::con::con_disable_fullscreen;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_inside_floating;
use crate::con::con_is_internal;
use crate::con::con_move_to_workspace;
use crate::con::con_toggle_fullscreen;
use crate::floating::floating_center;
use crate::floating::floating_check_size;
use crate::floating::floating_enable;
use crate::floating::floating_fix_coordinates;
use crate::tree::croot;
use crate::tree::focused;
use crate::util::rect_equals;
use crate::workspace::workspace_get;
use crate::workspace::workspace_show;
