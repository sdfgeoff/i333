use crate::con::arena::focus;
use crate::con::arena::nodes;
use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{floating, identity, resolve};
use crate::domains::{ContainerType, FullscreenMode};
use crate::handlers::add_ignore_event;
use crate::types::Direction;
use crate::types::FocusWrapping;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::OutputSelection;
use crate::types::Position;
use xcb::{x, Cookie, XidNew};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * tree.c: Everything that primarily modifies the layout tree data structure.
 *
 */
// Mechanical file-by-file Rust translation of src/tree.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{ev_timer, ev_timer_stop};

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::ignore_event_sequence_t;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut croot: *mut Con = std::ptr::null_mut::<Con>();

pub static mut focused: *mut Con = std::ptr::null_mut::<Con>();

unsafe fn _create___i3() -> *mut Con {
    let mut __i3: *mut Con = con_new(croot, ::core::ptr::null_mut::<i3Window>());
    (*__i3).name = Some(b"__i3".to_vec());
    (*__i3).type_0 = ContainerType::Output;
    (*__i3).layout = Layout::Output;
    con_fix_percent(croot);
    x_set_name(__i3, b"[i3 con] pseudo-output __i3".to_vec());
    (*__i3).rect.width = 1280_u32;
    (*__i3).rect.height = 1024_u32;
    ::log::debug!("../src/tree.c:_create___i3:39 - adding main content container\n");
    let content: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*content).type_0 = ContainerType::Con;
    (*content).name = Some(b"content".to_vec());
    (*content).layout = Layout::SplitH;
    x_set_name(content, b"[i3 con] content __i3".to_vec());
    con_attach(content, __i3, false);
    let ws: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*ws).type_0 = ContainerType::Workspace;
    (*ws).num = -(1_i32);
    (*ws).name = Some(b"__i3_scratch".to_vec());
    (*ws).layout = Layout::SplitH;
    con_attach(ws, content, false);
    x_set_name(ws, b"[i3 con] workspace __i3_scratch".to_vec());
    (*ws).fullscreen_mode = FullscreenMode::Output;
    __i3
}

fn geometry_rect(x: i16, y: i16, width: u16, height: u16) -> Rect {
    Rect {
        x: x as u32,
        y: y as u32,
        width: u32::from(width),
        height: u32::from(height),
    }
}
fn flatten_layouts(con: Layout, child: Layout, parent: Orientation) -> bool {
    match (con, child) {
        (Layout::SplitH, Layout::SplitV) => parent == Orientation::Vertical,
        (Layout::SplitV, Layout::SplitH) => parent == Orientation::Horizontal,
        _ => false,
    }
}

pub unsafe fn tree_restore(path: &std::path::Path, geometry: &x::GetGeometryReply) -> bool {
    let mut out: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut result: bool = false;
    let path_owner = resolve_or_exit(path);
    if !exists(&path_owner) {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = path_owner.as_os_str().as_bytes().to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::info!("{} does not exist, not restoring tree\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        &log_value_0,
                        " does not exist, not restoring tree\n".as_bytes(),
                    ],
                );
            }
        };
    } else {
        if let Ok(mut bytes) = slurp(&path_owner) {
            let len = bytes.len();
            // Keep termination at the parser boundary without truncating embedded NULs.
            bytes.push(0);
            croot = con_new(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*croot).rect = geometry_rect(
                geometry.x(),
                geometry.y(),
                geometry.width(),
                geometry.height(),
            );
            focused = croot;
            tree_append_json(
                focused,
                bytes.as_ptr().cast(),
                len,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            ::log::debug!("../src/tree.c:tree_restore:93 - appended tree, using new root\n");
            croot = nodes::first_ptr(croot);
            if !croot.is_null() {
                ::log::debug!(
                    "../src/tree.c:tree_restore:99 - new root = {}\n",
                    i3_common::logging::pointer(croot)
                );
                out = nodes::first_ptr(croot);
                ::log::debug!(
                    "../src/tree.c:tree_restore:101 - out = {}\n",
                    i3_common::logging::pointer(out)
                );
                ws = nodes::first_ptr(out);
                ::log::debug!(
                    "../src/tree.c:tree_restore:103 - ws = {}\n",
                    i3_common::logging::pointer(ws)
                );
                if (*out).name.as_deref() != Some(b"__i3".as_slice()) {
                    ::log::debug!("../src/tree.c:tree_restore:108 - Adding pseudo-output __i3 during inplace restart\n");
                    let mut __i3: *mut Con = _create___i3();
                    nodes::move_to_front(
                        identity(croot).expect("live root"),
                        identity(__i3).expect("live internal output"),
                    )
                    .expect("internal output child");
                }
                restore_open_placeholder_windows(croot);
                result = true;
            }
        }
    }
    result
}

