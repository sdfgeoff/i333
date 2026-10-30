//! Owned ordered floating membership. This module never dereferences Con links.
use super::{Arena, ConId, Entry, CONTAINERS};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MembershipError {
    StaleParent,
    StaleChild,
    SelfMembership,
    Cycle,
    AlreadyMember,
    OwnedElsewhere,
    NotMember,
}

impl Arena {
    fn floating_entry(&self, id: ConId) -> Option<&Entry> {
        self.entries
            .get(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn floating_entry_mut(&mut self, id: ConId) -> Option<&mut Entry> {
        self.entries
            .get_mut(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn floating_position(&self, parent: ConId, child: ConId) -> Result<usize, MembershipError> {
        let parent_entry = self
            .floating_entry(parent)
            .ok_or(MembershipError::StaleParent)?;
        let child_entry = self
            .floating_entry(child)
            .ok_or(MembershipError::StaleChild)?;
        if child_entry.floating_owner != Some(parent) {
            return Err(MembershipError::NotMember);
        }
        parent_entry
            .floating
            .iter()
            .position(|&id| id == child)
            .ok_or(MembershipError::NotMember)
    }
    fn floating_insert(
        &mut self,
        parent: ConId,
        child: ConId,
        prepend: bool,
    ) -> Result<(), MembershipError> {
        self.floating_entry(parent)
            .ok_or(MembershipError::StaleParent)?;
        let child_entry = self
            .floating_entry(child)
            .ok_or(MembershipError::StaleChild)?;
        if parent == child {
            return Err(MembershipError::SelfMembership);
        }
        if let Some(owner) = child_entry.floating_owner {
            return Err(if owner == parent {
                MembershipError::AlreadyMember
            } else {
                MembershipError::OwnedElsewhere
            });
        }
        let mut ancestor = Some(parent);
        while let Some(id) = ancestor {
            if id == child {
                return Err(MembershipError::Cycle);
            }
            ancestor = self
                .floating_entry(id)
                .and_then(|entry| entry.floating_owner);
        }
        // Every fallible membership check precedes mutation. No callback or
        // allocation of a Con occurs while the registry is borrowed here.
        let members = &mut self.floating_entry_mut(parent).unwrap().floating;
        if prepend {
            members.insert(0, child);
        } else {
            members.push(child);
        }
        self.floating_entry_mut(child).unwrap().floating_owner = Some(parent);
        Ok(())
    }
    fn floating_remove(&mut self, parent: ConId, child: ConId) -> Result<(), MembershipError> {
        let index = self.floating_position(parent, child)?;
        self.floating_entry_mut(parent)
            .unwrap()
            .floating
            .remove(index);
        self.floating_entry_mut(child).unwrap().floating_owner = None;
        Ok(())
    }
    fn floating_reorder(
        &mut self,
        parent: ConId,
        child: ConId,
        rotate: bool,
    ) -> Result<(), MembershipError> {
        let index = self.floating_position(parent, child)?;
        let members = &mut self.floating_entry_mut(parent).unwrap().floating;
        if rotate {
            let offset = (index + 1) % members.len();
            members.rotate_left(offset);
        } else {
            members.remove(index);
            members.push(child);
        }
        Ok(())
    }
    fn floating_first(&self, parent: ConId) -> Option<ConId> {
        self.floating_entry(parent)?.floating.first().copied()
    }
    fn floating_last(&self, parent: ConId) -> Option<ConId> {
        self.floating_entry(parent)?.floating.last().copied()
    }
    fn floating_next(&self, parent: ConId, child: ConId) -> Option<ConId> {
        let index = self.floating_position(parent, child).ok()?;
        self.floating_entry(parent)?
            .floating
            .get(index + 1)
            .copied()
    }
    fn floating_previous(&self, parent: ConId, child: ConId) -> Option<ConId> {
        let index = self.floating_position(parent, child).ok()?.checked_sub(1)?;
        self.floating_entry(parent)?.floating.get(index).copied()
    }
    pub(super) fn floating_retire(&mut self, id: ConId, owner: Option<ConId>, children: &[ConId]) {
        if let Some(owner) = owner.and_then(|owner| self.floating_entry_mut(owner)) {
            owner.floating.retain(|&candidate| candidate != id);
        }
        for &child in children {
            if let Some(child) = self.floating_entry_mut(child) {
                if child.floating_owner == Some(id) {
                    child.floating_owner = None;
                }
            }
        }
        // Typed ancestry intentionally survives membership removal: native
        // callers use the detached parent before reattachment.
    }
}

pub(crate) fn is_empty(parent: ConId) -> bool {
    CONTAINERS.with(|arena| arena.borrow().floating_first(parent).is_none())
}
pub(crate) fn first(parent: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().floating_first(parent))
}
pub(crate) fn last(parent: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().floating_last(parent))
}
pub(crate) fn next(parent: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().floating_next(parent, child))
}
pub(crate) fn previous(parent: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().floating_previous(parent, child))
}
pub(crate) fn append(parent: ConId, child: ConId) -> Result<(), MembershipError> {
    CONTAINERS.with(|arena| arena.borrow_mut().floating_insert(parent, child, false))
}
pub(crate) fn prepend(parent: ConId, child: ConId) -> Result<(), MembershipError> {
    CONTAINERS.with(|arena| arena.borrow_mut().floating_insert(parent, child, true))
}
pub(crate) fn remove(parent: ConId, child: ConId) -> Result<(), MembershipError> {
    CONTAINERS.with(|arena| arena.borrow_mut().floating_remove(parent, child))
}
pub(crate) fn move_to_back(parent: ConId, child: ConId) -> Result<(), MembershipError> {
    CONTAINERS.with(|arena| arena.borrow_mut().floating_reorder(parent, child, false))
}
pub(crate) fn rotate_after(parent: ConId, child: ConId) -> Result<(), MembershipError> {
    CONTAINERS.with(|arena| arena.borrow_mut().floating_reorder(parent, child, true))
}

