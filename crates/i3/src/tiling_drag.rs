use crate::con::arena::{containers, resolve};
use crate::con::{CF_NONE, CF_OUTPUT, CT_WORKSPACE};
pub use crate::types::xoutput as Output;
use crate::types::Direction;
use crate::types::DragResult;
use crate::types::DropType;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::Position;
use crate::xcursor::Cursor;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * tiling_drag.c: Reposition tiled windows by dragging.
 *
 */
// Indicator geometry consumes copied values; graph selection/move/focus remains
// an exclusive event-thread boundary. Callback pointers address local state only
// throughout synchronous drag_pointer; no references span its nested loop.

#[derive(Copy, Clone)]
#[repr(C)]
struct DropParameters {
    pub indicator: *mut xcb_window_t,
    pub target: *mut *mut Con,
    pub direction: *mut Direction,
    pub drop_type: *mut DropType,
}

unsafe fn is_tiling_drop_target(con: *mut Con) -> bool {
    if !con_has_managed_window(con)
        || con_is_floating(con) as i32 != 0
        || con_is_hidden(con) as i32 != 0
    {
        return false;
    }
    let ws: *mut Con = con_get_workspace(con);
    if con_is_internal(ws) {
        return false;
    }
    if !workspace_is_visible(ws) {
        return false;
    }
    let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fs.is_null() && fs != con {
        return false;
    }
    true
}

