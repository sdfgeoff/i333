// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
//! Native placeholder windows with owned display text and scoped event buffers.
use crate::con::arena::{floating, identity, resolve};
use crate::domains::{ContainerType, DockMatch};
use crate::xcursor::Cursor;
use i3_common::display::{CairoVisual, DisplayScreen};
use i3_common::draw_util::draw_util_text;
use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop, EV_READ,
};
use std::ffi::{CStr, CString};
use std::os::fd::AsRawFd;
use std::rc::Rc;
use xcb::ffi::xcb_connection_t;
use xcb::{x, Xid, XidNew};

#[repr(C)]
pub struct placeholder_state {
    pub window: u32,
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
// Placeholder windows intentionally use a second connection, independent of
// the main display registry. `restore_conn` is only its native request alias.
static mut RESTORE_CONNECTION: Option<Rc<xcb::Connection>> = None;
static mut RESTORE_RENDERER: Option<Rc<i3_common::draw_util::RenderingContext>> = None;
static mut RESTORE_SCREEN: Option<DisplayScreen> = None;
static mut RESTORE_VISUAL: Option<CairoVisual> = None;

/// Call only on the event thread after restore_connect, before restore_shutdown.
unsafe fn restore_connection() -> Rc<xcb::Connection> {
    RESTORE_CONNECTION
        .as_ref()
        .expect("placeholder connection is live")
        .clone()
}

static mut xcb_watcher: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
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
    let swallow = unsafe { &*swallow };
    let fields = [
        &swallow.class,
        &swallow.instance,
        &swallow.window_role,
        &swallow.title,
        &swallow.machine,
    ];
    serialize_swallow(fields.map(|field| field.as_ref().map(|regex| regex.pattern())))
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
    if restore_connection().has_error().is_err() {
        ::log::debug!(concat!(
            "../src/restore_layout.c:restore_xcb_prepare_cb:53 - ",
            "restore X11 connection has an error, reconnecting\n"
        ));
        restore_connect();
        return;
    }
    loop {
        // Keep only the event/error owner across callbacks; the temporary
        // connection clone is released before they can replace the display.
        let next = restore_connection().poll_for_event();
        match next {
            Ok(Some(event)) => {
                restore_handle_event(&event);
            }
            Err(xcb::Error::Protocol(error)) => {
                let error = &*error.as_raw();
                ::log::debug!(
                    concat!(
                        "../src/restore_layout.c:restore_xcb_prepare_cb:62 - ",
                        "X11 Error received (probably harmless)! sequence ",
                        "0x{:x}, error_code = {}\n"
                    ),
                    (i32::from(error.sequence)) as u32,
                    { (i32::from(error.error_code)) }
                );
            }
            Ok(None) | Err(xcb::Error::Connection(_)) => break,
        }
    }
    let _ = restore_connection().flush();
}

/// Stop placeholder callbacks and free graphics while their connection is live.
/// Called on the event thread before reconnecting or during main native cleanup.
pub unsafe fn restore_shutdown() {
    if !restore_conn.is_null() {
        ev_io_stop(main_loop, xcb_watcher);
        ev_prepare_stop(main_loop, xcb_prepare);
        while !state_head.tqh_first.is_null() {
            let state: *mut placeholder_state = state_head.tqh_first;
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            drop(Box::from_raw(state));
        }
        restore_conn = std::ptr::null_mut();
        drop(RESTORE_RENDERER.take());
        drop(RESTORE_VISUAL.take());
        drop(RESTORE_SCREEN.take());
        drop(RESTORE_CONNECTION.take());
        free(xcb_watcher as *mut ::core::ffi::c_void);
        free(xcb_prepare as *mut ::core::ffi::c_void);
    }
    xcb_watcher = std::ptr::null_mut();
    xcb_prepare = std::ptr::null_mut();
}

