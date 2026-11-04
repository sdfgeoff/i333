use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, resolve};
use crate::domains::{ContainerType, FullscreenMode};
use xcb::{x, Xid, XidNew};

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
    pub indicator: *mut x::Window,
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
    output = focus::first_ptr(croot);
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut visible_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let mut child = nodes::first_ptr(output_get_content(output));
            while !child.is_null() {
                if workspace_is_visible(child) {
                    visible_ws = child;
                    break;
                }
                child = nodes::next_ptr(child);
            }
            if !visible_ws.is_null() && con_num_children(visible_ws) == 0 {
                drop_targets += 1;
            }
        }
        output = focus::next_ptr(output);
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
    con_orientation(parent_ptr(con)) != orientation
        || (*parent_ptr(con)).layout == Layout::Stacked
        || (*parent_ptr(con)).layout == Layout::Tabbed
        || con_descend_direction(parent_ptr(con), reverse_direction) == con
}
unsafe fn drag_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: &x::ButtonPressEvent,
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
    if !(((*target).type_0) == ContainerType::Workspace) {
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
            connection().send_request(&x::DestroyWindow {
                window: *(*params).indicator,
            });
            *(*params).indicator = x::WINDOW_NONE;
        } else {
            match drop_type {
                DropType::Parent => {
                    while ((*parent_ptr(target)).type_0) != ContainerType::Workspace
                        && con_on_side_of_parent(parent_ptr(target), direction) as i32 != 0
                    {
                        target = parent_ptr(target);
                    }
                    rect =
                        adjust_rect((*parent_ptr(target)).rect, direction, parent_indicator_size);
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
        if (*(*params).indicator).is_none() {
            *(*params).indicator = create_drop_indicator(rect);
        } else {
            connection().send_request(&x::ConfigureWindow {
                window: *(*params).indicator,
                value_list: &[
                    x::ConfigWindow::X(rect.x as i32),
                    x::ConfigWindow::Y(rect.y as i32),
                    x::ConfigWindow::Width(rect.width),
                    x::ConfigWindow::Height(rect.height),
                ],
            });
        }
    }
    x_mask_event_mask(!((::xcb::x::EventMask::ENTER_WINDOW.bits()) as i32) as u32);
    let _ = connection().flush();
    *(*params).target = target;
    *(*params).direction = direction;
    *(*params).drop_type = drop_type;
}
unsafe fn create_drop_indicator(rect: Rect) -> x::Window {
    let indicator = x::Window::new(create_window(
        &connection(),
        rect,
        0,
        0,
        x::WindowClass::InputOutput,
        Cursor::Move,
        false,
        &[
            x::Cw::BackPixel(config.client.focused.indicator.colorpixel),
            x::Cw::OverrideRedirect(true),
        ],
    ));
    connection().send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: indicator,
        property: x::ATOM_WM_CLASS,
        r#type: x::ATOM_STRING,
        // Native code copied exactly sixteen bytes, excluding its extra NUL.
        data: b"i3-drag\0i3-drag\0",
    });
    connection().send_request(&x::MapWindow { window: indicator });
    connection().send_request(&x::CirculateWindow {
        direction: x::Circulate::RaiseLowest,
        window: indicator,
    });
    indicator
}

pub unsafe fn tiling_drag(con: *mut Con, event: &x::ButtonPressEvent, use_threshold: bool) {
    ::log::debug!(
        "../src/tiling_drag.c:tiling_drag:305 - Start dragging tiled container: con = {}\n",
        i3_common::logging::pointer(con)
    );
    let mut set_focus: bool = con == focused;
    let mut set_fs: bool = ((*con).fullscreen_mode) != FullscreenMode::None;
    x_mask_event_mask(!((::xcb::x::EventMask::ENTER_WINDOW.bits()) as i32) as u32);
    let _ = connection().flush();
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut direction: Direction = Direction::Left;
    let mut drop_type: DropType = DropType::Sibling;
    let mut indicator = Indicator {
        window: x::WINDOW_NONE,
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
        0 as u32,
        Cursor::Move,
        use_threshold,
        Some(
            drag_callback
                as unsafe fn(
                    *mut Con,
                    *mut Rect,
                    u32,
                    u32,
                    &x::ButtonPressEvent,
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
        ::log::debug!("../src/tiling_drag.c:tiling_drag:329 - drop aborted\n");
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
            ::log::debug!(
                "../src/tiling_drag.c:tiling_drag:340 - drop to center of {}\n",
                i3_common::logging::pointer(target)
            );
            mod_0 = config.swap_modifier & 0xffff as u32;
            swap_pressed = mod_0 != 0 as u32
                && xcb_interop::events::button_press_state(event) as u32 & mod_0 == mod_0;
            if swap_pressed {
                if !con_swap(con, target) {
                    return;
                }
            } else {
                con_move_to_target(con, target);
            }
        }
        DropType::Sibling => {
            ::log::debug!(
                "../src/tiling_drag.c:tiling_drag:352 - drop {} {}\n",
                position_name(position),
                i3_common::logging::pointer(target)
            );
            if con_orientation(parent_ptr(target)) != orientation {
                if parent_ptr(con) == parent_ptr(target)
                    && con_num_children(parent_ptr(target)) == 2
                {
                    (*parent_ptr(target)).layout = layout;
                } else {
                    tree_split(target, orientation);
                }
            }
            insert_con_into(con, target, position);
            ipc_send_window_event(c"move".as_ptr(), con);
        }
        DropType::Parent => {
            let parent_tabbed_or_stacked: bool = (*parent_ptr(target)).layout == Layout::Tabbed
                || (*parent_ptr(target)).layout == Layout::Stacked;
            ::log::debug!(
                "../src/tiling_drag.c:tiling_drag:375 - drop {} ({}) of {}{}\n",
                direction_name(direction),
                position_name(position),
                if parent_tabbed_or_stacked {
                    "tabbed/stacked "
                } else {
                    ""
                },
                i3_common::logging::pointer(target)
            );
            if parent_tabbed_or_stacked {
                if position == Position::Before {
                    target = nodes::first_ptr(parent_ptr(target));
                } else {
                    target = nodes::last_ptr(parent_ptr(target));
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
            ::log::error!("ERROR: dragged container somehow got fullscreen again.\n");
            __assert_fail(
                c"false".as_ptr(),
                c"../src/tiling_drag.c".as_ptr(),
                406 as u32,
                c"void tiling_drag(Con *, xcb_button_press_event_t *, _Bool)".as_ptr(),
            );
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
        con_enable_fullscreen(con, FullscreenMode::Output);
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
use crate::util::direction_name;
use crate::util::max;
use crate::util::min;
use crate::util::orientation_from_direction;
use crate::util::position_from_direction;
use crate::util::position_name;
use crate::x::x_mask_event_mask;
use i3_common::dpi::logical_px;

// Canonical Pango backend ABI; font policy remains local.

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
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
    window: x::Window,
}
impl Drop for Indicator {
    fn drop(&mut self) {
        if let Some(connection) = i3_common::display::connection() {
            connection.send_request(&x::DestroyWindow {
                window: self.window,
            });
        }
    }
}
fn connection() -> std::rc::Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
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

use crate::con::arena::nodes;
