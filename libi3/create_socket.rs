// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/create_socket.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, mode_t};
unsafe extern "C" {
    fn resolve_tilde(path: *const c_char) -> *mut c_char;
    fn sstrdup(string: *const c_char) -> *mut c_char;
    fn path_exists(path: *const c_char) -> bool;
    fn mkdirp(path: *const c_char, mode: mode_t) -> c_int;
    fn ipc_connect_impl(path: *const c_char) -> c_int;
    fn set_nonblock(fd: c_int);
    fn errorlog(fmt: *const c_char, ...);
    fn debuglog(fmt: *const c_char, ...);
}

/// # Safety
/// `filename` is a live C string. `out_socketpath` points to a writable null or
/// malloc-owned string pointer; its replacement is malloc-owned by the caller.
#[no_mangle]
pub unsafe extern "C" fn create_socket(
    filename: *const c_char,
    out_socketpath: *mut *mut c_char,
) -> c_int {
    // SAFETY: C ABI contracts preserve ownership and address sizes. The kernel
    // borrows sockaddr_un only for bind and pointers are freed as in upstream.
    unsafe {
        let resolved = resolve_tilde(filename);
        debuglog(
            c"%s:%s:%d - Creating UNIX socket at %s\n".as_ptr(),
            c"libi3/create_socket.c".as_ptr(),
            c"create_socket".as_ptr(),
            30,
            resolved,
        );
        let copy = sstrdup(resolved);
        let dir = libc::dirname(copy);
        if !path_exists(dir) {
            mkdirp(dir, 0o755);
        }
        libc::free(copy.cast());
        let mut sockfd = ipc_connect_impl(resolved);
        if sockfd != -1 {
            errorlog(
                c"[libi3] ERROR: Refusing to create UNIX socket at %s: Socket is already in use\n"
                    .as_ptr(),
                resolved,
            );
            libc::close(sockfd);
            *libc::__errno_location() = libc::EEXIST;
            return -1;
        }
        libc::unlink(resolved);
        sockfd = libc::socket(libc::AF_LOCAL, libc::SOCK_STREAM, 0);
        if sockfd < 0 {
            libc::perror(c"socket()".as_ptr());
            libc::free(resolved.cast());
            return -1;
        }
        libc::fcntl(sockfd, libc::F_SETFD, libc::FD_CLOEXEC);
        let mut addr: libc::sockaddr_un = core::mem::zeroed();
        addr.sun_family = libc::AF_LOCAL as libc::sa_family_t;
        libc::strncpy(
            addr.sun_path.as_mut_ptr(),
            resolved,
            addr.sun_path.len() - 1,
        );
        if libc::bind(
            sockfd,
            (&addr as *const libc::sockaddr_un).cast(),
            core::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
        ) < 0
        {
            libc::perror(c"bind()".as_ptr());
            libc::free(resolved.cast());
            return -1;
        }
        set_nonblock(sockfd);
        if libc::listen(sockfd, 5) < 0 {
            libc::perror(c"listen()".as_ptr());
            libc::free(resolved.cast());
            return -1;
        }
        libc::free((*out_socketpath).cast());
        *out_socketpath = resolved;
        sockfd
    }
}
