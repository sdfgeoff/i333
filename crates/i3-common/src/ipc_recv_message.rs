// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::fmt;
use std::io::{self, Read};

/// An owned i3 IPC frame. No UTF-8 or JSON interpretation is performed here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_type: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub enum ReceiveError {
    Io(io::Error),
    Eof,
    TruncatedHeader { received: usize },
    InvalidMagic([u8; 6]),
    TruncatedPayload { received: usize, expected: usize },
}

impl ReceiveError {
    /// Original protocol diagnostic bytes, including non-UTF-8 invalid magic.
    /// EOF between frames is distinguished from a protocol violation.
    pub fn diagnostic_bytes(&self) -> Vec<u8> {
        match self {
            Self::Io(error) => format!("IPC: read(): {error}").into_bytes(),
            Self::Eof => b"IPC: unexpected EOF".to_vec(),
            Self::TruncatedHeader { received } => format!(
                "IPC: unexpected EOF while reading header, got {received} bytes, want 14 bytes"
            ).into_bytes(),
            Self::InvalidMagic(magic) => {
                let mut result = b"IPC: invalid magic in header, got \"".to_vec();
                let end = magic.iter().position(|&byte| byte == 0).unwrap_or(magic.len());
                result.extend_from_slice(&magic[..end]);
                result.extend_from_slice(b"\", want \"i3-ipc\"");
                result
            }
            Self::TruncatedPayload { received, expected } => format!(
                "IPC: unexpected EOF while reading payload, got {received} bytes, want {expected} bytes"
            ).into_bytes(),
        }
    }
}
impl fmt::Display for ReceiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&String::from_utf8_lossy(&self.diagnostic_bytes()))
    }
}
impl std::error::Error for ReceiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

struct Header {
    length: usize,
    message_type: u32,
}

fn read_header<R: Read + ?Sized>(reader: &mut R) -> Result<Header, ReceiveError> {
    let mut header = [0; 14];
    let mut received = 0;
    while received < header.len() {
        match reader.read(&mut header[received..]) {
            Ok(0) if received == 0 => return Err(ReceiveError::Eof),
            Ok(0) => return Err(ReceiveError::TruncatedHeader { received }),
            Ok(count) => received += count,
            // The original header reader reports OS errors, including EINTR.
            Err(error) => return Err(ReceiveError::Io(error)),
        }
    }
    let magic: [u8; 6] = header[..6].try_into().unwrap();
    if magic != *b"i3-ipc" {
        return Err(ReceiveError::InvalidMagic(magic));
    }
    Ok(Header {
        length: u32::from_ne_bytes(header[6..10].try_into().unwrap()) as usize,
        message_type: u32::from_ne_bytes(header[10..14].try_into().unwrap()),
    })
}

fn read_payload<R: Read + ?Sized>(reader: &mut R, payload: &mut [u8]) -> Result<(), ReceiveError> {
    let mut received = 0;
    while received < payload.len() {
        match reader.read(&mut payload[received..]) {
            Ok(0) => {
                return Err(ReceiveError::TruncatedPayload {
                    received,
                    expected: payload.len(),
                })
            }
            Ok(count) => received += count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(ReceiveError::Io(error)),
        }
    }
    Ok(())
}

/// Read exactly one frame, leaving following frames untouched in the reader.
/// Owned payload storage is released automatically on success and failure.
pub fn receive_message<R: Read + ?Sized>(reader: &mut R) -> Result<Message, ReceiveError> {
    let header = read_header(reader)?;
    let mut payload = Vec::new();
    payload
        .try_reserve_exact(header.length)
        .map_err(|error| ReceiveError::Io(io::Error::new(io::ErrorKind::OutOfMemory, error)))?;
    payload.resize(header.length, 0);
    read_payload(reader, &mut payload)?;
    Ok(Message {
        message_type: header.message_type,
        payload,
    })
}

/// Legacy ABI boundary: malloc-owned output and integer status codes. Both this
/// adapter and the safe API use the same header and payload decoders above.
mod legacy {
    use super::*;
    use std::ffi::CString;
    use std::os::fd::RawFd;

    struct Descriptor {
        fd: RawFd,
        retry_would_block: bool,
    }
    impl Read for Descriptor {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            loop {
                // SAFETY: bytes is writable for its length; the kernel validates
                // the borrowed raw fd, including invalid descriptors.
                let count = unsafe { libc::read(self.fd, bytes.as_mut_ptr().cast(), bytes.len()) };
                if count >= 0 {
                    return Ok(count as usize);
                }
                let error = io::Error::last_os_error();
                if !(self.retry_would_block && error.kind() == io::ErrorKind::WouldBlock) {
                    return Err(error);
                }
            }
        }
    }

    fn status(error: ReceiveError) -> libc::c_int {
        match error {
            ReceiveError::Io(error) => {
                // SAFETY: Preserve the original errno contract for C callers.
                unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO) };
                -1
            }
            ReceiveError::Eof => -2,
            error => {
                let diagnostic =
                    CString::new(error.diagnostic_bytes()).expect("protocol diagnostic has no NUL");
                // SAFETY: diagnostic is a live C string and the format is static.
                unsafe { errorlog(c"[libi3] ERROR: %s\n".as_ptr(), diagnostic.as_ptr()) };
                -3
            }
        }
    }

    /// # Safety
    /// `reply_length` and `reply` must be writable; `message_type` may be null.
    /// The caller frees returned storage with free(), including on payload errors.

    pub unsafe fn ipc_recv_message(
        fd: libc::c_int,
        message_type: *mut u32,
        reply_length: *mut u32,
        reply: *mut *mut u8,
    ) -> libc::c_int {
        let mut reader = Descriptor {
            fd,
            retry_would_block: false,
        };
        let header = match read_header(&mut reader) {
            Ok(header) => header,
            Err(error) => return status(error),
        };
        // SAFETY: The caller provides writable outputs. scalloc returns malloc-
        // compatible zeroed storage, allowing a safe initialized slice for Read.
        let payload = unsafe {
            *reply_length = header.length as u32;
            if !message_type.is_null() {
                *message_type = header.message_type;
            }
            *reply = crate::safewrappers::scalloc(header.length, 1).cast();
            std::slice::from_raw_parts_mut(*reply, header.length)
        };
        reader.retry_would_block = true;
        match read_payload(&mut reader, payload) {
            Ok(()) => 0,
            Err(error) => status(error),
        }
    }
}
pub use legacy::ipc_recv_message;

use crate::logging::errorlog;
