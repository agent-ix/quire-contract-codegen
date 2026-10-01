//! Scratch crates the tests compile generated code in.
//!
//! Every such crate names the runtime with the spelling an emitted manifest carries
//! (`quire_contract_codegen::RUNTIME_DEPENDENCY_SOURCE`, one place) and is seeded with this
//! repository's own `Cargo.lock`, so the runtime it compiles against is the commit CG's lock
//! records and not whatever `main` the machine's shared git cache last fetched.

use std::{fs, path::Path};

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

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::{runtime_dependency, write_manifest};

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

    /// Trace: AD-004 L-11.
    #[test]
    fn a_scratch_crate_resolves_the_runtime_commit_cg_locks() {
        let directory =
            std::env::temp_dir().join(format!("quire-codegen-scratch-lock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(directory.join("src")).unwrap();
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
        let _ = std::fs::remove_dir_all(&directory);
    }
}
