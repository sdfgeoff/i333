use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{floating, identity, resolve};
use crate::domains::ContainerType;
use crate::flags::AdjacentEdges;
use crate::types::BorderStyle;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::TitleAlign;
use crate::types::Warping;
use crate::xcursor::Cursor;
use ::xcb::{shape, x, Cookie, Xid, XidNew};
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

// Adoption is confined to native reply/allocation producers in this module.
struct NativeReply<T>(*mut T);
impl<T> Drop for NativeReply<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.0.cast());
        }
    }
}
fn delete_window_event(window: u32, protocols: u32, delete: u32) -> x::ClientMessageEvent {
    x::ClientMessageEvent::new(
        x::Window::new(window),
        x::Atom::new(protocols),
        x::ClientMessageData::Data32([delete, x::CURRENT_TIME, 0, 0, 0]),
    )
}

fn connection() -> std::rc::Rc<::xcb::Connection> {
    i3_common::display::connection().expect("active XCB display")
}

fn set_event_mask(window: u32, mask: u32) {
    connection().send_request(&x::ChangeWindowAttributes {
        window: x::Window::new(window),
        value_list: &[x::Cw::EventMask(x::EventMask::from_bits_retain(mask))],
    });
}

// Preserve the native ICCCM parser's signed byte-length arithmetic and the
// existing consumer's payload clamp, including malformed oversized counts.
fn protocol_word_count(value_len: u32, reply_words: u32) -> usize {
    let native_bytes = value_len.wrapping_mul(4) as i32;
    ((native_bytes / 4) as u32 as usize).min(reply_words as usize)
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
    values: [x::Rectangle; 4],
    len: usize,
}
impl BorderRectangles {
    fn as_slice(&self) -> &[x::Rectangle] {
        &self.values[..self.len]
    }
}
fn border_rectangles(
    style: BorderStyle,
    leaf: bool,
    hidden: AdjacentEdges,
    border: Rect,
    rect: Rect,
) -> BorderRectangles {
    let mut result = BorderRectangles {
        values: [x::Rectangle {
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
    if !hidden.contains(AdjacentEdges::LEFT) {
        push(x::Rectangle {
            x: 0,
            y: 0,
            width: border.x as u16,
            height: rect.height as u16,
        });
    }
    if !hidden.contains(AdjacentEdges::RIGHT) {
        push(x::Rectangle {
            x: rect.width.wrapping_add(border.width.wrapping_add(border.x)) as i16,
            y: 0,
            width: border.width.wrapping_add(border.x).wrapping_neg() as u16,
            height: rect.height as u16,
        });
    }
    if !hidden.contains(AdjacentEdges::BOTTOM) {
        push(x::Rectangle {
            x: border.x as i16,
            y: rect
                .height
                .wrapping_add(border.height.wrapping_add(border.y)) as i16,
            width: rect.width.wrapping_add(border.width) as u16,
            height: border.height.wrapping_add(border.y).wrapping_neg() as u16,
        });
    }
    if style == BorderStyle::Pixel && !hidden.contains(AdjacentEdges::TOP) {
        push(x::Rectangle {
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
fn needs_pixmap(leaf: bool, border: BorderStyle, layout: Layout, kind: ContainerType) -> bool {
    kind != ContainerType::Root
        && kind != ContainerType::Output
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
    id: u32,
    mapped: bool,
    unmap_now: bool,
    child_mapped: bool,
    is_hidden: bool,
    is_maximized_vert: bool,
    is_maximized_horz: bool,
    con: *mut Con,
    need_reparent: bool,
    old_frame: u32,
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

pub static mut focused_id: u32 = 0_u32;
static mut last_focused: u32 = 0_u32;
static mut warp_to: *mut Rect = ::core::ptr::null::<Rect>() as *mut Rect;

static mut state_head: StateHead = StateHead {
    cqh_first: &raw const state_head as *mut StateHead as *mut ::core::ffi::c_void
        as *mut FrameState,
    cqh_last: &raw const state_head as *mut StateHead as *mut ::core::ffi::c_void
        as *mut FrameState,
};

static mut old_state_head: OldStateHead = OldStateHead {
    cqh_first: &raw const old_state_head as *mut OldStateHead as *mut ::core::ffi::c_void
        as *mut FrameState,
    cqh_last: &raw const old_state_head as *mut OldStateHead as *mut ::core::ffi::c_void
        as *mut FrameState,
};

static mut initial_mapping_head: InitialMappingHead = InitialMappingHead {
    tqh_first: ::core::ptr::null::<FrameState>() as *mut FrameState,
    tqh_last: unsafe { &raw mut initial_mapping_head.tqh_first },
};
unsafe fn state_for_frame(window: u32) -> *mut FrameState {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_head.cqh_first;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).id == window {
            return state;
        }
        state = (*state).state.cqe_next;
    }
    ::log::error!("ERROR: No state found for window 0x{:08x}\n", { (window) });
    __assert_fail(
        c"false".as_ptr(),
        c"../src/x.c".as_ptr(),
        99_u32,
        c"FrameState *state_for_frame(u32)".as_ptr(),
    )
}
unsafe fn change_ewmh_focus(new_focus: u32, old_focus: u32) {
    if new_focus == old_focus {
        return;
    }
    ewmh_update_active_window(new_focus);
    if new_focus != 0_i32 as u32 {
        ewmh_update_focused(new_focus, true);
    }
    if old_focus != 0_i32 as u32 {
        ewmh_update_focused(old_focus, false);
    }
}

pub unsafe fn x_con_init(con: *mut Con) {
    let connection = connection();
    let visual: u32 = get_visualid_by_depth((*con).depth);
    let mut win_colormap: u32 = 0;
    if (*con).depth as i32 != root_depth as i32 {
        let id = connection.generate_id::<x::Colormap>();
        win_colormap = id.resource_id();
        connection.send_request(&x::CreateColormap {
            alloc: x::ColormapAlloc::None,
            mid: id,
            window: x::Window::new(root),
            visual,
        });
        (*con).colormap = win_colormap;
    } else {
        win_colormap = colormap;
        (*con).colormap = 0_u32;
    }
    let event_mask = x::EventMask::BUTTON_PRESS
        | x::EventMask::BUTTON_RELEASE
        | x::EventMask::POINTER_MOTION
        | x::EventMask::EXPOSURE
        | x::EventMask::STRUCTURE_NOTIFY
        | x::EventMask::SUBSTRUCTURE_REDIRECT
        | x::EventMask::SUBSTRUCTURE_NOTIFY;
    let values = [
        x::Cw::BackPixel(crate::application::root_screen_info().black_pixel),
        x::Cw::BorderPixel(crate::application::root_screen_info().black_pixel),
        x::Cw::OverrideRedirect(true),
        x::Cw::EventMask(event_mask),
        x::Cw::Colormap(x::Colormap::new(win_colormap)),
    ];
    let dims: Rect = Rect {
        x: (-15i32) as u32,
        y: (-15i32) as u32,
        width: 10_u32,
        height: 10_u32,
    };
    let frame_id = create_window(
        &connection,
        dims,
        (*con).depth as u8,
        visual,
        x::WindowClass::InputOutput,
        Cursor::Pointer,
        false,
        &values,
    );
    draw_util_surface_init(
        &crate::application::rendering_context(),
        &raw mut (*con).frame,
        frame_id,
        get_visualtype_by_id(visual).as_deref(),
        dims.width as i32,
        dims.height as i32,
    );
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new((*con).frame.id),
        property: x::ATOM_WM_CLASS,
        r#type: x::ATOM_STRING,
        data: b"i3-frame\0i3-frame\0",
    });
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
    ::log::debug!(
        "../src/x.c:x_con_init:186 - Adding window 0x{:08x} to lists\n",
        ((*state).id) as u32
    );
    (*state).state.cqe_next = state_head.cqh_first;
    (*state).state.cqe_prev = &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState;
    if state_head.cqh_last == &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        state_head.cqh_last = state;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state;
    }
    state_head.cqh_first = state;
    (*state).old_state.cqe_next = old_state_head.cqh_first;
    (*state).old_state.cqe_prev =
        &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState;
    if old_state_head.cqh_last
        == &raw mut old_state_head as *mut ::core::ffi::c_void as *mut FrameState
    {
        old_state_head.cqh_last = state;
    } else {
        (*old_state_head.cqh_first).old_state.cqe_prev = state;
    }
    old_state_head.cqh_first = state;
    (*state).initial_mapping_order.tqe_next = ::core::ptr::null_mut::<FrameState>();
    (*state).initial_mapping_order.tqe_prev = initial_mapping_head.tqh_last;
    *initial_mapping_head.tqh_last = state;
    initial_mapping_head.tqh_last = &raw mut (*state).initial_mapping_order.tqe_next;
    ::log::debug!(
        "../src/x.c:x_con_init:190 - adding new state for window id 0x{:08x}\n",
        ((*state).id) as u32
    );
}

pub unsafe fn x_reinit(con: *mut Con) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id);
    if state.is_null() {
        ::log::error!("ERROR: window state not found\n");
        return;
    }
    ::log::debug!(
        "../src/x.c:x_reinit:207 - resetting state {} to initial\n",
        i3_common::logging::pointer(state)
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
    state = state_for_frame((*con).frame.id);
    if state.is_null() {
        ::log::error!("ERROR: window state for con not found\n");
        return;
    }
    (*state).need_reparent = true;
    (*state).old_frame = (*old).frame.id;
}

pub unsafe fn x_move_win(src: *mut Con, dest: *mut Con) {
    let mut state_src: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    let mut state_dest: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state_src = state_for_frame((*src).frame.id);
    if state_src.is_null() {
        ::log::error!("ERROR: window state for src not found\n");
        return;
    }
    state_dest = state_for_frame((*dest).frame.id);
    if state_dest.is_null() {
        ::log::error!("ERROR: window state for dest not found\n");
        return;
    }
    (*state_dest).con = (*state_src).con;
    (*state_src).con = ::core::ptr::null_mut::<Con>();
    if rect_equals(
        (*state_dest).window_rect,
        Rect {
            x: 0_u32,
            y: 0_u32,
            width: 0_u32,
            height: 0_u32,
        },
    ) {
        (*state_dest).window_rect = (*state_src).window_rect;
        ::log::debug!("../src/x.c:x_move_win:252 - COPYING RECT\n");
    }
}
unsafe fn _x_con_kill(con: *mut Con) {
    let connection = connection();
    if (*con).colormap as ::core::ffi::c_long != 0 {
        connection.send_request(&x::FreeColormap {
            cmap: x::Colormap::new((*con).colormap),
        });
    }
    draw_util_surface_free(conn, &raw mut (*con).frame);
    draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
    if !(*con).frame_buffer.release_pixmap() {
        connection.send_request(&x::FreePixmap {
            pixmap: x::Pixmap::new((*con).frame_buffer.id),
        });
    }
    (*con).frame_buffer.id = 0_u32;
    let state: *mut FrameState = state_for_frame((*con).frame.id);
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
        focused_id = 0_u32;
    }
    if (*con).frame.id == last_focused {
        last_focused = 0_u32;
    }
}

pub unsafe fn x_con_kill(con: *mut Con) {
    _x_con_kill(con);
    connection().send_request(&x::DestroyWindow {
        window: x::Window::new((*con).frame.id),
    });
}

pub unsafe fn x_con_reframe(con: *mut Con) {
    _x_con_kill(con);
    x_con_init(con);
}

pub unsafe fn window_supports_protocol(window: u32, atom: u32) -> bool {
    let connection = connection();
    let cookie = connection.send_request(&x::GetProperty {
        delete: false,
        window: x::Window::new(window),
        property: x::Atom::new(A_WM_PROTOCOLS),
        r#type: x::ATOM_ATOM,
        long_offset: 0,
        long_length: u32::MAX,
    });
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        return false;
    };
    if reply.r#type() != x::ATOM_ATOM || reply.format() != 32 {
        return false;
    }
    // SAFETY: the concrete owned reply contains its initialized 32-byte header
    // and length*4 trailing bytes. Bound the native parser's count before reading
    // words, rather than using its unchecked value_len-based generated slice.
    let words = unsafe {
        let raw = ::xcb::Reply::as_raw(&reply);
        let value_len = raw.add(16).cast::<u32>().read_unaligned();
        let count = protocol_word_count(value_len, reply.length());
        std::slice::from_raw_parts(raw.add(32), count * 4)
    };
    words
        .as_chunks::<4>()
        .0
        .iter()
        .any(|word| u32::from_ne_bytes(*word) == atom)
}

