//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).
//!
//! The emitted oracle crate's `Cargo.toml` is written here once and called by the three oracle
//! emitters. Its runtime, kernel and semantic-value dependencies name their source the way this crate's own
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

/// Source of declaration admission and structural equality in generated oracles.
pub const SEMANTIC_VALUE_DEPENDENCY_SOURCE: &str =
    "git = \"https://github.com/agent-ix/quire-semantic-value\", branch = \"main\"";

/// The `[package.metadata.kani]` table every generated oracle crate's manifest carries. CBMC
/// tracks heap objects field by field only up to 64 bytes by default; RT's `Value` and `ValueType`
/// are larger, and a non-field-sensitive read of them cannot be constant-folded, so the crate raises
/// the limit the way RT's own `Cargo.toml` does.
const ORACLE_KANI_METADATA: &str = "[package.metadata.kani]\nunstable = { unstable-options = true }\nflags = { cbmc-args = [\"--max-field-sensitivity-array-size\", \"1024\"] }\n\n";

/// The `Cargo.toml` of a generated oracle crate named `crate_name`: a library over the runtime's
/// `exact` feature, exact kernel and declaration-checking leaf; forbids unsafe code and carries Kani metadata.
pub(crate) fn oracle_crate_manifest(crate_name: &str) -> String {
    format!(
        "[package]\nname = \"{crate_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nquire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [\"exact\"] }}\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE} }}\nquire-semantic-value = {{ {SEMANTIC_VALUE_DEPENDENCY_SOURCE} }}\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n{ORACLE_KANI_METADATA}[workspace]\n"
    )
}

/// The equality ABI alone needs the public admitted-context facade.
pub(crate) fn equality_oracle_crate_manifest(crate_name: &str) -> String {
    let manifest = oracle_crate_manifest(crate_name);
    manifest.replacen("[dependencies]\n", "[dependencies]\nqsl-replay = { git = \"https://github.com/agent-ix/quire-spec-language\", branch = \"main\" }\n", 1)
}

/// The isolated observer workspace; only its dev dependencies enable charge logging.
pub(crate) fn scalar_observer_manifest() -> String {
    format!(
        "[package]\nname = \"quire-scalar-native-observer\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\nautobins = false\nautoexamples = false\n\n[[example]]\nname = \"ir791_native\"\npath = \"../src/bin/ir791_native.rs\"\n\n[dev-dependencies]\nquire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [\"exact\"] }}\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}\n\n[workspace]\nresolver = \"2\"\n"
    )
}

#[cfg(test)]
mod tests {
    use super::{
        equality_oracle_crate_manifest, oracle_crate_manifest, scalar_observer_manifest,
        EXACT_DEPENDENCY_SOURCE, RUNTIME_DEPENDENCY_SOURCE,
    };
    use ix_trace_rs::trace;

    #[trace("FR-014-AC-8", "TC-024")]
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

    #[trace("TC-024", "TC-029", "TC-031")]
    #[test]
    fn tc_024_oracle_manifest_names_the_runtime_by_branch_and_carries_no_pinned_revision() {
        let manifest = oracle_crate_manifest("probe-crate");
        assert!(manifest.contains("name = \"probe-crate\""));
        assert!(manifest.contains(
            "quire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", branch = \"main\", features = [\"exact\"] }"
        ));
        assert!(!manifest.contains("rev ="));
        assert!(manifest.contains(
            "quire-exact = { git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\" }"
        ));
        assert!(!manifest.contains("test-support"));
        assert!(!manifest.contains("[[example]]"));
        assert!(manifest.contains("publish = false"));
        assert!(manifest.contains("unsafe_code = \"forbid\""));
        assert!(manifest.contains("--max-field-sensitivity-array-size"));
    }
    #[trace("FR-018-AC-37", "FR-014-AC-8", "TC-029")]
    #[test]
    fn only_equality_manifests_acquire_the_admitted_context_facade() {
        let equality = equality_oracle_crate_manifest("equality-probe");
        assert!(equality.contains("qsl-replay = { git = \"https://github.com/agent-ix/quire-spec-language\", branch = \"main\" }"));
        assert!(!equality.contains("rev ="));
        assert!(!equality.contains("test-support"));
        assert!(!oracle_crate_manifest("scalar-probe").contains("qsl-replay"));
        assert!(!scalar_observer_manifest().contains("qsl-replay"));
    }

    #[trace("FR-017-AC-52", "TC-027")]
    #[test]
    fn observer_is_an_isolated_dev_only_example_over_the_canonical_adapter() {
        let manifest = scalar_observer_manifest();
        assert!(manifest.contains("autobins = false\nautoexamples = false"));
        assert_eq!(manifest.matches("[[example]]").count(), 1);
        assert!(manifest.contains("path = \"../src/bin/ir791_native.rs\""));
        assert!(manifest.contains("[workspace]\nresolver = \"2\""));
        assert!(!manifest.contains("[dependencies]"));
        assert!(!manifest.contains("[build-dependencies]"));
        assert!(!manifest.contains("[lib]"));
        let (_, dev) = manifest.split_once("[dev-dependencies]").unwrap();
        assert!(dev.contains(&format!(
            "quire-contract-runtime = {{ {RUNTIME_DEPENDENCY_SOURCE}, features = [\"exact\"] }}"
        )));
        assert!(dev.contains(&format!(
            "quire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}"
        )));
        let production = oracle_crate_manifest("probe-crate");
        assert!(!production.contains("test-support"));
        assert!(!production.contains("[dev-dependencies]"));
        assert!(!production.contains("[[example]]"));
    }
}
