// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::io::{self, Write};

/// Send an i3 IPC frame, retaining arbitrary binary payloads and message types.
/// Length and type use native byte order, as specified by the i3 IPC protocol.
pub fn send_message<W: Write + ?Sized>(
    writer: &mut W,
    message_type: u32,
    payload: &[u8],
) -> io::Result<()> {
    let length = u32::try_from(payload.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "IPC payload exceeds the u32 wire length",
        )
    })?;
    let mut header = [0; 14];
    header[..6].copy_from_slice(b"i3-ipc");
    header[6..10].copy_from_slice(&length.to_ne_bytes());
    header[10..14].copy_from_slice(&message_type.to_ne_bytes());
    writer.write_all(&header)?;
    writer.write_all(payload)
}

/// Legacy ABI and raw-descriptor I/O boundary. Framing lives only in send_message.
mod legacy {
    use super::*;
    use std::os::fd::RawFd;

    struct Descriptor(RawFd);
    impl Write for Descriptor {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            loop {
                // SAFETY: bytes is readable for its full length. The kernel
                // validates the raw descriptor; this wrapper never takes ownership.
                let written = unsafe { libc::write(self.0, bytes.as_ptr().cast(), bytes.len()) };
                if written >= 0 {
                    return Ok(written as usize);
                }
                let error = io::Error::last_os_error();
                // The old writeall helper retried EAGAIN as well as EINTR.
                if error.kind() != io::ErrorKind::WouldBlock {
                    return Err(error);
                }
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// # Safety
    /// `payload` must be readable for `message_size` bytes; null is allowed for zero.

    pub unsafe fn ipc_send_message(
        fd: libc::c_int,
        message_size: u32,
        message_type: u32,
        payload: *const u8,
    ) -> libc::c_int {
        let bytes = if message_size == 0 {
            &[]
        } else {
            // SAFETY: The caller supplies the readable payload buffer.
            unsafe { std::slice::from_raw_parts(payload, message_size as usize) }
        };
        match send_message(&mut Descriptor(fd), message_type, bytes) {
            Ok(()) => 0,
            Err(error) => {
                // SAFETY: Restore the underlying OS error after Rust I/O handling.
                unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO) };
                -1
            }
        }
    }
}
pub use legacy::ipc_send_message;
