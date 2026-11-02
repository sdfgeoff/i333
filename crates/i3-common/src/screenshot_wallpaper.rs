/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Keep a server-side copy of the root contents using maintained XCB requests.
#![deny(unsafe_op_in_unsafe_fn)]

use crate::display::ScreenInfo;
use xcb::{x, XidNew};

/// Capture the screen, including child windows, without waiting for a helper process.
/// Replaces the supplied screen's server-side root background pixmap.
pub fn set_screenshot_as_wallpaper(connection: &xcb::Connection, screen: ScreenInfo) {
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
