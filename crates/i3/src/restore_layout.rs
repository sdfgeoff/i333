use i3_common::draw_util::draw_util_text;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * restore_layout.c: Everything for restored containers that is not pure state
 *                   parsing (which can be found in load_layout.c).
 *
 *
 */
// Mechanical file-by-file Rust translation of src/restore_layout.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop,
    ev_watcher, EV_READ, EV__IOFDSET,
};
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_2 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_2 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_2 = 0;

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_7 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_7 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_7 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_7 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_9 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_9 = 1;
pub const M_HERE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_10 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_10 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_10 = 4;
pub const WM_TILING: C2RustUnnamed_10 = 3;
pub const WM_TILING_USER: C2RustUnnamed_10 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_10 = 1;
pub const WM_ANY: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_11 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_11 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_11 = 1;
pub const M_NODOCK: C2RustUnnamed_11 = 0;
pub const M_DONTCHECK: C2RustUnnamed_11 = -1;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_12 = 1;
pub const U_LATEST: C2RustUnnamed_12 = 0;
pub const U_DONTCHECK: C2RustUnnamed_12 = -1;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_13 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_13 = 1;
pub const W_NODOCK: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_16 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_16 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_16 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_16 = 2;
pub const A_COMMAND: C2RustUnnamed_16 = 1;
pub const A_ANY: C2RustUnnamed_16 = 0;

