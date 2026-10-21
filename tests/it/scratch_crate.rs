//! Scratch crates the tests compile generated code in.
//!
//! Every such crate names the runtime with the spelling an emitted manifest carries
//! (`quire_contract_codegen::RUNTIME_DEPENDENCY_SOURCE`, one place) and is seeded with this
//! repository's own `Cargo.lock`, so the runtime it compiles against is the commit CG's lock
//! records and not whatever `main` the machine's shared git cache last fetched.

use std::{
    collections::BTreeMap,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    sync::Mutex,
    thread::ThreadId,
};

use quire_contract_codegen::RUNTIME_DEPENDENCY_SOURCE;

/// The `quire-contract-runtime` dependency line of a scratch manifest, with `features`.
pub(crate) fn runtime_dependency(features: &[&str]) -> String {
    if features.is_empty() {
        return format!("quire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE} }}");
    }
    let features = features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("quire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [{features}] }}")
}

/// The `quire-exact` dependency line of a scratch manifest, with `features`, spelled as this
/// crate's `Cargo.toml` does so the seeded lock selects the same kernel revision.
pub(crate) fn exact_dependency(features: &[&str]) -> String {
    let features = features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "quire-exact = {{ git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\", features = [{features}] }}"
    )
}

/// Copies this repository's `Cargo.lock` beside the manifest in `directory`.
pub(crate) fn seed_lock(directory: &Path) {
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        directory.join("Cargo.lock"),
    )
    .expect("seed the scratch crate's lockfile");
}

/// Writes `manifest` as the `Cargo.toml` of `directory` and seeds the lock beside it.
pub(crate) fn write_manifest(directory: &Path, manifest: &str) {
    fs::write(directory.join("Cargo.toml"), manifest).expect("write the scratch manifest");
    seed_lock(directory);
}

/// The prefixes held right now, each with the thread that holds it. A thread asking for a
/// prefix it already holds would wait on its own lock forever, so that is refused up front.
static HELD: Mutex<BTreeMap<PathBuf, ThreadId>> = Mutex::new(BTreeMap::new());

/// Scratch crate directory at a stable path under the Cargo target directory.
///
/// A run killed before `Drop` leaves its directory behind; the next run claims the same
/// path and clears it, and `cargo clean` removes it with the rest of the target directory.
/// An exclusive advisory lock on a sibling file is held for the value's lifetime, so a
/// concurrent run waits instead of clearing a live directory, and the kernel releases the
/// lock when a run dies, which marks its directory as reclaimable. A thread that asks for a
/// prefix it still holds panics naming the prefix: each live directory of one thread needs
/// its own prefix.
pub(crate) struct TemporaryDirectory {
    pub(crate) path: PathBuf,
    _lock: fs::File,
}

impl TemporaryDirectory {
    pub(crate) fn new(prefix: &str) -> Self {
        Self::in_directory(Path::new(env!("CARGO_TARGET_TMPDIR")), prefix)
    }

