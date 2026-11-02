#!/usr/bin/env python3
"""Run the maintained-XCB probe on an owned private Xvfb, never the desktop."""
import argparse
import os
from pathlib import Path
import select
import subprocess
from regression_environment import find_xserver
from workspace_temp import make_tempdir

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--probe', type=Path, required=True)
args = parser.parse_args()
work = Path(make_tempdir(prefix='i333-maintained-xcb-'))
env = dict(os.environ)
for key in ('DISPLAY', 'I3SOCK', 'WAYLAND_DISPLAY', 'XAUTHORITY',
            'DBUS_SESSION_BUS_ADDRESS', 'NOTIFY_SOCKET', 'LISTEN_PID',
            'LISTEN_FDS', 'LISTEN_FDNAMES'):
    env.pop(key, None)
read_fd, write_fd = os.pipe()
server = None
try:
    with (work / 'xvfb.log').open('wb') as log:
        server = subprocess.Popen([str(find_xserver('Xvfb')), '-displayfd', str(write_fd),
                                   '-screen', '0', '160x120x24', '-nolisten', 'tcp',
                                   '-ac', '-noreset'], env=env, pass_fds=(write_fd,),
                                  stdout=log, stderr=subprocess.STDOUT)
        os.close(write_fd)
        write_fd = None
        if not select.select([read_fd], [], [], 10)[0]:
            raise RuntimeError('private Xvfb startup timed out')
        number = os.read(read_fd, 32).decode().strip()
        if not number.isdecimal():
            raise RuntimeError('private Xvfb failed to allocate a display')
        env.update(DISPLAY=':' + number, I333_MAINTAINED_XCB_PROBE='1')
        subprocess.run([str(args.probe.resolve()), '--exact',
                        'maintained_xcb_requests_and_borrowed_lifetime', '--nocapture'],
                       env=env, check=True, timeout=20)
        print('Maintained XCB property/background/screenshot/lifetime checks passed on private :' + number)
finally:
    os.close(read_fd)
    if write_fd is not None:
        os.close(write_fd)
    if server is not None and server.poll() is None:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()
            server.wait()
