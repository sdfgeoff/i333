#!/usr/bin/env python3
"""Private Xvfb wizard workflow/config parity, never the desktop or user config.

Only disposable executable copies are patched: the exact 33-byte installed
config.keycodes path is replaced with an equal-length cwd fixture filename.
Executable logic and originals stay untouched; original hashes are recorded.
Every generated 'reload' goes to an owned fake IPC listener, never a live WM.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import socket
import struct
import subprocess
import time

from Xlib import X
from workspace_temp import make_tempdir

_spec = importlib.util.spec_from_file_location('input_parity', Path(__file__).with_name('test-input-parity.py'))
_input = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_input)

ORIGINAL_TEMPLATE = b'/usr/local/etc/i3/config.keycodes'
FIXTURE_NAME = b'config.keycodes'.ljust(33, b'_')
assert len(FIXTURE_NAME) == len(ORIGINAL_TEMPLATE) == 33
TEMPLATE = (b'# WARNING omit this line\n# WARNING omit this one too\n'
            b'\n# preserved raw template\nset $mod Mod4\n'
            b'bindcode $mod+36 exec fixture-never-executed\n'
            b'bindcode Shift+38 exec shifted\n'
            b'bindcode --release $mod+39 exec released\n'
            b'\tbindcode $mod+24 exec raw-\xff\n'
            b'bindcode 40 "exec \\"quoted\\""\n')
CASES = [('welcome-abort', [], ['Escape'], False),
         ('choose-alt-abort', [], ['Return', 'Down', 'Escape'], False),
         ('generate-win', [], ['Return', 'Return'], True),
         ('generate-alt', [], ['Return', 'Down', 'Return'], True),
         ('headless-alt', ['-m', 'alt'], [], True)]


def copied_binary(binary, label, work):
    original = binary.read_bytes()
    count = original.count(ORIGINAL_TEMPLATE)
    assert count == 1, (binary, 'expected one embedded template path', count)
    patched = original.replace(ORIGINAL_TEMPLATE, FIXTURE_NAME)
    assert len(patched) == len(original)
    path = work / (label + '-wizard')
    path.write_bytes(patched)
    path.chmod(binary.stat().st_mode & 0o777)
    return path, {'original_sha256': hashlib.sha256(original).hexdigest(),
                  'fixture_sha256': hashlib.sha256(patched).hexdigest(), 'template_path_replacements': count}


def rendered(connection, child, expected_title, previous=None):
    deadline = time.monotonic() + 12
    stable = None
    while time.monotonic() < deadline:
        assert child.poll() is None, ('wizard exited before drawing', child.returncode)
        windows = _input.mapped_windows(connection.screen().root)
        if len(windows) == 1:
            window = windows[0]
            title = window.get_full_property(connection.intern_atom('_NET_WM_NAME'), X.AnyPropertyType)
            if title is not None and bytes(title.value) == expected_title:
                geometry = window.get_geometry()
                image = window.get_image(0, 0, geometry.width, geometry.height, X.ZPixmap, 0xffffffff)
                colors = {image.data[index:index + 4] for index in range(0, len(image.data), 4)}
                snapshot = {'geometry': [geometry.x, geometry.y, geometry.width, geometry.height],
                            'pixels_sha256': hashlib.sha256(image.data).hexdigest()}
                # Actual text/selection rendering contains foreground, background
                # and both prompt colors; mapping alone is insufficient.
                if len(colors) >= 4 and snapshot == stable and snapshot != previous:
                    return snapshot
                stable = snapshot if len(colors) >= 4 else None
        connection.sync()
        time.sleep(.04)
    raise AssertionError('wizard failed to complete requested stable rendering')


def invoke(binary, label, case, connection, environment, server, work):
    name, arguments, keys, generate = case
    case_work = work / name
    case_work.mkdir(exist_ok=True)
    config = case_work / 'i3/config'
    config.unlink(missing_ok=True)
    (case_work / os.fsdecode(FIXTURE_NAME)).write_bytes(TEMPLATE)
    env = environment.copy()
    env.update(HOME=str(case_work), XDG_CONFIG_HOME=str(case_work), TMPDIR=str(case_work))
    # Explicit relative socket and cwd keep the native Unix address bounded.
    sockpath = case_work / 'i.sock'
    sockpath.unlink(missing_ok=True)
    listener = socket.socket(socket.AF_UNIX)
    spelling = min((str(sockpath), os.path.relpath(sockpath)), key=lambda value: len(os.fsencode(value)))
    assert len(os.fsencode(spelling)) < 108
    listener.bind(spelling)
    listener.listen(1)
    listener.settimeout(12)
    child = None
    snapshots = []
    try:
        _input.require_private(connection, environment, server)
        assert not _input.mapped_windows(connection.screen().root)
        with (work / f'{label}-{name}.log').open('wb') as log:
            child = subprocess.Popen([str(binary), '--socket', 'i.sock', *arguments], env=env,
                                     cwd=case_work, stdout=log, stderr=log)
        if keys:
            deadline = time.monotonic() + 12
            grabbed = False
            while time.monotonic() < deadline:
                assert child.poll() is None, (name, child.returncode)
                reply = connection.screen().root.grab_keyboard(False, X.GrabModeAsync, X.GrabModeAsync, X.CurrentTime)
                if reply == X.AlreadyGrabbed:
                    grabbed = True
                    break
                if reply == X.GrabSuccess:
                    connection.ungrab_keyboard(X.CurrentTime)
                    connection.sync()
                time.sleep(.01)
            assert grabbed, 'wizard did not acquire private keyboard grab'
            snapshots.append(rendered(connection, child, b'i3: first configuration'))
            step = b'i3: first configuration'
            for index, key in enumerate(keys):
                _input.press(connection, environment, server, key)
                if key == 'Return' and index == 0:
                    step = b'i3: generate config'
                    snapshots.append(rendered(connection, child, step))
                elif key == 'Down':
                    snapshots.append(rendered(connection, child, step, snapshots[-1]))
        payload = None
        if generate:
            client, _ = listener.accept()
            with client:
                client.settimeout(12)
                header = _input.receive(client, 14)
                magic, size, kind = struct.unpack('=6sII', header)
                assert (magic, kind) == (b'i3-ipc', 0)
                payload = _input.receive(client, size)
                assert payload == b'reload'
        status = child.wait(timeout=12)
        assert status == 0, (name, status)
        assert config.exists() == generate
        written = config.read_bytes() if generate else None
        if generate:
            assert b'bindcode ' not in written
            assert b'raw-\xff' in written
            assert b'# WARNING' not in written
            expected_mod = b'Mod1' if name.endswith('alt') else b'Mod4'
            assert b'set $mod ' + expected_mod + b'\n' in written
            (work / f'{label}-{name}.config').write_bytes(written)
        connection.sync()
        deadline = time.monotonic() + 3
        while _input.mapped_windows(connection.screen().root) and time.monotonic() < deadline:
            connection.sync()
            time.sleep(.02)
        assert not _input.mapped_windows(connection.screen().root), 'wizard window survived session close'
        return {'status': status, 'snapshots': snapshots, 'config_hex': written.hex() if written is not None else None,
                'ipc': payload.decode() if payload is not None else None}
    finally:
        _input.stop(child)
        listener.close()
        sockpath.unlink(missing_ok=True)
        connection.sync()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path)
    group = parser.add_mutually_exclusive_group()
    group.add_argument('--candidate', type=Path)
    group.add_argument('--rust-bin-dir', type=Path)
    parser.add_argument('--xvfb', type=Path, default=shutil.which('Xvfb'))
    args = parser.parse_args()
    if args.xvfb is None:
        parser.error('Private Xvfb required; no desktop fallback is permitted')
    candidate = (args.candidate or (args.rust_bin_dir or _input.cargo_bin_dir()) / 'i3-config-wizard').resolve()
    binaries = [('candidate', candidate)]
    if args.reference:
        binaries.insert(0, ('reference', args.reference.resolve()))
    work = Path(make_tempdir(prefix='wizard-'))
    results, hashes = {}, {}
    try:
        with _input.private_server(args.xvfb.resolve(), work) as (connection, environment, server):
            for label, executable in binaries:
                copied, hashes[label] = copied_binary(executable, label, work)
                results[label] = {case[0]: invoke(copied, label, case, connection, environment, server, work) for case in CASES}
            if args.reference:
                assert results['candidate'] == results['reference'], results
        print(f'PASS: {len(CASES)} private wizard workflow/config cases' + (' match external C' if args.reference else ' satisfy fixtures'))
    finally:
        (work / 'results.json').write_text(json.dumps({'binaries': hashes, 'results': results}, indent=2) + '\n')
        print(f'Private wizard artifacts: {work}')


if __name__ == '__main__':
    main()
