#!/usr/bin/env python3
"""Check core/bar CLI behavior without connecting to a desktop X server.

Run after cargo build. --reference-dir optionally compares an external C build;
only executable paths, build version stamps and logger timestamps normalize.
Configuration validation uses explicit temporary files and private HOME/runtime
paths. DISPLAY/I3SOCK/WAYLAND_DISPLAY and desktop bus credentials are removed.
"""
import argparse
import os
from pathlib import Path
import re
import subprocess
from workspace_temp import temporary_directory

ROOT = Path(__file__).resolve().parent.parent


# Config fixtures run through the actual Cargo-built i3, rather than the external
# C parser helper used by part of the upstream suite. Byte strings preserve native
# quoting, malformed input and continuation rules.
CONFIG_FIXTURES = [
    ('variables', b'set $A 1\nset $a 2\nset $abc 9\ngaps inner $abc\ngaps outer $A\n', 0),
    ('nonrecursive', b'set $one $two\nset $two 3\ngaps inner $one\n', 1),
    ('quoted-mode', b'mode "resizer" {\n bindsym h resize shrink width 10 px or 10 ppt\n}\n', 0),
    ('include', b'include ./child.conf\ninclude ./child.conf\nworkspace "one" output "HDMI-1" "DP-1"\n', 0),
    ('bar', b'bar {\n status_command echo "one,two;three"\n bindsym button1 nop "hello"\n position top\n}\n', 0),
    ('continuation', b'# comment continuation \\\nfont pango:monospace 8\nset $modifier Mod4\nbindsym $modifier+Return exec \\\n echo hello\n', 0),
    ('first-line-error', b'invalid_directive\nfont pango:monospace 8\n', 1),
    ('block-recovery', b'bar {\n invalid_directive\n position top\n}\n', 1),
    ('binary-error', b'\xffinvalid\n', 1),
]


def candidate_directory(release):
    target = Path(os.environ.get('CARGO_TARGET_DIR', 'target'))
    if not target.is_absolute():
        target = ROOT / target
    return target / ('release' if release else 'debug')


def normalize(result, executable):
    output = []
    for data in (result.stdout, result.stderr):
        data = data.replace(str(executable).encode(), executable.name.encode())
        data = re.sub(rb'(?m)^(i3(?:bar)? version )(.+?)( \xc2\xa9 [^\n]*)$', rb'\1<VERSION>\3', data)
        data = re.sub(rb'(?m)^\d{2}/\d{2}/\d{2} \d{2}:\d{2}:\d{2} - ', b'<TIME> - ', data)
        output.append(data)
    return result.returncode, *output


def invoke(executable, arguments, env):
    return normalize(subprocess.run([str(executable), *arguments], env=env,
                                   capture_output=True, timeout=10), executable)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate-dir', '--rust-bin-dir', dest='candidate_dir', type=Path)
    parser.add_argument('--reference-dir', type=Path)
    parser.add_argument('--release', action='store_true')
    args = parser.parse_args()
    candidates = (args.candidate_dir or candidate_directory(args.release)).resolve()
    reference = args.reference_dir.resolve() if args.reference_dir else None
    for directory in [path for path in (candidates, reference) if path is not None]:
        for name in ('i3', 'i3bar'):
            if not os.access(directory / name, os.X_OK):
                parser.error(f'missing executable {directory / name}')
    env = {key: value for key, value in os.environ.items() if key not in {
        'DISPLAY', 'I3SOCK', 'WAYLAND_DISPLAY', 'XAUTHORITY', 'DBUS_SESSION_BUS_ADDRESS',
        'DESKTOP_STARTUP_ID', 'POSIXLY_CORRECT',
        'NOTIFY_SOCKET', 'LISTEN_PID', 'LISTEN_FDS', 'LISTEN_FDNAMES',
    }}
    env['LC_ALL'] = 'C'
    checked = 0
    with temporary_directory(prefix='i333-main-cli-') as temporary:
        work = Path(temporary)
        env.update(HOME=temporary, XDG_CONFIG_HOME=temporary, XDG_RUNTIME_DIR=temporary)
        valid = work / 'valid.conf'
        valid.write_text('font pango:monospace 8\nset $mod Mod4\n')
        invalid = work / 'invalid.conf'
        invalid.write_text('this_is_not_an_i3_configuration_directive\n')
        cases = {
            'i3': [
                (['--help'], 0), (['-h'], 0), (['-v'], 0), (['--unknown'], 1),
                (['-c'], 1), (['--conf'], 1), (['-l', '-h'], 0), (['-a', '-h'], 0),
                (['--disable', '-h'], 0), (['--force', '-h'], 0),
                (['--shmlog-size=0', '-h'], 0), (['--shmlog_size', '0', '-h'], 0),
                (['-C', '-c', str(valid)], 0), (['-C', '--config=' + str(valid)], 0),
                (['-V', '-C', '-c', str(valid)], 0),
                (['-C', '-c', str(invalid)], 1),
                (['--no-autostart', '--disable-randr15', '-C', '-c', str(valid)], 0),
                (['--disable_randr15', '--disable-signalhandler', '-C', '-c', str(valid)], 0),
                (['--fake-outputs=1280x800+0+0', '-C', '-c', str(valid)], 0),
            ],
            'i3bar': [
                (['--help'], 1), (['-h'], 1), (['-v'], 0), (['--ver'], 1),
                (['--unknown'], 1), (['--socket'], 1), (['--bar_id'], 1),
                (['--bar_id=bar-1', '-t', '-h'], 1),
                (['-bbar-1', '--transparency', '--help'], 1),
                ([f'--socket={work / "unused.sock"}', '--help'], 1),
                (['--verbose', '--version'], 0),
            ],
        }
        (work / 'child.conf').write_bytes(b'set $child 4\ngaps inner $child\n')
        for name, contents, status in CONFIG_FIXTURES:
            fixture = work / (name + '.conf')
            fixture.write_bytes(contents)
            cases['i3'].append((['-C', '-c', str(fixture)], status))
        for name, entries in cases.items():
            for arguments, status in entries:
                actual = invoke(candidates / name, arguments, env)
                assert actual[0] == status, (name, arguments, status, actual)
                if reference:
                    expected = invoke(reference / name, arguments, env)
                    assert actual == expected, (name, arguments, expected, actual)
                checked += 1
    print(f'PASS: {checked} core/bar CLI cases, including configuration-only validation; no X display')


if __name__ == '__main__':
    main()
