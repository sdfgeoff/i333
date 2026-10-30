use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, resolve};
use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::PopupDuringFullscreen;
use crate::types::WindowDock;
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

pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_OUTPUT: fullscreen_mode_t = 1;

pub const M_BELOW: ::core::ffi::c_uint = 2;

pub const A_TO_OUTPUT: ::core::ffi::c_uint = 16;
pub const A_TO_WORKSPACE_NUMBER: ::core::ffi::c_uint = 8;
pub const A_NO_FOCUS: ::core::ffi::c_uint = 4;
pub const A_TO_WORKSPACE: ::core::ffi::c_uint = 2;

pub const CT_WORKSPACE: ::core::ffi::c_uint = 4;
pub const CT_CON: ::core::ffi::c_uint = 2;

pub type Output = xoutput;
pub type i3Window = Window;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

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

/// Uniquely owned native reply/error allocation; no graph ownership is implied.
struct NativeReply<T>(*mut T);
impl<T> Drop for NativeReply<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.cast()) };
    }
}

fn tree_child_count(children: u16, wire_words: u32) -> Option<usize> {
    let count = usize::from(children);
    (count <= wire_words as usize).then_some(count)
}

/// Root and connection are valid native XCB objects on the event thread.
pub unsafe fn manage_existing_windows(root_0: xcb_window_t) {
    let reply = NativeReply(xcb_query_tree_reply(
        conn,
        xcb_query_tree(conn, root_0),
        std::ptr::null_mut(),
    ));
    if reply.0.is_null() {
        return;
    }
    let Some(count) = tree_child_count((*reply.0).children_len, (*reply.0).length) else {
        return;
    };
    let children = std::slice::from_raw_parts(xcb_query_tree_children(reply.0), count).to_vec();
    // Preserve request pipelining: issue every attribute query before managing
    // any child (which can invoke graph callbacks/native round trips).
    let cookies: Vec<_> = children
        .iter()
        .map(|&child| xcb_get_window_attributes(conn, child))
        .collect();
    drop(reply);
    for (child, cookie) in children.into_iter().zip(cookies) {
        manage_window(child, cookie, true);
    }
}

/// Native complete reply allocation, borrowed only for this copied first word.
/// The original accepted any format/type with at least one byte; reject only
/// undersized payloads that previously triggered an out-of-bounds u32 read.
unsafe fn first_property_word(reply: *const xcb_get_property_reply_t) -> Option<u32> {
    let header = reply.as_ref()?;
    let bytes = usize::try_from(header.value_len)
        .ok()?
        .checked_mul(usize::from(header.format / 8))?;
    if bytes < 4 || header.length == 0 {
        return None;
    }
    let pointer = xcb_get_property_value(reply.cast_mut()).cast::<u8>();
    if pointer.is_null() {
        return None;
    }
    Some(u32::from_ne_bytes(
        std::slice::from_raw_parts(pointer, 4).try_into().unwrap(),
    ))
}

unsafe fn request_failed(cookie: xcb_void_cookie_t) -> bool {
    let error = NativeReply(xcb_request_check(conn, cookie));
    !error.0.is_null()
}

pub unsafe fn restore_geometry() {
    debuglog(
        c"%s:%s:%d - Restoring geometry\n".as_ptr().cast_mut(),
        c"../src/manage.c".as_ptr(),
        c"restore_geometry".as_ptr(),
        79 as ::core::ffi::c_int,
    );
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
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
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as u32);
            (*con).window_rect.height = (*con)
                .window_rect
                .height
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as u32);
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
                (*con).rect.x as i16,
                (*con).rect.y as i16,
            );
        }
    }
    let mut fresh0: [u32; 1] = [XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int as u32];
    xcb_change_window_attributes(
        conn,
        root,
        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
        &raw mut fresh0 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_aux_sync(conn);
}

