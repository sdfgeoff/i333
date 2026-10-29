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
// Mechanical file-by-file Rust translation of src/resize.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type i3String = _i3String;

pub type i3Font = Font;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type callback_t = Option<
    unsafe fn(
        *mut Con,
        *mut Rect,
        u32,
        u32,
        *const xcb_button_press_event_t,
        *const ::core::ffi::c_void,
    ) -> (),
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_params {
    pub orientation: Orientation,
    pub output: *mut Con,
    pub helpwin: xcb_window_t,
    pub new_position: *mut u32,
    pub threshold_exceeded: *mut bool,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn resize_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    let params: *const callback_params = extra as *const callback_params;
    let output: *mut Con = (*params).output;
    debuglog(
        c"%s:%s:%d - new x = %d, y = %d\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_callback".as_ptr(),
        30 as ::core::ffi::c_int,
        new_x,
        new_y,
    );
    if !*(*params).threshold_exceeded {
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
                (*(*params).new_position)
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
                (*(*params).new_position)
                    .wrapping_add(new_y)
                    .wrapping_sub((*event).root_y as u32) as i16,
            );
        }
        *(*params).threshold_exceeded = true_0 != 0;
        return;
    }
    if (*params).orientation == Orientation::Horizontal {
        if new_x
            > (*output)
                .rect
                .x
                .wrapping_add((*output).rect.width)
                .wrapping_sub(25 as u32)
            || new_x < (*output).rect.x.wrapping_add(25 as u32)
        {
            return;
        }
        *(*params).new_position = new_x;
        xcb_configure_window(
            conn,
            (*params).helpwin,
            XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as u16,
            (*params).new_position as *const ::core::ffi::c_void,
        );
    } else {
        if new_y
            > (*output)
                .rect
                .y
                .wrapping_add((*output).rect.height)
                .wrapping_sub(25 as u32)
            || new_y < (*output).rect.y.wrapping_add(25 as u32)
        {
            return;
        }
        *(*params).new_position = new_y;
        xcb_configure_window(
            conn,
            (*params).helpwin,
            XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int as u16,
            (*params).new_position as *const ::core::ffi::c_void,
        );
    }
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
        73 as ::core::ffi::c_int,
        other,
        direction as ::core::ffi::c_uint,
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
            77 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let search_orientation: Orientation = orientation_from_direction(direction);
    let dir_backwards: bool = direction == Direction::Up || direction == Direction::Left;
    while (*first).type_0 as ::core::ffi::c_uint
        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*first).type_0 as ::core::ffi::c_uint
            != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
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
                if second.is_null() && both_sides as ::core::ffi::c_int == true_0 {
                    second = (*first).nodes.tqe_next as *mut Con;
                }
            } else {
                second = (*first).nodes.tqe_next as *mut Con;
                if second.is_null() && both_sides as ::core::ffi::c_int == true_0 {
                    second = *(*((*first).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
                }
            }
            if second.is_null() {
                debuglog(
                    c"%s:%s:%d - No second container in this direction found, trying to look further up in the tree...\n".as_ptr().cast_mut(),
                    c"../src/resize.c".as_ptr(),
                    c"resize_find_tiling_participants".as_ptr(),
                    110 as ::core::ffi::c_int,
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
        115 as ::core::ffi::c_int,
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
            119 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    true_0 != 0
}

pub unsafe fn percent_for_1px(con: *mut Con) -> ::core::ffi::c_double {
    let parent_size: ::core::ffi::c_int =
        con_rect_size_in_orientation((*con).parent as *mut Con) as ::core::ffi::c_int;
    let min_size: ::core::ffi::c_int =
        (if con_orientation((*con).parent as *mut Con) == Orientation::Horizontal {
            1 as u32
        } else {
            (1 as u32).wrapping_add((*con).deco_rect.height)
        }) as ::core::ffi::c_int;
    min_size as ::core::ffi::c_double / parent_size as ::core::ffi::c_double
}

pub unsafe fn resize_neighboring_cons(
    first: *mut Con,
    second: *mut Con,
    px: ::core::ffi::c_int,
    ppt: ::core::ffi::c_int,
) -> bool {
    if px * ppt == 0 as ::core::ffi::c_int {
    } else {
        __assert_fail(
            c"px * ppt == 0".as_ptr(),
            c"../src/resize.c".as_ptr(),
            147 as ::core::ffi::c_uint,
            c"_Bool resize_neighboring_cons(Con *, Con *, int, int)".as_ptr(),
        );
    };
    let parent: *mut Con = (*first).parent as *mut Con;
    let mut new_first_percent: ::core::ffi::c_double = 0.;
    let mut new_second_percent: ::core::ffi::c_double = 0.;
    if ppt != 0 {
        new_first_percent = (*first).percent + ppt as ::core::ffi::c_double / 100.0f64;
        new_second_percent = (*second).percent - ppt as ::core::ffi::c_double / 100.0f64;
    } else {
        let pct: ::core::ffi::c_double = px as ::core::ffi::c_double
            / con_rect_size_in_orientation((*first).parent as *mut Con) as ::core::ffi::c_double;
        new_first_percent = (*first).percent + pct;
        new_second_percent = (*second).percent - pct;
    }
    if new_first_percent < percent_for_1px(first) || new_second_percent < percent_for_1px(second) {
        return false_0 != 0;
    }
    (*first).percent = new_first_percent;
    (*second).percent = new_second_percent;
    con_fix_percent(parent);
    true_0 != 0
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
        177 as ::core::ffi::c_int,
        (*output).rect.x,
        (*output).rect.width,
    );
    debuglog(
        c"%s:%s:%d - first = %p / %s\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        178 as ::core::ffi::c_int,
        first,
        (*first).name,
    );
    debuglog(
        c"%s:%s:%d - second = %p / %s\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        179 as ::core::ffi::c_int,
        second,
        (*second).name,
    );
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as u32);
    xcb_flush(conn);
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 2] = [0; 2];
    mask = XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
    values[0 as ::core::ffi::c_int as usize] = 1 as u32;
    let grabwin: xcb_window_t = create_window(
        conn,
        (*output).rect as Rect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int as u16,
        Cursor::Pointer,
        true_0 != 0,
        mask,
        &raw mut values as *mut u32,
    );
    let mut initial_position: u32 = 0;
    let mut new_position: u32 = 0;
    let mut helprect: Rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    helprect.x = (*second).rect.x;
    helprect.y = (*second).rect.y;
    let ffirst: *mut Con = con_descend_focused(first);
    let fsecond: *mut Con = con_descend_focused(second);
    if orientation == Orientation::Horizontal {
        helprect.width = logical_px(2 as ::core::ffi::c_int) as u32;
        helprect.height = (*second).rect.height;
        let ffirst_right: u32 = (*ffirst).rect.x.wrapping_add((*ffirst).rect.width);
        let gap: u32 = (*fsecond).rect.x.wrapping_sub(ffirst_right);
        let middle: u32 = (*fsecond).rect.x.wrapping_sub(gap.wrapping_div(2 as u32));
        debuglog(
            c"%s:%s:%d - ffirst->rect = {.x = %u, .width = %u}\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            215 as ::core::ffi::c_int,
            (*ffirst).rect.x,
            (*ffirst).rect.width,
        );
        debuglog(
            c"%s:%s:%d - fsecond->rect = {.x = %u, .width = %u}\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            216 as ::core::ffi::c_int,
            (*fsecond).rect.x,
            (*fsecond).rect.width,
        );
        debuglog(
            c"%s:%s:%d - gap = %u, middle = %u\n".as_ptr().cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            217 as ::core::ffi::c_int,
            gap,
            middle,
        );
        initial_position = middle;
    } else {
        helprect.width = (*second).rect.width;
        helprect.height = logical_px(2 as ::core::ffi::c_int) as u32;
        let ffirst_bottom: u32 = (*ffirst).rect.y.wrapping_add((*ffirst).rect.height);
        let gap_0: u32 = (*fsecond).rect.y.wrapping_sub(ffirst_bottom);
        let middle_0: u32 = (*fsecond).rect.y.wrapping_sub(gap_0.wrapping_div(2 as u32));
        debuglog(
            c"%s:%s:%d - ffirst->rect = {.y = %u, .height = %u}\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            225 as ::core::ffi::c_int,
            (*ffirst).rect.y,
            (*ffirst).rect.height,
        );
        debuglog(
            c"%s:%s:%d - fsecond->rect = {.y = %u, .height = %u}\n"
                .as_ptr()
                .cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            226 as ::core::ffi::c_int,
            (*fsecond).rect.y,
            (*fsecond).rect.height,
        );
        debuglog(
            c"%s:%s:%d - gap = %u, middle = %u\n".as_ptr().cast_mut(),
            c"../src/resize.c".as_ptr(),
            c"resize_graphical_handler".as_ptr(),
            227 as ::core::ffi::c_int,
            gap_0,
            middle_0,
        );
        initial_position = middle_0;
    }
    mask = XCB_CW_BACK_PIXEL as ::core::ffi::c_int as u32;
    values[0 as ::core::ffi::c_int as usize] = config.client.focused.border.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
    values[1 as ::core::ffi::c_int as usize] = 1 as u32;
    let helpwin: xcb_window_t = create_window(
        conn,
        helprect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        if orientation == Orientation::Horizontal {
            Cursor::ResizeHorizontal
        } else {
            Cursor::ResizeVertical
        },
        false_0 != 0,
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
    xcb_circulate_window(
        conn,
        XCB_CIRCULATE_RAISE_LOWEST as ::core::ffi::c_int as u8,
        helpwin,
    );
    xcb_flush(conn);
    new_position = initial_position;
    let mut threshold_exceeded: bool = !use_threshold;
    let params: callback_params = callback_params {
        orientation,
        output,
        helpwin,
        new_position: &raw mut new_position,
        threshold_exceeded: &raw mut threshold_exceeded,
    };
    tree_render();
    let drag_result: DragResult = drag_pointer(
        ::core::ptr::null_mut::<Con>(),
        event,
        grabwin,
        Cursor::Pointer,
        use_threshold,
        Some(
            resize_callback
                as unsafe fn(
                    *mut Con,
                    *mut Rect,
                    u32,
                    u32,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw const params as *const ::core::ffi::c_void,
    );
    xcb_destroy_window(conn, helpwin);
    xcb_destroy_window(conn, grabwin);
    xcb_flush(conn);
    if drag_result == DragResult::Revert {
        return;
    }
    let pixels: ::core::ffi::c_int =
        new_position.wrapping_sub(initial_position) as ::core::ffi::c_int;
    debuglog(
        c"%s:%s:%d - Done, pixels = %d\n".as_ptr().cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        279 as ::core::ffi::c_int,
        pixels,
    );
    if pixels == 0 as ::core::ffi::c_int {
        return;
    }
    if (*first).percent > 0.0f64 {
    } else {
        __assert_fail(
            c"first->percent > 0.0".as_ptr(),
            c"../src/resize.c".as_ptr(),
            287 as ::core::ffi::c_uint,
            c"void resize_graphical_handler(Con *, Con *, orientation_t, const xcb_button_press_event_t *, _Bool)".as_ptr(),
        );
    };
    if (*second).percent > 0.0f64 {
    } else {
        __assert_fail(
            c"second->percent > 0.0".as_ptr(),
            c"../src/resize.c".as_ptr(),
            288 as ::core::ffi::c_uint,
            c"void resize_graphical_handler(Con *, Con *, orientation_t, const xcb_button_press_event_t *, _Bool)".as_ptr(),
        );
    };
    let result: bool =
        resize_neighboring_cons(first, second, pixels, 0 as ::core::ffi::c_int) as bool;
    debuglog(
        c"%s:%s:%d - Graphical resize %s: first->percent = %f, second->percent = %f.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/resize.c".as_ptr(),
        c"resize_graphical_handler".as_ptr(),
        291 as ::core::ffi::c_int,
        if result as ::core::ffi::c_int != 0 {
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

use i3_common::native_stdio::__assert_fail;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

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
