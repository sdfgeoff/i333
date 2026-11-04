use crate::con::arena::nodes;
use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{focus, ConId};
use crate::domains::{ContainerType, FullscreenMode, HideEdgeBordersMode, MatchInsertWhere};
use crate::flags::{AdjacentEdges, TilingDrag};
pub(crate) mod arena;
use crate::types::BorderStyle;
use crate::types::Direction;
use crate::types::FloatingState;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::MarkMode;
use crate::types::Orientation;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * con.c: Functions which deal with containers directly (creating containers,
 *        searching containers, getting specific properties from containers,
 *        …).
 *
 */
// Mechanical file-by-file Rust translation of src/con.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type Output = xoutput;
pub type i3Window = Window;

pub type adjacent_t = AdjacentEdges;

pub type tiling_drag_t = TilingDrag;

pub type ylength = usize;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_WORKSPACE: ::core::ffi::c_ulong =
    I3_IPC_EVENT_MASK | 0 as ::core::ffi::c_ulong;

pub unsafe fn con_force_split_parents_redraw(con: *mut Con) {
    let mut parent: *mut Con = con;
    while !parent.is_null()
        && ((*parent).type_0) != ContainerType::Workspace
        && ((*parent).type_0) != ContainerType::DockArea
    {
        if !con_is_leaf(parent) {
            free((*parent).deco_render_params as *mut ::core::ffi::c_void);
            (*parent).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        parent = parent_ptr(parent);
    }
}

pub unsafe fn con_new_skeleton(parent: *mut Con, window: *mut i3Window) -> *mut Con {
    let new = arena::allocate(Con::empty());
    (*new).on_remove_child =
        Some(con_on_remove_child as unsafe fn(*mut Con) -> ()) as Option<unsafe fn(*mut Con) -> ()>;
    (*new).type_0 = ContainerType::Con;
    (*new).window = window;
    (*new).max_user_border_style = config.default_border;
    (*new).border_style = (*new).max_user_border_style;
    (*new).current_border_width = -(1 as ::core::ffi::c_int);
    (*new).window_icon_padding = -(1 as ::core::ffi::c_int);
    if !window.is_null() {
        (*new).depth = (*window).depth;
    } else {
        (*new).depth = root_depth as u16;
    }
    ::log::debug!("../src/con.c:con_new_skeleton:52 - opening window\n");
    if !parent.is_null() {
        con_attach(new, parent, false);
    }
    new
}

pub unsafe fn con_new(parent: *mut Con, window: *mut i3Window) -> *mut Con {
    let new: *mut Con = con_new_skeleton(parent, window);
    x_con_init(new);
    new
}

pub unsafe fn con_free(con: *mut Con) {
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    // Retire the identity before cleanup; the local owner keeps storage live
    // until all native resources below have been released.
    let owner = arena::take_owner(con);
    while !(*con).swallow_head.tqh_first.is_null() {
        let match_0: *mut Match = (*con).swallow_head.tqh_first;
        if !(*match_0).matches.tqe_next.is_null() {
            (*(*match_0).matches.tqe_next).matches.tqe_prev = (*match_0).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
        }
        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
        drop(Box::from_raw(match_0));
    }
    while !(*con).marks_head.tqh_first.is_null() {
        let mut mark: *mut mark_t = (*con).marks_head.tqh_first;
        if !(*mark).marks.tqe_next.is_null() {
            (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
        } else {
            (*con).marks_head.tqh_last = (*mark).marks.tqe_prev;
        }
        *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
        free((*mark).name as *mut ::core::ffi::c_void);
        (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(mark as *mut ::core::ffi::c_void);
        mark = ::core::ptr::null_mut::<mark_t>();
    }
    ::log::debug!(
        "../src/con.c:con_free:96 - con {} freed\n",
        i3_common::logging::pointer(con)
    );
    // All graph links and native resources are released before Rust ownership.
    drop(owner);
}
unsafe fn _con_attach(con: *mut Con, parent: *mut Con, previous: *mut Con, ignore_focus: bool) {
    set_parent_ptr(con, parent).expect("live acyclic ancestry");
    let mut loop_0: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = previous;
    let mut node_owner = arena::identity(parent).expect("live child owner");
    let child_id = arena::identity(con).expect("live child");
    let mut focus_parent = parent;
    if ((*con).type_0) == ContainerType::Workspace {
        ::log::debug!(
            "../src/con.c:_con_attach:110 - it's a workspace. num = {}\n",
            { ((*con).num) }
        );
        current = nodes::first(node_owner)
            .and_then(arena::resolve)
            .unwrap_or(std::ptr::null_mut());
        if (*con).num == -1 || current.is_null() {
            nodes::append(node_owner, child_id).expect("detached workspace");
        } else if (*con).num < (*current).num {
            nodes::prepend(node_owner, child_id).expect("detached workspace");
        } else {
            while (*current).num != -1 && (*con).num >= (*current).num {
                current = nodes::next_ptr(current);
                if current.is_null() {
                    break;
                }
            }
            if current.is_null() {
                nodes::append(node_owner, child_id).expect("detached workspace");
            } else {
                nodes::insert_before(
                    node_owner,
                    child_id,
                    arena::identity(current).expect("live workspace anchor"),
                )
                .expect("detached workspace");
            }
        }
    } else if ((*parent).type_0) == ContainerType::DockArea {
        current = ::core::ptr::null_mut::<Con>();
        loop_0 = nodes::first(node_owner)
            .and_then(arena::resolve)
            .unwrap_or(std::ptr::null_mut());
        while !loop_0.is_null() {
            let mut result: ::core::ffi::c_int = strcasecmp_nullable(
                (*(*con).window).class_class_ptr(),
                (*(*loop_0).window).class_class_ptr(),
            );
            if result == 0 as ::core::ffi::c_int {
                result = strcasecmp_nullable(
                    (*(*con).window).class_instance_ptr(),
                    (*(*loop_0).window).class_instance_ptr(),
                );
            }
            if result < 0 as ::core::ffi::c_int {
                current = loop_0;
                break;
            } else {
                loop_0 = nodes::next_ptr(loop_0);
            }
        }
        if !current.is_null() {
            nodes::insert_before(
                node_owner,
                child_id,
                arena::identity(current).expect("live dock anchor"),
            )
            .expect("detached dock child");
        } else {
            nodes::append(node_owner, child_id).expect("detached dock child");
        }
    } else if ((*con).type_0) == ContainerType::Floating {
        ::log::debug!("../src/con.c:_con_attach:162 - Inserting into floating containers\n");
        arena::floating::append(
            arena::identity(parent).expect("live floating parent"),
            arena::identity(con).expect("live floating child"),
        )
        .expect("detached floating child");
    } else {
        if !ignore_focus {
            loop_0 = focus::first_ptr(parent);
            while !loop_0.is_null() {
                if ((*loop_0).type_0) == ContainerType::Floating {
                    loop_0 = focus::next_ptr(loop_0);
                } else {
                    current = loop_0;
                    break;
                }
            }
        }
        if !(*con).window.is_null()
            && ((*parent).type_0) == ContainerType::Workspace
            && (*parent).workspace_layout != Layout::Default
        {
            ::log::debug!("../src/con.c:_con_attach:186 - Parent is a workspace. Applying default layout...\n");
            let target: *mut Con = workspace_attach_to(parent);
            node_owner = arena::identity(target).expect("live workspace split");
            focus_parent = target;
            set_parent_ptr(con, target).expect("live acyclic ancestry");
            current = ::core::ptr::null_mut::<Con>();
            ::log::debug!("../src/con.c:_con_attach:195 - done\n");
        }
        if !current.is_null() && ((*parent).type_0) != ContainerType::Output {
            ::log::debug!(
                "../src/con.c:_con_attach:201 - Inserting con = {} after con {}\n",
                i3_common::logging::pointer(con),
                i3_common::logging::pointer(current)
            );
            nodes::insert_after(
                node_owner,
                child_id,
                arena::identity(current).expect("live insertion anchor"),
            )
            .expect("detached child");
        } else {
            nodes::append(node_owner, child_id).expect("detached child");
        }
    }
    focus::append(
        arena::identity(focus_parent).expect("live focus parent"),
        arena::identity(con).expect("live focus child"),
    )
    .expect("detached focus child");
    con_force_split_parents_redraw(con);
}

pub unsafe fn con_attach(con: *mut Con, parent: *mut Con, ignore_focus: bool) {
    _con_attach(con, parent, ::core::ptr::null_mut::<Con>(), ignore_focus);
}

pub unsafe fn con_detach(con: *mut Con) {
    con_force_split_parents_redraw(con);
    if ((*con).type_0) == ContainerType::Floating {
        arena::floating::remove(
            arena::identity(parent_ptr(con)).expect("live floating parent"),
            arena::identity(con).expect("live floating child"),
        )
        .expect("attached floating child");
    } else {
        nodes::remove(
            arena::identity(parent_ptr(con)).expect("live child owner"),
            arena::identity(con).expect("live child"),
        )
        .expect("attached child");
    };
    focus::remove(
        arena::identity(parent_ptr(con)).expect("live focus parent"),
        arena::identity(con).expect("live focus child"),
    )
    .expect("attached focus child");
}

pub unsafe fn con_focus(con: *mut Con) {
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            251 as ::core::ffi::c_uint,
            c"void con_focus(Con *)".as_ptr(),
        );
    };
    ::log::debug!(
        "../src/con.c:con_focus:252 - con_focus = {}\n",
        i3_common::logging::pointer(con)
    );
    focus::move_to_front(
        arena::identity(parent_ptr(con)).expect("live focus parent"),
        arena::identity(con).expect("live focus child"),
    )
    .expect("attached focus child");
    if !parent_ptr(parent_ptr(con)).is_null() {
        con_focus(parent_ptr(con));
    }
    focused = con;
    if (*con).urgent && con_is_leaf(con) {
        con_set_urgency(con, false);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
        ipc_send_window_event(c"urgent".as_ptr(), con);
    }
}
unsafe fn con_raise(con: *mut Con) {
    let floating: *mut Con = con_inside_floating(con);
    if !floating.is_null() {
        floating_raise_con(floating);
    }
}

pub unsafe fn con_activate(con: *mut Con) {
    con_focus(con);
    con_raise(con);
}

pub unsafe fn con_activate_unblock(con: *mut Con) {
    let ws: *mut Con = con_get_workspace(con);
    let previous_focus: *mut Con = focused;
    let fullscreen_on_ws: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fullscreen_on_ws.is_null()
        && fullscreen_on_ws != con
        && !con_has_parent(con, fullscreen_on_ws)
    {
        con_disable_fullscreen(fullscreen_on_ws);
    }
    con_activate(con);
    if ws != con_get_workspace(previous_focus) {
        con_activate(previous_focus);
        workspace_show(ws);
        con_activate(con);
    }
}

pub unsafe fn con_close(con: *mut Con, kill_window: KillWindow) {
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            337 as ::core::ffi::c_uint,
            c"void con_close(Con *, kill_window_t)".as_ptr(),
        );
    };
    ::log::debug!(
        "../src/con.c:con_close:338 - Closing con = {}.\n",
        i3_common::logging::pointer(con)
    );
    if ((*con).type_0) == ContainerType::Output || ((*con).type_0) == ContainerType::Root {
        ::log::debug!(
            "../src/con.c:con_close:342 - con = {} is of type {}, not closing anything.\n",
            i3_common::logging::pointer(con),
            (((*con).type_0).raw() as ::core::ffi::c_uint) as i32
        );
        return;
    }
    if ((*con).type_0) == ContainerType::Workspace {
        ::log::debug!(
            "../src/con.c:con_close:347 - con = {} is a workspace, closing all children instead.\n",
            i3_common::logging::pointer(con)
        );
        let owner = arena::identity(con).expect("live closing workspace");
        let mut child_id = focus::first(owner);
        while let Some(id) = child_id {
            let next_child = focus::next(owner, id);
            let Some(child) = arena::resolve(id) else {
                break;
            };
            ::log::debug!(
                "../src/con.c:con_close:350 - killing child = {}.\n",
                i3_common::logging::pointer(child)
            );
            tree_close_internal(child, kill_window, false);
            child_id = next_child;
        }
        return;
    }
    tree_close_internal(con, kill_window, false);
}

