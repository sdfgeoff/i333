"""Private test artifacts stay in the workspace, unless TMPDIR overrides it."""
import os
from pathlib import Path
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def temporary_root():
    directory = Path(os.environ.get("TMPDIR", str(ROOT / ".work/tmp"))).expanduser().resolve()
    directory.mkdir(parents=True, exist_ok=True)
    os.environ["TMPDIR"] = str(directory)
    return directory


def temporary_directory(*, prefix):
    return tempfile.TemporaryDirectory(prefix=prefix, dir=temporary_root())


def make_tempdir(*, prefix):
    return tempfile.mkdtemp(prefix=prefix, dir=temporary_root())
