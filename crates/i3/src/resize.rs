use crate::types::Direction;
use crate::types::DragResult;
use crate::types::Layout;
use crate::types::Orientation;
use crate::xcursor::Cursor;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * resize.c: Interactive resizing.
 *
 */
// Local policy and temporary values use owned Rust storage. Native entrypoints
// require live event-thread graph handles and a live display; callbacks may mutate
// the graph, so node references must never span those calls.

const CT_WORKSPACE: u32 = 4;
const CT_FLOATING_CON: u32 = 3;

struct DragState {
    orientation: Orientation,
    output: *mut Con,
    helpwin: xcb_window_t,
    new_position: u32,
    threshold_exceeded: bool,
}

// Both windows belong to this synchronous resize operation; the display
// connection stays live until the operation returns. No graph node is borrowed.
struct ResizeWindows {
    connection: *mut xcb_sys::xcb_connection_t,
    help: xcb_window_t,
    grab: xcb_window_t,
}
impl Drop for ResizeWindows {
    fn drop(&mut self) {
        unsafe {
            xcb_destroy_window(self.connection, self.help);
            xcb_destroy_window(self.connection, self.grab);
            xcb_flush(self.connection);
        }
    }
}

fn minimum_share(parent_size: i32, horizontal: bool, deco_height: u32) -> f64 {
    let minimum = if horizontal {
        1
    } else {
        1u32.wrapping_add(deco_height) as i32
    };
    minimum as f64 / parent_size as f64
}

fn proposed_shares(
    first: f64,
    second: f64,
    parent_size: u32,
    px: i32,
    ppt: i32,
    minimum_first: f64,
    minimum_second: f64,
) -> Option<(f64, f64)> {
    assert!(
        px == 0 || ppt == 0,
        "pixel and percentage changes are mutually exclusive"
    );
    let delta = if ppt != 0 {
        ppt as f64 / 100.0
    } else {
        px as f64 / parent_size as f64
    };
    let proposed = (first + delta, second - delta);
    if proposed.0 < minimum_first || proposed.1 < minimum_second {
        None
    } else {
        Some(proposed)
    }
}

fn divider_geometry(
    orientation: Orientation,
    second: Rect,
    first_focused: Rect,
    second_focused: Rect,
    thickness: u32,
) -> (Rect, u32) {
    let mut rect = second;
    let (start, extent, end) = if orientation == Orientation::Horizontal {
        rect.width = thickness;
        (first_focused.x, first_focused.width, second_focused.x)
    } else {
        rect.height = thickness;
        (first_focused.y, first_focused.height, second_focused.y)
    };
    let gap = end.wrapping_sub(start.wrapping_add(extent));
    (rect, end.wrapping_sub(gap / 2))
}

fn accepts_position(output: Rect, orientation: Orientation, position: u32) -> bool {
    let (start, extent) = if orientation == Orientation::Horizontal {
        (output.x, output.width)
    } else {
        (output.y, output.height)
    };
    position <= start.wrapping_add(extent).wrapping_sub(25) && position >= start.wrapping_add(25)
}

unsafe fn resize_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    // drag_pointer invokes this callback synchronously, on the event thread;
    // extra is the live unique stack context passed below, not an arbitrary pointer.
    let params = extra.cast_mut().cast::<DragState>();
    let output: *mut Con = (*params).output;
    debuglog(
        c"%s:%s:%d - new x = %d, y = %d\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_callback".as_ptr(),
        30,
        new_x,
        new_y,
    );
    if !(*params).threshold_exceeded {
        xcb_map_window(conn, (*params).helpwin);
        if (*params).orientation == Orientation::Horizontal {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as i16,
                0 as i16,
                0 as u16,
                0 as u16,
                ((*params).new_position)
                    .wrapping_add(new_x)
                    .wrapping_sub((*event).root_x as u32) as i16,
                new_y as i16,
            );
        } else {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as i16,
                0 as i16,
                0 as u16,
                0 as u16,
                new_x as i16,
                ((*params).new_position)
                    .wrapping_add(new_y)
                    .wrapping_sub((*event).root_y as u32) as i16,
            );
        }
        (*params).threshold_exceeded = true;
        return;
    }
    let position = if (*params).orientation == Orientation::Horizontal {
        new_x
    } else {
        new_y
    };
    if !accepts_position((*output).rect, (*params).orientation, position) {
        return;
    }
    (*params).new_position = position;
    let mask = if (*params).orientation == Orientation::Horizontal {
        XCB_CONFIG_WINDOW_X
    } else {
        XCB_CONFIG_WINDOW_Y
    };
    xcb_configure_window(
        conn,
        (*params).helpwin,
        mask as u16,
        (&raw const (*params).new_position).cast(),
    );
    xcb_flush(conn);
}

