use crate::types::BorderStyle;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::TitleAlign;
use crate::types::Warping;
use crate::xcursor::Cursor;
use i3_common::draw_util::draw_util_text;
use std::ffi::{CStr, CString};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * x.c: Interface to X11, transfers our in-memory state to X11 (see also
 *      render.c). Basically a big state machine.
 *
 */
// Local policy and temporary values use owned Rust storage. Native entrypoints
// require live event-thread graph handles and a live display; callbacks may mutate
// the graph, so node references must never span those calls.

type Output = xoutput;
const ADJ_LOWER_SCREEN_EDGE: u32 = 16;
const ADJ_UPPER_SCREEN_EDGE: u32 = 4;
const ADJ_RIGHT_SCREEN_EDGE: u32 = 2;
const ADJ_LEFT_SCREEN_EDGE: u32 = 1;
const CT_ROOT: u32 = 0;
const CT_OUTPUT: u32 = 1;
const CT_DOCKAREA: u32 = 5;
const CT_FLOATING_CON: u32 = 3;

// Adoption is confined to native reply/allocation producers in this module.
struct NativeReply<T>(*mut T);
impl<T> Drop for NativeReply<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.0.cast());
        }
    }
}
struct ProtocolReply(xcb_icccm_get_wm_protocols_reply_t);
impl Drop for ProtocolReply {
    fn drop(&mut self) {
        unsafe {
            xcb_icccm_get_wm_protocols_reply_wipe(&raw mut self.0);
        }
    }
}

fn delete_window_event(window: u32, protocols: u32, delete: u32) -> xcb_client_message_event_t {
    xcb_client_message_event_t {
        response_type: XCB_CLIENT_MESSAGE as u8,
        format: 32,
        sequence: 0,
        window,
        type_0: protocols,
        data: xcb_sys::xcb_client_message_data_t {
            data32: [delete, XCB_CURRENT_TIME as u32, 0, 0, 0],
        },
    }
}

fn format_marks<'a>(marks: impl IntoIterator<Item = &'a [u8]>) -> Option<Vec<u8>> {
    let mut text = Vec::new();
    let mut visible = false;
    for mark in marks {
        if mark.first() == Some(&b'_') {
            continue;
        }
        visible = true;
        text.push(b'[');
        text.extend_from_slice(mark);
        text.push(b']');
    }
    visible.then_some(text)
}