pub fn con_is_leaf(con: *mut Con) -> bool {
    nodes::first_ptr(con).is_null()
}

pub unsafe fn con_has_managed_window(con: *mut Con) -> bool {
    !con.is_null()
        && !(*con).window.is_null()
        && (*(*con).window).id != 0 as ::core::ffi::c_int as u32
        && !con_get_workspace(con).is_null()
}

pub unsafe fn con_has_children(con: *mut Con) -> bool {
    !con_is_leaf(con) || arena::identity(con).is_some_and(|id| !arena::floating::is_empty(id))
}

pub unsafe fn con_is_split(con: *mut Con) -> bool {
    if con_is_leaf(con) {
        return false;
    }
    match (*con).layout {
        Layout::DockArea | Layout::Output => false,
        Layout::Default | Layout::Stacked | Layout::Tabbed | Layout::SplitV | Layout::SplitH => {
            true
        }
    }
}

pub unsafe fn con_is_hidden(con: *mut Con) -> bool {
    let mut current: *mut Con = con;
    while !current.is_null() && ((*current).type_0) != ContainerType::Workspace {
        let parent: *mut Con = parent_ptr(current);
        if !parent.is_null()
            && ((*parent).layout == Layout::Tabbed || (*parent).layout == Layout::Stacked)
            && focus::first_ptr(parent) != current
        {
            return true;
        }
        current = parent;
    }
    false
}

pub unsafe fn con_is_maximized(con: *mut Con, orientation: Orientation) -> bool {
    if ((*con).fullscreen_mode) != FullscreenMode::None {
        return true;
    }
    let mut layout: Layout = Layout::Default;
    match orientation {
        Orientation::Horizontal => {
            layout = Layout::SplitH;
        }
        Orientation::Vertical => {
            layout = Layout::SplitV;
        }
        Orientation::None => {
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                458 as ::core::ffi::c_uint,
                c"_Bool con_is_maximized(Con *, orientation_t)".as_ptr(),
            );
        }
    }
    let mut current: *mut Con = con;
    loop {
        let parent: *mut Con = parent_ptr(current);
        if parent.is_null() || ((*current).type_0) == ContainerType::Workspace {
            return true;
        }
        if (*parent).layout == layout && con_num_children(parent) > 1 as ::core::ffi::c_int {
            return false;
        }
        if ((*parent).type_0) == ContainerType::Floating {
            return false;
        }
        current = parent;
    }
}

pub unsafe fn con_is_sticky(con: *mut Con) -> bool {
    if (*con).sticky {
        return true;
    }
    let mut child: *mut Con = nodes::first_ptr(con);

    while !child.is_null() {
        if con_is_sticky(child) {
            return true;
        }
        child = nodes::next_ptr(child);
    }
    false
}

pub unsafe fn con_accepts_window(con: *mut Con) -> bool {
    if ((*con).type_0) == ContainerType::Workspace {
        return false;
    }
    if con_is_split(con) {
        ::log::debug!("../src/con.c:con_accepts_window:518 - container {} does not accept windows, it is a split container.\n", i3_common::logging::pointer(con));
        return false;
    }
    (*con).window.is_null()
}

pub unsafe fn con_get_output(con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null() && ((*result).type_0) != ContainerType::Output {
        result = parent_ptr(result);
    }
    if !result.is_null() {
    } else {
        __assert_fail(
            c"result != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            538 as ::core::ffi::c_uint,
            c"Con *con_get_output(Con *)".as_ptr(),
        );
    };
    result
}

pub unsafe fn con_get_workspace(con: *mut Con) -> *mut Con {
    let mut result: *mut Con = con;
    while !result.is_null() && ((*result).type_0) != ContainerType::Workspace {
        result = parent_ptr(result);
    }
    result
}

pub unsafe fn con_parent_with_orientation(con: *mut Con, orientation: Orientation) -> *mut Con {
    ::log::debug!("../src/con.c:con_parent_with_orientation:560 - Searching for parent of Con {} with orientation {}\n", i3_common::logging::pointer(con), (orientation as ::core::ffi::c_uint) as i32);
    let mut parent: *mut Con = parent_ptr(con);
    if ((*parent).type_0) == ContainerType::Floating {
        return ::core::ptr::null_mut::<Con>();
    }
    while con_orientation(parent) != orientation {
        ::log::debug!(
            "../src/con.c:con_parent_with_orientation:566 - Need to go one level further up\n"
        );
        parent = parent_ptr(parent);
        if !parent.is_null()
            && (((*parent).type_0) == ContainerType::Floating
                || ((*parent).type_0) == ContainerType::Output
                || !parent_ptr(parent).is_null()
                    && ((*parent_ptr(parent)).type_0) == ContainerType::Output)
        {
            parent = ::core::ptr::null_mut::<Con>();
        }
        if parent.is_null() {
            break;
        }
    }
    ::log::debug!(
        "../src/con.c:con_parent_with_orientation:579 - Result: {}\n",
        i3_common::logging::pointer(parent)
    );
    parent
}

/// The event-thread graph rooted at `con` must remain live throughout traversal.
pub unsafe fn con_get_fullscreen_con(con: *mut Con, fullscreen_mode: FullscreenMode) -> *mut Con {
    let mut pending = std::collections::VecDeque::from([con]);
    while let Some(current) = pending.pop_front() {
        if current != con && (*current).fullscreen_mode == fullscreen_mode {
            return current;
        }
        let mut child = nodes::first_ptr(current);
        while !child.is_null() {
            pending.push_back(child);
            child = nodes::next_ptr(child);
        }
        if let Some(parent) = arena::identity(current) {
            for id in arena::floating::children(parent) {
                if let Some(child) = arena::resolve(id) {
                    pending.push_back(child);
                }
            }
        }
    }
    std::ptr::null_mut()
}

pub unsafe fn con_get_fullscreen_covering_ws(ws: *mut Con) -> *mut Con {
    if ws.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let fs: *mut Con = con_get_fullscreen_con(croot, FullscreenMode::Global);
    if fs.is_null() {
        return con_get_fullscreen_con(ws, FullscreenMode::Output);
    }
    fs
}

pub unsafe fn con_is_internal(con: *mut Con) -> bool {
    (*con)
        .name
        .as_deref()
        .is_some_and(|name| name.to_bytes().starts_with(b"__"))
}

pub unsafe fn con_is_floating(con: *mut Con) -> bool {
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            669 as ::core::ffi::c_uint,
            c"_Bool con_is_floating(Con *)".as_ptr(),
        );
    };
    ((*con).floating).is_floating()
}

pub unsafe fn con_is_docked(con: *mut Con) -> bool {
    if parent_ptr(con).is_null() {
        return false;
    }
    if ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
        return true;
    }
    con_is_docked(parent_ptr(con))
}

pub unsafe fn con_inside_floating(con: *mut Con) -> *mut Con {
    if con.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    if ((*con).type_0) == ContainerType::Floating {
        return con;
    }
    if ((*con).floating).is_floating() {
        return parent_ptr(con);
    }
    if ((*con).type_0) == ContainerType::Workspace || ((*con).type_0) == ContainerType::Output {
        return ::core::ptr::null_mut::<Con>();
    }
    con_inside_floating(parent_ptr(con))
}

pub unsafe fn con_inside_focused(con: *mut Con) -> bool {
    if con == focused {
        return true;
    }
    if parent_ptr(con).is_null() {
        return false;
    }
    con_inside_focused(parent_ptr(con))
}

pub fn con_has_parent(con: *mut Con, parent: *mut Con) -> bool {
    let Some(target) = arena::identity(parent) else {
        return false;
    };
    let mut current = arena::identity(con).and_then(arena::parent::parent);
    while let Some(id) = current {
        if id == target {
            return true;
        }
        current = arena::parent::parent(id);
    }
    false
}

pub unsafe fn con_by_window_id(window: u32) -> *mut Con {
    for id in arena::containers() {
        let Some(con) = arena::resolve(id) else {
            continue;
        };
        if !(*con).window.is_null() && (*(*con).window).id == window {
            return con;
        }
    }
    ::core::ptr::null_mut::<Con>()
}

pub fn con_by_con_id(target: ::core::ffi::c_long) -> *mut Con {
    arena::lookup_address(target as usize)
}

pub fn con_exists(con: *mut Con) -> bool {
    arena::identity(con).is_some()
}

pub unsafe fn con_by_frame_id(frame: u32) -> *mut Con {
    for id in arena::containers() {
        let Some(con) = arena::resolve(id) else {
            continue;
        };
        if (*con).frame.id == frame {
            return con;
        }
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_by_mark(mark: *const ::core::ffi::c_char) -> *mut Con {
    for id in arena::containers() {
        let Some(con) = arena::resolve(id) else {
            continue;
        };
        if con_has_mark(con, mark) {
            return con;
        }
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn con_find_transient_for_window(start: *mut Con, target: u32) -> bool {
    let mut transient_con: *mut Con = start;
    let mut count: ::core::ffi::c_int = con_num_windows(croot);
    while !transient_con.is_null()
        && !(*transient_con).window.is_null()
        && (*(*transient_con).window).transient_for as ::core::ffi::c_long != 0
    {
        ::log::debug!("../src/con.c:con_find_transient_for_window:829 - transient_con = 0x{:08x}, transient_con->window->transient_for = 0x{:08x}, target = 0x{:08x}\n", { ((*(*transient_con).window).id) }, { ((*(*transient_con).window).transient_for) }, { (target) });
        if (*(*transient_con).window).transient_for == target {
            return true;
        }
        let next_transient: *mut Con = con_by_window_id((*(*transient_con).window).transient_for);
        if next_transient.is_null() {
            break;
        }
        if transient_con == next_transient {
            break;
        }
        transient_con = next_transient;
        let fresh0 = count;
        count -= 1;
        if fresh0 <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    false
}

pub unsafe fn con_has_mark(con: *mut Con, mark: *const ::core::ffi::c_char) -> bool {
    let mut current: *mut mark_t = (*con).marks_head.tqh_first;

    while !current.is_null() {
        if strcmp((*current).name, mark) == 0 as ::core::ffi::c_int {
            return true;
        }
        current = (*current).marks.tqe_next;
    }
    false
}

pub unsafe fn con_mark_toggle(con: *mut Con, mark: *const ::core::ffi::c_char, mode: MarkMode) {
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            873 as ::core::ffi::c_uint,
            c"void con_mark_toggle(Con *, const char *, mark_mode_t)".as_ptr(),
        );
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = {
            let pointer: *const libc::c_char = mark;
            i3_common::logging::string_bytes(
                if pointer.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(pointer))
                },
                None,
            )
            .to_vec()
        };
        let log_value_1 = i3_common::logging::pointer(con);
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/con.c:con_mark_toggle:874 - Toggling mark \"{}\" on con = {}.\n",
                log_value_0_text,
                log_value_1
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/con.c:con_mark_toggle:874 - Toggling mark \"".as_bytes(),
                    &log_value_0,
                    format!("\" on con = {}.\n", log_value_1).as_bytes(),
                ],
            );
        }
    };
    if con_has_mark(con, mark) {
        con_unmark(con, mark);
    } else {
        con_mark(con, mark, mode);
    };
}

