use crate::randr::outputs;
use crate::types::DragResult;
use crate::types::FloatingState;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Position;
use crate::types::ScratchpadState;
use crate::xcursor::Cursor;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * floating.c: Floating windows.
 *
 */
// Mechanical file-by-file Rust translation of src/floating.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
extern "C" {

    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

}

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
pub type i3Window = Window;

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
pub struct resize_window_callback_params {
    pub corner: border_t,
    pub proportional: bool,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"void floating_move_to_pointer(Con *)\0",
    )
};

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            (*(*con).window).id,
            A_I3_FLOATING_WINDOW,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as u8,
            1 as u32,
            &raw mut val as *const ::core::ffi::c_void,
        );
    } else {
        xcb_delete_property(conn, (*(*con).window).id, A_I3_FLOATING_WINDOW);
    }
    xcb_flush(conn);
}

pub unsafe fn floating_check_size(floating_con: *mut Con, prefer_height: bool) {
    let floating_sane_min_height: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let floating_sane_min_width: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
    let focused_con: *mut Con = con_descend_focused(floating_con);
    debuglog(
        c"%s:%s:%d - deco_rect.height = %d\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_check_size".as_ptr(),
        82 as ::core::ffi::c_int,
        (*focused_con).deco_rect.height,
    );
    let mut border_rect: Rect = con_border_style_rect(focused_con);
    border_rect.width = border_rect.width.wrapping_neg();
    border_rect.height = border_rect.height.wrapping_neg();
    border_rect.width = border_rect
        .width
        .wrapping_add((2 as ::core::ffi::c_int * (*focused_con).border_width) as u32);
    border_rect.height = border_rect
        .height
        .wrapping_add((2 as ::core::ffi::c_int * (*focused_con).border_width) as u32);
    debuglog(
        c"%s:%s:%d - floating_check_size, want min width %d, min height %d, border extra: w=%d, h=%d\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_check_size".as_ptr(),
        97 as ::core::ffi::c_int,
        floating_sane_min_width,
        floating_sane_min_height,
        border_rect.width,
        border_rect.height,
    );
    let window: *mut i3Window = (*focused_con).window as *mut i3Window;
    if !window.is_null() {
        let min_width: ::core::ffi::c_int = if (*window).min_width != 0 {
            (*window).min_width
        } else {
            (*window).base_width
        };
        let min_height: ::core::ffi::c_int = if (*window).min_height != 0 {
            (*window).min_height
        } else {
            (*window).base_height
        };
        let base_width: ::core::ffi::c_int = if (*window).base_width != 0 {
            (*window).base_width
        } else {
            (*window).min_width
        };
        let base_height: ::core::ffi::c_int = if (*window).base_height != 0 {
            (*window).base_height
        } else {
            (*window).min_height
        };
        if min_width != 0 {
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
            (*floating_con).rect.width =
                max((*floating_con).rect.width as ::core::ffi::c_int, min_width) as u32;
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
        }
        if min_height != 0 {
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_sub(border_rect.height);
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                min_height,
            ) as u32;
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_add(border_rect.height);
        }
        if (*window).max_width != 0 {
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                (*window).max_width,
            ) as u32;
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
        }
        if (*window).max_height != 0 {
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_sub(border_rect.height);
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                (*window).max_height,
            ) as u32;
            (*floating_con).rect.height =
                (*floating_con).rect.height.wrapping_add(border_rect.height);
        }
        let min_ar: ::core::ffi::c_double = (*window).min_aspect_ratio;
        let max_ar: ::core::ffi::c_double = (*window).max_aspect_ratio;
        if (*floating_con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (min_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || max_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        {
            let mut width: ::core::ffi::c_double = (*floating_con)
                .rect
                .width
                .wrapping_sub((*window).base_width as u32)
                .wrapping_sub(border_rect.width)
                as ::core::ffi::c_double;
            let mut height: ::core::ffi::c_double = (*floating_con)
                .rect
                .height
                .wrapping_sub((*window).base_height as u32)
                .wrapping_sub(border_rect.height)
                as ::core::ffi::c_double;
            let ar: ::core::ffi::c_double = width / height;
            let mut new_ar: ::core::ffi::c_double =
                -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
            if min_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double && ar < min_ar {
                new_ar = min_ar;
            } else if max_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double && ar > max_ar {
                new_ar = max_ar;
            }
            if new_ar > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                if prefer_height {
                    width = round(height * new_ar);
                    height = round(width / new_ar);
                } else {
                    height = round(width / new_ar);
                    width = round(height * new_ar);
                }
                (*floating_con).rect.width = (width
                    + (*window).base_width as ::core::ffi::c_double
                    + border_rect.width as ::core::ffi::c_double)
                    as u32;
                (*floating_con).rect.height = (height
                    + (*window).base_height as ::core::ffi::c_double
                    + border_rect.height as ::core::ffi::c_double)
                    as u32;
            }
        }
        if (*window).height_increment != 0
            && (*floating_con).rect.height >= (base_height as u32).wrapping_add(border_rect.height)
        {
            (*floating_con).rect.height = (*floating_con)
                .rect
                .height
                .wrapping_sub((base_height as u32).wrapping_add(border_rect.height));
            (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(
                (*floating_con)
                    .rect
                    .height
                    .wrapping_rem((*window).height_increment as u32),
            );
            (*floating_con).rect.height = (*floating_con)
                .rect
                .height
                .wrapping_add((base_height as u32).wrapping_add(border_rect.height));
        }
        if (*window).width_increment != 0
            && (*floating_con).rect.width >= (base_width as u32).wrapping_add(border_rect.width)
        {
            (*floating_con).rect.width = (*floating_con)
                .rect
                .width
                .wrapping_sub((base_width as u32).wrapping_add(border_rect.width));
            (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(
                (*floating_con)
                    .rect
                    .width
                    .wrapping_rem((*window).width_increment as u32),
            );
            (*floating_con).rect.width = (*floating_con)
                .rect
                .width
                .wrapping_add((base_width as u32).wrapping_add(border_rect.width));
        }
    }
    if config.floating_minimum_height != -(1 as i32) {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(border_rect.height);
        if config.floating_minimum_height == 0 as i32 {
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                floating_sane_min_height,
            ) as u32;
        } else {
            (*floating_con).rect.height = max(
                (*floating_con).rect.height as ::core::ffi::c_int,
                config.floating_minimum_height as ::core::ffi::c_int,
            ) as u32;
        }
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(border_rect.height);
    }
    if config.floating_minimum_width != -(1 as i32) {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
        if config.floating_minimum_width == 0 as i32 {
            (*floating_con).rect.width = max(
                (*floating_con).rect.width as ::core::ffi::c_int,
                floating_sane_min_width,
            ) as u32;
        } else {
            (*floating_con).rect.width = max(
                (*floating_con).rect.width as ::core::ffi::c_int,
                config.floating_minimum_width as ::core::ffi::c_int,
            ) as u32;
        }
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
    }
    let floating_sane_max_dimensions: Rect = total_outputs_dimensions();
    if config.floating_maximum_height != -(1 as i32) {
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_sub(border_rect.height);
        if config.floating_maximum_height == 0 as i32 {
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                floating_sane_max_dimensions.height as ::core::ffi::c_int,
            ) as u32;
        } else {
            (*floating_con).rect.height = min(
                (*floating_con).rect.height as ::core::ffi::c_int,
                config.floating_maximum_height as ::core::ffi::c_int,
            ) as u32;
        }
        (*floating_con).rect.height = (*floating_con).rect.height.wrapping_add(border_rect.height);
    }
    if config.floating_maximum_width != -(1 as i32) {
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_sub(border_rect.width);
        if config.floating_maximum_width == 0 as i32 {
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                floating_sane_max_dimensions.width as ::core::ffi::c_int,
            ) as u32;
        } else {
            (*floating_con).rect.width = min(
                (*floating_con).rect.width as ::core::ffi::c_int,
                config.floating_maximum_width as ::core::ffi::c_int,
            ) as u32;
        }
        (*floating_con).rect.width = (*floating_con).rect.width.wrapping_add(border_rect.width);
    }
}