pub type C2RustUnnamed_18 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_18 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_18 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_18 = 3;
pub const CT_CON: C2RustUnnamed_18 = 2;
pub const CT_OUTPUT: C2RustUnnamed_18 = 1;
pub const CT_ROOT: C2RustUnnamed_18 = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_20 = 3;
pub const PDF_IGNORE: C2RustUnnamed_20 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_20 = 1;
pub const PDF_SMART: C2RustUnnamed_20 = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_21 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_21 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_22 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_22 = 2;
pub const FOWA_URGENT: C2RustUnnamed_22 = 1;
pub const FOWA_SMART: C2RustUnnamed_22 = 0;
pub type xcursor_cursor_t = ::core::ffi::c_uint;
pub const XCURSOR_CURSOR_MAX: xcursor_cursor_t = 9;
pub const XCURSOR_CURSOR_MOVE: xcursor_cursor_t = 8;
pub const XCURSOR_CURSOR_WATCH: xcursor_cursor_t = 7;
pub const XCURSOR_CURSOR_BOTTOM_RIGHT_CORNER: xcursor_cursor_t = 6;
pub const XCURSOR_CURSOR_BOTTOM_LEFT_CORNER: xcursor_cursor_t = 5;
pub const XCURSOR_CURSOR_TOP_RIGHT_CORNER: xcursor_cursor_t = 4;
pub const XCURSOR_CURSOR_TOP_LEFT_CORNER: xcursor_cursor_t = 3;
pub const XCURSOR_CURSOR_RESIZE_VERTICAL: xcursor_cursor_t = 2;
pub const XCURSOR_CURSOR_RESIZE_HORIZONTAL: xcursor_cursor_t = 1;
pub const XCURSOR_CURSOR_POINTER: xcursor_cursor_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct placeholder_state {
    pub window: xcb_window_t,
    pub con: *mut Con,
    pub rect: Rect,
    pub surface: surface_t,
    pub state: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut placeholder_state,
    pub tqe_prev: *mut *mut placeholder_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_head {
    pub tqh_first: *mut placeholder_state,
    pub tqh_last: *mut *mut placeholder_state,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut state_head: state_head = state_head {
    tqh_first: ::core::ptr::null::<placeholder_state>() as *mut placeholder_state,
    tqh_last: ::core::ptr::null::<*mut placeholder_state>() as *mut *mut placeholder_state,
};
static mut restore_conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut xcb_watcher: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut xcb_prepare: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;
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
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    if xcb_connection_has_error(restore_conn) != 0 {
        debuglog(
            c"%s:%s:%d - restore X11 connection has an error, reconnecting\n"
                .as_ptr()
                .cast_mut(),
            c"../src/restore_layout.c".as_ptr(),
            c"restore_xcb_prepare_cb".as_ptr(),
            53 as ::core::ffi::c_int,
        );
        restore_connect();
        return;
    }
    loop {
        event = xcb_poll_for_event(restore_conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            debuglog(
                c"%s:%s:%d - X11 Error received (probably harmless)! sequence 0x%x, error_code = %d\n".as_ptr().cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_xcb_prepare_cb".as_ptr(),
                62 as ::core::ffi::c_int,
                (*error).sequence as ::core::ffi::c_int,
                (*error).error_code as ::core::ffi::c_int,
            );
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            restore_handle_event(type_0, event);
            free(event as *mut ::core::ffi::c_void);
        }
    }
    xcb_flush(restore_conn);
}

pub unsafe fn restore_connect() {
    if !restore_conn.is_null() {
        ev_io_stop(main_loop, xcb_watcher);
        ev_prepare_stop(main_loop, xcb_prepare);
        while !state_head.tqh_first.is_null() {
            let state: *mut placeholder_state = state_head.tqh_first as *mut placeholder_state;
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            free(state as *mut ::core::ffi::c_void);
        }
        xcb_disconnect(restore_conn);
        free(xcb_watcher as *mut ::core::ffi::c_void);
        free(xcb_prepare as *mut ::core::ffi::c_void);
    }
    let mut screen: ::core::ffi::c_int = 0;
    restore_conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if restore_conn.is_null() || xcb_connection_has_error(restore_conn) != 0 {
        if !restore_conn.is_null() {
            xcb_disconnect(restore_conn);
        }
        errx(EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    xcb_watcher = scalloc(1 as size_t, ::core::mem::size_of::<ev_io>() as size_t) as *mut ev_io;
    xcb_prepare =
        scalloc(1 as size_t, ::core::mem::size_of::<ev_prepare>() as size_t) as *mut ev_prepare;
    let ref mut fresh0 = (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh0 = 0 as ::core::ffi::c_int;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh0;
    (*(xcb_watcher as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_watcher).cb = Some(
        restore_xcb_got_event
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_watcher as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_watcher).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    (*xcb_watcher).fd = xcb_get_file_descriptor(restore_conn);
    (*xcb_watcher).events = EV_READ as ::core::ffi::c_int | EV__IOFDSET as ::core::ffi::c_int;
    ev_io_start(main_loop, xcb_watcher);
    let ref mut fresh1 = (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).pending;
    *fresh1 = 0 as ::core::ffi::c_int;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).active = *fresh1;
    (*(xcb_prepare as *mut ::core::ffi::c_void as *mut ev_watcher)).priority =
        0 as ::core::ffi::c_int;
    (*xcb_prepare).cb = Some(
        restore_xcb_prepare_cb
            as unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>;
    memmove(
        &raw mut (*(xcb_prepare as *mut ev_watcher)).cb as *mut ::core::ffi::c_void,
        &raw mut (*xcb_prepare).cb as *const ::core::ffi::c_void,
        ::core::mem::size_of::<
            Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
        >() as size_t,
    );
    ev_prepare_start(main_loop, xcb_prepare);
}
unsafe fn update_placeholder_contents(state: *mut placeholder_state) {
    let foreground: color_t = config.client.placeholder.text;
    let background: color_t = config.client.placeholder.background;
    draw_util_clear_surface(&raw mut (*state).surface, background);
    xcb_aux_sync(restore_conn);
    let mut swallows: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    swallows = (*(*state).con).swallow_head.tqh_first as *mut Match;
    while !swallows.is_null() {
        let mut serialized: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !(*swallows).class.is_null() {
            let old_serialized: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                c"%s%sclass=\"%s\"".as_ptr(),
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    c"[".as_ptr()
                },
                if !serialized.is_null() {
                    c" ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*swallows).class).pattern,
            );
            free(old_serialized as *mut ::core::ffi::c_void);
        }
        if !(*swallows).instance.is_null() {
            let old_serialized_0: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                c"%s%sinstance=\"%s\"".as_ptr(),
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    c"[".as_ptr()
                },
                if !serialized.is_null() {
                    c" ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*swallows).instance).pattern,
            );
            free(old_serialized_0 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).window_role.is_null() {
            let old_serialized_1: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                c"%s%swindow_role=\"%s\"".as_ptr(),
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    c"[".as_ptr()
                },
                if !serialized.is_null() {
                    c" ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*swallows).window_role).pattern,
            );
            free(old_serialized_1 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).title.is_null() {
            let old_serialized_2: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                c"%s%stitle=\"%s\"".as_ptr(),
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    c"[".as_ptr()
                },
                if !serialized.is_null() {
                    c" ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*swallows).title).pattern,
            );
            free(old_serialized_2 as *mut ::core::ffi::c_void);
        }
        if !(*swallows).machine.is_null() {
            let old_serialized_3: *mut ::core::ffi::c_char = serialized;
            sasprintf(
                &raw mut serialized,
                c"%s%smachine=\"%s\"".as_ptr(),
                if !serialized.is_null() {
                    serialized as *const ::core::ffi::c_char
                } else {
                    c"[".as_ptr()
                },
                if !serialized.is_null() {
                    c" ".as_ptr()
                } else {
                    c"".as_ptr()
                },
                (*(*swallows).machine).pattern,
            );
            free(old_serialized_3 as *mut ::core::ffi::c_void);
        }
        if serialized.is_null() {
            debuglog(
                c"%s:%s:%d - This swallows specification is not serializable?!\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"update_placeholder_contents".as_ptr(),
                159 as ::core::ffi::c_int,
            );
        } else {
            let old_serialized_4: *mut ::core::ffi::c_char = serialized;
            sasprintf(&raw mut serialized, c"%s]".as_ptr(), serialized);
            free(old_serialized_4 as *mut ::core::ffi::c_void);
            debuglog(
                c"%s:%s:%d - con %p (placeholder 0x%08x) line %d: %s\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"update_placeholder_contents".as_ptr(),
                166 as ::core::ffi::c_int,
                (*state).con,
                (*state).window,
                n,
                serialized,
            );
            let str: *mut i3String = i3string_from_utf8(serialized);
            draw_util_text(
                str,
                &raw mut (*state).surface,
                foreground,
                background,
                logical_px(2 as ::core::ffi::c_int),
                n * (config.font.height + logical_px(2 as ::core::ffi::c_int))
                    + logical_px(2 as ::core::ffi::c_int),
                (*state).rect.width.wrapping_sub(
                    (2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int)) as uint32_t,
                ) as ::core::ffi::c_int,
            );
            i3string_free(str);
            n += 1;
            free(serialized as *mut ::core::ffi::c_void);
        }
        swallows = (*swallows).matches.tqe_next as *mut Match;
    }
    let line: *mut i3String = i3string_from_utf8(c"⌚".as_ptr());
    let text_width: ::core::ffi::c_int = predict_text_width(line);
    let x: ::core::ffi::c_int = (*state)
        .rect
        .width
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((text_width / 2 as ::core::ffi::c_int) as uint32_t)
        as ::core::ffi::c_int;
    let y: ::core::ffi::c_int = (*state)
        .rect
        .height
        .wrapping_div(2 as uint32_t)
        .wrapping_sub((config.font.height / 2 as ::core::ffi::c_int) as uint32_t)
        as ::core::ffi::c_int;
    draw_util_text(
        line,
        &raw mut (*state).surface,
        foreground,
        background,
        x,
        y,
        text_width,
    );
    i3string_free(line);
    xcb_aux_sync(restore_conn);
}
unsafe fn open_placeholder_window(con: *mut Con) {
    if con_is_leaf(con) as ::core::ffi::c_int != 0
        && ((*con).window.is_null() || (*(*con).window).id as ::core::ffi::c_long == XCB_NONE)
        && !(*con).swallow_head.tqh_first.is_null()
        && (*con).type_0 as ::core::ffi::c_uint
            == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut fresh2: [uint32_t; 2] = [
            config.client.placeholder.background.colorpixel,
            (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int) as uint32_t,
        ];
        let placeholder: xcb_window_t = create_window(
            restore_conn,
            (*con).rect as Rect,
            XCB_COPY_FROM_PARENT as uint16_t,
            XCB_COPY_FROM_PARENT as xcb_visualid_t,
            XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as uint16_t,
            XCURSOR_CURSOR_POINTER,
            true_0 != 0,
            (XCB_CW_BACK_PIXEL as ::core::ffi::c_int | XCB_CW_EVENT_MASK as ::core::ffi::c_int)
                as uint32_t,
            &raw mut fresh2 as *mut uint32_t,
        );
        let mut hints: xcb_icccm_wm_hints_t = xcb_icccm_wm_hints_t {
            flags: 0,
            input: 0,
            initial_state: 0,
            icon_pixmap: 0,
            icon_window: 0,
            icon_x: 0,
            icon_y: 0,
            icon_mask: 0,
            window_group: 0,
        };
        xcb_icccm_wm_hints_set_none(&raw mut hints);
        xcb_icccm_wm_hints_set_input(&raw mut hints, 0 as uint8_t);
        xcb_icccm_set_wm_hints(restore_conn, placeholder, &raw mut hints);
        if !(*con).name.is_null() {
            xcb_change_property(
                restore_conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
                placeholder,
                A__NET_WM_NAME,
                A_UTF8_STRING,
                8 as uint8_t,
                strlen((*con).name) as uint32_t,
                (*con).name as *const ::core::ffi::c_void,
            );
        }
        debuglog(
            c"%s:%s:%d - Created placeholder window 0x%08x for leaf container %p / %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/restore_layout.c".as_ptr(),
            c"open_placeholder_window".as_ptr(),
            219 as ::core::ffi::c_int,
            placeholder,
            con,
            (*con).name,
        );
        let state: *mut placeholder_state = scalloc(
            1 as size_t,
            ::core::mem::size_of::<placeholder_state>() as size_t,
        ) as *mut placeholder_state;
        (*state).window = placeholder;
        (*state).con = con;
        (*state).rect = (*con).rect as Rect;
        draw_util_surface_init(
            restore_conn,
            &raw mut (*state).surface,
            placeholder as xcb_drawable_t,
            get_visualtype(root_screen),
            (*state).rect.width as ::core::ffi::c_int,
            (*state).rect.height as ::core::ffi::c_int,
        );
        update_placeholder_contents(state);
        (*state).state.tqe_next = ::core::ptr::null_mut::<placeholder_state>();
        (*state).state.tqe_prev = state_head.tqh_last;
        *state_head.tqh_last = state as *mut placeholder_state;
        state_head.tqh_last = &raw mut (*state).state.tqe_next;
        let temp_id: *mut Match = smalloc(::core::mem::size_of::<Match>() as size_t) as *mut Match;
        match_init(temp_id);
        (*temp_id).dock = M_DONTCHECK;
        (*temp_id).id = placeholder;
        (*temp_id).matches.tqe_next = (*con).swallow_head.tqh_first;
        if !(*temp_id).matches.tqe_next.is_null() {
            (*(*con).swallow_head.tqh_first).matches.tqe_prev =
                &raw mut (*temp_id).matches.tqe_next;
        } else {
            (*con).swallow_head.tqh_last = &raw mut (*temp_id).matches.tqe_next;
        }
        (*con).swallow_head.tqh_first = temp_id as *mut Match;
        (*temp_id).matches.tqe_prev = &raw mut (*con).swallow_head.tqh_first;
    }
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
}

pub unsafe fn restore_open_placeholder_windows(parent: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*parent).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*parent).floating_head.tqh_first as *mut Con;
    while !child.is_null() {
        open_placeholder_window(child);
        child = (*child).floating_windows.tqe_next as *mut Con;
    }
    xcb_flush(restore_conn);
}