struct BorderRectangles {
    values: [xcb_rectangle_t; 4],
    len: usize,
}
impl BorderRectangles {
    fn as_slice(&self) -> &[xcb_rectangle_t] {
        &self.values[..self.len]
    }
}
fn border_rectangles(
    style: BorderStyle,
    leaf: bool,
    hidden: u32,
    border: Rect,
    rect: Rect,
) -> BorderRectangles {
    let mut result = BorderRectangles {
        values: [xcb_rectangle_t {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 4],
        len: 0,
    };
    if style == BorderStyle::None || !leaf {
        return result;
    }
    let mut push = |value| {
        result.values[result.len] = value;
        result.len += 1;
    };
    if hidden & ADJ_LEFT_SCREEN_EDGE == 0 {
        push(xcb_rectangle_t {
            x: 0,
            y: 0,
            width: border.x as u16,
            height: rect.height as u16,
        });
    }
    if hidden & ADJ_RIGHT_SCREEN_EDGE == 0 {
        push(xcb_rectangle_t {
            x: rect.width.wrapping_add(border.width.wrapping_add(border.x)) as i16,
            y: 0,
            width: border.width.wrapping_add(border.x).wrapping_neg() as u16,
            height: rect.height as u16,
        });
    }
    if hidden & ADJ_LOWER_SCREEN_EDGE == 0 {
        push(xcb_rectangle_t {
            x: border.x as i16,
            y: rect
                .height
                .wrapping_add(border.height.wrapping_add(border.y)) as i16,
            width: rect.width.wrapping_add(border.width) as u16,
            height: border.height.wrapping_add(border.y).wrapping_neg() as u16,
        });
    }
    if style == BorderStyle::Pixel && hidden & ADJ_UPPER_SCREEN_EDGE == 0 {
        push(xcb_rectangle_t {
            x: border.x as i16,
            y: 0,
            width: rect.width.wrapping_add(border.width) as u16,
            height: border.y as u16,
        });
    }
    result
}

fn same_decoration(first: &deco_render_params, second: &deco_render_params) -> bool {
    fn rect(a: Rect, b: Rect) -> bool {
        a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
    }
    fn color(a: color_t, b: color_t) -> bool {
        a.red.to_bits() == b.red.to_bits()
            && a.green.to_bits() == b.green.to_bits()
            && a.blue.to_bits() == b.blue.to_bits()
            && a.alpha.to_bits() == b.alpha.to_bits()
            && a.colorpixel == b.colorpixel
    }
    first.color == second.color
        && first.border_style == second.border_style
        && first.con_rect.w == second.con_rect.w
        && first.con_rect.h == second.con_rect.h
        && first.con_window_rect.w == second.con_window_rect.w
        && first.con_window_rect.h == second.con_window_rect.h
        && rect(first.con_deco_rect, second.con_deco_rect)
        && color(first.background, second.background)
        && first.parent_layout == second.parent_layout
        && first.con_is_leaf == second.con_is_leaf
}

struct TitlePlacement {
    icon_size: i32,
    has_icon: bool,
    icon_offset: i32,
    title_offset: i32,
    available: i32,
}
fn title_placement(
    align: TitleAlign,
    width: i32,
    height: u32,
    two_px: i32,
    configured_padding: i32,
    icon_present: bool,
    title_width: i32,
    mark_width: i32,
    title_padding: i32,
    icon_padding: i32,
) -> TitlePlacement {
    let mut size = (height.wrapping_sub(two_px as u32) as i32).max(0);
    let mut padding = icon_padding;
    let mut total = size.wrapping_add(2i32.wrapping_mul(padding));
    let has_icon = configured_padding > -1 && icon_present && total < width;
    if !has_icon {
        total = 0;
        padding = 0;
        size = 0;
    }
    let (icon_offset, title_offset) = match align {
        TitleAlign::Left => (padding, title_padding.wrapping_add(total)),
        TitleAlign::Center => {
            let offset = padding.max(
                width
                    .wrapping_sub(padding)
                    .wrapping_sub(size)
                    .wrapping_sub(title_width)
                    .wrapping_sub(title_padding)
                    .wrapping_sub(mark_width)
                    / 2,
            );
            (
                offset,
                title_padding.max(offset.wrapping_add(padding).wrapping_add(size)),
            )
        }
        TitleAlign::Right => {
            let title = title_padding.wrapping_add(mark_width).max(
                width
                    .wrapping_sub(title_padding)
                    .wrapping_sub(title_width)
                    .wrapping_sub(total),
            );
            (
                width
                    .wrapping_sub(size)
                    .wrapping_sub(padding)
                    .wrapping_sub(title_padding)
                    .min(title.wrapping_add(title_width).wrapping_add(padding)),
                title,
            )
        }
    };
    TitlePlacement {
        icon_size: size,
        has_icon,
        icon_offset,
        title_offset,
        available: width
            .wrapping_sub(mark_width)
            .wrapping_sub(2i32.wrapping_mul(title_padding))
            .wrapping_sub(total),
    }
}

fn pixmap_dimensions(rect: Rect) -> (i32, i32) {
    ((rect.width as i32).max(1), (rect.height as i32).max(1))
}
fn warp_midpoint(rect: Rect) -> (i32, i32) {
    (
        rect.x.wrapping_add(rect.width / 2) as i32,
        rect.y.wrapping_add(rect.height / 2) as i32,
    )
}

#[derive(Debug, PartialEq, Eq)]
enum FocusAction {
    Unmapped,
    TakeFocus,
    Direct,
}
fn focus_action(mapped: bool, take_focus: bool, refuses_input: bool) -> FocusAction {
    if !mapped {
        FocusAction::Unmapped
    } else if take_focus && refuses_input {
        FocusAction::TakeFocus
    } else {
        FocusAction::Direct
    }
}
fn needs_pixmap(leaf: bool, border: BorderStyle, layout: Layout, kind: i32) -> bool {
    kind != CT_ROOT as i32
        && kind != CT_OUTPUT as i32
        && ((leaf && border != BorderStyle::None)
            || matches!(layout, Layout::Stacked | Layout::Tabbed))
}
fn needs_reshape(
    old: Rect,
    new: Rect,
    old_window: Rect,
    new_window: Rect,
    floating: bool,
    was_floating: bool,
    reparent: bool,
) -> bool {
    reparent
        || old.width != new.width
        || old.height != new.height
        || old_window.width != new_window.width
        || old_window.height != new_window.height
        || (floating && !was_floating)
}
fn frame_extents(border: Rect) -> [u32; 4] {
    [
        border.x,
        0u32.wrapping_sub(border.width).wrapping_sub(border.x),
        border.y,
        0u32.wrapping_sub(border.height).wrapping_sub(border.y),
    ]
}
fn title_border(rect: Rect) -> [(f64, f64, f64, f64); 4] {
    [
        (rect.x as f64, rect.y as f64, 1.0, rect.height as f64),
        (
            rect.x.wrapping_add(rect.width).wrapping_sub(1) as f64,
            rect.y as f64,
            1.0,
            rect.height as f64,
        ),
        (rect.x as f64, rect.y as f64, rect.width as f64, 1.0),
        (
            rect.x as f64,
            rect.y.wrapping_add(rect.height).wrapping_sub(1) as f64,
            rect.width as f64,
            1.0,
        ),
    ]
}

struct FrameState {
    id: xcb_window_t,
    mapped: bool,
    unmap_now: bool,
    child_mapped: bool,
    is_hidden: bool,
    is_maximized_vert: bool,
    is_maximized_horz: bool,
    con: *mut Con,
    need_reparent: bool,
    old_frame: xcb_window_t,
    was_floating: bool,
    rect: Rect,
    window_rect: Rect,
    initial: bool,
    name: Option<CString>,
    state: StateLinks,
    old_state: StateLinks,
    initial_mapping_order: MappingLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct MappingLinks {
    tqe_next: *mut FrameState,
    tqe_prev: *mut *mut FrameState,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct StateLinks {
    cqe_next: *mut FrameState,
    cqe_prev: *mut FrameState,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct InitialMappingHead {
    tqh_first: *mut FrameState,
    tqh_last: *mut *mut FrameState,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct OldStateHead {
    cqh_first: *mut FrameState,
    cqh_last: *mut FrameState,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct StateHead {
    cqh_first: *mut FrameState,
    cqh_last: *mut FrameState,
}

pub static mut focused_id: xcb_window_t = XCB_NONE as xcb_window_t;
static mut last_focused: xcb_window_t = XCB_NONE as xcb_window_t;
static mut warp_to: *mut Rect = ::core::ptr::null::<Rect>() as *mut Rect;

static mut state_head: StateHead = unsafe {
    StateHead {
        cqh_first: &raw const state_head as *mut StateHead as *mut ::core::ffi::c_void
            as *mut FrameState,
        cqh_last: &raw const state_head as *mut StateHead as *mut ::core::ffi::c_void
            as *mut FrameState,
    }
};

static mut old_state_head: OldStateHead = unsafe {
    OldStateHead {
        cqh_first: &raw const old_state_head as *mut OldStateHead as *mut ::core::ffi::c_void
            as *mut FrameState,
        cqh_last: &raw const old_state_head as *mut OldStateHead as *mut ::core::ffi::c_void
            as *mut FrameState,
    }
};

static mut initial_mapping_head: InitialMappingHead = InitialMappingHead {
    tqh_first: ::core::ptr::null::<FrameState>() as *mut FrameState,
    tqh_last: unsafe { &raw mut initial_mapping_head.tqh_first },
};
unsafe fn state_for_frame(window: xcb_window_t) -> *mut FrameState {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_head.cqh_first as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).id == window {
            return state;
        }
        state = (*state).state.cqe_next as *mut FrameState;
    }
    errorlog(
        c"ERROR: No state found for window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        window,
    );
    if 0 != 0 {
    } else {
        __assert_fail(
            c"false".as_ptr(),
            c"../src/x.c".as_ptr(),
            99 as u32,
            c"FrameState *state_for_frame(xcb_window_t)".as_ptr(),
        );
    };
    ::core::ptr::null_mut::<FrameState>()
}
unsafe fn change_ewmh_focus(new_focus: xcb_window_t, old_focus: xcb_window_t) {
    if new_focus == old_focus {
        return;
    }
    ewmh_update_active_window(new_focus);
    if new_focus != XCB_WINDOW_NONE as i32 as xcb_window_t {
        ewmh_update_focused(new_focus, true);
    }
    if old_focus != XCB_WINDOW_NONE as i32 as xcb_window_t {
        ewmh_update_focused(old_focus, false);
    }
}

pub unsafe fn x_con_init(con: *mut Con) {
    let mut mask: u32 = 0 as u32;
    let mut values: [u32; 5] = [0; 5];
    let visual: xcb_visualid_t = get_visualid_by_depth((*con).depth);
    let mut win_colormap: xcb_colormap_t = 0;
    if (*con).depth as i32 != root_depth as i32 {
        win_colormap = xcb_generate_id(conn) as xcb_colormap_t;
        xcb_create_colormap(
            conn,
            XCB_COLORMAP_ALLOC_NONE as i32 as u8,
            win_colormap,
            root,
            visual,
        );
        (*con).colormap = win_colormap;
    } else {
        win_colormap = colormap;
        (*con).colormap = XCB_NONE as xcb_colormap_t;
    }
    mask |= XCB_CW_BACK_PIXEL as i32 as u32;
    values[0] = (*root_screen).black_pixel;
    mask |= XCB_CW_BORDER_PIXEL as i32 as u32;
    values[1] = (*root_screen).black_pixel;
    mask |= XCB_CW_OVERRIDE_REDIRECT as i32 as u32;
    values[2] = 1 as u32;
    mask |= XCB_CW_EVENT_MASK as i32 as u32;
    values[3] = ((XCB_EVENT_MASK_BUTTON_PRESS as i32
        | XCB_EVENT_MASK_BUTTON_RELEASE as i32
        | XCB_EVENT_MASK_POINTER_MOTION as i32
        | XCB_EVENT_MASK_EXPOSURE as i32
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_ENTER_WINDOW as i32)
        & !(XCB_EVENT_MASK_ENTER_WINDOW as i32)) as u32;
    mask |= XCB_CW_COLORMAP as i32 as u32;
    values[4] = win_colormap as u32;
    let dims: Rect = Rect {
        x: (-15i32) as u32,
        y: (-15i32) as u32,
        width: 10 as u32,
        height: 10 as u32,
    };
    let frame_id: xcb_window_t = create_window(
        conn,
        dims,
        (*con).depth,
        visual,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as i32 as u16,
        Cursor::Pointer,
        false,
        mask,
        &raw mut values as *mut u32,
    );
    draw_util_surface_init(
        conn,
        &raw mut (*con).frame,
        frame_id as xcb_drawable_t,
        get_visualtype_by_id(visual),
        dims.width as i32,
        dims.height as i32,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        (*con).frame.id as xcb_window_t,
        XCB_ATOM_WM_CLASS as i32 as xcb_atom_t,
        XCB_ATOM_STRING as i32 as xcb_atom_t,
        8 as u8,
        strlen(c"i3-frame".as_ptr())
            .wrapping_add(1 as usize)
            .wrapping_mul(2 as usize) as u32,
        b"i3-frame\0i3-frame\0\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
    );
    let state = Box::into_raw(Box::new(FrameState {
        id: (*con).frame.id,
        mapped: false,
        unmap_now: false,
        child_mapped: false,
        is_hidden: false,
        is_maximized_vert: false,
        is_maximized_horz: false,
        con: std::ptr::null_mut(),
        need_reparent: false,
        old_frame: 0,
        was_floating: false,
        rect: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        window_rect: Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        },
        initial: true,
        name: None,
        state: StateLinks {
            cqe_next: std::ptr::null_mut(),
            cqe_prev: std::ptr::null_mut(),
        },
        old_state: StateLinks {
            cqe_next: std::ptr::null_mut(),
            cqe_prev: std::ptr::null_mut(),
        },
        initial_mapping_order: MappingLinks {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: std::ptr::null_mut(),
        },
    }));
    debuglog(
        c"%s:%s:%d - Adding window 0x%08x to lists\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_con_init".as_ptr(),
        186,
        (*state).id,
    );
    (*state).state.cqe_next = state_head.cqh_first;
    (*state).state.cqe_prev = &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState;
    if state_head.cqh_last == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        state_head.cqh_last = state as *mut FrameState;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state as *mut FrameState;
    }
    state_head.cqh_first = state as *mut FrameState;
    (*state).old_state.cqe_next = old_state_head.cqh_first;
    (*state).old_state.cqe_prev =
        &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState;
    if old_state_head.cqh_last
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        old_state_head.cqh_last = state as *mut FrameState;
    } else {
        (*old_state_head.cqh_first).old_state.cqe_prev = state as *mut FrameState;
    }
    old_state_head.cqh_first = state as *mut FrameState;
    (*state).initial_mapping_order.tqe_next = ::core::ptr::null_mut::<FrameState>();
    (*state).initial_mapping_order.tqe_prev = initial_mapping_head.tqh_last;
    *initial_mapping_head.tqh_last = state as *mut FrameState;
    initial_mapping_head.tqh_last = &raw mut (*state).initial_mapping_order.tqe_next;
    debuglog(
        c"%s:%s:%d - adding new state for window id 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_con_init".as_ptr(),
        190,
        (*state).id,
    );
}

pub unsafe fn x_reinit(con: *mut Con) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(c"ERROR: window state not found\n".as_ptr().cast_mut());
        return;
    }
    debuglog(
        c"%s:%s:%d - resetting state %p to initial\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_reinit".as_ptr(),
        207,
        state,
    );
    (*state).initial = true;
    (*state).child_mapped = false;
    (*state).con = con;
    (*state).window_rect = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
}

pub unsafe fn x_reparent_child(con: *mut Con, old: *mut Con) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            c"ERROR: window state for con not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    (*state).need_reparent = true;
    (*state).old_frame = (*old).frame.id as xcb_window_t;
}

pub unsafe fn x_move_win(src: *mut Con, dest: *mut Con) {
    let mut state_src: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    let mut state_dest: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state_src = state_for_frame((*src).frame.id as xcb_window_t);
    if state_src.is_null() {
        errorlog(
            c"ERROR: window state for src not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    state_dest = state_for_frame((*dest).frame.id as xcb_window_t);
    if state_dest.is_null() {
        errorlog(
            c"ERROR: window state for dest not found\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    (*state_dest).con = (*state_src).con;
    (*state_src).con = ::core::ptr::null_mut::<Con>();
    if rect_equals(
        (*state_dest).window_rect,
        Rect {
            x: 0 as u32,
            y: 0 as u32,
            width: 0 as u32,
            height: 0 as u32,
        },
    ) {
        (*state_dest).window_rect = (*state_src).window_rect;
        debuglog(
            c"%s:%s:%d - COPYING RECT\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_move_win".as_ptr(),
            252,
        );
    }
}
unsafe fn _x_con_kill(con: *mut Con) {
    if (*con).colormap as ::core::ffi::c_long != XCB_NONE {
        xcb_free_colormap(conn, (*con).colormap);
    }
    draw_util_surface_free(conn, &raw mut (*con).frame);
    draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
    xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
    (*con).frame_buffer.id = XCB_NONE as xcb_drawable_t;
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).state.cqe_next == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        state_head.cqh_last = (*state).state.cqe_prev;
    } else {
        (*(*state).state.cqe_next).state.cqe_prev = (*state).state.cqe_prev;
    }
    if (*state).state.cqe_prev == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        state_head.cqh_first = (*state).state.cqe_next;
    } else {
        (*(*state).state.cqe_prev).state.cqe_next = (*state).state.cqe_next;
    }
    if (*state).old_state.cqe_next
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        old_state_head.cqh_last = (*state).old_state.cqe_prev;
    } else {
        (*(*state).old_state.cqe_next).old_state.cqe_prev = (*state).old_state.cqe_prev;
    }
    if (*state).old_state.cqe_prev
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        old_state_head.cqh_first = (*state).old_state.cqe_next;
    } else {
        (*(*state).old_state.cqe_prev).old_state.cqe_next = (*state).old_state.cqe_next;
    }
    if !(*state).initial_mapping_order.tqe_next.is_null() {
        (*(*state).initial_mapping_order.tqe_next)
            .initial_mapping_order
            .tqe_prev = (*state).initial_mapping_order.tqe_prev;
    } else {
        initial_mapping_head.tqh_last = (*state).initial_mapping_order.tqe_prev;
    }
    *(*state).initial_mapping_order.tqe_prev = (*state).initial_mapping_order.tqe_next;
    // All three intrusive memberships are gone before destroying owned fields.
    drop(Box::from_raw(state));
    if (*con).frame.id == focused_id {
        focused_id = XCB_NONE as xcb_window_t;
    }
    if (*con).frame.id == last_focused {
        last_focused = XCB_NONE as xcb_window_t;
    }
}

