use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::string::I3String;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::fd::FromRawFd;
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use xcb::{x, Xid, XidNew};
use xcb_interop::events::{key_press_state, key_release_state};
use xcb_interop::keysyms::KeySymbols;
use xcb_interop::transport::keyboard_grab_status;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-input/main.c: Utility which lets the user input commands and sends them
 *                  to i3.
 *
 */
// Mechanical file-by-file Rust translation of i3-input/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;

pub use i3_common::font::Font;

pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;

pub static mut visual_type: *mut cairo_sys::xcb_visualtype_t =
    ::core::ptr::null::<cairo_sys::xcb_visualtype_t>() as *mut cairo_sys::xcb_visualtype_t;
pub const EXIT_OK: i32 = 0;
pub const EXIT_CANCEL: i32 = 1;
pub const EXIT_ERROR: i32 = 2;
static mut win: u32 = 0;
static mut surface: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut font: Font = Font::empty();

pub static mut root: u32 = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut root_screen: Option<i3_common::display::DisplayScreen> = None;

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {}
/// One accepted key has an owned scalar and its historical X core-font code.
#[derive(Clone, Copy, Debug)]
struct Glyph {
    ucs2: xcb_char2b_t,
    character: char,
}
impl Glyph {
    fn from_keysym(sym: u32) -> Option<Self> {
        let ucs = super::keysym2ucs::keysym2ucs(sym)? as u16;
        if ucs == u16::MAX {
            return None;
        }
        // Preserve upstream's byte packing (>> 2, not >> 8). Changing this
        // while replacing storage would change existing non-Latin input.
        let ucs2 = xcb_char2b_t {
            byte1: ((ucs & 0xff00) >> 2) as u8,
            byte2: (ucs & 0xff) as u8,
        };
        // That packing produces high bytes 00/40/80/c0, never a surrogate.
        let character = char::from_u32(u32::from(ucs2.code_unit()))
            .expect("legacy key packing always produces a Unicode scalar");
        Some(Self { ucs2, character })
    }

    fn bytes<'a>(&self, buffer: &'a mut [u8; 4]) -> &'a [u8] {
        self.character.encode_utf8(buffer).as_bytes()
    }
}

/// All editing/text state belongs to the running window, not mutable globals.
struct InputState {
    glyphs: Vec<Glyph>,
    format: CString,
    prompt: Option<I3String>,
    prompt_offset: i32,
    limit: i32,
    modeswitch_active: bool,
}
impl InputState {
    fn new(settings: &InputSettings) -> Self {
        Self {
            glyphs: Vec::new(),
            format: settings.format.clone(),
            prompt: settings.prompt.as_deref().map(I3String::from_cstr),
            prompt_offset: 0,
            limit: settings.limit,
            modeswitch_active: false,
        }
    }

    fn push(&mut self, glyph: Glyph) -> bool {
        self.glyphs.push(glyph);
        usize::try_from(self.limit).ok() == Some(self.glyphs.len())
    }

    fn backspace(&mut self) -> bool {
        self.glyphs.pop().is_some()
    }

    fn input(&self) -> CString {
        // The old concatenation copied NUL glyphs too, but strlen projected the
        // command only up to the first NUL. Keep that projection without a C walk.
        let text: String = self
            .glyphs
            .iter()
            .map(|glyph| glyph.character)
            .take_while(|character| *character != '\0')
            .collect();
        CString::new(text).expect("the projected input contains no NUL")
    }

    fn command(&self) -> CString {
        i3_common::format_placeholders::expand_cstr(
            &self.format,
            &[(c"%s", self.input().as_c_str())],
        )
    }
}

