use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, floating, identity, resolve};
use crate::domains::{ContainerType, FullscreenMode};
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
    if ((*con).type_0) == ContainerType::Workspace {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_owner_1 = (*con).name.clone();
            let log_value_0 = i3_common::logging::string_bytes(log_owner_1.as_deref(), None);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                ::log::info!("'move scratchpad' used on a workspace \"{}\". Calling it recursively on all windows on this workspace.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "'move scratchpad' used on a workspace \"".as_bytes(),
                        log_value_0,
                        "\". Calling it recursively on all windows on this workspace.\n".as_bytes(),
                    ],
                );
            }
        };
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
    ::log::debug!(
        "../src/scratchpad.c:scratchpad_move:32 - should move con {} to __i3_scratch\n",
        i3_common::logging::pointer(con)
    );
    let mut __i3_scratch: *mut Con = workspace_get(c"__i3_scratch".as_ptr());
    if con_get_workspace(con) == __i3_scratch {
        ::log::debug!(
            "../src/scratchpad.c:scratchpad_move:36 - This window is already on __i3_scratch.\n"
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
        ::log::debug!("../src/scratchpad.c:scratchpad_move:64 - This window was never used as a scratchpad before.\n");
        if con == maybe_floating_con {
            ::log::debug!("../src/scratchpad.c:scratchpad_move:66 - It was in floating mode before, set scratchpad state to changed.\n");
            (*con).scratchpad_state = ScratchpadState::Changed;
        } else {
            ::log::debug!("../src/scratchpad.c:scratchpad_move:69 - It was in tiling mode before, set scratchpad state to fresh.\n");
            (*con).scratchpad_state = ScratchpadState::Fresh;
        }
    }
}

pub unsafe fn scratchpad_show(mut con: *mut Con) -> bool {
    ::log::debug!(
        "../src/scratchpad.c:scratchpad_show:85 - should show scratchpad window {}\n",
        i3_common::logging::pointer(con)
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
        ::log::debug!("../src/scratchpad.c:scratchpad_show:95 - Focused window is a scratchpad window, hiding it.\n");
        scratchpad_move(focused);
        return true;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null() && ((*fs).fullscreen_mode) == FullscreenMode::None {
        fs = parent_ptr(fs);
    }
    if !fs.is_null() && ((*fs).type_0) != ContainerType::Workspace {
        con_toggle_fullscreen(fs, FullscreenMode::Output);
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
            ::log::debug!("../src/scratchpad.c:scratchpad_show:119 - Found an unfocused scratchpad window on this workspace\n");
            ::log::debug!(
                "../src/scratchpad.c:scratchpad_show:120 - Focusing it: {}\n",
                i3_common::logging::pointer(walk_con)
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
            ::log::debug!("../src/scratchpad.c:scratchpad_show:139 - Found a visible scratchpad window on another workspace,\n");
            ::log::debug!(
                "../src/scratchpad.c:scratchpad_show:140 - moving it to this workspace: con = {}\n",
                i3_common::logging::pointer(walk_con)
            );
            con_move_to_workspace(floating, focused_ws, true, false, false);
            con_activate(con_descend_focused(walk_con));
            return true;
        }
    }
    if !con.is_null() && (*parent_ptr(con)).scratchpad_state == ScratchpadState::None {
        ::log::debug!("../src/scratchpad.c:scratchpad_show:150 - Window is not in the scratchpad, doing nothing.\n");
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
            ::log::debug!("../src/scratchpad.c:scratchpad_show:165 - Window is a scratchpad window, hiding it.\n");
            scratchpad_move(con);
            return true;
        }
    }
    if con.is_null() {
        con = floating::first(identity(__i3_scratch).expect("live scratchpad workspace"))
            .and_then(resolve)
            .unwrap_or(std::ptr::null_mut());
        if con.is_null() {
            ::log::info!("You don't have any scratchpad windows yet.\n");
            ::log::info!("Use 'move scratchpad' to move a window to the scratchpad.\n");
            return false;
        }
    } else {
        con = con_inside_floating(con);
    }
    con_move_to_workspace(con, active, true, false, false);
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        ::log::debug!("../src/scratchpad.c:scratchpad_show:193 - Adjusting size of this window.\n");
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
    ::log::debug!("../src/scratchpad.c:scratchpad_fix_resolution:253 - Current resolution: ({}, {}) {} x {}\n", ((*__i3_output).rect.x) as i32, ((*__i3_output).rect.y) as i32, ((*__i3_output).rect.width) as i32, ((*__i3_output).rect.height) as i32);
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut new_width: i32 = -(1);
    let mut new_height: i32 = -(1);
    output = nodes::first_ptr(croot);
    while !output.is_null() {
        if !(output == __i3_output) {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_owner_1 = (*output).name.clone();
                let log_value_0 = i3_common::logging::string_bytes(log_owner_1.as_deref(), None);
                let log_value_1 = ((*output).rect.x) as i32;
                let log_value_2 = ((*output).rect.y) as i32;
                let log_value_3 = ((*output).rect.width) as i32;
                let log_value_4 = ((*output).rect.height) as i32;
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                    ::log::debug!("../src/scratchpad.c:scratchpad_fix_resolution:263 - output {}'s resolution: ({}, {}) {} x {}\n", log_value_0_text, log_value_1, log_value_2, log_value_3, log_value_4);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Debug,
                        &[
                            "../src/scratchpad.c:scratchpad_fix_resolution:263 - output "
                                .as_bytes(),
                            log_value_0,
                            format!(
                                "'s resolution: ({}, {}) {} x {}\n",
                                log_value_1, log_value_2, log_value_3, log_value_4
                            )
                            .as_bytes(),
                        ],
                    );
                }
            };
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
        output = nodes::next_ptr(output);
    }
    let mut old_rect: Rect = (*__i3_output).rect as Rect;
    ::log::debug!(
        "../src/scratchpad.c:scratchpad_fix_resolution:276 - new width = {}, new height = {}\n",
        { (new_width) },
        { (new_height) }
    );
    (*__i3_output).rect.width = new_width as u32;
    (*__i3_output).rect.height = new_height as u32;
    let mut new_rect: Rect = (*__i3_output).rect as Rect;
    if rect_equals(new_rect, old_rect) {
        ::log::debug!(
            "../src/scratchpad.c:scratchpad_fix_resolution:283 - Scratchpad size unchanged.\n"
        );
        return;
    }
    ::log::debug!("../src/scratchpad.c:scratchpad_fix_resolution:287 - Fixing coordinates of scratchpad windows\n");
    for floating_id in floating::children(identity(__i3_scratch).expect("live floating parent")) {
        let Some(con) = resolve(floating_id) else {
            continue;
        };
        floating_fix_coordinates(con, &raw mut old_rect, &raw mut new_rect);
    }
}

// Canonical Cairo backend ABI; renderer logic remains local.

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

use crate::con::arena::nodes;
