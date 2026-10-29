/*-*- Mode: C; c-basic-offset: 8; indent-tabs-mode: nil -*-*/

/***
  Copyright 2010 Lennart Poettering

  Permission is hereby granted, free of charge, to any person
  obtaining a copy of this software and associated documentation files
  (the "Software"), to deal in the Software without restriction,
  including without limitation the rights to use, copy, modify, merge,
  publish, distribute, sublicense, and/or sell copies of the Software,
  and to permit persons to whom the Software is furnished to do so,
  subject to the following conditions:

  The above copyright notice and this permission notice shall be
  included in all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
  NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
  ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
  CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
***/
//! Small systemd protocol helpers. Environment values and notification buffers
//! are owned; the remaining unsafe operations are native syscalls or explicitly
//! documented compatibility boundaries.

use i3_common::{i333_va_list, native_stdio::vasprintf};
use std::ffi::{c_char, c_int, CStr, OsStr};
use std::fs;
use std::io;
use std::mem::{offset_of, size_of, MaybeUninit};
use std::os::fd::RawFd;
use std::os::linux::net::SocketAddrExt;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::{SocketAddr, UnixDatagram};
use std::path::Path;
use std::ptr::NonNull;

pub const SD_LISTEN_FDS_START: c_int = 3;

fn negative_errno(error: io::Error) -> c_int {
    -error.raw_os_error().unwrap_or(libc::EINVAL)
}

fn last_error() -> c_int {
    negative_errno(io::Error::last_os_error())
}

/// Match strtoul's decimal environment grammar, including unsigned negation,
/// locale whitespace, overflow precedence, and the empty FDS value.
fn environment_number(bytes: &[u8]) -> Result<libc::c_ulong, c_int> {
    let mut cursor = 0;
    while bytes.get(cursor).is_some_and(|byte| {
        // Every u8 is in isspace's valid unsigned-character domain.
        unsafe { libc::isspace(c_int::from(*byte)) != 0 }
    }) {
        cursor += 1;
    }
    let negative = bytes.get(cursor) == Some(&b'-');
    if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
        cursor += 1;
    }
    let digits_start = cursor;
    let mut number: libc::c_ulong = 0;
    let mut overflow = false;
    while let Some(byte @ b'0'..=b'9') = bytes.get(cursor) {
        match number
            .checked_mul(10)
            .and_then(|number| number.checked_add((byte - b'0').into()))
        {
            Some(value) => number = value,
            None => overflow = true,
        }
        cursor += 1;
    }
    if overflow {
        return Err(-libc::ERANGE);
    }
    // strtoul points back to the original start if it consumed no digits.
    if digits_start == cursor && !bytes.is_empty() || cursor != bytes.len() {
        return Err(-libc::EINVAL);
    }
    Ok(if negative {
        number.wrapping_neg()
    } else {
        number
    })
}

fn inherited_fds(
    pid: Option<&[u8]>,
    count: Option<&[u8]>,
    process_id: u32,
    mark_cloexec: impl FnMut(RawFd) -> Result<(), c_int>,
) -> c_int {
    inherited_fds_with_parser(pid, count, process_id, environment_number, mark_cloexec)
}

fn inherited_fds_with_parser(
    pid: Option<&[u8]>,
    count: Option<&[u8]>,
    process_id: u32,
    mut parse: impl FnMut(&[u8]) -> Result<libc::c_ulong, c_int>,
    mut mark_cloexec: impl FnMut(RawFd) -> Result<(), c_int>,
) -> c_int {
    let Some(pid) = pid else { return 0 };
    let pid = match parse(pid) {
        Ok(0) => return -libc::EINVAL,
        Ok(pid) => pid,
        Err(error) => return error,
    };
    if pid as libc::pid_t != process_id as libc::pid_t {
        return 0;
    }
    let Some(count) = count else { return 0 };
    let count = match parse(count) {
        Ok(count) => count as c_int,
        Err(error) => return error,
    };
    // The historical API narrows the unsigned count to int. Keep its wrapping
    // range/result, without introducing a debug-build overflow panic.
    for fd in SD_LISTEN_FDS_START..SD_LISTEN_FDS_START.wrapping_add(count) {
        if let Err(error) = mark_cloexec(fd) {
            return error;
        }
    }
    count
}