pub unsafe fn con_mark(con: *mut Con, mark: *const ::core::ffi::c_char, mode: MarkMode) {
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            888 as ::core::ffi::c_uint,
            c"void con_mark(Con *, const char *, mark_mode_t)".as_ptr(),
        );
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = {
            let pointer: *const libc::c_char = mark;
            i3_common::logging::string_bytes(
                if pointer.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(pointer))
                },
                None,
            )
            .to_vec()
        };
        let log_value_1 = i3_common::logging::pointer(con);
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/con.c:con_mark:889 - Setting mark \"{}\" on con = {}.\n",
                log_value_0_text,
                log_value_1
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/con.c:con_mark:889 - Setting mark \"".as_bytes(),
                    &log_value_0,
                    format!("\" on con = {}.\n", log_value_1).as_bytes(),
                ],
            );
        }
    };
    con_unmark(::core::ptr::null_mut::<Con>(), mark);
    if mode == MarkMode::Replace {
        ::log::debug!(
            "../src/con.c:con_mark:893 - Removing all existing marks on con = {}.\n",
            i3_common::logging::pointer(con)
        );
        while !(*con).marks_head.tqh_first.is_null() {
            let current: *const mark_t = (*con).marks_head.tqh_first;
            con_unmark(con, (*current).name);
        }
    }
    let new: *mut mark_t =
        scalloc(1_usize, ::core::mem::size_of::<mark_t>() as usize) as *mut mark_t;
    (*new).name = sstrdup(mark);
    (*new).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
    (*new).marks.tqe_prev = (*con).marks_head.tqh_last;
    *(*con).marks_head.tqh_last = new;
    (*con).marks_head.tqh_last = &raw mut (*new).marks.tqe_next;
    ipc_send_window_event(c"mark".as_ptr(), con);
    (*con).mark_changed = true;
}

pub unsafe fn con_unmark(con: *mut Con, name: *const ::core::ffi::c_char) {
    if name.is_null() {
        ::log::debug!("../src/con.c:con_unmark:919 - Unmarking all containers.\n");
        for id in arena::containers() {
            let Some(current) = arena::resolve(id) else {
                continue;
            };
            if !(!con.is_null() && current != con) && !(*current).marks_head.tqh_first.is_null() {
                while !(*current).marks_head.tqh_first.is_null() {
                    let mut mark: *mut mark_t = (*current).marks_head.tqh_first;
                    free((*mark).name as *mut ::core::ffi::c_void);
                    (*mark).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if !(*mark).marks.tqe_next.is_null() {
                        (*(*mark).marks.tqe_next).marks.tqe_prev = (*mark).marks.tqe_prev;
                    } else {
                        (*current).marks_head.tqh_last = (*mark).marks.tqe_prev;
                    }
                    *(*mark).marks.tqe_prev = (*mark).marks.tqe_next;
                    free(mark as *mut ::core::ffi::c_void);
                    mark = ::core::ptr::null_mut::<mark_t>();
                    ipc_send_window_event(c"mark".as_ptr(), current);
                }
                (*current).mark_changed = true;
            }
        }
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = name;
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!(
                    "../src/con.c:con_unmark:941 - Removing mark \"{}\".\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        "../src/con.c:con_unmark:941 - Removing mark \"".as_bytes(),
                        &log_value_0,
                        "\".\n".as_bytes(),
                    ],
                );
            }
        };
        let current = if con.is_null() {
            con_by_mark(name)
        } else {
            con
        };
        if current.is_null() {
            ::log::debug!("../src/con.c:con_unmark:944 - No container found with this mark, so there is nothing to do.\n");
            return;
        }
        ::log::debug!(
            "../src/con.c:con_unmark:948 - Found mark on con = {}. Removing it now.\n",
            i3_common::logging::pointer(current)
        );
        (*current).mark_changed = true;
        let mut mark_0: *mut mark_t = (*current).marks_head.tqh_first;

        while !mark_0.is_null() {
            if strcmp((*mark_0).name, name) != 0 as ::core::ffi::c_int {
                mark_0 = (*mark_0).marks.tqe_next;
            } else {
                free((*mark_0).name as *mut ::core::ffi::c_void);
                (*mark_0).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if !(*mark_0).marks.tqe_next.is_null() {
                    (*(*mark_0).marks.tqe_next).marks.tqe_prev = (*mark_0).marks.tqe_prev;
                } else {
                    (*current).marks_head.tqh_last = (*mark_0).marks.tqe_prev;
                }
                *(*mark_0).marks.tqe_prev = (*mark_0).marks.tqe_next;
                free(mark_0 as *mut ::core::ffi::c_void);
                mark_0 = ::core::ptr::null_mut::<mark_t>();
                ipc_send_window_event(c"mark".as_ptr(), current);
                break;
            }
        }
    };
}

pub unsafe fn con_for_window(
    con: *mut Con,
    window: *mut i3Window,
    store_match: *mut *mut Match,
) -> *mut Con {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    child = nodes::first_ptr(con);
    while !child.is_null() {
        match_0 = (*child).swallow_head.tqh_first;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let result: *mut Con = con_for_window(child, window, store_match);
        if !result.is_null() {
            return result;
        }
        child = nodes::next_ptr(child);
    }
    for id in arena::identity(con)
        .into_iter()
        .flat_map(arena::floating::children)
    {
        let Some(child) = arena::resolve(id) else {
            continue;
        };
        match_0 = (*child).swallow_head.tqh_first;
        while !match_0.is_null() {
            if !match_matches_window(match_0, window) {
                match_0 = (*match_0).matches.tqe_next;
            } else {
                if !store_match.is_null() {
                    *store_match = match_0;
                }
                return child;
            }
        }
        let result_0: *mut Con = con_for_window(child, window, store_match);
        if !result_0.is_null() {
            return result_0;
        }
    }
    ::core::ptr::null_mut::<Con>()
}
/// Snapshot identities, not ownership; the graph and every returned node must
/// remain live until the caller has finished restoring the focus order.
pub fn get_focus_order(con: *mut Con) -> Vec<ConId> {
    arena::identity(con).map_or_else(Vec::new, focus::snapshot)
}

/// Restore the retained children from a pre-move snapshot. Native floating
/// ancestry policy is evaluated before the atomic typed permutation update.
pub unsafe fn set_focus_order(con: *mut Con, focus_order: &[ConId]) {
    let owner = arena::identity(con).expect("live focus parent");
    let count = focus::snapshot(owner).len();
    let mut retained = Vec::with_capacity(count);
    for &id in focus_order {
        if retained.len() == count {
            break;
        }
        let current = arena::resolve(id).expect("live focus snapshot child");
        if (*con).type_0 != ContainerType::Workspace && !con_inside_floating(current).is_null() {
            continue;
        }
        retained.push(id);
    }
    assert_eq!(
        retained.len(),
        count,
        "focus snapshot must contain every retained child"
    );
    focus::replace_order(owner, &retained)
        .expect("focus snapshot must permute retained membership");
}

pub fn con_num_children(con: *mut Con) -> ::core::ffi::c_int {
    nodes::len_ptr(con) as ::core::ffi::c_int
}

pub unsafe fn con_num_visible_children(con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0;
    }
    let mut children = 0;
    let mut current = nodes::first_ptr(con);
    while !current.is_null() {
        if !con_is_hidden(current) && con_is_leaf(current) {
            children += 1;
        } else {
            children += con_num_visible_children(current);
        }
        current = nodes::next_ptr(current);
    }
    children
}

pub unsafe fn con_num_windows(con: *mut Con) -> ::core::ffi::c_int {
    if con.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if con_has_managed_window(con) {
        return 1 as ::core::ffi::c_int;
    }
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Con = nodes::first_ptr(con);

    while !current.is_null() {
        num += con_num_windows(current);
        current = nodes::next_ptr(current);
    }
    for id in arena::identity(con)
        .into_iter()
        .flat_map(arena::floating::children)
    {
        if let Some(current) = arena::resolve(id) {
            num += con_num_windows(current);
        }
    }
    num
}

fn normalize_percentages(percentages: &mut [f64]) {
    let mut total = 0.0;
    let mut positive = 0;
    for &percent in percentages.iter() {
        if percent > 0.0 {
            total += percent;
            positive += 1;
        }
    }
    if positive != percentages.len() {
        for percent in percentages.iter_mut() {
            if *percent <= 0.0 {
                *percent = if positive == 0 {
                    1.0
                } else {
                    total / positive as f64
                };
                total += *percent;
            }
        }
    }
    if total == 0.0 {
        let equal = 1.0 / percentages.len() as f64;
        percentages.fill(equal);
    } else if total != 1.0 {
        for percent in percentages {
            *percent /= total;
        }
    }
}

/// Requires the live event-thread child queue; no callbacks occur while its
/// copied percentages are normalized and written back in their original order.
pub unsafe fn con_fix_percent(con: *mut Con) {
    let mut children = Vec::new();
    let mut child = nodes::first_ptr(con);
    while !child.is_null() {
        children.push(child);
        child = nodes::next_ptr(child);
    }
    let mut percentages: Vec<_> = children.iter().map(|&child| (*child).percent).collect();
    normalize_percentages(&mut percentages);
    for (child, percent) in children.into_iter().zip(percentages) {
        (*child).percent = percent;
    }
}

pub unsafe fn con_toggle_fullscreen(con: *mut Con, fullscreen_mode: FullscreenMode) {
    if ((*con).type_0) == ContainerType::Workspace {
        ::log::debug!(
            "../src/con.c:con_toggle_fullscreen:1190 - You cannot make a workspace fullscreen.\n"
        );
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_owner_2 = (*con).name.clone();
        let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
            ::log::debug!(
                "../src/con.c:con_toggle_fullscreen:1194 - toggling fullscreen for {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/con.c:con_toggle_fullscreen:1194 - toggling fullscreen for {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if ((*con).fullscreen_mode) == FullscreenMode::None {
        con_enable_fullscreen(con, fullscreen_mode);
    } else {
        con_disable_fullscreen(con);
    };
}
unsafe fn con_set_fullscreen_mode(con: *mut Con, fullscreen_mode: FullscreenMode) {
    (*con).fullscreen_mode = fullscreen_mode;
    ::log::debug!(
        "../src/con.c:con_set_fullscreen_mode:1212 - mode now: {}\n",
        (((*con).fullscreen_mode).raw() as ::core::ffi::c_uint) as i32
    );
    ipc_send_window_event(c"fullscreen_mode".as_ptr(), con);
    if (*con).window.is_null() {
        return;
    }
    if ((*con).fullscreen_mode) != FullscreenMode::None {
        ::log::debug!("../src/con.c:con_set_fullscreen_mode:1225 - Setting _NET_WM_STATE_FULLSCREEN for con = {} / window = {}.\n", i3_common::logging::pointer(con), ((*(*con).window).id) as i32);
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    } else {
        ::log::debug!("../src/con.c:con_set_fullscreen_mode:1228 - Removing _NET_WM_STATE_FULLSCREEN for con = {} / window = {}.\n", i3_common::logging::pointer(con), ((*(*con).window).id) as i32);
        xcb_remove_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_FULLSCREEN,
        );
    };
}

