use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::FocusActivation;
use crate::types::KillWindow;
use crate::types::Layout;
use crate::types::WindowDock;
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
pub type i3Window = Window;

pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;

pub use crate::application::ignore_event_sequence_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ignore_Event {
    pub sequence: ignore_event_sequence_t,
    pub response_type: ::core::ffi::c_int,
    pub added: libc::time_t,
    pub ignore_events: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub sle_next: *mut Ignore_Event,
}

pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignore_head {
    pub slh_first: *mut Ignore_Event,
}
pub type cb_property_handler_t = Option<unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct property_handler_t {
    pub atom: xcb_atom_t,
    pub long_len: u32,
    pub cb: cb_property_handler_t,
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
static mut ignore_events: ignore_head = ignore_head {
    slh_first: ::core::ptr::null::<Ignore_Event>() as *mut Ignore_Event,
};

pub unsafe fn add_ignore_event(
    sequence: ignore_event_sequence_t,
    response_type: ::core::ffi::c_int,
) {
    let event: *mut Ignore_Event =
        smalloc(::core::mem::size_of::<Ignore_Event>() as usize) as *mut Ignore_Event;
    (*event).sequence = sequence;
    (*event).response_type = response_type;
    (*event).added = time(::core::ptr::null_mut::<libc::time_t>());
    (*event).ignore_events.sle_next = ignore_events.slh_first;
    ignore_events.slh_first = event;
}

pub unsafe fn event_is_ignored(
    sequence: ignore_event_sequence_t,
    response_type: ::core::ffi::c_int,
) -> bool {
    let mut event: *mut Ignore_Event = ::core::ptr::null_mut::<Ignore_Event>();
    let now: libc::time_t = time(::core::ptr::null_mut::<libc::time_t>());
    event = ignore_events.slh_first;
    while !event.is_null() {
        if now - (*event).added > 5 as libc::time_t {
            let save: *mut Ignore_Event = event;
            event = (*event).ignore_events.sle_next;
            if ignore_events.slh_first == save {
                ignore_events.slh_first = (*ignore_events.slh_first).ignore_events.sle_next;
            } else {
                let mut curelm: *mut Ignore_Event = ignore_events.slh_first;
                while (*curelm).ignore_events.sle_next != save {
                    curelm = (*curelm).ignore_events.sle_next;
                }
                (*curelm).ignore_events.sle_next =
                    (*(*curelm).ignore_events.sle_next).ignore_events.sle_next;
            }
            free(save as *mut ::core::ffi::c_void);
        } else {
            event = (*event).ignore_events.sle_next;
        }
    }
    event = ignore_events.slh_first;
    while !event.is_null() {
        if !((*event).sequence.value != sequence.value) {
            if !((*event).response_type != -(1 as ::core::ffi::c_int)
                && (*event).response_type != response_type)
            {
                return true_0 != 0;
            }
        }
        event = (*event).ignore_events.sle_next;
    }
    false_0 != 0
}
unsafe fn check_crossing_screen_boundary(x: u32, y: u32) {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    if config.disable_focus_follows_mouse {
        return;
    }
    output = get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
    if output.is_null() {
        errorlog(c"ERROR: ERROR: No such screen\n".as_ptr().cast_mut());
        return;
    }
    if (*output).con.is_null() {
        errorlog(
            c"ERROR: ERROR: The screen is not recognized by i3 (no container associated)\n"
                .as_ptr()
                .cast_mut(),
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
unsafe fn handle_enter_notify(event: *mut xcb_enter_notify_event_t, full_sequence: u32) {
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    last_timestamp = (*event).time;
    debuglog(
        c"%s:%s:%d - enter_notify for %08x, mode = %d, detail %d, serial %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_enter_notify".as_ptr(),
        133 as ::core::ffi::c_int,
        (*event).event,
        (*event).mode as ::core::ffi::c_int,
        (*event).detail as ::core::ffi::c_int,
        (*event).sequence as ::core::ffi::c_int,
    );
    debuglog(
        c"%s:%s:%d - coordinates %d, %d\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_enter_notify".as_ptr(),
        134 as ::core::ffi::c_int,
        (*event).event_x as ::core::ffi::c_int,
        (*event).event_y as ::core::ffi::c_int,
    );
    if (*event).mode as ::core::ffi::c_int != XCB_NOTIFY_MODE_NORMAL as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - This was not a normal notify, ignoring\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_enter_notify".as_ptr(),
            136 as ::core::ffi::c_int,
        );
        return;
    }
    if event_is_ignored(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        XCB_ENTER_NOTIFY,
    ) {
        debuglog(
            c"%s:%s:%d - Event ignored\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_enter_notify".as_ptr(),
            142 as ::core::ffi::c_int,
        );
        return;
    }
    let mut enter_child: bool = false_0 != 0;
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        con = con_by_window_id((*event).event);
        enter_child = true_0 != 0;
    }
    if con.is_null()
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - Getting screen at %d x %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_enter_notify".as_ptr(),
            157 as ::core::ffi::c_int,
            (*event).root_x as ::core::ffi::c_int,
            (*event).root_y as ::core::ffi::c_int,
        );
        check_crossing_screen_boundary((*event).root_x as u32, (*event).root_y as u32);
        return;
    }
    let layout: Layout = if enter_child as ::core::ffi::c_int != 0 {
        (*(*con).parent).layout
    } else {
        (*con).layout
    };
    if layout == Layout::Default {
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !child.is_null() {
            if rect_contains(
                (*child).deco_rect as Rect,
                (*event).event_x as u32,
                (*event).event_y as u32,
            ) {
                verboselog(
                    c"using child %p / %s instead!\n".as_ptr().cast_mut(),
                    child,
                    (*child).name,
                );
                con = child;
                break;
            } else {
                child = *(*((*child).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
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
    focused_id = XCB_NONE as xcb_window_t;
    con_focus(con_descend_focused(con));
    tree_render();
}
unsafe fn handle_motion_notify(event: *mut xcb_motion_notify_event_t) {
    last_timestamp = (*event).time;
    if (*event).child as ::core::ffi::c_long != XCB_NONE {
        return;
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_frame_id((*event).event);
    if con.is_null() {
        debuglog(
            c"%s:%s:%d - MotionNotify for an unknown container, checking if it crosses screen boundaries.\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_motion_notify".as_ptr(),
            214 as ::core::ffi::c_int,
        );
        check_crossing_screen_boundary((*event).root_x as u32, (*event).root_y as u32);
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
            (*event).event_x as u32,
            (*event).event_y as u32,
        ) {
            if (*(*con).parent).focus_head.tqh_first == con {
                return;
            }
            con_focus(con);
            x_push_changes(croot);
        }
    } else {
        let mut current: *mut Con = ::core::ptr::null_mut::<Con>();
        current = *(*((*con).nodes_head.tqh_last as *mut nodes_head)).tqh_last as *mut Con;
        while !current.is_null() {
            if !rect_contains(
                (*current).deco_rect as Rect,
                (*event).event_x as u32,
                (*event).event_y as u32,
            ) {
                current = *(*((*current).nodes.tqe_prev as *mut nodes_head)).tqh_last as *mut Con;
            } else {
                if (*con).focus_head.tqh_first == current {
                    return;
                }
                con_focus(current);
                x_push_changes(croot);
                return;
            }
        }
    }
}
unsafe fn handle_mapping_notify(event: *mut xcb_mapping_notify_event_t) {
    if (*event).request as ::core::ffi::c_int != XCB_MAPPING_KEYBOARD as ::core::ffi::c_int
        && (*event).request as ::core::ffi::c_int != XCB_MAPPING_MODIFIER as ::core::ffi::c_int
    {
        return;
    }
    debuglog(
        c"%s:%s:%d - Received mapping_notify for keyboard or modifier mapping, re-grabbing keys\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_mapping_notify".as_ptr(),
        269 as ::core::ffi::c_int,
    );
    xcb_refresh_keyboard_mapping(keysyms, event);
    xcb_numlock_mask = aio_get_mod_mask_for(XCB_NUM_LOCK as u32, keysyms) as ::core::ffi::c_uint;
    ungrab_all_keys(conn);
    translate_keysyms();
    grab_all_keys(conn);
}
unsafe fn handle_map_request(event: *const xcb_map_request_event_t, full_sequence: u32) {
    let cookie: xcb_get_window_attributes_cookie_t =
        xcb_get_window_attributes_unchecked(conn, (*event).window);
    debuglog(
        c"%s:%s:%d - window = 0x%08x, serial is %d.\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_map_request".as_ptr(),
        286 as ::core::ffi::c_int,
        (*event).window,
        (*event).sequence as ::core::ffi::c_int,
    );
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        -(1 as ::core::ffi::c_int),
    );
    manage_window((*event).window, cookie, false_0 != 0);
}
unsafe fn handle_configure_request(event: *mut xcb_configure_request_event_t) {
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        c"%s:%s:%d - window 0x%08x wants to be at %dx%d with %dx%d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_configure_request".as_ptr(),
        304 as ::core::ffi::c_int,
        (*event).window,
        (*event).x as ::core::ffi::c_int,
        (*event).y as ::core::ffi::c_int,
        (*event).width as ::core::ffi::c_int,
        (*event).height as ::core::ffi::c_int,
    );
    con = con_by_window_id((*event).window);
    if con.is_null() {
        debuglog(
            c"%s:%s:%d - Configure request for unmanaged window, can do that.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_configure_request".as_ptr(),
            309 as ::core::ffi::c_int,
        );
        let mut mask: u32 = 0 as u32;
        let mut values: [u32; 7] = [0; 7];
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as u32;
            let fresh0 = c;
            c = c + 1;
            values[fresh0 as usize] = (*event).x as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int as u32;
            let fresh1 = c;
            c = c + 1;
            values[fresh1 as usize] = (*event).y as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int as u32;
            let fresh2 = c;
            c = c + 1;
            values[fresh2 as usize] = (*event).width as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int as u32;
            let fresh3 = c;
            c = c + 1;
            values[fresh3 as usize] = (*event).height as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_BORDER_WIDTH as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_BORDER_WIDTH as ::core::ffi::c_int as u32;
            let fresh4 = c;
            c = c + 1;
            values[fresh4 as usize] = (*event).border_width as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_SIBLING as ::core::ffi::c_int as u32;
            let fresh5 = c;
            c = c + 1;
            values[fresh5 as usize] = (*event).sibling as u32;
        }
        if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int
            != 0
        {
            mask |= XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as u32;
            let fresh6 = c;
            c = c + 1;
            values[fresh6 as usize] = (*event).stack_mode as u32;
        }
        xcb_configure_window(
            conn,
            (*event).window,
            mask as u16,
            &raw mut values as *mut u32 as *const ::core::ffi::c_void,
        );
        xcb_flush(conn);
        return;
    }
    debuglog(
        c"%s:%s:%d - Configure request!\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_configure_request".as_ptr(),
        336 as ::core::ffi::c_int,
    );
    let workspace: *mut Con = con_get_workspace(con);
    if !workspace.is_null()
        && strcmp((*workspace).name, c"__i3_scratch".as_ptr()) == 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - This is a scratchpad container, ignoring ConfigureRequest\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_configure_request".as_ptr(),
            340 as ::core::ffi::c_int,
        );
    } else {
        fullscreen = con_get_fullscreen_covering_ws(workspace);
        if fullscreen != con
            && con_is_floating(con) as ::core::ffi::c_int != 0
            && con_is_leaf(con) as ::core::ffi::c_int != 0
        {
            let bsr: Rect = con_border_style_rect(con);
            let floatingcon: *mut Con = (*con).parent as *mut Con;
            let mut newrect: Rect = (*floatingcon).rect as Rect;
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                != 0
            {
                newrect.x = ((*event).x as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.x));
                debuglog(
                    c"%s:%s:%d - proposed x = %d, new x is %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    354 as ::core::ffi::c_int,
                    (*event).x as ::core::ffi::c_int,
                    newrect.x,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                != 0
            {
                newrect.y = ((*event).y as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.y));
                debuglog(
                    c"%s:%s:%d - proposed y = %d, new y is %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    358 as ::core::ffi::c_int,
                    (*event).y as ::core::ffi::c_int,
                    newrect.y,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                != 0
            {
                newrect.width = ((*event).width as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.width));
                newrect.width = newrect
                    .width
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
                debuglog(
                    c"%s:%s:%d - proposed width = %d, new width is %d (x11 border %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    364 as ::core::ffi::c_int,
                    (*event).width as ::core::ffi::c_int,
                    newrect.width,
                    (*con).border_width,
                );
            }
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                != 0
            {
                newrect.height = ((*event).height as u32)
                    .wrapping_add((-(1 as ::core::ffi::c_int) as u32).wrapping_mul(bsr.height));
                newrect.height = newrect
                    .height
                    .wrapping_add(((*con).border_width * 2 as ::core::ffi::c_int) as u32);
                debuglog(
                    c"%s:%s:%d - proposed height = %d, new height is %d (x11 border %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    370 as ::core::ffi::c_int,
                    (*event).height as ::core::ffi::c_int,
                    newrect.height,
                    (*con).border_width,
                );
            }
            debuglog(
                c"%s:%s:%d - Container is a floating leaf node, will do that.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_configure_request".as_ptr(),
                373 as ::core::ffi::c_int,
            );
            floating_reposition(floatingcon, newrect);
            return;
        }
        if !(*con).parent.is_null()
            && (*(*con).parent).type_0 as ::core::ffi::c_uint
                == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            debuglog(
                c"%s:%s:%d - Reconfiguring dock window (con = %p).\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_configure_request".as_ptr(),
                380 as ::core::ffi::c_int,
                con,
            );
            if (*event).value_mask as ::core::ffi::c_int
                & XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                != 0
            {
                debuglog(
                    c"%s:%s:%d - Dock client wants to change height to %d, we can do that.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    382 as ::core::ffi::c_int,
                    (*event).height as ::core::ffi::c_int,
                );
                (*con).geometry.height = (*event).height as u32;
                tree_render();
            }
            if (*event).value_mask as ::core::ffi::c_int & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                != 0
                || (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                    != 0
            {
                let x: i16 = (if (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                    != 0
                {
                    (*event).x as ::core::ffi::c_int
                } else {
                    (*con).geometry.x as i16 as ::core::ffi::c_int
                }) as i16;
                let y: i16 = (if (*event).value_mask as ::core::ffi::c_int
                    & XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                    != 0
                {
                    (*event).y as ::core::ffi::c_int
                } else {
                    (*con).geometry.y as i16 as ::core::ffi::c_int
                }) as i16;
                let current_output: *mut Con = con_get_output(con);
                let target: *mut Output =
                    get_output_containing(x as ::core::ffi::c_uint, y as ::core::ffi::c_uint);
                if !target.is_null() && current_output != (*target).con {
                    debuglog(
                        c"%s:%s:%d - Dock client is requested to be moved to output %s, moving it there.\n".as_ptr().cast_mut(),
                        c"../src/handlers.c".as_ptr(),
                        c"handle_configure_request".as_ptr(),
                        395 as ::core::ffi::c_int,
                        output_primary_name(target),
                    );
                    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
                    let nc: *mut Con = con_for_window(
                        (*target).con,
                        (*con).window as *mut i3Window,
                        &raw mut match_0,
                    );
                    debuglog(
                        c"%s:%s:%d - Dock client will be moved to container %p.\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/handlers.c".as_ptr(),
                        c"handle_configure_request".as_ptr(),
                        398 as ::core::ffi::c_int,
                        nc,
                    );
                    con_detach(con);
                    con_attach(con, nc, false_0 != 0);
                    tree_render();
                } else {
                    debuglog(
                        c"%s:%s:%d - Dock client will not be moved, we only support moving it to another output.\n".as_ptr().cast_mut(),
                        c"../src/handlers.c".as_ptr(),
                        c"handle_configure_request".as_ptr(),
                        404 as ::core::ffi::c_int,
                    );
                }
            }
        } else if (*event).value_mask as ::core::ffi::c_int
            & XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int
            != 0
        {
            debuglog(
                c"%s:%s:%d - window 0x%08x wants to be stacked %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_configure_request".as_ptr(),
                411 as ::core::ffi::c_int,
                (*event).window,
                (*event).stack_mode as ::core::ffi::c_int,
            );
            if (*event).stack_mode as ::core::ffi::c_int
                != XCB_STACK_MODE_ABOVE as ::core::ffi::c_int
            {
                debuglog(
                    c"%s:%s:%d - stack_mode != XCB_STACK_MODE_ABOVE, ignoring ConfigureRequest\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    416 as ::core::ffi::c_int,
                );
            } else if !fullscreen.is_null() || !con_is_leaf(con) {
                debuglog(
                    c"%s:%s:%d - fullscreen or not a leaf, ignoring ConfigureRequest\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    421 as ::core::ffi::c_int,
                );
            } else if workspace.is_null() {
                debuglog(
                    c"%s:%s:%d - Window is not being managed, ignoring ConfigureRequest\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    426 as ::core::ffi::c_int,
                );
            } else if config.focus_on_window_activation == FocusActivation::Focus
                || config.focus_on_window_activation == FocusActivation::Smart
                    && workspace_is_visible(workspace) as ::core::ffi::c_int != 0
            {
                debuglog(
                    c"%s:%s:%d - Focusing con = %p\n".as_ptr().cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    431 as ::core::ffi::c_int,
                    con,
                );
                workspace_show(workspace);
                con_activate_unblock(con);
                tree_render();
            } else if config.focus_on_window_activation == FocusActivation::Urgent
                || config.focus_on_window_activation == FocusActivation::Smart
                    && !workspace_is_visible(workspace)
            {
                debuglog(
                    c"%s:%s:%d - Marking con = %p urgent\n".as_ptr().cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    436 as ::core::ffi::c_int,
                    con,
                );
                con_set_urgency(con, true_0 != 0);
                con = remanage_window(con);
                tree_render();
            } else {
                debuglog(
                    c"%s:%s:%d - Ignoring request for con = %p.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_configure_request".as_ptr(),
                    441 as ::core::ffi::c_int,
                    con,
                );
            }
        }
    }
    fake_absolute_configure_notify(con);
}
unsafe fn handle_screen_change(e: *mut xcb_generic_event_t) {
    debuglog(
        c"%s:%s:%d - RandR screen change\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_screen_change".as_ptr(),
        455 as ::core::ffi::c_int,
    );
    let cookie: xcb_get_geometry_cookie_t = xcb_get_geometry(conn, root as xcb_drawable_t);
    let reply: *mut xcb_get_geometry_reply_t = xcb_get_geometry_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errorlog(
            c"ERROR: Could not get geometry of the root window, exiting\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_FAILURE);
    }
    debuglog(
        c"%s:%s:%d - root geometry reply: (%d, %d) %d x %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_screen_change".as_ptr(),
        465 as ::core::ffi::c_int,
        (*reply).x as ::core::ffi::c_int,
        (*reply).y as ::core::ffi::c_int,
        (*reply).width as ::core::ffi::c_int,
        (*reply).height as ::core::ffi::c_int,
    );
    (*croot).rect.width = (*reply).width as u32;
    (*croot).rect.height = (*reply).height as u32;
    randr_query_outputs();
    scratchpad_fix_resolution();
    ipc_send_event(
        c"output".as_ptr(),
        I3_IPC_EVENT_OUTPUT as u32,
        c"{\"change\":\"unspecified\"}".as_ptr(),
    );
}
unsafe fn handle_unmap_notify_event(event: *mut xcb_unmap_notify_event_t, full_sequence: u32) {
    debuglog(
        c"%s:%s:%d - UnmapNotify for 0x%08x (received from 0x%08x), serial %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_unmap_notify_event".as_ptr(),
        483 as ::core::ffi::c_int,
        (*event).window,
        (*event).event,
        (*event).sequence as ::core::ffi::c_int,
    );
    let mut cookie: xcb_get_input_focus_cookie_t = xcb_get_input_focus_cookie_t { sequence: 0 };
    let mut con: *mut Con = con_by_window_id((*event).window);
    if con.is_null() {
        con = con_by_frame_id((*event).window);
        if con.is_null() {
            verboselog(
                c"Not a managed window, ignoring UnmapNotify event\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return;
        }
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        }
        cookie = xcb_get_input_focus(conn);
        debuglog(
            c"%s:%s:%d - ignore_unmap = %d for frame of container %p\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_unmap_notify_event".as_ptr(),
            500 as ::core::ffi::c_int,
            (*con).ignore_unmap as ::core::ffi::c_int,
            con,
        );
    } else {
        cookie = xcb_get_input_focus(conn);
        if (*con).ignore_unmap as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            debuglog(
                c"%s:%s:%d - ignore_unmap = %d, dec\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_unmap_notify_event".as_ptr(),
                508 as ::core::ffi::c_int,
                (*con).ignore_unmap as ::core::ffi::c_int,
            );
            (*con).ignore_unmap = (*con).ignore_unmap.wrapping_sub(1);
        } else {
            xcb_delete_property(conn, (*event).window, A__NET_WM_DESKTOP);
            xcb_delete_property(conn, (*event).window, A__NET_WM_STATE);
            tree_close_internal(con, KillWindow::DontKill, false_0 != 0);
            tree_render();
        }
    }
    add_ignore_event(
        ignore_event_sequence_t {
            value: full_sequence,
        },
        XCB_ENTER_NOTIFY,
    );
    free(xcb_get_input_focus_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ) as *mut ::core::ffi::c_void);
}
unsafe fn handle_destroy_notify_event(event: *mut xcb_destroy_notify_event_t, full_sequence: u32) {
    debuglog(
        c"%s:%s:%d - destroy notify for 0x%08x, 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_destroy_notify_event".as_ptr(),
        555 as ::core::ffi::c_int,
        (*event).event,
        (*event).window,
    );
    let mut unmap: xcb_unmap_notify_event_t = xcb_unmap_notify_event_t {
        response_type: 0,
        pad0: 0,
        sequence: 0,
        event: 0,
        window: 0,
        from_configure: 0,
        pad1: [0; 3],
    };
    unmap.sequence = (*event).sequence;
    unmap.event = (*event).event;
    unmap.window = (*event).window;
    handle_unmap_notify_event(&raw mut unmap, full_sequence);
}
unsafe fn window_name_changed(window: *mut i3Window, old_name: *mut ::core::ffi::c_char) -> bool {
    if old_name.is_null() && (*window).name.is_null() {
        return false_0 != 0;
    }
    if (old_name == NULL as *mut ::core::ffi::c_char) as ::core::ffi::c_int
        ^ ((*window).name == NULL as *mut i3String) as ::core::ffi::c_int
        != 0
    {
        return true_0 != 0;
    }
    strcmp(old_name, i3string_as_utf8((*window).name)) != 0 as ::core::ffi::c_int
}
unsafe fn handle_windowname_change(mut con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    let mut old_name: *mut ::core::ffi::c_char = if !(*(*con).window).name.is_null() {
        sstrdup(i3string_as_utf8((*(*con).window).name))
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    window_update_name((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if window_name_changed((*con).window as *mut i3Window, old_name) {
        ipc_send_window_event(c"title".as_ptr(), con);
    }
    free(old_name as *mut ::core::ffi::c_void);
    old_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    true_0 != 0
}
unsafe fn handle_windowname_change_legacy(
    mut con: *mut Con,
    prop: *mut xcb_get_property_reply_t,
) -> bool {
    let mut old_name: *mut ::core::ffi::c_char = if !(*(*con).window).name.is_null() {
        sstrdup(i3string_as_utf8((*(*con).window).name))
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    window_update_name_legacy((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    x_push_changes(croot);
    if window_name_changed((*con).window as *mut i3Window, old_name) {
        ipc_send_window_event(c"title".as_ptr(), con);
    }
    free(old_name as *mut ::core::ffi::c_void);
    old_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    true_0 != 0
}
unsafe fn handle_windowrole_change(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_role((*con).window as *mut i3Window, prop);
    remanage_window(con);
    true_0 != 0
}
unsafe fn handle_expose_event(event: *mut xcb_expose_event_t) {
    let mut parent: *mut Con = ::core::ptr::null_mut::<Con>();
    debuglog(
        c"%s:%s:%d - window = %08x\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_expose_event".as_ptr(),
        642 as ::core::ffi::c_int,
        (*event).window,
    );
    parent = con_by_frame_id((*event).window);
    if parent.is_null() {
        verboselog(
            c"expose event for unknown window, ignoring\n"
                .as_ptr()
                .cast_mut(),
        );
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
    xcb_flush(conn);
}
pub const _NET_WM_MOVERESIZE_SIZE_TOPLEFT: u32 = 0 as u32;
pub const _NET_WM_MOVERESIZE_MOVE: u32 = 8 as u32;
pub const _NET_MOVERESIZE_WINDOW_X: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_Y: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_WIDTH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const _NET_MOVERESIZE_WINDOW_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
unsafe fn handle_net_wm_state_change(mut con: *mut Con, change: u32, atom: u32) {
    if atom == 0 as u32 {
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
        debuglog(
            c"%s:%s:%d - Received a client message to %s _NET_WM_STATE_FULLSCREEN.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_net_wm_state_change".as_ptr(),
            682 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_TOGGLE as u32
            || (*con).fullscreen_mode as ::core::ffi::c_uint
                != CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                && change == _NET_WM_STATE_REMOVE as u32
            || (*con).fullscreen_mode as ::core::ffi::c_uint
                == CF_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                && change == _NET_WM_STATE_ADD as u32
        {
            debuglog(
                c"%s:%s:%d - toggling fullscreen\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_net_wm_state_change".as_ptr(),
                688 as ::core::ffi::c_int,
            );
            con_toggle_fullscreen(con, CF_OUTPUT as ::core::ffi::c_int);
        }
    } else if atom == A__NET_WM_STATE_DEMANDS_ATTENTION {
        debuglog(
            c"%s:%s:%d - Received a client message to %s _NET_WM_STATE_DEMANDS_ATTENTION.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_net_wm_state_change".as_ptr(),
            692 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_ADD as u32 {
            con_set_urgency(con, true_0 != 0);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_REMOVE as u32 {
            con_set_urgency(con, false_0 != 0);
            con = remanage_window(con);
        } else if change == _NET_WM_STATE_TOGGLE as u32 {
            con_set_urgency(con, !(*con).urgent);
            con = remanage_window(con);
        }
    } else if atom == A__NET_WM_STATE_STICKY {
        debuglog(
            c"%s:%s:%d - Received a client message to %s _NET_WM_STATE_STICKY.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_net_wm_state_change".as_ptr(),
            706 as ::core::ffi::c_int,
            debug_change,
        );
        if change == _NET_WM_STATE_ADD as u32 {
            (*con).sticky = true_0 != 0;
        } else if change == _NET_WM_STATE_REMOVE as u32 {
            (*con).sticky = false_0 != 0;
        } else if change == _NET_WM_STATE_TOGGLE as u32 {
            (*con).sticky = !(*con).sticky;
        }
        debuglog(
            c"%s:%s:%d - New sticky status for con = %p is %i.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_net_wm_state_change".as_ptr(),
            716 as ::core::ffi::c_int,
            con,
            (*con).sticky as ::core::ffi::c_int,
        );
        ewmh_update_sticky((*(*con).window).id, (*con).sticky);
        output_push_sticky_windows(focused);
        ewmh_update_wm_desktop();
    } else {
        debuglog(
            c"%s:%s:%d - Unknown atom in ClientMessage to %s type %u\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_net_wm_state_change".as_ptr(),
            721 as ::core::ffi::c_int,
            debug_change,
            atom,
        );
        return;
    }
    tree_render();
}
unsafe fn handle_client_message(event: *mut xcb_client_message_event_t) {
    if sn_xcb_display_process_event(sndisplay, event as *mut xcb_generic_event_t) != 0 {
        return;
    }
    verboselog(
        c"ClientMessage for window 0x%08x\n".as_ptr().cast_mut(),
        (*event).window,
    );
    if (*event).type_0 == A__NET_WM_STATE {
        if (*event).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            debuglog(
                c"%s:%s:%d - Unknown format %d in ClientMessage\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                742 as ::core::ffi::c_int,
                (*event).format as ::core::ffi::c_int,
            );
            return;
        }
        let con: *mut Con = con_by_window_id((*event).window);
        if con.is_null() {
            debuglog(
                c"%s:%s:%d - Could not get window for client message\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                748 as ::core::ffi::c_int,
            );
            return;
        }
        let mut i: usize = 0 as usize;
        while i
            < (::core::mem::size_of::<[u32; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<u32>() as usize)
                .wrapping_sub(1_usize)
        {
            handle_net_wm_state_change(
                con,
                (*event).data.data32[0 as ::core::ffi::c_int as usize],
                (*event).data.data32[i.wrapping_add(1 as usize) as usize],
            );
            i = i.wrapping_add(1);
        }
    } else if (*event).type_0 == A__NET_ACTIVE_WINDOW {
        if (*event).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int {
            return;
        }
        debuglog(
            c"%s:%s:%d - _NET_ACTIVE_WINDOW: Window 0x%08x should be activated\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            760 as ::core::ffi::c_int,
            (*event).window,
        );
        let mut con_0: *mut Con = con_by_window_id((*event).window);
        if con_0.is_null() {
            debuglog(
                c"%s:%s:%d - Could not get window for client message\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                764 as ::core::ffi::c_int,
            );
            return;
        }
        let ws: *mut Con = con_get_workspace(con_0);
        if ws.is_null() {
            debuglog(
                c"%s:%s:%d - Window is not being managed, ignoring _NET_ACTIVE_WINDOW\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                770 as ::core::ffi::c_int,
            );
            return;
        }
        if con_is_internal(ws) as ::core::ffi::c_int != 0
            && ws != workspace_get(c"__i3_scratch".as_ptr())
        {
            debuglog(
                c"%s:%s:%d - Workspace is internal but not scratchpad, ignoring _NET_ACTIVE_WINDOW\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                775 as ::core::ffi::c_int,
            );
            return;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize] == 2 as u32 {
            debuglog(
                c"%s:%s:%d - This request came from a pager. Focusing con = %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                783 as ::core::ffi::c_int,
                con_0,
            );
            if con_is_internal(ws) {
                scratchpad_show(con_0);
            } else {
                workspace_show(ws);
                focused_id = XCB_NONE as xcb_window_t;
                con_activate_unblock(con_0);
            }
        } else {
            if con_is_internal(ws) {
                debuglog(
                    c"%s:%s:%d - Ignoring request to make con = %p active because it's on an internal workspace.\n".as_ptr().cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_client_message".as_ptr(),
                    796 as ::core::ffi::c_int,
                    con_0,
                );
                return;
            }
            if config.focus_on_window_activation == FocusActivation::Focus
                || config.focus_on_window_activation == FocusActivation::Smart
                    && workspace_is_visible(ws) as ::core::ffi::c_int != 0
            {
                debuglog(
                    c"%s:%s:%d - Focusing con = %p\n".as_ptr().cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_client_message".as_ptr(),
                    801 as ::core::ffi::c_int,
                    con_0,
                );
                con_activate_unblock(con_0);
            } else if config.focus_on_window_activation == FocusActivation::Urgent
                || config.focus_on_window_activation == FocusActivation::Smart
                    && !workspace_is_visible(ws)
            {
                debuglog(
                    c"%s:%s:%d - Marking con = %p urgent\n".as_ptr().cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_client_message".as_ptr(),
                    804 as ::core::ffi::c_int,
                    con_0,
                );
                con_set_urgency(con_0, true_0 != 0);
                con_0 = remanage_window(con_0);
            } else {
                debuglog(
                    c"%s:%s:%d - Ignoring request for con = %p.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_client_message".as_ptr(),
                    808 as ::core::ffi::c_int,
                    con_0,
                );
            }
        }
        tree_render();
    } else if (*event).type_0 == A_I3_SYNC {
        let window: xcb_window_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let rnd: u32 = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        sync_respond(window, rnd);
    } else if (*event).type_0 == A__NET_REQUEST_FRAME_EXTENTS {
        debuglog(
            c"%s:%s:%d - _NET_REQUEST_FRAME_EXTENTS for window 0x%08x\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            830 as ::core::ffi::c_int,
            (*event).window,
        );
        let mut r: Rect = Rect {
            x: config.default_border_width as u32,
            y: config.default_border_width as u32,
            width: render_deco_height() as u32,
            height: config.default_border_width as u32,
        };
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            (*event).window,
            A__NET_FRAME_EXTENTS,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as u8,
            4 as u32,
            &raw mut r as *const ::core::ffi::c_void,
        );
        xcb_flush(conn);
    } else if (*event).type_0 == A_WM_CHANGE_STATE {
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            == XCB_ICCCM_WM_STATE_ICONIC as ::core::ffi::c_int as u32
        {
            debuglog(
                c"%s:%s:%d - Client has requested iconic state, rejecting. (window = %08x)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                852 as ::core::ffi::c_int,
                (*event).window,
            );
            let mut data: [::core::ffi::c_long; 2] = [
                XCB_ICCCM_WM_STATE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_long,
                XCB_NONE,
            ];
            xcb_change_property(
                conn,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                (*event).window,
                A_WM_STATE,
                A_WM_STATE,
                32 as u8,
                2 as u32,
                &raw mut data as *mut ::core::ffi::c_long as *const ::core::ffi::c_void,
            );
        } else {
            debuglog(
                c"%s:%s:%d - Not handling WM_CHANGE_STATE request. (window = %08x, state = %d)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                857 as ::core::ffi::c_int,
                (*event).window,
                (*event).data.data32[0 as ::core::ffi::c_int as usize],
            );
        }
    } else if (*event).type_0 == A__NET_CURRENT_DESKTOP {
        debuglog(
            c"%s:%s:%d - Request to change current desktop to index %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            865 as ::core::ffi::c_int,
            (*event).data.data32[0 as ::core::ffi::c_int as usize],
        );
        let ws_0: *mut Con =
            ewmh_get_workspace_by_index((*event).data.data32[0 as ::core::ffi::c_int as usize]);
        if ws_0.is_null() {
            errorlog(
                c"ERROR: Could not determine workspace for this index, ignoring request.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return;
        }
        debuglog(
            c"%s:%s:%d - Handling request to focus workspace %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            872 as ::core::ffi::c_int,
            (*ws_0).name,
        );
        workspace_show(ws_0);
        tree_render();
    } else if (*event).type_0 == A__NET_WM_DESKTOP {
        let index: u32 = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        debuglog(
            c"%s:%s:%d - Request to move window %d to EWMH desktop index %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            877 as ::core::ffi::c_int,
            (*event).window,
            index,
        );
        let con_1: *mut Con = con_by_window_id((*event).window);
        if con_1.is_null() {
            debuglog(
                c"%s:%s:%d - Couldn't find con for window %d, ignoring the request.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                881 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        if index == NET_WM_DESKTOP_ALL as u32 {
            debuglog(
                c"%s:%s:%d - The window was requested to be visible on all workspaces, making it sticky and floating.\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                888 as ::core::ffi::c_int,
            );
            if floating_enable(con_1, false_0 != 0) {
                (*con_1).floating = FloatingState::AutoOn;
                (*con_1).sticky = true_0 != 0;
                ewmh_update_sticky((*(*con_1).window).id, true_0 != 0);
                output_push_sticky_windows(focused);
                run_assignments((*con_1).window as *mut i3Window);
            }
        } else {
            let ws_1: *mut Con = ewmh_get_workspace_by_index(index);
            if ws_1.is_null() {
                errorlog(
                    c"ERROR: Could not determine workspace for this index, ignoring request.\n"
                        .as_ptr()
                        .cast_mut(),
                );
                return;
            }
            con_move_to_workspace(con_1, ws_1, true_0 != 0, false_0 != 0, false_0 != 0);
        }
        tree_render();
        ewmh_update_wm_desktop();
    } else if (*event).type_0 == A__NET_CLOSE_WINDOW {
        let con_2: *mut Con = con_by_window_id((*event).window);
        if !con_2.is_null() {
            debuglog(
                c"%s:%s:%d - Handling _NET_CLOSE_WINDOW request (con = %p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                918 as ::core::ffi::c_int,
                con_2,
            );
            if (*event).data.data32[0 as ::core::ffi::c_int as usize] != 0 {
                last_timestamp =
                    (*event).data.data32[0 as ::core::ffi::c_int as usize] as xcb_timestamp_t;
            }
            tree_close_internal(con_2, KillWindow::Window, false_0 != 0);
            tree_render();
        } else {
            debuglog(
                c"%s:%s:%d - Couldn't find con for _NET_CLOSE_WINDOW request. (window = %08x)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                927 as ::core::ffi::c_int,
                (*event).window,
            );
        }
    } else if (*event).type_0 == A__NET_WM_MOVERESIZE {
        let con_3: *mut Con = con_by_window_id((*event).window);
        if con_3.is_null() || !con_is_floating(con_3) {
            debuglog(
                c"%s:%s:%d - Couldn't find con for _NET_WM_MOVERESIZE request, or con not floating (window = %08x)\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_client_message".as_ptr(),
                936 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        debuglog(
            c"%s:%s:%d - Handling _NET_WM_MOVERESIZE request (con = %p)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            939 as ::core::ffi::c_int,
            con_3,
        );
        let direction: u32 = (*event).data.data32[2 as ::core::ffi::c_int as usize];
        let x_root: u32 = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let y_root: u32 = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut fake: xcb_button_press_event_t = xcb_button_press_event_t {
            response_type: 0,
            detail: (*event).data.data32[3 as ::core::ffi::c_int as usize] as xcb_button_t,
            sequence: 0,
            time: 0,
            root: 0,
            event: 0,
            child: 0,
            root_x: x_root as i16,
            root_y: y_root as i16,
            event_x: x_root.wrapping_sub((*con_3).rect.x) as i16,
            event_y: y_root.wrapping_sub((*con_3).rect.y) as i16,
            state: 0,
            same_screen: 0,
            pad0: 0,
        };
        match direction {
            8 => {
                floating_drag_window((*con_3).parent as *mut Con, &raw mut fake, false_0 != 0);
            }
            0 => {
                floating_resize_window((*con_3).parent as *mut Con, false_0 != 0, &raw mut fake);
            }
            _ => {
                debuglog(
                    c"%s:%s:%d - _NET_WM_MOVERESIZE direction %d not implemented\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_client_message".as_ptr(),
                    958 as ::core::ffi::c_int,
                    direction,
                );
            }
        }
    } else if (*event).type_0 == A__NET_MOVERESIZE_WINDOW {
        debuglog(
            c"%s:%s:%d - Received _NET_MOVE_RESIZE_WINDOW. Handling by faking a configure request.\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            962 as ::core::ffi::c_int,
        );
        let mut _generated_event: *mut ::core::ffi::c_void = scalloc(32 as usize, 1 as usize);
        let mut generated_event: *mut xcb_configure_request_event_t =
            _generated_event as *mut xcb_configure_request_event_t;
        (*generated_event).window = (*event).window;
        (*generated_event).response_type = XCB_CONFIGURE_REQUEST as u8;
        (*generated_event).value_mask = 0 as u16;
        if (*event).data.data32[0 as ::core::ffi::c_int as usize] & _NET_MOVERESIZE_WINDOW_X as u32
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_X as ::core::ffi::c_int)
                as u16;
            (*generated_event).x = (*event).data.data32[1 as ::core::ffi::c_int as usize] as i16;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize] & _NET_MOVERESIZE_WINDOW_Y as u32
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int)
                as u16;
            (*generated_event).y = (*event).data.data32[2 as ::core::ffi::c_int as usize] as i16;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_WIDTH as u32
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int)
                as u16;
            (*generated_event).width =
                (*event).data.data32[3 as ::core::ffi::c_int as usize] as u16;
        }
        if (*event).data.data32[0 as ::core::ffi::c_int as usize]
            & _NET_MOVERESIZE_WINDOW_HEIGHT as u32
            != 0
        {
            (*generated_event).value_mask = ((*generated_event).value_mask as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int)
                as u16;
            (*generated_event).height =
                (*event).data.data32[4 as ::core::ffi::c_int as usize] as u16;
        }
        handle_configure_request(generated_event);
        free(generated_event as *mut ::core::ffi::c_void);
        generated_event = ::core::ptr::null_mut::<xcb_configure_request_event_t>();
    } else {
        debuglog(
            c"%s:%s:%d - Skipping client message for unhandled type %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_client_message".as_ptr(),
            991 as ::core::ffi::c_int,
            (*event).type_0,
        );
    };
}
unsafe fn handle_window_type(con: *mut Con, reply: *mut xcb_get_property_reply_t) -> bool {
    window_update_type((*con).window as *mut i3Window, reply);
    true_0 != 0
}
unsafe fn handle_normal_hints(con: *mut Con, mut reply: *mut xcb_get_property_reply_t) -> bool {
    let changed: bool = window_update_normal_hints(
        (*con).window as *mut i3Window,
        reply,
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>(),
    );
    if changed {
        let floating: *mut Con = con_inside_floating(con);
        if !floating.is_null() {
            floating_check_size(con, false_0 != 0);
            tree_render();
        }
    }
    free(reply as *mut ::core::ffi::c_void);
    reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    true_0 != 0
}
unsafe fn handle_hints(con: *mut Con, reply: *mut xcb_get_property_reply_t) -> bool {
    let mut urgency_hint: bool = false;
    window_update_hints((*con).window as *mut i3Window, reply, &raw mut urgency_hint);
    con_set_urgency(con, urgency_hint);
    remanage_window(con);
    tree_render();
    true_0 != 0
}
unsafe fn handle_transient_for(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_transient_for((*con).window as *mut i3Window, prop);
    true_0 != 0
}
unsafe fn handle_clientleader_change(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_leader((*con).window as *mut i3Window, prop);
    true_0 != 0
}
unsafe fn handle_focus_in(event: *mut xcb_focus_in_event_t) {
    debuglog(
        c"%s:%s:%d - focus change in, for window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_focus_in".as_ptr(),
        1064 as ::core::ffi::c_int,
        (*event).event,
    );
    if (*event).event == root {
        debuglog(
            c"%s:%s:%d - Received focus in for root window, refocusing the focused window.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_focus_in".as_ptr(),
            1067 as ::core::ffi::c_int,
        );
        con_focus(focused);
        focused_id = XCB_NONE as xcb_window_t;
        x_push_changes(croot);
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = con_by_window_id((*event).event);
    if con.is_null() || (*con).window.is_null() {
        return;
    }
    debuglog(
        c"%s:%s:%d - That is con %p / %s\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_focus_in".as_ptr(),
        1077 as ::core::ffi::c_int,
        con,
        (*con).name,
    );
    if (*event).mode as ::core::ffi::c_int == XCB_NOTIFY_MODE_GRAB as ::core::ffi::c_int
        || (*event).mode as ::core::ffi::c_int == XCB_NOTIFY_MODE_UNGRAB as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - FocusIn event for grab/ungrab, ignoring\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_focus_in".as_ptr(),
            1081 as ::core::ffi::c_int,
        );
        return;
    }
    if (*event).detail as ::core::ffi::c_int == XCB_NOTIFY_DETAIL_POINTER as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - notify detail is pointer, ignoring this event\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_focus_in".as_ptr(),
            1086 as ::core::ffi::c_int,
        );
        return;
    }
    if focused_id == (*event).event && con_inside_floating(con).is_null() {
        debuglog(
            c"%s:%s:%d - focus matches the currently focused window, not doing anything\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_focus_in".as_ptr(),
            1093 as ::core::ffi::c_int,
        );
        return;
    }
    if (*(*con).parent).type_0 as ::core::ffi::c_uint
        == CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debuglog(
            c"%s:%s:%d - This is a dock client, not focusing.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_focus_in".as_ptr(),
            1099 as ::core::ffi::c_int,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - focus is different / refocusing floating window: updating decorations\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_focus_in".as_ptr(),
        1103 as ::core::ffi::c_int,
    );
    con_activate_unblock(con);
    focused_id = (*event).event;
    tree_render();
}
unsafe fn handle_focus_out(event: *mut xcb_focus_in_event_t) {
    let con: *mut Con = con_by_window_id((*event).event);
    let mut window_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mode: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut detail: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !con.is_null() {
        window_name = (*con).name;
        if window_name.is_null() {
            window_name = c"<unnamed con>".as_ptr();
        }
    } else if (*event).event == root {
        window_name = c"<the root window>".as_ptr();
    } else {
        window_name = c"<unknown window>".as_ptr();
    }
    match (*event).mode as ::core::ffi::c_int {
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
    match (*event).detail as ::core::ffi::c_int {
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
    debuglog(
        c"%s:%s:%d - focus change out: window 0x%08x (con %p, %s) lost focus with detail=%s, mode=%s\n".as_ptr().cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_focus_out".as_ptr(),
        1179 as ::core::ffi::c_int,
        (*event).event,
        con,
        window_name,
        detail,
        mode,
    );
}
unsafe fn handle_configure_notify(event: *mut xcb_configure_notify_event_t) {
    if (*event).event != root {
        debuglog(
            c"%s:%s:%d - ConfigureNotify for non-root window 0x%08x, ignoring\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_configure_notify".as_ptr(),
            1189 as ::core::ffi::c_int,
            (*event).event,
        );
        return;
    }
    debuglog(
        c"%s:%s:%d - ConfigureNotify for root window 0x%08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/handlers.c".as_ptr(),
        c"handle_configure_notify".as_ptr(),
        1192 as ::core::ffi::c_int,
        (*event).event,
    );
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
unsafe fn handle_selection_clear(event: *mut xcb_selection_clear_event_t) {
    if (*event).selection != wm_sn {
        debuglog(
            c"%s:%s:%d - SelectionClear for unknown selection %d, ignoring\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_selection_clear".as_ptr(),
            1208 as ::core::ffi::c_int,
            (*event).selection,
        );
        return;
    }
    verboselog(c"Lost WM_Sn selection, exiting.\n".as_ptr().cast_mut());
    exit(EXIT_SUCCESS);
}
unsafe fn handle_class_change(mut con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_class((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    true_0 != 0
}
unsafe fn handle_machine_change(mut con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_machine((*con).window as *mut i3Window, prop);
    con = remanage_window(con);
    true_0 != 0
}
unsafe fn handle_motif_hints_change(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    let mut motif_border_style: BorderStyle = BorderStyle::None;
    let has_mwm_hints: bool = window_update_motif_hints(
        (*con).window as *mut i3Window,
        prop,
        &raw mut motif_border_style,
    );
    if has_mwm_hints as ::core::ffi::c_int != 0 && motif_border_style != (*con).border_style {
        debuglog(
            c"%s:%s:%d - Update border style of con %p to %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_motif_hints_change".as_ptr(),
            1246 as ::core::ffi::c_int,
            con,
            motif_border_style as ::core::ffi::c_uint,
        );
        con_set_border_style(con, motif_border_style, (*con).current_border_width);
        x_push_changes(croot);
    }
    true_0 != 0
}
unsafe fn handle_strut_partial_change(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_strut_partial((*con).window as *mut i3Window, prop);
    if (*con).parent.is_null()
        || (*(*con).parent).type_0 as ::core::ffi::c_uint
            != CT_DOCKAREA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return true_0 != 0;
    }
    let mut search_at: *mut Con = croot;
    let output: *mut Con = con_get_output(con);
    if !output.is_null() {
        debuglog(
            c"%s:%s:%d - Starting search at output %s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_strut_partial_change".as_ptr(),
            1270 as ::core::ffi::c_int,
            (*output).name,
        );
        search_at = output;
    }
    if (*(*con).window).reserved.top > 0 as u32 && (*(*con).window).reserved.bottom == 0 as u32 {
        debuglog(
            c"%s:%s:%d - Top dock client\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_strut_partial_change".as_ptr(),
            1276 as ::core::ffi::c_int,
        );
        (*(*con).window).dock = WindowDock::Top;
    } else if (*(*con).window).reserved.top == 0 as u32
        && (*(*con).window).reserved.bottom > 0 as u32
    {
        debuglog(
            c"%s:%s:%d - Bottom dock client\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_strut_partial_change".as_ptr(),
            1279 as ::core::ffi::c_int,
        );
        (*(*con).window).dock = WindowDock::Bottom;
    } else {
        debuglog(
            c"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n".as_ptr().cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_strut_partial_change".as_ptr(),
            1282 as ::core::ffi::c_int,
        );
        if (*con).geometry.y < (*search_at).rect.height.wrapping_div(2 as u32) {
            debuglog(
                c"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_strut_partial_change".as_ptr(),
                1285 as ::core::ffi::c_int,
                (*con).geometry.y,
                (*search_at).rect.height.wrapping_div(2 as u32),
            );
            (*(*con).window).dock = WindowDock::Top;
        } else {
            debuglog(
                c"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_strut_partial_change".as_ptr(),
                1289 as ::core::ffi::c_int,
                (*con).geometry.y,
                (*search_at).rect.height.wrapping_div(2 as u32),
            );
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
    con_attach(con, dockarea, true_0 != 0);
    tree_render();
    true_0 != 0
}
unsafe fn handle_windowicon_change(con: *mut Con, prop: *mut xcb_get_property_reply_t) -> bool {
    window_update_icon((*con).window as *mut i3Window, prop);
    x_push_changes(croot);
    true_0 != 0
}
static mut property_handlers: [property_handler_t; 13] = [property_handler_t {
    atom: 0,
    long_len: 0,
    cb: None,
}; 13];
pub const NUM_HANDLERS: usize = (::core::mem::size_of::<[property_handler_t; 13]>() as usize)
    .wrapping_div(::core::mem::size_of::<property_handler_t>() as usize);

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
        XCB_ATOM_WM_HINTS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[2 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[3 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_NORMAL_HINTS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[4 as ::core::ffi::c_int as usize].atom = A_WM_CLIENT_LEADER;
    property_handlers[5 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_TRANSIENT_FOR as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[6 as ::core::ffi::c_int as usize].atom = A_WM_WINDOW_ROLE;
    property_handlers[7 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[8 as ::core::ffi::c_int as usize].atom = A__NET_WM_STRUT_PARTIAL;
    property_handlers[9 as ::core::ffi::c_int as usize].atom = A__NET_WM_WINDOW_TYPE;
    property_handlers[10 as ::core::ffi::c_int as usize].atom =
        XCB_ATOM_WM_CLIENT_MACHINE as ::core::ffi::c_int as xcb_atom_t;
    property_handlers[11 as ::core::ffi::c_int as usize].atom = A__MOTIF_WM_HINTS;
    property_handlers[12 as ::core::ffi::c_int as usize].atom = A__NET_WM_ICON;
}
unsafe fn property_notify(state: u8, window: xcb_window_t, atom: xcb_atom_t) {
    let mut handler: *mut property_handler_t = ::core::ptr::null_mut::<property_handler_t>();
    let mut propr: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut err: *mut xcb_generic_error_t = ::core::ptr::null_mut::<xcb_generic_error_t>();
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut c: usize = 0 as usize;
    while c < NUM_HANDLERS {
        if property_handlers[c as usize].atom != atom {
            c = c.wrapping_add(1);
        } else {
            handler = (&raw mut property_handlers as *mut property_handler_t).offset(c as isize)
                as *mut property_handler_t;
            break;
        }
    }
    if handler.is_null() {
        return;
    }
    con = con_by_window_id(window);
    if con.is_null() || (*con).window.is_null() {
        debuglog(
            c"%s:%s:%d - Received property for atom %d for unknown client\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"property_notify".as_ptr(),
            1385 as ::core::ffi::c_int,
            atom,
        );
        return;
    }
    if state as ::core::ffi::c_int != XCB_PROPERTY_DELETE as ::core::ffi::c_int {
        let cookie: xcb_get_property_cookie_t = xcb_get_property(
            conn,
            0 as u8,
            window,
            atom,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as u32,
            (*handler).long_len,
        );
        propr = xcb_get_property_reply(conn, cookie, &raw mut err);
        if !err.is_null() {
            debuglog(
                c"%s:%s:%d - got error %d when getting property of atom %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"property_notify".as_ptr(),
                1393 as ::core::ffi::c_int,
                (*err).error_code as ::core::ffi::c_int,
                atom,
            );
            free(err as *mut ::core::ffi::c_void);
            err = ::core::ptr::null_mut::<xcb_generic_error_t>();
            return;
        }
    }
    if !(*handler).cb.expect("non-null function pointer")(con, propr) {
        free(propr as *mut ::core::ffi::c_void);
        propr = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    }
}

pub unsafe fn handle_event(type_0: ::core::ffi::c_int, event: *mut xcb_generic_event_t) {
    if type_0 != XCB_MOTION_NOTIFY {
        debuglog(
            c"%s:%s:%d - event type %d, xkb_base %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_event".as_ptr(),
            1412 as ::core::ffi::c_int,
            type_0,
            xkb_base,
        );
    }
    if randr_base > -(1 as ::core::ffi::c_int)
        && type_0 == randr_base + XCB_RANDR_SCREEN_CHANGE_NOTIFY
    {
        handle_screen_change(event);
        return;
    }
    if xkb_base > -(1 as ::core::ffi::c_int) && type_0 == xkb_base {
        debuglog(
            c"%s:%s:%d - xkb event, need to handle it.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_event".as_ptr(),
            1422 as ::core::ffi::c_int,
        );
        let state: *mut xcb_xkb_state_notify_event_t = event as *mut xcb_xkb_state_notify_event_t;
        if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_NEW_KEYBOARD_NOTIFY {
            debuglog(
                c"%s:%s:%d - xkb new keyboard notify, sequence %d, time %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_event".as_ptr(),
                1426 as ::core::ffi::c_int,
                (*state).sequence as ::core::ffi::c_int,
                (*state).time,
            );
            xcb_key_symbols_free(keysyms);
            keysyms = xcb_key_symbols_alloc(conn);
            if (*(event as *mut xcb_xkb_new_keyboard_notify_event_t)).changed as ::core::ffi::c_int
                & XCB_XKB_NKN_DETAIL_KEYCODES as ::core::ffi::c_int
                != 0
            {
                load_keymap();
            }
            ungrab_all_keys(conn);
            translate_keysyms();
            grab_all_keys(conn);
        } else if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_MAP_NOTIFY {
            if event_is_ignored(
                ignore_event_sequence_t {
                    value: (*event).full_sequence,
                },
                type_0,
            ) {
                debuglog(
                    c"%s:%s:%d - Ignoring map notify event for sequence %d.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_event".as_ptr(),
                    1437 as ::core::ffi::c_int,
                    (*state).sequence as ::core::ffi::c_int,
                );
            } else {
                debuglog(
                    c"%s:%s:%d - xkb map notify, sequence %d, time %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/handlers.c".as_ptr(),
                    c"handle_event".as_ptr(),
                    1439 as ::core::ffi::c_int,
                    (*state).sequence as ::core::ffi::c_int,
                    (*state).time,
                );
                add_ignore_event(
                    ignore_event_sequence_t {
                        value: (*event).full_sequence,
                    },
                    type_0,
                );
                xcb_key_symbols_free(keysyms);
                keysyms = xcb_key_symbols_alloc(conn);
                ungrab_all_keys(conn);
                translate_keysyms();
                grab_all_keys(conn);
                load_keymap();
            }
        } else if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_STATE_NOTIFY {
            debuglog(
                c"%s:%s:%d - xkb state group = %d\n".as_ptr().cast_mut(),
                c"../src/handlers.c".as_ptr(),
                c"handle_event".as_ptr(),
                1449 as ::core::ffi::c_int,
                (*state).group as ::core::ffi::c_int,
            );
            if xkb_current_group == (*state).group as ::core::ffi::c_int {
                return;
            }
            xkb_current_group = (*state).group as ::core::ffi::c_int;
            ungrab_all_keys(conn);
            grab_all_keys(conn);
        }
        return;
    }
    if shape_supported as ::core::ffi::c_int != 0 && type_0 == shape_base + XCB_SHAPE_NOTIFY {
        let shape: *mut xcb_shape_notify_event_t = event as *mut xcb_shape_notify_event_t;
        debuglog(
            c"%s:%s:%d - shape_notify_event for window 0x%08x, shape_kind = %d, shaped = %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/handlers.c".as_ptr(),
            c"handle_event".as_ptr(),
            1465 as ::core::ffi::c_int,
            (*shape).affected_window,
            (*shape).shape_kind as ::core::ffi::c_int,
            (*shape).shaped as ::core::ffi::c_int,
        );
        let con: *mut Con = con_by_window_id((*shape).affected_window);
        if con.is_null() {
            verboselog(
                c"Not a managed window 0x%08x, ignoring shape_notify_event\n"
                    .as_ptr()
                    .cast_mut(),
                (*shape).affected_window,
            );
            return;
        }
        if (*shape).shape_kind as ::core::ffi::c_int == XCB_SHAPE_SK_BOUNDING as ::core::ffi::c_int
            || (*shape).shape_kind as ::core::ffi::c_int == XCB_SHAPE_SK_INPUT as ::core::ffi::c_int
        {
            x_set_shape(
                con,
                (*shape).shape_kind as xcb_shape_sk_t,
                (*shape).shaped != 0,
            );
        }
        return;
    }
    match type_0 {
        XCB_KEY_PRESS | XCB_KEY_RELEASE => {
            handle_key_press(event as *mut xcb_key_press_event_t);
        }
        XCB_BUTTON_PRESS | XCB_BUTTON_RELEASE => {
            handle_button_press(event as *mut xcb_button_press_event_t);
        }
        XCB_MAP_REQUEST => {
            handle_map_request(
                event as *mut xcb_map_request_event_t,
                (*event).full_sequence,
            );
        }
        XCB_UNMAP_NOTIFY => {
            handle_unmap_notify_event(
                event as *mut xcb_unmap_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_DESTROY_NOTIFY => {
            handle_destroy_notify_event(
                event as *mut xcb_destroy_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_EXPOSE => {
            if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                handle_expose_event(event as *mut xcb_expose_event_t);
            }
        }
        XCB_MOTION_NOTIFY => {
            handle_motion_notify(event as *mut xcb_motion_notify_event_t);
        }
        XCB_ENTER_NOTIFY => {
            handle_enter_notify(
                event as *mut xcb_enter_notify_event_t,
                (*event).full_sequence,
            );
        }
        XCB_CLIENT_MESSAGE => {
            handle_client_message(event as *mut xcb_client_message_event_t);
        }
        XCB_CONFIGURE_REQUEST => {
            handle_configure_request(event as *mut xcb_configure_request_event_t);
        }
        XCB_MAPPING_NOTIFY => {
            handle_mapping_notify(event as *mut xcb_mapping_notify_event_t);
        }
        XCB_FOCUS_IN => {
            handle_focus_in(event as *mut xcb_focus_in_event_t);
        }
        XCB_FOCUS_OUT => {
            handle_focus_out(event as *mut xcb_focus_in_event_t);
        }
        XCB_PROPERTY_NOTIFY => {
            let e: *mut xcb_property_notify_event_t = event as *mut xcb_property_notify_event_t;
            last_timestamp = (*e).time;
            property_notify((*e).state, (*e).window, (*e).atom);
        }
        XCB_CONFIGURE_NOTIFY => {
            handle_configure_notify(event as *mut xcb_configure_notify_event_t);
        }
        XCB_SELECTION_CLEAR => {
            handle_selection_clear(event as *mut xcb_selection_clear_event_t);
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
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as u32,
            cb: Some(
                handle_windowname_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(handle_hints as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as u32,
            cb: Some(
                handle_windowname_change_legacy
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_normal_hints as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_clientleader_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_transient_for as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as u32,
            cb: Some(
                handle_windowrole_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as u32,
            cb: Some(
                handle_class_change as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_strut_partial_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_window_type as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 128 as u32,
            cb: Some(
                handle_machine_change as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: 5_usize.wrapping_mul(::core::mem::size_of::<u64>() as usize) as u32,
            cb: Some(
                handle_motif_hints_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
            ),
        },
        property_handler_t {
            atom: 0 as xcb_atom_t,
            long_len: UINT_MAX as u32,
            cb: Some(
                handle_windowicon_change
                    as unsafe fn(*mut Con, *mut xcb_get_property_reply_t) -> bool,
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
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_button_press_event_t, xcb_button_t, xcb_change_property,
    xcb_client_message_event_t, xcb_configure_notify_event_t, xcb_configure_request_event_t,
    xcb_configure_window, xcb_delete_property, xcb_destroy_notify_event_t, xcb_drawable_t,
    xcb_enter_notify_event_t, xcb_expose_event_t, xcb_flush, xcb_focus_in_event_t,
    xcb_generic_error_t, xcb_generic_event_t, xcb_get_geometry, xcb_get_geometry_cookie_t,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_input_focus,
    xcb_get_input_focus_cookie_t, xcb_get_input_focus_reply, xcb_get_property,
    xcb_get_property_cookie_t, xcb_get_property_reply, xcb_get_property_reply_t,
    xcb_get_window_attributes_cookie_t, xcb_get_window_attributes_unchecked, xcb_key_press_event_t,
    xcb_key_symbols_alloc, xcb_key_symbols_free, xcb_map_request_event_t,
    xcb_mapping_notify_event_t, xcb_motion_notify_event_t, xcb_property_notify_event_t,
    xcb_refresh_keyboard_mapping, xcb_selection_clear_event_t, xcb_shape_notify_event_t,
    xcb_shape_sk_t, xcb_timestamp_t, xcb_unmap_notify_event_t, xcb_window_t,
    xcb_xkb_new_keyboard_notify_event_t, xcb_xkb_state_notify_event_t, XCB_ATOM_CARDINAL,
    XCB_ATOM_WM_CLASS, XCB_ATOM_WM_CLIENT_MACHINE, XCB_ATOM_WM_HINTS, XCB_ATOM_WM_NAME,
    XCB_ATOM_WM_NORMAL_HINTS, XCB_ATOM_WM_TRANSIENT_FOR, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE,
    XCB_CLIENT_MESSAGE, XCB_CONFIGURE_NOTIFY, XCB_CONFIGURE_REQUEST,
    XCB_CONFIG_WINDOW_BORDER_WIDTH, XCB_CONFIG_WINDOW_HEIGHT, XCB_CONFIG_WINDOW_SIBLING,
    XCB_CONFIG_WINDOW_STACK_MODE, XCB_CONFIG_WINDOW_WIDTH, XCB_CONFIG_WINDOW_X,
    XCB_CONFIG_WINDOW_Y, XCB_DESTROY_NOTIFY, XCB_ENTER_NOTIFY, XCB_EXPOSE, XCB_FOCUS_IN,
    XCB_FOCUS_OUT, XCB_GET_PROPERTY_TYPE_ANY, XCB_ICCCM_WM_STATE_ICONIC, XCB_ICCCM_WM_STATE_NORMAL,
    XCB_KEY_PRESS, XCB_KEY_RELEASE, XCB_MAPPING_KEYBOARD, XCB_MAPPING_MODIFIER, XCB_MAPPING_NOTIFY,
    XCB_MAP_REQUEST, XCB_MOTION_NOTIFY, XCB_NONE, XCB_NOTIFY_DETAIL_POINTER, XCB_NOTIFY_MODE_GRAB,
    XCB_NOTIFY_MODE_NORMAL, XCB_NOTIFY_MODE_UNGRAB, XCB_NUM_LOCK, XCB_PROPERTY_DELETE,
    XCB_PROPERTY_NOTIFY, XCB_PROP_MODE_REPLACE, XCB_RANDR_SCREEN_CHANGE_NOTIFY,
    XCB_SELECTION_CLEAR, XCB_SHAPE_NOTIFY, XCB_SHAPE_SK_BOUNDING, XCB_SHAPE_SK_INPUT,
    XCB_STACK_MODE_ABOVE, XCB_UNMAP_NOTIFY, XCB_XKB_MAP_NOTIFY, XCB_XKB_NEW_KEYBOARD_NOTIFY,
    XCB_XKB_NKN_DETAIL_KEYCODES, XCB_XKB_STATE_NOTIFY,
};

use startup_notification_sys::{
    sn_monitor_context_new, sn_xcb_display_process_event, SnMonitorEvent,
};

pub use libc::{exit, free, strcmp, time};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::nodes_head;
pub use crate::types::xoutput;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::conn_screen;
use crate::application::force_xinerama;
use crate::application::keysyms;
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
use i3_common::get_mod_mask::aio_get_mod_mask_for;
use i3_common::string::i3string_as_utf8;

use crate::xcb::xcb_numlock_mask;
