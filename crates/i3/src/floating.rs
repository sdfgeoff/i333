use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{floating, focus, identity};
use crate::con::{CF_NONE, CT_CON, CT_FLOATING_CON, CT_WORKSPACE};
pub use crate::types::xoutput as Output;

const BORDER_LEFT: u32 = 1;
const BORDER_RIGHT: u32 = 2;
const BORDER_TOP: u32 = 4;
const BORDER_BOTTOM: u32 = 8;
use crate::randr::outputs;
use crate::types::DragResult;
use crate::types::FloatingState;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Position;
use crate::types::ScratchpadState;
use crate::xcursor::Cursor;

/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * floating.c: Floating windows.
 *
 */
// Geometry and size constraints consume copied values; native tree/list/display
// operations require the exclusive event thread. No graph borrow crosses a
// focus/move/render callback or the synchronous nested drag loop.

#[derive(Copy, Clone)]
#[repr(C)]
struct ResizeParameters {
    pub corner: u32,
    pub proportional: bool,
}

unsafe fn total_outputs_dimensions() -> Rect {
    if outputs.tqh_first.is_null() {
        return Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: (*root_screen).width_in_pixels as u32,
            height: (*root_screen).height_in_pixels as u32,
        };
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut outputs_dimensions: Rect = Rect {
        x: 0 as u32,
        y: 0 as u32,
        width: 0 as u32,
        height: 0 as u32,
    };
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        outputs_dimensions.height = outputs_dimensions
            .height
            .wrapping_add((*output).rect.height);
        outputs_dimensions.width = outputs_dimensions.width.wrapping_add((*output).rect.width);
        output = (*output).outputs.tqe_next as *mut Output;
    }
    outputs_dimensions
}
unsafe fn floating_set_hint_atom(con: *mut Con, floating: bool) {
    if !con_is_leaf(con) {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            floating_set_hint_atom(child, floating);
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    if (*con).window.is_null() {
        return;
    }
    if floating {
        let mut val: u32 = 1 as u32;
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as i32 as u8,
            (*(*con).window).id,
            A_I3_FLOATING_WINDOW,
            XCB_ATOM_CARDINAL as i32 as xcb_atom_t,
            32 as u8,
            1 as u32,
            &raw mut val as *const ::core::ffi::c_void,
        );
    } else {
        xcb_delete_property(conn, (*(*con).window).id, A_I3_FLOATING_WINDOW);
    }
    xcb_flush(conn);
}

