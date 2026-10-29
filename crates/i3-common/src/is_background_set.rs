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
use std::{ops::Deref, ptr, ptr::NonNull, slice};
use xcb_sys::{
    XCB_COPY_FROM_PARENT, XCB_CW_BACK_PIXEL, XCB_CW_OVERRIDE_REDIRECT, XCB_IMAGE_FORMAT_Z_PIXMAP,
    XCB_MAP_STATE_VIEWABLE, XCB_WINDOW_CLASS_INPUT_OUTPUT, xcb_aux_sync, xcb_clear_area,
    xcb_connection_t, xcb_create_window, xcb_destroy_window, xcb_generate_id,
    xcb_get_geometry_reply, xcb_get_geometry_unchecked, xcb_get_image_data, xcb_get_image_reply,
    xcb_get_image_unchecked, xcb_get_window_attributes_reply, xcb_get_window_attributes_unchecked,
    xcb_map_window, xcb_query_tree_children, xcb_query_tree_reply, xcb_query_tree_unchecked,
    xcb_screen_t, xcb_window_t,
};

/// An individually allocated XCB reply, including its trailing wire payload.
struct Reply<T>(NonNull<T>);

impl<T> Reply<T> {
    /// # Safety
    /// A non-null pointer must be an initialized, uniquely owned XCB reply
    /// allocation for T, released with libc::free and containing its declared payload.
    unsafe fn from_raw(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
}

impl<T> Deref for Reply<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: construction requires a live initialized reply owned by self.
        unsafe { self.0.as_ref() }
    }
}

impl<T> Drop for Reply<T> {
    fn drop(&mut self) {
        // SAFETY: XCB transfers this allocation to us; it has not been freed.
        unsafe { libc::free(self.0.as_ptr().cast()) };
    }
}

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
    (count <= payload_length(words)? / size_of::<xcb_window_t>()).then_some(count)
}

// Keep the original byte folding (including byte order and depth-as-byte-count),
// while a truncated/malformed reply can no longer read beyond its allocation.
fn image_pixel(depth: u8, data: &[u8]) -> u32 {
    data.iter()
        .take(usize::from(depth.min(4)))
        .fold(0, |pixel, &byte| (pixel << 8) | u32::from(byte))
}

/// # Safety
/// conn must be a live connection on which native requests/replies may be used.
unsafe fn unobscured_pixel(conn: *mut xcb_connection_t, screen: &xcb_screen_t) -> (u16, u16) {
    let mut region = Region::new(screen.width_in_pixels, screen.height_in_pixels);
    // SAFETY: the caller supplies a live connection; native replies transfer ownership.
    let tree = unsafe {
        Reply::from_raw(xcb_query_tree_reply(
            conn,
            xcb_query_tree_unchecked(conn, screen.root),
            ptr::null_mut(),
        ))
    };
    let Some(tree) = tree else {
        return region.first_pixel();
    };
    let Some(count) = child_count(tree.length, tree.children_len) else {
        return region.first_pixel();
    };
    // SAFETY: XCB allocated the declared payload; count was checked against its
    // size. The owned tree remains alive throughout use of the borrowed children.
    let children =
        unsafe { slice::from_raw_parts(xcb_query_tree_children(tree.0.as_ptr()), count) };
    let requests: Vec<_> = children
        .iter()
        .map(|&window| {
            // SAFETY: conn remains live; child IDs are values from the server reply.
            unsafe {
                (
                    xcb_get_geometry_unchecked(conn, window),
                    xcb_get_window_attributes_unchecked(conn, window),
                )
            }
        })
        .collect();
    // Issue every request before waiting, retaining the original pipelining.
    for (geometry_cookie, attributes_cookie) in requests {
        // SAFETY: cookies belong to conn; both returned allocations are owned.
        let (geometry, attributes) = unsafe {
            (
                Reply::from_raw(xcb_get_geometry_reply(
                    conn,
                    geometry_cookie,
                    ptr::null_mut(),
                )),
                Reply::from_raw(xcb_get_window_attributes_reply(
                    conn,
                    attributes_cookie,
                    ptr::null_mut(),
                )),
            )
        };
        if let (Some(geometry), Some(attributes)) = (geometry, attributes) {
            if u32::from(attributes.map_state) == XCB_MAP_STATE_VIEWABLE {
                region.subtract(cairo_rectangle_int_t {
                    x: geometry.x.into(),
                    y: geometry.y.into(),
                    width: geometry.width.into(),
                    height: geometry.height.into(),
                });
            }
        }
    }
    region.first_pixel()
}

/// # Safety
/// conn must be live and screen must describe its root; window must be an unused
/// resource ID. This deliberately maps/destroys a temporary override-redirect window.
unsafe fn flicker_window_at(
    conn: *mut xcb_connection_t,
    screen: &xcb_screen_t,
    x: i16,
    y: i16,
    window: xcb_window_t,
    pixel: u32,
) -> u32 {
    let values = [pixel, 1];
    // SAFETY: the caller guarantees native connection/screen/resource validity.
    // The value list contains both fields selected by the mask, in protocol order.
    let image = unsafe {
        xcb_create_window(
            conn,
            XCB_COPY_FROM_PARENT as u8,
            window,
            screen.root,
            x,
            y,
            10,
            10,
            0,
            XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            XCB_COPY_FROM_PARENT as u32,
            XCB_CW_BACK_PIXEL | XCB_CW_OVERRIDE_REDIRECT,
            values.as_ptr().cast(),
        );
        xcb_map_window(conn, window);
        xcb_clear_area(conn, 0, window, 0, 0, 0, 0);
        xcb_aux_sync(conn);
        xcb_destroy_window(conn, window);
        Reply::from_raw(xcb_get_image_reply(
            conn,
            xcb_get_image_unchecked(
                conn,
                XCB_IMAGE_FORMAT_Z_PIXMAP as u8,
                screen.root,
                x,
                y,
                1,
                1,
                u32::MAX,
            ),
            ptr::null_mut(),
        ))
    };
    let Some(image) = image else { return 0 };
    let Some(length) = payload_length(image.length) else {
        return 0;
    };
    // SAFETY: XCB owns a reply with this declared trailing payload; checked
    // length fits a Rust slice and image stays alive until after folding it.
    let data = unsafe { slice::from_raw_parts(xcb_get_image_data(image.0.as_ptr()), length) };
    image_pixel(image.depth, data)
}

/// Check whether the root background is restored after temporary windows disappear.
///
/// # Safety
/// conn must be a live XCB connection. screen must point to its initialized screen
/// record and remain readable throughout the call. The caller must permit native
/// root-window requests and temporary windows on that connection's display.
pub unsafe fn is_background_set(conn: *mut xcb_connection_t, screen: *mut xcb_screen_t) -> bool {
    // SAFETY: the caller guarantees the screen record is valid and remains live.
    let screen = unsafe { &*screen };
    // SAFETY: all native operations share the caller's live connection/screen.
    unsafe {
        let (x, y) = unobscured_pixel(conn, screen);
        let window = xcb_generate_id(conn);
        let first = flicker_window_at(conn, screen, x as i16, y as i16, window, screen.black_pixel);
        let second =
            flicker_window_at(conn, screen, x as i16, y as i16, window, screen.white_pixel);
        first == second
    }
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
