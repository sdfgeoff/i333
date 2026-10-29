#!/usr/bin/env python3
"""Run i3's complete upstream suite on private headless X servers.

Example (all dependencies remain in .work):
    python scripts/test-upstream-headless.py \
        --reference-build /path/to/external/i3-test-helpers \
        --perl-lib .work/i333-perl/lib/perl5 \
        --xvfb-dir .work/i333-xvfb/usr/bin \
        --xephyr-dir .work/i333-xephyr/usr/bin
Run cargo build first; Rust products default to target/debug (or target/release
with --release). Override with --rust-bin-dir. --reference-build must explicitly
point to three external precompiled test helpers: test.commands_parser,
test.config_parser, and test.inject_randr15. The runner generates its Perl files
from retained templates; it needs no C sources, headers, or build system in this
checkout. Add --baseline to use external C products from that directory too.
The Perl Inline::C test fixtures compile only inside the private temporary suite;
they need a C compiler and system XCB/libstartup-notification development headers.
The suite copy and full logs are preserved in a fresh .work/tmp/i333-suite-* directory (or under TMPDIR).

Dependency bootstrap used for the initial validation (no global installation):
    mkdir -p .work/tmp
    export TMPDIR="$PWD/.work/tmp"
    curl -sS -L https://cpanmin.us -o .work/i333-cpanm
    PERL_CPANM_HOME=.work/i333-cpanm-cache perl .work/i333-cpanm \
        --local-lib-contained .work/i333-perl --notest \
        --mirror https://cpan.metacpan.org --mirror-only \
        AnyEvent JSON::XS X11::XCB Inline Inline::C ExtUtils::PkgConfig IPC::Run
    mkdir -p .work/i333-perl/lib/perl5/AnyEvent
    cp AnyEvent-I3/lib/AnyEvent/I3.pm .work/i333-perl/lib/perl5/AnyEvent/I3.pm
Arch X-server extraction commands (use the archive if the mirror rotates):
    curl -sS -L https://geo.mirror.pkgbuild.com/extra/os/x86_64/xorg-server-xvfb-21.1.25-2-x86_64.pkg.tar.zst -o .work/i333-xvfb.pkg.tar.zst
    mkdir -p .work/i333-xvfb
    tar -xf .work/i333-xvfb.pkg.tar.zst -C .work/i333-xvfb
    curl -sS -L https://geo.mirror.pkgbuild.com/extra/os/x86_64/xorg-server-xephyr-21.1.25-2-x86_64.pkg.tar.zst -o .work/i333-xephyr.pkg.tar.zst
    mkdir -p .work/i333-xephyr
    tar -xf .work/i333-xephyr.pkg.tar.zst -C .work/i333-xephyr
Supply installed binaries with the directory options or PATH on other systems.
They require matching X11 runtime libraries, xauth, xkbcomp and keyboard data.

Never uses the desktop DISPLAY or I3SOCK. Xvfb is mandatory, never optional.
Cleanup on interruption/timeout signals only this runner's child process group.
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


def run_suite(args):
    reference = args.reference_build.resolve()
    rust_bin = (args.rust_bin_dir or cargo_bin_dir(args.release)).resolve()
    env = {key: value for key, value in os.environ.items()
           if key not in {"DISPLAY", "I3SOCK", "WAYLAND_DISPLAY", "XAUTHORITY", "DBUS_SESSION_BUS_ADDRESS"}}
    env["TMPDIR"] = str(temporary_root())
    if args.perl_lib:
        env["PERL5LIB"] = str(args.perl_lib.resolve())
    directories = [str(p.resolve()) for p in [args.xvfb_dir, args.xephyr_dir] if p]
    env["PATH"] = os.pathsep.join([*directories, env.get("PATH", "/usr/bin:/bin")])
    # Verify before copying or starting anything. complete-run.pl checks xvfb-run
    # even though it directly launches its private Xvfb parent server afterward.
    for name, directory in [("Xvfb", args.xvfb_dir), ("xvfb-run", args.xvfb_dir),
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
    parser.add_argument("--reference-build", type=Path, required=True,
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
    raise SystemExit(run_suite(arguments))
