use crate::tree::all_cons;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * manage.c: Initially managing new windows (or existing ones on restart).
 *
 */
// Mechanical file-by-file Rust translation of src/manage.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_3 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_3 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_3 = 0;

pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_8 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_8 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_8 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_8 = 0;
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

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_10 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_10 = 1;
pub const M_HERE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_11 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_11 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_11 = 4;
pub const WM_TILING: C2RustUnnamed_11 = 3;
pub const WM_TILING_USER: C2RustUnnamed_11 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_11 = 1;
pub const WM_ANY: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_12 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_12 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_12 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_12 = 1;
pub const M_NODOCK: C2RustUnnamed_12 = 0;
pub const M_DONTCHECK: C2RustUnnamed_12 = -1;
pub type C2RustUnnamed_13 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_13 = 1;
pub const U_LATEST: C2RustUnnamed_13 = 0;
pub const U_DONTCHECK: C2RustUnnamed_13 = -1;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_14 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_14 = 1;
pub const W_NODOCK: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_17 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_17 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_17 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_17 = 2;
pub const A_COMMAND: C2RustUnnamed_17 = 1;
pub const A_ANY: C2RustUnnamed_17 = 0;

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_19 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_19 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_19 = 3;
pub const CT_CON: C2RustUnnamed_19 = 2;
pub const CT_OUTPUT: C2RustUnnamed_19 = 1;
pub const CT_ROOT: C2RustUnnamed_19 = 0;

pub type Output = xoutput;
pub type i3Window = Window;
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

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_21 = 3;
pub const PDF_IGNORE: C2RustUnnamed_21 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_21 = 1;
pub const PDF_SMART: C2RustUnnamed_21 = 0;

pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_22 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_22 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_23 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_23 = 2;
pub const FOWA_URGENT: C2RustUnnamed_23 = 1;
pub const FOWA_SMART: C2RustUnnamed_23 = 0;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
unsafe fn _match_depth(win: *mut i3Window, con: *mut Con) -> xcb_window_t {
    let mut old_frame: xcb_window_t = XCB_NONE as xcb_window_t;
    if (*con).depth as ::core::ffi::c_int != (*win).depth as ::core::ffi::c_int {
        old_frame = (*con).frame.id as xcb_window_t;
        (*con).depth = (*win).depth;
        x_con_reframe(con);
    }
    old_frame
}
unsafe fn _remove_matches(con: *mut Con) {
    while !(*con).swallow_head.tqh_first.is_null() {
        let first: *mut Match = (*con).swallow_head.tqh_first as *mut Match;
        if !(*first).matches.tqe_next.is_null() {
            (*(*first).matches.tqe_next).matches.tqe_prev = (*first).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*first).matches.tqe_prev;
        }
        *(*first).matches.tqe_prev = (*first).matches.tqe_next;
        match_free(first);
        free(first as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn manage_existing_windows(root_0: xcb_window_t) {
    let mut reply: *mut xcb_query_tree_reply_t = ::core::ptr::null_mut::<xcb_query_tree_reply_t>();
    reply = xcb_query_tree_reply(
        conn,
        xcb_query_tree(conn, root_0),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        return;
    }
    let len: ::core::ffi::c_int = xcb_query_tree_children_length(reply);
    let cookies: *mut xcb_get_window_attributes_cookie_t = smalloc(
        (len as size_t)
            .wrapping_mul(::core::mem::size_of::<xcb_get_window_attributes_cookie_t>() as size_t),
    )
        as *mut xcb_get_window_attributes_cookie_t;
    let children: *mut xcb_window_t = xcb_query_tree_children(reply);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        *cookies.offset(i as isize) = xcb_get_window_attributes(conn, *children.offset(i as isize));
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < len {
        manage_window(
            *children.offset(i_0 as isize),
            *cookies.offset(i_0 as isize),
            true_0 != 0,
        );
        i_0 += 1;
    }
    free(reply as *mut ::core::ffi::c_void);
    free(cookies as *mut ::core::ffi::c_void);
}

pub unsafe fn restore_geometry() {
    debuglog(
        c"%s:%s:%d - Restoring geometry\n".as_ptr().cast_mut(),
        c"../src/manage.c".as_ptr(),
        c"restore_geometry".as_ptr(),
        79 as ::core::ffi::c_int,
    );
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() {
            debuglog(
                c"%s:%s:%d - Re-adding X11 border of %d px\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"restore_geometry".as_ptr(),
                84 as ::core::ffi::c_int,
                (*con).border_width,
            );
            (*con).window_rect.width = (*con)
                .window_rect
                .width
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
            (*con).window_rect.height = (*con)
                .window_rect
                .height
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as uint32_t);
            xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
            debuglog(
                c"%s:%s:%d - placing window %08x at %d %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"restore_geometry".as_ptr(),
                88 as ::core::ffi::c_int,
                (*(*con).window).id,
                (*con).rect.x,
                (*con).rect.y,
            );
            xcb_reparent_window(
                conn,
                (*(*con).window).id,
                root,
                (*con).rect.x as int16_t,
                (*con).rect.y as int16_t,
            );
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    let mut fresh0: [uint32_t; 1] =
        [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as uint32_t];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
        &raw mut fresh0 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_aux_sync(conn);
}

pub unsafe fn manage_window(
    window: xcb_window_t,
    cookie: xcb_get_window_attributes_cookie_t,
    needs_to_be_mapped: bool,
) {
    let mut values: [uint32_t; 1] = [0; 1];
    let mut event_mask_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut cwindow: *mut i3Window = ::core::ptr::null_mut::<i3Window>();
    let mut buttons: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut urgency_hint: bool = false;
    let mut motif_border_style: border_style_t = BS_NONE;
    let mut has_mwm_hints: bool = false;
    let mut type_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut state_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_id_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_ws: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wm_desktop_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut search_at: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut nc: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_from_restart_mode: bool = false;
    let mut old_frame: xcb_window_t = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut fs: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut set_focus: bool = false;
    let mut want_floating: bool = false;
    let mut rcookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    debuglog(
        c"%s:%s:%d - window 0x%08x\n".as_ptr().cast_mut(),
        c"../src/manage.c".as_ptr(),
        c"manage_window".as_ptr(),
        108 as ::core::ffi::c_int,
        window,
    );
    let d: xcb_drawable_t = window;
    let mut geomc: xcb_get_geometry_cookie_t = xcb_get_geometry_cookie_t { sequence: 0 };
    let mut geom: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut attr: *mut xcb_get_window_attributes_reply_t =
        ::core::ptr::null_mut::<xcb_get_window_attributes_reply_t>();
    let mut wm_type_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut strut_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut state_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut utf8_title_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut title_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut class_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut leader_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut transient_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut role_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut startup_id_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_hints_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_normal_hints_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut motif_wm_hints_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_user_time_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_desktop_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_machine_cookie: xcb_get_property_cookie_t =
        xcb_get_property_cookie_t { sequence: 0 };
    let mut wm_icon_cookie: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
    geomc = xcb_get_geometry(conn, d);
    attr = xcb_get_window_attributes_reply(
        conn,
        cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if attr.is_null() {
        debuglog(
            c"%s:%s:%d - Could not get attributes\n".as_ptr().cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"manage_window".as_ptr(),
            129 as ::core::ffi::c_int,
        );
        xcb_discard_reply(conn, geomc.sequence);
        return;
    }
    if needs_to_be_mapped as ::core::ffi::c_int != 0
        && (*attr).map_state as ::core::ffi::c_int != XCB_MAP_STATE_VIEWABLE as ::core::ffi::c_int
    {
        xcb_discard_reply(conn, geomc.sequence);
    } else if (*attr).override_redirect != 0 {
        xcb_discard_reply(conn, geomc.sequence);
    } else if !con_by_window_id(window).is_null() {
        debuglog(
            c"%s:%s:%d - already managed (by con %p)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"manage_window".as_ptr(),
            147 as ::core::ffi::c_int,
            con_by_window_id(window),
        );
        xcb_discard_reply(conn, geomc.sequence);
    } else {
        geom = xcb_get_geometry_reply(
            conn,
            geomc,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geom.is_null() {
            debuglog(
                c"%s:%s:%d - could not get geometry\n".as_ptr().cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"manage_window".as_ptr(),
                154 as ::core::ffi::c_int,
            );
        } else {
            values = [0; 1];
            values[0 as ::core::ffi::c_int as usize] = (XCB_EVENT_MASK_PROPERTY_CHANGE
                as ::core::ffi::c_int
                | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int)
                as uint32_t;
            event_mask_cookie = xcb_change_window_attributes_checked(
                conn,
                window,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
            );
            if !xcb_request_check(conn, event_mask_cookie).is_null() {
                verboselog(
                    c"Could not change event mask, the window probably already disappeared.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                wm_type_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_WINDOW_TYPE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                strut_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_STRUT_PARTIAL,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                state_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_STATE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                utf8_title_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_NAME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                leader_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A_WM_CLIENT_LEADER,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                transient_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_TRANSIENT_FOR as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                title_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                class_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                role_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A_WM_WINDOW_ROLE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    128 as uint32_t,
                );
                startup_id_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_STARTUP_ID,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    512 as uint32_t,
                );
                wm_hints_cookie = xcb_icccm_get_wm_hints(conn, window);
                wm_normal_hints_cookie = xcb_icccm_get_wm_normal_hints(conn, window);
                motif_wm_hints_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__MOTIF_WM_HINTS,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    5_usize.wrapping_mul(::core::mem::size_of::<uint64_t>() as usize) as uint32_t,
                );
                wm_user_time_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_USER_TIME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_desktop_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_DESKTOP,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_machine_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    XCB_ATOM_WM_CLIENT_MACHINE as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                wm_icon_cookie = xcb_get_property(
                    conn,
                    false_0 as uint8_t,
                    window,
                    A__NET_WM_ICON,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as uint32_t,
                    4294967295 as uint32_t,
                );
                cwindow = scalloc(1 as size_t, ::core::mem::size_of::<i3Window>() as size_t)
                    as *mut i3Window;
                (*cwindow).id = window;
                (*cwindow).depth = get_visual_depth((*attr).visual);
                buttons = bindings_get_buttons_to_grab();
                xcb_grab_buttons(conn, window, buttons);
                free(buttons as *mut ::core::ffi::c_void);
                buttons = ::core::ptr::null_mut::<::core::ffi::c_int>();
                window_update_class(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        class_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_name_legacy(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        title_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_name(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        utf8_title_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_icon(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_icon_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_leader(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        leader_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_transient_for(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        transient_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_strut_partial(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        strut_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                window_update_role(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        role_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                urgency_hint = false;
                window_update_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    &raw mut urgency_hint,
                );
                motif_border_style = BS_NONE;
                has_mwm_hints = window_update_motif_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        motif_wm_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    &raw mut motif_border_style,
                );
                window_update_normal_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_normal_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    geom,
                );
                window_update_machine(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        wm_machine_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                );
                type_reply = xcb_get_property_reply(
                    conn,
                    wm_type_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                state_reply = xcb_get_property_reply(
                    conn,
                    state_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                startup_id_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                startup_id_reply = xcb_get_property_reply(
                    conn,
                    startup_id_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                startup_ws = startup_workspace_for_window(cwindow, startup_id_reply);
                debuglog(
                    c"%s:%s:%d - startup workspace = %s\n".as_ptr().cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    227 as ::core::ffi::c_int,
                    startup_ws,
                );
                wm_desktop_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                wm_desktop_reply = xcb_get_property_reply(
                    conn,
                    wm_desktop_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as uint32_t;
                if !wm_desktop_reply.is_null()
                    && xcb_get_property_value_length(wm_desktop_reply) != 0 as ::core::ffi::c_int
                {
                    let wm_desktops: *mut uint32_t =
                        xcb_get_property_value(wm_desktop_reply) as *mut uint32_t;
                    (*cwindow).wm_desktop = *wm_desktops.offset(0 as ::core::ffi::c_int as isize)
                        as int32_t as uint32_t;
                }
                free(wm_desktop_reply as *mut ::core::ffi::c_void);
                wm_desktop_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                (*cwindow).needs_take_focus =
                    window_supports_protocol((*cwindow).id, A_WM_TAKE_FOCUS);
                (*cwindow).window_type = xcb_get_preferred_window_type(type_reply);
                search_at = croot;
                if xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_DOCK) {
                    verboselog(c"This window is of type dock\n".as_ptr().cast_mut());
                    let output: *mut Output = get_output_containing(
                        (*geom).x as ::core::ffi::c_uint,
                        (*geom).y as ::core::ffi::c_uint,
                    );
                    if !output.is_null() {
                        debuglog(
                            c"%s:%s:%d - Starting search at output %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            252 as ::core::ffi::c_int,
                            output_primary_name(output),
                        );
                        search_at = (*output).con;
                    }
                    if (*cwindow).reserved.top > 0 as uint32_t
                        && (*cwindow).reserved.bottom == 0 as uint32_t
                    {
                        debuglog(
                            c"%s:%s:%d - Top dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            258 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = W_DOCK_TOP;
                    } else if (*cwindow).reserved.top == 0 as uint32_t
                        && (*cwindow).reserved.bottom > 0 as uint32_t
                    {
                        debuglog(
                            c"%s:%s:%d - Bottom dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            261 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = W_DOCK_BOTTOM;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            264 as ::core::ffi::c_int,
                        );
                        if ((*geom).y as ::core::ffi::c_int)
                            < (*search_at).rect.height.wrapping_div(2 as uint32_t) as int16_t
                                as ::core::ffi::c_int
                        {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                267 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as uint32_t),
                            );
                            (*cwindow).dock = W_DOCK_TOP;
                        } else {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                271 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as uint32_t),
                            );
                            (*cwindow).dock = W_DOCK_BOTTOM;
                        }
                    }
                }
                debuglog(
                    c"%s:%s:%d - Initial geometry: (%d, %d, %d, %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    277 as ::core::ffi::c_int,
                    (*geom).x as ::core::ffi::c_int,
                    (*geom).y as ::core::ffi::c_int,
                    (*geom).width as ::core::ffi::c_int,
                    (*geom).height as ::core::ffi::c_int,
                );
                (*cwindow).swallowed = false_0 != 0;
                match_0 = ::core::ptr::null_mut::<Match>();
                nc = con_for_window(search_at, cwindow, &raw mut match_0);
                match_from_restart_mode =
                    !match_0.is_null() && (*match_0).restart_mode as ::core::ffi::c_int != 0;
                if nc.is_null() {
                    let mut wm_desktop_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                    let mut assignment: *mut Assignment = ::core::ptr::null_mut::<Assignment>();
                    assignment = assignment_for(cwindow, A_TO_WORKSPACE as ::core::ffi::c_int);
                    if !assignment.is_null() || {
                        assignment =
                            assignment_for(cwindow, A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int);
                        !assignment.is_null()
                    } {
                        debuglog(
                            c"%s:%s:%d - Assignment matches (%p)\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            291 as ::core::ffi::c_int,
                            match_0,
                        );
                        let mut assigned_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                        if (*assignment).type_0 as ::core::ffi::c_uint
                            == A_TO_WORKSPACE_NUMBER as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let parsed_num: ::core::ffi::c_long =
                                ws_name_to_number((*assignment).dest.workspace)
                                    as ::core::ffi::c_long;
                            assigned_ws =
                                get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
                        }
                        if assigned_ws.is_null() {
                            assigned_ws = workspace_get((*assignment).dest.workspace);
                        }
                        nc = con_descend_tiling_focused(assigned_ws);
                        debuglog(
                            c"%s:%s:%d - focused on ws %s: %p / %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            306 as ::core::ffi::c_int,
                            (*assigned_ws).name,
                            nc,
                            (*nc).name,
                        );
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                        if !workspace_is_visible(assigned_ws) {
                            urgency_hint = true_0 != 0;
                        }
                    } else if (*cwindow).wm_desktop != NET_WM_DESKTOP_NONE as uint32_t
                        && (*cwindow).wm_desktop != NET_WM_DESKTOP_ALL as uint32_t
                        && {
                            wm_desktop_ws = ewmh_get_workspace_by_index((*cwindow).wm_desktop);
                            !wm_desktop_ws.is_null()
                        }
                    {
                        debuglog(
                            c"%s:%s:%d - Using workspace %p / %s because _NET_WM_DESKTOP = %d.\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            325 as ::core::ffi::c_int,
                            wm_desktop_ws,
                            (*wm_desktop_ws).name,
                            (*cwindow).wm_desktop,
                        );
                        nc = con_descend_tiling_focused(wm_desktop_ws);
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                    } else if !startup_ws.is_null() {
                        debuglog(
                            c"%s:%s:%d - Using workspace on which this application was started (%s)\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            335 as ::core::ffi::c_int,
                            startup_ws,
                        );
                        nc = con_descend_tiling_focused(workspace_get(startup_ws));
                        debuglog(
                            c"%s:%s:%d - focused on ws %s: %p / %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            337 as ::core::ffi::c_int,
                            startup_ws,
                            nc,
                            (*nc).name,
                        );
                        if (*nc).type_0 as ::core::ffi::c_uint
                            == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con((*nc).parent as *mut Con, cwindow);
                        }
                    } else if (*focused).type_0 as ::core::ffi::c_uint
                        == CT_CON as ::core::ffi::c_int as ::core::ffi::c_uint
                        && con_accepts_window(focused) as ::core::ffi::c_int != 0
                    {
                        verboselog(
                            c"using current container, focused = %p, focused->name = %s\n"
                                .as_ptr()
                                .cast_mut(),
                            focused,
                            (*focused).name,
                        );
                        nc = focused;
                    } else {
                        nc = tree_open_con(::core::ptr::null_mut::<Con>(), cwindow);
                    }
                    assignment = assignment_for(cwindow, A_TO_OUTPUT as ::core::ffi::c_int);
                    if !assignment.is_null() {
                        con_move_to_output_name(nc, (*assignment).dest.output, true_0 != 0);
                    }
                } else {
                    if !match_0.is_null()
                        && (*match_0).insert_where as ::core::ffi::c_uint
                            == M_BELOW as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        nc = tree_open_con(nc, cwindow);
                    }
                    if !match_0.is_null()
                        && (*match_0).insert_where as ::core::ffi::c_uint
                            != M_BELOW as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        debuglog(
                            c"%s:%s:%d - Removing match %p from container %p\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            369 as ::core::ffi::c_int,
                            match_0,
                            nc,
                        );
                        if !(*match_0).matches.tqe_next.is_null() {
                            (*(*match_0).matches.tqe_next).matches.tqe_prev =
                                (*match_0).matches.tqe_prev;
                        } else {
                            (*nc).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                        }
                        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                        match_free(match_0);
                        free(match_0 as *mut ::core::ffi::c_void);
                        match_0 = ::core::ptr::null_mut::<Match>();
                    }
                    (*cwindow).swallowed = true_0 != 0;
                }
                debuglog(
                    c"%s:%s:%d - new container = %p\n".as_ptr().cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    378 as ::core::ffi::c_int,
                    nc,
                );
                if !(*nc).window.is_null() && (*nc).window != cwindow {
                    if !restore_kill_placeholder((*(*nc).window).id) {
                        debuglog(
                            c"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this to-be-managed window?!\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            381 as ::core::ffi::c_int,
                        );
                    } else {
                        _remove_matches(nc);
                    }
                }
                old_frame = XCB_NONE as xcb_window_t;
                if (*nc).window != cwindow && !(*nc).window.is_null() {
                    window_free((*nc).window as *mut i3Window);
                    old_frame = _match_depth(cwindow, nc);
                }
                (*nc).window = cwindow as *mut Window;
                x_reinit(nc);
                (*nc).border_width = (*geom).border_width as ::core::ffi::c_int;
                name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut name,
                    c"[i3 con] container around %p".as_ptr(),
                    cwindow,
                );
                x_set_name(nc, name);
                free(name as *mut ::core::ffi::c_void);
                ws = con_get_workspace(nc);
                fs = con_get_fullscreen_covering_ws(ws);
                if xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_FULLSCREEN) {
                    if fs != nc {
                        let output_0: *mut Output = get_output_with_dimensions(Rect {
                            x: (*geom).x as uint32_t,
                            y: (*geom).y as uint32_t,
                            width: (*geom).width as uint32_t,
                            height: (*geom).height as uint32_t,
                        });
                        if !output_0.is_null() {
                            con_move_to_output(nc, output_0, false_0 != 0);
                        }
                        con_toggle_fullscreen(nc, CF_OUTPUT as ::core::ffi::c_int);
                    }
                    fs = ::core::ptr::null_mut::<Con>();
                }
                set_focus = false_0 != 0;
                if fs.is_null() {
                    debuglog(
                        c"%s:%s:%d - Not in fullscreen mode, focusing\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        426 as ::core::ffi::c_int,
                    );
                    if (*cwindow).dock as u64 == 0 {
                        let current_output: *mut Con = con_get_output(focused);
                        let target_output: *mut Con = con_get_output(ws);
                        if workspace_is_visible(ws) as ::core::ffi::c_int != 0
                            && current_output == target_output
                        {
                            if !match_from_restart_mode {
                                set_focus = true_0 != 0;
                            } else {
                                debuglog(
                                    c"%s:%s:%d - not focusing, matched with restart_mode == true\n"
                                        .as_ptr()
                                        .cast_mut(),
                                    c"../src/manage.c".as_ptr(),
                                    c"manage_window".as_ptr(),
                                    438 as ::core::ffi::c_int,
                                );
                            }
                        } else {
                            debuglog(
                                c"%s:%s:%d - workspace not visible, not focusing\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                441 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        debuglog(
                            c"%s:%s:%d - dock, not focusing\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            444 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    debuglog(
                        c"%s:%s:%d - fs = %p, ws = %p, not focusing\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        447 as ::core::ffi::c_int,
                        fs,
                        ws,
                    );
                    let first: *mut Con = (*(*nc).parent).focus_head.tqh_first as *mut Con;
                    if first != nc {
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev = (*nc).focused.tqe_prev;
                        } else {
                            (*(*nc).parent).focus_head.tqh_last = (*nc).focused.tqe_prev;
                        }
                        *(*nc).focused.tqe_prev = (*nc).focused.tqe_next;
                        (*nc).focused.tqe_next = (*first).focused.tqe_next;
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev =
                                &raw mut (*nc).focused.tqe_next;
                        } else {
                            (*(*nc).parent).focus_head.tqh_last = &raw mut (*nc).focused.tqe_next;
                        }
                        (*first).focused.tqe_next = nc as *mut Con;
                        (*nc).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
                    }
                }
                want_floating = false_0 != 0;
                if xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_DIALOG)
                    as ::core::ffi::c_int
                    != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_UTILITY)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_TOOLBAR)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_NOTIFICATION)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply, A__NET_WM_WINDOW_TYPE_SPLASH)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_MODAL)
                        as ::core::ffi::c_int
                        != 0
                    || (*cwindow).max_width > 0 as ::core::ffi::c_int
                        && (*cwindow).max_height > 0 as ::core::ffi::c_int
                        && (*cwindow).min_height == (*cwindow).max_height
                        && (*cwindow).min_width == (*cwindow).max_width
                {
                    verboselog(
                        c"This window is a dialog window, setting floating\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                    want_floating = true_0 != 0;
                }
                if xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_STICKY) {
                    (*nc).sticky = true_0 != 0;
                }
                if (*cwindow).wm_desktop == NET_WM_DESKTOP_ALL as uint32_t
                    && (ws.is_null() || !con_is_internal(ws))
                {
                    debuglog(
                        c"%s:%s:%d - This window has _NET_WM_DESKTOP = 0xFFFFFFFF. Will float it and make it sticky.\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        484 as ::core::ffi::c_int,
                    );
                    (*nc).sticky = true_0 != 0;
                    want_floating = true_0 != 0;
                }
                free(state_reply as *mut ::core::ffi::c_void);
                state_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                free(type_reply as *mut ::core::ffi::c_void);
                type_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                if (*cwindow).transient_for as ::core::ffi::c_long != XCB_NONE
                    || (*cwindow).leader as ::core::ffi::c_long != XCB_NONE
                        && (*cwindow).leader != (*cwindow).id
                        && !con_by_window_id((*cwindow).leader).is_null()
                {
                    debuglog(
                        c"%s:%s:%d - This window is transient for another window, setting floating\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        496 as ::core::ffi::c_int,
                    );
                    want_floating = true_0 != 0;
                    if config.popup_during_fullscreen as ::core::ffi::c_uint
                        == PDF_LEAVE_FULLSCREEN as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !fs.is_null()
                    {
                        debuglog(
                            c"%s:%s:%d - There is a fullscreen window, leaving fullscreen mode\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            501 as ::core::ffi::c_int,
                        );
                        con_disable_fullscreen(fs);
                    } else if config.popup_during_fullscreen as ::core::ffi::c_uint
                        == PDF_SMART as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !fs.is_null()
                        && !(*fs).window.is_null()
                    {
                        set_focus = con_find_transient_for_window(nc, (*(*fs).window).id);
                    }
                }
                if (*cwindow).dock as u64 != 0 {
                    want_floating = false_0 != 0;
                }
                if (*nc).geometry.width == 0 as uint32_t {
                    (*nc).geometry = Rect {
                        x: (*geom).x as uint32_t,
                        y: (*geom).y as uint32_t,
                        width: (*geom).width as uint32_t,
                        height: (*geom).height as uint32_t,
                    } as Rect;
                }
                if config.popup_during_fullscreen as ::core::ffi::c_uint
                    == PDF_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    && want_floating as ::core::ffi::c_int != 0
                    && !fs.is_null()
                {
                    set_focus = true_0 != 0;
                }
                if want_floating {
                    debuglog(
                        c"%s:%s:%d - geometry = %d x %d\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        529 as ::core::ffi::c_int,
                        (*nc).geometry.width,
                        (*nc).geometry.height,
                    );
                    if floating_enable(nc, true_0 != 0) {
                        (*nc).floating = FLOATING_AUTO_ON;
                    }
                }
                if has_mwm_hints {
                    debuglog(
                        c"%s:%s:%d - MOTIF_WM_HINTS specifies decorations (border_style = %d)\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        536 as ::core::ffi::c_int,
                        motif_border_style as ::core::ffi::c_uint,
                    );
                    if want_floating {
                        con_set_border_style(
                            nc,
                            motif_border_style,
                            config.default_floating_border_width,
                        );
                    } else {
                        con_set_border_style(nc, motif_border_style, config.default_border_width);
                    }
                }
                if (*nc).current_border_width == -(1 as ::core::ffi::c_int) {
                    (*nc).current_border_width = if want_floating as ::core::ffi::c_int != 0 {
                        config.default_floating_border_width
                    } else {
                        config.default_border_width
                    };
                }
                values[0 as ::core::ffi::c_int as usize] = XCB_NONE as uint32_t;
                xcb_change_window_attributes(
                    conn,
                    window,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                    &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
                );
                rcookie = xcb_reparent_window_checked(
                    conn,
                    window,
                    (*nc).frame.id as xcb_window_t,
                    0 as int16_t,
                    0 as int16_t,
                );
                if !xcb_request_check(conn, rcookie).is_null() {
                    verboselog(
                        c"Could not reparent the window, aborting\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    values[0 as ::core::ffi::c_int as usize] = ((XCB_EVENT_MASK_PROPERTY_CHANGE
                        as ::core::ffi::c_int
                        | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int
                        | XCB_EVENT_MASK_FOCUS_CHANGE as ::core::ffi::c_int)
                        & !(XCB_EVENT_MASK_ENTER_WINDOW as ::core::ffi::c_int))
                        as uint32_t;
                    xcb_change_window_attributes(
                        conn,
                        window,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as uint32_t,
                        &raw mut values as *mut uint32_t as *const ::core::ffi::c_void,
                    );
                    xcb_flush(conn);
                    xcb_change_save_set(
                        conn,
                        XCB_SET_MODE_INSERT as ::core::ffi::c_int as uint8_t,
                        window,
                    );
                    if shape_supported {
                        xcb_shape_select_input(conn, window, true_0 as uint8_t);
                        let cookie_0: xcb_shape_query_extents_cookie_t =
                            xcb_shape_query_extents(conn, window);
                        let mut reply: *mut xcb_shape_query_extents_reply_t =
                            xcb_shape_query_extents_reply(
                                conn,
                                cookie_0,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        if !reply.is_null() && (*reply).bounding_shaped as ::core::ffi::c_int != 0 {
                            (*cwindow).shaped = true_0 != 0;
                        }
                        free(reply as *mut ::core::ffi::c_void);
                        reply = ::core::ptr::null_mut::<xcb_shape_query_extents_reply_t>();
                    }
                    run_assignments(cwindow);
                    ws = con_get_workspace(nc);
                    if !ws.is_null() && !workspace_is_visible(ws) {
                        (*ws).rect = (*(*ws).parent).rect;
                        render_con(ws);
                        set_focus = false_0 != 0;
                    }
                    render_con(croot);
                    (*cwindow).managed_since = time(::core::ptr::null_mut::<time_t>());
                    ipc_send_window_event(c"new".as_ptr(), nc);
                    if set_focus as ::core::ffi::c_int != 0
                        && !assignment_for(cwindow, A_NO_FOCUS as ::core::ffi::c_int).is_null()
                    {
                        if con_num_windows(ws) == 1 as ::core::ffi::c_int {
                            debuglog(
                                c"%s:%s:%d - This is the first window on this workspace, ignoring no_focus.\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                626 as ::core::ffi::c_int,
                            );
                        } else {
                            debuglog(
                                c"%s:%s:%d - no_focus was set for con = %p, not setting focus.\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                628 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false_0 != 0;
                        }
                    }
                    if set_focus {
                        debuglog(
                            c"%s:%s:%d - Checking con = %p for _NET_WM_USER_TIME.\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            634 as ::core::ffi::c_int,
                            nc,
                        );
                        let mut wm_user_time: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
                        let mut wm_user_time_reply: *mut xcb_get_property_reply_t =
                            xcb_get_property_reply(
                                conn,
                                wm_user_time_cookie,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        if !wm_user_time_reply.is_null()
                            && xcb_get_property_value_length(wm_user_time_reply)
                                != 0 as ::core::ffi::c_int
                            && {
                                wm_user_time =
                                    xcb_get_property_value(wm_user_time_reply) as *mut uint32_t;
                                !wm_user_time.is_null()
                            }
                            && *wm_user_time.offset(0 as ::core::ffi::c_int as isize)
                                == 0 as uint32_t
                        {
                            debuglog(
                                c"%s:%s:%d - _NET_WM_USER_TIME set to 0, not focusing con = %p.\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                641 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false_0 != 0;
                        }
                        free(wm_user_time_reply as *mut ::core::ffi::c_void);
                        wm_user_time_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
                    } else {
                        xcb_discard_reply(conn, wm_user_time_cookie.sequence);
                    }
                    if (*(*nc).window).doesnt_accept_focus as ::core::ffi::c_int != 0
                        && !(*(*nc).window).needs_take_focus
                    {
                        set_focus = false_0 != 0;
                    }
                    if set_focus as ::core::ffi::c_int != 0
                        && (*nc).mapped as ::core::ffi::c_int != 0
                    {
                        debuglog(
                            c"%s:%s:%d - Now setting focus.\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            662 as ::core::ffi::c_int,
                        );
                        con_activate(nc);
                    }
                    tree_render();
                    if old_frame as ::core::ffi::c_long != XCB_NONE {
                        xcb_destroy_window(conn, old_frame);
                    }
                    con_set_urgency(nc, urgency_hint);
                    (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as uint32_t;
                    ewmh_update_wm_desktop();
                    output_push_sticky_windows(focused);
                }
                free(geom as *mut ::core::ffi::c_void);
            }
        }
    }
    free(attr as *mut ::core::ffi::c_void);
}
unsafe fn placeholder_for_con(con: *mut Con) -> *mut Con {
    if (*(*con).window).swallowed {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let nc: *mut Con = con_for_window(croot, (*con).window as *mut i3Window, &raw mut match_0);
    if nc.is_null() || (*nc).window.is_null() || (*nc).window == (*con).window {
        return ::core::ptr::null_mut::<Con>();
    }
    if (*(*nc).window).managed_since > (*(*con).window).managed_since {
        return ::core::ptr::null_mut::<Con>();
    }
    nc
}

pub unsafe fn remanage_window(con: *mut Con) -> *mut Con {
    let nc: *mut Con = placeholder_for_con(con);
    if nc.is_null() {
        run_assignments((*con).window as *mut i3Window);
        return con;
    }
    if !restore_kill_placeholder((*(*nc).window).id) {
        debuglog(
            c"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this managed window?!\n".as_ptr().cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"remanage_window".as_ptr(),
            726 as ::core::ffi::c_int,
        );
    } else {
        _remove_matches(nc);
    }
    window_free((*nc).window as *mut i3Window);
    let old_frame: xcb_window_t = _match_depth((*con).window as *mut i3Window, nc);
    x_reparent_child(nc, con);
    let moved_workpaces: bool = con_get_workspace(nc) != con_get_workspace(con);
    con_merge_into(con, nc);
    if old_frame as ::core::ffi::c_long != XCB_NONE {
        xcb_destroy_window(conn, old_frame);
    }
    run_assignments((*nc).window as *mut i3Window);
    if moved_workpaces {
        startup_sequence_delete_by_window((*nc).window);
        ewmh_update_wm_desktop();
    }
    (*(*nc).window).swallowed = true_0 != 0;
    nc
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::bindings_get_buttons_to_grab;
use crate::ewmh::ewmh_update_wm_desktop;
use crate::restore_layout::restore_kill_placeholder;
use crate::tree::tree_render;
use crate::util::ws_name_to_number;
use crate::x::window_supports_protocol;
use i3_common::draw_util::get_visual_depth;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_sync, xcb_change_save_set, xcb_change_window_attributes,
    xcb_change_window_attributes_checked, xcb_destroy_window, xcb_discard_reply, xcb_drawable_t,
    xcb_flush, xcb_generic_error_t, xcb_get_geometry, xcb_get_geometry_cookie_t,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_property, xcb_get_property_cookie_t,
    xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_value,
    xcb_get_property_value_length, xcb_get_window_attributes, xcb_get_window_attributes_cookie_t,
    xcb_get_window_attributes_reply, xcb_get_window_attributes_reply_t, xcb_icccm_get_wm_hints,
    xcb_icccm_get_wm_normal_hints, xcb_query_tree, xcb_query_tree_children,
    xcb_query_tree_children_length, xcb_query_tree_reply, xcb_query_tree_reply_t,
    xcb_reparent_window, xcb_reparent_window_checked, xcb_request_check, xcb_shape_query_extents,
    xcb_shape_query_extents_cookie_t, xcb_shape_query_extents_reply,
    xcb_shape_query_extents_reply_t, xcb_shape_select_input, xcb_void_cookie_t, xcb_window_t,
    XCB_ATOM_WM_CLASS, XCB_ATOM_WM_CLIENT_MACHINE, XCB_ATOM_WM_NAME, XCB_ATOM_WM_TRANSIENT_FOR,
    XCB_CW_EVENT_MASK, XCB_EVENT_MASK_ENTER_WINDOW, XCB_EVENT_MASK_FOCUS_CHANGE,
    XCB_EVENT_MASK_PROPERTY_CHANGE, XCB_EVENT_MASK_STRUCTURE_NOTIFY,
    XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT, XCB_GET_PROPERTY_TYPE_ANY, XCB_MAP_STATE_VIEWABLE,
    XCB_NONE, XCB_SET_MODE_INSERT,
};

pub use libc::{free, time};

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::shape_supported;
use crate::application::A_WM_CLIENT_LEADER;
use crate::application::A_WM_TAKE_FOCUS;
use crate::application::A_WM_WINDOW_ROLE;
use crate::application::A__MOTIF_WM_HINTS;
use crate::application::A__NET_STARTUP_ID;
use crate::application::A__NET_WM_DESKTOP;
use crate::application::A__NET_WM_ICON;
use crate::application::A__NET_WM_NAME;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::application::A__NET_WM_STATE_MODAL;
use crate::application::A__NET_WM_STATE_STICKY;
use crate::application::A__NET_WM_STRUT_PARTIAL;
use crate::application::A__NET_WM_USER_TIME;
use crate::application::A__NET_WM_WINDOW_TYPE;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DOCK;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::assignments::assignment_for;
use crate::assignments::run_assignments;
use crate::con::con_accepts_window;
use crate::con::con_activate;
use crate::con::con_by_window_id;
use crate::con::con_descend_tiling_focused;
use crate::con::con_disable_fullscreen;
use crate::con::con_find_transient_for_window;
use crate::con::con_for_window;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_is_internal;
use crate::con::con_merge_into;
use crate::con::con_move_to_output;
use crate::con::con_move_to_output_name;
use crate::con::con_num_windows;
use crate::con::con_set_border_style;
use crate::con::con_set_urgency;
use crate::con::con_toggle_fullscreen;
use crate::config::config;
use crate::ewmh::ewmh_get_workspace_by_index;
use crate::floating::floating_enable;
use crate::ipc::ipc_send_window_event;
use crate::output::output_primary_name;
use crate::output::output_push_sticky_windows;
use crate::r#match::match_free;
use crate::randr::get_output_containing;
use crate::randr::get_output_with_dimensions;
use crate::render::render_con;
use crate::startup::startup_sequence_delete_by_window;
use crate::startup::startup_workspace_for_window;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_open_con;
use crate::window::window_free;
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
use crate::workspace::get_existing_workspace_by_num;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::x::x_con_reframe;
use crate::x::x_reinit;
use crate::x::x_reparent_child;
use crate::x::x_set_name;
use crate::xcb::xcb_get_preferred_window_type;
use crate::xcb::xcb_grab_buttons;
use crate::xcb::xcb_reply_contains_atom;
use crate::xcb::xcb_set_window_rect;