pub unsafe fn tree_init(geometry: &x::GetGeometryReply) {
    croot = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*croot).name = Some(b"root".to_vec());
    (*croot).type_0 = ContainerType::Root;
    (*croot).layout = Layout::SplitH;
    (*croot).rect = geometry_rect(
        geometry.x(),
        geometry.y(),
        geometry.width(),
        geometry.height(),
    );
    _create___i3();
}

pub unsafe fn tree_open_con(mut con: *mut Con, window: *mut i3Window) -> *mut Con {
    if con.is_null() {
        con = parent_ptr(focused);
        if ((*parent_ptr(con)).type_0) == ContainerType::Output
            && ((*con).type_0) != ContainerType::DockArea
        {
            con = focused;
        }
        if ((*con).type_0) == ContainerType::Floating {
            con = con_descend_tiling_focused(parent_ptr(con));
            if ((*con).type_0) != ContainerType::Workspace {
                con = parent_ptr(con);
            }
        }
        ::log::debug!(
            "../src/tree.c:tree_open_con:168 - con = {}\n",
            i3_common::logging::pointer(con)
        );
    }
    if !con.is_null() {
    } else {
        __assert_fail(
            c"con != NULL".as_ptr(),
            c"../src/tree.c".as_ptr(),
            171_u32,
            c"Con *tree_open_con(Con *, i3Window *)".as_ptr(),
        );
    };
    let new: *mut Con = con_new(con, window);
    (*new).layout = Layout::SplitH;
    con_fix_percent(con);
    new
}

