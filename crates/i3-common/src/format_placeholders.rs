// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::{CStr, CString};

/// Expand '%' placeholders in order, leaving unknown tokens unchanged.
/// The first matching nonempty '%' name wins when names share a prefix.
pub fn expand(format: &str, placeholders: &[(&str, &str)]) -> String {
    let entries: Vec<_> = placeholders
        .iter()
        .map(|(name, value)| (name.as_bytes(), value.as_bytes()))
        .collect();
    // Matching complete UTF-8 names in a UTF-8 format preserves UTF-8 boundaries.
    String::from_utf8(expand_bytes(format.as_bytes(), &entries))
        .expect("UTF-8 placeholder expansion")
}

/// Byte-preserving expansion for C strings, without a raw pointer interface.
pub fn expand_cstr(format: &CStr, placeholders: &[(&CStr, &CStr)]) -> CString {
    let entries: Vec<_> = placeholders
        .iter()
        .map(|(name, value)| (name.to_bytes(), value.to_bytes()))
        .collect();
    CString::new(expand_bytes(format.to_bytes(), &entries))
        .expect("C string inputs contain no embedded NUL")
}

fn expand_bytes(format: &[u8], placeholders: &[(&[u8], &[u8])]) -> Vec<u8> {
    let mut output = Vec::with_capacity(format.len());
    let mut remaining = format;
    while let Some((&first, tail)) = remaining.split_first() {
        let replacement = if first == b'%' {
            placeholders
                .iter()
                .find(|(name, _)| name.starts_with(b"%") && remaining.starts_with(name))
        } else {
            None
        };
        if let Some((name, value)) = replacement {
            output.extend_from_slice(value);
            remaining = &remaining[name.len()..];
        } else {
            output.push(first);
            remaining = tail;
        }
    }
    output
}

/// Legacy pointer layouts and allocation boundary, pending caller migration.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct placeholder_t {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
}

/// Legacy ABI adapter for [`expand_cstr`]. Returned storage belongs to free().
///
/// # Safety
/// `format` is null or a live C string. `placeholders` contains `num` entries,
/// each with live C-string name/value pointers. Nonpositive num means no entries.

pub unsafe fn format_placeholders(
    format: *mut libc::c_char,
    placeholders: *mut placeholder_t,
    num: libc::c_int,
) -> *mut libc::c_char {
    if format.is_null() {
        return std::ptr::null_mut();
    }
    let count = usize::try_from(num).unwrap_or(0);
    let entries = if count == 0 {
        &[]
    } else {
        // SAFETY: The caller supplies count live placeholder records.
        unsafe { std::slice::from_raw_parts(placeholders, count) }
    };
    let entries: Vec<_> = entries
        .iter()
        .map(|entry| {
            // SAFETY: Names and values follow the caller's C-string contract.
            unsafe { (CStr::from_ptr(entry.name), CStr::from_ptr(entry.value)) }
        })
        .collect();
    // SAFETY: format is nonnull and follows the caller's C-string contract.
    let expanded = expand_cstr(unsafe { CStr::from_ptr(format) }, &entries);
    // SAFETY: Duplicate the result into the existing malloc allocator boundary;
    // returning CString::into_raw would be incompatible with callers using free.
    unsafe { crate::safewrappers::sstrndup(expanded.as_ptr(), expanded.as_bytes().len()) }
}
