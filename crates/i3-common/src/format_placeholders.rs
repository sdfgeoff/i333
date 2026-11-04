#![forbid(unsafe_code)]

// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

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

/// Expand ordered placeholders without changing binary payload bytes.
pub fn expand_bytes(format: &[u8], placeholders: &[(&[u8], &[u8])]) -> Vec<u8> {
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