pub unsafe fn restore_connect() {
    restore_shutdown();
    // No extension event metadata: unknown native opcodes/subtypes remain
    // owned UnknownEvent values, exactly as on the main connection.
    let (connection, screen_index) = match xcb::Connection::connect(None) {
        Ok(connection) => connection,
        Err(_) => errx(EXIT_FAILURE, c"Cannot open display".as_ptr()),
    };
    restore_conn = connection.get_raw_conn().cast();
    let connection = Rc::new(connection);
    let screen = DisplayScreen::new(connection.clone(), screen_index)
        .expect("placeholder setup screen exists");
    RESTORE_VISUAL = screen.visual(screen.info().root_visual);
    RESTORE_RENDERER = Some(i3_common::draw_util::RenderingContext::new(screen.clone()));
    RESTORE_SCREEN = Some(screen);
    RESTORE_CONNECTION = Some(connection);
    xcb_watcher = scalloc(1_usize, ::core::mem::size_of::<ev_io>() as usize) as *mut ev_io;
    xcb_prepare =
        scalloc(1_usize, ::core::mem::size_of::<ev_prepare>() as usize) as *mut ev_prepare;
    xcb_watcher.write(ev_io::new(
        restore_connection().as_raw_fd(),
        EV_READ,
        restore_xcb_got_event,
    ));
    ev_io_start(main_loop, xcb_watcher);
    xcb_prepare.write(ev_prepare::new(restore_xcb_prepare_cb));
    ev_prepare_start(main_loop, xcb_prepare);
}
// ICCCM WM_HINTS contains nine 32-bit words. Advertise only InputHint,
// with input=false, so native placeholders never accept keyboard focus.
const PLACEHOLDER_HINTS: [u32; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];

unsafe fn restore_sync() {
    let connection = restore_connection();
    // xcb_aux_sync uses the checked GetInputFocus cookie and discards its error.
    let cookie = connection.send_request(&x::GetInputFocus {});
    let _ = connection.wait_for_reply(cookie);
}

