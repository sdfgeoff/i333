#!/usr/bin/env python3
"""Build the Rust workspace and all test targets from a clean copy without C or Meson.

Run after Cargo support is ready:
    python scripts/test-cargo-build.py

Workspace and member Cargo manifests/build.rs files, Rust sources and explicitly
listed configuration assets are copied. A fresh target directory prevents reuse of project objects.
Compiler wrappers allow native linking but reject C compilation; build-system
and parser-generator wrappers fail if invoked. Version checks run without any
DISPLAY, WAYLAND_DISPLAY or I3SOCK and never connect to the user's desktop.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
from workspace_temp import temporary_directory

ROOT = Path(__file__).resolve().parents[1]
BINARIES = ('i3', 'i3bar', 'i3-config-wizard', 'i3-dump-log', 'i3-input',
            'i3-msg', 'i3-nagbar')
SKIP_DIRS = {'.work', '.git', '.agents', '.aws', '.codex', '.venv', '__pycache__',
             'build', 'target'}
CONFIG_ASSETS = ('I3_VERSION', 'etc/config', 'etc/config.keycodes')
NATIVE_SOURCE_SUFFIXES = {'.c', '.h', '.cc', '.cpp', '.cxx', '.hh', '.hpp', '.hxx'}
CACHE_DIRS = {'.work', '.git', '.agents', '.aws', '.codex', '.venv', '.cache',
              '__pycache__', 'target'}



def assert_rust_only_repository():
    """Reject source/header leftovers, including those in old build directories."""
    leftovers = []
    for directory, folders, files in os.walk(ROOT):
        folders[:] = [name for name in folders if name not in CACHE_DIRS]
        for name in files:
            path = Path(directory) / name
            if path.suffix.lower() in NATIVE_SOURCE_SUFFIXES or name in {
                'meson.build', 'meson_options.txt',
            }:
                leftovers.append(str(path.relative_to(ROOT)))
    assert not leftovers, ('repository still contains native sources, headers or '
                           'Meson build definitions: ' + ', '.join(sorted(leftovers)))


def copy_clean_sources(destination):
    for name in ('Cargo.toml', 'Cargo.lock'):
        if not (ROOT / name).is_file():
            raise SystemExit(f'{name} is missing; workspace Cargo support is not ready')
    for name in ('rust-toolchain.toml', 'rust-toolchain', '.cargo/config.toml',
                 '.cargo/config', *CONFIG_ASSETS):
        source = ROOT / name
        if source.is_file():
            target = destination / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    count = 0
    for directory, folders, files in os.walk(ROOT):
        folders[:] = [name for name in folders if name not in SKIP_DIRS]
        for name in files:
            if not name.endswith('.rs') and name not in {'Cargo.toml', 'Cargo.lock'}:
                continue
            source = Path(directory) / name
            assert source.resolve().is_relative_to(ROOT), source
            target = destination / source.relative_to(ROOT)
            target.parent.mkdir(parents=True, exist_ok=True)
            if str(ROOT) in source.read_text():
                raise AssertionError(f'Cargo input contains original absolute path: {source}')
            shutil.copy2(source, target)
            count += name.endswith('.rs')
    forbidden = [path for path in destination.rglob('*')
                 if path.suffix in {'.c', '.h', '.pl', '.a', '.o'}]
    assert not forbidden, forbidden
    assert not (destination / 'build').exists()
    return count


def write_tool_guards(directory):
    """Keep the native linker usable while making C/generator fallbacks fail."""
    directory.mkdir()
    for name in ('cc', 'gcc', 'clang', 'c++', 'g++', 'clang++'):
        actual = shutil.which(name)
        if not actual:
            continue
        guard = directory / name
        guard.write_text(
            '#!/usr/bin/env python3\n'
            'import os, sys\n'
            'from pathlib import Path\n'
            'args = sys.argv[1:]\n'
            'for arg in args:\n'
            '    if arg in {"-c", "-E", "-S"} or Path(arg).suffix in '
            '{".c", ".cc", ".cpp", ".cxx", ".h", ".hpp"}:\n'
            '        sys.exit("clean Cargo build attempted native source compilation: " + repr(args))\n'
            f'os.execv({actual!r}, [{actual!r}, *args])\n'
        )
        guard.chmod(guard.stat().st_mode | stat.S_IXUSR)
    for name in ('meson', 'ninja', 'make', 'gmake', 'cmake', 'perl'):
        guard = directory / name
        guard.write_text('#!/bin/sh\necho "forbidden build dependency: ' + name
                         + '" >&2\nexit 97\n')
        guard.chmod(guard.stat().st_mode | stat.S_IXUSR)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--offline', action='store_true',
                        help='use only already cached Cargo dependencies')
    parser.add_argument('--toolchain', help='override the copied rust-toolchain.toml')
    args = parser.parse_args()
    assert_rust_only_repository()
    with temporary_directory(prefix='i333-clean-cargo-') as scratch:
        scratch = Path(scratch)
        source = scratch / 'source'
        source.mkdir()
        count = copy_clean_sources(source)
        guards = scratch / 'tool-guards'
        write_tool_guards(guards)
        env = os.environ.copy()
        for name in ('DISPLAY', 'WAYLAND_DISPLAY', 'I3SOCK', 'CARGO_TARGET_DIR',
                     'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'RUSTFLAGS',
                     'CARGO_ENCODED_RUSTFLAGS', 'I3_VERSION', 'RUSTUP_TOOLCHAIN',
                     'RUSTC', 'RUSTDOC'):
            env.pop(name, None)
        if args.toolchain:
            env['RUSTUP_TOOLCHAIN'] = args.toolchain
        env['PATH'] = str(guards) + os.pathsep + env['PATH']
        env['CC'] = str(guards / 'cc')
        env['CXX'] = str(guards / 'c++')
        env['CARGO_TARGET_DIR'] = str(scratch / 'target')
        metadata_command = ['cargo', 'metadata', '--locked', '--no-deps',
                            '--format-version', '1']
        if args.offline:
            metadata_command.append('--offline')
        metadata = subprocess.check_output(
            metadata_command,
            cwd=source, env=env, text=True,
        )
        metadata = json.loads(metadata)
        assert Path(metadata['workspace_root']) == source, metadata['workspace_root']
        binary_targets = set()
        for package in metadata['packages']:
            assert Path(package['manifest_path']).is_relative_to(source), package
            for dependency in package['dependencies']:
                if dependency.get('path'):
                    assert Path(dependency['path']).is_relative_to(source), dependency
            for target in package['targets']:
                assert Path(target['src_path']).is_relative_to(source), target
                if 'bin' in target['kind']:
                    binary_targets.add(target['name'])
        assert binary_targets == set(BINARIES), binary_targets
        for action, extra in [('build', ['--bins']), ('test', ['--no-run'])]:
            command = ['cargo', action, '--workspace', '--locked', *extra, '-j4']
            if args.offline:
                command.append('--offline')
            subprocess.run(command, cwd=source, env=env, check=True)
        for name in BINARIES:
            binary = scratch / 'target' / 'debug' / name
            assert binary.is_file(), f'Cargo did not build {name}'
            result = subprocess.run([str(binary), '-v'], cwd=source, env=env,
                                    capture_output=True, timeout=10, check=True)
            assert result.stdout.strip(), f'{name} returned an empty version'
            assert name.encode() in result.stdout, (name, result.stdout)
            assert not result.stderr, (name, result.stderr)
            print(result.stdout.decode().strip(), flush=True)
        print(f'Clean Cargo workspace build/test-target compilation passed: {count} Rust files, seven binaries, '
              'no C sources/objects, Meson, Perl, or existing build directory.')


if __name__ == '__main__':
    main()
