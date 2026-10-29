// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/format_placeholders.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int};
unsafe extern "C" {
    fn sstrdup(string: *const c_char) -> *mut c_char;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct placeholder_t {
    pub name: *const c_char,
    pub value: *const c_char,
}

/// # Safety
/// `format` is null or a live C string. `placeholders` has `num` entries whose
/// name/value pointers reference C strings. Placeholder names are nonempty and
/// start with '%', as required by upstream's sizing and replacement passes.
#[no_mangle]
pub unsafe extern "C" fn format_placeholders(
    format: *mut c_char,
    placeholders: *mut placeholder_t,
    num: c_int,
) -> *mut c_char {
    if format.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: Both scans only advance through caller-provided C strings. The
    // first pass computes storage for the second pass, matching upstream's VLA.
    unsafe {
        let mut buffer_len = (libc::strlen(format) + 1) as c_int;
        let mut walk = format;
        while *walk != 0 {
            for i in 0..num {
                let placeholder = &*placeholders.add(i as usize);
                if libc::strncmp(walk, placeholder.name, libc::strlen(placeholder.name)) != 0 {
                    continue;
                }
                buffer_len = (buffer_len as usize)
                    .wrapping_sub(libc::strlen(placeholder.name))
                    .wrapping_add(libc::strlen(placeholder.value))
                    as c_int;
                walk = walk.add(libc::strlen(placeholder.name) - 1);
                break;
            }
            walk = walk.add(1);
        }
        // PORT NOTE: Vec supplies the variable-size scratch buffer formerly a C
        // VLA; returned storage still comes from sstrdup, with identical ownership.
        let mut buffer = vec![0 as c_char; buffer_len as usize];
        let mut outwalk = buffer.as_mut_ptr();
        walk = format;
        while *walk != 0 {
            if *walk != b'%' as c_char {
                *outwalk = *walk;
                outwalk = outwalk.add(1);
                walk = walk.add(1);
                continue;
            }
            let mut matched = false;
            for i in 0..num {
                let placeholder = &*placeholders.add(i as usize);
                if libc::strncmp(walk, placeholder.name, libc::strlen(placeholder.name)) != 0 {
                    continue;
                }
                matched = true;
                outwalk =
                    outwalk.add(libc::sprintf(outwalk, c"%s".as_ptr(), placeholder.value) as usize);
                walk = walk.add(libc::strlen(placeholder.name) - 1);
                break;
            }
            if !matched {
                *outwalk = *walk;
                outwalk = outwalk.add(1);
            }
            walk = walk.add(1);
        }
        *outwalk = 0;
        sstrdup(buffer.as_ptr())
    }
}
