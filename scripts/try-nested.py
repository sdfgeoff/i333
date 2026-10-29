#!/usr/bin/env python3
"""Launch Cargo-built Rust i3 inside Xephyr, leaving the host WM untouched.

Run cargo build first, or cargo build --release and pass --release.
Use --candidate for an executable or --rust-bin-dir for a product directory.
"""
import argparse
import os
from pathlib import Path
import select
import shutil
import subprocess
from workspace_temp import temporary_directory

ROOT = Path(__file__).resolve().parents[1]


def cargo_bin_dir(release=False):
    """Use Cargo's normal target directory; explicit CLI paths can override it."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", "target"))
    if not target.is_absolute():
        target = ROOT / target
    return target / ("release" if release else "debug")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--xephyr', default=shutil.which('Xephyr'), help='Xephyr executable (default: search PATH)')
    candidates = parser.add_mutually_exclusive_group()
    candidates.add_argument('--candidate', type=Path, help='Explicit Rust i3 executable')
    candidates.add_argument('--rust-bin-dir', type=Path, help='Directory containing Cargo-built products')
    parser.add_argument('--release', action='store_true', help='Use Cargo target/release instead of target/debug')
    args = parser.parse_args()
    directory = args.rust_bin_dir or cargo_bin_dir(args.release)
    binary = (args.candidate or directory / 'i3').resolve()
    terminal = shutil.which('alacritty')
    if not binary.is_file() or not args.xephyr or not Path(args.xephyr).is_file() or not terminal:
        parser.error(f'Requires Cargo-built Rust i3 ({binary}), Xephyr and alacritty; run cargo build first')
    if not os.environ.get('DISPLAY'):
        parser.error('Run this from a terminal in your desktop X session')
    with temporary_directory(prefix='i333-nested-') as temporary:
        work = Path(temporary)
        config = work / 'config'
        config.write_text(f'''font pango:monospace 8
ipc-socket {work}/ipc.sock
focus_follows_mouse no
mouse_warping none
default_border pixel 2
set $mod Mod1
exec --no-startup-id {terminal}
bindsym $mod+Return exec {terminal}
bindsym $mod+Shift+q kill
bindsym $mod+h split h
bindsym $mod+v split v
bindsym $mod+f fullscreen toggle
bindsym $mod+s layout stacking
bindsym $mod+w layout tabbed
bindsym $mod+e layout toggle split
bindsym $mod+Shift+space floating toggle
bindsym $mod+Left focus left
bindsym $mod+Right focus right
bindsym $mod+Up focus up
bindsym $mod+Down focus down
'''+''.join(f'bindsym $mod+{i} workspace {i}\nbindsym $mod+Shift+{i} move container to workspace {i}\n' for i in range(1, 5)))
        read_fd, write_fd = os.pipe()
        server = None
        wm = None
        try:
            server = subprocess.Popen([args.xephyr, '-displayfd', str(write_fd),
                                       '-screen', '1280x800', '-nolisten', 'tcp',
                                       '-no-host-grab', '-ac', '-noreset'], pass_fds=(write_fd,))
            os.close(write_fd)
            write_fd = None
            if not select.select([read_fd], [], [], 10)[0]:
                raise RuntimeError('Xephyr startup timed out')
            number = os.read(read_fd, 32).decode().strip()
            if not number.isdecimal():
                raise RuntimeError('Xephyr failed to allocate a private display')
            nested_display = ':' + number
            if nested_display == os.environ['DISPLAY'].split('.')[0]:
                raise RuntimeError('Refusing to use the host display')
            env = {key: value for key, value in os.environ.items()
                   if key not in ('I3SOCK', 'WAYLAND_DISPLAY', 'XAUTHORITY',
                                  'DBUS_SESSION_BUS_ADDRESS', 'DESKTOP_STARTUP_ID')}
            env.update(DISPLAY=nested_display, I3SOCK=str(work / 'ipc.sock'),
                       XDG_CURRENT_DESKTOP='i3', WINIT_UNIX_BACKEND='x11')
            wm = subprocess.Popen([str(binary), '-a', '-c', str(config)], env=env, cwd=work)
            print(f'Rust i3 is inside Xephyr on {nested_display}. Alt+Enter opens a terminal.\n'
                  'Close the Xephyr window or press Ctrl+C here to end the nested session.', flush=True)
            while server.poll() is None:
                try:
                    wm.wait(timeout=0.5)
                    break
                except subprocess.TimeoutExpired:
                    pass
        except KeyboardInterrupt:
            pass
        finally:
            os.close(read_fd)
            if write_fd is not None:
                os.close(write_fd)
            # Only the two processes created here are signalled.
            for child in (wm, server):
                if child is not None and child.poll() is None:
                    child.terminate()
                    try:
                        child.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        child.kill()
                        child.wait()


if __name__ == '__main__':
    main()