fn mark_cloexec(fd: RawFd) -> Result<(), c_int> {
    // These integer-only syscalls validate even invalid descriptors themselves.
    // In particular, environment data does not justify borrowing an OwnedFd.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags < 0 {
        return Err(last_error());
    }
    if flags & libc::FD_CLOEXEC == 0
        && unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) } < 0
    {
        return Err(last_error());
    }
    Ok(())
}

/// Inspect activation variables without taking ownership of inherited fds or
/// mutating the process environment.
pub fn listen_fds() -> c_int {
    let pid = std::env::var_os("LISTEN_PID");
    let count = std::env::var_os("LISTEN_FDS");
    inherited_fds_with_parser(
        pid.as_deref().map(OsStrExt::as_bytes),
        count.as_deref().map(OsStrExt::as_bytes),
        std::process::id(),
        |bytes| {
            let result = environment_number(bytes);
            // The compatibility API also exposes strtoul's thread-local errno
            // side effect, distinct from its negative return-value convention.
            unsafe {
                *libc::__errno_location() = if result == Err(-libc::ERANGE) {
                    libc::ERANGE
                } else {
                    0
                };
            }
            result
        },
        mark_cloexec,
    )
}

/// Compatibility entry point that can also clear activation variables.
///
/// # Safety
/// If `unset_environment` is nonzero, the caller must ensure no other thread
/// reads or modifies the process environment, including native libraries.
pub unsafe fn sd_listen_fds(unset_environment: c_int) -> c_int {
    let result = listen_fds();
    if unset_environment != 0 {
        unsafe {
            libc::unsetenv(c"LISTEN_PID".as_ptr());
            libc::unsetenv(c"LISTEN_FDS".as_ptr());
        }
    }
    result
}

fn descriptor_metadata(fd: RawFd) -> Result<libc::stat, c_int> {
    if fd < 0 {
        return Err(-libc::EINVAL);
    }
    let mut metadata = MaybeUninit::<libc::stat>::zeroed();
    // fstat writes this complete native structure on success. Zeroing also
    // leaves any platform padding initialized; no descriptor ownership changes.
    if unsafe { libc::fstat(fd, metadata.as_mut_ptr()) } < 0 {
        return Err(last_error());
    }
    Ok(unsafe { metadata.assume_init() })
}

fn fifo_metadata(fd: RawFd) -> Result<Option<libc::stat>, c_int> {
    let metadata = descriptor_metadata(fd)?;
    Ok((metadata.st_mode & libc::S_IFMT == libc::S_IFIFO).then_some(metadata))
}

fn fifo_path_matches(metadata: &libc::stat, path: &Path) -> c_int {
    match fs::metadata(path) {
        Ok(path_metadata) => {
            (metadata.st_dev == path_metadata.dev() && metadata.st_ino == path_metadata.ino())
                as c_int
        }
        Err(error) if matches!(error.raw_os_error(), Some(libc::ENOENT | libc::ENOTDIR)) => 0,
        Err(error) => negative_errno(error),
    }
}

pub fn is_fifo(fd: RawFd, path: Option<&Path>) -> c_int {
    match fifo_metadata(fd) {
        Ok(Some(metadata)) => path.map_or(1, |path| fifo_path_matches(&metadata, path)),
        Ok(None) => 0,
        Err(error) => error,
    }
}

/// # Safety
/// A non-null path must point to a valid NUL-terminated string when `fd` is a FIFO.
pub unsafe fn sd_is_fifo(fd: RawFd, path: *const c_char) -> c_int {
    let metadata = match fifo_metadata(fd) {
        Ok(Some(metadata)) => metadata,
        Ok(None) => return 0,
        Err(error) => return error,
    };
    if path.is_null() {
        return 1;
    }
    let path = unsafe { CStr::from_ptr(path) };
    fifo_path_matches(&metadata, Path::new(OsStr::from_bytes(path.to_bytes())))
}

