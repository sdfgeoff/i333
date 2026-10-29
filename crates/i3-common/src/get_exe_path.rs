// i3 - an improved tiling window manager
// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).
// Direct translation of libi3/get_exe_path.c; C ABI and allocation ownership are preserved.

use libc::{c_char, c_int, c_void, size_t};
unsafe extern "C" {
    fn smalloc(size: size_t) -> *mut c_void;
    fn srealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn sstrdup(string: *const c_char) -> *mut c_char;
    fn sasprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
}

/// # Safety
/// `argv0` must point to a live NUL-terminated C string. The caller frees the result.
#[no_mangle]
pub unsafe extern "C" fn get_exe_path(argv0: *const c_char) -> *mut c_char {
    // SAFETY: All temporary strings use C allocation helpers. readlink/getcwd
    // writes are bounded by the current allocation, and readlink is NUL-terminated.
    unsafe {
        let mut destpath_size = 1024usize;
        let mut tmp_size = 1024usize;
        let mut destpath = smalloc(destpath_size).cast::<c_char>();
        let mut tmp = smalloc(tmp_size).cast::<c_char>();
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            #[cfg(target_os = "linux")]
            let exepath = c"/proc/self/exe";
            #[cfg(target_os = "freebsd")]
            let exepath = c"/proc/curproc/file";
            let mut linksize;
            loop {
                linksize = libc::readlink(exepath.as_ptr(), destpath, destpath_size);
                if linksize != destpath_size as isize {
                    break;
                }
                destpath_size *= 2;
                destpath = srealloc(destpath.cast(), destpath_size).cast();
            }
            if linksize != -1 {
                *destpath.add(linksize as usize) = 0;
                libc::free(tmp.cast());
                return destpath;
            }
        }
        if *argv0 == b'/' as c_char {
            libc::free(tmp.cast());
            libc::free(destpath.cast());
            return sstrdup(argv0);
        }
        if !libc::strchr(argv0, b'/' as c_int).is_null() {
            let mut retgcwd;
            loop {
                retgcwd = libc::getcwd(tmp, tmp_size);
                if !retgcwd.is_null() || *libc::__errno_location() != libc::ERANGE {
                    break;
                }
                tmp_size *= 2;
                tmp = srealloc(tmp.cast(), tmp_size).cast();
            }
            if !retgcwd.is_null() {
                libc::free(destpath.cast());
                sasprintf(&mut destpath, c"%s/%s".as_ptr(), tmp, argv0);
                libc::free(tmp.cast());
                return destpath;
            }
        }
        let mut path = libc::getenv(c"PATH".as_ptr());
        if path.is_null() {
            while libc::confstr(libc::_CS_PATH, tmp, tmp_size) > tmp_size {
                tmp_size *= 2;
                tmp = srealloc(tmp.cast(), tmp_size).cast();
            }
            sasprintf(&mut path, c":%s".as_ptr(), tmp);
        } else {
            path = sstrdup(path);
        }
        let mut string = path;
        loop {
            let component = libc::strtok(string, c":".as_ptr());
            if component.is_null() {
                break;
            }
            string = core::ptr::null_mut();
            libc::free(destpath.cast());
            sasprintf(&mut destpath, c"%s/%s".as_ptr(), component, argv0);
            if libc::access(destpath, libc::X_OK) == 0 {
                libc::free(path.cast());
                libc::free(tmp.cast());
                return destpath;
            }
        }
        libc::free(destpath.cast());
        libc::free(path.cast());
        libc::free(tmp.cast());
        sstrdup(c"/usr/bin/i3-nagbar".as_ptr())
    }
}
