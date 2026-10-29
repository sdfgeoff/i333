/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see LICENSE).
 */
//! Owned PCRE2 expressions through the maintained safe `pcre2` wrapper.
//!
//! Patterns retain upstream i3's PCRE2 UTF/UCP semantics, including native
//! lookarounds, backreferences, POSIX classes, anchors and recursion. No pattern
//! syntax is rewritten or approximated by a second regular-expression engine.

use crate::log::{errorlog, verboselog};
use pcre2::bytes::{Regex as CompiledRegex, RegexBuilder};
use std::ffi::{c_char, CStr, CString};
use std::fmt;
use std::ptr;

/// A compiled expression with Rust-owned matching and storage.
pub struct Regex {
    // Temporary legacy transport: matching files still read this C pointer.
    // The CString owns its storage; callers must not mutate or replace it.
    pub pattern: *mut c_char,
    state: Box<PatternState>,
}

struct PatternState {
    compiled: CompiledRegex,
    pattern: CString,
}

#[derive(Debug)]
pub struct CompileError {
    pub offset: usize,
    pub message: String,
    /// Native PCRE2 code, or None for an invalid Rust/C input representation.
    pub code: Option<i32>,
    source: Option<pcre2::Error>,
}
impl fmt::Display for CompileError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{} at byte {}", self.message, self.offset)
    }
}
impl std::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &dyn std::error::Error)
    }
}

/// Matching errors expose the native PCRE2 code, including malformed UTF-8.
pub type MatchError = pcre2::Error;

// pcre2 0.2.11's builder.ucp(true) also sets MATCH_INVALID_UTF. Upstream i3
// doesn't set that flag: malformed subjects must produce native UTF errors.
// A PCRE-native leading directive sets UCP without changing subject validation,
// capture numbering, matching instructions or the caller's pattern spelling.
const UCP_DIRECTIVE: &str = "(*UCP)";

impl Regex {
    pub fn compile(pattern: &CStr) -> Result<Self, CompileError> {
        Self::compile_bytes(pattern.to_bytes())
    }

    /// Compile UTF-8 pattern bytes, rejecting malformed text without replacement.
    pub fn compile_bytes(pattern: &[u8]) -> Result<Self, CompileError> {
        let text = std::str::from_utf8(pattern).map_err(|error| CompileError {
            offset: error.valid_up_to(),
            message: error.to_string(),
            code: None,
            source: None,
        })?;
        Self::compile_str(text)
    }

    /// Compile PCRE syntax with the same UTF/UCP defaults as upstream i3.
    /// Embedded NUL is rejected because remaining callers retain a C-string view.
    pub fn compile_str(pattern: &str) -> Result<Self, CompileError> {
        let owned_pattern = CString::new(pattern).map_err(|error| CompileError {
            offset: error.nul_position(),
            message: "regular expression contains NUL".to_owned(),
            code: None,
            source: None,
        })?;
        let effective = format!("{UCP_DIRECTIVE}{pattern}");
        // Leave JIT/newline/multiline/dotall settings at native defaults, as i3 did.
        let compiled = RegexBuilder::new()
            .utf(true)
            .build(&effective)
            .map_err(|error| {
                let native_offset = error.offset();
                let rendered = error.to_string();
                // The pinned safe wrapper exposes native code/offset; its Display
                // includes context. Strip that context for the old i3 log wording.
                let prefix = match native_offset {
                    Some(offset) => format!("PCRE2: error compiling pattern at offset {offset}: "),
                    None => "PCRE2: error compiling pattern: ".to_owned(),
                };
                let message = rendered
                    .strip_prefix(&prefix)
                    .unwrap_or(&rendered)
                    .to_owned();
                CompileError {
                    offset: native_offset
                        .unwrap_or(UCP_DIRECTIVE.len())
                        .saturating_sub(UCP_DIRECTIVE.len()),
                    message,
                    code: Some(error.code()),
                    source: Some(error),
                }
            })?;
        let state = Box::new(PatternState {
            compiled,
            pattern: owned_pattern,
        });
        let pattern = state.pattern.as_ptr().cast_mut();
        Ok(Self { pattern, state })
    }

    pub fn pattern(&self) -> &CStr {
        &self.state().pattern
    }
    pub fn pattern_str(&self) -> &str {
        self.pattern().to_str().expect("compiled UTF-8 pattern")
    }

