/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * output.c: Output (monitor) related functions.
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{focus, identity, resolve};
use crate::con::{
    con_activate, con_descend_focused, con_get_output, con_get_workspace, con_is_sticky,
    con_move_to_workspace, CT_CON, CT_FLOATING_CON,
};
use crate::randr::{get_output_by_name, get_output_next_wrap};
use crate::tree::{croot, focused};
use crate::types::{xoutput as Output, Con, Direction};
use crate::workspace::workspace_is_visible;
use std::ffi::CStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputQuery {
    Current,
    Next(Direction),
    Named,
}

fn output_query(name: &CStr) -> OutputQuery {
    for (keyword, query) in [
        (c"current", OutputQuery::Current),
        (c"left", OutputQuery::Next(Direction::Left)),
        (c"right", OutputQuery::Next(Direction::Right)),
        (c"up", OutputQuery::Next(Direction::Up)),
        (c"down", OutputQuery::Next(Direction::Down)),
    ] {
        // SAFETY: both CStr values guarantee live, terminated strings. Preserve
        // the current locale's strcasecmp semantics instead of ASCII folding.
        if unsafe { libc::strcasecmp(name.as_ptr(), keyword.as_ptr()) } == 0 {
            return query;
        }
    }
    OutputQuery::Named
}

#[derive(Debug, PartialEq, Eq)]
enum StickyMove {
    Skip,
    Move { restore_focus: bool },
}

fn sticky_move(floating: bool, sticky: bool, was_focused: bool) -> StickyMove {
    if floating && sticky {
        StickyMove::Move {
            restore_focus: was_focused,
        }
    } else {
        StickyMove::Skip
    }
}

/// Find an output node's content child, returning a borrowed native pointer.
///
/// # Safety
/// The caller owns event-thread access to a live output/container graph. The
/// returned pointer must not outlive graph mutations which remove that node.
pub unsafe fn output_get_content(output: *mut Con) -> *mut Con {
    // SAFETY: the caller provides the stable intrusive queue. No references
    // into it are created or held across callbacks.
    unsafe {
        let mut child = nodes::first_ptr(output);
        while !child.is_null() {
            if (*child).type_0 == CT_CON {
                return child;
            }
            child = nodes::next_ptr(child);
        }
        std::ptr::null_mut()
    }
}

/// Resolve a keyword or output name against the current native output graph.
///
/// # Safety
/// `output_str` is a readable NUL string; `current_output` and global focus/output
/// state are initialized and exclusively owned by this event thread. Returned
/// pointers borrow the graph and must not survive removal of their nodes.
pub unsafe fn get_output_from_string(
    current_output: *mut Output,
    output_str: *const libc::c_char,
) -> *mut Output {
    // SAFETY: the input string and all lookup globals satisfy the caller's
    // lifetime contract. Pure classification completes before native lookups.
    unsafe {
        match output_query(CStr::from_ptr(output_str)) {
            OutputQuery::Current => get_output_for_con(focused),
            OutputQuery::Next(direction) => get_output_next_wrap(direction, current_output),
            OutputQuery::Named => get_output_by_name(output_str, true),
        }
    }
}

/// Return the primary name borrowed from an output's native name list.
///
/// # Safety
/// `output` has a nonempty, initialized name list. Its first name and node must
/// remain live and exclusively accessible while the returned pointer is used.
pub unsafe fn output_primary_name(output: *mut Output) -> *mut libc::c_char {
    unsafe { (*(*output).names_head.slh_first).name }
}

/// Resolve the output containing a live native container.
///
/// # Safety
/// The caller owns the initialized event-thread tree and output list. `con`
/// belongs to a valid output with a live NUL name; the returned pointer borrows
/// that output until its next removal from the native graph.
pub unsafe fn get_output_for_con(con: *mut Con) -> *mut Output {
    unsafe {
        let output_con = con_get_output(con);
        let output = get_output_by_name((*output_con).name, true);
        if output.is_null() {
            i3_common::native_stdio::__assert_fail(
                c"output != NULL".as_ptr(),
                c"../src/output.c".as_ptr(),
                70,
                c"Output *get_output_for_con(Con *)".as_ptr(),
            );
        }
        output
    }
}

