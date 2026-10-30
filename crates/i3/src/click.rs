use crate::bindings::get_binding_from_xcb_event;
use crate::bindings::run_binding;
use crate::commands_parser::command_result_free;
use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::CT_DOCKAREA;
use crate::con::{TILING_DRAG_MODIFIER, TILING_DRAG_MODIFIER_OR_TITLEBAR, TILING_DRAG_TITLEBAR};
use crate::types::ClickDestination;
use crate::types::Direction;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::Position;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * click.c: Button press (mouse click) events.
 *
 */
// Copied event/geometry policy is safe Rust. Tree traversal and focus, replay,
// command and graphical drag actions require the exclusive native event thread;
// never retain graph references across those callbacks or nested event loops.

pub use crate::application::Binding;

pub const BORDER_BOTTOM: u32 = 8;
pub const BORDER_TOP: u32 = 4;
pub const BORDER_RIGHT: u32 = 2;
pub const BORDER_LEFT: u32 = 1;
pub use crate::assignments::CommandResult;

unsafe fn tiling_resize_for_border(
    con: *mut Con,
    border: u32,
    event: *mut xcb_button_press_event_t,
    use_threshold: bool,
) -> bool {
    debuglog(
        c"%s:%s:%d - border = %d, con = %p\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize_for_border".as_ptr(),
        25,
        border as u32,
        con,
    );
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = con;
    let mut search_direction: Direction = Direction::Left;
    match border as u32 {
        1 => {
            search_direction = Direction::Left;
        }
        2 => {
            search_direction = Direction::Right;
        }
        4 => {
            search_direction = Direction::Up;
        }
        8 => {
            search_direction = Direction::Down;
        }
        _ => {
            errorlog(
                c"ERROR: BUG: invalid border value %d\n".as_ptr().cast_mut(),
                border as u32,
            );
            return false;
        }
    }
    let res: bool =
        resize_find_tiling_participants(&raw mut first, &raw mut second, search_direction, false);
    if !res {
        debuglog(
            c"%s:%s:%d - No second container in this direction found.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"tiling_resize_for_border".as_ptr(),
            49,
        );
        return false;
    }
    if (*first).fullscreen_mode as u32 != (*second).fullscreen_mode as u32 {
        debuglog(
            c"%s:%s:%d - Avoiding resize between containers with different fullscreen modes, %d != %d\n".as_ptr().cast_mut(),
            c"../src/click.c".as_ptr(),
            c"tiling_resize_for_border".as_ptr(),
            53,
            (*first).fullscreen_mode as u32,
            (*second).fullscreen_mode as u32,
        );
        return false;
    }
    if first != second {
    } else {
        __assert_fail(
            c"first != second".as_ptr(),
            c"../src/click.c".as_ptr(),
            57 as u32,
            c"_Bool tiling_resize_for_border(Con *, u32, xcb_button_press_event_t *, _Bool)"
                .as_ptr(),
        );
    };
    if parent_ptr(first) == parent_ptr(second) {
    } else {
        __assert_fail(
            c"first->parent == second->parent".as_ptr(),
            c"../src/click.c".as_ptr(),
            58 as u32,
            c"_Bool tiling_resize_for_border(Con *, u32, xcb_button_press_event_t *, _Bool)"
                .as_ptr(),
        );
    };
    if search_direction == Direction::Up || search_direction == Direction::Left {
        let tmp: *mut Con = first;
        first = second;
        second = tmp;
    }
    let orientation: Orientation = if border as u32 == BORDER_LEFT as i32 as u32
        || border as u32 == BORDER_RIGHT as i32 as u32
    {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    };
    resize_graphical_handler(first, second, orientation, event, use_threshold);
    debuglog(
        c"%s:%s:%d - After resize handler, rendering\n"
            .as_ptr()
            .cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize_for_border".as_ptr(),
        71,
    );
    tree_render();
    true
}
unsafe fn floating_mod_on_tiled_client(
    con: *mut Con,
    event: *mut xcb_button_press_event_t,
) -> bool {
    let to_right: i32 = (*con).rect.width.wrapping_sub((*event).event_x as u32) as i32;
    let to_left: i32 = (*event).event_x as i32;
    let to_top: i32 = (*event).event_y as i32;
    let to_bottom: i32 = (*con).rect.height.wrapping_sub((*event).event_y as u32) as i32;
    debuglog(
        c"%s:%s:%d - click was %d px to the right, %d px to the left, %d px to top, %d px to bottom\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"floating_mod_on_tiled_client".as_ptr(),
        94,
        to_right,
        to_left,
        to_top,
        to_bottom,
    );
    nearest_border([to_left, to_top, to_right, to_bottom])
        .is_some_and(|border| tiling_resize_for_border(con, border, event, false))
}
unsafe fn tiling_resize(
    con: *mut Con,
    event: *mut xcb_button_press_event_t,
    dest: ClickDestination,
    use_threshold: bool,
) -> bool {
    let bsr: Rect = con_border_style_rect(con);
    debuglog(
        c"%s:%s:%d - BORDER x = %d, y = %d for con %p, window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize".as_ptr(),
        131,
        (*event).event_x as i32,
        (*event).event_y as i32,
        con,
        (*event).event,
    );
    debuglog(
        c"%s:%s:%d - checks for right >= %d\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize".as_ptr(),
        132,
        (*con).window_rect.x.wrapping_add((*con).window_rect.width),
    );
    border_hit(
        (*con).rect,
        (*con).window_rect,
        bsr,
        (*event).event_x,
        (*event).event_y,
        dest,
        use_threshold,
    )
    .is_some_and(|(border, threshold)| tiling_resize_for_border(con, border, event, threshold))
}
unsafe fn allow_replay_pointer(time: xcb_timestamp_t) {
    xcb_allow_events(conn, XCB_ALLOW_REPLAY_POINTER as i32 as u8, time);
    xcb_flush(conn);
    tree_render();
}
unsafe fn route_click(con: *mut Con, event: *mut xcb_button_press_event_t, dest: ClickDestination) {
    let mod_pressed = modifier_pressed((*event).state, config.floating_modifier);
    debuglog(
        c"%s:%s:%d - --> click properties: mod = %d, destination = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        173,
        mod_pressed as i32,
        dest as u32,
    );
    debuglog(
        c"%s:%s:%d - --> OUTCOME = %p\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        174,
        con,
    );
    debuglog(
        c"%s:%s:%d - type = %d, name = %s\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        175,
        (*con).type_0 as u32,
        (*con).name,
    );
    if (*parent_ptr(con)).type_0 as u32 == CT_DOCKAREA as i32 as u32 {
        allow_replay_pointer((*event).time);
        return;
    }
    let bind: *mut Binding = get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
    if !bind.is_null()
        && binding_destination(
            dest,
            (*bind).exclude_titlebar,
            (*bind).whole_window,
            (*bind).border,
        )
    {
        let result: *mut CommandResult = run_binding(bind, con);
        xcb_allow_events(conn, XCB_ALLOW_ASYNC_POINTER as i32 as u8, (*event).time);
        xcb_flush(conn);
        command_result_free(result);
        return;
    }
    if (*event).response_type as i32 == XCB_BUTTON_RELEASE {
        allow_replay_pointer((*event).time);
        return;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    let focused_workspace: *mut Con = con_get_workspace(focused);
    if ws.is_null() {
        ws = focus::first_ptr(output_get_content(con_get_output(con)));
        if ws.is_null() {
            allow_replay_pointer((*event).time);
            return;
        }
    }
    let floatingcon: *mut Con = con_inside_floating(con);
    let proportional: bool =
        (*event).state as i32 & XCB_KEY_BUT_MASK_SHIFT as i32 == XCB_KEY_BUT_MASK_SHIFT as i32;
    let in_stacked: bool =
        (*parent_ptr(con)).layout == Layout::Stacked || (*parent_ptr(con)).layout == Layout::Tabbed;
    let was_focused: bool = focused == con;
    let is_left_click: bool = (*event).detail as i32 == XCB_BUTTON_INDEX_1 as i32;
    let is_right_click: bool = (*event).detail as i32 == XCB_BUTTON_INDEX_3 as i32;
    let is_left_or_right_click: bool = is_left_click || is_right_click;
    let is_scroll: bool = (*event).detail as i32 == XCB_BUTTON_INDEX_4 as i32
        || (*event).detail as i32 == XCB_BUTTON_INDEX_5 as i32
        || (*event).detail as i32 == XCB_BUTTON_SCROLL_LEFT
        || (*event).detail as i32 == XCB_BUTTON_SCROLL_RIGHT;
    if in_stacked && dest == ClickDestination::Decoration && is_scroll {
        debuglog(
            c"%s:%s:%d - Scrolling on a window decoration\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"route_click".as_ptr(),
            234,
        );
        workspace_show(ws);
        let current: *mut Con = focus::first_ptr(parent_ptr(con));
        let direction: Position = if (*event).detail as i32 == XCB_BUTTON_INDEX_4 as i32
            || (*event).detail as i32 == XCB_BUTTON_SCROLL_LEFT
        {
            Position::Before
        } else {
            Position::After
        };
        let next: *mut Con = get_tree_next_sibling(current, direction);
        con_activate(con_descend_focused(if !next.is_null() {
            next
        } else {
            current
        }));
        allow_replay_pointer((*event).time);
        return;
    }
    if mod_pressed
        && is_left_click
        && floatingcon.is_null()
        && (config.tiling_drag as u32 == TILING_DRAG_MODIFIER as i32 as u32
            || config.tiling_drag as u32 == TILING_DRAG_MODIFIER_OR_TITLEBAR as i32 as u32)
        && has_drop_targets() as i32 != 0
    {
        let use_threshold: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold);
        allow_replay_pointer((*event).time);
        return;
    }
    let mut con_to_focus: *mut Con = con;
    if in_stacked && dest == ClickDestination::Decoration {
        if was_focused || !con_has_parent(focused, con) {
            while !focus::first_ptr(con_to_focus).is_null() {
                con_to_focus = focus::first_ptr(con_to_focus);
            }
        }
    }
    if ws != focused_workspace {
        workspace_show(ws);
    }
    con_activate(con_to_focus);
    let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !floatingcon.is_null() && fs != con {
        if mod_pressed && is_left_click {
            floating_drag_window(floatingcon, event, false);
            return;
        }
        if mod_pressed && is_right_click {
            debuglog(
                c"%s:%s:%d - floating resize due to floatingmodifier\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                293,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if (dest == ClickDestination::Border || dest == ClickDestination::Decoration)
            && is_left_or_right_click
        {
            debuglog(
                c"%s:%s:%d - tiling resize with fallback\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                301,
            );
            if tiling_resize(
                con,
                event,
                dest,
                dest == ClickDestination::Decoration && !was_focused,
            ) {
                allow_replay_pointer((*event).time);
                return;
            }
        }
        if dest == ClickDestination::Decoration && is_right_click {
            debuglog(
                c"%s:%s:%d - floating resize due to decoration right click\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                309,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest == ClickDestination::Border && is_left_or_right_click {
            debuglog(
                c"%s:%s:%d - floating resize due to border click\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                315,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest == ClickDestination::Decoration && is_left_click {
            floating_drag_window(floatingcon, event, !was_focused);
            return;
        }
        allow_replay_pointer((*event).time);
        return;
    }
    if is_left_click
        && (config.tiling_drag as u32 == TILING_DRAG_TITLEBAR as i32 as u32
            && dest == ClickDestination::Decoration
            || config.tiling_drag as u32 == TILING_DRAG_MODIFIER_OR_TITLEBAR as i32 as u32
                && (mod_pressed || dest == ClickDestination::Decoration))
        && has_drop_targets() as i32 != 0
    {
        allow_replay_pointer((*event).time);
        let use_threshold_0: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold_0);
        return;
    }
    if dest == ClickDestination::Inside && mod_pressed && is_right_click {
        if floating_mod_on_tiled_client(con, event) {
            return;
        }
        xcb_allow_events(conn, XCB_ALLOW_ASYNC_POINTER as i32 as u8, (*event).time);
        xcb_flush(conn);
        return;
    }
    if (dest == ClickDestination::Border || dest == ClickDestination::Decoration)
        && is_left_or_right_click
    {
        debuglog(
            c"%s:%s:%d - Trying to resize (tiling)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"route_click".as_ptr(),
            357,
        );
        tiling_resize(
            con,
            event,
            dest,
            dest == ClickDestination::Decoration && !was_focused,
        );
    }
    allow_replay_pointer((*event).time);
}

pub unsafe fn handle_button_press(event: *mut xcb_button_press_event_t) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        c"%s:%s:%d - Button %d (state %d) %s on window 0x%08x (child 0x%08x) at (%d, %d) (root %d, %d)\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"handle_button_press".as_ptr(),
        377,
        (*event).detail as i32,
        (*event).state as i32,
        if (*event).response_type as i32 == XCB_BUTTON_PRESS {
            c"press".as_ptr()
        } else {
            c"release".as_ptr()
        },
        (*event).event,
        (*event).child,
        (*event).event_x as i32,
        (*event).event_y as i32,
        (*event).root_x as i32,
        (*event).root_y as i32,
    );
    last_timestamp = (*event).time;
    con = con_by_window_id((*event).event);
    if !con.is_null() {
        route_click(con, event, ClickDestination::Inside);
        return;
    }
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        if (*event).event == root {
            let bind: *mut Binding = get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
            if !bind.is_null() && (*bind).whole_window as i32 != 0 {
                let result: *mut CommandResult = run_binding(bind, ::core::ptr::null_mut::<Con>());
                command_result_free(result);
            }
        }
        if (*event).event == root && (*event).response_type as i32 == XCB_BUTTON_PRESS {
            let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
            output = (*croot).nodes_head.tqh_first as *mut Con;
            while !output.is_null() {
                if con_is_internal(output) as i32 != 0
                    || !rect_contains(
                        (*output).rect,
                        (*event).event_x as u32,
                        (*event).event_y as u32,
                    )
                {
                    output = (*output).nodes.tqe_next as *mut Con;
                } else {
                    let ws: *mut Con = focus::first_ptr(output_get_content(output));
                    if ws != con_get_workspace(focused) {
                        workspace_show(ws);
                        tree_render();
                    }
                    return;
                }
            }
            return;
        }
        errorlog(
            c"ERROR: Clicked into unknown window?!\n"
                .as_ptr()
                .cast_mut(),
        );
        xcb_allow_events(conn, XCB_ALLOW_REPLAY_POINTER as i32 as u8, (*event).time);
        xcb_flush(conn);
        return;
    }
    if !(*con).window.is_null() {
        if rect_contains(
            (*con).deco_rect,
            (*event).event_x as u32,
            (*event).event_y as u32,
        ) {
            route_click(con, event, ClickDestination::Decoration);
            return;
        }
    } else {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !child.is_null() {
            if !rect_contains(
                (*child).deco_rect,
                (*event).event_x as u32,
                (*event).event_y as u32,
            ) {
                child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                route_click(child, event, ClickDestination::Decoration);
                return;
            }
        }
    }
    if (*event).child as ::core::ffi::c_long != XCB_NONE {
        debuglog(
            c"%s:%s:%d - event->child not XCB_NONE, so this is an event which originated from a click into the application, but the application did not handle it.\n".as_ptr().cast_mut(),
            c"../src/click.c".as_ptr(),
            c"handle_button_press".as_ptr(),
            443,
        );
        route_click(con, event, ClickDestination::Inside);
        return;
    }
    route_click(con, event, ClickDestination::Border);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tiling_drag::has_drop_targets;
use crate::tree::tree_render;

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_allow_events, xcb_button_press_event_t, xcb_flush, xcb_generic_event_t, xcb_timestamp_t,
    XCB_ALLOW_ASYNC_POINTER, XCB_ALLOW_REPLAY_POINTER, XCB_BUTTON_INDEX_1, XCB_BUTTON_INDEX_3,
    XCB_BUTTON_INDEX_4, XCB_BUTTON_INDEX_5, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE,
    XCB_BUTTON_SCROLL_LEFT, XCB_BUTTON_SCROLL_RIGHT, XCB_KEY_BUT_MASK_SHIFT, XCB_NONE,
};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::Con;
pub use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::last_timestamp;
use crate::application::root;
use crate::con::con_activate;
use crate::con::con_border_style_rect;
use crate::con::con_by_frame_id;
use crate::con::con_by_window_id;
use crate::con::con_descend_focused;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_has_parent;
use crate::con::con_inside_floating;
use crate::con::con_is_internal;
use crate::config::config;
use crate::floating::floating_drag_window;
use crate::floating::floating_resize_window;
use crate::output::output_get_content;
use crate::resize::resize_find_tiling_participants;
use crate::resize::resize_graphical_handler;
use crate::tiling_drag::tiling_drag;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::get_tree_next_sibling;
use crate::util::rect_contains;
use crate::workspace::workspace_show;