pub unsafe fn x_con_kill(con: *mut Con) {
    _x_con_kill(con);
    xcb_destroy_window(conn, (*con).frame.id as xcb_window_t);
}

pub unsafe fn x_con_reframe(con: *mut Con) {
    _x_con_kill(con);
    x_con_init(con);
}

pub unsafe fn window_supports_protocol(window: xcb_window_t, atom: xcb_atom_t) -> bool {
    let mut protocols: xcb_icccm_get_wm_protocols_reply_t = xcb_icccm_get_wm_protocols_reply_t {
        atoms_len: 0,
        atoms: ::core::ptr::null_mut::<xcb_atom_t>(),
        _reply: ::core::ptr::null_mut::<xcb_get_property_reply_t>(),
    };
    let cookie = xcb_icccm_get_wm_protocols(conn, window, A_WM_PROTOCOLS);
    if xcb_icccm_get_wm_protocols_reply(conn, cookie, &raw mut protocols, std::ptr::null_mut()) != 1
    {
        return false;
    }
    let reply = ProtocolReply(protocols);
    let raw = reply.0._reply;
    if raw.is_null() || (*raw).format != 32 {
        return false;
    }
    let available = ((*raw).length as usize).saturating_mul(4) / std::mem::size_of::<u32>();
    let count = (reply.0.atoms_len as usize).min(available);
    if count == 0 || reply.0.atoms.is_null() {
        return false;
    }
    // XCB owns an aligned allocation of reply.length*4 payload bytes; the
    // successful ICCCM parser supplies its first word pointer. Borrow until wipe.
    std::slice::from_raw_parts(reply.0.atoms, count).contains(&atom)
}

pub unsafe fn x_window_kill(window: xcb_window_t, kill_window: KillWindow) {
    if !window_supports_protocol(window, A_WM_DELETE_WINDOW) {
        if kill_window == KillWindow::Window {
            verboselog(
                c"Killing specific window 0x%08x\n".as_ptr().cast_mut(),
                window,
            );
            xcb_destroy_window(conn, window);
        } else {
            verboselog(
                c"Killing the X11 client which owns window 0x%08x\n"
                    .as_ptr()
                    .cast_mut(),
                window,
            );
            xcb_kill_client(conn, window as u32);
        }
        return;
    }
    let event = delete_window_event(window, A_WM_PROTOCOLS, A_WM_DELETE_WINDOW);
    verboselog(c"Sending WM_DELETE to the client\n".as_ptr().cast_mut());
    xcb_send_event(
        conn,
        0,
        window,
        XCB_EVENT_MASK_NO_EVENT as i32 as u32,
        (&raw const event).cast(),
    );
    xcb_flush(conn);
}
unsafe fn x_draw_title_border(
    con: *mut Con,
    p: *mut deco_render_params,
    dest_surface: *mut surface_t,
) {
    for (x, y, width, height) in title_border((*con).deco_rect) {
        draw_util_rectangle(dest_surface, (*(*p).color).border, x, y, width, height);
    }
}
unsafe fn x_draw_decoration_after_title(
    con: *mut Con,
    p: *mut deco_render_params,
    dest_surface: *mut surface_t,
) {
    if (*con).parent != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con->parent != NULL".as_ptr(),
            c"../src/x.c".as_ptr(),
            381 as u32,
            c"void x_draw_decoration_after_title(Con *, struct deco_render_params *, surface_t *)"
                .as_ptr(),
        );
    };
    let dr: *mut Rect = &raw mut (*con).deco_rect;
    if !font_is_pango() {
        draw_util_rectangle(
            dest_surface,
            (*(*p).color).background,
            (*dr)
                .x
                .wrapping_add((*dr).width)
                .wrapping_sub((2 * logical_px(1)) as u32) as f64,
            (*dr).y as f64,
            (2 * logical_px(1)) as f64,
            (*dr).height as f64,
        );
    }
    x_draw_title_border(con, p, dest_surface);
}
unsafe fn x_get_border_rectangles(con: *mut Con) -> BorderRectangles {
    border_rectangles(
        con_border_style(con),
        con_is_leaf(con),
        con_adjacent_borders(con) as u32 & config.hide_edge_borders as u32,
        con_border_style_rect(con),
        (*con).rect,
    )
}

