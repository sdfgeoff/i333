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

/// Private native errno/perror reporting; conversion itself is pure Rust.
pub(crate) mod diagnostic {
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
}
