use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, floating, identity, resolve};
use crate::types::ScratchpadState;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * scratchpad.c: Moving windows to the scratchpad and making them visible again.
 *
 */
// Local policy and temporary values use owned Rust storage. Native entrypoints
// require live event-thread graph handles and a live display; callbacks may mutate
// the graph, so node references must never span those calls.

const CF_NONE: u32 = 0;
const CF_OUTPUT: u32 = 1;
const CT_WORKSPACE: u32 = 4;

fn greatest_common_divisor(mut first: i32, mut second: i32) -> i32 {
    while second != 0 {
        (first, second) = (second, first.wrapping_rem(second));
    }
    first
}

fn common_resolution(current: Option<(i32, i32)>, width: u32, height: u32) -> (i32, i32) {
    fn lcm(first: i32, second: i32) -> i32 {
        // Match the native signed arithmetic's low bits. Zero-by-zero was an
        // invalid resolution in the original routine too, not a usable output.
        let divisor = greatest_common_divisor(first, second);
        assert_ne!(divisor, 0, "zero output resolution");
        first.wrapping_mul(second).wrapping_div(divisor)
    }
    match current {
        None => (width as i32, height as i32),
        Some((first, second)) => (lcm(first, width as i32), lcm(second, height as i32)),
    }
}

fn fresh_dimensions(output: Rect) -> (u32, u32) {
    (
        (output.width as f64 * 0.5) as u32,
        (output.height as f64 * 0.75) as u32,
    )
}

fn can_focus_visible(state: ScratchpadState, is_focused: bool) -> bool {
    state != ScratchpadState::None && !is_focused
}