pub unsafe fn x_draw_decoration(con: *mut Con) {
    let mut next: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut dest_surface: *mut surface_t = ::core::ptr::null_mut::<surface_t>();
    let mut text_offset_y: i32 = 0;
    let mut win: *mut Window = ::core::ptr::null_mut::<Window>();
    let mut deco_width: i32 = 0;
    let mut title_padding: i32 = 0;
    let mut mark_width: i32 = 0;
    let mut icon_size: i32 = 0;
    let mut has_icon: bool = false;
    let mut icon_offset_x: i32 = 0;
    let mut title_offset_x: i32 = 0;
    let parent: *mut Con = (*con).parent as *mut Con;
    let leaf: bool = con_is_leaf(con);
    if !leaf && (*parent).layout != Layout::Stacked && (*parent).layout != Layout::Tabbed
        || (*parent).type_0 as u32 == CT_OUTPUT
        || (*parent).type_0 as u32 == CT_DOCKAREA
        || (*con).type_0 as u32 == CT_FLOATING_CON
    {
        return;
    }
    if (*con).rect.height == 0 as u32 {
        return;
    }
    if leaf && (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE {
        return;
    }
    let color = if (*con).urgent {
        &raw mut config.client.urgent as *mut Colortriple
    } else if con == focused || con_inside_focused(con) {
        &raw mut config.client.focused as *mut Colortriple
    } else if con == (*parent).focus_head.tqh_first {
        if config.client.got_focused_tab_title && !leaf && con_descend_focused(con) == focused {
            &raw mut config.client.focused_tab_title as *mut Colortriple
        } else {
            &raw mut config.client.focused_inactive as *mut Colortriple
        }
    } else {
        &raw mut config.client.unfocused as *mut Colortriple
    };
    let r = (*con).rect;
    let w = (*con).window_rect;
    let candidate = deco_render_params {
        color,
        border_style: con_border_style(con),
        con_rect: width_height {
            w: r.width,
            h: r.height,
        },
        con_window_rect: width_height {
            w: w.width,
            h: w.height,
        },
        con_deco_rect: (*con).deco_rect,
        background: config.client.background,
        con_is_leaf: con_is_leaf(con),
        parent_layout: (*parent).layout,
    };
    if !(*con).deco_render_params.is_null()
        && ((*con).window.is_null() || !(*(*con).window).name_x_changed)
        && !(*parent).pixmap_recreated
        && !(*con).pixmap_recreated
        && !(*con).mark_changed
        && same_decoration(&candidate, &*(*con).deco_render_params)
    {
        // Cached fields match; no allocation or raw padding comparison is needed.
    } else {
        // Other graph modules free this published POD allocation with libc::free.
        let p = scalloc(1, std::mem::size_of::<deco_render_params>()).cast::<deco_render_params>();
        p.write(candidate);
        next = con;
        loop {
            next = (*next).nodes.tqe_next as *mut Con;
            if next.is_null() {
                break;
            }
            free((*next).deco_render_params as *mut ::core::ffi::c_void);
            (*next).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        }
        free((*con).deco_render_params as *mut ::core::ffi::c_void);
        (*con).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
        (*con).deco_render_params = p;
        if !(*con).window.is_null() && (*(*con).window).name_x_changed {
            (*(*con).window).name_x_changed = false;
        }
        (*parent).pixmap_recreated = false;
        (*con).pixmap_recreated = false;
        (*con).mark_changed = false;
        if !(*con).window.is_null() {
            draw_util_clear_surface(
                &raw mut (*con).frame_buffer,
                color_t {
                    red: 0.0f64,
                    green: 0.0f64,
                    blue: 0.0f64,
                    alpha: 0.,
                    colorpixel: 0,
                },
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as f64,
                0 as f64,
                r.width as f64,
                w.y as f64,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as f64,
                w.y.wrapping_add(w.height) as f64,
                r.width as f64,
                r.height.wrapping_sub(w.y.wrapping_add(w.height)) as f64,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                0 as f64,
                0 as f64,
                w.x as f64,
                r.height as f64,
            );
            draw_util_rectangle(
                &raw mut (*con).frame_buffer,
                config.client.background,
                w.x.wrapping_add(w.width) as f64,
                0 as f64,
                r.width.wrapping_sub(w.x.wrapping_add(w.width)) as f64,
                r.height as f64,
            );
        }
        if (*p).border_style != BorderStyle::None && (*p).con_is_leaf {
            let rectangles = x_get_border_rectangles(con);
            for rectangle in rectangles.as_slice() {
                draw_util_rectangle(
                    &raw mut (*con).frame_buffer,
                    (*(*p).color).child_border,
                    rectangle.x as f64,
                    rectangle.y as f64,
                    rectangle.width as f64,
                    rectangle.height as f64,
                );
            }
            let br: Rect = con_border_style_rect(con);
            if (*con).nodes.tqe_next.is_null()
                && (*(*((*con).nodes.tqe_prev as *mut nodes_head)).tqh_last).is_null()
                && (*(*con).parent).type_0 as u32 != CT_FLOATING_CON
            {
                if (*p).parent_layout == Layout::SplitH {
                    draw_util_rectangle(
                        &raw mut (*con).frame_buffer,
                        (*(*p).color).indicator,
                        r.width.wrapping_add(br.width.wrapping_add(br.x)) as f64,
                        br.y as f64,
                        br.width.wrapping_add(br.x).wrapping_neg() as f64,
                        r.height.wrapping_add(br.height) as f64,
                    );
                } else if (*p).parent_layout == Layout::SplitV {
                    draw_util_rectangle(
                        &raw mut (*con).frame_buffer,
                        (*(*p).color).indicator,
                        br.x as f64,
                        r.height.wrapping_add(br.height.wrapping_add(br.y)) as f64,
                        r.width.wrapping_add(br.width) as f64,
                        br.height.wrapping_add(br.y).wrapping_neg() as f64,
                    );
                }
            }
        }
        dest_surface = &raw mut (*parent).frame_buffer;
        if con_draw_decoration_into_frame(con) {
            debuglog(
                c"%s:%s:%d - using con->frame_buffer (for con->name=%s) as dest_surface\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_draw_decoration".as_ptr(),
                602,
                (*con).name,
            );
            dest_surface = &raw mut (*con).frame_buffer;
        } else {
            debuglog(
                c"%s:%s:%d - sticking to parent->frame_buffer = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_draw_decoration".as_ptr(),
                605,
                dest_surface,
            );
        }
        debuglog(
            c"%s:%s:%d - dest_surface %p is %d x %d (id=0x%08x)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_draw_decoration".as_ptr(),
            607,
            dest_surface,
            (*dest_surface).width,
            (*dest_surface).height,
            (*dest_surface).id,
        );
        if !((*dest_surface).id as ::core::ffi::c_long == XCB_NONE) {
            if con == (*(*con).parent).nodes_head.tqh_first {
                free((*(*con).parent).deco_render_params as *mut ::core::ffi::c_void);
                (*(*con).parent).deco_render_params = ::core::ptr::null_mut::<deco_render_params>();
            }
            if !((*p).border_style != BorderStyle::Normal
                || (*con).deco_rect.width == 0 as u32
                || (*con).deco_rect.height == 0 as u32)
            {
                debuglog(
                    c"%s:%s:%d - con->deco_rect = (x=%d, y=%d, w=%d, h=%d) for con->name=%s\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_draw_decoration".as_ptr(),
                    630,
                    (*con).deco_rect.x,
                    (*con).deco_rect.y,
                    (*con).deco_rect.width,
                    (*con).deco_rect.height,
                    (*con).name,
                );
                draw_util_rectangle(
                    dest_surface,
                    (*(*p).color).background,
                    (*con).deco_rect.x as f64,
                    (*con).deco_rect.y as f64,
                    (*con).deco_rect.width as f64,
                    (*con).deco_rect.height as f64,
                );
                x_draw_title_border(con, p, dest_surface);
                text_offset_y = (*con)
                    .deco_rect
                    .height
                    .wrapping_sub(config.font.height as u32)
                    .wrapping_div(2 as u32) as i32;
                win = (*con).window;
                deco_width = (*con).deco_rect.width as i32;
                title_padding = logical_px(2) as i32;
                mark_width = 0;
                if config.show_marks && !(*con).marks_head.tqh_first.is_null() {
                    let mut marks = Vec::new();
                    let mut mark = (*con).marks_head.tqh_first;
                    while !mark.is_null() {
                        marks.push(CStr::from_ptr((*mark).name).to_bytes().to_owned());
                        mark = (*mark).marks.tqe_next;
                    }
                    if let Some(formatted_mark) = format_marks(marks.iter().map(Vec::as_slice)) {
                        let mark_text = I3String::from_bytes(&formatted_mark);
                        let mark_0 = &mark_text;
                        mark_width = predict_text_width(mark_0);
                        let mark_offset_x: i32 = if config.title_align == TitleAlign::Right {
                            title_padding
                        } else {
                            deco_width - mark_width - title_padding
                        };
                        draw_util_text(
                            mark_0,
                            dest_surface,
                            (*(*p).color).text,
                            (*(*p).color).background,
                            (*con).deco_rect.x.wrapping_add(mark_offset_x as u32) as i32,
                            (*con).deco_rect.y.wrapping_add(text_offset_y as u32) as i32,
                            mark_width,
                        );
                        mark_width += title_padding;
                    }
                }
                let owned_title = if !(*con).title_format.is_null() {
                    Some(con_parse_title_format(con))
                } else if win.is_null() {
                    let tree = NativeReply(con_get_tree_representation(con));
                    let mut bytes = b"i3: ".to_vec();
                    bytes.extend_from_slice(CStr::from_ptr(tree.0).to_bytes());
                    Some(I3String::from_bytes(&bytes))
                } else {
                    None
                };
                let title = if let Some(owned) = owned_title.as_ref() {
                    Some(owned)
                } else if !win.is_null() && (*con).title_format.is_null() {
                    (*win).name.as_ref()
                } else {
                    None
                };
                if let Some(title) = title {
                    let placement = title_placement(
                        config.title_align,
                        deco_width,
                        (*con).deco_rect.height,
                        logical_px(2),
                        (*con).window_icon_padding,
                        !win.is_null() && !(*win).icon.is_null(),
                        predict_text_width(title),
                        mark_width,
                        title_padding,
                        logical_px((*con).window_icon_padding.max(1)),
                    );
                    icon_size = placement.icon_size;
                    has_icon = placement.has_icon;
                    icon_offset_x = placement.icon_offset;
                    title_offset_x = placement.title_offset;
                    draw_util_text(
                        title,
                        dest_surface,
                        (*(*p).color).text,
                        (*(*p).color).background,
                        (*con).deco_rect.x.wrapping_add(title_offset_x as u32) as i32,
                        (*con).deco_rect.y.wrapping_add(text_offset_y as u32) as i32,
                        placement.available,
                    );
                    if has_icon {
                        draw_util_image(
                            (*win).icon,
                            dest_surface,
                            (*con).deco_rect.x.wrapping_add(icon_offset_x as u32) as i32,
                            (*con).deco_rect.y.wrapping_add(logical_px(1) as u32) as i32,
                            icon_size,
                            icon_size,
                        );
                    }
                    drop(owned_title);
                    x_draw_decoration_after_title(con, p, dest_surface);
                }
            }
        }
    }
    draw_util_copy_surface(
        &raw mut (*con).frame_buffer,
        &raw mut (*con).frame,
        0 as f64,
        0 as f64,
        0 as f64,
        0 as f64,
        (*con).rect.width as f64,
        (*con).rect.height as f64,
    );
}

pub unsafe fn x_deco_recurse(con: *mut Con) {
    let leaf: bool =
        (*con).nodes_head.tqh_first.is_null() && (*con).floating_head.tqh_first.is_null();
    let state: *const FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    if !leaf {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = (*con).nodes_head.tqh_first as *mut Con;
        while !current.is_null() {
            x_deco_recurse(current);
            current = (*current).nodes.tqe_next as *mut Con;
        }
        current = (*con).floating_head.tqh_first as *mut Con;
        while !current.is_null() {
            x_deco_recurse(current);
            current = (*current).floating_windows.tqe_next as *mut Con;
        }
        if (*state).mapped {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as f64,
                0 as f64,
                0 as f64,
                0 as f64,
                (*con).rect.width as f64,
                (*con).rect.height as f64,
            );
        }
    }
    if (*con).type_0 as u32 != CT_ROOT
        && (*con).type_0 as u32 != CT_OUTPUT
        && (!leaf || (*con).mapped)
    {
        x_draw_decoration(con);
    }
}
unsafe fn set_hidden_state(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    let should_be_hidden: bool = con_is_hidden(con);
    if should_be_hidden == (*state).is_hidden {
        return;
    }
    if should_be_hidden {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_HIDDEN for con = %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_hidden_state".as_ptr(),
            820,
            con,
        );
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_HIDDEN,
        );
    } else {
        debuglog(
            c"%s:%s:%d - removing _NET_WM_STATE_HIDDEN for con = %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_hidden_state".as_ptr(),
            823,
            con,
        );
        xcb_remove_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_HIDDEN,
        );
    }
    (*state).is_hidden = should_be_hidden;
}
unsafe fn set_maximized_state(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    let con_maximized_horz: bool = con_is_maximized(con, Orientation::Horizontal);
    if con_maximized_horz != (*state).is_maximized_horz {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_HORZ for con %p(%s) to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_maximized_state".as_ptr(),
            843,
            con,
            (*con).name,
            con_maximized_horz as i32,
        );
        if con_maximized_horz {
            xcb_add_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_HORZ,
            );
        } else {
            xcb_remove_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_HORZ,
            );
        }
        (*state).is_maximized_horz = con_maximized_horz;
    }
    let con_maximized_vert: bool = con_is_maximized(con, Orientation::Vertical);
    if con_maximized_vert != (*state).is_maximized_vert {
        debuglog(
            c"%s:%s:%d - setting _NET_WM_STATE_MAXIMIZED_VERT for con %p(%s) to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"set_maximized_state".as_ptr(),
            856,
            con,
            (*con).name,
            con_maximized_vert as i32,
        );
        if con_maximized_vert {
            xcb_add_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_VERT,
            );
        } else {
            xcb_remove_property_atom(
                conn,
                (*(*con).window).id,
                A__NET_WM_STATE,
                A__NET_WM_STATE_MAXIMIZED_VERT,
            );
        }
        (*state).is_maximized_vert = con_maximized_vert;
    }
}
unsafe fn x_shape_frame(con: *mut Con, shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            873 as u32,
            c"void x_shape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    xcb_shape_combine(
        conn,
        XCB_SHAPE_SO_SET as i32 as xcb_shape_op_t,
        shape_kind as xcb_shape_kind_t,
        shape_kind as xcb_shape_kind_t,
        (*con).frame.id as xcb_window_t,
        (*con)
            .window_rect
            .x
            .wrapping_add((*con).border_width as u32) as i16,
        (*con)
            .window_rect
            .y
            .wrapping_add((*con).border_width as u32) as i16,
        (*(*con).window).id,
    );
    let rectangles = x_get_border_rectangles(con);
    if !rectangles.as_slice().is_empty() {
        xcb_shape_rectangles(
            conn,
            XCB_SHAPE_SO_UNION as u8,
            shape_kind as u8,
            XCB_CLIP_ORDERING_UNSORTED as u8,
            (*con).frame.id,
            0,
            0,
            rectangles.len as u32,
            rectangles.as_slice().as_ptr(),
        );
    }
}
unsafe fn x_unshape_frame(con: *mut Con, shape_kind: xcb_shape_sk_t) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            893 as u32,
            c"void x_unshape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    xcb_shape_mask(
        conn,
        XCB_SHAPE_SO_SET as i32 as xcb_shape_op_t,
        shape_kind as xcb_shape_kind_t,
        (*con).frame.id as xcb_window_t,
        0 as i16,
        0 as i16,
        XCB_PIXMAP_NONE as i32 as xcb_pixmap_t,
    );
}
unsafe fn set_shape_state(con: *mut Con, need_reshape: bool) {
    if !shape_supported || (*con).window.is_null() {
        return;
    }
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(
            c"ERROR: window state for con %p not found\n"
                .as_ptr()
                .cast_mut(),
            con,
        );
        return;
    }
    if need_reshape && con_is_floating(con) {
        if (*(*con).window).shaped {
            x_shape_frame(con, XCB_SHAPE_SK_BOUNDING);
        }
        if (*(*con).window).input_shaped {
            x_shape_frame(con, XCB_SHAPE_SK_INPUT);
        }
    }
    if (*state).was_floating && !con_is_floating(con) {
        if (*(*con).window).shaped {
            x_unshape_frame(con, XCB_SHAPE_SK_BOUNDING);
        }
        if (*(*con).window).input_shaped {
            x_unshape_frame(con, XCB_SHAPE_SK_INPUT);
        }
    }
}

