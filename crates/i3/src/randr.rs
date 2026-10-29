use crate::application::ws_assignments;
use crate::tree::all_cons;
use crate::types::Direction;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::OutputSelection;
use crate::workspace::output_triggers_assignment;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * For more information on RandR, please see the X.org RandR specification at
 * https://cgit.freedesktop.org/xorg/proto/randrproto/tree/randrproto.txt
 * (take your time to read it completely, it answers all questions).
 *
 */
// Mechanical file-by-file Rust translation of src/randr.c.
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
pub type i3Window = Window;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;

pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub use crate::application::outputs_head;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut primary: *mut xcb_randr_get_output_primary_reply_t =
    ::core::ptr::null::<xcb_randr_get_output_primary_reply_t>()
        as *mut xcb_randr_get_output_primary_reply_t;

pub static mut outputs: outputs_head = outputs_head {
    tqh_first: ::core::ptr::null::<xoutput>() as *mut xoutput,
    tqh_last: ::core::ptr::null::<*mut xoutput>() as *mut *mut xoutput,
};
static mut root_output: *mut Output = ::core::ptr::null::<Output>() as *mut Output;
static mut has_randr_1_5: bool = false_0 != 0;
unsafe fn get_output_by_id(id: xcb_randr_output_t) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).id == id {
            return output;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ::core::ptr::null_mut::<Output>()
}

