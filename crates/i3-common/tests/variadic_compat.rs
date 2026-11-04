#![feature(c_variadic)]
#![allow(
    dead_code,
    unused_mut,
    unused_assignments,
    non_camel_case_types,
    non_upper_case_globals,
    stable_features
)]
use i3_common::i333_va_list;
use libc::free;
unsafe extern "C" {
    fn vasprintf(
        buffer: *mut *mut core::ffi::c_char,
        format: *const core::ffi::c_char,
        arguments: core::ffi::VaList,
    ) -> core::ffi::c_int;
}

unsafe extern "C" fn copy_twice(format: *const core::ffi::c_char, mut args: ...) {
    let mut cursor = args.clone();
    let mut first = core::ptr::null_mut();
    let count = vasprintf(&mut first, format, i333_va_list!(cursor));
    assert!(count > 0);
    // Reinitialize from the original va_start cursor, exactly as errorlog does
    // for stdout and its separate error file. Both outputs must contain all args.
    cursor = args.clone();
    let mut second = core::ptr::null_mut();
    assert_eq!(vasprintf(&mut second, format, i333_va_list!(cursor)), count);
    let expected = b"1:2:3:4:5:6:7:8:hello:3.25";
    assert_eq!(std::ffi::CStr::from_ptr(first).to_bytes(), expected);
    assert_eq!(std::ffi::CStr::from_ptr(second).to_bytes(), expected);
    free(first.cast());
    free(second.cast());
}

fn main() {
    // SAFETY: Formats match the promoted C argument types; buffers are allocated
    // by vasprintf and released once with libc free after string inspection.
    unsafe {
        let format = c"%d:%d:%d:%d:%d:%d:%d:%d:%s:%.2f";
        copy_twice(
            format.as_ptr(),
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            c"hello".as_ptr(),
            3.25f64,
        );
    }
    println!("variadic formatting and independent cursor checks passed");
}