pub unsafe fn con_enable_fullscreen(con: *mut Con, fullscreen_mode: FullscreenMode) {
    if ((*con).type_0) == ContainerType::Workspace {
        ::log::debug!(
            "../src/con.c:con_enable_fullscreen:1246 - You cannot make a workspace fullscreen.\n"
        );
        return;
    }
    if (fullscreen_mode) == FullscreenMode::Global || (fullscreen_mode) == FullscreenMode::Output {
    } else {
        __assert_fail(
            c"fullscreen_mode == CF_GLOBAL || fullscreen_mode == CF_OUTPUT".as_ptr(),
            c"../src/con.c".as_ptr(),
            1250 as ::core::ffi::c_uint,
            c"void con_enable_fullscreen(Con *, fullscreen_mode_t)".as_ptr(),
        );
    };
    if (fullscreen_mode) == FullscreenMode::Global {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(con);
            let log_owner_2 = (*con).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!("../src/con.c:con_enable_fullscreen:1253 - enabling global fullscreen for {} / {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:con_enable_fullscreen:1253 - enabling global fullscreen for {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(con);
            let log_owner_2 = (*con).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!(
                    "../src/con.c:con_enable_fullscreen:1255 - enabling fullscreen for {} / {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:con_enable_fullscreen:1255 - enabling fullscreen for {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
    }
    if ((*con).fullscreen_mode) == fullscreen_mode {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(con);
            let log_owner_2 = (*con).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!("../src/con.c:con_enable_fullscreen:1259 - fullscreen already enabled for {} / {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:con_enable_fullscreen:1259 - fullscreen already enabled for {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
        return;
    }
    let con_ws: *mut Con = con_get_workspace(con);
    let mut fullscreen: *mut Con = con_get_fullscreen_con(croot, FullscreenMode::Global);
    if fullscreen.is_null() {
        fullscreen = con_get_fullscreen_con(con_ws, FullscreenMode::Output);
    }
    if !fullscreen.is_null() {
        con_disable_fullscreen(fullscreen);
    }
    let cur_ws: *mut Con = con_get_workspace(focused);
    let old_focused: *mut Con = focused;
    if (fullscreen_mode) == FullscreenMode::Global && cur_ws != con_ws {
        workspace_show(con_ws);
    }
    con_activate(con);
    if (fullscreen_mode) != FullscreenMode::Global && cur_ws != con_ws {
        con_activate(old_focused);
    }
    con_set_fullscreen_mode(con, fullscreen_mode);
}

pub unsafe fn con_disable_fullscreen(con: *mut Con) {
    if ((*con).type_0) == ContainerType::Workspace {
        ::log::debug!(
            "../src/con.c:con_disable_fullscreen:1297 - You cannot make a workspace fullscreen.\n"
        );
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_owner_2 = (*con).name.clone();
        let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
            ::log::debug!(
                "../src/con.c:con_disable_fullscreen:1301 - disabling fullscreen for {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/con.c:con_disable_fullscreen:1301 - disabling fullscreen for {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if ((*con).fullscreen_mode) == FullscreenMode::None {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(con);
            let log_owner_2 = (*con).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!("../src/con.c:con_disable_fullscreen:1304 - fullscreen already disabled for {} / {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:con_disable_fullscreen:1304 - fullscreen already disabled for {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
        return;
    }
    con_set_fullscreen_mode(con, FullscreenMode::None);
}
unsafe fn _con_move_to_con(
    mut con: *mut Con,
    mut target: *mut Con,
    behind_focused: bool,
    fix_coordinates: bool,
    dont_warp: bool,
    ignore_focus: bool,
    fix_percentage: bool,
) -> bool {
    let mut orig_target: *mut Con = target;
    let target_ws: *mut Con = con_get_workspace(target);
    if !ignore_focus && !con_fullscreen_permits_focusing(target_ws) {
        ::log::info!("Cannot move out of a fullscreen container.\n");
        return false;
    }
    if con_is_floating(con) {
        ::log::debug!(
            "../src/con.c:_con_move_to_con:1322 - Container is floating, using parent instead.\n"
        );
        con = parent_ptr(con);
    }
    let source_ws: *mut Con = con_get_workspace(con);
    if ((*con).type_0) == ContainerType::Workspace {
        let source_id = arena::identity(source_ws).expect("live source workspace");
        while let Some(child) = arena::floating::first(source_id).and_then(arena::resolve) {
            con_move_to_workspace(child, target_ws, true, true, false);
        }
        if con_is_leaf(con) {
            return false;
        }
        con = workspace_encapsulate(con);
        if con.is_null() {
            ::log::error!("ERROR: Workspace failed to move its contents into a container!\n");
            return false;
        }
    }
    let urgent: bool = (*con).urgent;
    let current_ws: *mut Con = con_get_workspace(focused);
    let source_output: *mut Con = con_get_output(con);
    let dest_output: *mut Con = con_get_output(target_ws);
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    if !ignore_focus && source_ws == current_ws && target_ws != source_ws {
        focus_next = con_descend_focused(source_ws);
        if focus_next == con || con_has_parent(focus_next, con) {
            focus_next = con_next_focused(con);
        }
    }
    if ((*target).type_0) != ContainerType::Workspace {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(target);
            let log_owner_2 = (*target).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            let log_value_2 = (((*target).type_0).raw() as ::core::ffi::c_uint) as i32;
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!(
                    "../src/con.c:_con_move_to_con:1369 - target originally = {} / {} / type {}\n",
                    log_value_0,
                    log_value_1_text,
                    log_value_2
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        format!(
                            "../src/con.c:_con_move_to_con:1369 - target originally = {} / ",
                            log_value_0
                        )
                        .as_bytes(),
                        log_value_1,
                        format!(" / type {}\n", log_value_2).as_bytes(),
                    ],
                );
            }
        };
        target = parent_ptr(target);
    }
    if ((*target).type_0) == ContainerType::Floating {
        ::log::debug!("../src/con.c:_con_move_to_con:1377 - floatingcon, going up even further\n");
        orig_target = target;
        target = parent_ptr(target);
    }
    if ((*con).type_0) == ContainerType::Floating {
        let ws: *mut Con = con_get_workspace(target);
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::pointer(ws);
            let log_owner_2 = (*ws).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!("../src/con.c:_con_move_to_con:1384 - This is a floating window, using workspace {} / {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:_con_move_to_con:1384 - This is a floating window, using workspace {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
        target = ws;
    }
    if source_output != dest_output {
        if fix_coordinates && ((*con).type_0) == ContainerType::Floating {
            floating_fix_coordinates(
                con,
                &raw mut (*source_output).rect,
                &raw mut (*dest_output).rect,
            );
        } else {
            ::log::debug!("../src/con.c:_con_move_to_con:1394 - Not fixing coordinates, fix_coordinates flag = {}\n", { (fix_coordinates as ::core::ffi::c_int) });
        }
    }
    let mut fullscreen: *mut Con = con_get_fullscreen_con(target_ws, FullscreenMode::Output);
    let con_has_fullscreen: bool = ((*con).fullscreen_mode) != FullscreenMode::None
        || !con_get_fullscreen_con(con, FullscreenMode::Global).is_null()
        || !con_get_fullscreen_con(con, FullscreenMode::Output).is_null();
    if con_has_fullscreen && !fullscreen.is_null() {
        con_toggle_fullscreen(fullscreen, FullscreenMode::Output);
        fullscreen = ::core::ptr::null_mut::<Con>();
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(target);
        let log_owner_2 = (*target).name.clone();
        let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
            ::log::debug!(
                "../src/con.c:_con_move_to_con:1414 - Re-attaching container to {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/con.c:_con_move_to_con:1414 - Re-attaching container to {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let parent: *mut Con = parent_ptr(con);
    con_detach(con);
    _con_attach(
        con,
        target,
        if behind_focused {
            ::core::ptr::null_mut::<Con>()
        } else {
            orig_target
        },
        !behind_focused,
    );
    if fix_percentage {
        con_fix_percent(parent);
        (*con).percent = 0.0f64;
        con_fix_percent(target);
    }
    if !ignore_focus && !con_is_internal(target_ws) && fullscreen.is_null() {
        let old_focus_ws: *mut Con = focus::first_ptr(output_get_content(dest_output));
        let old_focus: *mut Con = focused;
        con_activate(con_descend_focused(con));
        if old_focus_ws == current_ws && ((*old_focus).type_0) != ContainerType::Workspace {
            con_activate(old_focus);
        } else if con_get_workspace(focused) != old_focus_ws {
            con_focus(con_descend_focused(old_focus_ws));
        }
    }
    if !ignore_focus {
        workspace_show(current_ws);
        if dont_warp {
            ::log::debug!("../src/con.c:_con_move_to_con:1455 - x_set_warp_to(NULL) because dont_warp is set\n");
            x_set_warp_to(::core::ptr::null_mut::<Rect>());
        }
    }
    if !focus_next.is_null() {
        con_activate(con_descend_focused(focus_next));
    }
    if !con_is_leaf(con) {
        let mut child_0: *mut Con = nodes::first_ptr(con);

        while !child_0.is_null() {
            if !(*child_0).window.is_null() {
                startup_sequence_delete_by_window((*child_0).window);
            }
            child_0 = nodes::next_ptr(child_0);
        }
    }
    if !(*con).window.is_null() {
        startup_sequence_delete_by_window((*con).window);
    }
    if urgent {
        workspace_update_urgent_flag(source_ws);
        con_set_urgency(con, true);
    }
    free((*con).deco_render_params as *mut ::core::ffi::c_void);
    (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    (*parent)
        .on_remove_child
        .expect("non-null function pointer")(parent);
    ipc_send_window_event(c"move".as_ptr(), con);
    ewmh_update_wm_desktop();
    true
}

pub unsafe fn con_move_to_target(con: *mut Con, mut target: *mut Con) -> bool {
    if con_get_workspace(target) == workspace_get(c"__i3_scratch".as_ptr()) {
        ::log::debug!("../src/con.c:con_move_to_target:1501 - target container is in the scratchpad, moving container to scratchpad.\n");
        scratchpad_move(con);
        return true;
    }
    if con_is_floating(target) {
        ::log::debug!("../src/con.c:con_move_to_target:1508 - target container is floating, moving container to target's workspace.\n");
        con_move_to_workspace(con, con_get_workspace(target), true, false, false);
        return true;
    }
    if ((*target).type_0) == ContainerType::Workspace && con_is_leaf(target) {
        ::log::debug!("../src/con.c:con_move_to_target:1514 - target container is an empty workspace, simply moving the container there.\n");
        con_move_to_workspace(con, target, true, false, false);
        return true;
    }
    if con_is_split(target) {
        ::log::debug!("../src/con.c:con_move_to_target:1523 - target is a split container, descending to the currently focused child.\n");
        target = focus::first_ptr(target);
    }
    if con == target || con_has_parent(target, con) {
        ::log::debug!("../src/con.c:con_move_to_target:1528 - cannot move the container to or inside itself, aborting.\n");
        return false;
    }
    _con_move_to_con(con, target, false, true, false, false, true)
}

pub unsafe fn con_move_to_mark(con: *mut Con, mark: *const ::core::ffi::c_char) -> bool {
    let target: *mut Con = con_by_mark(mark);
    if target.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = mark;
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!(
                    "../src/con.c:con_move_to_mark:1542 - found no container with mark \"{}\"\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        "../src/con.c:con_move_to_mark:1542 - found no container with mark \""
                            .as_bytes(),
                        &log_value_0,
                        "\"\n".as_bytes(),
                    ],
                );
            }
        };
        return false;
    }
    con_move_to_target(con, target)
}

pub unsafe fn con_move_to_workspace(
    con: *mut Con,
    workspace: *mut Con,
    fix_coordinates: bool,
    dont_warp: bool,
    ignore_focus: bool,
) {
    if ((*workspace).type_0) == ContainerType::Workspace {
    } else {
        __assert_fail(
            c"workspace->type == CT_WORKSPACE".as_ptr(),
            c"../src/con.c".as_ptr(),
            1570 as ::core::ffi::c_uint,
            c"void con_move_to_workspace(Con *, Con *, _Bool, _Bool, _Bool)".as_ptr(),
        );
    };
    let source_ws: *mut Con = con_get_workspace(con);
    if workspace == source_ws {
        ::log::debug!("../src/con.c:con_move_to_workspace:1574 - Not moving, already there\n");
        return;
    }
    let target: *mut Con = con_descend_focused(workspace);
    _con_move_to_con(
        con,
        target,
        true,
        fix_coordinates,
        dont_warp,
        ignore_focus,
        true,
    );
}

pub unsafe fn con_move_to_output(con: *mut Con, output: *mut Output, fix_coordinates: bool) {
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
    while child == -(1 as ::core::ffi::c_int) as *mut Con && {
        child = ::core::ptr::null_mut::<Con>();
        true
    } {
        child = nodes::first_ptr(output_get_content((*output).con));
        while !child.is_null() {
            if !workspace_is_visible(child) {
                child = nodes::next_ptr(child);
            } else {
                ws = child;
                break;
            }
        }
    }
    if !ws.is_null() {
    } else {
        __assert_fail(
            c"ws != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            1590 as ::core::ffi::c_uint,
            c"void con_move_to_output(Con *, Output *, _Bool)".as_ptr(),
        );
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_value_1 = {
            let pointer: *const libc::c_char = output_primary_name(output);
            i3_common::logging::string_bytes(
                if pointer.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(pointer))
                },
                None,
            )
            .to_vec()
        };
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
            ::log::debug!(
                "../src/con.c:con_move_to_output:1591 - Moving con {} to output {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/con.c:con_move_to_output:1591 - Moving con {} to output ",
                        log_value_0
                    )
                    .as_bytes(),
                    &log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    con_move_to_workspace(con, ws, fix_coordinates, false, false);
}

pub unsafe fn con_move_to_output_name(
    con: *mut Con,
    name: *const ::core::ffi::c_char,
    fix_coordinates: bool,
) -> bool {
    let current_output: *mut Output = get_output_for_con(con);
    let output: *mut Output = get_output_from_string(current_output, name);
    if output.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = name;
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: Could not find output \"{}\"\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: Could not find output \"".as_bytes(),
                        &log_value_0,
                        "\"\n".as_bytes(),
                    ],
                );
            }
        };
        return false;
    }
    con_move_to_output(con, output, fix_coordinates);
    true
}

