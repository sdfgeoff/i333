use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::draw_util::draw_util_text;
use i3_common::draw_util::{OwnedWindow, RenderingContext};
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::string::I3String;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::fd::FromRawFd;
use std::rc::Rc;
use xcb::{randr, x, Xid, XidNew};
use xcb_interop::cursor::CursorContext;
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
use i3_common::get_exe_path::executable_path;
use std::os::unix::ffi::OsStrExt;

use i3_common::font::Font;

use i3_common::draw_util::color_t;
use i3_common::draw_util::surface_t;

pub static mut visual_type: *mut cairo_sys::xcb_visualtype_t = std::ptr::null_mut();

static mut root: u32 = 0;

pub static mut conn: *mut xcb_connection_t = std::ptr::null_mut();

pub static mut root_screen: Option<i3_common::display::DisplayScreen> = None;

/// Nagbar writes verbose and error records to stderr; debug stays suppressed.
pub fn write_log_bytes(level: i3_common::logging::Level, message: &[u8]) {
    use i3_common::logging::Level;
    unsafe {
        let stream = match level {
            Level::Error => stderr,
            Level::Verbose => stderr,
            Level::Debug => return,
        };
        libc::fwrite(message.as_ptr().cast(), message.len(), 1, stream);
    }
}

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

unsafe fn handle_button_release(state: &NagbarState, event: &x::ButtonReleaseEvent) -> bool {
    printf(
        c"button released on x = %d, y = %d\n".as_ptr(),
        i32::from(event.event_x()),
        i32::from(event.event_y()),
    );
    if state.close.contains(i32::from(event.event_x()), false) {
        return true;
    }
    let Some(button) = state.button_at(i32::from(event.event_x())) else {
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
    let exe_path = executable_path(std::ffi::OsStr::from_bytes(state.program.to_bytes()))
        .unwrap_or_else(|_| std::path::PathBuf::from("/usr/bin/i3-nagbar"));
    let exe_path =
        CString::new(exe_path.as_os_str().as_bytes()).expect("executable path contains no NUL");
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
    let text_width = predict_text_width(&render.font, &button.label).max(0);
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
        &render.font,
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
unsafe fn handle_expose(state: &mut NagbarState, connection: &xcb::Connection) {
    let render = &mut state.render;
    draw_util_clear_surface(&mut render.bar, render.color_background);
    let margin = logical_px(8);
    draw_util_text(
        &render.font,
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
    let _ = connection.flush();
}
unsafe fn set_window_position_primary(connection: &xcb::Connection, result: &mut x::Rectangle) {
    let root_window = x::Window::new(root);
    // Preserve the original request batching and reply order.
    let resources_cookie = connection.send_request(&randr::GetScreenResourcesCurrent {
        window: root_window,
    });
    let primary_cookie = connection.send_request(&randr::GetOutputPrimary {
        window: root_window,
    });
    let Ok(primary) = connection.wait_for_reply(primary_cookie) else {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not determine the primary output.\n");
        return;
    };
    let Ok(resources) = connection.wait_for_reply(resources_cookie) else {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not query screen resources.\n");
        return;
    };
    let output = connection.wait_for_reply(connection.send_request(&randr::GetOutputInfo {
        output: primary.output(),
        config_timestamp: resources.config_timestamp(),
    }));
    let Ok(output) = output else {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not query primary screen.\n");
        return;
    };
    if output.crtc().is_none() {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not query primary screen.\n");
        return;
    }
    let Ok(crtc) = connection.wait_for_reply(connection.send_request(&randr::GetCrtcInfo {
        crtc: output.crtc(),
        config_timestamp: resources.config_timestamp(),
    })) else {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not get CRTC.\n");
        return;
    };
    ::log::info!("[libi3] ../i3-nagbar/main.c Found primary output on position x = {} / y = {} / w = {} / h = {}.\n", (i32::from(crtc.x())) as i32, (i32::from(crtc.y())) as i32, (i32::from(crtc.width())) as i32, (i32::from(crtc.height())) as i32);
    if crtc.width() == 0 || crtc.height() == 0 {
        ::log::info!("[libi3] ../i3-nagbar/main.c Primary output is not active, ignoring it.\n");
    } else {
        result.x = crtc.x();
        result.y = crtc.y();
    }
}

struct ServerGrab<'a>(&'a xcb::Connection);
impl Drop for ServerGrab<'_> {
    fn drop(&mut self) {
        self.0.send_request(&x::UngrabServer {});
    }
}

unsafe fn set_window_position_focus(connection: &xcb::Connection, result: &mut x::Rectangle) {
    connection.send_request(&x::GrabServer {});
    let server_grab = ServerGrab(connection);
    let position = (|| {
        let Ok(input_focus) =
            connection.wait_for_reply(connection.send_request(&x::GetInputFocus {}))
        else {
            ::log::info!("[libi3] ../i3-nagbar/main.c Failed to receive the current input focus or no window has the input focus right now.\n");
            return None;
        };
        if input_focus.focus().is_none() {
            ::log::info!("[libi3] ../i3-nagbar/main.c Failed to receive the current input focus or no window has the input focus right now.\n");
            return None;
        }
        let Ok(geometry) = connection.wait_for_reply(connection.send_request(&x::GetGeometry {
            drawable: x::Drawable::Window(input_focus.focus()),
        })) else {
            ::log::info!("[libi3] ../i3-nagbar/main.c Failed to received window geometry.\n");
            return None;
        };
        let Ok(coordinates) =
            connection.wait_for_reply(connection.send_request(&x::TranslateCoordinates {
                src_window: input_focus.focus(),
                dst_window: x::Window::new(root),
                src_x: geometry.x(),
                src_y: geometry.y(),
            }))
        else {
            ::log::info!("[libi3] ../i3-nagbar/main.c Failed to translate coordinates.\n");
            return None;
        };
        ::log::info!(
            "[libi3] ../i3-nagbar/main.c Found current focus at x = {} / y = {}.\n",
            (i32::from(coordinates.dst_x())) as i32,
            (i32::from(coordinates.dst_y())) as i32
        );
        Some((coordinates.dst_x(), coordinates.dst_y()))
    })();
    drop(server_grab);
    if let Some((x, y)) = position {
        result.x = x;
        result.y = y;
    } else {
        ::log::info!("[libi3] ../i3-nagbar/main.c Could not position on focused output, trying to position on primary output.\n");
        set_window_position_primary(connection, result);
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
    registration: Option<i3_common::display::DisplayConnection>,
    connection: Option<Rc<xcb::Connection>>,
    rendering: Option<Rc<RenderingContext>>,
    window_owner: Option<OwnedWindow>,
    window: u32,
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
                bar: surface_t::empty(),
                font: Font::empty(),
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
            registration: None,
            connection: None,
            rendering: None,
            window_owner: None,
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
            if let Some(connection) = self.connection.as_ref() {
                if self.has_surface {
                    self.render.bar.clear();
                }
                if self.has_font {
                    self.render.font = Font::empty();
                }
                if let Some(window) = self.window_owner.as_mut() {
                    window.close();
                }
                drop(self.window_owner.take());
                self.window = 0;
                // Cairo/font references are gone before cache GC owners.
                drop(self.rendering.take());
                root_screen = None;
                visual_type = std::ptr::null_mut();
                root = 0;
                conn = std::ptr::null_mut();
                // Registry and owning connection drop after native teardown.
            }
        }
    }
}

