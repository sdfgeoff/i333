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

pub use crate::application::Binding;
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_1 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_1 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_1 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;

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
