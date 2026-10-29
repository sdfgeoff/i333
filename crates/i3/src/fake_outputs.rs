/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * Faking outputs is useful in pathological situations (like network X servers
 * which don’t support multi-monitor in a useful way) and for our testsuite.
 *
 */
//! Borrowed geometry parsing, followed by the legacy output-graph boundary.
#![deny(unsafe_op_in_unsafe_fn)]

use crate::log::{debuglog, errorlog};
use crate::output::output_primary_name;
use crate::randr::{init_ws_for_output, output_init_con, outputs};
use crate::types::{output_name, xoutput as Output, Rect};
use i3_common::safewrappers::{scalloc, sstrdup};
use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicI32, Ordering};

// The name counter deliberately survives repeated initialization calls.
static NUM_SCREENS: AtomicI32 = AtomicI32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ParsedOutput {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    primary: bool,
}

impl ParsedOutput {
    fn rect(self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn shrink(self, rect: &mut Rect) {
        // The original min helper takes signed int, even for unsigned extents.
        // Retain that ordering rather than silently fixing very large widths.
        rect.width = (rect.width as i32).min(self.width as i32) as u32;
        rect.height = (rect.height as i32).min(self.height as i32) as u32;
    }
}

/// Decode scanf's unsigned decimal prefix on this Linux LP64 target. Overflow
/// saturates at ULONG_MAX before conversion to u32; an in-range minus sign
/// negates modulo ULONG_MAX+1. Every remaining digit still counts as consumed.
fn unsigned_decimal<'a>(
    mut input: &'a [u8],
    is_space: &impl Fn(u8) -> bool,
) -> Option<(u32, &'a [u8])> {
    while input.first().is_some_and(|byte| is_space(*byte)) {
        input = &input[1..];
    }
    let negative = input.first() == Some(&b'-');
    if matches!(input.first(), Some(b'+' | b'-')) {
        input = &input[1..];
    }
    let mut value = 0u64;
    let mut overflow = false;
    let mut any_digit = false;
    while let Some((&digit, tail)) = input.split_first() {
        if !digit.is_ascii_digit() {
            break;
        }
        any_digit = true;
        value = match value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u64::from(digit - b'0')))
        {
            Some(value) => value,
            None => {
                overflow = true;
                u64::MAX
            }
        };
        input = tail;
    }
    if !any_digit {
        return None;
    }
    if negative && !overflow {
        value = value.wrapping_neg();
    }
    Some((value as u32, input))
}

fn delimiter(input: &[u8], expected: u8) -> Option<&[u8]> {
    match input.split_first() {
        Some((&actual, tail)) if actual == expected => Some(tail),
        _ => None,
    }
}

fn parse_output<'a>(
    input: &'a [u8],
    is_space: &impl Fn(u8) -> bool,
) -> Option<(ParsedOutput, &'a [u8])> {
    let (width, tail) = unsigned_decimal(input, is_space)?;
    let (height, tail) = unsigned_decimal(delimiter(tail, b'x')?, is_space)?;
    let (x, tail) = unsigned_decimal(delimiter(tail, b'+')?, is_space)?;
    let (y, mut tail) = unsigned_decimal(delimiter(tail, b'+')?, is_space)?;
    let primary = tail.first() == Some(&b'P');
    if primary {
        tail = &tail[1..];
    }
    if tail.first() == Some(&b',') {
        tail = &tail[1..];
    }
    Some((
        ParsedOutput {
            width,
            height,
            x,
            y,
            primary,
        },
        tail,
    ))
}

fn native_space(byte: u8) -> bool {
    // SAFETY: isspace requires EOF or an unsigned-char value. u8 supplies the
    // latter and retains scanf's current-locale whitespace behavior.
    unsafe { libc::isspace(i32::from(byte)) != 0 }
}

struct ParsedOutputs<'a> {
    remaining: &'a [u8],
}

impl<'a> ParsedOutputs<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            remaining: bytes.split(|byte| *byte == 0).next().unwrap_or_default(),
        }
    }
}

impl Iterator for ParsedOutputs<'_> {
    type Item = ParsedOutput;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((output, tail)) = parse_output(self.remaining, &native_space) else {
            self.remaining = &[];
            return None;
        };
        self.remaining = tail;
        Some(output)
    }
}
impl std::iter::FusedIterator for ParsedOutputs<'_> {}

/// # Safety
/// The caller owns the event-thread output graph for this traversal and every
/// queued output is a live allocation with valid intrusive links.
unsafe fn get_screen_at(x: u32, y: u32) -> *mut Output {
    // SAFETY: the caller guarantees the graph and its links stay valid.
    unsafe {
        let mut output = outputs.tqh_first;
        while !output.is_null() {
            if (*output).rect.x == x && (*output).rect.y == y {
                return output;
            }
            output = (*output).outputs.tqe_next;
        }
    }
    std::ptr::null_mut()
}

