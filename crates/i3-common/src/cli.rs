//! Byte-preserving GNU-style option scanning without process-global getopt state.
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};

#[derive(Clone, Copy)]
pub struct OptionSpec {
    pub name: &'static str,
    pub short: u8,
    pub argument: bool,
    short_enabled: bool,
    long_enabled: bool,
    equivalent_return: u8,
}
impl OptionSpec {
    pub const fn flag(short: u8, name: &'static str) -> Self {
        Self {
            name,
            short,
            argument: false,
            short_enabled: true,
            long_enabled: true,
            equivalent_return: short,
        }
    }
    pub const fn value(short: u8, name: &'static str) -> Self {
        Self {
            name,
            short,
            argument: true,
            short_enabled: true,
            long_enabled: true,
            equivalent_return: short,
        }
    }
    pub const fn long_only(mut self) -> Self {
        self.short_enabled = false;
        self
    }
    pub const fn short_only(mut self) -> Self {
        self.long_enabled = false;
        self
    }
    /// GNU getopt treats long spellings returning the same value/argument mode
    /// as equivalent when resolving an abbreviated name.
    pub const fn equivalent_return(mut self, value: u8) -> Self {
        self.equivalent_return = value;
        self
    }
}
#[derive(Debug, PartialEq)]
pub enum Item {
    Option { short: u8, value: Option<OsString> },
    Operand(OsString),
    Error(String),
}

pub struct Parser<'a> {
    arguments: Vec<OsString>,
    index: usize,
    cluster: Vec<u8>,
    position: usize,
    options: &'a [OptionSpec],
    posix: bool,
    finished: bool,
}
impl<'a> Parser<'a> {
    pub fn new(
        arguments: impl IntoIterator<Item = OsString>,
        options: &'a [OptionSpec],
        posix: bool,
    ) -> Self {
        Self {
            arguments: arguments.into_iter().collect(),
            index: 0,
            cluster: Vec::new(),
            position: 0,
            options,
            posix,
            finished: false,
        }
    }
    /// Consume the action operand immediately following a nagbar button label.
    /// It is an argument even when it starts with a dash.
    pub fn take_argument(&mut self) -> Option<OsString> {
        let argument = self.arguments.get(self.index)?.clone();
        self.index += 1;
        Some(argument)
    }
    fn short(&mut self) -> Item {
        let flag = self.cluster[self.position];
        self.position += 1;
        let Some(spec) = self
            .options
            .iter()
            .find(|spec| spec.short_enabled && spec.short == flag)
        else {
            return Item::Error(format!("invalid option -- '{}'", char::from(flag)));
        };
        let value = if spec.argument {
            if self.position < self.cluster.len() {
                let value = OsString::from_vec(self.cluster[self.position..].to_vec());
                self.position = self.cluster.len();
                Some(value)
            } else {
                match self.take_argument() {
                    Some(value) => Some(value),
                    None => {
                        return Item::Error(format!(
                            "option requires an argument -- '{}'",
                            char::from(flag)
                        ))
                    }
                }
            }
        } else {
            None
        };
        Item::Option { short: flag, value }
    }
}
impl Iterator for Parser<'_> {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.cluster.len() {
            return Some(self.short());
        }
        let argument = self.take_argument()?;
        let bytes = argument.as_bytes();
        if self.finished || !bytes.starts_with(b"-") || bytes == b"-" {
            self.finished |= self.posix;
            return Some(Item::Operand(argument));
        }
        if bytes == b"--" {
            self.finished = true;
            return self.next();
        }
        if let Some(long) = bytes.strip_prefix(b"--") {
            let split = long.iter().position(|byte| *byte == b'=');
            let name = split.map_or(long, |at| &long[..at]);
            // GNU long options prefer an exact name over longer names sharing
            // its prefix (notably nagbar's button/button-no-terminal pair).
            let matches: Vec<_> = if let Some(exact) = self
                .options
                .iter()
                .find(|spec| spec.long_enabled && spec.name.as_bytes() == name)
            {
                vec![exact]
            } else {
                self.options
                    .iter()
                    .filter(|spec| spec.long_enabled && spec.name.as_bytes().starts_with(name))
                    .collect()
            };
            let spec = match matches.as_slice() {
                [spec] => **spec,
                specs
                    if !specs.is_empty()
                        && specs.iter().all(|spec| {
                            spec.equivalent_return == specs[0].equivalent_return
                                && spec.argument == specs[0].argument
                        }) =>
                {
                    **specs.first().unwrap()
                }
                [] => {
                    return Some(Item::Error(format!(
                        "unrecognized option '{}'",
                        argument.to_string_lossy()
                    )))
                }
                _ => {
                    return Some(Item::Error(format!(
                        "option '{}' is ambiguous; possibilities:{}",
                        argument.to_string_lossy(),
                        matches
                            .iter()
                            .map(|spec| format!(" '--{}'", spec.name))
                            .collect::<String>()
                    )))
                }
            };
            let value = if spec.argument {
                if let Some(at) = split {
                    Some(OsString::from_vec(long[at + 1..].to_vec()))
                } else {
                    match self.take_argument() {
                        Some(value) => Some(value),
                        None => {
                            return Some(Item::Error(format!(
                                "option '--{}' requires an argument",
                                spec.name
                            )))
                        }
                    }
                }
            } else {
                if split.is_some() {
                    return Some(Item::Error(format!(
                        "option '--{}' doesn't allow an argument",
                        spec.name
                    )));
                }
                None
            };
            return Some(Item::Option {
                short: spec.short,
                value,
            });
        }
        self.cluster = bytes[1..].to_vec();
        self.position = 0;
        Some(self.short())
    }
}