pub unsafe fn x_window_kill(window: u32, kill_window: KillWindow) {
    let connection = connection();
    if !window_supports_protocol(window, A_WM_DELETE_WINDOW) {
        if kill_window == KillWindow::Window {
            ::log::info!("Killing specific window 0x{:08x}\n", { (window) });
            connection.send_request(&x::DestroyWindow {
                window: x::Window::new(window),
            });
        } else {
            ::log::info!("Killing the X11 client which owns window 0x{:08x}\n", {
                (window)
            });
            connection.send_request(&x::KillClient { resource: window });
        }
        return;
    }
    let event = delete_window_event(window, A_WM_PROTOCOLS, A_WM_DELETE_WINDOW);
    ::log::info!("Sending WM_DELETE to the client\n");
    connection.send_request(&x::SendEvent {
        propagate: false,
        destination: x::SendEventDest::Window(x::Window::new(window)),
        event_mask: x::EventMask::NO_EVENT,
        event: &event,
    });
    let _ = connection.flush();
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
    if parent_ptr(con) != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"con->parent != NULL".as_ptr(),
            c"../src/x.c".as_ptr(),
            381_u32,
            c"void x_draw_decoration_after_title(Con *, struct deco_render_params *, surface_t *)"
                .as_ptr(),
        );
    };
    let dr: *mut Rect = &raw mut (*con).deco_rect;
    if !font_is_pango(&crate::config::config.font) {
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
        con_adjacent_borders(con) & config.hide_edge_borders.edge_mask(),
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
    let parent: *mut Con = parent_ptr(con);
    let leaf: bool = con_is_leaf(con);
    if !leaf && (*parent).layout != Layout::Stacked && (*parent).layout != Layout::Tabbed
        || ((*parent).type_0) == ContainerType::Output
        || ((*parent).type_0) == ContainerType::DockArea
        || ((*con).type_0) == ContainerType::Floating
    {
        return;
    }
    if (*con).rect.height == 0_u32 {
        return;
    }
    if leaf && (*con).frame_buffer.id as ::core::ffi::c_long == 0 {
        return;
    }
    let color = if (*con).urgent {
        &raw mut config.client.urgent
    } else if con == focused || con_inside_focused(con) {
        &raw mut config.client.focused
    } else if con == focus::first_ptr(parent) {
        if config.client.got_focused_tab_title && !leaf && con_descend_focused(con) == focused {
            &raw mut config.client.focused_tab_title
        } else {
            &raw mut config.client.focused_inactive
        }
    } else {
        &raw mut config.client.unfocused
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
            next = nodes::next_ptr(next);
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
            if nodes::next_ptr(con).is_null()
                && (nodes::previous_ptr(con)).is_null()
                && ((*parent_ptr(con)).type_0) != ContainerType::Floating
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
            {
                let diagnostic_5_owner = (*con).name.clone();
                let diagnostic_5_bytes = string_bytes(diagnostic_5_owner.as_deref(), None);
                match (std::str::from_utf8(diagnostic_5_bytes),) { (Ok(diagnostic_5_text),) => ::log::debug!(concat!("../src/x.c:x_draw_decoration:602 - using ","con->frame_buffer (for con->name={}) as dest_surface\n"), diagnostic_5_text), _ => emit_parts(Level::Debug, &[b"../src/x.c:x_draw_decoration:602 - using con->frame_buffer (for con->name=", diagnostic_5_bytes, b") as dest_surface\n"]), }
            };
            dest_surface = &raw mut (*con).frame_buffer;
        } else {
            ::log::debug!(
                "../src/x.c:x_draw_decoration:605 - sticking to parent->frame_buffer = {}\n",
                i3_common::logging::pointer(dest_surface)
            );
        }
        ::log::debug!(
            "../src/x.c:x_draw_decoration:607 - dest_surface {} is {} x {} (id=0x{:08x})\n",
            i3_common::logging::pointer(dest_surface),
            ((*dest_surface).width) as i32,
            ((*dest_surface).height) as i32,
            ((*dest_surface).id) as u32
        );
        if !((*dest_surface).id as ::core::ffi::c_long == 0) {
            if con == nodes::first_ptr(parent_ptr(con)) {
                free((*parent_ptr(con)).deco_render_params as *mut ::core::ffi::c_void);
                (*parent_ptr(con)).deco_render_params =
                    ::core::ptr::null_mut::<deco_render_params>();
            }
            if !((*p).border_style != BorderStyle::Normal
                || (*con).deco_rect.width == 0_u32
                || (*con).deco_rect.height == 0_u32)
            {
                {
                    let diagnostic_5 = ((*con).deco_rect.x) as i32;
                    let diagnostic_6 = ((*con).deco_rect.y) as i32;
                    let diagnostic_7 = ((*con).deco_rect.width) as i32;
                    let diagnostic_8 = ((*con).deco_rect.height) as i32;
                    let diagnostic_9_owner = (*con).name.clone();
                    let diagnostic_9_bytes = string_bytes(diagnostic_9_owner.as_deref(), None);
                    match (std::str::from_utf8(diagnostic_9_bytes),) {
                        (Ok(diagnostic_9_text),) => ::log::debug!(
                            concat!(
                                "../src/x.c:x_draw_decoration:630 - con->deco_rect = ",
                                "(x={}, y={}, w={}, h={}) for con->name={}\n"
                            ),
                            diagnostic_5,
                            diagnostic_6,
                            diagnostic_7,
                            diagnostic_8,
                            diagnostic_9_text
                        ),
                        _ => emit_parts(
                            Level::Debug,
                            &[
                                b"../src/x.c:x_draw_decoration:630 - con->deco_rect = (x=",
                                format!("{}", diagnostic_5).as_bytes(),
                                b", y=",
                                format!("{}", diagnostic_6).as_bytes(),
                                b", w=",
                                format!("{}", diagnostic_7).as_bytes(),
                                b", h=",
                                format!("{}", diagnostic_8).as_bytes(),
                                b") for con->name=",
                                diagnostic_9_bytes,
                                b"\n",
                            ],
                        ),
                    }
                };
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
                    .wrapping_div(2_u32) as i32;
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
                        mark_width = predict_text_width(&crate::config::config.font, mark_0);
                        let mark_offset_x: i32 = if config.title_align == TitleAlign::Right {
                            title_padding
                        } else {
                            deco_width - mark_width - title_padding
                        };
                        draw_util_text(
                            &crate::config::config.font,
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
                let owned_title = if (*con).title_format.is_some() {
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
                } else if !win.is_null() && (*con).title_format.is_none() {
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
                        !win.is_null() && (*win).icon.is_some(),
                        predict_text_width(&crate::config::config.font, title),
                        mark_width,
                        title_padding,
                        logical_px((*con).window_icon_padding.max(1)),
                    );
                    icon_size = placement.icon_size;
                    has_icon = placement.has_icon;
                    icon_offset_x = placement.icon_offset;
                    title_offset_x = placement.title_offset;
                    draw_util_text(
                        &crate::config::config.font,
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
                            (*win).icon.as_ref().unwrap().as_raw(),
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
    let leaf: bool = nodes::first_ptr(con).is_null()
        && floating::is_empty(identity(con).expect("live floating parent"));
    let state: *const FrameState = state_for_frame((*con).frame.id);
    if !leaf {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = nodes::first_ptr(con);
        while !current.is_null() {
            x_deco_recurse(current);
            current = nodes::next_ptr(current);
        }
        for floating_id in floating::children(identity(con).expect("live floating parent")) {
            let Some(current) = resolve(floating_id) else {
                continue;
            };
            x_deco_recurse(current);
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
    if ((*con).type_0) != ContainerType::Root
        && ((*con).type_0) != ContainerType::Output
        && (!leaf || (*con).mapped)
    {
        x_draw_decoration(con);
    }
}
unsafe fn set_hidden_state(con: *mut Con) {
    if (*con).window.is_null() {
        return;
    }
    let state: *mut FrameState = state_for_frame((*con).frame.id);
    let should_be_hidden: bool = con_is_hidden(con);
    if should_be_hidden == (*state).is_hidden {
        return;
    }
    if should_be_hidden {
        ::log::debug!(
            "../src/x.c:set_hidden_state:820 - setting _NET_WM_STATE_HIDDEN for con = {}\n",
            i3_common::logging::pointer(con)
        );
        xcb_add_property_atom(
            conn,
            (*(*con).window).id,
            A__NET_WM_STATE,
            A__NET_WM_STATE_HIDDEN,
        );
    } else {
        ::log::debug!(
            "../src/x.c:set_hidden_state:823 - removing _NET_WM_STATE_HIDDEN for con = {}\n",
            i3_common::logging::pointer(con)
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
    let state: *mut FrameState = state_for_frame((*con).frame.id);
    let con_maximized_horz: bool = con_is_maximized(con, Orientation::Horizontal);
    if con_maximized_horz != (*state).is_maximized_horz {
        {
            let diagnostic_5 = i3_common::logging::pointer(con);
            let diagnostic_6_owner = (*con).name.clone();
            let diagnostic_6_bytes = string_bytes(diagnostic_6_owner.as_deref(), None);
            let diagnostic_7 = (con_maximized_horz as i32);
            match (std::str::from_utf8(diagnostic_6_bytes),) { (Ok(diagnostic_6_text),) => ::log::debug!(concat!("../src/x.c:set_maximized_state:843 - setting ","_NET_WM_STATE_MAXIMIZED_HORZ for con {}({}) to {}\n"), diagnostic_5, diagnostic_6_text, diagnostic_7), _ => emit_parts(Level::Debug, &[b"../src/x.c:set_maximized_state:843 - setting _NET_WM_STATE_MAXIMIZED_HORZ for con ", format!("{}", diagnostic_5).as_bytes(), b"(", diagnostic_6_bytes, b") to ", format!("{}", diagnostic_7).as_bytes(), b"\n"]), }
        };
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
        {
            let diagnostic_5 = i3_common::logging::pointer(con);
            let diagnostic_6_owner = (*con).name.clone();
            let diagnostic_6_bytes = string_bytes(diagnostic_6_owner.as_deref(), None);
            let diagnostic_7 = (con_maximized_vert as i32);
            match (std::str::from_utf8(diagnostic_6_bytes),) { (Ok(diagnostic_6_text),) => ::log::debug!(concat!("../src/x.c:set_maximized_state:856 - setting ","_NET_WM_STATE_MAXIMIZED_VERT for con {}({}) to {}\n"), diagnostic_5, diagnostic_6_text, diagnostic_7), _ => emit_parts(Level::Debug, &[b"../src/x.c:set_maximized_state:856 - setting _NET_WM_STATE_MAXIMIZED_VERT for con ", format!("{}", diagnostic_5).as_bytes(), b"(", diagnostic_6_bytes, b") to ", format!("{}", diagnostic_7).as_bytes(), b"\n"]), }
        };
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
unsafe fn x_shape_frame(con: *mut Con, shape_kind: shape::Sk) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            873_u32,
            c"void x_shape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    let connection = i3_common::display::connection().expect("active XCB display");
    connection.send_request(&shape::Combine {
        operation: shape::So::Set,
        destination_kind: shape_kind,
        source_kind: shape_kind,
        destination_window: x::Window::new((*con).frame.id),
        x_offset: (*con)
            .window_rect
            .x
            .wrapping_add((*con).border_width as u32) as i16,
        y_offset: (*con)
            .window_rect
            .y
            .wrapping_add((*con).border_width as u32) as i16,
        source_window: x::Window::new((*(*con).window).id),
    });
    let rectangles = x_get_border_rectangles(con);
    if !rectangles.as_slice().is_empty() {
        connection.send_request(&shape::Rectangles {
            operation: shape::So::Union,
            destination_kind: shape_kind,
            ordering: x::ClipOrdering::Unsorted,
            destination_window: x::Window::new((*con).frame.id),
            x_offset: 0,
            y_offset: 0,
            rectangles: rectangles.as_slice(),
        });
    }
}
unsafe fn x_unshape_frame(con: *mut Con, shape_kind: shape::Sk) {
    if !(*con).window.is_null() {
    } else {
        __assert_fail(
            c"con->window".as_ptr(),
            c"../src/x.c".as_ptr(),
            893_u32,
            c"void x_unshape_frame(Con *, xcb_shape_sk_t)".as_ptr(),
        );
    };
    let connection = i3_common::display::connection().expect("active XCB display");
    connection.send_request(&shape::Mask {
        operation: shape::So::Set,
        destination_kind: shape_kind,
        destination_window: x::Window::new((*con).frame.id),
        x_offset: 0,
        y_offset: 0,
        source_bitmap: x::Pixmap::new(0),
    });
}
unsafe fn set_shape_state(con: *mut Con, need_reshape: bool) {
    if !shape_supported || (*con).window.is_null() {
        return;
    }
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id);
    if state.is_null() {
        ::log::error!(
            "ERROR: window state for con {} not found\n",
            i3_common::logging::pointer(con)
        );
        return;
    }
    if need_reshape && con_is_floating(con) {
        if (*(*con).window).shaped {
            x_shape_frame(con, shape::Sk::Bounding);
        }
        if (*(*con).window).input_shaped {
            x_shape_frame(con, shape::Sk::Input);
        }
    }
    if (*state).was_floating && !con_is_floating(con) {
        if (*(*con).window).shaped {
            x_unshape_frame(con, shape::Sk::Bounding);
        }
        if (*(*con).window).input_shaped {
            x_unshape_frame(con, shape::Sk::Input);
        }
    }
}

pub unsafe fn x_push_node(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut rect: Rect = (*con).rect as Rect;
    let state: *mut FrameState = state_for_frame((*con).frame.id);
    if let Some(name) = (*state).name.take() {
        {
            let diagnostic_2_owner = name.clone();
            let diagnostic_2_bytes = diagnostic_2_owner.as_bytes();
            let diagnostic_3 = i3_common::logging::pointer(con);
            match (std::str::from_utf8(diagnostic_2_bytes),) {
                (Ok(diagnostic_2_text),) => ::log::debug!(
                    "pushing name {} for con {}\n",
                    diagnostic_2_text,
                    diagnostic_3
                ),
                _ => emit_parts(
                    Level::Debug,
                    &[
                        b"pushing name ",
                        diagnostic_2_bytes,
                        b" for con ",
                        format!("{}", diagnostic_3).as_bytes(),
                        b"\n",
                    ],
                ),
            }
        };
        connection().send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new((*con).frame.id),
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: name.as_bytes(),
        });
    }
    if (*con).window.is_null()
        && ((*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed)
    {
        let mut max_y: u32 = 0_u32;
        let mut max_height: u32 = 0_u32;
        current = nodes::first_ptr(con);
        while !current.is_null() {
            let dr: *mut Rect = &raw mut (*current).deco_rect;
            if (*dr).y >= max_y && (*dr).height >= max_height {
                max_y = (*dr).y;
                max_height = (*dr).height;
            }
            current = nodes::next_ptr(current);
        }
        rect.height = max_y.wrapping_add(max_height);
        if rect.height == 0_u32 {
            (*con).mapped = false;
        }
    } else if (*con).window.is_null() {
        (*con).mapped = false;
    }
    let mut need_reshape: bool = false;
    if (*state).need_reparent && !(*con).window.is_null() {
        ::log::debug!("../src/x.c:x_push_node:978 - Reparenting child window\n");
        let mut values: u32 = 0_u32;
        set_event_mask((*state).old_frame, values);
        set_event_mask((*(*con).window).id, values);
        connection().send_request(&x::ReparentWindow {
            window: x::Window::new((*(*con).window).id),
            parent: x::Window::new((*con).frame.id),
            x: 0_i16,
            y: 0_i16,
        });
        values = (x::EventMask::BUTTON_PRESS.bits() as i32
            | x::EventMask::BUTTON_RELEASE.bits() as i32
            | x::EventMask::POINTER_MOTION.bits() as i32
            | x::EventMask::EXPOSURE.bits() as i32
            | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
            | x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
            | x::EventMask::SUBSTRUCTURE_NOTIFY.bits() as i32
            | x::EventMask::ENTER_WINDOW.bits() as i32) as u32;
        set_event_mask((*state).old_frame, values);
        values = (x::EventMask::PROPERTY_CHANGE.bits() as i32
            | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
            | x::EventMask::FOCUS_CHANGE.bits() as i32) as u32;
        set_event_mask((*(*con).window).id, values);
        (*state).old_frame = 0_u32;
        (*state).need_reparent = false;
        (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
        ::log::debug!(
            concat!(
                "../src/x.c:x_push_node:999 - ignore_unmap for ",
                "reparenting of con {} (win 0x{:08x}) is now {}\n"
            ),
            i3_common::logging::pointer(con),
            { ((*(*con).window).id) },
            { ((*con).ignore_unmap as i32) }
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
        (*con).type_0,
    );
    ::log::debug!(
        concat!(
            "../src/x.c:x_push_node:1019 - Con {} (layout {}), ",
            "is_pixmap_needed = {}, rect.height = {}\n"
        ),
        i3_common::logging::pointer(con),
        (*con).layout as i32,
        if is_pixmap_needed { "yes" } else { "no" },
        (*con).rect.height as i32
    );
    let mut fake_notify: bool = false;
    if is_pixmap_needed && (*con).frame_buffer.id as ::core::ffi::c_long == 0
        || !rect_equals((*state).rect, rect) && rect.height > 0_u32
    {
        let has_rect_changed: bool = (*state).rect.x != rect.x
            || (*state).rect.y != rect.y
            || (*state).rect.width != rect.width
            || (*state).rect.height != rect.height;
        if !is_pixmap_needed && (*con).frame_buffer.id as ::core::ffi::c_long != 0 {
            draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
            if !(*con).frame_buffer.release_pixmap() {
                connection().send_request(&x::FreePixmap {
                    pixmap: x::Pixmap::new((*con).frame_buffer.id),
                });
            }
            (*con).frame_buffer.id = 0_u32;
        }
        if is_pixmap_needed
            && (has_rect_changed || (*con).frame_buffer.id as ::core::ffi::c_long == 0)
        {
            if (*con).frame_buffer.id as ::core::ffi::c_long == 0 {
                (*con).frame_buffer.id = connection().generate_id::<x::Pixmap>().resource_id();
            } else {
                draw_util_surface_free(conn, &raw mut (*con).frame_buffer);
                if !(*con).frame_buffer.release_pixmap() {
                    connection().send_request(&x::FreePixmap {
                        pixmap: x::Pixmap::new((*con).frame_buffer.id),
                    });
                }
            }
            let mut win_depth: u16 = root_depth as u16;
            if !(*con).window.is_null() {
                win_depth = (*(*con).window).depth;
            }
            let (width, height) = pixmap_dimensions(rect);
            ::log::debug!(
                concat!(
                    "../src/x.c:x_push_node:1068 - creating {} x {} pixmap ",
                    "for con {} (con->frame_buffer.id = (pixmap_t)0x{:08x}) ",
                    "(con->frame.id (drawable_t)0x{:08x})\n"
                ),
                { (width) },
                { (height) },
                i3_common::logging::pointer(con),
                { ((*con).frame_buffer.id) },
                { ((*con).frame.id) }
            );
            connection().send_request(&x::CreatePixmap {
                depth: win_depth as u8,
                pid: x::Pixmap::new((*con).frame_buffer.id),
                drawable: x::Drawable::Window(x::Window::new((*con).frame.id)),
                width: width as u16,
                height: height as u16,
            });
            draw_util_surface_init(
                &crate::application::rendering_context(),
                &raw mut (*con).frame_buffer,
                (*con).frame_buffer.id,
                get_visualtype_by_id(get_visualid_by_depth(win_depth)).as_deref(),
                width,
                height,
            );
            // SAFETY: this request created the unique backing pixmap. The
            // surface releases it after Cairo teardown at the existing explicit
            // free sites, or on final container drop.
            (*con).frame_buffer.attach_pixmap(OwnedPixmap::from_created(
                connection(),
                x::Pixmap::new((*con).frame_buffer.id),
            ));
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
            connection().send_request(&x::ChangeGc {
                gc: x::Gcontext::new((*con).frame_buffer.gc),
                value_list: &[x::Gc::GraphicsExposures(false)],
            });
            draw_util_surface_set_size(&raw mut (*con).frame, width, height);
            (*con).pixmap_recreated = true;
            if parent_ptr(con).is_null()
                || (*parent_ptr(con)).layout != Layout::Stacked
                || focus::first_ptr(parent_ptr(con)) == con
            {
                x_deco_recurse(con);
            }
        }
        ::log::debug!(
            "../src/x.c:x_push_node:1097 - setting rect ({}, {}, {}, {})\n",
            (rect.x) as i32,
            (rect.y) as i32,
            (rect.width) as i32,
            (rect.height) as i32
        );
        {
            let _ = connection().flush();
        };
        xcb_set_window_rect(conn, (*con).frame.id, rect);
        if (*con).frame_buffer.id as ::core::ffi::c_long != 0 {
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
        {
            let _ = connection().flush();
        };
        (*state).rect = rect;
        fake_notify = true;
    }
    if !(*con).window.is_null() && !rect_equals((*state).window_rect, (*con).window_rect as Rect) {
        ::log::debug!(
            "../src/x.c:x_push_node:1117 - setting window rect ({}, {}, {}, {})\n",
            ((*con).window_rect.x) as i32,
            ((*con).window_rect.y) as i32,
            ((*con).window_rect.width) as i32,
            ((*con).window_rect.height) as i32
        );
        xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
        (*state).window_rect = (*con).window_rect;
        fake_notify = true;
    }
    set_shape_state(con, need_reshape);
    if !con.is_null() && !(*con).window.is_null() {
        let bsr: Rect = con_border_style_rect(con);
        let extents = frame_extents(bsr);
        connection().send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new((*(*con).window).id),
            property: x::Atom::new(A__NET_FRAME_EXTENTS),
            r#type: x::ATOM_CARDINAL,
            data: &extents,
        });
    }
    if ((*state).mapped != (*con).mapped || !(*con).window.is_null() && !(*state).child_mapped)
        && (*con).mapped
    {
        if !(*con).window.is_null() {
            // The original format-32 write used two native longs; its first
            // eight bytes encode Normal and None on the supported LP64 target.
            let data = [1u32, 0];
            connection().send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x::Window::new((*(*con).window).id),
                property: x::Atom::new(A_WM_STATE),
                r#type: x::Atom::new(A_WM_STATE),
                data: &data,
            });
        }
        let mut values_0: u32 = 0;
        if !(*state).child_mapped && !(*con).window.is_null() {
            let cookie = connection().send_request(&x::MapWindow {
                window: x::Window::new((*(*con).window).id),
            });
            values_0 = (x::EventMask::PROPERTY_CHANGE.bits() as i32
                | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
                | x::EventMask::FOCUS_CHANGE.bits() as i32) as u32;
            set_event_mask((*(*con).window).id, values_0);
            ::log::debug!(
                "../src/x.c:x_push_node:1166 - mapping child window (serial {})\n",
                (cookie.sequence() as u32) as i32
            );
            (*state).child_mapped = true;
        }
        let cookie = connection().send_request(&x::MapWindow {
            window: x::Window::new((*con).frame.id),
        });
        values_0 = (x::EventMask::BUTTON_PRESS.bits() as i32
            | x::EventMask::BUTTON_RELEASE.bits() as i32
            | x::EventMask::POINTER_MOTION.bits() as i32
            | x::EventMask::EXPOSURE.bits() as i32
            | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
            | x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
            | x::EventMask::SUBSTRUCTURE_NOTIFY.bits() as i32
            | x::EventMask::ENTER_WINDOW.bits() as i32) as u32;
        set_event_mask((*con).frame.id, values_0);
        if (*con).frame_buffer.id as ::core::ffi::c_long != 0 {
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
        {
            let _ = connection().flush();
        };
        ::log::debug!(
            "../src/x.c:x_push_node:1181 - mapping container {:08x} (serial {})\n",
            { ((*con).frame.id) },
            (cookie.sequence() as u32) as i32
        );
        (*state).mapped = (*con).mapped;
    }
    (*state).unmap_now = (*state).mapped != (*con).mapped && !(*con).mapped;
    (*state).was_floating = con_is_floating(con);
    if fake_notify {
        ::log::debug!("../src/x.c:x_push_node:1189 - Sending fake configure notify\n");
        fake_absolute_configure_notify(con);
    }
    set_hidden_state(con);
    set_maximized_state(con);
    current = focus::first_ptr(con);
    while !current.is_null() {
        x_push_node(current);
        current = focus::next_ptr(current);
    }
}
unsafe fn x_push_node_unmaps(con: *mut Con) {
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    let state: *mut FrameState = state_for_frame((*con).frame.id);
    if (*state).unmap_now {
        if !(*con).window.is_null() {
            let data = [0u32, 0];
            connection().send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x::Window::new((*(*con).window).id),
                property: x::Atom::new(A_WM_STATE),
                r#type: x::Atom::new(A_WM_STATE),
                data: &data,
            });
        }
        let cookie = connection().send_request(&x::UnmapWindow {
            window: x::Window::new((*con).frame.id),
        });
        {
            let diagnostic_5 = i3_common::logging::pointer(con);
            let diagnostic_6_owner = (*con).name.clone();
            let diagnostic_6_bytes = string_bytes(diagnostic_6_owner.as_deref(), None);
            let diagnostic_7 = (cookie.sequence() as u32) as i32;
            match (std::str::from_utf8(diagnostic_6_bytes),) {
                (Ok(diagnostic_6_text),) => ::log::debug!(
                    concat!(
                        "../src/x.c:x_push_node_unmaps:1229 - unmapping ",
                        "container {} / {} (serial {})\n"
                    ),
                    diagnostic_5,
                    diagnostic_6_text,
                    diagnostic_7
                ),
                _ => emit_parts(
                    Level::Debug,
                    &[
                        b"../src/x.c:x_push_node_unmaps:1229 - unmapping container ",
                        format!("{}", diagnostic_5).as_bytes(),
                        b" / ",
                        diagnostic_6_bytes,
                        b" (serial ",
                        format!("{}", diagnostic_7).as_bytes(),
                        b")\n",
                    ],
                ),
            }
        };
        if !(*con).window.is_null() {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_add(1);
            ::log::debug!(
                concat!(
                    "../src/x.c:x_push_node_unmaps:1235 - ignore_unmap for ",
                    "con {} (frame 0x{:08x}) now {}\n"
                ),
                i3_common::logging::pointer(con),
                { ((*con).frame.id) },
                { ((*con).ignore_unmap as i32) }
            );
        }
        (*state).mapped = (*con).mapped;
    }
    current = nodes::first_ptr(con);
    while !current.is_null() {
        x_push_node_unmaps(current);
        current = nodes::next_ptr(current);
    }
    for floating_id in floating::children(identity(con).expect("live floating parent")) {
        let Some(current) = resolve(floating_id) else {
            continue;
        };
        x_push_node_unmaps(current);
    }
}
unsafe fn is_con_attached(con: *mut Con) -> bool {
    if parent_ptr(con).is_null() {
        return false;
    }
    let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
    current = nodes::first_ptr(parent_ptr(con));
    while !current.is_null() {
        if current == con {
            return true;
        }
        current = nodes::next_ptr(current);
    }
    false
}

pub unsafe fn x_push_changes(con: *mut Con) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    let mut pointer_cookie = None;
    if !warp_to.is_null() {
        pointer_cookie = Some(connection().send_request(&x::QueryPointer {
            window: x::Window::new(root),
        }));
    }
    ::log::debug!("../src/x.c:x_push_changes:1290 - -- PUSHING WINDOW STACK --\n");
    let mut values: u32 = x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32 as u32;
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            set_event_mask((*state).id, values);
        }
        state = (*state).state.cqe_prev;
    }
    let mut order_changed: bool = false;
    let mut stacking_changed: bool = false;
    let mut cnt: i32 = 0;
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if con_has_managed_window((*state).con) {
            cnt += 1;
        }
        state = (*state).state.cqe_prev;
    }
    let mut client_list_windows = Vec::with_capacity(cnt as usize);
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if con_has_managed_window((*state).con) {
            client_list_windows.push((*(*(*state).con).window).id);
        }
        let prev: *mut FrameState = (*state).state.cqe_prev;
        let old_prev: *mut FrameState = (*state).old_state.cqe_prev;
        if prev != old_prev {
            order_changed = true;
        }
        if ((*state).initial || order_changed)
            && prev != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState
        {
            stacking_changed = true;
            connection().send_request(&x::ConfigureWindow {
                window: x::Window::new((*prev).id),
                value_list: &[
                    x::ConfigWindow::Sibling(x::Window::new((*state).id)),
                    x::ConfigWindow::StackMode(x::StackMode::Above),
                ],
            });
        }
        (*state).initial = false;
        state = (*state).state.cqe_prev;
    }
    if stacking_changed {
        ::log::debug!(
            "../src/x.c:x_push_changes:1350 - Client list changed ({} clients)\n",
            { (cnt) }
        );
        ewmh_update_client_list_stacking(
            client_list_windows.as_mut_ptr(),
            client_list_windows.len() as i32,
        );
        client_list_windows.clear();
        state = initial_mapping_head.tqh_first;
        while !state.is_null() {
            if con_has_managed_window((*state).con) {
                client_list_windows.push((*(*(*state).con).window).id);
            }
            state = (*state).initial_mapping_order.tqe_next;
        }
        ewmh_update_client_list(
            client_list_windows.as_mut_ptr(),
            client_list_windows.len() as i32,
        );
    }
    ::log::debug!("../src/x.c:x_push_changes:1365 - PUSHING CHANGES\n");
    x_push_node(con);
    if !warp_to.is_null() {
        let pointer_reply = pointer_cookie
            .take()
            .and_then(|cookie| connection().wait_for_reply(cookie).ok());
        if let Some(pointer_reply) = pointer_reply {
            let (mid_x, mid_y) = warp_midpoint(*warp_to);
            let current: *mut Output =
                get_output_containing(pointer_reply.root_x() as u32, pointer_reply.root_y() as u32);
            let target: *mut Output = get_output_containing(mid_x as u32, mid_y as u32);
            if current != target {
                let temporary_root_mask: u32 =
                    x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32 as u32;
                set_event_mask(root, temporary_root_mask);
                connection().send_request(&x::WarpPointer {
                    src_window: x::Window::new(0),
                    dst_window: x::Window::new(root),
                    src_x: 0,
                    src_y: 0,
                    src_width: 0,
                    src_height: 0,
                    dst_x: mid_x as i16,
                    dst_y: mid_y as i16,
                });
                let root_mask: u32 = (x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
                    | x::EventMask::BUTTON_PRESS.bits() as i32
                    | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
                    | x::EventMask::POINTER_MOTION.bits() as i32
                    | x::EventMask::PROPERTY_CHANGE.bits() as i32
                    | x::EventMask::FOCUS_CHANGE.bits() as i32
                    | x::EventMask::ENTER_WINDOW.bits() as i32)
                    as u32;
                set_event_mask(root, root_mask);
            }
        } else {
            ::log::error!("ERROR: Could not query pointer position, not warping pointer\n");
        }
        warp_to = ::core::ptr::null_mut::<Rect>();
    }
    values = (x::EventMask::BUTTON_PRESS.bits() as i32
        | x::EventMask::BUTTON_RELEASE.bits() as i32
        | x::EventMask::POINTER_MOTION.bits() as i32
        | x::EventMask::EXPOSURE.bits() as i32
        | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
        | x::EventMask::SUBSTRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::ENTER_WINDOW.bits() as i32) as u32;
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            set_event_mask((*state).id, values);
        }
        state = (*state).state.cqe_prev;
    }
    x_deco_recurse(con);
    let mut to_focus: u32 = (*focused).frame.id;
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
            {
                let diagnostic_5 = i3_common::logging::pointer(focused);
                let diagnostic_6_owner = (*focused).name.clone();
                let diagnostic_6_bytes = string_bytes(diagnostic_6_owner.as_deref(), None);
                match (std::str::from_utf8(diagnostic_6_bytes),) {
                    (Ok(diagnostic_6_text),) => ::log::debug!(
                        concat!(
                            "../src/x.c:x_push_changes:1406 - Not updating focus ",
                            "(to {} / {}), focused window is not mapped.\n"
                        ),
                        diagnostic_5,
                        diagnostic_6_text
                    ),
                    _ => emit_parts(
                        Level::Debug,
                        &[
                            b"../src/x.c:x_push_changes:1406 - Not updating focus (to ",
                            format!("{}", diagnostic_5).as_bytes(),
                            b" / ",
                            diagnostic_6_bytes,
                            b"), focused window is not mapped.\n",
                        ],
                    ),
                }
            };
            focused_id = 0_u32;
        } else {
            if action == FocusAction::TakeFocus {
                {
                    let diagnostic_5 = (to_focus);
                    let diagnostic_6 = i3_common::logging::pointer(focused);
                    let diagnostic_7_owner = (*focused).name.clone();
                    let diagnostic_7_bytes = string_bytes(diagnostic_7_owner.as_deref(), None);
                    match (std::str::from_utf8(diagnostic_7_bytes),) { (Ok(diagnostic_7_text),) => ::log::debug!(concat!("../src/x.c:x_push_changes:1414 - Updating focus by ","sending WM_TAKE_FOCUS to window 0x{:08x} (focused: {} ","/ {})\n"), diagnostic_5, diagnostic_6, diagnostic_7_text), _ => emit_parts(Level::Debug, &[b"../src/x.c:x_push_changes:1414 - Updating focus by sending WM_TAKE_FOCUS to window 0x", format!("{:08x}", diagnostic_5).as_bytes(), b" (focused: ", format!("{}", diagnostic_6).as_bytes(), b" / ", diagnostic_7_bytes, b")\n"]), }
                };
                send_take_focus(to_focus, last_timestamp);
                change_ewmh_focus(
                    if con_has_managed_window(focused) {
                        (*(*focused).window).id
                    } else {
                        0_i32 as u32
                    },
                    last_focused,
                );
                if to_focus != last_focused && is_con_attached(focused) {
                    ipc_send_window_event(c"focus".as_ptr(), focused);
                }
            } else {
                {
                    let diagnostic_5 = i3_common::logging::pointer(focused);
                    let diagnostic_6_owner = (*focused).name.clone();
                    let diagnostic_6_bytes = string_bytes(diagnostic_6_owner.as_deref(), None);
                    let diagnostic_7 = (to_focus);
                    match (std::str::from_utf8(diagnostic_6_bytes),) {
                        (Ok(diagnostic_6_text),) => ::log::debug!(
                            concat!(
                                "../src/x.c:x_push_changes:1423 - Updating focus ",
                                "(focused: {} / {}) to X11 window 0x{:08x}\n"
                            ),
                            diagnostic_5,
                            diagnostic_6_text,
                            diagnostic_7
                        ),
                        _ => emit_parts(
                            Level::Debug,
                            &[
                                b"../src/x.c:x_push_changes:1423 - Updating focus (focused: ",
                                format!("{}", diagnostic_5).as_bytes(),
                                b" / ",
                                diagnostic_6_bytes,
                                b") to X11 window 0x",
                                format!("{:08x}", diagnostic_7).as_bytes(),
                                b"\n",
                            ],
                        ),
                    }
                };
                if !(*focused).window.is_null() {
                    values = ((x::EventMask::PROPERTY_CHANGE.bits() as i32
                        | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
                        | x::EventMask::FOCUS_CHANGE.bits() as i32)
                        & !(x::EventMask::FOCUS_CHANGE.bits() as i32))
                        as u32;
                    set_event_mask((*(*focused).window).id, values);
                }
                connection().send_request(&x::SetInputFocus {
                    revert_to: x::InputFocus::PointerRoot,
                    focus: x::Window::new(to_focus),
                    time: last_timestamp,
                });
                if !(*focused).window.is_null() {
                    values = (x::EventMask::PROPERTY_CHANGE.bits() as i32
                        | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
                        | x::EventMask::FOCUS_CHANGE.bits() as i32)
                        as u32;
                    set_event_mask((*(*focused).window).id, values);
                }
                change_ewmh_focus(
                    if con_has_managed_window(focused) {
                        (*(*focused).window).id
                    } else {
                        0_i32 as u32
                    },
                    last_focused,
                );
                if to_focus as ::core::ffi::c_long != 0
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
    if focused_id as ::core::ffi::c_long == 0 {
        ::log::debug!(
            concat!(
                "../src/x.c:x_push_changes:1451 - Still no window ",
                "focused, better set focus to the EWMH support window ",
                "({})\n"
            ),
            (ewmh_window) as i32
        );
        connection().send_request(&x::SetInputFocus {
            revert_to: x::InputFocus::PointerRoot,
            focus: x::Window::new(ewmh_window),
            time: last_timestamp,
        });
        change_ewmh_focus(0_i32 as u32, last_focused);
        focused_id = ewmh_window;
        last_focused = 0_u32;
    }
    {
        let _ = connection().flush();
    };
    ::log::debug!("../src/x.c:x_push_changes:1460 - ENDING CHANGES\n");
    values = ((x::EventMask::BUTTON_PRESS.bits() as i32
        | x::EventMask::BUTTON_RELEASE.bits() as i32
        | x::EventMask::POINTER_MOTION.bits() as i32
        | x::EventMask::EXPOSURE.bits() as i32
        | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
        | x::EventMask::SUBSTRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::ENTER_WINDOW.bits() as i32)
        & !(x::EventMask::ENTER_WINDOW.bits() as i32)) as u32;
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).unmap_now {
            set_event_mask((*state).id, values);
        }
        state = (*state).state.cqe_prev;
    }
    x_push_node_unmaps(con);
    state = state_head.cqh_first;
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
            old_state_head.cqh_first = state;
        } else {
            (*old_state_head.cqh_last).old_state.cqe_next = state;
        }
        old_state_head.cqh_last = state;
        state = (*state).state.cqe_next;
    }
    {
        let _ = connection().flush();
    };
}

