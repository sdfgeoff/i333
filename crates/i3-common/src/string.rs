// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use crate::ucs2_conversion::{
    decode_ucs2, encode_ucs2, ConversionIssue, EncodedUcs2, EncodingPolicy, Ucs2Char,
};
use std::cell::{Cell, OnceCell};
use std::ffi::{CStr, CString};

/// Owned UI text with lazy X core-font encoding and a stable borrowed C view.
/// Rendering borrows this value; its cached native encodings live until it drops.
#[derive(Debug, Clone)]
pub struct I3String {
    utf8: OnceCell<CString>,
    ucs2: OnceCell<EncodedUcs2>,
    markup: Cell<bool>,
    legacy_num_bytes: Cell<usize>,
    originated_as_ucs2: bool,
}

impl I3String {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self::from_bytes(text.as_ref().as_bytes())
    }
    /// Replace each invalid byte and embedded NUL with U+FFFD, matching GLib's
    /// make-valid behavior rather than grouping incomplete bytes as one error.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let text = make_valid(bytes);
        let count = text.len();
        Self {
            utf8: OnceCell::from(CString::new(text).expect("make_valid replaces NUL")),
            ucs2: OnceCell::new(),
            markup: Cell::new(false),
            legacy_num_bytes: Cell::new(count),
            originated_as_ucs2: false,
        }
    }
    /// Preserve the historical explicit-length font view while owning repaired
    /// text. Legacy rendering asks for the input byte count, even when replacing
    /// invalid bytes or NUL expands the stored UTF-8 buffer. The caller cannot
    /// supply an arbitrary length: repair never makes this view exceed storage.
    pub fn from_bytes_with_legacy_length(bytes: &[u8]) -> Self {
        let text = Self::from_bytes(bytes);
        text.legacy_num_bytes.set(bytes.len());
        text
    }

    pub fn from_cstr(text: &CStr) -> Self {
        Self::from_bytes(text.to_bytes())
    }
    pub fn from_ucs2(glyphs: &[Ucs2Char]) -> Self {
        Self {
            utf8: OnceCell::new(),
            ucs2: OnceCell::from(EncodedUcs2 {
                glyphs: glyphs.to_vec(),
                issue: None,
            }),
            markup: Cell::new(false),
            legacy_num_bytes: Cell::new(0),
            originated_as_ucs2: true,
        }
    }
    pub fn as_cstr(&self) -> Result<&CStr, ConversionIssue> {
        if self.utf8.get().is_none() {
            let text = decode_ucs2(&self.ucs2.get().expect("one source encoding exists").glyphs)?;
            // A C string's visible text ends at its first NUL, exactly as the
            // original strlen-based projection did. The UCS-2 cache stays intact.
            let prefix = text.split('\0').next().unwrap();
            self.legacy_num_bytes.set(prefix.len());
            let _ = self.utf8.set(CString::new(prefix).unwrap());
        }
        Ok(self.utf8.get().unwrap().as_c_str())
    }
    pub fn as_str(&self) -> Result<&str, ConversionIssue> {
        Ok(self
            .as_cstr()?
            .to_str()
            .expect("stored text is valid UTF-8"))
    }
    pub fn byte_len(&self) -> Result<usize, ConversionIssue> {
        Ok(self.as_cstr()?.to_bytes().len())
    }
    pub fn as_ucs2(&self) -> &[Ucs2Char] {
        &self
            .ucs2
            .get_or_init(|| {
                encode_ucs2(
                    self.utf8
                        .get()
                        .expect("one source encoding exists")
                        .as_bytes(),
                    EncodingPolicy::Ignore,
                )
            })
            .glyphs
    }
    pub fn glyph_count(&self) -> usize {
        self.as_ucs2().len()
    }
    pub fn is_markup(&self) -> bool {
        self.markup.get()
    }
    pub fn set_markup(&self, markup: bool) {
        self.markup.set(markup);
    }
    /// The UTF-8 view used by legacy native rendering and diagnostic adapters.
    /// Conversion failures retain the original diagnostic and null-view policy.
    pub fn cstr_or_report(&self) -> Option<&CStr> {
        match self.as_cstr() {
            Ok(value) => Some(value),
            Err(issue) => {
                crate::ucs2_conversion::legacy::report(issue, c"Converting to UTF-8 failed");
                None
            }
        }
    }

    /// Preserve explicit-input byte lengths, including repaired invalid bytes.
    pub fn rendering_byte_len(&self) -> usize {
        let _ = self.cstr_or_report();
        self.legacy_num_bytes.get()
    }

    /// Borrow the core-font glyph cache while preserving conversion diagnostics.
    pub fn rendering_glyphs(&self) -> &[Ucs2Char] {
        let conversion_needed = self.ucs2.get().is_none()
            || (!self.originated_as_ucs2
                && self
                    .ucs2
                    .get()
                    .is_some_and(|encoded| encoded.glyphs.is_empty()));
        let glyphs = self.as_ucs2();
        if conversion_needed {
            if let Some(issue) = self.ucs2.get().unwrap().issue {
                crate::ucs2_conversion::legacy::report(issue, c"Converting to UCS-2 failed");
            }
        }
        glyphs
    }

    /// An independent UTF-8 copy, matching the old copy operation rather than
    /// cloning a potentially different UCS-2 source/cache or legacy byte count.
    pub fn copy_utf8(&self) -> Option<Self> {
        let copy = Self::from_cstr(self.cstr_or_report()?);
        copy.set_markup(self.is_markup());
        Some(copy)
    }

    pub fn escaped_markup(&self) -> Result<Self, ConversionIssue> {
        Ok(Self::new(escape_markup(self.as_str()?)))
    }
}

