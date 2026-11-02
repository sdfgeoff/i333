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
import signal
import socket
import struct
import subprocess
from workspace_temp import temporary_directory, make_tempdir
import time

from Xlib import X, XK, display
from Xlib.protocol import event as wire_event

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


def private_environment(work):
    env = {key: value for key, value in os.environ.items()
           if key not in ("DISPLAY", "I3SOCK", "WAYLAND_DISPLAY", "XAUTHORITY",
                          "DBUS_SESSION_BUS_ADDRESS", "NOTIFY_SOCKET", "LISTEN_PID",
                          "LISTEN_FDS", "LISTEN_FDNAMES")}
    env.update(HOME=str(work), XDG_CONFIG_HOME=str(work), XDG_RUNTIME_DIR=str(work))
    return env


@contextlib.contextmanager
def session(binary, xvfb, *, force_xinerama=False, disable_randr15=False, with_process=False,
            font="pango:monospace 8"):
    with temporary_directory(prefix="i333-headless-") as temporary:
        work = Path(temporary)
        env = private_environment(work)
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
                config.write_text(f"""font {font}
ipc-socket {ipc}
focus_follows_mouse no
mouse_warping none
workspace_auto_back_and_forth no
default_border pixel 2
gaps inner 5
gaps outer 3
""" + ("force_xinerama yes\n" if force_xinerama else ""))
                arguments = [str(binary), "-a", "-c", str(config)]
                if disable_randr15:
                    arguments.append("--disable-randr15")
                wm = subprocess.Popen(arguments, env=env,
                                      cwd=work, stdout=log, stderr=log)
                wait_for(lambda: ipc.exists() or wm.poll() is not None, "WM startup")
                if wm.poll() is not None:
                    raise RuntimeError("WM exited: " + (work / "wm.log").read_text())
                connection = display.Display(env["DISPLAY"])
                yield (connection, ipc, wm) if with_process else (connection, ipc)
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


def exercise_output_backend(binary, xvfb, *, force_xinerama=False, disable_randr15=False):
    """Exercise each fallback backend on its own private server."""
    assert force_xinerama != disable_randr15
    backend = "xinerama" if force_xinerama else "randr14"
    snapshots = []
    with session(binary, xvfb, force_xinerama=force_xinerama,
                 disable_randr15=disable_randr15) as (_, ipc):
        def snapshot(label):
            outputs = request(ipc, 3)
            active = [output for output in outputs if output["active"]]
            assert len(active) == 1, outputs
            output = active[0]
            if force_xinerama:
                assert output["name"] == "xinerama-0", outputs
            assert output["rect"] == {"x": 0, "y": 0, "width": 1280, "height": 800}, outputs
            snapshots.append((label, {
                "outputs": outputs,
                "tree": normalize(request(ipc, 4)),
            }))

        snapshot(f"{backend}-startup")
        result = request(ipc, 0, f'workspace "7: {backend}"')
        assert result and all(item.get("success") for item in result), result
        snapshot(f"{backend}-workspace-switch")
    return snapshots


def has_window(node, window):
    return node.get("window") == window or any(
        has_window(child, window)
        for key in ("nodes", "floating_nodes") for child in node.get(key, []))


