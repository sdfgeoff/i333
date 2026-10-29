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
use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
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
use i3_common::safewrappers::sstrdup;

pub type i3String = _i3String;
pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;

pub type i3Font = Font;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;

pub type XID = ::core::ffi::c_ulong;
pub type Window = XID;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct button_t {
    pub label: *mut i3String,
    pub action: *mut ::core::ffi::c_char,
    pub x: i16,
    pub width: u16,
    pub terminal: bool,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_2 {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
    pub left_start_y: u32,
    pub left_end_y: u32,
    pub right_start_y: u32,
    pub right_end_y: u32,
    pub top_start_x: u32,
    pub top_end_x: u32,
    pub bottom_start_x: u32,
    pub bottom_end_x: u32,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const _PATH_BSHELL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/bin/sh\0") };

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;

pub static mut A__NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t = 0;

pub static mut A_I3_SOCKET_PATH: xcb_atom_t = 0;

pub static mut A__NET_WM_WINDOW_TYPE: xcb_atom_t = 0;

pub static mut A_CARDINAL: xcb_atom_t = 0;

pub static mut A_ATOM: xcb_atom_t = 0;

pub static mut A__NET_WM_STRUT_PARTIAL: xcb_atom_t = 0;
static mut argv0: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut win: xcb_window_t = 0;
static mut bar: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut font: i3Font = Font {
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
static mut prompt: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut btn_close: button_t = button_t {
    label: ::core::ptr::null::<i3String>() as *mut i3String,
    action: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    x: 0,
    width: 0,
    terminal: false,
};
static mut buttons: *mut button_t = ::core::ptr::null::<button_t>() as *mut button_t;
static mut buttoncnt: ::core::ffi::c_int = 0;
static mut color_background: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_button_background: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_border: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_border_bottom: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};
static mut color_text: color_t = color_t {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.,
    colorpixel: 0,
};

pub static mut root: xcb_window_t = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;

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
unsafe fn start_application(command: *const ::core::ffi::c_char) {
    printf(c"executing: %s\n".as_ptr(), command);
    if fork() == 0 as ::core::ffi::c_int {
        setsid();
        if fork() == 0 as ::core::ffi::c_int {
            execl(
                _PATH_BSHELL.as_ptr(),
                _PATH_BSHELL.as_ptr(),
                c"-c".as_ptr(),
                command,
                NULL,
            );
            err(libc::EXIT_FAILURE, c"execl return".as_ptr());
        }
        exit(0 as ::core::ffi::c_int);
    }
    wait(::core::ptr::null_mut::<::core::ffi::c_int>());
}
unsafe fn get_button_at(x: i16, y: i16) -> *mut button_t {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < buttoncnt {
        if x as ::core::ffi::c_int >= (*buttons.offset(c as isize)).x as ::core::ffi::c_int
            && x as ::core::ffi::c_int
                <= (*buttons.offset(c as isize)).x as ::core::ffi::c_int
                    + (*buttons.offset(c as isize)).width as ::core::ffi::c_int
        {
            return buttons.offset(c as isize) as *mut button_t;
        }
        c += 1;
    }
    ::core::ptr::null_mut::<button_t>()
}
unsafe fn handle_button_press(conn_0: *mut xcb_connection_t, event: *mut xcb_button_press_event_t) {
    printf(
        c"button pressed on x = %d, y = %d\n".as_ptr(),
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
}
unsafe fn handle_button_release(
    conn_0: *mut xcb_connection_t,
    event: *mut xcb_button_release_event_t,
) {
    printf(
        c"button released on x = %d, y = %d\n".as_ptr(),
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
    if (*event).event_x as ::core::ffi::c_int >= btn_close.x as ::core::ffi::c_int
        && ((*event).event_x as ::core::ffi::c_int)
            < btn_close.x as ::core::ffi::c_int + btn_close.width as ::core::ffi::c_int
    {
        exit(0 as ::core::ffi::c_int);
    }
    let button: *mut button_t = get_button_at((*event).event_x, (*event).event_y);
    if button.is_null() {
        return;
    }
    let script_path: *mut ::core::ffi::c_char = get_process_filename(c"nagbar-cmd".as_ptr());
    let fd: ::core::ffi::c_int = open(
        script_path,
        libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        warn(c"Could not create temporary script to store the nagbar command".as_ptr());
        return;
    }
    let script: *mut FILE = fdopen(fd, c"w".as_ptr());
    if script.is_null() {
        warn(c"Could not fdopen() temporary script to store the nagbar command".as_ptr());
        return;
    }
    fprintf(
        script,
        c"#!%s\nrm %s\n%s".as_ptr(),
        _PATH_BSHELL.as_ptr(),
        script_path,
        (*button).action,
    );
    fclose(script);
    let mut link_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let exe_path: *mut ::core::ffi::c_char = get_exe_path(argv0);
    sasprintf(&raw mut link_path, c"%s.nagbar_cmd".as_ptr(), script_path);
    if symlink(exe_path, link_path) == -(1 as ::core::ffi::c_int) {
        err(
            libc::EXIT_FAILURE,
            c"Failed to symlink %s to %s".as_ptr(),
            link_path,
            exe_path,
        );
    }
    let mut terminal_cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*button).terminal {
        sasprintf(
            &raw mut terminal_cmd,
            c"i3-sensible-terminal -e %s".as_ptr(),
            link_path,
        );
    } else {
        terminal_cmd = sstrdup(link_path);
    }
    printf(c"argv0 = %s\n".as_ptr(), argv0);
    printf(c"terminal_cmd = %s\n".as_ptr(), terminal_cmd);
    start_application(terminal_cmd);
    free(link_path as *mut ::core::ffi::c_void);
    free(terminal_cmd as *mut ::core::ffi::c_void);
    free(script_path as *mut ::core::ffi::c_void);
    free(exe_path as *mut ::core::ffi::c_void);
}
unsafe fn button_draw(button: *mut button_t, position: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let text_width: ::core::ffi::c_int = predict_text_width((*button).label);
    (*button).width = (text_width
        + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int)
        + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
        as u16;
    (*button).x = (position - (*button).width as ::core::ffi::c_int) as i16;
    draw_util_rectangle(
        &raw mut bar,
        color_border,
        (position - (*button).width as ::core::ffi::c_int) as ::core::ffi::c_double,
        (logical_px(8 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (*button).width as ::core::ffi::c_double,
        (font.height
            + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        &raw mut bar,
        color_button_background,
        (position - (*button).width as ::core::ffi::c_int + logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (logical_px(8 as ::core::ffi::c_int) - logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (text_width + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (font.height + 2 as ::core::ffi::c_int * logical_px(3 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_text(
        (*button).label,
        &raw mut bar,
        color_text,
        color_button_background,
        position - (*button).width as ::core::ffi::c_int
            + logical_px(3 as ::core::ffi::c_int)
            + logical_px(3 as ::core::ffi::c_int),
        logical_px(8 as ::core::ffi::c_int),
        200 as ::core::ffi::c_int,
    );
    (*button).width as ::core::ffi::c_int
}
unsafe fn handle_expose(conn_0: *mut xcb_connection_t) -> ::core::ffi::c_int {
    draw_util_clear_surface(&raw mut bar, color_background);
    draw_util_text(
        prompt,
        &raw mut bar,
        color_text,
        color_background,
        logical_px(8 as ::core::ffi::c_int),
        logical_px(8 as ::core::ffi::c_int),
        bar.width - 2 as ::core::ffi::c_int * logical_px(8 as ::core::ffi::c_int),
    );
    let mut position: ::core::ffi::c_int = bar.width
        - (logical_px(8 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int)
            - logical_px(3 as ::core::ffi::c_int));
    position -= button_draw(&raw mut btn_close, position);
    position -= logical_px(15 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < buttoncnt {
        position -= logical_px(20 as ::core::ffi::c_int);
        position -= button_draw(buttons.offset(i as isize) as *mut button_t, position);
        i += 1;
    }
    draw_util_rectangle(
        &raw mut bar,
        color_border_bottom,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        (bar.height - logical_px(2 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        bar.width as ::core::ffi::c_double,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    xcb_flush(conn_0);
    1 as ::core::ffi::c_int
}
unsafe fn set_window_position_primary(result: *mut xcb_rectangle_t) {
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
    primary = xcb_randr_get_output_primary_reply(
        conn,
        pcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if primary.is_null() {
        verboselog(
            c"[libi3] ../i3-nagbar/main.c Could not determine the primary output.\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        res = xcb_randr_get_screen_resources_current_reply(
            conn,
            rcookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if res.is_null() {
            verboselog(
                c"[libi3] ../i3-nagbar/main.c Could not query screen resources.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            output = xcb_randr_get_output_info_reply(
                conn,
                xcb_randr_get_output_info(conn, (*primary).output, (*res).config_timestamp),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if output.is_null() || (*output).crtc as ::core::ffi::c_long == XCB_NONE {
                verboselog(
                    c"[libi3] ../i3-nagbar/main.c Could not query primary screen.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                crtc = xcb_randr_get_crtc_info_reply(
                    conn,
                    xcb_randr_get_crtc_info(conn, (*output).crtc, (*res).config_timestamp),
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
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
                        (*result).x = (*crtc).x;
                        (*result).y = (*crtc).y;
                    }
                }
            }
        }
    }
    free(res as *mut ::core::ffi::c_void);
    free(primary as *mut ::core::ffi::c_void);
}
unsafe fn set_window_position_focus(result: *mut xcb_rectangle_t) {
    let mut success: bool = false_0 != 0;
    let mut input_focus: *mut xcb_get_input_focus_reply_t =
        ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    let mut geometry: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut coordinates: *mut xcb_translate_coordinates_reply_t =
        ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    xcb_grab_server(conn);
    input_focus = xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if input_focus.is_null() || (*input_focus).focus as ::core::ffi::c_long == XCB_NONE {
        verboselog(
            c"[libi3] ../i3-nagbar/main.c Failed to receive the current input focus or no window has the input focus right now.\n".as_ptr().cast_mut(),
        );
    } else {
        geometry = xcb_get_geometry_reply(
            conn,
            xcb_get_geometry(conn, (*input_focus).focus as xcb_drawable_t),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geometry.is_null() {
            verboselog(
                c"[libi3] ../i3-nagbar/main.c Failed to received window geometry.\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            coordinates = xcb_translate_coordinates_reply(
                conn,
                xcb_translate_coordinates(
                    conn,
                    (*input_focus).focus,
                    root,
                    (*geometry).x,
                    (*geometry).y,
                ),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
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
                (*result).x = (*coordinates).dst_x;
                (*result).y = (*coordinates).dst_y;
                success = true_0 != 0;
            }
        }
    }
    xcb_ungrab_server(conn);
    free(input_focus as *mut ::core::ffi::c_void);
    free(coordinates as *mut ::core::ffi::c_void);
    free(geometry as *mut ::core::ffi::c_void);
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

struct NagbarText {
    message: *mut i3String,
    close_label: *mut i3String,
    buttons: Vec<button_t>,
}
impl Drop for NagbarText {
    fn drop(&mut self) {
        // SAFETY: the event loop has returned. These native strings were
        // allocated for this session and callbacks only borrowed them.
        unsafe {
            prompt = std::ptr::null_mut();
            buttons = std::ptr::null_mut();
            buttoncnt = 0;
            btn_close.label = std::ptr::null_mut();
            i3string_free(self.message);
            i3string_free(self.close_label);
            for button in &self.buttons {
                i3string_free(button.label);
            }
        }
    }
}

unsafe fn run_window(settings: NagbarSettings) -> ::core::ffi::c_int {
    argv0 = settings.program.as_ptr().cast_mut();
    let pattern = settings.font.as_ptr();
    let position_on_primary = settings.primary;
    let bar_type = if settings.warning {
        MessageType::Warning
    } else {
        MessageType::Error
    };
    let mut text = NagbarText {
        message: i3string_from_utf8(settings.message.as_ptr()),
        close_label: i3string_from_utf8(c"X".as_ptr()),
        buttons: settings
            .buttons
            .iter()
            .map(|button| button_t {
                label: i3string_from_utf8(button.label.as_ptr()),
                action: button
                    .action
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |action| action.as_ptr().cast_mut()),
                x: 0,
                width: 0,
                terminal: button.terminal,
            })
            .collect(),
    };
    prompt = text.message;
    btn_close.label = text.close_label;
    buttoncnt = text.buttons.len() as i32;
    buttons = if text.buttons.is_empty() {
        std::ptr::null_mut()
    } else {
        text.buttons.as_mut_ptr()
    };
    let mut screens: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screens);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
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
    root = (*root_screen).root;
    if bar_type == MessageType::Error {
        color_button_background = draw_util_hex_to_color(c"#680a0a".as_ptr());
        color_background = draw_util_hex_to_color(c"#900000".as_ptr());
        color_text = draw_util_hex_to_color(c"#ffffff".as_ptr());
        color_border = draw_util_hex_to_color(c"#d92424".as_ptr());
        color_border_bottom = draw_util_hex_to_color(c"#470909".as_ptr());
    } else {
        color_button_background = draw_util_hex_to_color(c"#ffc100".as_ptr());
        color_background = draw_util_hex_to_color(c"#ffa800".as_ptr());
        color_text = draw_util_hex_to_color(c"#000000".as_ptr());
        color_border = draw_util_hex_to_color(c"#ab7100".as_ptr());
        color_border_bottom = draw_util_hex_to_color(c"#ab7100".as_ptr());
    }
    init_dpi();
    font = load_font(pattern, true_0 != 0);
    set_font(&raw mut font);
    let mut win_pos: xcb_rectangle_t = xcb_rectangle_t {
        x: 50 as i16,
        y: 50 as i16,
        width: 500 as u16,
        height: (font.height
            + 2 as ::core::ffi::c_int * logical_px(8 as ::core::ffi::c_int)
            + logical_px(2 as ::core::ffi::c_int)) as u16,
    };
    if position_on_primary {
        set_window_position_primary(&raw mut win_pos);
    } else {
        set_window_position_focus(&raw mut win_pos);
    }
    let mut cursor_ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut::<xcb_cursor_context_t>();
    if xcb_cursor_context_new(conn, root_screen, &raw mut cursor_ctx) < 0 as ::core::ffi::c_int {
        errx(
            libc::EXIT_FAILURE,
            c"Cannot allocate xcursor context".as_ptr(),
        );
    }
    let cursor: xcb_cursor_t = xcb_cursor_load_cursor(cursor_ctx, c"left_ptr".as_ptr());
    xcb_cursor_context_free(cursor_ctx);
    win = xcb_generate_id(conn) as xcb_window_t;
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
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int
            | XCB_CW_CURSOR as ::core::ffi::c_int) as u32,
        &raw mut fresh2 as *mut u32 as *const ::core::ffi::c_void,
    );
    if !sncontext.is_null() {
        sn_launchee_context_setup_window(sncontext, win as Window);
    }
    xcb_map_window(conn, win);
    let reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom _NET_WM_WINDOW_TYPE".as_ptr(),
        );
    }
    A__NET_WM_WINDOW_TYPE = (*reply).atom;
    free(reply as *mut ::core::ffi::c_void);
    let reply_0: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_WINDOW_TYPE_DOCK_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_0.is_null() {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom _NET_WM_WINDOW_TYPE_DOCK".as_ptr(),
        );
    }
    A__NET_WM_WINDOW_TYPE_DOCK = (*reply_0).atom;
    free(reply_0 as *mut ::core::ffi::c_void);
    let reply_1: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        _NET_WM_STRUT_PARTIAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_1.is_null() {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom _NET_WM_STRUT_PARTIAL".as_ptr(),
        );
    }
    A__NET_WM_STRUT_PARTIAL = (*reply_1).atom;
    free(reply_1 as *mut ::core::ffi::c_void);
    let reply_2: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        I3_SOCKET_PATH_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_2.is_null() {
        errx(
            libc::EXIT_FAILURE,
            c"Could not get atom I3_SOCKET_PATH".as_ptr(),
        );
    }
    A_I3_SOCKET_PATH = (*reply_2).atom;
    free(reply_2 as *mut ::core::ffi::c_void);
    let reply_3: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        ATOM_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_3.is_null() {
        errx(libc::EXIT_FAILURE, c"Could not get atom ATOM".as_ptr());
    }
    A_ATOM = (*reply_3).atom;
    free(reply_3 as *mut ::core::ffi::c_void);
    let reply_4: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        CARDINAL_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply_4.is_null() {
        errx(libc::EXIT_FAILURE, c"Could not get atom CARDINAL".as_ptr());
    }
    A_CARDINAL = (*reply_4).atom;
    free(reply_4 as *mut ::core::ffi::c_void);
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        win,
        A__NET_WM_WINDOW_TYPE,
        A_ATOM,
        32 as u8,
        1 as u32,
        &raw mut A__NET_WM_WINDOW_TYPE_DOCK as *mut ::core::ffi::c_uchar
            as *const ::core::ffi::c_void,
    );
    let mut strut_partial: C2RustUnnamed_2 = C2RustUnnamed_2 {
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
    memset(
        &raw mut strut_partial as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2RustUnnamed_2>() as usize,
    );
    strut_partial.top = (font.height + logical_px(6 as ::core::ffi::c_int)) as u32;
    strut_partial.top_start_x = 0 as u32;
    strut_partial.top_end_x = 800 as u32;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        win,
        A__NET_WM_STRUT_PARTIAL,
        A_CARDINAL,
        32 as u8,
        12 as u32,
        &raw mut strut_partial as *const ::core::ffi::c_void,
    );
    draw_util_surface_init(
        conn,
        &raw mut bar,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    if !sncontext.is_null() {
        sn_launchee_context_complete(sncontext);
        sn_launchee_context_unref(sncontext);
    }
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
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
                        handle_expose(conn);
                    }
                }
                XCB_BUTTON_PRESS => {
                    handle_button_press(conn, event as *mut xcb_button_press_event_t);
                }
                XCB_BUTTON_RELEASE => {
                    handle_button_release(conn, event as *mut xcb_button_release_event_t);
                }
                XCB_CONFIGURE_NOTIFY => {
                    let configure_notify: *mut xcb_configure_notify_event_t =
                        event as *mut xcb_configure_notify_event_t;
                    if (*configure_notify).width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*configure_notify).height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                    {
                        draw_util_surface_set_size(
                            &raw mut bar,
                            (*configure_notify).width as ::core::ffi::c_int,
                            (*configure_notify).height as ::core::ffi::c_int,
                        );
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    draw_util_surface_free(conn, &raw mut bar);
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
                    println!("i3-nagbar {}\ni3-nagbar [-m <message>] [-b <button> <action>] [-B <button> <action>] [-t warning|error] [-f <font>] [-v] [-p]", env!("I3_VERSION"));
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
pub use cairo_sys::{cairo_surface_t, cairo_t};

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_button_press_event_t, xcb_button_release_event_t,
    xcb_change_property, xcb_charinfo_t, xcb_configure_notify_event_t, xcb_connect,
    xcb_connection_has_error, xcb_connection_t, xcb_create_window, xcb_cursor_context_free,
    xcb_cursor_context_new, xcb_cursor_context_t, xcb_cursor_load_cursor, xcb_cursor_t,
    xcb_drawable_t, xcb_expose_event_t, xcb_flush, xcb_generate_id, xcb_generic_error_t,
    xcb_generic_event_t, xcb_get_geometry, xcb_get_geometry_reply, xcb_get_geometry_reply_t,
    xcb_get_input_focus, xcb_get_input_focus_reply, xcb_get_input_focus_reply_t, xcb_grab_server,
    xcb_intern_atom, xcb_intern_atom_cookie_t, xcb_intern_atom_reply, xcb_intern_atom_reply_t,
    xcb_map_window, xcb_query_font_reply_t, xcb_randr_get_crtc_info, xcb_randr_get_crtc_info_reply,
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

pub use libc::{
    execl, exit, fclose, fdopen, fork, fprintf, free, memset, open, printf, setsid, strlen,
    symlink, wait, FILE,
};

use i3_common::native_stdio::{err, errx, stderr, vfprintf, warn};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_process_filename::get_process_filename;
use i3_common::get_visualtype::get_visualtype;

pub use i3_common::string::I3String as _i3String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MessageType {
    Error = 0,
    Warning = 1,
}
