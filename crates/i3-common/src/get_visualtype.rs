/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Find the root visual in the screen's borrowed setup-buffer records.
#![deny(unsafe_op_in_unsafe_fn)]

use std::ptr;
use std::slice;
use xcb_sys::{
    xcb_depth_next, xcb_depth_visuals_iterator, xcb_screen_allowed_depths_iterator, xcb_screen_t,
    xcb_visualtype_t,
};

/// Return a root visual borrowed from the original XCB setup allocation.
///
/// # Safety
/// screen must point into a complete, valid XCB screen setup representation, including all
/// of its variable-length depth and visual records. That allocation must remain
/// alive for every use of the returned pointer, which must not be freed itself.
/// Callers must uphold exclusive access before mutating any returned record.
pub unsafe fn get_visualtype(screen: *mut xcb_screen_t) -> *mut xcb_visualtype_t {
    // SAFETY: the caller guarantees screen belongs to a complete setup buffer.
    let Some(screen_header) = (unsafe { screen.as_ref() }) else {
        return ptr::null_mut();
    };
    let root_visual = screen_header.root_visual;
    // SAFETY: the setup buffer contains the screen's complete depth records.
    let mut depths = unsafe { xcb_screen_allowed_depths_iterator(screen) };
    while depths.rem > 0 && !depths.data.is_null() {
        // SAFETY: each depth record belongs to the complete setup allocation.
        let visuals = unsafe { xcb_depth_visuals_iterator(depths.data) };
        if visuals.rem > 0 && !visuals.data.is_null() {
            // SAFETY: XCB's visual iterator describes contiguous initialized
            // visual records in the setup allocation, kept alive by the caller.
            let records = unsafe { slice::from_raw_parts(visuals.data, visuals.rem as usize) };
            if let Some(index) = records
                .iter()
                .position(|visual| visual.visual_id == root_visual)
            {
                // The match is in bounds, and the original pointer retains its
                // provenance into the caller's setup allocation.
                return visuals.data.wrapping_add(index);
            }
        }
        // SAFETY: this valid depth iterator stays within the complete setup buffer.
        unsafe { xcb_depth_next(&mut depths) };
    }
    ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;
    use xcb_sys::xcb_depth_t;

    // This mirrors the variable-length X11 setup wire layout: each depth header
    // is immediately followed by its declared visual records.
    #[repr(C)]
    struct ScreenSetup {
        screen: xcb_screen_t,
        first_depth: xcb_depth_t,
        first_visuals: [xcb_visualtype_t; 2],
        second_depth: xcb_depth_t,
        second_visuals: [xcb_visualtype_t; 1],
    }

    #[test]
    fn setup_traversal_borrows_the_matching_visual_across_depths() {
        let visual = |visual_id| xcb_visualtype_t {
            visual_id,
            _class: 0,
            bits_per_rgb_value: 0,
            colormap_entries: 0,
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
            pad0: [0; 4],
        };
        let depth = |visuals_len| xcb_depth_t {
            depth: 24,
            pad0: 0,
            visuals_len,
            pad1: [0; 4],
        };
        let mut setup = ScreenSetup {
            screen: xcb_screen_t {
                root: 0,
                default_colormap: 0,
                white_pixel: 0,
                black_pixel: 0,
                current_input_masks: 0,
                width_in_pixels: 0,
                height_in_pixels: 0,
                width_in_millimeters: 0,
                height_in_millimeters: 0,
                min_installed_maps: 0,
                max_installed_maps: 0,
                root_visual: 9,
                backing_stores: 0,
                save_unders: 0,
                root_depth: 24,
                allowed_depths_len: 2,
            },
            first_depth: depth(2),
            first_visuals: [visual(3), visual(5)],
            second_depth: depth(1),
            second_visuals: [visual(9)],
        };
        // SAFETY: ScreenSetup provides the complete initialized depth/visual
        // representation declared by its screen header and outlives the lookup.
        let result = unsafe { get_visualtype(&mut setup.screen) };
        assert_eq!(result, &raw mut setup.second_visuals[0]);
        setup.screen.root_visual = 5;
        // SAFETY: same complete live setup representation.
        assert_eq!(
            unsafe { get_visualtype(&mut setup.screen) },
            &raw mut setup.first_visuals[1]
        );
        setup.screen.root_visual = 7;
        // SAFETY: same complete live setup representation.
        assert!(unsafe { get_visualtype(&mut setup.screen) }.is_null());
        setup.screen.allowed_depths_len = 0;
        // SAFETY: zero depths requires no trailing records.
        assert!(unsafe { get_visualtype(&mut setup.screen) }.is_null());
    }
}