pub unsafe fn resize_find_tiling_participants(
    current: *mut *mut Con,
    other: *mut *mut Con,
    direction: Direction,
    both_sides: bool,
) -> bool {
    debuglog(
        c"%s:%s:%d - Find two participants for resizing container=%p in direction=%i\n"
            .as_ptr()
            .cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_find_tiling_participants".as_ptr(),
        73,
        other,
        direction as u32,
    );
    let mut first: *mut Con = *current;
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    if first.is_null() {
        debuglog(
            c"%s:%s:%d - Current container is NULL, aborting.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_find_tiling_participants".as_ptr(),
            77,
        );
        return false;
    }
    let search_orientation: Orientation = orientation_from_direction(direction);
    let dir_backwards: bool = direction == Direction::Up || direction == Direction::Left;
    while (*first).type_0 as u32 != CT_WORKSPACE
        && (*first).type_0 as u32 != CT_FLOATING_CON
        && second.is_null()
    {
        if con_orientation((*first).parent as *mut Con) != search_orientation
            || (*(*first).parent).layout == Layout::Stacked
            || (*(*first).parent).layout == Layout::Tabbed
        {
            first = (*first).parent as *mut Con;
        } else {
            if dir_backwards {
                second = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                if second.is_null() && both_sides {
                    second = (*first).nodes.tqe_next as *mut Con;
                }
            } else {
                second = (*first).nodes.tqe_next as *mut Con;
                if second.is_null() && both_sides {
                    second = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                }
            }
            if second.is_null() {
                debuglog(
                    c"%s:%s:%d - No second container in this direction found, trying to look further up in the tree...\n".as_ptr().cast_mut(),
                    c"../src/resize.c".as_ptr(),
                    c"resize_find_tiling_participants".as_ptr(),
                    110,
                );
                first = (*first).parent as *mut Con;
            }
        }
    }
    debuglog(
        c"%s:%s:%d - Found participants: first=%p and second=%p.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_find_tiling_participants".as_ptr(),
        115,
        first,
        second,
    );
    *current = first;
    *other = second;
    if first.is_null() || second.is_null() {
        debuglog(
            c"%s:%s:%d - Could not find two participants for this resize request.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_find_tiling_participants".as_ptr(),
            119,
        );
        return false;
    }
    true
}

pub unsafe fn percent_for_1px(con: *mut Con) -> f64 {
    let parent = (*con).parent;
    minimum_share(
        con_rect_size_in_orientation(parent) as i32,
        con_orientation(parent) == Orientation::Horizontal,
        (*con).deco_rect.height,
    )
}

pub unsafe fn resize_neighboring_cons(
    first: *mut Con,
    second: *mut Con,
    px: i32,
    ppt: i32,
) -> bool {
    let parent = (*first).parent;
    let Some((new_first_percent, new_second_percent)) = proposed_shares(
        (*first).percent,
        (*second).percent,
        con_rect_size_in_orientation(parent),
        px,
        ppt,
        percent_for_1px(first),
        percent_for_1px(second),
    ) else {
        return false;
    };
    (*first).percent = new_first_percent;
    (*second).percent = new_second_percent;
    con_fix_percent(parent);
    true
}