    fn state(&self) -> &PatternState {
        &self.state
    }

    pub fn is_match(&self, input: &CStr) -> Result<bool, MatchError> {
        self.is_match_bytes(input.to_bytes())
    }
    pub fn is_match_str(&self, input: &str) -> Result<bool, MatchError> {
        self.is_match_bytes(input.as_bytes())
    }
    /// Retain native UTF validation and error codes. Valid UTF-8 containing NUL
    /// can be matched through this owned byte API; the legacy C view stops at NUL.
    pub fn is_match_bytes(&self, input: &[u8]) -> Result<bool, MatchError> {
        self.state().compiled.is_match(input)
    }
}
/// Transitional pointer entry point, using Rust linkage.
/// # Safety
/// pattern must point to a live NUL-terminated string.
pub unsafe fn regex_new(pattern: *const c_char) -> *mut Regex {
    // SAFETY: The caller supplies a borrowed C string for this call.
    match Regex::compile(unsafe { CStr::from_ptr(pattern) }) {
        Ok(regex) => Box::into_raw(Box::new(regex)),
        Err(error) => {
            let message = CString::new(error.message).expect("regex diagnostic contains no NUL");
            unsafe {
                errorlog(
                    c"ERROR: PCRE regular expression compilation failed at %zu: %s\n"
                        .as_ptr()
                        .cast_mut(),
                    error.offset,
                    message.as_ptr(),
                )
            };
            ptr::null_mut()
        }
    }
}

/// # Safety
/// regex is null or is a still-owned pointer returned by regex_new.
pub unsafe fn regex_free(regex: *mut Regex) {
    if !regex.is_null() {
        unsafe { drop(Box::from_raw(regex)) };
    }
}

