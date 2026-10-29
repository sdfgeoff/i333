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

pub type Output = xoutput;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut num_screens: ::core::ffi::c_int = 0;
unsafe fn get_screen_at(x: ::core::ffi::c_uint, y: ::core::ffi::c_uint) -> *mut Output {
    let mut output: *mut Output = ::core::ptr::null_mut::<Output>();
    output = outputs.tqh_first as *mut Output;
    while !output.is_null() {
        if (*output).rect.x == x as u32 && (*output).rect.y == y as u32 {
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
            ) as u32;
            (*new_output).rect.height = min(
                (*new_output).rect.height as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
            ) as u32;
        } else {
            let output_name: *mut output_name =
                scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize)
                    as *mut output_name;
            new_output =
                scalloc(1 as usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
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
            (*new_output).rect.x = x as u32;
            (*new_output).rect.y = y as u32;
            (*new_output).rect.width = width as u32;
            (*new_output).rect.height = height as u32;
            if (*new_output).rect.x == 0 as u32 && (*new_output).rect.y == 0 as u32 {
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
