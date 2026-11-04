use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::domains::{ContainerType, FullscreenMode};
use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::FocusActivation;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::WindowDock;
use xcb::{x, Raw, Xid, XidNew};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * handlers.c: Small handlers for various events (keypresses, focus changes,
 *             …).
 *
 */
// Mechanical file-by-file Rust translation of src/handlers.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type Output = xoutput;
pub type i3Window = Window;

pub use crate::application::ignore_event_sequence_t;

pub type PropertyCallback = Option<unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool>;
#[derive(Copy, Clone)]
pub struct PropertyHandler {
    pub atom: u32,
    pub long_len: u32,
    pub cb: PropertyCallback,
}

pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 69] = unsafe {
    ::core::mem::transmute::<[u8; 69], [::core::ffi::c_char; 69]>(
        *b"_Bool handle_strut_partial_change(Con *, xcb_get_property_reply_t *)\0",
    )
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_OUTPUT: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 1 as ::core::ffi::c_ulong;
pub const _NET_WM_STATE_REMOVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _NET_WM_STATE_ADD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _NET_WM_STATE_TOGGLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

pub static mut randr_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub static mut xkb_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub static mut xkb_current_group: ::core::ffi::c_int = 0;

pub static mut shape_base: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
struct IgnoredEvent {
    sequence: u32,
    response: i32,
    added: libc::time_t,
}
#[derive(Default)]
struct IgnoredEvents(Vec<IgnoredEvent>);
impl IgnoredEvents {
    fn add(&mut self, sequence: u32, response: i32, now: libc::time_t) {
        self.0.insert(
            0,
            IgnoredEvent {
                sequence,
                response,
                added: now,
            },
        );
    }
    fn contains(&mut self, sequence: u32, response: i32, now: libc::time_t) -> bool {
        // Preserve wall-clock seconds, backward-clock behavior, and strict >5.
        self.0.retain(|event| now.wrapping_sub(event.added) <= 5);
        self.0.iter().any(|event| {
            event.sequence == sequence && (event.response == -1 || event.response == response)
        })
    }
}
static IGNORED_EVENTS: std::sync::Mutex<IgnoredEvents> =
    std::sync::Mutex::new(IgnoredEvents(Vec::new()));

pub fn add_ignore_event(sequence: ignore_event_sequence_t, response_type: i32) {
    let now = unsafe { time(std::ptr::null_mut()) };
    IGNORED_EVENTS
        .lock()
        .unwrap()
        .add(sequence.value, response_type, now);
}
pub fn event_is_ignored(sequence: ignore_event_sequence_t, response_type: i32) -> bool {
    let now = unsafe { time(std::ptr::null_mut()) };
    IGNORED_EVENTS
        .lock()
        .unwrap()
        .contains(sequence.value, response_type, now)
}
unsafe fn check_crossing_screen_boundary(x: u32, y: u32) {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    if config.disable_focus_follows_mouse {
        return;
    }
    output = get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if output.is_null() {
        ::log::error!("ERROR: ERROR: No such screen\n");
        return;
    }
    if (*output).con.is_null() {
        ::log::error!(
            "ERROR: ERROR: The screen is not recognized by i3 (no container associated)\n"
        );
        return;
    }
    let old_focused: *mut Con = focused;
    let next: *mut Con = con_descend_focused(output_get_content((*output).con));
    workspace_show(con_get_workspace(next));
    con_focus(next);
    if old_focused != focused {
        tree_render();
    }
}
fn enter_selectors(event: &x::EnterNotifyEvent) -> (u8, u8) {
    // Binding gap: mode/detail are open wire bytes, whereas generated getters
    // construct closed Rust enums. Both offsets are inside the fixed event.
    unsafe {
        let bytes = event.as_raw().cast::<u8>();
        (*bytes.add(30), *bytes.add(1))
    }
}

fn focus_selectors(event: &x::FocusInEvent) -> (u8, u8) {
    // FocusOut is the same maintained record type; preserve unknown values for
    // existing policy comparisons and diagnostic fallbacks.
    unsafe {
        let bytes = event.as_raw().cast::<u8>();
        (*bytes.add(8), *bytes.add(1))
    }
}

unsafe fn handle_enter_notify(event: &x::EnterNotifyEvent, full_sequence: u32) {
    let (event_mode, event_detail) = enter_selectors(event);
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    last_timestamp = event.time();
    ::log::debug!("../src/handlers.c:handle_enter_notify:133 - enter_notify for {:08x}, mode = {}, detail {}, serial {}\n", { (event.event().resource_id()) }, { (event_mode as ::core::ffi::c_int) }, { (event_detail as ::core::ffi::c_int) }, { (event.sequence() as ::core::ffi::c_int) });
    ::log::debug!(
        "../src/handlers.c:handle_enter_notify:134 - coordinates {}, {}\n",
        { (event.event_x() as ::core::ffi::c_int) },
        { (event.event_y() as ::core::ffi::c_int) }
    );
    if event_mode as ::core::ffi::c_int
        != (::xcb::x::NotifyMode::Normal as u32) as ::core::ffi::c_int
    {
        ::log::debug!(
            "../src/handlers.c:handle_enter_notify:136 - This was not a normal notify, ignoring\n"
        );
        return;
    }
    if event_is_ignored(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        (<::xcb::x::EnterNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32),
    ) {
        ::log::debug!("../src/handlers.c:handle_enter_notify:142 - Event ignored\n");
        return;
    }
    let mut enter_child: bool = false;
    con = con_by_frame_id(event.event().resource_id());
    if con.is_null() {
        con = con_by_window_id(event.event().resource_id());
        enter_child = true;
    }
    if con.is_null() || ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
        ::log::debug!(
            "../src/handlers.c:handle_enter_notify:157 - Getting screen at {} x {}\n",
            { (event.root_x() as ::core::ffi::c_int) },
            { (event.root_y() as ::core::ffi::c_int) }
        );
        check_crossing_screen_boundary(event.root_x() as u32, event.root_y() as u32);
        return;
    }
    let layout: Layout = if enter_child as ::core::ffi::c_int != 0 {
        (*parent_ptr(con)).layout
    } else {
        (*con).layout
    };
    if layout == Layout::Default {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = nodes::last_ptr(con);
        while !child.is_null() {
            if rect_contains(
                (*child).deco_rect as Rect,
                event.event_x() as u32,
                event.event_y() as u32,
            ) {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = i3_common::logging::pointer(child);
                    let log_pointer_2 = (*child).name_ptr();
                    let log_value_1 = i3_common::logging::string_bytes(
                        if log_pointer_2.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(log_pointer_2))
                        },
                        None,
                    )
                    .to_vec();
                    if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                        ::log::info!(
                            "using child {} / {} instead!\n",
                            log_value_0,
                            log_value_1_text
                        );
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                format!("using child {} / ", log_value_0).as_bytes(),
                                &log_value_1,
                                " instead!\n".as_bytes(),
                            ],
                        );
                    }
                };
                con = child;
                break;
            } else {
                child = nodes::previous_ptr(child);
            }
        }
    }
    if config.disable_focus_follows_mouse {
        return;
    }
    if con == focused {
        return;
    }
    let ws: *mut Con = con_get_workspace(con);
    if ws != con_get_workspace(focused) {
        workspace_show(ws);
    }
    focused_id = 0_u32;
    con_focus(con_descend_focused(con));
    tree_render();
}
unsafe fn handle_motion_notify(event: &x::MotionNotifyEvent) {
    last_timestamp = event.time();
    if event.child().resource_id() as ::core::ffi::c_long != 0 {
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_frame_id(event.event().resource_id());
    if con.is_null() {
        ::log::debug!("../src/handlers.c:handle_motion_notify:214 - MotionNotify for an unknown container, checking if it crosses screen boundaries.\n");
        check_crossing_screen_boundary(event.root_x() as u32, event.root_y() as u32);
        return;
    }
    if config.disable_focus_follows_mouse {
        return;
    }
    if (*con).layout != Layout::Default
        && (*con).layout != Layout::SplitV
        && (*con).layout != Layout::SplitH
    {
        return;
    }
    if !(*con).window.is_null() {
        if rect_contains(
            (*con).deco_rect as Rect,
            event.event_x() as u32,
            event.event_y() as u32,
        ) {
            if focus::first_ptr(parent_ptr(con)) == con {
                return;
            }
            con_focus(con);
            x_push_changes(croot);
        }
    } else {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = nodes::last_ptr(con);
        while !current.is_null() {
            if !rect_contains(
                (*current).deco_rect as Rect,
                event.event_x() as u32,
                event.event_y() as u32,
            ) {
                current = nodes::previous_ptr(current);
            } else {
                if focus::first_ptr(con) == current {
                    return;
                }
                con_focus(current);
                x_push_changes(croot);
                return;
            }
        }
    }
}
unsafe fn handle_mapping_notify(event: &x::MappingNotifyEvent) {
    // request() transmutes the native selector. Inspect its wire byte first;
    // unknown mappings remain ignored rather than becoming invalid Rust enums.
    let request = *event.as_raw().cast::<u8>().add(4);
    if request != x::Mapping::Keyboard as u8 && request != x::Mapping::Modifier as u8 {
        return;
    }
    ::log::debug!("../src/handlers.c:handle_mapping_notify:269 - Received mapping_notify for keyboard or modifier mapping, re-grabbing keys\n");
    crate::application::refresh_key_symbols(event);
    xcb_numlock_mask =
        crate::application::key_symbol_modifiers((xkbcommon::xkb::keysyms::KEY_Num_Lock));
    ungrab_all_keys();
    translate_keysyms();
    grab_all_keys();
}
unsafe fn handle_map_request(event: &x::MapRequestEvent, full_sequence: u32) {
    let cookie = i3_common::display::connection()
        .expect("main X11 connection is registered")
        .send_request_unchecked(&x::GetWindowAttributes {
            window: event.window(),
        });
    ::log::debug!(
        "../src/handlers.c:handle_map_request:286 - window = 0x{:08x}, serial is {}.\n",
        { (event.window().resource_id()) },
        { (event.sequence() as ::core::ffi::c_int) }
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        -(1 as ::core::ffi::c_int),
    );
    manage_window(
        event.window().resource_id(),
        crate::manage::AttributesCookie::Unchecked(cookie),
        false,
    );
}
fn configure_selectors(event: &x::ConfigureRequestEvent) -> (u8, u16) {
    // Binding gaps: the stack mode is an open byte, and the mask getter drops
    // unknown bits. Preserve both before policy filters the outgoing values.
    unsafe {
        let bytes = event.as_raw().cast::<u8>();
        (*bytes.add(1), bytes.add(26).cast::<u16>().read_unaligned())
    }
}

fn configure_values(event: &x::ConfigureRequestEvent) -> (u16, Vec<u32>) {
    let mut mask = 0;
    let mut values = Vec::with_capacity(7);
    for (bit, value) in [
        ((::xcb::x::ConfigWindowMask::X.bits()), event.x() as u32),
        ((::xcb::x::ConfigWindowMask::Y.bits()), event.y() as u32),
        (
            (::xcb::x::ConfigWindowMask::WIDTH.bits()),
            u32::from(event.width()),
        ),
        (
            (::xcb::x::ConfigWindowMask::HEIGHT.bits()),
            u32::from(event.height()),
        ),
        (
            (::xcb::x::ConfigWindowMask::BORDER_WIDTH.bits()),
            u32::from(event.border_width()),
        ),
        (
            (::xcb::x::ConfigWindowMask::SIBLING.bits()),
            event.sibling().resource_id(),
        ),
        (
            (::xcb::x::ConfigWindowMask::STACK_MODE.bits()),
            u32::from(configure_selectors(event).0),
        ),
    ] {
        if configure_selectors(event).1 & bit as u16 != 0 {
            mask |= bit as u16;
            values.push(value);
        }
    }
    (mask, values)
}

fn checked_stack_mode(raw: u8) -> Option<x::StackMode> {
    match raw {
        0 => Some(x::StackMode::Above),
        1 => Some(x::StackMode::Below),
        2 => Some(x::StackMode::TopIf),
        3 => Some(x::StackMode::BottomIf),
        4 => Some(x::StackMode::Opposite),
        _ => None,
    }
}
fn configure_window_values(event: &x::ConfigureRequestEvent) -> Vec<x::ConfigWindow> {
    let mut values = Vec::with_capacity(7);
    for (bit, value) in [
        (
            (::xcb::x::ConfigWindowMask::X.bits()),
            x::ConfigWindow::X(i32::from(event.x())),
        ),
        (
            (::xcb::x::ConfigWindowMask::Y.bits()),
            x::ConfigWindow::Y(i32::from(event.y())),
        ),
        (
            (::xcb::x::ConfigWindowMask::WIDTH.bits()),
            x::ConfigWindow::Width(u32::from(event.width())),
        ),
        (
            (::xcb::x::ConfigWindowMask::HEIGHT.bits()),
            x::ConfigWindow::Height(u32::from(event.height())),
        ),
        (
            (::xcb::x::ConfigWindowMask::BORDER_WIDTH.bits()),
            x::ConfigWindow::BorderWidth(u32::from(event.border_width())),
        ),
        (
            (::xcb::x::ConfigWindowMask::SIBLING.bits()),
            x::ConfigWindow::Sibling(x::Window::new(event.sibling().resource_id())),
        ),
    ] {
        if u32::from(configure_selectors(event).1) & bit != 0 {
            values.push(value);
        }
    }
    if u32::from(configure_selectors(event).1) & (::xcb::x::ConfigWindowMask::STACK_MODE.bits())
        != 0
    {
        if let Some(mode) = checked_stack_mode(configure_selectors(event).0) {
            values.push(x::ConfigWindow::StackMode(mode));
        }
    }
    values
}

unsafe fn handle_configure_request(event: &x::ConfigureRequestEvent) {
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    ::log::debug!("../src/handlers.c:handle_configure_request:304 - window 0x{:08x} wants to be at {}x{} with {}x{}\n", { (event.window().resource_id()) }, { (event.x() as ::core::ffi::c_int) }, { (event.y() as ::core::ffi::c_int) }, { (event.width() as ::core::ffi::c_int) }, { (event.height() as ::core::ffi::c_int) });
    con = con_by_window_id(event.window().resource_id());
    if con.is_null() {
        ::log::debug!("../src/handlers.c:handle_configure_request:309 - Configure request for unmanaged window, can do that.\n");
        let (mask, values) = configure_values(event);
        if mask & (::xcb::x::ConfigWindowMask::STACK_MODE.bits()) as u16 != 0
            && checked_stack_mode(configure_selectors(event).0).is_none()
        {
            // Keep the native asynchronous BadValue/error-queue behavior for
            // unknown selectors without ever constructing an invalid Rust enum.
            xcb_interop::transport::configure_window_with_open_stack_mode(
                &connection(),
                x::Window::new(event.window().resource_id()),
                mask,
                &values,
            );
        } else {
            connection().send_request(&x::ConfigureWindow {
                window: x::Window::new(event.window().resource_id()),
                value_list: &configure_window_values(event),
            });
        }
        let _ = connection().flush();
        return;
    }
    ::log::debug!("../src/handlers.c:handle_configure_request:336 - Configure request!\n");
    let workspace: *mut Con = con_get_workspace(con);
    if !workspace.is_null()
        && strcmp((*workspace).name_ptr(), c"__i3_scratch".as_ptr()) == 0 as ::core::ffi::c_int
    {
        ::log::debug!("../src/handlers.c:handle_configure_request:340 - This is a scratchpad container, ignoring ConfigureRequest\n");
    } else {
        fullscreen = con_get_fullscreen_covering_ws(workspace);
        if fullscreen != con
            && con_is_floating(con) as ::core::ffi::c_int != 0
            && con_is_leaf(con) as ::core::ffi::c_int != 0
        {
            let bsr: Rect = con_border_style_rect(con);
            let floatingcon: *mut Con = parent_ptr(con);
            let mut newrect: Rect = (*floatingcon).rect as Rect;
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::X.bits()) as ::core::ffi::c_int
                != 0
            {
                newrect.x = (event.x() as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.x));
                ::log::debug!("../src/handlers.c:handle_configure_request:354 - proposed x = {}, new x is {}\n", { (event.x() as ::core::ffi::c_int) }, (newrect.x) as i32);
            }
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::Y.bits()) as ::core::ffi::c_int
                != 0
            {
                newrect.y = (event.y() as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.y));
                ::log::debug!("../src/handlers.c:handle_configure_request:358 - proposed y = {}, new y is {}\n", { (event.y() as ::core::ffi::c_int) }, (newrect.y) as i32);
            }
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::WIDTH.bits()) as ::core::ffi::c_int
                != 0
            {
                newrect.width = (event.width() as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.width));
                newrect.width = newrect
                    .width
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
                ::log::debug!("../src/handlers.c:handle_configure_request:364 - proposed width = {}, new width is {} (x11 border {})\n", { (event.width() as ::core::ffi::c_int) }, (newrect.width) as i32, ((*con).border_width) as i32);
            }
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::HEIGHT.bits()) as ::core::ffi::c_int
                != 0
            {
                newrect.height = (event.height() as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.height));
                newrect.height = newrect
                    .height
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
                ::log::debug!("../src/handlers.c:handle_configure_request:370 - proposed height = {}, new height is {} (x11 border {})\n", { (event.height() as ::core::ffi::c_int) }, (newrect.height) as i32, ((*con).border_width) as i32);
            }
            ::log::debug!("../src/handlers.c:handle_configure_request:373 - Container is a floating leaf node, will do that.\n");
            floating_reposition(floatingcon, newrect);
            return;
        }
        if !parent_ptr(con).is_null() && ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
            ::log::debug!("../src/handlers.c:handle_configure_request:380 - Reconfiguring dock window (con = {}).\n", i3_common::logging::pointer(con));
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::HEIGHT.bits()) as ::core::ffi::c_int
                != 0
            {
                ::log::debug!("../src/handlers.c:handle_configure_request:382 - Dock client wants to change height to {}, we can do that.\n", { (event.height() as ::core::ffi::c_int) });
                (*con).geometry.height = event.height() as u32;
                tree_render();
            }
            if configure_selectors(event).1 as ::core::ffi::c_int
                & (::xcb::x::ConfigWindowMask::X.bits()) as ::core::ffi::c_int
                != 0
                || configure_selectors(event).1 as ::core::ffi::c_int
                    & (::xcb::x::ConfigWindowMask::Y.bits()) as ::core::ffi::c_int
                    != 0
            {
                let x: i16 = (if configure_selectors(event).1 as ::core::ffi::c_int
                    & (::xcb::x::ConfigWindowMask::X.bits()) as ::core::ffi::c_int
                    != 0
                {
                    event.x() as ::core::ffi::c_int
                } else {
                    (*con).geometry.x as i16 as ::core::ffi::c_int
                }) as i16;
                let y: i16 = (if configure_selectors(event).1 as ::core::ffi::c_int
                    & (::xcb::x::ConfigWindowMask::Y.bits()) as ::core::ffi::c_int
                    != 0
                {
                    event.y() as ::core::ffi::c_int
                } else {
                    (*con).geometry.y as i16 as ::core::ffi::c_int
                }) as i16;
                let current_output: *mut Con = con_get_output(con);
                let target: *mut Output =
                    get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
                if !target.is_null() && current_output != (*target).con {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_pointer_1 = output_primary_name(target);
                        let log_value_0 = i3_common::logging::string_bytes(
                            if log_pointer_1.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_1))
                            },
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                            ::log::debug!("../src/handlers.c:handle_configure_request:395 - Dock client is requested to be moved to output {}, moving it there.\n", log_value_0_text);
                        } else {
                            i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_configure_request:395 - Dock client is requested to be moved to output ".as_bytes(), &log_value_0, ", moving it there.\n".as_bytes() ]);
                        }
                    };
                    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
                    let nc: *mut Con = con_for_window(
                        (*target).con,
                        (*con).window as *mut i3Window,
                        &raw mut match_0,
                    );
                    ::log::debug!("../src/handlers.c:handle_configure_request:398 - Dock client will be moved to container {}.\n", i3_common::logging::pointer(nc));
                    con_detach(con);
                    con_attach(con, nc, false);
                    tree_render();
                } else {
                    ::log::debug!("../src/handlers.c:handle_configure_request:404 - Dock client will not be moved, we only support moving it to another output.\n");
                }
            }
        } else if configure_selectors(event).1 as ::core::ffi::c_int
            & (::xcb::x::ConfigWindowMask::STACK_MODE.bits()) as ::core::ffi::c_int
            != 0
        {
            ::log::debug!("../src/handlers.c:handle_configure_request:411 - window 0x{:08x} wants to be stacked {}\n", { (event.window().resource_id()) }, { (configure_selectors(event).0 as ::core::ffi::c_int) });
            if configure_selectors(event).0 as ::core::ffi::c_int
                != (::xcb::x::StackMode::Above as u32) as ::core::ffi::c_int
            {
                ::log::debug!("../src/handlers.c:handle_configure_request:416 - stack_mode != XCB_STACK_MODE_ABOVE, ignoring ConfigureRequest\n");
            } else if !fullscreen.is_null() || !con_is_leaf(con) {
                ::log::debug!("../src/handlers.c:handle_configure_request:421 - fullscreen or not a leaf, ignoring ConfigureRequest\n");
            } else if workspace.is_null() {
                ::log::debug!("../src/handlers.c:handle_configure_request:426 - Window is not being managed, ignoring ConfigureRequest\n");
            } else if config.focus_on_window_activation == FocusActivation::Focus
                || config.focus_on_window_activation == FocusActivation::Smart
                    && workspace_is_visible(workspace) as ::core::ffi::c_int != 0
            {
                ::log::debug!(
                    "../src/handlers.c:handle_configure_request:431 - Focusing con = {}\n",
                    i3_common::logging::pointer(con)
                );
                workspace_show(workspace);
                con_activate_unblock(con);
                tree_render();
            } else if config.focus_on_window_activation == FocusActivation::Urgent
                || config.focus_on_window_activation == FocusActivation::Smart
                    && !workspace_is_visible(workspace)
            {
                ::log::debug!(
                    "../src/handlers.c:handle_configure_request:436 - Marking con = {} urgent\n",
                    i3_common::logging::pointer(con)
                );
                con_set_urgency(con, true);
                con = remanage_window(con);
                tree_render();
            } else {
                ::log::debug!("../src/handlers.c:handle_configure_request:441 - Ignoring request for con = {}.\n", i3_common::logging::pointer(con));
            }
        }
    }
    fake_absolute_configure_notify(con);
}
unsafe fn handle_screen_change() {
    ::log::debug!("../src/handlers.c:handle_screen_change:455 - RandR screen change\n");
    let cookie = connection().send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(x::Window::new(root)),
    });
    let Some(reply) = connection().wait_for_reply(cookie).ok() else {
        ::log::error!("ERROR: Could not get geometry of the root window, exiting\n");
        exit(EXIT_FAILURE);
    };
    ::log::debug!(
        "../src/handlers.c:handle_screen_change:465 - root geometry reply: ({}, {}) {} x {}\n",
        { (reply.x() as ::core::ffi::c_int) },
        { (reply.y() as ::core::ffi::c_int) },
        { (reply.width() as ::core::ffi::c_int) },
        { (reply.height() as ::core::ffi::c_int) }
    );
    (*croot).rect.width = reply.width() as u32;
    (*croot).rect.height = reply.height() as u32;
    randr_query_outputs();
    scratchpad_fix_resolution();
    ipc_send_event(
        c"output".as_ptr(),
        I3_IPC_EVENT_OUTPUT as u32,
        c"{\"change\":\"unspecified\"}".as_ptr(),
    );
}
unsafe fn handle_unmap_notify_event(event: &x::UnmapNotifyEvent, full_sequence: u32) {
    handle_window_unmap(
        event.window().resource_id(),
        event.event().resource_id(),
        event.sequence(),
        full_sequence,
    );
}