pub unsafe fn floating_enable(con: *mut Con, automatic: bool) -> bool {
    let set_focus: bool = con == focused;
    if con_is_docked(con) {
        verboselog(
            c"Container is a dock window, not enabling floating mode.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false_0 != 0;
    }
    if con_is_floating(con) {
        verboselog(
            c"Container is already in floating mode, not doing anything.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false_0 != 0;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        verboselog(
            c"Container is a workspace, not enabling floating mode.\n"
                .as_ptr()
                .cast_mut(),
        );
        return false_0 != 0;
    }
    let mut focus_head_placeholder: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut focus_before_parent: bool = true_0 != 0;
    if !set_focus {
        let mut ancestor: *mut Con = con;
        while (*(*ancestor).parent).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            focus_before_parent = (focus_before_parent as ::core::ffi::c_int
                & ((*(*ancestor).parent).focus_head.tqh_first == ancestor) as ::core::ffi::c_int)
                != 0;
            ancestor = (*ancestor).parent as *mut Con;
        }
        if focus_before_parent {
            focus_head_placeholder =
                *(*((*ancestor).focused.tqe_prev as *mut focus_head)).tqh_last as *mut Con;
        } else {
            focus_head_placeholder = (*ancestor).focused.tqe_next as *mut Con;
        }
    }
    con_detach(con);
    con_fix_percent((*con).parent as *mut Con);
    let nc: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    let ws: *mut Con = con_get_workspace(con);
    (*nc).parent = ws as *mut Con;
    (*nc).type_0 = CT_FLOATING_CON;
    (*nc).layout = Layout::SplitH;
    (*nc).floating_windows.tqe_next = (*ws).floating_head.tqh_first;
    if !(*nc).floating_windows.tqe_next.is_null() {
        (*(*ws).floating_head.tqh_first).floating_windows.tqe_prev =
            &raw mut (*nc).floating_windows.tqe_next;
    } else {
        (*ws).floating_head.tqh_last = &raw mut (*nc).floating_windows.tqe_next;
    }
    (*ws).floating_head.tqh_first = nc as *mut Con;
    (*nc).floating_windows.tqe_prev = &raw mut (*ws).floating_head.tqh_first;
    let fh: *mut focus_head = &raw mut (*ws).focus_head;
    if focus_before_parent {
        if !focus_head_placeholder.is_null() {
            (*nc).focused.tqe_next = (*focus_head_placeholder).focused.tqe_next;
            if !(*nc).focused.tqe_next.is_null() {
                (*(*nc).focused.tqe_next).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
            } else {
                (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
            }
            (*focus_head_placeholder).focused.tqe_next = nc as *mut Con;
            (*nc).focused.tqe_prev = &raw mut (*focus_head_placeholder).focused.tqe_next;
        } else {
            (*nc).focused.tqe_next = (*fh).tqh_first;
            if !(*nc).focused.tqe_next.is_null() {
                (*(*fh).tqh_first).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
            } else {
                (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
            }
            (*fh).tqh_first = nc as *mut Con;
            (*nc).focused.tqe_prev = &raw mut (*fh).tqh_first;
        }
    } else if !focus_head_placeholder.is_null() {
        (*nc).focused.tqe_prev = (*focus_head_placeholder).focused.tqe_prev;
        (*nc).focused.tqe_next = focus_head_placeholder as *mut Con;
        *(*focus_head_placeholder).focused.tqe_prev = nc as *mut Con;
        (*focus_head_placeholder).focused.tqe_prev = &raw mut (*nc).focused.tqe_next;
    } else {
        (*nc).focused.tqe_next = ::core::ptr::null_mut::<Con>();
        (*nc).focused.tqe_prev = (*fh).tqh_last;
        *(*fh).tqh_last = nc as *mut Con;
        (*fh).tqh_last = &raw mut (*nc).focused.tqe_next;
    }
    if ((*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint)
        && con_num_children((*con).parent as *mut Con) == 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - Old container empty after setting this child to floating, closing\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_enable".as_ptr(),
            316 as ::core::ffi::c_int,
        );
        let parent: *mut Con = (*con).parent as *mut Con;
        (*con).parent = ::core::ptr::null_mut::<Con>();
        tree_close_internal(parent, KillWindow::DontKill, false_0 != 0);
    }
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut name,
        c"[i3 con] floatingcon around %p".as_ptr(),
        con,
    );
    x_set_name(nc, name);
    free(name as *mut ::core::ffi::c_void);
    debuglog(
        c"%s:%s:%d - Original rect: (%d, %d) with %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_enable".as_ptr(),
        328 as ::core::ffi::c_int,
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
        329 as ::core::ffi::c_int,
        (*con).geometry.x,
        (*con).geometry.y,
        (*con).geometry.width,
        (*con).geometry.height,
    );
    (*nc).rect = (*con).geometry;
    if rect_equals(
        (*nc).rect as Rect,
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
            334 as ::core::ffi::c_int,
        );
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            debuglog(
                c"%s:%s:%d - child geometry: %d x %d\n".as_ptr().cast_mut(),
                c"../src/floating.c".as_ptr(),
                c"floating_enable".as_ptr(),
                337 as ::core::ffi::c_int,
                (*child).geometry.width,
                (*child).geometry.height,
            );
            (*nc).rect.width = (*nc).rect.width.wrapping_add((*child).geometry.width);
            (*nc).rect.height = max(
                (*nc).rect.height as ::core::ffi::c_int,
                (*child).geometry.height as ::core::ffi::c_int,
            ) as u32;
            child = (*child).nodes.tqe_next as *mut Con;
        }
    }
    (*con).nodes.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).nodes.tqe_prev = (*nc).nodes_head.tqh_last;
    *(*nc).nodes_head.tqh_last = con as *mut Con;
    (*nc).nodes_head.tqh_last = &raw mut (*con).nodes.tqe_next;
    (*con).focused.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).focused.tqe_prev = (*nc).focus_head.tqh_last;
    *(*nc).focus_head.tqh_last = con as *mut Con;
    (*nc).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
    (*con).parent = nc as *mut Con;
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
        .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
    (*nc).rect.width = (*nc)
        .rect
        .width
        .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
    floating_check_size(nc, false_0 != 0);
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
                377 as ::core::ffi::c_int,
            );
            floating_center(nc, (*leader).rect as Rect);
        } else {
            floating_center(nc, (*ws).rect as Rect);
        }
    }
    let mut current_output: *mut Output = get_output_from_rect((*nc).rect as Rect);
    let correct_output: *mut Con = con_get_output(ws);
    if current_output.is_null() || (*current_output).con != correct_output {
        debuglog(
            c"%s:%s:%d - This floating window is on the wrong output, fixing coordinates (currently (%d, %d))\n".as_ptr().cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_enable".as_ptr(),
            391 as ::core::ffi::c_int,
            (*nc).rect.x,
            (*nc).rect.y,
        );
        if !current_output.is_null() {
            floating_fix_coordinates(
                nc,
                &raw mut (*(*current_output).con).rect,
                &raw mut (*correct_output).rect,
            );
            current_output = get_output_from_rect((*nc).rect as Rect);
        }
        if current_output.is_null() || (*current_output).con != correct_output {
            floating_center(nc, (*ws).rect as Rect);
        }
    }
    debuglog(
        c"%s:%s:%d - Floating rect: (%d, %d) with %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_enable".as_ptr(),
        405 as ::core::ffi::c_int,
        (*nc).rect.x,
        (*nc).rect.y,
        (*nc).rect.width,
        (*nc).rect.height,
    );
    render_con(nc);
    if set_focus {
        con_activate(con);
    }
    floating_set_hint_atom(nc, true_0 != 0);
    ipc_send_window_event(c"floating".as_ptr(), con);
    true_0 != 0
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
    if (*tiling_focused).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let parent: *mut Con = (*con).parent as *mut Con;
        con_detach(con);
        (*con).parent = ::core::ptr::null_mut::<Con>();
        tree_close_internal(parent, KillWindow::DontKill, true_0 != 0);
        con_attach(con, tiling_focused, false_0 != 0);
        (*con).percent = 0.0f64;
        con_fix_percent((*con).parent as *mut Con);
    } else {
        insert_con_into(con, tiling_focused, Position::After);
    }
    (*con).floating = FloatingState::UserOff;
    floating_set_hint_atom(con, false_0 != 0);
    ipc_send_window_event(c"floating".as_ptr(), con);
}

