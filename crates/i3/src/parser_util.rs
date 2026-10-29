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
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sasprintf(
        strp: *mut *mut ::core::ffi::c_char,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_entry {
    pub identifier: *const ::core::ffi::c_char,
    pub type_0: C2RustUnnamed_0,
    pub val: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_long,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const STACK_LONG: C2RustUnnamed_0 = 1;
pub const STACK_STR: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub stack: [stack_entry; 10],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn parser_push_string(
    mut stack: *mut stack,
    mut identifier: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
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
                let mut prev: *mut ::core::ffi::c_char = (*stack).stack[c as usize].val.str_0;
                sasprintf(
                    &raw mut (*(&raw mut (*stack).stack as *mut stack_entry).offset(c as isize))
                        .val
                        .str_0,
                    b"%s,%s\0" as *const u8 as *const ::core::ffi::c_char,
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
        b"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn parser_push_long(
    mut stack: *mut stack,
    mut identifier: *const ::core::ffi::c_char,
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
        b"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C" fn parser_get_string(
    mut stack: *const stack,
    mut identifier: *const ::core::ffi::c_char,
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
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn parser_get_long(
    mut stack: *const stack,
    mut identifier: *const ::core::ffi::c_char,
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
    return 0 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn parser_clear_stack(mut stack: *mut stack) {
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
