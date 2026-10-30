use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::string::I3String;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::fd::FromRawFd;
use std::ptr::NonNull;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-nagbar is a utility which displays a nag message, for example in the case
 * when the user has an error in their configuration file.
 *
 */
// Mechanical file-by-file Rust translation of i3-nagbar/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::dpi::{init_dpi, logical_px};
use i3_common::get_exe_path::get_exe_path;
use i3_common::i333_va_list;

use i3_common::font::C2RustUnnamed;
use i3_common::font::C2RustUnnamed_0;
use i3_common::font::Font;

use i3_common::draw_util::color_t;
use i3_common::draw_util::surface_t;

#[derive(Copy, Clone)]
#[repr(C)]
struct WmStrutPartial {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
    left_start_y: u32,
    left_end_y: u32,
    right_start_y: u32,
    right_end_y: u32,
    top_start_x: u32,
    top_end_x: u32,
    bottom_start_x: u32,
    bottom_end_x: u32,
}

const _: () = assert!(std::mem::size_of::<WmStrutPartial>() == 12 * 4);

pub static mut visual_type: *mut xcb_visualtype_t = std::ptr::null_mut();

static mut root: xcb_window_t = 0;

pub static mut conn: *mut xcb_connection_t = std::ptr::null_mut();

pub static mut root_screen: *mut xcb_screen_t = std::ptr::null_mut();

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {}
// Prepare Rust-owned bytes/files in the parent before this legacy process edge.
// Children retain the existing fork/setsid/exec and error/exit behavior; notably
// the legacy exec-failure diagnostic and exit are not async-signal-safe. No new
// Rust allocation, lock acquisition or destructor is introduced in either child.
// Command bytes belong to its forked address space until exec.
unsafe fn start_application(command: *const ::core::ffi::c_char) {
    printf(c"executing: %s\n".as_ptr(), command);
    if fork() == 0 as ::core::ffi::c_int {
        setsid();
        if fork() == 0 as ::core::ffi::c_int {
            execl(
                c"/bin/sh".as_ptr(),
                c"/bin/sh".as_ptr(),
                c"-c".as_ptr(),
                command,
                std::ptr::null::<libc::c_char>(),
            );
            err(libc::EXIT_FAILURE, c"execl return".as_ptr());
        }
        exit(0 as ::core::ffi::c_int);
    }
    wait(::core::ptr::null_mut::<::core::ffi::c_int>());
}
fn command_script(path: &CStr, action: Option<&CStr>) -> Vec<u8> {
    let mut bytes = b"#!/bin/sh\nrm ".to_vec();
    bytes.extend_from_slice(path.to_bytes());
    bytes.push(b'\n');
    bytes.extend_from_slice(action.unwrap_or(c"(null)").to_bytes());
    bytes
}

fn terminal_command(path: &CStr, terminal: bool) -> CString {
    let mut bytes = if terminal {
        b"i3-sensible-terminal -e ".to_vec()
    } else {
        Vec::new()
    };
    bytes.extend_from_slice(path.to_bytes());
    CString::new(bytes).expect("C string inputs contain no NUL")
}

// Native filename helpers return unique malloc-owned NUL-terminated strings.
unsafe fn take_native_string(raw: *mut libc::c_char) -> CString {
    let owned = CStr::from_ptr(raw).to_owned();
    free(raw.cast());
    owned
}