pub unsafe fn tree_close_internal(
    con: *mut Con,
    kill_window: KillWindow,
    dont_kill_parent: bool,
) -> bool {
    let parent: *mut Con = parent_ptr(con);
    if (*con).urgent {
        con_set_urgency(con, false);
        con_update_parents_urgency(con);
        workspace_update_urgent_flag(con_get_workspace(con));
    }
    ::log::debug!(
        "../src/tree.c:tree_close_internal:202 - closing {}, kill_window = {}\n",
        i3_common::logging::pointer(con),
        (kill_window as u32) as i32
    );
    let mut abort_kill: bool = false;
    let owner_id = identity(con).expect("live closing container");
    let mut child_id = nodes::first(owner_id);
    while let Some(id) = child_id {
        let next_child = nodes::next(owner_id, id);
        let Some(child) = resolve(id) else { break };
        ::log::debug!(
            "../src/tree.c:tree_close_internal:208 - killing child={}\n",
            i3_common::logging::pointer(child)
        );
        if !tree_close_internal(child, kill_window, true) {
            abort_kill = true;
        }
        child_id = next_child;
    }
    if abort_kill {
        ::log::debug!("../src/tree.c:tree_close_internal:216 - One of the children could not be killed immediately (WM_DELETE sent), aborting.\n");
        return false;
    }
    if !(*con).window.is_null() {
        if kill_window != KillWindow::DontKill {
            x_window_kill((*(*con).window).id, kill_window);
            return false;
        }

        connection().send_request(&x::ChangeWindowAttributes {
            window: x::Window::new((*(*con).window).id),
            value_list: &[x::Cw::EventMask(x::EventMask::empty())],
        });
        connection().send_request(&x::UnmapWindow {
            window: x::Window::new((*(*con).window).id),
        });
        let mut cookie = connection().send_request(&x::ReparentWindow {
            window: x::Window::new((*(*con).window).id),
            parent: x::Window::new(root),
            x: (*con).rect.x as i16,
            y: (*con).rect.y as i16,
        });
        add_ignore_event(
            ignore_event_sequence_t {
                // The existing ignored-event cache stores native full_sequence's low 32 bits.
                value: cookie.sequence() as u32,
            },
            0_i32,
        );
        cookie = connection().send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new((*(*con).window).id),
            property: x::Atom::new(A_WM_STATE),
            r#type: x::Atom::new(A_WM_STATE),
            data: &[(crate::domains::IcccmState::Withdrawn as u32), 0],
        });
        connection().send_request(&x::ChangeSaveSet {
            mode: x::SetMode::Delete,
            window: x::Window::new((*(*con).window).id),
        });
        if shape_supported {
            connection().send_request(&xcb::shape::SelectInput {
                destination_window: x::Window::new((*(*con).window).id),
                enable: false,
            });
        }
        add_ignore_event(
            ignore_event_sequence_t {
                // The existing ignored-event cache stores native full_sequence's low 32 bits.
                value: cookie.sequence() as u32,
            },
            0_i32,
        );
        ipc_send_window_event("close", con);
        window_free((*con).window as *mut i3Window);
        (*con).window = ::core::ptr::null_mut::<Window>();
    }
    let ws: *mut Con = con_get_workspace(con);
    let next: *mut Con = if con == focused {
        con_next_focused(con)
    } else {
        ::core::ptr::null_mut::<Con>()
    };
    // Focus is deferred past teardown and its callbacks; generation checking
    // prevents a deleted/reused address from becoming an unintended target.
    let next_id = crate::con::arena::identity(next);
    ::log::debug!(
        "../src/tree.c:tree_close_internal:267 - next = {}, focused = {}\n",
        i3_common::logging::pointer(next),
        i3_common::logging::pointer(focused)
    );
    con_detach(con);
    if !(*con).urgency_timer.is_null() {
        ::log::debug!(
            "../src/tree.c:tree_close_internal:274 - Removing urgency timer of con {}\n",
            i3_common::logging::pointer(con)
        );
        workspace_update_urgent_flag(ws);
        ev_timer_stop(main_loop, (*con).urgency_timer);
        free((*con).urgency_timer as *mut ::core::ffi::c_void);
        (*con).urgency_timer = ::core::ptr::null_mut::<ev_timer>();
    }
    if ((*con).type_0) != ContainerType::Floating {
        con_fix_percent(parent);
    }
    if !dont_kill_parent {
        tree_render();
    }
    x_con_kill(con);
    if ws == con {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_owner_1 = (*ws).name.clone();
            let log_value_0 = i3_common::logging::string_bytes(log_owner_1.as_deref(), None);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                ::log::debug!("../src/tree.c:tree_close_internal:302 - Closing workspace container {}, updating EWMH atoms\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Debug,
                    &[
                        "../src/tree.c:tree_close_internal:302 - Closing workspace container "
                            .as_bytes(),
                        log_value_0,
                        ", updating EWMH atoms\n".as_bytes(),
                    ],
                );
            }
        };
        ewmh_update_desktop_properties();
    }
    con_free(con);
    if let Some(next) = next_id.and_then(crate::con::arena::resolve) {
        con_activate(next);
    } else {
        ::log::debug!("../src/tree.c:tree_close_internal:311 - not changing focus, the container was not focused before\n");
    }
    if !dont_kill_parent {
        (*parent)
            .on_remove_child
            .expect("non-null function pointer")(parent);
    }
    true
}

