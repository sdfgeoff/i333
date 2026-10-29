"""Build standalone oracle probes using their normal Cargo integration targets.

Cargo test supplies --probe to the Python oracles, so that path never starts a
nested Cargo invocation or waits on its parent's target lock.
"""
import json
from pathlib import Path
import subprocess
import sys
from workspace_temp import temporary_root

ROOT = Path(__file__).resolve().parents[1]


def build_probe(package, target, offline=False, toolchain='nightly'):
    temporary_root()
    command = ['cargo', f'+{toolchain}', 'build', '--package', package,
               '--test', target, '--message-format=json-render-diagnostics']
    if offline:
        command.append('--offline')
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if result.returncode:
        print(result.stdout, end='', file=sys.stderr)
        print(result.stderr, end='', file=sys.stderr)
        raise SystemExit(result.returncode)
    for line in result.stdout.splitlines():
        message = json.loads(line)
        if (message.get('reason') == 'compiler-artifact'
                and message.get('target', {}).get('name') == target
                and message.get('executable')):
            return Path(message['executable'])
    raise RuntimeError(f'Cargo did not report an executable for {package}/{target}')