def exercise_lifecycle(binary, xvfb):
    """Probe WM ownership/recovery using only captured private child handles."""
    snapshots = []
    with session(binary, xvfb) as (connection, ipc):
        # Socket creation precedes startup completion. A round trip proves the
        # signal handler, keymap and event dispatcher have all been installed.
        assert request(ipc, 7)["human_readable"]
        work = ipc.parent
        env = private_environment(work)
        env["DISPLAY"] = connection.get_display_name()
        root = connection.screen().root
        client = root.create_window(40, 50, 240, 180, 0, connection.screen().root_depth)
        client.set_wm_class("replacement-client", "ReplacementClient")
        client.map()
        connection.sync()
        wait_for(lambda: has_window(request(ipc, 4), client.id), "initial private client")
        selection = connection.intern_atom("WM_S" + str(connection.get_default_screen()))
        owner = connection.get_selection_owner(selection)
        assert owner is not None
        wm_class = owner.get_full_property(connection.intern_atom("WM_CLASS"), X.AnyPropertyType)
        assert wm_class is not None
        assert bytes(wm_class.value) == b"i3-WM_Sn\0i3-WM_Sn\0"
        snapshots.append(("selection-owner-class", list(wm_class.value)))

        contender_ipc = work / "contender.sock"
        config = work / "contender.config"
        config.write_text((work / "config").read_text().replace(str(ipc), str(contender_ipc)))
        rejected = subprocess.run([str(binary), "-a", "-c", str(config)], cwd=work, env=env,
                                  stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=8)
        assert rejected.returncode == 1, rejected.stdout.decode(errors="replace")
        assert b"Another window manager is already running (WM_Sn is owned)" in rejected.stdout
        refusal = {
            "exit_code": rejected.returncode,
            "owner_retained": connection.get_selection_owner(selection).id == owner.id,
            "client_retained": has_window(request(ipc, 4), client.id),
        }
        assert refusal["owner_retained"] and refusal["client_retained"]
        snapshots.append(("selection-owner-refusal", refusal))
        replacement = None
        with (work / "replacement.log").open("wb") as log:
            try:
                replacement = subprocess.Popen([str(binary), "-a", "--replace", "-c", str(config)],
                                               cwd=work, env=env, stdout=log, stderr=log)

                def replacement_ready():
                    if replacement.poll() is not None:
                        raise AssertionError((work / "replacement.log").read_text(errors="replace"))
                    try:
                        return has_window(request(contender_ipc, 4), client.id)
                    except (OSError, RuntimeError):
                        return False

                wait_for(replacement_ready, "replacement WM to retain its private client")
                client.get_geometry()
                assert connection.get_selection_owner(selection).id != owner.id
                assert request(contender_ipc, 7)["human_readable"]
                snapshots.append(("selection-owner-replacement", normalize(request(contender_ipc, 4))))
                client.destroy()
                connection.sync()
            finally:
                # Only the Popen child created above is signalled.
                if replacement is not None and replacement.poll() is None:
                    replacement.terminate()
                    try:
                        replacement.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        replacement.kill()
                        replacement.wait()

    # A plain fixed font selects the actual XCB backend; it exercises native GC,
    # glyph drawing and width requests that Pango does not use.
    for backend, font in [("pango", "pango:monospace 8"), ("xcb", "fixed")]:
        with session(binary, xvfb, with_process=True, font=font) as (connection, ipc, wm):
            assert request(ipc, 7)["human_readable"]
            root = connection.screen().root
            initial = {window.id for window in root.query_tree().children}

            def dialogs():
                return [window for window in root.query_tree().children
                        if window.id not in initial and window.get_attributes().override_redirect
                        and window.get_attributes().map_state == X.IsViewable]

            # Signal only the Popen handle returned by this private session. Never
            # discover a process via desktop state, server atoms, or a process scan.
            wm.send_signal(signal.SIGABRT)
            wait_for(lambda: bool(dialogs()), "private crash dialog")
            dialog = dialogs()[0]
            dialog.send_event(wire_event.Expose(window=dialog, x=0, y=0, width=1, height=1, count=0),
                              event_mask=0)
            keycode = connection.keysym_to_keycode(XK.string_to_keysym("r"))
            assert keycode
            dialog.send_event(wire_event.KeyPress(time=X.CurrentTime, root=root, window=dialog,
                                                 child=X.NONE, root_x=0, root_y=0, event_x=0, event_y=0,
                                                 state=0, same_screen=1, detail=keycode), event_mask=0)
            connection.sync()
            wait_for(lambda: all(window.id != dialog.id for window in root.query_tree().children),
                     "restart from private crash dialog")

            def restarted_ipc_ready():
                try:
                    return bool(request(ipc, 7)["human_readable"])
                except (OSError, RuntimeError):
                    return False

            wait_for(restarted_ipc_ready, "restarted private IPC socket")
            assert wm.poll() is None
            reply = request(ipc, 0, "nop after-crash-restart")
            assert reply and all(item.get("success") for item in reply)
            snapshots.append((f"crash-dialog-{backend}-restart", {
                "wm_running": wm.poll() is None,
                "ipc_command_succeeded": all(item.get("success") for item in reply),
                "crash_dialog_removed": all(window.id != dialog.id for window in root.query_tree().children),
            }))
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
    def exercise_all(binary):
        return (exercise(binary, args.xvfb)
                + exercise_output_backend(binary, args.xvfb, force_xinerama=True)
                + exercise_output_backend(binary, args.xvfb, disable_randr15=True)
                + exercise_lifecycle(binary, args.xvfb))

    candidate = exercise_all(args.candidate.resolve())
    if args.reference is None:
        print(f"PASS: {len(candidate)} headless Rust state checkpoints")
        return
    reference = exercise_all(args.reference.resolve())
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
