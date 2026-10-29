// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/get_process_filename.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, mode_t};
unsafe extern "C" {
    fn sasprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn mkdirp(path: *const c_char, mode: mode_t) -> c_int;
    fn warn(fmt: *const c_char, ...);
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
}

/// # Safety
/// `prefix` must be a live C string. Calls must be serialized, as with the C
/// function's cached static directory. The caller frees the returned filename.
#[no_mangle]
pub unsafe extern "C" fn get_process_filename(prefix: *const c_char) -> *mut c_char {
    // SAFETY: DIR is touched only in i3's serialized event loop. All created
    // strings retain the upstream malloc ownership and mkdtemp mutates its buffer.
    unsafe {
        static mut DIR: *mut c_char = core::ptr::null_mut();
        if DIR.is_null() {
            DIR = libc::getenv(c"XDG_RUNTIME_DIR".as_ptr());
            if !DIR.is_null() {
                let mut tmp = core::ptr::null_mut();
                sasprintf(&mut tmp, c"%s/i3".as_ptr(), DIR);
                DIR = tmp;
                if mkdirp(DIR, 0o700) == -1 {
                    warn(c"Could not mkdirp(%s)".as_ptr(), DIR);
                    errx(
                        libc::EXIT_FAILURE,
                        c"Check permissions of $XDG_RUNTIME_DIR = '%s'".as_ptr(),
                        libc::getenv(c"XDG_RUNTIME_DIR".as_ptr()),
                    );
                }
            } else {
                let pw = libc::getpwuid(libc::getuid());
                let username = if pw.is_null() {
                    c"unknown".as_ptr()
                } else {
                    (*pw).pw_name
                };
                sasprintf(
                    core::ptr::addr_of_mut!(DIR),
                    c"/tmp/i3-%s.XXXXXX".as_ptr(),
                    username,
                );
                if libc::mkdtemp(DIR).is_null() {
                    libc::perror(c"mkdtemp()".as_ptr());
                    return core::ptr::null_mut();
                }
            }
        }
        let mut filename = core::ptr::null_mut();
        sasprintf(
            &mut filename,
            c"%s/%s.%d".as_ptr(),
            DIR,
            prefix,
            libc::getpid(),
        );
        filename
    }
}
