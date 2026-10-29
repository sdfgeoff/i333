/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 * Logging and the shared-memory wire protocol consumed by i3-dump-log.
 */
use std::ffi::{CStr, CString};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::ptr::NonNull;

use i3_common::get_process_filename::get_process_filename;
use i3_common::i333_va_list;
use i3_common::native_stdio::{stderr, stdout, vfprintf, vprintf, vsnprintf};
use i3_common::nonblock::set_nonblock;
use i3_common::safewrappers::writeall;
use libc::{
    __errno_location, atexit, fcntl, fflush, fileno, fopen, fprintf, fwrite, localtime_r, perror,
    strerror, strftime, sysconf, time, tm, FD_CLOEXEC, FILE, F_SETFD,
};
pub use libev_sys::{ev_io, ev_loop};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct i3_shmlog_header {
    pub offset_next_write: u32,
    pub offset_last_wrap: u32,
    pub size: u32,
    pub wrap_count: u32,
}
const HEADER_SIZE: usize = size_of::<i3_shmlog_header>();
const MESSAGE_CAPACITY: usize = 4096;

/// A checked operation on numeric offsets; it never borrows shared memory.
#[derive(Debug, PartialEq, Eq)]
struct RingWritePlan {
    start: usize,
    len: usize,
    wrapped: bool,
    next: i3_shmlog_header,
}
impl RingWritePlan {
    fn new(old: i3_shmlog_header, message_len: usize) -> Option<Self> {
        let size = old.size as usize;
        let valid_offset = |offset: u32| (HEADER_SIZE..=size).contains(&(offset as usize));
        if size <= HEADER_SIZE
            || !valid_offset(old.offset_next_write)
            || !valid_offset(old.offset_last_wrap)
        {
            return None;
        }
        let len = message_len.min(size - HEADER_SIZE);
        let wrapped = len >= size - old.offset_next_write as usize;
        let start = if wrapped {
            HEADER_SIZE
        } else {
            old.offset_next_write as usize
        };
        Some(Self {
            start,
            len,
            wrapped,
            next: i3_shmlog_header {
                offset_next_write: (start + len) as u32,
                offset_last_wrap: if wrapped {
                    old.offset_next_write
                } else {
                    old.offset_last_wrap
                },
                size: old.size,
                wrap_count: old.wrap_count.wrapping_add(u32::from(wrapped)),
            },
        })
    }
    fn record(&self, message: &[u8]) -> Vec<u8> {
        let mut record = message[..self.len].to_vec();
        // Formerly an out-of-bounds write when a configured ring was too small.
        if message.len() > self.len {
            if self.len >= 2 {
                record[self.len - 2] = b'\n';
            }
            if self.len > 0 {
                record[self.len - 1] = 0;
            }
        }
        // Preserve strncpy's padding, including embedded NULs in printf output.
        if let Some(nul) = record.iter().position(|byte| *byte == 0) {
            record[nul..].fill(0);
        }
        record
    }
}