pub unsafe fn restore_kill_placeholder(placeholder: xcb_window_t) -> bool {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != placeholder {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            xcb_destroy_window(restore_conn, (*state).window);
            draw_util_surface_free(restore_conn, &raw mut (*state).surface);
            if !(*state).state.tqe_next.is_null() {
                (*(*state).state.tqe_next).state.tqe_prev = (*state).state.tqe_prev;
            } else {
                state_head.tqh_last = (*state).state.tqe_prev;
            }
            *(*state).state.tqe_prev = (*state).state.tqe_next;
            free(state as *mut ::core::ffi::c_void);
            debuglog(
                c"%s:%s:%d - placeholder window 0x%08x destroyed.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_kill_placeholder".as_ptr(),
                284 as ::core::ffi::c_int,
                placeholder,
            );
            return true_0 != 0;
        }
    }
    debuglog(
        c"%s:%s:%d - 0x%08x is not a placeholder window, ignoring.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/restore_layout.c".as_ptr(),
        c"restore_kill_placeholder".as_ptr(),
        288 as ::core::ffi::c_int,
        placeholder,
    );
    false_0 != 0
}
unsafe fn expose_event(event: *mut xcb_expose_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                c"%s:%s:%d - refreshing window 0x%08x contents (con %p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"expose_event".as_ptr(),
                299 as ::core::ffi::c_int,
                (*state).window,
                (*state).con,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        c"ERROR: Received ExposeEvent for unknown window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        (*event).window,
    );
}
unsafe fn configure_notify(event: *mut xcb_configure_notify_event_t) {
    let mut state: *mut placeholder_state = ::core::ptr::null_mut::<placeholder_state>();
    state = state_head.tqh_first as *mut placeholder_state;
    while !state.is_null() {
        if (*state).window != (*event).window {
            state = (*state).state.tqe_next as *mut placeholder_state;
        } else {
            debuglog(
                c"%s:%s:%d - ConfigureNotify: window 0x%08x has now width=%d, height=%d (con %p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"configure_notify".as_ptr(),
                323 as ::core::ffi::c_int,
                (*state).window,
                (*event).width as ::core::ffi::c_int,
                (*event).height as ::core::ffi::c_int,
                (*state).con,
            );
            (*state).rect.width = (*event).width as uint32_t;
            (*state).rect.height = (*event).height as uint32_t;
            draw_util_surface_set_size(
                &raw mut (*state).surface,
                (*state).rect.width as ::core::ffi::c_int,
                (*state).rect.height as ::core::ffi::c_int,
            );
            update_placeholder_contents(state);
            return;
        }
    }
    errorlog(
        c"ERROR: Received ConfigureNotify for unknown window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        (*event).window,
    );
}
unsafe fn restore_handle_event(type_0: ::core::ffi::c_int, event: *mut xcb_generic_event_t) {
    match type_0 {
        XCB_EXPOSE => {
            if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                expose_event(event as *mut xcb_expose_event_t);
            }
        }
        XCB_CONFIGURE_NOTIFY => {
            configure_notify(event as *mut xcb_configure_notify_event_t);
        }
        _ => {
            debuglog(
                c"%s:%s:%d - Received unhandled X11 event of type %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/restore_layout.c".as_ptr(),
                c"restore_handle_event".as_ptr(),
                350 as ::core::ffi::c_int,
                type_0,
            );
        }
    };
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

