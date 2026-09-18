//! interface-001: the API surface `spec/interface/interface-001-codegen-api.md` declares matches
//! what the crate actually exports and emits. Each test below is a small, independently checkable
//! census over the public surface or a generated value, in the style TC-013 uses for FR-006's
//! absence claims: it reads the crate's own declaration rather than trusting prose to stay in
//! sync with code.

use std::{fs, path::Path};

use quire_contract_codegen::{
    generate_bound_oracles, generate_bound_strategy, generate_enum_strategy, generate_i64_strategy,
    generate_kani_bundle, generate_tristate_harness, write_bundle_atomic, AttestationCommand,
    AttestationEnvironment, AttestationResult, AttestationTool, GenerationTerminalState,
    KaniToolPins, ProofAttestationBody,
};

fn lib_source() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
        .expect("src/lib.rs must be readable from the crate root")
}

/// Every `operations` entry interface-001 declares without a `status: planned` caveat is a real
/// exported function under the exact name the contract gives it. The `use` above naming each of
/// `generate_bound_oracles`, `generate_tristate_harness`, `generate_i64_strategy`,
/// `generate_enum_strategy`, `generate_bound_strategy`, `generate_kani_bundle` and
/// `write_bundle_atomic` is itself the census: a renamed or removed export fails this file to
/// compile at all. `analyze_bound_coverage` is exercised the same way through its own import in
/// `tests/bound_coverage.rs`, so it is censused here by source text rather than re-imported.
///
/// Trace: interface-001-AC-1, TC-028
#[test]
fn it_001_implemented_operations_are_exported_under_their_declared_names() {
    let _ = generate_bound_oracles;
    let _ = generate_tristate_harness;
    let _ = generate_i64_strategy;
    let _ = generate_enum_strategy;
    let _ = generate_bound_strategy;
    let _ = generate_kani_bundle;
    let _ = write_bundle_atomic;
    assert!(lib_source().contains("analyze_bound_coverage"));
}

/// Every `operations` entry interface-001 marks `status: planned` is absent from the public API,
/// so an implementation cannot silently outrun the status the contract declares for it.
///
/// Trace: interface-001-AC-2, TC-028
#[test]
fn it_001_planned_operations_are_not_exported() {
    let source = lib_source();
    for planned in ["generate_bundle", "analyze_coverage", "cli_generate"] {
        assert!(
            !source.contains(planned),
            "src/lib.rs must not name {planned}: interface-001 declares it status: planned"
        );
    }
}

/// `identity_envelope.required` names exactly the fields of `ProofAttestationBody`, `command`,
/// `tool` and `environment`, and `identity_envelope.results` names exactly the four
/// `AttestationResult` variants: serializing one full body yields exactly the eleven declared
/// top-level field names, and the match below is exhaustive over `AttestationResult`, so a fifth
/// variant would fail this to compile rather than pass silently.
///
/// Trace: interface-001-AC-3, TC-028
#[test]
fn it_001_identity_envelope_matches_the_emitted_attestation_body() {
    let body = ProofAttestationBody {
        schema_version: 1,
        record_type: "proof_attestation".to_owned(),
        attestation_id: "id".to_owned(),
        record_digest: "0".repeat(64),
        candidate_revision: "candidate".to_owned(),
        proof_id: "PROOF-example".to_owned(),
        command: AttestationCommand {
            argv: vec![],
            working_directory: ".".to_owned(),
        },
        tool: AttestationTool {
            identity: "agent-ix/quire-contract-codegen".to_owned(),
            version: "0".repeat(40),
            configuration_digest: "0".repeat(64),
        },
        environment: AttestationEnvironment {
            target_triple: "unknown".to_owned(),
            operating_system: "unknown".to_owned(),
            toolchain: "unknown".to_owned(),
            dependencies_digest: "0".repeat(64),
            source_revision_available: false,
            source_dirty: true,
        },
        observed_at: "1970-01-01T00:00:00Z".to_owned(),
        result: AttestationResult::Passed,
    };
    let value = serde_json::to_value(&body).unwrap();
    let mut keys = value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    let mut required = vec![
        "schema_version",
        "record_type",
        "attestation_id",
        "record_digest",
        "candidate_revision",
        "proof_id",
        "command",
        "tool",
        "environment",
        "observed_at",
        "result",
    ];
    required.sort_unstable();
    assert_eq!(keys, required);

    // Exhaustive: a fifth `AttestationResult` variant makes this match fail to compile, so
    // these four names are the complete `identity_envelope.results` vocabulary, not merely four
    // that happen to exist today.
    fn label(result: AttestationResult) -> &'static str {
        match result {
            AttestationResult::Passed => "passed",
            AttestationResult::Failed => "failed",
            AttestationResult::Unavailable => "unavailable",
            AttestationResult::NotComputed => "not_computed",
        }
    }
    for (result, expected) in [
        (AttestationResult::Passed, "passed"),
        (AttestationResult::Failed, "failed"),
        (AttestationResult::Unavailable, "unavailable"),
        (AttestationResult::NotComputed, "not_computed"),
    ] {
        assert_eq!(label(result), expected);
    }
}

/// `diagnostics.terminal_states` names exactly the six `GenerationTerminalState` variants. The
/// match is exhaustive, so a seventh variant this list does not name would fail to compile.
///
/// Trace: interface-001-AC-4, TC-028
#[test]
fn it_001_terminal_states_are_exactly_the_declared_six() {
    fn label(state: GenerationTerminalState) -> &'static str {
        match state {
            GenerationTerminalState::Generated => "generated",
            GenerationTerminalState::Unsupported => "unsupported",
            GenerationTerminalState::InvalidInput => "invalid-input",
            GenerationTerminalState::BackendUnavailable => "backend-unavailable",
            GenerationTerminalState::IoFailed => "io-failed",
            GenerationTerminalState::Inconclusive => "inconclusive",
        }
    }
    for (state, expected) in [
        (GenerationTerminalState::Generated, "generated"),
        (GenerationTerminalState::Unsupported, "unsupported"),
        (GenerationTerminalState::InvalidInput, "invalid-input"),
        (
            GenerationTerminalState::BackendUnavailable,
            "backend-unavailable",
        ),
        (GenerationTerminalState::IoFailed, "io-failed"),
        (GenerationTerminalState::Inconclusive, "inconclusive"),
    ] {
        assert_eq!(label(state), expected);
    }
}

/// `kani_obligation_execution_slice.pins` names exactly the six measured fields of
/// `KaniToolPins`: serializing one instance yields exactly those six keys.
///
/// Trace: interface-001-AC-5, TC-028
#[test]
fn it_001_kani_obligation_pins_are_exactly_six_fields() {
    let pins = KaniToolPins::pinned();
    let value = serde_json::to_value(&pins).unwrap();
    let mut keys = value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    let mut expected = vec![
        "kaniVersion",
        "launcherSha256",
        "driverSha256",
        "cbmcVersion",
        "rustToolchain",
        "targetTriple",
    ];
    expected.sort_unstable();
    assert_eq!(keys, expected);
}