/// # Safety
/// regex and input must be live borrowed pointers from the corresponding APIs.
pub unsafe fn regex_matches(regex: *const Regex, input: *const c_char) -> bool {
    // SAFETY: The caller supplies a live expression and C string for this call.
    let expression = unsafe { &*regex };
    match expression.is_match(unsafe { CStr::from_ptr(input) }) {
        Ok(matched) => {
            let verb = if matched {
                c"matches"
            } else {
                c"does not match"
            };
            unsafe {
                verboselog(
                    c"Regular expression \"%s\" %s \"%s\"\n".as_ptr().cast_mut(),
                    expression.pattern,
                    verb.as_ptr(),
                    input,
                )
            };
            matched
        }
        Err(error) => {
            unsafe {
                errorlog(c"ERROR: PCRE error %d while trying to use regular expression \"%s\" on input \"%s\", see pcreapi(3)\n".as_ptr().cast_mut(),
                error.code(), expression.pattern, input)
            };
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expression(pattern: &str) -> Regex {
        Regex::compile_str(pattern).unwrap()
    }

    #[test]
    fn unicode_lookarounds_backreferences_and_case_flags_are_native() {
        let regex = expression(r"(?<=prefix-)\w+$");
        assert!(regex.is_match_str("prefix-café").unwrap());
        assert!(!regex.is_match_str("other-café").unwrap());
        assert_eq!(regex.pattern_str(), r"(?<=prefix-)\w+$");
        let regex = expression(r"(?i)^(foo)-\1$");
        assert!(regex.is_match_str("Foo-foo").unwrap());
        assert!(!regex.is_match_str("foo-bar").unwrap());
    }

    #[test]
    fn posix_classes_and_literal_class_dollars_do_not_become_anchors() {
        let regex = expression(r"^[[:alnum:]$]+$");
        for input in ["café", "α", "$", "١", "漢字"] {
            assert!(regex.is_match_str(input).unwrap(), "{input}");
        }
        for input in ["foo\n\n", "foo\nbar", "|", " "] {
            assert!(!regex.is_match_str(input).unwrap(), "{input:?}");
        }
        assert!(expression(r"^[[:^digit:]$]+$").is_match_str("α$").unwrap());
        assert!(!expression(r"^[[:^digit:]$]+$").is_match_str("١").unwrap());
        assert!(expression(r"^[[]$").is_match_str("[").unwrap());
        assert!(expression(r"^[a&&b]+$").is_match_str("&ab").unwrap());
    }

    #[test]
    fn final_newline_multiline_and_explicit_newline_conventions_are_preserved() {
        for pattern in [r"^foo$", r"^foo\Z"] {
            let regex = expression(pattern);
            assert!(regex.is_match_str("foo\n").unwrap());
            assert!(!regex.is_match_str("foo\n\n").unwrap());
        }
        assert!(!expression(r"^foo\z").is_match_str("foo\n").unwrap());
        assert!(expression(r"(?m)^foo$")
            .is_match_str("bar\nfoo\nend")
            .unwrap());
        assert!(expression(r"(*CRLF)^foo$").is_match_str("foo\r\n").unwrap());
        assert!(expression(r"^[$]$").is_match_str("$").unwrap());
        assert!(expression(r"^\$$").is_match_str("$").unwrap());
    }

    #[test]
    fn comments_and_quoted_literals_keep_dollars_and_brackets_literal() {
        assert!(expression(r"(?#comment [$)^foo$")
            .is_match_str("foo")
            .unwrap());
        assert!(expression("(?x)^foo$ # a comment [$\n")
            .is_match_str("foo")
            .unwrap());
        assert!(expression(r"^\Qfoo$[]\E$").is_match_str("foo$[]").unwrap());
        assert!(expression(r"(?x)^a(?-x:#)$").is_match_str("a#").unwrap());
    }

    #[test]
    fn pcre_horizontal_vertical_and_grapheme_escapes_keep_their_meaning() {
        let horizontal = expression(r"^\h+$");
        assert!(horizontal
            .is_match_str(" \t\u{a0}\u{180e}\u{3000}")
            .unwrap());
        assert!(!horizontal.is_match_str("f").unwrap());
        assert!(!horizontal.is_match_str("\n").unwrap());
        assert!(expression(r"^\v+$")
            .is_match_str("\n\r\u{85}\u{2028}")
            .unwrap());
        assert!(expression(r"^\V+$").is_match_str("abc\t").unwrap());
        assert!(expression(r"^\X$").is_match_str("a\u{301}").unwrap());
        assert!(!expression(r"^\w+$").is_match_str("\u{200d}").unwrap());
    }

    #[test]
    fn reset_recursion_backreference_spelling_and_branch_reset_remain_supported() {
        assert!(expression(r"foo\Kbar").is_match_str("foobar").unwrap());
        assert!(expression(r"^(?|a(b)|c(d))\1$")
            .is_match_str("abb")
            .unwrap());
        assert!(expression(r"^(?|a(b)|c(d))\1$")
            .is_match_str("cdd")
            .unwrap());
        let recursive = expression(r"^(?<nested>\((?:[^()]++|(?&nested))*\))$");
        assert!(recursive.is_match_str("((abc)(def))").unwrap());
        assert!(!recursive.is_match_str("((abc)").unwrap());
        assert!(expression(r"^(a|b)\g1$").is_match_str("aa").unwrap());
        assert!(!expression(r"^(a|b)\g1$").is_match_str("ab").unwrap());
        assert!(expression(r"^(a|b)\g{1}$").is_match_str("bb").unwrap());
    }

    #[test]
    fn byte_matching_retains_native_utf_errors_and_supports_pcre_byte_escape() {
        let regex = expression(".");
        let error = regex.is_match_bytes(b"\xff").unwrap_err();
        assert!(error.code() < -2);
        assert!(error.offset().is_none());
        assert!(regex.is_match_bytes(b"A\xff").is_err());
        assert!(expression(r"^\C$").is_match_str("A").unwrap());
        assert!(!expression(r"^\C$").is_match_str("é").unwrap());
        assert!(expression(r"^\x{00}$").is_match_bytes(b"\0").unwrap());
    }

    #[test]
    fn errors_name_original_pattern_offsets_and_do_not_repair_input() {
        let error = Regex::compile_str("(").err().unwrap();
        assert_eq!(error.offset, 1);
        assert_eq!(error.message, "missing closing parenthesis");
        assert!(error.code.is_some());
        let error = Regex::compile_str("é(").err().unwrap();
        assert_eq!(error.offset, 3);
        let error = Regex::compile_bytes(b"a\xff").err().unwrap();
        assert_eq!(error.offset, 1);
        assert!(error.code.is_none());
        assert!(Regex::compile_str("a\0b").is_err());
    }
}
