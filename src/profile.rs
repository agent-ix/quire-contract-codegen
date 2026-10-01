//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).
//!
//! The emitted oracle crate's `Cargo.toml` is written here once and called by the three oracle
//! emitters. Its runtime dependency names the runtime source the way this crate's own `Cargo.toml`
//! does: the git URL and `branch = "main"`. The commit is not spelled in the emitted manifest; a
//! consumer's lockfile records it, as this repository's `Cargo.lock` does for its own build.

/// The runtime source every emitted manifest depends on, spelled as in this crate's `Cargo.toml`.
const RUNTIME_DEPENDENCY_SOURCE: &str =
    "git = \"https://github.com/agent-ix/quire-contract-runtime\", branch = \"main\"";

/// The `[package.metadata.kani]` table every generated oracle crate's manifest carries. CBMC
/// tracks heap objects field by field only up to 64 bytes by default; RT's `Value` and `ValueType`
/// are larger, and a non-field-sensitive read of them cannot be constant-folded, so the crate raises
/// the limit the way RT's own `Cargo.toml` does.
const ORACLE_KANI_METADATA: &str = "[package.metadata.kani]\nunstable = { unstable-options = true }\nflags = { cbmc-args = [\"--max-field-sensitivity-array-size\", \"1024\"] }\n\n";

/// The `Cargo.toml` of a generated oracle crate named `crate_name`: a library over the runtime's
/// `exact` feature that forbids unsafe code and carries the Kani metadata.
pub(crate) fn oracle_crate_manifest(crate_name: &str) -> String {
    format!(
        "[package]\nname = \"{crate_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nquire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [\"exact\"] }}\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n{ORACLE_KANI_METADATA}[workspace]\n"
    )
}

#[cfg(test)]
mod tests {
    use super::oracle_crate_manifest;

    /// Trace: TC-024, TC-029, TC-031.
    #[test]
    fn tc_024_oracle_manifest_names_the_runtime_by_branch_and_carries_no_pinned_revision() {
        let manifest = oracle_crate_manifest("probe-crate");
        assert!(manifest.contains("name = \"probe-crate\""));
        assert!(manifest.contains(
            "quire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", branch = \"main\", features = [\"exact\"] }"
        ));
        assert!(!manifest.contains("rev ="));
        assert!(manifest.contains("publish = false"));
        assert!(manifest.contains("unsafe_code = \"forbid\""));
        assert!(manifest.contains("--max-field-sensitivity-array-size"));
    }
}
