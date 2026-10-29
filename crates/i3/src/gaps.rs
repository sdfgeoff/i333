use crate::application::ws_assignments;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * gaps.c: gaps logic: whether to display gaps at all, and how big
 *         they should be.
 *
 */
// Mechanical file-by-file Rust translation of src/gaps.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
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

pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
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

pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;

pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type direction_t = ::core::ffi::c_uint;
pub const D_DOWN: direction_t = 3;
pub const D_UP: direction_t = 2;
pub const D_RIGHT: direction_t = 1;
pub const D_LEFT: direction_t = 0;
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
pub type gaps_mask_t = ::core::ffi::c_uint;
pub const GAPS_OUTER: gaps_mask_t = 30;
pub const GAPS_HORIZONTAL: gaps_mask_t = 20;
pub const GAPS_VERTICAL: gaps_mask_t = 10;
pub const GAPS_LEFT: gaps_mask_t = 16;
pub const GAPS_BOTTOM: gaps_mask_t = 8;
pub const GAPS_RIGHT: gaps_mask_t = 4;
pub const GAPS_TOP: gaps_mask_t = 2;
pub const GAPS_INNER: gaps_mask_t = 1;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;
pub use crate::application::Workspace_Assignment;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_20 = 3;
pub const PDF_IGNORE: C2RustUnnamed_20 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_20 = 1;
pub const PDF_SMART: C2RustUnnamed_20 = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_21 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_21 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_22 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_22 = 2;
pub const FOWA_URGENT: C2RustUnnamed_22 = 1;
pub const FOWA_SMART: C2RustUnnamed_22 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn calculate_effective_gaps(con: *mut Con) -> gaps_t {
    let workspace: *mut Con = con_get_workspace(con);
    if workspace.is_null() {
        return gaps_t {
            inner: 0 as ::core::ffi::c_int,
            top: 0 as ::core::ffi::c_int,
            right: 0 as ::core::ffi::c_int,
            bottom: 0 as ::core::ffi::c_int,
            left: 0 as ::core::ffi::c_int,
        };
    }
    let one_child: bool = con_num_visible_children(workspace) <= 1 as ::core::ffi::c_int
        || con_num_children(workspace) == 1 as ::core::ffi::c_int
            && ((*(*workspace).nodes_head.tqh_first).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*workspace).nodes_head.tqh_first).layout as ::core::ffi::c_uint
                    == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint);
    if config.smart_gaps as ::core::ffi::c_uint
        == SMART_GAPS_ON as ::core::ffi::c_int as ::core::ffi::c_uint
        && one_child as ::core::ffi::c_int != 0
    {
        return gaps_t {
            inner: 0 as ::core::ffi::c_int,
            top: 0 as ::core::ffi::c_int,
            right: 0 as ::core::ffi::c_int,
            bottom: 0 as ::core::ffi::c_int,
            left: 0 as ::core::ffi::c_int,
        };
    }
    let mut gaps: gaps_t = gaps_t {
        inner: (*workspace).gaps.inner + config.gaps.inner,
        top: 0 as ::core::ffi::c_int,
        right: 0 as ::core::ffi::c_int,
        bottom: 0 as ::core::ffi::c_int,
        left: 0 as ::core::ffi::c_int,
    };
    if config.smart_gaps as ::core::ffi::c_uint
        != SMART_GAPS_INVERSE_OUTER as ::core::ffi::c_int as ::core::ffi::c_uint
        || one_child as ::core::ffi::c_int != 0
    {
        gaps.top = (*workspace).gaps.top + config.gaps.top;
        gaps.right = (*workspace).gaps.right + config.gaps.right;
        gaps.bottom = (*workspace).gaps.bottom + config.gaps.bottom;
        gaps.left = (*workspace).gaps.left + config.gaps.left;
    }
    gaps
}

pub unsafe fn gaps_should_inset_con(con: *mut Con, children: ::core::ffi::c_int) -> bool {
    if (*con).parent.is_null() {
        return false_0 != 0;
    }
    if !con_inside_floating(con).is_null() {
        return false_0 != 0;
    }
    let leaf_or_stacked_tabbed: bool = con_is_leaf(con) as ::core::ffi::c_int != 0
        || ((*con).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint);
    if leaf_or_stacked_tabbed as ::core::ffi::c_int != 0
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    if leaf_or_stacked_tabbed as ::core::ffi::c_int != 0
        && !con_inside_stacked_or_tabbed(con)
        && (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        && ((*(*con).parent).layout as ::core::ffi::c_uint
            == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*con).parent).layout as ::core::ffi::c_uint
                == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return true_0 != 0;
    }
    false_0 != 0
}

