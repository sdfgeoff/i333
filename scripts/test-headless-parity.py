#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["python-xlib==0.33"]
# ///
"""Exercise Rust WM behavior on private Xvfb servers; never use desktop DISPLAY.

Rust defaults to Cargo target/debug/i3; pass --release for target/release/i3.
For optional differential checking, supply an external C WM with --reference.
Override the Rust executable with --candidate, or pass --rust-bin-dir.
"""
import argparse
import contextlib
import json
import os
from pathlib import Path
import select
import shutil
import socket
import struct
import subprocess
from workspace_temp import temporary_directory, make_tempdir
import time

from Xlib import X, display

ROOT = Path(__file__).resolve().parents[1]


def cargo_bin_dir(release=False):
    """Use Cargo's normal target directory; explicit CLI paths can override it."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", "target"))
    if not target.is_absolute():
        target = ROOT / target
    return target / ("release" if release else "debug")


def receive(sock, length):
    data = bytearray()
    while len(data) < length:
        chunk = sock.recv(length - len(data))
        if not chunk:
            raise RuntimeError("Unexpected IPC EOF")
        data.extend(chunk)
    return bytes(data)


def request(path, kind, payload=""):
    with socket.socket(socket.AF_UNIX) as sock:
        sock.settimeout(5)
        sock.connect(str(path))
        payload = payload.encode()
        sock.sendall(b"i3-ipc" + struct.pack("=II", len(payload), kind) + payload)
        header = receive(sock, 14)
        assert header[:6] == b"i3-ipc"
        length, reply_type = struct.unpack("=II", header[6:])
        assert reply_type == kind
        return json.loads(receive(sock, length))


def wait_for(predicate, description):
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(0.02)
    raise AssertionError(f"Timed out waiting for {description}")


def normalize(node):
    # Container/X resource IDs and focus lists contain process-specific IDs.
    # Express focus as child indices so focus ordering remains comparable.
    children = node.get("nodes", []) + node.get("floating_nodes", [])
    child_ids = [child["id"] for child in children]
    focus = [child_ids.index(i) for i in node.get("focus", []) if i in child_ids]
    keys = ("type", "name", "num", "layout", "orientation", "percent", "rect",
            "window_rect", "deco_rect", "geometry", "border", "current_border_width",
            "focused", "visible", "urgent", "fullscreen_mode", "sticky", "floating",
            "scratchpad_state", "marks", "gaps", "window_properties", "workspace_layout")
    result = {key: node[key] for key in keys if key in node}
    result["focus"] = focus
    result["has_window"] = node.get("window") is not None
    result["nodes"] = [normalize(child) for child in node.get("nodes", [])]
    result["floating_nodes"] = [normalize(child) for child in node.get("floating_nodes", [])]
    return result


def count_windows(node):
    return int(node.get("window") is not None) + sum(
        count_windows(child) for key in ("nodes", "floating_nodes") for child in node.get(key, []))


@contextlib.contextmanager
def session(binary, xvfb):
    with temporary_directory(prefix="i333-headless-") as temporary:
        work = Path(temporary)
        env = {k: v for k, v in os.environ.items()
               if k not in ("DISPLAY", "I3SOCK", "WAYLAND_DISPLAY", "XAUTHORITY",
                            "DBUS_SESSION_BUS_ADDRESS", "NOTIFY_SOCKET", "LISTEN_PID",
                            "LISTEN_FDS", "LISTEN_FDNAMES")}
        env.update(HOME=str(work), XDG_CONFIG_HOME=str(work), XDG_RUNTIME_DIR=str(work))
        pipe_read, pipe_write = os.pipe()
        with (work / "xvfb.log").open("wb") as xlog, (work / "wm.log").open("wb") as log:
            server = subprocess.Popen([xvfb, "-displayfd", str(pipe_write), "-screen", "0",
                                       "1280x800x24", "-nolisten", "tcp", "-noreset"],
                                      env=env, pass_fds=(pipe_write,), stdout=xlog, stderr=xlog)
            os.close(pipe_write)
            wm = None
            connection = None
            try:
                if not select.select([pipe_read], [], [], 8)[0]:
                    raise RuntimeError("Xvfb did not allocate a display")
                number = os.read(pipe_read, 32).decode().strip()
                if not number.isdecimal():
                    raise RuntimeError("Xvfb failed: " + (work / "xvfb.log").read_text())
                env["DISPLAY"] = ":" + number
                ipc = work / "ipc.sock"
                config = work / "config"
                config.write_text(f"""font pango:monospace 8