// Safe copied values never borrow Window/Con/config across native operations.
#[derive(Clone, Copy, Default)]
struct SizeHints {
    min_width: i32,
    min_height: i32,
    base_width: i32,
    base_height: i32,
    max_width: i32,
    max_height: i32,
    width_increment: i32,
    height_increment: i32,
    min_aspect_ratio: f64,
    max_aspect_ratio: f64,
}
#[derive(Clone, Copy)]
struct SizeLimits {
    minimum_width: i32,
    minimum_height: i32,
    maximum_width: i32,
    maximum_height: i32,
}
fn constrained_size(
    mut rect: Rect,
    border_rect: Rect,
    hints: Option<SizeHints>,
    limits: SizeLimits,
    output_dimensions: Rect,
    fullscreen: bool,
    prefer_height: bool,
) -> Rect {
    let floating_sane_min_height = 50;
    let floating_sane_min_width = 75;

    if let Some(window) = hints {
        let min_width: i32 = if window.min_width != 0 {
            window.min_width
        } else {
            window.base_width
        };
        let min_height: i32 = if window.min_height != 0 {
            window.min_height
        } else {
            window.base_height
        };
        let base_width: i32 = if window.base_width != 0 {
            window.base_width
        } else {
            window.min_width
        };
        let base_height: i32 = if window.base_height != 0 {
            window.base_height
        } else {
            window.min_height
        };
        if min_width != 0 {
            rect.width = rect.width.wrapping_sub(border_rect.width);
            rect.width = max(rect.width as i32, min_width) as u32;
            rect.width = rect.width.wrapping_add(border_rect.width);
        }
        if min_height != 0 {
            rect.height = rect.height.wrapping_sub(border_rect.height);
            rect.height = max(rect.height as i32, min_height) as u32;
            rect.height = rect.height.wrapping_add(border_rect.height);
        }
        if window.max_width != 0 {
            rect.width = rect.width.wrapping_sub(border_rect.width);
            rect.width = min(rect.width as i32, window.max_width) as u32;
            rect.width = rect.width.wrapping_add(border_rect.width);
        }
        if window.max_height != 0 {
            rect.height = rect.height.wrapping_sub(border_rect.height);
            rect.height = min(rect.height as i32, window.max_height) as u32;
            rect.height = rect.height.wrapping_add(border_rect.height);
        }
        let min_ar: f64 = window.min_aspect_ratio;
        let max_ar: f64 = window.max_aspect_ratio;
        if !fullscreen && (min_ar > 0.0 || max_ar > 0.0) {
            let mut width: f64 = rect
                .width
                .wrapping_sub(window.base_width as u32)
                .wrapping_sub(border_rect.width) as f64;
            let mut height: f64 = rect
                .height
                .wrapping_sub(window.base_height as u32)
                .wrapping_sub(border_rect.height) as f64;
            let ar: f64 = width / height;
            let mut new_ar: f64 = -(1) as f64;
            if min_ar > 0.0 && ar < min_ar {
                new_ar = min_ar;
            } else if max_ar > 0.0 && ar > max_ar {
                new_ar = max_ar;
            }
            if new_ar > 0.0 {
                if prefer_height {
                    width = (height * new_ar).round();
                    height = (width / new_ar).round();
                } else {
                    height = (width / new_ar).round();
                    width = (height * new_ar).round();
                }
                rect.width = (width + window.base_width as f64 + border_rect.width as f64) as u32;
                rect.height =
                    (height + window.base_height as f64 + border_rect.height as f64) as u32;
            }
        }
        if window.height_increment != 0
            && rect.height >= (base_height as u32).wrapping_add(border_rect.height)
        {
            rect.height = rect
                .height
                .wrapping_sub((base_height as u32).wrapping_add(border_rect.height));
            rect.height = rect
                .height
                .wrapping_sub(rect.height.wrapping_rem(window.height_increment as u32));
            rect.height = rect
                .height
                .wrapping_add((base_height as u32).wrapping_add(border_rect.height));
        }
        if window.width_increment != 0
            && rect.width >= (base_width as u32).wrapping_add(border_rect.width)
        {
            rect.width = rect
                .width
                .wrapping_sub((base_width as u32).wrapping_add(border_rect.width));
            rect.width = rect
                .width
                .wrapping_sub(rect.width.wrapping_rem(window.width_increment as u32));
            rect.width = rect
                .width
                .wrapping_add((base_width as u32).wrapping_add(border_rect.width));
        }
    }
    if limits.minimum_height != -(1) {
        rect.height = rect.height.wrapping_sub(border_rect.height);
        if limits.minimum_height == 0 {
            rect.height = max(rect.height as i32, floating_sane_min_height) as u32;
        } else {
            rect.height = max(rect.height as i32, limits.minimum_height as i32) as u32;
        }
        rect.height = rect.height.wrapping_add(border_rect.height);
    }
    if limits.minimum_width != -(1) {
        rect.width = rect.width.wrapping_sub(border_rect.width);
        if limits.minimum_width == 0 {
            rect.width = max(rect.width as i32, floating_sane_min_width) as u32;
        } else {
            rect.width = max(rect.width as i32, limits.minimum_width as i32) as u32;
        }
        rect.width = rect.width.wrapping_add(border_rect.width);
    }
    let floating_sane_max_dimensions = output_dimensions;
    if limits.maximum_height != -(1) {
        rect.height = rect.height.wrapping_sub(border_rect.height);
        if limits.maximum_height == 0 {
            rect.height = min(
                rect.height as i32,
                floating_sane_max_dimensions.height as i32,
            ) as u32;
        } else {
            rect.height = min(rect.height as i32, limits.maximum_height as i32) as u32;
        }
        rect.height = rect.height.wrapping_add(border_rect.height);
    }
    if limits.maximum_width != -(1) {
        rect.width = rect.width.wrapping_sub(border_rect.width);
        if limits.maximum_width == 0 {
            rect.width = min(rect.width as i32, floating_sane_max_dimensions.width as i32) as u32;
        } else {
            rect.width = min(rect.width as i32, limits.maximum_width as i32) as u32;
        }
        rect.width = rect.width.wrapping_add(border_rect.width);
    }
    rect
}