pub unsafe fn resize_graphical_handler(
    first: *mut Con,
    second: *mut Con,
    orientation: Orientation,
    event: *const xcb_button_press_event_t,
    use_threshold: bool,
) {
    let output: *mut Con = con_get_output(first);
    debuglog(
        c"%s:%s:%d - x = %d, width = %d\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        177,
        (*output).rect.x,
        (*output).rect.width,
    );
    debuglog(
        c"%s:%s:%d - first = %p / %s\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        178,
        first,
        (*first).name,
    );
    debuglog(
        c"%s:%s:%d - second = %p / %s\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        179,
        second,
        (*second).name,
    );
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32);
    xcb_flush(conn);
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 2] = [0; 2];
    mask = XCB_CW_OVERRIDE_REDIRECT as i32 as u32;
    values[0] = 1 as u32;
    let grabwin: xcb_window_t = create_window(
        conn,
        (*output).rect as Rect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_ONLY as i32 as u16,
        Cursor::Pointer,
        true,
        mask,
        &raw mut values as *mut u32,
    );
    let first_focused = con_descend_focused(first);
    let second_focused = con_descend_focused(second);
    let (helprect, initial_position) = divider_geometry(
        orientation,
        (*second).rect,
        (*first_focused).rect,
        (*second_focused).rect,
        logical_px(2) as u32,
    );
    mask = XCB_CW_BACK_PIXEL as i32 as u32;
    values[0] = config.client.focused.border.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as i32 as u32;
    values[1] = 1 as u32;
    let helpwin: xcb_window_t = create_window(
        conn,
        helprect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as i32 as u16,
        if orientation == Orientation::Horizontal {
            Cursor::ResizeHorizontal
        } else {
            Cursor::ResizeVertical
        },
        false,
        mask,
        &raw mut values as *mut u32,
    );
    if !use_threshold {
        xcb_map_window(conn, helpwin);
        if orientation == Orientation::Horizontal {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as i16,
                0 as i16,
                0 as u16,
                0 as u16,
                initial_position as i16,
                (*event).root_y,
            );
        } else {
            xcb_warp_pointer(
                conn,
                XCB_NONE as xcb_window_t,
                (*event).root,
                0 as i16,
                0 as i16,
                0 as u16,
                0 as u16,
                (*event).root_x,
                initial_position as i16,
            );
        }
    }
    xcb_circulate_window(conn, XCB_CIRCULATE_RAISE_LOWEST as i32 as u8, helpwin);
    xcb_flush(conn);
    let windows = ResizeWindows {
        connection: conn,
        help: helpwin,
        grab: grabwin,
    };
    let mut params = DragState {
        orientation,
        output,
        helpwin,
        new_position: initial_position,
        threshold_exceeded: !use_threshold,
    };
    tree_render();
    let drag_result: DragResult = drag_pointer(
        ::core::ptr::null_mut::<Con>(),
        event,
        grabwin,
        Cursor::Pointer,
        use_threshold,
        Some(resize_callback),
        (&raw mut params).cast(),
    );
    drop(windows);
    if drag_result == DragResult::Revert {
        return;
    }
    let pixels: i32 = params.new_position.wrapping_sub(initial_position) as i32;
    debuglog(
        c"%s:%s:%d - Done, pixels = %d\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        279,
        pixels,
    );
    if pixels == 0 {
        return;
    }
    assert!(
        (*first).percent > 0.0 && (*second).percent > 0.0,
        "resize participants must have positive percentages"
    );
    let result: bool = resize_neighboring_cons(first, second, pixels, 0) as bool;
    debuglog(
        c"%s:%s:%d - Graphical resize %s: first->percent = %f, second->percent = %f.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        291,
        if result {
            c"successful".as_ptr()
        } else {
            c"failed".as_ptr()
        },
        (*first).percent,
        (*second).percent,
    );
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tree::tree_render;
use crate::util::orientation_from_direction;
use crate::x::x_mask_event_mask;
use i3_common::dpi::logical_px;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_button_press_event_t, xcb_circulate_window, xcb_configure_window, xcb_destroy_window,
    xcb_flush, xcb_map_window, xcb_visualid_t, xcb_warp_pointer, xcb_window_t,
    XCB_CIRCULATE_RAISE_LOWEST, XCB_CONFIG_WINDOW_X, XCB_CONFIG_WINDOW_Y, XCB_COPY_FROM_PARENT,
    XCB_CW_BACK_PIXEL, XCB_CW_OVERRIDE_REDIRECT, XCB_EVENT_MASK_ENTER_WINDOW, XCB_NONE,
    XCB_WINDOW_CLASS_INPUT_ONLY, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
use crate::types::nodes_head;
use crate::types::Con;
use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::con::con_descend_focused;
use crate::con::con_fix_percent;
use crate::con::con_get_output;
use crate::con::con_orientation;
use crate::con::con_rect_size_in_orientation;
use crate::config::config;
use crate::drag::drag_pointer;
use crate::xcb::create_window;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resizing_preserves_minimums_and_float_comparisons() {
        assert_eq!(minimum_share(100, false, 19), 0.2);
        assert_eq!(
            proposed_shares(0.5, 0.5, 100, 10, 0, 0.01, 0.01),
            Some((0.6, 0.4))
        );
        assert!(proposed_shares(0.5, 0.5, 100, 50, 0, 0.01, 0.01).is_none());
        assert!(proposed_shares(f64::NAN, 0.5, 0, 0, 0, 0.01, 0.01)
            .unwrap()
            .0
            .is_nan());
        assert!(minimum_share(0, true, 0).is_infinite());
    }
    #[test]
    fn divider_and_threshold_preserve_wrap_and_inclusive_edges() {
        let a = Rect {
            x: (-100i32) as u32,
            y: 20,
            width: 80,
            height: 40,
        };
        let b = Rect {
            x: 0,
            y: 70,
            width: 100,
            height: 50,
        };
        let (rect, middle) = divider_geometry(Orientation::Horizontal, b, a, b, 2);
        assert_eq!((rect.width, rect.height, middle), (2, 50, (-10i32) as u32));
        let out = Rect {
            x: 10,
            y: 20,
            width: 100,
            height: 100,
        };
        assert!(accepts_position(out, Orientation::Horizontal, 35));
        assert!(accepts_position(out, Orientation::Horizontal, 85));
        assert!(!accepts_position(out, Orientation::Horizontal, 34));
        assert_eq!(1u32.wrapping_sub(u32::MAX) as i32, 2);
    }
}
