use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerateError {
    KeysMustBeStrings,
    DepthExceeded,
    InvalidState,
    Complete,
    InvalidNumber,
    InvalidString,
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON generator error: {self:?}")
    }
}
impl std::error::Error for GenerateError {}

enum Frame {
    Array { values: usize },
    Object { fields: usize, key_expected: bool },
}

/// A streaming JSON writer, including open-array status/click streams.
#[derive(Default)]
pub struct Generator {
    buffer: Vec<u8>,
    stack: Vec<Frame>,
    root_started: bool,
    error: Option<GenerateError>,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }

    pub fn nesting_depth(&self) -> usize {
        self.stack.len()
    }

    /// Discard emitted bytes without changing the enclosing stream structure.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    fn fail<T>(&mut self, error: GenerateError) -> Result<T, GenerateError> {
        // Rejected values have not changed the stream state. Like YAJL, let
        // callers supply a valid value or string key after these errors.
        if !matches!(
            error,
            GenerateError::InvalidNumber | GenerateError::KeysMustBeStrings
        ) {
            self.error = Some(error);
        }
        Err(error)
    }

    fn before_value(&mut self) -> Result<(), GenerateError> {
        if let Some(error) = self.error {
            return Err(error);
        }
        match self.stack.last_mut() {
            Some(Frame::Array { values }) => {
                if *values > 0 {
                    self.buffer.push(b',');
                }
                *values += 1;
            }
            Some(Frame::Object {
                key_expected: true, ..
            }) => return self.fail(GenerateError::KeysMustBeStrings),
            Some(Frame::Object {
                fields,
                key_expected,
            }) => {
                self.buffer.push(b':');
                *fields += 1;
                *key_expected = true;
            }
            None if self.root_started => return self.fail(GenerateError::Complete),
            None => self.root_started = true,
        }
        Ok(())
    }

    /// Append a complete Serde value to the surrounding stream. Serialization
    /// happens before changing framing, so rejected values leave it untouched.
    pub fn serialized<T: serde::Serialize + ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), GenerateError> {
        // Reject impossible framing before invoking user-provided Serialize.
        if let Some(error) = self.error {
            return Err(error);
        }
        match self.stack.last() {
            Some(Frame::Object {
                key_expected: true, ..
            }) => return self.fail(GenerateError::KeysMustBeStrings),
            None if self.root_started => return self.fail(GenerateError::Complete),
            _ => {}
        }
        let bytes = serde_json::to_vec(value).map_err(|_| GenerateError::InvalidState)?;
        if self.stack.len().saturating_add(serialized_depth(&bytes)) > 128 {
            return self.fail(GenerateError::DepthExceeded);
        }
        self.before_value()?;
        self.buffer.extend_from_slice(&bytes);
        Ok(())
    }

    pub fn integer(&mut self, value: i64) -> Result<(), GenerateError> {
        self.before_value()?;
        serde_json::to_writer(&mut self.buffer, &value).expect("writing to Vec cannot fail");
        Ok(())
    }
    pub fn double(&mut self, value: f64) -> Result<(), GenerateError> {
        if !value.is_finite() {
            return self.fail(GenerateError::InvalidNumber);
        }
        self.before_value()?;
        serde_json::to_writer(&mut self.buffer, &value).expect("finite number is serializable");
        Ok(())
    }
    pub fn boolean(&mut self, value: bool) -> Result<(), GenerateError> {
        self.before_value()?;
        self.buffer
            .extend_from_slice(if value { b"true" } else { b"false" });
        Ok(())
    }
    pub fn null(&mut self) -> Result<(), GenerateError> {
        self.before_value()?;
        self.buffer.extend_from_slice(b"null");
        Ok(())
    }
    pub fn string(&mut self, value: &str) -> Result<(), GenerateError> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if let Some(Frame::Object {
            fields,
            key_expected: true,
        }) = self.stack.last()
        {
            if *fields > 0 {
                self.buffer.push(b',');
            }
            serde_json::to_writer(&mut self.buffer, value).expect("writing to Vec cannot fail");
            if let Some(Frame::Object { key_expected, .. }) = self.stack.last_mut() {
                *key_expected = false;
            }
        } else {
            self.before_value()?;
            serde_json::to_writer(&mut self.buffer, value).expect("writing to Vec cannot fail");
        }
        Ok(())
    }
    pub fn start_array(&mut self) -> Result<(), GenerateError> {
        if self.stack.len() >= 128 {
            return self.fail(GenerateError::DepthExceeded);
        }
        self.before_value()?;
        self.buffer.push(b'[');
        self.stack.push(Frame::Array { values: 0 });
        Ok(())
    }
    pub fn end_array(&mut self) -> Result<(), GenerateError> {
        if !matches!(self.stack.last(), Some(Frame::Array { .. })) {
            return self.fail(GenerateError::InvalidState);
        }
        self.stack.pop();
        self.buffer.push(b']');
        Ok(())
    }
    pub fn start_object(&mut self) -> Result<(), GenerateError> {
        if self.stack.len() >= 128 {
            return self.fail(GenerateError::DepthExceeded);
        }
        self.before_value()?;
        self.buffer.push(b'{');
        self.stack.push(Frame::Object {
            fields: 0,
            key_expected: true,
        });
        Ok(())
    }
    pub fn end_object(&mut self) -> Result<(), GenerateError> {
        if !matches!(
            self.stack.last(),
            Some(Frame::Object {
                key_expected: true,
                ..
            })
        ) {
            return self.fail(GenerateError::InvalidState);
        }
        self.stack.pop();
        self.buffer.push(b'}');
        Ok(())
    }
}

