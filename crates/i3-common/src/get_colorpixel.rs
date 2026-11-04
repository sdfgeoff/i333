/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Bounded color parsing and owned low-depth colormap caching.
#![deny(unsafe_op_in_unsafe_fn)]

use crate::display::DisplayScreen;
use std::collections::HashMap;
#[cfg(test)]
use std::ptr;
use std::sync::{LazyLock, Mutex};
use xcb::{x, XidNew};

#[derive(Default)]
struct ColorCache {
    pixels: HashMap<Vec<u8>, u32>,
}

impl ColorCache {
    fn get(&self, hex: &[u8]) -> Option<u32> {
        self.pixels.get(hex).copied()
    }

    fn remember(&mut self, hex: &[u8], pixel: u32) {
        // Upstream stores only seven bytes but compares against the whole input:
        // alpha/longer strings miss lookup, then refresh their seven-byte RGB key.
        // A map retains that lookup rule without leaking duplicate list nodes.
        self.pixels.insert(hex[..hex.len().min(7)].to_vec(), pixel);
    }
}

static COLORPIXELS: LazyLock<Mutex<ColorCache>> =
    LazyLock::new(|| Mutex::new(ColorCache::default()));

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Channels {
    red: i16,
    green: i16,
    blue: i16,
    alpha: i16,
}

impl Channels {
    pub(crate) fn parse(hex: &[u8]) -> Self {
        let pair = |start| {
            [
                hex.get(start).copied().unwrap_or(0),
                hex.get(start + 1).copied().unwrap_or(0),
            ]
        };
        Self {
            red: hex_component(pair(1)),
            green: hex_component(pair(3)),
            blue: hex_component(pair(5)),
            alpha: if hex.len() == 9 {
                hex_component(pair(7))
            } else {
                i16::from(u8::MAX)
            },
        }
    }

    /// Native draw colors retain signed strtol values; pixel channels separately
    /// wrap to u8, matching get_colorpixel's historical conversion.
    pub(crate) fn rgba(self) -> [f64; 4] {
        [self.red, self.green, self.blue, self.alpha].map(|channel| f64::from(channel) / 255.0)
    }

    fn argb(self) -> u32 {
        u32::from_be_bytes([
            self.alpha as u8,
            self.red as u8,
            self.green as u8,
            self.blue as u8,
        ])
    }

    fn native_rgb(self) -> (u16, u16, u16) {
        // 65535 * channel / 255 is exactly channel * 257 for every u8.
        (
            u16::from(self.red as u8) * 257,
            u16::from(self.green as u8) * 257,
            u16::from(self.blue as u8) * 257,
        )
    }
}

fn digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Preserve strtol(base 16)'s prefix/sign/locale-space rules for a two-byte group.
fn hex_component(pair: [u8; 2]) -> i16 {
    let mut bytes = pair.as_slice();
    while let Some(byte) = bytes.first() {
        // SAFETY: every u8 promoted to int is a valid isspace argument. This
        // retains the configured native locale without borrowing a C buffer.
        if unsafe { libc::isspace(i32::from(*byte)) } == 0 {
            break;
        }
        bytes = &bytes[1..];
    }
    let negative = bytes.first() == Some(&b'-');
    if matches!(bytes.first(), Some(b'-' | b'+')) {
        bytes = &bytes[1..];
    }
    let mut value = 0i16;
    for byte in bytes {
        let Some(nibble) = digit(*byte) else { break };
        value = value * 16 + i16::from(nibble);
    }
    if negative {
        -value
    } else {
        value
    }
}

