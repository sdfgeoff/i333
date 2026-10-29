use crate::types::Direction;
use crate::types::Layout;
use crate::types::Orientation;
use crate::types::Position;
use crate::types::ShutdownReason;
use i3_common::mkdirp::mkdirp;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * util.c: Utility functions, which can be useful everywhere within i3 (see
 *         also libi3).
 *
 */
// Mechanical file-by-file Rust translation of src/util.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use i3_json::compat::{yajl_alloc_funcs, yajl_gen};
use i3_json::compat::{yajl_gen_alloc, yajl_gen_free, yajl_gen_get_buf};
pub use libev_sys::{ev_child, ev_child_start, ev_child_stop, ev_loop, ev_watcher};

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

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEFAULT_DIR_MODE: ::core::ffi::c_int = S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IXGRP: ::core::ffi::c_int = S_IXUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IXOTH: ::core::ffi::c_int = S_IXGRP >> 3 as ::core::ffi::c_int;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;

pub unsafe fn min(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if a < b {
        a
    } else {
        b
    }
}

pub unsafe fn max(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if a > b {
        a
    } else {
        b
    }
}

pub unsafe fn rect_contains(rect: Rect, x: u32, y: u32) -> bool {
    x >= rect.x
        && x <= rect.x.wrapping_add(rect.width)
        && y >= rect.y
        && y <= rect.y.wrapping_add(rect.height)
}

pub unsafe fn rect_add(a: Rect, b: Rect) -> Rect {
    Rect {
        x: a.x.wrapping_add(b.x),
        y: a.y.wrapping_add(b.y),
        width: a.width.wrapping_add(b.width),
        height: a.height.wrapping_add(b.height),
    }
}

pub unsafe fn rect_sub(a: Rect, b: Rect) -> Rect {
    Rect {
        x: a.x.wrapping_sub(b.x),
        y: a.y.wrapping_sub(b.y),
        width: a.width.wrapping_sub(b.width),
        height: a.height.wrapping_sub(b.height),
    }
}

pub unsafe fn rect_sanitize_dimensions(mut rect: Rect) -> Rect {
    rect.width = if rect.width as i32 <= 0 as i32 {
        1 as u32
    } else {
        rect.width
    };
    rect.height = if rect.height as i32 <= 0 as i32 {
        1 as u32
    } else {
        rect.height
    };
    rect
}

pub unsafe fn rect_equals(a: Rect, b: Rect) -> bool {
    a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
}

