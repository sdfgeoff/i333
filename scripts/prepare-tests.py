#!/usr/bin/env python3
"""Prepare the complete offline Cargo regression suite once, under .work.

This explicit setup command may download test dependencies and compile the
pinned external C reference. Routine cargo test never invokes it. Nothing is
installed globally, and no window manager/display is started during setup.
Existing .work reference/Perl/X-server dependencies are reused when valid.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import urllib.request

from regression_environment import (ROOT, DEPS, ENVIRONMENT, PRODUCTS, HELPERS,
    MissingTestDependency, find_reference, find_perl_lib, find_python, find_xserver,
    native_fixture_tools, perl_ready)
from workspace_temp import temporary_root

UPSTREAM = 'be04c853ee7fa01a82a0c81f8217565395e2627c'
PERL_PACKAGES = ('AnyEvent', 'JSON::XS', 'X11::XCB', 'Inline', 'Inline::C',
                 'ExtUtils::PkgConfig', 'IPC::Run')
LOG = DEPS / 'prepare.log'


class PreparationError(RuntimeError):
    pass


def run(arguments, *, env=None, cwd=None):
    print('Preparing: ' + ' '.join(map(str, arguments)), flush=True)
    with LOG.open('ab') as log:
        log.write(('\n$ ' + ' '.join(map(str, arguments)) + '\n').encode())
        result = subprocess.run(list(map(str, arguments)), env=env, cwd=cwd,
                                stdout=log, stderr=subprocess.STDOUT)
    if result.returncode:
        tail = LOG.read_text(errors='replace')[-6000:]
        raise PreparationError(f'Command failed ({result.returncode}); log: {LOG}\n{tail}')


def download(url, destination, offline):
    if destination.is_file():
        return
    if offline:
        raise PreparationError(f'Offline setup needs cached download {destination}; rerun without --offline once.')
    destination.parent.mkdir(parents=True, exist_ok=True)
    partial = destination.with_suffix(destination.suffix + '.part')
    print(f'Downloading test dependency: {url}', flush=True)
    try:
        with urllib.request.urlopen(url, timeout=60) as source, partial.open('wb') as target:
            shutil.copyfileobj(source, target)
        partial.replace(destination)
    finally:
        partial.unlink(missing_ok=True)


def prepare_python(env, offline):
    try:
        return find_python()
    except MissingTestDependency:
        pass
    prefix = DEPS / 'python'
    interpreter = prefix / 'bin/python'
    uv = shutil.which('uv')
    if not interpreter.is_file():
        if uv:
            run([uv, 'venv', '--python', sys.executable, prefix], env=env)
        else:
            run([sys.executable, '-m', 'venv', prefix], env=env)
    if uv:
        command = [uv, 'pip', 'install', '--python', interpreter]
        if offline:
            command += ['--offline']
    else:
        command = [interpreter, '-m', 'pip', 'install', '--cache-dir', DEPS / 'pip-cache']
        if offline:
            command += ['--no-index']
    run([*command, 'python-xlib==0.33'], env=env)
    return interpreter


def meson_command(interpreter, env, offline):
    meson = shutil.which('meson') or str(DEPS / 'tools/bin/meson')
    if Path(meson).is_file():
        return [meson]
    # A host-provided Xlib interpreter may be outside .work. Never pip-install
    # Meson into that interpreter or change its global packages.
    prefix = DEPS / 'tools'
    interpreter = prefix / 'bin/python'
    uv = shutil.which('uv')
    if not interpreter.is_file():
        if uv:
            run([uv, 'venv', '--python', sys.executable, prefix], env=env)
        else:
            run([sys.executable, '-m', 'venv', prefix], env=env)
    if uv:
        command = [uv, 'pip', 'install', '--python', interpreter]
        if offline:
            command += ['--offline']
    else:
        command = [interpreter, '-m', 'pip', 'install', '--cache-dir', DEPS / 'pip-cache']
        if offline:
            command += ['--no-index']
    run([*command, 'meson==1.12.1'], env=env)
    return [str(interpreter.parent / 'meson')]


def extract_archive(archive, destination):
    destination.mkdir(parents=True, exist_ok=True)
    with tarfile.open(archive) as source:
        # Python's data filter rejects escaping links, special files and paths.
        source.extractall(destination, filter='data')


def prepare_reference(interpreter, env, offline, rebuild):
    if not rebuild:
        try:
            return find_reference()
        except MissingTestDependency:
            pass
    for name in ('git', 'cc', 'pkg-config', 'ninja'):
        if not shutil.which(name):
            raise PreparationError(f'{name} is required to prepare the C test reference; see docs/development.md.')
    if subprocess.run(['pkg-config', '--exists', 'yajl'], stdout=subprocess.DEVNULL,
                      stderr=subprocess.DEVNULL).returncode:
        raise PreparationError('The external C test reference needs YAJL development headers (libyajl-dev/yajl). '
                               'They are a one-time test fixture dependency, not a Rust product dependency.')
    source = DEPS / 'source' / f'i3-{UPSTREAM}'
    if not (source / 'meson.build').is_file():
        archive = DEPS / 'downloads' / f'i3-{UPSTREAM}.tar'
        archive.parent.mkdir(parents=True, exist_ok=True)
        available = subprocess.run(['git', 'cat-file', '-e', f'{UPSTREAM}^{{commit}}'], cwd=ROOT,
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0
        if available:
            with archive.open('wb') as target:
                subprocess.run(['git', 'archive', '--format=tar', f'--prefix=i3-{UPSTREAM}/', UPSTREAM],
                               cwd=ROOT, stdout=target, check=True)
        else:
            archive = archive.with_suffix('.tar.gz')
            download(f'https://codeload.github.com/i3/i3/tar.gz/{UPSTREAM}', archive, offline)
        extract_archive(archive, source.parent)
    build = DEPS / 'reference'
    meson = meson_command(interpreter, env, offline)
    setup = [*meson, 'setup', build, source, '-Ddocs=false', '-Dmans=false']
    if (build / 'build.ninja').exists():
        setup += ['--reconfigure']
    run(setup, env=env)
    run(['ninja', '-C', build, *PRODUCTS, *HELPERS], env=env)
    return build


def prepare_perl(env, offline):
    try:
        return find_perl_lib()
    except MissingTestDependency:
        pass
    prefix = DEPS / 'perl'
    cpanm = DEPS / 'cpanm'
    recovered = ROOT / '.work/recovery/i333-cpanm'
    if not cpanm.exists() and recovered.is_file():
        shutil.copy2(recovered, cpanm)
    download('https://cpanmin.us', cpanm, offline)
    command = ['perl', cpanm, '--local-lib-contained', prefix, '--notest',
               '--mirror', 'https://cpan.metacpan.org', '--mirror-only']
    if offline:
        # CPAN dependencies are cached under PERL_CPANM_HOME after initial setup.
        raise PreparationError('Perl modules are not prepared. Run setup once without --offline; '
                               'routine tests and subsequent prepared setup are offline.')
    run([*command, *PERL_PACKAGES], env=env)
    directory = prefix / 'lib/perl5'
    if not perl_ready(directory):
        raise PreparationError(f'Installed Perl modules failed to load from {directory}; inspect {LOG}.')
    return directory


def prepare_xserver(name, env, offline, explicit):
    if explicit is not None:
        executable = explicit.resolve()
    else:
        try:
            executable = find_xserver(name)
        except MissingTestDependency:
            executable = None
        if executable is not None:
            run([executable, '-help'], env=env)
            return executable
        os_release = Path('/etc/os-release').read_text() if Path('/etc/os-release').exists() else ''
        if 'ID=arch\n' not in os_release:
            raise PreparationError(f'{name} is unavailable. Install your distribution\'s Xvfb/Xephyr package '
                                   'or pass --xvfb/--xephyr with local executables; setup never installs globally.')
        package = 'xvfb' if name == 'Xvfb' else 'xephyr'
        filename = f'xorg-server-{package}-21.1.25-2-x86_64.pkg.tar.zst'
        archive = DEPS / 'downloads' / filename
        if not archive.is_file():
            download(f'https://archive.archlinux.org/packages/x/xorg-server-{package}/{filename}', archive, offline)
        destination = DEPS / package
        destination.mkdir(parents=True, exist_ok=True)
        # tar supports Arch's zstd packages; extraction only writes this private tree.
        run(['tar', '-xf', archive, '-C', destination], env=env)
        executable = destination / 'usr/bin' / name
    if not executable.is_file() or not os.access(executable, os.X_OK):
        raise PreparationError(f'{name} executable is invalid: {executable}')
    # Help needs no display and catches missing dynamic-library dependencies.
    run([executable, '-help'], env=env)
    return executable


def warm_inline_cache(perl_lib, env):
    cache = DEPS / 'inline'
    cache.mkdir(parents=True, exist_ok=True)
    local = env.copy()
    modules = [ROOT / 'AnyEvent-I3/lib', ROOT / 'testcases/lib']
    if perl_lib is not None:
        modules.insert(0, perl_lib)
    local['PERL5LIB'] = os.pathsep.join(map(str, modules))
    local['PERL_INLINE_DIRECTORY'] = str(cache)
    # require compiles/loads these native functions; unlike importing XTEST it
    # does not call inlinec_connect, emit tests, open a display or launch a WM.
    run(['perl', '-e', 'require i3test::XTEST;'], env=local, cwd=ROOT)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--offline', action='store_true', help='Reuse prepared/cached dependencies without network')
    parser.add_argument('--rebuild-reference', action='store_true', help='Explicitly rebuild the pinned external C fixture')
    parser.add_argument('--xvfb', type=Path, help='Use an existing local Xvfb executable')
    parser.add_argument('--xephyr', type=Path, help='Use an existing local Xephyr executable')
    args = parser.parse_args()
    DEPS.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    for name in ('DISPLAY', 'I3SOCK', 'WAYLAND_DISPLAY', 'XAUTHORITY', 'DBUS_SESSION_BUS_ADDRESS',
                 'NOTIFY_SOCKET', 'LISTEN_PID', 'LISTEN_FDS', 'LISTEN_FDNAMES'):
        env.pop(name, None)
    env.update(TMPDIR=str(temporary_root()), UV_CACHE_DIR=str(DEPS / 'uv-cache'),
               PERL_CPANM_HOME=str(DEPS / 'cpanm-cache'))
    native_fixture_tools()
    interpreter = prepare_python(env, args.offline)
    reference = prepare_reference(interpreter, env, args.offline, args.rebuild_reference)
    perl_lib = prepare_perl(env, args.offline)
    xvfb = prepare_xserver('Xvfb', env, args.offline, args.xvfb)
    xephyr = prepare_xserver('Xephyr', env, args.offline, args.xephyr)
    path = os.pathsep.join((str(xvfb.parent), str(xephyr.parent), env.get('PATH', '')))
    if not shutil.which('xvfb-run', path=path):
        raise PreparationError('xvfb-run is missing; supply the complete Xvfb package, not only its server binary.')
    warm_inline_cache(perl_lib, env)
    config = {'schema_version': 1, 'upstream_revision': UPSTREAM,
              'reference_build': str(reference.resolve()), 'python': str(interpreter.absolute()),
              'perl_lib': None if perl_lib is None else str(perl_lib.resolve()),
              'xvfb': str(xvfb.resolve()), 'xephyr': str(xephyr.resolve())}
    ENVIRONMENT.write_text(json.dumps(config, indent=2) + '\n')
    print(f'Test dependencies prepared: {ENVIRONMENT}\nRun cargo test. No setup or network is needed on later runs.')
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (PreparationError, MissingTestDependency, OSError, subprocess.SubprocessError) as error:
        print(f'Test preparation failed: {error}', file=sys.stderr)
        raise SystemExit(2)
