// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/nonblock.c; C ABI and allocation ownership are preserved.

use libc::c_int;
unsafe extern "C" {
    fn err(eval: c_int, fmt: *const libc::c_char, ...) -> !;
}

#[no_mangle]
pub extern "C" fn set_nonblock(sockfd: c_int) {
    // SAFETY: fcntl's integer commands do not dereference caller memory.
    unsafe {
        let mut flags = libc::fcntl(sockfd, libc::F_GETFL, 0);
        if flags & libc::O_NONBLOCK != 0 {
            return;
        }
        flags |= libc::O_NONBLOCK;
        if libc::fcntl(sockfd, libc::F_SETFL, flags) < 0 {
            err(-1, c"Could not set O_NONBLOCK".as_ptr());
        }
    }
}
