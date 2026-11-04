use crate::randr::outputs;
use crate::xcursor::Cursor;
use i3_common::draw_util::draw_util_text;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of src/sighandler.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

// The crash callback remains a legacy native signal edge: X11/rendering,
// logging and restart are not claimed async-signal-safe. New helpers below are
// geometry helpers are stack-only; backtrace arguments allocate before fork,
// as their native predecessors did. No display registry is consulted here.
use i3_common::display::CairoVisual;
use i3_common::string::I3String;
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use xcb::{x, Xid, XidNew};
type Output = crate::types::xoutput;

#[repr(C)]
pub struct dialog_t {
    pub id: u32,
    pub colormap: u32,
    pub dims: Rect,
    pub surface: surface_t,
    pub dialogs: DialogLinks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DialogLinks {
    pub tqe_next: *mut dialog_t,
    pub tqe_prev: *mut *mut dialog_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dialogs_head {
    pub tqh_first: *mut dialog_t,
    pub tqh_last: *mut *mut dialog_t,
}

pub const SIG_DFL: libc::sighandler_t = libc::SIG_DFL;
pub const SIGILL: i32 = 4_i32;
pub const SIGABRT: i32 = 6_i32;
pub const SIGFPE: i32 = 8_i32;
pub const SIGSEGV: i32 = 11_i32;
pub const SIGQUIT: i32 = 3_i32;
pub const SA_SIGINFO: i32 = 4_i32;
pub const SA_NODEFER: i32 = 0x40000000_i32;
pub const SA_RESETHAND: u32 = 0x80000000_u32;
pub const STDIN_FILENO: i32 = 0_i32;
pub const STDOUT_FILENO: i32 = 1_i32;
pub const STDERR_FILENO: i32 = 2_i32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut dialogs: dialogs_head = dialogs_head {
    tqh_first: std::ptr::null_mut::<dialog_t>(),
    tqh_last: std::ptr::null_mut::<*mut dialog_t>(),
};
static mut raised_signal: i32 = 0;
static mut CRASH_RENDERER: Option<i3_common::draw_util::NativeRenderingContext> = None;
static mut backtrace_done: i32 = 0_i32;
static mut message_intro: Option<I3String> = None;
static mut message_intro2: Option<I3String> = None;
static mut message_option_backtrace: Option<I3String> = None;
static mut message_option_restart: Option<I3String> = None;
static mut message_option_forget: Option<I3String> = None;
static mut dialog_width: i32 = 0;
static mut dialog_height: i32 = 0;
static mut border_width: i32 = 2_i32;
static mut margin: i32 = 4_i32;
fn backtrace_filename(directory: &[u8], pid: i32, suffix: i32) -> Vec<u8> {
    let directory = directory.split(|&byte| byte == 0).next().unwrap();
    let mut filename = directory.to_vec();
    filename.extend_from_slice(format!("/i3-backtrace.{pid}.{suffix}.txt").as_bytes());
    filename
}

fn backtrace_log_command(filename: &[u8]) -> Vec<u8> {
    let filename = filename.split(|&byte| byte == 0).next().unwrap();
    let mut command = b"set logging file ".to_vec();
    command.extend_from_slice(filename);
    command
}

unsafe fn sighandler_backtrace() -> i32 {
    let mut tmpdir: *mut ::core::ffi::c_char = getenv(c"TMPDIR".as_ptr());
    if tmpdir.is_null() {
        tmpdir = c"/tmp".as_ptr().cast_mut();
    }
    let directory = CStr::from_ptr(tmpdir).to_bytes();
    let pid_parent = getpid();
    let mut suffix = 0_i32;
    let filename = loop {
        let filename = backtrace_filename(directory, pid_parent, suffix);
        suffix += 1;
        if !exists(std::ffi::OsStr::from_bytes(&filename)) {
            break filename;
        }
    };
    // These two C strings exist only for execvp's native argv boundary. Build
    // every owned argument before fork, keeping the child path allocation-free.
    let pid_s = std::ffi::CString::new(pid_parent.to_string())
        .expect("a decimal process id contains no NUL");
    let gdb_log_cmd = std::ffi::CString::new(backtrace_log_command(&filename))
        .expect("native path components contain no NUL");
    let args = [
        c"gdb".as_ptr(),
        *start_argv,
        c"-p".as_ptr(),
        pid_s.as_ptr(),
        c"-batch".as_ptr(),
        c"-nx".as_ptr(),
        c"-ex".as_ptr(),
        gdb_log_cmd.as_ptr(),
        c"-ex".as_ptr(),
        c"set logging on".as_ptr(),
        c"-ex".as_ptr(),
        c"bt full".as_ptr(),
        c"-ex".as_ptr(),
        c"quit".as_ptr(),
        std::ptr::null(),
    ];
    // GDB is batch-only: a null device supplies EOF and discards normal stdout.
    // The old unconsumed pipes could block a large batch output indefinitely.
    let null_fd = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR);
    if null_fd < 0 {
        return -1;
    }
    let pid_gdb = fork();
    if pid_gdb < 0 {
        close(null_fd);
        return -1;
    }
    if pid_gdb == 0 {
        dup2(null_fd, STDIN_FILENO);
        dup2(null_fd, STDOUT_FILENO);
        close(STDERR_FILENO);
        if null_fd > STDERR_FILENO {
            close(null_fd);
        }
        execvp(args[0], args.as_ptr());
        err(EXIT_FAILURE, c"execvp(gdb)".as_ptr());
    }
    close(null_fd);
    drop((pid_s, gdb_log_cmd));
    let mut status: i32 = 0_i32;
    waitpid(
        pid_gdb as libc::pid_t,
        &raw mut status,
        0 as ::core::ffi::c_int,
    );
    if !child_succeeded(status) {
        crate::log::write_log_bytes(
            i3_common::logging::Level::Debug,
            b"../src/sighandler.c:sighandler_backtrace:123 - GDB did not run properly\n",
        );
        return -1;
    } else if !exists(std::ffi::OsStr::from_bytes(&filename)) {
        crate::log::write_log_bytes(i3_common::logging::Level::Debug, b"../src/sighandler.c:sighandler_backtrace:126 - GDB executed successfully, but no backtrace was generated\n");
        return -1;
    }
    1
}
unsafe fn prepare_crash_messages() {
    let visual_id = get_visualid_by_depth(root_depth as u16);
    CRASH_RENDERER = crate::application::root_display()
        .visual(visual_id)
        .map(|visual| {
            let mut renderer =
                i3_common::draw_util::NativeRenderingContext::new(conn, visual, root_depth);
            renderer.prepare(root);
            renderer
        });

    message_intro = Some(I3String::new(
        "i3 has just crashed. Please report a bug for this.",
    ));
    message_intro2 = Some(I3String::new(
        "To debug this problem, you can either attach gdb or choose from the following options:",
    ));
    message_option_backtrace = Some(I3String::new("- 'b' to save a backtrace (requires gdb)"));
    message_option_restart = Some(I3String::new("- 'r' to restart i3 in-place"));
    message_option_forget = Some(I3String::new(
        "- 'f' to forget the previous layout and restart i3",
    ));
}

unsafe fn sighandler_setup() {
    border_width = logical_px(border_width);
    margin = logical_px(margin);
    let num_lines: i32 = 5_i32;
    let width_longest_message: i32 = predict_text_width(
        &crate::config::config.font,
        (&*(&raw const message_intro2)).as_ref().unwrap(),
    );
    dialog_width = width_longest_message + 2_i32 * border_width + 2_i32 * margin;
    dialog_height = num_lines * config.font.height + 2_i32 * border_width + 2_i32 * margin;
}
fn dialog_geometry(output: Rect, width: i32, height: i32) -> Rect {
    Rect {
        x: output
            .x
            .wrapping_add(output.width / 2)
            .wrapping_sub((width as u32) / 2),
        y: output
            .y
            .wrapping_add(output.height / 2)
            .wrapping_sub((height as u32) / 2),
        width: width as u32,
        height: height as u32,
    }
}
fn dialog_attributes(black: u32, colormap: u32) -> [x::Cw; 4] {
    [
        x::Cw::BackPixel(black),
        x::Cw::BorderPixel(black),
        x::Cw::OverrideRedirect(true),
        x::Cw::Colormap(x::Colormap::new(colormap)),
    ]
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum CrashAction {
    Backtrace,
    Restart,
    Forget,
}
fn crash_action(symbol: u32) -> Option<CrashAction> {
    match symbol {
        98 => Some(CrashAction::Backtrace),
        114 => Some(CrashAction::Restart),
        102 => Some(CrashAction::Forget),
        _ => None,
    }
}
fn backtrace_color(status: i32) -> &'static str {
    if status < 0 {
        "#AA0000"
    } else if status > 0 {
        "#00AA00"
    } else {
        "#FFFFFF"
    }
}
fn child_succeeded(status: i32) -> bool {
    libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0
}
unsafe fn sighandler_create_dialogs() {
    // Never touch the event-thread RefCell/Rc registry from an async signal.
    // The main native owner remains live throughout these emergency operations.
    let Some(connection) = i3_common::xcb_connection::borrowed(conn) else {
        return;
    };
    let mut output: *mut Output = outputs.tqh_first as *mut Output;

    while !output.is_null() {
        if (*output).active {
            let dialog = Box::into_raw(Box::new(dialog_t {
                id: 0,
                colormap: 0,
                dims: Rect {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                },
                surface: surface_t::empty(),
                dialogs: DialogLinks {
                    tqe_next: std::ptr::null_mut(),
                    tqe_prev: std::ptr::null_mut(),
                },
            }));
            (*dialog).dialogs.tqe_next = ::core::ptr::null_mut::<dialog_t>();
            (*dialog).dialogs.tqe_prev = dialogs.tqh_last;
            *dialogs.tqh_last = dialog;
            dialogs.tqh_last = &raw mut (*dialog).dialogs.tqe_next;
            // Prepared before signal installation: no registry/Rc operation.
            let Some(renderer) = CRASH_RENDERER.as_mut() else {
                return;
            };
            let visual = renderer.visual().visual_id();
            let colormap: x::Colormap = connection.generate_id();
            (*dialog).colormap = colormap.resource_id();
            connection.send_request(&x::CreateColormap {
                alloc: x::ColormapAlloc::None,
                mid: colormap,
                window: x::Window::new(root),
                visual,
            });
            let values = dialog_attributes(root_screen_info().black_pixel, (*dialog).colormap);
            (*dialog).dims = dialog_geometry((*output).rect, dialog_width, dialog_height);
            (*dialog).id = create_window(
                &connection,
                (*dialog).dims,
                root_depth,
                visual,
                x::WindowClass::InputOutput,
                Cursor::Pointer,
                true,
                &values,
            );
            i3_common::draw_util::draw_util_surface_init_native(
                renderer,
                &raw mut (*dialog).surface,
                (*dialog).id as u32,
                (*dialog).dims.width as i32,
                (*dialog).dims.height as i32,
            );
            let keyboard = connection.send_request(&x::GrabKeyboard {
                owner_events: false,
                grab_window: x::Window::new((*dialog).id),
                time: x::CURRENT_TIME,
                pointer_mode: x::GrabMode::Async,
                keyboard_mode: x::GrabMode::Async,
            });
            xcb_interop::transport::discard_reply(&connection, keyboard);
            let pointer = connection.send_request(&x::GrabPointer {
                owner_events: false,
                grab_window: x::Window::new((*dialog).id),
                event_mask: x::EventMask::empty(),
                pointer_mode: x::GrabMode::Async,
                keyboard_mode: x::GrabMode::Async,
                confine_to: x::Window::new((*dialog).id),
                cursor: x::Cursor::none(),
                time: x::CURRENT_TIME,
            });
            xcb_interop::transport::discard_reply(&connection, pointer);
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    sighandler_handle_expose();
    let _ = connection.flush();
}
unsafe fn sighandler_destroy_dialogs() {
    let connection = i3_common::xcb_connection::borrowed(conn);
    while !dialogs.tqh_first.is_null() {
        let dialog: *mut dialog_t = dialogs.tqh_first;
        if let Some(connection) = &connection {
            connection.send_request(&x::FreeColormap {
                cmap: x::Colormap::new((*dialog).colormap),
            });
        }
        draw_util_surface_free(conn, &raw mut (*dialog).surface);
        if let Some(connection) = &connection {
            connection.send_request(&x::DestroyWindow {
                window: x::Window::new((*dialog).id),
            });
        }
        if !(*dialog).dialogs.tqe_next.is_null() {
            (*(*dialog).dialogs.tqe_next).dialogs.tqe_prev = (*dialog).dialogs.tqe_prev;
        } else {
            dialogs.tqh_last = (*dialog).dialogs.tqe_prev;
        }
        *(*dialog).dialogs.tqe_prev = (*dialog).dialogs.tqe_next;
        drop(Box::from_raw(dialog));
    }
    if let Some(connection) = i3_common::xcb_connection::borrowed(conn) {
        let _ = connection.flush();
    }
}
unsafe fn sighandler_handle_expose() {
    let mut current: *mut dialog_t = dialogs.tqh_first;

    while !current.is_null() {
        sighandler_draw_dialog(current);
        current = (*current).dialogs.tqe_next;
    }
    if let Some(connection) = i3_common::xcb_connection::borrowed(conn) {
        let _ = connection.flush();
    }
}
pub unsafe fn shutdown_crash_visual() {
    sighandler_destroy_dialogs();
    drop(CRASH_RENDERER.take());
}

unsafe fn sighandler_draw_dialog(dialog: *mut dialog_t) {
    let color_screen = crate::application::root_display_opt();
    let black: color_t = draw_util_hex_to_color(color_screen, "#000000") as color_t;
    let white: color_t = draw_util_hex_to_color(color_screen, "#FFFFFF") as color_t;
    let red: color_t = draw_util_hex_to_color(color_screen, "#FF0000") as color_t;
    draw_util_clear_surface(&raw mut (*dialog).surface, red);
    draw_util_rectangle(
        &raw mut (*dialog).surface,
        black,
        border_width as f64,
        border_width as f64,
        (*dialog)
            .dims
            .width
            .wrapping_sub((2_i32 * border_width) as u32) as f64,
        (*dialog)
            .dims
            .height
            .wrapping_sub((2_i32 * border_width) as u32) as f64,
    );
    let mut y: i32 = border_width + margin;
    let x: i32 = border_width + margin;
    let max_width: i32 = (*dialog).dims.width.wrapping_sub((2_i32 * x) as u32) as i32;
    draw_util_text(
        &crate::config::config.font,
        (&*(&raw const message_intro)).as_ref().unwrap(),
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        &crate::config::config.font,
        (&*(&raw const message_intro2)).as_ref().unwrap(),
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    let bt_color = backtrace_color(backtrace_done);
    draw_util_text(
        &crate::config::config.font,
        (&*(&raw const message_option_backtrace)).as_ref().unwrap(),
        &raw mut (*dialog).surface,
        draw_util_hex_to_color(color_screen, bt_color),
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        &crate::config::config.font,
        (&*(&raw const message_option_restart)).as_ref().unwrap(),
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        &crate::config::config.font,
        (&*(&raw const message_option_forget)).as_ref().unwrap(),
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
}
unsafe fn sighandler_handle_key_press(event: &xcb::x::KeyPressEvent) {
    let mut state = xcb_interop::events::key_press_state(event);
    state = (state as u32 & !xcb_numlock_mask) as u16;
    // The native press-lookup utility is exactly this keycode/column lookup.
    let sym = crate::application::key_symbol(event.detail(), i32::from(state));
    if crash_action(sym) == Some(CrashAction::Backtrace) {
        crate::log::write_log_bytes(i3_common::logging::Level::Debug, b"../src/sighandler.c:sighandler_handle_key_press:280 - User issued core-dump command.\n");
        backtrace_done = sighandler_backtrace();
        sighandler_handle_expose();
    } else if crash_action(sym) == Some(CrashAction::Restart) {
        sighandler_destroy_dialogs();
        i3_restart(false);
    } else if crash_action(sym) == Some(CrashAction::Forget) {
        sighandler_destroy_dialogs();
        i3_restart(true);
    }
}
unsafe extern "C" fn handle_signal(
    sig: ::core::ffi::c_int,
    info: *mut siginfo_t,
    data: *mut ::core::ffi::c_void,
) {
    // Nested restart, IPC and geometry helpers also log. Use the prepared
    // per-thread fast sink before any of them can consult the facade registry.
    i3_common::logging::enter_emergency_logging(crate::common_log_sink);
    // The fault may interrupt the facade sink's lock. Keep emergency records
    // on the existing native backend, including this function's key-event loop.
    // Preserve the legacy debug filter before allocating the formatted bytes.
    if crate::log::debug_record_enabled() {
        let message = format!(
            "../src/sighandler.c:handle_signal:296 - i3 crashed. SIG: {}\n",
            sig
        );
        crate::log::write_log_bytes(i3_common::logging::Level::Debug, message.as_bytes());
    };
    let mut action: sigaction = ::core::mem::zeroed();
    action.sa_sigaction = libc::SIG_DFL;
    action.sa_flags = 0_i32;
    sigemptyset(&raw mut action.sa_mask);
    sigaction(sig, &raw mut action, ::core::ptr::null_mut::<sigaction>());
    raised_signal = sig;
    sighandler_setup();
    sighandler_create_dialogs();
    // This emergency path cannot use the event-thread Rc/RefCell registry: the
    // signal may have interrupted a registry borrow. The main owner remains
    // live throughout the crash dialog. Restart exits without returning here;
    // this no-drop view never disconnects its native connection.
    let Some(connection) = i3_common::xcb_connection::borrowed(conn) else {
        return;
    };
    loop {
        match connection.wait_for_event() {
            Ok(xcb::Event::X(xcb::x::Event::KeyPress(event))) => {
                sighandler_handle_key_press(&event);
            }
            Ok(xcb::Event::X(xcb::x::Event::Expose(event))) if event.count() == 0 => {
                sighandler_handle_expose();
            }
            Ok(_) | Err(xcb::Error::Protocol(_)) => {}
            Err(xcb::Error::Connection(_)) => break,
        }
    }
}

pub unsafe fn setup_signal_handler() {
    prepare_crash_messages();
    let mut action: sigaction = ::core::mem::zeroed();
    action.sa_sigaction = handle_signal as *const () as usize;
    action.sa_flags = (SA_NODEFER as u32 | SA_RESETHAND | SA_SIGINFO as u32) as i32;
    sigemptyset(&raw mut action.sa_mask);
    if sigaction(
        SIGQUIT,
        &raw mut action,
        ::core::ptr::null_mut::<sigaction>(),
    ) == -1_i32
        || sigaction(
            SIGILL,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -1_i32
        || sigaction(
            SIGABRT,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -1_i32
        || sigaction(
            SIGFPE,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -1_i32
        || sigaction(
            SIGSEGV,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -1_i32
    {
        ::log::error!("ERROR: Could not setup signal handler.\n");
    }
}
pub const EXIT_FAILURE: i32 = 1_i32;
unsafe extern "C" fn run_static_initializers() {
    dialogs = dialogs_head {
        tqh_first: ::core::ptr::null_mut::<dialog_t>(),
        tqh_last: &raw mut dialogs.tqh_first,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::i3_restart;
use crate::xcb::get_visualid_by_depth;
use i3_common::dpi::logical_px;
use i3_common::path_exists::exists;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{
    close, dup2, execvp, fork, free, getenv, getpid, pipe, sigaction, sigemptyset, siginfo_t,
    waitpid,
};

use i3_common::native_stdio::err;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Rect;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::root_depth;
use crate::application::root_screen_info;
use crate::application::start_argv;
use crate::config::config;
use crate::xcb::create_window;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::font::predict_text_width;

use crate::xcb::xcb_numlock_mask;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backtrace_argument_bytes_match_native_printf_without_running_gdb() {
        for directory in [b"".as_slice(), b".work", b"dir/\xff", b"dir\0ignored\xff"] {
            let prefix = directory.split(|&byte| byte == 0).next().unwrap();
            let native_directory = std::ffi::CString::new(prefix).unwrap();
            for pid in [i32::MIN, -1, 0, 1, i32::MAX] {
                for suffix in [i32::MIN, -1, 0, 1, i32::MAX] {
                    let filename = backtrace_filename(directory, pid, suffix);
                    let mut native = [0u8; 256];
                    let count = unsafe {
                        libc::snprintf(
                            native.as_mut_ptr().cast(),
                            native.len(),
                            c"%s/i3-backtrace.%d.%d.txt".as_ptr(),
                            native_directory.as_ptr(),
                            pid,
                            suffix,
                        )
                    };
                    assert!(count >= 0 && (count as usize) < native.len());
                    assert_eq!(filename, &native[..count as usize]);
                    let native_filename = std::ffi::CString::new(filename.clone()).unwrap();
                    let count = unsafe {
                        libc::snprintf(
                            native.as_mut_ptr().cast(),
                            native.len(),
                            c"set logging file %s".as_ptr(),
                            native_filename.as_ptr(),
                        )
                    };
                    assert!(count >= 0 && (count as usize) < native.len());
                    let mut bounded_filename = filename;
                    bounded_filename.extend_from_slice(b"\0ignored\xff");
                    assert_eq!(
                        backtrace_log_command(&bounded_filename),
                        &native[..count as usize],
                    );
                }
            }
        }
    }

    #[test]
    fn crash_value_helpers_are_stack_only_and_preserve_native_projection() {
        let r = dialog_geometry(
            Rect {
                x: 10,
                y: 20,
                width: 3,
                height: 5,
            },
            8,
            10,
        );
        assert_eq!((r.x, r.y, r.width, r.height), (7, 17, 8, 10));
        assert!(
            matches!(dialog_attributes(4, 7), [x::Cw::BackPixel(4), x::Cw::BorderPixel(4), x::Cw::OverrideRedirect(true), x::Cw::Colormap(id)] if id.resource_id() == 7)
        );
        assert_eq!(crash_action(b'b' as u32), Some(CrashAction::Backtrace));
        assert_eq!(crash_action(b'B' as u32), None);
        assert_eq!(backtrace_color(-1), "#AA0000");
        assert_eq!(backtrace_color(1), "#00AA00");
        assert!(child_succeeded(0));
        assert!(!child_succeeded(256));
        assert!(!child_succeeded(9));
    }
}
