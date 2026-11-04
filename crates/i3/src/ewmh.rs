/* i3 - EWMH property updates. See LICENSE. */
//! Property payloads and index decisions use owned Rust values. The native
//! setters and tree traversals remain explicit event-thread unsafe boundaries.
use crate::con::arena::{floating, identity, resolve};
use crate::domains::ContainerType;
use ::xcb::{x, Xid, XidNew};
use i3_common::display::connection;
use std::ffi::CStr;
use std::ptr;

pub const NET_WM_DESKTOP_NONE: u32 = 0xfffffff0;
pub const NET_WM_DESKTOP_ALL: u32 = u32::MAX;
pub static mut ewmh_window: u32 = 0;

struct WorkspaceEntry {
    con: *mut Con,
    internal: bool,
    viewport: [u32; 2],
}

/// # Safety
/// The event thread owns a live, stable container graph. Native helper calls
/// must not remove its nodes during traversal. No borrowed graph data escapes.
unsafe fn workspace_entries() -> Vec<WorkspaceEntry> {
    let mut entries = Vec::new();
    let mut output = nodes::first_ptr(croot);
    while !output.is_null() {
        let mut workspace = nodes::first_ptr(output_get_content(output));
        while !workspace.is_null() {
            entries.push(WorkspaceEntry {
                con: workspace,
                internal: con_is_internal(workspace),
                viewport: [(*output).rect.x, (*output).rect.y],
            });
            workspace = nodes::next_ptr(workspace);
        }
        output = nodes::next_ptr(output);
    }
    entries
}

fn public_workspace_index<T: PartialEq>(
    entries: impl IntoIterator<Item = (T, bool)>,
    target: T,
) -> u32 {
    let mut index = 0u32;
    for (workspace, internal) in entries {
        if !internal {
            if workspace == target {
                return index;
            }
            index = index.wrapping_add(1);
        }
    }
    NET_WM_DESKTOP_NONE
}
fn public_workspace_at<T>(entries: impl IntoIterator<Item = (T, bool)>, index: u32) -> Option<T> {
    if index == NET_WM_DESKTOP_NONE {
        return None;
    }
    let mut current = 0u32;
    for (workspace, internal) in entries {
        if !internal {
            if current == index {
                return Some(workspace);
            }
            current = current.wrapping_add(1);
        }
    }
    None
}
// Internal workspaces are updated at the current index without consuming it.
fn workspace_desktop_indices<T>(entries: impl IntoIterator<Item = (T, bool)>) -> Vec<(T, u32)> {
    let mut index = 0u32;
    entries
        .into_iter()
        .map(|(workspace, internal)| {
            let current = index;
            if !internal {
                index = index.wrapping_add(1);
            }
            (workspace, current)
        })
        .collect()
}
fn desktop_names<'a>(names: impl IntoIterator<Item = &'a CStr>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for name in names {
        bytes.extend_from_slice(name.to_bytes_with_nul());
    }
    bytes
}
fn desktop_viewports(origins: impl IntoIterator<Item = [u32; 2]>) -> Vec<u32> {
    let mut words = Vec::new();
    for origin in origins {
        words.extend_from_slice(&origin);
    }
    words
}
fn current_desktop_changed(previous: u32, desktop: u32) -> bool {
    desktop != previous && desktop != NET_WM_DESKTOP_NONE
}
#[derive(Debug, PartialEq, Eq)]
enum DesktopProperty {
    Set(u32),
    Delete,
}
fn window_desktop(desktop: u32, sticky: bool, floating: bool, internal: bool) -> u32 {
    if sticky && floating || internal {
        NET_WM_DESKTOP_ALL
    } else {
        desktop
    }
}
fn desktop_property_change(previous: u32, desktop: u32) -> Option<DesktopProperty> {
    if previous == desktop {
        None
    } else if desktop == NET_WM_DESKTOP_NONE {
        Some(DesktopProperty::Delete)
    } else {
        Some(DesktopProperty::Set(desktop))
    }
}