/// # Safety
/// The event-thread graph and renderer must be initialized and exclusively
/// available to this call. Existing graph consumers own/free its malloc nodes.
unsafe fn attach_output(parsed: ParsedOutput) {
    // SAFETY: this is the single raw graph boundary; all geometry and input
    // traversal above it use ordinary borrowed bytes and checked Rust values.
    unsafe {
        debuglog(
            c"%s:%s:%d - Parsed output as width = %u, height = %u at (%u, %u)%s\n"
                .as_ptr()
                .cast_mut(),
            c"../src/fake_outputs.c".as_ptr(),
            c"fake_outputs_init".as_ptr(),
            53,
            parsed.width,
            parsed.height,
            parsed.x,
            parsed.y,
            if parsed.primary {
                c" (primary)".as_ptr()
            } else {
                c"".as_ptr()
            },
        );
        let mut output = get_screen_at(parsed.x, parsed.y);
        if !output.is_null() {
            debuglog(
                c"%s:%s:%d - Re-used old output %p\n".as_ptr().cast_mut(),
                c"../src/fake_outputs.c".as_ptr(),
                c"fake_outputs_init".as_ptr(),
                57,
                output,
            );
            parsed.shrink(&mut (*output).rect);
        } else {
            let name = CString::new(format!("fake-{}", NUM_SCREENS.load(Ordering::Relaxed)))
                .expect("an integer output name contains no NUL");
            // These nodes escape into the existing graph and are freed with
            // libc::free. Rust Box/CString raw allocations must not be handed
            // to that ownership contract.
            let output_name = scalloc(1, std::mem::size_of::<output_name>()).cast::<output_name>();
            output = scalloc(1, std::mem::size_of::<Output>()).cast::<Output>();
            (*output_name).name = sstrdup(name.as_ptr());
            (*output_name).names.sle_next = std::ptr::null_mut();
            (*output).names_head.slh_first = output_name;
            debuglog(
                c"%s:%s:%d - Created new fake output %s (%p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/fake_outputs.c".as_ptr(),
                c"fake_outputs_init".as_ptr(),
                68,
                output_primary_name(output),
                output,
            );
            (*output).active = true;
            (*output).rect = parsed.rect();
            if parsed.x == 0 && parsed.y == 0 {
                (*output).outputs.tqe_next = outputs.tqh_first;
                if !outputs.tqh_first.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*output).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*output).outputs.tqe_next;
                }
                outputs.tqh_first = output;
                (*output).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*output).outputs.tqe_next = std::ptr::null_mut();
                (*output).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = output;
                outputs.tqh_last = &raw mut (*output).outputs.tqe_next;
            }
            output_init_con(output);
            init_ws_for_output(output);
            NUM_SCREENS.fetch_add(1, Ordering::Relaxed);
        }
        (*output).primary = parsed.primary;
    }
}

