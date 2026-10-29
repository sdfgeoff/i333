/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * ewmh.c: Get/set certain EWMH properties easily.
 *
 */
// Mechanical file-by-file Rust translation of src/ewmh.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;

pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
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

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

pub static mut ewmh_window: xcb_window_t = 0;

pub unsafe fn ewmh_update_current_desktop() {
    static mut old_idx: uint32_t = NET_WM_DESKTOP_NONE as uint32_t;
    let idx: uint32_t = ewmh_get_workspace_index(focused) as uint32_t;
    if idx == old_idx || idx == NET_WM_DESKTOP_NONE as uint32_t {
        return;
    }
    old_idx = idx;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CURRENT_DESKTOP,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw const idx as *const ::core::ffi::c_void,
    );
}
unsafe fn ewmh_update_number_of_desktops() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    static mut old_idx: uint32_t = 0 as uint32_t;
    let mut idx: uint32_t = 0 as uint32_t;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                idx = idx.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    if idx == old_idx {
        return;
    }
    old_idx = idx;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_NUMBER_OF_DESKTOPS,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut idx as *const ::core::ffi::c_void,
    );
}
unsafe fn ewmh_update_desktop_names() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut msg_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                msg_length = (msg_length as size_t)
                    .wrapping_add(strlen((*ws).name).wrapping_add(1 as size_t))
                    as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let vla = msg_length as usize;
    let mut desktop_names: Vec<::core::ffi::c_char> = ::std::vec::from_elem(0, vla);
    let mut current_position: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                let mut i: size_t = 0 as size_t;
                while i < strlen((*ws).name).wrapping_add(1 as size_t) {
                    let fresh0 = current_position;
                    current_position = current_position + 1;
                    *desktop_names.as_mut_ptr().offset(fresh0 as isize) =
                        *(*ws).name.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_DESKTOP_NAMES,
        A_UTF8_STRING,
        8 as uint8_t,
        msg_length as uint32_t,
        desktop_names.as_mut_ptr() as *const ::core::ffi::c_void,
    );
}
unsafe fn ewmh_update_desktop_viewport() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut num_desktops: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                num_desktops += 1;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    let vla = (num_desktops * 2 as ::core::ffi::c_int) as usize;
    let mut viewports: Vec<uint32_t> = ::std::vec::from_elem(0, vla);
    let mut current_position: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                let fresh1 = current_position;
                current_position = current_position + 1;
                *viewports.as_mut_ptr().offset(fresh1 as isize) = (*output).rect.x;
                let fresh2 = current_position;
                current_position = current_position + 1;
                *viewports.as_mut_ptr().offset(fresh2 as isize) = (*output).rect.y;
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_DESKTOP_VIEWPORT,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        current_position as uint32_t,
        viewports.as_mut_ptr() as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_desktop_properties() {
    ewmh_update_number_of_desktops();
    ewmh_update_desktop_viewport();
    ewmh_update_current_desktop();
    ewmh_update_desktop_names();
    ewmh_update_wm_desktop();
}
unsafe fn ewmh_update_wm_desktop_recursively(con: *mut Con, desktop: uint32_t) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        ewmh_update_wm_desktop_recursively(child, desktop);
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if (*con).type_0 as ::core::ffi::c_uint
        == CT_WORKSPACE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        child = (*con).floating_head.tqh_first as *mut Con;
        while !child.is_null() {
            ewmh_update_wm_desktop_recursively(child, desktop);
            child = (*child).floating_windows.tqe_next as *mut Con;
        }
    }
    if !con_has_managed_window(con) {
        return;
    }
    let mut wm_desktop: uint32_t = desktop;
    if con_is_sticky(con) as ::core::ffi::c_int != 0
        && con_is_floating(con) as ::core::ffi::c_int != 0
    {
        wm_desktop = NET_WM_DESKTOP_ALL as uint32_t;
    }
    let ws: *mut Con = con_get_workspace(con);
    if !ws.is_null() && con_is_internal(ws) as ::core::ffi::c_int != 0 {
        wm_desktop = NET_WM_DESKTOP_ALL as uint32_t;
    }
    if (*(*con).window).wm_desktop == wm_desktop {
        return;
    }
    (*(*con).window).wm_desktop = wm_desktop;
    let window: xcb_window_t = (*(*con).window).id;
    if wm_desktop != NET_WM_DESKTOP_NONE as uint32_t {
        debuglog(
            c"%s:%s:%d - Setting _NET_WM_DESKTOP = %d for window 0x%08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ewmh.c".as_ptr(),
            c"ewmh_update_wm_desktop_recursively".as_ptr(),
            171 as ::core::ffi::c_int,
            wm_desktop,
            window,
        );
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            window,
            A__NET_WM_DESKTOP,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as uint8_t,
            1 as uint32_t,
            &raw mut wm_desktop as *const ::core::ffi::c_void,
        );
    } else {
        errorlog(
            c"ERROR: Failed to determine the proper EWMH desktop index for window 0x%08x, deleting _NET_WM_DESKTOP.\n".as_ptr().cast_mut(),
            window,
        );
        xcb_delete_property(conn, window, A__NET_WM_DESKTOP);
    };
}

