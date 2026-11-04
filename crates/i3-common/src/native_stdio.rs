//! Native C runtime entry points absent from `libc`'s public Rust bindings.
//!
//! All FILE pointers use libc's opaque type. Variadic cursor declarations live
//! beside the native test oracles that need them.
use core::ffi::{c_char, c_int, c_uint, c_ushort};
use libc::FILE;

/// The two fixed-decimal precisions used by native i3 diagnostics.
#[derive(Clone, Copy, Debug)]
pub enum FixedPrecision {
    Zero,
    Six,
}

/// Preserve native rounding, signed NaNs and LC_NUMERIC without a variadic ABI
/// or a caller-supplied format. Installed stdlib.h declares strfromd as
/// int(char*, size_t, const char*, double); its only nonnull argument is format.
pub fn fixed_decimal(value: f64, precision: FixedPrecision) -> Vec<u8> {
    let format = match precision {
        FixedPrecision::Zero => c"%.0f",
        FixedPrecision::Six => c"%.6f",
    };
    let mut bytes = vec![0; 512];
    loop {
        // SAFETY: The initialized output has its advertised capacity, and the
        // static format accepts exactly the typed double argument.
        let needed = unsafe {
            strfromd(
                bytes.as_mut_ptr().cast(),
                bytes.len(),
                format.as_ptr(),
                value,
            )
        };
        let needed = usize::try_from(needed).expect("fixed strfromd format succeeds");
        if needed < bytes.len() {
            bytes.truncate(needed);
            return bytes;
        }
        bytes.resize(
            needed
                .checked_add(1)
                .expect("native decimal length fits usize"),
            0,
        );
    }
}

unsafe extern "C" {
    fn strfromd(buffer: *mut c_char, capacity: usize, format: *const c_char, value: f64) -> c_int;
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
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

#[cfg(test)]
mod fixed_decimal_tests {
    use super::*;

    #[link(name = "m")]
    unsafe extern "C" {
        fn fegetround() -> c_int;
        fn fesetround(direction: c_int) -> c_int;
    }
    struct Rounding(c_int);
    impl Drop for Rounding {
        fn drop(&mut self) {
            unsafe {
                fesetround(self.0);
            }
        }
    }
    struct Locale {
        old: libc::locale_t,
        owned: libc::locale_t,
    }
    impl Drop for Locale {
        fn drop(&mut self) {
            unsafe {
                libc::uselocale(self.old);
                libc::freelocale(self.owned);
            }
        }
    }

    #[test]
    fn fixed_decimals_match_native_printf_across_values_rounding_and_available_locales() {
        let _rounding = Rounding(unsafe { fegetround() });
        // /usr/include/bits/fenv.h: x86 rounding-control values. Other targets
        // still compare the caller's active rounding mode without guessing ABI.
        #[cfg(target_arch = "x86_64")]
        let modes = [0, 0x400, 0x800, 0xc00];
        #[cfg(not(target_arch = "x86_64"))]
        let modes = [_rounding.0];
        let mut values = vec![
            0.0,
            -0.0,
            0.5,
            -0.5,
            1.25,
            -1.25,
            1.9999995,
            f64::MIN_POSITIVE,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            f64::from_bits(0xfff8_0000_0000_0001),
        ];
        let mut bits = 1u64;
        for _ in 0..128 {
            bits = bits.wrapping_mul(6364136223846793005).wrapping_add(1);
            values.push(f64::from_bits(bits));
        }
        for name in [
            c"C",
            c"C.UTF-8",
            c"en_US.UTF-8",
            c"de_DE.UTF-8",
            c"fr_FR.UTF-8",
        ] {
            let owned = unsafe {
                libc::newlocale(libc::LC_NUMERIC_MASK, name.as_ptr(), std::ptr::null_mut())
            };
            if owned.is_null() {
                continue;
            }
            let _locale = Locale {
                old: unsafe { libc::uselocale(owned) },
                owned,
            };
            for &mode in &modes {
                assert_eq!(unsafe { fesetround(mode) }, 0);
                for &value in &values {
                    for (precision, format) in [
                        (FixedPrecision::Zero, c"%.0f"),
                        (FixedPrecision::Six, c"%.6f"),
                    ] {
                        let mut expected = vec![0u8; 1024];
                        let count = unsafe {
                            libc::snprintf(
                                expected.as_mut_ptr().cast(),
                                expected.len(),
                                format.as_ptr(),
                                value,
                            )
                        };
                        assert!(count >= 0 && (count as usize) < expected.len());
                        expected.truncate(count as usize);
                        assert_eq!(
                            fixed_decimal(value, precision),
                            expected,
                            "{value:?}, {precision:?}, mode {mode}, {name:?}"
                        );
                    }
                }
            }
        }
    }
}