pub unsafe fn has_drop_targets() -> bool {
    let mut drop_targets: i32 = 0;
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        if is_tiling_drop_target(con) {
            drop_targets += 1;
        }
    }
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).focus_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut visible_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let mut child = (*output_get_content(output)).nodes_head.tqh_first;
            while !child.is_null() {
                if workspace_is_visible(child) {
                    visible_ws = child;
                    break;
                }
                child = (*child).nodes.tqe_next;
            }
            if !visible_ws.is_null() && con_num_children(visible_ws) == 0 {
                drop_targets += 1;
            }
        }
        output = (*output).focused.tqe_next as *mut Con;
    }
    drop_targets > 1
}
unsafe fn find_drop_target(x: u32, y: u32) -> *mut Con {
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        let rect: Rect = (*con).rect;
        if !rect_contains(rect, x, y) || !is_tiling_drop_target(con) {
            continue;
        } else {
            let ws: *mut Con = con_get_workspace(con);
            let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
            return if !fs.is_null() { fs } else { con };
        }
    }
    let output: *mut Output = get_output_containing(x as u32, y as u32);
    if output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let content: *mut Con = output_get_content((*output).con);
    con_descend_tiling_focused(content)
}
fn adjust_rect(mut rect: Rect, direction: Direction, threshold: u32) -> Rect {
    match direction {
        Direction::Left => {
            rect.width = threshold;
        }
        Direction::Up => {
            rect.height = threshold;
        }
        Direction::Right => {
            rect.x = rect.x.wrapping_add(rect.width.wrapping_sub(threshold));
            rect.width = threshold;
        }
        Direction::Down => {
            rect.y = rect.y.wrapping_add(rect.height.wrapping_sub(threshold));
            rect.height = threshold;
        }
    }
    rect
}
unsafe fn con_on_side_of_parent(con: *mut Con, direction: Direction) -> bool {
    let orientation: Orientation = orientation_from_direction(direction);
    let mut reverse_direction: Direction = Direction::Left;
    match direction {
        Direction::Left => {
            reverse_direction = Direction::Right;
        }
        Direction::Right => {
            reverse_direction = Direction::Left;
        }
        Direction::Up => {
            reverse_direction = Direction::Down;
        }
        Direction::Down => {
            reverse_direction = Direction::Up;
        }
    }
    con_orientation((*con).parent as *mut Con) != orientation
        || (*(*con).parent).layout == Layout::Stacked
        || (*(*con).parent).layout == Layout::Tabbed
        || con_descend_direction((*con).parent as *mut Con, reverse_direction) == con
}
unsafe fn drag_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    let mut target: *mut Con = find_drop_target(new_x, new_y);
    if target.is_null() {
        return;
    }
    let mut rect: Rect = (*target).rect;
    let mut direction: Direction = Direction::Left;
    let mut drop_type: DropType = DropType::Center;
    let mut draw_window: bool = true;
    let params: *const DropParameters = extra as *const DropParameters;
    if !((*target).type_0 as u32 == CT_WORKSPACE as i32 as u32) {
        let edges = indicator_edges(
            rect,
            new_x,
            new_y,
            logical_px(2),
            render_deco_height() + logical_px(5),
        );
        let direction_from_edges = edges.direction;
        direction = direction_from_edges;
        let sibling_indicator_size = edges.sibling;
        let parent_indicator_size = edges.parent;
        let d_min = edges.distance;
        let target_parent;
        let target_sibling;
        target_parent =
            d_min < parent_indicator_size && con_on_side_of_parent(target, direction) as i32 != 0;
        target_sibling = d_min < sibling_indicator_size;
        drop_type = if target_parent {
            DropType::Parent
        } else {
            if target_sibling {
                DropType::Sibling
            } else {
                DropType::Center
            }
        };
        if drop_type != DropType::Parent && target == con {
            draw_window = false;
            xcb_destroy_window(conn, *(*params).indicator);
            *(*params).indicator = 0 as xcb_window_t;
        } else {
            match drop_type {
                DropType::Parent => {
                    while (*(*target).parent).type_0 as u32 != CT_WORKSPACE as i32 as u32
                        && con_on_side_of_parent((*target).parent as *mut Con, direction) as i32
                            != 0
                    {
                        target = (*target).parent as *mut Con;
                    }
                    rect = adjust_rect((*(*target).parent).rect, direction, parent_indicator_size);
                }
                DropType::Center => {
                    rect = (*target).rect;
                    rect.x = rect.x.wrapping_add(sibling_indicator_size);
                    rect.y = rect.y.wrapping_add(sibling_indicator_size);
                    rect.width = rect
                        .width
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as u32));
                    rect.height = rect
                        .height
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as u32));
                }
                DropType::Sibling => {
                    rect = adjust_rect((*target).rect, direction, sibling_indicator_size);
                }
            }
        }
    }
    if draw_window {
        if *(*params).indicator == 0 as xcb_window_t {
            *(*params).indicator = create_drop_indicator(rect);
        } else {
            let values: [u32; 4] = [rect.x, rect.y, rect.width, rect.height];
            let mask: u32 = (XCB_CONFIG_WINDOW_X as i32
                | XCB_CONFIG_WINDOW_Y as i32
                | XCB_CONFIG_WINDOW_WIDTH as i32
                | XCB_CONFIG_WINDOW_HEIGHT as i32) as u32;
            xcb_configure_window(
                conn,
                *(*params).indicator,
                mask as u16,
                &raw const values as *const u32 as *const ::core::ffi::c_void,
            );
        }
    }
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32);
    xcb_flush(conn);
    *(*params).target = target;
    *(*params).direction = direction;
    *(*params).drop_type = drop_type;
}
unsafe fn create_drop_indicator(rect: Rect) -> xcb_window_t {
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 2] = [0; 2];
    mask = XCB_CW_BACK_PIXEL as i32 as u32;
    values[0 as usize] = config.client.focused.indicator.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as i32 as u32;
    values[1 as usize] = 1 as u32;
    let indicator: xcb_window_t = create_window(
        conn,
        rect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as i32 as u16,
        Cursor::Move,
        false,
        mask,
        &raw mut values as *mut u32,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        indicator,
        XCB_ATOM_WM_CLASS as i32 as xcb_atom_t,
        XCB_ATOM_STRING as i32 as xcb_atom_t,
        8 as u8,
        16,
        b"i3-drag\0i3-drag\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, indicator);
    xcb_circulate_window(conn, XCB_CIRCULATE_RAISE_LOWEST as i32 as u8, indicator);
    indicator
}

