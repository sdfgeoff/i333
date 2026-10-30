//! Owned focus history independent of ancestry and tiling/floating membership.
use super::{identity, resolve, Arena, ConId, Entry, CONTAINERS};
use crate::types::Con;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FocusError {
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
    fn focus_entry(&self, id: ConId) -> Option<&Entry> {
        self.entries
            .get(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn focus_entry_mut(&mut self, id: ConId) -> Option<&mut Entry> {
        self.entries
            .get_mut(&id.address)
            .filter(|entry| entry.generation == id.generation)
    }
    fn focus_owner(&self, child: ConId) -> Option<ConId> {
        let owner = self.focus_entry(child)?.focus_owner?;
        self.focus_entry(owner).map(|_| owner)
    }
    fn focus_position(&self, owner: ConId, child: ConId) -> Result<usize, FocusError> {
        let members = &self.focus_entry(owner).ok_or(FocusError::StaleOwner)?.focus;
        let child_entry = self.focus_entry(child).ok_or(FocusError::StaleChild)?;
        if child_entry.focus_owner != Some(owner) {
            return Err(FocusError::NotMember);
        }
        members.position(child).ok_or(FocusError::NotMember)
    }
    fn focus_ancestor(&self, ancestor: ConId, child: ConId) -> bool {
        let mut current = Some(child);
        while let Some(id) = current {
            if id == ancestor {
                return true;
            }
            current = self.focus_owner(id);
        }
        false
    }
    fn focus_validate_insert(&self, owner: ConId, child: ConId) -> Result<(), FocusError> {
        self.focus_entry(owner).ok_or(FocusError::StaleOwner)?;
        let entry = self.focus_entry(child).ok_or(FocusError::StaleChild)?;
        if owner == child {
            return Err(FocusError::SelfMembership);
        }
        if let Some(previous) = entry.focus_owner {
            return Err(if previous == owner {
                FocusError::AlreadyMember
            } else {
                FocusError::OwnedElsewhere
            });
        }
        if self.focus_ancestor(child, owner) {
            return Err(FocusError::Cycle);
        }
        Ok(())
    }
    fn focus_insertion(&self, owner: ConId, place: Place) -> Result<usize, FocusError> {
        let entry = self.focus_entry(owner).ok_or(FocusError::StaleOwner)?;
        match place {
            Place::Front => Ok(0),
            Place::Back => Ok(entry.focus.len()),
            Place::Before(anchor) | Place::After(anchor) => {
                self.focus_entry(anchor).ok_or(FocusError::StaleAnchor)?;
                let index = self.focus_position(owner, anchor)?;
                Ok(index + usize::from(matches!(place, Place::After(_))))
            }
        }
    }
    fn focus_insert(&mut self, owner: ConId, child: ConId, place: Place) -> Result<(), FocusError> {
        self.focus_validate_insert(owner, child)?;
        let index = self.focus_insertion(owner, place)?;
        self.focus_entry_mut(owner)
            .unwrap()
            .focus
            .insert(index, child);
        self.focus_entry_mut(child).unwrap().focus_owner = Some(owner);
        Ok(())
    }
    fn focus_remove(&mut self, owner: ConId, child: ConId) -> Result<(), FocusError> {
        let index = self.focus_position(owner, child)?;
        self.focus_entry_mut(owner).unwrap().focus.remove(index);
        self.focus_entry_mut(child).unwrap().focus_owner = None;
        Ok(())
    }
    fn focus_move(&mut self, owner: ConId, child: ConId, place: Place) -> Result<(), FocusError> {
        let from = self.focus_position(owner, child)?;
        let insertion = self.focus_insertion(owner, place)?;
        if matches!(place,Place::Before(anchor)|Place::After(anchor) if anchor==child) {
            return Ok(());
        }
        self.focus_entry_mut(owner)
            .unwrap()
            .focus
            .move_to(from, insertion);
        Ok(())
    }
    fn focus_replace(&mut self, owner: ConId, old: ConId, new: ConId) -> Result<(), FocusError> {
        let index = self.focus_position(owner, old)?;
        if old == new {
            return Ok(());
        }
        self.focus_validate_insert(owner, new)?;
        self.focus_entry_mut(owner)
            .unwrap()
            .focus
            .replace(index, new);
        self.focus_entry_mut(old).unwrap().focus_owner = None;
        self.focus_entry_mut(new).unwrap().focus_owner = Some(owner);
        Ok(())
    }
    fn focus_swap(&mut self, first: ConId, second: ConId) -> Result<(), FocusError> {
        self.focus_entry(first).ok_or(FocusError::StaleChild)?;
        self.focus_entry(second).ok_or(FocusError::StaleChild)?;
        let first_owner = self.focus_owner(first).ok_or(FocusError::NotMember)?;
        let second_owner = self.focus_owner(second).ok_or(FocusError::NotMember)?;
        let first_index = self.focus_position(first_owner, first)?;
        let second_index = self.focus_position(second_owner, second)?;
        if first == second {
            return Ok(());
        }
        if self.focus_ancestor(first, second) || self.focus_ancestor(second, first) {
            return Err(FocusError::Cycle);
        }
        if first_owner == second_owner {
            self.focus_entry_mut(first_owner)
                .unwrap()
                .focus
                .swap(first_index, second_index);
        } else {
            self.focus_entry_mut(first_owner)
                .unwrap()
                .focus
                .replace(first_index, second);
            self.focus_entry_mut(second_owner)
                .unwrap()
                .focus
                .replace(second_index, first);
            self.focus_entry_mut(first).unwrap().focus_owner = Some(second_owner);
            self.focus_entry_mut(second).unwrap().focus_owner = Some(first_owner);
        }
        Ok(())
    }
    fn focus_restore(&mut self, owner: ConId, order: &[ConId]) -> Result<(), FocusError> {
        let members = &self.focus_entry(owner).ok_or(FocusError::StaleOwner)?.focus;
        for &child in order {
            self.focus_position(owner, child)?;
        }
        if !members.is_permutation(order) {
            return Err(FocusError::InvalidOrder);
        }
        self.focus_entry_mut(owner)
            .unwrap()
            .focus
            .replace_order(order);
        Ok(())
    }
    fn focus_first(&self, owner: ConId) -> Option<ConId> {
        self.focus_entry(owner)?.focus.first()
    }
    fn focus_last(&self, owner: ConId) -> Option<ConId> {
        self.focus_entry(owner)?.focus.last()
    }
    fn focus_next(&self, owner: ConId, child: ConId, reverse: bool) -> Option<ConId> {
        let index = self.focus_position(owner, child).ok()?;
        let index = if reverse {
            index.checked_sub(1)?
        } else {
            index + 1
        };
        self.focus_entry(owner)?.focus.at(index)
    }
    pub(super) fn focus_retire(&mut self, id: ConId, owner: Option<ConId>, children: &[ConId]) {
        if let Some(owner) = owner.and_then(|owner| self.focus_entry_mut(owner)) {
            if let Some(index) = owner.focus.position(id) {
                owner.focus.remove(index);
            }
        }
        for &child in children {
            if let Some(child) = self.focus_entry_mut(child) {
                if child.focus_owner == Some(id) {
                    child.focus_owner = None;
                }
            }
        }
    }
}

pub(crate) fn owner(child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().focus_owner(child))
}
pub(crate) fn first(owner: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().focus_first(owner))
}
pub(crate) fn last(owner: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().focus_last(owner))
}
pub(crate) fn next(owner: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().focus_next(owner, child, false))
}
pub(crate) fn previous(owner: ConId, child: ConId) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().focus_next(owner, child, true))
}
pub(crate) fn is_empty(owner: ConId) -> bool {
    first(owner).is_none()
}
pub(crate) fn snapshot(owner: ConId) -> Vec<ConId> {
    CONTAINERS.with(|arena| {
        arena
            .borrow()
            .focus_entry(owner)
            .map_or_else(Vec::new, |entry| entry.focus.as_slice().to_vec())
    })
}
pub(crate) fn append(owner: ConId, child: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_insert(owner, child, Place::Back))
}
pub(crate) fn prepend(owner: ConId, child: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_insert(owner, child, Place::Front))
}
pub(crate) fn insert_before(owner: ConId, child: ConId, anchor: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .focus_insert(owner, child, Place::Before(anchor))
    })
}
pub(crate) fn insert_after(owner: ConId, child: ConId, anchor: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .focus_insert(owner, child, Place::After(anchor))
    })
}
pub(crate) fn remove(owner: ConId, child: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_remove(owner, child))
}
pub(crate) fn move_to_front(owner: ConId, child: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_move(owner, child, Place::Front))
}
pub(crate) fn move_to_back(owner: ConId, child: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_move(owner, child, Place::Back))
}
pub(crate) fn move_before(owner: ConId, child: ConId, anchor: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .focus_move(owner, child, Place::Before(anchor))
    })
}
pub(crate) fn move_after(owner: ConId, child: ConId, anchor: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| {
        arena
            .borrow_mut()
            .focus_move(owner, child, Place::After(anchor))
    })
}
pub(crate) fn replace(owner: ConId, old: ConId, new: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_replace(owner, old, new))
}
pub(crate) fn swap(first: ConId, second: ConId) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_swap(first, second))
}
pub(crate) fn replace_order(owner: ConId, order: &[ConId]) -> Result<(), FocusError> {
    CONTAINERS.with(|arena| arena.borrow_mut().focus_restore(owner, order))
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

pub(crate) struct FocusChildren {
    owner: ConId,
    current: Option<ConId>,
    reverse: bool,
    finished: bool,
}
impl Iterator for FocusChildren {
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
impl std::iter::FusedIterator for FocusChildren {}
pub(crate) fn children(owner: ConId) -> FocusChildren {
    FocusChildren {
        owner,
        current: None,
        reverse: false,
        finished: false,
    }
}
pub(crate) fn children_rev(owner: ConId) -> FocusChildren {
    FocusChildren {
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
        arena.focus_entry(owner).unwrap().focus.as_slice().to_vec()
    }
    fn state(arena: &Arena) -> Vec<(u64, Vec<ConId>, Option<ConId>)> {
        arena
            .order
            .iter()
            .map(|(&generation, address)| {
                let e = &arena.entries[address];
                (generation, e.focus.as_slice().to_vec(), e.focus_owner)
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
        arena.focus_insert(p, b, Place::Back).unwrap();
        arena.focus_insert(p, a, Place::Front).unwrap();
        arena.focus_insert(p, c, Place::After(b)).unwrap();
        arena.focus_insert(p, d, Place::Before(b)).unwrap();
        assert_eq!(order(&arena, p), vec![a, d, b, c]);
        arena.focus_move(p, a, Place::After(b)).unwrap();
        assert_eq!(order(&arena, p), vec![d, b, a, c]);
        arena.focus_move(p, c, Place::Before(d)).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, b, a]);
        arena.focus_move(p, b, Place::Front).unwrap();
        assert_eq!(order(&arena, p), vec![b, c, d, a]);
        arena.focus_move(p, b, Place::Back).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a, b]);
        arena.focus_move(p, b, Place::After(b)).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a, b]);
        assert_eq!(arena.focus_next(p, d, false), Some(a));
        assert_eq!(arena.focus_next(p, d, true), Some(c));
        assert_eq!(arena.focus_next(p, c, true), None);
        assert_eq!(arena.focus_next(p, b, false), None);
    }

    #[test]
    fn invalid_membership_order_and_cycle_updates_are_transactional() {
        let mut arena = Arena::default();
        let (_, p) = node(&mut arena);
        let (_, q) = node(&mut arena);
        let (_, a) = node(&mut arena);
        let (_, b) = node(&mut arena);
        let (_, c) = node(&mut arena);
        arena.focus_insert(p, a, Place::Back).unwrap();
        arena.focus_insert(a, b, Place::Back).unwrap();
        let stale = ConId {
            generation: c.generation + 100,
            ..c
        };
        let before = state(&arena);
        assert_eq!(
            arena.focus_insert(p, a, Place::Back),
            Err(FocusError::AlreadyMember)
        );
        assert_eq!(
            arena.focus_insert(q, a, Place::Back),
            Err(FocusError::OwnedElsewhere)
        );
        assert_eq!(
            arena.focus_insert(q, q, Place::Back),
            Err(FocusError::SelfMembership)
        );
        assert_eq!(
            arena.focus_insert(b, p, Place::Back),
            Err(FocusError::Cycle)
        );
        assert_eq!(
            arena.focus_insert(p, c, Place::After(stale)),
            Err(FocusError::StaleAnchor)
        );
        assert_eq!(
            arena.focus_move(p, a, Place::Before(c)),
            Err(FocusError::NotMember)
        );
        assert_eq!(arena.focus_remove(q, a), Err(FocusError::NotMember));
        assert_eq!(
            arena.focus_restore(p, &[a, a]),
            Err(FocusError::InvalidOrder)
        );
        assert_eq!(arena.focus_restore(p, &[]), Err(FocusError::InvalidOrder));
        assert_eq!(arena.focus_swap(a, b), Err(FocusError::Cycle));
        assert_eq!(arena.focus_swap(a, stale), Err(FocusError::StaleChild));
        assert_eq!(
            arena.focus_insert(stale, c, Place::Back),
            Err(FocusError::StaleOwner)
        );
        assert_eq!(
            arena.focus_insert(p, stale, Place::Back),
            Err(FocusError::StaleChild)
        );
        assert_eq!(
            arena.focus_insert(p, c, Place::Before(b)),
            Err(FocusError::NotMember)
        );
        assert_eq!(
            arena.focus_replace(p, a, b),
            Err(FocusError::OwnedElsewhere)
        );
        assert_eq!(
            arena.focus_restore(p, &[stale]),
            Err(FocusError::StaleChild)
        );
        assert_eq!(arena.focus_swap(a, c), Err(FocusError::NotMember));
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
            arena.focus_insert(p, id, Place::Back).unwrap();
        }
        arena.focus_insert(q, d, Place::Back).unwrap();
        arena.focus_swap(b, d).unwrap();
        assert_eq!(order(&arena, p), vec![a, d, c]);
        assert_eq!(order(&arena, q), vec![b]);
        assert_eq!(arena.focus_owner(b), Some(q));
        assert_eq!(arena.focus_owner(d), Some(p));
        arena.focus_swap(a, c).unwrap();
        assert_eq!(order(&arena, p), vec![c, d, a]);
        arena.focus_replace(p, d, wrapper).unwrap();
        assert_eq!(order(&arena, p), vec![c, wrapper, a]);
        assert_eq!(arena.focus_owner(d), None);
        arena.focus_insert(wrapper, d, Place::Back).unwrap();
        assert_eq!(order(&arena, wrapper), vec![d]);
        arena.focus_restore(p, &[a, wrapper, c]).unwrap();
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
        arena.focus_insert(p, a, Place::Back).unwrap();
        arena.focus_insert(p, b, Place::Back).unwrap();
        drop(arena.take(a_raw).unwrap());
        assert_eq!(order(&arena, p), vec![b]);
        let (_, replacement) = node(&mut arena);
        let stale = ConId {
            address: replacement.address,
            generation: a.generation,
        };
        let before = state(&arena);
        assert_eq!(
            arena.focus_insert(p, stale, Place::Back),
            Err(FocusError::StaleChild)
        );
        assert_eq!(state(&arena), before);
        drop(arena.take(p_raw).unwrap());
        assert_eq!(arena.focus_owner(b), None);
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
    fn real_attach_detach_preserves_independent_floating_focus_and_parent() {
        let p_raw = allocate(Con::empty());
        let child_raw = allocate(Con::empty());
        let p = identity(p_raw).unwrap();
        let child = identity(child_raw).unwrap();
        unsafe {
            (*p_raw).type_0 = crate::con::CT_WORKSPACE;
            (*child_raw).type_0 = crate::con::CT_FLOATING_CON;
            crate::con::con_attach(child_raw, p_raw, false);
            assert_eq!(first(p), Some(child));
            assert_eq!(super::super::floating::first(p), Some(child));
            crate::con::con_detach(child_raw);
            assert!(is_empty(p));
            assert!(super::super::floating::is_empty(p));
            assert_eq!(super::super::parent::parent(child), Some(p));
            release(child_raw);
            release(p_raw);
        }
    }

    #[test]
    fn actual_focus_propagation_runs_only_in_an_isolated_displayless_test_process() {
        const MARKER: &str = "I333_FOCUS_PROPAGATION_CHILD";
        if std::env::var_os(MARKER).is_none() {
            let mut command = std::process::Command::new(std::env::current_exe().unwrap());
            command.args(["--exact","con::arena::focus::tests::actual_focus_propagation_runs_only_in_an_isolated_displayless_test_process","--test-threads=1"]).env(MARKER,"1");
            for key in [
                "DISPLAY",
                "I3SOCK",
                "XAUTHORITY",
                "WAYLAND_DISPLAY",
                "DBUS_SESSION_BUS_ADDRESS",
                "NOTIFY_SOCKET",
                "LISTEN_FDS",
                "LISTEN_PID",
                "LISTEN_FDNAMES",
            ] {
                command.env_remove(key);
            }
            let result = command.output().unwrap();
            assert!(
                result.status.success(),
                "{}{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
            return;
        }
        let mut raws: Vec<_> = (0..5).map(|_| allocate(Con::empty())).collect();
        let ids: Vec<_> = raws.iter().map(|&raw| identity(raw).unwrap()).collect();
        for (child, parent) in [(1, 0), (2, 1), (3, 2), (4, 2)] {
            set_parent_ptr(raws[child], raws[parent]).unwrap();
            append(ids[parent], ids[child]).unwrap();
        }
        unsafe {
            (*raws[0]).type_0 = crate::con::CT_ROOT;
            (*raws[1]).type_0 = crate::con::CT_OUTPUT;
            (*raws[2]).type_0 = crate::con::CT_WORKSPACE;
            (*raws[3]).type_0 = crate::con::CT_CON;
            (*raws[4]).type_0 = crate::con::CT_CON;
            crate::con::con_focus(raws[4]);
            assert_eq!(first(ids[2]), Some(ids[4]));
            assert_eq!(first(ids[1]), Some(ids[2]));
            assert_eq!(first(ids[0]), Some(ids[1]));
            assert_eq!(crate::tree::focused, raws[4]);
            crate::tree::focused = std::ptr::null_mut();
            for raw in raws {
                release(raw);
            }
        }
    }
}
