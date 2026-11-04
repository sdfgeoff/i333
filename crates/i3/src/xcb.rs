use crate::handlers::add_ignore_event;
use crate::xcursor::Cursor;
use ::xcb::ffi::xcb_connection_t;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * xcb.c: Helper functions for easier usage of XCB
 *
 */
// Mechanical file-by-file Rust translation of src/xcb.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

use crate::application::ignore_event_sequence_t;
use ::xcb::{x, Reply, Xid, XidNew};
use i3_common::xcb_connection::borrowed;
struct ServerGrab<'a>(&'a ::xcb::Connection);
impl Drop for ServerGrab<'_> {
    fn drop(&mut self) {
        self.0.send_request(&x::UngrabServer {});
    }
}

fn atom_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_ne_bytes(*word))
        .collect()
}
/// Borrow bytes from an owned maintained reply only after validating its wire
/// count against the complete advertised allocation. Format 16/32 values remain
/// raw native-endian bytes; the generated value::<u8>() accessor only accepts 8.
pub(crate) fn property_payload(reply: &x::GetPropertyReply) -> Option<&[u8]> {
    let width = match reply.format() {
        8 => 1,
        16 => 2,
        32 => 4,
        _ => return None,
    };
    // SAFETY: the maintained reply owns its initialized 32-byte wire header.
    let raw = unsafe { reply.as_raw() };
    let count = unsafe { raw.add(16).cast::<u32>().read_unaligned() };
    let length = (count as usize).checked_mul(width)?;
    let available = (reply.length() as usize).checked_mul(4)?;
    if length > available || length > isize::MAX as usize {
        return None;
    }
    // SAFETY: checked byte range lies in the reply's advertised trailing body
    // and cannot outlive its maintained allocation owner.
    Some(unsafe { std::slice::from_raw_parts(raw.add(32), length) })
}
fn property_bytes(reply: Option<&x::GetPropertyReply>) -> Option<&[u8]> {
    let reply = reply?;
    (reply.format() == 32)
        .then(|| property_payload(reply))
        .flatten()
}

fn preferred_atom(atoms: impl IntoIterator<Item = u32>, supported: &[u32]) -> u32 {
    atoms
        .into_iter()
        .find(|atom| *atom != 0 && supported.contains(atom))
        .unwrap_or(0)
}
fn without_atom(atoms: impl IntoIterator<Item = u32>, removed: u32) -> Vec<u32> {
    atoms.into_iter().filter(|&atom| atom != removed).collect()
}
fn absolute_geometry(container: Rect, window: Rect) -> x::Rectangle {
    x::Rectangle {
        x: container.x.wrapping_add(window.x) as i16,
        y: container.y.wrapping_add(window.y) as i16,
        width: window.width as u16,
        height: window.height as u16,
    }
}
fn take_focus_message(
    window: u32,
    timestamp: u32,
    protocols: u32,
    take_focus: u32,
) -> x::ClientMessageEvent {
    x::ClientMessageEvent::new(
        x::Window::new(window),
        x::Atom::new(protocols),
        x::ClientMessageData::Data32([take_focus, timestamp, 0, 0, 0]),
    )
}

pub static mut xcb_numlock_mask: u32 = 0;

