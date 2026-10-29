use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::JsonContent;
use crate::types::Layout;
use crate::types::MarkMode;
use crate::types::ScratchpadState;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * load_layout.c: Restore (parts of) the layout, for example after an inplace
 *                restart.
 *
 */
// Mechanical file-by-file Rust translation of src/load_layout.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{
    yajl_alloc, yajl_complete_parse, yajl_config, yajl_free, yajl_free_error, yajl_get_error,
    yajl_parse,
};
pub use i3_json::compat::{
    yajl_alloc_funcs, yajl_allow_comments, yajl_allow_multiple_values, yajl_callbacks,
    yajl_dont_validate_strings, yajl_handle, yajl_status, yajl_status_client_canceled,
    yajl_status_ok,
};

pub type i3String = _i3String;

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
pub type i3Window = Window;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_mapping {
    pub old_id: ::core::ffi::c_int,
    pub focus_mappings: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut focus_mapping,
    pub tqe_prev: *mut *mut focus_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct focus_mappings_head {
    pub tqh_first: *mut focus_mapping,
    pub tqh_last: *mut *mut focus_mapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pending_marks {
    pub mark: *mut ::core::ffi::c_char,
    pub con_to_be_marked: *mut Con,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;
static mut last_key: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut incomplete: ::core::ffi::c_int = 0;
static mut json_node: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
static mut to_focus: *mut Con = ::core::ptr::null::<Con>() as *mut Con;
static mut parsing_gaps: bool = false;
static mut parsing_swallows: bool = false;
static mut parsing_rect: bool = false;
static mut parsing_actual_deco_rect: bool = false;
static mut parsing_deco_rect: bool = false;
static mut parsing_window_rect: bool = false;
static mut parsing_geometry: bool = false;
static mut parsing_focus: bool = false;
static mut parsing_marks: bool = false;

pub static mut current_swallow: *mut Match = ::core::ptr::null::<Match>() as *mut Match;
static mut swallow_is_empty: bool = false;
static mut num_marks: ::core::ffi::c_int = 0;

pub static mut marks: *mut pending_marks =
    ::core::ptr::null::<pending_marks>() as *mut pending_marks;
static mut focus_mappings: focus_mappings_head = focus_mappings_head {
    tqh_first: ::core::ptr::null::<focus_mapping>() as *mut focus_mapping,
    tqh_last: ::core::ptr::null::<*mut focus_mapping>() as *mut *mut focus_mapping,
};
unsafe fn json_start_map(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(
        c"start of map, last_key = %s\n".as_ptr().cast_mut(),
        last_key,
    );
    if parsing_swallows {
        verboselog(c"creating new swallow\n".as_ptr().cast_mut());
        current_swallow = smalloc(::core::mem::size_of::<Match>() as usize) as *mut Match;
        match_init(current_swallow);
        (*current_swallow).dock = M_DONTCHECK;
        (*current_swallow).matches.tqe_next = ::core::ptr::null_mut::<Match>();
        (*current_swallow).matches.tqe_prev = (*json_node).swallow_head.tqh_last;
        *(*json_node).swallow_head.tqh_last = current_swallow as *mut Match;
        (*json_node).swallow_head.tqh_last = &raw mut (*current_swallow).matches.tqe_next;
        swallow_is_empty = true_0 != 0;
    } else if !parsing_rect
        && !parsing_actual_deco_rect
        && !parsing_deco_rect
        && !parsing_window_rect
        && !parsing_geometry
        && !parsing_gaps
    {
        if !last_key.is_null()
            && strcasecmp(last_key, c"floating_nodes".as_ptr()) == 0 as ::core::ffi::c_int
        {
            debuglog(
                c"%s:%s:%d - New floating_node\n".as_ptr().cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_start_map".as_ptr(),
                72 as ::core::ffi::c_int,
            );
            let ws: *mut Con = con_get_workspace(json_node);
            json_node = con_new_skeleton(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*json_node).parent = ws as *mut Con;
            debuglog(
                c"%s:%s:%d - Parent is workspace = %p\n".as_ptr().cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_start_map".as_ptr(),
                77 as ::core::ffi::c_int,
                ws,
            );
        } else {
            let parent: *mut Con = json_node;
            json_node = con_new_skeleton(
                ::core::ptr::null_mut::<Con>(),
                ::core::ptr::null_mut::<i3Window>(),
            );
            (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*json_node).parent = parent as *mut Con;
        }
        incomplete += 1;
        debuglog(
            c"%s:%s:%d - incomplete = %d\n".as_ptr().cast_mut(),
            c"../src/load_layout.c".as_ptr(),
            c"json_start_map".as_ptr(),
            86 as ::core::ffi::c_int,
            incomplete,
        );
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_end_map(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(c"end of map\n".as_ptr().cast_mut());
    if !parsing_swallows
        && !parsing_rect
        && !parsing_actual_deco_rect
        && !parsing_deco_rect
        && !parsing_window_rect
        && !parsing_geometry
        && !parsing_gaps
    {
        if (*json_node).layout == Layout::Default {
            debuglog(
                c"%s:%s:%d - Setting layout = L_SPLITH\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_end_map".as_ptr(),
                103 as ::core::ffi::c_int,
            );
            (*json_node).layout = Layout::SplitH;
        }
        if con_is_split(json_node) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && !(*json_node).swallow_head.tqh_first.is_null()
        {
            debuglog(
                c"%s:%s:%d - sanity check: removing swallows specification from split container\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_end_map".as_ptr(),
                110 as ::core::ffi::c_int,
            );
            while !(*json_node).swallow_head.tqh_first.is_null() {
                let match_0: *mut Match = (*json_node).swallow_head.tqh_first as *mut Match;
                if !(*match_0).matches.tqe_next.is_null() {
                    (*(*match_0).matches.tqe_next).matches.tqe_prev = (*match_0).matches.tqe_prev;
                } else {
                    (*json_node).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                }
                *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                match_free(match_0);
                free(match_0 as *mut ::core::ffi::c_void);
            }
        }
        if (*json_node).type_0 as ::core::ffi::c_uint
            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - Attaching workspace. name = %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_end_map".as_ptr(),
                121 as ::core::ffi::c_int,
                (*json_node).name,
            );
            if (*json_node).name.is_null()
                || strcmp((*json_node).name, c"".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*json_node).name = sstrdup(c"unnamed".as_ptr());
            }
            let base: *mut ::core::ffi::c_char = sstrdup((*json_node).name);
            let mut cnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while !get_existing_workspace_by_name((*json_node).name).is_null() {
                free((*json_node).name as *mut ::core::ffi::c_void);
                (*json_node).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let fresh1 = cnt;
                cnt = cnt + 1;
                sasprintf(&raw mut (*json_node).name, c"%s_%d".as_ptr(), base, fresh1);
            }
            free(base as *mut ::core::ffi::c_void);
            (*json_node).num = ws_name_to_number((*json_node).name);
        }
        if (*json_node).type_0 as ::core::ffi::c_uint
            == CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - fixing parent which currently is %p / %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"json_end_map".as_ptr(),
                148 as ::core::ffi::c_int,
                (*json_node).parent,
                (*(*json_node).parent).name,
            );
            (*json_node).parent = con_get_workspace((*json_node).parent as *mut Con) as *mut Con;
            if rect_equals(
                (*json_node).rect as Rect,
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
                    c"../src/load_layout.c".as_ptr(),
                    c"json_end_map".as_ptr(),
                    155 as ::core::ffi::c_int,
                );
                let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
                child = (*json_node).nodes_head.tqh_first as *mut Con;
                while !child.is_null() {
                    debuglog(
                        c"%s:%s:%d - child geometry: %d x %d\n".as_ptr().cast_mut(),
                        c"../src/load_layout.c".as_ptr(),
                        c"json_end_map".as_ptr(),
                        158 as ::core::ffi::c_int,
                        (*child).geometry.width,
                        (*child).geometry.height,
                    );
                    (*json_node).rect.width = (*json_node)
                        .rect
                        .width
                        .wrapping_add((*child).geometry.width);
                    (*json_node).rect.height = max(
                        (*json_node).rect.height as ::core::ffi::c_int,
                        (*child).geometry.height as ::core::ffi::c_int,
                    ) as u32;
                    child = (*child).nodes.tqe_next as *mut Con;
                }
            }
            floating_check_size(json_node, false_0 != 0);
        }
        if num_marks > 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < num_marks {
                let con: *mut Con = (*marks.offset(i as isize)).con_to_be_marked;
                let mark: *mut ::core::ffi::c_char = (*marks.offset(i as isize)).mark;
                con_mark(con, mark, MarkMode::Add);
                free(mark as *mut ::core::ffi::c_void);
                i += 1;
            }
            free(marks as *mut ::core::ffi::c_void);
            marks = ::core::ptr::null_mut::<pending_marks>();
            num_marks = 0 as ::core::ffi::c_int;
        }
        verboselog(c"attaching\n".as_ptr().cast_mut());
        con_attach(json_node, (*json_node).parent as *mut Con, true_0 != 0);
        verboselog(c"Creating window\n".as_ptr().cast_mut());
        x_con_init(json_node);
        let old_floating_mode: FloatingState = (*json_node).floating;
        if (old_floating_mode).is_floating()
            && (*(*json_node).parent).type_0 as ::core::ffi::c_uint
                != CT_FLOATING_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            verboselog(
                c"Fixing floating node without CT_FLOATING_CON parent\n"
                    .as_ptr()
                    .cast_mut(),
            );
            (*json_node).floating = FloatingState::AutoOff;
            floating_enable(json_node, false_0 != 0);
            (*json_node).floating = old_floating_mode;
        }
        json_node = (*json_node).parent as *mut Con;
        incomplete -= 1;
        debuglog(
            c"%s:%s:%d - incomplete = %d\n".as_ptr().cast_mut(),
            c"../src/load_layout.c".as_ptr(),
            c"json_end_map".as_ptr(),
            198 as ::core::ffi::c_int,
            incomplete,
        );
    }
    if parsing_swallows as ::core::ffi::c_int != 0 && swallow_is_empty as ::core::ffi::c_int != 0 {
        errorlog(
            c"ERROR: Layout file is invalid: found an empty swallow definition.\n"
                .as_ptr()
                .cast_mut(),
        );
        return 0 as ::core::ffi::c_int;
    }
    parsing_gaps = false_0 != 0;
    parsing_rect = false_0 != 0;
    parsing_actual_deco_rect = false_0 != 0;
    parsing_deco_rect = false_0 != 0;
    parsing_window_rect = false_0 != 0;
    parsing_geometry = false_0 != 0;
    1 as ::core::ffi::c_int
}
unsafe fn json_end_array(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    verboselog(c"end of array\n".as_ptr().cast_mut());
    if !parsing_swallows && !parsing_focus && !parsing_marks {
        con_fix_percent(json_node);
    }
    if parsing_swallows {
        parsing_swallows = false_0 != 0;
    }
    if parsing_marks {
        parsing_marks = false_0 != 0;
    }
    if parsing_focus {
        let mut mapping: *mut focus_mapping = ::core::ptr::null_mut::<focus_mapping>();
        mapping = *(*(focus_mappings.tqh_last as *mut focus_mappings_head)).tqh_last;
        while !mapping.is_null() {
            verboselog(
                c"focus (reverse) %d\n".as_ptr().cast_mut(),
                (*mapping).old_id,
            );
            let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
            con = (*json_node).focus_head.tqh_first as *mut Con;
            while !con.is_null() {
                if (*con).old_id != (*mapping).old_id {
                    con = (*con).focused.tqe_next as *mut Con;
                } else {
                    verboselog(c"got it! %p\n".as_ptr().cast_mut(), con);
                    if !(*con).focused.tqe_next.is_null() {
                        (*(*con).focused.tqe_next).focused.tqe_prev = (*con).focused.tqe_prev;
                    } else {
                        (*json_node).focus_head.tqh_last = (*con).focused.tqe_prev;
                    }
                    *(*con).focused.tqe_prev = (*con).focused.tqe_next;
                    (*con).focused.tqe_next = (*json_node).focus_head.tqh_first;
                    if !(*con).focused.tqe_next.is_null() {
                        (*(*json_node).focus_head.tqh_first).focused.tqe_prev =
                            &raw mut (*con).focused.tqe_next;
                    } else {
                        (*json_node).focus_head.tqh_last = &raw mut (*con).focused.tqe_next;
                    }
                    (*json_node).focus_head.tqh_first = con as *mut Con;
                    (*con).focused.tqe_prev = &raw mut (*json_node).focus_head.tqh_first;
                    break;
                }
            }
            mapping = *(*((*mapping).focus_mappings.tqe_prev as *mut focus_mappings_head)).tqh_last;
        }
        while !focus_mappings.tqh_first.is_null() {
            mapping = focus_mappings.tqh_first;
            if !(*mapping).focus_mappings.tqe_next.is_null() {
                (*(*mapping).focus_mappings.tqe_next)
                    .focus_mappings
                    .tqe_prev = (*mapping).focus_mappings.tqe_prev;
            } else {
                focus_mappings.tqh_last = (*mapping).focus_mappings.tqe_prev;
            }
            *(*mapping).focus_mappings.tqe_prev = (*mapping).focus_mappings.tqe_next;
            free(mapping as *mut ::core::ffi::c_void);
        }
        parsing_focus = false_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_key(
    ctx: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    verboselog(
        c"key: %.*s\n".as_ptr().cast_mut(),
        len as ::core::ffi::c_int,
        val,
    );
    free(last_key as *mut ::core::ffi::c_void);
    last_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    last_key = scalloc(len.wrapping_add(1 as usize), 1 as usize) as *mut ::core::ffi::c_char;
    memcpy(
        last_key as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        len,
    );
    if strcasecmp(last_key, c"swallows".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_swallows = true_0 != 0;
    }
    if strcasecmp(last_key, c"gaps".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_gaps = true_0 != 0;
    }
    if strcasecmp(last_key, c"rect".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_rect = true_0 != 0;
    }
    if strcasecmp(last_key, c"actual_deco_rect".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_actual_deco_rect = true_0 != 0;
    }
    if strcasecmp(last_key, c"deco_rect".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_deco_rect = true_0 != 0;
    }
    if strcasecmp(last_key, c"window_rect".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_window_rect = true_0 != 0;
    }
    if strcasecmp(last_key, c"geometry".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_geometry = true_0 != 0;
    }
    if strcasecmp(last_key, c"focus".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_focus = true_0 != 0;
    }
    if strcasecmp(last_key, c"marks".as_ptr()) == 0 as ::core::ffi::c_int {
        num_marks = 0 as ::core::ffi::c_int;
        parsing_marks = true_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_string(
    ctx: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    verboselog(
        c"string: %.*s for key %s\n".as_ptr().cast_mut(),
        len as ::core::ffi::c_int,
        val,
        last_key,
    );
    if parsing_swallows {
        let mut sval: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut sval,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(last_key, c"class".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).class = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(last_key, c"instance".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).instance = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(last_key, c"window_role".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).window_role = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(last_key, c"title".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).title = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else if strcasecmp(last_key, c"machine".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).machine = regex_new(sval);
            swallow_is_empty = false_0 != 0;
        } else {
            errorlog(
                c"ERROR: swallow key %s unknown\n".as_ptr().cast_mut(),
                last_key,
            );
        }
        free(sval as *mut ::core::ffi::c_void);
    } else if parsing_marks {
        let mut mark: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut mark,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        num_marks += 1;
        marks = srealloc(
            marks as *mut ::core::ffi::c_void,
            (num_marks as usize).wrapping_mul(::core::mem::size_of::<pending_marks>() as usize),
        ) as *mut pending_marks;
        let ref mut fresh2 = (*marks.offset((num_marks - 1 as ::core::ffi::c_int) as isize)).mark;
        *fresh2 = mark;
        let ref mut fresh3 =
            (*marks.offset((num_marks - 1 as ::core::ffi::c_int) as isize)).con_to_be_marked;
        *fresh3 = json_node;
    } else if strcasecmp(last_key, c"name".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).name =
            scalloc(len.wrapping_add(1 as usize), 1 as usize) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).name as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
    } else if strcasecmp(last_key, c"title_format".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).title_format =
            scalloc(len.wrapping_add(1 as usize), 1 as usize) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).title_format as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
    } else if strcasecmp(last_key, c"sticky_group".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).sticky_group =
            scalloc(len.wrapping_add(1 as usize), 1 as usize) as *mut ::core::ffi::c_char;
        memcpy(
            (*json_node).sticky_group as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
        verboselog(
            c"sticky_group of this container is %s\n"
                .as_ptr()
                .cast_mut(),
            (*json_node).sticky_group,
        );
    } else if strcasecmp(last_key, c"orientation".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf, c"none".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(buf, c"horizontal".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*json_node).last_split_layout = Layout::SplitH;
        } else if strcasecmp(buf, c"vertical".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).last_split_layout = Layout::SplitV;
        } else {
            verboselog(c"Unhandled orientation: %s\n".as_ptr().cast_mut(), buf);
        }
        free(buf as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"border".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_0,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_0, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).border_style = BorderStyle::None;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else if strcasecmp(buf_0, c"1pixel".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).border_style = BorderStyle::Pixel;
            (*json_node).max_user_border_style = (*json_node).border_style;
            (*json_node).current_border_width = 1 as ::core::ffi::c_int;
        } else if strcasecmp(buf_0, c"pixel".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).border_style = BorderStyle::Pixel;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else if strcasecmp(buf_0, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).border_style = BorderStyle::Normal;
            (*json_node).max_user_border_style = (*json_node).border_style;
        } else {
            verboselog(c"Unhandled \"border\": %s\n".as_ptr().cast_mut(), buf_0);
        }
        free(buf_0 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"type".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_1,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_1, c"root".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_ROOT;
        } else if strcasecmp(buf_1, c"output".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_OUTPUT;
        } else if strcasecmp(buf_1, c"con".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_CON;
        } else if strcasecmp(buf_1, c"floating_con".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_FLOATING_CON;
        } else if strcasecmp(buf_1, c"workspace".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_WORKSPACE;
        } else if strcasecmp(buf_1, c"dockarea".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).type_0 = CT_DOCKAREA;
        } else {
            verboselog(c"Unhandled \"type\": %s\n".as_ptr().cast_mut(), buf_1);
        }
        free(buf_1 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"layout".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_2,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_2, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = (*json_node).last_split_layout;
        } else if strcasecmp(buf_2, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::Stacked;
        } else if strcasecmp(buf_2, c"tabbed".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::Tabbed;
        } else if strcasecmp(buf_2, c"dockarea".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::DockArea;
        } else if strcasecmp(buf_2, c"output".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::Output;
        } else if strcasecmp(buf_2, c"splith".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::SplitH;
        } else if strcasecmp(buf_2, c"splitv".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).layout = Layout::SplitV;
        } else {
            verboselog(c"Unhandled \"layout\": %s\n".as_ptr().cast_mut(), buf_2);
        }
        free(buf_2 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"workspace_layout".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_3,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_3, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).workspace_layout = Layout::Default;
        } else if strcasecmp(buf_3, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).workspace_layout = Layout::Stacked;
        } else if strcasecmp(buf_3, c"tabbed".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).workspace_layout = Layout::Tabbed;
        } else {
            verboselog(
                c"Unhandled \"workspace_layout\": %s\n".as_ptr().cast_mut(),
                buf_3,
            );
        }
        free(buf_3 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"last_split_layout".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_4,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_4, c"splith".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).last_split_layout = Layout::SplitH;
        } else if strcasecmp(buf_4, c"splitv".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).last_split_layout = Layout::SplitV;
        } else {
            verboselog(
                c"Unhandled \"last_splitlayout\": %s\n".as_ptr().cast_mut(),
                buf_4,
            );
        }
        free(buf_4 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"mark".as_ptr()) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - Found deprecated key \"mark\".\n"
                .as_ptr()
                .cast_mut(),
            c"../src/load_layout.c".as_ptr(),
            c"json_string".as_ptr(),
            443 as ::core::ffi::c_int,
        );
        let mut buf_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_5,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        con_mark(json_node, buf_5, MarkMode::Replace);
    } else if strcasecmp(last_key, c"floating".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_6,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_6, c"auto_off".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).floating = FloatingState::AutoOff;
        } else if strcasecmp(buf_6, c"auto_on".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).floating = FloatingState::AutoOn;
        } else if strcasecmp(buf_6, c"user_off".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).floating = FloatingState::UserOff;
        } else if strcasecmp(buf_6, c"user_on".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).floating = FloatingState::UserOn;
        }
        free(buf_6 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"scratchpad_state".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut buf_7: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut buf_7,
            c"%.*s".as_ptr(),
            len as ::core::ffi::c_int,
            val,
        );
        if strcasecmp(buf_7, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).scratchpad_state = ScratchpadState::None;
        } else if strcasecmp(buf_7, c"fresh".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).scratchpad_state = ScratchpadState::Fresh;
        } else if strcasecmp(buf_7, c"changed".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).scratchpad_state = ScratchpadState::Changed;
        }
        free(buf_7 as *mut ::core::ffi::c_void);
    } else if strcasecmp(last_key, c"previous_workspace_name".as_ptr()) == 0 as ::core::ffi::c_int {
        free(previous_workspace_name as *mut ::core::ffi::c_void);
        previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        previous_workspace_name = sstrndup(val as *const ::core::ffi::c_char, len);
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_int(
    ctx: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    verboselog(c"int %lld for key %s\n".as_ptr().cast_mut(), val, last_key);
    if strcasecmp(last_key, c"type".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).type_0 = val as ::core::ffi::c_uint;
    }
    if strcasecmp(last_key, c"fullscreen_mode".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).fullscreen_mode = val as fullscreen_mode_t;
    }
    if strcasecmp(last_key, c"num".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).num = val as ::core::ffi::c_int;
    }
    if strcasecmp(last_key, c"current_border_width".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).current_border_width = val as ::core::ffi::c_int;
    }
    if strcasecmp(last_key, c"window_icon_padding".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).window_icon_padding = val as ::core::ffi::c_int;
    }
    if strcasecmp(last_key, c"depth".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).depth = val as u16;
    }
    if !parsing_swallows && strcasecmp(last_key, c"id".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).old_id = val as ::core::ffi::c_int;
    }
    if parsing_focus {
        let focus_mapping: *mut focus_mapping =
            scalloc(1 as usize, ::core::mem::size_of::<focus_mapping>() as usize)
                as *mut focus_mapping;
        (*focus_mapping).old_id = val as ::core::ffi::c_int;
        (*focus_mapping).focus_mappings.tqe_next = ::core::ptr::null_mut::<focus_mapping>();
        (*focus_mapping).focus_mappings.tqe_prev = focus_mappings.tqh_last;
        *focus_mappings.tqh_last = focus_mapping;
        focus_mappings.tqh_last = &raw mut (*focus_mapping).focus_mappings.tqe_next;
    }
    if parsing_rect as ::core::ffi::c_int != 0
        || parsing_window_rect as ::core::ffi::c_int != 0
        || parsing_geometry as ::core::ffi::c_int != 0
    {
        let mut r: *mut Rect = ::core::ptr::null_mut::<Rect>();
        if parsing_rect {
            r = &raw mut (*json_node).rect as *mut Rect;
        } else if parsing_window_rect {
            r = &raw mut (*json_node).window_rect as *mut Rect;
        } else {
            r = &raw mut (*json_node).geometry as *mut Rect;
        }
        if strcasecmp(last_key, c"x".as_ptr()) == 0 as ::core::ffi::c_int {
            (*r).x = val as u32;
        } else if strcasecmp(last_key, c"y".as_ptr()) == 0 as ::core::ffi::c_int {
            (*r).y = val as u32;
        } else if strcasecmp(last_key, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
            (*r).width = val as u32;
        } else if strcasecmp(last_key, c"height".as_ptr()) == 0 as ::core::ffi::c_int {
            (*r).height = val as u32;
        } else {
            errorlog(
                c"ERROR: WARNING: unknown key %s in rect\n"
                    .as_ptr()
                    .cast_mut(),
                last_key,
            );
        }
        debuglog(
            c"%s:%s:%d - rect now: (%d, %d, %d, %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/load_layout.c".as_ptr(),
            c"json_int".as_ptr(),
            539 as ::core::ffi::c_int,
            (*r).x,
            (*r).y,
            (*r).width,
            (*r).height,
        );
    }
    if parsing_swallows {
        if strcasecmp(last_key, c"id".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).id = val as xcb_window_t;
            swallow_is_empty = false_0 != 0;
        }
        if strcasecmp(last_key, c"dock".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).dock = val as ::core::ffi::c_int;
            swallow_is_empty = false_0 != 0;
        }
        if strcasecmp(last_key, c"insert_where".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).insert_where = val as ::core::ffi::c_uint;
            swallow_is_empty = false_0 != 0;
        }
    }
    if parsing_gaps {
        if strcasecmp(last_key, c"inner".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).gaps.inner = val as ::core::ffi::c_int;
        } else if strcasecmp(last_key, c"top".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).gaps.top = val as ::core::ffi::c_int;
        } else if strcasecmp(last_key, c"right".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).gaps.right = val as ::core::ffi::c_int;
        } else if strcasecmp(last_key, c"bottom".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).gaps.bottom = val as ::core::ffi::c_int;
        } else if strcasecmp(last_key, c"left".as_ptr()) == 0 as ::core::ffi::c_int {
            (*json_node).gaps.left = val as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_bool(ctx: *mut ::core::ffi::c_void, val: ::core::ffi::c_int) -> ::core::ffi::c_int {
    verboselog(c"bool %d for key %s\n".as_ptr().cast_mut(), val, last_key);
    if strcasecmp(last_key, c"focused".as_ptr()) == 0 as ::core::ffi::c_int && val != 0 {
        to_focus = json_node;
    }
    if strcasecmp(last_key, c"sticky".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).sticky = val != 0;
    }
    if parsing_swallows {
        if strcasecmp(last_key, c"restart_mode".as_ptr()) == 0 as ::core::ffi::c_int {
            (*current_swallow).restart_mode = val != 0;
            swallow_is_empty = false_0 != 0;
        }
    }
    1 as ::core::ffi::c_int
}
unsafe fn json_double(
    ctx: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    verboselog(c"double %f for key %s\n".as_ptr().cast_mut(), val, last_key);
    if strcasecmp(last_key, c"percent".as_ptr()) == 0 as ::core::ffi::c_int {
        (*json_node).percent = val;
    }
    1 as ::core::ffi::c_int
}
static mut content_result: JsonContent = JsonContent::Unknown;
static mut content_level: ::core::ffi::c_int = 0;
unsafe fn json_determine_content_deeper(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    content_level += 1;
    1 as ::core::ffi::c_int
}
unsafe fn json_determine_content_shallower(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    content_level -= 1;
    1 as ::core::ffi::c_int
}
unsafe fn json_determine_content_string(
    ctx: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    len: usize,
) -> ::core::ffi::c_int {
    if strcasecmp(last_key, c"type".as_ptr()) != 0 as ::core::ffi::c_int
        || content_level > 1 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    debuglog(
        c"%s:%s:%d - string = %.*s, last_key = %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/load_layout.c".as_ptr(),
        c"json_determine_content_string".as_ptr(),
        618 as ::core::ffi::c_int,
        len as ::core::ffi::c_int,
        val,
        last_key,
    );
    if strncasecmp(
        val as *const ::core::ffi::c_char,
        c"workspace".as_ptr(),
        len,
    ) == 0 as ::core::ffi::c_int
    {
        content_result = JsonContent::Workspace;
    }
    0 as ::core::ffi::c_int
}

