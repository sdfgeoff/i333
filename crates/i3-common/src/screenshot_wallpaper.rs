/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Keep a server-side copy of the root contents using maintained XCB requests.
#![deny(unsafe_op_in_unsafe_fn)]

use xcb::{x, XidNew};
use xcb_sys::{xcb_connection_t, xcb_screen_t};

/// Capture the screen, including child windows, without waiting for a helper process.
///
/// # Safety
/// conn must be a live XCB connection on which requests may be issued. screen
/// must point to its initialized screen record and remain readable during this
/// call. The caller must permit replacing that screen's root background pixmap.
pub unsafe fn set_screenshot_as_wallpaper(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) {
    // SAFETY: the native owner and screen remain live for this synchronous call;
    // the temporary maintained view never disconnects the borrowed connection.
    let Some(connection) = (unsafe { crate::xcb_connection::borrowed(conn) }) else {
        return;
    };
    let screen = unsafe { &*screen };
    let root = x::Window::new(screen.root);
    let (width, height) = (screen.width_in_pixels, screen.height_in_pixels);
    let pixmap: x::Pixmap = connection.generate_id();
    let gc: x::Gcontext = connection.generate_id();
    // Requests retain their native order and unchecked error behavior. The root
    // retains the referenced server-side pixmap after our client IDs are freed.
    connection.send_request(&x::CreatePixmap {
        depth: screen.root_depth,
        pid: pixmap,
        drawable: x::Drawable::Window(root),
        width,
        height,
    });
    connection.send_request(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(root),
        value_list: &[
            x::Gc::Function(x::Gx::Copy),
            x::Gc::PlaneMask(u32::MAX),
            x::Gc::FillStyle(x::FillStyle::Solid),
            x::Gc::SubwindowMode(x::SubwindowMode::IncludeInferiors),
        ],
    });
    connection.send_request(&x::CopyArea {
        src_drawable: x::Drawable::Window(root),
        dst_drawable: x::Drawable::Pixmap(pixmap),
        gc,
        src_x: 0,
        src_y: 0,
        dst_x: 0,
        dst_y: 0,
        width,
        height,
    });
    connection.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::BackPixmap(pixmap)],
    });
    connection.send_request(&x::FreeGc { gc });
    connection.send_request(&x::FreePixmap { pixmap });
    let _ = connection.flush();
}
