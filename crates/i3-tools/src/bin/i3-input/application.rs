use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::string::I3String;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::fd::FromRawFd;
use std::os::unix::net::UnixStream;
use std::ptr::NonNull;
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

pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;

pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
pub const XK_BackSpace: ::core::ffi::c_int = 0xff08 as ::core::ffi::c_int;
pub const XK_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
pub const XK_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
pub const XK_Mode_switch: ::core::ffi::c_int = 0xff7e as ::core::ffi::c_int;

pub const EXIT_OK: i32 = 0;
pub const EXIT_CANCEL: i32 = 1;
pub const EXIT_ERROR: i32 = 2;
static mut symbols: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
static mut win: xcb_window_t = 0;
static mut surface: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut font: Font = Font {
    type_0: i3_common::font::FontType::None,
    height: 0,
    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    specific: C2RustUnnamed {
        xcb: C2RustUnnamed_0 {
            id: 0,
            info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
        },
    },
};

pub static mut root: xcb_window_t = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;

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
    fn from_keysym(sym: xcb_keysym_t) -> Option<Self> {
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

/// A uniquely owned, malloc-allocated native reply/event.
struct Native<T>(NonNull<T>);
impl<T> Native<T> {
    /// # Safety
    /// pointer is null or points to one initialized native allocation of T.
    unsafe fn from_raw(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
    fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
    fn get(&self) -> &T {
        // SAFETY: from_raw establishes valid initialized storage; self owns it.
        unsafe { self.0.as_ref() }
    }
}
impl<T> Drop for Native<T> {
    fn drop(&mut self) {
        // SAFETY: native XCB results are malloc allocations; this is their owner.
        unsafe { libc::free(self.as_ptr().cast()) }
    }
}
impl Native<xcb_get_property_reply_t> {
    fn bytes(&self) -> Option<&[u8]> {
        // SAFETY: this owns a complete XCB property reply.
        let length =
            usize::try_from(unsafe { xcb_get_property_value_length(self.as_ptr()) }).ok()?;
        if length == 0 {
            return Some(&[]);
        }
        if length > (self.get().length as usize).checked_mul(4)? {
            return None;
        }
        // SAFETY: the length is checked against the owned reply's payload;
        // the returned slice cannot outlive that reply.
        Some(unsafe {
            std::slice::from_raw_parts(xcb_get_property_value(self.as_ptr()).cast(), length)
        })
    }
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
    conn_0: *mut xcb_connection_t,
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
    if let Some(prompt) = state.prompt.as_mut() {
        draw_util_text(
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
        let mut input = I3String::from_ucs2(&glyphs);
        draw_util_text(
            &mut input,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset + state.prompt_offset,
            text_offset,
            logical_px(500) - text_offset,
        );
    }
    xcb_flush(conn_0);
    1 as ::core::ffi::c_int
}
unsafe fn handle_key_release(state: &mut InputState, event: &xcb_key_release_event_t) {
    printf(
        c"releasing %d, state raw = %d\n".as_ptr(),
        event.detail as libc::c_int,
        event.state as libc::c_int,
    );
    let mut lookup_event = *event;
    let sym = xcb_key_press_lookup_keysym(symbols, &mut lookup_event, event.state as libc::c_int);
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(c"Mode switch disabled\n".as_ptr());
        state.modeswitch_active = false;
    }
}
unsafe fn handle_key_press(
    state: &mut InputState,
    conn_0: *mut xcb_connection_t,
    event: &xcb_key_press_event_t,
) -> KeyAction {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        event.detail as ::core::ffi::c_int,
        event.state as ::core::ffi::c_int,
    );
    let mut col: ::core::ffi::c_int =
        event.state as ::core::ffi::c_int & XCB_MOD_MASK_SHIFT as ::core::ffi::c_int;
    if state.modeswitch_active {
        col += 2 as ::core::ffi::c_int;
    }
    let mut lookup_event = *event;
    let sym: xcb_keysym_t = xcb_key_press_lookup_keysym(symbols, &mut lookup_event, col);
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(c"Mode switch enabled\n".as_ptr());
        state.modeswitch_active = true;
        return KeyAction::Continue;
    }
    if sym == XK_Return as xcb_keysym_t {
        return KeyAction::Finish;
    }
    if sym == XK_BackSpace as xcb_keysym_t {
        if state.backspace() {
            handle_expose(state, conn_0);
        }
        return KeyAction::Continue;
    }
    if sym == XK_Escape as xcb_keysym_t {
        return KeyAction::Cancel;
    }
    printf(c"is_keypad_key = %d\n".as_ptr(), xcb_is_keypad_key(sym));
    printf(
        c"is_private_keypad_key = %d\n".as_ptr(),
        xcb_is_private_keypad_key(sym),
    );
    printf(c"xcb_is_cursor_key = %d\n".as_ptr(), xcb_is_cursor_key(sym));
    printf(c"xcb_is_pf_key = %d\n".as_ptr(), xcb_is_pf_key(sym));
    printf(
        c"xcb_is_function_key = %d\n".as_ptr(),
        xcb_is_function_key(sym),
    );
    printf(
        c"xcb_is_misc_function_key = %d\n".as_ptr(),
        xcb_is_misc_function_key(sym),
    );
    printf(
        c"xcb_is_modifier_key = %d\n".as_ptr(),
        xcb_is_modifier_key(sym),
    );
    if xcb_is_modifier_key(sym) != 0 || xcb_is_cursor_key(sym) != 0 {
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
    handle_expose(state, conn_0);
    KeyAction::Continue
}
/// A server grab must be released on every return from position lookup.
struct ServerGrab(*mut xcb_connection_t);
impl Drop for ServerGrab {
    fn drop(&mut self) {
        // SAFETY: this private guard borrows run_window's live connection.
        unsafe {
            xcb_ungrab_server(self.0);
            xcb_flush(self.0);
        }
    }
}

unsafe fn get_window_position() -> xcb_rectangle_t {
    let mut result = xcb_rectangle_t {
        x: logical_px(50) as i16,
        y: logical_px(50) as i16,
        width: logical_px(500) as u16,
        height: (font.height + 4 * logical_px(2)) as u16,
    };
    let atom = c"_NET_SUPPORTING_WM_CHECK";
    let reply = Native::from_raw(xcb_intern_atom_reply(
        conn,
        xcb_intern_atom(conn, 0, atom.to_bytes().len() as u16, atom.as_ptr()),
        std::ptr::null_mut(),
    ));
    let Some(reply) = reply else {
        errorlog(
            c"[libi3] ERROR: Could not intern atom _NET_SUPPORTING_WM_CHECK\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_ERROR);
    };
    let supporting = Native::from_raw(xcb_get_property_reply(
        conn,
        xcb_get_property(
            conn,
            0,
            root,
            reply.get().atom,
            XCB_ATOM_WINDOW as xcb_atom_t,
            0,
            32,
        ),
        std::ptr::null_mut(),
    ));
    let supporting_window = supporting.as_ref().and_then(|reply| {
        Some(u32::from_ne_bytes(
            reply.bytes()?.get(..4)?.try_into().ok()?,
        ))
    });
    xcb_grab_server(conn);
    let _server_grab = ServerGrab(conn);
    let Some(focus) = Native::from_raw(xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        std::ptr::null_mut(),
    )) else {
        return result;
    };
    let focused = focus.get().focus;
    if focused == XCB_NONE as xcb_window_t || Some(focused) == supporting_window {
        return result;
    }
    let Some(geometry) = Native::from_raw(xcb_get_geometry_reply(
        conn,
        xcb_get_geometry(conn, focused),
        std::ptr::null_mut(),
    )) else {
        return result;
    };
    let wm_class = Native::from_raw(xcb_get_property_reply(
        conn,
        xcb_get_property(
            conn,
            0,
            focused,
            XCB_ATOM_WM_CLASS as xcb_atom_t,
            XCB_GET_PROPERTY_TYPE_ANY as xcb_atom_t,
            0,
            32,
        ),
        std::ptr::null_mut(),
    ));
    let is_frame = wm_class
        .as_ref()
        .and_then(Native::bytes)
        .is_some_and(|bytes| bytes.split(|byte| *byte == 0).next() == Some(b"i3-frame".as_slice()));
    let position = if is_frame {
        (geometry.get().x, geometry.get().y)
    } else {
        let Some(coordinates) = Native::from_raw(xcb_translate_coordinates_reply(
            conn,
            xcb_translate_coordinates(conn, focused, root, geometry.get().x, geometry.get().y),
            std::ptr::null_mut(),
        )) else {
            return result;
        };
        (coordinates.get().dst_x, coordinates.get().dst_y)
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
    connection: NonNull<xcb_connection_t>,
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
                draw_util_surface_free(conn, &raw mut surface);
            }
            if self.has_font {
                i3_common::font::free_font();
            }
            if !symbols.is_null() {
                xcb_key_symbols_free(symbols);
            }
            symbols = std::ptr::null_mut();
            root_screen = std::ptr::null_mut();
            visual_type = std::ptr::null_mut();
            root = 0;
            conn = std::ptr::null_mut();
            xcb_disconnect(self.connection.as_ptr());
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
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    let mut resources = WindowResources {
        connection: NonNull::new(conn).expect("checked native connection"),
        has_font: false,
        has_surface: false,
    };
    // SAFETY: ipc_connect either returns its newly owned connected descriptor
    // or exits with the historical fatal diagnostic; no other owner remains.
    let mut socket = UnixStream::from_raw_fd(ipc_connect(socket_path));
    root_screen = xcb_aux_get_screen(conn, screen);
    root = (*root_screen).root;
    symbols = xcb_key_symbols_alloc(conn);
    if symbols.is_null() {
        errx(EXIT_ERROR, c"Could not allocate key symbols".as_ptr());
    }
    init_dpi();
    font = load_font(
        if !pattern.is_null() {
            pattern as *const ::core::ffi::c_char
        } else {
            c"pango:monospace 8".as_ptr()
        },
        true_0 != 0,
    );
    resources.has_font = true;
    set_font(&raw mut font);
    if let Some(prompt) = state.prompt.as_mut() {
        state.prompt_offset = predict_text_width(prompt);
    }
    let win_pos: xcb_rectangle_t = get_window_position() as xcb_rectangle_t;
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [u32; 3] = [
        0 as ::core::ffi::c_int as u32,
        1 as ::core::ffi::c_int as u32,
        XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int as u32,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as u8,
        win,
        root,
        win_pos.x,
        win_pos.y,
        win_pos.width,
        win_pos.height,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
            | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int) as u32,
        &raw mut fresh1 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, win);
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    xcb_flush(conn);
    resources.has_surface = true;
    let mut grab_status = None;
    for _ in 0..500 {
        let cookie = xcb_grab_keyboard(
            conn,
            0,
            win,
            XCB_CURRENT_TIME as xcb_timestamp_t,
            XCB_GRAB_MODE_ASYNC as u8,
            XCB_GRAB_MODE_ASYNC as u8,
        );
        let reply = Native::from_raw(xcb_grab_keyboard_reply(conn, cookie, std::ptr::null_mut()));
        std::thread::sleep(std::time::Duration::from_millis(1));
        grab_status = reply.as_ref().map(|reply| reply.get().status);
        if grab_status == Some(XCB_GRAB_STATUS_SUCCESS as u8) {
            break;
        }
    }
    if grab_status != Some(XCB_GRAB_STATUS_SUCCESS as u8) {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            grab_status.map_or(-1, i32::from),
        );
        return EXIT_ERROR;
    }
    xcb_flush(conn);
    loop {
        let Some(event) = Native::from_raw(xcb_wait_for_event(conn)) else {
            return EXIT_OK;
        };
        let header = event.get();
        if header.response_type == 0 {
            fprintf(
                stderr,
                c"X11 Error received! sequence %x\n".as_ptr(),
                header.sequence as libc::c_int,
            );
            continue;
        }
        // SAFETY: XCB event packets include the complete wire event. Its tag
        // determines the record type; references stay inside this owned event.
        let action = match (header.response_type & 0x7f) as i32 {
            XCB_KEY_PRESS => handle_key_press(
                &mut state,
                conn,
                &*event.as_ptr().cast::<xcb_key_press_event_t>(),
            ),
            XCB_KEY_RELEASE => {
                handle_key_release(
                    &mut state,
                    &*event.as_ptr().cast::<xcb_key_release_event_t>(),
                );
                KeyAction::Continue
            }
            XCB_EXPOSE => {
                if (*event.as_ptr().cast::<xcb_expose_event_t>()).count == 0 {
                    handle_expose(&mut state, conn);
                }
                KeyAction::Continue
            }
            _ => KeyAction::Continue,
        };
        match action {
            KeyAction::Continue => {}
            KeyAction::Cancel => return EXIT_CANCEL,
            KeyAction::Finish => {
                // Keep native diagnostic lines ahead of the owned Rust output.
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

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_charinfo_t, xcb_connect, xcb_connection_has_error,
    xcb_connection_t, xcb_create_window, xcb_disconnect, xcb_drawable_t, xcb_expose_event_t,
    xcb_flush, xcb_generate_id, xcb_get_geometry, xcb_get_geometry_reply, xcb_get_input_focus,
    xcb_get_input_focus_reply, xcb_get_property, xcb_get_property_reply, xcb_get_property_reply_t,
    xcb_get_property_value, xcb_get_property_value_length, xcb_grab_keyboard,
    xcb_grab_keyboard_reply, xcb_grab_server, xcb_intern_atom, xcb_intern_atom_reply,
    xcb_is_cursor_key, xcb_is_function_key, xcb_is_keypad_key, xcb_is_misc_function_key,
    xcb_is_modifier_key, xcb_is_pf_key, xcb_is_private_keypad_key, xcb_key_press_event_t,
    xcb_key_press_lookup_keysym, xcb_key_release_event_t, xcb_key_symbols_alloc,
    xcb_key_symbols_free, xcb_key_symbols_t, xcb_keysym_t, xcb_map_window, xcb_query_font_reply_t,
    xcb_rectangle_t, xcb_screen_t, xcb_timestamp_t, xcb_translate_coordinates,
    xcb_translate_coordinates_reply, xcb_ungrab_server, xcb_visualid_t, xcb_visualtype_t,
    xcb_wait_for_event, xcb_window_t, XCB_ATOM_WINDOW, XCB_ATOM_WM_CLASS, XCB_COPY_FROM_PARENT,
    XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL, XCB_CW_EVENT_MASK, XCB_CW_OVERRIDE_REDIRECT,
    XCB_EVENT_MASK_EXPOSURE, XCB_EXPOSE, XCB_GET_PROPERTY_TYPE_ANY, XCB_GRAB_MODE_ASYNC,
    XCB_GRAB_STATUS_SUCCESS, XCB_KEY_PRESS, XCB_KEY_RELEASE, XCB_MOD_MASK_SHIFT, XCB_NONE,
    XCB_WINDOW_CLASS_COPY_FROM_PARENT, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

use libc::{exit, fprintf, printf};

use i3_common::native_stdio::{errx, stderr, stdout, vfprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_visualtype::get_visualtype;
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
        // XCB pads each property reply to four bytes. The helper must not read
        // a claimed value length past that padding, unlike the old strcmp path.
        unsafe {
            let size = std::mem::size_of::<xcb_get_property_reply_t>() + 4;
            let pointer = libc::calloc(1, size).cast::<xcb_get_property_reply_t>();
            assert!(!pointer.is_null());
            (*pointer).format = 8;
            (*pointer).length = 1;
            (*pointer).value_len = 8;
            let reply = Native::from_raw(pointer).unwrap();
            assert!(reply.bytes().is_none());
            (*pointer).value_len = 4;
            std::ptr::copy_nonoverlapping(b"name".as_ptr(), pointer.add(1).cast(), 4);
            assert_eq!(reply.bytes(), Some(b"name".as_slice()));
        }
    }
}
