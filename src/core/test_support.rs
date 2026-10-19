//! Scratch directories shared by unit tests across subsystems.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// A scratch directory unique to this process and this call, so parallel tests never
/// collide and nothing here touches a real `$HOME` or `$CARGO_HOME`.
///
/// Directories live under the Cargo target directory (found from the test executable's own
/// path, since unit tests get no `CARGO_TARGET_TMPDIR`), named by process id. The first call in
/// a process removes the directories of processes that no longer exist, so a killed run's
/// leftovers are reclaimed by the next run, and `cargo clean` removes the rest.
pub(crate) fn discover_scratch(name: &str) -> PathBuf {
    static SWEEP: std::sync::Once = std::sync::Once::new();
    static CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let executable = env::current_exe().unwrap();
    let target = executable
        .ancestors()
        .nth(3)
        .expect("the test executable sits under a target directory");
    let base = target.join("tmp").join("quire-kani-discover");
    fs::create_dir_all(&base).unwrap();
    SWEEP.call_once(|| {
        for entry in fs::read_dir(&base).unwrap().flatten() {
            let owner = entry
                .file_name()
                .to_str()
                .and_then(|file_name| file_name.split('-').next_back())
                .and_then(|pid| pid.parse::<u32>().ok());
            if owner.is_some_and(|pid| !Path::new("/proc").join(pid.to_string()).exists()) {
                let _ = fs::remove_dir_all(entry.path());
            }
        }
    });
    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let path = base.join(format!("{name}-{call}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}
