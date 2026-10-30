//! Stable Rust ownership for containers on the window-manager event thread.
//!
//! Existing graph links and IPC IDs borrow addresses. Internal deferred work can
//! instead capture a generation-checked identity, preventing a reused address
//! from naming a different container. Registry borrows never cross callbacks.
use crate::types::Con;
use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConId {
    address: usize,
    generation: u64,
}

struct Entry {
    generation: u64,
    // Mutation is performed by the event-thread graph functions through raw
    // links. UnsafeCell avoids deriving mutable storage from a shared Con borrow.
    storage: Box<UnsafeCell<Con>>,
}

#[derive(Default)]
struct Arena {
    entries: HashMap<usize, Entry>,
    generation: u64,
}

impl Arena {
    fn insert(&mut self, con: Con) -> *mut Con {
        self.generation = self
            .generation
            .checked_add(1)
            .expect("container ID exhausted");
        let storage = Box::new(UnsafeCell::new(con));
        let raw = storage.get();
        // SAFETY: The allocation is stable and unpublished. Queue tails point
        // into that allocation and remain valid until its owner is removed.
        unsafe {
            (*raw).floating_head.tqh_last = &raw mut (*raw).floating_head.tqh_first;
            (*raw).nodes_head.tqh_last = &raw mut (*raw).nodes_head.tqh_first;
            (*raw).focus_head.tqh_last = &raw mut (*raw).focus_head.tqh_first;
            (*raw).swallow_head.tqh_last = &raw mut (*raw).swallow_head.tqh_first;
            (*raw).marks_head.tqh_last = &raw mut (*raw).marks_head.tqh_first;
        }
        let previous = self.entries.insert(
            raw as usize,
            Entry {
                generation: self.generation,
                storage,
            },
        );
        assert!(previous.is_none(), "duplicate live container address");
        raw
    }

    fn identity(&self, raw: *mut Con) -> Option<ConId> {
        let address = raw as usize;
        self.entries.get(&address).map(|entry| ConId {
            address,
            generation: entry.generation,
        })
    }

    fn resolve(&self, id: ConId) -> Option<*mut Con> {
        self.entries
            .get(&id.address)
            .filter(|entry| entry.generation == id.generation)
            .map(|entry| entry.storage.get())
    }

    fn take(&mut self, raw: *mut Con) -> Option<Box<UnsafeCell<Con>>> {
        self.entries
            .remove(&(raw as usize))
            .map(|entry| entry.storage)
    }
}

thread_local! {
    static CONTAINERS: RefCell<Arena> = RefCell::new(Arena::default());
}

pub(super) fn allocate(con: Con) -> *mut Con {
    CONTAINERS.with(|arena| arena.borrow_mut().insert(con))
}

pub(crate) fn identity(raw: *mut Con) -> Option<ConId> {
    CONTAINERS.with(|arena| arena.borrow().identity(raw))
}

/// Resolve an identity without borrowing its container. Callers still must obey
/// the event-thread graph lifetime contract before dereferencing the raw link.
pub(crate) fn resolve(id: ConId) -> Option<*mut Con> {
    CONTAINERS.with(|arena| arena.borrow().resolve(id))
}

pub(super) fn lookup_address(address: usize) -> *mut Con {
    CONTAINERS.with(|arena| {
        arena
            .borrow()
            .entries
            .get(&address)
            .map_or(ptr::null_mut(), |entry| entry.storage.get())
    })
}