use xcb_sys::{
    xcb_aux_sync, xcb_change_property, xcb_configure_notify_event_t, xcb_connect,
    xcb_connection_has_error, xcb_connection_t, xcb_destroy_window, xcb_disconnect, xcb_drawable_t,
    xcb_expose_event_t, xcb_flush, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_file_descriptor, xcb_icccm_set_wm_hints, xcb_icccm_wm_hints_set_input,
    xcb_icccm_wm_hints_set_none, xcb_icccm_wm_hints_t, xcb_poll_for_event, xcb_visualid_t,
    xcb_window_t, XCB_CONFIGURE_NOTIFY, XCB_COPY_FROM_PARENT, XCB_CW_BACK_PIXEL, XCB_CW_EVENT_MASK,
    XCB_EVENT_MASK_EXPOSURE, XCB_EVENT_MASK_STRUCTURE_NOTIFY, XCB_EXPOSE, XCB_NONE,
    XCB_PROP_MODE_REPLACE, XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::{free, memmove, strlen};

use i3_common::native_stdio::errx;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use crate::application::root_screen;
use crate::application::A_UTF8_STRING;
use crate::application::A__NET_WM_NAME;
use crate::con::con_is_leaf;
use crate::config::config;
use crate::r#match::match_init;
use crate::xcb::create_window;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_surface_set_size;
use i3_common::font::predict_text_width;
use i3_common::get_visualtype::get_visualtype;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
