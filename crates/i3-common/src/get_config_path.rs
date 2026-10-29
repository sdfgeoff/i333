// Copyright © 2009 Michael Stapelberg and contributors (see LICENSE).

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

/// Configuration search inputs, separate from process environment for callers
/// that manage their own configuration or need deterministic discovery.
#[derive(Debug, Clone)]
pub struct ConfigSearch {
    pub config_home: PathBuf,
    pub legacy_config: PathBuf,
    pub system_dirs: Vec<PathBuf>,
    pub system_config: PathBuf,
}

impl ConfigSearch {
    pub fn from_environment() -> Self {
        let dirs = std::env::var_os("XDG_CONFIG_DIRS")
            .unwrap_or_else(|| OsString::from("/usr/local/etc/xdg"));
        Self {
            config_home: std::env::var_os("XDG_CONFIG_HOME")
                .unwrap_or_else(|| OsString::from("~/.config"))
                .into(),
            legacy_config: "~/.i3/config".into(),
            // strtok skipped empty components; preserve that behavior explicitly.
            system_dirs: std::env::split_paths(&dirs)
                .filter(|path| !path.as_os_str().is_empty())
                .collect(),
            system_config: "/usr/local/etc/i3/config".into(),
        }
    }

    pub fn find(&self, use_system_paths: bool) -> io::Result<Option<PathBuf>> {
        let config_home = append_config(crate::resolve_tilde::resolve(&self.config_home)?);
        if crate::path_exists::exists(&config_home) {
            return Ok(Some(config_home));
        }
        let legacy = crate::resolve_tilde::resolve(&self.legacy_config)?;
        if crate::path_exists::exists(&legacy) {
            return Ok(Some(legacy));
        }
        if use_system_paths {
            for directory in &self.system_dirs {
                let candidate = append_config(crate::resolve_tilde::resolve(directory)?);
                if crate::path_exists::exists(&candidate) {
                    return Ok(Some(candidate));
                }
            }
            if crate::path_exists::exists(&self.system_config) {
                return Ok(Some(self.system_config.clone()));
            }
        }
        Ok(None)
    }
}

fn append_config(directory: PathBuf) -> PathBuf {
    // String-style append preserves the old meaning of an explicitly empty
    // XDG directory ("/i3/config") and repeated/trailing separators.
    let mut path = directory.into_os_string();
    path.push("/i3/config");
    path.into()
}

/// An explicit path is returned as given, even if it does not currently exist.
pub fn find_config_path(
    explicit: Option<&Path>,
    use_system_paths: bool,
) -> io::Result<Option<PathBuf>> {
    if let Some(path) = explicit {
        return Ok(Some(path.to_owned()));
    }
    ConfigSearch::from_environment().find(use_system_paths)
}

mod legacy {
    use super::*;
    use std::ffi::CStr;
    use std::os::unix::ffi::OsStrExt;
    use std::sync::Mutex;
    static OVERRIDE: Mutex<Option<PathBuf>> = Mutex::new(None);

    /// # Safety
    /// `override_path` is null or a live C string. Returned storage belongs to free.

    pub unsafe fn get_config_path(
        override_path: *const libc::c_char,
        use_system_paths: bool,
    ) -> *mut libc::c_char {
        let cached = {
            let mut cached = OVERRIDE
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if !override_path.is_null() {
                // SAFETY: Copy the caller's string so the remembered override
                // does not borrow an argv/config allocation that might be freed.
                let bytes = unsafe { CStr::from_ptr(override_path) }.to_bytes();
                *cached = Some(PathBuf::from(std::ffi::OsStr::from_bytes(bytes)));
            }
            cached.clone()
        };
        let found = find_config_path(cached.as_deref(), use_system_paths)
            .unwrap_or_else(|error| crate::resolve_tilde::abort(error));
        match found {
            None => std::ptr::null_mut(),
            Some(path) => {
                let bytes = path.as_os_str().as_bytes();
                // SAFETY: Preserve malloc/free ownership for existing callers.
                unsafe { crate::safewrappers::sstrndup(bytes.as_ptr().cast(), bytes.len()) }
            }
        }
    }
}
pub use legacy::get_config_path;