pub unsafe fn tiling_drag(
    con: *mut Con,
    event: *mut xcb_button_press_event_t,
    use_threshold: bool,
) {
    debuglog(
        c"%s:%s:%d - Start dragging tiled container: con = %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/tiling_drag.c".as_ptr(),
        c"tiling_drag".as_ptr(),
        305,
        con,
    );
    let mut set_focus: bool = con == focused;
    let mut set_fs: bool = (*con).fullscreen_mode as u32 != CF_NONE as i32 as u32;
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32);
    xcb_flush(conn);
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut direction: Direction = Direction::Left;
    let mut drop_type: DropType = DropType::Sibling;
    let mut indicator = Indicator {
        connection: conn,
        window: 0,
    };
    let params: DropParameters = DropParameters {
        indicator: &raw mut indicator.window,
        target: &raw mut target,
        direction: &raw mut direction,
        drop_type: &raw mut drop_type,
    };
    let drag_result: DragResult = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        Cursor::Move,
        use_threshold,
        Some(
            drag_callback
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
    drop(indicator);
    if drag_result == DragResult::Revert
        || target.is_null()
        || target == con && drop_type != DropType::Parent
        || !con_exists(target)
    {
        debuglog(
            c"%s:%s:%d - drop aborted\n".as_ptr().cast_mut(),
            c"../src/tiling_drag.c".as_ptr(),
            c"tiling_drag".as_ptr(),
            329,
        );
        return;
    }
    let orientation: Orientation = orientation_from_direction(direction);
    let position: Position = position_from_direction(direction);
    let layout: Layout = if orientation == Orientation::Vertical {
        Layout::SplitV
    } else {
        Layout::SplitH
    };
    con_disable_fullscreen(con);
    let mut mod_0: u32 = 0;
    let mut swap_pressed: bool = false;
    match drop_type {
        DropType::Center => {
            debuglog(
                c"%s:%s:%d - drop to center of %p\n".as_ptr().cast_mut(),
                c"../src/tiling_drag.c".as_ptr(),
                c"tiling_drag".as_ptr(),
                340,
                target,
            );
            mod_0 = config.swap_modifier & 0xffff as u32;
            swap_pressed = mod_0 != 0 as u32 && (*event).state as u32 & mod_0 == mod_0;
            if swap_pressed {
                if !con_swap(con, target) {
                    return;
                }
            } else {
                con_move_to_target(con, target);
            }
        }
        DropType::Sibling => {
            debuglog(
                c"%s:%s:%d - drop %s %p\n".as_ptr().cast_mut(),
                c"../src/tiling_drag.c".as_ptr(),
                c"tiling_drag".as_ptr(),
                352,
                position_to_string(position),
                target,
            );
            if con_orientation((*target).parent as *mut Con) != orientation {
                if (*con).parent == (*target).parent
                    && con_num_children((*target).parent as *mut Con) == 2
                {
                    (*(*target).parent).layout = layout;
                } else {
                    tree_split(target, orientation);
                }
            }
            insert_con_into(con, target, position);
            ipc_send_window_event(c"move".as_ptr(), con);
        }
        DropType::Parent => {
            let parent_tabbed_or_stacked: bool = (*(*target).parent).layout == Layout::Tabbed
                || (*(*target).parent).layout == Layout::Stacked;
            debuglog(
                c"%s:%s:%d - drop %s (%s) of %s%p\n".as_ptr().cast_mut(),
                c"../src/tiling_drag.c".as_ptr(),
                c"tiling_drag".as_ptr(),
                375,
                direction_to_string(direction),
                position_to_string(position),
                if parent_tabbed_or_stacked {
                    c"tabbed/stacked ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                target,
            );
            if parent_tabbed_or_stacked {
                if position == Position::Before {
                    target = (*(*target).parent).nodes_head.tqh_first as *mut Con;
                } else {
                    target = *(*((*(*target).parent).nodes_head.tqh_last as *mut nodes_head))
                        .tqh_last as *mut Con;
                }
            }
            if con != target {
                insert_con_into(con, target, position);
            }
            let old_focus: *mut Con = focused;
            tree_move(con, direction);
            if focused != old_focus && con_exists(old_focus) as i32 != 0 {
                con_activate(old_focus);
            }
        }
    }
    target = ::core::ptr::null_mut::<Con>();
    if set_focus || set_fs {
        let fs: *mut Con = con_get_fullscreen_covering_ws(con_get_workspace(con));
        if fs == con {
            errorlog(
                c"ERROR: dragged container somehow got fullscreen again.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            if 0 != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/tiling_drag.c".as_ptr(),
                    406 as u32,
                    c"void tiling_drag(Con *, xcb_button_press_event_t *, _Bool)".as_ptr(),
                );
            };
        } else if !fs.is_null() && set_focus && set_fs {
            con_disable_fullscreen(fs);
        } else if !fs.is_null() {
            set_focus = false;
            set_fs = set_focus;
        } else if !set_focus {
            set_focus = set_fs && con_get_workspace(focused) == con_get_workspace(con);
        }
    }
    if set_fs {
        con_enable_fullscreen(con, CF_OUTPUT);
    }
    if set_focus {
        workspace_show(con_get_workspace(con));
        con_focus(con);
    }
    tree_render();
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::render::render_deco_height;
use crate::tree::tree_render;
use crate::util::direction_to_string;
use crate::util::max;
use crate::util::min;
use crate::util::orientation_from_direction;
use crate::util::position_from_direction;
use crate::util::position_to_string;
use crate::x::x_mask_event_mask;
use i3_common::dpi::logical_px;

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_button_press_event_t, xcb_change_property, xcb_circulate_window,
    xcb_configure_window, xcb_connection_t, xcb_destroy_window, xcb_flush, xcb_map_window,
    xcb_visualid_t, xcb_window_t, XCB_ATOM_STRING, XCB_ATOM_WM_CLASS, XCB_CIRCULATE_RAISE_LOWEST,
    XCB_CONFIG_WINDOW_HEIGHT, XCB_CONFIG_WINDOW_WIDTH, XCB_CONFIG_WINDOW_X, XCB_CONFIG_WINDOW_Y,
    XCB_COPY_FROM_PARENT, XCB_CW_BACK_PIXEL, XCB_CW_OVERRIDE_REDIRECT, XCB_EVENT_MASK_ENTER_WINDOW,
    XCB_NONE, XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::con::con_activate;
use crate::con::con_descend_direction;
use crate::con::con_descend_tiling_focused;
use crate::con::con_disable_fullscreen;
use crate::con::con_enable_fullscreen;
use crate::con::con_exists;
use crate::con::con_focus;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_get_workspace;
use crate::con::con_has_managed_window;
use crate::con::con_is_floating;
use crate::con::con_is_hidden;
use crate::con::con_is_internal;
use crate::con::con_move_to_target;
use crate::con::con_num_children;
use crate::con::con_orientation;
use crate::con::con_swap;
use crate::config::config;
use crate::drag::drag_pointer;
use crate::ipc::ipc_send_window_event;
use crate::output::output_get_content;
use crate::r#move::insert_con_into;
use crate::r#move::tree_move;
use crate::randr::get_output_containing;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_split;
use crate::util::rect_contains;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::xcb::create_window;

struct Indicator {
    connection: *mut xcb_connection_t,
    window: xcb_window_t,
}
impl Drop for Indicator {
    fn drop(&mut self) {
        unsafe {
            xcb_destroy_window(self.connection, self.window);
        }
    }
}
struct IndicatorEdges {
    direction: Direction,
    distance: u32,
    sibling: u32,
    parent: u32,
}
fn indicator_edges(rect: Rect, x: u32, y: u32, min_size: i32, max_parent: i32) -> IndicatorEdges {
    let dimension = (rect.width as i32).min(rect.height as i32) as u32;
    let sibling = min_size.max((0.3 * f64::from(dimension)) as u32 as i32) as u32;
    let parent = max_parent.min(sibling.wrapping_sub(1) as i32) as u32;
    let distances = [
        x.wrapping_sub(rect.x),
        y.wrapping_sub(rect.y),
        rect.x.wrapping_add(rect.width).wrapping_sub(x),
        rect.y.wrapping_add(rect.height).wrapping_sub(y),
    ];
    let distance = distances
        .into_iter()
        .map(|value| value as i32)
        .min()
        .unwrap() as u32;
    let index = distances
        .iter()
        .position(|&value| value == distance)
        .unwrap();
    IndicatorEdges {
        direction: [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ][index],
        distance,
        sibling,
        parent,
    }
}
#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn indicator_edges_keep_left_up_right_down_ties_and_signed_min() {
        let edges = indicator_edges(
            Rect {
                x: 10,
                y: 20,
                width: 100,
                height: 100,
            },
            60,
            70,
            2,
            20,
        );
        assert_eq!(edges.direction, Direction::Left);
        assert_eq!((edges.distance, edges.sibling, edges.parent), (50, 30, 20));
        let tiny = indicator_edges(
            Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            1,
            1,
            2,
            20,
        );
        assert_eq!(tiny.direction, Direction::Right);
        assert_eq!((tiny.sibling, tiny.parent), (2, 1));
        let outside = indicator_edges(
            Rect {
                x: 10,
                y: 20,
                width: 100,
                height: 100,
            },
            9,
            50,
            2,
            20,
        );
        assert_eq!(
            (outside.direction, outside.distance),
            (Direction::Left, u32::MAX)
        );
    }
    #[test]
    fn strips_preserve_wrapping_for_threshold_larger_than_target() {
        let rect = Rect {
            x: 10,
            y: 20,
            width: 1,
            height: 2,
        };
        let right = adjust_rect(rect, Direction::Right, 5);
        assert_eq!((right.x, right.width), (6, 5));
        let down = adjust_rect(rect, Direction::Down, 5);
        assert_eq!((down.y, down.height), (17, 5));
    }
}