/// Push sticky floating windows onto each output's visible workspace.
///
/// # Safety
/// The caller owns exclusive access to initialized output/workspace/focus graphs.
/// `old_focus` is null or a live node throughout the call. Native move/activation
/// may mutate queues, so no other graph borrows may overlap this operation.
pub unsafe fn output_push_sticky_windows(old_focus: *mut Con) {
    // SAFETY: Traversal resolves IDs at native event-thread use boundaries.
    // Save workspace/window successors BEFORE moving nodes or activating focus;
    // no Rust references to graph nodes survive any of those callbacks.
    unsafe {
        let root_id = identity(croot).expect("live root focus owner");
        let mut output_id = focus::first(root_id);
        while let Some(current_output) = output_id {
            let Some(output) = resolve(current_output) else {
                break;
            };
            let content = output_get_content(output);
            let content_id = identity(content).expect("live output content");
            let mut visible_ws = nodes::first_ptr(content);
            while !visible_ws.is_null() && !workspace_is_visible(visible_ws) {
                visible_ws = nodes::next_ptr(visible_ws);
            }
            let mut workspace = focus::first(content_id);
            while let Some(current_ws_id) = workspace {
                // These successors intentionally precede move/activation callbacks.
                workspace = focus::next(content_id, current_ws_id);
                if resolve(current_ws_id).is_none() {
                    continue;
                }
                let mut child = focus::first(current_ws_id);
                while let Some(current_id) = child {
                    child = focus::next(current_ws_id, current_id);
                    let Some(current) = resolve(current_id) else {
                        continue;
                    };
                    let floating = (*current).type_0 == CT_FLOATING_CON;
                    let sticky = floating && con_is_sticky(current);
                    let was_focused = floating
                        && sticky
                        && !old_focus.is_null()
                        && current == parent_ptr(old_focus);
                    let StickyMove::Move { restore_focus } =
                        sticky_move(floating, sticky, was_focused)
                    else {
                        continue;
                    };
                    con_move_to_workspace(current, visible_ws, true, false, !restore_focus);
                    if restore_focus {
                        let current_workspace = con_get_workspace(focused);
                        con_activate(con_descend_focused(current));
                        con_activate(con_descend_focused(current_workspace));
                    }
                }
            }
            // Output focus may reorder during activation: query its live
            // successor only after completing this output's work.
            output_id = focus::next(root_id, current_output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_keep_order_case_insensitivity_and_raw_named_fallback() {
        for (name, expected) in [
            (c"current", OutputQuery::Current),
            (c"CuRrEnT", OutputQuery::Current),
            (c"LEFT", OutputQuery::Next(Direction::Left)),
            (c"right", OutputQuery::Next(Direction::Right)),
            (c"Up", OutputQuery::Next(Direction::Up)),
            (c"DOWN", OutputQuery::Next(Direction::Down)),
            (c"", OutputQuery::Named),
            (c"current-output", OutputQuery::Named),
            (c" left", OutputQuery::Named),
            (c"HDMI-1", OutputQuery::Named),
            (
                CStr::from_bytes_with_nul(b"\xff\xfe\0").unwrap(),
                OutputQuery::Named,
            ),
        ] {
            assert_eq!(output_query(name), expected);
        }
    }

    #[test]
    fn only_sticky_floating_windows_move_and_only_old_focus_restores_activation() {
        for was_focused in [false, true] {
            for (floating, sticky) in [(false, false), (false, true), (true, false)] {
                assert_eq!(sticky_move(floating, sticky, was_focused), StickyMove::Skip);
            }
            assert_eq!(
                sticky_move(true, true, was_focused),
                StickyMove::Move {
                    restore_focus: was_focused
                }
            );
        }
    }
}

use crate::con::arena::nodes;