pub unsafe fn tree_split(mut con: *mut Con, orientation: Orientation) {
    if con_is_floating(con) {
        ::log::debug!("../src/tree.c:tree_split:329 - Floating containers can't be split.\n");
        return;
    }
    if ((*con).type_0) == ContainerType::Workspace {
        if con_num_children(con) < 2_i32 {
            if con_num_children(con) == 0_i32 {
                ::log::debug!(
                    "../src/tree.c:tree_split:336 - Changing workspace_layout to L_DEFAULT\n"
                );
                (*con).workspace_layout = Layout::Default;
            }
            ::log::debug!("../src/tree.c:tree_split:339 - Changing orientation of workspace\n");
            (*con).layout = if orientation == Orientation::Horizontal {
                Layout::SplitH
            } else {
                Layout::SplitV
            };
            return;
        } else {
            con = workspace_encapsulate(con);
        }
    }
    let parent: *mut Con = parent_ptr(con);
    con_force_split_parents_redraw(con);
    if con_num_children(parent) == 1_i32
        && ((*parent).layout == Layout::SplitH || (*parent).layout == Layout::SplitV)
    {
        (*parent).layout = if orientation == Orientation::Horizontal {
            Layout::SplitH
        } else {
            Layout::SplitV
        };
        ::log::debug!(
            "../src/tree.c:tree_split:361 - Just changing orientation of existing container\n"
        );
        return;
    }
    ::log::debug!(
        "../src/tree.c:tree_split:365 - Splitting in orientation {}\n",
        (orientation as u32) as i32
    );
    let new: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    nodes::replace(
        identity(parent).expect("live parent"),
        identity(con).expect("live split child"),
        identity(new).expect("live split wrapper"),
    )
    .expect("replace child slot");
    focus::replace(
        identity(parent).expect("live focus parent"),
        identity(con).expect("live split child"),
        identity(new).expect("live split wrapper"),
    )
    .expect("replace split focus slot");
    set_parent_ptr(new, parent).expect("live acyclic ancestry");
    (*new).layout = if orientation == Orientation::Horizontal {
        Layout::SplitH
    } else {
        Layout::SplitV
    };
    (*new).percent = (*con).percent;
    (*con).percent = 0.0f64;
    con_attach(con, new, false);
}

pub unsafe fn level_up() -> bool {
    if ((*parent_ptr(focused)).type_0) == ContainerType::Floating {
        con_activate(parent_ptr(parent_ptr(focused)));
        return true;
    }
    if ((*parent_ptr(focused)).type_0) != ContainerType::Con
        && ((*parent_ptr(focused)).type_0) != ContainerType::Workspace
        || ((*focused).type_0) == ContainerType::Workspace
    {
        ::log::error!("ERROR: 'focus parent': Focus is already on the workspace, cannot go higher than that.\n");
        return false;
    }
    con_activate(parent_ptr(focused));
    true
}

pub unsafe fn level_down() -> bool {
    let mut next: *mut Con = focus::first_ptr(focused);
    if next.is_null() {
        ::log::debug!("../src/tree.c:level_down:413 - cannot go down\n");
        return false;
    } else if ((*next).type_0) == ContainerType::Floating {
        let child: *mut Con = focus::first_ptr(next);
        if child.is_null() {
            ::log::debug!("../src/tree.c:level_down:420 - cannot go down\n");
            return false;
        } else {
            next = focus::first_ptr(next);
        }
    }
    con_activate(next);
    true
}
unsafe fn mark_unmapped(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    (*con).mapped = false;
    current = nodes::first_ptr(con);
    while !current.is_null() {
        mark_unmapped(current);
        current = nodes::next_ptr(current);
    }
    if ((*con).type_0) == ContainerType::Workspace {
        for id in identity(con).into_iter().flat_map(floating::children) {
            if let Some(current) = resolve(id) {
                mark_unmapped(current);
            }
        }
    }
}

pub unsafe fn tree_render() {
    if croot.is_null() {
        return;
    }
    ::log::debug!("../src/tree.c:tree_render:457 - -- BEGIN RENDERING --\n");
    mark_unmapped(croot);
    (*croot).mapped = true;
    render_con(croot);
    x_push_changes(croot);
    ::log::debug!("../src/tree.c:tree_render:466 - -- END RENDERING --\n");
}
unsafe fn get_tree_next_workspace(con: *mut Con, direction: Direction) -> *mut Con {
    if !con_get_fullscreen_con(con, FullscreenMode::Global).is_null() {
        ::log::debug!("../src/tree.c:get_tree_next_workspace:471 - Cannot change workspace while in global fullscreen mode.\n");
        return ::core::ptr::null_mut::<Con>();
    }
    let x: u32 = (*con)
        .rect
        .x
        .wrapping_add((*con).rect.width.wrapping_div(2_u32));
    let y: u32 = (*con)
        .rect
        .y
        .wrapping_add((*con).rect.height.wrapping_div(2_u32));
    let current_output: *mut Output = get_output_containing(x, y);
    if current_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = output_primary_name(current_output);
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/tree.c:get_tree_next_workspace:483 - Current output is {}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/tree.c:get_tree_next_workspace:483 - Current output is ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let next_output: *mut Output =
        get_output_next(direction, current_output, OutputSelection::Closest);
    if next_output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = output_primary_name(next_output);
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/tree.c:get_tree_next_workspace:489 - Next output is {}\n",
                log_value_0_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    "../src/tree.c:get_tree_next_workspace:489 - Next output is ".as_bytes(),
                    &log_value_0,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    let mut child = nodes::first_ptr(output_get_content((*next_output).con));
    while !child.is_null() {
        if workspace_is_visible(child) {
            return child;
        }
        child = nodes::next_ptr(child);
    }
    std::ptr::null_mut()
}