pub unsafe fn x_raise_con(con: *const Con) {
    let state: *mut FrameState = state_for_frame((*con).frame.id);
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
        state_head.cqh_last = state;
    } else {
        (*state_head.cqh_first).state.cqe_prev = state;
    }
    state_head.cqh_first = state;
}

pub unsafe fn x_set_name(con: *mut Con, name: *const ::core::ffi::c_char) {
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_for_frame((*con).frame.id);
    if state.is_null() {
        ::log::error!("ERROR: window state not found\n");
        return;
    }
    // The incoming bytes may alias the previous name: clone before replacement.
    let name = CStr::from_ptr(name).to_owned();
    (*state).name = Some(name);
}

pub unsafe fn update_shmlog_atom() {
    let connection = connection();
    let path = CStr::from_ptr(shmlogname).to_bytes();
    if path.is_empty() {
        connection.send_request(&x::DeleteProperty {
            window: x::Window::new(root),
            property: x::Atom::new(A_I3_SHMLOG_PATH),
        });
    } else {
        connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new(root),
            property: x::Atom::new(A_I3_SHMLOG_PATH),
            r#type: x::Atom::new(A_UTF8_STRING),
            data: path,
        });
    }
}

pub unsafe fn x_set_i3_atoms() {
    let connection = connection();
    let socket_path = if current_socketpath.is_null() {
        &[][..]
    } else {
        CStr::from_ptr(current_socketpath).to_bytes()
    };
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(root),
        property: x::Atom::new(A_I3_SOCKET_PATH),
        r#type: x::Atom::new(A_UTF8_STRING),
        data: socket_path,
    });
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: x::Window::new(root),
        property: x::Atom::new(A_I3_PID),
        r#type: x::ATOM_CARDINAL,
        data: &[getpid() as u32],
    });
    for (atom, path) in [
        (A_I3_CONFIG_PATH, current_configpath),
        (A_I3_LOG_STREAM_SOCKET_PATH, current_log_stream_socket_path),
    ] {
        connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new(root),
            property: x::Atom::new(atom),
            r#type: x::Atom::new(A_UTF8_STRING),
            data: CStr::from_ptr(path).to_bytes(),
        });
    }
    update_shmlog_atom();
}