pub unsafe fn gaps_has_adjacent_container(con: *mut Con, direction: direction_t) -> bool {
    let workspace: *mut Con = con_get_workspace(con);
    let mut fullscreen: *mut Con = con_get_fullscreen_con(workspace, CF_GLOBAL);
    if fullscreen.is_null() {
        fullscreen = con_get_fullscreen_con(workspace, CF_OUTPUT);
    }
    if con == fullscreen {
        return false_0 != 0;
    }
    let mut first: *mut Con = con;
    let mut second: *mut Con = ::core::ptr::null_mut::<Con>();
    let found_neighbor: bool =
        resize_find_tiling_participants(&raw mut first, &raw mut second, direction, false_0 != 0);
    if !found_neighbor {
        return false_0 != 0;
    }
    if fullscreen.is_null() {
        return true_0 != 0;
    }
    con_has_parent(con, fullscreen) as ::core::ffi::c_int != 0
        && con_has_parent(second, fullscreen) as ::core::ffi::c_int != 0
}

pub unsafe fn gaps_for_workspace(ws: *mut Con) -> gaps_t {
    let mut gaps: gaps_t = gaps_t {
        inner: 0 as ::core::ffi::c_int,
        top: 0 as ::core::ffi::c_int,
        right: 0 as ::core::ffi::c_int,
        bottom: 0 as ::core::ffi::c_int,
        left: 0 as ::core::ffi::c_int,
    };
    let mut mask: gaps_mask_t = 0 as gaps_mask_t;
    let mut assignment: *mut Workspace_Assignment = ::core::ptr::null_mut::<Workspace_Assignment>();
    assignment = ws_assignments.tqh_first;
    while !assignment.is_null() {
        if strcmp((*assignment).name, (*ws).name) == 0 as ::core::ffi::c_int {
            gaps = (*assignment).gaps;
            mask = (*assignment).gaps_mask;
            break;
        } else {
            if (*ws).num != -(1 as ::core::ffi::c_int)
                && name_is_digits((*assignment).name) as ::core::ffi::c_int != 0
                && ws_name_to_number((*assignment).name) == (*ws).num
            {
                gaps = (*assignment).gaps;
                mask = (*assignment).gaps_mask;
            }
            assignment = (*assignment).ws_assignments.tqe_next;
        }
    }
    if mask as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        return gaps;
    }
    if mask as ::core::ffi::c_uint & GAPS_INNER as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.inner -= config.gaps.inner;
    }
    if mask as ::core::ffi::c_uint & GAPS_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.top -= config.gaps.top;
    }
    if mask as ::core::ffi::c_uint & GAPS_RIGHT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.right -= config.gaps.right;
    }
    if mask as ::core::ffi::c_uint & GAPS_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.bottom -= config.gaps.bottom;
    }
    if mask as ::core::ffi::c_uint & GAPS_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        gaps.left -= config.gaps.left;
    }
    gaps
}

pub unsafe fn gaps_reapply_workspace_assignments() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let content: *mut Con = output_get_content(output);
        workspace = (*content).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            debuglog(
                c"%s:%s:%d - updating gap assignments for workspace %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/gaps.c".as_ptr(),
                c"gaps_reapply_workspace_assignments".as_ptr(),
                171 as ::core::ffi::c_int,
                (*workspace).name,
            );
            (*workspace).gaps = gaps_for_workspace(workspace);
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::name_is_digits;
use crate::util::ws_name_to_number;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::strcmp;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::gaps_t;
pub use crate::types::Con;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_workspace;
use crate::con::con_has_parent;
use crate::con::con_inside_floating;
use crate::con::con_inside_stacked_or_tabbed;
use crate::con::con_is_leaf;
use crate::con::con_num_children;
use crate::con::con_num_visible_children;
use crate::config::config;
use crate::output::output_get_content;
use crate::resize::resize_find_tiling_participants;
use crate::tree::croot;
