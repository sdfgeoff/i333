//! Owned non-floating child order, independent of ancestry and focus history.
use super::{identity, resolve, Arena, ConId, Entry, CONTAINERS};
use crate::types::Con;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NodesError {
    StaleOwner,
    StaleChild,
    StaleAnchor,
    SelfMembership,
    Cycle,
    AlreadyMember,
    OwnedElsewhere,
    NotMember,
    InvalidOrder,
}
#[derive(Clone, Copy)]
enum Place {
    Front,
    Back,
    Before(ConId),
    After(ConId),
}

impl Arena {
    fn nodes_entry(&self, id: ConId) -> Option<&Entry> {
        self.entries
            .get(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn nodes_entry_mut(&mut self, id: ConId) -> Option<&mut Entry> {
        self.entries
            .get_mut(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn nodes_owner(&self, child: ConId) -> Option<ConId> {
        let owner = self.nodes_entry(child)?.nodes_owner?;
        self.nodes_entry(owner).map(|_| owner)
    }
    fn nodes_position(&self, owner: ConId, child: ConId) -> Result<usize, NodesError> {
        let members = &self.nodes_entry(owner).ok_or(NodesError::StaleOwner)?.nodes;
        let child_entry = self.nodes_entry(child).ok_or(NodesError::StaleChild)?;
        if child_entry.nodes_owner != Some(owner) {
            return Err(NodesError::NotMember);
        }
        members.position(child).ok_or(NodesError::NotMember)
    }
    fn nodes_ancestor(&self, ancestor: ConId, child: ConId) -> bool {
        let mut current = Some(child);
        while let Some(id) = current {
            if id == ancestor {
                return true;
            }
            current = self.nodes_owner(id);
        }
        false
    }
    fn nodes_validate_insert(&self, owner: ConId, child: ConId) -> Result<(), NodesError> {
        self.nodes_entry(owner).ok_or(NodesError::StaleOwner)?;
        let entry = self.nodes_entry(child).ok_or(NodesError::StaleChild)?;
        if owner == child {
            return Err(NodesError::SelfMembership);
        }
        if let Some(previous) = entry.nodes_owner {
            return Err(if previous == owner {
                NodesError::AlreadyMember
            } else {
                NodesError::OwnedElsewhere
            });
        }
        if self.nodes_ancestor(child, owner) {
            return Err(NodesError::Cycle);
        }
        Ok(())
    }
    fn nodes_insertion(&self, owner: ConId, place: Place) -> Result<usize, NodesError> {
        let entry = self.nodes_entry(owner).ok_or(NodesError::StaleOwner)?;
        match place {
            Place::Front => Ok(0),
            Place::Back => Ok(entry.nodes.len()),
            Place::Before(anchor) | Place::After(anchor) => {
                self.nodes_entry(anchor).ok_or(NodesError::StaleAnchor)?;
                let index = self.nodes_position(owner, anchor)?;
                Ok(index + usize::from(matches!(place, Place::After(_))))
            }
        }
    }
    fn nodes_insert(&mut self, owner: ConId, child: ConId, place: Place) -> Result<(), NodesError> {
        self.nodes_validate_insert(owner, child)?;
        let index = self.nodes_insertion(owner, place)?;
        self.nodes_entry_mut(owner)
            .unwrap()
            .nodes
            .insert(index, child);
        self.nodes_entry_mut(child).unwrap().nodes_owner = Some(owner);
        Ok(())
    }
    fn nodes_remove(&mut self, owner: ConId, child: ConId) -> Result<(), NodesError> {
        let index = self.nodes_position(owner, child)?;
        self.nodes_entry_mut(owner).unwrap().nodes.remove(index);
        self.nodes_entry_mut(child).unwrap().nodes_owner = None;
        Ok(())
    }
    fn nodes_move(&mut self, owner: ConId, child: ConId, place: Place) -> Result<(), NodesError> {
        let from = self.nodes_position(owner, child)?;
        let insertion = self.nodes_insertion(owner, place)?;
        if matches!(place,Place::Before(anchor)|Place::After(anchor) if anchor==child) {
            return Ok(());
        }
        self.nodes_entry_mut(owner)
            .unwrap()
            .nodes
            .move_to(from, insertion);
        Ok(())
    }
    fn nodes_replace(&mut self, owner: ConId, old: ConId, new: ConId) -> Result<(), NodesError> {
        let index = self.nodes_position(owner, old)?;
        if old == new {
            return Ok(());
        }
        self.nodes_validate_insert(owner, new)?;
        self.nodes_entry_mut(owner)
            .unwrap()
            .nodes
            .replace(index, new);
        self.nodes_entry_mut(old).unwrap().nodes_owner = None;
        self.nodes_entry_mut(new).unwrap().nodes_owner = Some(owner);
        Ok(())
    }
    fn nodes_swap(&mut self, first: ConId, second: ConId) -> Result<(), NodesError> {
        self.nodes_entry(first).ok_or(NodesError::StaleChild)?;
        self.nodes_entry(second).ok_or(NodesError::StaleChild)?;
        let first_owner = self.nodes_owner(first).ok_or(NodesError::NotMember)?;
        let second_owner = self.nodes_owner(second).ok_or(NodesError::NotMember)?;
        let first_index = self.nodes_position(first_owner, first)?;
        let second_index = self.nodes_position(second_owner, second)?;
        if first == second {
            return Ok(());
        }
        if self.nodes_ancestor(first, second) || self.nodes_ancestor(second, first) {
            return Err(NodesError::Cycle);
        }
        if first_owner == second_owner {
            self.nodes_entry_mut(first_owner)
                .unwrap()
                .nodes
                .swap(first_index, second_index);
        } else {
            self.nodes_entry_mut(first_owner)
                .unwrap()
                .nodes
                .replace(first_index, second);
            self.nodes_entry_mut(second_owner)
                .unwrap()
                .nodes
                .replace(second_index, first);
            self.nodes_entry_mut(first).unwrap().nodes_owner = Some(second_owner);
            self.nodes_entry_mut(second).unwrap().nodes_owner = Some(first_owner);
        }
        Ok(())
    }
    fn nodes_restore(&mut self, owner: ConId, order: &[ConId]) -> Result<(), NodesError> {
        let members = &self.nodes_entry(owner).ok_or(NodesError::StaleOwner)?.nodes;
        for &child in order {
            self.nodes_position(owner, child)?;
        }
        if !members.is_permutation(order) {
            return Err(NodesError::InvalidOrder);
        }
        self.nodes_entry_mut(owner)
            .unwrap()
            .nodes
            .replace_order(order);
        Ok(())
    }
    fn nodes_first(&self, owner: ConId) -> Option<ConId> {
        self.nodes_entry(owner)?.nodes.first()
    }
    fn nodes_last(&self, owner: ConId) -> Option<ConId> {
        self.nodes_entry(owner)?.nodes.last()
    }
    fn nodes_next(&self, owner: ConId, child: ConId, reverse: bool) -> Option<ConId> {
        let index = self.nodes_position(owner, child).ok()?;
        let index = if reverse {
            index.checked_sub(1)?
        } else {
            index + 1
        };
        self.nodes_entry(owner)?.nodes.at(index)
    }
    pub(super) fn nodes_retire(&mut self, id: ConId, owner: Option<ConId>, children: &[ConId]) {
        if let Some(owner) = owner.and_then(|owner| self.nodes_entry_mut(owner)) {
            if let Some(index) = owner.nodes.position(id) {
                owner.nodes.remove(index);
            }
        }
        for &child in children {
            if let Some(child) = self.nodes_entry_mut(child) {
                if child.nodes_owner == Some(id) {
                    child.nodes_owner = None;
                }
            }
        }
    }
}

pub(crate) fn owner(child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().nodes_owner(child))
}
pub(crate) fn first(owner: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().nodes_first(owner))
}
pub(crate) fn last(owner: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().nodes_last(owner))
}
pub(crate) fn next(owner: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().nodes_next(owner, child, false))
}
pub(crate) fn previous(owner: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().nodes_next(owner, child, true))
}
pub(crate) fn is_empty(owner: ConId) -> bool {
    first(owner).is_none()
}
pub(crate) fn len(owner: ConId) -> usize {
    CONTAINERS.with(|arena| {
        arena
            .borrow()
            .nodes_entry(owner)
            .map_or(0, |entry| entry.nodes.len())
    })
}
pub(crate) fn len_ptr(raw: *mut Con) -> usize {
    identity(raw).map_or(0, len)
}
pub(crate) fn snapshot(owner: ConId) -> Vec<ConId> {
    CONTAINERS.with(|arena| {
        arena
            .borrow()
            .nodes_entry(owner)
            .map_or_else(Vec::new, |entry| entry.nodes.as_slice().to_vec())
    })
}
pub(crate) fn append(owner: ConId, child: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_insert(owner, child, Place::Back))
}
pub(crate) fn prepend(owner: ConId, child: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_insert(owner, child, Place::Front))
}
pub(crate) fn insert_before(owner: ConId, child: ConId, anchor: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .nodes_insert(owner, child, Place::Before(anchor))
    })
}
pub(crate) fn insert_after(owner: ConId, child: ConId, anchor: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .nodes_insert(owner, child, Place::After(anchor))
    })
}
pub(crate) fn remove(owner: ConId, child: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_remove(owner, child))
}
pub(crate) fn move_to_front(owner: ConId, child: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_move(owner, child, Place::Front))
}
pub(crate) fn move_to_back(owner: ConId, child: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_move(owner, child, Place::Back))
}
pub(crate) fn move_before(owner: ConId, child: ConId, anchor: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .nodes_move(owner, child, Place::Before(anchor))
    })
}
pub(crate) fn move_after(owner: ConId, child: ConId, anchor: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .nodes_move(owner, child, Place::After(anchor))
    })
}
pub(crate) fn replace(owner: ConId, old: ConId, new: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_replace(owner, old, new))
}
pub(crate) fn swap(first: ConId, second: ConId) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_swap(first, second))
}
pub(crate) fn replace_order(owner: ConId, order: &[ConId]) -> Result<(), NodesError> {
    CONTAINERS.with(|arena| arena.borrow_mut().nodes_restore(owner, order))
}