pub unsafe fn json_validate(buf: *const ::core::ffi::c_char, len: usize) -> bool {
    let mut valid: bool = true_0 != 0;
    let hand: yajl_handle = yajl_alloc(
        ::core::ptr::null::<yajl_callbacks>(),
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    if yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as ::core::ffi::c_uint
        != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(c"ERROR: JSON parsing error: %s\n".as_ptr().cast_mut(), str);
        yajl_free_error(hand, str);
        valid = false_0 != 0;
    }
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    valid
}

pub unsafe fn json_determine_content(buf: *const ::core::ffi::c_char, len: usize) -> JsonContent {
    content_result = JsonContent::Container;
    content_level = 0 as ::core::ffi::c_int;
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: None,
            yajl_integer: None,
            yajl_double: None,
            yajl_number: None,
            yajl_string: Some(
                json_determine_content_string
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                json_determine_content_deeper
                    as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                json_key
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                json_determine_content_shallower
                    as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: Some(
                json_determine_content_deeper
                    as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_end_array: Some(
                json_determine_content_shallower
                    as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        }
    };
    let hand: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let stat: yajl_status =
        yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as yajl_status;
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint
        && stat as ::core::ffi::c_uint
            != yajl_status_client_canceled as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(c"ERROR: JSON parsing error: %s\n".as_ptr().cast_mut(), str);
        yajl_free_error(hand, str);
    }
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    content_result
}