pub unsafe fn name_is_digits(name: *const ::core::ffi::c_char) -> bool {
    let mut i: usize = 0 as usize;
    while i < strlen(name) {
        if *(*__ctype_b_loc())
            .offset(*name.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & i3_common::native_stdio::CTYPE_DIGIT as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return false_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    true_0 != 0
}

pub unsafe fn layout_from_name(layout_str: *const ::core::ffi::c_char, out: *mut Layout) -> bool {
    if strcmp(layout_str, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
        *out = Layout::Default;
        return true_0 != 0;
    }
    if strcasecmp(layout_str, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int
        || strcasecmp(layout_str, c"stacking".as_ptr()) == 0 as ::core::ffi::c_int
    {
        *out = Layout::Stacked;
        return true_0 != 0;
    }
    if strcasecmp(layout_str, c"tabbed".as_ptr()) == 0 as ::core::ffi::c_int {
        *out = Layout::Tabbed;
        return true_0 != 0;
    }
    if strcasecmp(layout_str, c"splitv".as_ptr()) == 0 as ::core::ffi::c_int {
        *out = Layout::SplitV;
        return true_0 != 0;
    }
    if strcasecmp(layout_str, c"splith".as_ptr()) == 0 as ::core::ffi::c_int {
        *out = Layout::SplitH;
        return true_0 != 0;
    }
    false_0 != 0
}

pub unsafe fn ws_name_to_number(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed_num: ::core::ffi::c_longlong =
        strtoll(name, &raw mut endptr, 10 as ::core::ffi::c_int);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || parsed_num > INT32_MAX as ::core::ffi::c_longlong
        || parsed_num < 0 as ::core::ffi::c_longlong
        || endptr == name as *mut ::core::ffi::c_char
    {
        parsed_num = -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    parsed_num as ::core::ffi::c_int
}

pub unsafe fn update_if_necessary(destination: *mut u32, new_value: u32) -> bool {
    let old_value: u32 = *destination;
    *destination = new_value;
    *destination != old_value
}

pub unsafe fn exec_i3_utility(name: *mut ::core::ffi::c_char, argv: *mut *mut ::core::ffi::c_char) {
    let mut binary: *mut ::core::ffi::c_char = name;
    let ref mut fresh0 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = binary;
    execvp(binary, argv as *const *const ::core::ffi::c_char);
    let pathbuf: *mut ::core::ffi::c_char =
        sstrdup(*start_argv.offset(0 as ::core::ffi::c_int as isize));
    let mut dir: *mut ::core::ffi::c_char = dirname(pathbuf);
    sasprintf(&raw mut binary, c"%s/%s".as_ptr(), dir, name);
    let ref mut fresh1 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh1 = binary;
    execvp(binary, argv as *const *const ::core::ffi::c_char);
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    if readlink(
        c"/proc/self/exe".as_ptr(),
        &raw mut buffer as *mut ::core::ffi::c_char,
        BUFSIZ as usize,
    ) == -(1 as ::core::ffi::c_int) as isize
    {
        warn(c"could not read /proc/self/exe".as_ptr());
        _exit(EXIT_FAILURE);
    }
    dir = dirname(&raw mut buffer as *mut ::core::ffi::c_char);
    sasprintf(&raw mut binary, c"%s/%s".as_ptr(), dir, name);
    let ref mut fresh2 = *argv.offset(0 as ::core::ffi::c_int as isize);
    *fresh2 = binary;
    execvp(binary, argv as *const *const ::core::ffi::c_char);
    warn(c"Could not start %s".as_ptr(), name);
    _exit(2 as ::core::ffi::c_int);
}
unsafe fn add_argument(
    original: *mut *mut ::core::ffi::c_char,
    opt_char: *mut ::core::ffi::c_char,
    opt_arg: *mut ::core::ffi::c_char,
    opt_name: *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut num_args: ::core::ffi::c_int = 0;
    num_args = 0 as ::core::ffi::c_int;
    while !(*original.offset(num_args as isize)).is_null() {
        num_args += 1;
    }
    let result: *mut *mut ::core::ffi::c_char = scalloc(
        (num_args + 3 as ::core::ffi::c_int) as usize,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
    ) as *mut *mut ::core::ffi::c_char;
    let mut write_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skip_next: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num_args {
        if skip_next {
            skip_next = false_0 != 0;
        } else if strcmp(*original.offset(i as isize), opt_char) == 0
            || !opt_name.is_null() && strcmp(*original.offset(i as isize), opt_name) == 0
        {
            if !opt_arg.is_null() {
                skip_next = true_0 != 0;
            }
        } else {
            let fresh3 = write_index;
            write_index = write_index + 1;
            let ref mut fresh4 = *result.offset(fresh3 as isize);
            *fresh4 = *original.offset(i as isize);
        }
        i += 1;
    }
    let fresh5 = write_index;
    write_index = write_index + 1;
    let ref mut fresh6 = *result.offset(fresh5 as isize);
    *fresh6 = opt_char;
    let ref mut fresh7 = *result.offset(write_index as isize);
    *fresh7 = opt_arg;
    result
}
unsafe fn store_restart_layout() -> *mut ::core::ffi::c_char {
    let prev_locale: *mut ::core::ffi::c_char = sstrdup(setlocale(
        LC_NUMERIC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ));
    setlocale(LC_NUMERIC, c"C".as_ptr());
    let gen: yajl_gen = yajl_gen_alloc(::core::ptr::null::<yajl_alloc_funcs>());
    dump_node(gen, croot, true_0 != 0);
    setlocale(LC_NUMERIC, prev_locale);
    free(prev_locale as *mut ::core::ffi::c_void);
    let mut payload: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: usize = 0;
    yajl_gen_get_buf(gen, &raw mut payload, &raw mut length);
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if config.restart_state_path.is_null() {
        filename = get_process_filename(c"restart-state".as_ptr());
        if filename.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        filename = resolve_tilde(config.restart_state_path);
    }
    let filenamecopy: *mut ::core::ffi::c_char = sstrdup(filename);
    let base: *mut ::core::ffi::c_char = dirname(filenamecopy);
    debuglog(
        c"%s:%s:%d - Creating \"%s\" for storing the restart layout\n"
            .as_ptr()
            .cast_mut(),
        c"../src/util.c".as_ptr(),
        c"store_restart_layout".as_ptr(),
        253 as ::core::ffi::c_int,
        base,
    );
    if mkdirp(base, DEFAULT_DIR_MODE as libc::mode_t) != 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Could not create \"%s\" for storing the restart layout, layout will be lost.\n".as_ptr().cast_mut(),
            base,
        );
    }
    free(filenamecopy as *mut ::core::ffi::c_void);
    let fd: ::core::ffi::c_int = open(filename, O_WRONLY | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    if fd == -(1 as ::core::ffi::c_int) {
        perror(c"open()".as_ptr());
        free(filename as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if writeall(fd, payload as *const ::core::ffi::c_void, length)
        == -(1 as ::core::ffi::c_int) as isize
    {
        errorlog(
            c"ERROR: Could not write restart layout to \"%s\", layout will be lost: %s\n"
                .as_ptr()
                .cast_mut(),
            filename,
            strerror(*__errno_location()),
        );
        free(filename as *mut ::core::ffi::c_void);
        close(fd);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    close(fd);
    if length > 0 as usize {
        debuglog(
            c"%s:%s:%d - layout: %.*s\n".as_ptr().cast_mut(),
            c"../src/util.c".as_ptr(),
            c"store_restart_layout".as_ptr(),
            276 as ::core::ffi::c_int,
            length as ::core::ffi::c_int,
            payload,
        );
    }
    yajl_gen_free(gen);
    filename
}

pub unsafe fn i3_restart(forget_layout: bool) {
    let restart_filename: *mut ::core::ffi::c_char = if forget_layout as ::core::ffi::c_int != 0 {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        store_restart_layout()
    };
    kill_nagbar(config_error_nagbar_pid, true_0 != 0);
    kill_nagbar(command_error_nagbar_pid, true_0 != 0);
    restore_geometry();
    ipc_shutdown(ShutdownReason::Restart, -(1 as ::core::ffi::c_int));
    verboselog(
        c"restarting \"%s\"...\n".as_ptr().cast_mut(),
        *start_argv.offset(0 as ::core::ffi::c_int as isize),
    );
    start_argv = add_argument(
        start_argv,
        c"-a".as_ptr().cast_mut(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if get_debug_logging() {
        start_argv = add_argument(
            start_argv,
            c"-d".as_ptr().cast_mut(),
            c"all".as_ptr().cast_mut(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
    if !restart_filename.is_null() {
        start_argv = add_argument(
            start_argv,
            c"--restart".as_ptr().cast_mut(),
            restart_filename,
            c"-r".as_ptr().cast_mut(),
        );
    }
    execvp(
        *start_argv.offset(0 as ::core::ffi::c_int as isize),
        start_argv as *const *const ::core::ffi::c_char,
    );
}

pub unsafe fn pango_escape_markup(mut input: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if !font_is_pango() {
        return input;
    }
    if input.is_null() {
        return input;
    }
    // The renderer receives owned, nul-terminated text. Repair invalid UTF-8
    // through the shared text model, escape in Rust, and retain malloc ownership
    // for the existing caller while it migrates to owned strings.
    let text = i3_common::string::I3String::from_cstr(std::ffi::CStr::from_ptr(input));
    let escaped_text =
        i3_common::string::escape_markup(text.as_str().expect("UTF-8 source is initialized"));
    let escaped =
        i3_common::safewrappers::sstrndup(escaped_text.as_ptr().cast(), escaped_text.len());
    free(input as *mut ::core::ffi::c_void);
    input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    escaped
}
unsafe extern "C" fn nagbar_exited(
    loop_0: *mut ev_loop,
    watcher: *mut ev_child,
    revents: ::core::ffi::c_int,
) {
    ev_child_stop(loop_0, watcher);
    let exitcode: ::core::ffi::c_int =
        ((*watcher).rstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    if !((*watcher).rstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
        errorlog(
            c"ERROR: i3-nagbar (%d) did not exit normally. This is not an error if the config was reloaded while a nagbar was active.\n".as_ptr().cast_mut(),
            (*watcher).pid,
        );
    } else if exitcode != 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: i3-nagbar (%d) process exited with status %d\n"
                .as_ptr()
                .cast_mut(),
            (*watcher).pid,
            exitcode,
        );
    } else {
        debuglog(
            c"%s:%s:%d - i3-nagbar (%d) process exited with status %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/util.c".as_ptr(),
            c"nagbar_exited".as_ptr(),
            348 as ::core::ffi::c_int,
            (*watcher).pid,
            exitcode,
        );
    }
    let nagbar_pid: *mut libc::pid_t = (*watcher).data as *mut libc::pid_t;
    if *nagbar_pid == (*watcher).pid {
        *nagbar_pid = -(1 as ::core::ffi::c_int) as libc::pid_t;
    }
}

pub unsafe fn start_nagbar(nagbar_pid: *mut libc::pid_t, argv: *mut *mut ::core::ffi::c_char) {
    if *nagbar_pid != -(1 as ::core::ffi::c_int) {
        debuglog(
            c"%s:%s:%d - i3-nagbar already running (PID %d), not starting again.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/util.c".as_ptr(),
            c"start_nagbar".as_ptr(),
            368 as ::core::ffi::c_int,
            *nagbar_pid,
        );
        return;
    }
    *nagbar_pid = fork() as libc::pid_t;
    if *nagbar_pid == -(1 as ::core::ffi::c_int) {
        warn(c"Could not fork()".as_ptr());
        return;
    }
    if *nagbar_pid == 0 as ::core::ffi::c_int {
        exec_i3_utility(c"i3-nagbar".as_ptr().cast_mut(), argv);
    }
    debuglog(
        c"%s:%s:%d - Starting i3-nagbar with PID %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/util.c".as_ptr(),
        c"start_nagbar".as_ptr(),
        383 as ::core::ffi::c_int,
        *nagbar_pid,
    );
    let child: *mut ev_child =
        smalloc(::core::mem::size_of::<ev_child>() as usize) as *mut ev_child;
    let ref mut fresh8 = (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh8 = 0 as ::core::ffi::c_int;
    (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh8;
    (*(child as *mut ::core::ffi::c_void as *mut ev_watcher)).priority = 0 as ::core::ffi::c_int;
    (*child).cb = Some(
        nagbar_exited
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(child as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*child).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>,
        >() as usize,
    );
    (*child).pid = *nagbar_pid as ::core::ffi::c_int;
    (*child).flags = (0 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
    (*child).data = nagbar_pid as *mut ::core::ffi::c_void;
    ev_child_start(main_loop, child);
}

pub unsafe fn kill_nagbar(nagbar_pid: libc::pid_t, wait_for_it: bool) {
    if nagbar_pid == -(1 as ::core::ffi::c_int) {
        return;
    }
    if kill(nagbar_pid as libc::pid_t, SIGTERM) == -(1 as ::core::ffi::c_int) {
        warn(c"kill(configerror_nagbar) failed".as_ptr());
    }
    if !wait_for_it {
        return;
    }
    waitpid(
        nagbar_pid as libc::pid_t,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    );
}

pub unsafe fn parse_long(
    str: *const ::core::ffi::c_char,
    out: *mut ::core::ffi::c_long,
    base: ::core::ffi::c_int,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let result: ::core::ffi::c_long = strtol(str, &raw mut end, base);
    if result == LONG_MIN
        || result == LONG_MAX
        || result < 0 as ::core::ffi::c_long
        || !end.is_null() && *end as ::core::ffi::c_int != '\0' as i32
    {
        *out = result;
        return false_0 != 0;
    }
    *out = result;
    true_0 != 0
}

pub unsafe fn slurp(path: *const ::core::ffi::c_char, buf: *mut *mut ::core::ffi::c_char) -> isize {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    f = fopen(path, c"r".as_ptr()) as *mut FILE;
    if f.is_null() {
        errorlog(
            c"ERROR: Cannot open file \"%s\": %s\n".as_ptr().cast_mut(),
            path,
            strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int) as isize;
    }
    let mut stbuf: stat = ::core::mem::zeroed();
    if fstat(fileno(f), &raw mut stbuf) != 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: Cannot fstat() \"%s\": %s\n".as_ptr().cast_mut(),
            path,
            strerror(*__errno_location()),
        );
        fclose(f);
        return -(1 as ::core::ffi::c_int) as isize;
    }
    *buf = scalloc((stbuf.st_size + 1 as libc::off_t) as usize, 1 as usize)
        as *mut ::core::ffi::c_char;
    let n: usize = fread(
        *buf as *mut ::core::ffi::c_void,
        1 as usize,
        stbuf.st_size as usize,
        f,
    ) as usize;
    fclose(f);
    if n as isize != stbuf.st_size as isize {
        errorlog(
            c"ERROR: File \"%s\" could not be read entirely: got %zd, want %li\n"
                .as_ptr()
                .cast_mut(),
            path,
            n,
            stbuf.st_size as i64,
        );
        free(*buf as *mut ::core::ffi::c_void);
        *buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return -(1 as ::core::ffi::c_int) as isize;
    }
    n as isize
}

pub fn orientation_from_direction(direction: Direction) -> Orientation {
    match direction {
        Direction::Left | Direction::Right => Orientation::Horizontal,
        Direction::Up | Direction::Down => Orientation::Vertical,
    }
}

pub fn position_from_direction(direction: Direction) -> Position {
    match direction {
        Direction::Left | Direction::Up => Position::Before,
        Direction::Right | Direction::Down => Position::After,
    }
}

pub fn direction_from_orientation_position(
    orientation: Orientation,
    position: Position,
) -> Direction {
    match (orientation, position) {
        (Orientation::Horizontal, Position::Before) => Direction::Left,
        (Orientation::Horizontal, Position::After) => Direction::Right,
        (Orientation::None | Orientation::Vertical, Position::Before) => Direction::Up,
        (Orientation::None | Orientation::Vertical, Position::After) => Direction::Down,
    }
}

pub fn direction_to_string(direction: Direction) -> *const ::core::ffi::c_char {
    match direction {
        Direction::Left => c"left".as_ptr(),
        Direction::Right => c"right".as_ptr(),
        Direction::Up => c"up".as_ptr(),
        Direction::Down => c"down".as_ptr(),
    }
}

pub fn position_to_string(position: Position) -> *const ::core::ffi::c_char {
    match position {
        Position::Before => c"before".as_ptr(),
        Position::After => c"after".as_ptr(),
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ipc::ipc_shutdown;
use crate::log::get_debug_logging;
use crate::manage::restore_geometry;
use i3_common::font::font_is_pango;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::writeall;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{
    __errno_location, _exit, close, dirname, execvp, fclose, fileno, fopen, fork, fread, free,
    fstat, kill, memmove, open, perror, readlink, setlocale, stat, strcasecmp, strcmp, strerror,
    strlen, strtol, strtoll, waitpid, FILE,
};

use i3_common::native_stdio::{__ctype_b_loc, warn};

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Rect;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::start_argv;
use crate::bindings::command_error_nagbar_pid;
use crate::config::config;
use crate::config_parser::config_error_nagbar_pid;
use crate::ipc::dump_node;
use crate::tree::croot;
use i3_common::get_process_filename::get_process_filename;
use i3_common::resolve_tilde::resolve_tilde;
