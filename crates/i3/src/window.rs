use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * window.c: Updates window attributes (X11 hints/properties).
 *
 */
// Mechanical file-by-file Rust translation of src/window.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
extern "C" {

    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

}
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

pub type C2RustUnnamed = ::core::ffi::c_uint;

pub type C2RustUnnamed_0 = ::core::ffi::c_uint;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_1 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_1 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_1 = 0;

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_6 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_6 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_6 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_6 = 0;
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

pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_8 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_8 = 1;
pub const M_HERE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_9 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_9 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_9 = 4;
pub const WM_TILING: C2RustUnnamed_9 = 3;
pub const WM_TILING_USER: C2RustUnnamed_9 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_9 = 1;
pub const WM_ANY: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_10 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_10 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_10 = 1;
pub const M_NODOCK: C2RustUnnamed_10 = 0;
pub const M_DONTCHECK: C2RustUnnamed_10 = -1;
pub type C2RustUnnamed_11 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_11 = 1;
pub const U_LATEST: C2RustUnnamed_11 = 0;
pub const U_DONTCHECK: C2RustUnnamed_11 = -1;

pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_12 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_12 = 1;
pub const W_NODOCK: C2RustUnnamed_12 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_15 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_15 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_15 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_15 = 2;
pub const A_COMMAND: C2RustUnnamed_15 = 1;
pub const A_ANY: C2RustUnnamed_15 = 0;

pub type C2RustUnnamed_17 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_17 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_17 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_17 = 3;
pub const CT_CON: C2RustUnnamed_17 = 2;
pub const CT_OUTPUT: C2RustUnnamed_17 = 1;
pub const CT_ROOT: C2RustUnnamed_17 = 0;
pub type i3Window = Window;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 90] = unsafe {
    ::core::mem::transmute::<
        [u8; 90],
        [::core::ffi::c_char; 90],
    >(
        *b"_Bool window_update_motif_hints(i3Window *, xcb_get_property_reply_t *, border_style_t *)\0",
    )
};

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DBL_EPSILON: ::core::ffi::c_double = __DBL_EPSILON__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn window_free(mut win: *mut i3Window) {
    free((*win).class_class as *mut ::core::ffi::c_void);
    (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).class_instance as *mut ::core::ffi::c_void);
    (*win).class_instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).role as *mut ::core::ffi::c_void);
    (*win).role = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).machine as *mut ::core::ffi::c_void);
    (*win).machine = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i3string_free((*win).name);
    cairo_surface_destroy((*win).icon);
    free((*win).ran_assignments as *mut ::core::ffi::c_void);
    (*win).ran_assignments = ::core::ptr::null_mut::<*mut Assignment>();
    free(win as *mut ::core::ffi::c_void);
    win = ::core::ptr::null_mut::<i3Window>();
}

