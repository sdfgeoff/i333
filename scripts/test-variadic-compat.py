#!/usr/bin/env python3
"""Check actual variadic forwarding with Cargo's integration target.

Plain cargo test includes this check on the current toolchain. This command is
an optional convenience for explicitly testing additional installed compilers;
it no longer creates a nested throwaway Cargo package.
"""
import argparse
from pathlib import Path
import subprocess
from cargo_probe import build_probe

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--toolchain', action='append', default=[])
    parser.add_argument('--probe', type=Path, help='Run an already-built variadic_compat executable')
    args = parser.parse_args()
    if args.probe:
        if args.toolchain:
            parser.error('--toolchain cannot be combined with --probe')
        subprocess.run([str(args.probe.resolve())], check=True)
        return
    for toolchain in args.toolchain or ['nightly']:
        probe = build_probe('i3-common', 'variadic_compat', offline=True, toolchain=toolchain)
        subprocess.run([str(probe)], check=True)


if __name__ == '__main__':
    main()