fn make_valid(mut bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len());
    while !bytes.is_empty() {
        match std::str::from_utf8(bytes) {
            Ok(valid) => {
                append_without_nul(&mut output, valid);
                break;
            }
            Err(error) => {
                let prefix = std::str::from_utf8(&bytes[..error.valid_up_to()]).unwrap();
                append_without_nul(&mut output, prefix);
                output.push('\u{fffd}');
                // GLib advances one byte, even for incomplete multi-byte tails.
                bytes = &bytes[error.valid_up_to() + 1..];
            }
        }
    }
    output
}
fn append_without_nul(output: &mut String, value: &str) {
    output.extend(value.chars().map(|character| {
        if character == '\0' {
            '\u{fffd}'
        } else {
            character
        }
    }));
}
/// Escape XML markup, including XML 1.1 control references used by GMarkup.
pub fn escape_markup(text: &str) -> String {
    use std::fmt::Write;
    let mut output = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '\'' => output.push_str("&apos;"),
            '"' => output.push_str("&quot;"),
            '\u{1}'..='\u{8}'
            | '\u{b}'..='\u{c}'
            | '\u{e}'..='\u{1f}'
            | '\u{7f}'..='\u{84}'
            | '\u{86}'..='\u{9f}' => {
                write!(output, "&#x{:x};", character as u32).unwrap();
            }
            other => output.push(other),
        }
    }
    output
}

pub use crate::ucs2_conversion::Ucs2Char as xcb_char2b_t;

// Raw adapters remain solely as an oracle for the historical ownership API.
#[cfg(test)]
mod legacy {
    use super::*;
    fn boxed(text: I3String) -> *mut I3String {
        Box::into_raw(Box::new(text))
    }
    unsafe fn text<'a>(raw: *mut I3String) -> &'a I3String {
        // SAFETY: Each entry point requires a live box from this module; access
        // is borrowed for that entry point and cached buffers remain stable.
        unsafe { &*raw }
    }
    fn utf8(text: &I3String) -> Option<&CStr> {
        text.cstr_or_report()
    }
    fn glyphs(text: &I3String) -> &[Ucs2Char] {
        text.rendering_glyphs()
    }
    /// # Safety
    /// Input is a live C string; release the returned box only with i3string_free.

    pub unsafe fn i3string_from_utf8(input: *const libc::c_char) -> *mut I3String {
        unsafe { i3string_from_utf8_with_length(input, -1) }
    }
    /// # Safety
    /// Input is a live C string, or readable for length bytes when nonnegative.