pub unsafe fn ewmh_update_wm_desktop() {
    let mut desktop: uint32_t = 0 as uint32_t;
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        let mut workspace: *mut Con = ::core::ptr::null_mut::<Con>();
        workspace = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !workspace.is_null() {
            ewmh_update_wm_desktop_recursively(workspace, desktop);
            if !con_is_internal(workspace) {
                desktop = desktop.wrapping_add(1);
            }
            workspace = (*workspace).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}

pub unsafe fn ewmh_update_active_window(mut window: xcb_window_t) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_ACTIVE_WINDOW,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut window as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_visible_name(window: xcb_window_t, name: *const ::core::ffi::c_char) {
    if !name.is_null() {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
            window,
            A__NET_WM_VISIBLE_NAME,
            A_UTF8_STRING,
            8 as uint8_t,
            strlen(name) as uint32_t,
            name as *const ::core::ffi::c_void,
        );
    } else {
        xcb_delete_property(conn, window, A__NET_WM_VISIBLE_NAME);
    };
}

pub unsafe fn ewmh_update_workarea() {
    xcb_delete_property(conn, root, A__NET_WORKAREA);
}

pub unsafe fn ewmh_update_client_list(list: *mut xcb_window_t, num_windows: ::core::ffi::c_int) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CLIENT_LIST,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        num_windows as uint32_t,
        list as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_client_list_stacking(
    stack: *mut xcb_window_t,
    num_windows: ::core::ffi::c_int,
) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_CLIENT_LIST_STACKING,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        num_windows as uint32_t,
        stack as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_sticky(window: xcb_window_t, sticky: bool) {
    if sticky {
        debuglog(
            c"%s:%s:%d - Setting _NET_WM_STATE_STICKY for window = %08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ewmh.c".as_ptr(),
            c"ewmh_update_sticky".as_ptr(),
            283 as ::core::ffi::c_int,
            window,
        );
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    } else {
        debuglog(
            c"%s:%s:%d - Removing _NET_WM_STATE_STICKY for window = %08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ewmh.c".as_ptr(),
            c"ewmh_update_sticky".as_ptr(),
            286 as ::core::ffi::c_int,
            window,
        );
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_STICKY);
    };
}

pub unsafe fn ewmh_update_focused(window: xcb_window_t, is_focused: bool) {
    if is_focused {
        debuglog(
            c"%s:%s:%d - Setting _NET_WM_STATE_FOCUSED for window = %08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ewmh.c".as_ptr(),
            c"ewmh_update_focused".as_ptr(),
            297 as ::core::ffi::c_int,
            window,
        );
        xcb_add_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    } else {
        debuglog(
            c"%s:%s:%d - Removing _NET_WM_STATE_FOCUSED for window = %08x.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/ewmh.c".as_ptr(),
            c"ewmh_update_focused".as_ptr(),
            300 as ::core::ffi::c_int,
            window,
        );
        xcb_remove_property_atom(conn, window, A__NET_WM_STATE, A__NET_WM_STATE_FOCUSED);
    };
}