ipc-socket {ipc}
focus_follows_mouse no
mouse_warping none
workspace_auto_back_and_forth no
default_border pixel 2
gaps inner 5
gaps outer 3
""")
                wm = subprocess.Popen([str(binary), "-a", "-c", str(config)], env=env,
                                      cwd=work, stdout=log, stderr=log)
                wait_for(lambda: ipc.exists() or wm.poll() is not None, "WM startup")
                if wm.poll() is not None:
                    raise RuntimeError("WM exited: " + (work / "wm.log").read_text())
                connection = display.Display(env["DISPLAY"])
                yield connection, ipc
            except BaseException:
                print((work / "wm.log").read_text(errors="replace")[-6000:])
                raise
            finally:
                os.close(pipe_read)
                if connection is not None:
                    connection.close()
                # Only these Popen child handles are signalled. Never pkill/killall.
                for child in (wm, server):
                    if child is not None and child.poll() is None:
                        child.terminate()
                        try:
                            child.wait(timeout=5)
                        except subprocess.TimeoutExpired:
                            child.kill()
                            child.wait()


def exercise(binary, xvfb):
    snapshots = []
    with session(binary, xvfb) as (connection, ipc):
        def snapshot(label):
            tree = request(ipc, 4)
            snapshots.append((label, normalize(tree)))

        def command(text):
            reply = request(ipc, 0, text)
            assert reply and all(item.get("success") for item in reply), (text, reply)
            # Force an X server round trip after each command, then an IPC read.
            connection.sync()
            snapshot(text)

        snapshot("startup")
        windows = []
        for index in range(4):
            window = connection.screen().root.create_window(
                50, 50, 320, 240, 0, connection.screen().root_depth,
                X.InputOutput, X.CopyFromParent, background_pixel=0x222222,
                event_mask=X.StructureNotifyMask)
            window.set_wm_name(f"parity-{index}")
            window.set_wm_class("parity", "Parity")
            window.map()
            connection.sync()
            windows.append(window)
            wait_for(lambda: count_windows(request(ipc, 4)) == index + 1, "mapped window")
            snapshot(f"map-{index}")
        for text in (
            'focus left', 'focus right', 'split v', 'layout stacking',
            'layout tabbed', 'layout splith', 'focus parent', 'layout splitv',
            'focus child', 'resize grow height 20 px or 5 ppt',
            'fullscreen enable', 'fullscreen disable', 'mark example',
            'floating enable', 'resize set 420 260', 'move position 100 100',
            'sticky enable', 'sticky disable', 'floating disable',
            'move container to workspace 2', 'workspace 2', 'split h',
            'workspace 1', '[title="parity-0"] move scratchpad',
            'scratchpad show', 'scratchpad show', 'workspace 2',
            'move container to workspace 1', 'workspace 1', 'layout tabbed',
            'layout stacking', 'layout splith', 'gaps inner current set 12',
            'gaps outer current set 8', 'border normal 3', 'border pixel 1',
            'unmark example', 'reload',
        ):
            command(text)
        # Query public metadata too, excluding IDs from workspace output.
        workspaces = request(ipc, 1)
        snapshots.append(("workspaces", [{k: v for k, v in ws.items() if k != "id"} for ws in workspaces]))
        snapshots.append(("marks", request(ipc, 5)))
        snapshots.append(("binding_modes", request(ipc, 8)))
        snapshots.append(("binding_state", request(ipc, 12)))
        version = {key: value for key, value in request(ipc, 7).items()
                   if key != "loaded_config_file_name"}
        # Only the human-readable Git/build stamp differs for the cached C
        # reference. Numeric major/minor/patch and all other fields stay exact.
        if isinstance(version.get("human_readable"), str):
            version["human_readable"] = "<BUILD>"
        snapshots.append(("version", version))
        for index, window in enumerate(windows):
            window.destroy()
            connection.sync()
            wait_for(lambda: count_windows(request(ipc, 4)) == len(windows) - index - 1,
                     "destroyed window")
            snapshot(f"destroy-{index}")
    return snapshots


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--xvfb", default=shutil.which("Xvfb"))
    parser.add_argument("--reference", type=Path, help="Optional external C i3 executable for differential checking")
    candidates = parser.add_mutually_exclusive_group()
    candidates.add_argument("--candidate", type=Path, help="Explicit Rust i3 executable")
    candidates.add_argument("--rust-bin-dir", type=Path, help="Directory containing Cargo-built products")
    parser.add_argument("--release", action="store_true", help="Use Cargo target/release instead of target/debug")
    args = parser.parse_args()
    directory = args.rust_bin_dir or cargo_bin_dir(args.release)
    args.candidate = (args.candidate or directory / "i3").resolve()
    for binary in [path for path in (args.reference, args.candidate) if path is not None]:
        if not binary.is_file() or not os.access(binary, os.X_OK):
            parser.error(f"Missing executable {binary}; run cargo build, or provide an existing external reference")
    if not args.xvfb:
        parser.error("Pass --xvfb /path/to/Xvfb; no desktop fallback is permitted")
    candidate = exercise(args.candidate.resolve(), args.xvfb)
    if args.reference is None:
        print(f"PASS: {len(candidate)} headless Rust state checkpoints")
        return
    reference = exercise(args.reference.resolve(), args.xvfb)
    for (label, expected), (other_label, actual) in zip(reference, candidate, strict=True):
        assert label == other_label
        if expected != actual:
            artifacts = Path(make_tempdir(prefix="i333-parity-diff-"))
            (artifacts / "expected.json").write_text(json.dumps(expected, indent=2))
            (artifacts / "actual.json").write_text(json.dumps(actual, indent=2))
            raise AssertionError(f"Behavior differs after {label}; see {artifacts}")
    print(f"PASS: {len(reference)} headless C/Rust state comparisons")


if __name__ == "__main__":
    main()
