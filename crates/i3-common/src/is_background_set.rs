/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Detect whether the root background repaints after a temporary window disappears.
#![deny(unsafe_op_in_unsafe_fn)]

use cairo_sys::{
    cairo_rectangle_int_t, cairo_region_create_rectangle, cairo_region_destroy,
    cairo_region_get_rectangle, cairo_region_num_rectangles, cairo_region_subtract_rectangle,
    cairo_region_t,
};
use std::ptr::NonNull;
use xcb::{x, XidNew};
use xcb_sys::{xcb_connection_t, xcb_screen_t};

struct Region(NonNull<cairo_region_t>);

impl Region {
    fn new(width: u16, height: u16) -> Self {
        let mut rectangle = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: width.into(),
            height: height.into(),
        };
        // SAFETY: Cairo copies this initialized rectangle. It returns a region
        // even on allocation failure (an error region), never a null pointer.
        Self(
            NonNull::new(unsafe { cairo_region_create_rectangle(&mut rectangle) })
                .expect("Cairo region"),
        )
    }

    fn subtract(&mut self, mut rectangle: cairo_rectangle_int_t) {
        // SAFETY: self owns a live region and the rectangle is initialized.
        unsafe { cairo_region_subtract_rectangle(self.0.as_ptr(), &mut rectangle) };
    }

    fn first_pixel(&self) -> (u16, u16) {
        // SAFETY: self owns a live region; index zero is used only when present.
        unsafe {
            if cairo_region_num_rectangles(self.0.as_ptr()) <= 0 {
                return (0, 0);
            }
            let mut rectangle = cairo_rectangle_int_t {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            cairo_region_get_rectangle(self.0.as_ptr(), 0, &mut rectangle);
            (rectangle.x as u16, rectangle.y as u16)
        }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        // SAFETY: this is the sole owning reference to the live Cairo region.
        unsafe { cairo_region_destroy(self.0.as_ptr()) };
    }
}

fn payload_length(words: u32) -> Option<usize> {
    usize::try_from(words)
        .ok()?
        .checked_mul(4)
        .filter(|&length| length <= isize::MAX as usize)
}

fn child_count(words: u32, children: u16) -> Option<usize> {
    let count = usize::from(children);
    (count <= payload_length(words)? / size_of::<x::Window>()).then_some(count)
}

// Keep the original byte folding (including byte order and depth-as-byte-count),
// while a truncated/malformed reply can no longer read beyond its allocation.
fn image_pixel(depth: u8, data: &[u8]) -> u32 {
    data.iter()
        .take(usize::from(depth.min(4)))
        .fold(0, |pixel, &byte| (pixel << 8) | u32::from(byte))
}

/// Find a pixel not covered by a viewable root child, retaining native batching.
fn unobscured_pixel(connection: &xcb::Connection, screen: &xcb_screen_t) -> (u16, u16) {
    let mut region = Region::new(screen.width_in_pixels, screen.height_in_pixels);
    let cookie = connection.send_request_unchecked(&x::QueryTree {
        window: x::Window::new(screen.root),
    });
    let Some(tree) = connection.wait_for_reply_unchecked(cookie).ok().flatten() else {
        return region.first_pixel();
    };
    // Keep bounds validation independent of the maintained accessor: malformed
    // child counts must not create an out-of-allocation slice.
    let raw = unsafe { xcb::Reply::as_raw(&tree) };
    let words = unsafe { raw.add(4).cast::<u32>().read_unaligned() };
    let children = unsafe { raw.add(16).cast::<u16>().read_unaligned() };
    if child_count(words, children).is_none() {
        return region.first_pixel();
    }
    let requests: Vec<_> = tree
        .children()
        .iter()
        .map(|&window| {
            (
                connection.send_request_unchecked(&x::GetGeometry {
                    drawable: x::Drawable::Window(window),
                }),
                connection.send_request_unchecked(&x::GetWindowAttributes { window }),
            )
        })
        .collect();
    // Issue every request before waiting, retaining the original pipelining.
    for (geometry, attributes) in requests {
        let geometry = connection.wait_for_reply_unchecked(geometry).ok().flatten();
        let attributes = connection
            .wait_for_reply_unchecked(attributes)
            .ok()
            .flatten();
        if let (Some(geometry), Some(attributes)) = (geometry, attributes) {
            // Compare the open wire value rather than turning an unexpected
            // native map-state discriminant into a Rust enum.
            let raw = unsafe { xcb::Reply::as_raw(&attributes) };
            let viewable = unsafe { raw.add(26).read() } == x::MapState::Viewable as u8;
            if viewable {
                region.subtract(cairo_rectangle_int_t {
                    x: geometry.x().into(),
                    y: geometry.y().into(),
                    width: geometry.width().into(),
                    height: geometry.height().into(),
                });
            }
        }
    }
    region.first_pixel()
}

