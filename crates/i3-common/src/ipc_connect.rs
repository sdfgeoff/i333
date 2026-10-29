// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/ipc_connect.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, c_void};
unsafe extern "C" {
    fn sstrdup(string: *const c_char) -> *mut c_char;
    fn root_atom_contents(
        atomname: *const c_char,
        provided_conn: *mut c_void,
        screen: c_int,
    ) -> *mut c_char;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
}

/// # Safety
/// `socket_path` is null or a live NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn ipc_connect(socket_path: *const c_char) -> c_int {
    // SAFETY: C helpers return malloc-owned strings; path is freed after connect.
    unsafe {
        let mut path = core::ptr::null_mut();
        if !socket_path.is_null() {
            path = sstrdup(socket_path);
        }
        if path.is_null() {
            path = libc::getenv(c"I3SOCK".as_ptr());
            if !path.is_null() {
                path = sstrdup(path);
            }
        }
        if path.is_null() {
            path = root_atom_contents(c"I3_SOCKET_PATH".as_ptr(), core::ptr::null_mut(), 0);
        }
        if path.is_null() {
            err(
                libc::EXIT_FAILURE,
                c"Could not determine i3 socket path".as_ptr(),
            );
        }
        let sockfd = ipc_connect_impl(path);
        if sockfd < 0 {
            err(
                libc::EXIT_FAILURE,
                c"Could not connect to i3 on socket %s".as_ptr(),
                path,
            );
        }
        libc::free(path.cast());
        sockfd
    }
}

/// # Safety
/// `socket_path` must point to a live NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn ipc_connect_impl(socket_path: *const c_char) -> c_int {
    // SAFETY: sockaddr_un is initialized completely, including its terminator;
    // strncpy bounds match upstream. The kernel borrows the address during connect.
    unsafe {
        let sockfd = libc::socket(libc::AF_LOCAL, libc::SOCK_STREAM, 0);
        if sockfd == -1 {
            err(libc::EXIT_FAILURE, c"Could not create socket".as_ptr());
        }
        libc::fcntl(sockfd, libc::F_SETFD, libc::FD_CLOEXEC);
        let mut addr: libc::sockaddr_un = core::mem::zeroed();
        addr.sun_family = libc::AF_LOCAL as libc::sa_family_t;
        libc::strncpy(
            addr.sun_path.as_mut_ptr(),
            socket_path,
            addr.sun_path.len() - 1,
        );
        if libc::connect(
            sockfd,
            (&addr as *const libc::sockaddr_un).cast(),
            core::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ) < 0
        {
            libc::close(sockfd);
            return -1;
        }
        sockfd
    }
}