/// # Safety
/// conn is this event thread's live XCB connection. XCB copies the bounded
/// payload synchronously; neither pointer nor graph references are retained.
unsafe fn set_bytes(window: u32, property: u32, type_: u32, bytes: &[u8]) {
    let _ = u32::try_from(bytes.len()).expect("EWMH byte payload exceeds X11 property limit");
    let Some(connection) = connection() else {
        return;
    };
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(window),
        property: x::Atom::new(property),
        r#type: x::Atom::new(type_),
        data: bytes,
    });
}
/// # Safety
/// Same connection/lifetime contract as set_bytes. Typed XCB serializes words
/// according to the connection's negotiated byte order.
unsafe fn set_words(window: u32, property: u32, type_: u32, words: &[u32]) {
    let _ = u32::try_from(words.len()).expect("EWMH word payload exceeds X11 property limit");
    let Some(connection) = connection() else {
        return;
    };
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(window),
        property: x::Atom::new(property),
        r#type: x::Atom::new(type_),
        data: words,
    });
}
/// # Safety
/// The event thread owns the live connection and the supplied window.
unsafe fn delete_property(window: u32, property: u32) {
    let Some(connection) = connection() else {
        return;
    };
    connection.send_request(&x::DeleteProperty {
        window: x::Window::new(window),
        property: x::Atom::new(property),
    });
}

/// # Safety
/// Called by the native event thread with a live connection and focused tree.
pub unsafe fn ewmh_update_current_desktop() {
    static mut PREVIOUS: u32 = NET_WM_DESKTOP_NONE;
    let index = ewmh_get_workspace_index(focused);
    if current_desktop_changed(PREVIOUS, index) {
        PREVIOUS = index;
        set_words(
            root,
            A__NET_CURRENT_DESKTOP,
            x::ATOM_CARDINAL.resource_id(),
            &[index],
        );
    }
}
unsafe fn ewmh_update_number_of_desktops() {
    static mut PREVIOUS: u32 = 0;
    let count = workspace_entries()
        .iter()
        .filter(|workspace| !workspace.internal)
        .fold(0u32, |count, _| count.wrapping_add(1));
    if count != PREVIOUS {
        PREVIOUS = count;
        set_words(
            root,
            A__NET_NUMBER_OF_DESKTOPS,
            x::ATOM_CARDINAL.resource_id(),
            &[count],
        );
    }
}
unsafe fn ewmh_update_desktop_names() {
    // Copy native names before submitting the property; preserve raw bytes and
    // one NUL per name, including an empty name and the last name.
    let names: Vec<_> = workspace_entries()
        .into_iter()
        .filter(|workspace| !workspace.internal)
        .map(|workspace| CStr::from_ptr((*workspace.con).name_ptr()).to_owned())
        .collect();
    let payload = desktop_names(names.iter().map(|name| name.as_c_str()));
    set_bytes(root, A__NET_DESKTOP_NAMES, A_UTF8_STRING, &payload);
}
unsafe fn ewmh_update_desktop_viewport() {
    let payload = desktop_viewports(
        workspace_entries()
            .into_iter()
            .filter(|workspace| !workspace.internal)
            .map(|workspace| workspace.viewport),
    );
    set_words(
        root,
        A__NET_DESKTOP_VIEWPORT,
        x::ATOM_CARDINAL.resource_id(),
        &payload,
    );
}
/// # Safety
/// The event thread owns the live tree and connection throughout all updates.
pub unsafe fn ewmh_update_desktop_properties() {
    // Keep request order and independent cache updates identical to upstream.
    ewmh_update_number_of_desktops();
    ewmh_update_desktop_viewport();
    ewmh_update_current_desktop();
    ewmh_update_desktop_names();
    ewmh_update_wm_desktop();
}

