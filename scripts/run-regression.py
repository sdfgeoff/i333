#!/usr/bin/env python3
"""Cargo's offline regression entry point; setup is a separate explicit command."""
import argparse
import os
from pathlib import Path
import subprocess
import sys

from regression_environment import (ROOT, MissingTestDependency, find_reference,
    find_xserver, find_perl_lib, find_python, native_fixture_tools, PRODUCTS, setup_error)
from workspace_temp import temporary_root


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--suite', required=True, choices=('ipc', 'cli', 'headless', 'input', 'upstream'))
    parser.add_argument('--bin-dir', required=True, type=Path,
                        help='Directory of regular executables supplied by Cargo integration tests')
    args = parser.parse_args()
    directory = args.bin_dir.resolve()
    for name in PRODUCTS:
        if not (directory / name).is_file() or not os.access(directory / name, os.X_OK):
            raise setup_error(f'Cargo product is missing: {directory / name}. '
                              'Run cargo test from the workspace root.')
    reference = find_reference()
    interpreter = Path(sys.executable)
    flags = []
    if args.suite == 'ipc':
        script = 'test-ipc-parity.py'
        flags = ['--rust-bin-dir', str(directory), '--reference', str(reference / 'i3-msg')]
    elif args.suite == 'cli':
        script = 'test-main-cli.py'
        flags = ['--candidate-dir', str(directory), '--reference-dir', str(reference)]
    elif args.suite in ('headless', 'input'):
        interpreter = find_python()
        xvfb = find_xserver('Xvfb')
        if args.suite == 'headless':
            script = 'test-headless-parity.py'
            flags = ['--rust-bin-dir', str(directory), '--reference', str(reference / 'i3')]
        else:
            script = 'test-input-parity.py'
            flags = ['--rust-bin-dir', str(directory), '--reference', str(reference / 'i3-input'), '--extended']
        flags += ['--xvfb', str(xvfb)]
    else:
        script = 'test-upstream-headless.py'
        native_fixture_tools()
        perl_lib = find_perl_lib()
        xvfb = find_xserver('Xvfb')
        xephyr = find_xserver('Xephyr')
        flags = ['--rust-bin-dir', str(directory), '--reference-build', str(reference),
                 '--xvfb-dir', str(xvfb.parent), '--xephyr-dir', str(xephyr.parent)]
        if perl_lib is not None:
            flags += ['--perl-lib', str(perl_lib)]
    env = os.environ.copy()
    env['TMPDIR'] = str(temporary_root())
    # Keep child fixture commands offline even if a dependency is missing.
    env['UV_OFFLINE'] = '1'
    command = [str(interpreter), str(ROOT / 'scripts' / script), *flags]
    print(f'Regression {args.suite}: ' + ' '.join(command), flush=True)
    return subprocess.run(command, cwd=ROOT, env=env).returncode


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (MissingTestDependency, OSError) as error:
        print(f'Regression prerequisites failed: {error}', file=sys.stderr)
        raise SystemExit(2)