unsafe fn update_placeholder_contents(state: *mut placeholder_state) {
    let foreground: color_t = config.client.placeholder.text;
    let background: color_t = config.client.placeholder.background;
    draw_util_clear_surface(&raw mut (*state).surface, background);
    restore_sync();
    let mut swallows: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    swallows = (*(*state).con).swallow_head.tqh_first;
    while !swallows.is_null() {
        if let Some(serialized) = swallow_text(swallows) {
            {
                let diagnostic_5 = i3_common::logging::pointer((*state).con);
                let diagnostic_6 = ((*state).window);
                let diagnostic_7 = (n) as i32;
                let diagnostic_8_owner = serialized.clone();
                let diagnostic_8_bytes = diagnostic_8_owner.as_bytes();
                match (std::str::from_utf8(diagnostic_8_bytes),) { (Ok(diagnostic_8_text),) => ::log::debug!(concat!("../src/restore_layout.c:update_placeholder_contents:166 - con {} (placeholder 0x{:08x}) line {}: {}\n"), diagnostic_5, diagnostic_6, diagnostic_7, diagnostic_8_text), _ => emit_parts(Level::Debug, &[b"../src/restore_layout.c:update_placeholder_contents:166 - con ", format!("{}", diagnostic_5).as_bytes(), b" (placeholder 0x", format!("{:08x}", diagnostic_6).as_bytes(), b") line ", format!("{}", diagnostic_7).as_bytes(), b": ", diagnostic_8_bytes, b"\n"]), }
            };
            let text = i3_common::string::I3String::from_cstr(&serialized);
            let (x, y, max_width) =
                text_geometry((*state).rect, config.font.height, logical_px(2), n);
            draw_util_text(
                &crate::config::config.font,
                &text,
                &raw mut (*state).surface,
                foreground,
                background,
                x,
                y,
                max_width,
            );
            n += 1;
        } else {
            ::log::debug!("../src/restore_layout.c:update_placeholder_contents:159 - This swallows specification is not serializable?!\n");
        }
        swallows = (*swallows).matches.tqe_next;
    }
    let line = i3_common::string::I3String::new("⌚");
    let text_width = predict_text_width(&crate::config::config.font, &line);
    let (x, y) = centered_text((*state).rect, text_width, config.font.height);
    draw_util_text(
        &crate::config::config.font,
        &line,
        &raw mut (*state).surface,
        foreground,
        background,
        x,
        y,
        text_width,
    );
    restore_sync();
}
unsafe fn open_placeholder_window(con: *mut Con) {
    if con_is_leaf(con) as ::core::ffi::c_int != 0
        && ((*con).window.is_null() || (*(*con).window).id as ::core::ffi::c_long == 0)
        && !(*con).swallow_head.tqh_first.is_null()
        && ((*con).type_0) == ContainerType::Con
    {
        let placeholder = create_window(
            &restore_connection(),
            (*con).rect,
            0,
            0,
            x::WindowClass::InputOutput,
            Cursor::Pointer,
            true,
            &[
                x::Cw::BackPixel(config.client.placeholder.background.colorpixel),
                x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::STRUCTURE_NOTIFY),
            ],
        );
        restore_connection().send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new(placeholder),
            property: x::ATOM_WM_HINTS,
            r#type: x::ATOM_WM_HINTS,
            data: &PLACEHOLDER_HINTS,
        });
        if !(*con).name.is_none() {
            restore_connection().send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x::Window::new(placeholder),
                property: x::Atom::new(A__NET_WM_NAME),
                r#type: x::Atom::new(A_UTF8_STRING),
                data: CStr::from_ptr((*con).name_ptr()).to_bytes(),
            });
        }
        {
            let diagnostic_5 = (placeholder);
            let diagnostic_6 = i3_common::logging::pointer(con);
            let diagnostic_7_owner = (*con).name.clone();
            let diagnostic_7_bytes = string_bytes(diagnostic_7_owner.as_deref(), None);
            match (std::str::from_utf8(diagnostic_7_bytes),) { (Ok(diagnostic_7_text),) => ::log::debug!(concat!("../src/restore_layout.c:open_placeholder_window:219 - ","Created placeholder window 0x{:08x} for leaf container ","{} / {}\n"), diagnostic_5, diagnostic_6, diagnostic_7_text), _ => emit_parts(Level::Debug, &[b"../src/restore_layout.c:open_placeholder_window:219 - Created placeholder window 0x", format!("{:08x}", diagnostic_5).as_bytes(), b" for leaf container ", format!("{}", diagnostic_6).as_bytes(), b" / ", diagnostic_7_bytes, b"\n"]), }
        };
        let state = Box::into_raw(Box::new(placeholder_state {
            window: placeholder,
            con,
            rect: (*con).rect,
            surface: surface_t::empty(),
            state: PlaceholderLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        }));
        draw_util_surface_init(
            RESTORE_RENDERER
                .as_ref()
                .expect("placeholder renderer initialized"),
            &raw mut (*state).surface,
            placeholder,
            RESTORE_VISUAL.as_ref(),
            (*state).rect.width as ::core::ffi::c_int,
            (*state).rect.height as ::core::ffi::c_int,
        );
        update_placeholder_contents(state);
        (*state).state.tqe_next = ::core::ptr::null_mut::<placeholder_state>();
        (*state).state.tqe_prev = state_head.tqh_last;
        *state_head.tqh_last = state;
        state_head.tqh_last = &raw mut (*state).state.tqe_next;
        let temp_id: *mut Match = Box::into_raw(Box::new(Match::empty()));

        (*temp_id).dock = DockMatch::DontCheck;
        (*temp_id).id = placeholder;
        (*temp_id).matches.tqe_next = (*con).swallow_head.tqh_first;
        if !(*temp_id).matches.tqe_next.is_null() {
            (*(*con).swallow_head.tqh_first).matches.tqe_prev =
                &raw mut (*temp_id).matches.tqe_next;
        } else {
            (*con).swallow_head.tqh_last = &raw mut (*temp_id).matches.tqe_next;
        }
        (*con).swallow_head.tqh_first = temp_id;
        (*temp_id).matches.tqe_prev = &raw mut (*con).swallow_head.tqh_first;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = nodes::first_ptr(con);
    while !child.is_null() {
        open_placeholder_window(child);
        child = nodes::next_ptr(child);
    }
    for floating_id in floating::children(identity(con).expect("live floating parent")) {
        let Some(child) = resolve(floating_id) else {
            continue;
        };
        open_placeholder_window(child);
    }
}