unsafe fn handle_button_release(state: &NagbarState, event: &xcb_button_release_event_t) -> bool {
    printf(
        c"button released on x = %d, y = %d\n".as_ptr(),
        i32::from(event.event_x),
        i32::from(event.event_y),
    );
    if state.close.contains(i32::from(event.event_x), false) {
        return true;
    }
    let Some(button) = state.button_at(i32::from(event.event_x)) else {
        return false;
    };
    let script_path = take_native_string(get_process_filename(c"nagbar-cmd".as_ptr()));
    let fd = open(
        script_path.as_ptr(),
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if fd == -1 {
        warn(c"Could not create temporary script to store the nagbar command".as_ptr());
        return false;
    }
    // SAFETY: open returned a uniquely owned valid file descriptor. Closing
    // this file releases only the descriptor, never the asynchronously read path.
    let mut script = std::fs::File::from_raw_fd(fd);
    let _ = script.write_all(&command_script(&script_path, button.action.as_deref()));
    drop(script);
    let exe_path = take_native_string(get_exe_path(state.program.as_ptr().cast_mut()));
    let mut link_bytes = script_path.to_bytes().to_vec();
    link_bytes.extend_from_slice(b".nagbar_cmd");
    let link_path = CString::new(link_bytes).expect("C string path has no NUL");
    if symlink(exe_path.as_ptr(), link_path.as_ptr()) == -1 {
        err(
            libc::EXIT_FAILURE,
            c"Failed to symlink %s to %s".as_ptr(),
            link_path.as_ptr(),
            exe_path.as_ptr(),
        );
    }
    let command = terminal_command(&link_path, button.terminal);
    printf(c"argv0 = %s\n".as_ptr(), state.program.as_ptr());
    printf(c"terminal_cmd = %s\n".as_ptr(), command.as_ptr());
    // The script removes itself and the special invocation removes the symlink.
    // Do not unlink either here: the detached terminal may not have read it yet.
    start_application(command.as_ptr());
    false
}
/// Native rendering requires the initialized event-thread display/font. All
/// labels and layout values are borrowed from the owned session.
unsafe fn button_draw(button: &mut Button, position: i32, render: &mut Renderer) -> i32 {
    let text_width = predict_text_width(&button.label).max(0);
    let padding = logical_px(3).max(0);
    let top = logical_px(8);
    (button.x, button.width) = button_layout(text_width, padding, position);
    draw_util_rectangle(
        &mut render.bar,
        render.color_border,
        f64::from(button.x),
        f64::from(top.saturating_sub(padding.saturating_mul(2))),
        f64::from(button.width),
        f64::from(render.font.height.saturating_add(padding.saturating_mul(4))),
    );
    draw_util_rectangle(
        &mut render.bar,
        render.color_button_background,
        f64::from(button.x.saturating_add(padding)),
        f64::from(top.saturating_sub(padding)),
        f64::from(text_width.saturating_add(padding.saturating_mul(2))),
        f64::from(render.font.height.saturating_add(padding.saturating_mul(2))),
    );
    draw_util_text(
        &button.label,
        &mut render.bar,
        render.color_text,
        render.color_button_background,
        button.x.saturating_add(padding.saturating_mul(2)),
        top,
        200,
    );
    button.width
}
unsafe fn handle_expose(state: &mut NagbarState) {
    let render = &mut state.render;
    draw_util_clear_surface(&mut render.bar, render.color_background);
    let margin = logical_px(8);
    draw_util_text(
        &state.message,
        &mut render.bar,
        render.color_text,
        render.color_background,
        margin,
        margin,
        render.bar.width.saturating_sub(margin.saturating_mul(2)),
    );
    let mut position = render
        .bar
        .width
        .saturating_sub(margin.saturating_sub(logical_px(3).saturating_mul(2)));
    position = position.saturating_sub(button_draw(&mut state.close, position, render));
    position = position.saturating_sub(logical_px(15));
    for button in &mut state.buttons {
        position = position.saturating_sub(logical_px(20));
        position = position.saturating_sub(button_draw(button, position, render));
    }
    draw_util_rectangle(
        &mut render.bar,
        render.color_border_bottom,
        0.0,
        f64::from(render.bar.height.saturating_sub(logical_px(2))),
        f64::from(render.bar.width),
        f64::from(logical_px(2)),
    );
    xcb_flush(conn);
}
unsafe fn set_window_position_primary(result: &mut xcb_rectangle_t) {
    let mut replies = NativeReplies::default();
    let mut output: *mut xcb_randr_get_output_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_output_info_reply_t>();
    let mut crtc: *mut xcb_randr_get_crtc_info_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_crtc_info_reply_t>();
    let rcookie: xcb_randr_get_screen_resources_current_cookie_t =
        xcb_randr_get_screen_resources_current(conn, root);
    let pcookie: xcb_randr_get_output_primary_cookie_t = xcb_randr_get_output_primary(conn, root);
    let mut primary: *mut xcb_randr_get_output_primary_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_output_primary_reply_t>();
    let mut res: *mut xcb_randr_get_screen_resources_current_reply_t =
        ::core::ptr::null_mut::<xcb_randr_get_screen_resources_current_reply_t>();
    primary = replies.own(xcb_randr_get_output_primary_reply(
        conn,
        pcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ));
    if primary.is_null() {
        verboselog(
            c"[libi3] ../i3-nagbar/main.c Could not determine the primary output.\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        res = replies.own(xcb_randr_get_screen_resources_current_reply(
            conn,
            rcookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ));
        if res.is_null() {
            verboselog(
                c"[libi3] ../i3-nagbar/main.c Could not query screen resources.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            output = replies.own(xcb_randr_get_output_info_reply(
                conn,
                xcb_randr_get_output_info(conn, (*primary).output, (*res).config_timestamp),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            ));
            if output.is_null() || (*output).crtc as ::core::ffi::c_long == XCB_NONE {
                verboselog(
                    c"[libi3] ../i3-nagbar/main.c Could not query primary screen.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                crtc = replies.own(xcb_randr_get_crtc_info_reply(
                    conn,
                    xcb_randr_get_crtc_info(conn, (*output).crtc, (*res).config_timestamp),
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                ));
                if crtc.is_null() {
                    verboselog(
                        c"[libi3] ../i3-nagbar/main.c Could not get CRTC.\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    verboselog(
                        c"[libi3] ../i3-nagbar/main.c Found primary output on position x = %i / y = %i / w = %i / h = %i.\n".as_ptr().cast_mut(),
                        (*crtc).x as ::core::ffi::c_int,
                        (*crtc).y as ::core::ffi::c_int,
                        (*crtc).width as ::core::ffi::c_int,
                        (*crtc).height as ::core::ffi::c_int,
                    );
                    if (*crtc).width as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || (*crtc).height as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        verboselog(
                            c"[libi3] ../i3-nagbar/main.c Primary output is not active, ignoring it.\n".as_ptr().cast_mut(),
                        );
                    } else {
                        result.x = (*crtc).x;
                        result.y = (*crtc).y;
                    }
                }
            }
        }
    }
}
// The native server lock must also be released if reply bookkeeping panics.
struct ServerGrab(*mut xcb_connection_t);
impl Drop for ServerGrab {
    fn drop(&mut self) {
        // SAFETY: the owning event-thread positioning call keeps this connection
        // live until the guard drops; construction follows a matching grab.
        unsafe {
            xcb_ungrab_server(self.0);
        }
    }
}