fn nearest_border([left, top, right, bottom]: [i32; 4]) -> Option<u32> {
    if right < left && right < top && right < bottom {
        Some(BORDER_RIGHT)
    } else if left < right && left < top && left < bottom {
        Some(BORDER_LEFT)
    } else if top < right && top < left && top < bottom {
        Some(BORDER_TOP)
    } else if bottom < right && bottom < left && bottom < top {
        Some(BORDER_BOTTOM)
    } else {
        None
    }
}
fn border_hit(
    rect: Rect,
    window: Rect,
    bsr: Rect,
    x: i16,
    y: i16,
    dest: ClickDestination,
    use_threshold: bool,
) -> Option<(u32, bool)> {
    if dest == ClickDestination::Decoration {
        return Some((BORDER_TOP, use_threshold));
    } else if dest == ClickDestination::Border {
        if y as i32 >= 0
            && y as i32 <= bsr.y as i32
            && x as i32 >= bsr.x as i32
            && x as i32 <= rect.width.wrapping_add(bsr.width) as i32
        {
            return Some((BORDER_TOP, false));
        }
    }
    if x as i32 >= 0
        && x as i32 <= bsr.x as i32
        && y as i32 >= bsr.y as i32
        && y as i32 <= rect.height.wrapping_add(bsr.height) as i32
    {
        return Some((BORDER_LEFT, false));
    }
    if x as i32 >= window.x.wrapping_add(window.width) as i32
        && y as i32 >= bsr.y as i32
        && y as i32 <= rect.height.wrapping_add(bsr.height) as i32
    {
        return Some((BORDER_RIGHT, false));
    }
    if y as i32 >= window.y.wrapping_add(window.height) as i32 {
        return Some((BORDER_BOTTOM, false));
    }
    None
}

