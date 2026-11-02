//! Exceptional native JSON output only. Ordinary messages use Serde directly.
//!
//! i3 historically rejects invalid native strings and non-finite numbers without
//! consuming their value slot. Later keys can therefore become values, or output
//! can stop midway through an object. Serde deliberately cannot express that
//! malformed protocol. This small formatter retains just those rejection and
//! depth-prefix rules; Serde still encodes every emitted scalar.
#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    KeysMustBeStrings,
    DepthExceeded,
    InvalidState,
    Complete,
    InvalidNumber,
}
#[derive(Clone, Copy)]
enum Scope {
    Array { populated: bool },
    Object { populated: bool, waiting: bool },
}
#[derive(Default)]
pub struct RejectedOutput {
    bytes: Vec<u8>,
    scopes: Vec<Scope>,
    root: bool,
    poisoned: Option<Rejection>,
}
impl RejectedOutput {
    pub fn new() -> Self {
        Self::default()
    }
    /// Switch an already-started command/click array to rejection handling.
    /// `bytes` is the trusted Serde prefix; it may be empty after a stream flush.
    pub fn resume_array(bytes: Vec<u8>, populated: bool) -> Self {
        Self {
            bytes,
            scopes: vec![Scope::Array { populated }],
            root: true,
            poisoned: None,
        }
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn clear(&mut self) {
        self.bytes.clear();
    }
    fn reject(&mut self, why: Rejection) -> Result<(), Rejection> {
        if !matches!(why, Rejection::KeysMustBeStrings | Rejection::InvalidNumber) {
            self.poisoned = Some(why);
        }
        Err(why)
    }
    fn slot(&mut self, text: bool) -> Result<(), Rejection> {
        if let Some(why) = self.poisoned {
            return Err(why);
        }
        match self.scopes.last_mut() {
            Some(Scope::Array { populated }) => {
                if *populated {
                    self.bytes.push(b',');
                }
                *populated = true;
            }
            Some(Scope::Object { populated, waiting }) => {
                if *waiting {
                    self.bytes.push(b':');
                    *waiting = false;
                    *populated = true;
                } else if text {
                    if *populated {
                        self.bytes.push(b',');
                    }
                    *waiting = true;
                } else {
                    return self.reject(Rejection::KeysMustBeStrings);
                }
            }
            None if self.root => return self.reject(Rejection::Complete),
            None => self.root = true,
        }
        Ok(())
    }
    fn scalar<T: serde::Serialize + ?Sized>(
        &mut self,
        value: &T,
        text: bool,
    ) -> Result<(), Rejection> {
        self.slot(text)?;
        serde_json::to_writer(&mut self.bytes, value).expect("validated scalar writing to Vec");
        Ok(())
    }
    pub fn string(&mut self, value: &str) -> Result<(), Rejection> {
        self.scalar(value, true)
    }
    pub fn integer(&mut self, value: i64) -> Result<(), Rejection> {
        self.scalar(&value, false)
    }
    pub fn boolean(&mut self, value: bool) -> Result<(), Rejection> {
        self.scalar(&value, false)
    }
    pub fn null(&mut self) -> Result<(), Rejection> {
        self.scalar(&(), false)
    }
    pub fn double(&mut self, value: f64) -> Result<(), Rejection> {
        if !value.is_finite() {
            return self.reject(Rejection::InvalidNumber);
        }
        self.scalar(&value, false)
    }
    fn open(&mut self, scope: Scope, delimiter: u8) -> Result<(), Rejection> {
        if self.scopes.len() >= 128 {
            return self.reject(Rejection::DepthExceeded);
        }
        self.slot(false)?;
        self.bytes.push(delimiter);
        self.scopes.push(scope);
        Ok(())
    }
    pub fn start_array(&mut self) -> Result<(), Rejection> {
        self.open(Scope::Array { populated: false }, b'[')
    }
    pub fn start_object(&mut self) -> Result<(), Rejection> {
        self.open(
            Scope::Object {
                populated: false,
                waiting: false,
            },
            b'{',
        )
    }
    fn close(&mut self, object: bool) -> Result<(), Rejection> {
        let matches = matches!(
            (self.scopes.last(), object),
            (Some(Scope::Array { .. }), false) | (Some(Scope::Object { waiting: false, .. }), true)
        );
        if !matches {
            return self.reject(Rejection::InvalidState);
        }
        // Matching closes are still accepted after a previous sticky error.
        self.scopes.pop();
        self.bytes.push(if object { b'}' } else { b']' });
        Ok(())
    }
    pub fn end_array(&mut self) -> Result<(), Rejection> {
        self.close(false)
    }
    pub fn end_object(&mut self) -> Result<(), Rejection> {
        self.close(true)
    }
}