unsafe fn set_window_position_focus(result: &mut xcb_rectangle_t) {
    let mut replies = NativeReplies::default();
    let mut success: bool = false;
    let mut input_focus: *mut xcb_get_input_focus_reply_t =
        ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    let mut geometry: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut coordinates: *mut xcb_translate_coordinates_reply_t =
        ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    xcb_grab_server(conn);
    let server_grab = ServerGrab(conn);
    input_focus = replies.own(xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ));
    if input_focus.is_null() || (*input_focus).focus as ::core::ffi::c_long == XCB_NONE {
        verboselog(
            c"[libi3] ../i3-nagbar/main.c Failed to receive the current input focus or no window has the input focus right now.\n".as_ptr().cast_mut(),
        );
    } else {
        geometry = replies.own(xcb_get_geometry_reply(
            conn,
            xcb_get_geometry(conn, (*input_focus).focus as xcb_drawable_t),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ));
        if geometry.is_null() {
            verboselog(
                c"[libi3] ../i3-nagbar/main.c Failed to received window geometry.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            coordinates = replies.own(xcb_translate_coordinates_reply(
                conn,
                xcb_translate_coordinates(
                    conn,
                    (*input_focus).focus,
                    root,
                    (*geometry).x,
                    (*geometry).y,
                ),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            ));
            if coordinates.is_null() {
                verboselog(
                    c"[libi3] ../i3-nagbar/main.c Failed to translate coordinates.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                verboselog(
                    c"[libi3] ../i3-nagbar/main.c Found current focus at x = %i / y = %i.\n"
                        .as_ptr()
                        .cast_mut(),
                    (*coordinates).dst_x as ::core::ffi::c_int,
                    (*coordinates).dst_y as ::core::ffi::c_int,
                );
                result.x = (*coordinates).dst_x;
                result.y = (*coordinates).dst_y;
                success = true;
            }
        }
    }
    drop(server_grab);
    if !success {
        verboselog(
            c"[libi3] ../i3-nagbar/main.c Could not position on focused output, trying to position on primary output.\n".as_ptr().cast_mut(),
        );
        set_window_position_primary(result);
    }
}
struct ButtonSettings {
    label: std::ffi::CString,
    action: Option<std::ffi::CString>,
    terminal: bool,
}
struct NagbarSettings {
    program: std::ffi::CString,
    font: std::ffi::CString,
    message: std::ffi::CString,
    warning: bool,
    primary: bool,
    buttons: Vec<ButtonSettings>,
}
const NAGBAR_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::value(b'f', "font"),
    i3_common::cli::OptionSpec::value(b'b', "button"),
    i3_common::cli::OptionSpec::value(b'B', "button-no-terminal"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
    i3_common::cli::OptionSpec::value(b'm', "message"),
    i3_common::cli::OptionSpec::value(b't', "type"),
    i3_common::cli::OptionSpec::flag(b'p', "primary"),
];