/// # Safety
/// con is a live graph node on the event thread. Traversal, cached window
/// mutation, and XCB submission are exclusive; logging must not mutate the tree.
unsafe fn ewmh_update_wm_desktop_recursively(con: *mut Con, desktop: u32) {
    let mut child = nodes::first_ptr(con);
    while !child.is_null() {
        ewmh_update_wm_desktop_recursively(child, desktop);
        child = nodes::next_ptr(child);
    }
    if (*con).type_0 == ContainerType::Workspace {
        for floating_id in floating::children(identity(con).expect("live floating parent")) {
            let Some(child) = resolve(floating_id) else {
                continue;
            };
            ewmh_update_wm_desktop_recursively(child, desktop);
        }
    }
    if !con_has_managed_window(con) {
        return;
    }
    let workspace = con_get_workspace(con);
    let sticky = con_is_sticky(con);
    let floating = sticky && con_is_floating(con);
    let desktop = window_desktop(
        desktop,
        sticky,
        floating,
        !workspace.is_null() && con_is_internal(workspace),
    );
    let Some(change) = desktop_property_change((*(*con).window).wm_desktop, desktop) else {
        return;
    };
    // Cache before issuing the native request, including the deletion sentinel.
    (*(*con).window).wm_desktop = desktop;
    let window = (*(*con).window).id;
    match change {
        DesktopProperty::Set(desktop) => {
            ::log::debug!("../src/ewmh.c:ewmh_update_wm_desktop_recursively:{} - Setting _NET_WM_DESKTOP = {} for window 0x{:08x}.\n", { (171i32) }, (desktop) as i32, { (window) });
            set_words(
                window,
                A__NET_WM_DESKTOP,
                x::ATOM_CARDINAL.resource_id(),
                &[desktop],
            );
        }
        DesktopProperty::Delete => {
            ::log::error!("ERROR: Failed to determine the proper EWMH desktop index for window 0x{:08x}, deleting _NET_WM_DESKTOP.\n", { (window) });
            delete_property(window, A__NET_WM_DESKTOP);
        }
    }
}
/// # Safety
/// The event thread owns the live workspace graph and native connection.
pub unsafe fn ewmh_update_wm_desktop() {
    let entries = workspace_entries()
        .into_iter()
        .map(|entry| (entry.con, entry.internal));
    for (workspace, desktop) in workspace_desktop_indices(entries) {
        ewmh_update_wm_desktop_recursively(workspace, desktop);
    }
}
/// # Safety
/// conn/root are live native event-thread resources.
pub unsafe fn ewmh_update_active_window(window: u32) {
    set_words(
        root,
        A__NET_ACTIVE_WINDOW,
        x::ATOM_WINDOW.resource_id(),
        &[window],
    );
}
/// # Safety
/// conn is the event thread's live XCB connection. The optional name bytes
/// are copied synchronously; None deletes the property and Some(empty) sets it.
pub unsafe fn ewmh_update_visible_name(window: u32, name: Option<&[u8]>) {
    if let Some(name) = name {
        set_bytes(window, A__NET_WM_VISIBLE_NAME, A_UTF8_STRING, name);
    } else {
        delete_property(window, A__NET_WM_VISIBLE_NAME);
    }
}
/// # Safety
/// conn/root are live native event-thread resources.
pub unsafe fn ewmh_update_workarea() {
    delete_property(root, A__NET_WORKAREA);
}
/// # Safety
/// list points to num_windows readable window IDs when nonzero. The count is
/// nonnegative; null is permitted for zero. conn/root belong to this thread.
pub unsafe fn ewmh_update_client_list(list: *mut u32, num_windows: i32) {
    let length = usize::try_from(num_windows).expect("negative EWMH client count");
    let windows = if length == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(list, length)
    };
    set_words(
        root,
        A__NET_CLIENT_LIST,
        x::ATOM_WINDOW.resource_id(),
        windows,
    );
}
/// # Safety
/// Same readable-span/count/connection contract as ewmh_update_client_list.
pub unsafe fn ewmh_update_client_list_stacking(stack: *mut u32, num_windows: i32) {
    let length = usize::try_from(num_windows).expect("negative EWMH stacking count");
    let windows = if length == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(stack, length)
    };
    set_words(
        root,
        A__NET_CLIENT_LIST_STACKING,
        x::ATOM_WINDOW.resource_id(),
        windows,
    );
}
/// # Safety
/// conn and the requested window belong to the native event thread.
pub unsafe fn ewmh_update_sticky(window: u32, sticky: bool) {
    if sticky {
        ::log::debug!("../src/ewmh.c:ewmh_update_sticky:{} - Setting _NET_WM_STATE_STICKY for window = {:08x}.\n", { 283_i32 }, { (window) });
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    } else {
        ::log::debug!("../src/ewmh.c:ewmh_update_sticky:{} - Removing _NET_WM_STATE_STICKY for window = {:08x}.\n", { 286_i32 }, { (window) });
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    };
}

