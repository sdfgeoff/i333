//! Owned captures shared by the command and configuration parsers.
#![deny(unsafe_op_in_unsafe_fn)]

#[derive(Debug, Clone)]
enum Capture {
    String(Vec<u8>),
    Long(i64),
}

#[derive(Debug, Clone)]
struct Entry {
    identifier: Vec<u8>,
    value: Capture,
}

/// Parser-local Rust storage. Construct normally; never calloc/memset this type.
/// Captures own bounded bytes. Dispatchers snapshot values before callbacks;
/// no borrowed parser input or identifier escapes a push.
#[derive(Debug, Default, Clone)]
pub struct ParserStack {
    entries: Vec<Entry>,
}

impl ParserStack {
    fn full(&self) -> bool {
        self.entries.len() == 10
    }

    pub(crate) fn push_string(&mut self, identifier: &[u8], value: &[u8]) {
        let identifier = first_nul(identifier);
        let value = first_nul(value);
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.identifier == identifier)
        {
            if let Capture::String(previous) = &mut entry.value {
                let mut joined = previous.clone();
                joined.push(b',');
                joined.extend_from_slice(value);
                *previous = joined;
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

    pub(crate) fn push_long(&mut self, identifier: &[u8], value: i64) {
        let identifier = first_nul(identifier);
        if self.full() {
            full_stack();
        }
        self.entries.push(Entry {
            identifier: identifier.to_owned(),
            value: Capture::Long(value),
        });
    }

    pub(crate) fn get_string(&self, identifier: &[u8]) -> Option<&[u8]> {
        let identifier = first_nul(identifier);
        self.entries
            .iter()
            .find(|entry| entry.identifier == identifier)
            .and_then(|entry| {
                if let Capture::String(value) = &entry.value {
                    Some(value.as_slice())
                } else {
                    None
                }
            })
    }

    pub(crate) fn get_long(&self, identifier: &[u8]) -> i64 {
        let identifier = first_nul(identifier);
        self.entries
            .iter()
            .find(|entry| entry.identifier == identifier)
            .map_or(0, |entry| {
                if let Capture::Long(value) = entry.value {
                    value
                } else {
                    0
                }
            })
    }
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }
}

fn first_nul(bytes: &[u8]) -> &[u8] {
    bytes.split(|&byte| byte == 0).next().unwrap()
}

fn full_stack() -> ! {
    // SAFETY: the static format takes no arguments; preserve native stderr text
    // and process exit instead of unwinding through graph/parser callbacks.
    unsafe {
        libc::fprintf(i3_common::native_stdio::stderr, c"BUG: parser stack full. This means either a bug in the code, or a new command which contains more than 10 identified tokens.\n".as_ptr())
    };
    std::process::exit(libc::EXIT_FAILURE)
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

pub(crate) fn expected_tokens(names: &[&[u8]], skip_error: bool) -> Vec<u8> {
    let mut output = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let bytes = *name;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_own_sources_join_identifiers_and_reset_independently() {
        let mut stack = ParserStack::default();
        {
            let id = b"name".to_vec();
            let value = vec![0xFF, b'a'];
            stack.push_string(&id, &value);
        }
        stack.push_string(b"name", b"second");
        stack.push_string(b"empty", b"");
        stack.push_long(b"number", i64::MIN);
        assert_eq!(stack.get_string(b"name").unwrap(), b"\xffa,second");
        assert_eq!(stack.get_string(b"empty"), Some(b"".as_slice()));
        assert_eq!(stack.get_long(b"number"), i64::MIN);
        assert_eq!(stack.get_string(b"missing"), None);
        assert_eq!(stack.get_long(b"missing"), 0);
        stack.push_string(b"name\0ignored", b"third\0ignored\xff");
        assert_eq!(
            stack.get_string(b"name\0other"),
            Some(b"\xffa,second,third".as_slice())
        );
        let copied = stack.clone();
        stack.clear();
        stack.clear();
        assert_eq!(
            copied.get_string(b"name"),
            Some(b"\xffa,second,third".as_slice())
        );
        assert_eq!(copied.get_string(b"empty"), Some(b"".as_slice()));
        assert_eq!(copied.get_long(b"number"), i64::MIN);
        stack.push_long(b"fresh", 42);
        assert_eq!(stack.entries.len(), 1);
        assert_eq!(stack.get_long(b"fresh"), 42);
    }

    #[test]
    fn capture_limit_retains_native_fatal_behavior_at_the_eleventh_entry() {
        const CHILD: &str = "I333_CAPTURE_LIMIT_CHILD";
        let mut stack = ParserStack::default();
        for value in 0..10 {
            stack.push_long(b"number", value);
        }
        assert_eq!(stack.entries.len(), 10);
        assert_eq!(stack.get_long(b"number"), 0); // duplicate long identifiers keep the first match
        if std::env::var_os(CHILD).is_some() {
            stack.push_long(b"overflow", 10);
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
            expected_tokens(&[b"'foo", b"number", b"error"], false),
            b"'foo', <number>, <error>"
        );
        assert_eq!(
            expected_tokens(&[b"'foo", b"number", b"error"], true),
            b"'foo', <number>, "
        );
    }

    #[test]
    fn command_and_config_lexers_keep_distinct_escaping_and_consumed_offsets() {
        type StringCase<'a> = (&'a [u8], bool, Option<&'a [u8]>, usize);
        let common: &[StringCase<'_>] = &[
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
