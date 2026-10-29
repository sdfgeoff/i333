#!/usr/bin/env python3
"""Compare nagbar rendering and close cleanup on an owned private Xvfb only.

Uses the prepared python-xlib interpreter, never the inherited desktop DISPLAY.
Optional --reference compares an external C binary. Action buttons contain ':'
fixtures and are never clicked; only the close button receives private input.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

from Xlib import X
from Xlib.ext import xtest
from workspace_temp import make_tempdir

# Share the existing private-display ownership checks without creating another
# subtly different fallback policy. This module does nothing at import time.
_spec = importlib.util.spec_from_file_location('input_parity', Path(__file__).with_name('test-input-parity.py'))
_input = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_input)

CASES = [
    ('default', [], 0xffffff),
    ('error', ['-t', 'error', '-m', 'Configuration error: example'], 0xffffff),
    ('warning', ['-t', 'warning', '-m', 'Warning: example'], 0x000000),
    ('buttons', ['-m', 'Two inert actions', '-b', 'First action', ':', '-B', 'Second action', ':'], 0xffffff),
]
TIMEOUT = 12


def rendered_window(connection, child, foreground):
    """Require actual prompt glyphs and two identical rasters after mapping."""
    root = connection.screen().root
    deadline = time.monotonic() + TIMEOUT
    previous = None
    while time.monotonic() < deadline:
        assert child.poll() is None, f'nagbar exited before drawing: {child.returncode}'
        windows = _input.mapped_windows(root)
        if len(windows) == 1:
            window = windows[0]
            geometry = window.get_geometry()
            if geometry.width >= 400 and geometry.height > 10:
                image = window.get_image(0, 0, geometry.width, geometry.height, X.ZPixmap, 0xffffffff)
                # Private Xvfb uses a 24-bit truecolor screen in 32-bit pixels.
                # Check prompt glyphs (not a button/border) to exclude blank
                # map/background races before comparing stable whole rasters.
                assert len(image.data) == geometry.width * geometry.height * 4
                byteorder = 'little' if connection.display.info.image_byte_order == X.LSBFirst else 'big'
                pixels = memoryview(image.data)
                glyph_pixels = 0
                for y in range(8, min(geometry.height - 2, 24)):
                    for x in range(8, 220):
                        offset = (y * geometry.width + x) * 4
                        if int.from_bytes(pixels[offset:offset + 4], byteorder) & 0xffffff == foreground:
                            glyph_pixels += 1
                snapshot = {
                    'geometry': [geometry.x, geometry.y, geometry.width, geometry.height, geometry.depth],
                    'pixels_sha256': hashlib.sha256(image.data).hexdigest(),
                    'prompt_foreground_pixels': glyph_pixels,
                }
                if glyph_pixels > 5 and snapshot == previous:
                    return window, snapshot, image.data
                previous = snapshot if glyph_pixels > 5 else None
        connection.sync()
        time.sleep(.04)
    raise AssertionError('nagbar never completed a stable, nonblank text render')


def invoke(binary, fixture, connection, environment, server, work, label):
    name, arguments, foreground = fixture
    _input.require_private(connection, environment, server)
    assert not _input.mapped_windows(connection.screen().root), 'previous fixture left a mapped window'
    stem = work / f'{label}-{name}'
    child = None
    try:
        with stem.with_suffix('.stdout').open('wb') as stdout, stem.with_suffix('.stderr').open('wb') as stderr:
            child = subprocess.Popen([str(binary), '-f', 'fixed', *arguments], cwd=work,
                                     env=environment, stdout=stdout, stderr=stderr)
            window, snapshot, pixels = rendered_window(connection, child, foreground)
            stem.with_suffix('.pixels').write_bytes(pixels)
            # This is a root child on our WM-free private server. The rightmost
            # close interior is disjoint from every action button in these cases.
            x, y, width, height, _ = snapshot['geometry']
            assert window.id in [item.id for item in connection.screen().root.query_tree().children]
            _input.require_private(connection, environment, server)
            xtest.fake_input(connection, X.MotionNotify, x=x + width - 5, y=y + height // 2)
            xtest.fake_input(connection, X.ButtonPress, 1)
            xtest.fake_input(connection, X.ButtonRelease, 1)
            connection.sync()
            status = child.wait(timeout=TIMEOUT)
            assert status == 0, (name, 'close status', status)
            snapshot['status'] = status
            deadline = time.monotonic() + 3
            while time.monotonic() < deadline:
                connection.sync()
                if not _input.mapped_windows(connection.screen().root):
                    return snapshot
                time.sleep(.02)
            raise AssertionError('nagbar window survived normal close/session cleanup')
    finally:
        _input.stop(child)
        connection.sync()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference', type=Path, help='Optional external C i3-nagbar')
    candidate = parser.add_mutually_exclusive_group()
    candidate.add_argument('--candidate', type=Path)
    candidate.add_argument('--rust-bin-dir', type=Path)
    parser.add_argument('--xvfb', type=Path, default=shutil.which('Xvfb'))
    args = parser.parse_args()
    if args.xvfb is None:
        parser.error('Pass --xvfb /path/to/Xvfb; no desktop fallback is permitted')
    binary = (args.candidate or (args.rust_bin_dir or _input.cargo_bin_dir()) / 'i3-nagbar').resolve()
    binaries = [('candidate', binary)]
    if args.reference:
        binaries.insert(0, ('reference', args.reference.resolve()))
    for path in [args.xvfb, *[path for _, path in binaries]]:
        if not path.is_file() or not os.access(path, os.X_OK):
            parser.error(f'Missing executable: {path}')
    work = Path(make_tempdir(prefix='nagbar-'))
    results = {}
    try:
        with _input.private_server(args.xvfb.resolve(), work) as (connection, environment, server):
            for label, executable in binaries:
                results[label] = {case[0]: invoke(executable, case, connection, environment, server, work, label)
                                  for case in CASES}
            if args.reference:
                assert results['candidate'] == results['reference'], results
        print(f'PASS: {len(CASES)} private nagbar render/close cases' +
              (' match external C' if args.reference else ' satisfy fixtures'))
    finally:
        (work / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
        print(f'Private nagbar artifacts: {work}')


if __name__ == '__main__':
    main()