/// Resolve a parsed color using only the caller's retained display owner.
/// Detached validation and TrueColor screens require no native requests.
pub(crate) fn resolve_pixel(screen: Option<&DisplayScreen>, hex: &[u8], channels: Channels) -> u32 {
    let Some(screen) = screen.filter(|screen| !matches!(screen.info().root_depth, 24 | 32)) else {
        return channels.argb();
    };
    if let Some(pixel) = COLORPIXELS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(hex)
    {
        return pixel;
    }
    let (red, green, blue) = channels.native_rgb();
    let connection = screen.connection();
    let cookie = connection.send_request(&x::AllocColor {
        cmap: x::Colormap::new(screen.info().default_colormap),
        red,
        green,
        blue,
    });
    let reply = connection.wait_for_reply(cookie).ok();
    let Some(reply) = reply else {
        crate::logging::emit(
            crate::logging::Level::Verbose,
            b"[libi3] ../libi3/get_colorpixel.c Could not allocate color\n",
        );
        std::process::exit(1);
    };
    let pixel = reply.pixel();
    COLORPIXELS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remember(hex, pixel);
    pixel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_depth_cache_preserves_full_input_lookup_and_seven_byte_storage() {
        let mut cache = ColorCache::default();
        cache.remember(b"#aabbcc11", 42);
        assert_eq!(cache.get(b"#aabbcc11"), None);
        assert_eq!(cache.get(b"#aabbcc"), Some(42));
        assert_eq!(cache.get(b"#AABBCC"), None);
        cache.remember(b"#aabbcc22", 73);
        assert_eq!(cache.get(b"#aabbcc"), Some(73));
        assert_eq!(cache.get(b"#aabbcc22"), None);
        assert_eq!(cache.pixels.len(), 1);
    }

    #[test]
    fn two_byte_channels_match_native_strtol_for_every_byte_pair() {
        for first in 0..=u8::MAX {
            for second in 0..=u8::MAX {
                let string = [first, second, 0];
                // SAFETY: this initialized array is NUL-terminated, and the
                // native parser only borrows it. No locale changes occur here.
                let expected = unsafe { libc::strtol(string.as_ptr().cast(), ptr::null_mut(), 16) };
                assert_eq!(
                    libc::c_long::from(hex_component([first, second])),
                    expected,
                    "pair {first:02x} {second:02x}"
                );
            }
        }
    }

    #[test]
    fn bounded_color_values_match_the_native_signed_and_wrapped_rules() {
        use std::ffi::CStr;
        for input in [
            &b"#123456\0"[..],
            &b"#12345678\0"[..],
            &b"#123456789\0"[..],
            &b"#12345\0"[..],
            &b"#fZ-f+f\0"[..],
            &b"#-f+f f-f\0"[..],
            &b"#0x0Xff\0"[..],
            &b"#\xff1\xfe2\xfd3\0"[..],
            &b"#12\03456\0"[..],
            &b"bad\xff\0"[..],
            &b"\0"[..],
            &b"#f\0"[..],
        ] {
            let original = input;
            let input = CStr::from_bytes_until_nul(input).unwrap();
            let bytes = input.to_bytes();
            let effective = if bytes.len() < 6 || bytes.first() != Some(&b'#') {
                c"#A9A9A9"
            } else {
                input
            };
            let bytes = effective.to_bytes_with_nul();
            let mut signed = [255i16; 4];
            for (index, value) in signed.iter_mut().enumerate() {
                if index == 3 && effective.count_bytes() != 9 {
                    continue;
                }
                let start = index * 2 + 1;
                let pair = [bytes[start], bytes[start + 1], 0];
                // SAFETY: exactly two initialized bytes and a trailing NUL;
                // strtol's signed result is the native drawing oracle.
                *value = unsafe { libc::strtol(pair.as_ptr().cast(), ptr::null_mut(), 16) } as i16;
            }
            let actual = crate::draw_util::draw_util_hex_to_color(None, original);
            assert_eq!(
                [actual.red, actual.green, actual.blue, actual.alpha].map(f64::to_bits),
                signed.map(|channel| (f64::from(channel) / 255.0).to_bits()),
                "RGBA input {:?}",
                input.to_bytes(),
            );
            assert_eq!(
                actual.colorpixel,
                u32::from_be_bytes([
                    signed[3] as u8,
                    signed[0] as u8,
                    signed[1] as u8,
                    signed[2] as u8
                ]),
                "pixel input {:?}",
                input.to_bytes(),
            );
        }
    }

    #[test]
    fn colors_accept_owned_rust_strings_and_bounded_non_utf8_without_terminators() {
        let owned = String::from("#fZ-f+f");
        let borrowed = crate::draw_util::draw_util_hex_to_color(None, &owned);
        let moved = crate::draw_util::draw_util_hex_to_color(None, owned);
        assert_eq!(borrowed.colorpixel, 0xff0ff10f);
        assert_eq!(moved.colorpixel, borrowed.colorpixel);
        assert_eq!(moved.green.to_bits(), (-15.0f64 / 255.0).to_bits());
        let interior_nul = String::from("#12345678\0ignored-alpha");
        let color = crate::draw_util::draw_util_hex_to_color(None, interior_nul);
        assert_eq!(color.colorpixel, 0x78123456);
        let binary = vec![b'#', 0xff, b'1', 0xfe, b'2', 0xfd, b'3'];
        assert_eq!(
            crate::draw_util::draw_util_hex_to_color(None, binary).colorpixel,
            0xff000000
        );
    }

    #[test]
    fn detached_color_fallback_keeps_diagnostic_bytes_without_display_lookup() {
        const CHILD: &str = "I333_COLOR_FALLBACK_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let result = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "get_colorpixel::tests::detached_color_fallback_keeps_diagnostic_bytes_without_display_lookup"])
                .env(CHILD, "1").output().unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            return;
        }
        fn unexpected_display() -> crate::display::DisplayContext {
            panic!("explicit detached colors must not consult the display registry");
        }
        static MESSAGES: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());
        fn record(level: crate::logging::Level, message: &[u8]) {
            assert_eq!(level, crate::logging::Level::Error);
            MESSAGES.lock().unwrap().push(message.to_vec());
        }
        crate::display::set_provider(unexpected_display);
        crate::logging::set_sink(record);
        for input in [&b""[..], &b"bad\xff\0ignored"[..], &b"#f"[..]] {
            let actual = crate::draw_util::draw_util_hex_to_color(None, input);
            assert_eq!(actual.colorpixel, 0xffa9a9a9);
        }
        assert_eq!(
            *MESSAGES.lock().unwrap(),
            [
                b"[libi3] ERROR: Could not parse color: \n".to_vec(),
                b"[libi3] ERROR: Could not parse color: bad\xff\n".to_vec(),
                b"[libi3] ERROR: Could not parse color: #f\n".to_vec(),
            ]
        );
    }

    #[test]
    fn packed_colors_preserve_alpha_length_prefix_and_partial_digit_rules() {
        for (input, expected) in [
            (&b"#123456"[..], 0xff123456),
            (&b"#12345678"[..], 0x78123456),
            (&b"x123456"[..], 0xff123456),
            (&b"#123456789"[..], 0xff123456),
            (&b"#fZ-f+f"[..], 0xff0ff10f),
            (&b""[..], 0xff000000),
            (&b"#f"[..], 0xff0f0000),
        ] {
            assert_eq!(Channels::parse(input).argb(), expected, "{input:?}");
        }
        assert_eq!(Channels::parse(b"#00ff80").native_rgb(), (0, 65535, 32896));
    }
}