fn socket_option(fd: RawFd, option: c_int) -> Result<c_int, c_int> {
    let mut value: c_int = 0;
    let mut length = size_of::<c_int>() as libc::socklen_t;
    // The kernel sees a correctly sized writable integer and length pair.
    if unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            option,
            (&raw mut value).cast(),
            &raw mut length,
        )
    } < 0
    {
        return Err(last_error());
    }
    if length as usize != size_of::<c_int>() {
        return Err(-libc::EINVAL);
    }
    Ok(value)
}

fn socket_matches(fd: RawFd, socket_type: c_int, listening: c_int) -> Result<bool, c_int> {
    if socket_type < 0 {
        return Err(-libc::EINVAL);
    }
    if descriptor_metadata(fd)?.st_mode & libc::S_IFMT != libc::S_IFSOCK {
        return Ok(false);
    }
    if socket_type != 0 && socket_option(fd, libc::SO_TYPE)? != socket_type {
        return Ok(false);
    }
    if listening >= 0 && (socket_option(fd, libc::SO_ACCEPTCONN)? == 0) != (listening == 0) {
        return Ok(false);
    }
    Ok(true)
}

// The zero-sized alignment member gives the byte buffer sockaddr_storage's
// native alignment without treating arbitrary address bytes as Rust fields.
#[repr(C)]
struct AddressStorage {
    _alignment: [libc::sockaddr_storage; 0],
    bytes: [u8; size_of::<libc::sockaddr_storage>()],
}

struct SocketAddress {
    storage: AddressStorage,
    length: usize,
}

impl SocketAddress {
    fn query(fd: RawFd) -> Result<Self, c_int> {
        let mut storage = AddressStorage {
            _alignment: [],
            bytes: [0; size_of::<libc::sockaddr_storage>()],
        };
        let mut length = storage.bytes.len() as libc::socklen_t;
        // The aligned, initialized buffer is writable for the supplied length.
        if unsafe { libc::getsockname(fd, storage.bytes.as_mut_ptr().cast(), &raw mut length) } < 0
        {
            return Err(last_error());
        }
        let length = length as usize;
        if !(size_of::<libc::sa_family_t>()..=storage.bytes.len()).contains(&length) {
            return Err(-libc::EINVAL);
        }
        Ok(Self { storage, length })
    }

    fn family(&self) -> c_int {
        libc::sa_family_t::from_ne_bytes([self.storage.bytes[0], self.storage.bytes[1]]).into()
    }

    fn port(&self) -> Result<Option<u16>, c_int> {
        let offset = match self.family() {
            libc::AF_INET if self.length >= size_of::<libc::sockaddr_in>() => {
                offset_of!(libc::sockaddr_in, sin_port)
            }
            libc::AF_INET6 if self.length >= size_of::<libc::sockaddr_in6>() => {
                offset_of!(libc::sockaddr_in6, sin6_port)
            }
            libc::AF_INET | libc::AF_INET6 => return Err(-libc::EINVAL),
            _ => return Ok(None),
        };
        Ok(Some(u16::from_be_bytes([
            self.storage.bytes[offset],
            self.storage.bytes[offset + 1],
        ])))
    }

    fn unix_path(&self) -> UnixPath {
        let offset = offset_of!(libc::sockaddr_un, sun_path);
        // Linux can report109 pathname bytes, including an appended NUL, for a
        // full108-byte sun_path. Keep the larger storage view just as the old
        // sockaddr_storage union did; do not truncate it to sockaddr_un.
        UnixPath {
            bytes: self.storage.bytes[offset..self.length].to_vec(),
        }
    }
}

pub fn sd_is_socket(fd: RawFd, family: c_int, socket_type: c_int, listening: c_int) -> c_int {
    if family < 0 {
        return -libc::EINVAL;
    }
    match socket_matches(fd, socket_type, listening) {
        Ok(false) => 0,
        Err(error) => error,
        Ok(true) if family == 0 => 1,
        Ok(true) => match SocketAddress::query(fd) {
            Ok(address) => (address.family() == family) as c_int,
            Err(error) => error,
        },
    }
}

