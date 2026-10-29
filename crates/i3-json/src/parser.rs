use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParseErrorKind {
    InvalidJson,
    Incomplete,
    Canceled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub offset: usize,
    pub message: String,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}
impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy)]
pub enum Event<'a> {
    Null,
    Boolean(bool),
    Number(&'a str),
    String(&'a [u8]),
    MapKey(&'a [u8]),
    StartMap,
    EndMap,
    StartArray,
    EndArray,
}

pub trait EventHandler {
    /// Returning false stops parsing immediately, without receiving later events.
    fn on_event(&mut self, event: Event<'_>) -> bool;
    fn error_message(&self) -> Option<&str> {
        None
    }
}
impl<F: for<'a> FnMut(Event<'a>) -> bool> EventHandler for F {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        self(event)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ParserOptions {
    pub allow_comments: bool,
    pub allow_multiple_values: bool,
    pub allow_trailing_garbage: bool,
    pub allow_partial_values: bool,
    pub allow_invalid_utf8: bool,
    /// Accept YAJL's extra vertical-tab/form-feed whitespace in legacy streams.
    pub allow_legacy_whitespace: bool,
}

#[derive(Clone, Copy)]
enum ArrayState {
    First,
    Value,
    Separator,
}
#[derive(Clone, Copy)]
enum ObjectState {
    First,
    Key,
    Colon,
    Value,
    Separator,
}
enum Frame {
    Array(ArrayState),
    Object(ObjectState),
}
enum Token {
    Punctuation(u8),
    String(Vec<u8>),
    Number(String),
    Boolean(bool),
    Null,
}

/// Incremental JSON events without buffering an entire document or status stream.
///
/// Only an unfinished scalar/comment and the nesting stack survive each call.
/// Strings and numbers are decoded/validated by serde_json before delivery.
pub struct Parser {
    handler: Box<dyn EventHandler>,
    options: ParserOptions,
    pending: Vec<u8>,
    stack: Vec<Frame>,
    roots: usize,
    stopped: bool,
    processed: usize,
    position: usize,
    bytes_consumed: usize,
    error: Option<ParseError>,
}

impl Parser {
    pub fn new(handler: impl EventHandler + 'static) -> Self {
        Self {
            handler: Box::new(handler),
            options: ParserOptions::default(),
            pending: Vec::new(),
            stack: Vec::new(),
            roots: 0,
            stopped: false,
            processed: 0,
            position: 0,
            bytes_consumed: 0,
            error: None,
        }
    }
    pub fn options_mut(&mut self) -> &mut ParserOptions {
        &mut self.options
    }
    pub fn bytes_consumed(&self) -> usize {
        self.bytes_consumed
    }
    pub fn buffered_bytes(&self) -> usize {
        self.pending.len()
    }
    pub fn error(&self) -> Option<&ParseError> {
        self.error.as_ref()
    }

    pub fn parse(&mut self, input: &[u8]) -> Result<(), ParseError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        if self.stopped {
            self.bytes_consumed = 0;
            return Ok(());
        }
        let previous = self.pending.len();
        self.pending.extend_from_slice(input);
        let result = self.pump(false);
        self.bytes_consumed = if self.stopped || result.is_err() {
            self.position.saturating_sub(previous).min(input.len())
        } else {
            input.len()
        };
        self.discard_consumed();
        result
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        if let Some(error) = &self.error {
            let mut error = error.clone();
            if error.kind == ParseErrorKind::Canceled {
                error.kind = ParseErrorKind::InvalidJson;
            }
            return Err(error);
        }
        if self.stopped {
            return Ok(());
        }
        let result = self.pump(true);
        self.discard_consumed();
        if let Err(error) = result {
            if error.kind == ParseErrorKind::Incomplete && self.options.allow_partial_values {
                self.error = None;
                return Ok(());
            }
            return Err(error);
        }
        if !self.options.allow_partial_values && (!self.stack.is_empty() || self.roots == 0) {
            return self.fail(ParseErrorKind::Incomplete, "parse error: premature EOF");
        }
        Ok(())
    }

    fn discard_consumed(&mut self) {
        self.processed += self.position;
        if self.stopped {
            self.pending.clear();
        } else {
            self.pending.drain(..self.position);
        }
        self.position = 0;
    }
    fn fail<T>(
        &mut self,
        kind: ParseErrorKind,
        message: impl Into<String>,
    ) -> Result<T, ParseError> {
        let error = ParseError {
            kind,
            offset: self.processed + self.position,
            message: message.into(),
        };
        self.error = Some(error.clone());
        Err(error)
    }
    fn emit(&mut self, event: Event<'_>) -> Result<(), ParseError> {
        if self.handler.on_event(event) {
            Ok(())
        } else if let Some(message) = self.handler.error_message().map(str::to_owned) {
            self.fail(ParseErrorKind::InvalidJson, message)
        } else {
            self.fail(
                ParseErrorKind::Canceled,
                "parse error: client canceled parse via callback return value",
            )
        }
    }
    fn before_value(&mut self) -> Result<(), ParseError> {
        match self.stack.last_mut() {
            Some(Frame::Array(state @ (ArrayState::First | ArrayState::Value))) => {
                *state = ArrayState::Separator
            }
            Some(Frame::Object(state @ ObjectState::Value)) => *state = ObjectState::Separator,
            Some(_) => {
                return self.fail(ParseErrorKind::InvalidJson, "parse error: unexpected value")
            }
            None if self.roots == 0 || self.options.allow_multiple_values => self.roots += 1,
            None => return self.fail(ParseErrorKind::InvalidJson, "parse error: trailing garbage"),
        }
        Ok(())
    }
    fn pump(&mut self, final_input: bool) -> Result<(), ParseError> {
        loop {
            let mut cursor = self.position;
            let token = match next_token(&self.pending, &mut cursor, self.options, final_input) {
                Ok(token) => token,
                Err(message) => {
                    self.position = cursor;
                    let kind = if final_input
                        && (message.contains("unterminated") || message.contains("premature EOF"))
                    {
                        ParseErrorKind::Incomplete
                    } else {
                        ParseErrorKind::InvalidJson
                    };
                    return self.fail(kind, message);
                }
            };
            self.position = cursor;
            let Some(token) = token else {
                return Ok(());
            };
            match token {
                Token::Punctuation(b'[') => {
                    self.before_value()?;
                    self.stack.push(Frame::Array(ArrayState::First));
                    self.emit(Event::StartArray)?;
                }
                Token::Punctuation(b'{') => {
                    self.before_value()?;
                    self.stack.push(Frame::Object(ObjectState::First));
                    self.emit(Event::StartMap)?;
                }
                Token::Punctuation(b']') => {
                    if !matches!(
                        self.stack.last(),
                        Some(Frame::Array(ArrayState::First | ArrayState::Separator))
                    ) {
                        return self.fail(
                            ParseErrorKind::InvalidJson,
                            "parse error: unexpected array end",
                        );
                    }
                    self.stack.pop();
                    self.emit(Event::EndArray)?;
                }
                Token::Punctuation(b'}') => {
                    if !matches!(
                        self.stack.last(),
                        Some(Frame::Object(ObjectState::First | ObjectState::Separator))
                    ) {
                        return self.fail(
                            ParseErrorKind::InvalidJson,
                            "parse error: unexpected map end",
                        );
                    }
                    self.stack.pop();
                    self.emit(Event::EndMap)?;
                }
                Token::Punctuation(b':') => match self.stack.last_mut() {
                    Some(Frame::Object(state @ ObjectState::Colon)) => *state = ObjectState::Value,
                    _ => {
                        return self
                            .fail(ParseErrorKind::InvalidJson, "parse error: unexpected colon")
                    }
                },
                Token::Punctuation(b',') => match self.stack.last_mut() {
                    Some(Frame::Object(state @ ObjectState::Separator)) => {
                        *state = ObjectState::Key
                    }
                    Some(Frame::Array(state @ ArrayState::Separator)) => *state = ArrayState::Value,
                    _ => {
                        return self
                            .fail(ParseErrorKind::InvalidJson, "parse error: unexpected comma")
                    }
                },
                Token::Punctuation(_) => unreachable!(),
                Token::String(value) => {
                    if let Some(Frame::Object(state @ (ObjectState::First | ObjectState::Key))) =
                        self.stack.last_mut()
                    {
                        *state = ObjectState::Colon;
                        self.emit(Event::MapKey(&value))?;
                    } else {
                        self.before_value()?;
                        self.emit(Event::String(&value))?;
                    }
                }
                Token::Number(value) => {
                    self.before_value()?;
                    self.emit(Event::Number(&value))?;
                }
                Token::Boolean(value) => {
                    self.before_value()?;
                    self.emit(Event::Boolean(value))?;
                }
                Token::Null => {
                    self.before_value()?;
                    self.emit(Event::Null)?;
                }
            }
            if self.stack.is_empty() && self.roots > 0 && self.options.allow_trailing_garbage {
                self.stopped = true;
                return Ok(());
            }
        }
    }
}

fn next_token(
    input: &[u8],
    cursor: &mut usize,
    options: ParserOptions,
    final_input: bool,
) -> Result<Option<Token>, String> {
    loop {
        while input.get(*cursor).is_some_and(|byte| {
            matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
                || (options.allow_legacy_whitespace && matches!(byte, 0x0b | 0x0c))
        }) {
            *cursor += 1;
        }
        if *cursor == input.len() {
            return Ok(None);
        }
        if input[*cursor] != b'/' || !options.allow_comments {
            break;
        }
        let start = *cursor;
        match input.get(start + 1) {
            Some(b'/') => {
                if let Some(end) = input[start + 2..].iter().position(|&byte| byte == b'\n') {
                    *cursor = start + 2 + end + 1;
                } else if final_input {
                    *cursor = input.len();
                    return Ok(None);
                } else {
                    return Ok(None);
                }
            }
            Some(b'*') => {
                if let Some(end) = input[start + 2..]
                    .windows(2)
                    .position(|bytes| bytes == b"*/")
                {
                    *cursor = start + 2 + end + 2;
                } else if final_input {
                    return Err("lexical error: unterminated comment".into());
                } else {
                    return Ok(None);
                }
            }
            None if !final_input => return Ok(None),
            _ => return Err("lexical error: invalid comment".into()),
        }
    }
    let start = *cursor;
    match input[start] {
        byte @ (b'[' | b']' | b'{' | b'}' | b',' | b':') => {
            *cursor += 1;
            Ok(Some(Token::Punctuation(byte)))
        }
        b'"' => {
            let mut escaped = false;
            let mut end = start + 1;
            while end < input.len() {
                match input[end] {
                    b'"' if !escaped => {
                        let raw = &input[start..=end];
                        let value = if options.allow_invalid_utf8 {
                            decode_bytes(raw)
                        } else {
                            serde_json::from_slice::<String>(raw)
                                .map(String::into_bytes)
                                .map_err(|error| error.to_string())
                        }?;
                        *cursor = end + 1;
                        return Ok(Some(Token::String(value)));
                    }
                    b'\\' if !escaped => escaped = true,
                    _ => escaped = false,
                }
                end += 1;
            }
            if final_input {
                Err("lexical error: unterminated string".into())
            } else {
                Ok(None)
            }
        }
        byte @ (b't' | b'f' | b'n') => {
            let (word, token) = match byte {
                b't' => (b"true".as_slice(), Token::Boolean(true)),
                b'f' => (b"false".as_slice(), Token::Boolean(false)),
                _ => (b"null".as_slice(), Token::Null),
            };
            let remaining = &input[start..];
            if remaining.len() < word.len() && word.starts_with(remaining) && !final_input {
                return Ok(None);
            }
            if !remaining.starts_with(word) {
                *cursor = start
                    + remaining
                        .iter()
                        .zip(word)
                        .take_while(|(a, b)| a == b)
                        .count();
                return Err("lexical error: invalid JSON literal".into());
            }
            *cursor = start + word.len();
            Ok(Some(token))
        }
        b'-' | b'0'..=b'9' => {
            // A JSON number ends before an adjacent sign or a digit following
            // its leading zero. Multiple-root streams may contain these tokens.
            let mut end = start;
            if input[end] == b'-' {
                end += 1;
            }
            macro_rules! require_digit {
                () => {
                    match input.get(end) {
                        Some(b'0'..=b'9') => (),
                        None if !final_input => return Ok(None),
                        _ => {
                            *cursor = end;
                            return Err("lexical error: missing digit in JSON number".into());
                        }
                    }
                };
            }
            require_digit!();
            if input[end] == b'0' {
                end += 1;
            } else {
                while input.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
            }
            if input.get(end) == Some(&b'.') {
                end += 1;
                require_digit!();
                while input.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
            }
            if matches!(input.get(end), Some(b'e' | b'E')) {
                end += 1;
                if matches!(input.get(end), Some(b'+' | b'-')) {
                    end += 1;
                }
                require_digit!();
                while input.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
            }
            if end == input.len() && !final_input {
                return Ok(None);
            }
            let raw = &input[start..end];
            serde_json::from_slice::<serde_json::Number>(raw)
                .map_err(|error| format!("lexical error: {error}"))?;
            *cursor = end;
            Ok(Some(Token::Number(
                String::from_utf8(raw.to_vec()).expect("number grammar is ASCII"),
            )))
        }
        b'/' => Err("lexical error: comments are disabled".into()),
        _ => {
            *cursor += 1;
            Err("lexical error: invalid char in json text".into())
        }
    }
}

fn decode_bytes(raw: &[u8]) -> Result<Vec<u8>, String> {
    use serde::Deserializer;
    struct Bytes;
    impl<'de> serde::de::Visitor<'de> for Bytes {
        type Value = Vec<u8>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("JSON byte string")
        }
        fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
        fn visit_borrowed_bytes<E: serde::de::Error>(
            self,
            value: &'de [u8],
        ) -> Result<Self::Value, E> {
            Ok(value.to_vec())
        }
    }
    serde_json::Deserializer::from_slice(raw)
        .deserialize_bytes(Bytes)
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    fn recorder() -> (Parser, Rc<RefCell<Vec<String>>>) {
        let events = Rc::new(RefCell::new(Vec::new()));
        let output = events.clone();
        let parser = Parser::new(move |event: Event<'_>| {
            output.borrow_mut().push(format!("{event:?}"));
            true
        });
        (parser, events)
    }
    #[test]
    fn every_byte_boundary_decodes_the_same_unicode_and_number_events() {
        let input = br#"{"escaped":"a\"b\n\uD83D\uDE00","numbers":[-9223372036854775808,1.25e+4,true,null]}"#;
        let (mut whole, expected) = recorder();
        whole.parse(input).unwrap();
        whole.finish().unwrap();
        for boundary in 0..=input.len() {
            let (mut split, actual) = recorder();
            split.parse(&input[..boundary]).unwrap();
            split.parse(&input[boundary..]).unwrap();
            split.finish().unwrap();
            assert_eq!(*expected.borrow(), *actual.borrow(), "split at {boundary}");
        }
    }
    #[test]
    fn never_closed_status_array_emits_updates_without_retaining_the_stream() {
        let (mut parser, events) = recorder();
        parser.parse(b"[").unwrap();
        for index in 0..2000 {
            let comma = if index == 0 { "" } else { "," };
            parser
                .parse(format!("{comma}[{{\"full_text\":\"item {index}\"}}]").as_bytes())
                .unwrap();
            assert_eq!(parser.buffered_bytes(), 0);
        }
        assert_eq!(
            events
                .borrow()
                .iter()
                .filter(|e| e.as_str() == "EndArray")
                .count(),
            2000
        );
    }
    #[test]
    fn header_stops_before_status_stream_and_reports_consumed_bytes() {
        let (mut parser, _) = recorder();
        parser.options_mut().allow_trailing_garbage = true;
        parser.parse(b"{\"version\":1}\n[").unwrap();
        assert_eq!(parser.bytes_consumed(), 13);
        parser.finish().unwrap();
    }
    #[test]
    fn comments_and_multiple_layout_roots_can_cross_chunks() {
        let (mut parser, events) = recorder();
        parser.options_mut().allow_comments = true;
        parser.options_mut().allow_multiple_values = true;
        for chunk in [
            b"/".as_slice(),
            b"* comment *".as_slice(),
            b"/{\"x\":1}//x\n{\"y\":2}".as_slice(),
        ] {
            parser.parse(chunk).unwrap();
        }
        parser.finish().unwrap();
        assert_eq!(
            events
                .borrow()
                .iter()
                .filter(|e| e.as_str() == "EndMap")
                .count(),
            2
        );
    }
    #[test]
    fn malformed_and_truncated_inputs_are_distinct() {
        for bytes in [
            b"[1,]".as_slice(),
            b"{\"x\" 1}".as_slice(),
            b"[01]".as_slice(),
            b"true false".as_slice(),
        ] {
            let (mut parser, _) = recorder();
            assert!(parser.parse(bytes).is_err() || parser.finish().is_err());
        }
        let (mut parser, _) = recorder();
        parser.parse(b"{\"x\":").unwrap();
        assert_eq!(
            parser.finish().unwrap_err().kind,
            ParseErrorKind::Incomplete
        );
    }
    #[test]
    fn partial_values_allow_unclosed_structures_but_not_malformed_scalars() {
        for input in [b"\"unfinished".as_slice(), b"[1,".as_slice()] {
            let (mut parser, _) = recorder();
            parser.parse(input).unwrap();
            assert_eq!(
                parser.finish().unwrap_err().kind,
                ParseErrorKind::Incomplete
            );
            let (mut partial, _) = recorder();
            partial.options_mut().allow_partial_values = true;
            partial.parse(input).unwrap();
            partial.finish().unwrap();
        }
        for input in [
            b"tru".as_slice(),
            b"1e".as_slice(),
            b"-".as_slice(),
            b"1.".as_slice(),
        ] {
            let (mut parser, _) = recorder();
            parser.options_mut().allow_partial_values = true;
            parser.parse(input).unwrap();
            assert_eq!(
                parser.finish().unwrap_err().kind,
                ParseErrorKind::InvalidJson
            );
        }
    }
    #[test]
    fn isolated_surrogates_and_structurally_invalid_utf8_stay_rejected() {
        for input in [
            br#""\uD800""#.as_slice(),
            br#""\uDC00""#.as_slice(),
            b"\"\xc0\xaf\"".as_slice(),
            b"\"\xed\xa0\x80\"".as_slice(),
        ] {
            let (mut parser, _) = recorder();
            assert!(parser.parse(input).is_err());
        }
    }
    #[test]
    fn multiple_root_number_boundaries_preserve_leading_zero_and_sign_tokens() {
        let (mut parser, events) = recorder();
        parser.options_mut().allow_multiple_values = true;
        parser.parse(b"01-2").unwrap();
        parser.finish().unwrap();
        assert_eq!(
            *events.borrow(),
            ["Number(\"0\")", "Number(\"1\")", "Number(\"-2\")"]
        );
    }
    #[test]
    fn non_json_ascii_whitespace_is_rejected() {
        let (mut parser, _) = recorder();
        assert!(parser.parse(b"[\x0b]").is_err());
        let (mut legacy, _) = recorder();
        legacy.options_mut().allow_legacy_whitespace = true;
        legacy.parse(b"[\x0b\x0c]").unwrap();
        legacy.finish().unwrap();
    }
    #[test]
    fn callbacks_can_cancel_and_invalid_utf8_requires_an_explicit_option() {
        let mut parser = Parser::new(|_: Event<'_>| false);
        assert_eq!(
            parser.parse(b"[]").unwrap_err().kind,
            ParseErrorKind::Canceled
        );
        assert_eq!(
            parser.finish().unwrap_err().kind,
            ParseErrorKind::InvalidJson
        );
        let (mut parser, _) = recorder();
        assert!(parser.parse(b"\"\xff\"").is_err());
        let (mut parser, events) = recorder();
        parser.options_mut().allow_invalid_utf8 = true;
        parser.parse(b"\"\xff\"").unwrap();
        parser.finish().unwrap();
        assert!(events.borrow()[0].contains("255"));
    }
}
