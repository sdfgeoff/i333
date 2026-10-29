use crate::bindings::get_binding_from_xcb_event;
use crate::bindings::run_binding;
use crate::commands_parser::command_result_free;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * key_press.c: key press handler
 *
 */
// Mechanical file-by-file Rust translation of src/key_press.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type i3String = _i3String;

pub use crate::application::Binding;
pub type i3_event_state_mask_t = u32;

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
pub use crate::assignments::CommandResult;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn handle_key_press(event: *mut xcb_key_press_event_t) {
    let key_release: bool = (*event).response_type as ::core::ffi::c_int == XCB_KEY_RELEASE;
    last_timestamp = (*event).time;
    debuglog(
        c"%s:%s:%d - %s %d, state raw = 0x%x\n".as_ptr().cast_mut(),
        c"../src/key_press.c".as_ptr(),
        c"handle_key_press".as_ptr(),
        23 as ::core::ffi::c_int,
        if key_release as ::core::ffi::c_int != 0 {
            c"KeyRelease".as_ptr()
        } else {
            c"KeyPress".as_ptr()
        },
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let bind: *mut Binding = get_binding_from_xcb_event(event as *mut xcb_generic_event_t);
    if bind.is_null() {
        return;
    }
    let result: *mut CommandResult = run_binding(bind, ::core::ptr::null_mut::<Con>());
    command_result_free(result);
}

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{xcb_generic_event_t, xcb_key_press_event_t, XCB_KEY_RELEASE};

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::last_timestamp;
