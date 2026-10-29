//! Owned captures shared by the command and configuration parsers.
#![deny(unsafe_op_in_unsafe_fn)]
use std::ffi::{CStr, CString};

#[derive(Debug)]
enum Capture {
    String(CString),
    Long(libc::c_long),
}

#[derive(Debug)]
struct Entry {
    identifier: CString,
    value: Capture,
}

/// Parser-local Rust storage. Construct normally; never calloc/memset this type.
/// CString buffers keep callback arguments stable until that capture is changed
/// or cleared. No borrowed parser input or identifier pointer escapes a push.
#[derive(Debug, Default)]
pub struct ParserStack {
    entries: Vec<Entry>,
}

impl ParserStack {
    fn full(&self) -> bool {
        self.entries.len() == 10
    }

    fn push_string(&mut self, identifier: &CStr, value: &CStr) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.identifier.as_c_str() == identifier)
        {
            if let Capture::String(previous) = &mut entry.value {
                let mut joined = previous.as_bytes().to_vec();
                joined.push(b',');
                joined.extend_from_slice(value.to_bytes());
                *previous = CString::new(joined).expect("C string captures contain no NUL");
                return;
            }
            // Generated grammar must never append a string to a numeric capture.
            // The old union read was invalid here; diagnose the grammar violation.
            panic!("parser identifier changes capture type");
        }
        if self.full() {
            full_stack();
        }
        self.entries.push(Entry {
            identifier: identifier.to_owned(),
            value: Capture::String(value.to_owned()),
        });
    }

    fn push_long(&mut self, identifier: &CStr, value: libc::c_long) {
        if self.full() {
            full_stack();
        }
        self.entries.push(Entry {
            identifier: identifier.to_owned(),
            value: Capture::Long(value),
        });
    }

    fn get_string(&self, identifier: &CStr) -> Option<&CStr> {
        self.entries
            .iter()
            .find(|entry| entry.identifier.as_c_str() == identifier)
            .and_then(|entry| {
                if let Capture::String(value) = &entry.value {
                    Some(value.as_c_str())
                } else {
                    None
                }
            })
    }

    fn get_long(&self, identifier: &CStr) -> libc::c_long {
        self.entries
            .iter()
            .find(|entry| entry.identifier.as_c_str() == identifier)
            .map_or(0, |entry| {
                if let Capture::Long(value) = entry.value {
                    value
                } else {
                    0
                }
            })
    }
}