pub unsafe fn x_push_node(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut rect: Rect = (*con).rect as Rect;
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    if let Some(name) = (*state).name.take() {
        debuglog(
            c"pushing name %s for con %p\n".as_ptr().cast_mut(),
            name.as_ptr(),
            con,
        );
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as u8,
            (*con).frame.id,
            XCB_ATOM_WM_NAME as u32,
            XCB_ATOM_STRING as u32,
            8,
            name.as_bytes().len() as u32,
            name.as_ptr().cast(),
        );
    }
    if (*con).window.is_null()
        && ((*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed)
    {
        let mut max_y: u32 = 0 as u32;
        let mut max_height: u32 = 0 as u32;
        current = (*con).nodes_head.tqh_first as *mut Con;
        while !current.is_null() {
            let dr: *mut Rect = &raw mut (*current).deco_rect;
            if (*dr).y >= max_y && (*dr).height >= max_height {
                max_y = (*dr).y;
                max_height = (*dr).height;
            }
            current = (*current).nodes.tqe_next as *mut Con;
        }
        rect.height = max_y.wrapping_add(max_height);
        if rect.height == 0 as u32 {
            (*con).mapped = false;
        }
    } else if (*con).window.is_null() {
        (*con).mapped = false;
    }
    let mut need_reshape: bool = false;
    if (*state).need_reparent && !(*con).window.is_null() {
        debuglog(
            c"%s:%s:%d - Reparenting child window\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            978,
        );
        let mut values: [u32; 1] = [XCB_NONE as u32];
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as i32 as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as i32 as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_reparent_window(
            conn,
            (*(*con).window).id,
            (*con).frame.id as xcb_window_t,
            0 as i16,
            0 as i16,
        );
        values[0] = (XCB_EVENT_MASK_BUTTON_PRESS as i32
            | XCB_EVENT_MASK_BUTTON_RELEASE as i32
            | XCB_EVENT_MASK_POINTER_MOTION as i32
            | XCB_EVENT_MASK_EXPOSURE as i32
            | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
            | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
            | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
            | XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32;
        xcb_change_window_attributes(
            conn,
            (*state).old_frame,
            XCB_CW_EVENT_MASK as i32 as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        values[0] = (XCB_EVENT_MASK_PROPERTY_CHANGE as i32
            | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
            | XCB_EVENT_MASK_FOCUS_CHANGE as i32) as u32;
        xcb_change_window_attributes(
            conn,
            (*(*con).window).id,
            XCB_CW_EVENT_MASK as i32 as u32,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        (*state).old_frame = XCB_NONE as xcb_window_t;
        (*state).need_reparent = false;
        (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
        debuglog(
            c"%s:%s:%d - ignore_unmap for reparenting of con %p (win 0x%08x) is now %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            999,
            con,
            (*(*con).window).id,
            (*con).ignore_unmap as i32,
        );
        need_reshape = true;
    }
    need_reshape = needs_reshape(
        (*state).rect,
        rect,
        (*state).window_rect,
        (*con).window_rect,
        con_is_floating(con),
        (*state).was_floating,
        need_reshape,
    );
    let is_pixmap_needed = needs_pixmap(
        con_is_leaf(con),
        con_border_style(con),
        (*con).layout,
        (*con).type_0 as i32,
    );
    debuglog(
        c"%s:%s:%d - Con %p (layout %d), is_pixmap_needed = %s, rect.height = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_node".as_ptr(),
        1019,
        con,
        (*con).layout as u32,
        if is_pixmap_needed {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
        (*con).rect.height,
    );
    let mut fake_notify: bool = false;
    if is_pixmap_needed && (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE
        || !rect_equals((*state).rect, rect) && rect.height > 0 as u32
    {
        let has_rect_changed: bool = (*state).rect.x != rect.x
            || (*state).rect.y != rect.y
            || (*state).rect.width != rect.width
            || (*state).rect.height != rect.height;
        if !is_pixmap_needed && (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
            xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
            (*con).frame_buffer.id = XCB_NONE as xcb_drawable_t;
        }
        if is_pixmap_needed
            && (has_rect_changed || (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE)
        {
            if (*con).frame_buffer.id as ::core::ffi::c_long == XCB_NONE {
                (*con).frame_buffer.id = xcb_generate_id(conn) as xcb_drawable_t;
            } else {
                draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
                xcb_free_pixmap(conn, (*con).frame_buffer.id as xcb_pixmap_t);
            }
            let mut win_depth: u16 = root_depth as u16;
            if !(*con).window.is_null() {
                win_depth = (*(*con).window).depth;
            }
            let (width, height) = pixmap_dimensions(rect);
            debuglog(
                c"%s:%s:%d - creating %d x %d pixmap for con %p (con->frame_buffer.id = (pixmap_t)0x%08x) (con->frame.id (drawable_t)0x%08x)\n".as_ptr().cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node".as_ptr(),
                1068,
                width,
                height,
                con,
                (*con).frame_buffer.id,
                (*con).frame.id,
            );
            xcb_create_pixmap(
                conn,
                win_depth as u8,
                (*con).frame_buffer.id as xcb_pixmap_t,
                (*con).frame.id,
                width as u16,
                height as u16,
            );
            draw_util_surface_init(
                conn,
                &raw mut (*con).frame_buffer,
                (*con).frame_buffer.id,
                get_visualtype_by_id(get_visualid_by_depth(win_depth)),
                width,
                height,
            );
            draw_util_clear_surface(
                &raw mut (*con).frame_buffer,
                color_t {
                    red: 0.0f64,
                    green: 0.0f64,
                    blue: 0.0f64,
                    alpha: 0.,
                    colorpixel: 0,
                },
            );
            let mut fresh4: [u32; 1] = [0 as u32];
            xcb_change_gc(
                conn,
                (*con).frame_buffer.gc,
                XCB_GC_GRAPHICS_EXPOSURES as i32 as u32,
                &raw mut fresh4 as *mut u32 as *const ::core::ffi::c_void,
            );
            draw_util_surface_set_size(&raw mut (*con).frame, width, height);
            (*con).pixmap_recreated = true;
            if (*con).parent.is_null()
                || (*(*con).parent).layout != Layout::Stacked
                || (*(*con).parent).focus_head.tqh_first == con
            {
                x_deco_recurse(con);
            }
        }
        debuglog(
            c"%s:%s:%d - setting rect (%d, %d, %d, %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1097,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        );
        xcb_flush(conn);
        xcb_set_window_rect(conn, (*con).frame.id as xcb_window_t, rect);
        if (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as f64,
                0 as f64,
                0 as f64,
                0 as f64,
                (*con).rect.width as f64,
                (*con).rect.height as f64,
            );
        }
        xcb_flush(conn);
        (*state).rect = rect;
        fake_notify = true;
    }
    if !(*con).window.is_null() && !rect_equals((*state).window_rect, (*con).window_rect as Rect) {
        debuglog(
            c"%s:%s:%d - setting window rect (%d, %d, %d, %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1117,
            (*con).window_rect.x,
            (*con).window_rect.y,
            (*con).window_rect.width,
            (*con).window_rect.height,
        );
        xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
        (*state).window_rect = (*con).window_rect;
        fake_notify = true;
    }
    set_shape_state(con, need_reshape);
    if !con.is_null() && !(*con).window.is_null() {
        let bsr: Rect = con_border_style_rect(con);
        let extents = frame_extents(bsr);
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as i32 as u8,
            (*(*con).window).id,
            A__NET_FRAME_EXTENTS,
            XCB_ATOM_CARDINAL as i32 as xcb_atom_t,
            32 as u8,
            4 as u32,
            extents.as_ptr().cast(),
        );
    }
    if ((*state).mapped != (*con).mapped || !(*con).window.is_null() && !(*state).child_mapped)
        && (*con).mapped
    {
        let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
        if !(*con).window.is_null() {
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_NORMAL as i32 as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as i32 as u8,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as u8,
                2 as u32,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let mut values_0: [u32; 1] = [0; 1];
        if !(*state).child_mapped && !(*con).window.is_null() {
            cookie = xcb_map_window(conn, (*(*con).window).id);
            values_0[0] = (XCB_EVENT_MASK_PROPERTY_CHANGE as i32
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
                | XCB_EVENT_MASK_FOCUS_CHANGE as i32) as u32;
            xcb_change_window_attributes(
                conn,
                (*(*con).window).id,
                XCB_CW_EVENT_MASK as i32 as u32,
                &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
            );
            debuglog(
                c"%s:%s:%d - mapping child window (serial %d)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node".as_ptr(),
                1166,
                cookie.sequence,
            );
            (*state).child_mapped = true;
        }
        cookie = xcb_map_window(conn, (*con).frame.id as xcb_window_t);
        values_0[0] = (XCB_EVENT_MASK_BUTTON_PRESS as i32
            | XCB_EVENT_MASK_BUTTON_RELEASE as i32
            | XCB_EVENT_MASK_POINTER_MOTION as i32
            | XCB_EVENT_MASK_EXPOSURE as i32
            | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
            | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
            | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
            | XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32;
        xcb_change_window_attributes(
            conn,
            (*con).frame.id as xcb_window_t,
            XCB_CW_EVENT_MASK as i32 as u32,
            &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
        );
        if (*con).frame_buffer.id as ::core::ffi::c_long != XCB_NONE {
            draw_util_copy_surface(
                &raw mut (*con).frame_buffer,
                &raw mut (*con).frame,
                0 as f64,
                0 as f64,
                0 as f64,
                0 as f64,
                (*con).rect.width as f64,
                (*con).rect.height as f64,
            );
        }
        xcb_flush(conn);
        debuglog(
            c"%s:%s:%d - mapping container %08x (serial %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1181,
            (*con).frame.id,
            cookie.sequence,
        );
        (*state).mapped = (*con).mapped;
    }
    (*state).unmap_now = (*state).mapped != (*con).mapped && !(*con).mapped;
    (*state).was_floating = con_is_floating(con);
    if fake_notify {
        debuglog(
            c"%s:%s:%d - Sending fake configure notify\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node".as_ptr(),
            1189,
        );
        fake_absolute_configure_notify(con);
    }
    set_hidden_state(con);
    set_maximized_state(con);
    current = (*con).focus_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node(current);
        current = (*current).focused.tqe_next as *mut Con;
    }
}
unsafe fn x_push_node_unmaps(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).unmap_now {
        if !(*con).window.is_null() {
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_WITHDRAWN as i32 as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as i32 as u8,
                (*(*con).window).id,
                A_WM_STATE,
                A_WM_STATE,
                32 as u8,
                2 as u32,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        }
        let cookie: xcb_void_cookie_t =
            xcb_unmap_window(conn, (*con).frame.id as xcb_window_t) as xcb_void_cookie_t;
        debuglog(
            c"%s:%s:%d - unmapping container %p / %s (serial %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_node_unmaps".as_ptr(),
            1229,
            con,
            (*con).name,
            cookie.sequence,
        );
        if !(*con).window.is_null() {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
            debuglog(
                c"%s:%s:%d - ignore_unmap for con %p (frame 0x%08x) now %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_node_unmaps".as_ptr(),
                1235,
                con,
                (*con).frame.id,
                (*con).ignore_unmap as i32,
            );
        }
        (*state).mapped = (*con).mapped;
    }
    current = (*con).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node_unmaps(current);
        current = (*current).nodes.tqe_next as *mut Con;
    }
    current = (*con).floating_head.tqh_first as *mut Con;
    while !current.is_null() {
        x_push_node_unmaps(current);
        current = (*current).floating_windows.tqe_next as *mut Con;
    }
}
unsafe fn is_con_attached(con: *mut Con) -> bool {
    if (*con).parent.is_null() {
        return false;
    }
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = (*(*con).parent).nodes_head.tqh_first as *mut Con;
    while !current.is_null() {
        if current == con {
            return true;
        }
        current = (*current).nodes.tqe_next as *mut Con;
    }
    false
}

pub unsafe fn x_push_changes(con: *mut Con) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    let mut pointercookie: xcb_query_pointer_cookie_t = xcb_query_pointer_cookie_t { sequence: 0 };
    if !warp_to.is_null() {
        pointercookie = xcb_query_pointer(conn, root);
    }
    debuglog(
        c"%s:%s:%d - -- PUSHING WINDOW STACK --\n"
            .as_ptr()
            .cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1290,
    );
    let mut values: [u32; 1] = [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32 as u32];
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as i32 as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut FrameState;
    }
    let mut order_changed: bool = false;
    let mut stacking_changed: bool = false;
    let mut cnt: i32 = 0;
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if con_has_managed_window((*state).con) {
            cnt += 1;
        }
        state = (*state).state.cqe_prev as *mut FrameState;
    }
    let mut client_list_windows = Vec::with_capacity(cnt as usize);
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if con_has_managed_window((*state).con) {
            client_list_windows.push((*(*(*state).con).window).id);
        }
        let prev: *mut FrameState = (*state).state.cqe_prev as *mut FrameState;
        let old_prev: *mut FrameState = (*state).old_state.cqe_prev as *mut FrameState;
        if prev != old_prev {
            order_changed = true;
        }
        if ((*state).initial || order_changed)
            && prev != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
        {
            stacking_changed = true;
            let mut mask: u32 = 0 as u32;
            mask |= XCB_CONFIG_WINDOW_SIBLING as i32 as u32;
            mask |= XCB_CONFIG_WINDOW_STACK_MODE as i32 as u32;
            let mut values_0: [u32; 2] = [(*state).id, XCB_STACK_MODE_ABOVE as i32 as u32];
            xcb_configure_window(
                conn,
                (*prev).id,
                mask as u16,
                &raw mut values_0 as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        (*state).initial = false;
        state = (*state).state.cqe_prev as *mut FrameState;
    }
    if stacking_changed {
        debuglog(
            c"%s:%s:%d - Client list changed (%i clients)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_changes".as_ptr(),
            1350,
            cnt,
        );
        ewmh_update_client_list_stacking(
            client_list_windows.as_mut_ptr(),
            client_list_windows.len() as i32,
        );
        client_list_windows.clear();
        state = initial_mapping_head.tqh_first as *mut FrameState;
        while !state.is_null() {
            if con_has_managed_window((*state).con) {
                client_list_windows.push((*(*(*state).con).window).id);
            }
            state = (*state).initial_mapping_order.tqe_next as *mut FrameState;
        }
        ewmh_update_client_list(
            client_list_windows.as_mut_ptr(),
            client_list_windows.len() as i32,
        );
    }
    debuglog(
        c"%s:%s:%d - PUSHING CHANGES\n".as_ptr().cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1365,
    );
    x_push_node(con);
    if !warp_to.is_null() {
        let pointer_reply = NativeReply(xcb_query_pointer_reply(
            conn,
            pointercookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ));
        let pointerreply = pointer_reply.0;
        if pointerreply.is_null() {
            errorlog(
                c"ERROR: Could not query pointer position, not warping pointer\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            let (mid_x, mid_y) = warp_midpoint(*warp_to);
            let current: *mut Output =
                get_output_containing((*pointerreply).root_x as u32, (*pointerreply).root_y as u32);
            let target: *mut Output = get_output_containing(mid_x as u32, mid_y as u32);
            if current != target {
                let mut fresh7: [u32; 1] = [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32 as u32];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as i32 as u32,
                    &raw mut fresh7 as *mut u32 as *const ::core::ffi::c_void,
                );
                xcb_warp_pointer(
                    conn,
                    XCB_NONE as xcb_window_t,
                    root,
                    0 as i16,
                    0 as i16,
                    0 as u16,
                    0 as u16,
                    mid_x as i16,
                    mid_y as i16,
                );
                let mut fresh8: [u32; 1] = [(XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
                    | XCB_EVENT_MASK_BUTTON_PRESS as i32
                    | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
                    | XCB_EVENT_MASK_POINTER_MOTION as i32
                    | XCB_EVENT_MASK_PROPERTY_CHANGE as i32
                    | XCB_EVENT_MASK_FOCUS_CHANGE as i32
                    | XCB_EVENT_MASK_ENTER_WINDOW as i32)
                    as u32];
                xcb_change_window_attributes(
                    conn,
                    root,
                    XCB_CW_EVENT_MASK as i32 as u32,
                    &raw mut fresh8 as *mut u32 as *const ::core::ffi::c_void,
                );
            }
        }
        warp_to = ::core::ptr::null_mut::<Rect>();
    }
    values[0] = (XCB_EVENT_MASK_BUTTON_PRESS as i32
        | XCB_EVENT_MASK_BUTTON_RELEASE as i32
        | XCB_EVENT_MASK_POINTER_MOTION as i32
        | XCB_EVENT_MASK_EXPOSURE as i32
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32;
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as i32 as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut FrameState;
    }
    x_deco_recurse(con);
    let mut to_focus: xcb_window_t = (*focused).frame.id as xcb_window_t;
    if !(*focused).window.is_null() {
        to_focus = (*(*focused).window).id;
    }
    if focused_id != to_focus {
        let action = focus_action(
            (*focused).mapped,
            !(*focused).window.is_null() && (*(*focused).window).needs_take_focus,
            !(*focused).window.is_null() && (*(*focused).window).doesnt_accept_focus,
        );
        if action == FocusAction::Unmapped {
            debuglog(
                c"%s:%s:%d - Not updating focus (to %p / %s), focused window is not mapped.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/x.c".as_ptr(),
                c"x_push_changes".as_ptr(),
                1406,
                focused,
                (*focused).name,
            );
            focused_id = XCB_NONE as xcb_window_t;
        } else {
            if action == FocusAction::TakeFocus {
                debuglog(
                    c"%s:%s:%d - Updating focus by sending WM_TAKE_FOCUS to window 0x%08x (focused: %p / %s)\n".as_ptr().cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_push_changes".as_ptr(),
                    1414,
                    to_focus,
                    focused,
                    (*focused).name,
                );
                send_take_focus(to_focus, last_timestamp);
                change_ewmh_focus(
                    if con_has_managed_window(focused) {
                        (*(*focused).window).id
                    } else {
                        XCB_WINDOW_NONE as i32 as xcb_window_t
                    },
                    last_focused,
                );
                if to_focus != last_focused && is_con_attached(focused) {
                    ipc_send_window_event(c"focus".as_ptr(), focused);
                }
            } else {
                debuglog(
                    c"%s:%s:%d - Updating focus (focused: %p / %s) to X11 window 0x%08x\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/x.c".as_ptr(),
                    c"x_push_changes".as_ptr(),
                    1423,
                    focused,
                    (*focused).name,
                    to_focus,
                );
                if !(*focused).window.is_null() {
                    values[0] = ((XCB_EVENT_MASK_PROPERTY_CHANGE as i32
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
                        | XCB_EVENT_MASK_FOCUS_CHANGE as i32)
                        & !(XCB_EVENT_MASK_FOCUS_CHANGE as i32))
                        as u32;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as i32 as u32,
                        &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                    );
                }
                xcb_set_input_focus(
                    conn,
                    XCB_INPUT_FOCUS_POINTER_ROOT as i32 as u8,
                    to_focus,
                    last_timestamp,
                );
                if !(*focused).window.is_null() {
                    values[0] = (XCB_EVENT_MASK_PROPERTY_CHANGE as i32
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
                        | XCB_EVENT_MASK_FOCUS_CHANGE as i32)
                        as u32;
                    xcb_change_window_attributes(
                        conn,
                        (*(*focused).window).id,
                        XCB_CW_EVENT_MASK as i32 as u32,
                        &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                    );
                }
                change_ewmh_focus(
                    if con_has_managed_window(focused) {
                        (*(*focused).window).id
                    } else {
                        XCB_WINDOW_NONE as i32 as xcb_window_t
                    },
                    last_focused,
                );
                if to_focus as ::core::ffi::c_long != XCB_NONE
                    && to_focus != last_focused
                    && !(*focused).window.is_null()
                    && is_con_attached(focused)
                {
                    ipc_send_window_event(c"focus".as_ptr(), focused);
                }
            }
            last_focused = to_focus;
            focused_id = last_focused;
        }
    }
    if focused_id as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            c"%s:%s:%d - Still no window focused, better set focus to the EWMH support window (%d)\n".as_ptr().cast_mut(),
            c"../src/x.c".as_ptr(),
            c"x_push_changes".as_ptr(),
            1451,
            ewmh_window,
        );
        xcb_set_input_focus(
            conn,
            XCB_INPUT_FOCUS_POINTER_ROOT as i32 as u8,
            ewmh_window,
            last_timestamp,
        );
        change_ewmh_focus(XCB_WINDOW_NONE as i32 as xcb_window_t, last_focused);
        focused_id = ewmh_window;
        last_focused = XCB_NONE as xcb_window_t;
    }
    xcb_flush(conn);
    debuglog(
        c"%s:%s:%d - ENDING CHANGES\n".as_ptr().cast_mut(),
        c"../src/x.c".as_ptr(),
        c"x_push_changes".as_ptr(),
        1460,
    );
    values[0] = ((XCB_EVENT_MASK_BUTTON_PRESS as i32
        | XCB_EVENT_MASK_BUTTON_RELEASE as i32
        | XCB_EVENT_MASK_POINTER_MOTION as i32
        | XCB_EVENT_MASK_EXPOSURE as i32
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_ENTER_WINDOW as i32)
        & !(XCB_EVENT_MASK_ENTER_WINDOW as i32)) as u32;
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).unmap_now {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as i32 as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut FrameState;
    }
    x_push_node_unmaps(con);
    state = state_head.cqh_first as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).old_state.cqe_next
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
        {
            old_state_head.cqh_last = (*state).old_state.cqe_prev;
        } else {
            (*(*state).old_state.cqe_next).old_state.cqe_prev = (*state).old_state.cqe_prev;
        }
        if (*state).old_state.cqe_prev
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
        {
            old_state_head.cqh_first = (*state).old_state.cqe_next;
        } else {
            (*(*state).old_state.cqe_prev).old_state.cqe_next = (*state).old_state.cqe_next;
        }
        (*state).old_state.cqe_next =
            &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState;
        (*state).old_state.cqe_prev = old_state_head.cqh_last;
        if old_state_head.cqh_first
            == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
        {
            old_state_head.cqh_first = state as *mut FrameState;
        } else {
            (*old_state_head.cqh_last).old_state.cqe_next = state as *mut FrameState;
        }
        old_state_head.cqh_last = state as *mut FrameState;
        state = (*state).state.cqe_next as *mut FrameState;
    }
    xcb_flush(conn);
}