struct Button {
    label: I3String,
    action: Option<CString>,
    x: i32,
    width: i32,
    terminal: bool,
}
impl Button {
    // Compatibility: action buttons include the right edge, close excludes it;
    // the historical hit test intentionally ignores y and mouse button number.
    fn contains(&self, x: i32, inclusive: bool) -> bool {
        let right = self.x.saturating_add(self.width);
        x >= self.x && if inclusive { x <= right } else { x < right }
    }
}

fn button_layout(text_width: i32, padding: i32, position: i32) -> (i32, i32) {
    let width = text_width
        .max(0)
        .saturating_add(padding.max(0).saturating_mul(4));
    (position.saturating_sub(width), width)
}

struct Renderer {
    bar: surface_t,
    font: Font,
    color_background: color_t,
    color_button_background: color_t,
    color_border: color_t,
    color_border_bottom: color_t,
    color_text: color_t,
}
struct NagbarState {
    program: CString,
    message: I3String,
    close: Button,
    buttons: Vec<Button>,
    render: Renderer,
    connection: Option<NonNull<xcb_connection_t>>,
    window: xcb_window_t,
    has_font: bool,
    has_surface: bool,
}
impl NagbarState {
    fn button_at(&self, x: i32) -> Option<&Button> {
        self.buttons.iter().find(|button| button.contains(x, true))
    }
    fn new(settings: NagbarSettings) -> Self {
        Self {
            program: settings.program,
            message: I3String::from_cstr(&settings.message),
            close: Button {
                label: I3String::new("X"),
                action: None,
                x: 0,
                width: 0,
                terminal: false,
            },
            buttons: settings
                .buttons
                .into_iter()
                .map(|button| Button {
                    label: I3String::from_cstr(&button.label),
                    action: button.action,
                    x: 0,
                    width: 0,
                    terminal: button.terminal,
                })
                .collect(),
            render: Renderer {
                bar: surface_t {
                    id: 0,
                    gc: 0,
                    owns_gc: false,
                    width: 0,
                    height: 0,
                    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
                    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
                },
                font: Font {
                    type_0: i3_common::font::FontType::None,
                    height: 0,
                    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
                    specific: C2RustUnnamed {
                        xcb: C2RustUnnamed_0 {
                            id: 0,
                            info: ::core::ptr::null::<xcb_query_font_reply_t>()
                                as *mut xcb_query_font_reply_t,
                            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
                        },
                    },
                },
                color_background: color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    colorpixel: 0,
                },
                color_button_background: color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    colorpixel: 0,
                },
                color_border: color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    colorpixel: 0,
                },
                color_border_bottom: color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    colorpixel: 0,
                },
                color_text: color_t {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                    colorpixel: 0,
                },
            },
            connection: None,
            window: 0,
            has_font: false,
            has_surface: false,
        }
    }
}
impl Drop for NagbarState {
    fn drop(&mut self) {
        // SAFETY: this session uniquely owns the native display resources, and
        // all event borrows have ended. Native destructors run before disconnect;
        // provider globals are only borrowed on this same event thread.
        unsafe {
            if let Some(connection) = self.connection {
                if self.has_surface {
                    draw_util_surface_free(connection.as_ptr(), &mut self.render.bar);
                }
                if self.has_font {
                    i3_common::font::free_font();
                }
                if self.window != 0 {
                    xcb_destroy_window(connection.as_ptr(), self.window);
                }
                root_screen = std::ptr::null_mut();
                visual_type = std::ptr::null_mut();
                root = 0;
                conn = std::ptr::null_mut();
                xcb_disconnect(connection.as_ptr());
            }
        }
    }
}

// Scoped malloc-owned native replies/events. The constructor remains unsafe:
// it cannot validate arbitrary raw pointers or turn borrowed storage into owners.
// A value must only be constructed from a unique malloc-owned live allocation
// returned by XCB (or null); it must outlive every borrowed event view.
struct NativeAllocation<T>(NonNull<T>);
impl<T> NativeAllocation<T> {
    unsafe fn new(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
    fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T> Drop for NativeAllocation<T> {
    fn drop(&mut self) {
        unsafe { free(self.0.as_ptr().cast()) }
    }
}

#[derive(Default)]
struct NativeReplies(Vec<NonNull<libc::c_void>>);
impl NativeReplies {
    /// Caller transfers unique ownership of a malloc allocation (or null),
    /// retaining only borrows which end before this group is dropped.
    unsafe fn own<T>(&mut self, pointer: *mut T) -> *mut T {
        if let Some(pointer) = NonNull::new(pointer) {
            self.0.push(pointer.cast());
        }
        pointer
    }
}
impl Drop for NativeReplies {
    fn drop(&mut self) {
        for pointer in self.0.drain(..) {
            unsafe { free(pointer.as_ptr()) }
        }
    }
}

unsafe fn atom_reply(cookie: xcb_intern_atom_cookie_t, name: &CStr) -> xcb_atom_t {
    let Some(reply) =
        NativeAllocation::new(xcb_intern_atom_reply(conn, cookie, std::ptr::null_mut()))
    else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom %s".as_ptr(),
            name.as_ptr(),
        );
    };
    (*reply.as_ptr()).atom
}