    pub unsafe fn i3string_from_utf8_with_length(
        input: *const libc::c_char,
        length: isize,
    ) -> *mut I3String {
        // SAFETY: The caller supplies either a C string or a readable byte span.
        let bytes = if length < 0 {
            unsafe { CStr::from_ptr(input) }.to_bytes()
        } else if length == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(input.cast::<u8>(), length as usize) }
        };
        let result = I3String::from_bytes(bytes);
        // The old explicit-length constructor reports the original byte count,
        // even when make-valid expands invalid bytes. Keep that quirk only here.
        if length >= 0 {
            result.legacy_num_bytes.set(length as usize);
        }
        boxed(result)
    }
    /// # Safety
    /// Input is a live C string.

    pub unsafe fn i3string_from_markup(input: *const libc::c_char) -> *mut I3String {
        let result = unsafe { i3string_from_utf8(input) };
        unsafe { text(result) }.set_markup(true);
        result
    }
    /// # Safety
    /// Input is readable for length bytes.

    pub unsafe fn i3string_from_markup_with_length(
        input: *const libc::c_char,
        length: usize,
    ) -> *mut I3String {
        let result = unsafe { i3string_from_utf8_with_length(input, length as isize) };
        unsafe { text(result) }.set_markup(true);
        result
    }
    /// # Safety
    /// Input contains count readable UCS-2 units, or may be null for count zero.

    pub unsafe fn i3string_from_ucs2(input: *const Ucs2Char, count: usize) -> *mut I3String {
        let glyphs = if count == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        boxed(I3String::from_ucs2(glyphs))
    }
    /// # Safety
    /// raw is a live text box. Its copy is independently owned.

    pub unsafe fn i3string_copy(raw: *mut I3String) -> *mut I3String {
        let source = unsafe { text(raw) };
        let Some(value) = utf8(source) else {
            return std::ptr::null_mut();
        };
        let copy = I3String::from_cstr(value);
        copy.set_markup(source.is_markup());
        boxed(copy)
    }
    /// # Safety
    /// raw is null or a still-owned box returned by an i3string constructor.

    pub unsafe fn i3string_free(raw: *mut I3String) {
        if !raw.is_null() {
            unsafe { drop(Box::from_raw(raw)) };
        }
    }
    /// # Safety
    /// raw is a live text box; the borrowed pointer lasts until i3string_free.

    pub unsafe fn i3string_as_utf8(raw: *mut I3String) -> *const libc::c_char {
        utf8(unsafe { text(raw) }).map_or(std::ptr::null(), CStr::as_ptr)
    }
    /// # Safety
    /// raw is a live text box; the borrowed pointer lasts until i3string_free.

    pub unsafe fn i3string_as_ucs2(raw: *mut I3String) -> *const Ucs2Char {
        let source = unsafe { text(raw) };
        let glyphs = glyphs(source);
        if glyphs.is_empty() && !source.originated_as_ucs2 {
            std::ptr::null()
        } else {
            glyphs.as_ptr()
        }
    }
    /// # Safety
    /// raw is a live text box.

    pub unsafe fn i3string_get_num_bytes(raw: *mut I3String) -> usize {
        let source = unsafe { text(raw) };
        let _ = utf8(source);
        source.legacy_num_bytes.get()
    }
    /// # Safety
    /// raw is a live text box.

    pub unsafe fn i3string_get_num_glyphs(raw: *mut I3String) -> usize {
        glyphs(unsafe { text(raw) }).len()
    }
    /// # Safety
    /// raw is a live text box.

    pub unsafe fn i3string_is_markup(raw: *mut I3String) -> bool {
        unsafe { text(raw) }.is_markup()
    }
    /// # Safety
    /// raw is a live text box.

    pub unsafe fn i3string_set_markup(raw: *mut I3String, markup: bool) {
        unsafe { text(raw) }.set_markup(markup);
    }
    /// # Safety
    /// raw is a live text box; release the returned independent box with i3string_free.

    pub unsafe fn i3string_escape_markup(raw: *mut I3String) -> *mut I3String {
        let source = unsafe { text(raw) };
        match source.escaped_markup() {
            Ok(escaped) => boxed(escaped),
            Err(issue) => {
                crate::ucs2_conversion::legacy::report(issue, c"Converting to UTF-8 failed");
                std::ptr::null_mut()
            }
        }
    }
}
#[cfg(test)]
pub use legacy::*;

#[cfg(test)]
mod owned_length_tests {
    use super::I3String;

    #[test]
    fn legacy_length_keeps_the_input_count_after_nul_and_invalid_byte_repair() {
        let input = b"a\0\xff";
        let text = I3String::from_bytes_with_legacy_length(input);
        assert_eq!(text.as_str().unwrap(), "a\u{fffd}\u{fffd}");
        assert_eq!(text.legacy_num_bytes.get(), input.len());
        assert!(text.byte_len().unwrap() > input.len());
        assert_eq!(
            I3String::from_bytes(input).legacy_num_bytes.get(),
            text.byte_len().unwrap()
        );
    }

    #[test]
    fn borrowed_render_views_and_utf8_copy_keep_distinct_length_contracts() {
        let original = I3String::from_bytes_with_legacy_length(b"A\xff");
        original.set_markup(true);
        let address = original.cstr_or_report().unwrap().as_ptr();
        assert_eq!(original.rendering_byte_len(), 2);
        assert_eq!(original.rendering_glyphs(), &[0x41.into(), 0xfffd.into()]);
        assert_eq!(original.cstr_or_report().unwrap().as_ptr(), address);
        let copy = original.copy_utf8().unwrap();
        assert_eq!(copy.as_str(), Ok("A�"));
        assert_eq!(copy.rendering_byte_len(), 4);
        assert!(copy.is_markup());
        assert_ne!(copy.cstr_or_report().unwrap().as_ptr(), address);
        original.set_markup(false);
        assert!(copy.is_markup());
    }

    #[test]
    fn core_font_rendering_preserves_embedded_nul_and_ucs2_source_after_utf8_projection() {
        let glyphs = [0x41.into(), 0.into(), 0x42.into()];
        let original = I3String::from_ucs2(&glyphs);
        assert_eq!(original.rendering_glyphs(), glyphs);
        assert_eq!(original.cstr_or_report().unwrap().to_bytes(), b"A");
        assert_eq!(original.rendering_byte_len(), 1);
        assert_eq!(original.rendering_glyphs(), glyphs);
        assert_eq!(
            original.copy_utf8().unwrap().rendering_glyphs(),
            &[0x41.into()]
        );
    }
}