// Native address adapters perform only validated identity and metadata lookups.
// Returned pointers keep the native caller's lifetime/aliasing contract.
pub(crate) fn first_ptr(raw: *mut Con) -> *mut Con {
    identity(raw)
        .and_then(first)
        .and_then(resolve)
        .unwrap_or(std::ptr::null_mut())
}
pub(crate) fn last_ptr(raw: *mut Con) -> *mut Con {
    identity(raw)
        .and_then(last)
        .and_then(resolve)
        .unwrap_or(std::ptr::null_mut())
}
pub(crate) fn next_ptr(raw: *mut Con) -> *mut Con {
    identity(raw)
        .and_then(|id| owner(id).and_then(|parent| next(parent, id)))
        .and_then(resolve)
        .unwrap_or(std::ptr::null_mut())
}
pub(crate) fn previous_ptr(raw: *mut Con) -> *mut Con {
    identity(raw)
        .and_then(|id| owner(id).and_then(|parent| previous(parent, id)))
        .and_then(resolve)
        .unwrap_or(std::ptr::null_mut())
}

pub(crate) struct NodeChildren {
    owner: ConId,
    current: Option<ConId>,
    reverse: bool,
    finished: bool,
}
impl Iterator for NodeChildren {
    type Item = ConId;
    fn next(&mut self) -> Option<ConId> {
        if self.finished {
            return None;
        }
        let id = match (self.current, self.reverse) {
            (None, false) => first(self.owner),
            (None, true) => last(self.owner),
            (Some(id), false) => next(self.owner, id),
            (Some(id), true) => previous(self.owner, id),
        };
        self.current = id;
        self.finished = id.is_none();
        id
    }
}
impl std::iter::FusedIterator for NodeChildren {}
pub(crate) fn children(owner: ConId) -> NodeChildren {
    NodeChildren {
        owner,
        current: None,
        reverse: false,
        finished: false,
    }
}
pub(crate) fn children_rev(owner: ConId) -> NodeChildren {
    NodeChildren {
        owner,
        current: None,
        reverse: true,
        finished: false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::con::arena::{allocate, parent::set_parent_ptr, release};
    fn node(arena: &mut Arena) -> (*mut Con, ConId) {
        let raw = arena.insert(Con::empty());
        (raw, arena.identity(raw).unwrap())
    }
    fn order(arena: &Arena, owner: ConId) -> Vec<ConId> {
        arena.nodes_entry(owner).unwrap().nodes.as_slice().to_vec()
    }
    fn state(arena: &Arena) -> Vec<(u64, Vec<ConId>, Option<ConId>)> {
        arena
            .order
            .iter()
            .map(|(&generation, address)| {
                let e = &arena.entries[address];
                (generation, e.nodes.as_slice().to_vec(), e.nodes_owner)
            })
            .collect()
    }

    #[test]
    fn relative_insert_and_move_preserve_every_other_members_position() {
        let mut arena = Arena::default();
        let (_, p) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        let (_, d) = node(&mut arena);
        arena.nodes_insert(p, b, Place::Back).unwrap();
        arena.nodes_insert(p, a, Place::Front).unwrap();
        arena.nodes_insert(p, c, Place::After(b)).unwrap();
        arena.nodes_insert(p, d, Place::Before(b)).unwrap();
        assert_eq!(order(&arena, p), vec![a, d, b, c]);
        arena.nodes_move(p, a, Place::After(b)).unwrap();
        assert_eq!(order(&arena, p), vec![d, b, a, c]);
        arena.nodes_move(p, c, Place::Before(d)).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, b, a]);
        arena.nodes_move(p, b, Place::Front).unwrap();
        assert_eq!(order(&arena, p), vec![b, c, d, a]);
        arena.nodes_move(p, b, Place::Back).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a, b]);
        arena.nodes_move(p, b, Place::After(b)).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a, b]);
        assert_eq!(arena.nodes_next(p, d, false), Some(a));
        assert_eq!(arena.nodes_next(p, d, true), Some(c));
        assert_eq!(arena.nodes_next(p, c, true), None);
        assert_eq!(arena.nodes_next(p, b, false), None);
    }

    #[test]
    fn invalid_membership_order_and_cycle_updates_are_transactional() {
        let mut arena = Arena::default();
        let (_, p) = node(&mut arena);
        let (_, q) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        arena.nodes_insert(p, a, Place::Back).unwrap();
        arena.nodes_insert(a, b, Place::Back).unwrap();
        let stale = ConId {
            generation: c.generation + 100,
            ..c
        };
        let before = state(&arena);
        assert_eq!(
            arena.nodes_insert(p, a, Place::Back),
            Err(NodesError::AlreadyMember)
        );
        assert_eq!(
            arena.nodes_insert(q, a, Place::Back),
            Err(NodesError::OwnedElsewhere)
        );
        assert_eq!(
            arena.nodes_insert(q, q, Place::Back),
            Err(NodesError::SelfMembership)
        );
        assert_eq!(
            arena.nodes_insert(b, p, Place::Back),
            Err(NodesError::Cycle)
        );
        assert_eq!(
            arena.nodes_insert(p, c, Place::After(stale)),
            Err(NodesError::StaleAnchor)
        );
        assert_eq!(
            arena.nodes_move(p, a, Place::Before(c)),
            Err(NodesError::NotMember)
        );
        assert_eq!(arena.nodes_remove(q, a), Err(NodesError::NotMember));
        assert_eq!(
            arena.nodes_restore(p, &[a, a]),
            Err(NodesError::InvalidOrder)
        );
        assert_eq!(arena.nodes_restore(p, &[]), Err(NodesError::InvalidOrder));
        assert_eq!(arena.nodes_swap(a, b), Err(NodesError::Cycle));
        assert_eq!(arena.nodes_swap(a, stale), Err(NodesError::StaleChild));
        assert_eq!(
            arena.nodes_insert(stale, c, Place::Back),
            Err(NodesError::StaleOwner)
        );
        assert_eq!(
            arena.nodes_insert(p, stale, Place::Back),
            Err(NodesError::StaleChild)
        );
        assert_eq!(
            arena.nodes_insert(p, c, Place::Before(b)),
            Err(NodesError::NotMember)
        );
        assert_eq!(
            arena.nodes_replace(p, a, b),
            Err(NodesError::OwnedElsewhere)
        );
        assert_eq!(
            arena.nodes_restore(p, &[stale]),
            Err(NodesError::StaleChild)
        );
        assert_eq!(arena.nodes_swap(a, c), Err(NodesError::NotMember));
        assert_eq!(state(&arena), before);
    }

    #[test]
    fn cross_owner_swap_and_wrapper_replacement_keep_exact_original_slots() {
        let mut arena = Arena::default();
        let (_, p) = node(&mut arena);
        let (_, q) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        let (_, d) = node(&mut arena);
        let (_, wrapper) = node(&mut arena);
        for id in [a, b, c] {
            arena.nodes_insert(p, id, Place::Back).unwrap();
        }
        arena.nodes_insert(q, d, Place::Back).unwrap();
        arena.nodes_swap(b, d).unwrap();
        assert_eq!(order(&arena, p), vec![a, d, c]);
        assert_eq!(order(&arena, q), vec![b]);
        assert_eq!(arena.nodes_owner(b), Some(q));
        assert_eq!(arena.nodes_owner(d), Some(p));
        arena.nodes_swap(a, c).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a]);
        arena.nodes_replace(p, d, wrapper).unwrap();
        assert_eq!(order(&arena, p), vec![c, wrapper, a]);
        assert_eq!(arena.nodes_owner(d), None);
        arena.nodes_insert(wrapper, d, Place::Back).unwrap();
        assert_eq!(order(&arena, wrapper), vec![d]);
        arena.nodes_restore(p, &[a, wrapper, c]).unwrap();
        assert_eq!(order(&arena, p), vec![a, wrapper, c]);
    }

    #[test]
    fn live_forward_reverse_nested_and_prefetched_traversal_handle_removal() {
        let p_raw = allocate(Con::empty());
        let p = identity(p_raw).unwrap();
        let raws: Vec<_> = (0..4).map(|_| allocate(Con::empty())).collect();
        let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
        for &id in &ids[..3] {
            append(p, id).unwrap();
        }
        let mut forward = children(p);
        assert_eq!(forward.next(), Some(ids[0]));
        remove(p, ids[1]).unwrap();
        append(p, ids[3]).unwrap();
        assert_eq!(
            children_rev(p).collect::<Vec<_>>(),
            vec![ids[3], ids[2], ids[0]]
        );
        assert_eq!(forward.next(), Some(ids[2]));
        assert_eq!(forward.next(), Some(ids[3]));
        assert_eq!(forward.next(), None);
        append(p, ids[1]).unwrap();
        assert_eq!(forward.next(), None);
        let mut reverse = children_rev(p);
        assert_eq!(reverse.next(), Some(ids[1]));
        remove(p, ids[2]).unwrap();
        assert_eq!(reverse.next(), Some(ids[3]));
        remove(p, ids[3]).unwrap();
        assert_eq!(reverse.next(), None);
        let mut current = first(p);
        let mut visited = Vec::new();
        while let Some(id) = current {
            let after = next(p, id);
            remove(p, id).unwrap();
            visited.push(id);
            current = after;
        }
        assert_eq!(visited, vec![ids[0], ids[1]]);
        unsafe {
            for raw in raws {
                release(raw);
            }
            release(p_raw);
        }
    }

    #[test]
    fn retirement_and_stale_generation_never_leave_rediscoverable_membership() {
        let mut arena = Arena::default();
        let (p_raw, p) = node(&mut arena);
        let (a_raw, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        arena.nodes_insert(p, a, Place::Back).unwrap();
        arena.nodes_insert(p, b, Place::Back).unwrap();
        drop(arena.take(a_raw).unwrap());
        assert_eq!(order(&arena, p), vec![b]);
        let (_, replacement) = node(&mut arena);
        let stale = ConId {
            address: replacement.address,
            generation: a.generation,
        };
        let before = state(&arena);
        assert_eq!(
            arena.nodes_insert(p, stale, Place::Back),
            Err(NodesError::StaleChild)
        );
        assert_eq!(state(&arena), before);
        drop(arena.take(p_raw).unwrap());
        assert_eq!(arena.nodes_owner(b), None);
    }

    #[test]
    fn safe_address_queries_never_dereference_arbitrary_native_pointers() {
        for address in [0, 1, usize::MAX] {
            let raw = std::ptr::without_provenance_mut::<Con>(address);
            assert!(first_ptr(raw).is_null());
            assert!(last_ptr(raw).is_null());
            assert!(next_ptr(raw).is_null());
            assert!(previous_ptr(raw).is_null());
        }
    }

    #[test]
    fn actual_workspace_attach_keeps_signed_numeric_stable_ties_and_named_tail() {
        let raw_parent = allocate(Con::empty());
        let p = identity(raw_parent).unwrap();
        let values = [2, -1, 1, 2, -1, -2];
        let mut raws = Vec::new();
        unsafe {
            (*raw_parent).type_0 = crate::con::CT_CON;
            for number in values {
                let raw = allocate(Con::empty());
                (*raw).type_0 = crate::con::CT_WORKSPACE;
                (*raw).num = number;
                crate::con::con_attach(raw, raw_parent, false);
                raws.push(raw);
            }
            let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
            assert_eq!(
                snapshot(p),
                vec![ids[5], ids[2], ids[0], ids[3], ids[1], ids[4]]
            );
            assert_eq!(super::super::focus::snapshot(p), ids);
            assert_eq!(crate::con::con_num_children(raw_parent), 6);
            for raw in raws {
                crate::con::con_detach(raw);
                release(raw);
            }
            assert!(crate::con::con_is_leaf(raw_parent));
            release(raw_parent);
        }
    }

    #[test]
    fn actual_dock_attach_uses_stable_nullable_native_class_then_instance_order() {
        use std::ffi::CString;
        let raw_parent = allocate(Con::empty());
        let p = identity(raw_parent).unwrap();
        let classes = [
            (Some("Beta"), Some("b")),
            (None, None),
            (Some("alpha"), Some("z")),
            (Some("ALPHA"), Some("A")),
            (Some("Alpha"), Some("a")),
        ];
        let text: Vec<_> = classes
            .iter()
            .map(|(class, instance)| {
                (
                    class.map(|s| CString::new(s).unwrap()),
                    instance.map(|s| CString::new(s).unwrap()),
                )
            })
            .collect();
        let mut windows = Vec::new();
        let mut raws = Vec::new();
        unsafe {
            (*raw_parent).type_0 = crate::con::CT_DOCKAREA;
            for (class, instance) in &text {
                let mut window = Box::new(crate::types::Window::default());
                window.class_class = class
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |text| text.as_ptr().cast_mut());
                window.class_instance = instance
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |text| text.as_ptr().cast_mut());
                let raw = allocate(Con::empty());
                (*raw).type_0 = crate::con::CT_CON;
                (*raw).window = &raw mut *window;
                crate::con::con_attach(raw, raw_parent, false);
                raws.push(raw);
                windows.push(window);
            }
            let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
            assert_eq!(snapshot(p), vec![ids[1], ids[3], ids[4], ids[2], ids[0]]);
            assert_eq!(super::super::focus::snapshot(p), ids);
            for raw in raws {
                crate::con::con_detach(raw);
                release(raw);
            }
            release(raw_parent);
        }
        // These fixture Windows borrow CString class views; ordinary Box drop
        // releases their Rust fields, never calls the native window_free API.
        drop(windows);
        drop(text);
    }

    #[test]
    fn wrapper_slot_replacement_and_actual_attach_detach_preserve_all_relations() {
        let raw_parent = allocate(Con::empty());
        let child = allocate(Con::empty());
        let wrapper = allocate(Con::empty());
        let p = identity(raw_parent).unwrap();
        let child_id = identity(child).unwrap();
        let wrapper_id = identity(wrapper).unwrap();
        unsafe {
            (*raw_parent).type_0 = crate::con::CT_WORKSPACE;
            (*child).type_0 = crate::con::CT_CON;
            (*wrapper).type_0 = crate::con::CT_CON;
            crate::con::con_attach(child, raw_parent, true);
            replace(p, child_id, wrapper_id).unwrap();
            super::super::focus::replace(p, child_id, wrapper_id).unwrap();
            set_parent_ptr(wrapper, raw_parent).unwrap();
            crate::con::con_attach(child, wrapper, true);
            assert_eq!(first(p), Some(wrapper_id));
            assert_eq!(first(wrapper_id), Some(child_id));
            assert_eq!(super::super::parent::parent(child_id), Some(wrapper_id));
            assert_eq!(super::super::focus::first(p), Some(wrapper_id));
            assert_eq!(super::super::focus::first(wrapper_id), Some(child_id));
            crate::con::con_detach(child);
            assert!(is_empty(wrapper_id));
            assert!(super::super::focus::is_empty(wrapper_id));
            assert_eq!(super::super::parent::parent(child_id), Some(wrapper_id));
            crate::con::con_detach(wrapper);
            assert!(is_empty(p));
            release(child);
            release(wrapper);
            release(raw_parent);
        }
    }

    #[test]
    fn ordinary_attach_honors_remembered_focus_and_explicit_previous_anchor() {
        let raw_parent = allocate(Con::empty());
        let p = identity(raw_parent).unwrap();
        let raws: Vec<_> = (0..4).map(|_| allocate(Con::empty())).collect();
        let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
        unsafe {
            (*raw_parent).type_0 = crate::con::CT_WORKSPACE;
            for &raw in &raws {
                (*raw).type_0 = crate::con::CT_CON;
            }
            crate::con::con_attach(raws[0], raw_parent, false);
            crate::con::con_attach(raws[1], raw_parent, false);
            super::super::focus::move_to_front(p, ids[0]).unwrap();
            crate::con::con_attach(raws[2], raw_parent, false);
            assert_eq!(snapshot(p), vec![ids[0], ids[2], ids[1]]);
            crate::con::_con_attach(raws[3], raw_parent, raws[2], true);
            assert_eq!(snapshot(p), vec![ids[0], ids[2], ids[3], ids[1]]);
            assert_eq!(super::super::focus::snapshot(p), ids);
            for raw in raws {
                crate::con::con_detach(raw);
                release(raw);
            }
            release(raw_parent);
        }
    }

    #[test]
    fn safe_existence_and_iterative_ancestry_queries_handle_unknown_and_deep_graphs() {
        let mut raws = Vec::new();
        for _ in 0..1024 {
            let raw = allocate(Con::empty());
            if let Some(&parent) = raws.last() {
                set_parent_ptr(raw, parent).unwrap();
            }
            raws.push(raw);
        }
        let first = raws[0];
        let last = *raws.last().unwrap();
        assert!(crate::con::con_exists(first));
        assert!(crate::con::con_has_parent(last, first));
        assert!(!crate::con::con_has_parent(first, last));
        assert!(!crate::con::con_has_parent(first, first));
        for address in [0, 1, usize::MAX] {
            let invalid = std::ptr::without_provenance_mut::<Con>(address);
            assert!(!crate::con::con_exists(invalid));
            assert!(!crate::con::con_has_parent(invalid, first));
            assert!(!crate::con::con_has_parent(last, invalid));
            assert_eq!(crate::con::con_num_children(invalid), 0);
            assert_eq!(len_ptr(invalid), 0);
        }
        unsafe {
            for raw in raws {
                release(raw);
            }
        }
        assert!(!crate::con::con_exists(first));
    }
}
