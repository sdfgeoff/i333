/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * parse_json_header.c: Parse the JSON protocol header to determine
 *                      protocol version and features.
 *
 */
//! Protocol header parsing owns its state and borrows the input buffer.

use i3_json::{Event, EventHandler, Parser};
use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    pub stop_signal: i32,
    pub cont_signal: i32,
    // The legacy reader only changes this field when it sees a boolean.
    pub click_events: Option<bool>,
    pub consumed: usize,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: 0,
            stop_signal: libc::SIGSTOP,
            cont_signal: libc::SIGCONT,
            click_events: None,
            consumed: 0,
        }
    }
}

#[derive(Default)]
struct HeaderReader {
    header: Header,
    key: Option<HeaderKey>,
}

#[derive(Clone, Copy)]
enum HeaderKey {
    Version,
    StopSignal,
    ContSignal,
    ClickEvents,
}

fn key_matches(key: &[u8], expected: &CStr) -> bool {
    key.len() == expected.to_bytes().len()
        // SAFETY: Both byte spans contain at least key.len() readable bytes.
        // strncasecmp only reads them and preserves the process locale's
        // comparison rules, including its treatment of embedded NUL bytes.
        && unsafe { libc::strncasecmp(key.as_ptr().cast(), expected.as_ptr(), key.len()) == 0 }
}

struct SharedReader(Rc<RefCell<HeaderReader>>);

impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => {
                let key = if key_matches(key, c"version") {
                    Some(HeaderKey::Version)
                } else if key_matches(key, c"stop_signal") {
                    Some(HeaderKey::StopSignal)
                } else if key_matches(key, c"cont_signal") {
                    Some(HeaderKey::ContSignal)
                } else if key_matches(key, c"click_events") {
                    Some(HeaderKey::ClickEvents)
                } else {
                    // Unknown keys deliberately retain the previous key, as
                    // upstream's header callback does.
                    reader.key
                };
                reader.key = key;
            }
            Event::Boolean(value) if matches!(reader.key, Some(HeaderKey::ClickEvents)) => {
                reader.header.click_events = Some(value);
            }
            Event::Number(value) if !value.contains(['.', 'e', 'E']) => {
                let value = match value.parse::<i64>() {
                    Ok(value) if value != i64::MIN => value,
                    _ => {
                        // Match the typed legacy integer callback's range.
                        return false;
                    }
                };
                match reader.key {
                    Some(HeaderKey::Version) => reader.header.version = value as u32,
                    Some(HeaderKey::StopSignal) => reader.header.stop_signal = value as i32,
                    Some(HeaderKey::ContSignal) => reader.header.cont_signal = value as i32,
                    _ => {}
                }
            }
            _ => {}
        }
        true
    }

    fn error_message(&self) -> Option<&str> {
        // The parser only needs a stable error category here: failed headers
        // reset their fields and fall back to plain text, without diagnostics.
        None
    }
}

/// Parse the first JSON value without consuming a following status stream.
///
/// This intentionally does not finish the parser: like upstream, a fragmented
/// first read can already contain useful header fields before the map closes.
/// Invalid JSON resets the protocol fields but keeps any click-events update.
pub fn parse_json_header(buffer: &[u8]) -> Header {
    let reader = Rc::new(RefCell::new(HeaderReader::default()));
    let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
    parser.options_mut().allow_trailing_garbage = true;
    parser.options_mut().allow_legacy_whitespace = true;
    let valid = parser.parse(buffer).is_ok();
    let consumed = parser.bytes_consumed();
    drop(parser);
    let mut header = Rc::try_unwrap(reader)
        .ok()
        .expect("parser released its reader")
        .into_inner()
        .header;
    if valid {
        header.consumed = consumed;
    } else {
        header = Header {
            click_events: header.click_events,
            ..Header::default()
        };
    }
    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_consumes_only_its_root_and_handles_each_partial_prefix() {
        let header = br#"{"version":1,"click_events":true,"stop_signal":0}[[{"full_text":"hi"}]]"#;
        let end = header.iter().position(|byte| *byte == b'}').unwrap() + 1;
        let parsed = parse_json_header(header);
        assert_eq!(parsed.version, 1);
        assert_eq!(parsed.stop_signal, 0);
        assert_eq!(parsed.click_events, Some(true));
        assert_eq!(parsed.consumed, end);
        for split in 0..=end {
            let prefix = parse_json_header(&header[..split]);
            assert!(prefix.consumed <= split);
            if split == end {
                assert_eq!(prefix, parsed);
            }
        }
    }

    #[test]
    fn unknown_keys_retain_the_last_recognized_header_field() {
        let parsed =
            parse_json_header(br#"{"VERSION":1,"ignored":7,"click_events":true,"other":false}"#);
        assert_eq!(parsed.version, 7);
        assert_eq!(parsed.click_events, Some(false));
    }

    #[test]
    fn invalid_numbers_reset_fields_but_preserve_click_updates() {
        for number in ["9223372036854775808", "-9223372036854775808"] {
            let parsed = parse_json_header(
                format!(r#"{{"click_events":true,"version":{number}}}"#).as_bytes(),
            );
            assert_eq!(
                parsed,
                Header {
                    click_events: Some(true),
                    ..Header::default()
                }
            );
        }
        let parsed =
            parse_json_header(br#"{"version":-1,"stop_signal":4294967297,"cont_signal":1.0}"#);
        assert_eq!(parsed.version, u32::MAX);
        assert_eq!(parsed.stop_signal, 1);
        assert_eq!(parsed.cont_signal, libc::SIGCONT);
    }
}