// DestroyNotify shares this semantic operation without manufacturing an Unmap
// packet, response opcode or appended native full_sequence field.
unsafe fn handle_window_unmap(window: u32, event_window: u32, sequence: u16, full_sequence: u32) {
    ::log::debug!("../src/handlers.c:handle_unmap_notify_event:483 - UnmapNotify for 0x{:08x} (received from 0x{:08x}), serial {}\n", { (window) }, { (event_window) }, { (sequence as ::core::ffi::c_int) });
    let cookie;
    let mut con: *mut Con = con_by_window_id(window);
    if con.is_null() {
        con = con_by_frame_id(window);
        if con.is_null() {
            ::log::info!("Not a managed window, ignoring UnmapNotify event\n");
            return;
        }
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        }
        cookie = connection().send_request(&x::GetInputFocus {});
        ::log::debug!("../src/handlers.c:handle_unmap_notify_event:500 - ignore_unmap = {} for frame of container {}\n", { ((*con).ignore_unmap as ::core::ffi::c_int) }, i3_common::logging::pointer(con));
    } else {
        cookie = connection().send_request(&x::GetInputFocus {});
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            ::log::debug!(
                "../src/handlers.c:handle_unmap_notify_event:508 - ignore_unmap = {}, dec\n",
                { ((*con).ignore_unmap as ::core::ffi::c_int) }
            );
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        } else {
            connection().send_request(&x::DeleteProperty {
                window: x::Window::new(window),
                property: x::Atom::new(A__NET_WM_DESKTOP),
            });
            connection().send_request(&x::DeleteProperty {
                window: x::Window::new(window),
                property: x::Atom::new(A__NET_WM_STATE),
            });
            tree_close_internal(con, KillWindow::DontKill, false);
            tree_render();
        }
    }
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        (<::xcb::x::EnterNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32),
    );
    let _ = connection().wait_for_reply(cookie);
}
unsafe fn handle_destroy_notify_event(event: &x::DestroyNotifyEvent, full_sequence: u32) {
    ::log::debug!("../src/handlers.c:handle_destroy_notify_event:555 - destroy notify for 0x{:08x}, 0x{:08x}\n", { (event.event().resource_id()) }, { (event.window().resource_id()) });
    handle_window_unmap(
        event.window().resource_id(),
        event.event().resource_id(),
        event.sequence(),
        full_sequence,
    );
}
/// Copy only the native first-NUL text view before property/remanage callbacks
/// can replace/free the original Window or its string.
unsafe fn window_name_snapshot(window: *mut i3Window) -> Option<Vec<u8>> {
    (*window)
        .name
        .as_ref()
        .and_then(|name| name.cstr_or_report().map(|text| text.to_bytes().to_vec()))
}
unsafe fn handle_windowname_change(mut con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    let old_name = window_name_snapshot((*con).window);
    window_update_name((*con).window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if old_name != window_name_snapshot((*con).window) {
        ipc_send_window_event(c"title".as_ptr(), con);
    }
    true
}
unsafe fn handle_windowname_change_legacy(
    mut con: *mut Con,
    prop: Option<&x::GetPropertyReply>,
) -> bool {
    let old_name = window_name_snapshot((*con).window);
    window_update_name_legacy((*con).window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if old_name != window_name_snapshot((*con).window) {
        ipc_send_window_event(c"title".as_ptr(), con);
    }
    true
}
unsafe fn handle_windowrole_change(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_role((*con).window as *mut i3Window, prop);
    remanage_window(con);
    true
}
unsafe fn handle_expose_event(event: &x::ExposeEvent) {
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    ::log::debug!(
        "../src/handlers.c:handle_expose_event:642 - window = {:08x}\n",
        { (event.window().resource_id()) }
    );
    parent = con_by_frame_id(event.window().resource_id());
    if parent.is_null() {
        ::log::info!("expose event for unknown window, ignoring\n");
        return;
    }
    draw_util_copy_surface(
        &raw mut (*parent).frame_buffer,
        &raw mut (*parent).frame,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*parent).rect.width as ::core::ffi::c_double,
        (*parent).rect.height as ::core::ffi::c_double,
    );
    let _ = connection().flush();
}
pub const _NET_MOVERESIZE_WINDOW_X: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_Y: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_WIDTH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
unsafe fn handle_net_wm_state_change(mut con: *mut Con, change: u32, atom: u32) {
    if atom == 0_u32 {
        return;
    }
    let debug_change: *const ::core::ffi::c_char = if change == _NET_WM_STATE_REMOVE as u32 {
        c"remove".as_ptr()
    } else if change == _NET_WM_STATE_ADD as u32 {
        c"add".as_ptr()
    } else {
        c"toggle".as_ptr()
    };
    if atom == A__NET_WM_STATE_FULLSCREEN {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = debug_change;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_net_wm_state_change:682 - Received a client message to {} _NET_WM_STATE_FULLSCREEN.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_net_wm_state_change:682 - Received a client message to ".as_bytes(), &log_value_0, " _NET_WM_STATE_FULLSCREEN.\n".as_bytes() ]);
            }
        };
        if change == _NET_WM_STATE_TOGGLE as u32
            || ((*con).fullscreen_mode) != FullscreenMode::None
                && change == _NET_WM_STATE_REMOVE as u32
            || ((*con).fullscreen_mode) == FullscreenMode::None
                && change == _NET_WM_STATE_ADD as u32
        {
            ::log::debug!(
                "../src/handlers.c:handle_net_wm_state_change:688 - toggling fullscreen\n"
            );
            con_toggle_fullscreen(con, FullscreenMode::Output);
        }
    } else if atom == A__NET_WM_STATE_DEMANDS_ATTENTION {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = debug_change;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_net_wm_state_change:692 - Received a client message to {} _NET_WM_STATE_DEMANDS_ATTENTION.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_net_wm_state_change:692 - Received a client message to ".as_bytes(), &log_value_0, " _NET_WM_STATE_DEMANDS_ATTENTION.\n".as_bytes() ]);
            }
        };
        if change == _NET_WM_STATE_ADD as u32 {
            con_set_urgency(con, true);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_REMOVE as u32 {
            con_set_urgency(con, false);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_TOGGLE as u32 {
            con_set_urgency(con, !(*con).urgent);
            con = remanage_window(con);
        }
    } else if atom == A__NET_WM_STATE_STICKY {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = debug_change;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_net_wm_state_change:706 - Received a client message to {} _NET_WM_STATE_STICKY.\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_net_wm_state_change:706 - Received a client message to ".as_bytes(), &log_value_0, " _NET_WM_STATE_STICKY.\n".as_bytes() ]);
            }
        };
        if change == _NET_WM_STATE_ADD as u32 {
            (*con).sticky = true;
        } else if change == _NET_WM_STATE_REMOVE as u32 {
            (*con).sticky = false;
        } else if change == _NET_WM_STATE_TOGGLE as u32 {
            (*con).sticky = !(*con).sticky;
        }
        ::log::debug!("../src/handlers.c:handle_net_wm_state_change:716 - New sticky status for con = {} is {}.\n", i3_common::logging::pointer(con), { ((*con).sticky as ::core::ffi::c_int) });
        ewmh_update_sticky((*(*con).window).id, (*con).sticky);
        output_push_sticky_windows(focused);
        ewmh_update_wm_desktop();
    } else {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = debug_change;
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            let log_value_1 = (atom);
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_net_wm_state_change:721 - Unknown atom in ClientMessage to {} type {}\n", log_value_0_text, log_value_1);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_net_wm_state_change:721 - Unknown atom in ClientMessage to ".as_bytes(), &log_value_0, format!(" type {}\n", log_value_1).as_bytes() ]);
            }
        };
        return;
    }
    tree_render();
}
unsafe fn handle_client_message(event: &x::ClientMessageEvent) {
    if sn_xcb_display_process_event(sndisplay, event.as_raw().cast()) != 0 {
        return;
    }
    let words = xcb_interop::events::client_message_words(event);
    ::log::info!("ClientMessage for window 0x{:08x}\n", {
        (event.window().resource_id())
    });
    if event.r#type().resource_id() == A__NET_WM_STATE {
        if event.format() as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            ::log::debug!("../src/handlers.c:handle_client_message:742 - Unknown format {} in ClientMessage\n", { (event.format() as ::core::ffi::c_int) });
            return;
        }
        let con: *mut Con = con_by_window_id(event.window().resource_id());
        if con.is_null() {
            ::log::debug!("../src/handlers.c:handle_client_message:748 - Could not get window for client message\n");
            return;
        }
        let mut i: usize = 0_usize;
        while i
            < (::core::mem::size_of::<[u32; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<u32>() as usize)
                .wrapping_sub(1_usize)
        {
            handle_net_wm_state_change(
                con,
                words[0 as ::core::ffi::c_int as usize],
                words[i.wrapping_add(1_usize)],
            );
            i = i.wrapping_add(1);
        }
    } else if event.r#type().resource_id() == A__NET_ACTIVE_WINDOW {
        if event.format() as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            return;
        }
        ::log::debug!("../src/handlers.c:handle_client_message:760 - _NET_ACTIVE_WINDOW: Window 0x{:08x} should be activated\n", { (event.window().resource_id()) });
        let mut con_0: *mut Con = con_by_window_id(event.window().resource_id());
        if con_0.is_null() {
            ::log::debug!("../src/handlers.c:handle_client_message:764 - Could not get window for client message\n");
            return;
        }
        let ws: *mut Con = con_get_workspace(con_0);
        if ws.is_null() {
            ::log::debug!("../src/handlers.c:handle_client_message:770 - Window is not being managed, ignoring _NET_ACTIVE_WINDOW\n");
            return;
        }
        if con_is_internal(ws) as ::core::ffi::c_int != 0
            && ws != workspace_get(c"__i3_scratch".as_ptr())
        {
            ::log::debug!("../src/handlers.c:handle_client_message:775 - Workspace is internal but not scratchpad, ignoring _NET_ACTIVE_WINDOW\n");
            return;
        }
        if words[0 as ::core::ffi::c_int as usize] == 2_u32 {
            ::log::debug!("../src/handlers.c:handle_client_message:783 - This request came from a pager. Focusing con = {}\n", i3_common::logging::pointer(con_0));
            if con_is_internal(ws) {
                scratchpad_show(con_0);
            } else {
                workspace_show(ws);
                focused_id = 0_u32;
                con_activate_unblock(con_0);
            }
        } else {
            if con_is_internal(ws) {
                ::log::debug!("../src/handlers.c:handle_client_message:796 - Ignoring request to make con = {} active because it's on an internal workspace.\n", i3_common::logging::pointer(con_0));
                return;
            }
            if config.focus_on_window_activation == FocusActivation::Focus
                || config.focus_on_window_activation == FocusActivation::Smart
                    && workspace_is_visible(ws) as ::core::ffi::c_int != 0
            {
                ::log::debug!(
                    "../src/handlers.c:handle_client_message:801 - Focusing con = {}\n",
                    i3_common::logging::pointer(con_0)
                );
                con_activate_unblock(con_0);
            } else if config.focus_on_window_activation == FocusActivation::Urgent
                || config.focus_on_window_activation == FocusActivation::Smart
                    && !workspace_is_visible(ws)
            {
                ::log::debug!(
                    "../src/handlers.c:handle_client_message:804 - Marking con = {} urgent\n",
                    i3_common::logging::pointer(con_0)
                );
                con_set_urgency(con_0, true);
                con_0 = remanage_window(con_0);
            } else {
                ::log::debug!("../src/handlers.c:handle_client_message:808 - Ignoring request for con = {}.\n", i3_common::logging::pointer(con_0));
            }
        }
        tree_render();
    } else if event.r#type().resource_id() == A_I3_SYNC {
        let window: u32 = words[0 as ::core::ffi::c_int as usize];
        let rnd: u32 = words[1 as ::core::ffi::c_int as usize];
        sync_respond(window, rnd);
    } else if event.r#type().resource_id() == A__NET_REQUEST_FRAME_EXTENTS {
        ::log::debug!("../src/handlers.c:handle_client_message:830 - _NET_REQUEST_FRAME_EXTENTS for window 0x{:08x}\n", { (event.window().resource_id()) });
        let mut r: Rect = Rect {
            x: config.default_border_width as u32,
            y: config.default_border_width as u32,
            width: render_deco_height() as u32,
            height: config.default_border_width as u32,
        };
        connection().send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: x::Window::new(event.window().resource_id()),
            property: x::Atom::new(A__NET_FRAME_EXTENTS),
            r#type: x::ATOM_CARDINAL,
            data: &[r.x, r.y, r.width, r.height],
        });
        let _ = connection().flush();
    } else if event.r#type().resource_id() == A_WM_CHANGE_STATE {
        if words[0 as ::core::ffi::c_int as usize]
            == (crate::domains::IcccmState::Iconic as u32) as ::core::ffi::c_int as u32
        {
            ::log::debug!("../src/handlers.c:handle_client_message:852 - Client has requested iconic state, rejecting. (window = {:08x})\n", { (event.window().resource_id()) });
            connection().send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: x::Window::new(event.window().resource_id()),
                property: x::Atom::new(A_WM_STATE),
                r#type: x::Atom::new(A_WM_STATE),
                data: &[(crate::domains::IcccmState::Normal as u32), 0],
            });
        } else {
            ::log::debug!("../src/handlers.c:handle_client_message:857 - Not handling WM_CHANGE_STATE request. (window = {:08x}, state = {})\n", { (event.window().resource_id()) }, (words[0 as ::core::ffi::c_int as usize]) as i32);
        }
    } else if event.r#type().resource_id() == A__NET_CURRENT_DESKTOP {
        ::log::debug!("../src/handlers.c:handle_client_message:865 - Request to change current desktop to index {}\n", (words[0 as ::core::ffi::c_int as usize]) as i32);
        let ws_0: *mut Con = ewmh_get_workspace_by_index(words[0 as ::core::ffi::c_int as usize]);
        if ws_0.is_null() {
            ::log::error!(
                "ERROR: Could not determine workspace for this index, ignoring request.\n"
            );
            return;
        }
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = (*ws_0).name_ptr();
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_client_message:872 - Handling request to focus workspace {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_client_message:872 - Handling request to focus workspace ".as_bytes(), &log_value_0, "\n".as_bytes() ]);
            }
        };
        workspace_show(ws_0);
        tree_render();
    } else if event.r#type().resource_id() == A__NET_WM_DESKTOP {
        let index: u32 = words[0 as ::core::ffi::c_int as usize];
        ::log::debug!("../src/handlers.c:handle_client_message:877 - Request to move window {} to EWMH desktop index {}\n", (event.window().resource_id()) as i32, (index) as i32);
        let con_1: *mut Con = con_by_window_id(event.window().resource_id());
        if con_1.is_null() {
            ::log::debug!("../src/handlers.c:handle_client_message:881 - Couldn't find con for window {}, ignoring the request.\n", (event.window().resource_id()) as i32);
            return;
        }
        if index == NET_WM_DESKTOP_ALL {
            ::log::debug!("../src/handlers.c:handle_client_message:888 - The window was requested to be visible on all workspaces, making it sticky and floating.\n");
            if floating_enable(con_1, false) {
                (*con_1).floating = FloatingState::AutoOn;
                (*con_1).sticky = true;
                ewmh_update_sticky((*(*con_1).window).id, true);
                output_push_sticky_windows(focused);
                run_assignments((*con_1).window as *mut i3Window);
            }
        } else {
            let ws_1: *mut Con = ewmh_get_workspace_by_index(index);
            if ws_1.is_null() {
                ::log::error!(
                    "ERROR: Could not determine workspace for this index, ignoring request.\n"
                );
                return;
            }
            con_move_to_workspace(con_1, ws_1, true, false, false);
        }
        tree_render();
        ewmh_update_wm_desktop();
    } else if event.r#type().resource_id() == A__NET_CLOSE_WINDOW {
        let con_2: *mut Con = con_by_window_id(event.window().resource_id());
        if !con_2.is_null() {
            ::log::debug!("../src/handlers.c:handle_client_message:918 - Handling _NET_CLOSE_WINDOW request (con = {})\n", i3_common::logging::pointer(con_2));
            if words[0 as ::core::ffi::c_int as usize] != 0 {
                last_timestamp = words[0 as ::core::ffi::c_int as usize] as u32;
            }
            tree_close_internal(con_2, KillWindow::Window, false);
            tree_render();
        } else {
            ::log::debug!("../src/handlers.c:handle_client_message:927 - Couldn't find con for _NET_CLOSE_WINDOW request. (window = {:08x})\n", { (event.window().resource_id()) });
        }
    } else if event.r#type().resource_id() == A__NET_WM_MOVERESIZE {
        let con_3: *mut Con = con_by_window_id(event.window().resource_id());
        if con_3.is_null() || !con_is_floating(con_3) {
            ::log::debug!("../src/handlers.c:handle_client_message:936 - Couldn't find con for _NET_WM_MOVERESIZE request, or con not floating (window = {:08x})\n", { (event.window().resource_id()) });
            return;
        }
        ::log::debug!("../src/handlers.c:handle_client_message:939 - Handling _NET_WM_MOVERESIZE request (con = {})\n", i3_common::logging::pointer(con_3));
        let direction: u32 = words[2 as ::core::ffi::c_int as usize];
        let x_root: u32 = words[0 as ::core::ffi::c_int as usize];
        let y_root: u32 = words[1 as ::core::ffi::c_int as usize];
        // This local initiating press is never queued; callbacks borrow only
        // its copied button/coordinate fields throughout the synchronous drag.
        let fake = x::ButtonPressEvent::new(
            words[3] as u8,
            0,
            x::WINDOW_NONE,
            x::WINDOW_NONE,
            x::WINDOW_NONE,
            x_root as i16,
            y_root as i16,
            x_root.wrapping_sub((*con_3).rect.x) as i16,
            y_root.wrapping_sub((*con_3).rect.y) as i16,
            x::KeyButMask::empty(),
            false,
        );
        match direction {
            8 => {
                floating_drag_window(parent_ptr(con_3), &fake, false);
            }
            0 => {
                floating_resize_window(parent_ptr(con_3), false, &fake);
            }
            _ => {
                ::log::debug!("../src/handlers.c:handle_client_message:958 - _NET_WM_MOVERESIZE direction {} not implemented\n", (direction) as i32);
            }
        }
    } else if event.r#type().resource_id() == A__NET_MOVERESIZE_WINDOW {
        ::log::debug!("../src/handlers.c:handle_client_message:962 - Received _NET_MOVE_RESIZE_WINDOW. Handling by faking a configure request.\n");
        let mut value_mask = x::ConfigWindowMask::empty();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        if words[0] & _NET_MOVERESIZE_WINDOW_X as u32 != 0 {
            value_mask |= x::ConfigWindowMask::X;
            x = words[1] as i16;
        }
        if words[0] & _NET_MOVERESIZE_WINDOW_Y as u32 != 0 {
            value_mask |= x::ConfigWindowMask::Y;
            y = words[2] as i16;
        }
        if words[0] & _NET_MOVERESIZE_WINDOW_WIDTH as u32 != 0 {
            value_mask |= x::ConfigWindowMask::WIDTH;
            width = words[3] as u16;
        }
        if words[0] & _NET_MOVERESIZE_WINDOW_HEIGHT as u32 != 0 {
            value_mask |= x::ConfigWindowMask::HEIGHT;
            height = words[4] as u16;
        }
        let generated_event = x::ConfigureRequestEvent::new(
            x::StackMode::Above,
            x::Window::none(),
            event.window(),
            x::Window::none(),
            x,
            y,
            width,
            height,
            0,
            value_mask,
        );
        // This synthetic body is passed directly to its typed leaf. It has no
        // appended native full_sequence and must not enter queue dispatch.
        handle_configure_request(&generated_event);
    } else {
        ::log::debug!("../src/handlers.c:handle_client_message:991 - Skipping client message for unhandled type {}\n", (event.r#type().resource_id()) as i32);
    };
}
unsafe fn handle_window_type(con: *mut Con, reply: Option<&x::GetPropertyReply>) -> bool {
    window_update_type((*con).window as *mut i3Window, reply);
    true
}
unsafe fn handle_normal_hints(con: *mut Con, reply: Option<&x::GetPropertyReply>) -> bool {
    let changed: bool = window_update_normal_hints((*con).window as *mut i3Window, reply, None);
    if changed {
        let floating: *mut Con = con_inside_floating(con);
        if !floating.is_null() {
            floating_check_size(con, false);
            tree_render();
        }
    }
    true
}
unsafe fn handle_hints(con: *mut Con, reply: Option<&x::GetPropertyReply>) -> bool {
    let mut urgency_hint: bool = false;
    window_update_hints((*con).window as *mut i3Window, reply, &raw mut urgency_hint);
    con_set_urgency(con, urgency_hint);
    remanage_window(con);
    tree_render();
    true
}
unsafe fn handle_transient_for(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_transient_for((*con).window as *mut i3Window, prop);
    true
}
unsafe fn handle_clientleader_change(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_leader((*con).window as *mut i3Window, prop);
    true
}
unsafe fn handle_focus_in(event: &x::FocusInEvent) {
    let (event_mode, event_detail) = focus_selectors(event);
    ::log::debug!(
        "../src/handlers.c:handle_focus_in:1064 - focus change in, for window 0x{:08x}\n",
        { (event.event().resource_id()) }
    );
    if event.event().resource_id() == root {
        ::log::debug!("../src/handlers.c:handle_focus_in:1067 - Received focus in for root window, refocusing the focused window.\n");
        con_focus(focused);
        focused_id = 0_u32;
        x_push_changes(croot);
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_window_id(event.event().resource_id());
    if con.is_null() || (*con).window.is_null() {
        return;
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(con);
        let log_pointer_2 = (*con).name_ptr();
        let log_value_1 = i3_common::logging::string_bytes(
            if log_pointer_2.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_2))
            },
            None,
        )
        .to_vec();
        if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
            ::log::debug!(
                "../src/handlers.c:handle_focus_in:1077 - That is con {} / {}\n",
                log_value_0,
                log_value_1_text
            );
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Debug,
                &[
                    format!(
                        "../src/handlers.c:handle_focus_in:1077 - That is con {} / ",
                        log_value_0
                    )
                    .as_bytes(),
                    &log_value_1,
                    "\n".as_bytes(),
                ],
            );
        }
    };
    if event_mode as ::core::ffi::c_int == (::xcb::x::NotifyMode::Grab as u32) as ::core::ffi::c_int
        || event_mode as ::core::ffi::c_int
            == (::xcb::x::NotifyMode::Ungrab as u32) as ::core::ffi::c_int
    {
        ::log::debug!(
            "../src/handlers.c:handle_focus_in:1081 - FocusIn event for grab/ungrab, ignoring\n"
        );
        return;
    }
    if event_detail as ::core::ffi::c_int
        == (::xcb::x::NotifyDetail::Pointer as u32) as ::core::ffi::c_int
    {
        ::log::debug!("../src/handlers.c:handle_focus_in:1086 - notify detail is pointer, ignoring this event\n");
        return;
    }
    if focused_id == event.event().resource_id() && con_inside_floating(con).is_null() {
        ::log::debug!("../src/handlers.c:handle_focus_in:1093 - focus matches the currently focused window, not doing anything\n");
        return;
    }
    if ((*parent_ptr(con)).type_0) == ContainerType::DockArea {
        ::log::debug!(
            "../src/handlers.c:handle_focus_in:1099 - This is a dock client, not focusing.\n"
        );
        return;
    }
    ::log::debug!("../src/handlers.c:handle_focus_in:1103 - focus is different / refocusing floating window: updating decorations\n");
    con_activate_unblock(con);
    focused_id = event.event().resource_id();
    tree_render();
}
unsafe fn handle_focus_out(event: &x::FocusInEvent) {
    let (event_mode, event_detail) = focus_selectors(event);
    let con: *mut Con = con_by_window_id(event.event().resource_id());
    let mut window_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mode: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut detail: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let owned_window_name = (!con.is_null()).then(|| (*con).name.clone()).flatten();
    if !con.is_null() {
        window_name = owned_window_name
            .as_deref()
            .map_or(std::ptr::null(), std::ffi::CStr::as_ptr);
        if window_name.is_null() {
            window_name = c"<unnamed con>".as_ptr();
        }
    } else if event.event().resource_id() == root {
        window_name = c"<the root window>".as_ptr();
    } else {
        window_name = c"<unknown window>".as_ptr();
    }
    match event_mode as ::core::ffi::c_int {
        0 => {
            mode = c"Normal".as_ptr();
        }
        1 => {
            mode = c"Grab".as_ptr();
        }
        2 => {
            mode = c"Ungrab".as_ptr();
        }
        3 => {
            mode = c"WhileGrabbed".as_ptr();
        }
        _ => {
            mode = c"<unknown>".as_ptr();
        }
    }
    match event_detail as ::core::ffi::c_int {
        0 => {
            detail = c"Ancestor".as_ptr();
        }
        1 => {
            detail = c"Virtual".as_ptr();
        }
        2 => {
            detail = c"Inferior".as_ptr();
        }
        3 => {
            detail = c"Nonlinear".as_ptr();
        }
        4 => {
            detail = c"NonlinearVirtual".as_ptr();
        }
        5 => {
            detail = c"Pointer".as_ptr();
        }
        6 => {
            detail = c"PointerRoot".as_ptr();
        }
        7 => {
            detail = c"NONE".as_ptr();
        }
        _ => {
            detail = c"unknown".as_ptr();
        }
    }
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = (event.event().resource_id());
        let log_value_1 = i3_common::logging::pointer(con);
        let log_pointer_3 = window_name;
        let log_value_2 = i3_common::logging::string_bytes(
            if log_pointer_3.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_3))
            },
            None,
        )
        .to_vec();
        let log_pointer_4 = detail;
        let log_value_3 = i3_common::logging::string_bytes(
            if log_pointer_4.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_4))
            },
            None,
        )
        .to_vec();
        let log_pointer_5 = mode;
        let log_value_4 = i3_common::logging::string_bytes(
            if log_pointer_5.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_5))
            },
            None,
        )
        .to_vec();
        if let (Ok(log_value_2_text), Ok(log_value_3_text), Ok(log_value_4_text)) = (
            std::str::from_utf8(&log_value_2),
            std::str::from_utf8(&log_value_3),
            std::str::from_utf8(&log_value_4),
        ) {
            ::log::debug!("../src/handlers.c:handle_focus_out:1179 - focus change out: window 0x{:08x} (con {}, {}) lost focus with detail={}, mode={}\n", log_value_0, log_value_1, log_value_2_text, log_value_3_text, log_value_4_text);
        } else {
            i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/handlers.c:handle_focus_out:1179 - focus change out: window 0x{:08x} (con {}, ", log_value_0, log_value_1).as_bytes(), &log_value_2, ") lost focus with detail=".as_bytes(), &log_value_3, ", mode=".as_bytes(), &log_value_4, "\n".as_bytes() ]);
        }
    };
}
unsafe fn handle_configure_notify(event: &x::ConfigureNotifyEvent) {
    if event.event().resource_id() != root {
        ::log::debug!("../src/handlers.c:handle_configure_notify:1189 - ConfigureNotify for non-root window 0x{:08x}, ignoring\n", { (event.event().resource_id()) });
        return;
    }
    ::log::debug!("../src/handlers.c:handle_configure_notify:1192 - ConfigureNotify for root window 0x{:08x}\n", { (event.event().resource_id()) });
    if force_xinerama {
        return;
    }
    randr_query_outputs();
    ipc_send_event(
        c"output".as_ptr(),
        I3_IPC_EVENT_OUTPUT as u32,
        c"{\"change\":\"unspecified\"}".as_ptr(),
    );
}
unsafe fn handle_selection_clear(event: &x::SelectionClearEvent) {
    if event.selection().resource_id() != wm_sn {
        ::log::debug!("../src/handlers.c:handle_selection_clear:1208 - SelectionClear for unknown selection {}, ignoring\n", (event.selection().resource_id()) as i32);
        return;
    }
    ::log::info!("Lost WM_Sn selection, exiting.\n");
    exit(EXIT_SUCCESS);
}
unsafe fn handle_class_change(mut con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_class((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    true
}
unsafe fn handle_machine_change(mut con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_machine((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    true
}
unsafe fn handle_motif_hints_change(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    let mut motif_border_style: BorderStyle = BorderStyle::None;
    let has_mwm_hints: bool = window_update_motif_hints(
        (*con).window as *mut i3Window,
        prop,
        &raw mut motif_border_style,
    );
    if has_mwm_hints as ::core::ffi::c_int != 0 && motif_border_style != (*con).border_style {
        ::log::debug!("../src/handlers.c:handle_motif_hints_change:1246 - Update border style of con {} to {}\n", i3_common::logging::pointer(con), (motif_border_style as ::core::ffi::c_uint) as i32);
        con_set_border_style(con, motif_border_style, (*con).current_border_width);
        x_push_changes(croot);
    }
    true
}
unsafe fn handle_strut_partial_change(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_strut_partial((*con).window as *mut i3Window, prop);
    if parent_ptr(con).is_null() || ((*parent_ptr(con)).type_0) != ContainerType::DockArea {
        return true;
    }
    let mut search_at: *mut Con = croot;
    let output: *mut Con = con_get_output(con);
    if !output.is_null() {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_pointer_1 = (*output).name_ptr();
            let log_value_0 = i3_common::logging::string_bytes(
                if log_pointer_1.is_null() {
                    None
                } else {
                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                },
                None,
            )
            .to_vec();
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::debug!("../src/handlers.c:handle_strut_partial_change:1270 - Starting search at output {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/handlers.c:handle_strut_partial_change:1270 - Starting search at output ".as_bytes(), &log_value_0, "\n".as_bytes() ]);
            }
        };
        search_at = output;
    }
    if (*(*con).window).reserved.top > 0_u32 && (*(*con).window).reserved.bottom == 0_u32 {
        ::log::debug!("../src/handlers.c:handle_strut_partial_change:1276 - Top dock client\n");
        (*(*con).window).dock = WindowDock::Top;
    } else if (*(*con).window).reserved.top == 0_u32 && (*(*con).window).reserved.bottom > 0_u32 {
        ::log::debug!("../src/handlers.c:handle_strut_partial_change:1279 - Bottom dock client\n");
        (*(*con).window).dock = WindowDock::Bottom;
    } else {
        ::log::debug!("../src/handlers.c:handle_strut_partial_change:1282 - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n");
        if (*con).geometry.y < (*search_at).rect.height.wrapping_div(2_u32) {
            ::log::debug!("../src/handlers.c:handle_strut_partial_change:1285 - geom->y = {} < rect.height / 2 = {}, it is a top dock client\n", ((*con).geometry.y) as i32, ((*search_at).rect.height.wrapping_div(2_u32)) as i32);
            (*(*con).window).dock = WindowDock::Top;
        } else {
            ::log::debug!("../src/handlers.c:handle_strut_partial_change:1289 - geom->y = {} >= rect.height / 2 = {}, it is a bottom dock client\n", ((*con).geometry.y) as i32, ((*search_at).rect.height.wrapping_div(2_u32)) as i32);
            (*(*con).window).dock = WindowDock::Bottom;
        }
    }
    let dockarea: *mut Con = con_for_window(
        search_at,
        (*con).window as *mut i3Window,
        ::core::ptr::null_mut::<*mut Match>(),
    );
    if dockarea != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Con {
    } else {
        __assert_fail(
            c"dockarea != NULL".as_ptr(),
            c"../src/handlers.c".as_ptr(),
            1296 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    con_detach(con);
    con_attach(con, dockarea, true);
    tree_render();
    true
}
unsafe fn handle_windowicon_change(con: *mut Con, prop: Option<&x::GetPropertyReply>) -> bool {
    window_update_icon((*con).window as *mut i3Window, prop);
    x_push_changes(croot);
    true
}
static mut property_handlers: [PropertyHandler; 13] = [PropertyHandler {
    atom: 0,
    long_len: 0,
    cb: None,
}; 13];
pub const NUM_HANDLERS: usize = 13;

pub unsafe fn property_handlers_init() {
    sn_monitor_context_new(
        sndisplay,
        conn_screen,
        Some(
            startup_monitor_event
                as unsafe extern "C" fn(*mut SnMonitorEvent, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
        None,
    );
    property_handlers[0 as ::core::ffi::c_int as usize].atom = A__NET_WM_NAME;
    property_handlers[1 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_HINTS.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[2 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_NAME.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[3 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_NORMAL_HINTS.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[4 as ::core::ffi::c_int as usize].atom = A_WM_CLIENT_LEADER;
    property_handlers[5 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_TRANSIENT_FOR.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[6 as ::core::ffi::c_int as usize].atom = A_WM_WINDOW_ROLE;
    property_handlers[7 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_CLASS.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[8 as ::core::ffi::c_int as usize].atom = A__NET_WM_STRUT_PARTIAL;
    property_handlers[9 as ::core::ffi::c_int as usize].atom = A__NET_WM_WINDOW_TYPE;
    property_handlers[10 as ::core::ffi::c_int as usize].atom =
        (::xcb::x::ATOM_WM_CLIENT_MACHINE.resource_id()) as ::core::ffi::c_int as u32;
    property_handlers[11 as ::core::ffi::c_int as usize].atom = A__MOTIF_WM_HINTS;
    property_handlers[12 as ::core::ffi::c_int as usize].atom = A__NET_WM_ICON;
}
/// The request retains ownership across callbacks, including unwinding. A
/// handler can borrow the property but cannot consume or retain its allocation.
unsafe fn dispatch_property(
    callback: unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
    con: *mut Con,
    reply: Option<x::GetPropertyReply>,
) {
    callback(con, reply.as_ref());
}

unsafe fn property_notify(state: u8, window: u32, atom: u32) {
    // Snapshot metadata; callback dispatch never retains a table borrow.
    let handler = (0..NUM_HANDLERS).find_map(|index| {
        let handler = property_handlers[index];
        (handler.atom == atom).then_some(handler)
    });
    let Some(handler) = handler else {
        return;
    };
    let mut reply = None;
    let con;
    con = con_by_window_id(window);
    if con.is_null() || (*con).window.is_null() {
        ::log::debug!("../src/handlers.c:property_notify:1385 - Received property for atom {} for unknown client\n", (atom) as i32);
        return;
    }
    if state != x::Property::Delete as u8 {
        let result = {
            let connection =
                i3_common::display::connection().expect("main X11 connection is registered");
            let cookie = connection.send_request(&x::GetProperty {
                delete: false,
                window: x::Window::new(window),
                property: x::Atom::new(atom),
                r#type: x::ATOM_ANY,
                long_offset: 0,
                long_length: handler.long_len,
            });
            connection.wait_for_reply(cookie)
        };
        match result {
            Ok(property) => reply = Some(property),
            Err(xcb::Error::Protocol(error)) => {
                ::log::debug!("../src/handlers.c:property_notify:1393 - got error {} when getting property of atom {}\n", { (i32::from((*error.as_raw()).error_code)) }, (atom) as i32);
                return;
            }
            // A native null reply without a protocol error still dispatched a
            // missing property. Keep that behavior on connection failures.
            Err(xcb::Error::Connection(_)) => {}
        }
    }
    dispatch_property(handler.cb.expect("non-null function pointer"), con, reply);
}

#[derive(Debug, PartialEq, Eq)]
struct ShapeChange {
    window: u32,
    kind: u8,
    shaped: u8,
}

fn shape_change(event: &xcb::Event, event_base: i32) -> Option<ShapeChange> {
    // The event owner guarantees a fixed wire body. Check the negotiated
    // extension opcode before borrowing its canonical view; never transfer its
    // allocation to that view or invoke the closed shape_kind enum getter.
    unsafe {
        let raw = event.as_raw();
        if !(0..=127).contains(&event_base) || i32::from((*raw).response_type & 0x7f) != event_base
        {
            return None;
        }
        let notification = std::mem::ManuallyDrop::new(xcb::shape::NotifyEvent::from_raw(raw));
        let bytes = raw.cast::<u8>();
        Some(ShapeChange {
            window: notification.affected_window().resource_id(),
            kind: *bytes.add(1),
            shaped: *bytes.add(20),
        })
    }
}

/// Dispatch an owned native queue event by borrowing its maintained owner.
///
/// # Safety
/// The event must come from the native queue, with libxcb's appended 32-bit
/// full_sequence field. A 32-byte event factory must go directly to its typed
/// leaf instead. Callers also hold exclusive access to the event-thread graph.
pub unsafe fn handle_event(owner: &xcb::Event) {
    let event = owner.as_raw();
    let type_0 = i32::from((*event).response_type & 0x7f);
    if type_0 != (<::xcb::x::MotionNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32) {
        ::log::debug!(
            "../src/handlers.c:handle_event:1412 - event type {}, xkb_base {}\n",
            { (type_0) },
            { (xkb_base) }
        );
    }
    if randr_base > -(1 as ::core::ffi::c_int)
        && type_0
            == randr_base
                + (<::xcb::randr::ScreenChangeNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32)
    {
        handle_screen_change();
        return;
    }
    if xkb_base > -(1 as ::core::ffi::c_int) && type_0 == xkb_base {
        ::log::debug!("../src/handlers.c:handle_event:1422 - xkb event, need to handle it.\n");
        match xcb_interop::events::xkb_event(owner, xkb_base as u8) {
            Some(xcb_interop::events::XkbEvent::NewKeyboard(state)) => {
                ::log::debug!("../src/handlers.c:handle_event:1426 - xkb new keyboard notify, sequence {}, time {}\n", { (state.sequence as ::core::ffi::c_int) }, (state.time) as i32);
                crate::application::rebuild_key_symbols();
                if state.changed as ::core::ffi::c_int
                    & (::xcb::xkb::NknDetail::KEYCODES.bits()) as ::core::ffi::c_int
                    != 0
                {
                    load_keymap();
                }
                ungrab_all_keys();
                translate_keysyms();
                grab_all_keys();
            }
            Some(xcb_interop::events::XkbEvent::Map(state)) => {
                if event_is_ignored(
                    ignore_event_sequence_t {
                        value: (*event).full_sequence,
                    },
                    type_0,
                ) {
                    ::log::debug!("../src/handlers.c:handle_event:1437 - Ignoring map notify event for sequence {}.\n", { (state.sequence as ::core::ffi::c_int) });
                } else {
                    ::log::debug!("../src/handlers.c:handle_event:1439 - xkb map notify, sequence {}, time {}\n", { (state.sequence as ::core::ffi::c_int) }, (state.time) as i32);
                    add_ignore_event(
                        ignore_event_sequence_t {
                            value: (*event).full_sequence,
                        },
                        type_0,
                    );
                    crate::application::rebuild_key_symbols();
                    ungrab_all_keys();
                    translate_keysyms();
                    grab_all_keys();
                    load_keymap();
                }
            }
            Some(xcb_interop::events::XkbEvent::State(state)) => {
                ::log::debug!(
                    "../src/handlers.c:handle_event:1449 - xkb state group = {}\n",
                    { (state.group as ::core::ffi::c_int) }
                );
                if xkb_current_group == state.group as ::core::ffi::c_int {
                    return;
                }
                xkb_current_group = state.group as ::core::ffi::c_int;
                ungrab_all_keys();
                grab_all_keys();
            }
            None => {}
        }
        return;
    }
    if shape_supported as ::core::ffi::c_int != 0
        && type_0 == shape_base + (<::xcb::shape::NotifyEvent as ::xcb::BaseEvent>::NUMBER as i32)
    {
        let Some(shape) = shape_change(owner, shape_base) else {
            return;
        };
        ::log::debug!("../src/handlers.c:handle_event:1465 - shape_notify_event for window 0x{:08x}, shape_kind = {}, shaped = {}\n", { (shape.window) }, { (shape.kind as ::core::ffi::c_int) }, { (shape.shaped as ::core::ffi::c_int) });
        let con: *mut Con = con_by_window_id(shape.window);
        if con.is_null() {
            ::log::info!(
                "Not a managed window 0x{:08x}, ignoring shape_notify_event\n",
                { (shape.window) }
            );
            return;
        }
        if shape.kind as ::core::ffi::c_int
            == (::xcb::shape::Sk::Bounding as u32) as ::core::ffi::c_int
            || shape.kind as ::core::ffi::c_int
                == (::xcb::shape::Sk::Input as u32) as ::core::ffi::c_int
        {
            x_set_shape(
                con,
                if shape.kind == (::xcb::shape::Sk::Bounding as u32) as u8 {
                    xcb::shape::Sk::Bounding
                } else {
                    xcb::shape::Sk::Input
                },
                shape.shaped != 0,
            );
        }
        return;
    }
    match owner {
        xcb::Event::X(x::Event::KeyPress(key) | x::Event::KeyRelease(key)) => {
            handle_key_press(key);
        }
        xcb::Event::X(x::Event::ButtonPress(button) | x::Event::ButtonRelease(button)) => {
            handle_button_press(button);
        }
        xcb::Event::X(x::Event::MapRequest(request)) => {
            handle_map_request(request, (*event).full_sequence);
        }
        xcb::Event::X(x::Event::UnmapNotify(notification)) => {
            handle_unmap_notify_event(notification, (*event).full_sequence);
        }
        xcb::Event::X(x::Event::DestroyNotify(notification)) => {
            handle_destroy_notify_event(notification, (*event).full_sequence);
        }
        xcb::Event::X(x::Event::Expose(expose)) => {
            if expose.count() == 0 {
                handle_expose_event(expose);
            }
        }
        xcb::Event::X(x::Event::MotionNotify(notification)) => {
            handle_motion_notify(notification);
        }
        xcb::Event::X(x::Event::EnterNotify(notification)) => {
            handle_enter_notify(notification, (*event).full_sequence);
        }
        xcb::Event::X(x::Event::ClientMessage(message)) => {
            handle_client_message(message);
        }
        xcb::Event::X(x::Event::ConfigureRequest(request)) => {
            handle_configure_request(request);
        }
        xcb::Event::X(x::Event::MappingNotify(mapping)) => {
            handle_mapping_notify(mapping);
        }
        xcb::Event::X(x::Event::FocusIn(notification)) => {
            handle_focus_in(notification);
        }
        xcb::Event::X(x::Event::FocusOut(notification)) => {
            handle_focus_out(notification);
        }
        xcb::Event::X(x::Event::PropertyNotify(property)) => {
            last_timestamp = property.time();
            property_notify(
                xcb_interop::events::property_notify_state(property),
                property.window().resource_id(),
                property.atom().resource_id(),
            );
        }
        xcb::Event::X(x::Event::ConfigureNotify(notification)) => {
            handle_configure_notify(notification);
        }
        xcb::Event::X(x::Event::SelectionClear(notification)) => {
            handle_selection_clear(notification);
        }
        _ => {}
    };
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    property_handlers = [
        PropertyHandler {
            atom: 0_u32,
            long_len: 128_u32,
            cb: Some(
                handle_windowname_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(handle_hints as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: 128_u32,
            cb: Some(
                handle_windowname_change_legacy
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_normal_hints as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_clientleader_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_transient_for as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: 128_u32,
            cb: Some(
                handle_windowrole_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: 128_u32,
            cb: Some(
                handle_class_change as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_strut_partial_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_window_type as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: 128_u32,
            cb: Some(
                handle_machine_change as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: 5_usize.wrapping_mul(::core::mem::size_of::<u64>() as usize) as u32,
            cb: Some(
                handle_motif_hints_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
        PropertyHandler {
            atom: 0_u32,
            long_len: UINT_MAX,
            cb: Some(
                handle_windowicon_change
                    as unsafe fn(*mut Con, Option<&x::GetPropertyReply>) -> bool,
            ),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::load_keymap;
use crate::bindings::translate_keysyms;
use crate::ewmh::ewmh_update_sticky;
use crate::ewmh::ewmh_update_wm_desktop;
use crate::ipc::ipc_send_event;
use crate::randr::randr_query_outputs;
use crate::render::render_deco_height;
use crate::scratchpad::scratchpad_fix_resolution;
use crate::sync::sync_respond;
use crate::tree::tree_render;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use startup_notification_sys::{
    sn_monitor_context_new, sn_xcb_display_process_event, SnMonitorEvent,
};

pub use libc::{exit, free, strcmp, time};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::conn_screen;
use crate::application::force_xinerama;
use crate::application::last_timestamp;
use crate::application::root;
use crate::application::shape_supported;
use crate::application::sndisplay;
use crate::application::wm_sn;
use crate::application::A_I3_SYNC;
use crate::application::A_WM_CHANGE_STATE;
use crate::application::A_WM_CLIENT_LEADER;
use crate::application::A_WM_STATE;
use crate::application::A_WM_WINDOW_ROLE;
use crate::application::A__MOTIF_WM_HINTS;
use crate::application::A__NET_ACTIVE_WINDOW;
use crate::application::A__NET_CLOSE_WINDOW;
use crate::application::A__NET_CURRENT_DESKTOP;
use crate::application::A__NET_FRAME_EXTENTS;
use crate::application::A__NET_MOVERESIZE_WINDOW;
use crate::application::A__NET_REQUEST_FRAME_EXTENTS;
use crate::application::A__NET_WM_DESKTOP;
use crate::application::A__NET_WM_ICON;
use crate::application::A__NET_WM_MOVERESIZE;
use crate::application::A__NET_WM_NAME;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_DEMANDS_ATTENTION;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::application::A__NET_WM_STATE_STICKY;
use crate::application::A__NET_WM_STRUT_PARTIAL;
use crate::application::A__NET_WM_WINDOW_TYPE;
use crate::assignments::run_assignments;
use crate::bindings::grab_all_keys;
use crate::click::handle_button_press;
use crate::con::con_activate_unblock;
use crate::con::con_attach;
use crate::con::con_border_style_rect;
use crate::con::con_by_frame_id;
use crate::con::con_by_window_id;
use crate::con::con_descend_focused;
use crate::con::con_detach;
use crate::con::con_focus;
use crate::con::con_for_window;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_inside_floating;
use crate::con::con_is_floating;
use crate::con::con_is_internal;
use crate::con::con_is_leaf;
use crate::con::con_move_to_workspace;
use crate::con::con_set_border_style;
use crate::con::con_set_urgency;
use crate::con::con_toggle_fullscreen;
use crate::config::config;
use crate::config::ungrab_all_keys;
use crate::ewmh::ewmh_get_workspace_by_index;
use crate::floating::floating_check_size;
use crate::floating::floating_drag_window;
use crate::floating::floating_enable;
use crate::floating::floating_reposition;
use crate::floating::floating_resize_window;
use crate::ipc::ipc_send_window_event;
use crate::key_press::handle_key_press;
use crate::manage::manage_window;
use crate::manage::remanage_window;
use crate::output::output_get_content;
use crate::output::output_primary_name;
use crate::output::output_push_sticky_windows;
use crate::randr::get_output_containing;
use crate::scratchpad::scratchpad_show;
use crate::startup::startup_monitor_event;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_close_internal;
use crate::util::rect_contains;
use crate::window::window_update_class;
use crate::window::window_update_hints;
use crate::window::window_update_icon;
use crate::window::window_update_leader;
use crate::window::window_update_machine;
use crate::window::window_update_motif_hints;
use crate::window::window_update_name;
use crate::window::window_update_name_legacy;
use crate::window::window_update_normal_hints;
use crate::window::window_update_role;
use crate::window::window_update_strut_partial;
use crate::window::window_update_transient_for;
use crate::window::window_update_type;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::workspace::workspace_show;
use crate::x::focused_id;
use crate::x::x_push_changes;
use crate::x::x_set_shape;
use crate::xcb::fake_absolute_configure_notify;
use i3_common::draw_util::draw_util_copy_surface;

use crate::xcb::xcb_numlock_mask;

#[cfg(test)]
mod owned_event_tests {
    use super::*;
    #[test]
    fn shape_ingress_checks_opcode_and_keeps_open_kind_and_shaped_bytes() {
        unsafe {
            let raw = libc::calloc(1, std::mem::size_of::<xcb::ffi::xcb_generic_event_t>())
                .cast::<xcb::ffi::xcb_generic_event_t>();
            assert!(!raw.is_null());
            (*raw).response_type = 64 | 0x80;
            raw.cast::<u8>()
                .add(4)
                .cast::<u32>()
                .write_unaligned(0x12345678);
            let event = xcb::resolve_event(raw, &[]);
            assert_eq!(shape_change(&event, -1), None);
            assert_eq!(shape_change(&event, 63), None);
            assert_eq!(shape_change(&event, 128), None);
            for kind in 0..=u8::MAX {
                for shaped in 0..=u8::MAX {
                    raw.cast::<u8>().add(1).write(kind);
                    raw.cast::<u8>().add(20).write(shaped);
                    assert_eq!(
                        shape_change(&event, 64),
                        Some(ShapeChange {
                            window: 0x12345678,
                            kind,
                            shaped
                        })
                    );
                }
            }
        }
    }

    #[test]
    fn crossing_and_focus_selectors_preserve_all_native_bytes_without_enum_conversion() {
        for (opcode, mode_offset) in [(7, 30), (9, 8), (10, 8)] {
            // Use a genuine queue-sized allocation owned by resolve_event;
            // each concrete adapter borrows only the fixed 32-byte event body.
            let raw = unsafe {
                libc::calloc(1, std::mem::size_of::<xcb::ffi::xcb_generic_event_t>())
                    .cast::<xcb::ffi::xcb_generic_event_t>()
            };
            assert!(!raw.is_null());
            unsafe { (*raw).response_type = opcode };
            let event = unsafe { xcb::resolve_event(raw, &[]) };
            for mode in 0..=u8::MAX {
                let detail = mode.wrapping_add(17);
                unsafe {
                    raw.cast::<u8>().add(mode_offset).write(mode);
                    raw.cast::<u8>().add(1).write(detail);
                }
                let selectors = match &event {
                    xcb::Event::X(x::Event::EnterNotify(event)) => enter_selectors(event),
                    xcb::Event::X(x::Event::FocusIn(event) | x::Event::FocusOut(event)) => {
                        focus_selectors(event)
                    }
                    _ => panic!("unexpected native event opcode"),
                };
                assert_eq!(selectors, (mode, detail));
            }
        }
    }

    #[test]
    fn unknown_and_pointer_mapping_selectors_are_ignored_before_native_cache_access() {
        for request in [x::Mapping::Pointer as u8, u8::MAX] {
            unsafe {
                let raw = libc::calloc(1, std::mem::size_of::<xcb::ffi::xcb_generic_event_t>())
                    .cast::<xcb::ffi::xcb_generic_event_t>();
                assert!(!raw.is_null());
                (*raw).response_type =
                    (<::xcb::x::MappingNotifyEvent as ::xcb::BaseEvent>::NUMBER as i32) as u8;
                raw.cast::<u8>().add(4).write(request);
                let event = xcb::resolve_event(raw, &[]);
                let xcb::Event::X(x::Event::MappingNotify(mapping)) = &event else {
                    panic!("mapping opcode was not decoded as a maintained event");
                };
                // No display or key-symbol owner exists in this test. Processing
                // an unknown selector must stop before a lookup or enum getter.
                handle_mapping_notify(mapping);
            }
        }
    }
    #[test]
    fn property_dispatch_keeps_owned_reply_alive_through_a_borrowing_callback_unwind() {
        unsafe fn borrow_then_panic(_: *mut Con, reply: Option<&x::GetPropertyReply>) -> bool {
            assert_eq!(reply.unwrap().format(), 8);
            panic!("callback unwound while borrowing its reply");
        }
        let raw = unsafe { libc::calloc(1, 32).cast::<u8>() };
        assert!(!raw.is_null());
        unsafe {
            raw.write(1);
            raw.add(1).write(8);
        }
        let reply: x::GetPropertyReply = unsafe { xcb::Reply::from_raw(raw) };
        let result = std::panic::catch_unwind(|| unsafe {
            dispatch_property(borrow_then_panic, std::ptr::null_mut(), Some(reply));
        });
        assert!(result.is_err());
    }
    #[test]
    fn configure_selectors_retain_every_mask_bit_and_unknown_stack_mode() {
        let event = x::ConfigureRequestEvent::new(
            x::StackMode::Above,
            x::Window::none(),
            x::Window::none(),
            x::Window::none(),
            0,
            0,
            0,
            0,
            0,
            x::ConfigWindowMask::empty(),
        );
        for mask in 0..=u16::MAX {
            let mode = mask as u8;
            unsafe {
                event.as_raw().cast::<u8>().add(1).write(mode);
                event
                    .as_raw()
                    .cast::<u8>()
                    .add(26)
                    .cast::<u16>()
                    .write_unaligned(mask);
            }
            assert_eq!(configure_selectors(&event), (mode, mask));
        }
    }

    #[test]
    fn configure_packets_keep_mask_order_signed_positions_and_ignored_bits() {
        let event = x::ConfigureRequestEvent::new(
            x::StackMode::Opposite,
            x::Window::none(),
            x::Window::none(),
            x::Window::new(99),
            -3,
            -2,
            640,
            480,
            2,
            x::ConfigWindowMask::empty(),
        );
        unsafe {
            event
                .as_raw()
                .cast::<u8>()
                .add(26)
                .cast::<u16>()
                .write_unaligned(u16::MAX)
        };
        assert_eq!(
            configure_values(&event),
            (
                127,
                vec![(-3i32) as u32, (-2i32) as u32, 640, 480, 2, 99, 4]
            )
        );
        let typed = configure_window_values(&event);
        assert!(matches!(
            typed.as_slice(),
            [
                x::ConfigWindow::X(-3),
                x::ConfigWindow::Y(-2),
                x::ConfigWindow::Width(640),
                x::ConfigWindow::Height(480),
                x::ConfigWindow::BorderWidth(2),
                x::ConfigWindow::Sibling(_),
                x::ConfigWindow::StackMode(x::StackMode::Opposite),
            ]
        ));
        unsafe { event.as_raw().cast::<u8>().add(1).write(u8::MAX) };
        assert_eq!(checked_stack_mode(configure_selectors(&event).0), None);
        assert_eq!(configure_values(&event).1.last(), Some(&255));
        unsafe { event.as_raw().cast::<u8>().add(1).write(4) };
        unsafe {
            event
                .as_raw()
                .cast::<u8>()
                .add(26)
                .cast::<u16>()
                .write_unaligned(
                    ((::xcb::x::ConfigWindowMask::Y.bits())
                        | (::xcb::x::ConfigWindowMask::SIBLING.bits())) as u16,
                )
        };
        assert_eq!(
            configure_values(&event),
            (configure_selectors(&event).1, vec![(-2i32) as u32, 99])
        );
        unsafe {
            event
                .as_raw()
                .cast::<u8>()
                .add(26)
                .cast::<u16>()
                .write_unaligned(0x8000)
        };
        assert_eq!(configure_values(&event), (0, vec![]));
    }
    #[test]
    fn name_snapshot_outlives_replaced_native_text_and_preserves_repaired_native_view() {
        let mut window = Box::new(i3Window::default());
        window.name = Some(i3_common::string::I3String::from_bytes(b"first\0ignored"));
        unsafe {
            let snapshot = window_name_snapshot(&raw mut *window);
            window.name = Some(i3_common::string::I3String::from_bytes(b"second"));
            assert_eq!(snapshot.as_deref(), Some("first�ignored".as_bytes()));
            assert_ne!(snapshot, window_name_snapshot(&raw mut *window));
            window.name = None;
            assert_eq!(window_name_snapshot(&raw mut *window), None);
        }
    }
    #[test]
    fn ignored_events_keep_strict_ttl_wildcards_duplicates_and_backward_clock() {
        let mut events = IgnoredEvents::default();
        events.add(7, 2, 100);
        events.add(7, -1, 101);
        assert!(events.contains(7, 99, 106));
        assert_eq!(events.0.len(), 1);
        assert!(!events.contains(7, 99, 107));
        events.add(8, 3, 200);
        assert!(events.contains(8, 3, 190));
        assert!(!events.contains(8, 4, 190));
        assert_eq!(events.0.len(), 1);
    }
    #[test]
    fn ignored_events_remain_newest_first_without_consuming_matches() {
        let mut events = IgnoredEvents::default();
        events.add(1, 2, 0);
        events.add(1, 3, 1);
        events.add(2, -1, 2);
        assert_eq!(
            events
                .0
                .iter()
                .map(|event| (event.sequence, event.response))
                .collect::<Vec<_>>(),
            [(2, -1), (1, 3), (1, 2)]
        );
        assert!(events.contains(1, 2, 5));
        assert!(events.contains(1, 2, 5));
        assert!(events.contains(1, 3, 6));
        assert!(!events.contains(1, 2, 6));
    }
}

use crate::con::arena::nodes;

fn connection() -> std::rc::Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
}
