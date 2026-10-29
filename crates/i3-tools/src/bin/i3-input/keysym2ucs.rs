/* $XFree86$
 * This module converts keysym values into the corresponding ISO 10646-1
 * (UCS, Unicode) values.
 *
 * The array keysymtab[] contains pairs of X11 keysym values for graphical
 * characters and the corresponding Unicode value. The function
 * keysym2ucs() maps a keysym onto a Unicode value using a binary search,
 * therefore keysymtab[] must remain SORTED by keysym value.
 *
 * The keysym -> UTF-8 conversion will hopefully one day be provided
 * by Xlib via XmbLookupString() and should ideally not have to be
 * done in X applications. But we are not there yet.
 *
 * We allow to represent any UCS character in the range U+00000000 to
 * U+00FFFFFF by a keysym value in the range 0x01000000 to 0x01ffffff.
 * This admittedly does not cover the entire 31-bit space of UCS, but
 * it does cover all of the characters up to U+10FFFF, which can be
 * represented by UTF-16, and more, and it is very unlikely that higher
 * UCS codes will ever be assigned by ISO. So to get Unicode character
 * U+ABCD you can directly use keysym 0x1000abcd.
 *
 * NOTE: The comments in the table below contain the actual character
 * encoded in UTF-8, so for viewing and editing best use an editor in
 * UTF-8 mode.
 *
 * Author: Markus G. Kuhn <mkuhn@acm.org>, University of Cambridge, June 1999
 *
 * Special thanks to Richard Verhoeven <river@win.tue.nl> for preparing
 * an initial draft of the mapping table.
 *
 * This software is in the public domain. Share and enjoy!
 */
