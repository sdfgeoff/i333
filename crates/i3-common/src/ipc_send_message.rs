// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

#![forbid(unsafe_code)]

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

/// Send using the historical descriptor policy of retrying WouldBlock writes.
/// Ordinary send_message keeps standard Write behavior for modern callers.
pub fn send_message_retry_would_block<W: Write + ?Sized>(
    writer: &mut W,
    message_type: u32,
    payload: &[u8],
) -> io::Result<()> {
    struct RetryWouldBlock<'a, W: Write + ?Sized>(&'a mut W);
    impl<W: Write + ?Sized> Write for RetryWouldBlock<'_, W> {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            loop {
                match self.0.write(bytes) {
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => continue,
                    result => return result,
                }
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            self.0.flush()
        }
    }
    send_message(&mut RetryWouldBlock(writer), message_type, payload)
}
