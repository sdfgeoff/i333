//! Generation-checked ancestry independent of ordered child membership.
//!
//! Detaching a node deliberately retains its parent until reattachment or an
//! explicit clear. Retired parents become absent through generation validation;
//! reusing their addresses never reconnects surviving children.
use super::{identity, resolve, Arena, ConId, Entry, CONTAINERS};
use crate::types::Con;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParentError {
    StaleChild,
    StaleParent,
    SelfParent,
    Cycle,
}

impl Arena {
    fn parent_entry(&self, id: ConId) -> Option<&Entry> {
        self.entries
            .get(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }

    fn parent_id(&self, child: ConId) -> Option<ConId> {
        let parent = self.parent_entry(child)?.parent?;
        self.parent_entry(parent).map(|_| parent)
    }

    fn set_parent_id(&mut self, child: ConId, parent: Option<ConId>) -> Result<(), ParentError> {
        self.parent_entry(child).ok_or(ParentError::StaleChild)?;
        if let Some(parent) = parent {
            self.parent_entry(parent).ok_or(ParentError::StaleParent)?;
            if parent == child {
                return Err(ParentError::SelfParent);
            }
            let mut ancestor = Some(parent);
            while let Some(current) = ancestor {
                if current == child {
                    return Err(ParentError::Cycle);
                }
                ancestor = self.parent_id(current);
            }
        }
        // All checks finish before the single write. Queue membership and
        // native callbacks are intentionally separate from ancestry updates.
        self.entries.get_mut(&child.address).unwrap().parent = parent;
        Ok(())
    }
}

pub(crate) fn parent(child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().parent_id(child))
}

pub(crate) fn set_parent(child: ConId, parent: Option<ConId>) -> Result<(), ParentError> {
    CONTAINERS.with(|arena| arena.borrow_mut().set_parent_id(child, parent))
}

/// Native graph adapter: validate addresses, read owned IDs, resolve live parent.
/// No input pointer is dereferenced; using the result remains the caller's native
/// lifetime/aliasing responsibility. Deferred work should capture ConId instead.
pub(crate) fn parent_ptr(raw: *mut Con) -> *mut Con {
    identity(raw)
        .and_then(parent)
        .and_then(resolve)
        .unwrap_or(std::ptr::null_mut())
}