fn modifier_pressed(state: u16, configured: u32) -> bool {
    let mask = configured & 0xffff;
    mask != 0 && u32::from(state) & mask == mask
}
fn binding_destination(
    dest: ClickDestination,
    exclude_titlebar: bool,
    whole_window: bool,
    border: bool,
) -> bool {
    match dest {
        ClickDestination::Decoration => !exclude_titlebar,
        ClickDestination::Inside => whole_window,
        ClickDestination::Border => border,
    }
}
#[cfg(test)]
mod owned_tests {
    use super::*;
    fn rect(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x,
            y,
            width: w,
            height: h,
        }
    }
    #[test]
    fn border_precedence_threshold_and_signed_coordinates_remain_distinct() {
        assert_eq!(nearest_border([1, 1, 9, 9]), None);
        assert_eq!(nearest_border([-1, 5, 11, 5]), Some(BORDER_LEFT));
        let bounds = rect(0, 0, 100, 100);
        let window = rect(2, 20, 96, 78);
        let borders = rect(2, 20, u32::MAX - 3, u32::MAX - 21);
        assert_eq!(
            border_hit(
                bounds,
                window,
                borders,
                1,
                30,
                ClickDestination::Border,
                true
            ),
            Some((BORDER_LEFT, false))
        );
        assert_eq!(
            border_hit(
                bounds,
                window,
                borders,
                2,
                1,
                ClickDestination::Border,
                true
            ),
            Some((BORDER_TOP, false))
        );
        assert_eq!(
            border_hit(
                bounds,
                window,
                borders,
                -1,
                -1,
                ClickDestination::Decoration,
                true
            ),
            Some((BORDER_TOP, true))
        );
        assert_eq!(
            border_hit(
                bounds,
                window,
                borders,
                99,
                50,
                ClickDestination::Inside,
                true
            ),
            Some((BORDER_RIGHT, false))
        );
    }
    #[test]
    fn modifier_zero_high_bits_and_binding_destination_flags() {
        assert!(!modifier_pressed(0xffff, 0x10000));
        assert!(modifier_pressed(5, 0x10005));
        assert!(!modifier_pressed(4, 5));
        assert!(!binding_destination(
            ClickDestination::Decoration,
            true,
            true,
            true
        ));
        assert!(binding_destination(
            ClickDestination::Border,
            true,
            false,
            true
        ));
        assert!(!binding_destination(
            ClickDestination::Inside,
            false,
            false,
            true
        ));
    }
}