pub fn sd_is_socket_inet(
    fd: RawFd,
    family: c_int,
    socket_type: c_int,
    listening: c_int,
    port: u16,
) -> c_int {
    if !matches!(family, 0 | libc::AF_INET | libc::AF_INET6) {
        return -libc::EINVAL;
    }
    match socket_matches(fd, socket_type, listening) {
        Ok(false) => return 0,
        Err(error) => return error,
        Ok(true) => {}
    }
    let address = match SocketAddress::query(fd) {
        Ok(address) => address,
        Err(error) => return error,
    };
    if !matches!(address.family(), libc::AF_INET | libc::AF_INET6)
        || family != 0 && address.family() != family
    {
        return 0;
    }
    if port == 0 {
        return 1;
    }
    match address.port() {
        Ok(actual) => (actual == Some(port)) as c_int,
        Err(error) => error,
    }
}

struct UnixPath {
    bytes: Vec<u8>,
}

impl UnixPath {
    fn matches(&self, requested: &[u8]) -> c_int {
        if requested.is_empty() {
            return (self.bytes.is_empty()) as c_int;
        }
        if requested[0] == 0 {
            return (self.bytes.len() == requested.len()
                && self.bytes.get(..requested.len()) == Some(requested))
                as c_int;
        }
        // Filesystem requests include the trailing NUL. The historical API
        // permits a returned address with a longer length than the request.
        (self.bytes.len() >= requested.len()
            && self.bytes.get(..requested.len()) == Some(requested)) as c_int
    }
}

fn unix_socket_path(
    fd: RawFd,
    socket_type: c_int,
    listening: c_int,
) -> Result<Option<UnixPath>, c_int> {
    if !socket_matches(fd, socket_type, listening)? {
        return Ok(None);
    }
    let address = SocketAddress::query(fd)?;
    if address.family() != libc::AF_UNIX {
        return Ok(None);
    }
    Ok(Some(address.unix_path()))
}

/// # Safety
/// A non-null path must be readable as a C string when length is zero. With an
/// explicit length, abstract paths need `length` bytes and filesystem paths
/// need `length + 1` readable bytes, as in the original API.
pub unsafe fn sd_is_socket_unix(
    fd: RawFd,
    socket_type: c_int,
    listening: c_int,
    path: *const c_char,
    length: usize,
) -> c_int {
    let address = match unix_socket_path(fd, socket_type, listening) {
        Ok(Some(address)) => address,
        Ok(None) => return 0,
        Err(error) => return error,
    };
    if path.is_null() {
        return 1;
    }
    if length == 0 {
        let path = unsafe { CStr::from_ptr(path) };
        return address.matches(if path.is_empty() {
            &[]
        } else {
            path.to_bytes_with_nul()
        });
    }
    let abstract_path = unsafe { *path == 0 };
    let Some(length) = length.checked_add(usize::from(!abstract_path)) else {
        return -libc::EINVAL;
    };
    // No valid sockaddr_un can match a longer request. Check this before
    // constructing a slice so malformed lengths never overread socket storage.
    if length > address.bytes.len() {
        return 0;
    }
    let requested = unsafe { std::slice::from_raw_parts(path.cast::<u8>(), length) };
    address.matches(requested)
}

fn notification_address(endpoint: &[u8]) -> Result<SocketAddr, c_int> {
    if endpoint.len() < 2 || endpoint.len() > 108 || !matches!(endpoint[0], b'@' | b'/') {
        return Err(-libc::EINVAL);
    }
    // The old strncpy copies only107 bytes even for an allowed108-byte value.
    // In an abstract name the final zero is significant, so retain it exactly.
    if endpoint[0] == b'@' {
        let mut name = endpoint[1..endpoint.len().min(107)].to_vec();
        if endpoint.len() == 108 {
            name.push(0);
        }
        SocketAddr::from_abstract_name(name).map_err(negative_errno)
    } else {
        SocketAddr::from_pathname(OsStr::from_bytes(&endpoint[..endpoint.len().min(107)]))
            .map_err(negative_errno)
    }
}