/// The exported name remains malloc-compatible for the legacy application API.
struct ShmFile {
    fd: OwnedFd,
    name: NonNull<libc::c_char>,
}
impl Drop for ShmFile {
    fn drop(&mut self) {
        unsafe {
            libc::shm_unlink(self.name.as_ptr());
            libc::free(self.name.as_ptr().cast());
        }
    }
}
struct MappedLog {
    file: ShmFile,
    base: NonNull<u8>,
    cursor: i3_shmlog_header,
}
impl MappedLog {
    /// The caller must prevent truncation of this SHM object while mapped and
    /// serialize logger mutations on the event thread. Other processes may read
    /// its bytes: no Rust reference to the mapping is ever constructed.
    unsafe fn open(name: &CStr, size: usize) -> Result<Self, (&'static CStr, std::io::Error)> {
        let failure = |stage| (stage, std::io::Error::last_os_error());
        if size <= HEADER_SIZE || size > u32::MAX as usize {
            return Err((
                c"size SHM segment",
                std::io::Error::from_raw_os_error(libc::EINVAL),
            ));
        }
        let name = NonNull::new(unsafe { libc::strdup(name.as_ptr()) })
            .ok_or_else(|| failure(c"allocate SHM name"))?;
        let fd = unsafe { libc::shm_open(name.as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o600) };
        if fd < 0 {
            let error = failure(c"shm_open SHM segment");
            unsafe {
                libc::free(name.as_ptr().cast());
            }
            return Err(error);
        }
        let file = ShmFile {
            fd: unsafe { OwnedFd::from_raw_fd(fd) },
            name,
        };
        let ret = unsafe { libc::posix_fallocate(fd, 0, size as libc::off_t) };
        if ret != 0 {
            return Err((
                c"ftruncate SHM segment",
                std::io::Error::from_raw_os_error(ret),
            ));
        }
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            )
        };
        if ptr == libc::MAP_FAILED {
            return Err(failure(c"mmap SHM segment"));
        }
        let Some(base) = NonNull::new(ptr.cast::<u8>()) else {
            unsafe {
                libc::munmap(ptr, size);
            }
            return Err((
                c"mmap SHM segment",
                std::io::Error::from_raw_os_error(libc::ENOMEM),
            ));
        };
        let log = Self {
            file,
            base,
            cursor: i3_shmlog_header {
                offset_next_write: HEADER_SIZE as u32,
                offset_last_wrap: size as u32,
                size: size as u32,
                wrap_count: 0,
            },
        };
        unsafe {
            std::ptr::write_bytes(base.as_ptr(), 0, size);
            log.publish(log.cursor);
        }
        Ok(log)
    }
    unsafe fn publish(&self, cursor: i3_shmlog_header) {
        let header = self.base.as_ptr().cast::<i3_shmlog_header>();
        // Raw field stores retain the wire layout without claiming exclusivity
        // over memory read by dump-log in another process.
        unsafe {
            (&raw mut (*header).offset_next_write).write_volatile(cursor.offset_next_write);
            (&raw mut (*header).offset_last_wrap).write_volatile(cursor.offset_last_wrap);
            (&raw mut (*header).size).write_volatile(cursor.size);
            (&raw mut (*header).wrap_count).write_volatile(cursor.wrap_count);
        }
    }
    /// Requires the same mapping lifetime and event-thread contract as open.
    unsafe fn write(&mut self, message: &[u8]) {
        let plan = RingWritePlan::new(self.cursor, message.len())
            .expect("owned ring cursor remains valid");
        let record = plan.record(message);
        unsafe {
            if plan.wrapped {
                self.publish(i3_shmlog_header {
                    offset_next_write: HEADER_SIZE as u32,
                    offset_last_wrap: self.cursor.offset_next_write,
                    ..self.cursor
                });
                let header = self.base.as_ptr().cast::<i3_shmlog_header>();
                (&raw mut (*header).wrap_count).write_volatile(plan.next.wrap_count);
            }
            std::ptr::copy_nonoverlapping(
                record.as_ptr(),
                self.base.as_ptr().add(plan.start),
                record.len(),
            );
            self.publish(plan.next);
        }
        self.cursor = plan.next;
    }
}
impl Drop for MappedLog {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.base.as_ptr().cast(), self.cursor.size as usize);
        }
    }
}

// These globals are accessed only on i3's event thread. Native variadic entry
// points additionally require valid printf arguments. Logging is not promised
// async-signal-safe; no mutex or long-lived global mutable borrow is introduced.
static mut LOG_BUFFER: Option<MappedLog> = None;
static mut CLIENTS: Vec<OwnedFd> = Vec::new();
static mut debug_logging: bool = false;
static mut verbose: bool = false;
static mut errorfile: *mut FILE = std::ptr::null_mut();
pub static mut errorfilename: *mut libc::c_char = std::ptr::null_mut();
pub static mut shmlogname: *mut libc::c_char = c"".as_ptr().cast_mut();
pub static mut shmlog_size: libc::c_int = 0;
static mut physical_mem_bytes: libc::c_longlong = 0;
pub static mut current_log_stream_socket_path: *mut libc::c_char = std::ptr::null_mut();
unsafe fn logbuffer_is_closed() -> bool {
    unsafe { (*(&raw const LOG_BUFFER)).is_none() }
}

