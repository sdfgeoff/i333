use crate::randr::outputs;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * Faking outputs is useful in pathological situations (like network X servers
 * which don’t support multi-monitor in a useful way) and for our testsuite.
 *
 */
// Mechanical file-by-file Rust translation of src/fake_outputs.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;

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
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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

pub unsafe fn fake_outputs_init(output_spec: *const ::core::ffi::c_char) {
    let mut walk: *const ::core::ffi::c_char = output_spec;
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_uint = 0;
    let mut width: ::core::ffi::c_uint = 0;
    let mut height: ::core::ffi::c_uint = 0;
    let mut chars_consumed: ::core::ffi::c_int = 0;
    while sscanf(
        walk,
        c"%ux%u+%u+%u%n".as_ptr(),
        &raw mut width,
        &raw mut height,
        &raw mut x,
        &raw mut y,
        &raw mut chars_consumed,
    ) == 4 as ::core::ffi::c_int
    {
        walk = walk.offset(chars_consumed as isize);
        let mut primary: bool = false_0 != 0;
        if *walk as ::core::ffi::c_int == 'P' as i32 {
            primary = true_0 != 0;
            walk = walk.offset(1);
        }
        if *walk as ::core::ffi::c_int == ',' as i32 {
            walk = walk.offset(1);
        }
        debuglog(
            c"%s:%s:%d - Parsed output as width = %u, height = %u at (%u, %u)%s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/fake_outputs.c".as_ptr(),
            c"fake_outputs_init".as_ptr(),
            53 as ::core::ffi::c_int,
            width,
            height,
            x,
            y,
            if primary as ::core::ffi::c_int != 0 {
                c" (primary)".as_ptr()
            } else {
                c"".as_ptr()
            },
        );
        let mut new_output: *mut Output = get_screen_at(x, y);
        if !new_output.is_null() {
            debuglog(
                c"%s:%s:%d - Re-used old output %p\n".as_ptr().cast_mut(),
                c"../src/fake_outputs.c".as_ptr(),
                c"fake_outputs_init".as_ptr(),
                57 as ::core::ffi::c_int,
                new_output,
            );
            (*new_output).rect.width = min(
                (*new_output).rect.width as ::core::ffi::c_int,
                width as ::core::ffi::c_int,
            ) as uint32_t;
            (*new_output).rect.height = min(
                (*new_output).rect.height as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            ) as uint32_t;
        } else {
            let output_name: *mut output_name =
                scalloc(1 as size_t, ::core::mem::size_of::<output_name>() as size_t)
                    as *mut output_name;
            new_output =
                scalloc(1 as size_t, ::core::mem::size_of::<Output>() as size_t) as *mut Output;
            sasprintf(
                &raw mut (*output_name).name,
                c"fake-%d".as_ptr(),
                num_screens,
            );
            (*new_output).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
            (*output_name).names.sle_next = (*new_output).names_head.slh_first;
            (*new_output).names_head.slh_first = output_name;
            debuglog(
                c"%s:%s:%d - Created new fake output %s (%p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/fake_outputs.c".as_ptr(),
                c"fake_outputs_init".as_ptr(),
                68 as ::core::ffi::c_int,
                output_primary_name(new_output),
                new_output,
            );
            (*new_output).active = true_0 != 0;
            (*new_output).rect.x = x as uint32_t;
            (*new_output).rect.y = y as uint32_t;
            (*new_output).rect.width = width as uint32_t;
            (*new_output).rect.height = height as uint32_t;
            if (*new_output).rect.x == 0 as uint32_t && (*new_output).rect.y == 0 as uint32_t {
                (*new_output).outputs.tqe_next = outputs.tqh_first;
                if !(*new_output).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*new_output).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*new_output).outputs.tqe_next;
                }
                outputs.tqh_first = new_output as *mut xoutput;
                (*new_output).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*new_output).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*new_output).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = new_output as *mut xoutput;
                outputs.tqh_last = &raw mut (*new_output).outputs.tqe_next;
            }
            output_init_con(new_output);
            init_ws_for_output(new_output);
            num_screens += 1;
        }
        (*new_output).primary = primary;
    }
    if num_screens == 0 as ::core::ffi::c_int {
        errorlog(
            c"ERROR: No screens found. Please fix your setup. i3 will exit now.\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_FAILURE);
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::min;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{exit, sscanf};

use crate::log::{debuglog, errorlog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::output_name;
pub use crate::types::xoutput;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::output::output_primary_name;
use crate::randr::init_ws_for_output;
use crate::randr::output_init_con;