/// # Safety
/// conn and the requested window belong to the native event thread.
pub unsafe fn ewmh_update_focused(window: u32, is_focused: bool) {
    if is_focused {
        ::log::debug!("../src/ewmh.c:ewmh_update_focused:{} - Setting _NET_WM_STATE_FOCUSED for window = {:08x}.\n", { 297_i32 }, { (window) });
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    } else {
        ::log::debug!("../src/ewmh.c:ewmh_update_focused:{} - Removing _NET_WM_STATE_FOCUSED for window = {:08x}.\n", { 300_i32 }, { (window) });
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    };
}

/// # Safety
/// The event thread owns initialized atoms, root and conn. Called during setup.
pub unsafe fn ewmh_setup_hints() {
    let supported_atoms: [u32; 37] = [
        A__NET_SUPPORTED,
        A__NET_SUPPORTING_WM_CHECK,
        A__NET_WM_NAME,
        A__NET_WM_VISIBLE_NAME,
        A__NET_WM_MOVERESIZE,
        A__NET_WM_STATE_STICKY,
        A__NET_WM_STATE_FULLSCREEN,
        A__NET_WM_STATE_DEMANDS_ATTENTION,
        A__NET_WM_STATE_MODAL,
        A__NET_WM_STATE_HIDDEN,
        A__NET_WM_STATE_FOCUSED,
        A__NET_WM_STATE_MAXIMIZED_VERT,
        A__NET_WM_STATE_MAXIMIZED_HORZ,
        A__NET_WM_STATE,
        A__NET_WM_WINDOW_TYPE,
        A__NET_WM_WINDOW_TYPE_NORMAL,
        A__NET_WM_WINDOW_TYPE_DOCK,
        A__NET_WM_WINDOW_TYPE_DIALOG,
        A__NET_WM_WINDOW_TYPE_UTILITY,
        A__NET_WM_WINDOW_TYPE_TOOLBAR,
        A__NET_WM_WINDOW_TYPE_SPLASH,
        A__NET_WM_WINDOW_TYPE_MENU,
        A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
        A__NET_WM_WINDOW_TYPE_POPUP_MENU,
        A__NET_WM_WINDOW_TYPE_TOOLTIP,
        A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        A__NET_WM_DESKTOP,
        A__NET_WM_STRUT_PARTIAL,
        A__NET_CLIENT_LIST,
        A__NET_CLIENT_LIST_STACKING,
        A__NET_CURRENT_DESKTOP,
        A__NET_NUMBER_OF_DESKTOPS,
        A__NET_DESKTOP_NAMES,
        A__NET_DESKTOP_VIEWPORT,
        A__NET_ACTIVE_WINDOW,
        A__NET_CLOSE_WINDOW,
        A__NET_MOVERESIZE_WINDOW,
    ];
    let Some(connection) = connection() else {
        return;
    };
    let wid: x::Window = connection.generate_id();
    let window = wid.resource_id();
    ewmh_window = window;
    connection.send_request(&x::CreateWindow {
        depth: 0,
        wid,
        parent: x::Window::new(root),
        x: -1,
        y: -1,
        width: 1,
        height: 1,
        border_width: 0,
        class: x::WindowClass::InputOnly,
        visual: 0,
        value_list: &[x::Cw::OverrideRedirect(true)],
    });
    set_words(
        window,
        A__NET_SUPPORTING_WM_CHECK,
        x::ATOM_WINDOW.resource_id(),
        &[window],
    );
    set_bytes(window, A__NET_WM_NAME, A_UTF8_STRING, b"i3");
    set_words(
        root,
        A__NET_SUPPORTING_WM_CHECK,
        x::ATOM_WINDOW.resource_id(),
        &[window],
    );
    set_bytes(root, A__NET_WM_NAME, A_UTF8_STRING, b"i3");
    set_words(
        root,
        A__NET_SUPPORTED,
        x::ATOM_ATOM.resource_id(),
        &supported_atoms,
    );
    connection.send_request(&x::MapWindow { window: wid });
    connection.send_request(&x::ConfigureWindow {
        window: wid,
        value_list: &[x::ConfigWindow::StackMode(x::StackMode::Below)],
    });
}
/// # Safety
/// The event thread owns a stable workspace graph. Returned pointer is borrowed
/// and must not be used after a subsequent tree mutation removes its node.
pub unsafe fn ewmh_get_workspace_by_index(index: u32) -> *mut Con {
    if index == NET_WM_DESKTOP_NONE {
        return ptr::null_mut();
    }
    public_workspace_at(
        workspace_entries()
            .into_iter()
            .map(|entry| (entry.con, entry.internal)),
        index,
    )
    .unwrap_or(ptr::null_mut())
}
/// # Safety
/// con belongs to the event thread's stable tree (as required by con_get_workspace).
pub unsafe fn ewmh_get_workspace_index(con: *mut Con) -> u32 {
    let target = con_get_workspace(con);
    public_workspace_index(
        workspace_entries()
            .into_iter()
            .map(|entry| (entry.con, entry.internal)),
        target,
    )
}