pub unsafe fn x_raise_con(con: *const Con) {
    let state: *mut FrameState = state_for_frame((*con).frame.id as xcb_window_t);
    if (*state).state.cqe_next == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        state_head.cqh_last = (*state).state.cqe_prev;
    } else {
        (*(*state).state.cqe_next).state.cqe_prev = (*state).state.cqe_prev;
    }
    if (*state).state.cqe_prev == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        state_head.cqh_first = (*state).state.cqe_next;
    } else {
        (*(*state).state.cqe_prev).state.cqe_next = (*state).state.cqe_next;
    }
    (*state).state.cqe_next = state_head.cqh_first;
    (*state).state.cqe_prev = &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState;
    if state_head.cqh_last == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        state_head.cqh_last = state as *mut FrameState;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state as *mut FrameState;
    }
    state_head.cqh_first = state as *mut FrameState;
}

pub unsafe fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id as xcb_window_t);
    if state.is_null() {
        errorlog(c"ERROR: window state not found\n".as_ptr().cast_mut());
        return;
    }
    // The incoming bytes may alias the previous name: clone before replacement.
    let name = CStr::from_ptr(name).to_owned();
    (*state).name = Some(name);
}

pub unsafe fn update_shmlog_atom() {
    if *shmlogname as i32 == '\0' as i32 {
        xcb_delete_property(conn, root, A_I3_SHMLOG_PATH);
    } else {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as i32 as u8,
            root,
            A_I3_SHMLOG_PATH,
            A_UTF8_STRING,
            8 as u8,
            strlen(shmlogname) as u32,
            shmlogname as *const ::core::ffi::c_void,
        );
    };
}

