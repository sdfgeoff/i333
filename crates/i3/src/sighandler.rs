use crate::randr::outputs;
use crate::xcursor::Cursor;
use i3_common::draw_util::draw_util_text;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 */
// Mechanical file-by-file Rust translation of src/sighandler.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub use libc::sighandler_t as __sighandler_t;

pub type i3String = _i3String;

pub type i3Font = Font;

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub const M_BELOW: ::core::ffi::c_uint = 2;
pub const M_ASSIGN_WS: ::core::ffi::c_uint = 1;
pub const M_HERE: ::core::ffi::c_uint = 0;

pub const M_DOCK_BOTTOM: ::core::ffi::c_int = 3;
pub const M_DOCK_TOP: ::core::ffi::c_int = 2;
pub const M_DOCK_ANY: ::core::ffi::c_int = 1;
pub const M_NODOCK: ::core::ffi::c_int = 0;
pub const M_DONTCHECK: ::core::ffi::c_int = -1;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;
pub const A_COMMAND: ::core::ffi::c_uint = 1;
pub const A_ANY: ::core::ffi::c_uint = 0;

pub const CT_DOCKAREA: ::core::ffi::c_uint = 5;
pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_FLOATING_CON: ::core::ffi::c_uint = 3;
pub const CT_CON: ::core::ffi::c_uint = 2;
pub const CT_OUTPUT: ::core::ffi::c_uint = 1;
pub const CT_ROOT: ::core::ffi::c_uint = 0;