pub unsafe fn toggle_floating_mode(con: *mut Con, automatic: bool) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
        479 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if !(*con).floating_windows.tqe_next.is_null() {
        (*(*con).floating_windows.tqe_next)
            .floating_windows
            .tqe_prev = (*con).floating_windows.tqe_prev;
    } else {
        (*(*con).parent).floating_head.tqh_last = (*con).floating_windows.tqe_prev;
    }
    *(*con).floating_windows.tqe_prev = (*con).floating_windows.tqe_next;
    (*con).floating_windows.tqe_next = ::core::ptr::null_mut::<Con>();
    (*con).floating_windows.tqe_prev = (*(*con).parent).floating_head.tqh_last;
    *(*(*con).parent).floating_head.tqh_last = con as *mut Con;
    (*(*con).parent).floating_head.tqh_last = &raw mut (*con).floating_windows.tqe_next;
}

pub unsafe fn floating_maybe_reassign_ws(con: *mut Con) -> bool {
    if con_is_internal(con_get_workspace(con)) {
        debuglog(
            c"%s:%s:%d - Con in an internal workspace\n"
                .as_ptr()
                .cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_maybe_reassign_ws".as_ptr(),
            491 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    let output: *mut Output = get_output_from_rect((*con).rect as Rect);
    if output.is_null() {
        errorlog(
            c"ERROR: No output found at destination coordinates?\n"
                .as_ptr()
                .cast_mut(),
        );
        return false_0 != 0;
    }
    if con_get_output(con) == (*output).con {
        debuglog(
            c"%s:%s:%d - still the same ws\n".as_ptr().cast_mut(),
            c"../src/floating.c".as_ptr(),
            c"floating_maybe_reassign_ws".as_ptr(),
            503 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    debuglog(
        c"%s:%s:%d - Need to re-assign!\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_maybe_reassign_ws".as_ptr(),
        507 as ::core::ffi::c_int,
    );
    let content: *mut Con = output_get_content((*output).con);
    let ws: *mut Con = (*content).focus_head.tqh_first as *mut Con;
    debuglog(
        c"%s:%s:%d - Moving con %p / %s to workspace %p / %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_maybe_reassign_ws".as_ptr(),
        511 as ::core::ffi::c_int,
        con,
        (*con).name,
        ws,
        (*ws).name,
    );
    let mut needs_focus: *mut Con = con_descend_focused(con);
    if !con_inside_focused(needs_focus) {
        needs_focus = ::core::ptr::null_mut::<Con>();
    }
    con_move_to_workspace(con, ws, false_0 != 0, true_0 != 0, false_0 != 0);
    if !needs_focus.is_null() {
        workspace_show(ws);
        con_activate(needs_focus);
    }
    true_0 != 0
}

pub unsafe fn floating_center(con: *mut Con, rect: Rect) {
    (*con).rect.x = rect
        .x
        .wrapping_add(rect.width.wrapping_div(2 as u32))
        .wrapping_sub((*con).rect.width.wrapping_div(2 as u32));
    (*con).rect.y = rect
        .y
        .wrapping_add(rect.height.wrapping_div(2 as u32))
        .wrapping_sub((*con).rect.height.wrapping_div(2 as u32));
}

pub unsafe fn floating_move_to_pointer(con: *mut Con) {
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"con->type == CT_FLOATING_CON".as_ptr(),
            c"../src/floating.c".as_ptr(),
            538 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
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
    let output: *mut Output = get_output_containing(
        (*reply).root_x as ::core::ffi::c_uint,
        (*reply).root_y as ::core::ffi::c_uint,
    );
    if output.is_null() {
        errorlog(
            c"ERROR: The pointer is not on any output, cannot move the container here.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let mut x: i32 =
        ((*reply).root_x as u32).wrapping_sub((*con).rect.width.wrapping_div(2 as u32)) as i32;
    let mut y: i32 =
        ((*reply).root_y as u32).wrapping_sub((*con).rect.height.wrapping_div(2 as u32)) as i32;
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_query_pointer_reply_t>();
    x = if x > (*output).rect.x as i32 {
        x
    } else {
        (*output).rect.x as i32
    };
    y = if y > (*output).rect.y as i32 {
        y
    } else {
        (*output).rect.y as i32
    };
    if (x as u32).wrapping_add((*con).rect.width)
        > (*output).rect.x.wrapping_add((*output).rect.width)
    {
        x = (*output)
            .rect
            .x
            .wrapping_add((*output).rect.width)
            .wrapping_sub((*con).rect.width) as i32;
    }
    if (y as u32).wrapping_add((*con).rect.height)
        > (*output).rect.y.wrapping_add((*output).rect.height)
    {
        y = (*output)
            .rect
            .y
            .wrapping_add((*output).rect.height)
            .wrapping_sub((*con).rect.height) as i32;
    }
    floating_reposition(
        con,
        Rect {
            x: x as u32,
            y: y as u32,
            width: (*con).rect.width,
            height: (*con).rect.height,
        },
    );
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
        595 as ::core::ffi::c_int,
    );
    tree_render();
    let initial_rect: Rect = (*con).rect as Rect;
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
            608 as ::core::ffi::c_int,
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
    let params: *const resize_window_callback_params =
        extra as *const resize_window_callback_params;
    let corner: border_t = (*params).corner;
    let mut dest_x: i32 = (*con).rect.x as i32;
    let mut dest_y: i32 = (*con).rect.y as i32;
    let mut dest_width: u32 = 0;
    let mut dest_height: u32 = 0;
    let ratio: ::core::ffi::c_double =
        (*old_rect).width as ::core::ffi::c_double / (*old_rect).height as ::core::ffi::c_double;
    if corner as ::core::ffi::c_uint & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_width = (*old_rect)
            .width
            .wrapping_sub(new_x.wrapping_sub((*event).root_x as u32));
    } else {
        dest_width = (*old_rect)
            .width
            .wrapping_add(new_x.wrapping_sub((*event).root_x as u32));
    }
    if corner as ::core::ffi::c_uint & BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_height = (*old_rect)
            .height
            .wrapping_sub(new_y.wrapping_sub((*event).root_y as u32));
    } else {
        dest_height = (*old_rect)
            .height
            .wrapping_add(new_y.wrapping_sub((*event).root_y as u32));
    }
    if (*params).proportional {
        dest_width = max(
            dest_width as ::core::ffi::c_int,
            (dest_height as ::core::ffi::c_double * ratio) as ::core::ffi::c_int,
        ) as u32;
        dest_height = max(
            dest_height as ::core::ffi::c_int,
            (dest_width as ::core::ffi::c_double / ratio) as ::core::ffi::c_int,
        ) as u32;
    }
    (*con).rect = Rect {
        x: dest_x as u32,
        y: dest_y as u32,
        width: dest_width,
        height: dest_height,
    } as Rect;
    floating_check_size(con, false_0 != 0);
    if corner as ::core::ffi::c_uint & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        dest_x = (*old_rect)
            .x
            .wrapping_add((*old_rect).width.wrapping_sub((*con).rect.width))
            as i32;
    }
    if corner as ::core::ffi::c_uint & BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
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
        699 as ::core::ffi::c_int,
    );
    tree_render();
    let mut corner: border_t = 0 as border_t;
    if (*event).event_x as ::core::ffi::c_int
        <= (*con).rect.width.wrapping_div(2 as u32) as i16 as ::core::ffi::c_int
    {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    } else {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    let mut cursor: Cursor = Cursor::Pointer;
    if (*event).event_y as ::core::ffi::c_int
        <= (*con).rect.height.wrapping_div(2 as u32) as i16 as ::core::ffi::c_int
    {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint | BORDER_TOP as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cursor = if corner as ::core::ffi::c_uint
            & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            Cursor::TopLeftCorner
        } else {
            Cursor::TopRightCorner
        };
    } else {
        corner = ::core::mem::transmute::<::core::ffi::c_uint, border_t>(
            corner as ::core::ffi::c_uint
                | BORDER_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cursor = if corner as ::core::ffi::c_uint
            & BORDER_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            Cursor::BottomLeftCorner
        } else {
            Cursor::BottomRightCorner
        };
    }
    let mut params: resize_window_callback_params = resize_window_callback_params {
        corner,
        proportional,
    };
    let initial_rect: Rect = (*con).rect as Rect;
    let drag_result: DragResult = drag_pointer(
        con,
        event,
        XCB_NONE as xcb_window_t,
        cursor,
        false_0 != 0,
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
            732 as ::core::ffi::c_int,
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
        return false_0 != 0;
    }
    (*con).rect = newrect as Rect;
    floating_maybe_reassign_ws(con);
    if (*con).scratchpad_state == ScratchpadState::Fresh {
        (*con).scratchpad_state = ScratchpadState::Changed;
    }
    tree_render();
    true_0 != 0
}

pub unsafe fn floating_resize(floating_con: *mut Con, x: u32, y: u32) {
    debuglog(
        c"%s:%s:%d - floating resize to %dx%d px\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_resize".as_ptr(),
        783 as ::core::ffi::c_int,
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
            787 as ::core::ffi::c_int,
        );
        return;
    }
    let wi: ::core::ffi::c_int = (*(*focused_con).window).width_increment;
    let hi: ::core::ffi::c_int = (*(*focused_con).window).height_increment;
    let prefer_height: bool = (*rect).width == x;
    (*rect).width = x;
    (*rect).height = y;
    if wi != 0 {
        (*rect).width = (*rect).width.wrapping_add(
            ((wi - 1 as ::core::ffi::c_int) as u32)
                .wrapping_sub((*rect).width)
                .wrapping_rem(wi as u32),
        );
    }
    if hi != 0 {
        (*rect).height = (*rect).height.wrapping_add(
            ((hi - 1 as ::core::ffi::c_int) as u32)
                .wrapping_sub((*rect).height)
                .wrapping_rem(hi as u32),
        );
    }
    floating_check_size(floating_con, prefer_height);
    if (*floating_con).scratchpad_state == ScratchpadState::Fresh {
        (*floating_con).scratchpad_state = ScratchpadState::Changed;
    }
}