unsafe fn get_tree_next(mut con: *mut Con, direction: Direction) -> *mut Con {
    let previous: bool = position_from_direction(direction) == Position::Before;
    let orientation: Orientation = orientation_from_direction(direction);
    let mut first_wrap: *mut Con = ::core::ptr::null_mut::<Con>();
    if !(((*con).type_0) == ContainerType::Workspace) {
        while ((*con).type_0) != ContainerType::Workspace {
            if ((*con).fullscreen_mode) == FullscreenMode::Output {
                con = con_get_workspace(con);
                break;
            } else {
                if ((*con).fullscreen_mode) == FullscreenMode::Global {
                    return first_wrap;
                }
                let parent: *mut Con = parent_ptr(con);
                if ((*con).type_0) == ContainerType::Floating {
                    if orientation != Orientation::Horizontal {
                        return ::core::ptr::null_mut::<Con>();
                    }
                    let parent_id = identity(parent).expect("live floating parent");
                    let child_id = identity(con).expect("live floating child");
                    let next = if previous {
                        floating::previous(parent_id, child_id)
                            .or_else(|| floating::last(parent_id))
                    } else {
                        floating::next(parent_id, child_id).or_else(|| floating::first(parent_id))
                    }
                    .and_then(resolve)
                    .unwrap_or(std::ptr::null_mut());
                    if !next.is_null() {
                    } else {
                        __assert_fail(
                            c"next".as_ptr(),
                            c"../src/tree.c".as_ptr(),
                            543_u32,
                            c"Con *get_tree_next(Con *, direction_t)".as_ptr(),
                        );
                    };
                    return next;
                }
                if con_num_children(parent) > 1_i32 && con_orientation(parent) == orientation {
                    let next_0: *mut Con = if previous {
                        nodes::previous_ptr(con)
                    } else {
                        nodes::next_ptr(con)
                    };
                    if !next_0.is_null() && con_fullscreen_permits_focusing(next_0) {
                        return next_0;
                    }
                    let wrap: *mut Con = if previous {
                        nodes::last_ptr(parent)
                    } else {
                        nodes::first_ptr(parent)
                    };
                    match config.focus_wrapping {
                        FocusWrapping::Workspace | FocusWrapping::On => {
                            if first_wrap.is_null() && con_fullscreen_permits_focusing(wrap) {
                                first_wrap = wrap;
                            }
                        }
                        FocusWrapping::Force => {
                            if !next_0.is_null() {
                                return ::core::ptr::null_mut::<Con>();
                            }
                            return if con_fullscreen_permits_focusing(wrap) {
                                wrap
                            } else {
                                ::core::ptr::null_mut::<Con>()
                            };
                        }
                        FocusWrapping::Off => {}
                    }
                }
                con = parent;
            }
        }
        if ((*con).type_0) == ContainerType::Workspace {
        } else {
            __assert_fail(
                c"con->type == CT_WORKSPACE".as_ptr(),
                c"../src/tree.c".as_ptr(),
                579_u32,
                c"Con *get_tree_next(Con *, direction_t)".as_ptr(),
            );
        };
        if config.focus_wrapping == FocusWrapping::Workspace {
            return first_wrap;
        }
    }
    let workspace: *mut Con = get_tree_next_workspace(con, direction);
    if !workspace.is_null() {
        workspace
    } else {
        first_wrap
    }
}

