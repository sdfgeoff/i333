#!/usr/bin/env python3
"""Run the complete upstream suite on private X servers.

cargo test runs this automatically using prepared dependencies. Prepare them
once with python3 scripts/prepare-tests.py; normal tests never download tools or
rebuild the external C reference. Direct invocation accepts --rust-bin-dir,
--reference-build, --perl-lib, --xvfb-dir and --xephyr-dir overrides. --baseline
uses external C products as well as the three native test helpers.

The retained Inline::C fixtures use a persistent .work/test-deps/inline cache;
only new or changed fixture source is compiled. Each suite's logs and validation
hashes remain in .work/tmp (or TMPDIR). Never uses the desktop DISPLAY/I3SOCK.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
from workspace_temp import make_tempdir, temporary_root
from regression_environment import (DEPS, MissingTestDependency, find_reference, find_perl_lib,
                                    find_xserver, native_fixture_tools)

ROOT = Path(__file__).resolve().parent.parent
PRODUCTS = ["i3", "i3bar", "i3-config-wizard", "i3-dump-log", "i3-input", "i3-msg", "i3-nagbar"]
HELPERS = ["test.commands_parser", "test.config_parser", "test.inject_randr15"]


def cargo_bin_dir(release=False):
    """Use Cargo's normal target directory; explicit CLI paths can override it."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", "target"))
    if not target.is_absolute():
        target = ROOT / target
    return target / ("release" if release else "debug")


def executable(name, directory, env):
    path = str(directory / name) if directory else shutil.which(name, path=env["PATH"])
    if not path or not os.access(path, os.X_OK):
        raise SystemExit(f"Required executable {name} is unavailable; provide its directory explicitly")
    return Path(path).resolve()


def failure_details(suite, output, limit=12000):
    """Surface failing TAP records without flooding Cargo/CI with passing tests."""
    summary = re.search(r"Test Summary Report\n.*?Result: \w+", output, re.S)
    failed = set(re.findall(r"^(.+?\.t) \(Wstat:", summary[0] if summary else "", re.M))
    sections = []
    if summary:
        sections.append("Upstream failure summary:\n" + summary[0])
    for path in sorted(suite.glob("testsuite-*/complete-run.log")):
        try:
            text = path.read_text(errors="replace")
        except OSError as error:
            sections.append(f"Could not read {path}: {error}")
            continue
        records = list(re.finditer(r"^output for (.+):\n", text, re.M))
        for index, record in enumerate(records):
            end = records[index + 1].start() if index + 1 < len(records) else len(text)
            body = text[record.end():end]
            markers = list(re.finditer(r"^(?:not ok\b(?![^\n]*#\s*(?:TODO|SKIP)\b)|Bail out!)", body, re.M | re.I))
            if record[1] not in failed and (failed or not markers):
                continue
            # Include diagnostic lines after each failed assertion and nearby
            # context. Aborted tests without an assertion retain their last lines.
            excerpts = []
            for marker in markers[:3]:
                before = body.rfind("\n", 0, max(0, marker.start() - 1)) + 1
                excerpt = "\n".join(body[before:].splitlines()[:24])
                excerpts.append(excerpt[:2500])
            if not excerpts:
                excerpts.append(body[-3000:])
            sections.append(f"Failing TAP record: {record[1]}\n" + "\n...\n".join(excerpts))
    if len(sections) <= int(summary is not None):
        sections.append("Runner output tail:\n" + output[-6000:])
    report = "\n\n".join(sections)
    encoded = report.encode("utf-8")
    if len(encoded) > limit:
        suffix = b"\n[Failure details truncated; full runner/TAP logs remain in the suite directory.]"
        report = encoded[:max(0, limit - len(suffix))].decode("utf-8", errors="ignore") + suffix.decode()
    return report