pub unsafe fn restore_open_placeholder_windows(parent: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = nodes::first_ptr(parent);
    while !child.is_null() {
        open_placeholder_window(child);
        child = nodes::next_ptr(child);
    }
    for floating_id in floating::children(identity(parent).expect("live floating parent")) {
        let Some(child) = resolve(floating_id) else {
            continue;
        };
        open_placeholder_window(child);
    }
    let _ = restore_connection().flush();
}

pub unsafe fn restore_kill_placeholder(placeholder: u32) -> bool {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first;
    while !state.is_null() {
        if (*state).window != placeholder {
            state = (*state).state.tqe_next;
        } else {
            restore_connection().send_request(&x::DestroyWindow {
                window: x::Window::new((*state).window),
            });
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            drop(Box::from_raw(state));
            ::log::debug!(
                concat!(
                    "../src/restore_layout.c:restore_kill_placeholder:284 - ",
                    "placeholder window 0x{:08x} destroyed.\n"
                ),
                { (placeholder) }
            );
            return true;
        }
    }
    ::log::debug!(
        concat!(
            "../src/restore_layout.c:restore_kill_placeholder:288 - ",
            "0x{:08x} is not a placeholder window, ignoring.\n"
        ),
        { (placeholder) }
    );
    false
}
unsafe fn expose_event(event: &x::ExposeEvent) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first;
    while !state.is_null() {
        if (*state).window != event.window().resource_id() {
            state = (*state).state.tqe_next;
        } else {
            ::log::debug!(
                concat!(
                    "../src/restore_layout.c:expose_event:299 - refreshing ",
                    "window 0x{:08x} contents (con {})\n"
                ),
                ((*state).window) as u32,
                i3_common::logging::pointer((*state).con)
            );
            update_placeholder_contents(state);
            return;
        }
    }
    ::log::error!(
        "ERROR: Received ExposeEvent for unknown window 0x{:08x}\n",
        { (event.window().resource_id()) }
    );
}
unsafe fn configure_notify(event: &x::ConfigureNotifyEvent) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first;
    while !state.is_null() {
        if (*state).window != event.window().resource_id() {
            state = (*state).state.tqe_next;
        } else {
            ::log::debug!(
                concat!(
                    "../src/restore_layout.c:configure_notify:323 - ",
                    "ConfigureNotify: window 0x{:08x} has now width={}, ",
                    "height={} (con {})\n"
                ),
                ((*state).window) as u32,
                { (event.width() as ::core::ffi::c_int) },
                { (event.height() as ::core::ffi::c_int) },
                i3_common::logging::pointer((*state).con)
            );
            (*state).rect.width = event.width() as u32;
            (*state).rect.height = event.height() as u32;
            draw_util_surface_set_size(
                &raw mut (*state).surface,
                (*state).rect.width as ::core::ffi::c_int,
                (*state).rect.height as ::core::ffi::c_int,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    ::log::error!(
        "ERROR: Received ConfigureNotify for unknown window 0x{:08x}\n",
        { (event.window().resource_id()) }
    );
}
unsafe fn restore_handle_event(event: &xcb::Event) {
    match event {
        xcb::Event::X(x::Event::Expose(event)) => {
            if event.count() == 0 {
                expose_event(event);
            }
        }
        xcb::Event::X(x::Event::ConfigureNotify(event)) => configure_notify(event),
        _ => {
            ::log::debug!(
                concat!(
                    "../src/restore_layout.c:restore_handle_event:350 - ",
                    "Received unhandled X11 event of type {}\n"
                ),
                { (i32::from((*event.as_raw()).response_type & 0x7f)) }
            );
        }
    }
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

pub use libc::{free, memmove, strlen};

use i3_common::native_stdio::errx;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::A_UTF8_STRING;
use crate::application::A__NET_WM_NAME;
use crate::con::con_is_leaf;
use crate::config::config;
use crate::xcb::create_window;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::font::predict_text_width;

use crate::con::arena::nodes;

use i3_common::logging::{emit_parts, string_bytes, Level};

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
