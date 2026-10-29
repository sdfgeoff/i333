use crate::randr::outputs;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * This is LEGACY code (we support RandR, which can do much more than
 * Xinerama), but necessary for the poor users of the nVidia binary
 * driver which does not support RandR in 2011 *sigh*.
 *
 */
// Mechanical file-by-file Rust translation of src/xinerama.c.
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

pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_0 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_0 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_5 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_5 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_5 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_5 = 0;
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

pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_7 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_7 = 1;
pub const M_HERE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_8 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_8 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_8 = 4;
pub const WM_TILING: C2RustUnnamed_8 = 3;
pub const WM_TILING_USER: C2RustUnnamed_8 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_8 = 1;
pub const WM_ANY: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_9 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_9 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_9 = 1;
pub const M_NODOCK: C2RustUnnamed_9 = 0;
pub const M_DONTCHECK: C2RustUnnamed_9 = -1;
pub type C2RustUnnamed_10 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_10 = 1;
pub const U_LATEST: C2RustUnnamed_10 = 0;
pub const U_DONTCHECK: C2RustUnnamed_10 = -1;

pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_11 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_11 = 1;
pub const W_NODOCK: C2RustUnnamed_11 = 0;

pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_14 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_14 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_14 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_14 = 2;
pub const A_COMMAND: C2RustUnnamed_14 = 1;
pub const A_ANY: C2RustUnnamed_14 = 0;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_16 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_16 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_16 = 3;
pub const CT_CON: C2RustUnnamed_16 = 2;
pub const CT_OUTPUT: C2RustUnnamed_16 = 1;
pub const CT_ROOT: C2RustUnnamed_16 = 0;

pub type Output = xoutput;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut num_screens: ::core::ffi::c_int = 0;
unsafe fn get_screen_at(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).rect.x == x as uint32_t && (*output).rect.y == y as uint32_t {
            return output;
        }
        output = (*output).outputs.tqe_next as *mut Output;
    }
    ::core::ptr::null_mut::<Output>()
}
unsafe fn query_screens(conn_0: *mut xcb_connection_t) {
    let reply: *mut xcb_xinerama_query_screens_reply_t = xcb_xinerama_query_screens_reply(
        conn_0,
        xcb_xinerama_query_screens_unchecked(conn_0),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if reply.is_null() {
        errorlog(
            c"ERROR: Couldn't get Xinerama screens\n"
                .as_ptr()
                .cast_mut(),
        );
        return;
    }
    let screen_info: *const xcb_xinerama_screen_info_t =
        xcb_xinerama_query_screens_screen_info(reply);
    let screens: ::core::ffi::c_int = xcb_xinerama_query_screens_screen_info_length(reply);
    let mut screen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while screen < screens {
        let mut s: *mut Output = get_screen_at(
            (*screen_info.offset(screen as isize)).x_org as ::core::ffi::c_uint,
            (*screen_info.offset(screen as isize)).y_org as ::core::ffi::c_uint,
        );
        if !s.is_null() {
            debuglog(
                c"%s:%s:%d - Re-used old Xinerama screen %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"query_screens".as_ptr(),
                50 as ::core::ffi::c_int,
                s,
            );
            (*s).rect.width = min(
                (*s).rect.width as ::core::ffi::c_int,
                (*screen_info.offset(screen as isize)).width as ::core::ffi::c_int,
            ) as uint32_t;
            (*s).rect.height = min(
                (*s).rect.height as ::core::ffi::c_int,
                (*screen_info.offset(screen as isize)).height as ::core::ffi::c_int,
            ) as uint32_t;
        } else {
            s = scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
            let output_name: *mut output_name =
                scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t)
                    as *mut output_name;
            sasprintf(
                &raw mut (*output_name).name,
                c"xinerama-%d".as_ptr(),
                num_screens,
            );
            (*s).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
            (*output_name).names.sle_next = (*s).names_head.slh_first;
            (*s).names_head.slh_first = output_name;
            debuglog(
                c"%s:%s:%d - Created new Xinerama screen %s (%p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"query_screens".as_ptr(),
                61 as ::core::ffi::c_int,
                output_primary_name(s),
                s,
            );
            (*s).active = true_0 != 0;
            (*s).rect.x = (*screen_info.offset(screen as isize)).x_org as uint32_t;
            (*s).rect.y = (*screen_info.offset(screen as isize)).y_org as uint32_t;
            (*s).rect.width = (*screen_info.offset(screen as isize)).width as uint32_t;
            (*s).rect.height = (*screen_info.offset(screen as isize)).height as uint32_t;
            if (*s).rect.x == 0 as uint32_t && (*s).rect.y == 0 as uint32_t {
                (*s).outputs.tqe_next = outputs.tqh_first;
                if !(*s).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*s).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*s).outputs.tqe_next;
                }
                outputs.tqh_first = s as *mut xoutput;
                (*s).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*s).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*s).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = s as *mut xoutput;
                outputs.tqh_last = &raw mut (*s).outputs.tqe_next;
            }
            output_init_con(s);
            init_ws_for_output(s);
            num_screens += 1;
        }
        debuglog(
            c"%s:%s:%d - found Xinerama screen: %d x %d at %d x %d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/xinerama.c".as_ptr(),
            c"query_screens".as_ptr(),
            80 as ::core::ffi::c_int,
            (*screen_info.offset(screen as isize)).width as ::core::ffi::c_int,
            (*screen_info.offset(screen as isize)).height as ::core::ffi::c_int,
            (*screen_info.offset(screen as isize)).x_org as ::core::ffi::c_int,
            (*screen_info.offset(screen as isize)).y_org as ::core::ffi::c_int,
        );
        screen += 1;
    }
    free(reply as *mut ::core::ffi::c_void);
    if num_screens == 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: No screens found. Please fix your setup. i3 will exit now.\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_SUCCESS);
    }
}
unsafe fn use_root_output(conn_0: *mut xcb_connection_t) {
    let s: *mut Output = create_root_output(conn_0);
    (*s).active = true_0 != 0;
    (*s).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
    (*s).outputs.tqe_prev = outputs.tqh_last;
    *outputs.tqh_last = s as *mut xoutput;
    outputs.tqh_last = &raw mut (*s).outputs.tqe_next;
    output_init_con(s);
    init_ws_for_output(s);
}