pub unsafe fn floating_fix_coordinates(con: *mut Con, old_rect: *mut Rect, new_rect: *mut Rect) {
    debuglog(
        c"%s:%s:%d - Fixing coordinates of floating window %p (rect (%d, %d), %d x %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_fix_coordinates".as_ptr(),
        817 as ::core::ffi::c_int,
        con,
        (*con).rect.x,
        (*con).rect.y,
        (*con).rect.width,
        (*con).rect.height,
    );
    debuglog(
        c"%s:%s:%d - old_rect = (%d, %d), %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_fix_coordinates".as_ptr(),
        819 as ::core::ffi::c_int,
        (*old_rect).x,
        (*old_rect).y,
        (*old_rect).width,
        (*old_rect).height,
    );
    debuglog(
        c"%s:%s:%d - new_rect = (%d, %d), %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_fix_coordinates".as_ptr(),
        821 as ::core::ffi::c_int,
        (*new_rect).x,
        (*new_rect).y,
        (*new_rect).width,
        (*new_rect).height,
    );
    let rel_x: i32 = (*con)
        .rect
        .x
        .wrapping_sub((*old_rect).x)
        .wrapping_add((*con).rect.width.wrapping_div(2 as u32) as i32 as u32)
        as i32;
    let rel_y: i32 = (*con)
        .rect
        .y
        .wrapping_sub((*old_rect).y)
        .wrapping_add((*con).rect.height.wrapping_div(2 as u32) as i32 as u32)
        as i32;
    debuglog(
        c"%s:%s:%d - rel_x = %d, rel_y = %d, fraction_x = %f, fraction_y = %f, output->w = %d, output->h = %d\n".as_ptr().cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_fix_coordinates".as_ptr(),
        830 as ::core::ffi::c_int,
        rel_x,
        rel_y,
        rel_x as ::core::ffi::c_double / (*old_rect).width as ::core::ffi::c_double,
        rel_y as ::core::ffi::c_double / (*old_rect).height as ::core::ffi::c_double,
        (*old_rect).width,
        (*old_rect).height,
    );
    (*con).rect.x = ((*new_rect).x as i32 as ::core::ffi::c_double
        + (rel_x * (*new_rect).width as i32) as ::core::ffi::c_double
            / (*old_rect).width as i32 as ::core::ffi::c_double
        - (*con).rect.width.wrapping_div(2 as u32) as i32 as ::core::ffi::c_double)
        as u32;
    (*con).rect.y = ((*new_rect).y as i32 as ::core::ffi::c_double
        + (rel_y * (*new_rect).height as i32) as ::core::ffi::c_double
            / (*old_rect).height as i32 as ::core::ffi::c_double
        - (*con).rect.height.wrapping_div(2 as u32) as i32 as ::core::ffi::c_double)
        as u32;
    debuglog(
        c"%s:%s:%d - Resulting coordinates: x = %d, y = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/floating.c".as_ptr(),
        c"floating_fix_coordinates".as_ptr(),
        834 as ::core::ffi::c_int,
        (*con).rect.x,
        (*con).rect.y,
    );
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::tree::tree_render;
use crate::util::max;
use crate::util::min;

// Canonical Cairo backend ABI; renderer logic remains local.

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
pub use crate::types::focus_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

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