/// Deliberately map/destroy a temporary override-redirect window on the supplied
/// connection. The resource ID comes from that connection and is reused only
/// after an image round trip confirms the previous destroy has completed.
fn flicker_window_at(
    connection: &xcb::Connection,
    screen: &xcb_screen_t,
    x: i16,
    y: i16,
    window: x::Window,
    pixel: u32,
) -> u32 {
    let root = x::Window::new(screen.root);
    connection.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: root,
        x,
        y,
        width: 10,
        height: 10,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: x::COPY_FROM_PARENT,
        value_list: &[x::Cw::BackPixel(pixel), x::Cw::OverrideRedirect(true)],
    });
    connection.send_request(&x::MapWindow { window });
    connection.send_request(&x::ClearArea {
        exposures: false,
        window,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    });
    // xcb_aux_sync uses xcb_get_input_focus (the checked reply request), then
    // waits with a null error destination. Preserve that routing and discard
    // its optional error rather than leaving it on the event queue.
    let sync = connection.send_request(&x::GetInputFocus {});
    let _ = connection.wait_for_reply(sync);
    connection.send_request(&x::DestroyWindow { window });
    let image = connection.send_request_unchecked(&x::GetImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(root),
        x,
        y,
        width: 1,
        height: 1,
        plane_mask: u32::MAX,
    });
    let Some(image) = connection.wait_for_reply_unchecked(image).ok().flatten() else {
        return 0;
    };
    image_pixel(image.depth(), image.data())
}

/// Check whether the root background is restored after temporary windows disappear.
///
/// # Safety
/// conn must be a live XCB connection. screen must point to its initialized screen
/// record and remain readable throughout the call. The caller must permit native
/// root-window requests and temporary windows on that connection's display.
pub unsafe fn is_background_set(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) -> bool {
    // SAFETY: the native owner and screen remain live throughout this call; no
    // callback or disconnect occurs while the non-owning maintained view exists.
    let Some(connection) = (unsafe { crate::xcb_connection::borrowed(conn) }) else {
        return false;
    };
    let screen = unsafe { &*screen };
    let (x, y) = unobscured_pixel(&connection, screen);
    let window = connection.generate_id();
    let first = flicker_window_at(
        &connection,
        screen,
        x as i16,
        y as i16,
        window,
        screen.black_pixel,
    );
    let second = flicker_window_at(
        &connection,
        screen,
        x as i16,
        y as i16,
        window,
        screen.white_pixel,
    );
    first == second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children_must_fit_declared_wire_payload() {
        assert_eq!(child_count(0, 0), Some(0));
        assert_eq!(child_count(0, 1), None);
        assert_eq!(child_count(2, 2), Some(2));
        assert_eq!(child_count(2, 3), None);
        assert_eq!(
            child_count(u32::MAX, u16::MAX),
            payload_length(u32::MAX).map(|_| usize::from(u16::MAX))
        );
    }

    #[test]
    fn pixel_folding_preserves_depth_and_handles_short_payloads() {
        assert_eq!(image_pixel(0, &[1, 2, 3, 4]), 0);
        assert_eq!(image_pixel(1, &[1, 2, 3, 4]), 1);
        assert_eq!(image_pixel(2, &[1, 2, 3, 4]), 0x0102);
        assert_eq!(image_pixel(24, &[1, 2, 3, 4, 5]), 0x01020304);
        assert_eq!(image_pixel(32, &[]), 0);
        assert_eq!(image_pixel(32, &[0xAB, 0xCD]), 0xABCD);
    }

    #[test]
    fn unobscured_region_finds_holes_and_falls_back_when_covered() {
        let mut region = Region::new(20, 10);
        assert_eq!(region.first_pixel(), (0, 0));
        region.subtract(cairo_rectangle_int_t {
            x: -5,
            y: -2,
            width: 15,
            height: 12,
        });
        assert_eq!(region.first_pixel(), (10, 0));
        region.subtract(cairo_rectangle_int_t {
            x: 10,
            y: 0,
            width: 10,
            height: 10,
        });
        assert_eq!(region.first_pixel(), (0, 0));
        assert_eq!(Region::new(0, 0).first_pixel(), (0, 0));
    }
}
