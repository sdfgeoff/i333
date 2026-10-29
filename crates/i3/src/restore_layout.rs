// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Native placeholder windows with owned display text and scoped event buffers.
use crate::xcursor::Cursor;
use i3_common::draw_util::draw_util_text;
use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop, EV_READ,
};
use std::ffi::{CStr, CString};
use std::ptr::NonNull;
const true_0: i32 = 1;
const false_0: i32 = 0;
const CT_CON: u32 = 2;
const M_DONTCHECK: i32 = -1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct placeholder_state {
    pub window: xcb_window_t,
    pub con: *mut Con,
    pub rect: Rect,
    pub surface: surface_t,
    pub state: PlaceholderLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlaceholderLinks {
    pub tqe_next: *mut placeholder_state,
    pub tqe_prev: *mut *mut placeholder_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_head {
    pub tqh_first: *mut placeholder_state,
    pub tqh_last: *mut *mut placeholder_state,
}

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut state_head: state_head = state_head {
    tqh_first: ::core::ptr::null::<placeholder_state>() as *mut placeholder_state,
    tqh_last: ::core::ptr::null::<*mut placeholder_state>() as *mut *mut placeholder_state,
};
static mut restore_conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut xcb_watcher: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
struct NativeEvent(NonNull<xcb_generic_event_t>);
impl NativeEvent {
    unsafe fn take(ptr: *mut xcb_generic_event_t) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}
impl Drop for NativeEvent {
    fn drop(&mut self) {
        unsafe {
            free(self.0.as_ptr().cast());
        }
    }
}
fn serialize_swallow(fields: [Option<&CStr>; 5]) -> Option<CString> {
    let mut output = Vec::new();
    for (name, value) in [
        b"class".as_slice(),
        b"instance",
        b"window_role",
        b"title",
        b"machine",
    ]
    .into_iter()
    .zip(fields)
    {
        if let Some(value) = value {
            output.push(if output.is_empty() { b'[' } else { b' ' });
            output.extend_from_slice(name);
            output.extend_from_slice(b"=\"");
            output.extend_from_slice(value.to_bytes());
            output.push(b'"');
        }
    }
    if output.is_empty() {
        None
    } else {
        output.push(b']');
        Some(CString::new(output).unwrap())
    }
}
/// Borrow only pattern values long enough to copy the display text; never keep
/// a Match/Con reference across drawing/native callbacks.
unsafe fn swallow_text(swallow: *const Match) -> Option<CString> {
    let fields = unsafe {
        [
            (*swallow).class,
            (*swallow).instance,
            (*swallow).window_role,
            (*swallow).title,
            (*swallow).machine,
        ]
    };
    serialize_swallow(fields.map(|field| unsafe { field.as_ref() }.map(|regex| regex.pattern())))
}
fn text_geometry(rect: Rect, font_height: i32, padding: i32, line: i32) -> (i32, i32, i32) {
    (
        padding,
        line.wrapping_mul(font_height.wrapping_add(padding))
            .wrapping_add(padding),
        rect.width.wrapping_sub(padding.wrapping_mul(2) as u32) as i32,
    )
}
fn centered_text(rect: Rect, width: i32, height: i32) -> (i32, i32) {
    (
        rect.width.wrapping_div(2).wrapping_sub((width / 2) as u32) as i32,
        rect.height
            .wrapping_div(2)
            .wrapping_sub((height / 2) as u32) as i32,
    )
}
unsafe extern "C" fn restore_xcb_got_event(
    loop_0: *mut ev_loop,
    w: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
}
unsafe extern "C" fn restore_xcb_prepare_cb(
    loop_0: *mut ev_loop,
    w: *mut ev_prepare,
    revents: ::core::ffi::c_int,
) {
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    if xcb_connection_has_error(restore_conn) != 0 {
        debuglog(
            c"%s:%s:%d - restore X11 connection has an error, reconnecting\n"
                .as_ptr()
                .cast_mut(),
            c"../src/restore_layout.c".as_ptr(),
            c"restore_xcb_prepare_cb".as_ptr(),
            53 as ::core::ffi::c_int,
        );
        restore_connect();
        return;
    }
    loop {
        let Some(owned_event) = NativeEvent::take(xcb_poll_for_event(restore_conn)) else {
            break;
        };
        event = owned_event.0.as_ptr();
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_xcb_prepare_cb".as_ptr(),
                62 as ::core::ffi::c_int,
                (*error).sequence as ::core::ffi::c_int,
                (*error).error_code as ::core::ffi::c_int,
            );
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            restore_handle_event(type_0, event);
        }
    }
    xcb_flush(restore_conn);
}