pub unsafe fn x_set_warp_to(rect: *mut Rect) {
    if config.mouse_warping != Warping::None {
        warp_to = rect;
    }
}

pub unsafe fn x_mask_event_mask(mask: u32) {
    let mut values: u32 = (x::EventMask::BUTTON_PRESS.bits() as i32
        | x::EventMask::BUTTON_RELEASE.bits() as i32
        | x::EventMask::POINTER_MOTION.bits() as i32
        | x::EventMask::EXPOSURE.bits() as i32
        | x::EventMask::STRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::SUBSTRUCTURE_REDIRECT.bits() as i32
        | x::EventMask::SUBSTRUCTURE_NOTIFY.bits() as i32
        | x::EventMask::ENTER_WINDOW.bits() as i32) as u32
        & mask;
    let mut state: *mut FrameState = ::core::ptr::null_mut::<FrameState>();
    state = state_head.cqh_last;
    while state != &raw mut state_head as *mut ::core::ffi::c_void as *mut FrameState {
        if (*state).mapped {
            set_event_mask((*state).id, values);
        }
        state = (*state).state.cqe_prev;
    }
}

pub unsafe fn x_set_shape(con: *mut Con, kind: shape::Sk, enable: bool) {
    if state_for_frame((*con).frame.id).is_null() {
        ::log::error!(
            "ERROR: window state for con {} not found\n",
            i3_common::logging::pointer(con)
        );
        return;
    }
    match kind {
        shape::Sk::Bounding => {
            (*(*con).window).shaped = enable;
        }
        shape::Sk::Input => {
            (*(*con).window).input_shaped = enable;
        }
        _ => {
            ::log::error!(
                "ERROR: Received unknown shape event kind for con {}. This is a bug.\n",
                i3_common::logging::pointer(con)
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
        let _ = i3_common::display::connection()
            .expect("active XCB display")
            .flush();
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

use libc::{free, getpid, strlen};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
use crate::types::deco_render_params;
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
use crate::application::root_screen_info;
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
use i3_common::draw_util::OwnedPixmap;
use i3_common::font::predict_text_width;

use crate::con::arena::nodes;

use i3_common::logging::{emit_parts, string_bytes, Level};

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
        use ::xcb::Raw;
        let bytes = unsafe { std::slice::from_raw_parts(event.as_raw().cast::<u8>(), 32) };
        let mut expected = [0u8; 32];
        expected[0] = 33;
        expected[1] = 32;
        expected[4..8].copy_from_slice(&u32::MAX.to_ne_bytes());
        expected[8..12].copy_from_slice(&12u32.to_ne_bytes());
        expected[12..16].copy_from_slice(&13u32.to_ne_bytes());
        assert_eq!(bytes, expected);
    }
    #[test]
    fn protocol_counts_keep_signed_native_arithmetic_and_payload_bounds() {
        assert_eq!(protocol_word_count(3, 8), 3);
        assert_eq!(protocol_word_count(8, 3), 3);
        assert_eq!(protocol_word_count(0, u32::MAX), 0);
        assert_eq!(protocol_word_count(0x4000_0000, 8), 0);
        assert_eq!(protocol_word_count(0x8000_0003, 8), 3);
        assert_eq!(protocol_word_count(u32::MAX, 8), 8);
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
        let all = border_rectangles(
            BorderStyle::Pixel,
            true,
            AdjacentEdges::empty(),
            border,
            rect,
        );
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
            border_rectangles(BorderStyle::Normal, true, AdjacentEdges::LEFT, border, rect).len,
            2
        );
        assert_eq!(
            border_rectangles(
                BorderStyle::None,
                true,
                AdjacentEdges::empty(),
                border,
                rect
            )
            .len,
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
            ContainerType::Root
        ));
        assert!(needs_pixmap(
            false,
            BorderStyle::None,
            Layout::Tabbed,
            ContainerType::Con
        ));
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