pub unsafe fn window_update_class(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - WM_CLASS not set.\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_class".as_ptr(),
            36 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let prop_length: size_t = xcb_get_property_value_length(prop) as size_t;
    let new_class: *mut ::core::ffi::c_char =
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char;
    let class_class_index: size_t =
        (strnlen(new_class, prop_length) as size_t).wrapping_add(1 as size_t);
    free((*win).class_instance as *mut ::core::ffi::c_void);
    (*win).class_instance = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*win).class_class as *mut ::core::ffi::c_void);
    (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).class_instance = sstrndup(new_class, prop_length);
    if class_class_index < prop_length {
        (*win).class_class = sstrndup(
            new_class.offset(class_class_index as isize),
            prop_length.wrapping_sub(class_class_index),
        );
    } else {
        (*win).class_class = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    verboselog(
        c"WM_CLASS changed to %s (instance), %s (class)\n"
            .as_ptr()
            .cast_mut(),
        (*win).class_instance,
        (*win).class_class,
    );
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_name(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - _NET_WM_NAME not specified, not changing\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_name".as_ptr(),
            70 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    i3string_free((*win).name);
    let len: ::core::ffi::c_int = xcb_get_property_value_length(prop) as ::core::ffi::c_int;
    let name: *mut ::core::ffi::c_char = sstrndup(
        xcb_get_property_value(prop) as *const ::core::ffi::c_char,
        len as size_t,
    );
    (*win).name = i3string_from_utf8(name);
    free(name as *mut ::core::ffi::c_void);
    let con: *mut Con = con_by_window_id((*win).id);
    if !con.is_null() && !(*con).title_format.is_null() {
        let mut name_0: *mut i3String = con_parse_title_format(con);
        ewmh_update_visible_name((*win).id, i3string_as_utf8(name_0));
        i3string_free(name_0);
        name_0 = ::core::ptr::null_mut::<i3String>();
    }
    (*win).name_x_changed = true_0 != 0;
    verboselog(
        c"_NET_WM_NAME changed to \"%s\"\n".as_ptr().cast_mut(),
        i3string_as_utf8((*win).name),
    );
    (*win).uses_net_wm_name = true_0 != 0;
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_name_legacy(
    win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - WM_NAME not set (_NET_WM_NAME is what you want anyways).\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_name_legacy".as_ptr(),
            106 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    if (*win).uses_net_wm_name {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    i3string_free((*win).name);
    let len: ::core::ffi::c_int = xcb_get_property_value_length(prop) as ::core::ffi::c_int;
    let name: *mut ::core::ffi::c_char = sstrndup(
        xcb_get_property_value(prop) as *const ::core::ffi::c_char,
        len as size_t,
    );
    (*win).name = i3string_from_utf8(name);
    free(name as *mut ::core::ffi::c_void);
    let con: *mut Con = con_by_window_id((*win).id);
    if !con.is_null() && !(*con).title_format.is_null() {
        let mut name_0: *mut i3String = con_parse_title_format(con);
        ewmh_update_visible_name((*win).id, i3string_as_utf8(name_0));
        i3string_free(name_0);
        name_0 = ::core::ptr::null_mut::<i3String>();
    }
    verboselog(
        c"WM_NAME changed to \"%s\"\n".as_ptr().cast_mut(),
        i3string_as_utf8((*win).name),
    );
    verboselog(
        c"Using legacy window title. Note that in order to get Unicode window titles in i3, the application has to set _NET_WM_NAME (UTF-8)\n".as_ptr().cast_mut(),
    );
    (*win).name_x_changed = true_0 != 0;
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_leader(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - CLIENT_LEADER not set on window 0x%08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_leader".as_ptr(),
            145 as ::core::ffi::c_int,
            (*win).id,
        );
        (*win).leader = XCB_NONE as xcb_window_t;
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let leader: *mut xcb_window_t = xcb_get_property_value(prop) as *mut xcb_window_t;
    if leader.is_null() {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        c"%s:%s:%d - Client leader changed to %08x\n"
            .as_ptr()
            .cast_mut(),
        c"../src/window.c".as_ptr(),
        c"window_update_leader".as_ptr(),
        157 as ::core::ffi::c_int,
        *leader,
    );
    (*win).leader = *leader;
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_transient_for(
    win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - TRANSIENT_FOR not set on window 0x%08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_transient_for".as_ptr(),
            170 as ::core::ffi::c_int,
            (*win).id,
        );
        (*win).transient_for = XCB_NONE as xcb_window_t;
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut transient_for: xcb_window_t = 0;
    if xcb_icccm_get_wm_transient_for_from_reply(&raw mut transient_for, prop) == 0 {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        c"%s:%s:%d - Transient for changed to 0x%08x (window 0x%08x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/window.c".as_ptr(),
        c"window_update_transient_for".as_ptr(),
        182 as ::core::ffi::c_int,
        transient_for,
        (*win).id,
    );
    (*win).transient_for = transient_for;
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_strut_partial(
    win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - _NET_WM_STRUT_PARTIAL not set.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_strut_partial".as_ptr(),
            195 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut strut: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    strut = xcb_get_property_value(prop) as *mut uint32_t;
    if strut.is_null() {
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    debuglog(
        c"%s:%s:%d - Reserved pixels changed to: left = %d, right = %d, top = %d, bottom = %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/window.c".as_ptr(),
        c"window_update_strut_partial".as_ptr(),
        207 as ::core::ffi::c_int,
        *strut.offset(0 as ::core::ffi::c_int as isize),
        *strut.offset(1 as ::core::ffi::c_int as isize),
        *strut.offset(2 as ::core::ffi::c_int as isize),
        *strut.offset(3 as ::core::ffi::c_int as isize),
    );
    (*win).reserved = reservedpx {
        left: *strut.offset(0 as ::core::ffi::c_int as isize),
        right: *strut.offset(1 as ::core::ffi::c_int as isize),
        top: *strut.offset(2 as ::core::ffi::c_int as isize),
        bottom: *strut.offset(3 as ::core::ffi::c_int as isize),
    };
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_role(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - WM_WINDOW_ROLE not set.\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_role".as_ptr(),
            220 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut new_role: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut new_role,
        c"%.*s".as_ptr(),
        xcb_get_property_value_length(prop),
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char,
    );
    free((*win).role as *mut ::core::ffi::c_void);
    (*win).role = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).role = new_role;
    verboselog(
        c"WM_WINDOW_ROLE changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win).role,
    );
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_type(window: *mut i3Window, reply: *mut xcb_get_property_reply_t) {
    let new_type: xcb_atom_t = xcb_get_preferred_window_type(reply);
    free(reply as *mut ::core::ffi::c_void);
    if new_type as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            c"%s:%s:%d - cannot read _NET_WM_WINDOW_TYPE from window.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_type".as_ptr(),
            243 as ::core::ffi::c_int,
        );
        return;
    }
    (*window).window_type = new_type;
    verboselog(
        c"_NET_WM_WINDOW_TYPE changed to %i.\n".as_ptr().cast_mut(),
        (*window).window_type,
    );
    run_assignments(window);
}

pub unsafe fn window_update_normal_hints(
    win: *mut i3Window,
    reply: *mut xcb_get_property_reply_t,
    geom: *mut xcb_get_geometry_reply_t,
) -> bool {
    let mut changed: bool = false_0 != 0;
    let mut size_hints: xcb_size_hints_t = xcb_size_hints_t {
        flags: 0,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        min_width: 0,
        min_height: 0,
        max_width: 0,
        max_height: 0,
        width_inc: 0,
        height_inc: 0,
        min_aspect_num: 0,
        min_aspect_den: 0,
        max_aspect_num: 0,
        max_aspect_den: 0,
        base_width: 0,
        base_height: 0,
        win_gravity: 0,
    };
    let mut success: bool = false;
    if !reply.is_null() {
        success = xcb_icccm_get_wm_size_hints_from_reply(&raw mut size_hints, reply) != 0;
    } else {
        success = xcb_icccm_get_wm_normal_hints_reply(
            conn,
            xcb_icccm_get_wm_normal_hints_unchecked(conn, (*win).id),
            &raw mut size_hints,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ) != 0;
    }
    if !success {
        debuglog(
            c"%s:%s:%d - Could not get WM_NORMAL_HINTS\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            269 as ::core::ffi::c_int,
        );
        return false_0 != 0;
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MIN_SIZE as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            c"%s:%s:%d - Minimum size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            282 as ::core::ffi::c_int,
            size_hints.min_width,
            size_hints.min_height,
        );
        if (*win).min_width as int32_t != size_hints.min_width {
            (*win).min_width = size_hints.min_width as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).min_height as int32_t != size_hints.min_height {
            (*win).min_height = size_hints.min_height as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MAX_SIZE as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            c"%s:%s:%d - Maximum size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            289 as ::core::ffi::c_int,
            size_hints.max_width,
            size_hints.max_height,
        );
        let max_width: ::core::ffi::c_int = max(
            0 as ::core::ffi::c_int,
            size_hints.max_width as ::core::ffi::c_int,
        );
        let max_height: ::core::ffi::c_int = max(
            0 as ::core::ffi::c_int,
            size_hints.max_height as ::core::ffi::c_int,
        );
        if (*win).max_width != max_width {
            (*win).max_width = max_width;
            changed = true_0 != 0;
        }
        if (*win).max_height != max_height {
            (*win).max_height = max_height;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            c"%s:%s:%d - Clearing maximum size\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            297 as ::core::ffi::c_int,
        );
        if (*win).max_width != 0 as ::core::ffi::c_int {
            (*win).max_width = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).max_height != 0 as ::core::ffi::c_int {
            (*win).max_height = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_RESIZE_INC as ::core::ffi::c_int as uint32_t != 0 {
        debuglog(
            c"%s:%s:%d - Size increments: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            304 as ::core::ffi::c_int,
            size_hints.width_inc,
            size_hints.height_inc,
        );
        if size_hints.width_inc > 0 as int32_t && size_hints.width_inc < 0xffff as int32_t {
            if (*win).width_increment as int32_t != size_hints.width_inc {
                (*win).width_increment = size_hints.width_inc as ::core::ffi::c_int;
                changed = true_0 != 0;
            }
        } else if (*win).width_increment != 0 as ::core::ffi::c_int {
            (*win).width_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if size_hints.height_inc > 0 as int32_t && size_hints.height_inc < 0xffff as int32_t {
            if (*win).height_increment as int32_t != size_hints.height_inc {
                (*win).height_increment = size_hints.height_inc as ::core::ffi::c_int;
                changed = true_0 != 0;
            }
        } else if (*win).height_increment != 0 as ::core::ffi::c_int {
            (*win).height_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            c"%s:%s:%d - Clearing size increments\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            318 as ::core::ffi::c_int,
        );
        if (*win).width_increment != 0 as ::core::ffi::c_int {
            (*win).width_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).height_increment != 0 as ::core::ffi::c_int {
            (*win).height_increment = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_BASE_SIZE as ::core::ffi::c_int as uint32_t != 0
        && (*win).base_width >= 0 as ::core::ffi::c_int
        && (*win).base_height >= 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - Base size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            327 as ::core::ffi::c_int,
            size_hints.base_width,
            size_hints.base_height,
        );
        if (*win).base_width as int32_t != size_hints.base_width {
            (*win).base_width = size_hints.base_width as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).base_height as int32_t != size_hints.base_height {
            (*win).base_height = size_hints.base_height as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            c"%s:%s:%d - Clearing base size\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            332 as ::core::ffi::c_int,
        );
        if (*win).base_width != 0 as ::core::ffi::c_int {
            (*win).base_width = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
        if (*win).base_height != 0 as ::core::ffi::c_int {
            (*win).base_height = 0 as ::core::ffi::c_int;
            changed = true_0 != 0;
        }
    }
    if !geom.is_null()
        && (size_hints.flags & XCB_ICCCM_SIZE_HINT_US_POSITION as ::core::ffi::c_int as uint32_t
            != 0
            || size_hints.flags & XCB_ICCCM_SIZE_HINT_P_POSITION as ::core::ffi::c_int as uint32_t
                != 0)
        && (size_hints.flags & XCB_ICCCM_SIZE_HINT_US_SIZE as ::core::ffi::c_int as uint32_t != 0
            || size_hints.flags & XCB_ICCCM_SIZE_HINT_P_SIZE as ::core::ffi::c_int as uint32_t != 0)
    {
        debuglog(
            c"%s:%s:%d - Setting geometry x=%d y=%d w=%d h=%d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            341 as ::core::ffi::c_int,
            size_hints.x,
            size_hints.y,
            size_hints.width,
            size_hints.height,
        );
        (*geom).x = size_hints.x as int16_t;
        (*geom).y = size_hints.y as int16_t;
        (*geom).width = size_hints.width as uint16_t;
        (*geom).height = size_hints.height as uint16_t;
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_ASPECT as ::core::ffi::c_int as uint32_t != 0
        && size_hints.min_aspect_num >= 0 as int32_t
        && size_hints.min_aspect_den > 0 as int32_t
        && size_hints.max_aspect_num >= 0 as int32_t
        && size_hints.max_aspect_den > 0 as int32_t
    {
        let min_aspect: ::core::ffi::c_double = size_hints.min_aspect_num as ::core::ffi::c_double
            / size_hints.min_aspect_den as ::core::ffi::c_double;
        let max_aspect: ::core::ffi::c_double = size_hints.max_aspect_num as ::core::ffi::c_double
            / size_hints.max_aspect_den as ::core::ffi::c_double;
        debuglog(
            c"%s:%s:%d - Aspect ratio set: minimum %f, maximum %f\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            355 as ::core::ffi::c_int,
            min_aspect,
            max_aspect,
        );
        if fabs((*win).min_aspect_ratio - min_aspect) > DBL_EPSILON {
            (*win).min_aspect_ratio = min_aspect;
            changed = true_0 != 0;
        }
        if fabs((*win).max_aspect_ratio - max_aspect) > DBL_EPSILON {
            (*win).max_aspect_ratio = max_aspect;
            changed = true_0 != 0;
        }
    } else {
        debuglog(
            c"%s:%s:%d - Clearing aspect ratios\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            365 as ::core::ffi::c_int,
        );
        if (*win).min_aspect_ratio != 0.0f64 {
            (*win).min_aspect_ratio = 0.0f64;
            changed = true_0 != 0;
        }
        if (*win).max_aspect_ratio != 0.0f64 {
            (*win).max_aspect_ratio = 0.0f64;
            changed = true_0 != 0;
        }
    }
    changed
}

pub unsafe fn window_update_hints(
    win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
    urgency_hint: *mut bool,
) {
    if !urgency_hint.is_null() {
        *urgency_hint = false_0 != 0;
    }
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - WM_HINTS not set.\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_hints".as_ptr(),
            384 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
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
    if xcb_icccm_get_wm_hints_from_reply(&raw mut hints, prop) == 0 {
        debuglog(
            c"%s:%s:%d - Could not get WM_HINTS\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_hints".as_ptr(),
            392 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        return;
    }
    if hints.flags & XCB_ICCCM_WM_HINT_INPUT as ::core::ffi::c_int as int32_t != 0 {
        (*win).doesnt_accept_focus = hints.input == 0;
        verboselog(
            c"WM_HINTS.input changed to \"%d\"\n".as_ptr().cast_mut(),
            hints.input,
        );
    }
    if !urgency_hint.is_null() {
        *urgency_hint = xcb_icccm_wm_hints_get_urgency(&raw mut hints) != 0 as uint32_t;
    }
    free(prop as *mut ::core::ffi::c_void);
}
pub const MWM_HINTS_FLAGS_FIELD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MWM_HINTS_DECORATIONS_FIELD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MWM_HINTS_DECORATIONS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_ALL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const MWM_DECOR_BORDER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const MWM_DECOR_TITLE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
unsafe fn border_style_from_motif_value(value: uint32_t) -> border_style_t {
    if value & MWM_DECOR_ALL as uint32_t != 0 {
        if value & MWM_DECOR_TITLE as uint32_t != 0 {
            if value & MWM_DECOR_BORDER as uint32_t != 0 {
                return BS_NONE;
            }
            return BS_PIXEL;
        }
        return BS_NORMAL;
    }
    if value & MWM_DECOR_TITLE as uint32_t != 0 {
        return BS_NORMAL;
    }
    if value & MWM_DECOR_BORDER as uint32_t != 0 {
        return BS_PIXEL;
    }
    BS_NONE
}

pub unsafe fn window_update_motif_hints(
    win: *mut i3Window,
    mut prop: *mut xcb_get_property_reply_t,
    motif_border_style: *mut border_style_t,
) -> bool {
    if prop.is_null() {
        return false_0 != 0;
    }
    if motif_border_style != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut border_style_t {
    } else {
        __assert_fail(
            c"motif_border_style != NULL".as_ptr(),
            c"../src/window.c".as_ptr(),
            469 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    if xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return false_0 != 0;
    }
    let motif_hints: *mut uint32_t = xcb_get_property_value(prop) as *mut uint32_t;
    if *motif_hints.offset(MWM_HINTS_FLAGS_FIELD as isize) & MWM_HINTS_DECORATIONS as uint32_t != 0
    {
        *motif_border_style = border_style_from_motif_value(
            *motif_hints.offset(MWM_HINTS_DECORATIONS_FIELD as isize),
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return true_0 != 0;
    }
    free(prop as *mut ::core::ffi::c_void);
    prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    false_0 != 0
}

pub unsafe fn window_update_machine(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        debuglog(
            c"%s:%s:%d - WM_CLIENT_MACHINE not set.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_machine".as_ptr(),
            509 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    free((*win).machine as *mut ::core::ffi::c_void);
    (*win).machine = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*win).machine = sstrndup(
        xcb_get_property_value(prop) as *mut ::core::ffi::c_char,
        xcb_get_property_value_length(prop) as size_t,
    );
    verboselog(
        c"WM_CLIENT_MACHINE changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win).machine,
    );
    free(prop as *mut ::core::ffi::c_void);
}

pub unsafe fn window_update_icon(win: *mut i3Window, mut prop: *mut xcb_get_property_reply_t) {
    let mut data: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut width: uint32_t = 0 as uint32_t;
    let mut height: uint32_t = 0 as uint32_t;
    let mut len: uint64_t = 0 as uint64_t;
    let pref_size: uint32_t =
        (render_deco_height() - logical_px(2 as ::core::ffi::c_int)) as uint32_t;
    if prop.is_null()
        || (*prop).type_0 != XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t
        || (*prop).format as ::core::ffi::c_int != 32 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - _NET_WM_ICON is not set\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_icon".as_ptr(),
            528 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    let mut prop_value_len: uint32_t = xcb_get_property_value_length(prop) as uint32_t;
    let mut prop_value: *mut uint32_t = xcb_get_property_value(prop) as *mut uint32_t;
    while prop_value_len as usize
        > (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(2_usize)
        && !prop_value.is_null()
        && *prop_value.offset(0 as ::core::ffi::c_int as isize) != 0
        && *prop_value.offset(1 as ::core::ffi::c_int as isize) != 0
    {
        let cur_width: uint32_t = *prop_value.offset(0 as ::core::ffi::c_int as isize);
        let cur_height: uint32_t = *prop_value.offset(1 as ::core::ffi::c_int as isize);
        let cur_len: uint64_t = (cur_width as uint64_t).wrapping_mul(cur_height as uint64_t);
        let expected_len: uint64_t = cur_len
            .wrapping_add(2 as uint64_t)
            .wrapping_mul(4 as uint64_t);
        if expected_len > prop_value_len as uint64_t {
            break;
        }
        debuglog(
            c"%s:%s:%d - Found _NET_WM_ICON of size: (%d,%d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_icon".as_ptr(),
            555 as ::core::ffi::c_int,
            cur_width,
            cur_height,
        );
        let at_least_preferred_size: bool = cur_width >= pref_size && cur_height >= pref_size;
        let smaller_than_current: bool = cur_width < width || cur_height < height;
        let larger_than_current: bool = cur_width > width || cur_height > height;
        let not_yet_at_preferred: bool = width < pref_size || height < pref_size;
        if len == 0 as uint64_t
            || at_least_preferred_size as ::core::ffi::c_int != 0
                && (smaller_than_current as ::core::ffi::c_int != 0
                    || not_yet_at_preferred as ::core::ffi::c_int != 0)
            || !at_least_preferred_size
                && not_yet_at_preferred as ::core::ffi::c_int != 0
                && larger_than_current as ::core::ffi::c_int != 0
        {
            len = cur_len;
            width = cur_width;
            height = cur_height;
            data = prop_value;
        }
        if width == pref_size && height == pref_size {
            break;
        }
        prop_value_len =
            (prop_value_len as uint64_t).wrapping_sub(expected_len) as uint32_t as uint32_t;
        prop_value = (prop_value as *mut uint8_t).offset(expected_len as isize) as *mut uint32_t;
    }
    if data.is_null() {
        debuglog(
            c"%s:%s:%d - Could not get _NET_WM_ICON\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_icon".as_ptr(),
            587 as ::core::ffi::c_int,
        );
        free(prop as *mut ::core::ffi::c_void);
        prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
        return;
    }
    debuglog(
        c"%s:%s:%d - Using icon of size (%d,%d) (preferred size: %d)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/window.c".as_ptr(),
        c"window_update_icon".as_ptr(),
        593 as ::core::ffi::c_int,
        width,
        height,
        pref_size,
    );
    (*win).name_x_changed = true_0 != 0;
    let icon: *mut uint32_t = smalloc((len as size_t).wrapping_mul(4 as size_t)) as *mut uint32_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < len {
        let pixel: uint32_t = *data.offset((2 as uint64_t).wrapping_add(i) as isize);
        let a: uint8_t = (pixel >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut r: uint8_t = (pixel >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut g: uint8_t = (pixel >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let mut b: uint8_t = (pixel >> 0 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        r = (r as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        g = (g as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        b = (b as ::core::ffi::c_int * a as ::core::ffi::c_int / 0xff as ::core::ffi::c_int)
            as uint8_t;
        *icon.offset(i as isize) = (a as uint32_t) << 24 as ::core::ffi::c_int
            | ((r as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t
            | ((g as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t
            | b as uint32_t;
        i = i.wrapping_add(1);
    }
    if !(*win).icon.is_null() {
        cairo_surface_destroy((*win).icon);
    }
    (*win).icon = cairo_image_surface_create_for_data(
        icon as *mut ::core::ffi::c_uchar,
        FORMAT_A_RGB32,
        width as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        width.wrapping_mul(4 as uint32_t) as ::core::ffi::c_int,
    );
    static mut free_data: cairo_user_data_key_t = cairo_user_data_key_t { unused: 0 };
    cairo_surface_set_user_data(
        (*win).icon,
        &raw mut free_data,
        icon as *mut ::core::ffi::c_void,
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    free(prop as *mut ::core::ffi::c_void);
    prop = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
}
pub const __DBL_EPSILON__: ::core::ffi::c_double = 2.2204460492503131e-16f64;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::ewmh::ewmh_update_visible_name;
use crate::render::render_deco_height;
use crate::util::max;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrndup;

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{
    cairo_image_surface_create_for_data, cairo_surface_destroy, cairo_surface_set_user_data,
    cairo_user_data_key_t, FORMAT_A_RGB32,
};

use xcb_sys::{
    xcb_atom_t, xcb_generic_error_t, xcb_get_geometry_reply_t, xcb_get_property_reply_t,
    xcb_get_property_value, xcb_get_property_value_length, xcb_icccm_get_wm_hints_from_reply,
    xcb_icccm_get_wm_normal_hints_reply, xcb_icccm_get_wm_normal_hints_unchecked,
    xcb_icccm_get_wm_size_hints_from_reply, xcb_icccm_get_wm_transient_for_from_reply,
    xcb_icccm_wm_hints_get_urgency, xcb_icccm_wm_hints_t, xcb_size_hints_t, xcb_window_t,
    XCB_ATOM_CARDINAL, XCB_ICCCM_SIZE_HINT_BASE_SIZE, XCB_ICCCM_SIZE_HINT_P_ASPECT,
    XCB_ICCCM_SIZE_HINT_P_MAX_SIZE, XCB_ICCCM_SIZE_HINT_P_MIN_SIZE, XCB_ICCCM_SIZE_HINT_P_POSITION,
    XCB_ICCCM_SIZE_HINT_P_RESIZE_INC, XCB_ICCCM_SIZE_HINT_P_SIZE, XCB_ICCCM_SIZE_HINT_US_POSITION,
    XCB_ICCCM_SIZE_HINT_US_SIZE, XCB_ICCCM_WM_HINT_INPUT, XCB_NONE,
};

pub use libc::{free, strnlen};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::reservedpx;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::assignments::run_assignments;
use crate::con::con_by_window_id;
use crate::con::con_parse_title_format;
use crate::xcb::xcb_get_preferred_window_type;
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8;
