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

// JSON structure and scalar decoding belong to serde_json. This adapter only
// preserves the event protocol's incremental commits, source offsets and legacy
// comments/whitespace. Replaying an unfinished value never repeats callbacks.
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::io::{self, Read};

#[derive(Debug, Clone)]
enum OwnedEvent {
    Null,
    Boolean(bool),
    Number(String),
    String(Vec<u8>),
    MapKey(Vec<u8>),
    StartMap,
    EndMap,
    StartArray,
    EndArray,
}
impl OwnedEvent {
    fn borrowed(&self) -> Event<'_> {
        match self {
            Self::Null => Event::Null,
            Self::Boolean(value) => Event::Boolean(*value),
            Self::Number(value) => Event::Number(value),
            Self::String(value) => Event::String(value),
            Self::MapKey(value) => Event::MapKey(value),
            Self::StartMap => Event::StartMap,
            Self::EndMap => Event::EndMap,
            Self::StartArray => Event::StartArray,
            Self::EndArray => Event::EndArray,
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum Prefix {
    None,
    Array,
    Map,
}
impl Prefix {
    fn bytes(self) -> &'static [u8] {
        match self {
            Self::None => b"",
            Self::Array => b"[null",
            Self::Map => b"{\"\":null",
        }
    }
    fn events(self) -> usize {
        match self {
            Self::None => 0,
            Self::Array => 2,
            Self::Map => 3,
        }
    }
}
#[derive(Clone, Copy)]
struct Checkpoint {
    end: usize,
    events: usize,
    prefix: Prefix,
    roots: usize,
}
struct Stamped {
    event: OwnedEvent,
    end: usize,
}
struct Context<'a> {
    bytes: &'a [u8],
    position: Cell<usize>,
    final_input: bool,
    invalid_utf8: bool,
    depth: Cell<usize>,
    incomplete_scalar: Cell<bool>,
    events: RefCell<Vec<Stamped>>,
    checkpoint: Cell<Option<Checkpoint>>,
    roots: Cell<usize>,
}
impl Context<'_> {
    fn emit(&self, event: OwnedEvent, end: usize) {
        self.events.borrow_mut().push(Stamped { event, end });
    }
    fn last_end(&self) -> usize {
        self.events.borrow().last().map_or(0, |event| event.end)
    }
    fn checkpoint(&self, prefix: Prefix) {
        self.checkpoint.set(Some(Checkpoint {
            end: self.last_end(),
            events: self.events.borrow().len(),
            prefix,
            roots: self.roots.get(),
        }));
    }
}
struct Input<'a>(&'a Context<'a>);
impl Read for Input<'_> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        let pos = self.0.position.get();
        if let Some(&byte) = self.0.bytes.get(pos) {
            out[0] = byte;
            self.0.position.set(pos + 1);
            Ok(1)
        } else if self.0.final_input {
            Ok(0)
        } else {
            Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "more JSON input required",
            ))
        }
    }
}
#[derive(Clone, Copy)]
enum Hint {
    Root(usize),
    Sequence,
    MapValue,
}
struct ValueSeed<'a, 'b> {
    context: &'a Context<'b>,
    hint: Hint,
}
impl<'de> DeserializeSeed<'de> for ValueSeed<'_, '_> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        let context = self.context;
        let mut start = match self.hint {
            Hint::Root(start) => start,
            Hint::Sequence => context.position.get().saturating_sub(1),
            Hint::MapValue => context.position.get(),
        };
        while context
            .bytes
            .get(start)
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            start += 1;
        }
        let first = context.bytes.get(start).copied();
        let result = match first {
            Some(b'-' | b'0'..=b'9') => {
                let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
                let end = start + raw.get().len();
                context.emit(OwnedEvent::Number(raw.get().to_owned()), end);
                Ok(())
            }
            Some(b'"') if context.invalid_utf8 => deserializer.deserialize_bytes(ScalarVisitor {
                context,
                key: false,
            }),
            _ => deserializer.deserialize_any(ValueVisitor { context }),
        };
        if result.is_err() && matches!(first, Some(b'-' | b'0'..=b'9' | b't' | b'f' | b'n')) {
            context.incomplete_scalar.set(true);
        }
        result
    }
}
struct ScalarVisitor<'a, 'b> {
    context: &'a Context<'b>,
    key: bool,
}
impl ScalarVisitor<'_, '_> {
    fn emit(&self, bytes: &[u8]) {
        let value = if self.key {
            OwnedEvent::MapKey(bytes.to_vec())
        } else {
            OwnedEvent::String(bytes.to_vec())
        };
        self.context.emit(value, self.context.position.get());
    }
}
impl<'de> Visitor<'de> for ScalarVisitor<'_, '_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JSON text")
    }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<(), E> {
        self.emit(value.as_bytes());
        Ok(())
    }
    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<(), E> {
        self.visit_str(&value)
    }
    fn visit_bytes<E: serde::de::Error>(self, value: &[u8]) -> Result<(), E> {
        self.emit(value);
        Ok(())
    }
    fn visit_byte_buf<E: serde::de::Error>(self, value: Vec<u8>) -> Result<(), E> {
        self.visit_bytes(&value)
    }
}
struct KeySeed<'a, 'b>(&'a Context<'b>);
impl<'de> DeserializeSeed<'de> for KeySeed<'_, '_> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        let visitor = ScalarVisitor {
            context: self.0,
            key: true,
        };
        if self.0.invalid_utf8 {
            deserializer.deserialize_bytes(visitor)
        } else {
            deserializer.deserialize_string(visitor)
        }
    }
}
struct ValueVisitor<'a, 'b> {
    context: &'a Context<'b>,
}
impl<'de> Visitor<'de> for ValueVisitor<'_, '_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JSON value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<(), E> {
        self.context
            .emit(OwnedEvent::Null, self.context.position.get());
        Ok(())
    }
    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<(), E> {
        self.context
            .emit(OwnedEvent::Boolean(value), self.context.position.get());
        Ok(())
    }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<(), E> {
        ScalarVisitor {
            context: self.context,
            key: false,
        }
        .visit_str(value)
    }
    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<(), E> {
        self.visit_str(&value)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        let context = self.context;
        context.emit(OwnedEvent::StartArray, context.position.get());
        let depth = context.depth.get();
        context.depth.set(depth + 1);
        while seq
            .next_element_seed(ValueSeed {
                context,
                hint: Hint::Sequence,
            })?
            .is_some()
        {
            if depth == 0 {
                context.checkpoint(Prefix::Array);
            }
        }
        context.emit(OwnedEvent::EndArray, context.position.get());
        context.depth.set(depth);
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let context = self.context;
        context.emit(OwnedEvent::StartMap, context.position.get());
        let depth = context.depth.get();
        context.depth.set(depth + 1);
        while map.next_key_seed(KeySeed(context))?.is_some() {
            map.next_value_seed(ValueSeed {
                context,
                hint: Hint::MapValue,
            })?;
            if depth == 0 {
                context.checkpoint(Prefix::Map);
            }
        }
        context.emit(OwnedEvent::EndMap, context.position.get());
        context.depth.set(depth);
        Ok(())
    }
}
struct Normalized {
    bytes: Vec<u8>,
    ends: Vec<usize>,
    unfinished_comment: bool,
    max_depth: usize,
}
fn normalize(input: &[u8], options: ParserOptions) -> Normalized {
    let mut result = Normalized {
        bytes: Vec::with_capacity(input.len()),
        ends: vec![0],
        unfinished_comment: false,
        max_depth: 0,
    };
    let mut quote = false;
    let mut escape = false;
    let mut depth = 0_usize;
    let mut i = 0;
    let mut number_start = true;
    while i < input.len() {
        let byte = input[i];
        if !quote
            && options.allow_comments
            && byte == b'/'
            && matches!(input.get(i + 1), Some(b'/' | b'*'))
        {
            let block = input[i + 1] == b'*';
            let start = i;
            i += 2;
            if block {
                while i + 1 < input.len() && &input[i..i + 2] != b"*/" {
                    i += 1;
                }
                if i + 1 < input.len() {
                    i += 2;
                } else {
                    i = input.len();
                    result.unfinished_comment = true;
                }
            } else {
                while i < input.len() && input[i] != b'\n' {
                    i += 1;
                }
            }
            for index in start..i {
                result
                    .bytes
                    .push(if input[index] == b'\n' { b'\n' } else { b' ' });
                result.ends.push(index + 1);
            }
            number_start = true;
            continue;
        }
        if !quote && options.allow_comments && byte == b'/' && i + 1 == input.len() {
            result.unfinished_comment = true;
            result.bytes.push(b' ');
            result.ends.push(i + 1);
            i += 1;
            continue;
        }
        if !quote && options.allow_legacy_whitespace && matches!(byte, 0x0b | 0x0c) {
            result.bytes.push(b' ');
            result.ends.push(i + 1);
            number_start = true;
            i += 1;
            continue;
        }
        result.bytes.push(byte);
        result.ends.push(i + 1);
        if quote {
            if escape {
                escape = false;
            } else if byte == b'\\' {
                escape = true;
            } else if byte == b'"' {
                quote = false;
                number_start = true;
            }
        } else {
            if byte == b'"' {
                quote = true;
            }
            if matches!(byte, b'[' | b'{') {
                depth += 1;
                result.max_depth = result.max_depth.max(depth);
            } else if matches!(byte, b']' | b'}') {
                depth = depth.saturating_sub(1);
            }
            // YAJL accepts adjacent root numbers such as 01. A separating
            // space keeps its scalar boundary while Serde validates each value.
            if byte == b'0' && number_start && input.get(i + 1).is_some_and(u8::is_ascii_digit) {
                result.bytes.push(b' ');
                result.ends.push(i + 1);
                number_start = true;
            } else if byte == b'-' && number_start { /* signed number starts here */
            } else {
                number_start = byte.is_ascii_whitespace()
                    || matches!(byte, b'[' | b'{' | b']' | b'}' | b',' | b':');
            }
        }
        i += 1;
    }
    result
}
struct Decoded {
    events: Vec<Stamped>,
    checkpoint: Option<Checkpoint>,
    stopped: bool,
    error: Option<(ParseErrorKind, usize, String)>,
}
fn decode(
    normalized: &Normalized,
    options: ParserOptions,
    final_input: bool,
    roots_before: usize,
) -> Decoded {
    let context = Context {
        bytes: &normalized.bytes,
        position: Cell::new(0),
        final_input,
        invalid_utf8: options.allow_invalid_utf8,
        depth: Cell::new(0),
        incomplete_scalar: Cell::new(false),
        events: RefCell::new(Vec::new()),
        checkpoint: Cell::new(None),
        roots: Cell::new(roots_before),
    };
    let mut de = serde_json::Deserializer::from_reader(Input(&context));
    de.disable_recursion_limit();
    let mut cursor = 0;
    let mut stopped = false;
    let mut error = None;
    loop {
        let start = normalized.bytes[cursor..]
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .map(|offset| cursor + offset);
        let Some(start) = start else { break };
        if context.roots.get() > 0 && !options.allow_multiple_values {
            error = Some((
                ParseErrorKind::InvalidJson,
                start,
                "parse error: trailing garbage".into(),
            ));
            break;
        }
        let result = ValueSeed {
            context: &context,
            hint: Hint::Root(start),
        }
        .deserialize(&mut de);
        match result {
            Ok(()) => {
                cursor = context.last_end();
                context.roots.set(context.roots.get() + 1);
                context.checkpoint(Prefix::None);
                if options.allow_trailing_garbage {
                    stopped = true;
                    break;
                }
            }
            Err(err) => {
                let incomplete =
                    err.is_eof() || err.io_error_kind() == Some(io::ErrorKind::WouldBlock);
                if !incomplete || final_input {
                    let kind = if incomplete && !context.incomplete_scalar.get() {
                        ParseErrorKind::Incomplete
                    } else {
                        ParseErrorKind::InvalidJson
                    };
                    error = Some((kind, context.position.get(), err.to_string()));
                }
                break;
            }
        }
    }
    if final_input && error.is_none() && normalized.unfinished_comment {
        error = Some((
            ParseErrorKind::Incomplete,
            normalized.bytes.len(),
            "lexical error: unterminated comment".into(),
        ));
    }
    Decoded {
        events: context.events.into_inner(),
        checkpoint: context.checkpoint.get(),
        stopped,
        error,
    }
}

