//! Build-script support shared by the workspace members.
//!
//! Each member calls [`configure`] for its own compiler cfgs and version.
//! The common crate calls [`link_platform_libraries`] for platform libraries.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Configure the calling member's target, variadic ABI and version string.
///
/// This runs inside the member's build script, so Cargo environment variables
/// describe that member rather than this support library.
pub fn configure() {
    assert_eq!(
        env::var("TARGET").as_deref(),
        Ok("x86_64-unknown-linux-gnu"),
        "The translated native ABI currently supports x86_64-unknown-linux-gnu only"
    );
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=I3_VERSION");
    println!("cargo:rustc-check-cfg=cfg(i333_new_va_list)");

    // Rust 1.99 changed the VaList API. Keep the compatibility cfg local to
    // every member that compiles a variadic translation.
    let rustc = env::var_os("RUSTC").expect("Cargo supplies RUSTC");
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("query rustc version");
    assert!(output.status.success(), "rustc --version failed");
    let version =
        compiler_version(&String::from_utf8_lossy(&output.stdout)).expect("rustc version number");
    if version >= (1, 99) {
        println!("cargo:rustc-cfg=i333_new_va_list");
    }

    let manifest = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("Cargo supplies manifest directory"),
    );
    let root = workspace_root(&manifest);
    if let Some(git_dir) = git_directory(&root) {
        track_git_version(&git_dir);
    }
    let version = env::var("I3_VERSION")
        .ok()
        .or_else(|| git_version(&root))
        .unwrap_or_else(|| {
            // Read at execution time: env! here would use i3-build's package
            // version instead of the package whose build script is running.
            format!(
                "{}-rust",
                env::var("CARGO_PKG_VERSION").expect("Cargo supplies package version")
            )
        });
    assert!(
        !version.contains(['\n', '\r', '\0']),
        "I3_VERSION must be a single line without NUL"
    );
    println!("cargo:rustc-env=I3_VERSION={version}");
}

fn compiler_version(output: &str) -> Option<(u32, u32)> {
    let mut version = output
        .strip_prefix("rustc ")?
        .split_whitespace()
        .next()?
        .split('.');
    Some((version.next()?.parse().ok()?, version.next()?.parse().ok()?))
}

fn workspace_root(manifest: &Path) -> PathBuf {
    manifest
        .ancestors()
        .find(|directory| {
            fs::read_to_string(directory.join("Cargo.toml"))
                .ok()
                .is_some_and(|text| text.lines().any(|line| line.trim() == "[workspace]"))
        })
        .unwrap_or(manifest)
        .to_path_buf()
}

fn git_directory(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return Some(dot_git);
    }
    let text = fs::read_to_string(&dot_git).ok()?;
    let directory = PathBuf::from(text.trim().strip_prefix("gitdir: ")?);
    Some(if directory.is_absolute() {
        directory
    } else {
        root.join(directory)
    })
}

fn track_if_present(path: &Path) {
    if path.is_file() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn track_git_version(git_dir: &Path) {
    track_if_present(&git_dir.join("HEAD"));
    // Worktrees keep references and packed-refs in the common Git directory.
    let common = fs::read_to_string(git_dir.join("commondir"))
        .ok()
        .map(|relative| git_dir.join(relative.trim()))
        .unwrap_or_else(|| git_dir.to_path_buf());
    track_if_present(&common.join("packed-refs"));
    if let Ok(head) = fs::read_to_string(git_dir.join("HEAD")) {
        if let Some(reference) = head.trim().strip_prefix("ref: ") {
            track_if_present(&common.join(reference));
        }
    }
}

fn git_version(root: &Path) -> Option<String> {
    // An archive nested inside another Git checkout must still use the archive
    // fallback rather than accidentally describing the enclosing repository.
    git_directory(root)?;
    let output = Command::new("git")
        .current_dir(root)
        .args(["describe", "--always", "--dirty=+"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Link platform libraries used directly by the translated Linux backend.
/// Native binding crates own discovery and version checks for their libraries.
pub fn link_platform_libraries() {
    for library in ["m", "rt"] {
        println!("cargo:rustc-link-lib={library}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = env::temp_dir().join(format!(
                "i3-build-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn compiler_versions_cover_both_variadic_apis() {
        assert_eq!(
            compiler_version("rustc 1.92.0-nightly (hash date)"),
            Some((1, 92))
        );
        assert_eq!(
            compiler_version("rustc 1.99.0-nightly (hash date)"),
            Some((1, 99))
        );
        assert_eq!(compiler_version("rustc 2.0.0 (hash date)"), Some((2, 0)));
        assert_eq!(compiler_version("cargo 1.99.0"), None);
        assert_eq!(compiler_version("rustc unknown"), None);
    }

    #[test]
    fn archive_members_find_workspace_without_git() {
        let fixture = Fixture::new();
        let member = fixture.0.join("crates/i3");
        fs::create_dir_all(&member).unwrap();
        fs::write(
            fixture.0.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/i3\"]\n",
        )
        .unwrap();
        fs::write(member.join("Cargo.toml"), "[package]\nname = \"i3\"\n").unwrap();
        assert_eq!(workspace_root(&member), fixture.0);
        assert!(git_version(&fixture.0).is_none());
    }

    #[test]
    fn worktree_git_file_resolves_relative_to_workspace_root() {
        let fixture = Fixture::new();
        fs::write(
            fixture.0.join(".git"),
            "gitdir: ../repository/worktrees/port\n",
        )
        .unwrap();
        assert_eq!(
            git_directory(&fixture.0),
            Some(fixture.0.join("../repository/worktrees/port"))
        );
    }
}
