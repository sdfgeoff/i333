//! JSON generation and incremental parsing implemented entirely in Rust.
//!
//! Scalar encoding and decoding use serde_json. The event parser also handles
//! status streams whose outer array stays open across arbitrarily many updates.

mod generator;
mod parser;

pub use generator::{GenerateError, Generator};
pub use parser::{Event, EventHandler, ParseError, ParseErrorKind, Parser, ParserOptions};
