// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/ipc_recv_message.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, c_void, size_t};
unsafe extern "C" {
    fn smalloc(size: size_t) -> *mut c_void;
    fn errorlog(fmt: *const c_char, ...);
}

/// # Safety
/// `reply_length` and `reply` are writable output pointers; `message_type` is
/// either null or writable. The caller frees the returned buffer with free().
#[no_mangle]
pub unsafe extern "C" fn ipc_recv_message(
    sockfd: c_int,
    message_type: *mut u32,
    reply_length: *mut u32,
    reply: *mut *mut u8,
) -> c_int {
    // SAFETY: Output pointers follow the C contract. Reads are bounded by the
    // header size or allocated payload size; protocol fields use unaligned copies.
    unsafe {
        let to_read = 14u32;
        let mut msg = [0u8; 14];
        let mut read_bytes = 0u32;
        while read_bytes < to_read {
            let n = libc::read(
                sockfd,
                msg.as_mut_ptr().add(read_bytes as usize).cast(),
                (to_read - read_bytes) as usize,
            );
            if n == -1 {
                return -1;
            }
            if n == 0 {
                if read_bytes == 0 {
                    return -2;
                }
                errorlog(c"[libi3] ERROR: IPC: unexpected EOF while reading header, got %u bytes, want %u bytes\n".as_ptr(), read_bytes, to_read);
                return -3;
            }
            read_bytes += n as u32;
        }
        if &msg[..6] != b"i3-ipc" {
            errorlog(
                c"[libi3] ERROR: IPC: invalid magic in header, got \"%.*s\", want \"%s\"\n"
                    .as_ptr(),
                6 as c_int,
                msg.as_ptr(),
                c"i3-ipc".as_ptr(),
            );
            return -3;
        }
        core::ptr::copy_nonoverlapping(msg.as_ptr().add(6), reply_length.cast::<u8>(), 4);
        if !message_type.is_null() {
            core::ptr::copy_nonoverlapping(msg.as_ptr().add(10), message_type.cast::<u8>(), 4);
        }
        *reply = smalloc(*reply_length as usize).cast();
        read_bytes = 0;
        while read_bytes < *reply_length {
            let n = libc::read(
                sockfd,
                (*reply).add(read_bytes as usize).cast(),
                (*reply_length - read_bytes) as usize,
            );
            if n == -1 {
                let errno = *libc::__errno_location();
                if errno == libc::EINTR || errno == libc::EAGAIN {
                    continue;
                }
                return -1;
            }
            if n == 0 {
                errorlog(c"[libi3] ERROR: IPC: unexpected EOF while reading payload, got %u bytes, want %u bytes\n".as_ptr(), read_bytes, *reply_length);
                return -3;
            }
            read_bytes += n as u32;
        }
        0
    }
}