/// Borrow only a property payload whose native declared byte length fits.
fn property_bytes(reply: &x::GetPropertyReply) -> Option<&[u8]> {
    let raw = unsafe { xcb::Reply::as_raw(reply) };
    let values = unsafe { raw.add(16).cast::<u32>().read_unaligned() };
    // Preserve native unsigned multiplication followed by its signed-int result.
    let length = usize::try_from(values.wrapping_mul(u32::from(reply.format()) / 8) as i32).ok()?;
    if length > (reply.length() as usize).checked_mul(4)? {
        return None;
    }
    Some(unsafe { std::slice::from_raw_parts(raw.add(32), length) })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KeyAction {
    Continue,
    Finish,
    Cancel,
}

fn write_line(output: &mut impl Write, prefix: &[u8], value: &[u8]) {
    // Debug output failures never prevented command submission in upstream.
    let _ = output.write_all(prefix);
    let _ = output.write_all(value);
    let _ = output.write_all(b"\n");
}

fn finish_input(state: &InputState, socket: &mut impl Write) -> i32 {
    let mut output = std::io::stdout().lock();
    for glyph in &state.glyphs {
        let mut encoded = [0; 4];
        write_line(&mut output, b"at ", &glyph.bytes(&mut encoded)[..1]);
    }
    let input = state.input();
    write_line(&mut output, b"output = ", input.to_bytes());
    let occurrences = state
        .format
        .to_bytes()
        .windows(2)
        .filter(|part| *part == b"%s")
        .count();
    let _ = writeln!(output, "occurrences = {occurrences}");
    let command = state.command();
    write_line(&mut output, b"command = ", command.to_bytes());
    if i3_common::ipc_send_message::send_message(socket, 0, command.to_bytes()).is_ok() {
        EXIT_OK
    } else {
        EXIT_ERROR
    }
}

unsafe fn handle_expose(
    state: &mut InputState,
    connection: &xcb::Connection,
) -> ::core::ffi::c_int {
    printf(c"expose!\n".as_ptr());
    let border_color: color_t = draw_util_hex_to_color(c"#FF0000".as_ptr());
    let fg_color: color_t = draw_util_hex_to_color(c"#FFFFFF".as_ptr());
    let bg_color: color_t = draw_util_hex_to_color(c"#000000".as_ptr());
    let text_offset: ::core::ffi::c_int =
        logical_px(2 as ::core::ffi::c_int) + logical_px(2 as ::core::ffi::c_int);
    draw_util_rectangle(
        &raw mut surface,
        border_color,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        surface.width as ::core::ffi::c_double,
        surface.height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        &raw mut surface,
        bg_color,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        (surface.width - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (surface.height - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    if let Some(prompt) = state.prompt.as_ref() {
        draw_util_text(
            &font,
            prompt,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset,
            text_offset,
            logical_px(500 as ::core::ffi::c_int) - text_offset,
        );
    }
    if !state.glyphs.is_empty() {
        let glyphs: Vec<_> = state.glyphs.iter().map(|glyph| glyph.ucs2).collect();
        let input = I3String::from_ucs2(&glyphs);
        draw_util_text(
            &font,
            &input,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset + state.prompt_offset,
            text_offset,
            logical_px(500) - text_offset,
        );
    }
    let _ = connection.flush();
    1 as ::core::ffi::c_int
}
unsafe fn handle_key_release(
    state: &mut InputState,
    symbols: &KeySymbols,
    event: &x::KeyReleaseEvent,
) {
    printf(
        c"releasing %d, state raw = %d\n".as_ptr(),
        event.detail() as libc::c_int,
        key_release_state(event) as libc::c_int,
    );
    let sym = symbols.keysym(event.detail(), key_release_state(event) as i32);
    if sym == xkeysym::key::Mode_switch {
        printf(c"Mode switch disabled\n".as_ptr());
        state.modeswitch_active = false;
    }
}
unsafe fn handle_key_press(
    state: &mut InputState,
    connection: &xcb::Connection,
    symbols: &KeySymbols,
    event: &x::KeyPressEvent,
) -> KeyAction {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        event.detail() as ::core::ffi::c_int,
        key_press_state(event) as ::core::ffi::c_int,
    );
    let mut col: ::core::ffi::c_int =
        i32::from(key_press_state(event) & x::KeyButMask::SHIFT.bits() as u16);
    if state.modeswitch_active {
        col += 2 as ::core::ffi::c_int;
    }
    let sym = symbols.keysym(event.detail(), col);
    if sym == xkeysym::key::Mode_switch {
        printf(c"Mode switch enabled\n".as_ptr());
        state.modeswitch_active = true;
        return KeyAction::Continue;
    }
    if sym == xkeysym::key::Return {
        return KeyAction::Finish;
    }
    if sym == xkeysym::key::BackSpace {
        if state.backspace() {
            handle_expose(state, connection);
        }
        return KeyAction::Continue;
    }
    if sym == xkeysym::key::Escape {
        return KeyAction::Cancel;
    }
    printf(
        c"is_keypad_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_keypad_key()),
    );
    printf(
        c"is_private_keypad_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_private_keypad_key()),
    );
    printf(
        c"xcb_is_cursor_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_cursor_key()),
    );
    printf(
        c"xcb_is_pf_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_pf_key()),
    );
    printf(
        c"xcb_is_function_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_function_key()),
    );
    printf(
        c"xcb_is_misc_function_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_misc_function_key()),
    );
    printf(
        c"xcb_is_modifier_key = %d\n".as_ptr(),
        i32::from(xkeysym::Keysym::new(sym).is_modifier_key()),
    );
    if xkeysym::Keysym::new(sym).is_modifier_key() || xkeysym::Keysym::new(sym).is_cursor_key() {
        return KeyAction::Continue;
    }
    printf(c"sym = %c (%d)\n".as_ptr(), sym, sym);
    let Some(glyph) = Glyph::from_keysym(sym) else {
        fprintf(
            stderr,
            c"Keysym could not be converted to UCS, skipping\n".as_ptr(),
        );
        return KeyAction::Continue;
    };
    let inp = glyph.ucs2;
    printf(
        c"inp.byte1 = %02x, inp.byte2 = %02x\n".as_ptr(),
        inp.byte1 as ::core::ffi::c_int,
        inp.byte2 as ::core::ffi::c_int,
    );
    let mut encoded = [0; 4];
    let bytes = glyph.bytes(&mut encoded);
    let mut terminated = [0u8; 5];
    terminated[..bytes.len()].copy_from_slice(bytes);
    printf(
        c"converted to %s\n".as_ptr(),
        terminated.as_ptr().cast::<libc::c_char>(),
    );
    if state.push(glyph) {
        return KeyAction::Finish;
    }
    handle_expose(state, connection);
    KeyAction::Continue
}
/// A server grab must be released on every return from position lookup.
struct ServerGrab<'a>(&'a xcb::Connection);
impl Drop for ServerGrab<'_> {
    fn drop(&mut self) {
        self.0.send_request(&x::UngrabServer {});
        let _ = self.0.flush();
    }
}

unsafe fn get_window_position(connection: &xcb::Connection) -> x::Rectangle {
    let mut result = x::Rectangle {
        x: logical_px(50) as i16,
        y: logical_px(50) as i16,
        width: logical_px(500) as u16,
        height: (font.height + 4 * logical_px(2)) as u16,
    };
    let cookie = connection.send_request(&x::InternAtom {
        only_if_exists: false,
        name: b"_NET_SUPPORTING_WM_CHECK",
    });
    let Ok(atom) = connection.wait_for_reply(cookie) else {
        errorlog(
            c"[libi3] ERROR: Could not intern atom _NET_SUPPORTING_WM_CHECK\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_ERROR);
    };
    let root_window = x::Window::new(root);
    let supporting = connection.wait_for_reply(connection.send_request(&x::GetProperty {
        delete: false,
        window: root_window,
        property: atom.atom(),
        r#type: x::ATOM_WINDOW,
        long_offset: 0,
        long_length: 32,
    }));
    let supporting_window = supporting.as_ref().ok().and_then(|reply| {
        Some(u32::from_ne_bytes(
            property_bytes(reply)?.get(..4)?.try_into().ok()?,
        ))
    });
    connection.send_request(&x::GrabServer {});
    let _server_grab = ServerGrab(connection);
    let Ok(focus) = connection.wait_for_reply(connection.send_request(&x::GetInputFocus {})) else {
        return result;
    };
    let focused = focus.focus();
    if focused.is_none() || Some(focused.resource_id()) == supporting_window {
        return result;
    }
    let Ok(geometry) = connection.wait_for_reply(connection.send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(focused),
    })) else {
        return result;
    };
    let wm_class = connection.wait_for_reply(connection.send_request(&x::GetProperty {
        delete: false,
        window: focused,
        property: x::ATOM_WM_CLASS,
        r#type: x::ATOM_ANY,
        long_offset: 0,
        long_length: 32,
    }));
    let is_frame = wm_class
        .as_ref()
        .ok()
        .and_then(property_bytes)
        .is_some_and(|bytes| bytes.split(|byte| *byte == 0).next() == Some(b"i3-frame".as_slice()));
    let position = if is_frame {
        (geometry.x(), geometry.y())
    } else {
        let Ok(coordinates) =
            connection.wait_for_reply(connection.send_request(&x::TranslateCoordinates {
                src_window: focused,
                dst_window: root_window,
                src_x: geometry.x(),
                src_y: geometry.y(),
            }))
        else {
            return result;
        };
        (coordinates.dst_x(), coordinates.dst_y())
    };
    result.x = result.x.wrapping_add(position.0);
    result.y = result.y.wrapping_add(position.1);
    result
}
#[derive(Debug)]
struct InputSettings {
    socket: Option<std::ffi::CString>,
    font: Option<std::ffi::CString>,
    prompt: Option<std::ffi::CString>,
    format: std::ffi::CString,
    limit: i32,
}
impl Default for InputSettings {
    fn default() -> Self {
        Self {
            socket: None,
            font: None,
            prompt: None,
            format: c"%s".to_owned(),
            limit: 0,
        }
    }
}
const INPUT_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::value(b's', "socket"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::value(b'l', "limit"),
    i3_common::cli::OptionSpec::value(b'P', "prompt"),
    i3_common::cli::OptionSpec::value(b'p', "prefix"),
    i3_common::cli::OptionSpec::value(b'F', "format"),
    i3_common::cli::OptionSpec::value(b'f', "font"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
];

/// Session-owned native resources. Only the display-provider globals borrow them.
struct WindowResources {
    _registration: i3_common::display::DisplayConnection,
    connection: Rc<xcb::Connection>,
    symbols: Option<KeySymbols>,
    has_font: bool,
    has_surface: bool,
}
impl Drop for WindowResources {
    fn drop(&mut self) {
        // SAFETY: run_window is the sole event-thread owner. The loop and its
        // borrowed events have finished before this guard drops. Font/surface
        // destruction still needs the live connection and display globals.
        unsafe {
            if self.has_surface {
                draw_util_surface_free(self.connection.get_raw_conn().cast(), &raw mut surface);
            }
            if self.has_font {
                font = Font::empty();
            }
            drop(self.symbols.take());
            root_screen = None;
            visual_type = std::ptr::null_mut();
            root = 0;
            conn = std::ptr::null_mut();
            // The registry and connection fields drop after native teardown.
        }
    }
}

unsafe fn run_window(settings: InputSettings) -> ::core::ffi::c_int {
    let socket_path = settings
        .socket
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    let pattern = settings
        .font
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    let mut state = InputState::new(&settings);
    printf(c"using format \"%s\"\n".as_ptr(), state.format.as_ptr());
    let Ok((connection, screen)) = xcb::Connection::connect(None) else {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    };
    let connection = Rc::new(connection);
    conn = connection.get_raw_conn().cast();
    let registration = i3_common::display::install_connection(connection.clone());
    let mut resources = WindowResources {
        _registration: registration,
        connection,
        symbols: None,
        has_font: false,
        has_surface: false,
    };
    // SAFETY: ipc_connect either returns its newly owned connected descriptor
    // or exits with the historical fatal diagnostic; no other owner remains.
    let mut socket = UnixStream::from_raw_fd(ipc_connect(socket_path));
    root_screen = i3_common::display::DisplayScreen::new(resources.connection.clone(), screen);
    let screen_info = root_screen
        .as_ref()
        .expect("initialized display screen")
        .info();
    root = screen_info.root;
    visual_type = i3_common::display::visual(screen_info.root_visual)
        .expect("root Cairo visual")
        .as_ptr();
    resources.symbols = Some(
        KeySymbols::new(resources.connection.clone())
            .unwrap_or_else(|| errx(EXIT_ERROR, c"Could not allocate key symbols".as_ptr())),
    );
    init_dpi();
    font = load_font(
        CStr::from_ptr(if !pattern.is_null() {
            pattern as *const ::core::ffi::c_char
        } else {
            c"pango:monospace 8".as_ptr()
        }),
        true,
    );
    resources.has_font = true;

    if let Some(prompt) = state.prompt.as_ref() {
        state.prompt_offset = predict_text_width(&font, prompt);
    }
    let connection = &*resources.connection;
    let win_pos = get_window_position(connection);
    let window: x::Window = connection.generate_id();
    win = window.resource_id();
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
            x::Cw::OverrideRedirect(true),
            x::Cw::EventMask(x::EventMask::EXPOSURE),
        ],
    });
    connection.send_request(&x::MapWindow { window });
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as u32,
        None,
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    let _ = connection.flush();
    resources.has_surface = true;
    let mut grab_status = None;
    for _ in 0..500 {
        let cookie = connection.send_request(&x::GrabKeyboard {
            owner_events: false,
            grab_window: window,
            time: x::CURRENT_TIME,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        });
        let reply = connection.wait_for_reply(cookie);
        std::thread::sleep(std::time::Duration::from_millis(1));
        grab_status = reply.as_ref().ok().map(keyboard_grab_status);
        if grab_status == Some(Ok(x::GrabStatus::Success)) {
            break;
        }
    }
    if grab_status != Some(Ok(x::GrabStatus::Success)) {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            grab_status.map_or(-1, |status| {
                status.map_or_else(i32::from, |status| status as i32)
            }),
        );
        return EXIT_ERROR;
    }
    let _ = connection.flush();
    let symbols = resources
        .symbols
        .as_ref()
        .expect("input key-symbol cache initialized");
    loop {
        let event = match connection.wait_for_event() {
            Ok(event) => event,
            Err(xcb::Error::Protocol(error)) => {
                // The maintained error owns its native header. Preserve the
                // original 16-bit sequence diagnostic and continue processing.
                fprintf(
                    stderr,
                    c"X11 Error received! sequence %x\n".as_ptr(),
                    (*error.as_raw()).sequence as libc::c_int,
                );
                continue;
            }
            Err(xcb::Error::Connection(_)) => return EXIT_OK,
        };
        let action = match event {
            xcb::Event::X(x::Event::KeyPress(event)) => {
                handle_key_press(&mut state, connection, symbols, &event)
            }
            xcb::Event::X(x::Event::KeyRelease(event)) => {
                handle_key_release(&mut state, symbols, &event);
                KeyAction::Continue
            }
            xcb::Event::X(x::Event::Expose(event)) => {
                if event.count() == 0 {
                    handle_expose(&mut state, connection);
                }
                KeyAction::Continue
            }
            _ => KeyAction::Continue,
        };
        match action {
            KeyAction::Continue => {}
            KeyAction::Cancel => return EXIT_CANCEL,
            KeyAction::Finish => {
                libc::fflush(stdout);
                return finish_input(&state, &mut socket);
            }
        }
    }
}