pub unsafe fn tree_next(con: *mut Con, direction: Direction) {
    let next: *mut Con = get_tree_next(con, direction);
    if next.is_null() {
        return;
    }
    if ((*next).type_0) == ContainerType::Workspace {
        let mut focus: *mut Con = con_descend_focused(next);
        if ((*focus).fullscreen_mode) == FullscreenMode::None {
            let focus_tiling: *mut Con = con_descend_tiling_focused(next);
            if focus_tiling != next {
                focus = focus_tiling;
            }
        }
        workspace_show(next);
        con_activate(focus);
        x_set_warp_to(&raw mut (*focus).rect);
        return;
    } else if ((*next).type_0) == ContainerType::Floating {
        let parent: *mut Con = parent_ptr(next);
        floating::rotate_after(
            identity(parent).expect("live floating parent"),
            identity(next).expect("live floating child"),
        )
        .expect("attached floating child");
    }
    workspace_show(con_get_workspace(next));
    con_activate(con_descend_focused(next));
}

pub unsafe fn get_tree_next_sibling(con: *mut Con, direction: Position) -> *mut Con {
    let to_focus: *mut Con = if direction == Position::Before {
        nodes::previous_ptr(con)
    } else {
        nodes::next_ptr(con)
    };
    if !to_focus.is_null() && con_fullscreen_permits_focusing(to_focus) {
        return to_focus;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn tree_flatten(con: *mut Con) {
    let mut focus_next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let parent: *mut Con = parent_ptr(con);
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_owner_2 = (*con).name.clone();
        let log_value_1 = i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
            ::log::debug!(
                "../src/tree.c:tree_flatten:659 - Checking if I can flatten con = {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/tree.c:tree_flatten:659 - Checking if I can flatten con = {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if !(((*con).type_0) != ContainerType::Con
        || (*parent).layout == Layout::Output
        || !(*con).window.is_null())
    {
        child = nodes::first_ptr(con);
        if !(child.is_null() || !nodes::next_ptr(child).is_null()) {
            ::log::debug!(
                "../src/tree.c:tree_flatten:674 - child = {}, con = {}, parent = {}\n",
                i3_common::logging::pointer(child),
                i3_common::logging::pointer(con),
                i3_common::logging::pointer(parent)
            );
            if con_is_split(con)
                && con_is_split(child)
                && flatten_layouts((*con).layout, (*child).layout, con_orientation(parent))
            {
                ::log::debug!("../src/tree.c:tree_flatten:687 - Alright, I have to flatten this situation now. Stay calm.\n");
                focus_next = focus::first_ptr(child);
                ::log::debug!("../src/tree.c:tree_flatten:691 - detaching...\n");
                while !nodes::first_ptr(child).is_null() {
                    current = nodes::first_ptr(child);
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = i3_common::logging::pointer(current);
                        let log_owner_2 = (*current).name.clone();
                        let log_value_1 =
                            i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
                        if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                            ::log::debug!(
                                "../src/tree.c:tree_flatten:695 - detaching current={} / {}\n",
                                log_value_0,
                                log_value_1_text
                            );
                        } else {
                            i3_common::logging::emit_parts(
                                i3_common::logging::Level::Debug,
                                &[
                                    format!(
                                        "../src/tree.c:tree_flatten:695 - detaching current={} / ",
                                        log_value_0
                                    )
                                    .as_bytes(),
                                    log_value_1,
                                    "\n".as_bytes(),
                                ],
                            );
                        }
                    };
                    con_detach(current);
                    ::log::debug!("../src/tree.c:tree_flatten:697 - re-attaching\n");
                    set_parent_ptr(current, parent).expect("live acyclic ancestry");
                    nodes::insert_before(
                        identity(parent).expect("live flatten parent"),
                        identity(current).expect("live lifted child"),
                        identity(con).expect("live obsolete wrapper"),
                    )
                    .expect("detached lifted child");
                    ::log::debug!("../src/tree.c:tree_flatten:704 - attaching to focus list\n");
                    focus::append(
                        identity(parent).expect("live focus parent"),
                        identity(current).expect("live lifted child"),
                    )
                    .expect("detached lifted focus child");
                    (*current).percent = (*con).percent;
                }
                ::log::debug!("../src/tree.c:tree_flatten:708 - re-attached all\n");
                if !focus_next.is_null() && focus::first_ptr(parent) == con {
                    ::log::debug!(
                        "../src/tree.c:tree_flatten:713 - restoring focus to focus_next={}\n",
                        i3_common::logging::pointer(focus_next)
                    );
                    focus::move_to_front(
                        identity(parent).expect("live focus parent"),
                        identity(focus_next).expect("live remembered focus child"),
                    )
                    .expect("lifted focus child");
                    ::log::debug!("../src/tree.c:tree_flatten:716 - restored focus.\n");
                }
                ::log::debug!("../src/tree.c:tree_flatten:720 - closing redundant cons\n");
                tree_close_internal(con, KillWindow::DontKill, true);
                return;
            }
        }
    }
    if let Some(owner_id) = identity(con) {
        let mut current_id = nodes::first(owner_id);
        while let Some(id) = current_id {
            let next = nodes::next(owner_id, id);
            if let Some(current) = resolve(id) {
                tree_flatten(current);
            }
            current_id = next;
        }
    }
    if let Some(parent_id) = identity(con) {
        let mut current = floating::first(parent_id);
        while let Some(id) = current {
            // This traversal intentionally captures the successor before
            // recursion can flatten/delete/reparent the current container.
            let next = floating::next(parent_id, id);
            if let Some(con) = resolve(id) {
                tree_flatten(con);
            }
            current = next;
        }
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_desktop_properties;
use crate::util::orientation_from_direction;
use crate::util::position_from_direction;
use crate::util::slurp;
use crate::x::x_window_kill;
use i3_common::path_exists::exists;
use i3_common::safewrappers::sstrdup;
use std::os::unix::ffi::OsStrExt;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{free, strcmp};

use i3_common::native_stdio::__assert_fail;

// Shared Rust model payload types.
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::root;
use crate::application::shape_supported;
use crate::application::A_WM_STATE;
use crate::con::con_activate;
use crate::con::con_attach;
use crate::con::con_descend_focused;
use crate::con::con_descend_tiling_focused;
use crate::con::con_detach;
use crate::con::con_fix_percent;
use crate::con::con_force_split_parents_redraw;
use crate::con::con_free;
use crate::con::con_fullscreen_permits_focusing;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_workspace;
use crate::con::con_is_floating;
use crate::con::con_is_split;
use crate::con::con_new;
use crate::con::con_next_focused;
use crate::con::con_num_children;
use crate::con::con_orientation;
use crate::con::con_set_urgency;
use crate::con::con_update_parents_urgency;
use crate::config::config;
use crate::ipc::ipc_send_window_event;
use crate::load_layout::tree_append_json;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::randr::get_output_containing;
use crate::randr::get_output_next;
use crate::render::render_con;
use crate::restore_layout::restore_open_placeholder_windows;
use crate::window::window_free;
use crate::workspace::workspace_encapsulate;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::workspace::workspace_update_urgent_flag;
use crate::x::x_con_kill;
use crate::x::x_push_changes;
use crate::x::x_set_name;
use crate::x::x_set_warp_to;
use i3_common::resolve_tilde::resolve_or_exit;

fn connection() -> std::rc::Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
}

#[cfg(test)]
mod owned_policy_tests {
    use super::*;
    #[test]
    fn restore_geometry_keeps_native_signed_coordinate_wrapping() {
        let rect = geometry_rect(-1, -32768, 65535, 0);
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (u32::MAX, (-32768i32) as u32, 65535, 0)
        );
    }
    #[test]
    fn flatten_requires_opposite_split_and_matching_parent_orientation() {
        assert!(flatten_layouts(
            Layout::SplitH,
            Layout::SplitV,
            Orientation::Vertical
        ));
        assert!(flatten_layouts(
            Layout::SplitV,
            Layout::SplitH,
            Orientation::Horizontal
        ));
        for con in [
            Layout::SplitH,
            Layout::SplitV,
            Layout::Tabbed,
            Layout::Stacked,
        ] {
            assert!(!flatten_layouts(con, con, Orientation::Horizontal));
            assert!(!flatten_layouts(con, Layout::Tabbed, Orientation::Vertical));
        }
        assert!(!flatten_layouts(
            Layout::SplitH,
            Layout::SplitV,
            Orientation::Horizontal
        ));
    }
}
