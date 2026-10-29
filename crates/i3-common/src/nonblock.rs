// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::io;
use std::os::fd::{AsFd, AsRawFd, RawFd};

/// Change a borrowed descriptor's nonblocking mode without changing other flags.
pub fn set_nonblocking<F: AsFd + ?Sized>(fd: &F, enabled: bool) -> io::Result<()> {
    set_raw_nonblocking(fd.as_fd().as_raw_fd(), enabled)
}

fn set_raw_nonblocking(fd: RawFd, enabled: bool) -> io::Result<()> {
    // SAFETY: These fcntl commands operate on integer descriptors and flags;
    // invalid descriptors are reported by the kernel, with no memory access.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
    if flags < 0 {
        return Err(io::Error::last_os_error());
    }
    let updated = if enabled {
        flags | libc::O_NONBLOCK
    } else {
        flags & !libc::O_NONBLOCK
    };
    if updated != flags && unsafe { libc::fcntl(fd, libc::F_SETFL, updated) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Legacy ABI adapters; these retain fatal-error behavior until callers migrate.
mod legacy {
    use super::*;

    pub fn set_nonblock(fd: libc::c_int) {
        if let Err(error) = set_raw_nonblocking(fd, true) {
            // SAFETY: This target currently uses Linux's errno ABI. The format
            // is a static C string; err terminates instead of unwinding over FFI.
            unsafe {
                *libc::__errno_location() = error.raw_os_error().unwrap_or(libc::EIO);
                err(-1, c"Could not set O_NONBLOCK".as_ptr());
            }
        }
    }
}
pub use legacy::set_nonblock;

use crate::native_stdio::err;