pub unsafe fn manage_window(
    window: xcb_window_t,
    cookie: xcb_get_window_attributes_cookie_t,
    needs_to_be_mapped: bool,
) {
    let mut values: [u32; 1] = [0; 1];
    let mut event_mask_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut cwindow: *mut i3Window = ::core::ptr::null_mut::<i3Window>();
    let mut buttons: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut urgency_hint: bool = false;
    let mut motif_border_style: BorderStyle = BorderStyle::None;
    let mut has_mwm_hints: bool = false;
    let mut type_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut state_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_id_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut startup_ws: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let _attribute_owner = NativeReply(attr);
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
        let _geometry_owner = NativeReply(geom);
        if geom.is_null() {
            debuglog(
                c"%s:%s:%d - could not get geometry\n".as_ptr().cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"manage_window".as_ptr(),
                154 as ::core::ffi::c_int,
            );
        } else {
            values = [0; 1];
            values[0 as ::core::ffi::c_int as usize] =
                (XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int) as u32;
            event_mask_cookie = xcb_change_window_attributes_checked(
                conn,
                window,
                XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            if request_failed(event_mask_cookie) {
                verboselog(
                    c"Could not change event mask, the window probably already disappeared.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                wm_type_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_WINDOW_TYPE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                strut_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_STRUT_PARTIAL,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                state_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_STATE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                utf8_title_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_NAME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    128 as u32,
                );
                leader_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A_WM_CLIENT_LEADER,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                transient_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    XCB_ATOM_WM_TRANSIENT_FOR as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                title_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    128 as u32,
                );
                class_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    128 as u32,
                );
                role_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A_WM_WINDOW_ROLE,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    128 as u32,
                );
                startup_id_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_STARTUP_ID,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    512 as u32,
                );
                wm_hints_cookie = xcb_icccm_get_wm_hints(conn, window);
                wm_normal_hints_cookie = xcb_icccm_get_wm_normal_hints(conn, window);
                motif_wm_hints_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__MOTIF_WM_HINTS,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    5_usize.wrapping_mul(::core::mem::size_of::<u64>() as usize) as u32,
                );
                wm_user_time_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_USER_TIME,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                wm_desktop_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_DESKTOP,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                wm_machine_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    XCB_ATOM_WM_CLIENT_MACHINE as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                wm_icon_cookie = xcb_get_property(
                    conn,
                    false_0 as u8,
                    window,
                    A__NET_WM_ICON,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    4294967295 as u32,
                );
                cwindow = Box::into_raw(Box::new(i3Window::default()));
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
                motif_border_style = BorderStyle::None;
                has_mwm_hints = window_update_motif_hints(
                    cwindow,
                    xcb_get_property_reply(
                        conn,
                        motif_wm_hints_cookie,
                        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                    ),
                    &raw mut motif_border_style,
                );
                let normal_hints_reply = xcb_get_property_reply(
                    conn,
                    wm_normal_hints_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                // window_update_normal_hints borrows its reply; this request owns it.
                window_update_normal_hints(cwindow, normal_hints_reply, geom);
                free(normal_hints_reply.cast());
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
                let wm_desktop_reply = xcb_get_property_reply(
                    conn,
                    wm_desktop_cookie,
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                let _desktop_owner = NativeReply(wm_desktop_reply);
                (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as u32;
                if let Some(desktop) = first_property_word(wm_desktop_reply) {
                    (*cwindow).wm_desktop = desktop;
                }
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
                    if (*cwindow).reserved.top > 0 as u32 && (*cwindow).reserved.bottom == 0 as u32
                    {
                        debuglog(
                            c"%s:%s:%d - Top dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            258 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = WindowDock::Top;
                    } else if (*cwindow).reserved.top == 0 as u32
                        && (*cwindow).reserved.bottom > 0 as u32
                    {
                        debuglog(
                            c"%s:%s:%d - Bottom dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            261 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = WindowDock::Bottom;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            264 as ::core::ffi::c_int,
                        );
                        if ((*geom).y as ::core::ffi::c_int)
                            < (*search_at).rect.height.wrapping_div(2 as u32) as i16
                                as ::core::ffi::c_int
                        {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                267 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as u32),
                            );
                            (*cwindow).dock = WindowDock::Top;
                        } else {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                271 as ::core::ffi::c_int,
                                (*geom).y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as u32),
                            );
                            (*cwindow).dock = WindowDock::Bottom;
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
                (*cwindow).swallowed = false;
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
                            nc = tree_open_con(parent_ptr(nc), cwindow);
                        }
                        if !workspace_is_visible(assigned_ws) {
                            urgency_hint = true;
                        }
                    } else if (*cwindow).wm_desktop != NET_WM_DESKTOP_NONE as u32
                        && (*cwindow).wm_desktop != NET_WM_DESKTOP_ALL as u32
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
                            nc = tree_open_con(parent_ptr(nc), cwindow);
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
                            nc = tree_open_con(parent_ptr(nc), cwindow);
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
                        con_move_to_output_name(nc, (*assignment).dest.output, true);
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
                    (*cwindow).swallowed = true;
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
                            x: (*geom).x as u32,
                            y: (*geom).y as u32,
                            width: (*geom).width as u32,
                            height: (*geom).height as u32,
                        });
                        if !output_0.is_null() {
                            con_move_to_output(nc, output_0, false);
                        }
                        con_toggle_fullscreen(nc, CF_OUTPUT as ::core::ffi::c_int);
                    }
                    fs = ::core::ptr::null_mut::<Con>();
                }
                set_focus = false;
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
                                set_focus = true;
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
                    let first: *mut Con = (*parent_ptr(nc)).focus_head.tqh_first as *mut Con;
                    if first != nc {
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev = (*nc).focused.tqe_prev;
                        } else {
                            (*parent_ptr(nc)).focus_head.tqh_last = (*nc).focused.tqe_prev;
                        }
                        *(*nc).focused.tqe_prev = (*nc).focused.tqe_next;
                        (*nc).focused.tqe_next = (*first).focused.tqe_next;
                        if !(*nc).focused.tqe_next.is_null() {
                            (*(*nc).focused.tqe_next).focused.tqe_prev =
                                &raw mut (*nc).focused.tqe_next;
                        } else {
                            (*parent_ptr(nc)).focus_head.tqh_last = &raw mut (*nc).focused.tqe_next;
                        }
                        (*first).focused.tqe_next = nc as *mut Con;
                        (*nc).focused.tqe_prev = &raw mut (*first).focused.tqe_next;
                    }
                }
                want_floating = false;
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
                    want_floating = true;
                }
                if xcb_reply_contains_atom(state_reply, A__NET_WM_STATE_STICKY) {
                    (*nc).sticky = true;
                }
                if (*cwindow).wm_desktop == NET_WM_DESKTOP_ALL as u32
                    && (ws.is_null() || !con_is_internal(ws))
                {
                    debuglog(
                        c"%s:%s:%d - This window has _NET_WM_DESKTOP = 0xFFFFFFFF. Will float it and make it sticky.\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        484 as ::core::ffi::c_int,
                    );
                    (*nc).sticky = true;
                    want_floating = true;
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
                    want_floating = true;
                    if config.popup_during_fullscreen == PopupDuringFullscreen::LeaveFullscreen
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
                    } else if config.popup_during_fullscreen == PopupDuringFullscreen::Smart
                        && !fs.is_null()
                        && !(*fs).window.is_null()
                    {
                        set_focus = con_find_transient_for_window(nc, (*(*fs).window).id);
                    }
                }
                if (*cwindow).dock as u64 != 0 {
                    want_floating = false;
                }
                if (*nc).geometry.width == 0 as u32 {
                    (*nc).geometry = Rect {
                        x: (*geom).x as u32,
                        y: (*geom).y as u32,
                        width: (*geom).width as u32,
                        height: (*geom).height as u32,
                    } as Rect;
                }
                if config.popup_during_fullscreen == PopupDuringFullscreen::All
                    && want_floating as ::core::ffi::c_int != 0
                    && !fs.is_null()
                {
                    set_focus = true;
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
                    if floating_enable(nc, true) {
                        (*nc).floating = FloatingState::AutoOn;
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
                values[0 as ::core::ffi::c_int as usize] = XCB_NONE as u32;
                xcb_change_window_attributes(
                    conn,
                    window,
                    XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                    &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                );
                rcookie = xcb_reparent_window_checked(
                    conn,
                    window,
                    (*nc).frame.id as xcb_window_t,
                    0 as i16,
                    0 as i16,
                );
                if request_failed(rcookie) {
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
                        as u32;
                    xcb_change_window_attributes(
                        conn,
                        window,
                        XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32,
                        &raw mut values as *mut u32 as *const ::core::ffi::c_void,
                    );
                    xcb_flush(conn);
                    xcb_change_save_set(
                        conn,
                        XCB_SET_MODE_INSERT as ::core::ffi::c_int as u8,
                        window,
                    );
                    if shape_supported {
                        xcb_shape_select_input(conn, window, true_0 as u8);
                        let cookie_0: xcb_shape_query_extents_cookie_t =
                            xcb_shape_query_extents(conn, window);
                        let mut reply: *mut xcb_shape_query_extents_reply_t =
                            xcb_shape_query_extents_reply(
                                conn,
                                cookie_0,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        if !reply.is_null() && (*reply).bounding_shaped as ::core::ffi::c_int != 0 {
                            (*cwindow).shaped = true;
                        }
                        free(reply as *mut ::core::ffi::c_void);
                        reply = ::core::ptr::null_mut::<xcb_shape_query_extents_reply_t>();
                    }
                    run_assignments(cwindow);
                    ws = con_get_workspace(nc);
                    if !ws.is_null() && !workspace_is_visible(ws) {
                        (*ws).rect = (*parent_ptr(ws)).rect;
                        render_con(ws);
                        set_focus = false;
                    }
                    render_con(croot);
                    (*cwindow).managed_since = time(::core::ptr::null_mut::<libc::time_t>());
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
                            set_focus = false;
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
                        let wm_user_time_reply: *mut xcb_get_property_reply_t =
                            xcb_get_property_reply(
                                conn,
                                wm_user_time_cookie,
                                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                            );
                        let _time_owner = NativeReply(wm_user_time_reply);
                        if first_property_word(wm_user_time_reply) == Some(0) {
                            debuglog(
                                c"%s:%s:%d - _NET_WM_USER_TIME set to 0, not focusing con = %p.\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                641 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false;
                        }
                    } else {
                        xcb_discard_reply(conn, wm_user_time_cookie.sequence);
                    }
                    if (*(*nc).window).doesnt_accept_focus as ::core::ffi::c_int != 0
                        && !(*(*nc).window).needs_take_focus
                    {
                        set_focus = false;
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
                    (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as u32;
                    ewmh_update_wm_desktop();
                    output_push_sticky_windows(focused);
                }
            }
        }
    }
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
    (*(*nc).window).swallowed = true;
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

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_sync, xcb_change_save_set, xcb_change_window_attributes,
    xcb_change_window_attributes_checked, xcb_destroy_window, xcb_discard_reply, xcb_drawable_t,
    xcb_flush, xcb_generic_error_t, xcb_get_geometry, xcb_get_geometry_cookie_t,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_property, xcb_get_property_cookie_t,
    xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_value,
    xcb_get_window_attributes, xcb_get_window_attributes_cookie_t, xcb_get_window_attributes_reply,
    xcb_get_window_attributes_reply_t, xcb_icccm_get_wm_hints, xcb_icccm_get_wm_normal_hints,
    xcb_query_tree, xcb_query_tree_children, xcb_query_tree_reply, xcb_reparent_window,
    xcb_reparent_window_checked, xcb_request_check, xcb_shape_query_extents,
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

#[cfg(test)]
mod owned_reply_tests {
    use super::*;
    #[test]
    fn tree_child_count_is_limited_by_native_wire_allocation() {
        assert_eq!(tree_child_count(0, 0), Some(0));
        assert_eq!(tree_child_count(2, 2), Some(2));
        assert_eq!(tree_child_count(2, 1), None);
        assert_eq!(
            tree_child_count(u16::MAX, u32::MAX),
            Some(u16::MAX as usize)
        );
    }
    #[test]
    fn first_property_word_checks_complete_bytes_and_retains_native_formats() {
        // u32 storage gives the native header its required alignment; payload
        // bytes are read by copy, without assuming a u32-aligned value view.
        let mut storage = vec![0u32; 9];
        let reply = storage.as_mut_ptr().cast::<xcb_get_property_reply_t>();
        unsafe {
            (*reply).length = 1;
            (*reply).format = 8;
            (*reply).value_len = 3;
            std::ptr::copy_nonoverlapping(
                [0x12u8, 0x34, 0x56, 0x78].as_ptr(),
                reply.add(1).cast(),
                4,
            );
            assert_eq!(first_property_word(reply), None);
            (*reply).value_len = 4;
            assert_eq!(
                first_property_word(reply),
                Some(u32::from_ne_bytes([0x12, 0x34, 0x56, 0x78]))
            );
            (*reply).format = 32;
            (*reply).value_len = 1;
            assert!(first_property_word(reply).is_some());
            (*reply).length = 0;
            assert_eq!(first_property_word(reply), None);
            assert_eq!(first_property_word(std::ptr::null()), None);
        }
    }
}
