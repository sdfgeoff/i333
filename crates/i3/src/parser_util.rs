use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * parser_util.c: utility functions for the config and commands parser
 *
 */
// Mechanical file-by-file Rust translation of src/parser_util.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;

pub use crate::commands::stack_entry;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_0 = 1;
pub const STACK_STR: C2RustUnnamed_0 = 0;
pub use crate::commands::stack;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn parser_push_string(
    stack: *mut stack,
    identifier: *const ::core::ffi::c_char,
    str: *const ::core::ffi::c_char,
) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !(*stack).stack[c as usize].identifier.is_null()
            && strcmp((*stack).stack[c as usize].identifier, identifier) != 0 as ::core::ffi::c_int
        {
            c += 1;
        } else {
            if (*stack).stack[c as usize].identifier.is_null() {
                (*stack).stack[c as usize].identifier = identifier;
                (*stack).stack[c as usize].val.str_0 = sstrdup(str);
                (*stack).stack[c as usize].type_0 = STACK_STR;
            } else {
                let prev: *mut ::core::ffi::c_char = (*stack).stack[c as usize].val.str_0;
                sasprintf(
                    &raw mut (*(&raw mut (*stack).stack as *mut stack_entry).offset(c as isize))
                        .val
                        .str_0,
                    c"%s,%s".as_ptr(),
                    prev,
                    str,
                );
                free(prev as *mut ::core::ffi::c_void);
            }
            return;
        }
    }
    fprintf(
        stderr,
        c"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr(),
    );
    exit(EXIT_FAILURE);
}

pub unsafe fn parser_push_long(
    stack: *mut stack,
    identifier: *const ::core::ffi::c_char,
    num: ::core::ffi::c_long,
) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if !(*stack).stack[c as usize].identifier.is_null() {
            c += 1;
        } else {
            (*stack).stack[c as usize].identifier = identifier;
            (*stack).stack[c as usize].val.num = num;
            (*stack).stack[c as usize].type_0 = STACK_LONG;
            return;
        }
    }
    fprintf(
        stderr,
        c"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr(),
    );
    exit(EXIT_FAILURE);
}

pub unsafe fn parser_get_string(
    stack: *const stack,
    identifier: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if (*stack).stack[c as usize].identifier.is_null() {
            break;
        }
        if strcmp(identifier, (*stack).stack[c as usize].identifier) == 0 as ::core::ffi::c_int {
            return (*stack).stack[c as usize].val.str_0;
        }
        c += 1;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}

pub unsafe fn parser_get_long(
    stack: *const stack,
    identifier: *const ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if (*stack).stack[c as usize].identifier.is_null() {
            break;
        }
        if strcmp(identifier, (*stack).stack[c as usize].identifier) == 0 as ::core::ffi::c_int {
            return (*stack).stack[c as usize].val.num;
        }
        c += 1;
    }
    0 as ::core::ffi::c_long
}

pub unsafe fn parser_clear_stack(stack: *mut stack) {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < 10 as ::core::ffi::c_int {
        if (*stack).stack[c as usize].type_0 as ::core::ffi::c_uint
            == STACK_STR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            free((*stack).stack[c as usize].val.str_0 as *mut ::core::ffi::c_void);
        }
        (*stack).stack[c as usize].identifier = ::core::ptr::null::<::core::ffi::c_char>();
        (*stack).stack[c as usize].val.str_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*stack).stack[c as usize].val.num = 0 as ::core::ffi::c_long;
        c += 1;
    }
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::sstrdup;

pub use libc::{exit, fprintf, free, strcmp};

use i3_common::native_stdio::stderr;