/// # Safety
/// The container was allocated here; all external graph memberships have been
/// unlinked and native resources released. No callback may use its address again.
pub(super) unsafe fn release(raw: *mut Con) {
    let storage = CONTAINERS
        .with(|arena| arena.borrow_mut().take(raw))
        .expect("container ownership released twice or outside its arena");
    // Drop outside the registry borrow so future owned fields can safely run
    // destructors that themselves interact with the event-thread registry.
    drop(storage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_storage_survives_registry_growth_and_self_links_remain_valid() {
        let mut arena = Arena::default();
        let first = arena.insert(Con::empty());
        let id = arena.identity(first).unwrap();
        unsafe {
            (*first).num = 42;
        }
        for _ in 0..1024 {
            arena.insert(Con::empty());
        }
        assert_eq!(arena.resolve(id), Some(first));
        unsafe {
            assert_eq!((*first).num, 42);
            assert_eq!(
                (*first).nodes_head.tqh_last,
                &raw mut (*first).nodes_head.tqh_first
            );
            assert_eq!(
                (*first).focus_head.tqh_last,
                &raw mut (*first).focus_head.tqh_first
            );
            assert_eq!(
                (*first).marks_head.tqh_last,
                &raw mut (*first).marks_head.tqh_first
            );
        }
    }

    #[test]
    fn removing_ownership_invalidates_ids_even_if_the_address_is_reused() {
        let mut arena = Arena::default();
        let first = arena.insert(Con::empty());
        let stale = arena.identity(first).unwrap();
        drop(arena.take(first).unwrap());
        assert!(arena.resolve(stale).is_none());
        assert!(arena.take(first).is_none());
        let second = arena.insert(Con::empty());
        let current = arena.identity(second).unwrap();
        assert_ne!(stale.generation, current.generation);
        // Exercise address reuse deterministically, independent of the allocator.
        let reused_stale = ConId {
            address: current.address,
            generation: stale.generation,
        };
        assert!(arena.resolve(reused_stale).is_none());
        assert_eq!(arena.resolve(current), Some(second));
    }

    #[test]
    fn detaching_and_deleting_children_repairs_tiling_and_focus_memberships() {
        let mut arena = Arena::default();
        let parent = arena.insert(Con::empty());
        let first = arena.insert(Con::empty());
        let second = arena.insert(Con::empty());
        let first_id = arena.identity(first).unwrap();
        let second_id = arena.identity(second).unwrap();
        unsafe {
            // A workspace stops redraw propagation without any X connection.
            (*parent).type_0 = super::super::CT_WORKSPACE;
            (*first).parent = parent;
            (*second).parent = parent;
            (*parent).nodes_head.tqh_first = first;
            (*parent).nodes_head.tqh_last = &raw mut (*second).nodes.tqe_next;
            (*first).nodes.tqe_prev = &raw mut (*parent).nodes_head.tqh_first;
            (*first).nodes.tqe_next = second;
            (*second).nodes.tqe_prev = &raw mut (*first).nodes.tqe_next;
            (*parent).focus_head.tqh_first = second;
            (*parent).focus_head.tqh_last = &raw mut (*first).focused.tqe_next;
            (*second).focused.tqe_prev = &raw mut (*parent).focus_head.tqh_first;
            (*second).focused.tqe_next = first;
            (*first).focused.tqe_prev = &raw mut (*second).focused.tqe_next;
            super::super::con_detach(first);
            drop(arena.take(first).unwrap());
            assert!(arena.resolve(first_id).is_none());
            assert_eq!((*parent).nodes_head.tqh_first, second);
            assert_eq!(
                (*second).nodes.tqe_prev,
                &raw mut (*parent).nodes_head.tqh_first
            );
            assert_eq!(
                (*parent).focus_head.tqh_last,
                &raw mut (*second).focused.tqe_next
            );
            super::super::con_detach(second);
            drop(arena.take(second).unwrap());
            assert!(arena.resolve(second_id).is_none());
            assert!((*parent).nodes_head.tqh_first.is_null());
            assert!((*parent).focus_head.tqh_first.is_null());
            assert_eq!(
                (*parent).nodes_head.tqh_last,
                &raw mut (*parent).nodes_head.tqh_first
            );
            assert_eq!(
                (*parent).focus_head.tqh_last,
                &raw mut (*parent).focus_head.tqh_first
            );
        }
    }

    #[test]
    fn removed_owner_drops_after_registry_borrow_and_allows_reentrant_work() {
        let first = allocate(Con::empty());
        let stale = identity(first).unwrap();
        let owner = CONTAINERS
            .with(|arena| arena.borrow_mut().take(first))
            .unwrap();
        // A callback during teardown can allocate/query independently; no RefCell
        // borrow is held over that callback or the removed owner's destructor.
        let callback = || {
            assert!(resolve(stale).is_none());
            let next = allocate(Con::empty());
            let id = identity(next).unwrap();
            assert_eq!(resolve(id), Some(next));
            unsafe {
                release(next);
            }
        };
        callback();
        drop(owner);
    }
}