pub unsafe fn init_logging() {
    if errorfilename.is_null() {
        errorfilename = get_process_filename(c"errorlog".as_ptr());
        if errorfilename.is_null() {
            fprintf(stderr, c"Could not initialize errorlog\n".as_ptr());
        } else {
            errorfile = fopen(errorfilename, c"w".as_ptr()) as *mut FILE;
            if errorfile.is_null() {
                fprintf(
                    stderr,
                    c"Could not initialize errorlog on %s: %s\n".as_ptr(),
                    errorfilename,
                    strerror(*__errno_location()),
                );
            } else if fcntl(fileno(errorfile), F_SETFD, FD_CLOEXEC) != 0 {
                fprintf(stderr, c"Could not set close-on-exec flag\n".as_ptr());
            }
        }
    }
    if physical_mem_bytes == 0 as ::core::ffi::c_longlong {
        physical_mem_bytes = sysconf(libc::_SC_PHYS_PAGES as ::core::ffi::c_int)
            as ::core::ffi::c_longlong
            * sysconf(libc::_SC_PAGESIZE as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    if shmlog_size > 0 as ::core::ffi::c_int && logbuffer_is_closed() {
        open_logbuffer();
    } else if shmlog_size <= 0 as ::core::ffi::c_int && !logbuffer_is_closed() {
        close_logbuffer();
    }
    atexit(purge_zerobyte_logfile);
}

/// Event-thread only; replaces any previous owned mapping and compatibility name.
pub unsafe fn open_logbuffer() {
    let size = if physical_mem_bytes as f64 * 0.01 < shmlog_size as f64 {
        (physical_mem_bytes as f64 * 0.01) as libc::c_int
    } else {
        shmlog_size
    };
    close_logbuffer();
    let name = CString::new(format!("/i3-log-{}", std::process::id())).expect("numeric SHM name");
    match MappedLog::open(&name, size.max(0) as usize) {
        Ok(log) => {
            shmlogname = log.file.name.as_ptr();
            (&raw mut LOG_BUFFER).write(Some(log));
        }
        Err((stage, error)) => {
            fprintf(
                stderr,
                c"Could not %s for the i3 log: %s\n".as_ptr(),
                stage.as_ptr(),
                strerror(error.raw_os_error().unwrap_or(libc::EINVAL)),
            );
        }
    }
}

pub unsafe fn close_logbuffer() {
    shmlogname = c"".as_ptr().cast_mut();
    drop((&raw mut LOG_BUFFER).replace(None));
}
pub unsafe fn set_verbosity(value: bool) {
    verbose = value;
}
pub unsafe fn get_debug_logging() -> bool {
    debug_logging
}
pub unsafe fn set_debug_logging(value: bool) {
    debug_logging = value;
}

fn timestamp() -> Vec<u8> {
    let mut buffer = [0u8; MESSAGE_CAPACITY];
    let mut result = std::mem::MaybeUninit::<tm>::uninit();
    let len = unsafe {
        let now = time(std::ptr::null_mut());
        if localtime_r(&now, result.as_mut_ptr()).is_null() {
            return Vec::new();
        }
        strftime(
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            c"%x %X - ".as_ptr(),
            result.as_ptr(),
        )
    };
    buffer[..len].to_vec()
}

/// fmt and args must satisfy printf's native variadic contract. The prefix
/// comes from timestamp, which cannot exceed this record's fixed capacity.
unsafe fn format_record(
    prefix: &[u8],
    fmt: *const libc::c_char,
    mut args: std::ffi::VaList,
) -> (Vec<u8>, bool) {
    assert!(prefix.len() < MESSAGE_CAPACITY);
    let mut message = vec![0u8; MESSAGE_CAPACITY];
    message[..prefix.len()].copy_from_slice(prefix);
    let needed = unsafe {
        vsnprintf(
            message.as_mut_ptr().add(prefix.len()).cast(),
            message.len() - prefix.len(),
            fmt,
            i333_va_list!(args),
        )
    };
    let len = prefix.len().wrapping_add(needed as usize);
    let truncated = len >= message.len();
    if truncated {
        message[MESSAGE_CAPACITY - 2] = b'\n';
    }
    message.truncate(len.min(MESSAGE_CAPACITY));
    (message, truncated)
}

unsafe fn vlog(print: bool, fmt: *const libc::c_char, mut args: std::ffi::VaList) {
    let prefix = timestamp();
    if logbuffer_is_closed() {
        fwrite(prefix.as_ptr().cast(), prefix.len(), 1, stdout);
        vprintf(fmt, i333_va_list!(args));
        return;
    }
    let (message, truncated) = format_record(&prefix, fmt, i333_va_list!(args));
    if truncated {
        fprintf(stderr, c"BUG: single log message > 4k\n".as_ptr());
    }
    let message = message.as_slice();
    // Release the global slot before native output. The owned mapping itself
    // contains no references to shared bytes.
    let mut log = (&raw mut LOG_BUFFER).replace(None);
    if let Some(log) = log.as_mut() {
        log.write(message);
    }
    (&raw mut LOG_BUFFER).write(log);
    if print {
        fwrite(message.as_ptr().cast(), message.len(), 1, stdout);
    }
    broadcast(message);
}

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    if logbuffer_is_closed() && !verbose {
        return;
    }
    args_0 = args.clone();
    vlog(verbose, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vlog(true, fmt, i333_va_list!(args_0));
    if errorfile.is_null() {
        return;
    }
    args_0 = args.clone();
    vfprintf(errorfile, fmt, i333_va_list!(args_0));
    fflush(errorfile);
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    if logbuffer_is_closed() && !debug_logging {
        return;
    }
    args_0 = args.clone();
    vlog(debug_logging, fmt, i333_va_list!(args_0));
}

pub extern "C" fn purge_zerobyte_logfile() {
    // atexit runs after event-thread logging has stopped; copy the legacy name
    // before filesystem operations rather than modifying its parent separator.
    let path = unsafe {
        if errorfilename.is_null() {
            return;
        }
        std::path::PathBuf::from(std::ffi::OsStr::from_bytes(
            CStr::from_ptr(errorfilename).to_bytes(),
        ))
    };
    if std::fs::metadata(&path).is_ok_and(|metadata| metadata.len() == 0)
        && std::fs::remove_file(&path).is_ok()
    {
        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
}

/// The watcher must be live and event-thread serialized with logging callbacks.
pub unsafe extern "C" fn log_new_client(_loop: *mut ev_loop, w: *mut ev_io, _events: libc::c_int) {
    let fd = libc::accept((*w).fd, std::ptr::null_mut(), std::ptr::null_mut());
    if fd < 0 {
        if *__errno_location() != libc::EINTR {
            perror(c"accept()".as_ptr());
        }
        return;
    }
    let client = OwnedFd::from_raw_fd(fd);
    fcntl(fd, F_SETFD, FD_CLOEXEC);
    set_nonblock(fd);
    (*(&raw mut CLIENTS)).push(client);
    debuglog(
        c"%s:%s:%d - log: new client connected on fd %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/log.c".as_ptr(),
        c"log_new_client".as_ptr(),
        419 as libc::c_int,
        fd,
    );
}
unsafe fn broadcast(message: &[u8]) {
    let mut clients = (&raw mut CLIENTS).replace(Vec::new());
    clients
        .retain(|client| writeall(client.as_raw_fd(), message.as_ptr().cast(), message.len()) >= 0);
    clients.append(&mut *(&raw mut CLIENTS));
    (&raw mut CLIENTS).write(clients);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::FileExt;

    fn cursor(size: u32, next: u32) -> i3_shmlog_header {
        i3_shmlog_header {
            offset_next_write: next,
            offset_last_wrap: size,
            size,
            wrap_count: 0,
        }
    }

    #[test]
    fn ring_wraps_on_exact_fit_and_wrap_counter_rolls_over() {
        assert_eq!(HEADER_SIZE, 16);
        let plan = RingWritePlan::new(cursor(64, 20), 43).unwrap();
        assert_eq!(
            (plan.start, plan.next.offset_next_write, plan.wrapped),
            (20, 63, false)
        );
        let plan = RingWritePlan::new(cursor(64, 20), 44).unwrap();
        assert_eq!(
            (
                plan.start,
                plan.next.offset_next_write,
                plan.next.offset_last_wrap
            ),
            (16, 60, 20)
        );
        assert_eq!(plan.next.wrap_count, 1);
        let plan = RingWritePlan::new(
            i3_shmlog_header {
                wrap_count: u32::MAX,
                ..cursor(64, 64)
            },
            1,
        )
        .unwrap();
        assert_eq!(plan.next.wrap_count, 0);
        assert_eq!(plan.next.offset_last_wrap, 64);
    }

    #[test]
    fn ring_rejects_bad_headers_and_bounds_even_huge_messages() {
        for size in 0..=16 {
            assert!(RingWritePlan::new(cursor(size, 16), 1).is_none());
        }
        for next in [0, 15, 65, u32::MAX] {
            assert!(RingWritePlan::new(cursor(64, next), 1).is_none());
        }
        assert!(RingWritePlan::new(
            i3_shmlog_header {
                offset_last_wrap: 65,
                ..cursor(64, 16)
            },
            1
        )
        .is_none());
        let plan = RingWritePlan::new(cursor(17, 16), usize::MAX).unwrap();
        assert_eq!(
            (plan.start, plan.len, plan.next.offset_next_write),
            (16, 1, 17)
        );
        assert_eq!(plan.record(b"too big"), b"\0");
    }

    #[test]
    fn ring_preserves_strncpy_padding_and_bounds_small_records() {
        let plan = RingWritePlan::new(cursor(64, 16), 7).unwrap();
        assert_eq!(plan.record(b"ab\0cdef"), b"ab\0\0\0\0\0");
        let plan = RingWritePlan::new(cursor(20, 16), 7).unwrap();
        assert_eq!(plan.record(b"abcdefg"), b"ab\n\0");
        let mut legacy_truncation = vec![b'x'; MESSAGE_CAPACITY];
        legacy_truncation[MESSAGE_CAPACITY - 2] = b'\n';
        legacy_truncation[MESSAGE_CAPACITY - 1] = 0;
        let plan = RingWritePlan::new(cursor(8192, 16), MESSAGE_CAPACITY).unwrap();
        assert_eq!(plan.record(&legacy_truncation), legacy_truncation);
    }

    #[test]
    fn private_shm_is_readable_as_dump_log_wire_bytes_and_unlinked_on_drop() {
        let name = CString::new(format!(
            "/i333-test-log-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
        .unwrap();
        // This test exclusively owns the fresh name and never truncates it.
        let mut log = unsafe { MappedLog::open(&name, 64) }.unwrap();
        let reader = std::fs::File::from(log.file.fd.try_clone().unwrap());
        let snapshot = || {
            let mut bytes = vec![0; 64];
            reader.read_exact_at(&mut bytes, 0).unwrap();
            let word = |offset| u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap());
            let header = [word(0), word(4), word(8), word(12)];
            (header, bytes)
        };
        assert_eq!(snapshot().0, [16, 64, 64, 0]);
        unsafe {
            log.write(b"first\n");
        }
        assert_eq!(snapshot().0, [22, 64, 64, 0]);
        unsafe {
            log.write(&[b'y'; 42]);
        } // exact-fit wrap, leaving the old marker
        let (header, bytes) = snapshot();
        assert_eq!(header, [58, 22, 64, 1]);
        assert_eq!(&bytes[16..58], &[b'y'; 42]);
        unsafe {
            log.write(&[b'z'; 100]);
        } // oversized payload stays inside the mapping
        let (header, bytes) = snapshot();
        assert_eq!(header, [64, 58, 64, 2]);
        assert_eq!(&bytes[62..64], b"\n\0");
        drop(log);
        // The duplicated reader still owns the file, but the public name is gone.
        let fd = unsafe { libc::shm_open(name.as_ptr(), libc::O_RDONLY, 0) };
        if fd >= 0 {
            unsafe {
                libc::close(fd);
            }
            panic!("dropped SHM name still exists");
        }
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ENOENT)
        );
        assert_eq!(snapshot().0, [64, 58, 64, 2]);
    }

    unsafe extern "C" fn render(output: *mut (Vec<u8>, bool), fmt: *const libc::c_char, args: ...) {
        let mut args = args.clone();
        unsafe {
            *output = format_record(b"T - ", fmt, i333_va_list!(args));
        }
    }

    #[test]
    fn native_printf_records_keep_embedded_nuls_and_legacy_4k_truncation() {
        let mut result = (Vec::new(), false);
        unsafe {
            render(
                &mut result,
                c"%s%cend".as_ptr(),
                c"text".as_ptr(),
                0 as libc::c_int,
            );
        }
        assert_eq!(result, (b"T - text\0end".to_vec(), false));
        let long = CString::new(vec![b'x'; 5000]).unwrap();
        unsafe {
            render(&mut result, c"%s".as_ptr(), long.as_ptr());
        }
        assert!(result.1);
        assert_eq!(result.0.len(), MESSAGE_CAPACITY);
        assert_eq!(&result.0[MESSAGE_CAPACITY - 2..], b"\n\0");
        let limit = CString::new(vec![b'y'; MESSAGE_CAPACITY - 5]).unwrap();
        unsafe {
            render(&mut result, c"%s".as_ptr(), limit.as_ptr());
        }
        assert_eq!(result.0.len(), MESSAGE_CAPACITY - 1);
        assert!(!result.1);
        assert_eq!(result.0.last(), Some(&b'y'));
    }

    #[test]
    fn tiny_shm_rejected_before_any_object_is_created() {
        assert_eq!(
            unsafe { MappedLog::open(c"/i333-invalid-test-size", HEADER_SIZE) }
                .err()
                .unwrap()
                .1
                .raw_os_error(),
            Some(libc::EINVAL)
        );
    }
}