fn notify_socket(endpoint: &[u8], state: &[u8]) -> c_int {
    let address = match notification_address(endpoint) {
        Ok(address) => address,
        Err(error) => return error,
    };
    let socket = match UnixDatagram::unbound() {
        Ok(socket) => socket,
        Err(error) => return negative_errno(error),
    };
    match socket.send_to_addr(state, &address) {
        Ok(_) => 1,
        Err(error) => negative_errno(error),
    }
}

/// Send to the owned NOTIFY_SOCKET value without mutating the environment.
pub fn notify(state: &CStr) -> c_int {
    std::env::var_os("NOTIFY_SOCKET").map_or(0, |endpoint| {
        notify_socket(endpoint.as_bytes(), state.to_bytes())
    })
}

/// # Safety
/// If `unset_environment` is nonzero, no other thread may read or modify the
/// process environment, including through native libraries.
pub unsafe fn sd_notify(unset_environment: c_int, state: &CStr) -> c_int {
    let Some(endpoint) = std::env::var_os("NOTIFY_SOCKET") else {
        return 0;
    };
    let result = notify_socket(endpoint.as_bytes(), state.to_bytes());
    if unset_environment != 0 {
        unsafe {
            libc::unsetenv(c"NOTIFY_SOCKET".as_ptr());
        }
    }
    result
}

struct FormattedMessage(NonNull<c_char>);
impl Drop for FormattedMessage {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr().cast());
        }
    }
}

/// # Safety
/// `format` and all variadic arguments must satisfy printf's contracts. When
/// clearing NOTIFY_SOCKET, the environment must also be exclusively accessible.
pub unsafe extern "C" fn sd_notifyf(
    unset_environment: c_int,
    format: *const c_char,
    args: ...
) -> c_int {
    let mut arguments = args.clone();
    let mut message = std::ptr::null_mut();
    let result = unsafe { vasprintf(&raw mut message, format, i333_va_list!(arguments)) };
    let Some(message) = NonNull::new(message).filter(|_| result >= 0) else {
        return -libc::ENOMEM;
    };
    let message = FormattedMessage(message);
    let state = unsafe { CStr::from_ptr(message.0.as_ptr()) };
    unsafe { sd_notify(unset_environment, state) }
}