pub unsafe fn con_orientation(con: *mut Con) -> Orientation {
    match (*con).layout {
        Layout::SplitV | Layout::Stacked => Orientation::Vertical,
        Layout::SplitH | Layout::Tabbed => Orientation::Horizontal,
        Layout::Default => {
            ::log::error!("ERROR: Someone called con_orientation() on a con with L_DEFAULT, this is a bug in the code.\n");
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                1634,
                c"orientation_t con_orientation(Con *)".as_ptr(),
            )
        }
        Layout::DockArea | Layout::Output => {
            ::log::error!(
                "ERROR: con_orientation() called on dockarea/output ({}) container {}\n",
                ((*con).layout as ::core::ffi::c_uint) as i32,
                i3_common::logging::pointer(con)
            );
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                1639,
                c"orientation_t con_orientation(Con *)".as_ptr(),
            )
        }
    }
}

pub unsafe fn con_next_focused(mut con: *mut Con) -> *mut Con {
    if ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
        ::log::debug!("../src/con.c:con_next_focused:1654 - selecting workspace for dock client\n");
        return con_descend_focused(output_get_content(parent_ptr(parent_ptr(con))));
    }
    if con_is_floating(con) {
        con = parent_ptr(con);
    }
    let mut next: *mut Con = focus::first_ptr(parent_ptr(con));
    if next != con {
        ::log::debug!(
            "../src/con.c:con_next_focused:1665 - Using first entry {}\n",
            i3_common::logging::pointer(next)
        );
    } else {
        next = focus::next_ptr(con);
        if next.is_null() {
            next = parent_ptr(con);
        }
    }
    while !focus::first_ptr(next).is_null() && focus::first_ptr(next) != con {
        next = focus::first_ptr(next);
    }
    if ((*con).type_0) == ContainerType::Floating && next != parent_ptr(con) {
        next = con_descend_focused(next);
    }
    next
}

pub unsafe fn con_descend_focused(con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    while next != focused && !focus::first_ptr(next).is_null() {
        next = focus::first_ptr(next);
    }
    next
}

pub unsafe fn con_descend_tiling_focused(con: *mut Con) -> *mut Con {
    let mut next: *mut Con = con;
    let mut before: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if next == focused {
        return next;
    }
    loop {
        before = next;
        child = focus::first_ptr(next);
        while !child.is_null() {
            if ((*child).type_0) == ContainerType::Floating {
                child = focus::next_ptr(child);
            } else {
                next = child;
                break;
            }
        }
        if !(before != next && next != focused) {
            break;
        }
    }
    next
}

pub unsafe fn con_descend_direction(con: *mut Con, direction: Direction) -> *mut Con {
    let mut most: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let orientation: Orientation = con_orientation(con);
    ::log::debug!("../src/con.c:con_descend_direction:1741 - con_descend_direction({}, orientation {}, direction {})\n", i3_common::logging::pointer(con), ((orientation) as u32) as i32, (direction as ::core::ffi::c_uint) as i32);
    if direction == Direction::Left || direction == Direction::Right {
        if orientation == Orientation::Horizontal {
            if direction == Direction::Right {
                most = nodes::first_ptr(con);
            } else {
                most = nodes::last_ptr(con);
            }
        } else if orientation == Orientation::Vertical {
            current = focus::first_ptr(con);
            while !current.is_null() {
                if ((*current).type_0) != ContainerType::Floating {
                    most = current;
                    break;
                } else {
                    current = focus::next_ptr(current);
                }
            }
        } else {
            return con;
        }
    }
    if direction == Direction::Up || direction == Direction::Down {
        if orientation == Orientation::Vertical {
            if direction == Direction::Up {
                most = nodes::last_ptr(con);
            } else {
                most = nodes::first_ptr(con);
            }
        } else if orientation == Orientation::Horizontal {
            current = focus::first_ptr(con);
            while !current.is_null() {
                if ((*current).type_0) != ContainerType::Floating {
                    most = current;
                    break;
                } else {
                    current = focus::next_ptr(current);
                }
            }
        } else {
            return con;
        }
    }
    if most.is_null() {
        return con;
    }
    con_descend_direction(most, direction)
}
fn has_outer_gaps(gaps: gaps_t) -> bool {
    gaps.top > 0 as ::core::ffi::c_int
        || gaps.right > 0 as ::core::ffi::c_int
        || gaps.bottom > 0 as ::core::ffi::c_int
        || gaps.left > 0 as ::core::ffi::c_int
}

pub unsafe fn con_draw_decoration_into_frame(con: *mut Con) -> bool {
    con_is_leaf(con)
        && con_border_style(con) == BorderStyle::Normal
        && (parent_ptr(con).is_null()
            || (*parent_ptr(con)).layout != Layout::Tabbed
                && (*parent_ptr(con)).layout != Layout::Stacked)
}
fn border_rect(style: BorderStyle, width: i32, hidden: adjacent_t) -> Rect {
    if style == BorderStyle::None {
        return Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
    }
    let width = width as u32;
    let doubled = width.wrapping_mul(2).wrapping_neg();
    let mut rect = Rect {
        x: width,
        y: if style == BorderStyle::Normal {
            0
        } else {
            width
        },
        width: doubled,
        height: if style == BorderStyle::Normal {
            width.wrapping_neg()
        } else {
            doubled
        },
    };
    if hidden.contains(AdjacentEdges::LEFT) {
        rect.x = rect.x.wrapping_sub(width);
        rect.width = rect.width.wrapping_add(width);
    }
    if hidden.contains(AdjacentEdges::RIGHT) {
        rect.width = rect.width.wrapping_add(width);
    }
    if hidden.contains(AdjacentEdges::TOP) && style != BorderStyle::Normal {
        rect.y = rect.y.wrapping_sub(width);
        rect.height = rect.height.wrapping_add(width);
    }
    if hidden.contains(AdjacentEdges::BOTTOM) {
        rect.height = rect.height.wrapping_add(width);
    }
    rect
}
fn adjacent_borders(rect: Rect, workspace: Rect) -> adjacent_t {
    let mut result = AdjacentEdges::empty();
    if rect.x == workspace.x {
        result |= AdjacentEdges::LEFT;
    }
    if rect.x.wrapping_add(rect.width) == workspace.x.wrapping_add(workspace.width) {
        result |= AdjacentEdges::RIGHT;
    }
    if rect.y == workspace.y {
        result |= AdjacentEdges::TOP;
    }
    if rect.y.wrapping_add(rect.height) == workspace.y.wrapping_add(workspace.height) {
        result |= AdjacentEdges::BOTTOM;
    }
    result
}

unsafe fn con_border_style_rect_without_title(con: *mut Con) -> Rect {
    if (config.hide_edge_borders == HideEdgeBordersMode::Smart
        && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
        || config.hide_edge_borders == HideEdgeBordersMode::SmartNoGaps
            && con_num_visible_children(con_get_workspace(con)) <= 1 as ::core::ffi::c_int
            && !has_outer_gaps(calculate_effective_gaps(con)))
        && !con_is_floating(con)
    {
        return Rect {
            x: 0_u32,
            y: 0_u32,
            width: 0_u32,
            height: 0_u32,
        };
    }
    let mut border_width: ::core::ffi::c_int = (*con).current_border_width;
    ::log::debug!("../src/con.c:con_border_style_rect_without_title:1831 - The border width for con is set to: {}\n", { ((*con).current_border_width) });
    if (*con).current_border_width < 0 as ::core::ffi::c_int {
        if con_is_floating(con) {
            border_width = config.default_floating_border_width;
        } else {
            border_width = config.default_border_width;
        }
    }
    ::log::debug!("../src/con.c:con_border_style_rect_without_title:1840 - Effective border width is set to: {}\n", (border_width) as i32);
    let border_style: BorderStyle = con_border_style(con);
    if border_style == BorderStyle::None {
        return border_rect(border_style, border_width, AdjacentEdges::empty());
    }
    let mask = if config.hide_edge_borders == HideEdgeBordersMode::SmartNoGaps {
        HideEdgeBordersMode::None
    } else {
        config.hide_edge_borders
    };
    let hidden = con_adjacent_borders(con) & mask.edge_mask();
    border_rect(border_style, border_width, hidden)
}

pub unsafe fn con_border_style_rect(con: *mut Con) -> Rect {
    let mut result: Rect = con_border_style_rect_without_title(con);
    if con_border_style(con) == BorderStyle::Normal && con_draw_decoration_into_frame(con) {
        let deco_height: ::core::ffi::c_int = render_deco_height() as ::core::ffi::c_int;
        result.y = result.y.wrapping_add(deco_height as u32);
        result.height = result.height.wrapping_sub(deco_height as u32);
    }
    result
}

pub unsafe fn con_adjacent_borders(con: *mut Con) -> adjacent_t {
    if con_is_floating(con) {
        return AdjacentEdges::empty();
    }
    let workspace = con_get_workspace(con);
    adjacent_borders((*con).rect, (*workspace).rect)
}

pub unsafe fn con_border_style(con: *mut Con) -> BorderStyle {
    if ((*con).fullscreen_mode) == FullscreenMode::Output
        || ((*con).fullscreen_mode) == FullscreenMode::Global
    {
        ::log::debug!(
            "../src/con.c:con_border_style:1933 - this one is fullscreen! overriding BS_NONE\n"
        );
        return BorderStyle::None;
    }
    if !parent_ptr(con).is_null() {
        if (*parent_ptr(con)).layout == Layout::Stacked {
            return if con_num_children(parent_ptr(con)) == 1 as ::core::ffi::c_int {
                (*con).border_style
            } else {
                BorderStyle::Normal
            };
        }
        if (*parent_ptr(con)).layout == Layout::Tabbed && (*con).border_style != BorderStyle::Normal
        {
            return if con_num_children(parent_ptr(con)) == 1 as ::core::ffi::c_int {
                (*con).border_style
            } else {
                BorderStyle::Normal
            };
        }
        if ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
            return BorderStyle::None;
        }
    }
    (*con).border_style
}

