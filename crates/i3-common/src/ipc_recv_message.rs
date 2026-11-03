// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

#![deny(unsafe_code)]

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

fn read_payload<R: Read + ?Sized>(
    reader: &mut R,
    payload: &mut [u8],
    retry_would_block: bool,
) -> Result<(), ReceiveError> {
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
            Err(error)
                if error.kind() == io::ErrorKind::Interrupted
                    || (retry_would_block && error.kind() == io::ErrorKind::WouldBlock) =>
            {
                continue
            }
            Err(error) => return Err(ReceiveError::Io(error)),
        }
    }
    Ok(())
}

/// Read exactly one frame, leaving following frames untouched in the reader.
/// Owned payload storage is released automatically on success and failure.
pub fn receive_message<R: Read + ?Sized>(reader: &mut R) -> Result<Message, ReceiveError> {
    receive(reader, false)
}

/// Compatibility mode for the original nonblocking server and legacy clients.
/// Header OS errors return immediately, including Interrupted and WouldBlock.
/// Once a complete header is consumed, payload Interrupted/WouldBlock errors
/// retry without discarding bytes already received. No following frame is read.
pub fn receive_message_retry_payload<R: Read + ?Sized>(
    reader: &mut R,
) -> Result<Message, ReceiveError> {
    receive(reader, true)
}

fn receive<R: Read + ?Sized>(
    reader: &mut R,
    retry_would_block: bool,
) -> Result<Message, ReceiveError> {
    let header = read_header(reader)?;
    let mut payload = Vec::new();
    payload
        .try_reserve_exact(header.length)
        .map_err(|error| ReceiveError::Io(io::Error::new(io::ErrorKind::OutOfMemory, error)))?;
    payload.resize(header.length, 0);
    read_payload(reader, &mut payload, retry_would_block)?;
    Ok(Message {
        message_type: header.message_type,
        payload,
    })
}

/// Report the original libi3 protocol diagnostics after releasing reader borrows.
/// OS failures and EOF between frames remain the caller's reporting decision.
pub fn report_receive_error(error: &ReceiveError) {
    if matches!(error, ReceiveError::Io(_) | ReceiveError::Eof) {
        return;
    }
    let mut diagnostic = b"[libi3] ERROR: ".to_vec();
    diagnostic.extend_from_slice(&error.diagnostic_bytes());
    diagnostic.push(b'\n');
    crate::logging::emit(crate::logging::Level::Error, &diagnostic);
}