// Serde produced valid JSON; only quoted-string/escape state is needed to count
// nesting. Brackets inside text do not contribute to the structural limit.
fn serialized_depth(bytes: &[u8]) -> usize {
    let mut depth = 0_usize;
    let mut maximum = 0;
    let mut quoted = false;
    let mut escaped = false;
    for byte in bytes {
        if quoted {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                quoted = false;
            }
        } else {
            match byte {
                b'"' => quoted = true,
                b'[' | b'{' => {
                    depth += 1;
                    maximum = maximum.max(depth);
                }
                b']' | b'}' => depth -= 1,
                _ => {}
            }
        }
    }
    maximum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialized_depth_counts_structure_and_respects_parent_frames() {
        let mut output = Generator::new();
        for _ in 0..127 {
            output.start_array().unwrap();
        }
        output.serialized(&["[[[", "\\\"{}}]"]).unwrap();
        assert_eq!(output.nesting_depth(), 127);
        output.start_array().unwrap();
        let before = output.bytes().to_vec();
        assert_eq!(
            output.serialized(&Vec::<u8>::new()),
            Err(GenerateError::DepthExceeded)
        );
        assert_eq!(output.bytes(), before);
        assert_eq!(output.boolean(true), Err(GenerateError::DepthExceeded));
        assert_eq!(serialized_depth(br#"{"a":[["}\"[["]]}"#), 3);
    }
    #[test]
    fn impossible_framing_does_not_invoke_serde() {
        struct MustNotRun;
        impl serde::Serialize for MustNotRun {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                panic!("Serialize must not run for rejected framing")
            }
        }
        let mut output = Generator::new();
        output.start_object().unwrap();
        assert_eq!(
            output.serialized(&MustNotRun),
            Err(GenerateError::KeysMustBeStrings)
        );
        output.string("key").unwrap();
        output.boolean(true).unwrap();
        output.end_object().unwrap();
        assert_eq!(output.serialized(&MustNotRun), Err(GenerateError::Complete));
        assert_eq!(output.serialized(&MustNotRun), Err(GenerateError::Complete));
    }
    #[test]
    fn serialized_values_share_stream_framing_without_consuming_rejected_values() {
        struct Rejected;
        impl serde::Serialize for Rejected {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("rejected value"))
            }
        }
        let mut generator = Generator::new();
        generator.start_array().unwrap();
        assert_eq!(
            generator.serialized(&Rejected),
            Err(GenerateError::InvalidState)
        );
        assert_eq!(generator.bytes(), b"[");
        generator
            .serialized(&(true, "quoted \"value\"", -1_i64))
            .unwrap();
        generator.clear();
        generator.serialized(&[1, 2]).unwrap();
        generator.end_array().unwrap();
        assert_eq!(generator.bytes(), b",[1,2]]");
    }
    #[test]
    fn scalar_encoding_uses_serde_and_preserves_key_order() {
        let mut generator = Generator::new();
        generator.start_object().unwrap();
        generator.string("z").unwrap();
        generator.string("quoted \"value\"\n雪").unwrap();
        generator.string("a").unwrap();
        generator.start_array().unwrap();
        generator.integer(i64::MIN).unwrap();
        generator.double(1.25).unwrap();
        generator.boolean(true).unwrap();
        generator.null().unwrap();
        generator.end_array().unwrap();
        generator.end_object().unwrap();
        assert_eq!(
            generator.bytes(),
            "{\"z\":\"quoted \\\"value\\\"\\n雪\",\"a\":[-9223372036854775808,1.25,true,null]}"
                .as_bytes()
        );
        let parsed: serde_json::Value = serde_json::from_slice(generator.bytes()).unwrap();
        assert_eq!(parsed["a"][0], i64::MIN);
    }
    #[test]
    fn clearing_preserves_never_closed_outer_array() {
        let mut generator = Generator::new();
        generator.start_array().unwrap();
        generator.start_object().unwrap();
        generator.end_object().unwrap();
        assert_eq!(generator.bytes(), b"[{}");
        generator.clear();
        generator.start_object().unwrap();
        generator.end_object().unwrap();
        assert_eq!(generator.bytes(), b",{}");
        generator.end_array().unwrap();
        assert_eq!(generator.bytes(), b",{}]");
    }
    #[test]
    fn rejected_values_leave_stream_ready_for_a_valid_retry() {
        let mut generator = Generator::new();
        generator.start_array().unwrap();
        assert_eq!(
            generator.double(f64::NAN),
            Err(GenerateError::InvalidNumber)
        );
        generator.null().unwrap();
        generator.end_array().unwrap();
        assert_eq!(generator.bytes(), b"[null]");

        let mut generator = Generator::new();
        generator.start_object().unwrap();
        assert_eq!(generator.integer(1), Err(GenerateError::KeysMustBeStrings));
        generator.string("key").unwrap();
        generator.boolean(true).unwrap();
        generator.end_object().unwrap();
        assert_eq!(generator.bytes(), br#"{"key":true}"#);
    }

    #[test]
    fn rejects_invalid_structure_and_non_finite_numbers() {
        let mut generator = Generator::new();
        generator.start_object().unwrap();
        assert_eq!(generator.integer(1), Err(GenerateError::KeysMustBeStrings));
        let mut generator = Generator::new();
        assert_eq!(
            generator.double(f64::NAN),
            Err(GenerateError::InvalidNumber)
        );
    }
}