pub unsafe fn restore_connect() {
    if !restore_conn.is_null() {
        ev_io_stop(main_loop, xcb_watcher);
        ev_prepare_stop(main_loop, xcb_prepare);
        while !state_head.tqh_first.is_null() {
            let state: *mut placeholder_state = state_head.tqh_first as *mut placeholder_state;
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            free(state as *mut ::core::ffi::c_void);
        }
        xcb_disconnect(restore_conn);
        free(xcb_watcher as *mut ::core::ffi::c_void);
        free(xcb_prepare as *mut ::core::ffi::c_void);
    }
    let mut screen: ::core::ffi::c_int = 0;
    restore_conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if restore_conn.is_null() || xcb_connection_has_error(restore_conn) != 0 {
        if !restore_conn.is_null() {
            xcb_disconnect(restore_conn);
        }
        errx(EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    xcb_watcher = scalloc(1 as usize, ::core::mem::size_of::<ev_io>() as usize) as *mut ev_io;
    xcb_prepare =
        scalloc(1 as usize, ::core::mem::size_of::<ev_prepare>() as usize) as *mut ev_prepare;
    xcb_watcher.write(ev_io::new(
        xcb_get_file_descriptor(restore_conn),
        EV_READ,
        restore_xcb_got_event,
    ));
    ev_io_start(main_loop, xcb_watcher);
    xcb_prepare.write(ev_prepare::new(restore_xcb_prepare_cb));
    ev_prepare_start(main_loop, xcb_prepare);
}
unsafe fn update_placeholder_contents(state: *mut placeholder_state) {
    let foreground: color_t = config.client.placeholder.text;
    let background: color_t = config.client.placeholder.background;
    draw_util_clear_surface(&raw mut (*state).surface, background);
    xcb_aux_sync(restore_conn);
    let mut swallows: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    swallows = (*(*state).con).swallow_head.tqh_first as *mut Match;
    while !swallows.is_null() {
        if let Some(serialized) = swallow_text(swallows) {
            debuglog(
                c"%s:%s:%d - con %p (placeholder 0x%08x) line %d: %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"update_placeholder_contents".as_ptr(),
                166 as libc::c_int,
                (*state).con,
                (*state).window,
                n,
                serialized.as_ptr(),
            );
            let text = i3_common::string::I3String::from_cstr(&serialized);
            let (x, y, max_width) =
                text_geometry((*state).rect, config.font.height, logical_px(2), n);
            draw_util_text(
                (&raw const text).cast_mut(),
                &raw mut (*state).surface,
                foreground,
                background,
                x,
                y,
                max_width,
            );
            n += 1;
        } else {
            debuglog(
                c"%s:%s:%d - This swallows specification is not serializable?!\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"update_placeholder_contents".as_ptr(),
                159 as libc::c_int,
            );
        }
        swallows = (*swallows).matches.tqe_next as *mut Match;
    }
    let line = i3_common::string::I3String::new("⌚");
    let text_width = predict_text_width((&raw const line).cast_mut());
    let (x, y) = centered_text((*state).rect, text_width, config.font.height);
    draw_util_text(
        (&raw const line).cast_mut(),
        &raw mut (*state).surface,
        foreground,
        background,
        x,
        y,
        text_width,
    );
    xcb_aux_sync(restore_conn);
}
unsafe fn open_placeholder_window(con: *mut Con) {
    if con_is_leaf(con) as ::core::ffi::c_int != 0
        && ((*con).window.is_null() || (*(*con).window).id as ::core::ffi::c_long == XCB_NONE)
        && !(*con).swallow_head.tqh_first.is_null()
        && (*con).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut fresh2: [u32; 2] = [
            config.client.placeholder.background.colorpixel,
            (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int) as u32,
        ];
        let placeholder: xcb_window_t = create_window(
            restore_conn,
            (*con).rect as Rect,
            XCB_COPY_FROM_PARENT as u16,
            XCB_COPY_FROM_PARENT as xcb_visualid_t,
            XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
            Cursor::Pointer,
            true_0 != 0,
            (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int)
                as u32,
            &raw mut fresh2 as *mut u32,
        );
        let mut hints: xcb_icccm_wm_hints_t = xcb_icccm_wm_hints_t {
            flags: 0,
            input: 0,
            initial_state: 0,
            icon_pixmap: 0,
            icon_window: 0,
            icon_x: 0,
            icon_y: 0,
            icon_mask: 0,
            window_group: 0,
        };
        xcb_icccm_wm_hints_set_none(&raw mut hints);
        xcb_icccm_wm_hints_set_input(&raw mut hints, 0 as u8);
        xcb_icccm_set_wm_hints(restore_conn, placeholder, &raw mut hints);
        if !(*con).name.is_null() {
            xcb_change_property(
                restore_conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                placeholder,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as u8,
                strlen((*con).name) as u32,
                (*con).name as *const ::core::ffi::c_void,
            );
        }
        debuglog(
            c"%s:%s:%d - Created placeholder window 0x%08x for leaf container %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/restore_layout.c".as_ptr(),
            c"open_placeholder_window".as_ptr(),
            219 as ::core::ffi::c_int,
            placeholder,
            con,
            (*con).name,
        );
        let state: *mut placeholder_state = scalloc(
            1 as usize,
            ::core::mem::size_of::<placeholder_state>() as usize,
        ) as *mut placeholder_state;
        (*state).window = placeholder;
        (*state).con = con;
        (*state).rect = (*con).rect as Rect;
        draw_util_surface_init(
            restore_conn,
            &raw mut (*state).surface,
            placeholder as xcb_drawable_t,
            get_visualtype(root_screen),
            (*state).rect.width as ::core::ffi::c_int,
            (*state).rect.height as ::core::ffi::c_int,
        );
        update_placeholder_contents(state);
        (*state).state.tqe_next = ::core::ptr::null_mut::<placeholder_state>();
        (*state).state.tqe_prev = state_head.tqh_last;
        *state_head.tqh_last = state as *mut placeholder_state;
        state_head.tqh_last = &raw mut (*state).state.tqe_next;
        let temp_id: *mut Match = smalloc(::core::mem::size_of::<Match>() as usize) as *mut Match;
        match_init(temp_id);
        (*temp_id).dock = M_DONTCHECK;
        (*temp_id).id = placeholder;
        (*temp_id).matches.tqe_next = (*con).swallow_head.tqh_first;
        if !(*temp_id).matches.tqe_next.is_null() {
            (*(*con).swallow_head.tqh_first).matches.tqe_prev =
                &raw mut (*temp_id).matches.tqe_next;
        } else {
            (*con).swallow_head.tqh_last = &raw mut (*temp_id).matches.tqe_next;
        }
        (*con).swallow_head.tqh_first = temp_id as *mut Match;
        (*temp_id).matches.tqe_prev = &raw mut (*con).swallow_head.tqh_first;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
}

pub unsafe fn restore_open_placeholder_windows(parent: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*parent).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*parent).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    xcb_flush(restore_conn);
}

