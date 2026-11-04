//! Stable Rust ownership for containers on the window-manager event thread.
//!
//! Existing graph links and IPC IDs borrow addresses. Internal deferred work can
//! instead capture a generation-checked identity, preventing a reused address
//! from naming a different container. Registry borrows never cross callbacks.
use crate::domains::ContainerType;
pub(crate) mod floating;
pub(crate) mod focus;
pub(crate) mod nodes;
mod ordered;
pub(crate) mod parent;

use crate::types::{Con, Window};
use std::cell::{RefCell, UnsafeCell};
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
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
    floating: Vec<ConId>,
    floating_owner: Option<ConId>,
    parent: Option<ConId>,
    focus: ordered::OrderedIds,
    focus_owner: Option<ConId>,
    nodes: ordered::OrderedIds,
    nodes_owner: Option<ConId>,
}

#[derive(Default)]
struct Arena {
    entries: HashMap<usize, Entry>,
    order: BTreeMap<u64, usize>,
    generation: u64,
    // Only manager-created Windows are owned here. Con.window remains a
    // borrowed stable address, including externally owned fixture windows.
    // Keep this last: rendering/container records drop before Window payloads.
    windows: HashMap<usize, Box<UnsafeCell<Window>>>,
}

// Surface owners drop with each container, before C atexit releases display
// and visual owners. Their destructors never consult the TLS arena registry.

impl Arena {
    fn insert_window(&mut self, window: Window) -> *mut Window {
        let storage = Box::new(UnsafeCell::new(window));
        let raw = storage.get();
        let previous = self.windows.insert(raw as usize, storage);
        assert!(previous.is_none(), "duplicate live window address");
        raw
    }

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
            (*raw).swallow_head.tqh_last = &raw mut (*raw).swallow_head.tqh_first;
            (*raw).marks_head.tqh_last = &raw mut (*raw).marks_head.tqh_first;
        }
        let previous = self.entries.insert(
            raw as usize,
            Entry {
                generation: self.generation,
                storage,
                floating: Vec::new(),
                floating_owner: None,
                parent: None,
                focus: ordered::OrderedIds::default(),
                focus_owner: None,
                nodes: ordered::OrderedIds::default(),
                nodes_owner: None,
            },
        );
        assert!(previous.is_none(), "duplicate live container address");
        self.order.insert(self.generation, raw as usize);
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

    fn next_after(&self, generation: u64) -> Option<ConId> {
        self.order
            .range((Excluded(generation), Unbounded))
            .next()
            .map(|(&generation, &address)| ConId {
                address,
                generation,
            })
    }

    fn take(&mut self, raw: *mut Con) -> Option<Box<UnsafeCell<Con>>> {
        let entry = self.entries.remove(&(raw as usize))?;
        self.order.remove(&entry.generation);
        let id = ConId {
            address: raw as usize,
            generation: entry.generation,
        };
        self.floating_retire(id, entry.floating_owner, &entry.floating);
        self.focus_retire(id, entry.focus_owner, entry.focus.as_slice());
        self.nodes_retire(id, entry.nodes_owner, entry.nodes.as_slice());
        Some(entry.storage)
    }
}

thread_local! {
    static CONTAINERS: RefCell<Arena> = RefCell::new(Arena::default());
}

/// A live creation-order traversal with no registry borrow across its body.
/// Newly appended containers are visible before exhaustion; removed entries are
/// skipped. After the first `None`, traversal remains exhausted.
#[derive(Default)]
pub(crate) struct Containers {
    generation: u64,
    finished: bool,
}
impl Iterator for Containers {
    type Item = ConId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = CONTAINERS.with(|arena| arena.borrow().next_after(self.generation));
        match next {
            Some(id) => self.generation = id.generation,
            None => self.finished = true,
        }
        next
    }
}
impl std::iter::FusedIterator for Containers {}

pub(crate) fn containers() -> Containers {
    Containers::default()
}