pub unsafe fn ewmh_setup_hints() {
    let mut supported_atoms: [xcb_atom_t; 37] = [
        A__NET_SUPPORTED,
        A__NET_SUPPORTING_WM_CHECK,
        A__NET_WM_NAME,
        A__NET_WM_VISIBLE_NAME,
        A__NET_WM_MOVERESIZE,
        A__NET_WM_STATE_STICKY,
        A__NET_WM_STATE_FULLSCREEN,
        A__NET_WM_STATE_DEMANDS_ATTENTION,
        A__NET_WM_STATE_MODAL,
        A__NET_WM_STATE_HIDDEN,
        A__NET_WM_STATE_FOCUSED,
        A__NET_WM_STATE_MAXIMIZED_VERT,
        A__NET_WM_STATE_MAXIMIZED_HORZ,
        A__NET_WM_STATE,
        A__NET_WM_WINDOW_TYPE,
        A__NET_WM_WINDOW_TYPE_NORMAL,
        A__NET_WM_WINDOW_TYPE_DOCK,
        A__NET_WM_WINDOW_TYPE_DIALOG,
        A__NET_WM_WINDOW_TYPE_UTILITY,
        A__NET_WM_WINDOW_TYPE_TOOLBAR,
        A__NET_WM_WINDOW_TYPE_SPLASH,
        A__NET_WM_WINDOW_TYPE_MENU,
        A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
        A__NET_WM_WINDOW_TYPE_POPUP_MENU,
        A__NET_WM_WINDOW_TYPE_TOOLTIP,
        A__NET_WM_WINDOW_TYPE_NOTIFICATION,
        A__NET_WM_DESKTOP,
        A__NET_WM_STRUT_PARTIAL,
        A__NET_CLIENT_LIST,
        A__NET_CLIENT_LIST_STACKING,
        A__NET_CURRENT_DESKTOP,
        A__NET_NUMBER_OF_DESKTOPS,
        A__NET_DESKTOP_NAMES,
        A__NET_DESKTOP_VIEWPORT,
        A__NET_ACTIVE_WINDOW,
        A__NET_CLOSE_WINDOW,
        A__NET_MOVERESIZE_WINDOW,
    ];
    ewmh_window = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh3: [uint32_t; 1] = [1 as ::core::ffi::c_int as uint32_t];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as uint8_t,
        ewmh_window,
        root,
        -(1 as ::core::ffi::c_int) as int16_t,
        -(1 as ::core::ffi::c_int) as int16_t,
        1 as uint16_t,
        1 as uint16_t,
        0 as uint16_t,
        XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int as uint16_t,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as uint32_t,
        &raw mut fresh3 as *mut uint32_t as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        ewmh_window,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        ewmh_window,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(c"i3".as_ptr()) as uint32_t,
        c"i3".as_ptr() as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        1 as uint32_t,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as uint8_t,
        strlen(c"i3".as_ptr()) as uint32_t,
        c"i3".as_ptr() as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as uint8_t,
        root,
        A__NET_SUPPORTED,
        XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
        32 as uint8_t,
        (::core::mem::size_of::<[xcb_atom_t; 37]>() as usize)
            .wrapping_div(::core::mem::size_of::<xcb_atom_t>() as usize) as uint32_t,
        &raw mut supported_atoms as *mut xcb_atom_t as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, ewmh_window);
    let mut fresh4: [uint32_t; 1] = [XCB_STACK_MODE_BELOW as ::core::ffi::c_int as uint32_t];
    xcb_configure_window(
        conn,
        ewmh_window,
        XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as uint16_t,
        &raw mut fresh4 as *mut uint32_t as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_get_workspace_by_index(idx: uint32_t) -> *mut Con {
    if idx == NET_WM_DESKTOP_NONE as uint32_t {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut current_index: uint32_t = 0 as uint32_t;
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                if current_index == idx {
                    return ws;
                }
                current_index = current_index.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    ::core::ptr::null_mut::<Con>()
}

pub unsafe fn ewmh_get_workspace_index(con: *mut Con) -> uint32_t {
    let mut index: uint32_t = 0 as uint32_t;
    let target_workspace: *mut Con = con_get_workspace(con);
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    output = (*croot).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        ws = (*output_get_content(output)).nodes_head.tqh_first as *mut Con;
        while !ws.is_null() {
            if !con_is_internal(ws) {
                if ws == target_workspace {
                    return index;
                }
                index = index.wrapping_add(1);
            }
            ws = (*ws).nodes.tqe_next as *mut Con;
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
    NET_WM_DESKTOP_NONE as uint32_t
}

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_atom_t, xcb_change_property, xcb_configure_window, xcb_create_window, xcb_delete_property,
    xcb_generate_id, xcb_map_window, xcb_visualid_t, xcb_window_t, XCB_ATOM_ATOM,
    XCB_ATOM_CARDINAL, XCB_ATOM_WINDOW, XCB_CONFIG_WINDOW_STACK_MODE, XCB_COPY_FROM_PARENT,
    XCB_CW_OVERRIDE_REDIRECT, XCB_PROP_MODE_REPLACE, XCB_STACK_MODE_BELOW,
    XCB_WINDOW_CLASS_INPUT_ONLY,
};

pub use libc::strlen;

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::A_UTF8_STRING;
use crate::application::A__NET_ACTIVE_WINDOW;
use crate::application::A__NET_CLIENT_LIST;
use crate::application::A__NET_CLIENT_LIST_STACKING;
use crate::application::A__NET_CLOSE_WINDOW;
use crate::application::A__NET_CURRENT_DESKTOP;
use crate::application::A__NET_DESKTOP_NAMES;
use crate::application::A__NET_DESKTOP_VIEWPORT;
use crate::application::A__NET_MOVERESIZE_WINDOW;
use crate::application::A__NET_NUMBER_OF_DESKTOPS;
use crate::application::A__NET_SUPPORTED;
use crate::application::A__NET_SUPPORTING_WM_CHECK;
use crate::application::A__NET_WM_DESKTOP;
use crate::application::A__NET_WM_MOVERESIZE;
use crate::application::A__NET_WM_NAME;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_DEMANDS_ATTENTION;
use crate::application::A__NET_WM_STATE_FOCUSED;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::application::A__NET_WM_STATE_HIDDEN;
use crate::application::A__NET_WM_STATE_MAXIMIZED_HORZ;
use crate::application::A__NET_WM_STATE_MAXIMIZED_VERT;
use crate::application::A__NET_WM_STATE_MODAL;
use crate::application::A__NET_WM_STATE_STICKY;
use crate::application::A__NET_WM_STRUT_PARTIAL;
use crate::application::A__NET_WM_VISIBLE_NAME;
use crate::application::A__NET_WM_WINDOW_TYPE;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DOCK;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::application::A__NET_WORKAREA;
use crate::con::con_get_workspace;
use crate::con::con_has_managed_window;
use crate::con::con_is_floating;
use crate::con::con_is_internal;
use crate::con::con_is_sticky;
use crate::output::output_get_content;
use crate::tree::croot;
use crate::tree::focused;
use crate::xcb::xcb_add_property_atom;
use crate::xcb::xcb_remove_property_atom;