pub unsafe fn restore_kill_placeholder(placeholder: xcb_window_t) -> bool {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != placeholder {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            xcb_destroy_window(restore_conn, (*state).window);
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            free(state as *mut ::core::ffi::c_void);
            debuglog(
                c"%s:%s:%d - placeholder window 0x%08x destroyed.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_kill_placeholder".as_ptr(),
                284 as ::core::ffi::c_int,
                placeholder,
            );
            return true_0 != 0;
        }
    }
    debuglog(
        c"%s:%s:%d - 0x%08x is not a placeholder window, ignoring.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/restore_layout.c".as_ptr(),
        c"restore_kill_placeholder".as_ptr(),
        288 as ::core::ffi::c_int,
        placeholder,
    );
    false_0 != 0
}
unsafe fn expose_event(event: *mut xcb_expose_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                c"%s:%s:%d - refreshing window 0x%08x contents (con %p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"expose_event".as_ptr(),
                299 as ::core::ffi::c_int,
                (*state).window,
                (*state).con,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        c"ERROR: Received ExposeEvent for unknown window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        (*event).window,
    );
}
unsafe fn configure_notify(event: *mut xcb_configure_notify_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                c"%s:%s:%d - ConfigureNotify: window 0x%08x has now width=%d, height=%d (con %p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"configure_notify".as_ptr(),
                323 as ::core::ffi::c_int,
                (*state).window,
                (*event).width as ::core::ffi::c_int,
                (*event).height as ::core::ffi::c_int,
                (*state).con,
            );
            (*state).rect.width = (*event).width as u32;
            (*state).rect.height = (*event).height as u32;
            draw_util_surface_set_size(
                &raw mut (*state).surface,
                (*state).rect.width as ::core::ffi::c_int,
                (*state).rect.height as ::core::ffi::c_int,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        c"ERROR: Received ConfigureNotify for unknown window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        (*event).window,
    );
}
unsafe fn restore_handle_event(type_0: ::core::ffi::c_int, event: *mut xcb_generic_event_t) {
    match type_0 {
        XCB_EXPOSE => {
            if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                expose_event(event as *mut xcb_expose_event_t);
            }
        }
        XCB_CONFIGURE_NOTIFY => {
            configure_notify(event as *mut xcb_configure_notify_event_t);
        }
        _ => {
            debuglog(
                c"%s:%s:%d - Received unhandled X11 event of type %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_handle_event".as_ptr(),
                350 as ::core::ffi::c_int,
                type_0,
            );
        }
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    state_head = state_head {
        tqh_first: ::core::ptr::null_mut::<placeholder_state>(),
        tqh_last: &raw mut state_head.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_aux_sync, xcb_change_property, xcb_configure_notify_event_t, xcb_connect,
    xcb_connection_has_error, xcb_connection_t, xcb_destroy_window, xcb_disconnect, xcb_drawable_t,
    xcb_expose_event_t, xcb_flush, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_file_descriptor, xcb_icccm_set_wm_hints, xcb_icccm_wm_hints_set_input,
    xcb_icccm_wm_hints_set_none, xcb_icccm_wm_hints_t, xcb_poll_for_event, xcb_visualid_t,
    xcb_window_t, XCB_CONFIGURE_NOTIFY, XCB_COPY_FROM_PARENT, XCB_CW_BACK_PIXEL, XCB_CW_EVENT_MASK,
    XCB_EVENT_MASK_EXPOSURE, XCB_EVENT_MASK_STRUCTURE_NOTIFY, XCB_EXPOSE, XCB_NONE,
    XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::{free, memmove, strlen};

use i3_common::native_stdio::errx;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::root_screen;
use crate::application::A_UTF8_STRING;
use crate::application::A__NET_WM_NAME;
use crate::con::con_is_leaf;
use crate::config::config;
use crate::r#match::match_init;
use crate::xcb::create_window;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::font::predict_text_width;
use i3_common::get_visualtype::get_visualtype;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swallow_display_retains_order_raw_bytes_quotes_and_empty_values() {
        assert!(serialize_swallow([None; 5]).is_none());
        assert_eq!(
            serialize_swallow([Some(c"A\xff"), Some(c""), None, Some(c"q\"\\"), None])
                .unwrap()
                .to_bytes(),
            b"[class=\"A\xff\" instance=\"\" title=\"q\"\\\"]"
        );
    }
    #[test]
    fn placeholder_geometry_preserves_unsigned_center_and_width_projection() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 3,
            height: 5,
        };
        assert_eq!(text_geometry(rect, 10, 2, 3), (2, 38, -1));
        assert_eq!(centered_text(rect, 8, 10), (-3, -3));
    }
}
