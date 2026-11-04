use crate::flags::AssignmentKind;
/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 * Assignments for specific windows (for_window).
 */

use crate::application::assignments;
use crate::commands_parser::parse_command;
use crate::r#match::match_matches_window;
use crate::tree::tree_render;
use crate::types::{Assignment, AssignmentDest, Window};

/// Owned command-parser outcome, independent of the mutable window graph.
#[derive(Debug, Default)]
pub struct CommandResult {
    pub parse_error: bool,
    pub error_message: Option<Vec<u8>>,
    pub needs_tree_render: bool,
}

fn window_command(id: u32, command: &[u8]) -> Vec<u8> {
    // The original native printf used %d, interpreting the XID's bits as a
    // signed int. Keep that behavior even for IDs above i32::MAX.
    let mut bytes = format!("[id=\"{}\"] ", id as i32).into_bytes();
    bytes.extend_from_slice(command.split(|byte| *byte == 0).next().unwrap());
    bytes
}

fn accepts_assignment(kind: AssignmentKind, filter: AssignmentKind) -> bool {
    kind.accepts(filter)
}

fn already_ran(ran: &[*mut Assignment], assignment: *mut Assignment) -> bool {
    ran.contains(&assignment)
}

/// The Window is live on the event thread; this borrowed slice never crosses
/// matching/command callbacks. Entries identify global Assignment nodes only.
unsafe fn window_already_ran(window: *const Window, assignment: *mut Assignment) -> bool {
    already_ran(unsafe { &(*window).ran_assignments }, assignment)
}

/// Record a borrowed node identity before any callback can reload configuration.
unsafe fn remember_assignment(window: *mut Window, assignment: *mut Assignment) {
    let ran = unsafe { &mut (*window).ran_assignments };
    assert!(ran.len() < u32::MAX as usize, "assignment count overflow");
    ran.push(assignment);
}

/// Operates on i3's live event-thread graph. Matching and parsing may call back
/// into tree/configuration code: keep traversal pointers raw and borrow only
/// individual fields before each callback. Reload invalidation is detected by
/// the original first-assignment sentinel before following the current node.
pub unsafe fn run_assignments(window: *mut Window) {
    if window.is_null() {
        return;
    }
    ::log::debug!(
        "../src/assignments.c:run_assignments:22 - Checking if any assignments match this window\n"
    );
    let mut needs_tree_render = false;
    let old_first_assignment = assignments.tqh_first;
    let mut current = assignments.tqh_first;
    while !current.is_null() {
        if (*current).kind() == AssignmentKind::COMMAND
            && match_matches_window(&raw mut (*current).match_0, window)
        {
            if window_already_ran(window, current) {
                ::log::debug!("../src/assignments.c:run_assignments:40 - This assignment already ran for the given window, not executing it again.\n");
            } else {
                remember_assignment(window, current);
                // Own both the command bytes and ID before callbacks can reload
                // and free the assignment list. Non-UTF-8 commands stay bytes.
                let command = {
                    let AssignmentDest::Command(command) = &(*current).dest else {
                        unreachable!("command assignment has a command destination");
                    };
                    let expanded = window_command((*window).id, command);
                    {
                        let log_value_0 = command.clone();
                        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                            ::log::debug!("../src/assignments.c:run_assignments:56 - matching assignment, execute command {}\n", log_value_0_text);
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice("../src/assignments.c:run_assignments:56 - matching assignment, execute command ".as_bytes());
                            message.extend_from_slice(&log_value_0);
                            message.extend_from_slice("\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                        }
                    };
                    expanded
                };
                {
                    let result = parse_command(&command, None, std::ptr::null_mut());
                    needs_tree_render |= result.needs_tree_render;
                }
                if old_first_assignment != assignments.tqh_first {
                    break;
                }
            }
        }
        current = (*current).assignments.tqe_next;
    }
    if needs_tree_render {
        tree_render();
    }
}

/// window and the global assignment list must satisfy match_matches_window's
/// live event-thread graph contract. The returned pointer borrows the current
/// configuration and must not be retained across a configuration reload.
pub unsafe fn assignment_for(window: *mut Window, filter: AssignmentKind) -> *mut Assignment {
    let mut assignment = assignments.tqh_first;
    while !assignment.is_null() {
        if accepts_assignment((*assignment).kind(), filter)
            && match_matches_window(&raw mut (*assignment).match_0, window)
        {
            ::log::debug!("../src/assignments.c:assignment_for:95 - got a matching assignment\n");
            return assignment;
        }
        assignment = (*assignment).assignments.tqe_next;
    }
    std::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_command_matches_native_signed_printf_and_preserves_bytes() {
        let commands = [c"", c"move workspace 2; mark \"%s\"", c"\xff\xfe title"];
        for id in [0u32, 1, i32::MAX as u32, 0x8000_0000, u32::MAX] {
            for command in commands {
                let mut native = [0u8; 256];
                let len = unsafe {
                    libc::snprintf(
                        native.as_mut_ptr().cast(),
                        native.len(),
                        c"[id=\"%d\"] %s".as_ptr(),
                        id,
                        command.as_ptr(),
                    )
                };
                assert!(len >= 0 && (len as usize) < native.len());
                assert_eq!(
                    window_command(id, command.to_bytes()),
                    &native[..len as usize]
                );
            }
        }
    }

    #[test]
    fn selection_preserves_any_and_bitmask_behavior() {
        let accepts_assignment = |kind: u32, filter: i32| {
            super::accepts_assignment(
                AssignmentKind::from_bits_retain(kind),
                AssignmentKind::from_bits_retain(filter as u32),
            )
        };
        let kinds = [0, 1, 2, 4, 8, 16, 3, 0x8000_0000];
        for kind in kinds {
            assert!(accepts_assignment(kind, 0));
        }
        assert!(accepts_assignment(3, 1));
        assert!(accepts_assignment(3, 2));
        assert!(!accepts_assignment(2, 1));
        assert!(!accepts_assignment(0, -1));
        assert!(accepts_assignment(0x8000_0000, i32::MIN));
        assert!(accepts_assignment(16, -1));
    }

    #[test]
    fn dedup_uses_assignment_identity_and_ignores_unused_capacity() {
        let first = Assignment::new(
            AssignmentDest::Command(b"nop".to_vec()),
            crate::types::Match::empty(),
        );
        let second = Assignment::new(
            AssignmentDest::Command(b"nop".to_vec()),
            crate::types::Match::empty(),
        );
        let first = std::ptr::from_ref(&first).cast_mut();
        let second = std::ptr::from_ref(&second).cast_mut();
        let entries = [first, second, first];
        assert!(!already_ran(&[], first));
        assert!(already_ran(&entries[..1], first));
        assert!(!already_ran(&entries[..1], second));
        assert!(already_ran(&entries[..2], second));
        assert!(already_ran(&entries, first));
    }
}