pub unsafe fn xinerama_init() {
    if (*xcb_get_extension_data(conn, &raw mut xcb_xinerama_id)).present == 0 {
        debuglog(
            c"%s:%s:%d - Xinerama extension not found, using root output.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/xinerama.c".as_ptr(),
            c"xinerama_init".as_ptr(),
            111 as ::core::ffi::c_int,
        );
        use_root_output(conn);
    } else {
        let mut reply: *mut xcb_xinerama_is_active_reply_t = xcb_xinerama_is_active_reply(
            conn,
            xcb_xinerama_is_active(conn),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if reply.is_null() || (*reply).state == 0 {
            debuglog(
                c"%s:%s:%d - Xinerama is not active (in your X-Server), using root output.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"xinerama_init".as_ptr(),
                117 as ::core::ffi::c_int,
            );
            use_root_output(conn);
        } else {
            query_screens(conn);
        }
        free(reply as *mut ::core::ffi::c_void);
        reply = ::core::ptr::null_mut::<xcb_xinerama_is_active_reply_t>();
    };
}
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::min;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{
    xcb_connection_t, xcb_generic_error_t, xcb_get_extension_data, xcb_xinerama_id,
    xcb_xinerama_is_active, xcb_xinerama_is_active_reply, xcb_xinerama_is_active_reply_t,
    xcb_xinerama_query_screens_reply, xcb_xinerama_query_screens_reply_t,
    xcb_xinerama_query_screens_screen_info, xcb_xinerama_query_screens_screen_info_length,
    xcb_xinerama_query_screens_unchecked, xcb_xinerama_screen_info_t,
};

pub use libc::{exit, free};

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::output_name;
pub use crate::types::xoutput;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::output::output_primary_name;
use crate::randr::create_root_output;
use crate::randr::init_ws_for_output;
use crate::randr::output_init_con;
