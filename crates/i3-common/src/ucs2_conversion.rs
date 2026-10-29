// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::fmt;

/// X core fonts use big-endian UCS-2 code units, not UTF-16 surrogate pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Ucs2Char {
    pub byte1: u8,
    pub byte2: u8,
}
impl From<u16> for Ucs2Char {
    fn from(value: u16) -> Self {
        let [byte1, byte2] = value.to_be_bytes();
        Self { byte1, byte2 }
    }
}
impl Ucs2Char {
    pub fn code_unit(self) -> u16 {
        u16::from_be_bytes([self.byte1, self.byte2])
    }
}
pub type xcb_char2b_t = Ucs2Char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionIssue {
    InvalidSequence,
    IncompleteSequence,
}
impl fmt::Display for ConversionIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidSequence => "invalid or unrepresentable UCS-2 character",
            Self::IncompleteSequence => "incomplete UTF-8 sequence",
        })
    }
}
impl std::error::Error for ConversionIssue {}

/// GNU iconv's //IGNORE behavior, or the historical strict fallback policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingPolicy {
    Ignore,
    Strict,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedUcs2 {
    pub glyphs: Vec<Ucs2Char>,
    pub issue: Option<ConversionIssue>,
}

/// Encode without UTF-16 surrogate substitution. Ignore skips invalid UTF-8 and
/// non-BMP scalars and continues; incomplete tails stop conversion. Strict stops
/// at the first invalid/unrepresentable scalar. Both retain the converted prefix.
pub fn encode_ucs2(input: &[u8], policy: EncodingPolicy) -> EncodedUcs2 {
    let mut result = EncodedUcs2 {
        glyphs: Vec::new(),
        issue: None,
    };
    let mut remaining = input;
    while !remaining.is_empty() {
        let (valid, error) = match std::str::from_utf8(remaining) {
            Ok(valid) => (valid, None),
            Err(error) => (
                std::str::from_utf8(&remaining[..error.valid_up_to()]).unwrap(),
                Some(error),
            ),
        };
        for scalar in valid.chars() {
            if let Ok(code_unit) = u16::try_from(scalar as u32) {
                result.glyphs.push(code_unit.into());
            } else {
                result.issue = Some(ConversionIssue::InvalidSequence);
                if policy == EncodingPolicy::Strict {
                    return result;
                }
            }
        }
        let Some(error) = error else {
            break;
        };
        match error.error_len() {
            None => {
                result.issue = Some(ConversionIssue::IncompleteSequence);
                break;
            }
            Some(length) => {
                result.issue = Some(ConversionIssue::InvalidSequence);
                if policy == EncodingPolicy::Strict {
                    break;
                }
                remaining = &remaining[error.valid_up_to() + length..];
            }
        }
    }
    result
}

/// Decode UCS-2 strictly. Any surrogate code unit rejects the whole conversion;
/// adjacent surrogate units must never be combined into a supplementary scalar.
pub fn decode_ucs2(input: &[Ucs2Char]) -> Result<String, ConversionIssue> {
    input
        .iter()
        .map(|glyph| {
            char::from_u32(glyph.code_unit() as u32).ok_or(ConversionIssue::InvalidSequence)
        })
        .collect()
}

/// Remaining malloc/free and errno adapters. Conversion itself is pure Rust.
pub(crate) mod legacy {
    use super::*;
    use std::ffi::CStr;

    pub(crate) fn report(issue: ConversionIssue, message: &CStr) {
        // SAFETY: Static/borrowed C text is valid; errno is thread-local on this
        // Linux target. These diagnostics preserve the old iconv error wording.
        unsafe {
            *libc::__errno_location() = match issue {
                ConversionIssue::InvalidSequence => libc::EILSEQ,
                ConversionIssue::IncompleteSequence => libc::EINVAL,
            };
            libc::perror(message.as_ptr());
        }
    }

    /// # Safety
    /// `text` contains `count` readable UCS-2 units, or may be null for count zero.

    pub unsafe fn convert_ucs2_to_utf8(text: *mut Ucs2Char, count: usize) -> *mut libc::c_char {
        let glyphs = if count == 0 {
            &[]
        } else {
            // SAFETY: The caller supplies count readable units.
            unsafe { std::slice::from_raw_parts(text, count) }
        };
        match decode_ucs2(glyphs) {
            Err(issue) => {
                report(issue, c"Converting to UTF-8 failed");
                std::ptr::null_mut()
            }
            Ok(text) => {
                let bytes = text.as_bytes();
                // SAFETY: Preserve all encoded bytes, including data after a NUL
                // code unit, and append the malloc-owned C terminator.
                unsafe {
                    let result = crate::safewrappers::scalloc(bytes.len() + 1, 1).cast::<u8>();
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
                    result.cast()
                }
            }
        }
    }

    /// # Safety
    /// `input` is a live C string; `length` is null or writable. Returned storage
    /// belongs to free(). An empty conversion returns null and length zero.

    pub unsafe fn convert_utf8_to_ucs2(
        input: *mut libc::c_char,
        length: *mut usize,
    ) -> *mut Ucs2Char {
        // SAFETY: The caller supplies a valid C string.
        let input = unsafe { CStr::from_ptr(input) };
        // Linux's original preferred backend is UCS-2BE//IGNORE. Pure Rust needs
        // no charset-open fallback; Strict remains available as an explicit policy.
        let encoded = encode_ucs2(input.to_bytes(), EncodingPolicy::Ignore);
        if let Some(issue) = encoded.issue {
            report(issue, c"Converting to UCS-2 failed");
        }
        // SAFETY: Output is writable when supplied; preserve the original empty
        // realloc(0) result, and malloc ownership for nonempty converted buffers.
        unsafe {
            if !length.is_null() {
                *length = encoded.glyphs.len();
            }
            if encoded.glyphs.is_empty() {
                return std::ptr::null_mut();
            }
            let output =
                crate::safewrappers::smalloc(std::mem::size_of_val(encoded.glyphs.as_slice()))
                    .cast::<Ucs2Char>();
            std::ptr::copy_nonoverlapping(encoded.glyphs.as_ptr(), output, encoded.glyphs.len());
            output
        }
    }
}
pub use legacy::{convert_ucs2_to_utf8, convert_utf8_to_ucs2};
