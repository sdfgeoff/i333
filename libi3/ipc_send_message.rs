// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/ipc_send_message.c; C ABI and allocation ownership are preserved.

use libc::{c_int, c_void, size_t, ssize_t};
unsafe extern "C" {
    fn writeall(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

#[repr(C, packed)]
struct I3IpcHeader {
    magic: [u8; 6],
    size: u32,
    message_type: u32,
}

/// # Safety
/// `payload` must be readable for `message_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn ipc_send_message(
    sockfd: c_int,
    message_size: u32,
    message_type: u32,
    payload: *const u8,
) -> c_int {
    let header = I3IpcHeader {
        magic: *b"i3-ipc",
        size: message_size,
        message_type,
    };
    // SAFETY: The packed header has exactly the 14-byte native-endian C layout;
    // payload validity is the caller's contract. writeall borrows both buffers.
    unsafe {
        if writeall(
            sockfd,
            (&header as *const I3IpcHeader).cast(),
            core::mem::size_of::<I3IpcHeader>(),
        ) == -1
        {
            return -1;
        }
        if writeall(sockfd, payload.cast(), message_size as usize) == -1 {
            return -1;
        }
    }
    0
}