pub unsafe fn x_set_i3_atoms() {
    let mut pid: libc::pid_t = getpid() as libc::pid_t;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        root,
        A_I3_SOCKET_PATH,
        A_UTF8_STRING,
        8 as u8,
        (if current_socketpath.is_null() {
            0 as usize
        } else {
            strlen(current_socketpath)
        }) as u32,
        current_socketpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        root,
        A_I3_PID,
        XCB_ATOM_CARDINAL as i32 as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut pid as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        root,
        A_I3_CONFIG_PATH,
        A_UTF8_STRING,
        8 as u8,
        strlen(current_configpath) as u32,
        current_configpath as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as i32 as u8,
        root,
        A_I3_LOG_STREAM_SOCKET_PATH,
        A_UTF8_STRING,
        8 as u8,
        strlen(current_log_stream_socket_path) as u32,
        current_log_stream_socket_path as *const ::core::ffi::c_void,
    );
    update_shmlog_atom();
}

pub unsafe fn x_set_warp_to(rect: *mut Rect) {
    if config.mouse_warping != Warping::None {
        warp_to = rect;
    }
}

pub unsafe fn x_mask_event_mask(mask: u32) {
    let mut values: [u32; 1] = [(XCB_EVENT_MASK_BUTTON_PRESS as i32
        | XCB_EVENT_MASK_BUTTON_RELEASE as i32
        | XCB_EVENT_MASK_POINTER_MOTION as i32
        | XCB_EVENT_MASK_EXPOSURE as i32
        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as i32
        | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY as i32
        | XCB_EVENT_MASK_ENTER_WINDOW as i32) as u32
        & mask];
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_head.cqh_last as *mut FrameState;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            xcb_change_window_attributes(
                conn,
                (*state).id,
                XCB_CW_EVENT_MASK as i32 as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
        }
        state = (*state).state.cqe_prev as *mut FrameState;
    }
}