/// Resolve the successor after the loop body, matching live native iteration.
/// Removing the current member or its parent stops traversal. Callers that
/// intentionally remove the current member must prefetch next() before removal.
pub(crate) struct FloatingChildren {
    parent: ConId,
    current: Option<ConId>,
    finished: bool,
}
impl Iterator for FloatingChildren {
    type Item = ConId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = match self.current {
            None => first(self.parent),
            Some(current) => next(self.parent, current),
        };
        self.current = next;
        self.finished = next.is_none();
        next
    }
}
impl std::iter::FusedIterator for FloatingChildren {}
pub(crate) fn children(parent: ConId) -> FloatingChildren {
    FloatingChildren {
        parent,
        current: None,
        finished: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::con::arena::focus;
    use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
    use crate::con::arena::{allocate, identity, release, resolve};
    use crate::types::Con;

    fn node(arena: &mut Arena) -> (*mut Con, ConId) {
        let raw = arena.insert(Con::empty());
        (raw, arena.identity(raw).unwrap())
    }
    fn order(arena: &Arena, parent: ConId) -> Vec<ConId> {
        arena
            .floating_entry(parent)
            .map_or_else(Vec::new, |entry| entry.floating.clone())
    }
    fn state(arena: &Arena) -> Vec<(u64, Vec<ConId>, Option<ConId>)> {
        arena
            .order
            .iter()
            .map(|(&generation, address)| {
                let entry = &arena.entries[address];
                (generation, entry.floating.clone(), entry.floating_owner)
            })
            .collect()
    }

    #[test]
    fn ordered_membership_queries_remove_and_reparent_preserve_single_ownership() {
        let mut arena = Arena::default();
        let (_, parent) = node(&mut arena);
        let (_, other) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        assert_eq!(arena.floating_first(parent), None);
        arena.floating_insert(parent, b, false).unwrap();
        arena.floating_insert(parent, c, false).unwrap();
        arena.floating_insert(parent, a, true).unwrap();
        assert_eq!(order(&arena, parent), vec![a, b, c]);
        assert_eq!(arena.floating_first(parent), Some(a));
        assert_eq!(arena.floating_last(parent), Some(c));
        assert_eq!(arena.floating_next(parent, a), Some(b));
        assert_eq!(arena.floating_previous(parent, c), Some(b));
        assert_eq!(arena.floating_previous(parent, a), None);
        assert_eq!(arena.floating_next(parent, c), None);
        arena.floating_remove(parent, b).unwrap();
        assert_eq!(order(&arena, parent), vec![a, c]);
        assert_eq!(arena.floating_entry(b).unwrap().floating_owner, None);
        arena.floating_insert(other, b, false).unwrap();
        assert_eq!(order(&arena, other), vec![b]);
        arena.floating_remove(parent, a).unwrap();
        arena.floating_remove(parent, c).unwrap();
        assert_eq!(arena.floating_first(parent), None);
        assert_eq!(arena.floating_last(parent), None);
    }

    #[test]
    fn invalid_mutations_are_atomic_for_stale_wrong_owner_duplicate_and_cycle() {
        let mut arena = Arena::default();
        let (_, parent) = node(&mut arena);
        let (_, other) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        arena.floating_insert(parent, a, false).unwrap();
        arena.floating_insert(a, b, false).unwrap();
        let initial = state(&arena);
        let stale_parent = ConId {
            generation: parent.generation + 100,
            ..parent
        };
        let stale_child = ConId {
            generation: a.generation + 100,
            ..a
        };
        assert_eq!(
            arena.floating_insert(parent, a, false),
            Err(MembershipError::AlreadyMember)
        );
        assert_eq!(
            arena.floating_insert(other, a, false),
            Err(MembershipError::OwnedElsewhere)
        );
        assert_eq!(
            arena.floating_insert(other, other, false),
            Err(MembershipError::SelfMembership)
        );
        assert_eq!(
            arena.floating_insert(b, parent, false),
            Err(MembershipError::Cycle)
        );
        assert_eq!(
            arena.floating_insert(stale_parent, other, false),
            Err(MembershipError::StaleParent)
        );
        assert_eq!(
            arena.floating_insert(other, stale_child, false),
            Err(MembershipError::StaleChild)
        );
        assert_eq!(
            arena.floating_remove(other, a),
            Err(MembershipError::NotMember)
        );
        assert_eq!(
            arena.floating_reorder(other, a, false),
            Err(MembershipError::NotMember)
        );
        assert_eq!(
            arena.floating_reorder(parent, stale_child, true),
            Err(MembershipError::StaleChild)
        );
        assert_eq!(state(&arena), initial);
        assert_eq!(arena.floating_first(stale_parent), None);
        assert_eq!(arena.floating_next(parent, stale_child), None);
    }

    #[test]
    fn raising_and_rotation_have_distinct_order_and_singleton_behavior() {
        let mut arena = Arena::default();
        let (_, parent) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        let (_, d) = node(&mut arena);
        for child in [a, b, c, d] {
            arena.floating_insert(parent, child, false).unwrap();
        }
        arena.floating_reorder(parent, b, true).unwrap();
        assert_eq!(order(&arena, parent), vec![c, d, a, b]);
        arena.floating_reorder(parent, d, false).unwrap();
        assert_eq!(order(&arena, parent), vec![c, a, b, d]);
        arena.floating_reorder(parent, d, true).unwrap();
        assert_eq!(order(&arena, parent), vec![c, a, b, d]);
        arena.floating_reorder(parent, c, true).unwrap();
        assert_eq!(order(&arena, parent), vec![a, b, d, c]);
        for child in [a, b, d] {
            arena.floating_remove(parent, child).unwrap();
        }
        arena.floating_reorder(parent, c, true).unwrap();
        arena.floating_reorder(parent, c, false).unwrap();
        assert_eq!(order(&arena, parent), vec![c]);
    }

    #[test]
    fn retiring_children_and_parents_cleans_reverse_membership_and_stale_ids() {
        let mut arena = Arena::default();
        let (parent_raw, parent) = node(&mut arena);
        let (first_raw, first) = node(&mut arena);
        let (_, second) = node(&mut arena);
        arena.floating_insert(parent, first, false).unwrap();
        arena.floating_insert(parent, second, false).unwrap();
        drop(arena.take(first_raw).unwrap());
        assert_eq!(order(&arena, parent), vec![second]);
        assert!(arena.resolve(first).is_none());
        let (_, replacement) = node(&mut arena);
        let reused_stale = ConId {
            address: replacement.address,
            generation: first.generation,
        };
        assert_eq!(
            arena.floating_insert(parent, reused_stale, false),
            Err(MembershipError::StaleChild)
        );
        drop(arena.take(parent_raw).unwrap());
        assert_eq!(arena.floating_entry(second).unwrap().floating_owner, None);
        assert_eq!(
            arena.floating_insert(parent, second, false),
            Err(MembershipError::StaleParent)
        );
    }

    #[test]
    fn live_children_skip_deleted_successor_see_append_and_support_nested_iteration() {
        let parent_raw = allocate(Con::empty());
        let a_raw = allocate(Con::empty());
        let b_raw = allocate(Con::empty());
        let c_raw = allocate(Con::empty());
        let parent = identity(parent_raw).unwrap();
        let a = identity(a_raw).unwrap();
        let b = identity(b_raw).unwrap();
        let c = identity(c_raw).unwrap();
        append(parent, a).unwrap();
        append(parent, b).unwrap();
        let mut iter = children(parent);
        assert_eq!(iter.next(), Some(a));
        remove(parent, b).unwrap();
        append(parent, c).unwrap();
        assert_eq!(children(parent).collect::<Vec<_>>(), vec![a, c]);
        assert_eq!(iter.next(), Some(c));
        remove(parent, c).unwrap();
        assert_eq!(iter.next(), None);
        append(parent, b).unwrap();
        assert_eq!(iter.next(), None);
        let mut second_iter = children(parent);
        assert_eq!(second_iter.next(), Some(a));
        unsafe {
            release(a_raw);
        }
        assert_eq!(second_iter.next(), None);
        assert_eq!(children(parent).collect::<Vec<_>>(), vec![b]);
        unsafe {
            for raw in [parent_raw, b_raw, c_raw] {
                release(raw);
            }
        }
    }

    #[test]
    fn prefetched_successor_and_requeried_head_drain_survive_current_removal() {
        let parent_raw = allocate(Con::empty());
        let raws: Vec<_> = (0..3).map(|_| allocate(Con::empty())).collect();
        let parent = identity(parent_raw).unwrap();
        let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
        for &id in &ids {
            append(parent, id).unwrap();
        }
        let mut current = first(parent);
        let mut visited = Vec::new();
        while let Some(id) = current {
            let successor = next(parent, id);
            remove(parent, id).unwrap();
            visited.push(id);
            current = successor;
        }
        assert_eq!(visited, ids);
        for &id in &ids {
            append(parent, id).unwrap();
        }
        let mut drained = Vec::new();
        while let Some(id) = first(parent) {
            remove(parent, id).unwrap();
            drained.push(id);
        }
        assert_eq!(drained, ids);
        unsafe {
            for raw in raws {
                release(raw);
            }
            release(parent_raw);
        }
    }

    #[test]
    fn real_raise_moves_selected_to_back_without_rotating_other_members() {
        let parent = allocate(Con::empty());
        let parent_id = identity(parent).unwrap();
        let raws: Vec<_> = (0..3).map(|_| allocate(Con::empty())).collect();
        let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
        for (&raw, &id) in raws.iter().zip(&ids) {
            unsafe {
                set_parent_ptr(raw, parent).expect("live acyclic ancestry");
                (*raw).type_0 = crate::con::CT_FLOATING_CON;
            }
            append(parent_id, id).unwrap();
        }
        unsafe {
            crate::floating::floating_raise_con(raws[1]);
            assert_eq!(
                children(parent_id).collect::<Vec<_>>(),
                vec![ids[0], ids[2], ids[1]]
            );
            crate::floating::floating_raise_con(raws[1]);
            assert_eq!(
                children(parent_id).collect::<Vec<_>>(),
                vec![ids[0], ids[2], ids[1]]
            );
            assert_eq!(parent_ptr(raws[1]), parent);
            for raw in raws {
                release(raw);
            }
            release(parent);
        }
    }

    #[test]
    fn real_attach_and_detach_update_floating_and_focus_but_keep_detached_parent() {
        let parent = allocate(Con::empty());
        let child = allocate(Con::empty());
        let parent_id = identity(parent).unwrap();
        let child_id = identity(child).unwrap();
        unsafe {
            (*parent).type_0 = crate::con::CT_WORKSPACE;
            (*child).type_0 = crate::con::CT_FLOATING_CON;
            crate::con::con_attach(child, parent, false);
            assert_eq!(parent_ptr(child), parent);
            assert_eq!(first(parent_id), Some(child_id));
            assert_eq!(focus::first_ptr(parent), child);
            crate::con::con_detach(child);
            assert!(is_empty(parent_id));
            assert!(focus::first_ptr(parent).is_null());
            assert_eq!(parent_ptr(child), parent);
            assert_eq!(resolve(child_id), Some(child));
            release(child);
            release(parent);
        }
    }
}