pub unsafe fn tree_append_json(
    con: *mut Con,
    buf: *const ::core::ffi::c_char,
    len: usize,
    errormsg: *mut *mut ::core::ffi::c_char,
) {
    static mut callbacks: yajl_callbacks = unsafe {
        yajl_callbacks {
            yajl_null: None,
            yajl_boolean: Some(
                json_bool
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_integer: Some(
                json_int
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_longlong,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_double: Some(
                json_double
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_double,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_number: None,
            yajl_string: Some(
                json_string
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_start_map: Some(
                json_start_map as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_map_key: Some(
                json_key
                    as unsafe fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_uchar,
                        usize,
                    ) -> ::core::ffi::c_int,
            ),
            yajl_end_map: Some(
                json_end_map as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
            yajl_start_array: None,
            yajl_end_array: Some(
                json_end_array as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        }
    };
    let hand: yajl_handle = yajl_alloc(
        &raw mut callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_config(hand, yajl_allow_comments, true_0);
    yajl_config(hand, yajl_allow_multiple_values, true_0);
    yajl_config(hand, yajl_dont_validate_strings, true_0);
    json_node = con;
    to_focus = ::core::ptr::null_mut::<Con>();
    parsing_gaps = false_0 != 0;
    incomplete = 0 as ::core::ffi::c_int;
    parsing_swallows = false_0 != 0;
    parsing_rect = false_0 != 0;
    parsing_actual_deco_rect = false_0 != 0;
    parsing_deco_rect = false_0 != 0;
    parsing_window_rect = false_0 != 0;
    parsing_geometry = false_0 != 0;
    parsing_focus = false_0 != 0;
    parsing_marks = false_0 != 0;
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let stat: yajl_status =
        yajl_parse(hand, buf as *const ::core::ffi::c_uchar, len) as yajl_status;
    if stat as ::core::ffi::c_uint != yajl_status_ok as ::core::ffi::c_int as ::core::ffi::c_uint {
        let str: *mut ::core::ffi::c_uchar = yajl_get_error(
            hand,
            1 as ::core::ffi::c_int,
            buf as *const ::core::ffi::c_uchar,
            len,
        );
        errorlog(c"ERROR: JSON parsing error: %s\n".as_ptr().cast_mut(), str);
        if !errormsg.is_null() {
            *errormsg = sstrdup(str as *const ::core::ffi::c_char);
        }
        yajl_free_error(hand, str);
        loop {
            let fresh0 = incomplete;
            incomplete = incomplete - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            let parent: *mut Con = (*json_node).parent as *mut Con;
            debuglog(
                c"%s:%s:%d - freeing incomplete container %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/load_layout.c".as_ptr(),
                c"tree_append_json".as_ptr(),
                744 as ::core::ffi::c_int,
                json_node,
            );
            if json_node == to_focus {
                to_focus = ::core::ptr::null_mut::<Con>();
            }
            con_free(json_node);
            json_node = parent;
        }
    }
    con_fix_percent(con);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    yajl_complete_parse(hand);
    yajl_free(hand);
    if !to_focus.is_null() {
        con_activate(to_focus);
    }
}
unsafe extern "C" fn run_static_initializers() {
    focus_mappings = focus_mappings_head {
        tqh_first: ::core::ptr::null_mut::<focus_mapping>(),
        tqh_last: &raw mut focus_mappings.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

use crate::regex::regex_new;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::max;
use crate::util::ws_name_to_number;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::sstrndup;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::xcb_window_t;

pub use libc::{free, memcpy, setlocale, strcasecmp, strcmp, strncasecmp};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_activate;
use crate::con::con_attach;
use crate::con::con_fix_percent;
use crate::con::con_free;
use crate::con::con_get_workspace;
use crate::con::con_is_split;
use crate::con::con_mark;
use crate::con::con_new_skeleton;
use crate::floating::floating_check_size;
use crate::floating::floating_enable;
use crate::r#match::match_free;
use crate::r#match::match_init;
use crate::util::rect_equals;
use crate::workspace::get_existing_workspace_by_name;
use crate::workspace::previous_workspace_name;
use crate::x::x_con_init;
