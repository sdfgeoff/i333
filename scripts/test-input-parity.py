#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["python-xlib==0.33"]
# ///
"""Exercise i3-input on a private Xvfb server and capture its IPC commands.

Run after cargo build:
    uv run scripts/test-input-parity.py --xvfb /path/to/Xvfb
Add --release for target/release; --candidate or --rust-bin-dir overrides Cargo's
normal output path. --reference optionally compares an external C executable;
standalone Rust fixtures do not require it. --extended includes the historical
512-glyph boundary. Input editing, limits, cancellation, formatting and legacy
Unicode packing are checked against explicit expected IPC commands/exit codes.

Does not start/replace a window manager or inject any input on an inherited
DISPLAY. The only signalled processes are Popen children owned by this runner.
Artifacts default to the repository's .work/tmp directory; TMPDIR overrides it.
"""
import argparse
from contextlib import contextmanager
import json
import os
from pathlib import Path
import select
import socket
import shutil
import struct
import subprocess
import threading
import time

from Xlib import X, Xatom, XK, display, error
from Xlib.ext import xtest
from workspace_temp import make_tempdir

ROOT = Path(__file__).resolve().parents[1]
TIMEOUT = 12

CASES = [
    ('ascii', [], ['a', 'b', 'Return'], 0, b'ab'),
    ('backspace', [], ['a', 'b', 'BackSpace', 'c', 'Return'], 0, b'ac'),
    ('empty-backspace', [], ['BackSpace', 'Return'], 0, b''),
    ('shift', [], ['a', 'B', 'Return'], 0, b'aB'),
    ('escape-empty', [], ['Escape'], 1, None),
    ('escape-entered', [], ['a', 'Escape'], 1, None),
    ('limit-three', ['-l', '3'], ['a', 'b', 'c'], 0, b'abc'),
    ('limit-one', ['-l', '1'], ['x'], 0, b'x'),
    ('format-repeated', ['-F', 'mark %s; rename %s'], ['a', 'b', 'Return'], 0, b'mark ab; rename ab'),
    ('format-overlap', ['-F', '%s|%%s|%s'], ['a', 'b', 'Return'], 0, b'ab|%ab|ab'),
    ('format-literal', ['-F', 'fixed-command'], ['a', 'Return'], 0, b'fixed-command'),
    ('root-discovery', [], ['x', 'Return'], 0, b'x'),
    ('latin-one', [], [255, 'Return'], 0, 'é'.encode()),
    ('latin-one-backspace', [], [255, 'BackSpace', 'a', 'Return'], 0, b'a'),
    ('limit-utf8-glyph', ['-l', '1'], [255], 0, 'é'.encode()),
    # Upstream C uses (ucs & 0xff00) >> 2, then truncates to byte1.
    # Compare the exact resulting command, rather than silently fixing that quirk.
    ('higher-ucs-legacy', [], [254, 'Return'], 0, '¬'.encode()),
]


def receive(connection, length):
    body = bytearray()
    while len(body) < length:
        part = connection.recv(length - len(body))
        if not part:
            if not body:
                return None
            raise AssertionError('partial IPC frame at EOF')
        body.extend(part)
    return bytes(body)


def stop(child):
    if child is not None and child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=3)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=3)