// Shared model types retain field layout while using one Rust identity.
use crate::types::Con;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::A_UTF8_STRING;
use crate::application::A__NET_ACTIVE_WINDOW;
use crate::application::A__NET_CLIENT_LIST;
use crate::application::A__NET_CLIENT_LIST_STACKING;
use crate::application::A__NET_CLOSE_WINDOW;
use crate::application::A__NET_CURRENT_DESKTOP;
use crate::application::A__NET_DESKTOP_NAMES;
use crate::application::A__NET_DESKTOP_VIEWPORT;
use crate::application::A__NET_MOVERESIZE_WINDOW;
use crate::application::A__NET_NUMBER_OF_DESKTOPS;
use crate::application::A__NET_SUPPORTED;
use crate::application::A__NET_SUPPORTING_WM_CHECK;
use crate::application::A__NET_WM_DESKTOP;
use crate::application::A__NET_WM_MOVERESIZE;
use crate::application::A__NET_WM_NAME;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_DEMANDS_ATTENTION;
use crate::application::A__NET_WM_STATE_FOCUSED;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::application::A__NET_WM_STATE_HIDDEN;
use crate::application::A__NET_WM_STATE_MAXIMIZED_HORZ;
use crate::application::A__NET_WM_STATE_MAXIMIZED_VERT;
use crate::application::A__NET_WM_STATE_MODAL;
use crate::application::A__NET_WM_STATE_STICKY;
use crate::application::A__NET_WM_STRUT_PARTIAL;
use crate::application::A__NET_WM_VISIBLE_NAME;
use crate::application::A__NET_WM_WINDOW_TYPE;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DOCK;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::application::A__NET_WORKAREA;
use crate::con::con_get_workspace;
use crate::con::con_has_managed_window;
use crate::con::con_is_floating;
use crate::con::con_is_internal;
use crate::con::con_is_sticky;
use crate::output::output_get_content;
use crate::tree::croot;
use crate::tree::focused;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;