/// Create a helper window using the caller's retained connection. Independent
/// placeholder/emergency connections must never be replaced by the registry.
///
/// Safety: the caller serializes access to the event-thread root/cursor caches.
/// Values are typed and borrowed only while the maintained request serializes.
pub unsafe fn create_window(
    connection: &::xcb::Connection,
    dims: Rect,
    mut depth: u8,
    mut visual: u32,
    window_class: x::WindowClass,
    cursor: Cursor,
    map: bool,
    values: &[x::Cw],
) -> u32 {
    let result: x::Window = connection.generate_id();
    if window_class == x::WindowClass::InputOnly {
        depth = 0;
        visual = 0;
    }
    let cookie = connection.send_request(&x::CreateWindow {
        depth,
        wid: result,
        parent: x::Window::new(root),
        x: dims.x as i16,
        y: dims.y as i16,
        width: dims.width as u16,
        height: dims.height as u16,
        border_width: 0,
        class: window_class,
        visual,
        value_list: values,
    });
    // Native CreateWindow is unchecked, but still followed by request_check.
    // Retain that barrier without claiming errors from the event queue.
    if let Some(code) = xcb_interop::transport::check_unchecked(connection, cookie) {
        ::log::error!("ERROR: Could not create window. Error code: {}.\n", {
            (i32::from(code))
        });
    }
    connection.send_request(&x::ChangeWindowAttributes {
        window: result,
        value_list: &[x::Cw::Cursor(x::Cursor::new(xcursor_get_cursor(cursor)))],
    });
    if map {
        connection.send_request(&x::MapWindow { window: result });
    }
    result.resource_id()
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn fake_absolute_configure_notify(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let absolute = absolute_geometry((*con).rect, (*con).window_rect);
    ::log::debug!(
        "../src/xcb.c:fake_absolute_configure_notify:75 - fake rect = ({}, {}, {}, {})\n",
        { (absolute.x as i32) },
        { (absolute.y as i32) },
        { (absolute.width as i32) },
        { (absolute.height as i32) }
    );
    if let Some(connection) = i3_common::display::connection() {
        send_configure_notify(
            &connection,
            absolute,
            x::Window::new((*(*con).window).id),
            (*con).border_width,
        );
    }
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn send_take_focus(window: u32, timestamp: u32) {
    let event = take_focus_message(window, timestamp, A_WM_PROTOCOLS, A_WM_TAKE_FOCUS);
    ::log::debug!("../src/xcb.c:send_take_focus:98 - Sending WM_TAKE_FOCUS to the client\n");
    let Some(connection) = i3_common::display::connection() else {
        return;
    };
    connection.send_request(&x::SendEvent {
        propagate: false,
        destination: x::SendEventDest::Window(x::Window::new(window)),
        event_mask: x::EventMask::NO_EVENT,
        event: &event,
    });
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_set_window_rect(conn_0: *mut xcb_connection_t, window: u32, r: Rect) {
    let Some(connection) = borrowed(conn_0) else {
        return;
    };
    let cookie = connection.send_request(&x::ConfigureWindow {
        window: x::Window::new(window),
        value_list: &[
            x::ConfigWindow::X(r.x as i32),
            x::ConfigWindow::Y(r.y as i32),
            x::ConfigWindow::Width(r.width),
            x::ConfigWindow::Height(r.height),
        ],
    });
    add_ignore_event(
        ignore_event_sequence_t {
            value: ::xcb::Cookie::sequence(&cookie) as u32,
        },
        -1_i32,
    );
}

/// reply borrows a complete native allocation on the owning event thread.
pub unsafe fn xcb_get_preferred_window_type(reply: Option<&x::GetPropertyReply>) -> u32 {
    let Some(bytes) = property_bytes(reply) else {
        return 0;
    };
    preferred_atom(
        atom_words(bytes),
        &[
            A__NET_WM_WINDOW_TYPE_NORMAL,
            A__NET_WM_WINDOW_TYPE_DIALOG,
            A__NET_WM_WINDOW_TYPE_UTILITY,
            A__NET_WM_WINDOW_TYPE_TOOLBAR,
            A__NET_WM_WINDOW_TYPE_SPLASH,
            A__NET_WM_WINDOW_TYPE_MENU,
            A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
            A__NET_WM_WINDOW_TYPE_POPUP_MENU,
            A__NET_WM_WINDOW_TYPE_TOOLTIP,
            A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        ],
    )
}
/// prop borrows a complete native property reply for this event-thread query.
pub unsafe fn xcb_reply_contains_atom(prop: Option<&x::GetPropertyReply>, atom: u32) -> bool {
    property_bytes(prop).is_some_and(|bytes| atom_words(bytes).contains(&atom))
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn get_visualtype_by_id(
    visual_id: u32,
) -> Option<std::rc::Rc<i3_common::display::CairoVisual>> {
    // The native helper searched only the selected main screen. Preserve its
    // missing-ID fallback even when another screen has a matching visual.
    let present = crate::application::root_display()
        .screen()
        .allowed_depths()
        .any(|depth| {
            depth
                .visuals()
                .iter()
                .any(|visual| visual.visual_id() == visual_id)
        });
    present
        .then(|| i3_common::display::visual(visual_id))
        .flatten()
}

/// The event thread retains the selected setup owner during this lookup.
/// Return the first visual at the complete requested depth, or zero when absent.
pub unsafe fn get_visualid_by_depth(depth: u16) -> u32 {
    crate::application::root_display()
        .screen()
        .allowed_depths()
        .find_map(|allowed| {
            (u16::from(allowed.depth()) == depth)
                .then(|| allowed.visuals().first().map(|visual| visual.visual_id()))
                .flatten()
        })
        .unwrap_or(0)
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_add_property_atom(
    conn_0: *mut xcb_connection_t,
    window: u32,
    property: u32,
    atom: u32,
) {
    let Some(connection) = borrowed(conn_0) else {
        return;
    };
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Append,
        window: x::Window::new(window),
        property: x::Atom::new(property),
        r#type: x::ATOM_ATOM,
        data: &[atom],
    });
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_remove_property_atom(
    conn_0: *mut xcb_connection_t,
    window: u32,
    property: u32,
    atom: u32,
) {
    let Some(connection) = borrowed(conn_0) else {
        return;
    };
    connection.send_request(&x::GrabServer {});
    let _grab = ServerGrab(&connection);
    let cookie = connection.send_request(&x::GetProperty {
        delete: false,
        window: x::Window::new(window),
        property: x::Atom::new(property),
        r#type: x::ATOM_ANY,
        long_offset: 0,
        long_length: 4096,
    });
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        return;
    };
    let Some(bytes) = property_bytes(Some(&reply)) else {
        return;
    };
    if bytes.is_empty() {
        return;
    }
    let values = without_atom(atom_words(bytes), atom);
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(window),
        property: x::Atom::new(property),
        r#type: x::ATOM_ATOM,
        data: &values,
    });
}

/// Native display/graph boundary: requires a live event-thread connection and
/// initialized, exclusively accessible pointers/resources for this operation.
pub unsafe fn xcb_grab_buttons(conn_0: *mut xcb_connection_t, window: u32, buttons: *mut i32) {
    let Some(connection) = borrowed(conn_0) else {
        return;
    };
    let mut i = 0;
    while *buttons.offset(i) > 0 {
        // Buttons beyond five are valid i3 configuration values even though
        // maintained XCB's ButtonIndex enum cannot represent them. The narrow
        // utility adapter retains the complete u8 domain without transmuting it.
        xcb_interop::transport::grab_button(
            &connection,
            false,
            x::Window::new(window),
            x::EventMask::BUTTON_PRESS,
            x::GrabMode::Sync,
            x::GrabMode::Async,
            x::Window::new(root),
            x::Cursor::none(),
            *buttons.offset(i) as u8,
            x::ModMask::ANY,
        );
        i += 1;
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::xcursor::xcursor_get_cursor;

// Canonical Cairo backend ABI; renderer logic remains local.

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::A_WM_PROTOCOLS;
use crate::application::A_WM_TAKE_FOCUS;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use i3_common::fake_configure_notify::send_configure_notify;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configure_geometry_preserves_unsigned_addition_and_wire_truncation() {
        let rect = absolute_geometry(
            Rect {
                x: u32::MAX,
                y: 32767,
                width: 0,
                height: 0,
            },
            Rect {
                x: 2,
                y: 1,
                width: 65537,
                height: u32::MAX,
            },
        );
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (1, i16::MIN, 1, u16::MAX)
        );
    }
    #[test]
    fn atom_selection_and_removal_keep_property_order_and_duplicate_semantics() {
        let words = [99u32, 7, 8, 7, 0];
        let bytes: Vec<_> = words.iter().flat_map(|word| word.to_ne_bytes()).collect();
        assert_eq!(preferred_atom(atom_words(&bytes), &[8, 7]), 7);
        assert_eq!(without_atom(atom_words(&bytes), 7), [99, 8, 0]);
        assert_eq!(preferred_atom([], &[7]), 0);
        assert!(atom_words(&bytes[..3]).is_empty());
    }
    #[test]
    fn take_focus_packet_initializes_every_native_data_word() {
        let event = take_focus_message(0xffffffff, 0x12345678, 12, 34);
        let mut expected = [0u8; 32];
        expected[0] = 33;
        expected[1] = 32;
        expected[4..8].copy_from_slice(&u32::MAX.to_ne_bytes());
        expected[8..12].copy_from_slice(&12u32.to_ne_bytes());
        expected[12..16].copy_from_slice(&34u32.to_ne_bytes());
        expected[16..20].copy_from_slice(&0x12345678u32.to_ne_bytes());
        // SAFETY: the maintained event owns its complete initialized 32-byte
        // representation. Borrow only for the comparison, then let it drop.
        let bytes =
            unsafe { std::slice::from_raw_parts(::xcb::Raw::as_raw(&event).cast::<u8>(), 32) };
        assert_eq!(bytes, expected);
    }
}
