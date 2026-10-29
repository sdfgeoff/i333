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
// Mechanical file-by-file Rust translation of src/sd-daemon.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;

#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub sa: sockaddr,
    pub in4: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
    pub storage: sockaddr_storage,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;

pub type in_addr_t = u32;

pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_TYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SO_ACCEPTCONN: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn sd_listen_fds(unset_environment: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut r: ::core::ffi::c_int = 0;
    let mut e: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_ulong = 0;
    e = getenv(c"LISTEN_PID".as_ptr());
    if e.is_null() {
        r = 0 as ::core::ffi::c_int;
    } else {
        *__errno_location() = 0 as ::core::ffi::c_int;
        l = strtoul(e, &raw mut p, 10 as ::core::ffi::c_int);
        if *__errno_location() != 0 as ::core::ffi::c_int {
            r = -*__errno_location();
        } else if p.is_null() || *p as ::core::ffi::c_int != 0 || l <= 0 as ::core::ffi::c_ulong {
            r = -EINVAL;
        } else if getpid() != l as libc::pid_t {
            r = 0 as ::core::ffi::c_int;
        } else {
            e = getenv(c"LISTEN_FDS".as_ptr());
            if e.is_null() {
                r = 0 as ::core::ffi::c_int;
            } else {
                *__errno_location() = 0 as ::core::ffi::c_int;
                l = strtoul(e, &raw mut p, 10 as ::core::ffi::c_int);
                if *__errno_location() != 0 as ::core::ffi::c_int {
                    r = -*__errno_location();
                } else if p.is_null() || *p as ::core::ffi::c_int != 0 {
                    r = -EINVAL;
                } else {
                    let mut fd: ::core::ffi::c_int = SD_LISTEN_FDS_START;
                    loop {
                        if !(fd < SD_LISTEN_FDS_START + l as ::core::ffi::c_int) {
                            current_block = 6669252993407410313;
                            break;
                        }
                        let mut flags: ::core::ffi::c_int = 0;
                        flags = fcntl(fd, F_GETFD);
                        if flags < 0 as ::core::ffi::c_int {
                            r = -*__errno_location();
                            current_block = 15648823136422126542;
                            break;
                        } else {
                            if !(flags & FD_CLOEXEC != 0) {
                                if fcntl(fd, F_SETFD, flags | FD_CLOEXEC) < 0 as ::core::ffi::c_int
                                {
                                    r = -*__errno_location();
                                    current_block = 15648823136422126542;
                                    break;
                                }
                            }
                            fd += 1;
                        }
                    }
                    match current_block {
                        15648823136422126542 => {}
                        _ => {
                            r = l as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    if unset_environment != 0 {
        unsetenv(c"LISTEN_PID".as_ptr());
        unsetenv(c"LISTEN_FDS".as_ptr());
    }
    r
}

pub unsafe fn sd_is_fifo(
    fd: ::core::ffi::c_int,
    path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut st_fd: stat = ::core::mem::zeroed();
    if fd < 0 as ::core::ffi::c_int {
        return -EINVAL;
    }
    memset(
        &raw mut st_fd as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stat>() as usize,
    );
    if fstat(fd, &raw mut st_fd) < 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if !(st_fd.st_mode & __S_IFMT as libc::mode_t == 0o10000 as libc::mode_t) {
        return 0 as ::core::ffi::c_int;
    }
    if !path.is_null() {
        let mut st_path: stat = ::core::mem::zeroed();
        if stat(path, &raw mut st_path) < 0 as ::core::ffi::c_int {
            if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
                return 0 as ::core::ffi::c_int;
            }
            return -*__errno_location();
        }
        return (st_path.st_dev == st_fd.st_dev && st_path.st_ino == st_fd.st_ino)
            as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
unsafe fn sd_is_socket_internal(
    fd: ::core::ffi::c_int,
    type_0: ::core::ffi::c_int,
    listening: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut st_fd: stat = ::core::mem::zeroed();
    if fd < 0 as ::core::ffi::c_int || type_0 < 0 as ::core::ffi::c_int {
        return -EINVAL;
    }
    if fstat(fd, &raw mut st_fd) < 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if !(st_fd.st_mode & __S_IFMT as libc::mode_t == 0o140000 as libc::mode_t) {
        return 0 as ::core::ffi::c_int;
    }
    if type_0 != 0 as ::core::ffi::c_int {
        let mut other_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut l: libc::socklen_t =
            ::core::mem::size_of::<::core::ffi::c_int>() as libc::socklen_t;
        if getsockopt(
            fd,
            SOL_SOCKET,
            SO_TYPE,
            &raw mut other_type as *mut ::core::ffi::c_void,
            &raw mut l,
        ) < 0 as ::core::ffi::c_int
        {
            return -*__errno_location();
        }
        if l as usize != ::core::mem::size_of::<::core::ffi::c_int>() as usize {
            return -EINVAL;
        }
        if other_type != type_0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if listening >= 0 as ::core::ffi::c_int {
        let mut accepting: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut l_0: libc::socklen_t =
            ::core::mem::size_of::<::core::ffi::c_int>() as libc::socklen_t;
        if getsockopt(
            fd,
            SOL_SOCKET,
            SO_ACCEPTCONN,
            &raw mut accepting as *mut ::core::ffi::c_void,
            &raw mut l_0,
        ) < 0 as ::core::ffi::c_int
        {
            return -*__errno_location();
        }
        if l_0 as usize != ::core::mem::size_of::<::core::ffi::c_int>() as usize {
            return -EINVAL;
        }
        if (accepting == 0) as ::core::ffi::c_int != (listening == 0) as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}

pub unsafe fn sd_is_socket(
    fd: ::core::ffi::c_int,
    family: ::core::ffi::c_int,
    type_0: ::core::ffi::c_int,
    listening: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    if family < 0 as ::core::ffi::c_int {
        return -EINVAL;
    }
    r = sd_is_socket_internal(fd, type_0, listening);
    if r <= 0 as ::core::ffi::c_int {
        return r;
    }
    if family > 0 as ::core::ffi::c_int {
        let mut sockaddr: sockaddr_union = sockaddr_union {
            sa: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        };
        let mut l: libc::socklen_t = 0;
        memset(
            &raw mut sockaddr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_union>() as usize,
        );
        l = ::core::mem::size_of::<sockaddr_union>() as libc::socklen_t;
        if getsockname(fd, &raw mut sockaddr.sa, &raw mut l) < 0 as ::core::ffi::c_int {
            return -*__errno_location();
        }
        if (l as usize) < ::core::mem::size_of::<libc::sa_family_t>() as usize {
            return -EINVAL;
        }
        return (sockaddr.sa.sa_family as ::core::ffi::c_int == family) as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}

pub unsafe fn sd_is_socket_inet(
    fd: ::core::ffi::c_int,
    family: ::core::ffi::c_int,
    type_0: ::core::ffi::c_int,
    listening: ::core::ffi::c_int,
    port: u16,
) -> ::core::ffi::c_int {
    let mut sockaddr: sockaddr_union = sockaddr_union {
        sa: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut l: libc::socklen_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    if family != 0 as ::core::ffi::c_int && family != AF_INET && family != AF_INET6 {
        return -EINVAL;
    }
    r = sd_is_socket_internal(fd, type_0, listening);
    if r <= 0 as ::core::ffi::c_int {
        return r;
    }
    memset(
        &raw mut sockaddr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_union>() as usize,
    );
    l = ::core::mem::size_of::<sockaddr_union>() as libc::socklen_t;
    if getsockname(fd, &raw mut sockaddr.sa, &raw mut l) < 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if (l as usize) < ::core::mem::size_of::<libc::sa_family_t>() as usize {
        return -EINVAL;
    }
    if sockaddr.sa.sa_family as ::core::ffi::c_int != AF_INET
        && sockaddr.sa.sa_family as ::core::ffi::c_int != AF_INET6
    {
        return 0 as ::core::ffi::c_int;
    }
    if family > 0 as ::core::ffi::c_int {
        if sockaddr.sa.sa_family as ::core::ffi::c_int != family {
            return 0 as ::core::ffi::c_int;
        }
    }
    if port as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if sockaddr.sa.sa_family as ::core::ffi::c_int == AF_INET {
            if (l as usize) < ::core::mem::size_of::<sockaddr_in>() as usize {
                return -EINVAL;
            }
            return (htons(port) as ::core::ffi::c_int
                == sockaddr.in4.sin_port as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        } else {
            if (l as usize) < ::core::mem::size_of::<sockaddr_in6>() as usize {
                return -EINVAL;
            }
            return (htons(port) as ::core::ffi::c_int
                == sockaddr.in6.sin6_port as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}

pub unsafe fn sd_is_socket_unix(
    fd: ::core::ffi::c_int,
    type_0: ::core::ffi::c_int,
    listening: ::core::ffi::c_int,
    path: *const ::core::ffi::c_char,
    mut length: usize,
) -> ::core::ffi::c_int {
    let mut sockaddr: sockaddr_union = sockaddr_union {
        sa: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut l: libc::socklen_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    r = sd_is_socket_internal(fd, type_0, listening);
    if r <= 0 as ::core::ffi::c_int {
        return r;
    }
    memset(
        &raw mut sockaddr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_union>() as usize,
    );
    l = ::core::mem::size_of::<sockaddr_union>() as libc::socklen_t;
    if getsockname(fd, &raw mut sockaddr.sa, &raw mut l) < 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if (l as usize) < ::core::mem::size_of::<libc::sa_family_t>() as usize {
        return -EINVAL;
    }
    if sockaddr.sa.sa_family as ::core::ffi::c_int != AF_UNIX {
        return 0 as ::core::ffi::c_int;
    }
    if !path.is_null() {
        if length <= 0 as usize {
            length = strlen(path);
        }
        if length <= 0 as usize {
            return (l as ::core::ffi::c_ulong == 2 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        }
        if *path.offset(0 as ::core::ffi::c_int as isize) != 0 {
            return (l as usize >= (2 as usize).wrapping_add(length).wrapping_add(1 as usize)
                && memcmp(
                    path as *const ::core::ffi::c_void,
                    &raw mut sockaddr.un.sun_path as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    length.wrapping_add(1 as usize),
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        } else {
            return (l as usize == (2 as usize).wrapping_add(length)
                && memcmp(
                    path as *const ::core::ffi::c_void,
                    &raw mut sockaddr.un.sun_path as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    length,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}

pub unsafe fn sd_notify(
    unset_environment: ::core::ffi::c_int,
    state: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut r: ::core::ffi::c_int = 0;
    let mut msghdr: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut iovec: iovec = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut sockaddr: sockaddr_union = sockaddr_union {
        sa: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut e: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if state.is_null() {
        r = -EINVAL;
    } else {
        e = getenv(c"NOTIFY_SOCKET".as_ptr());
        if e.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if *e.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '@' as i32
            && *e.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32
            || *e.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            r = -EINVAL;
        } else if strlen(e) > ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize {
            r = -EINVAL;
        } else {
            fd = socket(
                AF_UNIX,
                libc::SOCK_DGRAM as ::core::ffi::c_int | libc::SOCK_CLOEXEC as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if fd < 0 as ::core::ffi::c_int {
                r = -*__errno_location();
            } else {
                memset(
                    &raw mut sockaddr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<sockaddr_union>() as usize,
                );
                sockaddr.sa.sa_family = AF_UNIX as libc::sa_family_t;
                strncpy(
                    &raw mut sockaddr.un.sun_path as *mut ::core::ffi::c_char,
                    e,
                    (::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize)
                        .wrapping_sub(1 as usize),
                );
                if sockaddr.un.sun_path[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == '@' as i32
                {
                    sockaddr.un.sun_path[0 as ::core::ffi::c_int as usize] =
                        0 as ::core::ffi::c_char;
                }
                memset(
                    &raw mut iovec as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<iovec>() as usize,
                );
                iovec.iov_base = state as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                iovec.iov_len = strlen(state);
                memset(
                    &raw mut msghdr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<msghdr>() as usize,
                );
                msghdr.msg_name = &raw mut sockaddr as *mut ::core::ffi::c_void;
                msghdr.msg_namelen = (2 as usize).wrapping_add(strlen(e)) as libc::socklen_t;
                if msghdr.msg_namelen as usize > ::core::mem::size_of::<sockaddr_un>() as usize {
                    msghdr.msg_namelen = ::core::mem::size_of::<sockaddr_un>() as libc::socklen_t;
                }
                msghdr.msg_iov = &raw mut iovec;
                msghdr.msg_iovlen = 1 as usize;
                if sendmsg(
                    fd,
                    &raw mut msghdr,
                    libc::MSG_NOSIGNAL as ::core::ffi::c_int,
                ) < 0 as isize
                {
                    r = -*__errno_location();
                } else {
                    r = 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    if unset_environment != 0 {
        unsetenv(c"NOTIFY_SOCKET".as_ptr());
    }
    if fd >= 0 as ::core::ffi::c_int {
        close(fd);
    }
    r
}

pub unsafe extern "C" fn sd_notifyf(
    unset_environment: ::core::ffi::c_int,
    format: *const ::core::ffi::c_char,
    args: ...
) -> ::core::ffi::c_int {
    let mut ap;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ap = args.clone();
    let mut r: ::core::ffi::c_int = vasprintf(&raw mut p, format, i333_va_list!(ap));
    if r < 0 as ::core::ffi::c_int || p.is_null() {
        return -ENOMEM;
    }
    r = sd_notify(unset_environment, p);
    free(p as *mut ::core::ffi::c_void);
    r
}

pub unsafe fn sd_booted() -> ::core::ffi::c_int {
    let mut a: stat = ::core::mem::zeroed();
    let mut b: stat = ::core::mem::zeroed();
    if lstat(c"/sys/fs/cgroup".as_ptr(), &raw mut a) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if lstat(c"/sys/fs/cgroup/systemd".as_ptr(), &raw mut b) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (a.st_dev != b.st_dev) as ::core::ffi::c_int
}

pub use libc::{
    __errno_location, close, fcntl, free, fstat, getenv, getpid, getsockname, getsockopt, htons,
    iovec, lstat, memcmp, memset, msghdr, sendmsg, sockaddr, sockaddr_in, sockaddr_in6,
    sockaddr_un, socket, stat, strlen, strncpy, strtoul, unsetenv,
};

use i3_common::native_stdio::vasprintf;

pub use libc::sockaddr_storage;