@contextmanager
def private_server(xvfb, work):
    environment = {key: value for key, value in os.environ.items() if key not in {
        'DISPLAY', 'I3SOCK', 'WAYLAND_DISPLAY', 'XAUTHORITY', 'DBUS_SESSION_BUS_ADDRESS',
        'NOTIFY_SOCKET', 'LISTEN_PID', 'LISTEN_FDS', 'LISTEN_FDNAMES'}}
    environment.update(HOME=str(work), XDG_CONFIG_HOME=str(work), XDG_RUNTIME_DIR=str(work),
                       TMPDIR=str(work))
    read_fd, write_fd = os.pipe()
    connection = None
    server = None
    try:
        with (work / 'xvfb.log').open('wb') as log:
            server = subprocess.Popen([str(xvfb), '-displayfd', str(write_fd), '-screen', '0',
                                       '1024x768x24', '-nolisten', 'tcp', '-noreset'],
                                      env=environment, pass_fds=(write_fd,), stdout=log, stderr=log)
        os.close(write_fd)
        write_fd = None
        assert server.pid != os.getpid()
        if not select.select([read_fd], [], [], 8)[0]:
            raise RuntimeError('private Xvfb did not allocate a display')
        number = os.read(read_fd, 32).decode().strip()
        assert number.isdecimal(), number
        private_display = ':' + number
        assert server.poll() is None
        # The display comes only from the pipe belonging to our new Xvfb process.
        environment['DISPLAY'] = private_display
        connection = display.Display(private_display)
        assert connection.get_display_name() == private_display
        assert connection.has_extension('XTEST'), 'private Xvfb lacks XTEST'
        # Only this private server gets these otherwise unused keycode mappings.
        connection.change_keyboard_mapping(254, [(0x20ac,), (0xe9,)])
        connection.sync()
        yield connection, environment, server
    finally:
        if write_fd is not None:
            os.close(write_fd)
        os.close(read_fd)
        if connection is not None:
            connection.close()
        stop(server)


def require_private(connection, environment, server):
    assert server.pid > 0 and server.poll() is None, 'owned private server is not running'
    assert connection.get_display_name() == environment['DISPLAY']
    assert environment['DISPLAY'].startswith(':') and environment['DISPLAY'][1:].isdecimal()


def press(connection, environment, server, key):
    require_private(connection, environment, server)
    if isinstance(key, int):
        code, shift = key, False
    else:
        keysym = XK.string_to_keysym(key)
        assert keysym, ('unknown keysym', key)
        possibilities = list(connection.keysym_to_keycodes(keysym))
        assert possibilities, ('unmapped keysym', key)
        code, level = next((pair for pair in possibilities if pair[1] <= 1), possibilities[0])
        assert level <= 1, (key, code, level)
        shift = bool(level & 1)
    shift_code = connection.keysym_to_keycode(XK.string_to_keysym('Shift_L'))
    if shift:
        xtest.fake_input(connection, X.KeyPress, shift_code)
    xtest.fake_input(connection, X.KeyPress, code)
    xtest.fake_input(connection, X.KeyRelease, code)
    if shift:
        xtest.fake_input(connection, X.KeyRelease, shift_code)
    connection.sync()
    time.sleep(.005)


def mapped_windows(root):
    windows = []
    for window in root.query_tree().children:
        try:
            if window.get_attributes().map_state == X.IsViewable:
                windows.append(window)
        except error.BadWindow:
            pass  # The exiting child can destroy a window after QueryTree.
    return windows


