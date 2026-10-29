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

pub type i3String = _i3String;

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

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;

pub static mut ewmh_window: xcb_window_t = 0;

pub unsafe fn ewmh_update_current_desktop() {
    static mut old_idx: u32 = NET_WM_DESKTOP_NONE as u32;
    let idx: u32 = ewmh_get_workspace_index(focused) as u32;
    if idx == old_idx || idx == NET_WM_DESKTOP_NONE as u32 {
        return;
    }
    old_idx = idx;
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_CURRENT_DESKTOP,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw const idx as *const ::core::ffi::c_void,
    );
}
unsafe fn ewmh_update_number_of_desktops() {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    static mut old_idx: u32 = 0 as u32;
    let mut idx: u32 = 0 as u32;
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_NUMBER_OF_DESKTOPS,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
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
                msg_length = (msg_length as usize)
                    .wrapping_add(strlen((*ws).name).wrapping_add(1 as usize))
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
                let mut i: usize = 0 as usize;
                while i < strlen((*ws).name).wrapping_add(1 as usize) {
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_DESKTOP_NAMES,
        A_UTF8_STRING,
        8 as u8,
        msg_length as u32,
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
    let mut viewports: Vec<u32> = ::std::vec::from_elem(0, vla);
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_DESKTOP_VIEWPORT,
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        current_position as u32,
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
unsafe fn ewmh_update_wm_desktop_recursively(con: *mut Con, desktop: u32) {
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
    let mut wm_desktop: u32 = desktop;
    if con_is_sticky(con) as ::core::ffi::c_int != 0
        && con_is_floating(con) as ::core::ffi::c_int != 0
    {
        wm_desktop = NET_WM_DESKTOP_ALL as u32;
    }
    let ws: *mut Con = con_get_workspace(con);
    if !ws.is_null() && con_is_internal(ws) as ::core::ffi::c_int != 0 {
        wm_desktop = NET_WM_DESKTOP_ALL as u32;
    }
    if (*(*con).window).wm_desktop == wm_desktop {
        return;
    }
    (*(*con).window).wm_desktop = wm_desktop;
    let window: xcb_window_t = (*(*con).window).id;
    if wm_desktop != NET_WM_DESKTOP_NONE as u32 {
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
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            window,
            A__NET_WM_DESKTOP,
            XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
            32 as u8,
            1 as u32,
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
    let mut desktop: u32 = 0 as u32;
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_ACTIVE_WINDOW,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut window as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_visible_name(window: xcb_window_t, name: *const ::core::ffi::c_char) {
    if !name.is_null() {
        xcb_change_property(
            conn,
            XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
            window,
            A__NET_WM_VISIBLE_NAME,
            A_UTF8_STRING,
            8 as u8,
            strlen(name) as u32,
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
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_CLIENT_LIST,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        num_windows as u32,
        list as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_update_client_list_stacking(
    stack: *mut xcb_window_t,
    num_windows: ::core::ffi::c_int,
) {
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_CLIENT_LIST_STACKING,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        num_windows as u32,
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
    let mut fresh3: [u32; 1] = [1 as ::core::ffi::c_int as u32];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as u8,
        ewmh_window,
        root,
        -(1 as ::core::ffi::c_int) as i16,
        -(1 as ::core::ffi::c_int) as i16,
        1 as u16,
        1 as u16,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_ONLY as ::core::ffi::c_int as u16,
        XCB_COPY_FROM_PARENT as xcb_visualid_t,
        XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32,
        &raw mut fresh3 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        ewmh_window,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        ewmh_window,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as u8,
        strlen(c"i3".as_ptr()) as u32,
        c"i3".as_ptr() as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_SUPPORTING_WM_CHECK,
        XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut ewmh_window as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_WM_NAME,
        A_UTF8_STRING,
        8 as u8,
        strlen(c"i3".as_ptr()) as u32,
        c"i3".as_ptr() as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        conn,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        root,
        A__NET_SUPPORTED,
        XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        (::core::mem::size_of::<[xcb_atom_t; 37]>() as usize)
            .wrapping_div(::core::mem::size_of::<xcb_atom_t>() as usize) as u32,
        &raw mut supported_atoms as *mut xcb_atom_t as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, ewmh_window);
    let mut fresh4: [u32; 1] = [XCB_STACK_MODE_BELOW as ::core::ffi::c_int as u32];
    xcb_configure_window(
        conn,
        ewmh_window,
        XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int as u16,
        &raw mut fresh4 as *mut u32 as *const ::core::ffi::c_void,
    );
}

pub unsafe fn ewmh_get_workspace_by_index(idx: u32) -> *mut Con {
    if idx == NET_WM_DESKTOP_NONE as u32 {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut current_index: u32 = 0 as u32;
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

pub unsafe fn ewmh_get_workspace_index(con: *mut Con) -> u32 {
    let mut index: u32 = 0 as u32;
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
    NET_WM_DESKTOP_NONE as u32
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