/// Publish a fully constructed manager-owned Window with a stable address.
pub(crate) fn allocate_window(window: Window) -> *mut Window {
    CONTAINERS.with(|arena| arena.borrow_mut().insert_window(window))
}

/// Remove the manager's unique owner without keeping a registry borrow through
/// payload destruction. A foreign/unknown/already-released pointer is rejected;
/// external fixture boxes must be dropped by their own owner instead.
pub(crate) fn take_window(raw: *mut Window) -> Box<UnsafeCell<Window>> {
    CONTAINERS
        .with(|arena| arena.borrow_mut().windows.remove(&(raw as usize)))
        .expect("managed window ownership released twice or foreign window pointer")
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

/// Remove graph-wide membership and retain ownership while native resources
/// are cleaned up. The caller must finish using all raw links before dropping
/// this owner. No registry borrow is held while that cleanup runs.
pub(super) fn take_owner(raw: *mut Con) -> Box<UnsafeCell<Con>> {
    CONTAINERS
        .with(|arena| arena.borrow_mut().take(raw))
        .expect("container ownership released twice or outside its arena")
}

/// # Safety
/// The container was allocated here; all external graph memberships have been
/// unlinked and native resources released. No callback may use its address again.
#[cfg(test)]
pub(super) unsafe fn release(raw: *mut Con) {
    drop(take_owner(raw));
}

#[cfg(test)]
mod tests {
    use super::parent::{parent_ptr, set_parent_ptr};
    use super::*;

    #[test]
    fn participant_search_returns_neighbors_and_retains_failed_ancestor() {
        use crate::con::arena::parent::set_parent_ptr;
        use crate::domains::ContainerType;
        use crate::resize::resize_find_tiling_participants;
        use crate::types::{Direction, Layout};

        let mut workspace_value = Con::empty();
        workspace_value.type_0 = ContainerType::Workspace;
        workspace_value.layout = Layout::SplitH;
        let workspace = allocate(workspace_value);
        let mut child_value = Con::empty();
        child_value.type_0 = ContainerType::Con;
        let first = allocate(child_value);
        let mut child_value = Con::empty();
        child_value.type_0 = ContainerType::Con;
        let second = allocate(child_value);
        for child in [first, second] {
            set_parent_ptr(child, workspace).unwrap();
            nodes::append(identity(workspace).unwrap(), identity(child).unwrap()).unwrap();
        }
        // SAFETY: the fixture owns every live node on this test's event thread;
        // searching needs only ancestry and sibling order, with no X11 access.
        unsafe {
            assert_eq!(
                resize_find_tiling_participants(first, Direction::Right, false),
                Ok((first, second)),
            );
            assert_eq!(
                resize_find_tiling_participants(first, Direction::Left, true),
                Ok((first, second)),
            );
            // The failed search still yields its final workspace ancestor,
            // matching the old output argument used by resize_set_tiling.
            assert_eq!(
                resize_find_tiling_participants(first, Direction::Left, false),
                Err(workspace),
            );
            assert_eq!(
                resize_find_tiling_participants(workspace, Direction::Left, false),
                Err(workspace),
            );
            assert_eq!(
                resize_find_tiling_participants(std::ptr::null_mut(), Direction::Left, false),
                Err(std::ptr::null_mut()),
            );
            release(first);
            release(second);
            release(workspace);
        }
    }

    #[test]
    fn managed_windows_have_stable_owners_and_release_payloads_at_arena_drop() {
        unsafe {
            let image = cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, 1, 1);
            let observer = cairo_sys::cairo_surface_reference(image);
            let mut assignment = Box::new(crate::types::Assignment::new(
                crate::types::AssignmentDest::Command(b"nop".to_vec()),
                crate::types::Match::empty(),
            ));
            let assignment_ptr = std::ptr::from_mut(&mut *assignment);
            let mut window = Window {
                class_class: Some(b"\xffClass".to_vec()),
                ..Default::default()
            };
            window.ran_assignments.push(assignment_ptr);
            window.icon = Some(i3_common::draw_util::CairoSurface::from_raw_owned(image));
            let mut arena = Arena::default();
            let raw = arena.insert_window(window);
            for _ in 0..256 {
                arena.insert_window(Window::default());
            }
            assert_eq!((*raw).class_class.as_deref(), Some(&b"\xffClass"[..]));
            assert_eq!((*raw).ran_assignments, [assignment_ptr]);
            drop(arena);
            // Window Drop reclaimed its icon; only our observer remains. The
            // assignment identity was borrowed, so its separate owner survives.
            assert_eq!(cairo_sys::cairo_surface_get_reference_count(observer), 1);
            cairo_sys::cairo_surface_destroy(observer);
            assert_eq!(assignment.kind(), crate::flags::AssignmentKind::COMMAND);
        }
    }

    #[test]
    fn removing_managed_window_owner_ends_registry_borrow_before_drop() {
        let raw = allocate_window(Window::default());
        let owner = take_window(raw);
        CONTAINERS.with(|arena| assert!(arena.try_borrow_mut().is_ok()));
        drop(owner);
        let foreign = Box::new(Window::default());
        let foreign_address = std::ptr::from_ref(&*foreign) as usize;
        assert!(std::panic::catch_unwind(|| take_window(foreign_address as *mut Window)).is_err());
        assert!(foreign.class_class.is_none());
    }

    #[test]
    fn arena_drop_releases_both_graphics_records_before_container_storage() {
        unsafe fn image_frame() -> (
            i3_common::draw_util::surface_t,
            *mut cairo_sys::cairo_surface_t,
            *mut cairo_sys::cairo_t,
        ) {
            let image = cairo_sys::cairo_image_surface_create(cairo_sys::FORMAT_A_RGB32, 1, 1);
            let context = cairo_sys::cairo_create(image);
            assert_eq!(cairo_sys::cairo_status(context), cairo_sys::STATUS_SUCCESS);
            let observer_image = cairo_sys::cairo_surface_reference(image);
            let observer_context = cairo_sys::cairo_reference(context);
            (
                i3_common::draw_util::surface_t::from_raw_cairo(0, 1, 1, image, context),
                observer_image,
                observer_context,
            )
        }
        unsafe {
            let (frame, frame_image, frame_context) = image_frame();
            let (buffer, buffer_image, buffer_context) = image_frame();
            let observers = [(frame_image, frame_context), (buffer_image, buffer_context)].map(
                |(image, context)| {
                    (
                        image,
                        context,
                        cairo_sys::cairo_surface_get_reference_count(image),
                    )
                },
            );
            let mut arena = Arena::default();
            let mut con = Con::empty();
            con.frame = frame;
            con.frame_buffer = buffer;
            arena.insert(con);
            drop(arena);
            for (image, context, original_count) in observers {
                assert_eq!(cairo_sys::cairo_get_reference_count(context), 1);
                // Cairo may retain multiple target references inside a context;
                // the arena must release its independent surface reference.
                assert_eq!(
                    cairo_sys::cairo_surface_get_reference_count(image),
                    original_count - 1
                );
                cairo_sys::cairo_destroy(context);
                assert_eq!(cairo_sys::cairo_surface_get_reference_count(image), 1);
                cairo_sys::cairo_surface_destroy(image);
            }
        }
    }

    #[test]
    fn live_iterator_preserves_creation_order_through_removal_and_append() {
        let first = allocate(Con::empty());
        let second = allocate(Con::empty());
        let third = allocate(Con::empty());
        let first_id = identity(first).unwrap();
        let second_id = identity(second).unwrap();
        let third_id = identity(third).unwrap();
        let mut iter = containers();
        assert_eq!(iter.next(), Some(first_id));
        // Removing both the current and next container never dereferences the
        // deleted current link and must not lose the surviving later entry.
        unsafe {
            release(first);
            release(second);
        }
        let appended = allocate(Con::empty());
        let appended_id = identity(appended).unwrap();
        assert!(resolve(first_id).is_none());
        assert!(resolve(second_id).is_none());
        assert_eq!(iter.next(), Some(third_id));
        // A callback may start an independent traversal while the outer cursor
        // is suspended, with neither traversal holding an arena borrow.
        assert_eq!(
            containers().collect::<Vec<_>>(),
            vec![third_id, appended_id]
        );
        unsafe {
            release(third);
        }
        assert_eq!(iter.next(), Some(appended_id));
        unsafe {
            release(appended);
        }
        assert_eq!(iter.next(), None);
        let after_exhaustion = allocate(Con::empty());
        assert_eq!(iter.next(), None);
        assert_eq!(
            containers().collect::<Vec<_>>(),
            vec![identity(after_exhaustion).unwrap()]
        );
        unsafe {
            release(after_exhaustion);
        }
    }

    #[test]
    fn retiring_owner_invalidates_traversal_and_identity_before_storage_drops() {
        let raw = allocate(Con::empty());
        let id = identity(raw).unwrap();
        let owner = take_owner(raw);
        assert!(resolve(id).is_none());
        assert!(identity(raw).is_none());
        assert!(!containers().any(|candidate| candidate == id));
        // Native teardown still has valid stable storage, but callback lookups
        // cannot rediscover an already retired container.
        unsafe {
            (*raw).num = 91;
            assert_eq!((*owner.get()).num, 91);
        }
        let replacement = allocate(Con::empty());
        assert_ne!(raw, replacement);
        unsafe {
            release(replacement);
        }
        drop(owner);
    }

    #[test]
    fn production_lookups_choose_first_created_live_match() {
        let mut first_window = Box::new(crate::types::Window::default());
        let mut second_window = Box::new(crate::types::Window::default());
        first_window.id = 0x123;
        second_window.id = 0x123;
        let mut first_value = Con::empty();
        first_value.frame.id = 0x456;
        first_value.window = &raw mut *first_window;
        let first = allocate(first_value);
        let mut second_value = Con::empty();
        second_value.frame.id = 0x456;
        second_value.window = &raw mut *second_window;
        let second = allocate(second_value);
        unsafe {
            assert_eq!(super::super::con_by_window_id(0x123), first);
            assert_eq!(super::super::con_by_frame_id(0x456), first);
            assert_eq!(super::super::con_by_con_id(first as i64), first);
            release(first);
            assert!(super::super::con_by_con_id(first as i64).is_null());
            assert_eq!(super::super::con_by_window_id(0x123), second);
            assert_eq!(super::super::con_by_frame_id(0x456), second);
            release(second);
            assert!(super::super::con_by_window_id(0x123).is_null());
        }
    }

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
        let parent = allocate(Con::empty());
        let first = allocate(Con::empty());
        let second = allocate(Con::empty());
        let first_id = identity(first).unwrap();
        let second_id = identity(second).unwrap();
        unsafe {
            // A workspace stops redraw propagation without any X connection.
            (*parent).type_0 = crate::domains::ContainerType::Workspace;
            set_parent_ptr(first, parent).expect("live acyclic ancestry");
            set_parent_ptr(second, parent).expect("live acyclic ancestry");
            nodes::append(identity(parent).unwrap(), first_id).unwrap();
            nodes::append(identity(parent).unwrap(), second_id).unwrap();
            focus::append(identity(parent).unwrap(), second_id).unwrap();
            focus::append(identity(parent).unwrap(), first_id).unwrap();
            super::super::con_detach(first);
            release(first);
            assert!(resolve(first_id).is_none());
            assert_eq!(nodes::first_ptr(parent), second);
            assert!(nodes::previous_ptr(second).is_null());
            assert_eq!(focus::last_ptr(parent), second);
            super::super::con_detach(second);
            release(second);
            assert!(resolve(second_id).is_none());
            assert!(nodes::first_ptr(parent).is_null());
            assert!(focus::first_ptr(parent).is_null());
            assert!(nodes::last_ptr(parent).is_null());
            assert!(focus::last_ptr(parent).is_null());
            release(parent);
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
