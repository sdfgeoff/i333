use crate::bindings::get_binding_from_xcb_event;
use crate::bindings::run_binding;
use crate::commands_parser::command_result_free;
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
// Mechanical file-by-file Rust translation of src/click.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type i3String = _i3String;

pub type i3Font = Font;

pub use crate::application::Binding;
pub type i3_event_state_mask_t = u32;

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

pub type border_t = ::core::ffi::c_uint;
pub const BORDER_BOTTOM: border_t = 8;
pub const BORDER_TOP: border_t = 4;
pub const BORDER_RIGHT: border_t = 2;
pub const BORDER_LEFT: border_t = 1;
pub use crate::assignments::CommandResult;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 83] = unsafe {
    ::core::mem::transmute::<[u8; 83], [::core::ffi::c_char; 83]>(
        *b"_Bool tiling_resize_for_border(Con *, border_t, xcb_button_press_event_t *, _Bool)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

unsafe fn tiling_resize_for_border(
    con: *mut Con,
    border: border_t,
    event: *mut xcb_button_press_event_t,
    use_threshold: bool,
) -> bool {
    debuglog(
        c"%s:%s:%d - border = %d, con = %p\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize_for_border".as_ptr(),
        25 as ::core::ffi::c_int,
        border as ::core::ffi::c_uint,
        con,
    );
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut first: *mut Con = con;
    let mut search_direction: Direction = Direction::Left;
    match border as ::core::ffi::c_uint {
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
                border as ::core::ffi::c_uint,
            );
            return false_0 != 0;
        }
    }
    let res: bool = resize_find_tiling_participants(
        &raw mut first,
        &raw mut second,
        search_direction,
        false_0 != 0,
    );
    if !res {
        debuglog(
            c"%s:%s:%d - No second container in this direction found.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"tiling_resize_for_border".as_ptr(),
            49 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if (*first).fullscreen_mode as ::core::ffi::c_uint
        != (*second).fullscreen_mode as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Avoiding resize between containers with different fullscreen modes, %d != %d\n".as_ptr().cast_mut(),
            c"../src/click.c".as_ptr(),
            c"tiling_resize_for_border".as_ptr(),
            53 as ::core::ffi::c_int,
            (*first).fullscreen_mode as ::core::ffi::c_uint,
            (*second).fullscreen_mode as ::core::ffi::c_uint,
        );
        return false_0 != 0;
    }
    if first != second {
    } else {
        __assert_fail(
            c"first != second".as_ptr(),
            c"../src/click.c".as_ptr(),
            57 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if (*first).parent == (*second).parent {
    } else {
        __assert_fail(
            c"first->parent == second->parent".as_ptr(),
            c"../src/click.c".as_ptr(),
            58 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if search_direction == Direction::Up || search_direction == Direction::Left {
        let tmp: *mut Con = first;
        first = second;
        second = tmp;
    }
    let orientation: Orientation = if border as ::core::ffi::c_uint
        == BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
        || border as ::core::ffi::c_uint
            == BORDER_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
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
        71 as ::core::ffi::c_int,
    );
    tree_render();
    true_0 != 0
}
unsafe fn floating_mod_on_tiled_client(
    con: *mut Con,
    event: *mut xcb_button_press_event_t,
) -> bool {
    let to_right: ::core::ffi::c_int =
        (*con).rect.width.wrapping_sub((*event).event_x as u32) as ::core::ffi::c_int;
    let to_left: ::core::ffi::c_int = (*event).event_x as ::core::ffi::c_int;
    let to_top: ::core::ffi::c_int = (*event).event_y as ::core::ffi::c_int;
    let to_bottom: ::core::ffi::c_int =
        (*con).rect.height.wrapping_sub((*event).event_y as u32) as ::core::ffi::c_int;
    debuglog(
        c"%s:%s:%d - click was %d px to the right, %d px to the left, %d px to top, %d px to bottom\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"floating_mod_on_tiled_client".as_ptr(),
        94 as ::core::ffi::c_int,
        to_right,
        to_left,
        to_top,
        to_bottom,
    );
    if to_right < to_left && to_right < to_top && to_right < to_bottom {
        return tiling_resize_for_border(con, BORDER_RIGHT, event, false_0 != 0);
    }
    if to_left < to_right && to_left < to_top && to_left < to_bottom {
        return tiling_resize_for_border(con, BORDER_LEFT, event, false_0 != 0);
    }
    if to_top < to_right && to_top < to_left && to_top < to_bottom {
        return tiling_resize_for_border(con, BORDER_TOP, event, false_0 != 0);
    }
    if to_bottom < to_right && to_bottom < to_left && to_bottom < to_top {
        return tiling_resize_for_border(con, BORDER_BOTTOM, event, false_0 != 0);
    }
    false_0 != 0
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
        131 as ::core::ffi::c_int,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
        con,
        (*event).event,
    );
    debuglog(
        c"%s:%s:%d - checks for right >= %d\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"tiling_resize".as_ptr(),
        132 as ::core::ffi::c_int,
        (*con).window_rect.x.wrapping_add((*con).window_rect.width),
    );
    if dest == ClickDestination::Decoration {
        return tiling_resize_for_border(con, BORDER_TOP, event, use_threshold);
    } else if dest == ClickDestination::Border {
        if (*event).event_y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && (*event).event_y as i32 <= bsr.y as i32
            && (*event).event_x as i32 >= bsr.x as i32
            && (*event).event_x as i32 <= (*con).rect.width.wrapping_add(bsr.width) as i32
        {
            return tiling_resize_for_border(con, BORDER_TOP, event, false_0 != 0);
        }
    }
    if (*event).event_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && (*event).event_x as i32 <= bsr.x as i32
        && (*event).event_y as i32 >= bsr.y as i32
        && (*event).event_y as i32 <= (*con).rect.height.wrapping_add(bsr.height) as i32
    {
        return tiling_resize_for_border(con, BORDER_LEFT, event, false_0 != 0);
    }
    if (*event).event_x as i32 >= (*con).window_rect.x.wrapping_add((*con).window_rect.width) as i32
        && (*event).event_y as i32 >= bsr.y as i32
        && (*event).event_y as i32 <= (*con).rect.height.wrapping_add(bsr.height) as i32
    {
        return tiling_resize_for_border(con, BORDER_RIGHT, event, false_0 != 0);
    }
    if (*event).event_y as i32
        >= (*con).window_rect.y.wrapping_add((*con).window_rect.height) as i32
    {
        return tiling_resize_for_border(con, BORDER_BOTTOM, event, false_0 != 0);
    }
    false_0 != 0
}
unsafe fn allow_replay_pointer(time: xcb_timestamp_t) {
    xcb_allow_events(
        conn,
        XCB_ALLOW_REPLAY_POINTER as ::core::ffi::c_int as u8,
        time,
    );
    xcb_flush(conn);
    tree_render();
}
unsafe fn route_click(con: *mut Con, event: *mut xcb_button_press_event_t, dest: ClickDestination) {
    let mod_0: u32 = config.floating_modifier & 0xffff as u32;
    let mod_pressed: bool = mod_0 != 0 as u32 && (*event).state as u32 & mod_0 == mod_0;
    debuglog(
        c"%s:%s:%d - --> click properties: mod = %d, destination = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        173 as ::core::ffi::c_int,
        mod_pressed as ::core::ffi::c_int,
        dest as ::core::ffi::c_uint,
    );
    debuglog(
        c"%s:%s:%d - --> OUTCOME = %p\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        174 as ::core::ffi::c_int,
        con,
    );
    debuglog(
        c"%s:%s:%d - type = %d, name = %s\n".as_ptr().cast_mut(),
        c"../src/click.c".as_ptr(),
        c"route_click".as_ptr(),
        175 as ::core::ffi::c_int,
        (*con).type_0 as ::core::ffi::c_uint,
        (*con).name,
    );
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        allow_replay_pointer((*event).time);
        return;
    }
    let bind: *mut Binding = get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
    if !bind.is_null()
        && (dest == ClickDestination::Decoration && !(*bind).exclude_titlebar
            || dest == ClickDestination::Inside && (*bind).whole_window as ::core::ffi::c_int != 0
            || dest == ClickDestination::Border && (*bind).border as ::core::ffi::c_int != 0)
    {
        let result: *mut CommandResult = run_binding(bind, con);
        xcb_allow_events(
            conn,
            XCB_ALLOW_ASYNC_POINTER as ::core::ffi::c_int as u8,
            (*event).time,
        );
        xcb_flush(conn);
        command_result_free(result);
        return;
    }
    if (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_RELEASE {
        allow_replay_pointer((*event).time);
        return;
    }
    let mut ws: *mut Con = con_get_workspace(con);
    let focused_workspace: *mut Con = con_get_workspace(focused);
    if ws.is_null() {
        ws = (*output_get_content(con_get_output(con)))
            .focus_head
            .tqh_first as *mut Con;
        if ws.is_null() {
            allow_replay_pointer((*event).time);
            return;
        }
    }
    let floatingcon: *mut Con = con_inside_floating(con);
    let proportional: bool = (*event).state as ::core::ffi::c_int
        & XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int
        == XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int;
    let in_stacked: bool =
        (*(*con).parent).layout == Layout::Stacked || (*(*con).parent).layout == Layout::Tabbed;
    let was_focused: bool = focused == con;
    let is_left_click: bool =
        (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_1 as ::core::ffi::c_int;
    let is_right_click: bool =
        (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_3 as ::core::ffi::c_int;
    let is_left_or_right_click: bool =
        is_left_click as ::core::ffi::c_int != 0 || is_right_click as ::core::ffi::c_int != 0;
    let is_scroll: bool = (*event).detail as ::core::ffi::c_int
        == XCB_BUTTON_INDEX_4 as ::core::ffi::c_int
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_INDEX_5 as ::core::ffi::c_int
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_LEFT
        || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_RIGHT;
    if in_stacked as ::core::ffi::c_int != 0
        && dest == ClickDestination::Decoration
        && is_scroll as ::core::ffi::c_int != 0
    {
        debuglog(
            c"%s:%s:%d - Scrolling on a window decoration\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"route_click".as_ptr(),
            234 as ::core::ffi::c_int,
        );
        workspace_show(ws);
        let current: *mut Con = (*(*con).parent).focus_head.tqh_first as *mut Con;
        let direction: Position = if (*event).detail as ::core::ffi::c_int
            == XCB_BUTTON_INDEX_4 as ::core::ffi::c_int
            || (*event).detail as ::core::ffi::c_int == XCB_BUTTON_SCROLL_LEFT
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
    if mod_pressed as ::core::ffi::c_int != 0
        && is_left_click as ::core::ffi::c_int != 0
        && floatingcon.is_null()
        && (config.tiling_drag as ::core::ffi::c_uint
            == TILING_DRAG_MODIFIER as ::core::ffi::c_int as ::core::ffi::c_uint
            || config.tiling_drag as ::core::ffi::c_uint
                == TILING_DRAG_MODIFIER_OR_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint)
        && has_drop_targets() as ::core::ffi::c_int != 0
    {
        let use_threshold: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold);
        allow_replay_pointer((*event).time);
        return;
    }
    let mut con_to_focus: *mut Con = con;
    if in_stacked as ::core::ffi::c_int != 0 && dest == ClickDestination::Decoration {
        if was_focused as ::core::ffi::c_int != 0 || !con_has_parent(focused, con) {
            while !(*con_to_focus).focus_head.tqh_first.is_null() {
                con_to_focus = (*con_to_focus).focus_head.tqh_first as *mut Con;
            }
        }
    }
    if ws != focused_workspace {
        workspace_show(ws);
    }
    con_activate(con_to_focus);
    let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !floatingcon.is_null() && fs != con {
        if mod_pressed as ::core::ffi::c_int != 0 && is_left_click as ::core::ffi::c_int != 0 {
            floating_drag_window(floatingcon, event, false_0 != 0);
            return;
        }
        if mod_pressed as ::core::ffi::c_int != 0 && is_right_click as ::core::ffi::c_int != 0 {
            debuglog(
                c"%s:%s:%d - floating resize due to floatingmodifier\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                293 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if (dest == ClickDestination::Border || dest == ClickDestination::Decoration)
            && is_left_or_right_click as ::core::ffi::c_int != 0
        {
            debuglog(
                c"%s:%s:%d - tiling resize with fallback\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                301 as ::core::ffi::c_int,
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
        if dest == ClickDestination::Decoration && is_right_click as ::core::ffi::c_int != 0 {
            debuglog(
                c"%s:%s:%d - floating resize due to decoration right click\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                309 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest == ClickDestination::Border && is_left_or_right_click as ::core::ffi::c_int != 0 {
            debuglog(
                c"%s:%s:%d - floating resize due to border click\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/click.c".as_ptr(),
                c"route_click".as_ptr(),
                315 as ::core::ffi::c_int,
            );
            floating_resize_window(floatingcon, proportional, event);
            return;
        }
        if dest == ClickDestination::Decoration && is_left_click as ::core::ffi::c_int != 0 {
            floating_drag_window(floatingcon, event, !was_focused);
            return;
        }
        allow_replay_pointer((*event).time);
        return;
    }
    if is_left_click as ::core::ffi::c_int != 0
        && (config.tiling_drag as ::core::ffi::c_uint
            == TILING_DRAG_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint
            && dest == ClickDestination::Decoration
            || config.tiling_drag as ::core::ffi::c_uint
                == TILING_DRAG_MODIFIER_OR_TITLEBAR as ::core::ffi::c_int as ::core::ffi::c_uint
                && (mod_pressed as ::core::ffi::c_int != 0 || dest == ClickDestination::Decoration))
        && has_drop_targets() as ::core::ffi::c_int != 0
    {
        allow_replay_pointer((*event).time);
        let use_threshold_0: bool = !mod_pressed;
        tiling_drag(con, event, use_threshold_0);
        return;
    }
    if dest == ClickDestination::Inside
        && mod_pressed as ::core::ffi::c_int != 0
        && is_right_click as ::core::ffi::c_int != 0
    {
        if floating_mod_on_tiled_client(con, event) {
            return;
        }
        xcb_allow_events(
            conn,
            XCB_ALLOW_ASYNC_POINTER as ::core::ffi::c_int as u8,
            (*event).time,
        );
        xcb_flush(conn);
        return;
    }
    if (dest == ClickDestination::Border || dest == ClickDestination::Decoration)
        && is_left_or_right_click as ::core::ffi::c_int != 0
    {
        debuglog(
            c"%s:%s:%d - Trying to resize (tiling)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/click.c".as_ptr(),
            c"route_click".as_ptr(),
            357 as ::core::ffi::c_int,
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
        377 as ::core::ffi::c_int,
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
        if (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS {
            c"press".as_ptr()
        } else {
            c"release".as_ptr()
        },
        (*event).event,
        (*event).child,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
        (*event).root_x as ::core::ffi::c_int,
        (*event).root_y as ::core::ffi::c_int,
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
            if !bind.is_null() && (*bind).whole_window as ::core::ffi::c_int != 0 {
                let result: *mut CommandResult = run_binding(bind, ::core::ptr::null_mut::<Con>());
                command_result_free(result);
            }
        }
        if (*event).event == root
            && (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS
        {
            let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
            output = (*croot).nodes_head.tqh_first as *mut Con;
            while !output.is_null() {
                if con_is_internal(output) as ::core::ffi::c_int != 0
                    || !rect_contains(
                        (*output).rect as Rect,
                        (*event).event_x as u32,
                        (*event).event_y as u32,
                    )
                {
                    output = (*output).nodes.tqe_next as *mut Con;
                } else {
                    let ws: *mut Con =
                        (*output_get_content(output)).focus_head.tqh_first as *mut Con;
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
        xcb_allow_events(
            conn,
            XCB_ALLOW_REPLAY_POINTER as ::core::ffi::c_int as u8,
            (*event).time,
        );
        xcb_flush(conn);
        return;
    }
    if !(*con).window.is_null() {
        if rect_contains(
            (*con).deco_rect as Rect,
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
                (*child).deco_rect as Rect,
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
            443 as ::core::ffi::c_int,
        );
        route_click(con, event, ClickDestination::Inside);
        return;
    }
    route_click(con, event, ClickDestination::Border);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tiling_drag::has_drop_targets;
use crate::tree::tree_render;

// Canonical Cairo backend ABI; renderer logic remains local.

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
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

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