def run_suite(args):
    reference = (args.reference_build or find_reference()).resolve()
    if args.perl_lib is None:
        args.perl_lib = find_perl_lib()
    if args.xvfb_dir is None:
        args.xvfb_dir = find_xserver('Xvfb').parent
    if args.xephyr_dir is None:
        args.xephyr_dir = find_xserver('Xephyr').parent
    native_fixture_tools()
    rust_bin = (args.rust_bin_dir or cargo_bin_dir(args.release)).resolve()
    env = {key: value for key, value in os.environ.items()
           if key not in {"DISPLAY", "I3SOCK", "WAYLAND_DISPLAY", "XAUTHORITY",
                          "DBUS_SESSION_BUS_ADDRESS", "NOTIFY_SOCKET", "LISTEN_PID",
                          "LISTEN_FDS", "LISTEN_FDNAMES"}}
    env["TMPDIR"] = str(temporary_root())
    # Inline hashes source/configuration itself. Persist unchanged native fixture
    # modules instead of recompiling them in every fresh suite directory.
    inline_cache = DEPS / 'inline'
    inline_cache.mkdir(parents=True, exist_ok=True)
    env['PERL_INLINE_DIRECTORY'] = str(inline_cache)
    env.pop('PERL5LIB', None)
    if args.perl_lib:
        env["PERL5LIB"] = str(args.perl_lib.resolve())
    directories = [str(p.resolve()) for p in [args.xvfb_dir, args.xephyr_dir] if p]
    env["PATH"] = os.pathsep.join([*directories, env.get("PATH", "/usr/bin:/bin")])
    # Verify before copying or starting anything. complete-run.pl checks xvfb-run
    # even though it directly launches its private Xvfb parent server afterward.
    for name, directory in [("Xvfb", args.xvfb_dir), ("xvfb-run", None),
                            ("Xephyr", args.xephyr_dir), ("perl", None), ("xauth", None)]:
        executable(name, directory, env)
    for name in [*PRODUCTS, *HELPERS]:
        source = reference / name if args.baseline or name in HELPERS else rust_bin / name
        if not source.is_file() or not os.access(source, os.X_OK):
            raise SystemExit(f"Missing executable {source}; run cargo build and supply the three external precompiled test helpers with --reference-build")
    templates = {
        "complete-run.pl": ROOT / "testcases/complete-run.pl.in",
        "i3test.pm": ROOT / "testcases/lib/i3test.pm.in",
    }
    for source in templates.values():
        if not source.is_file():
            raise SystemExit(f"Missing retained Perl suite template {source}")

    suite = Path(make_tempdir(prefix="i333-suite-c-" if args.baseline else "i333-suite-rust-"))
    for name, source in templates.items():
        text = source.read_text().replace("@abs_top_builddir@", str(suite)).replace("@abs_top_srcdir@", str(ROOT))
        (suite / name).write_text(text)
    # The upstream Perl module is pure Perl: use it directly without a Makefile
    # or an install step. Its generated blib path is expected by the templates.
    perl_module = suite / "AnyEvent-I3/blib/lib/AnyEvent"
    perl_module.mkdir(parents=True)
    shutil.copy2(ROOT / "AnyEvent-I3/lib/AnyEvent/I3.pm", perl_module / "I3.pm")
    for name in [*PRODUCTS, *HELPERS]:
        source = reference / name if args.baseline or name in HELPERS else rust_bin / name
        shutil.copy2(source, suite / name)
    log_path = suite / "runner.log"
    print(f"Private suite directory: {suite}", flush=True)
    print(f"Runner log: {log_path}", flush=True)
    tests = [str(Path(test).resolve()) if Path(test).is_file() else test for test in args.tests]
    command = ["perl", "./complete-run.pl", f"--parallel={args.parallel}", "--keep-xserver-output", *tests]
    with log_path.open("wb") as log:
        process = subprocess.Popen(command, cwd=suite, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            return_code = process.wait(timeout=args.timeout)
        except (subprocess.TimeoutExpired, KeyboardInterrupt):
            # This new session contains only our suite and its private X servers.
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            raise
    output = log_path.read_text(errors="replace")
    summary = re.search(r"(All tests successful\.\n)?Files=.+?\nResult: \w+", output)
    if return_code != 0:
        print(failure_details(suite, output), flush=True)
    else:
        print(summary[0] if summary else output[-6000:])
    hashes = {}
    import hashlib
    for name in PRODUCTS:
        hashes[name] = hashlib.sha256((suite / name).read_bytes()).hexdigest()
    (suite / "validation.json").write_text(json.dumps({
        "mode": "c-baseline" if args.baseline else "rust",
        "return_code": return_code,
        "summary": summary[0] if summary else None,
        "product_sha256": hashes,
        "parallel": args.parallel,
        "desktop_environment_removed": True,
    }, indent=2) + "\n")
    if return_code == 0 and not summary:
        raise SystemExit("Runner returned success without a TAP summary; inspect the preserved log")
    return return_code


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--reference-build", type=Path,
                        help="External directory containing the three precompiled upstream test helpers")
    parser.add_argument("--rust-bin-dir", type=Path, help="Directory containing Cargo-built products")
    parser.add_argument("--release", action="store_true", help="Use Cargo target/release instead of target/debug")
    parser.add_argument("--perl-lib", type=Path)
    parser.add_argument("--xvfb-dir", type=Path)
    parser.add_argument("--xephyr-dir", type=Path)
    parser.add_argument("--parallel", type=int, default=4)
    parser.add_argument("--timeout", type=int, default=300)
    parser.add_argument("--baseline", action="store_true")
    parser.add_argument("tests", nargs="*", help="Optional upstream test files; otherwise run all")
    arguments = parser.parse_args()
    if arguments.parallel < 1 or arguments.timeout < 1:
        parser.error("parallelism and timeout must be positive")
    try:
        raise SystemExit(run_suite(arguments))
    except MissingTestDependency as error:
        parser.exit(2, f'Regression prerequisites failed: {error}\n')
