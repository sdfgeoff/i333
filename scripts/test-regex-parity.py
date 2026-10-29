#!/usr/bin/env python3
"""Compare the owned regex module against upstream i3's PCRE2 UTF/UCP flags.

    python scripts/test-regex-parity.py --offline

The optional oracle uses the host PCRE2 shared library through Python ctypes;
this adds no native declarations to the products. Plain cargo test includes
the oracle using a prebuilt integration probe that includes the real regex.rs.
Standalone runs build that same integration target when --probe is absent.
No X server, window manager, input devices or session IPC sockets are used.
"""
import argparse
import ctypes as c
import ctypes.util
from pathlib import Path
import subprocess
from cargo_probe import build_probe

ROOT = Path(__file__).resolve().parents[1]
# Same compile flags used by upstream src/regex.c. Constants come from pcre2.h.
PCRE2_UTF_UCP = 0x00080000 | 0x00020000
PATTERNS = [
    r'^foo$', r'(?m)^foo$', r'^foo\Z', r'^foo\z', r'^[$]$', r'^\$$',
    r'^[[:alnum:]$]+$', r'^[[:^digit:]$]+$', r'^[[:alpha:]]+$',
    r'^[[:xdigit:]]+$', r'^[[:graph:]]+$', r'^[[:punct:]]+$',
    r'^\Qfoo$[]\E$', r'(?#a$)^foo$', '(?x)^foo$ # comment [$\n',
    r'(?x)^a(?-x:#)$', r'^[[]$', r'^[a&&b]+$', r'^[a--b]+$',
    r'^[a||b]+$', r'^[a~~b]+$', r'^\h+$', r'^\H+$', r'^\v+$', r'^\V+$',
    r'^\R$', r'^\w+$', r'^\b\w+\b$', r'^\d+$', r'^\s+$',
    r'(?i)^(foo)-\1$', r'(?<=a)b', r'(?<!a)b', r'(?=ab)a',
    r'(?>a|ab)c', r'a++a', r'(?<x>a|b)\k<x>', r'(a|b)\g1',
    r'(a|b)\g{1}', r'(?P<x>a|b)(?P=x)', r'\X', r'(*SKIP)(*FAIL)',
    r'(?R)', r'^(?|a(b)|c(d))\1$', r'(?U)^a+$', r'\x{1f600}',
    r'\p{Xan}+', r'\p{Xwd}+', r'^\N+$', r'(?n)(a)', r'^\cA$',
    r'^\u0061$', r'^\e$', r'^\0$', r'\<foo\>',
    r'(?J)(?<x>a)|(?<x>b)', r'(?i)k', r'(?s)a.b', r'foo\Kbar',
    r'^(?<nested>\((?:[^()]++|(?&nested))*\))$', r'(*CRLF)^foo$',
    r'(*ANYCRLF)(?m)^foo$', r'(*UTF)(*UCP)^\w+$', r'^\C$', r'.',
    '(', 'é(', r'[z-a]', r'(?<a>x)\k<missing>', r'[',
]
SUBJECTS = [
    '', 'foo', 'foo\n', 'foo\n\n', 'bar\nfoo\nend', '$', 'é', 'café',
    'α', '𐐀', '\u200d', '\u0301', '\u2160', '١', '１', 'Ｆ', 'f',
    ' ', '\t', '\v', '\n', '\r\n', '\u0085', '\u00a0', '\u180e', '\u2028',
    '\u202f', '\u205f', '\u3000', 'a', 'b', 'ab', 'aa', 'bb', 'abc', 'ac',
    'foo$[]', 'a#', '[', '&', '|', '~', '-', 'Foo-foo', '😀', 'a\nb',
    '\x01', '\x1b', 'K', 'K', 'ſ', 'abb', 'cdd', 'foobar', '((abc)(def))',
    '((abc)', '\0', 'a\0b',
]
INVALID_SUBJECTS = [b'\xff', b'A\xff', b'\x80', b'\xc0\xaf', b'\xed\xa0\x80',
                    b'\xf4\x90\x80\x80', b'\xe2\x82', b'\xf0\x9f\x98']