pub fn sd_booted() -> c_int {
    match (
        fs::symlink_metadata("/sys/fs/cgroup"),
        fs::symlink_metadata("/sys/fs/cgroup/systemd"),
    ) {
        (Ok(root), Ok(systemd)) => (root.dev() != systemd.dev()) as c_int,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, UdpSocket};
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::net::UnixListener;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    struct WorkDir(PathBuf);
    impl WorkDir {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let root = std::env::var_os("TMPDIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.work/tmp")
                });
            let path = root.join(format!(
                "sd-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for WorkDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn inherited_count_grammar_and_fd_error_order_match_strtoul() {
        for (bytes, expected) in [
            (&b""[..], Ok(0)),
            (&b"  +12"[..], Ok(12)),
            (&b"-1"[..], Ok(libc::c_ulong::MAX)),
            (&b"  "[..], Err(-libc::EINVAL)),
            (&b"+"[..], Err(-libc::EINVAL)),
            (&b"12 "[..], Err(-libc::EINVAL)),
            (&b"0x10"[..], Err(-libc::EINVAL)),
            (&b"18446744073709551616junk"[..], Err(-libc::ERANGE)),
        ] {
            assert_eq!(environment_number(bytes), expected, "{bytes:?}");
        }
        let mut descriptors = Vec::new();
        assert_eq!(
            inherited_fds(Some(b"42"), Some(b"3"), 42, |fd| {
                descriptors.push(fd);
                if fd == 4 {
                    Err(-libc::EBADF)
                } else {
                    Ok(())
                }
            }),
            -libc::EBADF
        );
        assert_eq!(descriptors, [3, 4]);
        assert_eq!(
            inherited_fds(Some(b"43"), Some(b"bad"), 42, |_| panic!()),
            0
        );
        assert_eq!(
            inherited_fds(Some(b"4294967338"), Some(b"0"), 42, |_| panic!()),
            0
        );
        assert_eq!(
            inherited_fds(Some(b"42"), Some(b"-1"), 42, |_| panic!()),
            -1
        );
        assert_eq!(
            inherited_fds(Some(b"42"), Some(b"2147483647"), 42, |_| panic!()),
            i32::MAX
        );
    }

    #[test]
    fn environment_number_matches_native_strtoul_for_byte_prefixes_and_overflows() {
        fn native(bytes: &[u8]) -> Result<libc::c_ulong, c_int> {
            let text = std::ffi::CString::new(bytes).unwrap();
            let mut end = std::ptr::null_mut();
            // strtoul's output is confined to its supplied string/end pointer;
            // errno is thread-local, so no process environment changes occur.
            unsafe {
                *libc::__errno_location() = 0;
                let value = libc::strtoul(text.as_ptr(), &raw mut end, 10);
                let error = *libc::__errno_location();
                if error != 0 {
                    Err(-error)
                } else if *end != 0 {
                    Err(-libc::EINVAL)
                } else {
                    Ok(value)
                }
            }
        }
        for prefix in 1..=255 {
            for suffix in [&b"42"[..], &b"-1"[..], &b"18446744073709551616junk"[..]] {
                let mut bytes = vec![prefix];
                bytes.extend_from_slice(suffix);
                assert_eq!(environment_number(&bytes), native(&bytes), "{bytes:?}");
            }
        }
    }

    #[test]
    fn full_length_filesystem_socket_and_notification_truncation_are_preserved() {
        let work = WorkDir::new();
        let prefix = work.0.as_os_str().as_bytes().len() + 1;
        assert!(prefix < 107);
        let path = work.0.join("s".repeat(107 - prefix));
        let receiver = UnixDatagram::bind(&path).unwrap();
        receiver
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut endpoint = path.as_os_str().as_bytes().to_vec();
        endpoint.push(b'x');
        assert_eq!(endpoint.len(), 108);
        assert_eq!(notify_socket(&endpoint, b"truncated"), 1);
        let mut buffer = [0; 16];
        let received = receiver.recv(&mut buffer).unwrap();
        assert_eq!(&buffer[..received], b"truncated");

        // A kernel-bound108-byte pathname has no terminating NUL in sun_path.
        // getsockname appends it and can return111 bytes into larger storage.
        let path = work.0.join("t".repeat(108 - prefix));
        let raw_receiver = UnixDatagram::unbound().unwrap();
        let mut address = AddressStorage {
            _alignment: [],
            bytes: [0; size_of::<libc::sockaddr_storage>()],
        };
        address.bytes[..2].copy_from_slice(&(libc::AF_UNIX as libc::sa_family_t).to_ne_bytes());
        address.bytes[2..110].copy_from_slice(path.as_os_str().as_bytes());
        assert_eq!(
            unsafe { libc::bind(raw_receiver.as_raw_fd(), address.bytes.as_ptr().cast(), 110) },
            0
        );
        let actual = unix_socket_path(raw_receiver.as_raw_fd(), 0, -1)
            .unwrap()
            .unwrap();
        let requested = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(actual.bytes.len(), 109);
        assert_eq!(actual.matches(requested.as_bytes_with_nul()), 1);
    }

    #[test]
    fn notifications_reach_only_explicit_private_socket_endpoints() {
        let work = WorkDir::new();
        let path = work.0.join("n");
        let receiver = UnixDatagram::bind(&path).unwrap();
        receiver
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        assert_eq!(
            notify_socket(path.as_os_str().as_bytes(), b"READY=1\nSTATUS=ok"),
            1
        );
        let mut buffer = [0; 32];
        let size = receiver.recv(&mut buffer).unwrap();
        assert_eq!(&buffer[..size], b"READY=1\nSTATUS=ok");

        let name = format!("i333-sd-{}-abstract", std::process::id());
        let address = SocketAddr::from_abstract_name(name.as_bytes()).unwrap();
        let abstract_receiver = UnixDatagram::bind_addr(&address).unwrap();
        abstract_receiver
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        assert_eq!(notify_socket(format!("@{name}").as_bytes(), b""), 1);
        assert_eq!(abstract_receiver.recv(&mut buffer).unwrap(), 0);
        for endpoint in [&b""[..], &b"@"[..], &b"relative"[..], &b"/"[..]] {
            assert_eq!(notify_socket(endpoint, b"READY=1"), -libc::EINVAL);
        }
    }

    #[test]
    fn maximum_abstract_name_preserves_truncated_final_nul() {
        let mut endpoint = vec![b'x'; 108];
        endpoint[0] = b'@';
        let actual = notification_address(&endpoint).unwrap();
        let name = actual.as_abstract_name().unwrap();
        assert_eq!(name.len(), 107);
        assert_eq!(&name[..106], &endpoint[1..107]);
        assert_eq!(name[106], 0);
        assert_eq!(
            notification_address(&vec![b'@'; 109]).unwrap_err(),
            -libc::EINVAL
        );
    }

    #[test]
    fn socket_probes_preserve_family_port_and_listening_rules() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        assert_eq!(
            sd_is_socket(listener.as_raw_fd(), libc::AF_INET, libc::SOCK_STREAM, 1),
            1
        );
        assert_eq!(
            sd_is_socket_inet(listener.as_raw_fd(), 0, libc::SOCK_STREAM, 1, port),
            1
        );
        assert_eq!(
            sd_is_socket_inet(listener.as_raw_fd(), libc::AF_INET6, 0, -1, 0),
            0
        );
        assert_eq!(
            sd_is_socket(listener.as_raw_fd(), 0, libc::SOCK_DGRAM, -1),
            0
        );
        let datagram = UdpSocket::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        assert_eq!(
            sd_is_socket(datagram.as_raw_fd(), libc::AF_INET, libc::SOCK_DGRAM, 0),
            1
        );
        assert_eq!(
            sd_is_socket_inet(-1, libc::AF_UNIX, 0, -1, 0),
            -libc::EINVAL
        );
        assert_eq!(sd_is_socket(-1, 0, 0, -1), -libc::EINVAL);
    }

    #[test]
    fn unix_path_probe_distinguishes_filesystem_abstract_and_unnamed() {
        let work = WorkDir::new();
        let path = work.0.join("s");
        let receiver = UnixListener::bind(&path).unwrap();
        let address = unix_socket_path(receiver.as_raw_fd(), libc::SOCK_STREAM, 1)
            .unwrap()
            .unwrap();
        let pathname = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(address.matches(pathname.as_bytes_with_nul()), 1);
        assert_eq!(address.matches(b""), 0);
        assert_eq!(address.matches(b"\0not-this-name"), 0);
        let unnamed = UnixDatagram::unbound().unwrap();
        assert_eq!(
            unix_socket_path(unnamed.as_raw_fd(), 0, -1)
                .unwrap()
                .unwrap()
                .matches(b""),
            1
        );
        assert_eq!(
            unix_socket_path(receiver.as_raw_fd(), libc::SOCK_DGRAM, -1)
                .unwrap()
                .is_none(),
            true
        );
        // Native callers may supply an explicit length; an oversized length
        // cannot match and must not cause a raw buffer read.
        assert_eq!(
            unsafe {
                sd_is_socket_unix(receiver.as_raw_fd(), 0, -1, pathname.as_ptr(), usize::MAX)
            },
            -libc::EINVAL
        );
    }

    #[test]
    fn fifo_probe_matches_inode_and_does_not_close_descriptor() {
        let work = WorkDir::new();
        let path = work.0.join("f");
        let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(c_path.as_ptr(), 0o600) }, 0);
        let fifo = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(&path)
            .unwrap();
        assert_eq!(is_fifo(fifo.as_raw_fd(), Some(&path)), 1);
        assert_eq!(is_fifo(fifo.as_raw_fd(), Some(&work.0.join("missing"))), 0);
        assert_eq!(is_fifo(fifo.as_raw_fd(), None), 1);
        assert_eq!(mark_cloexec(fifo.as_raw_fd()), Ok(()));
        assert!(fifo.metadata().unwrap().is_file() == false);
        assert_eq!(sd_is_socket(fifo.as_raw_fd(), 0, 0, -1), 0);
    }
}
