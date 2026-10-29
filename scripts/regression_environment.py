"""Resolve prepared regression tools without installing or downloading anything."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
DEPS = ROOT / '.work/test-deps'
ENVIRONMENT = DEPS / 'environment.json'
PRODUCTS = ('i3', 'i3bar', 'i3-config-wizard', 'i3-dump-log', 'i3-input', 'i3-msg', 'i3-nagbar')
HELPERS = ('test.commands_parser', 'test.config_parser', 'test.inject_randr15')
PREPARE = 'python3 scripts/prepare-tests.py'
PERL_MODULES = ('AnyEvent', 'JSON::XS', 'X11::XCB::Connection', 'Inline', 'Inline::C',
                'ExtUtils::PkgConfig', 'IPC::Run')


class MissingTestDependency(RuntimeError):
    pass


def setup_error(message):
    return MissingTestDependency(f'{message}\nPrepare test dependencies once: {PREPARE}\n'
                                 'See docs/development.md, “One-time regression setup”. '
                                 'Tests never download dependencies or use the desktop display.')


def settings():
    path = Path(os.environ.get('I333_TEST_ENVIRONMENT', ENVIRONMENT)).expanduser()
    if not path.exists():
        return {}
    try:
        result = json.loads(path.read_text())
        if not isinstance(result, dict) or result.get('schema_version') != 1:
            raise ValueError('unsupported environment schema')
        return result
    except (OSError, ValueError) as error:
        raise setup_error(f'Cannot read prepared test environment {path}: {error}') from error


def configured_paths(name, defaults):
    # Preserve a venv interpreter's spelling: resolving its symlink would invoke
    # the system interpreter without that venv's installed test dependencies.
    def path(value):
        value = Path(value).expanduser()
        return value.absolute() if name == 'python' else value.resolve()

    variable = 'I333_TEST_' + name.upper()
    value = os.environ.get(variable)
    if value is not None:
        return [path(value)]
    value = settings().get(name)
    if value:
        return [path(value)]
    return [path(value) for value in defaults if value]


def find_reference():
    for directory in configured_paths('reference_build', (DEPS / 'reference', ROOT / '.work/c-reference')):
        if all((directory / name).is_file() and os.access(directory / name, os.X_OK)
               for name in (*PRODUCTS, *HELPERS)):
            return directory
    raise setup_error('The pinned external C reference and its three parser/RandR helpers are missing.')


def find_xserver(name):
    key = name.lower()
    package = 'xvfb' if name == 'Xvfb' else 'xephyr'
    defaults = (DEPS / package / 'usr/bin' / name,
                ROOT / f'.work/recovery/i333-{package}/usr/bin' / name,
                ROOT / f'.work/i333-{package}/usr/bin' / name,
                shutil.which(name))
    for executable in configured_paths(key, defaults):
        if executable.is_file() and os.access(executable, os.X_OK):
            return executable
    raise setup_error(f'Private {name} is unavailable; install it or configure I333_TEST_{key.upper()}.')


def perl_environment(directory):
    env = os.environ.copy()
    env.pop('PERL5LIB', None)
    if directory is not None:
        env['PERL5LIB'] = str(directory)
    return env


def perl_ready(directory):
    if not shutil.which('perl'):
        return False
    command = '; '.join('require ' + name for name in PERL_MODULES) + ';'
    result = subprocess.run(['perl', '-e', command], env=perl_environment(directory),
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return result.returncode == 0


def find_perl_lib():
    candidates = configured_paths('perl_lib', (DEPS / 'perl/lib/perl5',
        ROOT / '.work/recovery/i333-perl/lib/perl5', ROOT / '.work/i333-perl/lib/perl5'))
    for directory in candidates:
        if directory.is_dir() and perl_ready(directory):
            return directory
    if 'I333_TEST_PERL_LIB' not in os.environ and not settings().get('perl_lib') and perl_ready(None):
        return None
    raise setup_error('Perl test modules are missing or incompatible with the installed Perl. '
                      'Required: ' + ', '.join(PERL_MODULES) + '.')


def python_ready(executable):
    try:
        return subprocess.run([str(executable), '-c', 'import Xlib, six'],
                              stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0
    except OSError:
        return False


def find_python():
    for executable in configured_paths('python', (DEPS / 'python/bin/python', sys.executable)):
        if executable.is_file() and os.access(executable, os.X_OK) and python_ready(executable):
            return executable
    raise setup_error('Python Xlib is unavailable to the prepared interpreter. '
                      'The one-time setup installs python-xlib==0.33 locally.')


def native_fixture_tools():
    missing = [name for name in ('perl', 'cc', 'pkg-config', 'xauth', 'xkbcomp') if not shutil.which(name)]
    if missing:
        raise setup_error('Missing upstream fixture tools: ' + ', '.join(missing))
    # X11::XCB checks these libraries at configure time, then generates XS from
    # xcb-proto XML files. Check all of them before starting a CPAN build.
    packages = ('xcb', 'xcb-composite', 'xcb-icccm', 'xcb-randr', 'xcb-util',
                'xcb-xinerama', 'xcb-xkb', 'xcb-proto', 'xcb-xtest',
                'libstartup-notification-1.0')
    missing = [name for name in packages if subprocess.run(
        ['pkg-config', '--exists', name], stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL).returncode]
    if missing:
        raise setup_error('Missing native pkg-config dependencies for Perl/XCB test fixtures: '
                          + ', '.join(missing))
    xml_directory = subprocess.check_output(
        ['pkg-config', '--variable=xcbincludedir', 'xcb-proto'], text=True).strip()
    xml_files = ('xproto.xml', 'xinerama.xml', 'randr.xml', 'xkb.xml', 'composite.xml')
    missing_xml = [name for name in xml_files if not (Path(xml_directory) / name).is_file()]
    if not xml_directory or missing_xml:
        raise setup_error('xcb-proto protocol XML files are missing from '
                          + repr(xml_directory) + ': ' + ', '.join(missing_xml))
