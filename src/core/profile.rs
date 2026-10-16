//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).
//!
//! The emitted oracle crate's `Cargo.toml` is written here once and called by the three oracle
//! emitters. Its runtime and kernel dependencies name their source the way this crate's own
//! `Cargo.toml` does: git URL and `branch = "main"`. The commit is not spelled in the emitted
//! manifest; a consumer's lockfile records it.

/// The runtime source every emitted manifest depends on, spelled as in this crate's `Cargo.toml`:
/// the `git` and `branch` keys of the dependency table, with no `rev`. Public so a scratch crate
/// that compiles generated code names the runtime the same way and cannot drift from the emitter.
pub const RUNTIME_DEPENDENCY_SOURCE: &str =
    "git = \"https://github.com/agent-ix/quire-contract-runtime\", branch = \"main\"";

/// The authoritative exact-kernel source used by generated scalar, equality and function
/// oracles, with no revision duplicated outside Cargo's lockfile.
pub const EXACT_DEPENDENCY_SOURCE: &str =
    "git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\"";

/// The `[package.metadata.kani]` table every generated oracle crate's manifest carries. CBMC
/// tracks heap objects field by field only up to 64 bytes by default; RT's `Value` and `ValueType`
/// are larger, and a non-field-sensitive read of them cannot be constant-folded, so the crate raises
/// the limit the way RT's own `Cargo.toml` does.
const ORACLE_KANI_METADATA: &str = "[package.metadata.kani]\nunstable = { unstable-options = true }\nflags = { cbmc-args = [\"--max-field-sensitivity-array-size\", \"1024\"] }\n\n";

/// The `Cargo.toml` of a generated oracle crate named `crate_name`: a library over the runtime's
/// `exact` feature and exact kernel that forbids unsafe code and carries the Kani metadata.
pub(crate) fn oracle_crate_manifest(crate_name: &str) -> String {
    oracle_crate_manifest_with_observer(crate_name, false)
}

/// The scalar oracle manifest with the native receipt observer as an example target.
pub(crate) fn scalar_oracle_crate_manifest(crate_name: &str) -> String {
    oracle_crate_manifest_with_observer(crate_name, true)
}

fn oracle_crate_manifest_with_observer(crate_name: &str, observer: bool) -> String {
    let example = if observer {
        "[[example]]\nname = \"ir791_native\"\npath = \"examples/ir791_native.rs\"\n\n"
    } else {
        ""
    };
    let dev_dependency = if observer {
        format!("[dev-dependencies]\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}\n\n")
    } else {
        String::new()
    };
    format!(
        "[package]\nname = \"{crate_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n{example}[dependencies]\nquire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [\"exact\"] }}\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE} }}\n\n{dev_dependency}[lints.rust]\nunsafe_code = \"forbid\"\n\n{ORACLE_KANI_METADATA}[workspace]\n"
    )
}

#[cfg(test)]
mod tests {
    use super::scalar_oracle_crate_manifest;

    /// Trace: FR-014-AC-8, TC-024.
    #[test]
    fn runtime_boolean_outcome_has_the_kernel_type() {
        use quire_exact::{Meter, Outcome, ScalarLimits};

        fn accepts_kernel_outcome(_: Outcome<bool>) {}

        let mut meter = Meter::new(ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        });
        let produced = quire_contract_runtime::exact::evaluate_boolean_short_circuit(
            quire_contract_runtime::exact::ShortCircuitConnective::And,
            true,
            || Outcome::Completed(true),
            &mut meter,
        );
        accepts_kernel_outcome(produced);
    }

    /// Trace: TC-024, TC-029, TC-031.
    #[test]
    fn tc_024_oracle_manifest_names_the_runtime_by_branch_and_carries_no_pinned_revision() {
        let manifest = scalar_oracle_crate_manifest("probe-crate");
        assert!(manifest.contains("name = \"probe-crate\""));
        assert!(manifest.contains(
            "quire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", branch = \"main\", features = [\"exact\"] }"
        ));
        assert!(!manifest.contains("rev ="));
        assert!(manifest.contains(
            "quire-exact = { git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\" }"
        ));
        assert!(manifest
            .contains("[[example]]\nname = \"ir791_native\"\npath = \"examples/ir791_native.rs\""));
        assert!(manifest.contains("[dev-dependencies]\nquire-exact = { git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\", features = [\"test-support\"] }"));
        assert!(manifest.contains("publish = false"));
        assert!(manifest.contains("unsafe_code = \"forbid\""));
        assert!(manifest.contains("--max-field-sensitivity-array-size"));
    }
}
