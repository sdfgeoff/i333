use crate::tree::all_cons;
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
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_3 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_3 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_3 = 0;

pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_8 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_8 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_8 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_8 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_10 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_10 = 1;
pub const M_HERE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_11 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_11 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_11 = 4;
pub const WM_TILING: C2RustUnnamed_11 = 3;
pub const WM_TILING_USER: C2RustUnnamed_11 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_11 = 1;
pub const WM_ANY: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_12 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_12 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_12 = 1;
pub const M_NODOCK: C2RustUnnamed_12 = 0;
pub const M_DONTCHECK: C2RustUnnamed_12 = -1;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_13 = 1;
pub const U_LATEST: C2RustUnnamed_13 = 0;
pub const U_DONTCHECK: C2RustUnnamed_13 = -1;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_14 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_14 = 1;
pub const W_NODOCK: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_17 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_17 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_17 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_17 = 2;
pub const A_COMMAND: C2RustUnnamed_17 = 1;
pub const A_ANY: C2RustUnnamed_17 = 0;

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_19 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_19 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_19 = 3;
pub const CT_CON: C2RustUnnamed_19 = 2;
pub const CT_OUTPUT: C2RustUnnamed_19 = 1;
pub const CT_ROOT: C2RustUnnamed_19 = 0;

pub type Output = xoutput;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
pub type orientation_t = ::core::ffi::c_uint;
pub const VERT: orientation_t = 2;
pub const HORIZ: orientation_t = 1;
pub const NO_ORIENTATION: orientation_t = 0;
pub type position_t = ::core::ffi::c_uint;
pub const AFTER: position_t = 1;
pub const BEFORE: position_t = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;
pub const DT_PARENT: drop_type_t = 2;
pub const DT_SIBLING: drop_type_t = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_21 = 3;
pub const PDF_IGNORE: C2RustUnnamed_21 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_21 = 1;
pub const PDF_SMART: C2RustUnnamed_21 = 0;

pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_22 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_22 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_23 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_23 = 2;
pub const FOWA_URGENT: C2RustUnnamed_23 = 1;
pub const FOWA_SMART: C2RustUnnamed_23 = 0;
pub const DT_CENTER: drop_type_t = 1;
pub type drop_type_t = ::core::ffi::c_uint;
pub const DRAG_REVERT: drag_result_t = 2;
pub type drag_result_t = ::core::ffi::c_uint;
pub const DRAG_ABORT: drag_result_t = 3;
pub const DRAG_SUCCESS: drag_result_t = 1;
pub const DRAGGING: drag_result_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_params {
    pub indicator: *mut xcb_window_t,
    pub target: *mut *mut Con,
    pub direction: *mut direction_t,
    pub drop_type: *mut drop_type_t,
}
pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;
pub type callback_t = Option<
    unsafe fn(
        *mut Con,
        *mut Rect,
        uint32_t,
        uint32_t,
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
unsafe fn find_drop_target(x: uint32_t, y: uint32_t) -> *mut Con {
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
unsafe fn adjust_rect(mut rect: Rect, direction: direction_t, threshold: uint32_t) -> Rect {
    match direction as ::core::ffi::c_uint {
        0 => {
            rect.width = threshold;
        }
        2 => {
            rect.height = threshold;
        }
        1 => {
            rect.x = rect.x.wrapping_add(rect.width.wrapping_sub(threshold));
            rect.width = threshold;
        }
        3 => {
            rect.y = rect.y.wrapping_add(rect.height.wrapping_sub(threshold));
            rect.height = threshold;
        }
        _ => {}
    }
    rect
}
unsafe fn con_on_side_of_parent(con: *mut Con, direction: direction_t) -> bool {
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let mut reverse_direction: direction_t = D_LEFT;
    match direction as ::core::ffi::c_uint {
        0 => {
            reverse_direction = D_RIGHT;
        }
        1 => {
            reverse_direction = D_LEFT;
        }
        2 => {
            reverse_direction = D_DOWN;
        }
        3 => {
            reverse_direction = D_UP;
        }
        _ => {}
    }
    con_orientation((*con).parent as *mut Con) as ::core::ffi::c_uint
        != orientation as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).layout as ::core::ffi::c_uint
            == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        || con_descend_direction((*con).parent as *mut Con, reverse_direction) == con
}
unsafe fn drag_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: uint32_t,
    new_y: uint32_t,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    let mut min_rect_dimension: uint32_t = 0;
    let mut sibling_indicator_size: uint32_t = 0;
    let mut parent_indicator_size: uint32_t = 0;
    let mut d_left: uint32_t = 0;
    let mut d_top: uint32_t = 0;
    let mut d_right: uint32_t = 0;
    let mut d_bottom: uint32_t = 0;
    let mut d_min: uint32_t = 0;
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
    let mut direction: direction_t = D_LEFT;
    let mut drop_type: drop_type_t = DT_CENTER;
    let mut draw_window: bool = true_0 != 0;
    let params: *const callback_params = extra as *const callback_params;
    if !((*target).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        min_rect_dimension = min(
            rect.width as ::core::ffi::c_int,
            rect.height as ::core::ffi::c_int,
        ) as uint32_t;
        sibling_indicator_size = max(
            logical_px(2 as ::core::ffi::c_int),
            (sibling_indicator_percent_of_rect * min_rect_dimension as ::core::ffi::c_double)
                as uint32_t as ::core::ffi::c_int,
        ) as uint32_t;
        parent_indicator_size = min(
            parent_indicator_max_size as ::core::ffi::c_int,
            sibling_indicator_size.wrapping_sub(1 as uint32_t) as ::core::ffi::c_int,
        ) as uint32_t;
        d_left = new_x.wrapping_sub(rect.x);
        d_top = new_y.wrapping_sub(rect.y);
        d_right = rect.x.wrapping_add(rect.width).wrapping_sub(new_x);
        d_bottom = rect.y.wrapping_add(rect.height).wrapping_sub(new_y);
        d_min = min(
            min(d_left as ::core::ffi::c_int, d_right as ::core::ffi::c_int),
            min(d_top as ::core::ffi::c_int, d_bottom as ::core::ffi::c_int),
        ) as uint32_t;
        if d_left == d_min {
            direction = D_LEFT;
        } else if d_top == d_min {
            direction = D_UP;
        } else if d_right == d_min {
            direction = D_RIGHT;
        } else if d_bottom == d_min {
            direction = D_DOWN;
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
        drop_type = (if target_parent as ::core::ffi::c_int != 0 {
            DT_PARENT as ::core::ffi::c_int
        } else if target_sibling as ::core::ffi::c_int != 0 {
            DT_SIBLING as ::core::ffi::c_int
        } else {
            DT_CENTER as ::core::ffi::c_int
        }) as drop_type_t;
        if drop_type as ::core::ffi::c_uint
            != DT_PARENT as ::core::ffi::c_int as ::core::ffi::c_uint
            && target == con
        {
            draw_window = false_0 != 0;
            xcb_destroy_window(conn, *(*params).indicator);
            *(*params).indicator = 0 as xcb_window_t;
        } else {
            match drop_type as ::core::ffi::c_uint {
                2 => {
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
                1 => {
                    rect = (*target).rect as Rect;
                    rect.x = rect.x.wrapping_add(sibling_indicator_size);
                    rect.y = rect.y.wrapping_add(sibling_indicator_size);
                    rect.width = rect
                        .width
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as uint32_t));
                    rect.height = rect
                        .height
                        .wrapping_sub(sibling_indicator_size.wrapping_mul(2 as uint32_t));
                }
                0 => {
                    rect = adjust_rect((*target).rect as Rect, direction, sibling_indicator_size);
                }
                _ => {}
            }
        }
    }
    if draw_window {
        if *(*params).indicator == 0 as xcb_window_t {
            *(*params).indicator = create_drop_indicator(rect);
        } else {
            let values: [uint32_t; 4] = [rect.x, rect.y, rect.width, rect.height];
            let mask: uint32_t = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int)
                as uint32_t;
            xcb_configure_window(
                conn,
                *(*params).indicator,
                mask as uint16_t,
                &raw const values as *const uint32_t as *const ::core::ffi::c_void,
            );
        }
    }
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t);
    xcb_flush(conn);
    *(*params).target = target;
    *(*params).direction = direction;
    *(*params).drop_type = drop_type;
}
unsafe fn create_drop_indicator(rect: Rect) -> xcb_window_t {
    let mut mask: uint32_t = 0 as uint32_t;
    let mut values: [uint32_t; 2] = [0; 2];
    mask = XCB_CW_BACK_PIXEL as ::core::ffi::c_int as uint32_t;
    values[0 as ::core::ffi::c_int as usize] = config.client.focused.indicator.colorpixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t;
    values[1 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    let indicator: xcb_window_t = create_window(
        conn,
        rect,
        XCB_COPY_FROM_PARENT as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
        XCURSOR_CURSOR_MOVE,
        false_0 != 0,
        mask,
        &raw mut values as *mut uint32_t,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        indicator,
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
        XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
        8 as uint8_t,
        strlen(c"i3-drag".as_ptr())
            .wrapping_add(1 as size_t)
            .wrapping_mul(2 as size_t) as uint32_t,
        b"i3-drag\0i3-drag\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, indicator);
    xcb_circulate_window(
        conn,
        XCB_CIRCULATE_RAISE_LOWEST as ::core::ffi::c_int as uint8_t,
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
    x_mask_event_mask(!(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int) as uint32_t);
    xcb_flush(conn);
    let mut target: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut direction: direction_t = D_LEFT;
    let mut drop_type: drop_type_t = DT_SIBLING;
    let mut indicator: xcb_window_t = 0 as xcb_window_t;
    let params: callback_params = callback_params {
        indicator: &raw mut indicator,
        target: &raw mut target,
        direction: &raw mut direction,
        drop_type: &raw mut drop_type,
    };
    let drag_result: drag_result_t = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        XCURSOR_CURSOR_MOVE as ::core::ffi::c_int,
        use_threshold,
        Some(
            drag_callback
                as unsafe fn(
                    *mut Con,
                    *mut Rect,
                    uint32_t,
                    uint32_t,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw const params as *const ::core::ffi::c_void,
    );
    xcb_destroy_window(conn, indicator);
    if drag_result as ::core::ffi::c_uint
        == DRAG_REVERT as ::core::ffi::c_int as ::core::ffi::c_uint
        || target.is_null()
        || target == con
            && drop_type as ::core::ffi::c_uint
                != DT_PARENT as ::core::ffi::c_int as ::core::ffi::c_uint
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
    let orientation: orientation_t = orientation_from_direction(direction) as orientation_t;
    let position: position_t = position_from_direction(direction) as position_t;
    let layout: layout_t = (if orientation as ::core::ffi::c_uint
        == VERT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        L_SPLITV as ::core::ffi::c_int
    } else {
        L_SPLITH as ::core::ffi::c_int
    }) as layout_t;
    con_disable_fullscreen(con);
    let mut mod_0: uint32_t = 0;
    let mut swap_pressed: bool = false;
    match drop_type as ::core::ffi::c_uint {
        1 => {
            debuglog(
                c"%s:%s:%d - drop to center of %p\n".as_ptr().cast_mut(),
                c"../src/tiling_drag.c".as_ptr(),
                c"tiling_drag".as_ptr(),
                340 as ::core::ffi::c_int,
                target,
            );
            mod_0 = config.swap_modifier & 0xffff as uint32_t;
            swap_pressed = mod_0 != 0 as uint32_t && (*event).state as uint32_t & mod_0 == mod_0;
            if swap_pressed {
                if !con_swap(con, target) {
                    return;
                }
            } else {
                con_move_to_target(con, target);
            }
        }
        0 => {
            debuglog(
                c"%s:%s:%d - drop %s %p\n".as_ptr().cast_mut(),
                c"../src/tiling_drag.c".as_ptr(),
                c"tiling_drag".as_ptr(),
                352 as ::core::ffi::c_int,
                position_to_string(position),
                target,
            );
            if con_orientation((*target).parent as *mut Con) as ::core::ffi::c_uint
                != orientation as ::core::ffi::c_uint
            {
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
        2 => {
            let parent_tabbed_or_stacked: bool = (*(*target).parent).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*target).parent).layout as ::core::ffi::c_uint
                    == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint;
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
                if position as ::core::ffi::c_uint
                    == BEFORE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
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
        _ => {}
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
