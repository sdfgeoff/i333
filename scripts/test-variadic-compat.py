#!/usr/bin/env python3
"""Check printf forwarding and independent va_copy cursors on selected Rusts.

    python scripts/test-variadic-compat.py --toolchain nightly --toolchain esp

Compiles a temporary Cargo probe using only Rust and the cached libc crate.
No C compiler, X connection or window manager is used. Strings, floating
arguments and spilled integer args exercise the same printf ABI used by i3's logging and allocation helpers.
"""
import argparse
import json
from pathlib import Path
import re
import subprocess
from workspace_temp import temporary_directory
import tomllib

ROOT = Path(__file__).resolve().parents[1]
SOURCE = r'''
#![feature(c_variadic)]
#![allow(dead_code, unused_mut, unused_assignments, non_camel_case_types,
         non_upper_case_globals, stable_features)]
// Include the actual forwarding modules without building graphical dependencies.
extern crate self as i3_common;
#[path = VARIADIC_COMPAT_PATH]
mod variadic_compat;
#[path = SAFETYWRAPPERS_PATH]
mod safewrappers;
#[path = NATIVE_STDIO_PATH]
mod native_stdio;
use native_stdio::vasprintf;
use libc::free;

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
    with temporary_directory(prefix='i333-variadic-compat-') as work:
        work = Path(work)
        source = work / 'check.rs'
        libc_version = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['dependencies']['libc']
        (work / 'Cargo.toml').write_text(
            '[package]\nname = "i333-variadic-probe"\nversion = "0.0.0"\nedition = "2021"\n'
            '[workspace]\n[dependencies]\nlibc = ' + json.dumps(libc_version) + '\n'
            '[[bin]]\nname = "check"\npath = "check.rs"\n')
        source.write_text(SOURCE.replace('SAFETYWRAPPERS_PATH', json.dumps(
            str(ROOT / 'crates/i3-common/src/safewrappers.rs'))).replace('VARIADIC_COMPAT_PATH',
            json.dumps(str(ROOT / 'crates/i3-common/src/variadic_compat.rs'))).replace(
            'NATIVE_STDIO_PATH', json.dumps(str(ROOT / 'crates/i3-common/src/native_stdio.rs'))))
        for toolchain in args.toolchain or [None]:
            rustc = ['rustc'] + ([f'+{toolchain}'] if toolchain else [])
            version = subprocess.check_output([*rustc, '--version'], text=True)
            minor = int(re.match(r'rustc 1\.(\d+)', version)[1])
            cargo = ['cargo'] + ([f'+{toolchain}'] if toolchain else [])
            target = work / ('target-' + (toolchain or 'default'))
            command = [*cargo, 'rustc', '--offline', '--manifest-path',
                       str(work / 'Cargo.toml'), '--target-dir', str(target),
                       '--bin', 'check', '--', '--check-cfg=cfg(i333_new_va_list)']
            if minor >= 99:
                command.extend(['--cfg', 'i333_new_va_list'])
            subprocess.run(command, check=True)
            subprocess.run([str(target / 'debug/check')], check=True)
            print(version.strip(), flush=True)


if __name__ == '__main__':
    main()