/// Initialize outputs from `WIDTHxHEIGHT+X+Y[P][,]` prefixes.
///
/// # Safety
/// output_spec is a live NUL-terminated string. The caller has exclusive
/// event-thread access to the initialized output/container graph and renderer.
pub unsafe fn fake_outputs_init(output_spec: *const libc::c_char) {
    // SAFETY: the caller supplies a live C string; no pointer walking escapes.
    let specification = unsafe { CStr::from_ptr(output_spec) };
    for parsed in ParsedOutputs::new(specification.to_bytes()) {
        // SAFETY: the caller owns the initialized graph throughout this call.
        unsafe { attach_output(parsed) };
    }
    if NUM_SCREENS.load(Ordering::Relaxed) == 0 {
        // SAFETY: a fixed C format with no arguments, followed by process exit.
        unsafe {
            errorlog(
                c"ERROR: No screens found. Please fix your setup. i3 will exit now.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn native_prefix(bytes: &[u8]) -> Option<(ParsedOutput, usize)> {
        let bytes = bytes.split(|byte| *byte == 0).next().unwrap_or_default();
        let input = CString::new(bytes).unwrap();
        let (mut width, mut height, mut x, mut y) = (0u32, 0u32, 0u32, 0u32);
        let mut consumed = 0i32;
        // SAFETY: fixed format with four writable unsigned-int values and a
        // writable int for %n. input is a complete borrowed C string.
        let count = unsafe {
            libc::sscanf(
                input.as_ptr(),
                c"%ux%u+%u+%u%n".as_ptr(),
                &mut width,
                &mut height,
                &mut x,
                &mut y,
                &mut consumed,
            )
        };
        if count != 4 {
            return None;
        }
        let mut consumed = usize::try_from(consumed).expect("small test input count");
        let primary = bytes.get(consumed) == Some(&b'P');
        if primary {
            consumed += 1;
        }
        if bytes.get(consumed) == Some(&b',') {
            consumed += 1;
        }
        Some((
            ParsedOutput {
                width,
                height,
                x,
                y,
                primary,
            },
            consumed,
        ))
    }

    fn compare_native(bytes: &[u8]) {
        let rust = parse_output(bytes, &native_space)
            .map(|(output, tail)| (output, bytes.len() - tail.len()));
        assert_eq!(rust, native_prefix(bytes), "input: {bytes:?}");
    }

    #[test]
    fn decimal_grammar_and_extreme_values_match_original_scanf() {
        let forms: &[&[u8]] = &[
            b"0",
            b"00",
            b"1",
            b"00004",
            b"4294967295",
            b"4294967296",
            b"18446744073709551614",
            b"18446744073709551615",
            b"18446744073709551616",
            b"9999999999999999999999999999999999999",
            b"-0",
            b"-1",
            b"-4294967296",
            b"-18446744073709551615",
            b"-18446744073709551616",
            b"+0",
            b"+1",
            b"+18446744073709551616",
            b" \t+42",
            b"\n-42",
            b"\x0b\x0c42",
            b"",
            b"+",
            b"-",
            b"--1",
            b"+-1",
            b"-+1",
            b"+ 1",
            b"1e3",
            b"3.5",
            b"0x10",
            b"0X10",
            b"_1",
        ];
        for value in forms {
            for position in 0..4 {
                for suffix in [b"".as_slice(), b"P,tail", b",2x3+4+5", b" PP,"] {
                    let mut input = Vec::new();
                    for (index, separator) in [b"x".as_slice(), b"+", b"+", b""].iter().enumerate()
                    {
                        input.extend_from_slice(if index == position { value } else { b"1" });
                        input.extend_from_slice(separator);
                    }
                    input.extend_from_slice(suffix);
                    compare_native(&input);
                }
            }
        }
        for byte in 0..=255u8 {
            for (prefix, suffix) in [
                (b"".as_slice(), b"1x2+3+4".as_slice()),
                (b"1x", b"2+3+4"),
                (b"1x2", b"+3+4"),
                (b"1x2+3+", b"4P,"),
            ] {
                let mut input = prefix.to_vec();
                input.push(byte);
                input.extend_from_slice(suffix);
                compare_native(&input);
            }
        }
    }

    #[test]
    fn iteration_keeps_valid_prefixes_and_exact_optional_delimiters() {
        let values: Vec<_> =
            ParsedOutputs::new(b"1920x1080+0+0P,1280x720+1920+0junk,1x1+1+1").collect();
        assert_eq!(
            values,
            [
                ParsedOutput {
                    width: 1920,
                    height: 1080,
                    x: 0,
                    y: 0,
                    primary: true
                },
                ParsedOutput {
                    width: 1280,
                    height: 720,
                    x: 1920,
                    y: 0,
                    primary: false
                },
            ]
        );
        // Delimiters are optional; whitespace belongs to the next numeric scan,
        // while whitespace before P prevents that P from becoming a suffix.
        assert_eq!(ParsedOutputs::new(b"1x2+3+4P5x6+7+8").count(), 2);
        assert_eq!(ParsedOutputs::new(b"1x2+3+4 5x6+7+8").count(), 2);
        assert_eq!(ParsedOutputs::new(b"1x2+3+4 P,5x6+7+8").count(), 1);
        assert_eq!(ParsedOutputs::new(b"1x2+3+4,,5x6+7+8").count(), 1);
        assert_eq!(ParsedOutputs::new(b"1x2+3+4\0,5x6+7+8").count(), 1);
        let mut invalid = ParsedOutputs::new(b"broken,1x2+3+4");
        assert!(invalid.next().is_none());
        assert!(invalid.next().is_none());
    }

    #[test]
    fn resize_retains_legacy_signed_minimum_ordering() {
        let mut rect = Rect {
            x: 10,
            y: 20,
            width: 1,
            height: u32::MAX,
        };
        let parsed = ParsedOutput {
            width: 0x8000_0000,
            height: 0,
            x: 10,
            y: 20,
            primary: false,
        };
        parsed.shrink(&mut rect);
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (10, 20, 0x8000_0000, u32::MAX)
        );
        let mut ordinary = Rect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        ParsedOutput {
            width: 1280,
            height: 1200,
            ..parsed
        }
        .shrink(&mut ordinary);
        assert_eq!((ordinary.width, ordinary.height), (1280, 1080));
    }
}