class Oracle:
    def __init__(self):
        library = ctypes.util.find_library('pcre2-8')
        if not library:
            raise SystemExit('PCRE2 shared library is required for native regex compatibility tests')
        self.lib = c.CDLL(library)
        signatures = {
            'pcre2_compile_8': ([c.c_void_p, c.c_size_t, c.c_uint,
                               c.POINTER(c.c_int), c.POINTER(c.c_size_t), c.c_void_p], c.c_void_p),
            'pcre2_match_data_create_from_pattern_8': ([c.c_void_p, c.c_void_p], c.c_void_p),
            'pcre2_match_8': ([c.c_void_p, c.c_void_p, c.c_size_t, c.c_size_t,
                              c.c_uint, c.c_void_p, c.c_void_p], c.c_int),
            'pcre2_match_data_free_8': ([c.c_void_p], None),
            'pcre2_code_free_8': ([c.c_void_p], None),
            'pcre2_get_error_message_8': ([c.c_int, c.c_void_p, c.c_size_t], c.c_int),
            'pcre2_config_8': ([c.c_uint, c.c_void_p], c.c_int),
        }
        for name, (arguments, result) in signatures.items():
            function = getattr(self.lib, name)
            function.argtypes, function.restype = arguments, result

    def version(self):
        version = c.create_string_buffer(100)
        self.lib.pcre2_config_8(11, version)  # PCRE2_CONFIG_VERSION
        return version.value.decode()

    def evaluate(self, pattern, subject):
        error, offset = c.c_int(), c.c_size_t()
        source = c.create_string_buffer(pattern)
        code = self.lib.pcre2_compile_8(source, len(pattern), PCRE2_UTF_UCP,
                                      c.byref(error), c.byref(offset), None)
        if not code:
            message = c.create_string_buffer(256)
            self.lib.pcre2_get_error_message_8(error.value, message, len(message))
            return ('compile_error', str(offset.value), str(error.value), message.value.hex())
        try:
            data = self.lib.pcre2_match_data_create_from_pattern_8(code, None)
            if not data:
                raise MemoryError('could not allocate oracle match data')
            try:
                buffer = c.create_string_buffer(subject)
                result = self.lib.pcre2_match_8(code, buffer, len(subject), 0, 0, data, None)
            finally:
                self.lib.pcre2_match_data_free_8(data)
            if result == -1:
                return ('match', '0')
            if result >= 0:
                return ('match', '1')
            return ('match_error', str(result))
        finally:
            self.lib.pcre2_code_free_8(code)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--offline', action='store_true', help='use cached Cargo dependencies only')
    parser.add_argument('--probe', type=Path, help='Use a Cargo-built native_regex_oracle executable; do not invoke Cargo')
    args = parser.parse_args()
    oracle = Oracle()
    subjects = [text.encode() for text in SUBJECTS] + INVALID_SUBJECTS
    cases = [(pattern.encode(), subject) for pattern in PATTERNS for subject in subjects]
    probe = args.probe.resolve() if args.probe else build_probe("i3", "native_regex_oracle", args.offline)
    results = subprocess.run([str(probe), "--probe"], input=''.join(
        pattern.hex() + '\t' + subject.hex() + '\n' for pattern, subject in cases),
        capture_output=True, text=True, check=True, timeout=60).stdout.splitlines()
    assert len(results) == len(cases), (len(results), len(cases))
    for (pattern, subject), result in zip(cases, results):
        expected = oracle.evaluate(pattern, subject)
        actual = tuple(result.split('\t'))
        assert actual == expected, (pattern, subject, actual, expected)
    print(f'{len(cases)} regex compile/match/error comparisons passed against PCRE2 {oracle.version()}')


if __name__ == '__main__':
    main()
