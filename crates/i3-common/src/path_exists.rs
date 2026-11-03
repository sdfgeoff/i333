// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

#![forbid(unsafe_code)]

use std::path::Path;

/// Follow symbolic links and report whether metadata can be read for the path.
/// Missing paths and other metadata errors both return false, as stat did.
pub fn exists(path: impl AsRef<Path>) -> bool {
    std::fs::metadata(path).is_ok()
}
