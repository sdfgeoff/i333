/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * render.c: Renders (determines position/sizes) the layout tree, updating the
 *           various rects. Needs to be pushed to X11 (see x.c) to be visible.
 *
 */
// Mechanical file-by-file Rust translation of src/render.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
extern "C" {

    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn lround(__x: ::core::ffi::c_double) -> ::core::ffi::c_long;

}
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

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_19 = 3;
pub const PDF_IGNORE: C2RustUnnamed_19 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_19 = 1;
pub const PDF_SMART: C2RustUnnamed_19 = 0;

pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_20 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_20 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_21 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_21 = 2;
pub const FOWA_URGENT: C2RustUnnamed_21 = 1;
pub const FOWA_SMART: C2RustUnnamed_21 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_params {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub deco_height: ::core::ffi::c_int,
    pub rect: Rect,
    pub children: ::core::ffi::c_int,
    pub sizes: *mut ::core::ffi::c_int,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn render_deco_height() -> ::core::ffi::c_int {
    let mut deco_height: ::core::ffi::c_int = config.font.height + 4 as ::core::ffi::c_int;
    if config.font.height & 0x1 as ::core::ffi::c_int != 0 {
        deco_height += 1;
    }
    deco_height
}

pub unsafe fn render_con(con: *mut Con) {
    let mut params: render_params = render_params {
        x: (*con).rect.x as ::core::ffi::c_int,
        y: (*con).rect.y as ::core::ffi::c_int,
        deco_height: 0,
        rect: (*con).rect as Rect,
        children: con_num_children(con),
        sizes: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    };
    debuglog(
        c"%s:%s:%d - Rendering node %p / %s / layout %d / children %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/render.c".as_ptr(),
        c"render_con".as_ptr(),
        51 as ::core::ffi::c_int,
        con,
        (*con).name,
        (*con).layout as ::core::ffi::c_uint,
        params.children,
    );
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let gaps: gaps_t = calculate_effective_gaps(con);
        let inset: Rect = Rect {
            x: gaps.left as uint32_t,
            y: gaps.top as uint32_t,
            width: -(gaps.left + gaps.right) as uint32_t,
            height: -(gaps.top + gaps.bottom) as uint32_t,
        };
        (*con).rect = rect_add((*con).rect as Rect, inset) as Rect;
        params.rect = rect_add(params.rect, inset);
        params.x += gaps.left;
        params.y += gaps.top;
    }
    if gaps_should_inset_con(con, params.children) {
        let gaps_0: gaps_t = calculate_effective_gaps(con);
        let mut inset_0: Rect = Rect {
            x: (if gaps_has_adjacent_container(con, D_LEFT) as ::core::ffi::c_int != 0 {
                gaps_0.inner / 2 as ::core::ffi::c_int
            } else {
                gaps_0.inner
            }) as uint32_t,
            y: (if gaps_has_adjacent_container(con, D_UP) as ::core::ffi::c_int != 0 {
                gaps_0.inner / 2 as ::core::ffi::c_int
            } else {
                gaps_0.inner
            }) as uint32_t,
            width: (if gaps_has_adjacent_container(con, D_RIGHT) as ::core::ffi::c_int != 0 {
                -(gaps_0.inner / 2 as ::core::ffi::c_int)
            } else {
                -gaps_0.inner
            }) as uint32_t,
            height: (if gaps_has_adjacent_container(con, D_DOWN) as ::core::ffi::c_int != 0 {
                -(gaps_0.inner / 2 as ::core::ffi::c_int)
            } else {
                -gaps_0.inner
            }) as uint32_t,
        };
        inset_0.width = inset_0.width.wrapping_sub(inset_0.x);
        inset_0.height = inset_0.height.wrapping_sub(inset_0.y);
        if (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            params.rect = rect_add(params.rect, inset_0);
            (*con).rect = rect_add((*con).rect as Rect, inset_0) as Rect;
        }
        inset_0.height = 0 as uint32_t;
        params.x = (*con).rect.x as ::core::ffi::c_int;
        params.y = (*con).rect.y as ::core::ffi::c_int;
    }
    (*con).mapped = true_0 != 0;
    if !(*con).window.is_null() {
        let mut inset_1: Rect = Rect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: (*con).rect.width,
            height: (*con).rect.height,
        };
        if (*con).fullscreen_mode as ::core::ffi::c_uint
            == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - deco_rect.height = %d\n".as_ptr().cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_con".as_ptr(),
                101 as ::core::ffi::c_int,
                (*con).deco_rect.height,
            );
            let bsr: Rect = con_border_style_rect(con);
            debuglog(
                c"%s:%s:%d - bsr at %dx%d with size %dx%d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_con".as_ptr(),
                104 as ::core::ffi::c_int,
                bsr.x,
                bsr.y,
                bsr.width,
                bsr.height,
            );
            inset_1 = rect_add(inset_1, bsr);
        }
        inset_1.width = inset_1
            .width
            .wrapping_sub((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
        inset_1.height = inset_1
            .height
            .wrapping_sub((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
        inset_1 = rect_sanitize_dimensions(inset_1);
        (*con).window_rect = inset_1 as Rect;
        debuglog(
            c"%s:%s:%d - child will be at %dx%d with size %dx%d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_con".as_ptr(),
            123 as ::core::ffi::c_int,
            inset_1.x,
            inset_1.y,
            inset_1.width,
            inset_1.height,
        );
    }
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*con).type_0 as ::core::ffi::c_uint
        != CT_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fullscreen = con_get_fullscreen_con(
            con,
            (if (*con).type_0 as ::core::ffi::c_uint
                == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                CF_GLOBAL as ::core::ffi::c_int
            } else {
                CF_OUTPUT as ::core::ffi::c_int
            }) as fullscreen_mode_t,
        );
    }
    if !fullscreen.is_null() {
        (*fullscreen).rect = params.rect as Rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        if (*con).type_0 as ::core::ffi::c_uint
            != CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return;
        }
    }
    params.deco_height = render_deco_height();
    params.sizes = precalculate_sizes(con, &raw mut params);
    if (*con).layout as ::core::ffi::c_uint == L_OUTPUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !con_is_internal(con) {
            render_output(con);
        }
    } else if (*con).type_0 as ::core::ffi::c_uint
        == CT_ROOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        render_root(con, fullscreen);
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if params.children > 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    c"params.children > 0".as_ptr(),
                    c"../src/render.c".as_ptr(),
                    164 as ::core::ffi::c_uint,
                    c"void render_con(Con *)".as_ptr(),
                );
            };
            if (*con).layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*con).layout as ::core::ffi::c_uint
                    == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_split(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_stacked(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_tabbed(con, child, &raw mut params, i);
            } else if (*con).layout as ::core::ffi::c_uint
                == L_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                render_con_dockarea(con, child, &raw mut params);
            }
            (*child).rect = rect_sanitize_dimensions((*child).rect as Rect) as Rect;
            debuglog(
                c"%s:%s:%d - child at (%d, %d) with (%d x %d)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_con".as_ptr(),
                179 as ::core::ffi::c_int,
                (*child).rect.x,
                (*child).rect.y,
                (*child).rect.width,
                (*child).rect.height,
            );
            x_raise_con(child);
            render_con(child);
            if (*con).layout as ::core::ffi::c_uint
                == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*con).layout as ::core::ffi::c_uint
                    == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if con_is_leaf(child) {
                    if (*child).border_style as ::core::ffi::c_uint
                        == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*child).deco_rect.width = (*child).rect.width;
                    }
                }
            }
            i += 1;
            child = (*child).nodes.tqe_next as *mut Con;
        }
        if (*con).layout as ::core::ffi::c_uint
            == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*con).layout as ::core::ffi::c_uint
                == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            child = *(*((*con).focus_head.tqh_last as *mut focus_head)).tqh_last as *mut Con;
            while !child.is_null() {
                x_raise_con(child);
                child = *(*((*child).focused.tqe_prev as *mut focus_head)).tqh_last as *mut Con;
            }
            child = (*con).focus_head.tqh_first as *mut Con;
            if !child.is_null() {
                render_con(child);
            }
            if params.children != 1 as ::core::ffi::c_int {
                x_raise_con(con);
            }
        }
    }
    free(params.sizes as *mut ::core::ffi::c_void);
    params.sizes = ::core::ptr::null_mut::<::core::ffi::c_int>();
}
unsafe fn precalculate_sizes(con: *mut Con, p: *mut render_params) -> *mut ::core::ffi::c_int {
    if (*con).layout as ::core::ffi::c_uint != L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*con).layout as ::core::ffi::c_uint
            != L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*p).children <= 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    let sizes: *mut ::core::ffi::c_int = smalloc(
        ((*p).children as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    if !(*con).nodes_head.tqh_first.is_null() as ::core::ffi::c_int != 0 {
    } else {
        __assert_fail(
            c"!TAILQ_EMPTY(&con->nodes_head)".as_ptr(),
            c"../src/render.c".as_ptr(),
            230 as ::core::ffi::c_uint,
            c"int *precalculate_sizes(Con *, render_params *)".as_ptr(),
        );
    };
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut assigned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let total: ::core::ffi::c_int = con_rect_size_in_orientation(con) as ::core::ffi::c_int;
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        let percentage: ::core::ffi::c_double = if (*child).percent > 0.0f64 {
            (*child).percent
        } else {
            1.0f64 / (*p).children as ::core::ffi::c_double
        };
        let fresh0 = i;
        i = i + 1;
        let ref mut fresh1 = *sizes.offset(fresh0 as isize);
        *fresh1 = lround(percentage * total as ::core::ffi::c_double) as ::core::ffi::c_int;
        assigned += *fresh1;
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if assigned == total
        || assigned > total && assigned - total <= (*p).children * 2 as ::core::ffi::c_int
        || assigned < total && total - assigned <= (*p).children * 2 as ::core::ffi::c_int
    {
    } else {
        __assert_fail(
            c"assigned == total || (assigned > total && assigned - total <= p->children * 2) || (assigned < total && total - assigned <= p->children * 2)".as_ptr(),
            c"../src/render.c".as_ptr(),
            241 as ::core::ffi::c_uint,
            c"int *precalculate_sizes(Con *, render_params *)".as_ptr(),
        );
    };
    let signal: ::core::ffi::c_int = if assigned < total {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    while assigned != total {
        i = 0 as ::core::ffi::c_int;
        while i < (*p).children && assigned != total {
            *sizes.offset(i as isize) += signal;
            assigned += signal;
            i += 1;
        }
    }
    sizes
}
unsafe fn fullscreen_blocks_floating_render(fullscreen: *mut Con, floating: *mut Con) -> bool {
    if fullscreen.is_null() {
        return false_0 != 0;
    }
    match config.popup_during_fullscreen as ::core::ffi::c_uint {
        1 | 2 => return true_0 != 0,
        0 => {
            return (*fullscreen).window.is_null()
                || !con_find_transient_for_window(
                    con_descend_focused(floating),
                    (*(*fullscreen).window).id,
                );
        }
        3 => return con_has_parent(fullscreen, floating),
        _ => {}
    }
    false_0 != 0
}
unsafe fn render_root(con: *mut Con, fullscreen: *mut Con) {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    if fullscreen.is_null() {
        output = (*con).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            render_con(output);
            output = (*output).nodes.tqe_next as *mut Con;
        }
    }
    debuglog(
        c"%s:%s:%d - Rendering floating windows:\n"
            .as_ptr()
            .cast_mut(),
        c"../src/render.c".as_ptr(),
        c"render_root".as_ptr(),
        285 as ::core::ffi::c_int,
    );
    output = (*con).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let content: *mut Con = output_get_content(output);
            if content.is_null() || (*content).focus_head.tqh_first.is_null() {
                debuglog(
                    c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/render.c".as_ptr(),
                    c"render_root".as_ptr(),
                    293 as ::core::ffi::c_int,
                );
            } else {
                let workspace: *mut Con = (*content).focus_head.tqh_first as *mut Con;
                let fullscreen_0: *mut Con = con_get_fullscreen_covering_ws(workspace);
                let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
                child = (*workspace).floating_head.tqh_first as *mut Con;
                while !child.is_null() {
                    if !fullscreen_blocks_floating_render(fullscreen_0, child) {
                        debuglog(
                            c"%s:%s:%d - floating child at (%d,%d) with %d x %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/render.c".as_ptr(),
                            c"render_root".as_ptr(),
                            304 as ::core::ffi::c_int,
                            (*child).rect.x,
                            (*child).rect.y,
                            (*child).rect.width,
                            (*child).rect.height,
                        );
                        x_raise_con(child);
                        render_con(child);
                    }
                    child = (*child).floating_windows.tqe_next as *mut Con;
                }
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}
unsafe fn render_output(con: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut dockchild: *mut Con = ::core::ptr::null_mut::<Con>();
    let x: ::core::ffi::c_int = (*con).rect.x as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = (*con).rect.y as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = (*con).rect.height as ::core::ffi::c_int;
    let mut content: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if !content.is_null() {
                debuglog(
                    c"%s:%s:%d - More than one CT_CON on output container\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/render.c".as_ptr(),
                    c"render_output".as_ptr(),
                    329 as ::core::ffi::c_int,
                );
                if 0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        c"false".as_ptr(),
                        c"../src/render.c".as_ptr(),
                        330 as ::core::ffi::c_uint,
                        c"void render_output(Con *)".as_ptr(),
                    );
                };
            }
            content = child;
        } else if (*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - Child %p of type %d is inside the OUTPUT con\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_output".as_ptr(),
                334 as ::core::ffi::c_int,
                child,
                (*child).type_0 as ::core::ffi::c_uint,
            );
            if 0 as ::core::ffi::c_int != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/render.c".as_ptr(),
                    335 as ::core::ffi::c_uint,
                    c"void render_output(Con *)".as_ptr(),
                );
            };
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if content.is_null() {
        debuglog(
            c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            340 as ::core::ffi::c_int,
        );
        return;
    }
    let ws: *mut Con = con_get_fullscreen_con(content, CF_OUTPUT);
    if ws.is_null() {
        debuglog(
            c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            349 as ::core::ffi::c_int,
        );
        return;
    }
    let fullscreen: *mut Con = con_get_fullscreen_con(ws, CF_OUTPUT);
    if !fullscreen.is_null() {
        (*fullscreen).rect = (*con).rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        return;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !((*child).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            (*child).rect.height = 0 as uint32_t;
            dockchild = (*child).nodes_head.tqh_first as *mut Con;
            while !dockchild.is_null() {
                (*child).rect.height = (*child)
                    .rect
                    .height
                    .wrapping_add((*dockchild).geometry.height);
                dockchild = (*dockchild).nodes.tqe_next as *mut Con;
            }
            height = (height as uint32_t).wrapping_sub((*child).rect.height) as ::core::ffi::c_int
                as ::core::ffi::c_int;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*child).rect.x = x as uint32_t;
            (*child).rect.y = y as uint32_t;
            (*child).rect.width = (*con).rect.width;
            (*child).rect.height = height as uint32_t;
        }
        (*child).rect.x = x as uint32_t;
        (*child).rect.y = y as uint32_t;
        (*child).rect.width = (*con).rect.width;
        (*child).deco_rect.x = 0 as uint32_t;
        (*child).deco_rect.y = 0 as uint32_t;
        (*child).deco_rect.width = 0 as uint32_t;
        (*child).deco_rect.height = 0 as uint32_t;
        y = (y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        debuglog(
            c"%s:%s:%d - child at (%d, %d) with (%d x %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            396 as ::core::ffi::c_int,
            (*child).rect.x,
            (*child).rect.y,
            (*child).rect.width,
            (*child).rect.height,
        );
        x_raise_con(child);
        render_con(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
}
unsafe fn render_con_split(
    con: *mut Con,
    child: *mut Con,
    p: *mut render_params,
    i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*con).layout as ::core::ffi::c_uint
            == L_SPLITV as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"con->layout == L_SPLITH || con->layout == L_SPLITV".as_ptr(),
            c"../src/render.c".as_ptr(),
            403 as ::core::ffi::c_uint,
            c"void render_con_split(Con *, Con *, render_params *, int)".as_ptr(),
        );
    };
    if (*con).layout as ::core::ffi::c_uint == L_SPLITH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.x = (*p).x as uint32_t;
        (*child).rect.y = (*p).y as uint32_t;
        (*child).rect.width = *(*p).sizes.offset(i as isize) as uint32_t;
        (*child).rect.height = (*p).rect.height;
        (*p).x = ((*p).x as uint32_t).wrapping_add((*child).rect.width) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    } else {
        (*child).rect.x = (*p).x as uint32_t;
        (*child).rect.y = (*p).y as uint32_t;
        (*child).rect.width = (*p).rect.width;
        (*child).rect.height = *(*p).sizes.offset(i as isize) as uint32_t;
        (*p).y = ((*p).y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    }
    if con_is_leaf(child) {
        if (*child).border_style as ::core::ffi::c_uint
            == BS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*child).deco_rect.x = 0 as uint32_t;
            (*child).deco_rect.y = 0 as uint32_t;
            (*child).deco_rect.width = (*child).rect.width;
            (*child).deco_rect.height = (*p).deco_height as uint32_t;
        } else {
            (*child).deco_rect.x = 0 as uint32_t;
            (*child).deco_rect.y = 0 as uint32_t;
            (*child).deco_rect.width = 0 as uint32_t;
            (*child).deco_rect.height = 0 as uint32_t;
        }
    }
}
unsafe fn render_con_stacked(
    con: *mut Con,
    child: *mut Con,
    p: *mut render_params,
    i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint
        == L_STACKED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"con->layout == L_STACKED".as_ptr(),
            c"../src/render.c".as_ptr(),
            438 as ::core::ffi::c_uint,
            c"void render_con_stacked(Con *, Con *, render_params *, int)".as_ptr(),
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*p).rect.height;
    (*child).deco_rect.x = ((*p).x as uint32_t).wrapping_sub((*con).rect.x);
    (*child).deco_rect.y = ((*p).y as uint32_t)
        .wrapping_sub((*con).rect.y)
        .wrapping_add((i * (*p).deco_height) as uint32_t);
    (*child).deco_rect.width = (*child).rect.width;
    (*child).deco_rect.height = (*p).deco_height as uint32_t;
    if (*p).children > 1 as ::core::ffi::c_int
        || (*child).border_style as ::core::ffi::c_uint
            != BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*child).border_style as ::core::ffi::c_uint
                != BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.y = (*child)
            .rect
            .y
            .wrapping_add(((*p).deco_height * (*p).children) as uint32_t);
        (*child).rect.height = (*child)
            .rect
            .height
            .wrapping_sub(((*p).deco_height * (*p).children) as uint32_t);
    }
}
unsafe fn render_con_tabbed(
    con: *mut Con,
    child: *mut Con,
    p: *mut render_params,
    i: ::core::ffi::c_int,
) {
    if (*con).layout as ::core::ffi::c_uint == L_TABBED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"con->layout == L_TABBED".as_ptr(),
            c"../src/render.c".as_ptr(),
            457 as ::core::ffi::c_uint,
            c"void render_con_tabbed(Con *, Con *, render_params *, int)".as_ptr(),
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*p).rect.height;
    (*child).deco_rect.width = floor(
        ((*child).rect.width as ::core::ffi::c_float / (*p).children as ::core::ffi::c_float)
            as ::core::ffi::c_double,
    ) as uint32_t;
    (*child).deco_rect.x = ((*p).x as uint32_t)
        .wrapping_sub((*con).rect.x)
        .wrapping_add((i as uint32_t).wrapping_mul((*child).deco_rect.width));
    (*child).deco_rect.y = ((*p).y as uint32_t).wrapping_sub((*con).rect.y);
    if i == (*p).children - 1 as ::core::ffi::c_int {
        (*child).deco_rect.width = (*child).rect.width.wrapping_sub((*child).deco_rect.x);
    }
    if (*p).children > 1 as ::core::ffi::c_int
        || (*child).border_style as ::core::ffi::c_uint
            != BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*child).border_style as ::core::ffi::c_uint
                != BS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*child).rect.y = (*child).rect.y.wrapping_add((*p).deco_height as uint32_t);
        (*child).rect.height = (*child)
            .rect
            .height
            .wrapping_sub((*p).deco_height as uint32_t);
        (*child).deco_rect.height = (*p).deco_height as uint32_t;
    } else {
        (*child).deco_rect.height = (if (*child).border_style as ::core::ffi::c_uint
            == BS_PIXEL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t;
    };
}
unsafe fn render_con_dockarea(con: *mut Con, child: *mut Con, p: *mut render_params) {
    if (*con).layout as ::core::ffi::c_uint
        == L_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        __assert_fail(
            c"con->layout == L_DOCKAREA".as_ptr(),
            c"../src/render.c".as_ptr(),
            484 as ::core::ffi::c_uint,
            c"void render_con_dockarea(Con *, Con *, render_params *)".as_ptr(),
        );
    };
    (*child).rect.x = (*p).x as uint32_t;
    (*child).rect.y = (*p).y as uint32_t;
    (*child).rect.width = (*p).rect.width;
    (*child).rect.height = (*child).geometry.height;
    (*child).deco_rect.x = 0 as uint32_t;
    (*child).deco_rect.y = 0 as uint32_t;
    (*child).deco_rect.width = 0 as uint32_t;
    (*child).deco_rect.height = 0 as uint32_t;
    (*p).y = ((*p).y as uint32_t).wrapping_add((*child).rect.height) as ::core::ffi::c_int
        as ::core::ffi::c_int;
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::free;

use i3_common::native_stdio::__assert_fail;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::focus_head;
pub use crate::types::gaps_t;
pub use crate::types::Con;
pub use crate::types::Rect;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_border_style_rect;
use crate::con::con_descend_focused;
use crate::con::con_find_transient_for_window;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_has_parent;
use crate::con::con_is_internal;
use crate::con::con_is_leaf;
use crate::con::con_num_children;
use crate::con::con_rect_size_in_orientation;
use crate::config::config;
use crate::gaps::calculate_effective_gaps;
use crate::gaps::gaps_has_adjacent_container;
use crate::gaps::gaps_should_inset_con;
use crate::output::output_get_content;
use crate::util::rect_add;
use crate::util::rect_sanitize_dimensions;
use crate::x::x_raise_con;