pub fn main() {
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::OsStrExt;
    let mut arguments = std::env::args_os();
    let program = arguments.next().unwrap_or_else(|| "i3-input".into());
    let mut settings = InputSettings::default();
    let parser = Parser::new(
        arguments,
        INPUT_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    );
    for item in parser {
        match item {
            Item::Error(message) => eprintln!("{}: {message}", program.to_string_lossy()),
            Item::Operand(_) => {}
            Item::Option { short, value } => match short {
                b'v' => {
                    println!("i3-input {}", env!("I3_VERSION"));
                    return;
                }
                b'h' => {
                    println!("i3-input {}\ni3-input [-s <socket>] [-F <format>] [-l <limit>] [-P <prompt>] [-f <font>] [-v]\n\nExample:\n    i3-input -F 'workspace \"%s\"' -P 'Switch to workspace: '", env!("I3_VERSION"));
                    return;
                }
                b'l' => settings.limit = i3_common::cli::decimal_prefix(value.unwrap().as_bytes()),
                short => {
                    let mut bytes = value.unwrap().as_bytes().to_vec();
                    if short == b'p' {
                        eprintln!("i3-input: WARNING: the -p option is DEPRECATED in favor of the -F (format) option");
                        bytes.extend_from_slice(b"%s");
                    }
                    let value =
                        std::ffi::CString::new(bytes).expect("OS arguments have no NUL bytes");
                    match short {
                        b's' => settings.socket = Some(value),
                        b'P' => settings.prompt = Some(value),
                        b'f' => settings.font = Some(value),
                        b'F' | b'p' => settings.format = value,
                        _ => unreachable!(),
                    }
                }
            },
        }
    }
    // SAFETY: settings owns all C string arguments for the full event-loop
    // lifetime. Native X callbacks continue to follow their existing contracts.
    std::process::exit(unsafe { run_window(settings) });
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::dpi::init_dpi;
use i3_common::dpi::logical_px;

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{cairo_surface_t, cairo_t};

// Canonical Pango backend ABI; font policy remains local.

use xcb::ffi::xcb_connection_t;

use libc::{exit, fprintf, printf};

use i3_common::native_stdio::{errx, stderr, stdout, vfprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::ipc_connect::ipc_connect;

pub use i3_common::ucs2_conversion::Ucs2Char as xcb_char2b_t;

#[cfg(test)]
mod tests {
    use super::*;

    fn state(format: &CStr, limit: i32) -> InputState {
        InputState::new(&InputSettings {
            format: format.to_owned(),
            limit,
            ..InputSettings::default()
        })
    }

    fn append(state: &mut InputState, keysym: u32) -> bool {
        state.push(Glyph::from_keysym(keysym).expect("test keysym is representable"))
    }

    #[test]
    fn unlimited_input_and_explicit_limits_can_cross_old_512_slot_boundary() {
        let mut unlimited = state(c"%s", 0);
        for _ in 0..1025 {
            assert!(!append(&mut unlimited, b'a'.into()));
        }
        assert_eq!(unlimited.input().to_bytes(), vec![b'a'; 1025]);
        assert!(unlimited.backspace());
        assert_eq!(unlimited.glyphs.len(), 1024);

        let mut limited = state(c"%s", 513);
        for index in 0..513 {
            assert_eq!(append(&mut limited, b'b'.into()), index == 512);
        }
        let mut negative = state(c"%s", -1);
        assert!(!append(&mut negative, b'a'.into()));
    }

    #[test]
    fn backspace_removes_whole_multibyte_glyphs_and_empty_input_is_safe() {
        let mut input = state(c"%s", 0);
        assert!(!input.backspace());
        append(&mut input, b'a'.into());
        append(&mut input, 0x00e9);
        assert_eq!(input.input().to_bytes(), "aé".as_bytes());
        assert!(input.backspace());
        assert_eq!(input.input().to_bytes(), b"a");
        assert!(input.backspace());
        assert_eq!(input.command().to_bytes(), b"");
    }

    #[test]
    fn unicode_keys_retain_historical_ucs2_packing_and_rejection() {
        let latin = Glyph::from_keysym(0x00e9).unwrap();
        assert_eq!(latin.ucs2.code_unit(), 0x00e9);
        let greek = Glyph::from_keysym(0x0100_03b1).unwrap();
        assert_eq!(greek.ucs2.code_unit(), 0xc0b1);
        let supplementary = Glyph::from_keysym(0x0101_f642).unwrap();
        assert_eq!(supplementary.ucs2.code_unit(), 0x8042);
        assert!(Glyph::from_keysym(0x0100_ffff).is_none());
        assert!(Glyph::from_keysym(0x00ff_ffff).is_none());
    }

    #[test]
    fn format_expansion_preserves_percent_rules_raw_bytes_and_nul_projection() {
        let mut input = state(c"[%s] %%s %s%", 0);
        append(&mut input, 0x00e9);
        assert_eq!(input.command().to_bytes(), "[é] %é é%".as_bytes());
        input.format = CString::new(vec![0xff, b'%', b's']).unwrap();
        assert_eq!(input.command().to_bytes(), &[0xff, 0xc3, 0xa9]);
        append(&mut input, 0x0100_0000);
        append(&mut input, b'z'.into());
        assert_eq!(input.input().to_bytes(), "é".as_bytes());
        input.format = c"".to_owned();
        assert!(input.command().to_bytes().is_empty());
    }

    #[test]
    fn property_views_reject_truncated_values_and_allow_nonterminated_classes() {
        unsafe {
            let pointer = libc::calloc(1, 36).cast::<u8>();
            assert!(!pointer.is_null());
            pointer.write(1);
            pointer.add(1).write(8);
            pointer.add(4).cast::<u32>().write_unaligned(1);
            pointer.add(16).cast::<u32>().write_unaligned(8);
            let reply: x::GetPropertyReply = xcb::Reply::from_raw(pointer);
            assert!(property_bytes(&reply).is_none());
            pointer.add(16).cast::<u32>().write_unaligned(4);
            std::ptr::copy_nonoverlapping(b"name".as_ptr(), pointer.add(32), 4);
            assert_eq!(property_bytes(&reply), Some(b"name".as_slice()));
        }
    }

    #[test]
    fn closed_keyboard_grab_statuses_preserve_unknown_native_values() {
        for code in 0..=u8::MAX {
            unsafe {
                let pointer = libc::calloc(1, 32).cast::<u8>();
                assert!(!pointer.is_null());
                pointer.write(1);
                pointer.add(1).write(code);
                let reply: x::GrabKeyboardReply = xcb::Reply::from_raw(pointer);
                let status = keyboard_grab_status(&reply);
                assert_eq!(status.map_or_else(|code| code, |status| status as u8), code);
                assert_eq!(status.is_ok(), code <= 4);
            }
        }
    }

    // Independent native utility oracle; no protocol records or production
    // bindings are needed for these seven scalar classification functions.
    mod native_categories {
        #[link(name = "xcb-keysyms")]
        unsafe extern "C" {
            pub fn xcb_is_keypad_key(keysym: u32) -> i32;
            pub fn xcb_is_private_keypad_key(keysym: u32) -> i32;
            pub fn xcb_is_cursor_key(keysym: u32) -> i32;
            pub fn xcb_is_pf_key(keysym: u32) -> i32;
            pub fn xcb_is_function_key(keysym: u32) -> i32;
            pub fn xcb_is_misc_function_key(keysym: u32) -> i32;
            pub fn xcb_is_modifier_key(keysym: u32) -> i32;
        }
    }

    #[test]
    fn rust_keysym_categories_match_native_legacy_and_private_ranges() {
        let native: [unsafe extern "C" fn(u32) -> i32; 7] = [
            native_categories::xcb_is_keypad_key,
            native_categories::xcb_is_private_keypad_key,
            native_categories::xcb_is_cursor_key,
            native_categories::xcb_is_pf_key,
            native_categories::xcb_is_function_key,
            native_categories::xcb_is_misc_function_key,
            native_categories::xcb_is_modifier_key,
        ];
        let rust: [fn(xkeysym::Keysym) -> bool; 7] = [
            xkeysym::Keysym::is_keypad_key,
            xkeysym::Keysym::is_private_keypad_key,
            xkeysym::Keysym::is_cursor_key,
            xkeysym::Keysym::is_pf_key,
            xkeysym::Keysym::is_function_key,
            xkeysym::Keysym::is_misc_function_key,
            xkeysym::Keysym::is_modifier_key,
        ];
        for raw in (0..=0xffff).chain(0x11000000..=0x1100ffff).chain([
            0x100fffff,
            0x11010000,
            0x0101f642,
            u32::MAX,
        ]) {
            for (index, (native, rust)) in native.iter().zip(rust).enumerate() {
                assert_eq!(
                    rust(xkeysym::Keysym::new(raw)),
                    unsafe { native(raw) != 0 },
                    "keysym {raw:#x}, category {index}"
                );
            }
        }
    }
}