use crate::con::arena::nodes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_name_payload_preserves_raw_bytes_empty_names_and_final_nul() {
        let names = [c"1: web", c"", c"\xff\xfe"];
        assert_eq!(desktop_names(names), b"1: web\0\0\xff\xfe\0");
        assert!(desktop_names(std::iter::empty()).is_empty());
    }

    #[test]
    fn viewports_keep_output_order_and_unsigned_negative_coordinate_bits() {
        let viewports = desktop_viewports([[(-1920i32) as u32, 0], [0, (-720i32) as u32], [0, 0]]);
        assert_eq!(viewports, [0xfffff880, 0, 0, 0xfffffd30, 0, 0]);
        // These are 32-bit CARDINAL elements, not a byte count or a packed
        // machine-sized integer array. XCB consumes native-endian words.
        let wire_bytes: Vec<_> = viewports
            .iter()
            .flat_map(|word| word.to_ne_bytes())
            .collect();
        assert_eq!(wire_bytes.len(), 24);
        assert_eq!(&wire_bytes[..4], &0xfffff880u32.to_ne_bytes());
        assert!(desktop_viewports([]).is_empty());
    }

    #[test]
    fn internal_workspaces_are_updated_but_do_not_consume_public_indices() {
        let entries = [
            ("scratch", true),
            ("one", false),
            ("internal", true),
            ("two", false),
        ];
        assert_eq!(
            workspace_desktop_indices(entries),
            [("scratch", 0), ("one", 0), ("internal", 1), ("two", 1)]
        );
        assert_eq!(public_workspace_index(entries, "one"), 0);
        assert_eq!(public_workspace_index(entries, "two"), 1);
        assert_eq!(
            public_workspace_index(entries, "scratch"),
            NET_WM_DESKTOP_NONE
        );
        assert_eq!(
            public_workspace_index(entries, "missing"),
            NET_WM_DESKTOP_NONE
        );
        assert_eq!(public_workspace_at(entries, 0), Some("one"));
        assert_eq!(public_workspace_at(entries, 1), Some("two"));
        assert_eq!(public_workspace_at(entries, 2), None);
        assert_eq!(public_workspace_at(entries, NET_WM_DESKTOP_NONE), None);
        assert_eq!(public_workspace_at(entries, NET_WM_DESKTOP_ALL), None);
    }

    #[test]
    fn missing_current_desktop_retains_cache_but_missing_window_desktop_deletes() {
        assert!(!current_desktop_changed(3, 3));
        assert!(!current_desktop_changed(3, NET_WM_DESKTOP_NONE));
        assert!(current_desktop_changed(NET_WM_DESKTOP_NONE, 0));
        assert_eq!(
            desktop_property_change(3, NET_WM_DESKTOP_NONE),
            Some(DesktopProperty::Delete)
        );
        assert_eq!(
            desktop_property_change(NET_WM_DESKTOP_NONE, NET_WM_DESKTOP_NONE),
            None
        );
        assert_eq!(
            desktop_property_change(NET_WM_DESKTOP_NONE, 0),
            Some(DesktopProperty::Set(0))
        );
        assert_eq!(
            desktop_property_change(NET_WM_DESKTOP_ALL, NET_WM_DESKTOP_ALL),
            None
        );
    }

    #[test]
    fn sticky_requires_floating_and_internal_workspaces_always_use_all_desktops() {
        assert_eq!(window_desktop(4, true, false, false), 4);
        assert_eq!(window_desktop(4, false, true, false), 4);
        assert_eq!(window_desktop(4, true, true, false), NET_WM_DESKTOP_ALL);
        assert_eq!(
            window_desktop(NET_WM_DESKTOP_NONE, false, false, true),
            NET_WM_DESKTOP_ALL
        );
        assert_eq!(
            desktop_property_change(4, window_desktop(4, true, true, false)),
            Some(DesktopProperty::Set(NET_WM_DESKTOP_ALL))
        );
        assert_eq!(
            desktop_property_change(NET_WM_DESKTOP_ALL, window_desktop(4, false, true, false)),
            Some(DesktopProperty::Set(4))
        );
    }
}