pub struct Parser {
    handler: Box<dyn EventHandler>,
    options: ParserOptions,
    pending: Vec<u8>,
    prefix: Prefix,
    seen: usize,
    processed: usize,
    roots: usize,
    stopped: bool,
    bytes_consumed: usize,
    position: usize,
    error: Option<ParseError>,
}
impl Parser {
    pub fn new(handler: impl EventHandler + 'static) -> Self {
        Self {
            handler: Box::new(handler),
            options: ParserOptions::default(),
            pending: Vec::new(),
            prefix: Prefix::None,
            seen: 0,
            processed: 0,
            roots: 0,
            stopped: false,
            bytes_consumed: 0,
            position: 0,
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
        self.pending.len().saturating_sub(self.prefix.bytes().len())
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
        self.bytes_consumed = match &result {
            Ok(()) if !self.stopped => input.len(),
            _ => self.position.saturating_sub(previous).min(input.len()),
        };
        result
    }
    fn pump(&mut self, final_input: bool) -> Result<(), ParseError> {
        let normalized = normalize(&self.pending, self.options);
        let decoded = if normalized.max_depth > 64 {
            std::thread::scope(|scope| {
                std::thread::Builder::new()
                    .stack_size(
                        normalized
                            .max_depth
                            .saturating_mul(16 * 1024)
                            .saturating_add(2 * 1024 * 1024),
                    )
                    .spawn_scoped(scope, || {
                        decode(&normalized, self.options, final_input, self.roots)
                    })
                    .expect("allocate JSON parsing stack")
                    .join()
                    .expect("Serde JSON visitor panicked")
            })
        } else {
            decode(&normalized, self.options, final_input, self.roots)
        };
        let raw_end = |position: usize| normalized.ends[position.min(normalized.ends.len() - 1)];
        for event in decoded.events.iter().skip(self.seen) {
            if !self.handler.on_event(event.event.borrowed()) {
                let message = self.handler.error_message().map(str::to_owned);
                let error = ParseError {
                    kind: if message.is_some() {
                        ParseErrorKind::InvalidJson
                    } else {
                        ParseErrorKind::Canceled
                    },
                    offset: self.processed
                        + raw_end(event.end).saturating_sub(self.prefix.bytes().len()),
                    message: message.unwrap_or_else(|| {
                        "parse error: client canceled parse via callback return value".into()
                    }),
                };
                self.position = raw_end(event.end);
                self.error = Some(error.clone());
                return Err(error);
            }
        }
        self.seen = decoded.events.len();
        self.stopped = decoded.stopped;
        if let Some((kind, position, message)) = decoded.error {
            let error = ParseError {
                kind,
                offset: self.processed
                    + raw_end(position).saturating_sub(self.prefix.bytes().len()),
                message,
            };
            self.position = raw_end(position);
            self.error = Some(error.clone());
            return Err(error);
        }
        self.position = self.pending.len();
        if let Some(checkpoint) = decoded.checkpoint {
            let end = raw_end(checkpoint.end);
            if self.stopped {
                self.position = end;
            }
            self.processed += end.saturating_sub(self.prefix.bytes().len());
            self.pending.drain(..end);
            self.pending
                .splice(..0, checkpoint.prefix.bytes().iter().copied());
            self.prefix = checkpoint.prefix;
            self.seen =
                checkpoint.prefix.events() + decoded.events.len().saturating_sub(checkpoint.events);
            self.roots = checkpoint.roots;
        }
        if self.stopped {
            self.pending.clear();
            self.prefix = Prefix::None;
            self.seen = 0;
        }
        Ok(())
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
        if let Err(error) = self.pump(true) {
            if error.kind == ParseErrorKind::Incomplete && self.options.allow_partial_values {
                self.error = None;
                return Ok(());
            }
            return Err(error);
        }
        if !self.options.allow_partial_values
            && (self.roots == 0
                || normalize(&self.pending, self.options)
                    .bytes
                    .iter()
                    .any(|byte| !byte.is_ascii_whitespace()))
        {
            let error = ParseError {
                kind: ParseErrorKind::Incomplete,
                offset: self.processed,
                message: "parse error: premature EOF".into(),
            };
            self.error = Some(error.clone());
            return Err(error);
        }
        Ok(())
    }
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
    fn serde_private_number_key_is_an_ordinary_object() {
        let (mut parser, events) = recorder();
        parser
            .parse(br#"{"$serde_json::private::Number":"1e999","number":1e999}"#)
            .unwrap();
        parser.finish().unwrap();
        assert_eq!(*events.borrow(), [
            "StartMap", "MapKey([36, 115, 101, 114, 100, 101, 95, 106, 115, 111, 110, 58, 58, 112, 114, 105, 118, 97, 116, 101, 58, 58, 78, 117, 109, 98, 101, 114])",
            "String([49, 101, 57, 57, 57])", "MapKey([110, 117, 109, 98, 101, 114])", "Number(\"1e999\")", "EndMap",
        ]);
    }
    #[test]
    fn numeric_tokens_remain_pending_until_a_boundary() {
        let (mut parser, events) = recorder();
        parser.parse(b"1").unwrap();
        assert!(events.borrow().is_empty());
        parser.parse(b"2").unwrap();
        assert!(events.borrow().is_empty());
        parser.finish().unwrap();
        assert_eq!(*events.borrow(), ["Number(\"12\")"]);
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