// Mechanical file-by-file Rust translation of i3-input/keysym2ucs.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type xcb_keysym_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct codepair {
    pub keysym: ::core::ffi::c_ushort,
    pub ucs: ::core::ffi::c_ushort,
}
#[no_mangle]
pub static mut keysymtab: [codepair; 739] = [
    codepair {
        keysym: 0x1a1 as ::core::ffi::c_ushort,
        ucs: 0x104 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1a2 as ::core::ffi::c_ushort,
        ucs: 0x2d8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1a3 as ::core::ffi::c_ushort,
        ucs: 0x141 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1a5 as ::core::ffi::c_ushort,
        ucs: 0x13d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1a6 as ::core::ffi::c_ushort,
        ucs: 0x15a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1a9 as ::core::ffi::c_ushort,
        ucs: 0x160 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1aa as ::core::ffi::c_ushort,
        ucs: 0x15e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ab as ::core::ffi::c_ushort,
        ucs: 0x164 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ac as ::core::ffi::c_ushort,
        ucs: 0x179 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ae as ::core::ffi::c_ushort,
        ucs: 0x17d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1af as ::core::ffi::c_ushort,
        ucs: 0x17b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b1 as ::core::ffi::c_ushort,
        ucs: 0x105 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b2 as ::core::ffi::c_ushort,
        ucs: 0x2db as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b3 as ::core::ffi::c_ushort,
        ucs: 0x142 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b5 as ::core::ffi::c_ushort,
        ucs: 0x13e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b6 as ::core::ffi::c_ushort,
        ucs: 0x15b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b7 as ::core::ffi::c_ushort,
        ucs: 0x2c7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1b9 as ::core::ffi::c_ushort,
        ucs: 0x161 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ba as ::core::ffi::c_ushort,
        ucs: 0x15f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1bb as ::core::ffi::c_ushort,
        ucs: 0x165 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1bc as ::core::ffi::c_ushort,
        ucs: 0x17a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1bd as ::core::ffi::c_ushort,
        ucs: 0x2dd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1be as ::core::ffi::c_ushort,
        ucs: 0x17e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1bf as ::core::ffi::c_ushort,
        ucs: 0x17c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1c0 as ::core::ffi::c_ushort,
        ucs: 0x154 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1c3 as ::core::ffi::c_ushort,
        ucs: 0x102 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1c5 as ::core::ffi::c_ushort,
        ucs: 0x139 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1c6 as ::core::ffi::c_ushort,
        ucs: 0x106 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1c8 as ::core::ffi::c_ushort,
        ucs: 0x10c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ca as ::core::ffi::c_ushort,
        ucs: 0x118 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1cc as ::core::ffi::c_ushort,
        ucs: 0x11a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1cf as ::core::ffi::c_ushort,
        ucs: 0x10e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d0 as ::core::ffi::c_ushort,
        ucs: 0x110 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d1 as ::core::ffi::c_ushort,
        ucs: 0x143 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d2 as ::core::ffi::c_ushort,
        ucs: 0x147 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d5 as ::core::ffi::c_ushort,
        ucs: 0x150 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d8 as ::core::ffi::c_ushort,
        ucs: 0x158 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1d9 as ::core::ffi::c_ushort,
        ucs: 0x16e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1db as ::core::ffi::c_ushort,
        ucs: 0x170 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1de as ::core::ffi::c_ushort,
        ucs: 0x162 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1e0 as ::core::ffi::c_ushort,
        ucs: 0x155 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1e3 as ::core::ffi::c_ushort,
        ucs: 0x103 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1e5 as ::core::ffi::c_ushort,
        ucs: 0x13a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1e6 as ::core::ffi::c_ushort,
        ucs: 0x107 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1e8 as ::core::ffi::c_ushort,
        ucs: 0x10d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ea as ::core::ffi::c_ushort,
        ucs: 0x119 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ec as ::core::ffi::c_ushort,
        ucs: 0x11b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ef as ::core::ffi::c_ushort,
        ucs: 0x10f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f0 as ::core::ffi::c_ushort,
        ucs: 0x111 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f1 as ::core::ffi::c_ushort,
        ucs: 0x144 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f2 as ::core::ffi::c_ushort,
        ucs: 0x148 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f5 as ::core::ffi::c_ushort,
        ucs: 0x151 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f8 as ::core::ffi::c_ushort,
        ucs: 0x159 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1f9 as ::core::ffi::c_ushort,
        ucs: 0x16f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1fb as ::core::ffi::c_ushort,
        ucs: 0x171 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1fe as ::core::ffi::c_ushort,
        ucs: 0x163 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x1ff as ::core::ffi::c_ushort,
        ucs: 0x2d9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2a1 as ::core::ffi::c_ushort,
        ucs: 0x126 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2a6 as ::core::ffi::c_ushort,
        ucs: 0x124 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2a9 as ::core::ffi::c_ushort,
        ucs: 0x130 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2ab as ::core::ffi::c_ushort,
        ucs: 0x11e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2ac as ::core::ffi::c_ushort,
        ucs: 0x134 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2b1 as ::core::ffi::c_ushort,
        ucs: 0x127 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2b6 as ::core::ffi::c_ushort,
        ucs: 0x125 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2b9 as ::core::ffi::c_ushort,
        ucs: 0x131 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2bb as ::core::ffi::c_ushort,
        ucs: 0x11f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2bc as ::core::ffi::c_ushort,
        ucs: 0x135 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2c5 as ::core::ffi::c_ushort,
        ucs: 0x10a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2c6 as ::core::ffi::c_ushort,
        ucs: 0x108 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2d5 as ::core::ffi::c_ushort,
        ucs: 0x120 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2d8 as ::core::ffi::c_ushort,
        ucs: 0x11c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2dd as ::core::ffi::c_ushort,
        ucs: 0x16c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2de as ::core::ffi::c_ushort,
        ucs: 0x15c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2e5 as ::core::ffi::c_ushort,
        ucs: 0x10b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2e6 as ::core::ffi::c_ushort,
        ucs: 0x109 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2f5 as ::core::ffi::c_ushort,
        ucs: 0x121 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2f8 as ::core::ffi::c_ushort,
        ucs: 0x11d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2fd as ::core::ffi::c_ushort,
        ucs: 0x16d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x2fe as ::core::ffi::c_ushort,
        ucs: 0x15d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3a2 as ::core::ffi::c_ushort,
        ucs: 0x138 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3a3 as ::core::ffi::c_ushort,
        ucs: 0x156 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3a5 as ::core::ffi::c_ushort,
        ucs: 0x128 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3a6 as ::core::ffi::c_ushort,
        ucs: 0x13b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3aa as ::core::ffi::c_ushort,
        ucs: 0x112 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3ab as ::core::ffi::c_ushort,
        ucs: 0x122 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3ac as ::core::ffi::c_ushort,
        ucs: 0x166 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3b3 as ::core::ffi::c_ushort,
        ucs: 0x157 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3b5 as ::core::ffi::c_ushort,
        ucs: 0x129 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3b6 as ::core::ffi::c_ushort,
        ucs: 0x13c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3ba as ::core::ffi::c_ushort,
        ucs: 0x113 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3bb as ::core::ffi::c_ushort,
        ucs: 0x123 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3bc as ::core::ffi::c_ushort,
        ucs: 0x167 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3bd as ::core::ffi::c_ushort,
        ucs: 0x14a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3bf as ::core::ffi::c_ushort,
        ucs: 0x14b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3c0 as ::core::ffi::c_ushort,
        ucs: 0x100 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3c7 as ::core::ffi::c_ushort,
        ucs: 0x12e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3cc as ::core::ffi::c_ushort,
        ucs: 0x116 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3cf as ::core::ffi::c_ushort,
        ucs: 0x12a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3d1 as ::core::ffi::c_ushort,
        ucs: 0x145 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3d2 as ::core::ffi::c_ushort,
        ucs: 0x14c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3d3 as ::core::ffi::c_ushort,
        ucs: 0x136 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3d9 as ::core::ffi::c_ushort,
        ucs: 0x172 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3dd as ::core::ffi::c_ushort,
        ucs: 0x168 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3de as ::core::ffi::c_ushort,
        ucs: 0x16a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3e0 as ::core::ffi::c_ushort,
        ucs: 0x101 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3e7 as ::core::ffi::c_ushort,
        ucs: 0x12f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3ec as ::core::ffi::c_ushort,
        ucs: 0x117 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3ef as ::core::ffi::c_ushort,
        ucs: 0x12b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3f1 as ::core::ffi::c_ushort,
        ucs: 0x146 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3f2 as ::core::ffi::c_ushort,
        ucs: 0x14d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3f3 as ::core::ffi::c_ushort,
        ucs: 0x137 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3f9 as ::core::ffi::c_ushort,
        ucs: 0x173 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3fd as ::core::ffi::c_ushort,
        ucs: 0x169 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x3fe as ::core::ffi::c_ushort,
        ucs: 0x16b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x47e as ::core::ffi::c_ushort,
        ucs: 0x203e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a1 as ::core::ffi::c_ushort,
        ucs: 0x3002 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a2 as ::core::ffi::c_ushort,
        ucs: 0x300c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a3 as ::core::ffi::c_ushort,
        ucs: 0x300d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a4 as ::core::ffi::c_ushort,
        ucs: 0x3001 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a5 as ::core::ffi::c_ushort,
        ucs: 0x30fb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a6 as ::core::ffi::c_ushort,
        ucs: 0x30f2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a7 as ::core::ffi::c_ushort,
        ucs: 0x30a1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a8 as ::core::ffi::c_ushort,
        ucs: 0x30a3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4a9 as ::core::ffi::c_ushort,
        ucs: 0x30a5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4aa as ::core::ffi::c_ushort,
        ucs: 0x30a7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ab as ::core::ffi::c_ushort,
        ucs: 0x30a9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ac as ::core::ffi::c_ushort,
        ucs: 0x30e3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ad as ::core::ffi::c_ushort,
        ucs: 0x30e5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ae as ::core::ffi::c_ushort,
        ucs: 0x30e7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4af as ::core::ffi::c_ushort,
        ucs: 0x30c3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b0 as ::core::ffi::c_ushort,
        ucs: 0x30fc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b1 as ::core::ffi::c_ushort,
        ucs: 0x30a2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b2 as ::core::ffi::c_ushort,
        ucs: 0x30a4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b3 as ::core::ffi::c_ushort,
        ucs: 0x30a6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b4 as ::core::ffi::c_ushort,
        ucs: 0x30a8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b5 as ::core::ffi::c_ushort,
        ucs: 0x30aa as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b6 as ::core::ffi::c_ushort,
        ucs: 0x30ab as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b7 as ::core::ffi::c_ushort,
        ucs: 0x30ad as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b8 as ::core::ffi::c_ushort,
        ucs: 0x30af as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4b9 as ::core::ffi::c_ushort,
        ucs: 0x30b1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ba as ::core::ffi::c_ushort,
        ucs: 0x30b3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4bb as ::core::ffi::c_ushort,
        ucs: 0x30b5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4bc as ::core::ffi::c_ushort,
        ucs: 0x30b7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4bd as ::core::ffi::c_ushort,
        ucs: 0x30b9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4be as ::core::ffi::c_ushort,
        ucs: 0x30bb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4bf as ::core::ffi::c_ushort,
        ucs: 0x30bd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c0 as ::core::ffi::c_ushort,
        ucs: 0x30bf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c1 as ::core::ffi::c_ushort,
        ucs: 0x30c1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c2 as ::core::ffi::c_ushort,
        ucs: 0x30c4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c3 as ::core::ffi::c_ushort,
        ucs: 0x30c6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c4 as ::core::ffi::c_ushort,
        ucs: 0x30c8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c5 as ::core::ffi::c_ushort,
        ucs: 0x30ca as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c6 as ::core::ffi::c_ushort,
        ucs: 0x30cb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c7 as ::core::ffi::c_ushort,
        ucs: 0x30cc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c8 as ::core::ffi::c_ushort,
        ucs: 0x30cd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4c9 as ::core::ffi::c_ushort,
        ucs: 0x30ce as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ca as ::core::ffi::c_ushort,
        ucs: 0x30cf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4cb as ::core::ffi::c_ushort,
        ucs: 0x30d2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4cc as ::core::ffi::c_ushort,
        ucs: 0x30d5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4cd as ::core::ffi::c_ushort,
        ucs: 0x30d8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4ce as ::core::ffi::c_ushort,
        ucs: 0x30db as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4cf as ::core::ffi::c_ushort,
        ucs: 0x30de as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d0 as ::core::ffi::c_ushort,
        ucs: 0x30df as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d1 as ::core::ffi::c_ushort,
        ucs: 0x30e0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d2 as ::core::ffi::c_ushort,
        ucs: 0x30e1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d3 as ::core::ffi::c_ushort,
        ucs: 0x30e2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d4 as ::core::ffi::c_ushort,
        ucs: 0x30e4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d5 as ::core::ffi::c_ushort,
        ucs: 0x30e6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d6 as ::core::ffi::c_ushort,
        ucs: 0x30e8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d7 as ::core::ffi::c_ushort,
        ucs: 0x30e9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d8 as ::core::ffi::c_ushort,
        ucs: 0x30ea as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4d9 as ::core::ffi::c_ushort,
        ucs: 0x30eb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4da as ::core::ffi::c_ushort,
        ucs: 0x30ec as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4db as ::core::ffi::c_ushort,
        ucs: 0x30ed as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4dc as ::core::ffi::c_ushort,
        ucs: 0x30ef as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4dd as ::core::ffi::c_ushort,
        ucs: 0x30f3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4de as ::core::ffi::c_ushort,
        ucs: 0x309b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x4df as ::core::ffi::c_ushort,
        ucs: 0x309c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ac as ::core::ffi::c_ushort,
        ucs: 0x60c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5bb as ::core::ffi::c_ushort,
        ucs: 0x61b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5bf as ::core::ffi::c_ushort,
        ucs: 0x61f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c1 as ::core::ffi::c_ushort,
        ucs: 0x621 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c2 as ::core::ffi::c_ushort,
        ucs: 0x622 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c3 as ::core::ffi::c_ushort,
        ucs: 0x623 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c4 as ::core::ffi::c_ushort,
        ucs: 0x624 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c5 as ::core::ffi::c_ushort,
        ucs: 0x625 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c6 as ::core::ffi::c_ushort,
        ucs: 0x626 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c7 as ::core::ffi::c_ushort,
        ucs: 0x627 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c8 as ::core::ffi::c_ushort,
        ucs: 0x628 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5c9 as ::core::ffi::c_ushort,
        ucs: 0x629 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ca as ::core::ffi::c_ushort,
        ucs: 0x62a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5cb as ::core::ffi::c_ushort,
        ucs: 0x62b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5cc as ::core::ffi::c_ushort,
        ucs: 0x62c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5cd as ::core::ffi::c_ushort,
        ucs: 0x62d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ce as ::core::ffi::c_ushort,
        ucs: 0x62e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5cf as ::core::ffi::c_ushort,
        ucs: 0x62f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d0 as ::core::ffi::c_ushort,
        ucs: 0x630 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d1 as ::core::ffi::c_ushort,
        ucs: 0x631 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d2 as ::core::ffi::c_ushort,
        ucs: 0x632 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d3 as ::core::ffi::c_ushort,
        ucs: 0x633 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d4 as ::core::ffi::c_ushort,
        ucs: 0x634 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d5 as ::core::ffi::c_ushort,
        ucs: 0x635 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d6 as ::core::ffi::c_ushort,
        ucs: 0x636 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d7 as ::core::ffi::c_ushort,
        ucs: 0x637 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d8 as ::core::ffi::c_ushort,
        ucs: 0x638 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5d9 as ::core::ffi::c_ushort,
        ucs: 0x639 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5da as ::core::ffi::c_ushort,
        ucs: 0x63a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e0 as ::core::ffi::c_ushort,
        ucs: 0x640 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e1 as ::core::ffi::c_ushort,
        ucs: 0x641 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e2 as ::core::ffi::c_ushort,
        ucs: 0x642 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e3 as ::core::ffi::c_ushort,
        ucs: 0x643 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e4 as ::core::ffi::c_ushort,
        ucs: 0x644 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e5 as ::core::ffi::c_ushort,
        ucs: 0x645 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e6 as ::core::ffi::c_ushort,
        ucs: 0x646 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e7 as ::core::ffi::c_ushort,
        ucs: 0x647 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e8 as ::core::ffi::c_ushort,
        ucs: 0x648 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5e9 as ::core::ffi::c_ushort,
        ucs: 0x649 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ea as ::core::ffi::c_ushort,
        ucs: 0x64a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5eb as ::core::ffi::c_ushort,
        ucs: 0x64b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ec as ::core::ffi::c_ushort,
        ucs: 0x64c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ed as ::core::ffi::c_ushort,
        ucs: 0x64d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ee as ::core::ffi::c_ushort,
        ucs: 0x64e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5ef as ::core::ffi::c_ushort,
        ucs: 0x64f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5f0 as ::core::ffi::c_ushort,
        ucs: 0x650 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5f1 as ::core::ffi::c_ushort,
        ucs: 0x651 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x5f2 as ::core::ffi::c_ushort,
        ucs: 0x652 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a1 as ::core::ffi::c_ushort,
        ucs: 0x452 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a2 as ::core::ffi::c_ushort,
        ucs: 0x453 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a3 as ::core::ffi::c_ushort,
        ucs: 0x451 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a4 as ::core::ffi::c_ushort,
        ucs: 0x454 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a5 as ::core::ffi::c_ushort,
        ucs: 0x455 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a6 as ::core::ffi::c_ushort,
        ucs: 0x456 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a7 as ::core::ffi::c_ushort,
        ucs: 0x457 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a8 as ::core::ffi::c_ushort,
        ucs: 0x458 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6a9 as ::core::ffi::c_ushort,
        ucs: 0x459 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6aa as ::core::ffi::c_ushort,
        ucs: 0x45a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ab as ::core::ffi::c_ushort,
        ucs: 0x45b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ac as ::core::ffi::c_ushort,
        ucs: 0x45c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ae as ::core::ffi::c_ushort,
        ucs: 0x45e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6af as ::core::ffi::c_ushort,
        ucs: 0x45f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b0 as ::core::ffi::c_ushort,
        ucs: 0x2116 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b1 as ::core::ffi::c_ushort,
        ucs: 0x402 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b2 as ::core::ffi::c_ushort,
        ucs: 0x403 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b3 as ::core::ffi::c_ushort,
        ucs: 0x401 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b4 as ::core::ffi::c_ushort,
        ucs: 0x404 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b5 as ::core::ffi::c_ushort,
        ucs: 0x405 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b6 as ::core::ffi::c_ushort,
        ucs: 0x406 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b7 as ::core::ffi::c_ushort,
        ucs: 0x407 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b8 as ::core::ffi::c_ushort,
        ucs: 0x408 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6b9 as ::core::ffi::c_ushort,
        ucs: 0x409 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ba as ::core::ffi::c_ushort,
        ucs: 0x40a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6bb as ::core::ffi::c_ushort,
        ucs: 0x40b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6bc as ::core::ffi::c_ushort,
        ucs: 0x40c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6be as ::core::ffi::c_ushort,
        ucs: 0x40e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6bf as ::core::ffi::c_ushort,
        ucs: 0x40f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c0 as ::core::ffi::c_ushort,
        ucs: 0x44e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c1 as ::core::ffi::c_ushort,
        ucs: 0x430 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c2 as ::core::ffi::c_ushort,
        ucs: 0x431 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c3 as ::core::ffi::c_ushort,
        ucs: 0x446 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c4 as ::core::ffi::c_ushort,
        ucs: 0x434 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c5 as ::core::ffi::c_ushort,
        ucs: 0x435 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c6 as ::core::ffi::c_ushort,
        ucs: 0x444 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c7 as ::core::ffi::c_ushort,
        ucs: 0x433 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c8 as ::core::ffi::c_ushort,
        ucs: 0x445 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6c9 as ::core::ffi::c_ushort,
        ucs: 0x438 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ca as ::core::ffi::c_ushort,
        ucs: 0x439 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6cb as ::core::ffi::c_ushort,
        ucs: 0x43a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6cc as ::core::ffi::c_ushort,
        ucs: 0x43b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6cd as ::core::ffi::c_ushort,
        ucs: 0x43c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ce as ::core::ffi::c_ushort,
        ucs: 0x43d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6cf as ::core::ffi::c_ushort,
        ucs: 0x43e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d0 as ::core::ffi::c_ushort,
        ucs: 0x43f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d1 as ::core::ffi::c_ushort,
        ucs: 0x44f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d2 as ::core::ffi::c_ushort,
        ucs: 0x440 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d3 as ::core::ffi::c_ushort,
        ucs: 0x441 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d4 as ::core::ffi::c_ushort,
        ucs: 0x442 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d5 as ::core::ffi::c_ushort,
        ucs: 0x443 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d6 as ::core::ffi::c_ushort,
        ucs: 0x436 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d7 as ::core::ffi::c_ushort,
        ucs: 0x432 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d8 as ::core::ffi::c_ushort,
        ucs: 0x44c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6d9 as ::core::ffi::c_ushort,
        ucs: 0x44b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6da as ::core::ffi::c_ushort,
        ucs: 0x437 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6db as ::core::ffi::c_ushort,
        ucs: 0x448 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6dc as ::core::ffi::c_ushort,
        ucs: 0x44d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6dd as ::core::ffi::c_ushort,
        ucs: 0x449 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6de as ::core::ffi::c_ushort,
        ucs: 0x447 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6df as ::core::ffi::c_ushort,
        ucs: 0x44a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e0 as ::core::ffi::c_ushort,
        ucs: 0x42e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e1 as ::core::ffi::c_ushort,
        ucs: 0x410 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e2 as ::core::ffi::c_ushort,
        ucs: 0x411 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e3 as ::core::ffi::c_ushort,
        ucs: 0x426 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e4 as ::core::ffi::c_ushort,
        ucs: 0x414 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e5 as ::core::ffi::c_ushort,
        ucs: 0x415 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e6 as ::core::ffi::c_ushort,
        ucs: 0x424 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e7 as ::core::ffi::c_ushort,
        ucs: 0x413 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e8 as ::core::ffi::c_ushort,
        ucs: 0x425 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6e9 as ::core::ffi::c_ushort,
        ucs: 0x418 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ea as ::core::ffi::c_ushort,
        ucs: 0x419 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6eb as ::core::ffi::c_ushort,
        ucs: 0x41a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ec as ::core::ffi::c_ushort,
        ucs: 0x41b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ed as ::core::ffi::c_ushort,
        ucs: 0x41c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ee as ::core::ffi::c_ushort,
        ucs: 0x41d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ef as ::core::ffi::c_ushort,
        ucs: 0x41e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f0 as ::core::ffi::c_ushort,
        ucs: 0x41f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f1 as ::core::ffi::c_ushort,
        ucs: 0x42f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f2 as ::core::ffi::c_ushort,
        ucs: 0x420 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f3 as ::core::ffi::c_ushort,
        ucs: 0x421 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f4 as ::core::ffi::c_ushort,
        ucs: 0x422 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f5 as ::core::ffi::c_ushort,
        ucs: 0x423 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f6 as ::core::ffi::c_ushort,
        ucs: 0x416 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f7 as ::core::ffi::c_ushort,
        ucs: 0x412 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f8 as ::core::ffi::c_ushort,
        ucs: 0x42c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6f9 as ::core::ffi::c_ushort,
        ucs: 0x42b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6fa as ::core::ffi::c_ushort,
        ucs: 0x417 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6fb as ::core::ffi::c_ushort,
        ucs: 0x428 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6fc as ::core::ffi::c_ushort,
        ucs: 0x42d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6fd as ::core::ffi::c_ushort,
        ucs: 0x429 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6fe as ::core::ffi::c_ushort,
        ucs: 0x427 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x6ff as ::core::ffi::c_ushort,
        ucs: 0x42a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a1 as ::core::ffi::c_ushort,
        ucs: 0x386 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a2 as ::core::ffi::c_ushort,
        ucs: 0x388 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a3 as ::core::ffi::c_ushort,
        ucs: 0x389 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a4 as ::core::ffi::c_ushort,
        ucs: 0x38a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a5 as ::core::ffi::c_ushort,
        ucs: 0x3aa as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a7 as ::core::ffi::c_ushort,
        ucs: 0x38c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a8 as ::core::ffi::c_ushort,
        ucs: 0x38e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7a9 as ::core::ffi::c_ushort,
        ucs: 0x3ab as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ab as ::core::ffi::c_ushort,
        ucs: 0x38f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ae as ::core::ffi::c_ushort,
        ucs: 0x385 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7af as ::core::ffi::c_ushort,
        ucs: 0x2015 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b1 as ::core::ffi::c_ushort,
        ucs: 0x3ac as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b2 as ::core::ffi::c_ushort,
        ucs: 0x3ad as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b3 as ::core::ffi::c_ushort,
        ucs: 0x3ae as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b4 as ::core::ffi::c_ushort,
        ucs: 0x3af as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b5 as ::core::ffi::c_ushort,
        ucs: 0x3ca as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b6 as ::core::ffi::c_ushort,
        ucs: 0x390 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b7 as ::core::ffi::c_ushort,
        ucs: 0x3cc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b8 as ::core::ffi::c_ushort,
        ucs: 0x3cd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7b9 as ::core::ffi::c_ushort,
        ucs: 0x3cb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ba as ::core::ffi::c_ushort,
        ucs: 0x3b0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7bb as ::core::ffi::c_ushort,
        ucs: 0x3ce as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c1 as ::core::ffi::c_ushort,
        ucs: 0x391 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c2 as ::core::ffi::c_ushort,
        ucs: 0x392 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c3 as ::core::ffi::c_ushort,
        ucs: 0x393 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c4 as ::core::ffi::c_ushort,
        ucs: 0x394 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c5 as ::core::ffi::c_ushort,
        ucs: 0x395 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c6 as ::core::ffi::c_ushort,
        ucs: 0x396 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c7 as ::core::ffi::c_ushort,
        ucs: 0x397 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c8 as ::core::ffi::c_ushort,
        ucs: 0x398 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7c9 as ::core::ffi::c_ushort,
        ucs: 0x399 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ca as ::core::ffi::c_ushort,
        ucs: 0x39a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7cb as ::core::ffi::c_ushort,
        ucs: 0x39b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7cc as ::core::ffi::c_ushort,
        ucs: 0x39c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7cd as ::core::ffi::c_ushort,
        ucs: 0x39d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ce as ::core::ffi::c_ushort,
        ucs: 0x39e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7cf as ::core::ffi::c_ushort,
        ucs: 0x39f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d0 as ::core::ffi::c_ushort,
        ucs: 0x3a0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d1 as ::core::ffi::c_ushort,
        ucs: 0x3a1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d2 as ::core::ffi::c_ushort,
        ucs: 0x3a3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d4 as ::core::ffi::c_ushort,
        ucs: 0x3a4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d5 as ::core::ffi::c_ushort,
        ucs: 0x3a5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d6 as ::core::ffi::c_ushort,
        ucs: 0x3a6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d7 as ::core::ffi::c_ushort,
        ucs: 0x3a7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d8 as ::core::ffi::c_ushort,
        ucs: 0x3a8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7d9 as ::core::ffi::c_ushort,
        ucs: 0x3a9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e1 as ::core::ffi::c_ushort,
        ucs: 0x3b1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e2 as ::core::ffi::c_ushort,
        ucs: 0x3b2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e3 as ::core::ffi::c_ushort,
        ucs: 0x3b3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e4 as ::core::ffi::c_ushort,
        ucs: 0x3b4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e5 as ::core::ffi::c_ushort,
        ucs: 0x3b5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e6 as ::core::ffi::c_ushort,
        ucs: 0x3b6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e7 as ::core::ffi::c_ushort,
        ucs: 0x3b7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e8 as ::core::ffi::c_ushort,
        ucs: 0x3b8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7e9 as ::core::ffi::c_ushort,
        ucs: 0x3b9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ea as ::core::ffi::c_ushort,
        ucs: 0x3ba as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7eb as ::core::ffi::c_ushort,
        ucs: 0x3bb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ec as ::core::ffi::c_ushort,
        ucs: 0x3bc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ed as ::core::ffi::c_ushort,
        ucs: 0x3bd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ee as ::core::ffi::c_ushort,
        ucs: 0x3be as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7ef as ::core::ffi::c_ushort,
        ucs: 0x3bf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f0 as ::core::ffi::c_ushort,
        ucs: 0x3c0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f1 as ::core::ffi::c_ushort,
        ucs: 0x3c1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f2 as ::core::ffi::c_ushort,
        ucs: 0x3c3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f3 as ::core::ffi::c_ushort,
        ucs: 0x3c2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f4 as ::core::ffi::c_ushort,
        ucs: 0x3c4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f5 as ::core::ffi::c_ushort,
        ucs: 0x3c5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f6 as ::core::ffi::c_ushort,
        ucs: 0x3c6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f7 as ::core::ffi::c_ushort,
        ucs: 0x3c7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f8 as ::core::ffi::c_ushort,
        ucs: 0x3c8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x7f9 as ::core::ffi::c_ushort,
        ucs: 0x3c9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8a4 as ::core::ffi::c_ushort,
        ucs: 0x2320 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8a5 as ::core::ffi::c_ushort,
        ucs: 0x2321 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8a6 as ::core::ffi::c_ushort,
        ucs: 0x2502 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8bc as ::core::ffi::c_ushort,
        ucs: 0x2264 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8bd as ::core::ffi::c_ushort,
        ucs: 0x2260 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8be as ::core::ffi::c_ushort,
        ucs: 0x2265 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8bf as ::core::ffi::c_ushort,
        ucs: 0x222b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8c0 as ::core::ffi::c_ushort,
        ucs: 0x2234 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8c1 as ::core::ffi::c_ushort,
        ucs: 0x221d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8c2 as ::core::ffi::c_ushort,
        ucs: 0x221e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8c5 as ::core::ffi::c_ushort,
        ucs: 0x2207 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8c8 as ::core::ffi::c_ushort,
        ucs: 0x2245 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8cd as ::core::ffi::c_ushort,
        ucs: 0x21d4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8ce as ::core::ffi::c_ushort,
        ucs: 0x21d2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8cf as ::core::ffi::c_ushort,
        ucs: 0x2261 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8d6 as ::core::ffi::c_ushort,
        ucs: 0x221a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8da as ::core::ffi::c_ushort,
        ucs: 0x2282 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8db as ::core::ffi::c_ushort,
        ucs: 0x2283 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8dc as ::core::ffi::c_ushort,
        ucs: 0x2229 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8dd as ::core::ffi::c_ushort,
        ucs: 0x222a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8de as ::core::ffi::c_ushort,
        ucs: 0x2227 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8df as ::core::ffi::c_ushort,
        ucs: 0x2228 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8ef as ::core::ffi::c_ushort,
        ucs: 0x2202 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8f6 as ::core::ffi::c_ushort,
        ucs: 0x192 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8fb as ::core::ffi::c_ushort,
        ucs: 0x2190 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8fc as ::core::ffi::c_ushort,
        ucs: 0x2191 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8fd as ::core::ffi::c_ushort,
        ucs: 0x2192 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x8fe as ::core::ffi::c_ushort,
        ucs: 0x2193 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9df as ::core::ffi::c_ushort,
        ucs: 0x2422 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e0 as ::core::ffi::c_ushort,
        ucs: 0x25c6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e1 as ::core::ffi::c_ushort,
        ucs: 0x2592 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e2 as ::core::ffi::c_ushort,
        ucs: 0x2409 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e3 as ::core::ffi::c_ushort,
        ucs: 0x240c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e4 as ::core::ffi::c_ushort,
        ucs: 0x240d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e5 as ::core::ffi::c_ushort,
        ucs: 0x240a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e8 as ::core::ffi::c_ushort,
        ucs: 0x2424 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9e9 as ::core::ffi::c_ushort,
        ucs: 0x240b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9ea as ::core::ffi::c_ushort,
        ucs: 0x2518 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9eb as ::core::ffi::c_ushort,
        ucs: 0x2510 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9ec as ::core::ffi::c_ushort,
        ucs: 0x250c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9ed as ::core::ffi::c_ushort,
        ucs: 0x2514 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9ee as ::core::ffi::c_ushort,
        ucs: 0x253c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f1 as ::core::ffi::c_ushort,
        ucs: 0x2500 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f4 as ::core::ffi::c_ushort,
        ucs: 0x251c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f5 as ::core::ffi::c_ushort,
        ucs: 0x2524 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f6 as ::core::ffi::c_ushort,
        ucs: 0x2534 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f7 as ::core::ffi::c_ushort,
        ucs: 0x252c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x9f8 as ::core::ffi::c_ushort,
        ucs: 0x2502 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa1 as ::core::ffi::c_ushort,
        ucs: 0x2003 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa2 as ::core::ffi::c_ushort,
        ucs: 0x2002 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa3 as ::core::ffi::c_ushort,
        ucs: 0x2004 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa4 as ::core::ffi::c_ushort,
        ucs: 0x2005 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa5 as ::core::ffi::c_ushort,
        ucs: 0x2007 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa6 as ::core::ffi::c_ushort,
        ucs: 0x2008 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa7 as ::core::ffi::c_ushort,
        ucs: 0x2009 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa8 as ::core::ffi::c_ushort,
        ucs: 0x200a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaa9 as ::core::ffi::c_ushort,
        ucs: 0x2014 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaaa as ::core::ffi::c_ushort,
        ucs: 0x2013 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaae as ::core::ffi::c_ushort,
        ucs: 0x2026 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab0 as ::core::ffi::c_ushort,
        ucs: 0x2153 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab1 as ::core::ffi::c_ushort,
        ucs: 0x2154 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab2 as ::core::ffi::c_ushort,
        ucs: 0x2155 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab3 as ::core::ffi::c_ushort,
        ucs: 0x2156 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab4 as ::core::ffi::c_ushort,
        ucs: 0x2157 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab5 as ::core::ffi::c_ushort,
        ucs: 0x2158 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab6 as ::core::ffi::c_ushort,
        ucs: 0x2159 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab7 as ::core::ffi::c_ushort,
        ucs: 0x215a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xab8 as ::core::ffi::c_ushort,
        ucs: 0x2105 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xabb as ::core::ffi::c_ushort,
        ucs: 0x2012 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xabc as ::core::ffi::c_ushort,
        ucs: 0x2329 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xabd as ::core::ffi::c_ushort,
        ucs: 0x2e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xabe as ::core::ffi::c_ushort,
        ucs: 0x232a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xac3 as ::core::ffi::c_ushort,
        ucs: 0x215b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xac4 as ::core::ffi::c_ushort,
        ucs: 0x215c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xac5 as ::core::ffi::c_ushort,
        ucs: 0x215d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xac6 as ::core::ffi::c_ushort,
        ucs: 0x215e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xac9 as ::core::ffi::c_ushort,
        ucs: 0x2122 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaca as ::core::ffi::c_ushort,
        ucs: 0x2613 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xacc as ::core::ffi::c_ushort,
        ucs: 0x25c1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xacd as ::core::ffi::c_ushort,
        ucs: 0x25b7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xace as ::core::ffi::c_ushort,
        ucs: 0x25cb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xacf as ::core::ffi::c_ushort,
        ucs: 0x25a1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad0 as ::core::ffi::c_ushort,
        ucs: 0x2018 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad1 as ::core::ffi::c_ushort,
        ucs: 0x2019 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad2 as ::core::ffi::c_ushort,
        ucs: 0x201c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad3 as ::core::ffi::c_ushort,
        ucs: 0x201d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad4 as ::core::ffi::c_ushort,
        ucs: 0x211e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad6 as ::core::ffi::c_ushort,
        ucs: 0x2032 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad7 as ::core::ffi::c_ushort,
        ucs: 0x2033 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xad9 as ::core::ffi::c_ushort,
        ucs: 0x271d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xadb as ::core::ffi::c_ushort,
        ucs: 0x25ac as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xadc as ::core::ffi::c_ushort,
        ucs: 0x25c0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xadd as ::core::ffi::c_ushort,
        ucs: 0x25b6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xade as ::core::ffi::c_ushort,
        ucs: 0x25cf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xadf as ::core::ffi::c_ushort,
        ucs: 0x25a0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae0 as ::core::ffi::c_ushort,
        ucs: 0x25e6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae1 as ::core::ffi::c_ushort,
        ucs: 0x25ab as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae2 as ::core::ffi::c_ushort,
        ucs: 0x25ad as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae3 as ::core::ffi::c_ushort,
        ucs: 0x25b3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae4 as ::core::ffi::c_ushort,
        ucs: 0x25bd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae5 as ::core::ffi::c_ushort,
        ucs: 0x2606 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae6 as ::core::ffi::c_ushort,
        ucs: 0x2022 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae7 as ::core::ffi::c_ushort,
        ucs: 0x25aa as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae8 as ::core::ffi::c_ushort,
        ucs: 0x25b2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xae9 as ::core::ffi::c_ushort,
        ucs: 0x25bc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaea as ::core::ffi::c_ushort,
        ucs: 0x261c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaeb as ::core::ffi::c_ushort,
        ucs: 0x261e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaec as ::core::ffi::c_ushort,
        ucs: 0x2663 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaed as ::core::ffi::c_ushort,
        ucs: 0x2666 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaee as ::core::ffi::c_ushort,
        ucs: 0x2665 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf0 as ::core::ffi::c_ushort,
        ucs: 0x2720 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf1 as ::core::ffi::c_ushort,
        ucs: 0x2020 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf2 as ::core::ffi::c_ushort,
        ucs: 0x2021 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf3 as ::core::ffi::c_ushort,
        ucs: 0x2713 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf4 as ::core::ffi::c_ushort,
        ucs: 0x2717 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf5 as ::core::ffi::c_ushort,
        ucs: 0x266f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf6 as ::core::ffi::c_ushort,
        ucs: 0x266d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf7 as ::core::ffi::c_ushort,
        ucs: 0x2642 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf8 as ::core::ffi::c_ushort,
        ucs: 0x2640 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xaf9 as ::core::ffi::c_ushort,
        ucs: 0x260e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xafa as ::core::ffi::c_ushort,
        ucs: 0x2315 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xafb as ::core::ffi::c_ushort,
        ucs: 0x2117 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xafc as ::core::ffi::c_ushort,
        ucs: 0x2038 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xafd as ::core::ffi::c_ushort,
        ucs: 0x201a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xafe as ::core::ffi::c_ushort,
        ucs: 0x201e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xba3 as ::core::ffi::c_ushort,
        ucs: 0x3c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xba6 as ::core::ffi::c_ushort,
        ucs: 0x3e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xba8 as ::core::ffi::c_ushort,
        ucs: 0x2228 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xba9 as ::core::ffi::c_ushort,
        ucs: 0x2227 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbc0 as ::core::ffi::c_ushort,
        ucs: 0xaf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbc2 as ::core::ffi::c_ushort,
        ucs: 0x22a4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbc3 as ::core::ffi::c_ushort,
        ucs: 0x2229 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbc4 as ::core::ffi::c_ushort,
        ucs: 0x230a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbc6 as ::core::ffi::c_ushort,
        ucs: 0x5f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbca as ::core::ffi::c_ushort,
        ucs: 0x2218 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbcc as ::core::ffi::c_ushort,
        ucs: 0x2395 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbce as ::core::ffi::c_ushort,
        ucs: 0x22a5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbcf as ::core::ffi::c_ushort,
        ucs: 0x25cb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbd3 as ::core::ffi::c_ushort,
        ucs: 0x2308 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbd6 as ::core::ffi::c_ushort,
        ucs: 0x222a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbd8 as ::core::ffi::c_ushort,
        ucs: 0x2283 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbda as ::core::ffi::c_ushort,
        ucs: 0x2282 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbdc as ::core::ffi::c_ushort,
        ucs: 0x22a3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xbfc as ::core::ffi::c_ushort,
        ucs: 0x22a2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcdf as ::core::ffi::c_ushort,
        ucs: 0x2017 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce0 as ::core::ffi::c_ushort,
        ucs: 0x5d0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce1 as ::core::ffi::c_ushort,
        ucs: 0x5d1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce2 as ::core::ffi::c_ushort,
        ucs: 0x5d2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce3 as ::core::ffi::c_ushort,
        ucs: 0x5d3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce4 as ::core::ffi::c_ushort,
        ucs: 0x5d4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce5 as ::core::ffi::c_ushort,
        ucs: 0x5d5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce6 as ::core::ffi::c_ushort,
        ucs: 0x5d6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce7 as ::core::ffi::c_ushort,
        ucs: 0x5d7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce8 as ::core::ffi::c_ushort,
        ucs: 0x5d8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xce9 as ::core::ffi::c_ushort,
        ucs: 0x5d9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcea as ::core::ffi::c_ushort,
        ucs: 0x5da as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xceb as ::core::ffi::c_ushort,
        ucs: 0x5db as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcec as ::core::ffi::c_ushort,
        ucs: 0x5dc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xced as ::core::ffi::c_ushort,
        ucs: 0x5dd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcee as ::core::ffi::c_ushort,
        ucs: 0x5de as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcef as ::core::ffi::c_ushort,
        ucs: 0x5df as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf0 as ::core::ffi::c_ushort,
        ucs: 0x5e0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf1 as ::core::ffi::c_ushort,
        ucs: 0x5e1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf2 as ::core::ffi::c_ushort,
        ucs: 0x5e2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf3 as ::core::ffi::c_ushort,
        ucs: 0x5e3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf4 as ::core::ffi::c_ushort,
        ucs: 0x5e4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf5 as ::core::ffi::c_ushort,
        ucs: 0x5e5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf6 as ::core::ffi::c_ushort,
        ucs: 0x5e6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf7 as ::core::ffi::c_ushort,
        ucs: 0x5e7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf8 as ::core::ffi::c_ushort,
        ucs: 0x5e8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcf9 as ::core::ffi::c_ushort,
        ucs: 0x5e9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xcfa as ::core::ffi::c_ushort,
        ucs: 0x5ea as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda1 as ::core::ffi::c_ushort,
        ucs: 0xe01 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda2 as ::core::ffi::c_ushort,
        ucs: 0xe02 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda3 as ::core::ffi::c_ushort,
        ucs: 0xe03 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda4 as ::core::ffi::c_ushort,
        ucs: 0xe04 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda5 as ::core::ffi::c_ushort,
        ucs: 0xe05 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda6 as ::core::ffi::c_ushort,
        ucs: 0xe06 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda7 as ::core::ffi::c_ushort,
        ucs: 0xe07 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda8 as ::core::ffi::c_ushort,
        ucs: 0xe08 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xda9 as ::core::ffi::c_ushort,
        ucs: 0xe09 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdaa as ::core::ffi::c_ushort,
        ucs: 0xe0a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdab as ::core::ffi::c_ushort,
        ucs: 0xe0b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdac as ::core::ffi::c_ushort,
        ucs: 0xe0c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdad as ::core::ffi::c_ushort,
        ucs: 0xe0d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdae as ::core::ffi::c_ushort,
        ucs: 0xe0e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdaf as ::core::ffi::c_ushort,
        ucs: 0xe0f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb0 as ::core::ffi::c_ushort,
        ucs: 0xe10 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb1 as ::core::ffi::c_ushort,
        ucs: 0xe11 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb2 as ::core::ffi::c_ushort,
        ucs: 0xe12 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb3 as ::core::ffi::c_ushort,
        ucs: 0xe13 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb4 as ::core::ffi::c_ushort,
        ucs: 0xe14 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb5 as ::core::ffi::c_ushort,
        ucs: 0xe15 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb6 as ::core::ffi::c_ushort,
        ucs: 0xe16 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb7 as ::core::ffi::c_ushort,
        ucs: 0xe17 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb8 as ::core::ffi::c_ushort,
        ucs: 0xe18 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdb9 as ::core::ffi::c_ushort,
        ucs: 0xe19 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdba as ::core::ffi::c_ushort,
        ucs: 0xe1a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdbb as ::core::ffi::c_ushort,
        ucs: 0xe1b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdbc as ::core::ffi::c_ushort,
        ucs: 0xe1c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdbd as ::core::ffi::c_ushort,
        ucs: 0xe1d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdbe as ::core::ffi::c_ushort,
        ucs: 0xe1e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdbf as ::core::ffi::c_ushort,
        ucs: 0xe1f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc0 as ::core::ffi::c_ushort,
        ucs: 0xe20 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc1 as ::core::ffi::c_ushort,
        ucs: 0xe21 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc2 as ::core::ffi::c_ushort,
        ucs: 0xe22 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc3 as ::core::ffi::c_ushort,
        ucs: 0xe23 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc4 as ::core::ffi::c_ushort,
        ucs: 0xe24 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc5 as ::core::ffi::c_ushort,
        ucs: 0xe25 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc6 as ::core::ffi::c_ushort,
        ucs: 0xe26 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc7 as ::core::ffi::c_ushort,
        ucs: 0xe27 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc8 as ::core::ffi::c_ushort,
        ucs: 0xe28 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdc9 as ::core::ffi::c_ushort,
        ucs: 0xe29 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdca as ::core::ffi::c_ushort,
        ucs: 0xe2a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdcb as ::core::ffi::c_ushort,
        ucs: 0xe2b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdcc as ::core::ffi::c_ushort,
        ucs: 0xe2c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdcd as ::core::ffi::c_ushort,
        ucs: 0xe2d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdce as ::core::ffi::c_ushort,
        ucs: 0xe2e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdcf as ::core::ffi::c_ushort,
        ucs: 0xe2f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd0 as ::core::ffi::c_ushort,
        ucs: 0xe30 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd1 as ::core::ffi::c_ushort,
        ucs: 0xe31 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd2 as ::core::ffi::c_ushort,
        ucs: 0xe32 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd3 as ::core::ffi::c_ushort,
        ucs: 0xe33 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd4 as ::core::ffi::c_ushort,
        ucs: 0xe34 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd5 as ::core::ffi::c_ushort,
        ucs: 0xe35 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd6 as ::core::ffi::c_ushort,
        ucs: 0xe36 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd7 as ::core::ffi::c_ushort,
        ucs: 0xe37 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd8 as ::core::ffi::c_ushort,
        ucs: 0xe38 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdd9 as ::core::ffi::c_ushort,
        ucs: 0xe39 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdda as ::core::ffi::c_ushort,
        ucs: 0xe3a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdde as ::core::ffi::c_ushort,
        ucs: 0xe3e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xddf as ::core::ffi::c_ushort,
        ucs: 0xe3f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde0 as ::core::ffi::c_ushort,
        ucs: 0xe40 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde1 as ::core::ffi::c_ushort,
        ucs: 0xe41 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde2 as ::core::ffi::c_ushort,
        ucs: 0xe42 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde3 as ::core::ffi::c_ushort,
        ucs: 0xe43 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde4 as ::core::ffi::c_ushort,
        ucs: 0xe44 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde5 as ::core::ffi::c_ushort,
        ucs: 0xe45 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde6 as ::core::ffi::c_ushort,
        ucs: 0xe46 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde7 as ::core::ffi::c_ushort,
        ucs: 0xe47 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde8 as ::core::ffi::c_ushort,
        ucs: 0xe48 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xde9 as ::core::ffi::c_ushort,
        ucs: 0xe49 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdea as ::core::ffi::c_ushort,
        ucs: 0xe4a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdeb as ::core::ffi::c_ushort,
        ucs: 0xe4b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdec as ::core::ffi::c_ushort,
        ucs: 0xe4c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xded as ::core::ffi::c_ushort,
        ucs: 0xe4d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf0 as ::core::ffi::c_ushort,
        ucs: 0xe50 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf1 as ::core::ffi::c_ushort,
        ucs: 0xe51 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf2 as ::core::ffi::c_ushort,
        ucs: 0xe52 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf3 as ::core::ffi::c_ushort,
        ucs: 0xe53 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf4 as ::core::ffi::c_ushort,
        ucs: 0xe54 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf5 as ::core::ffi::c_ushort,
        ucs: 0xe55 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf6 as ::core::ffi::c_ushort,
        ucs: 0xe56 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf7 as ::core::ffi::c_ushort,
        ucs: 0xe57 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf8 as ::core::ffi::c_ushort,
        ucs: 0xe58 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xdf9 as ::core::ffi::c_ushort,
        ucs: 0xe59 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea1 as ::core::ffi::c_ushort,
        ucs: 0x3131 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea2 as ::core::ffi::c_ushort,
        ucs: 0x3132 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea3 as ::core::ffi::c_ushort,
        ucs: 0x3133 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea4 as ::core::ffi::c_ushort,
        ucs: 0x3134 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea5 as ::core::ffi::c_ushort,
        ucs: 0x3135 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea6 as ::core::ffi::c_ushort,
        ucs: 0x3136 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea7 as ::core::ffi::c_ushort,
        ucs: 0x3137 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea8 as ::core::ffi::c_ushort,
        ucs: 0x3138 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xea9 as ::core::ffi::c_ushort,
        ucs: 0x3139 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeaa as ::core::ffi::c_ushort,
        ucs: 0x313a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeab as ::core::ffi::c_ushort,
        ucs: 0x313b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeac as ::core::ffi::c_ushort,
        ucs: 0x313c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xead as ::core::ffi::c_ushort,
        ucs: 0x313d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeae as ::core::ffi::c_ushort,
        ucs: 0x313e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeaf as ::core::ffi::c_ushort,
        ucs: 0x313f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb0 as ::core::ffi::c_ushort,
        ucs: 0x3140 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb1 as ::core::ffi::c_ushort,
        ucs: 0x3141 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb2 as ::core::ffi::c_ushort,
        ucs: 0x3142 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb3 as ::core::ffi::c_ushort,
        ucs: 0x3143 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb4 as ::core::ffi::c_ushort,
        ucs: 0x3144 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb5 as ::core::ffi::c_ushort,
        ucs: 0x3145 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb6 as ::core::ffi::c_ushort,
        ucs: 0x3146 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb7 as ::core::ffi::c_ushort,
        ucs: 0x3147 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb8 as ::core::ffi::c_ushort,
        ucs: 0x3148 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeb9 as ::core::ffi::c_ushort,
        ucs: 0x3149 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeba as ::core::ffi::c_ushort,
        ucs: 0x314a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xebb as ::core::ffi::c_ushort,
        ucs: 0x314b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xebc as ::core::ffi::c_ushort,
        ucs: 0x314c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xebd as ::core::ffi::c_ushort,
        ucs: 0x314d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xebe as ::core::ffi::c_ushort,
        ucs: 0x314e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xebf as ::core::ffi::c_ushort,
        ucs: 0x314f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec0 as ::core::ffi::c_ushort,
        ucs: 0x3150 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec1 as ::core::ffi::c_ushort,
        ucs: 0x3151 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec2 as ::core::ffi::c_ushort,
        ucs: 0x3152 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec3 as ::core::ffi::c_ushort,
        ucs: 0x3153 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec4 as ::core::ffi::c_ushort,
        ucs: 0x3154 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec5 as ::core::ffi::c_ushort,
        ucs: 0x3155 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec6 as ::core::ffi::c_ushort,
        ucs: 0x3156 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec7 as ::core::ffi::c_ushort,
        ucs: 0x3157 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec8 as ::core::ffi::c_ushort,
        ucs: 0x3158 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xec9 as ::core::ffi::c_ushort,
        ucs: 0x3159 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeca as ::core::ffi::c_ushort,
        ucs: 0x315a as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xecb as ::core::ffi::c_ushort,
        ucs: 0x315b as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xecc as ::core::ffi::c_ushort,
        ucs: 0x315c as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xecd as ::core::ffi::c_ushort,
        ucs: 0x315d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xece as ::core::ffi::c_ushort,
        ucs: 0x315e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xecf as ::core::ffi::c_ushort,
        ucs: 0x315f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed0 as ::core::ffi::c_ushort,
        ucs: 0x3160 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed1 as ::core::ffi::c_ushort,
        ucs: 0x3161 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed2 as ::core::ffi::c_ushort,
        ucs: 0x3162 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed3 as ::core::ffi::c_ushort,
        ucs: 0x3163 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed4 as ::core::ffi::c_ushort,
        ucs: 0x11a8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed5 as ::core::ffi::c_ushort,
        ucs: 0x11a9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed6 as ::core::ffi::c_ushort,
        ucs: 0x11aa as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed7 as ::core::ffi::c_ushort,
        ucs: 0x11ab as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed8 as ::core::ffi::c_ushort,
        ucs: 0x11ac as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xed9 as ::core::ffi::c_ushort,
        ucs: 0x11ad as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeda as ::core::ffi::c_ushort,
        ucs: 0x11ae as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xedb as ::core::ffi::c_ushort,
        ucs: 0x11af as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xedc as ::core::ffi::c_ushort,
        ucs: 0x11b0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xedd as ::core::ffi::c_ushort,
        ucs: 0x11b1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xede as ::core::ffi::c_ushort,
        ucs: 0x11b2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xedf as ::core::ffi::c_ushort,
        ucs: 0x11b3 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee0 as ::core::ffi::c_ushort,
        ucs: 0x11b4 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee1 as ::core::ffi::c_ushort,
        ucs: 0x11b5 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee2 as ::core::ffi::c_ushort,
        ucs: 0x11b6 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee3 as ::core::ffi::c_ushort,
        ucs: 0x11b7 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee4 as ::core::ffi::c_ushort,
        ucs: 0x11b8 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee5 as ::core::ffi::c_ushort,
        ucs: 0x11b9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee6 as ::core::ffi::c_ushort,
        ucs: 0x11ba as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee7 as ::core::ffi::c_ushort,
        ucs: 0x11bb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee8 as ::core::ffi::c_ushort,
        ucs: 0x11bc as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xee9 as ::core::ffi::c_ushort,
        ucs: 0x11bd as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeea as ::core::ffi::c_ushort,
        ucs: 0x11be as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeeb as ::core::ffi::c_ushort,
        ucs: 0x11bf as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeec as ::core::ffi::c_ushort,
        ucs: 0x11c0 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeed as ::core::ffi::c_ushort,
        ucs: 0x11c1 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeee as ::core::ffi::c_ushort,
        ucs: 0x11c2 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeef as ::core::ffi::c_ushort,
        ucs: 0x316d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef0 as ::core::ffi::c_ushort,
        ucs: 0x3171 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef1 as ::core::ffi::c_ushort,
        ucs: 0x3178 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef2 as ::core::ffi::c_ushort,
        ucs: 0x317f as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef4 as ::core::ffi::c_ushort,
        ucs: 0x3184 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef5 as ::core::ffi::c_ushort,
        ucs: 0x3186 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef6 as ::core::ffi::c_ushort,
        ucs: 0x318d as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef7 as ::core::ffi::c_ushort,
        ucs: 0x318e as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xef8 as ::core::ffi::c_ushort,
        ucs: 0x11eb as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xefa as ::core::ffi::c_ushort,
        ucs: 0x11f9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0xeff as ::core::ffi::c_ushort,
        ucs: 0x20a9 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x13bc as ::core::ffi::c_ushort,
        ucs: 0x152 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x13bd as ::core::ffi::c_ushort,
        ucs: 0x153 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x13be as ::core::ffi::c_ushort,
        ucs: 0x178 as ::core::ffi::c_ushort,
    },
    codepair {
        keysym: 0x20ac as ::core::ffi::c_ushort,
        ucs: 0x20ac as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub unsafe extern "C" fn keysym2ucs(mut keysym: xcb_keysym_t) -> ::core::ffi::c_long {
    let mut min: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max: ::core::ffi::c_int = (::core::mem::size_of::<[codepair; 739]>() as usize)
        .wrapping_div(::core::mem::size_of::<codepair>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    let mut mid: ::core::ffi::c_int = 0;
    if keysym >= 0x20 as xcb_keysym_t && keysym <= 0x7e as xcb_keysym_t
        || keysym >= 0xa0 as xcb_keysym_t && keysym <= 0xff as xcb_keysym_t
    {
        return keysym as ::core::ffi::c_long;
    }
    if keysym & 0xff000000 as xcb_keysym_t == 0x1000000 as xcb_keysym_t {
        return (keysym & 0xffffff as xcb_keysym_t) as ::core::ffi::c_long;
    }
    while max >= min {
        mid = (min + max) / 2 as ::core::ffi::c_int;
        if (keysymtab[mid as usize].keysym as xcb_keysym_t) < keysym {
            min = mid + 1 as ::core::ffi::c_int;
        } else if keysymtab[mid as usize].keysym as xcb_keysym_t > keysym {
            max = mid - 1 as ::core::ffi::c_int;
        } else {
            return keysymtab[mid as usize].ucs as ::core::ffi::c_long;
        }
    }
    return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