/// Requires a live event-thread-owned floating tree and initialized display/config.
/// Copy all hint/policy inputs before computing; no graph borrow escapes the adapter.
pub unsafe fn floating_check_size(floating_con: *mut Con, prefer_height: bool) {
    let focused_con = con_descend_focused(floating_con);
    let mut border = con_border_style_rect(focused_con);
    border.width = border
        .width
        .wrapping_neg()
        .wrapping_add((2 * (*focused_con).border_width) as u32);
    border.height = border
        .height
        .wrapping_neg()
        .wrapping_add((2 * (*focused_con).border_width) as u32);
    let hints = (*focused_con).window.as_ref().map(|w| SizeHints {
        min_width: w.min_width,
        min_height: w.min_height,
        base_width: w.base_width,
        base_height: w.base_height,
        max_width: w.max_width,
        max_height: w.max_height,
        width_increment: w.width_increment,
        height_increment: w.height_increment,
        min_aspect_ratio: w.min_aspect_ratio,
        max_aspect_ratio: w.max_aspect_ratio,
    });
    let limits = SizeLimits {
        minimum_width: config.floating_minimum_width,
        minimum_height: config.floating_minimum_height,
        maximum_width: config.floating_maximum_width,
        maximum_height: config.floating_maximum_height,
    };
    (*floating_con).rect = constrained_size(
        (*floating_con).rect,
        border,
        hints,
        limits,
        total_outputs_dimensions(),
        (*floating_con).fullscreen_mode != CF_NONE,
        prefer_height,
    );
}