pub unsafe fn x_set_shape(con: *mut Con, kind: xcb_shape_sk_t, enable: bool) {
    if state_for_frame((*con).frame.id as xcb_window_t).is_null() {
        errorlog(
            c"ERROR: window state for con %p not found\n"
                .as_ptr()
                .cast_mut(),
            con,
        );
        return;
    }
    match kind as u32 {
        0 => {
            (*(*con).window).shaped = enable;
        }
        2 => {
            (*(*con).window).input_shaped = enable;
        }
        _ => {
            errorlog(
                c"ERROR: Received unknown shape event kind for con %p. This is a bug.\n"
                    .as_ptr()
                    .cast_mut(),
                con,
            );
            return;
        }
    }
    if con_is_floating(con) {
        if enable {
            x_shape_frame(con, kind);
        } else {
            x_unshape_frame(con, kind);
        }
        xcb_flush(conn);
    }
}
// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_active_window;
use crate::ewmh::ewmh_update_client_list;
use crate::ewmh::ewmh_update_client_list_stacking;
use crate::ewmh::ewmh_update_focused;
use crate::xcb::get_visualid_by_depth;
use crate::xcb::send_take_focus;
use i3_common::dpi::logical_px;
use i3_common::font::font_is_pango;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_change_gc, xcb_change_property, xcb_change_window_attributes,
    xcb_client_message_event_t, xcb_colormap_t, xcb_configure_window, xcb_create_colormap,
    xcb_create_pixmap, xcb_delete_property, xcb_destroy_window, xcb_drawable_t, xcb_flush,
    xcb_free_colormap, xcb_free_pixmap, xcb_generate_id, xcb_generic_error_t,
    xcb_get_property_cookie_t, xcb_get_property_reply_t, xcb_icccm_get_wm_protocols,
    xcb_icccm_get_wm_protocols_reply, xcb_icccm_get_wm_protocols_reply_t,
    xcb_icccm_get_wm_protocols_reply_wipe, xcb_kill_client, xcb_map_window, xcb_pixmap_t,
    xcb_query_pointer, xcb_query_pointer_cookie_t, xcb_query_pointer_reply,
    xcb_query_pointer_reply_t, xcb_rectangle_t, xcb_reparent_window, xcb_send_event,
    xcb_set_input_focus, xcb_shape_combine, xcb_shape_kind_t, xcb_shape_mask, xcb_shape_op_t,
    xcb_shape_rectangles, xcb_shape_sk_t, xcb_unmap_window, xcb_visualid_t, xcb_void_cookie_t,
    xcb_warp_pointer, xcb_window_t, XCB_ATOM_CARDINAL, XCB_ATOM_STRING, XCB_ATOM_WM_CLASS,
    XCB_ATOM_WM_NAME, XCB_CLIENT_MESSAGE, XCB_CLIP_ORDERING_UNSORTED, XCB_COLORMAP_ALLOC_NONE,
    XCB_CONFIG_WINDOW_SIBLING, XCB_CONFIG_WINDOW_STACK_MODE, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL,
    XCB_CW_BORDER_PIXEL, XCB_CW_COLORMAP, XCB_CW_EVENT_MASK, XCB_CW_OVERRIDE_REDIRECT,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_ENTER_WINDOW,
    XCB_EVENT_MASK_EXPOSURE, XCB_EVENT_MASK_FOCUS_CHANGE, XCB_EVENT_MASK_NO_EVENT,
    XCB_EVENT_MASK_POINTER_MOTION, XCB_EVENT_MASK_PROPERTY_CHANGE, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY, XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT,
    XCB_GC_GRAPHICS_EXPOSURES, XCB_ICCCM_WM_STATE_NORMAL, XCB_ICCCM_WM_STATE_WITHDRAWN,
    XCB_INPUT_FOCUS_POINTER_ROOT, XCB_NONE, XCB_PIXMAP_NONE, XCB_PROP_MODE_REPLACE,
    XCB_SHAPE_SK_BOUNDING, XCB_SHAPE_SK_INPUT, XCB_SHAPE_SO_SET, XCB_SHAPE_SO_UNION,
    XCB_STACK_MODE_ABOVE, XCB_WINDOW_CLASS_INPUT_OUTPUT, XCB_WINDOW_NONE,
};

use libc::{free, getpid, strlen};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
use crate::types::deco_render_params;
use crate::types::nodes_head;
use crate::types::width_height;
use crate::types::xoutput;
use crate::types::Colortriple;
use crate::types::Con;
use crate::types::Rect;
use crate::types::Window;
use i3_common::draw_util::color_t;
use i3_common::draw_util::surface_t;
use i3_common::string::I3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::colormap;
use crate::application::conn;
use crate::application::last_timestamp;
use crate::application::root;
use crate::application::root_depth;
use crate::application::root_screen;
use crate::application::shape_supported;
use crate::application::A_I3_CONFIG_PATH;
use crate::application::A_I3_LOG_STREAM_SOCKET_PATH;
use crate::application::A_I3_PID;
use crate::application::A_I3_SHMLOG_PATH;
use crate::application::A_I3_SOCKET_PATH;
use crate::application::A_UTF8_STRING;
use crate::application::A_WM_DELETE_WINDOW;
use crate::application::A_WM_PROTOCOLS;
use crate::application::A_WM_STATE;
use crate::application::A__NET_FRAME_EXTENTS;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_HIDDEN;
use crate::application::A__NET_WM_STATE_MAXIMIZED_HORZ;
use crate::application::A__NET_WM_STATE_MAXIMIZED_VERT;
use crate::con::con_adjacent_borders;
use crate::con::con_border_style;
use crate::con::con_border_style_rect;
use crate::con::con_descend_focused;
use crate::con::con_draw_decoration_into_frame;
use crate::con::con_get_tree_representation;
use crate::con::con_has_managed_window;
use crate::con::con_inside_focused;
use crate::con::con_is_floating;
use crate::con::con_is_hidden;
use crate::con::con_is_leaf;
use crate::con::con_is_maximized;
use crate::con::con_parse_title_format;
use crate::config::config;
use crate::config::current_configpath;
use crate::ewmh::ewmh_window;
use crate::ipc::current_socketpath;
use crate::ipc::ipc_send_window_event;
use crate::log::current_log_stream_socket_path;
use crate::log::shmlogname;
use crate::randr::get_output_containing;
use crate::tree::focused;
use crate::util::rect_equals;
use crate::xcb::create_window;
use crate::xcb::fake_absolute_configure_notify;
use crate::xcb::get_visualtype_by_id;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;
use crate::xcb::xcb_set_window_rect;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_copy_surface;
use i3_common::draw_util::draw_util_image;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::font::predict_text_width;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn marks_and_delete_wire_preserve_raw_bytes_and_protocol_fields() {
        assert_eq!(
            format_marks([b"_hidden".as_slice(), b"", b"raw\xff"]),
            Some(b"[][raw\xff]".to_vec())
        );
        assert_eq!(format_marks([b"_hidden".as_slice()]), None);
        let event = delete_window_event(0xffff_ffff, 12, 13);
        assert_eq!(
            (
                event.response_type,
                event.format,
                event.sequence,
                event.window,
                event.type_0
            ),
            (33, 32, 0, u32::MAX, 12)
        );
        assert_eq!(unsafe { event.data.data32 }, [13, 0, 0, 0, 0]);
        assert_eq!(std::mem::size_of_val(&event), 32);
    }
    #[test]
    fn border_geometry_retains_hidden_edges_and_native_narrowing() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 80,
        };
        let border = Rect {
            x: 2,
            y: 2,
            width: (-4i32) as u32,
            height: (-4i32) as u32,
        };
        let all = border_rectangles(BorderStyle::Pixel, true, 0, border, rect);
        let tuples: Vec<_> = all
            .as_slice()
            .iter()
            .map(|r| (r.x, r.y, r.width, r.height))
            .collect();
        assert_eq!(
            tuples,
            [(0, 0, 2, 80), (98, 0, 2, 80), (2, 78, 96, 2), (2, 0, 96, 2)]
        );
        assert_eq!(
            border_rectangles(
                BorderStyle::Normal,
                true,
                ADJ_LEFT_SCREEN_EDGE,
                border,
                rect
            )
            .len,
            2
        );
        assert_eq!(
            border_rectangles(BorderStyle::None, true, 0, border, rect).len,
            0
        );
        assert_eq!(frame_extents(border), [2, 2, 2, 2]);
        assert_eq!(
            pixmap_dimensions(Rect {
                width: u32::MAX,
                height: 0,
                ..rect
            }),
            (1, 1)
        );
        assert_eq!(
            warp_midpoint(Rect {
                x: (-10i32) as u32,
                y: 0,
                width: 21,
                height: 3
            }),
            (0, 1)
        );
    }
    #[test]
    fn title_alignment_preserves_icon_and_mark_constraints() {
        let left = title_placement(TitleAlign::Left, 100, 18, 2, 1, true, 30, 10, 2, 1);
        assert!(left.has_icon);
        assert_eq!(
            (
                left.icon_size,
                left.icon_offset,
                left.title_offset,
                left.available
            ),
            (16, 1, 20, 68)
        );
        let center = title_placement(TitleAlign::Center, 100, 18, 2, -1, true, 30, 10, 2, 1);
        assert!(!center.has_icon);
        assert_eq!(
            (center.icon_offset, center.title_offset, center.available),
            (29, 29, 86)
        );
        let right = title_placement(TitleAlign::Right, 100, 18, 2, 1, true, 30, 10, 2, 1);
        assert_eq!((right.title_offset, right.icon_offset), (50, 81));
    }
    #[test]
    fn cached_decoration_uses_field_bits_without_padding() {
        let color = color_t {
            red: 0.0,
            green: 0.0,
            blue: f64::NAN,
            alpha: 1.0,
            colorpixel: 7,
        };
        let mut first = deco_render_params {
            color: std::ptr::null_mut(),
            border_style: BorderStyle::Normal,
            con_rect: width_height { w: 100, h: 80 },
            con_window_rect: width_height { w: 96, h: 60 },
            con_deco_rect: Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 18,
            },
            background: color,
            parent_layout: Layout::SplitH,
            con_is_leaf: true,
        };
        let second = first;
        assert!(same_decoration(&first, &second));
        first.background.red = -0.0;
        assert!(!same_decoration(&first, &second));
        assert_eq!(focus_action(false, true, true), FocusAction::Unmapped);
        assert_eq!(focus_action(true, true, true), FocusAction::TakeFocus);
        assert_eq!(focus_action(true, true, false), FocusAction::Direct);
        assert!(!needs_pixmap(
            true,
            BorderStyle::Normal,
            Layout::Stacked,
            CT_ROOT as i32
        ));
        assert!(needs_pixmap(false, BorderStyle::None, Layout::Tabbed, 2));
        assert!(!needs_reshape(
            first.con_deco_rect,
            first.con_deco_rect,
            first.con_deco_rect,
            first.con_deco_rect,
            false,
            false,
            false
        ));
    }
}