def invoke(binary, case, connection, environment, server, work):
    name, arguments, keys, expected_status, expected_command = case
    require_private(connection, environment, server)
    sockpath = work / 'i.sock'
    sockpath.unlink(missing_ok=True)
    # Keep the native Unix address short even in a deeply nested checkout.
    # The child resolves i.sock relative to its private cwd; the server can bind
    # the same file using whichever spelling is shorter from this process.
    bind_path = min((str(sockpath), os.path.relpath(sockpath)), key=lambda p: len(os.fsencode(p)))
    assert len(os.fsencode(bind_path)) < 108, ('Unix socket path too long; set TMPDIR to a shorter private directory', bind_path)
    listener = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    listener.bind(bind_path)
    listener.listen(1)
    listener.settimeout(TIMEOUT)
    result, errors = [], []

    def serve():
        try:
            client, _ = listener.accept()
            with client:
                client.settimeout(TIMEOUT)
                header = receive(client, 14)
                if header is None:
                    result.append(None)
                    return
                magic, size, kind = struct.unpack('=6sII', header)
                assert magic == b'i3-ipc' and kind == 0, (magic, size, kind)
                payload = b'' if size == 0 else receive(client, size)
                assert payload is not None
                result.append(payload)
        except BaseException as error:
            errors.append(repr(error))

    thread = threading.Thread(target=serve, daemon=True)
    thread.start()
    root = connection.screen().root
    atom = connection.intern_atom('I3_SOCKET_PATH')
    root.change_property(atom, Xatom.STRING, 8, b'i.sock')
    connection.sync()
    args = list(arguments)
    if name != 'root-discovery':
        args = ['-s', 'i.sock', *args]
    child = None
    try:
        with (work / f'{binary.parent.name}-{name}.log').open('wb') as log:
            child = subprocess.Popen([str(binary), *args], env=environment,
                                     cwd=work, stdout=log, stderr=log)
        deadline = time.monotonic() + 8
        ready = False
        while time.monotonic() < deadline:
            require_private(connection, environment, server)
            if child.poll() is not None:
                raise AssertionError(f'{binary} exited before private keyboard grab; case {name}')
            mapped = mapped_windows(root)
            if mapped:
                reply = root.grab_keyboard(False, X.GrabModeAsync, X.GrabModeAsync, X.CurrentTime)
                if reply == X.AlreadyGrabbed:
                    ready = True
                    break
                if reply == X.GrabSuccess:
                    connection.ungrab_keyboard(X.CurrentTime)
                    connection.sync()
            time.sleep(.01)
        assert ready, f'{binary} did not obtain private keyboard grab; case {name}'
        for key in keys:
            press(connection, environment, server, key)
        status = child.wait(timeout=TIMEOUT)
        thread.join(timeout=TIMEOUT)
        assert not thread.is_alive(), 'private IPC capture did not finish'
        assert not errors, errors
        assert result == [expected_command], (binary.name, name, result, expected_command)
        assert status == expected_status, (binary.name, name, status, expected_status)
        return {'status': status, 'command_hex': None if result[0] is None else result[0].hex()}
    finally:
        stop(child)
        listener.close()
        sockpath.unlink(missing_ok=True)
        # Wait until the server has processed this child's disconnection before
        # another program is eligible to receive synthetic keys.
        connection.sync()
        deadline = time.monotonic() + 3
        while server.poll() is None and time.monotonic() < deadline:
            if not mapped_windows(root):
                break
            time.sleep(.01)
        else:
            raise AssertionError('owned input window did not disappear after child exit')


def cargo_bin_dir(release=False):
    target = Path(os.environ.get('CARGO_TARGET_DIR', 'target'))
    if not target.is_absolute():
        target = ROOT / target
    return target / ('release' if release else 'debug')


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--reference', type=Path, help='Optional external C i3-input executable')
    candidates = parser.add_mutually_exclusive_group()
    candidates.add_argument('--candidate', type=Path, help='Explicit candidate i3-input executable')
    candidates.add_argument('--rust-bin-dir', type=Path, help='Directory containing Cargo-built products')
    parser.add_argument('--release', action='store_true', help='Use target/release instead of target/debug')
    parser.add_argument('--xvfb', type=Path, default=shutil.which('Xvfb'),
                        help='Private Xvfb executable (required; never falls back to desktop DISPLAY)')
    parser.add_argument('--extended', action='store_true', help='Also compare the historical 512-glyph boundary')
    args = parser.parse_args()
    if args.xvfb is None:
        parser.error('Xvfb is required; pass --xvfb /path/to/Xvfb')
    args.xvfb = args.xvfb.resolve()
    directory = args.rust_bin_dir or cargo_bin_dir(args.release)
    args.candidate = (args.candidate or directory / 'i3-input').resolve()
    binaries = [('candidate', args.candidate)]
    if args.reference:
        binaries.insert(0, ('reference', args.reference.resolve()))
    for binary in [args.xvfb, *[binary for _, binary in binaries]]:
        if not binary.is_file() or not os.access(binary, os.X_OK):
            parser.error(f'Missing executable {binary}; build products first or provide an explicit path')
    work = Path(make_tempdir(prefix='ipt-'))
    cases = list(CASES)
    if args.extended:
        cases.append(('capacity-512', [], ['a'] * 512 + ['Return'], 0, b'a' * 512))
    results = {}
    try:
        with private_server(args.xvfb, work) as (connection, environment, server):
            for name, binary in binaries:
                outcomes = {}
                results[name] = outcomes
                for case in cases:
                    outcomes[case[0]] = invoke(binary, case, connection, environment, server, work)
            if args.reference:
                assert results['reference'] == results['candidate']
        print(f'PASS: {len(cases)} private Xvfb input cases' +
              (' match external C' if args.reference else ' match expected fixtures'))
    finally:
        (work / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
        print(f'Private input artifacts: {work}')


if __name__ == '__main__':
    main()