/// The accepted numeric prefix used by the legacy -l option. Arithmetic is
/// explicit and defined, including the narrowing behavior of atoi on Linux.
pub fn decimal_prefix(bytes: &[u8]) -> i32 {
    let bytes = bytes.trim_ascii_start();
    let (negative, digits) = match bytes.first() {
        Some(b'-') => (true, &bytes[1..]),
        Some(b'+') => (false, &bytes[1..]),
        _ => (false, bytes),
    };
    let value = digits
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .fold(0i32, |value, byte| {
            value.wrapping_mul(10).wrapping_add(i32::from(byte - b'0'))
        });
    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const OPTIONS: &[OptionSpec] = &[
        OptionSpec::flag(b'v', "version"),
        OptionSpec::flag(b'V', "verbose"),
        OptionSpec::value(b's', "socket"),
    ];
    fn parser(values: &[&[u8]], posix: bool) -> Parser<'static> {
        Parser::new(
            values
                .iter()
                .map(|value| OsString::from_vec(value.to_vec())),
            OPTIONS,
            posix,
        )
    }
    #[test]
    fn permutation_clusters_and_raw_arguments() {
        let items: Vec<_> = parser(
            &[b"operand", b"-Vv", b"--sock=/tmp/\xff", b"--", b"-v"],
            false,
        )
        .collect();
        assert_eq!(
            items,
            vec![
                Item::Operand("operand".into()),
                Item::Option {
                    short: b'V',
                    value: None
                },
                Item::Option {
                    short: b'v',
                    value: None
                },
                Item::Option {
                    short: b's',
                    value: Some(OsString::from_vec(b"/tmp/\xff".to_vec()))
                },
                Item::Operand("-v".into())
            ]
        );
    }
    #[test]
    fn option_errors_and_posix_operands() {
        assert_eq!(
            parser(&[b"--v", b"--soc", b"-x"], false).collect::<Vec<_>>(),
            vec![
                Item::Error(
                    "option '--v' is ambiguous; possibilities: '--version' '--verbose'".into()
                ),
                Item::Option {
                    short: b's',
                    value: Some("-x".into())
                }
            ]
        );
        assert_eq!(
            parser(&[b"--socket"], false).next(),
            Some(Item::Error("option '--socket' requires an argument".into()))
        );
        assert_eq!(
            parser(&[b"operand", b"-V"], true).collect::<Vec<_>>(),
            vec![Item::Operand("operand".into()), Item::Operand("-V".into())]
        );
    }
    #[test]
    fn explicit_action_operand_is_not_an_option() {
        let mut options = parser(&[b"-slabel", b"--not-an-option", b"-v"], false);
        assert_eq!(
            options.next(),
            Some(Item::Option {
                short: b's',
                value: Some("label".into())
            })
        );
        assert_eq!(options.take_argument(), Some("--not-an-option".into()));
        assert_eq!(
            options.next(),
            Some(Item::Option {
                short: b'v',
                value: None
            })
        );
    }
    #[test]
    fn exact_long_name_wins_over_longer_prefix_matches() {
        let options = [
            OptionSpec::value(b'b', "button"),
            OptionSpec::value(b'B', "button-no-terminal"),
        ];
        let mut parser = Parser::new(["--button", "label"].map(OsString::from), &options, false);
        assert_eq!(
            parser.next(),
            Some(Item::Option {
                short: b'b',
                value: Some("label".into())
            })
        );
        let mut parser = Parser::new(["--but"].map(OsString::from), &options, false);
        assert!(matches!(parser.next(), Some(Item::Error(_))));
    }
    #[test]
    fn decimal_prefix_keeps_atoi_accepted_inputs() {
        for (bytes, expected) in [
            (b"  -12px".as_slice(), -12),
            (b"+42", 42),
            (b"bad", 0),
            (b"", 0),
            (b"4294967297", 1),
        ] {
            assert_eq!(decimal_prefix(bytes), expected);
        }
    }
}