/// Own one native display session on the application's sole event thread.
///
/// # Safety
/// The common display provider and font globals must have no concurrent users;
/// this call installs and later clears their borrowed session pointers.
unsafe fn run_window(settings: NagbarSettings) -> ::core::ffi::c_int {
    let pattern = settings.font.clone();
    let position_on_primary = settings.primary;
    let bar_type = if settings.warning {
        MessageType::Warning
    } else {
        MessageType::Error
    };
    // The common font backend retains this exact field address until free_font.
    // Keep the session allocation stable throughout the native event loop.
    let mut state = Box::new(NagbarState::new(settings));
    let mut screens: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screens);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    state.connection = NonNull::new(conn);
    let mut _NET_WM_WINDOW_TYPE_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_WINDOW_TYPE".as_ptr()) as u16,
        c"_NET_WM_WINDOW_TYPE".as_ptr(),
    );
    let mut _NET_WM_WINDOW_TYPE_DOCK_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_WINDOW_TYPE_DOCK".as_ptr()) as u16,
        c"_NET_WM_WINDOW_TYPE_DOCK".as_ptr(),
    );
    let mut _NET_WM_STRUT_PARTIAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_WM_STRUT_PARTIAL".as_ptr()) as u16,
        c"_NET_WM_STRUT_PARTIAL".as_ptr(),
    );
    let I3_SOCKET_PATH_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"I3_SOCKET_PATH".as_ptr()) as u16,
        c"I3_SOCKET_PATH".as_ptr(),
    );
    let ATOM_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"ATOM".as_ptr()) as u16,
        c"ATOM".as_ptr(),
    );
    let CARDINAL_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"CARDINAL".as_ptr()) as u16,
        c"CARDINAL".as_ptr(),
    );
    let sndisplay: *mut SnDisplay = sn_xcb_display_new(conn, None, None);
    let sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screens);
    sn_display_unref(sndisplay);
    root_screen = xcb_aux_get_screen(conn, screens);
    if root_screen.is_null() {
        errx(libc::EXIT_FAILURE, c"Cannot find display screen".as_ptr());
    }
    root = (*root_screen).root;
    if bar_type == MessageType::Error {
        state.render.color_button_background = draw_util_hex_to_color(c"#680a0a".as_ptr());
        state.render.color_background = draw_util_hex_to_color(c"#900000".as_ptr());
        state.render.color_text = draw_util_hex_to_color(c"#ffffff".as_ptr());
        state.render.color_border = draw_util_hex_to_color(c"#d92424".as_ptr());
        state.render.color_border_bottom = draw_util_hex_to_color(c"#470909".as_ptr());
    } else {
        state.render.color_button_background = draw_util_hex_to_color(c"#ffc100".as_ptr());
        state.render.color_background = draw_util_hex_to_color(c"#ffa800".as_ptr());
        state.render.color_text = draw_util_hex_to_color(c"#000000".as_ptr());
        state.render.color_border = draw_util_hex_to_color(c"#ab7100".as_ptr());
        state.render.color_border_bottom = draw_util_hex_to_color(c"#ab7100".as_ptr());
    }
    init_dpi();
    state.render.font = load_font(pattern.as_ptr(), true);
    state.has_font = true;
    set_font(&mut state.render.font);
    let mut win_pos: xcb_rectangle_t = xcb_rectangle_t {
        x: 50 as i16,
        y: 50 as i16,
        width: 500 as u16,
        height: state
            .render
            .font
            .height
            .saturating_add(logical_px(8).saturating_mul(2))
            .saturating_add(logical_px(2))
            .clamp(1, i32::from(u16::MAX)) as u16,
    };
    if position_on_primary {
        set_window_position_primary(&mut win_pos);
    } else {
        set_window_position_focus(&mut win_pos);
    }
    let mut cursor_ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut::<xcb_cursor_context_t>();
    if xcb_cursor_context_new(conn, root_screen, &raw mut cursor_ctx) < 0 || cursor_ctx.is_null() {
        errx(
            libc::EXIT_FAILURE,
            c"Cannot allocate xcursor context".as_ptr(),
        );
    }
    let cursor: xcb_cursor_t = xcb_cursor_load_cursor(cursor_ctx, c"left_ptr".as_ptr());
    xcb_cursor_context_free(cursor_ctx);
    state.window = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh2: [u32; 3] = [
        0 as ::core::ffi::c_int as u32,
        (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
            | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
            | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int) as u32,
        cursor,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as u8,
        state.window,
        root,
        win_pos.x,
        win_pos.y,
        win_pos.width,
        win_pos.height,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int
            | XCB_CW_CURSOR as ::core::ffi::c_int) as u32,
        &raw mut fresh2 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_sys::xcb_free_cursor(conn, cursor);
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, state.window as libc::c_ulong);
    }
    xcb_map_window(conn, state.window);
    let A__NET_WM_WINDOW_TYPE = atom_reply(_NET_WM_WINDOW_TYPE_cookie, c"_NET_WM_WINDOW_TYPE");
    let A__NET_WM_WINDOW_TYPE_DOCK =
        atom_reply(_NET_WM_WINDOW_TYPE_DOCK_cookie, c"_NET_WM_WINDOW_TYPE_DOCK");
    let A__NET_WM_STRUT_PARTIAL =
        atom_reply(_NET_WM_STRUT_PARTIAL_cookie, c"_NET_WM_STRUT_PARTIAL");
    let A_I3_SOCKET_PATH = atom_reply(I3_SOCKET_PATH_cookie, c"I3_SOCKET_PATH");
    let A_ATOM = atom_reply(ATOM_cookie, c"ATOM");
    let A_CARDINAL = atom_reply(CARDINAL_cookie, c"CARDINAL");
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        state.window,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as u8,
        1 as u32,
        (&raw const A__NET_WM_WINDOW_TYPE_DOCK).cast(),
    );
    let mut strut_partial: WmStrutPartial = WmStrutPartial {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        left_start_y: 0,
        left_end_y: 0,
        right_start_y: 0,
        right_end_y: 0,
        top_start_x: 0,
        top_end_x: 0,
        bottom_start_x: 0,
        bottom_end_x: 0,
    };
    strut_partial.top = state
        .render
        .font
        .height
        .saturating_add(logical_px(6))
        .max(0) as u32;
    strut_partial.top_start_x = 0 as u32;
    strut_partial.top_end_x = 800 as u32;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        state.window,
        A__NET_WM_STRUT_PARTIAL,
        A_CARDINAL,
        32 as u8,
        12 as u32,
        &raw mut strut_partial as *const ::core::ffi::c_void,
    );
    draw_util_surface_init(
        conn,
        &mut state.render.bar,
        state.window as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    state.has_surface = true;
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
        sn_launchee_context_unref(sncontext);
    }
    xcb_flush(conn);
    loop {
        let event = xcb_wait_for_event(conn);
        let Some(owned_event) = NativeAllocation::new(event) else {
            break;
        };
        let event = owned_event.as_ptr();
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                c"X11 Error received! sequence %x\n".as_ptr(),
                (*event).sequence as ::core::ffi::c_int,
            );
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose(&mut state);
                    }
                }
                XCB_BUTTON_PRESS => {
                    let press = &*event.cast::<xcb_button_press_event_t>();
                    printf(
                        c"button pressed on x = %d, y = %d\n".as_ptr(),
                        i32::from(press.event_x),
                        i32::from(press.event_y),
                    );
                }
                XCB_BUTTON_RELEASE => {
                    if handle_button_release(&state, &*event.cast::<xcb_button_release_event_t>()) {
                        return 0;
                    }
                }
                XCB_CONFIGURE_NOTIFY => {
                    let configure_notify: *mut xcb_configure_notify_event_t =
                        event as *mut xcb_configure_notify_event_t;
                    if (*configure_notify).width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*configure_notify).height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                    {
                        draw_util_surface_set_size(
                            &mut state.render.bar,
                            (*configure_notify).width as ::core::ffi::c_int,
                            (*configure_notify).height as ::core::ffi::c_int,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    0 as ::core::ffi::c_int
}

pub fn main() {
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    use std::os::unix::process::CommandExt;
    let mut arguments = std::env::args_os();
    let program = arguments.next().unwrap_or_else(|| "i3-nagbar".into());
    // Terminal button scripts invoke this special symlink. Preserve its
    // remove-and-exec contract without allocating or mutating a C argv array.
    if program.as_bytes().len() > b".nagbar_cmd".len()
        && program.as_bytes().ends_with(b".nagbar_cmd")
    {
        let _ = std::fs::remove_file(&program);
        let path = std::ffi::OsString::from_vec(
            program.as_bytes()[..program.as_bytes().len() - b".nagbar_cmd".len()].to_vec(),
        );
        let error = std::process::Command::new("/bin/sh").arg(&path).exec();
        let error = error.to_string();
        eprintln!(
            "{}: execl(/bin/sh, /bin/sh, {}): {}",
            std::path::Path::new(&program)
                .file_name()
                .unwrap_or(&program)
                .to_string_lossy(),
            path.to_string_lossy(),
            error.split(" (os error ").next().unwrap_or(&error)
        );
        std::process::exit(1);
    }
    let c_string = |value: std::ffi::OsString| {
        std::ffi::CString::new(value.as_bytes()).expect("OS arguments have no NUL bytes")
    };
    let mut settings = NagbarSettings {
        program: c_string(program.clone()),
        font: c"pango:monospace 8".to_owned(),
        message: c"Please do not run this program.".to_owned(),
        warning: false,
        primary: false,
        buttons: Vec::new(),
    };
    let mut parser = Parser::new(
        arguments,
        NAGBAR_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    );
    while let Some(item) = parser.next() {
        match item {
            Item::Error(message) => eprintln!("{}: {message}", program.to_string_lossy()),
            Item::Operand(_) => {}
            Item::Option { short, value } => match short {
                b'v' => {
                    println!("i3-nagbar {}", env!("I3_VERSION"));
                    return;
                }
                b'h' => {
                    println!(
                        "i3-nagbar {}\ni3-nagbar [-m <message>] [-b <button> <action>] [-B <button> <action>] [-t warning|error] [-f <font>] [-v] [-p]",
                        env!("I3_VERSION")
                    );
                    return;
                }
                b'p' => settings.primary = true,
                b'f' => settings.font = c_string(value.unwrap()),
                b'm' => settings.message = c_string(value.unwrap()),
                b't' => {
                    settings.warning = value.unwrap().as_bytes().eq_ignore_ascii_case(b"warning")
                }
                b'b' | b'B' => {
                    let label = c_string(value.unwrap());
                    let action = parser.take_argument().map(c_string);
                    println!(
                        "button with label *{}* and action *{}*",
                        label.to_string_lossy(),
                        action
                            .as_ref()
                            .map_or_else(|| "(null)".into(), |action| action.to_string_lossy())
                    );
                    settings.buttons.push(ButtonSettings {
                        label,
                        action,
                        terminal: short == b'b',
                    });
                    println!("now {} buttons", settings.buttons.len());
                }
                _ => unreachable!(),
            },
        }
    }
    // SAFETY: owned settings and text buffers outlive all native callbacks.
    std::process::exit(unsafe { run_window(settings) });
}

// Canonical Cairo backend ABI; renderer logic remains local.
use cairo_sys::{cairo_surface_t, cairo_t};

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_button_press_event_t, xcb_button_release_event_t,
    xcb_change_property, xcb_charinfo_t, xcb_configure_notify_event_t, xcb_connect,
    xcb_connection_has_error, xcb_connection_t, xcb_create_window, xcb_cursor_context_free,
    xcb_cursor_context_new, xcb_cursor_context_t, xcb_cursor_load_cursor, xcb_cursor_t,
    xcb_destroy_window, xcb_disconnect, xcb_drawable_t, xcb_expose_event_t, xcb_flush,
    xcb_generate_id, xcb_generic_error_t, xcb_generic_event_t, xcb_get_geometry,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_input_focus,
    xcb_get_input_focus_reply, xcb_get_input_focus_reply_t, xcb_grab_server, xcb_intern_atom,
    xcb_intern_atom_cookie_t, xcb_intern_atom_reply, xcb_intern_atom_reply_t, xcb_map_window,
    xcb_query_font_reply_t, xcb_randr_get_crtc_info, xcb_randr_get_crtc_info_reply,
    xcb_randr_get_crtc_info_reply_t, xcb_randr_get_output_info, xcb_randr_get_output_info_reply,
    xcb_randr_get_output_info_reply_t, xcb_randr_get_output_primary,
    xcb_randr_get_output_primary_cookie_t, xcb_randr_get_output_primary_reply,
    xcb_randr_get_output_primary_reply_t, xcb_randr_get_screen_resources_current,
    xcb_randr_get_screen_resources_current_cookie_t, xcb_randr_get_screen_resources_current_reply,
    xcb_randr_get_screen_resources_current_reply_t, xcb_rectangle_t, xcb_screen_t,
    xcb_translate_coordinates, xcb_translate_coordinates_reply, xcb_translate_coordinates_reply_t,
    xcb_ungrab_server, xcb_visualid_t, xcb_visualtype_t, xcb_wait_for_event, xcb_window_t,
    XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE, XCB_CONFIGURE_NOTIFY, XCB_COPY_FROM_PARENT,
    XCB_CW_BACK_PIXEL, XCB_CW_CURSOR, XCB_CW_EVENT_MASK, XCB_EVENT_MASK_BUTTON_PRESS,
    XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_EXPOSURE, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    XCB_EXPOSE, XCB_NONE, XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_COPY_FROM_PARENT,
    XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

use startup_notification_sys::{
    sn_display_unref, sn_launchee_context_complete, sn_launchee_context_new_from_environment,
    sn_launchee_context_setup_window, sn_launchee_context_unref, sn_xcb_display_new, SnDisplay,
    SnLauncheeContext,
};

use libc::{execl, exit, fork, fprintf, free, open, printf, setsid, strlen, symlink, wait};

use i3_common::native_stdio::{err, errx, stderr, vfprintf, warn};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_process_filename::get_process_filename;
use i3_common::get_visualtype::get_visualtype;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MessageType {
    Error = 0,
    Warning = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> NagbarSettings {
        NagbarSettings {
            program: c"nagbar".to_owned(),
            font: c"fixed".to_owned(),
            message: c"<b>message</b>".to_owned(),
            warning: false,
            primary: false,
            buttons: [c"first", c"second"]
                .into_iter()
                .map(|label| ButtonSettings {
                    label: label.to_owned(),
                    action: Some(label.to_owned()),
                    terminal: false,
                })
                .collect(),
        }
    }

    #[test]
    fn owned_buttons_preserve_order_and_horizontal_boundary_rules() {
        let mut state = NagbarState::new(settings());
        state.close.x = 90;
        state.close.width = 10;
        assert!(!state.close.contains(89, false));
        assert!(state.close.contains(90, false));
        assert!(state.close.contains(99, false));
        assert!(!state.close.contains(100, false));
        for button in &mut state.buttons {
            button.x = -10;
            button.width = 20;
        }
        assert!(state.button_at(-11).is_none());
        assert_eq!(
            state.button_at(-10).unwrap().action.as_deref(),
            Some(c"first")
        );
        assert_eq!(
            state.button_at(10).unwrap().action.as_deref(),
            Some(c"first")
        );
        assert!(state.button_at(11).is_none());
        state.buttons[0].x = 100;
        assert_eq!(
            state.button_at(0).unwrap().action.as_deref(),
            Some(c"second")
        );
        state.buttons[0].x = i32::MAX - 1;
        state.buttons[0].width = 10;
        assert!(state.buttons[0].contains(i32::MAX, true));
        // Construction and destruction require no display or native text owner.
        assert_eq!(state.buttons.len(), 2);
    }

    #[test]
    fn checked_layout_handles_long_labels_and_offscreen_buttons() {
        assert_eq!(button_layout(50, 3, 500), (438, 62));
        assert_eq!(button_layout(50, 3, 10), (-52, 62));
        assert_eq!(button_layout(70_000, 3, 80_000), (9_988, 70_012));
        assert_eq!(button_layout(-10, -1, 50), (50, 0));
        assert_eq!(
            button_layout(i32::MAX, i32::MAX, i32::MIN),
            (i32::MIN, i32::MAX)
        );
    }

    #[test]
    fn command_bytes_keep_self_cleanup_and_delayed_terminal_handoff() {
        let path = CString::new(b"/persistent/nagbar-\xff".as_slice()).unwrap();
        let action = CString::new(b"printf '%s' '\xff';\necho done".as_slice()).unwrap();
        let mut expected = b"#!/bin/sh\nrm /persistent/nagbar-\xff\n".to_vec();
        expected.extend_from_slice(action.to_bytes());
        assert_eq!(command_script(&path, Some(&action)), expected);
        assert_eq!(
            command_script(c"path with spaces", None),
            b"#!/bin/sh\nrm path with spaces\n(null)"
        );
        assert_eq!(terminal_command(&path, false).to_bytes(), path.to_bytes());
        let mut expected = b"i3-sensible-terminal -e ".to_vec();
        expected.extend_from_slice(path.to_bytes());
        assert_eq!(terminal_command(&path, true).to_bytes(), expected);
        // These helpers only construct bytes; no script or user command is run.
    }
}