fn full_stack() -> ! {
    // SAFETY: the static format takes no arguments; preserve native stderr text
    // and process exit instead of unwinding through graph/parser callbacks.
    unsafe {
        libc::fprintf(i3_common::native_stdio::stderr, c"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr())
    };
    std::process::exit(libc::EXIT_FAILURE)
}

/// # Safety
/// stack is a live normally constructed parser-local stack, uniquely borrowed
/// during this call. identifier/value are live C strings, copied before return.
pub unsafe fn parser_push_string(
    stack: *mut ParserStack,
    identifier: *const libc::c_char,
    value: *const libc::c_char,
) {
    // SAFETY: caller supplies the exclusive stack and live C strings.
    unsafe { (&mut *stack).push_string(CStr::from_ptr(identifier), CStr::from_ptr(value)) };
}

/// # Safety
/// stack is live and exclusively borrowed here; identifier is a live C string.
pub unsafe fn parser_push_long(
    stack: *mut ParserStack,
    identifier: *const libc::c_char,
    value: libc::c_long,
) {
    unsafe { (&mut *stack).push_long(CStr::from_ptr(identifier), value) };
}

/// # Safety
/// stack and identifier remain live during the call. The result borrows a capture
/// until its next mutation/clear or stack destruction; it must never be freed.
pub unsafe fn parser_get_string(
    stack: *const ParserStack,
    identifier: *const libc::c_char,
) -> *const libc::c_char {
    unsafe { (&*stack).get_string(CStr::from_ptr(identifier)) }
        .map_or(std::ptr::null(), CStr::as_ptr)
}

/// # Safety
/// stack and identifier are readable during this call, without concurrent mutation.
pub unsafe fn parser_get_long(
    stack: *const ParserStack,
    identifier: *const libc::c_char,
) -> libc::c_long {
    unsafe { (&*stack).get_long(CStr::from_ptr(identifier)) }
}

/// # Safety
/// stack is a live normally constructed parser stack, exclusively borrowed here.
/// Previously returned capture pointers must no longer be used.
pub unsafe fn parser_clear_stack(stack: *mut ParserStack) {
    unsafe { (&mut *stack).entries.clear() };
}

/// The two grammars have intentionally different quoting and string terminators.
#[derive(Clone, Copy)]
pub(crate) enum StringGrammar {
    Command,
    Config,
}

pub(crate) struct LexedString {
    pub bytes: Option<Vec<u8>>,
    pub consumed: usize,
}

pub(crate) fn lex_string(input: &[u8], as_word: bool, grammar: StringGrammar) -> LexedString {
    let end = input
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(input.len());
    let input = &input[..end];
    let quoted = input.first() == Some(&b'"');
    let beginning = usize::from(quoted);
    let mut cursor = beginning;
    while cursor < input.len() {
        let byte = input[cursor];
        if quoted {
            match grammar {
                StringGrammar::Command => {
                    if byte == b'"' {
                        break;
                    }
                    if byte == b'\\' && cursor + 1 < input.len() {
                        cursor += 1;
                    }
                }
                StringGrammar::Config => {
                    if byte == b'"' && input.get(cursor.wrapping_sub(1)) != Some(&b'\\') {
                        break;
                    }
                }
            }
        } else if as_word {
            if b" \t],;\r\n".contains(&byte) {
                break;
            }
        } else {
            let separator = match grammar {
                StringGrammar::Command => b";,\r\n".contains(&byte),
                StringGrammar::Config => b"\r\n".contains(&byte),
            };
            if separator {
                break;
            }
        }
        cursor += 1;
    }
    let mut bytes = Vec::with_capacity(cursor.saturating_sub(beginning));
    let mut position = beginning;
    while position < cursor {
        if input[position] == b'\\'
            && input.get(position + 1).is_some_and(|&next| {
                next == b'"' || (matches!(grammar, StringGrammar::Command) && next == b'\\')
            })
        {
            position += 1;
        }
        bytes.push(input[position]);
        position += 1;
    }
    LexedString {
        bytes: (cursor != beginning).then_some(bytes),
        consumed: cursor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_own_sources_join_identifiers_and_reset_independently() {
        let mut stack = ParserStack::default();
        {
            let id = CString::new("name").unwrap();
            let value = CString::new(vec![0xFF, b'a']).unwrap();
            stack.push_string(&id, &value);
        }
        stack.push_string(c"name", c"second");
        stack.push_string(c"empty", c"");
        stack.push_long(c"number", libc::c_long::MIN);
        assert_eq!(
            stack.get_string(c"name").unwrap().to_bytes(),
            b"\xffa,second"
        );
        assert_eq!(stack.get_string(c"empty"), Some(c""));
        assert_eq!(stack.get_long(c"number"), libc::c_long::MIN);
        assert_eq!(stack.get_string(c"missing"), None);
        assert_eq!(stack.get_long(c"missing"), 0);
        // SAFETY: the normal Rust stack is uniquely borrowed and no capture views remain.
        unsafe {
            parser_clear_stack(&mut stack);
            parser_clear_stack(&mut stack);
        }
        stack.push_long(c"fresh", 42);
        assert_eq!(stack.entries.len(), 1);
        assert_eq!(stack.get_long(c"fresh"), 42);
    }

    #[test]
    fn capture_limit_retains_native_fatal_behavior_at_the_eleventh_entry() {
        const CHILD: &str = "I333_CAPTURE_LIMIT_CHILD";
        let mut stack = ParserStack::default();
        for value in 0..10 {
            stack.push_long(c"number", value);
        }
        assert_eq!(stack.entries.len(), 10);
        assert_eq!(stack.get_long(c"number"), 0); // duplicate long identifiers keep the first match
        if std::env::var_os(CHILD).is_some() {
            stack.push_long(c"overflow", 10);
            panic!("full stack must terminate");
        }
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "parser_util::tests::capture_limit_retains_native_fatal_behavior_at_the_eleventh_entry", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert_eq!(result.status.code(), Some(libc::EXIT_FAILURE));
        assert!(String::from_utf8_lossy(&result.stderr).contains("BUG: parser stack full."));
    }

    #[test]
    fn expected_token_text_preserves_skipped_recovery_separator_bytes() {
        assert_eq!(
            expected_tokens(&[c"'foo", c"number", c"error"], false),
            b"'foo', <number>, <error>"
        );
        assert_eq!(
            expected_tokens(&[c"'foo", c"number", c"error"], true),
            b"'foo', <number>, "
        );
    }

    #[test]
    fn command_and_config_lexers_keep_distinct_escaping_and_consumed_offsets() {
        let common: &[(&[u8], bool, Option<&[u8]>, usize)] = &[
            (b"", false, None, 0),
            (b"\"\"", false, None, 1),
            (b"\"abc", false, Some(b"abc"), 4),
            (b"abc\\", false, Some(b"abc\\"), 4),
            (b"\"abc\\", false, Some(b"abc\\"), 5),
            (br#""a\"b"tail"#, false, Some(b"a\"b"), 5),
            (b"name]tail", true, Some(b"name"), 4),
            (b"\xffa", true, Some(b"\xffa"), 2),
            (b"a\0tail", false, Some(b"a"), 1),
        ];
        for grammar in [StringGrammar::Command, StringGrammar::Config] {
            for &(input, word, expected, consumed) in common {
                let actual = lex_string(input, word, grammar);
                assert_eq!(actual.bytes.as_deref(), expected, "{input:?}");
                assert_eq!(actual.consumed, consumed, "{input:?}");
            }
        }
        let command = lex_string(b"a,b;c\nnext", false, StringGrammar::Command);
        let config = lex_string(b"a,b;c\nnext", false, StringGrammar::Config);
        assert_eq!(
            (command.bytes.as_deref(), command.consumed),
            (Some(b"a".as_slice()), 1)
        );
        assert_eq!(
            (config.bytes.as_deref(), config.consumed),
            (Some(b"a,b;c".as_slice()), 5)
        );
        let command = lex_string(br#""a\\b"tail"#, false, StringGrammar::Command);
        let config = lex_string(br#""a\\b"tail"#, false, StringGrammar::Config);
        assert_eq!(command.bytes.as_deref(), Some(b"a\\b".as_slice()));
        assert_eq!(config.bytes.as_deref(), Some(b"a\\\\b".as_slice()));
        assert_eq!(command.consumed, 5);
        assert_eq!(config.consumed, 5);
        let command = lex_string(br#""a\\"tail"#, false, StringGrammar::Command);
        let config = lex_string(br#""a\\"tail"#, false, StringGrammar::Config);
        assert_eq!(
            (command.bytes.as_deref(), command.consumed),
            (Some(b"a\\".as_slice()), 4)
        );
        assert_eq!(
            (config.bytes.as_deref(), config.consumed),
            (Some(b"a\\\"tail".as_slice()), 9)
        );
    }
}

pub(crate) fn expected_tokens(names: &[&CStr], skip_error: bool) -> Vec<u8> {
    let mut output = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let bytes = name.to_bytes();
        if skip_error && bytes == b"error" {
            continue;
        }
        if let Some(literal) = bytes.strip_prefix(b"'") {
            output.push(b'\'');
            output.extend_from_slice(literal);
            output.push(b'\'');
        } else {
            output.push(b'<');
            output.extend_from_slice(bytes);
            output.push(b'>');
        }
        // Preserve the old diagnostic's separator position even when an error
        // recovery token is skipped at the end of the generated alternatives.
        if index + 1 < names.len() {
            output.extend_from_slice(b", ");
        }
    }
    output
}