    fn in_directory(base: &Path, prefix: &str) -> Self {
        fs::create_dir_all(base).unwrap();
        let path = base.join(prefix);
        let lock_path = base.join(format!("{prefix}.lock"));
        let me = std::thread::current().id();
        if HELD.lock().unwrap().get(&path) == Some(&me) {
            panic!(
                "scratch prefix `{prefix}` is already held by this thread; \
                 a second live directory needs its own prefix ({})",
                lock_path.display()
            );
        }
        let lock = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&lock_path)
            .unwrap();
        match lock.try_lock() {
            Ok(()) => {}
            Err(fs::TryLockError::WouldBlock) => {
                eprintln!("waiting for the scratch lock {}", lock_path.display());
                lock.lock().unwrap();
            }
            Err(fs::TryLockError::Error(error)) => {
                panic!("locking {}: {error}", lock_path.display())
            }
        }
        HELD.lock().unwrap().insert(path.clone(), me);
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => panic!(
                "clearing stale scratch directory {}: {error}",
                path.display()
            ),
        }
        fs::create_dir_all(path.join("src")).unwrap();
        Self { path, _lock: lock }
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
        HELD.lock().unwrap().remove(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::{runtime_dependency, write_manifest};

    #[test]
    fn scratch_directory_lives_under_target_tmpdir() {
        let scratch = super::TemporaryDirectory::new("quire-bound-populations-location");
        assert!(
            scratch.path.starts_with(env!("CARGO_TARGET_TMPDIR")),
            "{} is outside the target directory",
            scratch.path.display()
        );
        assert!(!scratch.path.starts_with(std::env::temp_dir()));
    }

    #[test]
    fn scratch_directory_reclaims_leftover_from_dead_run() {
        let base = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
        let leftover = base.join("quire-bound-populations-reclaim");
        std::fs::create_dir_all(leftover.join("target")).unwrap();
        std::fs::write(leftover.join("target/stale"), "x").unwrap();
        let scratch =
            super::TemporaryDirectory::in_directory(base, "quire-bound-populations-reclaim");
        assert_eq!(scratch.path, leftover);
        assert!(!leftover.join("target/stale").exists());
        assert!(leftover.join("src").is_dir());
    }

    #[test]
    fn scratch_directory_is_locked_while_live() {
        let base = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
        let scratch = super::TemporaryDirectory::in_directory(base, "quire-bound-populations-live");
        let probe = std::fs::File::open(base.join("quire-bound-populations-live.lock")).unwrap();
        assert!(matches!(
            probe.try_lock(),
            Err(std::fs::TryLockError::WouldBlock)
        ));
        drop(scratch);
        // A child forked by a concurrent test can briefly hold a duplicate of the descriptor
        // until it execs, so the release is awaited rather than assumed instantaneous.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while probe.try_lock().is_err() {
            assert!(
                std::time::Instant::now() < deadline,
                "the lock is released once the directory drops"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    #[test]
    fn dropping_the_directory_removes_it() {
        let scratch = super::TemporaryDirectory::new("quire-scratch-drop");
        let path = scratch.path.clone();
        assert!(path.is_dir());
        drop(scratch);
        assert!(!path.exists());
    }

    #[test]
    fn a_waiting_thread_proceeds_after_the_holder_drops() {
        let base = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
        let holder = super::TemporaryDirectory::in_directory(base, "quire-scratch-wait");
        let marker = holder.path.join("holder");
        std::fs::write(&marker, "x").unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let waiter = std::thread::spawn(move || {
            let scratch = super::TemporaryDirectory::in_directory(
                std::path::Path::new(env!("CARGO_TARGET_TMPDIR")),
                "quire-scratch-wait",
            );
            sender.send(scratch.path.join("holder").exists()).unwrap();
        });
        assert!(
            receiver
                .recv_timeout(std::time::Duration::from_millis(300))
                .is_err(),
            "the waiter must block while the holder is live"
        );
        drop(holder);
        let saw_holder_file = receiver
            .recv_timeout(std::time::Duration::from_secs(30))
            .expect("the waiter proceeds once the holder drops");
        assert!(
            !saw_holder_file,
            "the waiter starts from a cleared directory"
        );
        waiter.join().unwrap();
    }

    #[test]
    #[should_panic(expected = "scratch prefix `quire-scratch-reentrant` is already held")]
    fn reacquiring_a_held_prefix_on_one_thread_panics_naming_it() {
        let _first = super::TemporaryDirectory::new("quire-scratch-reentrant");
        let _second = super::TemporaryDirectory::new("quire-scratch-reentrant");
    }

    /// The `source` of the `quire-contract-runtime` entry in `lock`.
    fn locked_runtime_source(lock: &str) -> String {
        let block = lock
            .split("[[package]]")
            .find(|block| block.contains("name = \"quire-contract-runtime\""))
            .expect("CG's lock names the runtime");
        block
            .lines()
            .find_map(|line| line.strip_prefix("source = \""))
            .and_then(|rest| rest.strip_suffix('"'))
            .expect("the runtime entry has a source")
            .to_owned()
    }

    /// Architecture evidence: AD-004 L-11.
    #[test]
    fn a_scratch_crate_resolves_the_runtime_commit_cg_locks() {
        let scratch = super::TemporaryDirectory::new("quire-codegen-scratch-lock");
        let directory = scratch.path.clone();
        std::fs::write(directory.join("src/lib.rs"), "").unwrap();
        write_manifest(
            &directory,
            &format!(
                "[package]\nname = \"scratch-lock-probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\n{}\n\n[workspace]\n",
                runtime_dependency(&[])
            ),
        );
        let output = Command::new(env!("CARGO"))
            .args(["metadata", "--offline", "--format-version", "1"])
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let resolved = metadata["packages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|package| package["name"] == "quire-contract-runtime")
            .expect("the scratch crate depends on the runtime")["source"]
            .as_str()
            .unwrap()
            .to_owned();
        let lock = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        )
        .unwrap();
        assert_eq!(resolved, locked_runtime_source(&lock));
    }
}
