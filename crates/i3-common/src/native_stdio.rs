//! Native C runtime entry points absent from `libc`'s public Rust bindings.
//!
//! All FILE pointers use libc's opaque type. VaList arguments deliberately use
//! Rust's compiler-provided ABI; forwarding sites use `i333_va_list!` across
//! the old borrowed and newer owned VaList compiler versions.
use core::ffi::{c_char, c_int, c_uint, c_ushort, VaList};
use libc::FILE;

unsafe extern "C" {
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub fn vfprintf(stream: *mut FILE, format: *const c_char, arguments: VaList) -> c_int;
    pub fn vprintf(format: *const c_char, arguments: VaList) -> c_int;
    pub fn vsnprintf(
        buffer: *mut c_char,
        capacity: usize,
        format: *const c_char,
        arguments: VaList,
    ) -> c_int;
    pub fn vasprintf(buffer: *mut *mut c_char, format: *const c_char, arguments: VaList) -> c_int;
    pub fn err(status: c_int, format: *const c_char, ...) -> !;
    pub fn errx(status: c_int, format: *const c_char, ...) -> !;
    pub fn warn(format: *const c_char, ...);
    pub fn __ctype_b_loc() -> *mut *const c_ushort;
    pub fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: usize,
    pub we_wordv: *mut *mut ::core::ffi::c_char,
    pub we_offs: usize,
}

unsafe extern "C" {
    pub fn wordexp(words: *const c_char, result: *mut wordexp_t, flags: c_int) -> c_int;
    pub fn wordfree(result: *mut wordexp_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn native_word_expansion_preserves_quoted_words() {
        unsafe {
            let mut result: wordexp_t = std::mem::zeroed();
            // WRDE_NOCMD rejects command substitution; this test has no shell commands.
            assert_eq!(
                wordexp(c"i3-native-wordexp 'two words'".as_ptr(), &mut result, 4),
                0
            );
            assert_eq!(result.we_wordc, 2);
            assert_eq!(
                CStr::from_ptr(*result.we_wordv).to_bytes(),
                b"i3-native-wordexp"
            );
            assert_eq!(
                CStr::from_ptr(*result.we_wordv.add(1)).to_bytes(),
                b"two words"
            );
            wordfree(&mut result);
        }
    }
}

// glibc's x86_64 character-class table stores network-order flag bits.
// libc exposes __ctype_b_loc through this module, but not these private masks.
pub const CTYPE_SPACE: c_ushort = 0x2000;
pub const CTYPE_DIGIT: c_ushort = 0x0800;