unsafe fn atom_reply(
    connection: &xcb::Connection,
    cookie: x::InternAtomCookie,
    name: &CStr,
) -> x::Atom {
    let Ok(reply) = connection.wait_for_reply(cookie) else {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom %s".as_ptr(),
            name.as_ptr(),
        );
    };
    reply.atom()
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
    // The session owns its font and backing surface through display teardown.
    // Rendering borrows the font explicitly for each operation.
    let mut state = Box::new(NagbarState::new(settings));
    let Ok((connection, screens)) = xcb::Connection::connect(None) else {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    };
    let connection = Rc::new(connection);
    conn = connection.get_raw_conn().cast();
    let registration = i3_common::display::install_connection(connection.clone());
    state.registration = Some(registration);
    state.connection = Some(connection.clone());
    let intern = |name: &CStr| {
        connection.send_request(&x::InternAtom {
            only_if_exists: false,
            name: &name.to_bytes()[..name.to_bytes().len() as u16 as usize],
        })
    };
    let window_type_cookie = intern(c"_NET_WM_WINDOW_TYPE");
    let dock_cookie = intern(c"_NET_WM_WINDOW_TYPE_DOCK");
    let strut_cookie = intern(c"_NET_WM_STRUT_PARTIAL");
    let socket_cookie = intern(c"I3_SOCKET_PATH");
    let atom_cookie = intern(c"ATOM");
    let cardinal_cookie = intern(c"CARDINAL");
    let sndisplay: *mut SnDisplay = sn_xcb_display_new(conn.cast(), None, None);
    let sncontext: *mut SnLauncheeContext =
        sn_launchee_context_new_from_environment(sndisplay, screens);
    sn_display_unref(sndisplay);
    root_screen = i3_common::display::DisplayScreen::new(connection.clone(), screens);
    if root_screen.is_none() {
        errx(libc::EXIT_FAILURE, c"Cannot find display screen".as_ptr());
    }
    let screen_info = root_screen
        .as_ref()
        .expect("initialized display screen")
        .info();
    root = screen_info.root;
    state.rendering = Some(RenderingContext::new(
        root_screen
            .as_ref()
            .expect("initialized display screen")
            .clone(),
    ));
    visual_type = i3_common::display::visual(screen_info.root_visual)
        .expect("root Cairo visual")
        .as_ptr();
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
    let font_context = i3_common::font::FontContext::for_screen(
        root_screen.as_ref().expect("initialized display screen"),
    );
    state.render.font = load_font(&font_context, &pattern, true);
    state.has_font = true;

    let mut win_pos = x::Rectangle {
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
        set_window_position_primary(&connection, &mut win_pos);
    } else {
        set_window_position_focus(&connection, &mut win_pos);
    }
    let cursor = {
        let context = CursorContext::new(connection.clone(), screens).unwrap_or_else(|_| {
            errx(
                libc::EXIT_FAILURE,
                c"Cannot allocate xcursor context".as_ptr(),
            )
        });
        context.load(c"left_ptr")
    };
    let window: x::Window = connection.generate_id();
    state.window = window.resource_id();
    connection.send_request(&x::CreateWindow {
        depth: 0,
        wid: window,
        parent: x::Window::new(root),
        x: win_pos.x,
        y: win_pos.y,
        width: win_pos.width,
        height: win_pos.height,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: 0,
        value_list: &[
            x::Cw::BackPixel(0),
            x::Cw::EventMask(
                x::EventMask::EXPOSURE
                    | x::EventMask::STRUCTURE_NOTIFY
                    | x::EventMask::BUTTON_PRESS
                    | x::EventMask::BUTTON_RELEASE,
            ),
            x::Cw::Cursor(cursor),
        ],
    });
    // Unique leaf XID; preserve the unchecked create/error route and the exact
    // former explicit close position in the session destructor.
    state.window_owner = Some(OwnedWindow::from_created(connection.clone(), window));
    connection.send_request(&x::FreeCursor { cursor });
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, state.window as libc::c_ulong);
    }
    connection.send_request(&x::MapWindow { window });
    let window_type = atom_reply(&connection, window_type_cookie, c"_NET_WM_WINDOW_TYPE");
    let dock = atom_reply(&connection, dock_cookie, c"_NET_WM_WINDOW_TYPE_DOCK");
    let strut = atom_reply(&connection, strut_cookie, c"_NET_WM_STRUT_PARTIAL");
    let _socket = atom_reply(&connection, socket_cookie, c"I3_SOCKET_PATH");
    let atom = atom_reply(&connection, atom_cookie, c"ATOM");
    let cardinal = atom_reply(&connection, cardinal_cookie, c"CARDINAL");
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: window_type,
        r#type: atom,
        data: &[dock.resource_id()],
    });
    let mut strut_partial = [0u32; 12];
    strut_partial[2] = state
        .render
        .font
        .height
        .saturating_add(logical_px(6))
        .max(0) as u32;
    strut_partial[9] = 800;
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: strut,
        r#type: cardinal,
        data: &strut_partial,
    });
    let rendering = state.rendering.as_ref().expect("session rendering context");
    let visual = rendering
        .visual(rendering.screen().info().root_visual)
        .expect("root visual belongs to the selected screen");
    state.render.bar = surface_t::for_drawable(
        rendering.clone(),
        state.window as u32,
        visual,
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    state.has_surface = true;
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
        sn_launchee_context_unref(sncontext);
    }
    let _ = connection.flush();
    loop {
        let event = match connection.wait_for_event() {
            Ok(event) => event,
            Err(xcb::Error::Protocol(error)) => {
                fprintf(
                    stderr,
                    c"X11 Error received! sequence %x\n".as_ptr(),
                    (*error.as_raw()).sequence as i32,
                );
                continue;
            }
            Err(xcb::Error::Connection(_)) => break,
        };
        match event {
            xcb::Event::X(x::Event::Expose(event)) => {
                if event.count() == 0 {
                    handle_expose(&mut state, &connection);
                }
            }
            xcb::Event::X(x::Event::ButtonPress(event)) => {
                printf(
                    c"button pressed on x = %d, y = %d\n".as_ptr(),
                    i32::from(event.event_x()),
                    i32::from(event.event_y()),
                );
            }
            xcb::Event::X(x::Event::ButtonRelease(event)) => {
                if handle_button_release(&state, &event) {
                    return 0;
                }
            }
            xcb::Event::X(x::Event::ConfigureNotify(event)) => {
                if event.width() > 0 && event.height() > 0 {
                    draw_util_surface_set_size(
                        &mut state.render.bar,
                        i32::from(event.width()),
                        i32::from(event.height()),
                    );
                }
            }
            _ => {}
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

use xcb::ffi::xcb_connection_t;

use startup_notification_sys::{
    sn_display_unref, sn_launchee_context_complete, sn_launchee_context_new_from_environment,
    sn_launchee_context_setup_window, sn_launchee_context_unref, sn_xcb_display_new, SnDisplay,
    SnLauncheeContext,
};

use libc::{execl, exit, fork, fprintf, free, open, printf, setsid, strlen, symlink, wait};

use i3_common::native_stdio::{err, errx, stderr, warn};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_process_filename::get_process_filename;

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
