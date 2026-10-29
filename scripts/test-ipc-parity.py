#!/usr/bin/env python3
"""Check Rust i3-msg using private Unix sockets, never a desktop session.

Run after cargo build:
    python scripts/test-ipc-parity.py
Use --release after cargo build --release, or pass --rust-bin-dir/--candidate.
For optional differential checking, supply an external C client with --reference.

Each subprocess gets a fresh environment without DISPLAY or I3SOCK, an explicit
socket path, and a bounded timeout. Server failures propagate to the test runner.
"""
import argparse
import os
from pathlib import Path
import socket
import struct
import subprocess
from workspace_temp import temporary_directory
import threading
import unittest

ROOT = Path(__file__).resolve().parent.parent
TIMEOUT = 5
C_BIN = None
RUST_BIN = ROOT / "target" / "debug" / "i3-msg"
ENV = {key: value for key, value in os.environ.items() if key not in {"DISPLAY", "I3SOCK", "WAYLAND_DISPLAY"}}


def cargo_bin_dir(release=False):
    """Use Cargo's normal target directory; explicit CLI paths can override it."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", "target"))
    if not target.is_absolute():
        target = ROOT / target
    return target / ("release" if release else "debug")


def frame(kind, body):
    return struct.pack("=6sII", b"i3-ipc", len(body), kind) + body


def receive_exact(connection, size):
    result = bytearray()
    while len(result) < size:
        chunk = connection.recv(size - len(result))
        if not chunk:
            raise AssertionError(f"client closed after {len(result)} of {size} bytes")
        result.extend(chunk)
    return bytes(result)


def normalize(result, executable):
    # err(3) includes the executable name; fixture paths also vary by invocation.
    return (
        result.returncode,
        result.stdout,
        result.stderr.replace(str(executable).encode(), b"i3-msg").replace(executable.name.encode(), b"i3-msg"),
    )


def invoke(executable, arguments, expected_request=None, response=b"", fragmented=False):
    """Return exit status/stdout/stderr and propagate private server exceptions."""
    if expected_request is None:
        result = subprocess.run([str(executable), *arguments], env=ENV, capture_output=True, timeout=TIMEOUT)
        return normalize(result, executable)

    with temporary_directory(prefix="i333-ipc-") as directory:
        path = directory + "/ipc"
        errors = []
        listener = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        listener.bind(path)
        listener.listen(1)
        listener.settimeout(TIMEOUT)

        def serve():
            try:
                connection, _ = listener.accept()
                with connection:
                    connection.settimeout(TIMEOUT)
                    magic, length, kind = struct.unpack("=6sII", receive_exact(connection, 14))
                    payload = receive_exact(connection, length)
                    if (magic, kind, payload) != (b"i3-ipc", *expected_request):
                        raise AssertionError(f"unexpected request: {(magic, kind, payload)!r}")
                    if fragmented:
                        # Very small writes exercise framing across arbitrary stream boundaries.
                        for offset in range(0, len(response), 3):
                            connection.sendall(response[offset:offset + 3])
                    else:
                        connection.sendall(response)
            except Exception as error:
                errors.append(error)
            finally:
                listener.close()

        worker = threading.Thread(target=serve, daemon=True)
        worker.start()
        try:
            result = subprocess.run([str(executable), "--socket", path, *arguments], env=ENV, capture_output=True, timeout=TIMEOUT)
        finally:
            worker.join(TIMEOUT + 1)
        if worker.is_alive():
            raise AssertionError("private IPC fixture did not terminate")
        if errors:
            raise errors[0]
        return normalize(result, executable)


class IpcParity(unittest.TestCase):
    def parity(self, arguments, expected_request=None, response=b"", fragmented=False, expected_code=0):
        translated = invoke(RUST_BIN, arguments, expected_request, response, fragmented)
        if C_BIN is not None:
            reference = invoke(C_BIN, arguments, expected_request, response, fragmented)
            self.assertEqual(reference, translated)
        self.assertEqual(translated[0], expected_code)
        return translated

    def test_cli_without_connection(self):
        for args, status in [(["--help"], 0), (["--version"], 0), (["-m"], 1),
                             (["--type", "unknown"], 1), (["--unknown"], 1),
                             (["--socket"], 1), (["--type"], 1)]:
            with self.subTest(args=args):
                self.parity(args, expected_code=status)

    def test_command_payload_and_success(self):
        success = frame(0, b'[{"success":true}]')
        for args, payload in [([], b""), (["mark", "hello"], b"mark hello"),
                              (["exec", "echo", "caf\u00e9"], "exec echo caf\u00e9".encode()),
                              (["--", "-strange"], b"-strange"),
                              (["--type=COMMAND", "nop"], b"nop"),
                              (["-trun_command", "nop"], b"nop")]:
            with self.subTest(args=args):
                self.parity(args, (0, payload), success, fragmented=True)

    def test_command_failure_and_raw_quiet(self):
        failure = b'[{"success":false,"error":"bad command","input":"bad","errorposition":"^^^"}]'
        for options, code, output in [([], 2, failure + b"\n"), (["-q"], 2, b""),
                                      (["-r"], 0, failure + b"\n"), (["-qr"], 0, failure + b"\n")]:
            with self.subTest(options=options):
                result = self.parity([*options, "bad"], (0, b"bad"), frame(0, failure), expected_code=code)
                self.assertEqual(result[1], output)

    def test_multiple_command_replies(self):
        reply = b'[{"success":true},{"success":false,"error":"failed"},{"success":true}]'
        self.parity(["nop; bad; nop"], (0, b"nop; bad; nop"), frame(0, reply), expected_code=2)

    def test_query_types_and_quiet(self):
        for name, kind in [("get_workspaces", 1), ("get_outputs", 3), ("get_tree", 4),
                           ("get_marks", 5), ("get_bar_config", 6), ("get_version", 7),
                           ("get_binding_modes", 8), ("send_tick", 10), ("get_binding_state", 12)]:
            for quiet in [False, True]:
                with self.subTest(name=name, quiet=quiet):
                    args = ["--type", name] + (["--quiet"] if quiet else [])
                    result = self.parity(args, (kind, b""), frame(kind, b"[]"), fragmented=True)
                    self.assertEqual(result[1], b"" if quiet else b"[]\n")

    def test_config_extraction(self):
        reply = b'{"config":"set $mod Mod4\\n# Unicode: \\u2603\\n"}'
        for options, output in [([], "set $mod Mod4\n# Unicode: \u2603\n".encode()),
                                (["-q"], "set $mod Mod4\n# Unicode: \u2603\n".encode()),
                                (["-r"], reply + b"\n"), (["-qr"], reply + b"\n")]:
            with self.subTest(options=options):
                result = self.parity(["-t", "get_config", *options], (9, b""), frame(9, reply))
                self.assertEqual(result[1], output)

    def test_subscribe_one_event(self):
        event = b'{"change":"new"}'
        reply = frame(2, b'{"success":true}') + frame(0x80000003, event)
        for options in [[], ["-q"], ["-r"], ["-qr"]]:
            with self.subTest(options=options):
                result = self.parity(["-t", "subscribe", *options, '["window"]'], (2, b'["window"]'), reply)
                self.assertEqual(result[1], b"" if "q" in "".join(options) else event + b"\n")

    def test_subscribe_monitor_reads_all_events(self):
        event1, event2 = b'{"first":true}', b'{"first":false}'
        reply = frame(2, b'{"success":true}') + frame(0x80000007, event1) + frame(0x80000007, event2)
        for options in [["-m"], ["-qm"]]:
            with self.subTest(options=options):
                result = self.parity(["-t", "subscribe", *options, '["tick"]'], (2, b'["tick"]'), reply, expected_code=1)
                self.assertEqual(result[1], b"" if "q" in options[0] else event1 + b"\n" + event2 + b"\n")

    def test_wrong_reply_type(self):
        self.parity(["-t", "get_tree"], (4, b""), frame(3, b"[]"), expected_code=1)

    def test_subscribe_requires_event_type(self):
        response = frame(2, b'{"success":true}') + frame(0, b"[]")
        self.parity(["-t", "subscribe", '["window"]'], (2, b'["window"]'), response, expected_code=1)

    def test_invalid_json_command_and_config(self):
        for args, kind in [([], 0), (["-t", "get_config"], 9)]:
            with self.subTest(kind=kind):
                self.parity(args, (kind, b""), frame(kind, b"not-json"), expected_code=1)
        self.parity(["-r"], (0, b""), frame(0, b"not-json"))

    def test_empty_payload(self):
        result = self.parity(["-t", "get_tree"], (4, b""), frame(4, b""))
        self.assertEqual(result[1], b"\n")

    def test_invalid_magic(self):
        self.parity([], (0, b""), b"wrong!" + struct.pack("=II", 0, 0), expected_code=1)

    def test_truncated_header(self):
        self.parity([], (0, b""), b"i3-ipc\x02", expected_code=1)

    def test_truncated_payload(self):
        self.parity([], (0, b""), struct.pack("=6sII", b"i3-ipc", 5, 0) + b"[]", expected_code=1)

    def test_eof_before_header(self):
        self.parity([], (0, b""), b"", expected_code=1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", "--c-bin", dest="c_bin", type=Path,
                        help="Optional external C i3-msg executable for differential checking")
    candidates = parser.add_mutually_exclusive_group()
    candidates.add_argument("--rust-bin", "--candidate", type=Path,
                            help="Explicit Rust i3-msg executable")
    candidates.add_argument("--rust-bin-dir", type=Path,
                            help="Directory containing Cargo-built products")
    parser.add_argument("--release", action="store_true", help="Use Cargo target/release instead of target/debug")
    options, remaining = parser.parse_known_args()
    directory = options.rust_bin_dir or cargo_bin_dir(options.release)
    C_BIN = options.c_bin.resolve() if options.c_bin else None
    RUST_BIN = (options.rust_bin or directory / "i3-msg").resolve()
    for executable in [path for path in [C_BIN, RUST_BIN] if path is not None]:
        if not executable.is_file():
            parser.error(f"missing executable {executable}; run cargo build, or provide an existing external reference")
    unittest.main(argv=[__file__, *remaining])
