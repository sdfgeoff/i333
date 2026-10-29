use crate::tree::all_cons;
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
// Mechanical file-by-file Rust translation of src/tiling_drag.c.
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

pub type Output = xoutput;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_params {
    pub indicator: *mut xcb_window_t,
    pub target: *mut *mut Con,
    pub direction: *mut Direction,
    pub drop_type: *mut DropType,
}

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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe fn is_tiling_drop_target(con: *mut Con) -> bool {
    if !con_has_managed_window(con)
        || con_is_floating(con) as ::core::ffi::c_int != 0
        || con_is_hidden(con) as ::core::ffi::c_int != 0
    {
        return false_0 != 0;
    }
    let ws: *mut Con = con_get_workspace(con);
    if con_is_internal(ws) {
        return false_0 != 0;
    }
    if !workspace_is_visible(ws) {
        return false_0 != 0;
    }
    let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
    if !fs.is_null() && fs != con {
        return false_0 != 0;
    }
    true_0 != 0
}

pub unsafe fn has_drop_targets() -> bool {
    let mut drop_targets: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if is_tiling_drop_target(con) {
            drop_targets += 1;
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).focus_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let mut visible_ws: *mut Con = ::core::ptr::null_mut::<Con>();
            let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
            while child == -(1 as ::core::ffi::c_int) as *mut Con && {
                child = ::core::ptr::null_mut::<Con>();
                true_0 != 0
            } {
                child = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
                while !child.is_null() {
                    if !workspace_is_visible(child) {
                        child = (*child).nodes.tqe_next as *mut Con;
                    } else {
                        visible_ws = child;
                        break;
                    }
                }
            }
            if !visible_ws.is_null() && con_num_children(visible_ws) == 0 as ::core::ffi::c_int {
                drop_targets += 1;
            }
        }
        output = (*output).focused.tqe_next as *mut Con;
    }
    drop_targets > 1 as ::core::ffi::c_int
}
unsafe fn find_drop_target(x: u32, y: u32) -> *mut Con {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        let rect: Rect = (*con).rect as Rect;
        if !rect_contains(rect, x, y) || !is_tiling_drop_target(con) {
            con = (*con).all_cons.tqe_next as *mut Con;
        } else {
            let ws: *mut Con = con_get_workspace(con);
            let fs: *mut Con = con_get_fullscreen_covering_ws(ws);
            return if !fs.is_null() { fs } else { con };
        }
    }
    let output: *mut Output =
        get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if output.is_null() {
        return ::core::ptr::null_mut::<Con>();
    }
    let content: *mut Con = output_get_content((*output).con);
    con_descend_tiling_focused(content)
}
unsafe fn adjust_rect(mut rect: Rect, direction: Direction, threshold: u32) -> Rect {
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
    let mut min_rect_dimension: u32 = 0;
    let mut sibling_indicator_size: u32 = 0;
    let mut parent_indicator_size: u32 = 0;
    let mut d_left: u32 = 0;
    let mut d_top: u32 = 0;
    let mut d_right: u32 = 0;
    let mut d_bottom: u32 = 0;
    let mut d_min: u32 = 0;
    let mut target_parent: bool = false;
    let mut target_sibling: bool = false;
    let sibling_indicator_percent_of_rect: ::core::ffi::c_double = 0.3f64;
    let parent_indicator_max_size: ::core::ffi::c_double =
        (render_deco_height() + logical_px(5 as ::core::ffi::c_int)) as ::core::ffi::c_double;
    let mut target: *mut Con = find_drop_target(new_x, new_y);
    if target.is_null() {
        return;
    }
    let mut rect: Rect = (*target).rect as Rect;
    let mut direction: Direction = Direction::Left;
    let mut drop_type: DropType = DropType::Center;
    let mut draw_window: bool = true_0 != 0;
    let params: *const callback_params = extra as *const callback_params;
    if !((*target).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        min_rect_dimension = min(
            rect.width as ::core::ffi::c_int,
            rect.height as ::core::ffi::c_int,
        ) as u32;
        sibling_indicator_size = max(
            logical_px(2 as ::core::ffi::c_int),
            (sibling_indicator_percent_of_rect * min_rect_dimension as ::core::ffi::c_double) as u32
                as ::core::ffi::c_int,
        ) as u32;
        parent_indicator_size = min(
            parent_indicator_max_size as ::core::ffi::c_int,
            sibling_indicator_size.wrapping_sub(1 as u32) as ::core::ffi::c_int,
        ) as u32;
        d_left = new_x.wrapping_sub(rect.x);
        d_top = new_y.wrapping_sub(rect.y);
        d_right = rect.x.wrapping_add(rect.width).wrapping_sub(new_x);
        d_bottom = rect.y.wrapping_add(rect.height).wrapping_sub(new_y);
        d_min = min(
            min(d_left as ::core::ffi::c_int, d_right as ::core::ffi::c_int),
            min(d_top as ::core::ffi::c_int, d_bottom as ::core::ffi::c_int),
        ) as u32;
        if d_left == d_min {
            direction = Direction::Left;
        } else if d_top == d_min {
            direction = Direction::Up;
        } else if d_right == d_min {
            direction = Direction::Right;
        } else if d_bottom == d_min {
            direction = Direction::Down;
        } else {
            errorlog(c"ERROR: min() is broken\n".as_ptr().cast_mut());
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/tiling_drag.c".as_ptr(),
                    212 as ::core::ffi::c_uint,
                    c"void drag_callback(Con *, Rect *, uint32_t, uint32_t, const xcb_button_press_event_t *, const void *)".as_ptr(),
                );
            };
        }
        target_parent = d_min < parent_indicator_size
            && con_on_side_of_parent(target, direction) as ::core::ffi::c_int != 0;
        target_sibling = d_min < sibling_indicator_size;
        drop_type = if target_parent as ::core::ffi::c_int != 0 {
            DropType::Parent
        } else {
            if target_sibling as ::core::ffi::c_int != 0 {
                DropType::Sibling
            } else {
                DropType::Center
            }
        };
        if drop_type != DropType::Parent && target == con {
            draw_window = false_0 != 0;
            xcb_destroy_window(conn, *(*params).indicator);
            *(*params).indicator = 0 as xcb_window_t;
        } else {
            match drop_type {
                DropType::Parent => {
                    while (*(*target).parent).type_0 as ::core::ffi::c_uint
                        != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && con_on_side_of_parent((*target).parent as *mut Con, direction)
                            as ::core::ffi::c_int
                            != 0
                    {
                        target = (*target).parent as *mut Con;
                    }
                    rect = adjust_rect(
                        (*(*target).parent).rect as Rect,
                        direction,
                        parent_indicator_size,
                    );
                }
                DropType::Center => {
                    rect = (*target).rect as Rect;
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
                    rect = adjust_rect((*target).rect as Rect, direction, sibling_indicator_size);
                }
            }
        }
    }
    if draw_window {
        if *(*params).indicator == 0 as xcb_window_t {
            *(*params).indicator = create_drop_indicator(rect);
        } else {
            let values: [u32; 4] = [rect.x, rect.y, rect.width, rect.height];
            let mask: u32 = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int)
                as u32;
            xcb_configure_window(
                conn,
                *(*params).indicator,
                mask as u16,
                &raw const values as *const u32 as *const ::core::ffi::c_void,
            );
        }
    }
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as u32);
    xcb_flush(conn);
    *(*params).target = target;
    *(*params).direction = direction;
    *(*params).drop_type = drop_type;
}
unsafe fn create_drop_indicator(rect: Rect) -> xcb_window_t {
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 2] = [0; 2];
    mask = XCB_CW_BACK_PIXEL as ::core::ffi::c_int as u32;
    values[0 as ::core::ffi::c_int as usize] = config.client.focused.indicator.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
    values[1 as ::core::ffi::c_int as usize] = 1 as u32;
    let indicator: xcb_window_t = create_window(
        conn,
        rect,
        XCB_COPY_FROM_PARENT as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        Cursor::Move,
        false_0 != 0,
        mask,
        &raw mut values as *mut u32,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        indicator,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as u8,
        strlen(c"i3-drag".as_ptr())
            .wrapping_add(1 as usize)
            .wrapping_mul(2 as usize) as u32,
        b"i3-drag\0i3-drag\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, indicator);
    xcb_circulate_window(
        conn,
        XCB_CIRCULATE_RAISE_LOWEST as ::core::ffi::c_int as u8,
        indicator,
    );
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
        305 as ::core::ffi::c_int,
        con,
    );
    let mut set_focus: bool = con == focused;
    let mut set_fs: bool = (*con).fullscreen_mode as ::core::ffi::c_uint
        != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint;
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as u32);
    xcb_flush(conn);
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut direction: Direction = Direction::Left;
    let mut drop_type: DropType = DropType::Sibling;
    let mut indicator: xcb_window_t = 0 as xcb_window_t;
    let params: callback_params = callback_params {
        indicator: &raw mut indicator,
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
    xcb_destroy_window(conn, indicator);
    if drag_result == DragResult::Revert
        || target.is_null()
        || target == con && drop_type != DropType::Parent
        || !con_exists(target)
    {
        debuglog(
            c"%s:%s:%d - drop aborted\n".as_ptr().cast_mut(),
            c"../src/tiling_drag.c".as_ptr(),
            c"tiling_drag".as_ptr(),
            329 as ::core::ffi::c_int,
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
                340 as ::core::ffi::c_int,
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
                352 as ::core::ffi::c_int,
                position_to_string(position),
                target,
            );
            if con_orientation((*target).parent as *mut Con) != orientation {
                if (*con).parent == (*target).parent
                    && con_num_children((*target).parent as *mut Con) == 2 as ::core::ffi::c_int
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
                375 as ::core::ffi::c_int,
                direction_to_string(direction),
                position_to_string(position),
                if parent_tabbed_or_stacked as ::core::ffi::c_int != 0 {
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
            if focused != old_focus && con_exists(old_focus) as ::core::ffi::c_int != 0 {
                con_activate(old_focus);
            }
        }
    }
    target = ::core::ptr::null_mut::<Con>();
    if set_focus as ::core::ffi::c_int != 0 || set_fs as ::core::ffi::c_int != 0 {
        let fs: *mut Con = con_get_fullscreen_covering_ws(con_get_workspace(con));
        if fs == con {
            errorlog(
                c"ERROR: dragged container somehow got fullscreen again.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/tiling_drag.c".as_ptr(),
                    406 as ::core::ffi::c_uint,
                    c"void tiling_drag(Con *, xcb_button_press_event_t *, _Bool)".as_ptr(),
                );
            };
        } else if !fs.is_null()
            && set_focus as ::core::ffi::c_int != 0
            && set_fs as ::core::ffi::c_int != 0
        {
            con_disable_fullscreen(fs);
        } else if !fs.is_null() {
            set_focus = false_0 != 0;
            set_fs = set_focus;
        } else if !set_focus {
            set_focus = set_fs as ::core::ffi::c_int != 0
                && con_get_workspace(focused) == con_get_workspace(con);
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

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_button_press_event_t, xcb_change_property, xcb_circulate_window,
    xcb_configure_window, xcb_destroy_window, xcb_flush, xcb_map_window, xcb_visualid_t,
    xcb_window_t, XCB_ATOM_STRING, XCB_ATOM_WM_CLASS, XCB_CIRCULATE_RAISE_LOWEST,
    XCB_CONFIG_WINDOW_HEIGHT, XCB_CONFIG_WINDOW_WIDTH, XCB_CONFIG_WINDOW_X, XCB_CONFIG_WINDOW_Y,
    XCB_COPY_FROM_PARENT, XCB_CW_BACK_PIXEL, XCB_CW_OVERRIDE_REDIRECT, XCB_EVENT_MASK_ENTER_WINDOW,
    XCB_NONE, XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::strlen;

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

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