/// Native graph adapter with no pointer dereference. A null parent explicitly
/// clears detached ancestry; all non-null addresses must identify live owners.
pub(crate) fn set_parent_ptr(raw: *mut Con, parent_raw: *mut Con) -> Result<(), ParentError> {
    let child = identity(raw).ok_or(ParentError::StaleChild)?;
    let parent = if parent_raw.is_null() {
        None
    } else {
        Some(identity(parent_raw).ok_or(ParentError::StaleParent)?)
    };
    set_parent(child, parent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::con::arena::{allocate, release};

    fn node(arena: &mut Arena) -> (*mut Con, ConId) {
        let raw = arena.insert(Con::empty());
        (raw, arena.identity(raw).unwrap())
    }
    fn ancestry(arena: &Arena) -> Vec<(u64, Option<ConId>)> {
        arena
            .order
            .iter()
            .map(|(&generation, address)| (generation, arena.entries[address].parent))
            .collect()
    }

    #[test]
    fn detached_ancestry_chains_noop_clear_and_reparent_need_no_list_membership() {
        let mut arena = Arena::default();
        let (_, root) = node(&mut arena);
        let (_, split) = node(&mut arena);
        let (_, child) = node(&mut arena);
        assert_eq!(arena.parent_id(root), None);
        arena.set_parent_id(split, Some(root)).unwrap();
        arena.set_parent_id(child, Some(split)).unwrap();
        assert_eq!(arena.parent_id(child), Some(split));
        assert_eq!(arena.parent_id(split), Some(root));
        let before = ancestry(&arena);
        arena.set_parent_id(child, Some(split)).unwrap();
        assert_eq!(ancestry(&arena), before);
        arena.set_parent_id(split, None).unwrap();
        assert_eq!(arena.parent_id(child), Some(split));
        assert_eq!(arena.parent_id(split), None);
        arena.set_parent_id(child, Some(root)).unwrap();
        assert_eq!(arena.parent_id(child), Some(root));
    }

    #[test]
    fn cycle_self_and_stale_updates_fail_without_changing_any_ancestry() {
        let mut arena = Arena::default();
        let (_, root) = node(&mut arena);
        let (_, split) = node(&mut arena);
        let (_, child) = node(&mut arena);
        arena.set_parent_id(split, Some(root)).unwrap();
        arena.set_parent_id(child, Some(split)).unwrap();
        let before = ancestry(&arena);
        let stale_child = ConId {
            generation: child.generation + 100,
            ..child
        };
        let stale_root = ConId {
            generation: root.generation + 100,
            ..root
        };
        assert_eq!(
            arena.set_parent_id(root, Some(child)),
            Err(ParentError::Cycle)
        );
        assert_eq!(
            arena.set_parent_id(split, Some(child)),
            Err(ParentError::Cycle)
        );
        assert_eq!(
            arena.set_parent_id(child, Some(child)),
            Err(ParentError::SelfParent)
        );
        assert_eq!(
            arena.set_parent_id(child, Some(stale_root)),
            Err(ParentError::StaleParent)
        );
        assert_eq!(
            arena.set_parent_id(stale_child, None),
            Err(ParentError::StaleChild)
        );
        assert_eq!(ancestry(&arena), before);
        assert_eq!(arena.parent_id(stale_child), None);
    }

    #[test]
    fn retired_parent_and_reused_addresses_never_reconnect_children() {
        let mut arena = Arena::default();
        let (parent_raw, old_parent) = node(&mut arena);
        let (_, child) = node(&mut arena);
        arena.set_parent_id(child, Some(old_parent)).unwrap();
        drop(arena.take(parent_raw).unwrap());
        assert_eq!(arena.parent_id(child), None);
        let (_, replacement) = node(&mut arena);
        assert_eq!(arena.parent_id(child), None);
        // Deterministic address-reuse simulation preserves the old generation.
        let stale_reused = ConId {
            address: replacement.address,
            generation: old_parent.generation,
        };
        arena.entries.get_mut(&child.address).unwrap().parent = Some(stale_reused);
        assert_eq!(arena.parent_id(child), None);
        assert_eq!(
            arena.set_parent_id(child, Some(stale_reused)),
            Err(ParentError::StaleParent)
        );
        arena.set_parent_id(child, Some(replacement)).unwrap();
        assert_eq!(arena.parent_id(child), Some(replacement));
        assert_eq!(
            arena.set_parent_id(replacement, Some(child)),
            Err(ParentError::Cycle)
        );
    }

    #[test]
    fn safe_native_adapters_validate_arbitrary_addresses_without_dereferencing() {
        let child = allocate(Con::empty());
        let owner = allocate(Con::empty());
        assert!(parent_ptr(std::ptr::null_mut()).is_null());
        assert_eq!(
            set_parent_ptr(std::ptr::null_mut(), owner),
            Err(ParentError::StaleChild)
        );
        for address in [1, usize::MAX] {
            let invalid = std::ptr::without_provenance_mut::<Con>(address);
            assert!(parent_ptr(invalid).is_null());
            assert_eq!(set_parent_ptr(invalid, owner), Err(ParentError::StaleChild));
            assert_eq!(
                set_parent_ptr(child, invalid),
                Err(ParentError::StaleParent)
            );
        }
        set_parent_ptr(child, owner).unwrap();
        assert_eq!(parent_ptr(child), owner);
        set_parent_ptr(child, std::ptr::null_mut()).unwrap();
        assert!(parent_ptr(child).is_null());
        unsafe {
            release(child);
            release(owner);
        }
    }

    #[test]
    fn real_detach_preserves_parent_and_reattach_changes_ancestry_with_membership() {
        let first = allocate(Con::empty());
        let second = allocate(Con::empty());
        let child = allocate(Con::empty());
        unsafe {
            (*first).type_0 = crate::con::CT_WORKSPACE;
            (*second).type_0 = crate::con::CT_WORKSPACE;
            (*child).type_0 = crate::con::CT_FLOATING_CON;
            crate::con::con_attach(child, first, false);
            assert_eq!(parent_ptr(child), first);
            crate::con::con_detach(child);
            assert_eq!(parent_ptr(child), first);
            crate::con::con_attach(child, second, false);
            assert_eq!(parent_ptr(child), second);
            assert!(super::super::floating::is_empty(identity(first).unwrap()));
            assert_eq!(
                super::super::floating::first(identity(second).unwrap()),
                identity(child)
            );
            crate::con::con_detach(child);
            set_parent_ptr(child, std::ptr::null_mut()).unwrap();
            assert!(parent_ptr(child).is_null());
            release(child);
            release(first);
            release(second);
        }
    }

    #[test]
    fn valid_swap_and_new_wrapper_assignment_order_remains_cycle_free() {
        let mut arena = Arena::default();
        let (_, root) = node(&mut arena);
        let (_, left) = node(&mut arena);
        let (_, right) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        for id in [left, right] {
            arena.set_parent_id(id, Some(root)).unwrap();
        }
        arena.set_parent_id(a, Some(left)).unwrap();
        arena.set_parent_id(b, Some(right)).unwrap();
        // con_swap rejects ancestors beforehand; these original two writes work
        // without a temporarily cyclic state or broad mutation transaction.
        let a_parent = arena.parent_id(a);
        arena.set_parent_id(a, arena.parent_id(b)).unwrap();
        arena.set_parent_id(b, a_parent).unwrap();
        assert_eq!(arena.parent_id(a), Some(right));
        assert_eq!(arena.parent_id(b), Some(left));
        let (_, wrapper) = node(&mut arena);
        arena.set_parent_id(wrapper, Some(right)).unwrap();
        arena.set_parent_id(a, Some(wrapper)).unwrap();
        assert_eq!(arena.parent_id(arena.parent_id(a).unwrap()), Some(right));
    }
}