pub unsafe fn con_set_border_style(
    con: *mut Con,
    mut border_style: BorderStyle,
    border_width: ::core::ffi::c_int,
) {
    if border_style as ::core::ffi::c_uint > (*con).max_user_border_style as ::core::ffi::c_uint {
        border_style = (*con).max_user_border_style;
    }
    if !con_is_floating(con) {
        (*con).border_style = border_style;
        (*con).current_border_width = border_width;
        return;
    }
    let parent: *mut Con = parent_ptr(con);
    let mut bsr: Rect = con_border_style_rect(con);
    (*con).rect = rect_add((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_add((*parent).rect as Rect, bsr) as Rect;
    (*con).border_style = border_style;
    (*con).current_border_width = border_width;
    bsr = con_border_style_rect(con);
    (*con).rect = rect_sub((*con).rect as Rect, bsr) as Rect;
    (*parent).rect = rect_sub((*parent).rect as Rect, bsr) as Rect;
}

pub unsafe fn con_set_layout(mut con: *mut Con, layout: Layout) {
    ::log::debug!(
        "../src/con.c:con_set_layout:1999 - con_set_layout({}, {}), con->type = {}\n",
        i3_common::logging::pointer(con),
        (layout as ::core::ffi::c_uint) as i32,
        (((*con).type_0).raw() as ::core::ffi::c_uint) as i32
    );
    if ((*con).type_0) != ContainerType::Workspace {
        con = parent_ptr(con);
    }
    if (*con).layout == Layout::SplitH || (*con).layout == Layout::SplitV {
        (*con).last_split_layout = (*con).layout;
    }
    if ((*con).type_0) == ContainerType::Workspace {
        if con_num_children(con) == 0 as ::core::ffi::c_int {
            let ws_layout: Layout = if layout == Layout::Stacked || layout == Layout::Tabbed {
                layout
            } else {
                Layout::Default
            };
            ::log::debug!(
                "../src/con.c:con_set_layout:2022 - Setting workspace_layout to {}\n",
                (ws_layout as ::core::ffi::c_uint) as i32
            );
            (*con).workspace_layout = ws_layout;
            ::log::debug!(
                "../src/con.c:con_set_layout:2024 - Setting layout to {}\n",
                (layout as ::core::ffi::c_uint) as i32
            );
            (*con).layout = layout;
        } else if layout == Layout::Stacked
            || layout == Layout::Tabbed
            || layout == Layout::SplitV
            || layout == Layout::SplitH
        {
            ::log::debug!("../src/con.c:con_set_layout:2027 - Creating new split container\n");
            let new: *mut Con = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            set_parent_ptr(new, con).expect("live acyclic ancestry");
            (*new).layout = layout;
            (*new).last_split_layout = (*con).last_split_layout;
            let focus_order = get_focus_order(con);
            ::log::debug!("../src/con.c:con_set_layout:2040 - Moving cons\n");
            while !nodes::first_ptr(con).is_null() {
                let child: *mut Con = nodes::first_ptr(con);
                con_detach(child);
                con_attach(child, new, true);
            }
            set_focus_order(new, &focus_order);
            ::log::debug!("../src/con.c:con_set_layout:2051 - Attaching new split to ws\n");
            con_attach(new, con, false);
            tree_flatten(croot);
            con_force_split_parents_redraw(con);
            return;
        }
    }
    if ((*con).type_0) != ContainerType::Workspace
        && ((*parent_ptr(con)).type_0) != ContainerType::Workspace
        && con_num_children(con) == 1 as ::core::ffi::c_int
        && con_num_children(parent_ptr(con)) == 1 as ::core::ffi::c_int
    {
        let parent: *mut Con = parent_ptr(con);
        let child_0: *mut Con = nodes::first_ptr(con);
        con_detach(child_0);
        con_attach(child_0, parent, true);
        (*parent).last_split_layout = (*con).last_split_layout;
        tree_close_internal(con, KillWindow::DontKill, true);
        con = parent;
    }
    if layout == Layout::Default {
        (*con).layout = (*con).last_split_layout;
        if (*con).layout == Layout::Default {
            (*con).layout = Layout::SplitH;
        }
    } else {
        (*con).layout = layout;
    }
    con_force_split_parents_redraw(con);
}

fn layout_toggle(
    mode: &std::ffi::CStr,
    current: Layout,
    last_split: Layout,
) -> (Option<Layout>, Vec<std::ffi::CString>) {
    let equals = |left: &std::ffi::CStr, right: &std::ffi::CStr| unsafe {
        libc::strcasecmp(left.as_ptr(), right.as_ptr()) == 0
    };
    let mut invalid = Vec::new();
    if equals(mode, c"split") || mode.to_bytes().contains(&b' ') {
        let mut next = None;
        let mut found = false;
        for bytes in mode
            .to_bytes()
            .split(|&byte| byte == b' ')
            .filter(|bytes| !bytes.is_empty())
        {
            let token = std::ffi::CString::new(bytes).unwrap();
            let layout = if equals(&token, c"split") {
                match current {
                    Layout::SplitH => Layout::SplitV,
                    Layout::SplitV => Layout::SplitH,
                    _ if last_split == Layout::Default => Layout::SplitH,
                    _ => last_split,
                }
            } else {
                match layout_from_name(&token) {
                    Some(layout) if layout != Layout::Default => layout,
                    _ => {
                        invalid.push(token);
                        continue;
                    }
                }
            };
            if next.is_none() {
                next = Some(layout);
            }
            if found {
                next = Some(layout);
                break;
            }
            if current == layout {
                found = true;
            }
        }
        (next, invalid)
    } else {
        let all = equals(mode, c"all");
        let next = if all || equals(mode, c"default") {
            match current {
                Layout::Stacked => Some(Layout::Tabbed),
                Layout::Tabbed => Some(if all { Layout::SplitH } else { last_split }),
                Layout::SplitH if all => Some(Layout::SplitV),
                Layout::SplitH | Layout::SplitV => Some(Layout::Stacked),
                _ => None,
            }
        } else {
            None
        };
        (next, invalid)
    }
}

pub unsafe fn con_toggle_layout(con: *mut Con, toggle_mode: *const ::core::ffi::c_char) {
    let parent = if (*con).type_0 == ContainerType::Workspace {
        con
    } else {
        parent_ptr(con)
    };
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_value_1 = {
            let pointer: *const libc::c_char = toggle_mode;
            i3_common::logging::string_bytes(
                if pointer.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(pointer))
                },
                None,
            )
            .to_vec()
        };
        let log_value_2 = i3_common::logging::pointer(parent);
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
            ::log::debug!(
                "../src/con.c:con_toggle_layout:2117 - con_toggle_layout({}, {}), parent = {}\n",
                log_value_0,
                log_value_1_text,
                log_value_2
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/con.c:con_toggle_layout:2117 - con_toggle_layout({}, ",
                        log_value_0
                    )
                    .as_bytes(),
                    &log_value_1,
                    format!("), parent = {}\n", log_value_2).as_bytes(),
                ],
            );
        }
    };
    let mode = std::ffi::CStr::from_ptr(toggle_mode);
    let (next, invalid) = layout_toggle(mode, (*parent).layout, (*parent).last_split_layout);
    for token in invalid {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = i3_common::logging::string_bytes(Some(token.as_c_str()), None);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                ::log::error!(
                    "ERROR: The token '{}' was not recognized and has been skipped.\n",
                    log_value_0_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: The token '".as_bytes(),
                        log_value_0,
                        "' was not recognized and has been skipped.\n".as_bytes(),
                    ],
                );
            }
        };
    }
    if let Some(next) = next {
        con_set_layout(con, next);
    }
}

unsafe fn con_on_remove_child(con: *mut Con) {
    ::log::debug!("../src/con.c:con_on_remove_child:2206 - on_remove_child\n");
    if ((*con).type_0) == ContainerType::Output
        || ((*con).type_0) == ContainerType::Root
        || ((*con).type_0) == ContainerType::DockArea
        || !parent_ptr(con).is_null() && ((*parent_ptr(con)).type_0) == ContainerType::Output
    {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = (((*con).type_0).raw() as ::core::ffi::c_uint) as i32;
            let log_owner_2 = (*con).name.clone();
            let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                ::log::debug!(
                    "../src/con.c:con_on_remove_child:2214 - not handling, type = {}, name = {}\n",
                    log_value_0,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/con.c:con_on_remove_child:2214 - not handling, type = {}, name = ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
            }
        };
        return;
    }
    if ((*con).type_0) == ContainerType::Workspace {
        if focus::first_ptr(con).is_null() && !workspace_is_visible(con) {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer(con);
                let log_owner_2 = (*con).name.clone();
                let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
                if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                    ::log::info!(
                        "Closing old workspace ({} / {}), it is empty\n",
                        log_value_0,
                        log_value_1_text
                    );
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Verbose,
                        &[
                            format!("Closing old workspace ({} / ", log_value_0).as_bytes(),
                            log_value_1,
                            "), it is empty\n".as_bytes(),
                        ],
                    );
                }
            };
            let payload =
                ipc_marshal_workspace_event(c"empty".as_ptr(), con, ::core::ptr::null_mut::<Con>());
            tree_close_internal(con, KillWindow::DontKill, false);
            ipc_send_event(
                c"workspace".as_ptr(),
                I3_IPC_EVENT_WORKSPACE as u32,
                payload.as_ptr(),
            );
        }
        return;
    }
    con_force_split_parents_redraw(con);
    (*con).urgent = con_has_urgent_child(con);
    con_update_parents_urgency(con);
    let children: ::core::ffi::c_int = con_num_children(con);
    if children == 0 as ::core::ffi::c_int {
        ::log::debug!("../src/con.c:con_on_remove_child:2243 - Container empty, closing\n");
        tree_close_internal(con, KillWindow::DontKill, false);
    }
}