pub unsafe fn scratchpad_move(mut con: *mut Con) {
    if (*con).type_0 as u32 == CT_WORKSPACE {
        verboselog(
            c"'move scratchpad' used on a workspace \"%s\". Calling it recursively on all windows on this workspace.\n".as_ptr().cast_mut(),
            (*con).name,
        );
        let owner = crate::con::arena::identity(con).expect("live scratchpad workspace");
        let mut current = focus::first(owner);
        while let Some(id) = current {
            // Save the generation-checked successor BEFORE moving this member.
            current = focus::next(owner, id);
            if let Some(raw) = crate::con::arena::resolve(id) {
                scratchpad_move(raw);
            }
        }
        return;
    }
    debuglog(
        c"%s:%s:%d - should move con %p to __i3_scratch\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_move".as_ptr(),
        32,
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
            36,
        );
        return;
    }
    con_disable_fullscreen(con);
    let maybe_floating_con: *mut Con = con_inside_floating(con);
    if maybe_floating_con.is_null() {
        floating_enable(con, false);
        con = parent_ptr(con);
    } else {
        con = maybe_floating_con;
    }
    con_move_to_workspace(con, __i3_scratch, true, true, false);
    if (*con).scratchpad_state == ScratchpadState::None {
        debuglog(
            c"%s:%s:%d - This window was never used as a scratchpad before.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_move".as_ptr(),
            64,
        );
        if con == maybe_floating_con {
            debuglog(
                c"%s:%s:%d - It was in floating mode before, set scratchpad state to changed.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_move".as_ptr(),
                66,
            );
            (*con).scratchpad_state = ScratchpadState::Changed;
        } else {
            debuglog(
                c"%s:%s:%d - It was in tiling mode before, set scratchpad state to fresh.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_move".as_ptr(),
                69,
            );
            (*con).scratchpad_state = ScratchpadState::Fresh;
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
        85,
        con,
    );
    let mut __i3_scratch: *mut Con = workspace_get(c"__i3_scratch".as_ptr());
    let mut floating: *mut Con = ::core::ptr::null_mut::<Con>();
    if con.is_null()
        && {
            floating = con_inside_floating(focused);
            !floating.is_null()
        }
        && (*floating).scratchpad_state != ScratchpadState::None
    {
        debuglog(
            c"%s:%s:%d - Focused window is a scratchpad window, hiding it.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            95,
        );
        scratchpad_move(focused);
        return true;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null() && (*fs).fullscreen_mode as u32 == CF_NONE {
        fs = parent_ptr(fs);
    }
    if !fs.is_null() && (*fs).type_0 as u32 != CT_WORKSPACE {
        con_toggle_fullscreen(fs, CF_OUTPUT as i32);
    }
    let mut focused_ws: *mut Con = con_get_workspace(focused);
    for floating_id in floating::children(identity(focused_ws).expect("live floating parent")) {
        let Some(walk_con) = resolve(floating_id) else {
            continue;
        };
        if con.is_null()
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && can_focus_visible(
                (*floating).scratchpad_state,
                floating == con_inside_floating(focused),
            )
        {
            debuglog(
                c"%s:%s:%d - Found an unfocused scratchpad window on this workspace\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                119,
            );
            debuglog(
                c"%s:%s:%d - Focusing it: %p\n".as_ptr().cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                120,
                walk_con,
            );
            con_activate(con_descend_tiling_focused(walk_con));
            return true;
        }
    }
    focused_ws = con_get_workspace(focused);
    for id in containers() {
        let Some(walk_con) = resolve(id) else {
            continue;
        };
        let walk_ws: *mut Con = con_get_workspace(walk_con);
        if con.is_null()
            && !walk_ws.is_null()
            && !con_is_internal(walk_ws)
            && focused_ws != walk_ws
            && {
                floating = con_inside_floating(walk_con);
                !floating.is_null()
            }
            && (*floating).scratchpad_state != ScratchpadState::None
        {
            debuglog(
                c"%s:%s:%d - Found a visible scratchpad window on another workspace,\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                139,
            );
            debuglog(
                c"%s:%s:%d - moving it to this workspace: con = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                140,
                walk_con,
            );
            con_move_to_workspace(floating, focused_ws, true, false, false);
            con_activate(con_descend_focused(walk_con));
            return true;
        }
    }
    if !con.is_null() && (*parent_ptr(con)).scratchpad_state == ScratchpadState::None {
        debuglog(
            c"%s:%s:%d - Window is not in the scratchpad, doing nothing.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            150,
        );
        return false;
    }
    let active: *mut Con = con_get_workspace(focused);
    let current: *mut Con = con_get_workspace(con);
    if !con.is_null()
        && {
            floating = con_inside_floating(con);
            !floating.is_null()
        }
        && (*floating).scratchpad_state != ScratchpadState::None
        && current != __i3_scratch
    {
        if current == active {
            debuglog(
                c"%s:%s:%d - Window is a scratchpad window, hiding it.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_show".as_ptr(),
                165,
            );
            scratchpad_move(con);
            return true;
        }
    }
    if con.is_null() {
        con = floating::first(identity(__i3_scratch).expect("live scratchpad workspace"))
            .and_then(resolve)
            .unwrap_or(std::ptr::null_mut());
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
            return false;
        }
    } else {
        con = con_inside_floating(con);
    }
    con_move_to_workspace(con, active, true, false, false);
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        debuglog(
            c"%s:%s:%d - Adjusting size of this window.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_show".as_ptr(),
            193,
        );
        let output: *mut Con = con_get_output(con);
        let (width, height) = fresh_dimensions((*output).rect);
        (*con).rect.width = width;
        (*con).rect.height = height;
        floating_check_size(con, false);
        floating_center(con, (*con_get_workspace(con)).rect as Rect);
    }
    if current != active {
        workspace_show(active);
    }
    con_activate(con_descend_focused(con));
    true
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
        253,
        (*__i3_output).rect.x,
        (*__i3_output).rect.y,
        (*__i3_output).rect.width,
        (*__i3_output).rect.height,
    );
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut new_width: i32 = -(1);
    let mut new_height: i32 = -(1);
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !(output == __i3_output) {
            debuglog(
                c"%s:%s:%d - output %s's resolution: (%d, %d) %d x %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/scratchpad.c".as_ptr(),
                c"scratchpad_fix_resolution".as_ptr(),
                263,
                (*output).name,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if new_width == -(1) {
                (new_width, new_height) =
                    common_resolution(None, (*output).rect.width, (*output).rect.height);
            } else {
                (new_width, new_height) = common_resolution(
                    Some((new_width, new_height)),
                    (*output).rect.width,
                    (*output).rect.height,
                );
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
        276,
        new_width,
        new_height,
    );
    (*__i3_output).rect.width = new_width as u32;
    (*__i3_output).rect.height = new_height as u32;
    let mut new_rect: Rect = (*__i3_output).rect as Rect;
    if rect_equals(new_rect, old_rect) {
        debuglog(
            c"%s:%s:%d - Scratchpad size unchanged.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/scratchpad.c".as_ptr(),
            c"scratchpad_fix_resolution".as_ptr(),
            283,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - Fixing coordinates of scratchpad windows\n"
            .as_ptr()
            .cast_mut(),
        c"../src/scratchpad.c".as_ptr(),
        c"scratchpad_fix_resolution".as_ptr(),
        287,
    );
    for floating_id in floating::children(identity(__i3_scratch).expect("live floating parent")) {
        let Some(con) = resolve(floating_id) else {
            continue;
        };
        floating_fix_coordinates(con, &raw mut old_rect, &raw mut new_rect);
    }
}

// Canonical Cairo backend ABI; renderer logic remains local.

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
use crate::types::Con;
use crate::types::Rect;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolution_and_fresh_dimensions_preserve_integer_rounding() {
        assert_eq!(common_resolution(None, 1920, 1080), (1920, 1080));
        assert_eq!(
            common_resolution(Some((1920, 1080)), 1280, 1024),
            (3840, 138240)
        );
        assert_eq!(common_resolution(None, u32::MAX, 1), (-1, 1));
        assert_eq!(
            fresh_dimensions(Rect {
                x: 0,
                y: 0,
                width: 101,
                height: 103
            }),
            (50, 77)
        );
        assert_eq!(greatest_common_divisor(1920, 1280), 640);
    }
    #[test]
    fn visible_candidate_policy_preserves_scratchpad_and_focus_state() {
        assert!(can_focus_visible(ScratchpadState::Fresh, false));
        assert!(can_focus_visible(ScratchpadState::Changed, false));
        assert!(!can_focus_visible(ScratchpadState::None, false));
        assert!(!can_focus_visible(ScratchpadState::Changed, true));
    }
}
