/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * mode.c: Handle mode event and show current binding mode in the bar
 *
 */
//! Binding-mode events use owned parser state and owned display text.

use i3_common::string::I3String;
use i3_json::{Event, EventHandler, ParseError, ParseErrorKind, Parser};
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Mode {
    pub name: Option<I3String>,
    pub name_width: i32,
}

#[derive(Debug)]
enum Update {
    Markup(bool),
    Name(I3String),
}

#[derive(Debug)]
pub struct ModeEvent {
    updates: Vec<Update>,
}

impl ModeEvent {
    /// Measure each completed map just as the streaming callbacks did. The
    /// caller supplies font access, keeping native rendering outside parsing.
    pub fn into_mode(self, mut measure: impl FnMut(&I3String) -> i32, verbose: bool) -> Mode {
        let mut mode = Mode::default();
        for update in self.updates {
            match update {
                Update::Markup(value) if verbose => {
                    let _ = writeln!(
                        std::io::stdout().lock(),
                        "[../i3bar/src/mode.c:50] Setting pango_markup to {}.",
                        i32::from(value)
                    );
                }
                Update::Name(name) => {
                    mode.name_width = measure(&name);
                    if verbose {
                        let _ = writeln!(
                            std::io::stdout().lock(),
                            "[../i3bar/src/mode.c:83] Got mode change: {}",
                            name.as_str().expect("mode text is UTF-8")
                        );
                    }
                    mode.name = if name.as_str().expect("mode text is UTF-8") == "default" {
                        None
                    } else {
                        Some(name)
                    };
                }
                _ => {}
            }
        }
        mode
    }
}

#[derive(Default)]
struct ModeReader {
    key: Option<Vec<u8>>,
    name: Option<Vec<u8>>,
    markup: bool,
    updates: Vec<Update>,
}
struct SharedReader(Rc<RefCell<ModeReader>>);

// printf("%.*s") and strcmp both stop at an embedded NUL in upstream's
// callbacks. The JSON decoder has already validated the complete byte string.
fn c_string_prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}

impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => reader.key = Some(c_string_prefix(key).to_vec()),
            Event::String(value) => {
                if reader.key.take().as_deref() != Some(b"change") {
                    return false;
                }
                reader.name = Some(c_string_prefix(value).to_vec());
            }
            Event::Boolean(value) => {
                if reader.key.take().as_deref() != Some(b"pango_markup") {
                    return false;
                }
                reader.markup = value;
                reader.updates.push(Update::Markup(value));
            }
            Event::EndMap => {
                let Some(bytes) = reader.name.as_deref() else {
                    return false;
                };
                let name = I3String::from_bytes(bytes);
                name.set_markup(reader.markup);
                reader.updates.push(Update::Name(name));
                reader.key = None;
            }
            _ => {}
        }
        true
    }
}

/// Decode the completed maps in a mode event, without requiring an outer map
/// to have finished (as in the upstream incremental reader). Missing names are
/// errors rather than null pointers passed into string and font helpers.
pub fn parse_mode_json(json: &[u8]) -> Result<ModeEvent, ParseError> {
    let reader = Rc::new(RefCell::new(ModeReader::default()));
    let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
    parser.options_mut().allow_legacy_whitespace = true;
    parser.parse(json)?;
    drop(parser);
    let updates = Rc::try_unwrap(reader)
        .ok()
        .expect("parser released its reader")
        .into_inner()
        .updates;
    if !updates
        .iter()
        .any(|update| matches!(update, Update::Name(_)))
    {
        return Err(ParseError {
            kind: ParseErrorKind::Incomplete,
            offset: json.len(),
            message: "mode event has no completed change".into(),
        });
    }
    Ok(ModeEvent { updates })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(json: &[u8]) -> Mode {
        parse_mode_json(json)
            .unwrap()
            .into_mode(|name| name.byte_len().unwrap() as i32, false)
    }

    #[test]
    fn change_is_owned_and_default_clears_the_indicator() {
        let payload = br#"{"change":"resize","pango_markup":true}"#.to_vec();
        let event = parse_mode_json(&payload).unwrap();
        drop(payload);
        let parsed = event.into_mode(|name| name.byte_len().unwrap() as i32, false);
        assert_eq!(parsed.name_width, 6);
        let name = parsed.name.unwrap();
        assert_eq!(name.as_str().unwrap(), "resize");
        assert!(name.is_markup());
        let parsed = mode(br#"{"change":"default"}"#);
        assert!(parsed.name.is_none());
        assert_eq!(parsed.name_width, 7);
    }

    #[test]
    fn duplicate_nested_and_nul_terminated_fields_match_legacy_callbacks() {
        let parsed = mode(
            br#"{"change":"first","nested":{"change":"last\u0000ignored","pango_markup":true}}"#,
        );
        let name = parsed.name.unwrap();
        assert_eq!(name.as_str().unwrap(), "last");
        assert!(name.is_markup());
        let parsed = mode(br#"{"change\u0000ignored":"named","ignored":42}"#);
        assert_eq!(parsed.name.unwrap().as_str().unwrap(), "named");
        // The old reader accepts a completed nested map before the outer map
        // finishes; do not accidentally require Parser::finish here.
        let partial = mode(br#"{"nested":{"change":"partial"},"#);
        assert_eq!(partial.name.unwrap().as_str().unwrap(), "partial");
    }

    #[test]
    fn unsupported_values_missing_names_and_incomplete_events_are_errors() {
        for payload in [
            br#"{"change":"named","unknown":"value"}"#.as_slice(),
            br#"{"change":"named","unknown":true}"#,
            br#"{}"#,
            br#"{"change":"named""#,
            br#"null"#,
        ] {
            assert!(parse_mode_json(payload).is_err(), "{:?}", payload);
        }
        assert_eq!(
            mode(br#"{"change":"named","ignored":null}"#)
                .name
                .unwrap()
                .as_str()
                .unwrap(),
            "named"
        );
    }
}
