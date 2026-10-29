#!/usr/bin/env python3
"""Check printf forwarding and independent va_copy cursors on selected Rusts.

    python scripts/test-variadic-compat.py --toolchain nightly --toolchain esp

Compiles only Rust into temporary files. No Cargo, C compiler, X connection or
window manager is used. Strings, floating arguments and spilled integer args
exercise the same printf ABI used by i3's logging and allocation helpers.
"""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SOURCE = r'''
#![feature(c_variadic)]
#![allow(dead_code, unused_mut, unused_assignments, non_camel_case_types,
         non_upper_case_globals, stable_features)]
#[path = SAFETYWRAPPERS_PATH]
mod safewrappers;
include!(VARIADIC_COMPAT_PATH);
extern "C" {
    fn vasprintf(out: *mut *mut core::ffi::c_char, format: *const core::ffi::c_char,
                 arguments: core::ffi::VaList) -> core::ffi::c_int;
    fn free(ptr: *mut core::ffi::c_void);
}

unsafe extern "C" fn copy_twice(format: *const core::ffi::c_char, mut args: ...) {
    let mut cursor = args.clone();
    let mut first = core::ptr::null_mut();
    let count = vasprintf(&mut first, format, i333_va_list!(cursor));
    assert!(count > 0);
    // Reinitialize from the original va_start cursor, exactly as errorlog does
    // for stdout and its separate error file. Both outputs must contain all args.
    cursor = args.clone();
    let mut second = core::ptr::null_mut();
    assert_eq!(vasprintf(&mut second, format, i333_va_list!(cursor)), count);
    let expected = b"1:2:3:4:5:6:7:8:hello:3.25";
    assert_eq!(std::ffi::CStr::from_ptr(first).to_bytes(), expected);
    assert_eq!(std::ffi::CStr::from_ptr(second).to_bytes(), expected);
    free(first.cast());
    free(second.cast());
}

fn main() {
    // SAFETY: Formats match the promoted C argument types; buffers are allocated
    // by vasprintf and released once with libc free after string inspection.
    unsafe {
        let format = c"%d:%d:%d:%d:%d:%d:%d:%d:%s:%.2f";
        let mut value = core::ptr::null_mut();
        let count = safewrappers::sasprintf(&mut value, format.as_ptr(),
            1, 2, 3, 4, 5, 6, 7, 8, c"hello".as_ptr(), 3.25f64);
        let expected = b"1:2:3:4:5:6:7:8:hello:3.25";
        assert_eq!(count as usize, expected.len());
        assert_eq!(std::ffi::CStr::from_ptr(value).to_bytes(), expected);
        free(value.cast());
        copy_twice(format.as_ptr(), 1, 2, 3, 4, 5, 6, 7, 8,
                   c"hello".as_ptr(), 3.25f64);
    }
    println!("variadic formatting and independent cursor checks passed");
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--toolchain', action='append', default=[])
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='i333-variadic-compat-') as work:
        work = Path(work)
        source = work / 'check.rs'
        source.write_text(SOURCE.replace('SAFETYWRAPPERS_PATH', json.dumps(
            str(ROOT / 'libi3/safewrappers.rs'))).replace('VARIADIC_COMPAT_PATH',
            json.dumps(str(ROOT / 'rust/variadic-compat.rs'))))
        for toolchain in args.toolchain or [None]:
            rustc = ['rustc'] + ([f'+{toolchain}'] if toolchain else [])
            version = subprocess.check_output([*rustc, '--version'], text=True)
            minor = int(re.match(r'rustc 1\.(\d+)', version)[1])
            command = [*rustc, '--edition=2021', '--check-cfg=cfg(i333_new_va_list)',
                       str(source), '-o', str(work / 'check')]
            if minor >= 99:
                command.extend(['--cfg', 'i333_new_va_list'])
            subprocess.run(command, check=True)
            subprocess.run([str(work / 'check')], check=True)
            print(version.strip(), flush=True)


if __name__ == '__main__':
    main()