pub unsafe fn con_minimum_size(con: *mut Con) -> Rect {
    ::log::debug!(
        "../src/con.c:con_minimum_size:2254 - Determining minimum size for con {}\n",
        i3_common::logging::pointer(con)
    );
    if con_is_leaf(con) {
        ::log::debug!("../src/con.c:con_minimum_size:2257 - leaf node, returning 75x50\n");
        return Rect {
            x: 0_u32,
            y: 0_u32,
            width: 75_u32,
            height: 50_u32,
        };
    }
    if ((*con).type_0) == ContainerType::Floating {
        ::log::debug!("../src/con.c:con_minimum_size:2262 - floating con\n");
        let child: *mut Con = nodes::first_ptr(con);
        return con_minimum_size(child);
    }
    if (*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed {
        let mut max_width: u32 = 0_u32;
        let mut max_height: u32 = 0_u32;
        let mut deco_height: u32 = 0_u32;
        let mut child_0: *mut Con = nodes::first_ptr(con);

        while !child_0.is_null() {
            let min: Rect = con_minimum_size(child_0);
            deco_height = deco_height.wrapping_add((*child_0).deco_rect.height);
            max_width = max(
                max_width as ::core::ffi::c_int,
                min.width as ::core::ffi::c_int,
            ) as u32;
            max_height = max(
                max_height as ::core::ffi::c_int,
                min.height as ::core::ffi::c_int,
            ) as u32;
            child_0 = nodes::next_ptr(child_0);
        }
        ::log::debug!("../src/con.c:con_minimum_size:2277 - stacked/tabbed now, returning {} x {} + deco_rect = {}\n", (max_width) as i32, (max_height) as i32, (deco_height) as i32);
        return Rect {
            x: 0_u32,
            y: 0_u32,
            width: max_width,
            height: max_height.wrapping_add(deco_height),
        };
    }
    if con_is_split(con) {
        let mut width: u32 = 0_u32;
        let mut height: u32 = 0_u32;
        let mut child_1: *mut Con = nodes::first_ptr(con);

        while !child_1.is_null() {
            let min_0: Rect = con_minimum_size(child_1);
            if (*con).layout == Layout::SplitH {
                width = width.wrapping_add(min_0.width);
                height = max(
                    height as ::core::ffi::c_int,
                    min_0.height as ::core::ffi::c_int,
                ) as u32;
            } else {
                height = height.wrapping_add(min_0.height);
                width = max(
                    width as ::core::ffi::c_int,
                    min_0.width as ::core::ffi::c_int,
                ) as u32;
            }
            child_1 = nodes::next_ptr(child_1);
        }
        ::log::debug!("../src/con.c:con_minimum_size:2297 - split container, returning width = {} x height = {}\n", (width) as i32, (height) as i32);
        return Rect {
            x: 0_u32,
            y: 0_u32,
            width,
            height,
        };
    }
    ::log::error!(
        "ERROR: Unhandled case, type = {}, layout = {}, split = {}\n",
        (((*con).type_0).raw() as ::core::ffi::c_uint) as i32,
        ((*con).layout as ::core::ffi::c_uint) as i32,
        { (con_is_split(con) as ::core::ffi::c_int) }
    );
    __assert_fail(
        c"false".as_ptr(),
        c"../src/con.c".as_ptr(),
        2303 as ::core::ffi::c_uint,
        c"Rect con_minimum_size(Con *)".as_ptr(),
    );
}

pub unsafe fn con_fullscreen_permits_focusing(con: *mut Con) -> bool {
    if focused.is_null() {
        return true;
    }
    let mut fs: *mut Con = focused;
    while !fs.is_null() && ((*fs).fullscreen_mode) == FullscreenMode::None {
        fs = parent_ptr(fs);
    }
    if !fs.is_null() {
    } else {
        __assert_fail(
            c"fs != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2341 as ::core::ffi::c_uint,
            c"_Bool con_fullscreen_permits_focusing(Con *)".as_ptr(),
        );
    };
    if ((*fs).fullscreen_mode) != FullscreenMode::None {
    } else {
        __assert_fail(
            c"fs->fullscreen_mode != CF_NONE".as_ptr(),
            c"../src/con.c".as_ptr(),
            2344 as ::core::ffi::c_uint,
            c"_Bool con_fullscreen_permits_focusing(Con *)".as_ptr(),
        );
    };
    if ((*fs).type_0) == ContainerType::Workspace {
        return true;
    }
    if con == fs {
        return true;
    }
    if ((*fs).fullscreen_mode) == FullscreenMode::Output
        && con_get_workspace(con) != con_get_workspace(fs)
    {
        return true;
    }
    con_has_parent(con, fs)
}

pub unsafe fn con_has_urgent_child(con: *mut Con) -> bool {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    if con_is_leaf(con) {
        return (*con).urgent;
    }
    child = nodes::first_ptr(con);
    while !child.is_null() {
        if con_has_urgent_child(child) {
            return true;
        }
        child = nodes::next_ptr(child);
    }
    false
}

pub unsafe fn con_update_parents_urgency(con: *mut Con) {
    let mut parent: *mut Con = parent_ptr(con);
    if ((*con).type_0) == ContainerType::Workspace {
        return;
    }
    let new_urgency_value: bool = (*con).urgent;
    while !parent.is_null()
        && ((*parent).type_0) != ContainerType::Workspace
        && ((*parent).type_0) != ContainerType::DockArea
    {
        if new_urgency_value {
            (*parent).urgent = true;
        } else if !con_has_urgent_child(parent) {
            (*parent).urgent = false;
        }
        parent = parent_ptr(parent);
    }
}

pub unsafe fn con_set_urgency(con: *mut Con, urgent: bool) {
    if urgent && focused == con {
        ::log::debug!(
            "../src/con.c:con_set_urgency:2426 - Ignoring urgency flag for current client\n"
        );
        return;
    }
    let old_urgent: bool = (*con).urgent;
    if (*con).urgency_timer.is_null() {
        (*con).urgent = urgent;
    } else {
        ::log::debug!("../src/con.c:con_set_urgency:2435 - Discarding urgency WM_HINT because timer is running\n");
    }
    if !(*con).window.is_null() {
        if (*con).urgent {
            gettimeofday(&raw mut (*(*con).window).urgent, ::core::ptr::null_mut());
        } else {
            (*(*con).window).urgent.tv_sec = 0 as libc::time_t;
            (*(*con).window).urgent.tv_usec = 0 as libc::suseconds_t;
        }
    }
    con_update_parents_urgency(con);
    let mut ws: *mut Con = con_get_workspace(con);

    if !ws.is_null() {
        workspace_update_urgent_flag(ws);
    }
    if (*con).urgent != old_urgent {
        ::log::info!("Urgency flag changed to {}\n", {
            ((*con).urgent as ::core::ffi::c_int)
        });
        ipc_send_window_event(c"urgent".as_ptr(), con);
    }
}

/// Borrowed graph names are copied before returning; the native adapter below
/// retains the existing malloc ownership contract used by diagnostics/title code.
unsafe fn tree_representation(con: *mut Con) -> Vec<u8> {
    if con_is_leaf(con) {
        let name = if (*con).window.is_null() {
            c"nowin".as_ptr()
        } else if (*(*con).window).class_instance.is_none() {
            c"noinstance".as_ptr()
        } else {
            (*(*con).window).class_instance_ptr()
        };
        return std::ffi::CStr::from_ptr(name).to_bytes().to_vec();
    }
    let label = match (*con).layout {
        Layout::Default => b'D',
        Layout::SplitV => b'V',
        Layout::SplitH => b'H',
        Layout::Tabbed => b'T',
        Layout::Stacked => b'S',
        _ => {
            ::log::error!("ERROR: BUG: Code not updated to account for new layout type\n");
            __assert_fail(
                c"false".as_ptr(),
                c"../src/con.c".as_ptr(),
                2503,
                c"char *con_get_tree_representation(Con *)".as_ptr(),
            );
        }
    };
    let mut text = vec![label, b'['];
    let mut child = nodes::first_ptr(con);
    while !child.is_null() {
        if child != nodes::first_ptr(con) {
            text.push(b' ');
        }
        text.extend(tree_representation(child));
        child = nodes::next_ptr(child);
    }
    text.push(b']');
    text
}

pub unsafe fn con_get_tree_representation(con: *mut Con) -> *mut ::core::ffi::c_char {
    let text = tree_representation(con);
    i3_common::safewrappers::sstrndup(text.as_ptr().cast(), text.len())
}

fn title_capture(bytes: &[u8], markup: bool) -> std::ffi::CString {
    if markup {
        let text = i3_common::string::I3String::from_bytes(bytes);
        std::ffi::CString::new(i3_common::string::escape_markup(text.as_str().unwrap())).unwrap()
    } else {
        std::ffi::CString::new(bytes).expect("borrowed C string contains no NUL")
    }
}

/// `con`, its optional window, title format and borrowed string fields are live
/// event-thread data throughout this call; no graph callbacks occur here.
pub unsafe fn con_parse_title_format(con: *mut Con) -> i3_common::string::I3String {
    // Snapshot before text conversion/logging can reenter the model.
    let format = (*con)
        .title_format
        .clone()
        .expect("container has a title format");
    let win = (*con).window;
    let markup = font_is_pango(&crate::config::config.font);
    let capture = |pointer: *const libc::c_char| {
        let bytes = if pointer.is_null() {
            &[][..]
        } else {
            std::ffi::CStr::from_ptr(pointer).to_bytes()
        };
        title_capture(bytes, markup)
    };
    let (title, class, instance, machine) = if win.is_null() {
        (
            title_capture(&tree_representation(con), markup),
            c"i3-frame".to_owned(),
            c"i3-frame".to_owned(),
            c"".to_owned(),
        )
    } else {
        let title = (*win).name.clone();
        let class = (*win).class_class.clone();
        let instance = (*win).class_instance.clone();
        let machine = (*win).machine.clone();
        (
            capture(
                title
                    .as_ref()
                    .and_then(|text| text.cstr_or_report())
                    .map_or(std::ptr::null(), std::ffi::CStr::as_ptr),
            ),
            title_capture(
                class.as_deref().map_or(&[][..], std::ffi::CStr::to_bytes),
                markup,
            ),
            title_capture(
                instance
                    .as_deref()
                    .map_or(&[][..], std::ffi::CStr::to_bytes),
                markup,
            ),
            title_capture(
                machine.as_deref().map_or(&[][..], std::ffi::CStr::to_bytes),
                markup,
            ),
        )
    };
    let expanded = i3_common::format_placeholders::expand_cstr(
        &format,
        &[
            (c"%title", &title),
            (c"%class", &class),
            (c"%instance", &instance),
            (c"%machine", &machine),
        ],
    );
    let formatted = i3_common::string::I3String::from_cstr(&expanded);
    formatted.set_markup(markup);
    formatted
}

pub unsafe fn con_swap(first: *mut Con, second: *mut Con) -> bool {
    if !first.is_null() {
    } else {
        __assert_fail(
            c"first != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2581 as ::core::ffi::c_uint,
            c"_Bool con_swap(Con *, Con *)".as_ptr(),
        );
    };
    if !second.is_null() {
    } else {
        __assert_fail(
            c"second != NULL".as_ptr(),
            c"../src/con.c".as_ptr(),
            2582 as ::core::ffi::c_uint,
            c"_Bool con_swap(Con *, Con *)".as_ptr(),
        );
    };
    ::log::debug!(
        "../src/con.c:con_swap:2583 - Swapping containers {} / {}\n",
        i3_common::logging::pointer(first),
        i3_common::logging::pointer(second)
    );
    if ((*first).type_0) != ContainerType::Con {
        ::log::error!(
            "ERROR: Only regular containers can be swapped, but found con = {} with type = {}.\n",
            i3_common::logging::pointer(first),
            (((*first).type_0).raw() as ::core::ffi::c_uint) as i32
        );
        return false;
    }
    if ((*second).type_0) != ContainerType::Con {
        ::log::error!(
            "ERROR: Only regular containers can be swapped, but found con = {} with type = {}.\n",
            i3_common::logging::pointer(second),
            (((*second).type_0).raw() as ::core::ffi::c_uint) as i32
        );
        return false;
    }
    if first == second {
        ::log::debug!(
            "../src/con.c:con_swap:2596 - Swapping container {} with itself, nothing to do.\n",
            i3_common::logging::pointer(first)
        );
        return false;
    }
    if con_has_parent(first, second) || con_has_parent(second, first) {
        ::log::error!("ERROR: Cannot swap containers {} and {} because they are in a parent-child relationship.\n", i3_common::logging::pointer(first), i3_common::logging::pointer(second));
        return false;
    }
    let ws1: *mut Con = con_get_workspace(first);
    let ws2: *mut Con = con_get_workspace(second);
    let mut restore_focus: *mut Con = ::core::ptr::null_mut::<Con>();
    if ws1 == ws2 && ws1 == con_get_workspace(focused) {
        restore_focus = focused;
    } else if first == focused || con_has_parent(focused, first) {
        restore_focus = second;
    } else if second == focused || con_has_parent(focused, second) {
        restore_focus = first;
    }
    nodes::swap(
        arena::identity(first).expect("live first child"),
        arena::identity(second).expect("live second child"),
    )
    .expect("swappable child slots");
    focus::swap(
        arena::identity(first).expect("live first focus child"),
        arena::identity(second).expect("live second focus child"),
    )
    .expect("swappable focus members");
    let tmp_SWAP: *mut Con = parent_ptr(first);
    set_parent_ptr(first, parent_ptr(second)).expect("live acyclic ancestry");
    set_parent_ptr(second, tmp_SWAP).expect("live acyclic ancestry");
    // Swap raw payload fields without introducing mutable references to node storage.
    std::ptr::swap(&raw mut (*first).floating, &raw mut (*second).floating);
    std::ptr::swap(&raw mut (*first).rect, &raw mut (*second).rect);
    std::ptr::swap(
        &raw mut (*first).window_rect,
        &raw mut (*second).window_rect,
    );
    std::ptr::swap(&raw mut (*first).percent, &raw mut (*second).percent);
    if !restore_focus.is_null() {
        con_focus(restore_focus);
    }
    con_set_urgency(first, (*first).urgent);
    con_set_urgency(second, (*second).urgent);
    let second_fullscreen_mode: FullscreenMode = (*second).fullscreen_mode;
    if ((*first).fullscreen_mode) == FullscreenMode::None {
        con_disable_fullscreen(second);
    } else {
        con_enable_fullscreen(second, (*first).fullscreen_mode);
    }
    if (second_fullscreen_mode) == FullscreenMode::None {
        con_disable_fullscreen(first);
    } else {
        con_enable_fullscreen(first, second_fullscreen_mode);
    }
    con_fix_percent(parent_ptr(first));
    con_fix_percent(parent_ptr(second));
    free((*first).deco_render_params as *mut ::core::ffi::c_void);
    (*first).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    free((*second).deco_render_params as *mut ::core::ffi::c_void);
    (*second).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
    con_force_split_parents_redraw(first);
    con_force_split_parents_redraw(second);
    true
}

pub unsafe fn con_rect_size_in_orientation(con: *mut Con) -> u32 {
    if con_orientation(con) == Orientation::Horizontal {
        (*con).rect.width
    } else {
        (*con).rect.height
    }
}

pub unsafe fn con_merge_into(old: *mut Con, new: *mut Con) {
    (*new).window = (*old).window;
    (*old).window = ::core::ptr::null_mut::<Window>();
    if let Some(format) = (*old).title_format.take() {
        (*new).title_format = Some(format);
    }
    if let Some(group) = (*old).sticky_group.take() {
        (*new).sticky_group = Some(group);
    }
    (*new).sticky = (*old).sticky;
    con_set_urgency(new, (*old).urgent);
    let mut mark: *mut mark_t = (*old).marks_head.tqh_first;

    while !mark.is_null() {
        (*mark).marks.tqe_next = ::core::ptr::null_mut::<mark_t>();
        (*mark).marks.tqe_prev = (*new).marks_head.tqh_last;
        *(*new).marks_head.tqh_last = mark;
        (*new).marks_head.tqh_last = &raw mut (*mark).marks.tqe_next;
        ipc_send_window_event(c"mark".as_ptr(), new);
        mark = (*mark).marks.tqe_next;
    }
    (*new).mark_changed = !(*old).marks_head.tqh_first.is_null();
    (*old).marks_head.tqh_first = ::core::ptr::null_mut::<mark_t>();
    (*old).marks_head.tqh_last = &raw mut (*old).marks_head.tqh_first;
    tree_close_internal(old, KillWindow::DontKill, false);
}

pub unsafe fn con_inside_stacked_or_tabbed(con: *mut Con) -> bool {
    if parent_ptr(con).is_null() {
        return false;
    }
    if (*parent_ptr(con)).layout == Layout::Stacked || (*parent_ptr(con)).layout == Layout::Tabbed {
        return true;
    }
    con_inside_stacked_or_tabbed(parent_ptr(con))
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_wm_desktop;
use crate::ipc::ipc_send_event;
use crate::render::render_deco_height;
use crate::util::layout_from_name;
use crate::util::max;
use i3_common::font::font_is_pango;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::strcasecmp_nullable;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{free, gettimeofday, strcasecmp, strcmp};

use i3_common::native_stdio::__assert_fail;

// Shared Rust model payload types.
pub use crate::types::deco_render_params;
pub use crate::types::gaps_t;
pub use crate::types::mark_t;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root_depth;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::config::config;
use crate::floating::floating_fix_coordinates;
use crate::floating::floating_raise_con;
use crate::gaps::calculate_effective_gaps;
use crate::ipc::ipc_marshal_workspace_event;
use crate::ipc::ipc_send_window_event;
use crate::output::get_output_for_con;
use crate::output::get_output_from_string;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::r#match::match_matches_window;
use crate::scratchpad::scratchpad_move;
use crate::startup::startup_sequence_delete_by_window;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::tree::tree_flatten;
use crate::util::rect_add;
use crate::util::rect_sub;
use crate::workspace::workspace_attach_to;
use crate::workspace::workspace_encapsulate;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::workspace::workspace_update_urgent_flag;
use crate::x::x_con_init;
use crate::x::x_set_warp_to;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;

#[cfg(test)]
mod owned_graph_tests {
    use super::*;
    #[test]
    fn border_geometry_preserves_wrapping_normal_title_edge_and_adjacency_masks() {
        let all =
            AdjacentEdges::LEFT | AdjacentEdges::RIGHT | AdjacentEdges::TOP | AdjacentEdges::BOTTOM;
        let plain = border_rect(BorderStyle::Pixel, 2, AdjacentEdges::empty());
        assert_eq!(
            (plain.x, plain.y, plain.width, plain.height),
            (2, 2, (-4i32) as u32, (-4i32) as u32)
        );
        let hidden = border_rect(BorderStyle::Pixel, 2, all);
        assert_eq!(
            (hidden.x, hidden.y, hidden.width, hidden.height),
            (0, 0, 0, 0)
        );
        let normal = border_rect(BorderStyle::Normal, 2, all);
        assert_eq!(
            (normal.x, normal.y, normal.width, normal.height),
            (0, 0, 0, 0)
        );
        let normal = border_rect(BorderStyle::Normal, 2, AdjacentEdges::TOP);
        assert_eq!((normal.y, normal.height), (0, (-2i32) as u32));
        let wrapped = Rect {
            x: u32::MAX,
            y: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(adjacent_borders(wrapped, wrapped), all);
        let right = Rect {
            x: 1,
            y: 9,
            width: 1,
            height: 3,
        };
        assert_eq!(adjacent_borders(right, wrapped), AdjacentEdges::RIGHT);
    }
    #[test]
    fn percentages_preserve_sequential_assignment_and_nan_comparisons() {
        let mut values = [0.5, 0.0, 0.0];
        normalize_percentages(&mut values);
        assert_eq!(values, [0.25, 0.25, 0.5]);
        let mut values = [-1.0, 0.0];
        normalize_percentages(&mut values);
        assert_eq!(values, [0.5, 0.5]);
        let mut values = [f64::NAN, 0.0];
        normalize_percentages(&mut values);
        assert!(values[0].is_nan());
        assert_eq!(values[1], 1.0);
        let mut values = [f64::NAN];
        normalize_percentages(&mut values);
        assert_eq!(values, [1.0]);
        normalize_percentages(&mut []);
    }
    #[test]
    fn fullscreen_breadth_first_excludes_root_and_enqueues_floating_last() {
        let root = arena::allocate(Con::empty());
        let tiling = arena::allocate(Con::empty());
        let floating = arena::allocate(Con::empty());
        let grandchild = arena::allocate(Con::empty());
        arena::floating::append(
            arena::identity(root).unwrap(),
            arena::identity(floating).unwrap(),
        )
        .unwrap();
        unsafe {
            (*root).fullscreen_mode = FullscreenMode::Output;
            nodes::append(
                arena::identity(root).unwrap(),
                arena::identity(tiling).unwrap(),
            )
            .unwrap();
            nodes::append(
                arena::identity(tiling).unwrap(),
                arena::identity(grandchild).unwrap(),
            )
            .unwrap();
            (*floating).fullscreen_mode = FullscreenMode::Output;
            (*grandchild).fullscreen_mode = FullscreenMode::Output;
            assert_eq!(
                con_get_fullscreen_con(root, FullscreenMode::Output),
                floating
            );
            (*tiling).fullscreen_mode = FullscreenMode::Output;
            assert_eq!(con_get_fullscreen_con(root, FullscreenMode::Output), tiling);
            (*tiling).fullscreen_mode = FullscreenMode::None;
            (*floating).fullscreen_mode = FullscreenMode::None;
            (*grandchild).fullscreen_mode = FullscreenMode::None;
            assert!(con_get_fullscreen_con(root, FullscreenMode::Output).is_null());
            for con in [root, tiling, floating, grandchild] {
                arena::release(con);
            }
        }
    }
    #[test]
    fn focus_snapshot_is_bounded_and_filters_workspace_floating_nodes() {
        let workspace = arena::allocate(Con::empty());
        let split = arena::allocate(Con::empty());
        let first = arena::allocate(Con::empty());
        let second = arena::allocate(Con::empty());
        let floating = arena::allocate(Con::empty());
        set_parent_ptr(split, workspace).unwrap();
        set_parent_ptr(first, split).unwrap();
        set_parent_ptr(second, split).unwrap();
        let id = |raw| arena::identity(raw).unwrap();
        focus::append(id(split), id(first)).unwrap();
        focus::append(id(split), id(second)).unwrap();
        unsafe {
            (*workspace).type_0 = ContainerType::Workspace;
            (*split).type_0 = ContainerType::Con;
            (*floating).type_0 = ContainerType::Floating;
            assert_eq!(get_focus_order(split), vec![id(first), id(second)]);
            set_focus_order(split, &[id(floating), id(second), id(first)]);
            assert_eq!(get_focus_order(split), vec![id(second), id(first)]);
            assert_eq!(focus::previous_ptr(first), second);
            assert_eq!(focus::last_ptr(split), first);
            for raw in [workspace, split, first, second, floating] {
                arena::release(raw);
            }
        }
    }
    #[test]
    fn tree_text_owns_ordered_bytes_and_malloc_adapter() {
        let root = arena::allocate(Con::empty());
        let first = arena::allocate(Con::empty());
        let second = arena::allocate(Con::empty());
        nodes::append(
            arena::identity(root).unwrap(),
            arena::identity(first).unwrap(),
        )
        .unwrap();
        nodes::append(
            arena::identity(root).unwrap(),
            arena::identity(second).unwrap(),
        )
        .unwrap();
        unsafe {
            (*root).layout = Layout::SplitH;
            assert_eq!(tree_representation(root), b"H[nowin nowin]");
            let text = con_get_tree_representation(root);
            assert_eq!(std::ffi::CStr::from_ptr(text).to_bytes(), b"H[nowin nowin]");
            libc::free(text.cast());
            for raw in [root, first, second] {
                arena::release(raw);
            }
        }
    }
    #[test]
    fn layout_toggle_retains_space_only_tokenization_aliases_errors_and_wrap() {
        assert_eq!(
            layout_toggle(c"split", Layout::Tabbed, Layout::Default).0,
            Some(Layout::SplitH)
        );
        assert_eq!(
            layout_toggle(c"split", Layout::SplitH, Layout::Default).0,
            Some(Layout::SplitV)
        );
        assert_eq!(
            layout_toggle(c"all", Layout::SplitV, Layout::SplitH).0,
            Some(Layout::Stacked)
        );
        assert_eq!(
            layout_toggle(c"default", Layout::Tabbed, Layout::SplitV).0,
            Some(Layout::SplitV)
        );
        let (next, invalid) = layout_toggle(
            c"  nonsense stacked invalid tabbed splitv",
            Layout::Stacked,
            Layout::SplitH,
        );
        assert_eq!(next, Some(Layout::Tabbed));
        assert_eq!(
            invalid
                .iter()
                .map(|token| token.as_bytes())
                .collect::<Vec<_>>(),
            [b"nonsense".as_slice(), b"invalid"]
        );
        assert_eq!(
            layout_toggle(c"stacked tabbed", Layout::Tabbed, Layout::SplitH).0,
            Some(Layout::Stacked)
        );
        assert_eq!(
            layout_toggle(c"stacked\ttabbed", Layout::SplitH, Layout::SplitH).0,
            None
        );
        let (next, invalid) = layout_toggle(c"default bad", Layout::SplitH, Layout::SplitH);
        assert_eq!(next, None);
        assert_eq!(invalid.len(), 2);
    }
    #[test]
    fn title_captures_escape_and_repair_only_for_markup() {
        assert_eq!(title_capture(b"<&\xff", false).as_bytes(), b"<&\xff");
        assert_eq!(
            title_capture(b"<&\xff", true).as_bytes(),
            "&lt;&amp;�".as_bytes()
        );
        let title = title_capture(b"title", false);
        let machine = title_capture(b"host<&", true);
        let text = i3_common::format_placeholders::expand_cstr(
            c"%machine:%title:%unknown",
            &[(c"%title", &title), (c"%machine", &machine)],
        );
        assert_eq!(text.as_bytes(), b"host&lt;&amp;:title:%unknown");
    }
}
