//! Narrow printf presentation rules retained by converted diagnostic callsites.
#![cfg_attr(not(test), deny(unsafe_code))]

use std::ffi::CStr;
use std::fmt;

/// Snapshot a pointer's address without borrowing or dereferencing its pointee.
/// Plain `%p` writes `(nil)` for null on the supported native libc.
pub fn pointer<T>(value: *const T) -> Pointer {
    Pointer(value as usize)
}

pub struct Pointer(usize);
impl fmt::Display for Pointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            formatter.write_str("(nil)")
        } else {
            write!(formatter, "0x{:x}", self.0)
        }
    }
}

/// Bytes selected by `%s`/`%.*s`, including glibc's null-string precision rule.
/// Precision counts bytes; `CStr` has already bounded the first NUL. Callers
/// snapshot these bytes before dispatch if a callback may release their owner.
pub fn string_bytes(value: Option<&CStr>, precision: Option<usize>) -> &[u8] {
    let bytes = match value {
        Some(value) => value.to_bytes(),
        None if precision.is_some_and(|precision| precision < 6) => b"",
        None => b"(null)",
    };
    &bytes[..precision.unwrap_or(bytes.len()).min(bytes.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_display_matches_native_plain_pointer_for_null_and_addresses() {
        for address in [0, 1, 0x1234, usize::MAX] {
            let pointer = address as *const libc::c_void;
            let mut output = [0u8; 64];
            let count = unsafe {
                libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    c"%p".as_ptr(),
                    pointer,
                )
            };
            assert!(count >= 0 && (count as usize) < output.len());
            assert_eq!(
                format!("{}", super::pointer(pointer)).as_bytes(),
                &output[..count as usize]
            );
        }
    }

    #[test]
    fn string_precision_matches_native_null_binary_and_multibyte_strings() {
        for value in [None, Some(c""), Some(c"a\xffbcdef"), Some(c"éclair")] {
            for precision in 0..10 {
                let mut output = [0u8; 64];
                let pointer = value.map_or(std::ptr::null(), CStr::as_ptr);
                let count = unsafe {
                    libc::snprintf(
                        output.as_mut_ptr().cast(),
                        output.len(),
                        c"%.*s".as_ptr(),
                        precision as i32,
                        pointer,
                    )
                };
                assert!(count >= 0 && (count as usize) < output.len());
                assert_eq!(
                    string_bytes(value, Some(precision)),
                    &output[..count as usize]
                );
            }
            let mut output = [0u8; 64];
            let pointer = value.map_or(std::ptr::null(), CStr::as_ptr);
            let count = unsafe {
                libc::snprintf(
                    output.as_mut_ptr().cast(),
                    output.len(),
                    c"%s".as_ptr(),
                    pointer,
                )
            };
            assert_eq!(string_bytes(value, None), &output[..count as usize]);
        }
    }
}