pub type Output = xoutput;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dialog_t {
    pub id: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub dims: Rect,
    pub surface: surface_t,
    pub dialogs: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut dialog_t,
    pub tqe_prev: *mut *mut dialog_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dialogs_head {
    pub tqh_first: *mut dialog_t,
    pub tqh_last: *mut *mut dialog_t,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SIG_DFL: __sighandler_t = libc::SIG_DFL;
pub const SIGILL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGABRT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SA_SIGINFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SA_NODEFER: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SA_RESETHAND: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut dialogs: dialogs_head = dialogs_head {
    tqh_first: ::core::ptr::null::<dialog_t>() as *mut dialog_t,
    tqh_last: ::core::ptr::null::<*mut dialog_t>() as *mut *mut dialog_t,
};
static mut raised_signal: ::core::ffi::c_int = 0;
static mut backtrace_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut message_intro: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_intro2: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_backtrace: *mut i3String =
    ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_restart: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut message_option_forget: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut dialog_width: ::core::ffi::c_int = 0;
static mut dialog_height: ::core::ffi::c_int = 0;
static mut border_width: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut margin: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe fn sighandler_backtrace() -> ::core::ffi::c_int {
    let mut tmpdir: *mut ::core::ffi::c_char = getenv(c"TMPDIR".as_ptr());
    if tmpdir.is_null() {
        tmpdir = c"/tmp".as_ptr().cast_mut();
    }
    let pid_parent: libc::pid_t = getpid() as libc::pid_t;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        free(filename as *mut ::core::ffi::c_void);
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut filename,
            c"%s/i3-backtrace.%d.%d.txt".as_ptr(),
            tmpdir,
            pid_parent,
            suffix,
        );
        suffix += 1;
        if !path_exists(filename) {
            break;
        }
    }
    let pid_gdb: libc::pid_t = fork() as libc::pid_t;
    if pid_gdb < 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - Failed to fork for GDB\n".as_ptr().cast_mut(),
            c"../src/sighandler.c".as_ptr(),
            c"sighandler_backtrace".as_ptr(),
            69 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    } else if pid_gdb == 0 as ::core::ffi::c_int {
        let mut stdin_pipe: [::core::ffi::c_int; 2] = [0; 2];
        let mut stdout_pipe: [::core::ffi::c_int; 2] = [0; 2];
        if pipe(&raw mut stdin_pipe as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
            errorlog(c"ERROR: Failed to init stdin_pipe\n".as_ptr().cast_mut());
            return -(1 as ::core::ffi::c_int);
        }
        if pipe(&raw mut stdout_pipe as *mut ::core::ffi::c_int) == -(1 as ::core::ffi::c_int) {
            errorlog(c"ERROR: Failed to init stdout_pipe\n".as_ptr().cast_mut());
            return -(1 as ::core::ffi::c_int);
        }
        close(STDIN_FILENO);
        close(STDOUT_FILENO);
        close(STDERR_FILENO);
        dup2(stdin_pipe[0 as ::core::ffi::c_int as usize], STDIN_FILENO);
        dup2(stdout_pipe[1 as ::core::ffi::c_int as usize], STDOUT_FILENO);
        let mut pid_s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut gdb_log_cmd: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut pid_s, c"%d".as_ptr(), pid_parent);
        sasprintf(
            &raw mut gdb_log_cmd,
            c"set logging file %s".as_ptr(),
            filename,
        );
        let mut args: [*mut ::core::ffi::c_char; 15] = [
            c"gdb".as_ptr().cast_mut(),
            *start_argv.offset(0 as ::core::ffi::c_int as isize),
            c"-p".as_ptr().cast_mut(),
            pid_s,
            c"-batch".as_ptr().cast_mut(),
            c"-nx".as_ptr().cast_mut(),
            c"-ex".as_ptr().cast_mut(),
            gdb_log_cmd,
            c"-ex".as_ptr().cast_mut(),
            c"set logging on".as_ptr().cast_mut(),
            c"-ex".as_ptr().cast_mut(),
            c"bt full".as_ptr().cast_mut(),
            c"-ex".as_ptr().cast_mut(),
            c"quit".as_ptr().cast_mut(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        execvp(
            args[0 as ::core::ffi::c_int as usize],
            &raw mut args as *mut *mut ::core::ffi::c_char as *const *const ::core::ffi::c_char,
        );
        err(EXIT_FAILURE, c"execvp(gdb)".as_ptr());
    }
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    waitpid(
        pid_gdb as libc::pid_t,
        &raw mut status,
        0 as ::core::ffi::c_int,
    );
    if !(status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - GDB did not run properly\n".as_ptr().cast_mut(),
            c"../src/sighandler.c".as_ptr(),
            c"sighandler_backtrace".as_ptr(),
            123 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    } else if !path_exists(filename) {
        debuglog(
            c"%s:%s:%d - GDB executed successfully, but no backtrace was generated\n"
                .as_ptr()
                .cast_mut(),
            c"../src/sighandler.c".as_ptr(),
            c"sighandler_backtrace".as_ptr(),
            126 as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    1 as ::core::ffi::c_int
}
unsafe fn sighandler_setup() {
    border_width = logical_px(border_width);
    margin = logical_px(margin);
    let num_lines: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    message_intro =
        i3string_from_utf8(c"i3 has just crashed. Please report a bug for this.".as_ptr());
    message_intro2 = i3string_from_utf8(
        c"To debug this problem, you can either attach gdb or choose from the following options:"
            .as_ptr(),
    );
    message_option_backtrace =
        i3string_from_utf8(c"- 'b' to save a backtrace (requires gdb)".as_ptr());
    message_option_restart = i3string_from_utf8(c"- 'r' to restart i3 in-place".as_ptr());
    message_option_forget =
        i3string_from_utf8(c"- 'f' to forget the previous layout and restart i3".as_ptr());
    let width_longest_message: ::core::ffi::c_int = predict_text_width(message_intro2);
    dialog_width = width_longest_message
        + 2 as ::core::ffi::c_int * border_width
        + 2 as ::core::ffi::c_int * margin;
    dialog_height = num_lines * config.font.height
        + 2 as ::core::ffi::c_int * border_width
        + 2 as ::core::ffi::c_int * margin;
}
unsafe fn sighandler_create_dialogs() {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).active {
            let dialog: *mut dialog_t =
                scalloc(1 as usize, ::core::mem::size_of::<dialog_t>() as usize) as *mut dialog_t;
            (*dialog).dialogs.tqe_next = ::core::ptr::null_mut::<dialog_t>();
            (*dialog).dialogs.tqe_prev = dialogs.tqh_last;
            *dialogs.tqh_last = dialog as *mut dialog_t;
            dialogs.tqh_last = &raw mut (*dialog).dialogs.tqe_next;
            let visual: xcb_visualid_t = get_visualid_by_depth(root_depth as u16);
            (*dialog).colormap = xcb_generate_id(conn) as xcb_colormap_t;
            xcb_create_colormap(
                conn,
                XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as u8,
                (*dialog).colormap,
                root,
                visual,
            );
            let mut mask: u32 = 0 as u32;
            let mut values: [u32; 4] = [0; 4];
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            mask |= XCB_CW_BACK_PIXEL as ::core::ffi::c_int as u32;
            let fresh0 = i;
            i = i + 1;
            values[fresh0 as usize] = (*root_screen).black_pixel;
            mask |= XCB_CW_BORDER_PIXEL as ::core::ffi::c_int as u32;
            let fresh1 = i;
            i = i + 1;
            values[fresh1 as usize] = (*root_screen).black_pixel;
            mask |= XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
            let fresh2 = i;
            i = i + 1;
            values[fresh2 as usize] = 1 as u32;
            mask |= XCB_CW_COLORMAP as ::core::ffi::c_int as u32;
            let fresh3 = i;
            i = i + 1;
            values[fresh3 as usize] = (*dialog).colormap as u32;
            (*dialog).dims.x = (*output)
                .rect
                .x
                .wrapping_add((*output).rect.width.wrapping_div(2 as u32));
            (*dialog).dims.y = (*output)
                .rect
                .y
                .wrapping_add((*output).rect.height.wrapping_div(2 as u32));
            (*dialog).dims.width = dialog_width as u32;
            (*dialog).dims.height = dialog_height as u32;
            (*dialog).dims.x = (*dialog)
                .dims
                .x
                .wrapping_sub((*dialog).dims.width.wrapping_div(2 as u32));
            (*dialog).dims.y = (*dialog)
                .dims
                .y
                .wrapping_sub((*dialog).dims.height.wrapping_div(2 as u32));
            (*dialog).id = create_window(
                conn,
                (*dialog).dims,
                root_depth as u16,
                visual,
                XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
                Cursor::Pointer,
                true_0 != 0,
                mask,
                &raw mut values as *mut u32,
            );
            draw_util_surface_init(
                conn,
                &raw mut (*dialog).surface,
                (*dialog).id as xcb_drawable_t,
                get_visualtype_by_id(visual),
                (*dialog).dims.width as ::core::ffi::c_int,
                (*dialog).dims.height as ::core::ffi::c_int,
            );
            xcb_grab_keyboard(
                conn,
                false_0 as u8,
                (*dialog).id,
                XCB_CURRENT_TIME as xcb_timestamp_t,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
            );
            xcb_grab_pointer(
                conn,
                false_0 as u8,
                (*dialog).id,
                XCB_NONE as u16,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
                XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
                (*dialog).id,
                XCB_NONE as xcb_cursor_t,
                XCB_CURRENT_TIME as xcb_timestamp_t,
            );
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    sighandler_handle_expose();
    xcb_flush(conn);
}
unsafe fn sighandler_destroy_dialogs() {
    while !dialogs.tqh_first.is_null() {
        let dialog: *mut dialog_t = dialogs.tqh_first as *mut dialog_t;
        xcb_free_colormap(conn, (*dialog).colormap);
        draw_util_surface_free(conn, &raw mut (*dialog).surface);
        xcb_destroy_window(conn, (*dialog).id);
        if !(*dialog).dialogs.tqe_next.is_null() {
            (*(*dialog).dialogs.tqe_next).dialogs.tqe_prev = (*dialog).dialogs.tqe_prev;
        } else {
            dialogs.tqh_last = (*dialog).dialogs.tqe_prev;
        }
        *(*dialog).dialogs.tqe_prev = (*dialog).dialogs.tqe_next;
        free(dialog as *mut ::core::ffi::c_void);
    }
    xcb_flush(conn);
}
unsafe fn sighandler_handle_expose() {
    let mut current: *mut dialog_t = ::core::ptr::null_mut::<dialog_t>();
    current = dialogs.tqh_first as *mut dialog_t;
    while !current.is_null() {
        sighandler_draw_dialog(current);
        current = (*current).dialogs.tqe_next as *mut dialog_t;
    }
    xcb_flush(conn);
}
unsafe fn sighandler_draw_dialog(dialog: *mut dialog_t) {
    let black: color_t = draw_util_hex_to_color(c"#000000".as_ptr()) as color_t;
    let white: color_t = draw_util_hex_to_color(c"#FFFFFF".as_ptr()) as color_t;
    let red: color_t = draw_util_hex_to_color(c"#FF0000".as_ptr()) as color_t;
    draw_util_clear_surface(&raw mut (*dialog).surface, red);
    draw_util_rectangle(
        &raw mut (*dialog).surface,
        black,
        border_width as ::core::ffi::c_double,
        border_width as ::core::ffi::c_double,
        (*dialog)
            .dims
            .width
            .wrapping_sub((2 as ::core::ffi::c_int * border_width) as u32)
            as ::core::ffi::c_double,
        (*dialog)
            .dims
            .height
            .wrapping_sub((2 as ::core::ffi::c_int * border_width) as u32)
            as ::core::ffi::c_double,
    );
    let mut y: ::core::ffi::c_int = border_width + margin;
    let x: ::core::ffi::c_int = border_width + margin;
    let max_width: ::core::ffi::c_int = (*dialog)
        .dims
        .width
        .wrapping_sub((2 as ::core::ffi::c_int * x) as u32)
        as ::core::ffi::c_int;
    draw_util_text(
        message_intro,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_intro2,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    let mut bt_color: *mut ::core::ffi::c_char = c"#FFFFFF".as_ptr().cast_mut();
    if backtrace_done < 0 as ::core::ffi::c_int {
        bt_color = c"#AA0000".as_ptr().cast_mut();
    } else if backtrace_done > 0 as ::core::ffi::c_int {
        bt_color = c"#00AA00".as_ptr().cast_mut();
    }
    draw_util_text(
        message_option_backtrace,
        &raw mut (*dialog).surface,
        draw_util_hex_to_color(bt_color),
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_option_restart,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
    draw_util_text(
        message_option_forget,
        &raw mut (*dialog).surface,
        white,
        black,
        x,
        y,
        max_width,
    );
    y += config.font.height;
}
unsafe fn sighandler_handle_key_press(event: *mut xcb_key_press_event_t) {
    let mut state: u16 = (*event).state;
    state = (state as ::core::ffi::c_uint & !xcb_numlock_mask) as u16;
    let sym: xcb_keysym_t =
        xcb_key_press_lookup_keysym(keysyms, event, state as ::core::ffi::c_int);
    if sym == 'b' as i32 as xcb_keysym_t {
        debuglog(
            c"%s:%s:%d - User issued core-dump command.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/sighandler.c".as_ptr(),
            c"sighandler_handle_key_press".as_ptr(),
            280 as ::core::ffi::c_int,
        );
        backtrace_done = sighandler_backtrace();
        sighandler_handle_expose();
    } else if sym == 'r' as i32 as xcb_keysym_t {
        sighandler_destroy_dialogs();
        i3_restart(false_0 != 0);
    } else if sym == 'f' as i32 as xcb_keysym_t {
        sighandler_destroy_dialogs();
        i3_restart(true_0 != 0);
    }
}
unsafe extern "C" fn handle_signal(
    sig: ::core::ffi::c_int,
    info: *mut siginfo_t,
    data: *mut ::core::ffi::c_void,
) {
    debuglog(
        c"%s:%s:%d - i3 crashed. SIG: %d\n".as_ptr().cast_mut(),
        c"../src/sighandler.c".as_ptr(),
        c"handle_signal".as_ptr(),
        296 as ::core::ffi::c_int,
        sig,
    );
    let mut action: sigaction = ::core::mem::zeroed();
    action.sa_sigaction = libc::SIG_DFL;
    action.sa_flags = 0 as ::core::ffi::c_int;
    sigemptyset(&raw mut action.sa_mask);
    sigaction(sig, &raw mut action, ::core::ptr::null_mut::<sigaction>());
    raised_signal = sig;
    sighandler_setup();
    sighandler_create_dialogs();
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        let type_0: ::core::ffi::c_int =
            (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
        match type_0 {
            XCB_KEY_PRESS => {
                sighandler_handle_key_press(event as *mut xcb_key_press_event_t);
            }
            XCB_EXPOSE => {
                if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    sighandler_handle_expose();
                }
            }
            _ => {}
        }
        free(event as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn setup_signal_handler() {
    let mut action: sigaction = ::core::mem::zeroed();
    action.sa_sigaction = handle_signal as *const () as usize;
    action.sa_flags = (SA_NODEFER as ::core::ffi::c_uint
        | SA_RESETHAND
        | SA_SIGINFO as ::core::ffi::c_uint) as ::core::ffi::c_int;
    sigemptyset(&raw mut action.sa_mask);
    if sigaction(
        SIGQUIT,
        &raw mut action,
        ::core::ptr::null_mut::<sigaction>(),
    ) == -(1 as ::core::ffi::c_int)
        || sigaction(
            SIGILL,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
        || sigaction(
            SIGABRT,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
        || sigaction(
            SIGFPE,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
        || sigaction(
            SIGSEGV,
            &raw mut action,
            ::core::ptr::null_mut::<sigaction>(),
        ) == -(1 as ::core::ffi::c_int)
    {
        errorlog(
            c"ERROR: Could not setup signal handler.\n"
                .as_ptr()
                .cast_mut(),
        );
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
use i3_common::path_exists::path_exists;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_colormap_t, xcb_create_colormap, xcb_cursor_t, xcb_destroy_window, xcb_drawable_t,
    xcb_expose_event_t, xcb_flush, xcb_free_colormap, xcb_generate_id, xcb_generic_event_t,
    xcb_grab_keyboard, xcb_grab_pointer, xcb_key_press_event_t, xcb_key_press_lookup_keysym,
    xcb_keysym_t, xcb_timestamp_t, xcb_visualid_t, xcb_wait_for_event, xcb_window_t,
    XCB_COLORMAP_ALLOC_NONE, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL, XCB_CW_BORDER_PIXEL,
    XCB_CW_COLORMAP, XCB_CW_OVERRIDE_REDIRECT, XCB_EXPOSE, XCB_GRAB_MODE_ASYNC, XCB_KEY_PRESS,
    XCB_NONE, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::{
    close, dup2, execvp, fork, free, getenv, getpid, pipe, sigaction, sigemptyset, siginfo_t,
    waitpid,
};

use i3_common::native_stdio::err;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Rect;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::keysyms;
use crate::application::root;
use crate::application::root_depth;
use crate::application::root_screen;
use crate::application::start_argv;
use crate::config::config;
use crate::xcb::create_window;
use crate::xcb::get_visualtype_by_id;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::font::predict_text_width;
use i3_common::string::i3string_from_utf8;

use crate::xcb::xcb_numlock_mask;
