/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE).
 */
//! Bounded color parsing and owned low-depth colormap caching.
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::ptr::{self, NonNull};
use std::sync::{LazyLock, Mutex};
use xcb_sys::{xcb_alloc_color, xcb_alloc_color_reply, xcb_alloc_color_reply_t};

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
struct Channels {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Channels {
    fn parse(hex: &[u8]) -> Self {
        let pair = |start| {
            [
                hex.get(start).copied().unwrap_or(0),
                hex.get(start + 1).copied().unwrap_or(0),
            ]
        };
        Self {
            red: hex_byte(pair(1)),
            green: hex_byte(pair(3)),
            blue: hex_byte(pair(5)),
            alpha: if hex.len() == 9 {
                hex_byte(pair(7))
            } else {
                u8::MAX
            },
        }
    }

    fn argb(self) -> u32 {
        u32::from_be_bytes([self.alpha, self.red, self.green, self.blue])
    }

    fn native_rgb(self) -> (u16, u16, u16) {
        // 65535 * channel / 255 is exactly channel * 257 for every u8.
        (
            u16::from(self.red) * 257,
            u16::from(self.green) * 257,
            u16::from(self.blue) * 257,
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
fn hex_byte(pair: [u8; 2]) -> u8 {
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
    let mut value = 0u8;
    for byte in bytes {
        let Some(nibble) = digit(*byte) else { break };
        value = value * 16 + nibble;
    }
    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

struct ColorReply(NonNull<xcb_alloc_color_reply_t>);

impl ColorReply {
    fn pixel(&self) -> u32 {
        // SAFETY: this guard owns the complete reply returned by XCB.
        unsafe { self.0.as_ref().pixel }
    }
}

impl Drop for ColorReply {
    fn drop(&mut self) {
        // SAFETY: XCB transfers this malloc-compatible reply to its sole owner.
        unsafe { libc::free(self.0.as_ptr().cast()) }
    }
}

/// Get the packed ARGB value or allocate a color in the current native colormap.
///
/// # Safety
/// hex must point to a live NUL-terminated string. The registered display
/// provider must keep its screen and connection live for this call. Null screen
/// contexts need no connection and use the same ARGB fallback as TrueColor.
pub unsafe fn get_colorpixel(hex: *const c_char) -> u32 {
    if hex.is_null() {
        return Channels::parse(&[]).argb();
    }
    // SAFETY: the caller guarantees this string lives for the call. Parse using
    // its actual byte length; malformed short strings no longer read past NUL.
    let hex = unsafe { CStr::from_ptr(hex) }.to_bytes();
    let channels = Channels::parse(hex);
    let crate::display::DisplayContext {
        connection,
        root_screen,
        ..
    } = crate::display::context();
    // SAFETY: the display provider keeps any non-null screen live by contract.
    let screen = unsafe { root_screen.as_ref() };
    let Some(screen) = screen.filter(|screen| !matches!(screen.root_depth, 24 | 32)) else {
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
    // SAFETY: the display provider keeps the connection live and its screen's
    // default colormap belongs to that connection. The returned reply is owned.
    let reply = unsafe {
        let cookie = xcb_alloc_color(connection, screen.default_colormap, red, green, blue);
        xcb_alloc_color_reply(connection, cookie, ptr::null_mut())
    };
    let Some(reply) = NonNull::new(reply).map(ColorReply) else {
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
                let expected =
                    unsafe { libc::strtol(string.as_ptr().cast(), ptr::null_mut(), 16) } as u8;
                assert_eq!(
                    hex_byte([first, second]),
                    expected,
                    "pair {first:02x} {second:02x}"
                );
            }
        }
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