pub unsafe fn get_output_by_name(
    name: *const ::core::ffi::c_char,
    require_active: bool,
) -> *mut Output {
    let get_primary: bool = strcasecmp(c"primary".as_ptr(), name) == 0 as ::core::ffi::c_int;
    let get_non_primary: bool = strcasecmp(c"nonprimary".as_ptr(), name) == 0 as ::core::ffi::c_int;
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(require_active as ::core::ffi::c_int != 0 && !(*output).active) {
            if (*output).primary as ::core::ffi::c_int != 0
                && get_primary as ::core::ffi::c_int != 0
            {
                return output;
            }
            if !(*output).primary && get_non_primary as ::core::ffi::c_int != 0 {
                return output;
            }
            let mut output_name: *mut output_name = ::core::ptr::null_mut::<output_name>();
            output_name = (*output).names_head.slh_first;
            while !output_name.is_null() {
                if strcasecmp((*output_name).name, name) == 0 as ::core::ffi::c_int {
                    return output;
                }
                output_name = (*output_name).names.sle_next;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ::core::ptr::null_mut::<Output>()
}

pub unsafe fn get_first_output() -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut result: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            if (*output).primary {
                return output;
            }
            if result.is_null() {
                result = output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    if !result.is_null() {
        return result;
    }
    errx(EXIT_FAILURE, c"No usable outputs available.\n".as_ptr());
}
unsafe fn any_randr_output_active() -> bool {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if output != root_output
            && !(*output).to_be_disabled
            && (*output).active as ::core::ffi::c_int != 0
        {
            return true_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    false_0 != 0
}

pub unsafe fn get_output_containing(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            debuglog(
                c"%s:%s:%d - comparing x=%d y=%d with x=%d and y=%d width %d height %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"get_output_containing".as_ptr(),
                129 as ::core::ffi::c_int,
                x,
                y,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if x as u32 >= (*output).rect.x
                && (x as u32) < (*output).rect.x.wrapping_add((*output).rect.width)
                && y as u32 >= (*output).rect.y
                && (y as u32) < (*output).rect.y.wrapping_add((*output).rect.height)
            {
                return output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ::core::ptr::null_mut::<Output>()
}

pub unsafe fn get_output_from_rect(rect: Rect) -> *mut Output {
    let mid_x: ::core::ffi::c_uint = (rect.x as ::core::ffi::c_uint)
        .wrapping_add((rect.width as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint));
    let mid_y: ::core::ffi::c_uint = (rect.y as ::core::ffi::c_uint)
        .wrapping_add((rect.height as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint));
    let output: *mut Output = get_output_containing(mid_x, mid_y);
    if !output.is_null() {
        output
    } else {
        output_containing_rect(rect)
    }
}

pub unsafe fn get_output_with_dimensions(rect: Rect) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            debuglog(
                c"%s:%s:%d - comparing x=%d y=%d %dx%d with x=%d and y=%d %dx%d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"get_output_with_dimensions".as_ptr(),
                166 as ::core::ffi::c_int,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            if rect.x == (*output).rect.x
                && rect.width == (*output).rect.width
                && rect.y == (*output).rect.y
                && rect.height == (*output).rect.height
            {
                return output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ::core::ptr::null_mut::<Output>()
}

pub unsafe fn output_containing_rect(rect: Rect) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let lx: ::core::ffi::c_int = rect.x as ::core::ffi::c_int;
    let uy: ::core::ffi::c_int = rect.y as ::core::ffi::c_int;
    let rx: ::core::ffi::c_int = rect.x.wrapping_add(rect.width) as ::core::ffi::c_int;
    let by: ::core::ffi::c_int = rect.y.wrapping_add(rect.height) as ::core::ffi::c_int;
    let mut result: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let lx_o: ::core::ffi::c_int = (*output).rect.x as ::core::ffi::c_int;
            let uy_o: ::core::ffi::c_int = (*output).rect.y as ::core::ffi::c_int;
            let rx_o: ::core::ffi::c_int =
                (*output).rect.x.wrapping_add((*output).rect.width) as ::core::ffi::c_int;
            let by_o: ::core::ffi::c_int =
                (*output).rect.y.wrapping_add((*output).rect.height) as ::core::ffi::c_int;
            debuglog(
                c"%s:%s:%d - comparing x=%d y=%d with x=%d and y=%d width %d height %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"output_containing_rect".as_ptr(),
                199 as ::core::ffi::c_int,
                rect.x,
                rect.y,
                (*output).rect.x,
                (*output).rect.y,
                (*output).rect.width,
                (*output).rect.height,
            );
            let left: ::core::ffi::c_int = max(lx, lx_o) as ::core::ffi::c_int;
            let right: ::core::ffi::c_int = min(rx, rx_o) as ::core::ffi::c_int;
            let bottom: ::core::ffi::c_int = min(by, by_o) as ::core::ffi::c_int;
            let top: ::core::ffi::c_int = max(uy, uy_o) as ::core::ffi::c_int;
            if left < right && bottom > top {
                let max_area: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                let area: ::core::ffi::c_long =
                    ((right - left) * (bottom - top)) as ::core::ffi::c_long;
                if area > max_area {
                    result = output;
                }
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    result
}

pub unsafe fn get_output_next_wrap(direction: Direction, current: *mut Output) -> *mut Output {
    let mut best: *mut Output = get_output_next(direction, current, OutputSelection::Closest);
    if best.is_null() {
        let mut opposite: Direction = Direction::Left;
        if direction == Direction::Right {
            opposite = Direction::Left;
        } else if direction == Direction::Left {
            opposite = Direction::Right;
        } else if direction == Direction::Down {
            opposite = Direction::Up;
        } else {
            opposite = Direction::Down;
        }
        best = get_output_next(opposite, current, OutputSelection::Farthest);
    }
    if best.is_null() {
        best = current;
    }
    debuglog(
        c"%s:%s:%d - current = %s, best = %s\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"get_output_next_wrap".as_ptr(),
        244 as ::core::ffi::c_int,
        output_primary_name(current),
        output_primary_name(best),
    );
    best
}

pub unsafe fn get_output_next(
    direction: Direction,
    current: *mut Output,
    close_far: OutputSelection,
) -> *mut Output {
    let cur: *const Rect = &raw mut (*current).rect;
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut best: *mut Output = ::core::ptr::null_mut::<Output>();
    let mut current_block_3: u64;
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let other: *const Rect = &raw mut (*output).rect;
            if direction == Direction::Right && (*other).x > (*cur).x
                || direction == Direction::Left && (*other).x < (*cur).x
            {
                if (*other).y.wrapping_add((*other).height) <= (*cur).y
                    || (*cur).y.wrapping_add((*cur).height) <= (*other).y
                {
                    current_block_3 = 16668937799742929182;
                } else {
                    current_block_3 = 11650488183268122163;
                }
            } else if direction == Direction::Down && (*other).y > (*cur).y
                || direction == Direction::Up && (*other).y < (*cur).y
            {
                if (*other).x.wrapping_add((*other).width) <= (*cur).x
                    || (*cur).x.wrapping_add((*cur).width) <= (*other).x
                {
                    current_block_3 = 16668937799742929182;
                } else {
                    current_block_3 = 11650488183268122163;
                }
            } else {
                current_block_3 = 16668937799742929182;
            }
            match current_block_3 {
                16668937799742929182 => {}
                _ => {
                    if best.is_null() {
                        best = output;
                    } else if close_far == OutputSelection::Closest {
                        if direction == Direction::Right && (*other).x < (*best).rect.x
                            || direction == Direction::Left && (*other).x > (*best).rect.x
                            || direction == Direction::Down && (*other).y < (*best).rect.y
                            || direction == Direction::Up && (*other).y > (*best).rect.y
                        {
                            best = output;
                        }
                    } else if direction == Direction::Right && (*other).x > (*best).rect.x
                        || direction == Direction::Left && (*other).x < (*best).rect.x
                        || direction == Direction::Down && (*other).y > (*best).rect.y
                        || direction == Direction::Up && (*other).y < (*best).rect.y
                    {
                        best = output;
                    }
                }
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    debuglog(
        c"%s:%s:%d - current = %s, best = %s\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"get_output_next".as_ptr(),
        317 as ::core::ffi::c_int,
        output_primary_name(current),
        if !best.is_null() {
            output_primary_name(best) as *const ::core::ffi::c_char
        } else {
            c"NULL".as_ptr()
        },
    );
    best
}

pub unsafe fn create_root_output(conn_0: *mut xcb_connection_t) -> *mut Output {
    let s: *mut Output =
        scalloc(1 as usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
    (*s).active = false_0 != 0;
    (*s).rect.x = 0 as u32;
    (*s).rect.y = 0 as u32;
    (*s).rect.width = (*root_screen).width_in_pixels as u32;
    (*s).rect.height = (*root_screen).height_in_pixels as u32;
    let output_name: *mut output_name =
        scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize) as *mut output_name;
    (*output_name).name = c"xroot-0".as_ptr().cast_mut();
    (*s).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
    (*output_name).names.sle_next = (*s).names_head.slh_first;
    (*s).names_head.slh_first = output_name;
    s
}

pub unsafe fn output_init_con(output: *mut Output) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut reused: bool = false_0 != 0;
    debuglog(
        c"%s:%s:%d - init_con for output %s\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_init_con".as_ptr(),
        351 as ::core::ffi::c_int,
        output_primary_name(output),
    );
    current = (*croot).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if strcmp((*current).name, output_primary_name(output)) != 0 as ::core::ffi::c_int {
            current = (*current).nodes.tqe_next as *mut Con;
        } else {
            con = current;
            reused = true_0 != 0;
            debuglog(
                c"%s:%s:%d - Using existing con %p / %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"output_init_con".as_ptr(),
                362 as ::core::ffi::c_int,
                con,
                (*con).name,
            );
            break;
        }
    }
    if con.is_null() {
        con = con_new(croot, ::core::ptr::null_mut::<i3Window>());
        free((*con).name as *mut ::core::ffi::c_void);
        (*con).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*con).name = sstrdup(output_primary_name(output));
        (*con).type_0 = CT_OUTPUT;
        (*con).layout = Layout::Output;
        con_fix_percent(croot);
    }
    (*con).rect = (*output).rect as Rect;
    (*output).con = con;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(&raw mut name, c"[i3 con] output %s".as_ptr(), (*con).name);
    x_set_name(con, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if reused {
        debuglog(
            c"%s:%s:%d - Not adding workspace, this was a reused con\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"output_init_con".as_ptr(),
            383 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - Changing layout, adding top/bottom dockarea\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_init_con".as_ptr(),
        387 as ::core::ffi::c_int,
    );
    let topdock: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*topdock).type_0 = CT_DOCKAREA;
    (*topdock).layout = Layout::DockArea;
    let mut match_0: *mut Match =
        scalloc(1 as usize, ::core::mem::size_of::<Match>() as usize) as *mut Match;
    match_init(match_0);
    (*match_0).dock = M_DOCK_TOP;
    (*match_0).insert_where = M_BELOW;
    (*match_0).matches.tqe_next = ::core::ptr::null_mut::<Match>();
    (*match_0).matches.tqe_prev = (*topdock).swallow_head.tqh_last;
    *(*topdock).swallow_head.tqh_last = match_0 as *mut Match;
    (*topdock).swallow_head.tqh_last = &raw mut (*match_0).matches.tqe_next;
    free((*topdock).name as *mut ::core::ffi::c_void);
    (*topdock).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*topdock).name = sstrdup(c"topdock".as_ptr());
    sasprintf(
        &raw mut name,
        c"[i3 con] top dockarea %s".as_ptr(),
        (*con).name,
    );
    x_set_name(topdock, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    debuglog(
        c"%s:%s:%d - attaching\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_init_con".as_ptr(),
        404 as ::core::ffi::c_int,
    );
    con_attach(topdock, con, false_0 != 0);
    debuglog(
        c"%s:%s:%d - adding main content container\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_init_con".as_ptr(),
        409 as ::core::ffi::c_int,
    );
    let content: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*content).type_0 = CT_CON;
    (*content).layout = Layout::SplitH;
    free((*content).name as *mut ::core::ffi::c_void);
    (*content).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*content).name = sstrdup(c"content".as_ptr());
    sasprintf(&raw mut name, c"[i3 con] content %s".as_ptr(), (*con).name);
    x_set_name(content, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    con_attach(content, con, false_0 != 0);
    let bottomdock: *mut Con = con_new(
        ::core::ptr::null_mut::<Con>(),
        ::core::ptr::null_mut::<i3Window>(),
    );
    (*bottomdock).type_0 = CT_DOCKAREA;
    (*bottomdock).layout = Layout::DockArea;
    match_0 = scalloc(1 as usize, ::core::mem::size_of::<Match>() as usize) as *mut Match;
    match_init(match_0);
    (*match_0).dock = M_DOCK_BOTTOM;
    (*match_0).insert_where = M_BELOW;
    (*match_0).matches.tqe_next = ::core::ptr::null_mut::<Match>();
    (*match_0).matches.tqe_prev = (*bottomdock).swallow_head.tqh_last;
    *(*bottomdock).swallow_head.tqh_last = match_0 as *mut Match;
    (*bottomdock).swallow_head.tqh_last = &raw mut (*match_0).matches.tqe_next;
    free((*bottomdock).name as *mut ::core::ffi::c_void);
    (*bottomdock).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*bottomdock).name = sstrdup(c"bottomdock".as_ptr());
    sasprintf(
        &raw mut name,
        c"[i3 con] bottom dockarea %s".as_ptr(),
        (*con).name,
    );
    x_set_name(bottomdock, name);
    free(name as *mut ::core::ffi::c_void);
    name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    debuglog(
        c"%s:%s:%d - attaching\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_init_con".as_ptr(),
        438 as ::core::ffi::c_int,
    );
    con_attach(bottomdock, con, false_0 != 0);
    if !(*content).focused.tqe_next.is_null() {
        (*(*content).focused.tqe_next).focused.tqe_prev = (*content).focused.tqe_prev;
    } else {
        (*con).focus_head.tqh_last = (*content).focused.tqe_prev;
    }
    *(*content).focused.tqe_prev = (*content).focused.tqe_next;
    (*content).focused.tqe_next = (*con).focus_head.tqh_first;
    if !(*content).focused.tqe_next.is_null() {
        (*(*con).focus_head.tqh_first).focused.tqe_prev = &raw mut (*content).focused.tqe_next;
    } else {
        (*con).focus_head.tqh_last = &raw mut (*content).focused.tqe_next;
    }
    (*con).focus_head.tqh_first = content as *mut Con;
    (*content).focused.tqe_prev = &raw mut (*con).focus_head.tqh_first;
}

pub unsafe fn init_ws_for_output(output: *mut Output) {
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    let current_block: u64;
    let content: *mut Con = output_get_content((*output).con);
    let previous_focus: *mut Con = con_get_workspace(focused);
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    workspace = all_cons.tqh_first as *mut Con;
    while !workspace.is_null() {
        if !((*workspace).type_0 as ::core::ffi::c_uint
            != CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            || con_is_internal(workspace) as ::core::ffi::c_int != 0)
        {
            let workspace_out: *mut Con =
                get_assigned_output((*workspace).name, (*workspace).num as ::core::ffi::c_long);
            if !((*output).con != workspace_out) {
                debuglog(
                    c"%s:%s:%d - Moving workspace \"%s\" from output \"%s\" to \"%s\" due to assignment\n".as_ptr().cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"init_ws_for_output".as_ptr(),
                    479 as ::core::ffi::c_int,
                    (*workspace).name,
                    output_primary_name(get_output_for_con(workspace)),
                    output_primary_name(output),
                );
                (*content).rect = (*(*output).con).rect;
                workspace_move_to_output(workspace, output);
            }
        }
        workspace = (*workspace).all_cons.tqe_next as *mut Con;
    }
    focused = content;
    if !(*content).nodes_head.tqh_first.is_null() {
        let mut visible: *mut Con = ::core::ptr::null_mut::<Con>();
        let mut child: *mut Con = -(1 as ::core::ffi::c_int) as *mut Con;
        while child == -(1 as ::core::ffi::c_int) as *mut Con && {
            child = ::core::ptr::null_mut::<Con>();
            true_0 != 0
        } {
            child = (*content).nodes_head.tqh_first as *mut Con;
            while !child.is_null() {
                if !((*child).fullscreen_mode as ::core::ffi::c_uint
                    == CF_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    child = (*child).nodes.tqe_next as *mut Con;
                } else {
                    visible = child;
                    break;
                }
            }
        }
        if visible.is_null() {
            visible = (*content).nodes_head.tqh_first as *mut Con;
            workspace_show(visible);
        }
    } else {
        assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
        assignment = ws_assignments.tqh_first;
        loop {
            if assignment.is_null() {
                current_block = 11057878835866523405;
                break;
            }
            if !output_triggers_assignment(output, assignment) {
                assignment = (*assignment).ws_assignments.tqe_next;
            } else {
                verboselog(
                    c"Initializing first assigned workspace \"%s\" for output \"%s\"\n"
                        .as_ptr()
                        .cast_mut(),
                    (*assignment).name,
                    (*assignment).output,
                );
                workspace_show_by_name((*assignment).name);
                current_block = 4825863742504697761;
                break;
            }
        }
        match current_block {
            4825863742504697761 => {}
            _ => {
                debuglog(
                    c"%s:%s:%d - Now adding a workspace\n".as_ptr().cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"init_ws_for_output".as_ptr(),
                    518 as ::core::ffi::c_int,
                );
                workspace_show(create_workspace_on_output(output, content));
            }
        }
    }
    if !previous_focus.is_null() {
        workspace_show(previous_focus);
    }
}
unsafe fn output_change_mode(conn_0: *mut xcb_connection_t, output: *mut Output) {
    debuglog(
        c"%s:%s:%d - Output mode changed, updating rect\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"output_change_mode".as_ptr(),
        539 as ::core::ffi::c_int,
    );
    if (*output).con != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"output->con != NULL".as_ptr(),
            c"../src/randr.c".as_ptr(),
            540 as ::core::ffi::c_uint,
            c"void output_change_mode(xcb_connection_t *, Output *)".as_ptr(),
        );
    };
    (*(*output).con).rect = (*output).rect as Rect;
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let content: *const Con = output_get_content((*output).con);
    workspace = (*content).nodes_head.tqh_first as *mut Con;
    while !workspace.is_null() {
        child = (*workspace).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            floating_fix_coordinates(
                child,
                &raw mut (*workspace).rect,
                &raw mut (*(*output).con).rect,
            );
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
        workspace = (*workspace).nodes.tqe_next as *mut Con;
    }
    if config.default_orientation == Orientation::None {
        workspace = (*content).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            if !(con_num_children(workspace) > 1 as ::core::ffi::c_int) {
                (*workspace).layout = if (*output).rect.height > (*output).rect.width {
                    Layout::SplitV
                } else {
                    Layout::SplitH
                };
                debuglog(
                    c"%s:%s:%d - Setting workspace [%d,%s]'s layout to %d.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"output_change_mode".as_ptr(),
                    568 as ::core::ffi::c_int,
                    (*workspace).num,
                    (*workspace).name,
                    (*workspace).layout as ::core::ffi::c_uint,
                );
                child = (*workspace).nodes_head.tqh_first as *mut Con;
                if !child.is_null() {
                    if (*child).layout == Layout::SplitV || (*child).layout == Layout::SplitH {
                        (*child).layout = (*workspace).layout;
                    }
                    debuglog(
                        c"%s:%s:%d - Setting child [%d,%s]'s layout to %d.\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/randr.c".as_ptr(),
                        c"output_change_mode".as_ptr(),
                        573 as ::core::ffi::c_int,
                        (*child).num,
                        (*child).name,
                        (*child).layout as ::core::ffi::c_uint,
                    );
                }
            }
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
    }
}
unsafe fn randr_query_outputs_15() -> bool {
    if !has_randr_1_5 {
        return false_0 != 0;
    }
    debuglog(
        c"%s:%s:%d - Querying outputs using RandR 1.5\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"randr_query_outputs_15".as_ptr(),
        593 as ::core::ffi::c_int,
    );
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let monitors: *mut xcb_randr_get_monitors_reply_t = xcb_randr_get_monitors_reply(
        conn,
        xcb_randr_get_monitors(conn, root, true_0 as u8),
        &raw mut err,
    );
    if !err.is_null() {
        errorlog(
            c"ERROR: Could not get RandR monitors: X11 error code %d\n"
                .as_ptr()
                .cast_mut(),
            (*err).error_code as ::core::ffi::c_int,
        );
        free(err as *mut ::core::ffi::c_void);
        return false_0 != 0;
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if output != root_output {
            (*output).to_be_disabled = true_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    debuglog(
        c"%s:%s:%d - %d RandR monitors found (timestamp %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"randr_query_outputs_15".as_ptr(),
        616 as ::core::ffi::c_int,
        xcb_randr_get_monitors_monitors_length(monitors),
        (*monitors).timestamp,
    );
    let mut iter: xcb_randr_monitor_info_iterator_t = xcb_randr_monitor_info_iterator_t {
        data: ::core::ptr::null_mut::<xcb_randr_monitor_info_t>(),
        rem: 0,
        index: 0,
    };
    iter = xcb_randr_get_monitors_monitors_iterator(monitors);
    while iter.rem != 0 {
        let monitor_info: *const xcb_randr_monitor_info_t = iter.data;
        let atom_reply: *mut xcb_get_atom_name_reply_t = xcb_get_atom_name_reply(
            conn,
            xcb_get_atom_name(conn, (*monitor_info).name),
            &raw mut err,
        );
        if !err.is_null() {
            errorlog(
                c"ERROR: Could not get RandR monitor name: X11 error code %d\n"
                    .as_ptr()
                    .cast_mut(),
                (*err).error_code as ::core::ffi::c_int,
            );
            free(err as *mut ::core::ffi::c_void);
        } else {
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut name,
                c"%.*s".as_ptr(),
                xcb_get_atom_name_name_length(atom_reply),
                xcb_get_atom_name_name(atom_reply),
            );
            free(atom_reply as *mut ::core::ffi::c_void);
            let mut new: *mut Output = get_output_by_name(name, false_0 != 0);
            if new.is_null() {
                new = scalloc(1 as usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
                (*new).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
                let randr_outputs: *mut xcb_randr_output_t =
                    xcb_randr_monitor_info_outputs(monitor_info);
                let randr_output_len: ::core::ffi::c_int =
                    xcb_randr_monitor_info_outputs_length(monitor_info);
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < randr_output_len {
                    let randr_output: xcb_randr_output_t = *randr_outputs.offset(i as isize);
                    let mut info: *mut xcb_randr_get_output_info_reply_t =
                        xcb_randr_get_output_info_reply(
                            conn,
                            xcb_randr_get_output_info(conn, randr_output, (*monitors).timestamp),
                            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                        );
                    if !info.is_null() && (*info).crtc as ::core::ffi::c_long != XCB_NONE {
                        let mut oname: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut oname,
                            c"%.*s".as_ptr(),
                            xcb_randr_get_output_info_name_length(info),
                            xcb_randr_get_output_info_name(info),
                        );
                        if strcmp(name, oname) != 0 as ::core::ffi::c_int {
                            let output_name: *mut output_name =
                                scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize)
                                    as *mut output_name;
                            (*output_name).name = sstrdup(oname);
                            (*output_name).names.sle_next = (*new).names_head.slh_first;
                            (*new).names_head.slh_first = output_name;
                        }
                        free(oname as *mut ::core::ffi::c_void);
                    }
                    free(info as *mut ::core::ffi::c_void);
                    info = ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
                    i += 1;
                }
                let output_name_0: *mut output_name =
                    scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize)
                        as *mut output_name;
                (*output_name_0).name = sstrdup(name);
                (*output_name_0).names.sle_next = (*new).names_head.slh_first;
                (*new).names_head.slh_first = output_name_0;
                if (*monitor_info).primary != 0 {
                    (*new).outputs.tqe_next = outputs.tqh_first;
                    if !(*new).outputs.tqe_next.is_null() {
                        (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                    } else {
                        outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                    }
                    outputs.tqh_first = new as *mut xoutput;
                    (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
                } else {
                    (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                    (*new).outputs.tqe_prev = outputs.tqh_last;
                    *outputs.tqh_last = new as *mut xoutput;
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
            }
            (*new).active = true_0 != 0;
            (*new).to_be_disabled = false_0 != 0;
            (*new).primary = (*monitor_info).primary != 0;
            let update_x: bool =
                update_if_necessary(&raw mut (*new).rect.x, (*monitor_info).x as u32) as bool;
            let update_y: bool =
                update_if_necessary(&raw mut (*new).rect.y, (*monitor_info).y as u32) as bool;
            let update_w: bool =
                update_if_necessary(&raw mut (*new).rect.width, (*monitor_info).width as u32)
                    as bool;
            let update_h: bool =
                update_if_necessary(&raw mut (*new).rect.height, (*monitor_info).height as u32)
                    as bool;
            (*new).changed = update_x as ::core::ffi::c_int != 0
                || update_y as ::core::ffi::c_int != 0
                || update_w as ::core::ffi::c_int != 0
                || update_h as ::core::ffi::c_int != 0;
            debuglog(
                c"%s:%s:%d - name %s, x %d, y %d, width %d px, height %d px, width %d mm, height %d mm, primary %d, automatic %d\n".as_ptr().cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"randr_query_outputs_15".as_ptr(),
                699 as ::core::ffi::c_int,
                name,
                (*monitor_info).x as ::core::ffi::c_int,
                (*monitor_info).y as ::core::ffi::c_int,
                (*monitor_info).width as ::core::ffi::c_int,
                (*monitor_info).height as ::core::ffi::c_int,
                (*monitor_info).width_in_millimeters,
                (*monitor_info).height_in_millimeters,
                (*monitor_info).primary as ::core::ffi::c_int,
                (*monitor_info).automatic as ::core::ffi::c_int,
            );
            free(name as *mut ::core::ffi::c_void);
        }
        xcb_randr_monitor_info_next(&raw mut iter);
    }
    free(monitors as *mut ::core::ffi::c_void);
    true_0 != 0
}
unsafe fn handle_output(
    conn_0: *mut xcb_connection_t,
    id: xcb_randr_output_t,
    output: *mut xcb_randr_get_output_info_reply_t,
    cts: xcb_timestamp_t,
    res: *mut xcb_randr_get_screen_resources_current_reply_t,
) {
    let mut crtc: *mut xcb_randr_get_crtc_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_crtc_info_reply_t>();
    let mut new: *mut Output = get_output_by_id(id);
    let existing: bool = !new.is_null();
    if !existing {
        new = scalloc(1 as usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
        (*new).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
    }
    (*new).id = id;
    (*new).primary = !primary.is_null() && (*primary).output == id;
    while !(*new).names_head.slh_first.is_null() {
        free((*(*new).names_head.slh_first).name as *mut ::core::ffi::c_void);
        (*(*new).names_head.slh_first).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut old_head: *mut output_name = (*new).names_head.slh_first;
        (*new).names_head.slh_first = (*(*new).names_head.slh_first).names.sle_next;
        free(old_head as *mut ::core::ffi::c_void);
        old_head = ::core::ptr::null_mut::<output_name>();
    }
    let output_name: *mut output_name =
        scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize) as *mut output_name;
    sasprintf(
        &raw mut (*output_name).name,
        c"%.*s".as_ptr(),
        xcb_randr_get_output_info_name_length(output),
        xcb_randr_get_output_info_name(output),
    );
    (*output_name).names.sle_next = (*new).names_head.slh_first;
    (*new).names_head.slh_first = output_name;
    debuglog(
        c"%s:%s:%d - found output with name %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"handle_output".as_ptr(),
        742 as ::core::ffi::c_int,
        output_primary_name(new),
    );
    if (*output).crtc as ::core::ffi::c_long == XCB_NONE {
        if !existing {
            if (*new).primary {
                (*new).outputs.tqe_next = outputs.tqh_first;
                if !(*new).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
                outputs.tqh_first = new as *mut xoutput;
                (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new as *mut xoutput;
                outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
            }
        } else if (*new).active {
            (*new).to_be_disabled = true_0 != 0;
        }
        return;
    }
    let icookie: xcb_randr_get_crtc_info_cookie_t =
        xcb_randr_get_crtc_info(conn_0, (*output).crtc, cts) as xcb_randr_get_crtc_info_cookie_t;
    crtc = xcb_randr_get_crtc_info_reply(
        conn_0,
        icookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if crtc.is_null() {
        debuglog(
            c"%s:%s:%d - Skipping output %s: could not get CRTC (%p)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"handle_output".as_ptr(),
            763 as ::core::ffi::c_int,
            output_primary_name(new),
            crtc,
        );
        return;
    }
    let update_x: bool = update_if_necessary(&raw mut (*new).rect.x, (*crtc).x as u32) as bool;
    let update_y: bool = update_if_necessary(&raw mut (*new).rect.y, (*crtc).y as u32) as bool;
    let update_w: bool =
        update_if_necessary(&raw mut (*new).rect.width, (*crtc).width as u32) as bool;
    let update_h: bool =
        update_if_necessary(&raw mut (*new).rect.height, (*crtc).height as u32) as bool;
    let updated: bool = update_x as ::core::ffi::c_int != 0
        || update_y as ::core::ffi::c_int != 0
        || update_w as ::core::ffi::c_int != 0
        || update_h as ::core::ffi::c_int != 0;
    free(crtc as *mut ::core::ffi::c_void);
    (*new).active = (*new).rect.width != 0 as u32 && (*new).rect.height != 0 as u32;
    if !(*new).active {
        debuglog(
            c"%s:%s:%d - width/height 0/0, disabling output\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"handle_output".as_ptr(),
            775 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - mode: %dx%d+%d+%d\n".as_ptr().cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"handle_output".as_ptr(),
        780 as ::core::ffi::c_int,
        (*new).rect.width,
        (*new).rect.height,
        (*new).rect.x,
        (*new).rect.y,
    );
    if !updated || !existing {
        if !existing {
            if (*new).primary {
                (*new).outputs.tqe_next = outputs.tqh_first;
                if !(*new).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
                }
                outputs.tqh_first = new as *mut xoutput;
                (*new).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new as *mut xoutput;
                outputs.tqh_last = &raw mut (*new).outputs.tqe_next;
            }
        }
        return;
    }
    (*new).changed = true_0 != 0;
}
unsafe fn randr_query_outputs_14() {
    debuglog(
        c"%s:%s:%d - Querying outputs using RandR ≤ 1.4\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"randr_query_outputs_14".as_ptr(),
        804 as ::core::ffi::c_int,
    );
    let rcookie: xcb_randr_get_screen_resources_current_cookie_t =
        xcb_randr_get_screen_resources_current(conn, root)
            as xcb_randr_get_screen_resources_current_cookie_t;
    let pcookie: xcb_randr_get_output_primary_cookie_t =
        xcb_randr_get_output_primary(conn, root) as xcb_randr_get_output_primary_cookie_t;
    primary = xcb_randr_get_output_primary_reply(
        conn,
        pcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if primary.is_null() {
        errorlog(
            c"ERROR: Could not get RandR primary output\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        debuglog(
            c"%s:%s:%d - primary output is %08x\n".as_ptr().cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"randr_query_outputs_14".as_ptr(),
            813 as ::core::ffi::c_int,
            (*primary).output,
        );
    }
    let mut res: *mut xcb_randr_get_screen_resources_current_reply_t =
        xcb_randr_get_screen_resources_current_reply(
            conn,
            rcookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
    if res.is_null() {
        errorlog(
            c"ERROR: Could not query screen resources.\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let cts: xcb_timestamp_t = (*res).config_timestamp;
    let len: ::core::ffi::c_int =
        xcb_randr_get_screen_resources_current_outputs_length(res) as ::core::ffi::c_int;
    let randr_outputs: *mut xcb_randr_output_t =
        xcb_randr_get_screen_resources_current_outputs(res);
    let vla = len as usize;
    let mut ocookie: Vec<xcb_randr_get_output_info_cookie_t> =
        ::std::vec::from_elem(xcb_randr_get_output_info_cookie_t { sequence: 0 }, vla);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        *ocookie.as_mut_ptr().offset(i as isize) =
            xcb_randr_get_output_info(conn, *randr_outputs.offset(i as isize), cts);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < len {
        let mut output: *mut xcb_randr_get_output_info_reply_t =
            ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
        output = xcb_randr_get_output_info_reply(
            conn,
            *ocookie.as_mut_ptr().offset(i_0 as isize),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if !output.is_null() {
            handle_output(conn, *randr_outputs.offset(i_0 as isize), output, cts, res);
            free(output as *mut ::core::ffi::c_void);
        }
        i_0 += 1;
    }
    free(res as *mut ::core::ffi::c_void);
    res = ::core::ptr::null_mut::<xcb_randr_get_screen_resources_current_reply_t>();
}
unsafe fn move_content(con: *mut Con) {
    let first: *mut Con = (*get_first_output()).con;
    let first_content: *mut Con = output_get_content(first);
    let next: *mut Con = focused;
    let old_content: *const Con = output_get_content(con);
    while !(*old_content).nodes_head.tqh_first.is_null() {
        let current: *mut Con = (*old_content).nodes_head.tqh_first as *mut Con;
        if current != next && (*current).focus_head.tqh_first.is_null() {
            debuglog(
                c"%s:%s:%d - Getting rid of current = %p / %s (empty, unfocused)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"move_content".as_ptr(),
                875 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            tree_close_internal(current, KillWindow::DontKill, false_0 != 0);
        } else {
            debuglog(
                c"%s:%s:%d - Detaching current = %p / %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"move_content".as_ptr(),
                879 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            con_detach(current);
            debuglog(
                c"%s:%s:%d - Re-attaching current = %p / %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"move_content".as_ptr(),
                881 as ::core::ffi::c_int,
                current,
                (*current).name,
            );
            con_attach(current, first_content, false_0 != 0);
            debuglog(
                c"%s:%s:%d - Fixing the coordinates of floating containers\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"move_content".as_ptr(),
                883 as ::core::ffi::c_int,
            );
            let mut floating_con: *mut Con = ::core::ptr::null_mut::<Con>();
            floating_con = (*current).floating_head.tqh_first as *mut Con;
            while !floating_con.is_null() {
                floating_fix_coordinates(
                    floating_con,
                    &raw mut (*con).rect,
                    &raw mut (*first).rect,
                );
                floating_con = (*floating_con).floating_windows.tqe_next as *mut Con;
            }
        }
    }
    if !next.is_null() {
        debuglog(
            c"%s:%s:%d - now focusing next = %p\n".as_ptr().cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"move_content".as_ptr(),
            892 as ::core::ffi::c_int,
            next,
        );
        con_focus(next);
        workspace_show(con_get_workspace(next));
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !((*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            debuglog(
                c"%s:%s:%d - Handling dock con %p\n".as_ptr().cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"move_content".as_ptr(),
                903 as ::core::ffi::c_int,
                child,
            );
            while !(*child).nodes_head.tqh_first.is_null() {
                let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
                let dock: *mut Con = (*child).nodes_head.tqh_first as *mut Con;
                let nc: *mut Con =
                    con_for_window(first, (*dock).window as *mut i3Window, &raw mut match_0);
                debuglog(
                    c"%s:%s:%d - Moving dock client %p to nc %p\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"move_content".as_ptr(),
                    908 as ::core::ffi::c_int,
                    dock,
                    nc,
                );
                con_detach(dock);
                debuglog(
                    c"%s:%s:%d - Re-attaching\n".as_ptr().cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"move_content".as_ptr(),
                    910 as ::core::ffi::c_int,
                );
                con_attach(dock, nc, false_0 != 0);
                debuglog(
                    c"%s:%s:%d - Done\n".as_ptr().cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"move_content".as_ptr(),
                    912 as ::core::ffi::c_int,
                );
            }
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    debuglog(
        c"%s:%s:%d - Destroying disappearing con %p\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"move_content".as_ptr(),
        916 as ::core::ffi::c_int,
        con,
    );
    tree_close_internal(con, KillWindow::DontKill, true_0 != 0);
}

pub unsafe fn randr_query_outputs() {
    if !randr_query_outputs_15() {
        randr_query_outputs_14();
    }
    if any_randr_output_active() {
        debuglog(
            c"%s:%s:%d - Active RandR output found. Disabling root output.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"randr_query_outputs".as_ptr(),
            933 as ::core::ffi::c_int,
        );
        if !root_output.is_null() && (*root_output).active as ::core::ffi::c_int != 0 {
            (*root_output).to_be_disabled = true_0 != 0;
        }
    } else {
        debuglog(
            c"%s:%s:%d - No active RandR output found. Enabling root output.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"randr_query_outputs".as_ptr(),
            938 as ::core::ffi::c_int,
        );
        (*root_output).active = true_0 != 0;
    }
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(!(*output).active || (*output).to_be_disabled as ::core::ffi::c_int != 0) {
            debuglog(
                c"%s:%s:%d - output %p / %s, position (%d, %d), checking for clones\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"randr_query_outputs".as_ptr(),
                950 as ::core::ffi::c_int,
                output,
                output_primary_name(output),
                (*output).rect.x,
                (*output).rect.y,
            );
            let mut other: *mut Output = output;
            while !other.is_null() {
                if !(other == output
                    || !(*other).active
                    || (*other).to_be_disabled as ::core::ffi::c_int != 0)
                {
                    if !((*other).rect.x != (*output).rect.x || (*other).rect.y != (*output).rect.y)
                    {
                        debuglog(
                            c"%s:%s:%d - output %p has the same position, its mode = %d x %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/randr.c".as_ptr(),
                            c"randr_query_outputs".as_ptr(),
                            965 as ::core::ffi::c_int,
                            other,
                            (*other).rect.width,
                            (*other).rect.height,
                        );
                        let width: u32 = min(
                            (*other).rect.width as ::core::ffi::c_int,
                            (*output).rect.width as ::core::ffi::c_int,
                        ) as u32;
                        let height: u32 = min(
                            (*other).rect.height as ::core::ffi::c_int,
                            (*output).rect.height as ::core::ffi::c_int,
                        ) as u32;
                        let update_w: bool =
                            update_if_necessary(&raw mut (*output).rect.width, width) as bool;
                        let update_h: bool =
                            update_if_necessary(&raw mut (*output).rect.height, height) as bool;
                        if update_w as ::core::ffi::c_int != 0
                            || update_h as ::core::ffi::c_int != 0
                        {
                            (*output).changed = true_0 != 0;
                        }
                        update_if_necessary(&raw mut (*other).rect.width, width);
                        update_if_necessary(&raw mut (*other).rect.height, height);
                        debuglog(
                            c"%s:%s:%d - disabling output %p (%s)\n".as_ptr().cast_mut(),
                            c"../src/randr.c".as_ptr(),
                            c"randr_query_outputs".as_ptr(),
                            978 as ::core::ffi::c_int,
                            other,
                            output_primary_name(other),
                        );
                        (*other).to_be_disabled = true_0 != 0;
                        debuglog(
                            c"%s:%s:%d - new output mode %d x %d, other mode %d x %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/randr.c".as_ptr(),
                            c"randr_query_outputs".as_ptr(),
                            983 as ::core::ffi::c_int,
                            (*output).rect.width,
                            (*output).rect.height,
                            (*other).rect.width,
                            (*other).rect.height,
                        );
                    }
                }
                other = (*other).outputs.tqe_next as *mut Output;
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active as ::core::ffi::c_int != 0 && (*output).con.is_null() {
            debuglog(
                c"%s:%s:%d - Need to initialize a Con for output %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"randr_query_outputs".as_ptr(),
                993 as ::core::ffi::c_int,
                output_primary_name(output),
            );
            output_init_con(output);
            (*output).changed = false_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    let mut con: *mut Con = (*croot).nodes_head.tqh_first as *mut Con;
    while !con.is_null() {
        let next: *mut Con = (*con).nodes.tqe_next as *mut Con;
        if !con_is_internal(con) && get_output_by_name((*con).name, true_0 != 0).is_null() {
            debuglog(
                c"%s:%s:%d - No output %s found, moving its old content to first output\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"randr_query_outputs".as_ptr(),
                1007 as ::core::ffi::c_int,
                (*con).name,
            );
            move_content(con);
        }
        con = next;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).to_be_disabled {
            randr_disable_output(output);
        }
        if (*output).changed {
            output_change_mode(conn, output);
            (*output).changed = false_0 != 0;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let content: *mut Con = output_get_content((*output).con);
            if (*content).nodes_head.tqh_first.is_null() {
                debuglog(
                    c"%s:%s:%d - Should add ws for output %s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/randr.c".as_ptr(),
                    c"randr_query_outputs".as_ptr(),
                    1035 as ::core::ffi::c_int,
                    output_primary_name(output),
                );
                init_ws_for_output(output);
            }
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if !(!(*output).primary || (*output).con.is_null()) {
            debuglog(
                c"%s:%s:%d - Focusing primary output %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/randr.c".as_ptr(),
                c"randr_query_outputs".as_ptr(),
                1045 as ::core::ffi::c_int,
                output_primary_name(output),
            );
            let content_0: *mut Con = output_get_content((*output).con);
            let ws: *mut Con = (*content_0).focus_head.tqh_first as *mut Con;
            workspace_show(ws);
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ewmh_update_desktop_properties();
    tree_render();
    free(primary as *mut ::core::ffi::c_void);
    primary = ::core::ptr::null_mut::<xcb_randr_get_output_primary_reply_t>();
}

pub unsafe fn randr_disable_output(output: *mut Output) {
    if (*output).to_be_disabled as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"output->to_be_disabled".as_ptr(),
            c"../src/randr.c".as_ptr(),
            1063 as ::core::ffi::c_uint,
            c"void randr_disable_output(Output *)".as_ptr(),
        );
    };
    (*output).active = false_0 != 0;
    debuglog(
        c"%s:%s:%d - Output %s disabled, re-assigning workspaces/docks\n"
            .as_ptr()
            .cast_mut(),
        c"../src/randr.c".as_ptr(),
        c"randr_disable_output".as_ptr(),
        1066 as ::core::ffi::c_int,
        output_primary_name(output),
    );
    if !(*output).con.is_null() {
        let con: *mut Con = (*output).con;
        (*output).con = ::core::ptr::null_mut::<Con>();
        move_content(con);
    }
    (*output).to_be_disabled = false_0 != 0;
    (*output).changed = false_0 != 0;
}
unsafe fn fallback_to_root_output() {
    (*root_output).active = true_0 != 0;
    output_init_con(root_output);
    init_ws_for_output(root_output);
}

pub unsafe fn randr_init(event_base: *mut ::core::ffi::c_int, disable_randr15: bool) {
    root_output = create_root_output(conn);
    (*root_output).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
    (*root_output).outputs.tqe_prev = outputs.tqh_last;
    *outputs.tqh_last = root_output as *mut xoutput;
    outputs.tqh_last = &raw mut (*root_output).outputs.tqe_next;
    let extension_reply: *const xcb_query_extension_reply_t =
        xcb_get_extension_data(conn, &raw mut xcb_randr_id) as *const xcb_query_extension_reply_t;
    if (*extension_reply).present == 0 {
        debuglog(
            c"%s:%s:%d - RandR is not present, activating root output.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/randr.c".as_ptr(),
            c"randr_init".as_ptr(),
            1096 as ::core::ffi::c_int,
        );
        fallback_to_root_output();
        return;
    }
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let randr_version: *mut xcb_randr_query_version_reply_t = xcb_randr_query_version_reply(
        conn,
        xcb_randr_query_version(
            conn,
            XCB_RANDR_MAJOR_VERSION as u32,
            XCB_RANDR_MINOR_VERSION as u32,
        ),
        &raw mut err,
    );
    if !err.is_null() {
        errorlog(
            c"ERROR: Could not query RandR version: X11 error code %d\n"
                .as_ptr()
                .cast_mut(),
            (*err).error_code as ::core::ffi::c_int,
        );
        free(err as *mut ::core::ffi::c_void);
        fallback_to_root_output();
        return;
    }
    has_randr_1_5 = (*randr_version).major_version >= 1 as u32
        && (*randr_version).minor_version >= 5 as u32
        && !disable_randr15;
    free(randr_version as *mut ::core::ffi::c_void);
    randr_query_outputs();
    if !event_base.is_null() {
        *event_base = (*extension_reply).first_event as ::core::ffi::c_int;
    }
    xcb_randr_select_input(
        conn,
        root,
        (XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE as ::core::ffi::c_int
            | XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY as ::core::ffi::c_int) as u16,
    );
    xcb_flush(conn);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    outputs = outputs_head {
        tqh_first: ::core::ptr::null_mut::<xoutput>(),
        tqh_last: &raw mut outputs.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_desktop_properties;
use crate::tree::tree_render;
use crate::util::max;
use crate::util::min;
use crate::util::update_if_necessary;
use crate::workspace::workspace_show_by_name;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_connection_t, xcb_flush, xcb_generic_error_t, xcb_get_atom_name, xcb_get_atom_name_name,
    xcb_get_atom_name_name_length, xcb_get_atom_name_reply, xcb_get_atom_name_reply_t,
    xcb_get_extension_data, xcb_query_extension_reply_t, xcb_randr_get_crtc_info,
    xcb_randr_get_crtc_info_cookie_t, xcb_randr_get_crtc_info_reply,
    xcb_randr_get_crtc_info_reply_t, xcb_randr_get_monitors,
    xcb_randr_get_monitors_monitors_iterator, xcb_randr_get_monitors_monitors_length,
    xcb_randr_get_monitors_reply, xcb_randr_get_monitors_reply_t, xcb_randr_get_output_info,
    xcb_randr_get_output_info_cookie_t, xcb_randr_get_output_info_name,
    xcb_randr_get_output_info_name_length, xcb_randr_get_output_info_reply,
    xcb_randr_get_output_info_reply_t, xcb_randr_get_output_primary,
    xcb_randr_get_output_primary_cookie_t, xcb_randr_get_output_primary_reply,
    xcb_randr_get_output_primary_reply_t, xcb_randr_get_screen_resources_current,
    xcb_randr_get_screen_resources_current_cookie_t,
    xcb_randr_get_screen_resources_current_outputs,
    xcb_randr_get_screen_resources_current_outputs_length,
    xcb_randr_get_screen_resources_current_reply, xcb_randr_get_screen_resources_current_reply_t,
    xcb_randr_id, xcb_randr_monitor_info_iterator_t, xcb_randr_monitor_info_next,
    xcb_randr_monitor_info_outputs, xcb_randr_monitor_info_outputs_length,
    xcb_randr_monitor_info_t, xcb_randr_output_t, xcb_randr_query_version,
    xcb_randr_query_version_reply, xcb_randr_query_version_reply_t, xcb_randr_select_input,
    xcb_timestamp_t, XCB_NONE, XCB_RANDR_MAJOR_VERSION, XCB_RANDR_MINOR_VERSION,
    XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE, XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE,
    XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY, XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE,
};

pub use libc::{free, strcasecmp, strcmp};

use i3_common::native_stdio::{__assert_fail, errx};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::output_name;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::root_screen;
use crate::con::con_attach;
use crate::con::con_detach;
use crate::con::con_fix_percent;
use crate::con::con_focus;
use crate::con::con_for_window;
use crate::con::con_get_workspace;
use crate::con::con_is_internal;
use crate::con::con_new;
use crate::con::con_num_children;
use crate::config::config;
use crate::floating::floating_fix_coordinates;
use crate::output::get_output_for_con;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::r#match::match_init;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::workspace::create_workspace_on_output;
use crate::workspace::get_assigned_output;
use crate::workspace::workspace_move_to_output;
use crate::workspace::workspace_show;
use crate::x::x_set_name;