pub unsafe fn floating_enable(con: *mut Con, automatic: bool) -> bool {
    let set_focus: bool = con == focused;
    if con_is_docked(con) {
        verboselog(
            c"Container is a dock window, not enabling floating mode.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false;
    }
    if con_is_floating(con) {
        verboselog(
            c"Container is already in floating mode, not doing anything.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false;
    }
    if (*con).type_0 as u32 == CT_WORKSPACE as i32 as u32 {
        verboselog(
            c"Container is a workspace, not enabling floating mode.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false;
    }
    let mut focus_placeholder = None;
    let mut focus_before_parent: bool = true;
    if !set_focus {
        let mut ancestor: *mut Con = con;
        while (*parent_ptr(ancestor)).type_0 as u32 != CT_WORKSPACE as i32 as u32 {
            focus_before_parent = (focus_before_parent as i32
                & (focus::first_ptr(parent_ptr(ancestor)) == ancestor) as i32)
                != 0;
            ancestor = parent_ptr(ancestor);
        }
        focus_placeholder = if focus_before_parent {
            identity(focus::previous_ptr(ancestor))
        } else {
            identity(focus::next_ptr(ancestor))
        };
    }
    con_detach(con);
    con_fix_percent(parent_ptr(con));
    let nc: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<Window>(),
    );
    let ws: *mut Con = con_get_workspace(con);
    set_parent_ptr(nc, ws as *mut Con).expect("live acyclic ancestry");
    (*nc).type_0 = CT_FLOATING_CON;
    (*nc).layout = Layout::SplitH;
    floating::prepend(
        identity(ws).expect("live floating workspace"),
        identity(nc).expect("live floating wrapper"),
    )
    .expect("new floating wrapper has no existing membership");
    let focus_owner = identity(ws).expect("live workspace focus owner");
    let wrapper_id = identity(nc).expect("live floating wrapper");
    if focus_before_parent {
        if let Some(anchor) = focus_placeholder {
            focus::insert_after(focus_owner, wrapper_id, anchor).expect("saved focus placeholder");
        } else {
            focus::prepend(focus_owner, wrapper_id).expect("new focus wrapper");
        }
    } else if let Some(anchor) = focus_placeholder {
        focus::insert_before(focus_owner, wrapper_id, anchor).expect("saved focus placeholder");
    } else {
        focus::append(focus_owner, wrapper_id).expect("new focus wrapper");
    }
    if ((*parent_ptr(con)).type_0 as u32 == CT_CON as i32 as u32
        || (*parent_ptr(con)).type_0 as u32 == CT_FLOATING_CON as i32 as u32)
        && con_num_children(parent_ptr(con)) == 0
    {
        debuglog(
            c"%s:%s:%d - Old container empty after setting this child to floating, closing\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_enable".as_ptr(),
            316,
        );
        let parent: *mut Con = parent_ptr(con);
        set_parent_ptr(con, ::core::ptr::null_mut::<Con>()).expect("live acyclic ancestry");
        tree_close_internal(parent, KillWindow::DontKill, false);
    }
    let name = std::ffi::CString::new(format!("[i3 con] floatingcon around {con:p}")).unwrap();
    x_set_name(nc, name.as_ptr());
    debuglog(
        c"%s:%s:%d - Original rect: (%d, %d) with %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_enable".as_ptr(),
        328,
        (*con).rect.x,
        (*con).rect.y,
        (*con).rect.width,
        (*con).rect.height,
    );
    debuglog(
        c"%s:%s:%d - Geometry = (%d, %d) with %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_enable".as_ptr(),
        329,
        (*con).geometry.x,
        (*con).geometry.y,
        (*con).geometry.width,
        (*con).geometry.height,
    );
    (*nc).rect = (*con).geometry;
    if rect_equals(
        (*nc).rect,
        Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: 0 as u32,
            height: 0 as u32,
        },
    ) {
        debuglog(
            c"%s:%s:%d - Geometry not set, combining children\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_enable".as_ptr(),
            334,
        );
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            debuglog(
                c"%s:%s:%d - child geometry: %d x %d\n".as_ptr().cast_mut(),
                c"../src/floating.c".as_ptr(),
                c"floating_enable".as_ptr(),
                337,
                (*child).geometry.width,
                (*child).geometry.height,
            );
            (*nc).rect.width = (*nc).rect.width.wrapping_add((*child).geometry.width);
            (*nc).rect.height =
                max((*nc).rect.height as i32, (*child).geometry.height as i32) as u32;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).nodes.tqe_prev = (*nc).nodes_head.tqh_last;
    *(*nc).nodes_head.tqh_last = con as *mut Con;
    (*nc).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
    focus::append(
        identity(nc).expect("live floating wrapper"),
        identity(con).expect("live wrapped child"),
    )
    .expect("detached wrapped focus child");
    set_parent_ptr(con, nc as *mut Con).expect("live acyclic ancestry");
    (*con).percent = 1.0f64;
    (*con).floating = FloatingState::UserOn;
    if automatic {
        (*con).max_user_border_style = config.default_floating_border;
        (*con).border_style = (*con).max_user_border_style;
    }
    let bsr: Rect = con_border_style_rect(con);
    (*nc).rect.height = (*nc).rect.height.wrapping_sub(bsr.height);
    (*nc).rect.width = (*nc).rect.width.wrapping_sub(bsr.width);
    (*nc).rect.height = (*nc)
        .rect
        .height
        .wrapping_add(((*con).border_width * 2) as u32);
    (*nc).rect.width = (*nc)
        .rect
        .width
        .wrapping_add(((*con).border_width * 2) as u32);
    floating_check_size(nc, false);
    if (*nc).rect.x == 0 as u32 && (*nc).rect.y == 0 as u32 {
        let mut leader: *mut Con = ::core::ptr::null_mut::<Con>();
        if !(*con).window.is_null()
            && (*(*con).window).leader as ::core::ffi::c_long != XCB_NONE
            && (*(*con).window).id != (*(*con).window).leader
            && {
                leader = con_by_window_id((*(*con).window).leader);
                !leader.is_null()
            }
        {
            debuglog(
                c"%s:%s:%d - Centering above leader\n".as_ptr().cast_mut(),
                c"../src/floating.c".as_ptr(),
                c"floating_enable".as_ptr(),
                377,
            );
            floating_center(nc, (*leader).rect);
        } else {
            floating_center(nc, (*ws).rect);
        }
    }
    let mut current_output: *mut Output = get_output_from_rect((*nc).rect);
    let correct_output: *mut Con = con_get_output(ws);
    if current_output.is_null() || (*current_output).con != correct_output {
        debuglog(
            c"%s:%s:%d - This floating window is on the wrong output, fixing coordinates (currently (%d, %d))\n".as_ptr().cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_enable".as_ptr(),
            391,
            (*nc).rect.x,
            (*nc).rect.y,
        );
        if !current_output.is_null() {
            floating_fix_coordinates(
                nc,
                &raw mut (*(*current_output).con).rect,
                &raw mut (*correct_output).rect,
            );
            current_output = get_output_from_rect((*nc).rect);
        }
        if current_output.is_null() || (*current_output).con != correct_output {
            floating_center(nc, (*ws).rect);
        }
    }
    debuglog(
        c"%s:%s:%d - Floating rect: (%d, %d) with %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_enable".as_ptr(),
        405,
        (*nc).rect.x,
        (*nc).rect.y,
        (*nc).rect.width,
        (*nc).rect.height,
    );
    render_con(nc);
    if set_focus {
        con_activate(con);
    }
    floating_set_hint_atom(nc, true);
    ipc_send_window_event(c"floating".as_ptr(), con);
    true
}

pub unsafe fn floating_disable(con: *mut Con) {
    if !con_is_floating(con) {
        verboselog(
            c"Container isn't floating, not doing anything.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let ws: *mut Con = con_get_workspace(con);
    if con_is_internal(ws) {
        verboselog(
            c"Can't disable floating for container in internal workspace.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let tiling_focused: *mut Con = con_descend_tiling_focused(ws);
    if (*tiling_focused).type_0 as u32 == CT_WORKSPACE as i32 as u32 {
        let parent: *mut Con = parent_ptr(con);
        con_detach(con);
        set_parent_ptr(con, ::core::ptr::null_mut::<Con>()).expect("live acyclic ancestry");
        tree_close_internal(parent, KillWindow::DontKill, true);
        con_attach(con, tiling_focused, false);
        (*con).percent = 0.0f64;
        con_fix_percent(parent_ptr(con));
    } else {
        insert_con_into(con, tiling_focused, Position::After);
    }
    (*con).floating = FloatingState::UserOff;
    floating_set_hint_atom(con, false);
    ipc_send_window_event(c"floating".as_ptr(), con);
}

pub unsafe fn toggle_floating_mode(con: *mut Con, automatic: bool) {
    if (*con).type_0 as u32 == CT_FLOATING_CON as i32 as u32 {
        errorlog(
            c"ERROR: Cannot toggle floating mode on con = %p because it is of type CT_FLOATING_CON.\n".as_ptr().cast_mut(),
            con,
        );
        return;
    }
    if con_is_floating(con) {
        verboselog(
            c"already floating, re-setting to tiling\n"
                .as_ptr()
                .cast_mut(),
        );
        floating_disable(con);
        return;
    }
    floating_enable(con, automatic);
}

pub unsafe fn floating_raise_con(con: *mut Con) {
    debuglog(
        c"%s:%s:%d - Raising floating con %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_raise_con".as_ptr(),
        479,
        con,
        (*con).name,
    );
    floating::move_to_back(
        identity(parent_ptr(con)).expect("live floating parent"),
        identity(con).expect("live floating wrapper"),
    )
    .expect("raised container belongs to its floating parent");
}

pub unsafe fn floating_maybe_reassign_ws(con: *mut Con) -> bool {
    if con_is_internal(con_get_workspace(con)) {
        debuglog(
            c"%s:%s:%d - Con in an internal workspace\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_maybe_reassign_ws".as_ptr(),
            491,
        );
        return false;
    }
    let output: *mut Output = get_output_from_rect((*con).rect);
    if output.is_null() {
        errorlog(
            c"ERROR: No output found at destination coordinates?\n"
                .as_ptr()
                .cast_mut(),
        );
        return false;
    }
    if con_get_output(con) == (*output).con {
        debuglog(
            c"%s:%s:%d - still the same ws\n".as_ptr().cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_maybe_reassign_ws".as_ptr(),
            503,
        );
        return false;
    }
    debuglog(
        c"%s:%s:%d - Need to re-assign!\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_maybe_reassign_ws".as_ptr(),
        507,
    );
    let content: *mut Con = output_get_content((*output).con);
    let ws: *mut Con = focus::first_ptr(content);
    debuglog(
        c"%s:%s:%d - Moving con %p / %s to workspace %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_maybe_reassign_ws".as_ptr(),
        511,
        con,
        (*con).name,
        ws,
        (*ws).name,
    );
    let mut needs_focus: *mut Con = con_descend_focused(con);
    if !con_inside_focused(needs_focus) {
        needs_focus = ::core::ptr::null_mut::<Con>();
    }
    con_move_to_workspace(con, ws, false, true, false);
    if !needs_focus.is_null() {
        workspace_show(ws);
        con_activate(needs_focus);
    }
    true
}

fn centered(mut target: Rect, rect: Rect) -> Rect {
    target.x = rect
        .x
        .wrapping_add(rect.width.wrapping_div(2 as u32))
        .wrapping_sub(target.width.wrapping_div(2 as u32));
    target.y = rect
        .y
        .wrapping_add(rect.height.wrapping_div(2 as u32))
        .wrapping_sub(target.height.wrapping_div(2 as u32));

    target
}
pub unsafe fn floating_center(con: *mut Con, rect: Rect) {
    (*con).rect = centered((*con).rect, rect);
}

pub unsafe fn floating_move_to_pointer(con: *mut Con) {
    if (*con).type_0 as u32 == CT_FLOATING_CON as i32 as u32 {
    } else {
        __assert_fail(
            c"con->type == CT_FLOATING_CON".as_ptr(),
            c"../src/floating.c".as_ptr(),
            538 as u32,
            c"void floating_move_to_pointer(Con *)".as_ptr(),
        );
    };
    let mut reply: *mut xcb_query_pointer_reply_t = xcb_query_pointer_reply(
        conn,
        xcb_query_pointer(conn, root),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errorlog(
            c"ERROR: could not query pointer position, not moving this container\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let pointer = ((*reply).root_x as u32, (*reply).root_y as u32);
    free(reply.cast());
    let output = get_output_containing(pointer.0, pointer.1);
    if output.is_null() {
        errorlog(
            c"ERROR: The pointer is not on any output, cannot move the container here.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let destination = pointer_placement((*con).rect, (*output).rect, pointer);
    floating_reposition(con, destination);
}
unsafe fn drag_window_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    (*con).rect.x = (*old_rect)
        .x
        .wrapping_add(new_x.wrapping_sub((*event).root_x as u32));
    (*con).rect.y = (*old_rect)
        .y
        .wrapping_add(new_y.wrapping_sub((*event).root_y as u32));
    render_con(con);
    x_push_node(con);
    xcb_flush(conn);
    if !floating_maybe_reassign_ws(con) {
        return;
    }
    x_set_warp_to(::core::ptr::null_mut::<Rect>());
    tree_render();
}

pub unsafe fn floating_drag_window(
    con: *mut Con,
    event: *const xcb_button_press_event_t,
    use_threshold: bool,
) {
    debuglog(
        c"%s:%s:%d - floating_drag_window\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_drag_window".as_ptr(),
        595,
    );
    tree_render();
    let initial_rect: Rect = (*con).rect;
    let drag_result: DragResult = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        Cursor::Move,
        use_threshold,
        Some(
            drag_window_callback
                as unsafe fn(
                    *mut Con,
                    *mut Rect,
                    u32,
                    u32,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    if !con_exists(con) {
        debuglog(
            c"%s:%s:%d - The container has been closed in the meantime.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_drag_window".as_ptr(),
            608,
        );
        return;
    }
    if drag_result == DragResult::Revert {
        floating_reposition(con, initial_rect);
        return;
    }
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        (*con).scratchpad_state = ScratchpadState::Changed;
    }
    tree_render();
}
unsafe fn resize_window_callback(
    con: *mut Con,
    old_rect: *mut Rect,
    new_x: u32,
    new_y: u32,
    event: *const xcb_button_press_event_t,
    extra: *const ::core::ffi::c_void,
) {
    let params: *const ResizeParameters = extra as *const ResizeParameters;
    let corner: u32 = (*params).corner;
    let mut dest_x: i32 = (*con).rect.x as i32;
    let mut dest_y: i32 = (*con).rect.y as i32;
    let (dest_width, dest_height) = dragged_size(
        *old_rect,
        corner,
        (*params).proportional,
        (new_x, new_y),
        ((*event).root_x as u32, (*event).root_y as u32),
    );
    (*con).rect = Rect {
        x: dest_x as u32,
        y: dest_y as u32,
        width: dest_width,
        height: dest_height,
    };
    floating_check_size(con, false);
    if corner as u32 & BORDER_LEFT as i32 as u32 != 0 {
        dest_x = (*old_rect)
            .x
            .wrapping_add((*old_rect).width.wrapping_sub((*con).rect.width))
            as i32;
    }
    if corner as u32 & BORDER_TOP as i32 as u32 != 0 {
        dest_y = (*old_rect)
            .y
            .wrapping_add((*old_rect).height.wrapping_sub((*con).rect.height))
            as i32;
    }
    (*con).rect.x = dest_x as u32;
    (*con).rect.y = dest_y as u32;
    render_con(con);
    x_push_changes(croot);
}

pub unsafe fn floating_resize_window(
    con: *mut Con,
    proportional: bool,
    event: *const xcb_button_press_event_t,
) {
    debuglog(
        c"%s:%s:%d - floating_resize_window\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_resize_window".as_ptr(),
        699,
    );
    tree_render();
    let (corner, cursor) = resize_corner((*con).rect, (*event).event_x, (*event).event_y);
    let mut params: ResizeParameters = ResizeParameters {
        corner,
        proportional,
    };
    let initial_rect: Rect = (*con).rect;
    let drag_result: DragResult = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        cursor,
        false,
        Some(
            resize_window_callback
                as unsafe fn(
                    *mut Con,
                    *mut Rect,
                    u32,
                    u32,
                    *const xcb_button_press_event_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut params as *const ::core::ffi::c_void,
    );
    if !con_exists(con) {
        debuglog(
            c"%s:%s:%d - The container has been closed in the meantime.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_resize_window".as_ptr(),
            732,
        );
        return;
    }
    if drag_result == DragResult::Revert {
        floating_reposition(con, initial_rect);
    }
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        (*con).scratchpad_state = ScratchpadState::Changed;
    }
}

pub unsafe fn floating_reposition(con: *mut Con, newrect: Rect) -> bool {
    if output_containing_rect(newrect).is_null() {
        errorlog(
            c"ERROR: No output found at destination coordinates. Not repositioning.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false;
    }
    (*con).rect = newrect;
    floating_maybe_reassign_ws(con);
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        (*con).scratchpad_state = ScratchpadState::Changed;
    }
    tree_render();
    true
}

pub unsafe fn floating_resize(floating_con: *mut Con, x: u32, y: u32) {
    debuglog(
        c"%s:%s:%d - floating resize to %dx%d px\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_resize".as_ptr(),
        783,
        x,
        y,
    );
    let rect: *mut Rect = &raw mut (*floating_con).rect;
    let focused_con: *mut Con = con_descend_focused(floating_con);
    if (*focused_con).window.is_null() {
        debuglog(
            c"%s:%s:%d - No window is focused. Not resizing.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_resize".as_ptr(),
            787,
        );
        return;
    }
    let wi: i32 = (*(*focused_con).window).width_increment;
    let hi: i32 = (*(*focused_con).window).height_increment;
    let prefer_height: bool = (*rect).width == x;
    (*rect).width = x;
    (*rect).height = y;
    (*rect).width = increment_up((*rect).width, wi);
    (*rect).height = increment_up((*rect).height, hi);
    floating_check_size(floating_con, prefer_height);
    if (*floating_con).scratchpad_state == ScratchpadState::Fresh {
        (*floating_con).scratchpad_state = ScratchpadState::Changed;
    }
}

fn remapped(mut target: Rect, old_rect: Rect, new_rect: Rect) -> Rect {
    let rel_x: i32 = target
        .x
        .wrapping_sub(old_rect.x)
        .wrapping_add(target.width.wrapping_div(2 as u32) as i32 as u32)
        as i32;
    let rel_y: i32 = target
        .y
        .wrapping_sub(old_rect.y)
        .wrapping_add(target.height.wrapping_div(2 as u32) as i32 as u32)
        as i32;

    target.x = (new_rect.x as i32 as f64
        + (rel_x * new_rect.width as i32) as f64 / old_rect.width as i32 as f64
        - target.width.wrapping_div(2 as u32) as i32 as f64) as u32;
    target.y = (new_rect.y as i32 as f64
        + (rel_y * new_rect.height as i32) as f64 / old_rect.height as i32 as f64
        - target.height.wrapping_div(2 as u32) as i32 as f64) as u32;

    target
}
pub unsafe fn floating_fix_coordinates(con: *mut Con, old_rect: *mut Rect, new_rect: *mut Rect) {
    (*con).rect = remapped((*con).rect, *old_rect, *new_rect);
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tree::tree_render;
use crate::util::max;
use crate::util::min;

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_button_press_event_t, xcb_change_property, xcb_delete_property, xcb_flush,
    xcb_generic_error_t, xcb_query_pointer, xcb_query_pointer_reply, xcb_query_pointer_reply_t,
    xcb_window_t, XCB_ATOM_CARDINAL, XCB_NONE, XCB_PROP_MODE_REPLACE,
};

pub use libc::free;

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::root_screen;
use crate::application::A_I3_FLOATING_WINDOW;
use crate::con::con_activate;
use crate::con::con_attach;
use crate::con::con_border_style_rect;
use crate::con::con_by_window_id;
use crate::con::con_descend_focused;
use crate::con::con_descend_tiling_focused;
use crate::con::con_detach;
use crate::con::con_exists;
use crate::con::con_fix_percent;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_inside_focused;
use crate::con::con_is_docked;
use crate::con::con_is_floating;
use crate::con::con_is_internal;
use crate::con::con_is_leaf;
use crate::con::con_move_to_workspace;
use crate::con::con_new;
use crate::con::con_num_children;
use crate::config::config;
use crate::drag::drag_pointer;
use crate::ipc::ipc_send_window_event;
use crate::output::output_get_content;
use crate::r#move::insert_con_into;
use crate::randr::get_output_containing;
use crate::randr::get_output_from_rect;
use crate::randr::output_containing_rect;
use crate::render::render_con;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::util::rect_equals;
use crate::workspace::workspace_show;
use crate::x::x_push_changes;
use crate::x::x_push_node;
use crate::x::x_set_name;
use crate::x::x_set_warp_to;

fn resize_corner(rect: Rect, x: i16, y: i16) -> (u32, Cursor) {
    let left = x <= (rect.width / 2) as i16;
    let top = y <= (rect.height / 2) as i16;
    (
        (if left { BORDER_LEFT } else { BORDER_RIGHT })
            | (if top { BORDER_TOP } else { BORDER_BOTTOM }),
        match (left, top) {
            (true, true) => Cursor::TopLeftCorner,
            (false, true) => Cursor::TopRightCorner,
            (true, false) => Cursor::BottomLeftCorner,
            (false, false) => Cursor::BottomRightCorner,
        },
    )
}

fn dragged_size(
    old: Rect,
    corner: u32,
    proportional: bool,
    new: (u32, u32),
    origin: (u32, u32),
) -> (u32, u32) {
    let dx = new.0.wrapping_sub(origin.0);
    let dy = new.1.wrapping_sub(origin.1);
    let mut width = if corner & BORDER_LEFT != 0 {
        old.width.wrapping_sub(dx)
    } else {
        old.width.wrapping_add(dx)
    };
    let mut height = if corner & BORDER_TOP != 0 {
        old.height.wrapping_sub(dy)
    } else {
        old.height.wrapping_add(dy)
    };
    if proportional {
        let ratio = f64::from(old.width) / f64::from(old.height);
        width = (width as i32).max((f64::from(height) * ratio) as i32) as u32;
        height = (height as i32).max((f64::from(width) / ratio) as i32) as u32;
    }
    (width, height)
}
fn pointer_placement(mut rect: Rect, output: Rect, pointer: (u32, u32)) -> Rect {
    let mut x = (pointer.0.wrapping_sub(rect.width / 2) as i32).max(output.x as i32);
    let mut y = (pointer.1.wrapping_sub(rect.height / 2) as i32).max(output.y as i32);
    if (x as u32).wrapping_add(rect.width) > output.x.wrapping_add(output.width) {
        x = output.x.wrapping_add(output.width).wrapping_sub(rect.width) as i32;
    }
    if (y as u32).wrapping_add(rect.height) > output.y.wrapping_add(output.height) {
        y = output
            .y
            .wrapping_add(output.height)
            .wrapping_sub(rect.height) as i32;
    }
    rect.x = x as u32;
    rect.y = y as u32;
    rect
}
fn increment_up(value: u32, increment: i32) -> u32 {
    if increment == 0 {
        value
    } else {
        value.wrapping_add(
            ((increment - 1) as u32)
                .wrapping_sub(value)
                .wrapping_rem(increment as u32),
        )
    }
}
#[cfg(test)]
mod owned_tests {
    use super::*;
    fn rect(w: u32, h: u32) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }
    fn unlimited() -> SizeLimits {
        SizeLimits {
            minimum_width: -1,
            minimum_height: -1,
            maximum_width: -1,
            maximum_height: -1,
        }
    }
    #[test]
    fn client_constraints_keep_aspect_base_increment_and_config_order() {
        let hints = SizeHints {
            base_width: 10,
            base_height: 10,
            min_width: 40,
            min_height: 30,
            max_width: 200,
            max_height: 200,
            width_increment: 7,
            height_increment: 3,
            min_aspect_ratio: 2.,
            ..SizeHints::default()
        };
        let result = constrained_size(
            rect(41, 80),
            rect(2, 4),
            Some(hints),
            unlimited(),
            rect(1000, 1000),
            false,
            true,
        );
        assert_eq!((result.width, result.height), (138, 80));
        let full = constrained_size(
            rect(41, 80),
            rect(2, 4),
            Some(hints),
            unlimited(),
            rect(1000, 1000),
            true,
            true,
        );
        assert_eq!((full.width, full.height), (40, 80));
        let limits = SizeLimits {
            minimum_width: 0,
            minimum_height: 0,
            maximum_width: 100,
            maximum_height: 40,
        };
        let clipped = constrained_size(
            rect(1, 1),
            rect(2, 4),
            None,
            limits,
            rect(1000, 1000),
            false,
            false,
        );
        assert_eq!((clipped.width, clipped.height), (77, 44));
    }
    #[test]
    fn geometry_keeps_odd_centering_signed_corner_cast_and_wrapping_drag() {
        let centered = centered(
            rect(5, 7),
            Rect {
                x: 10,
                y: 20,
                width: 10,
                height: 10,
            },
        );
        assert_eq!((centered.x, centered.y), (13, 22));
        let (corner, _) = resize_corner(rect(131072, 100), 1, 50);
        assert_eq!(corner, BORDER_RIGHT | BORDER_TOP);
        assert_eq!(
            dragged_size(
                rect(100, 50),
                BORDER_LEFT | BORDER_TOP,
                false,
                (5, 10),
                (10, 20)
            ),
            (105, 60)
        );
        assert_eq!(
            dragged_size(
                rect(100, 50),
                BORDER_RIGHT | BORDER_BOTTOM,
                true,
                (10, 10),
                (0, 0)
            ),
            (120, 60)
        );
        let placed = pointer_placement(rect(120, 120), rect(100, 100), (50, 50));
        assert_eq!((placed.x, placed.y), (u32::MAX - 19, u32::MAX - 19));
        assert_eq!(increment_up(10, 3), 12); // Historical unsigned (inc-1-value) remainder.
        assert_eq!(increment_up(0, 0), 0);
    }
    #[test]
    fn coordinate_remap_keeps_signed_relative_center_and_truncation() {
        let mapped = remapped(
            Rect {
                x: 110,
                y: 60,
                width: 20,
                height: 10,
            },
            Rect {
                x: 100,
                y: 50,
                width: 100,
                height: 100,
            },
            Rect {
                x: 0,
                y: 0,
                width: 200,
                height: 50,
            },
        );
        assert_eq!((mapped.x, mapped.y), (30, 2));
    }
}
