//! interface-001: the API surface `spec/interface/interface-001-codegen-api.md` declares matches
//! what the crate actually exports and emits. Each test below is a small, independently checkable
//! census over the public surface or a generated value, in the style TC-013 uses for FR-006's
//! absence claims: it reads the crate's own declaration rather than trusting prose to stay in
//! sync with code.
//!
//! Every test also parses the contract document's own fenced YAML block and compares the parsed
//! vocabulary against the Rust vocabulary, rather than a hand-copied literal: a seventh
//! `GenerationTerminalState` variant, a renamed `KaniToolPins` field, or an `operations` entry
//! added without updating its export/status caveat drifts the parsed contract text away from the
//! hardcoded Rust-side census below and fails the assertion, so contract-versus-code drift is
//! actually caught rather than merely restated in two places by hand. `quire coverage` cannot
//! resolve these criteria as declared rows (see the contract document's Open items), so they are
//! recorded `Inspection` there; this file is the real verification.

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

fn contract_source() -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("spec/interface/interface-001-codegen-api.md"),
    )
    .expect("spec/interface/interface-001-codegen-api.md must be readable from the crate root")
}

/// The contract document's one fenced ```yaml block, as text.
fn contract_yaml() -> String {
    let source = contract_source();
    let fence_start = source
        .find("```yaml\n")
        .expect("the contract document must carry a fenced yaml block");
    let body_start = fence_start + "```yaml\n".len();
    let rest = &source[body_start..];
    let body_end = rest
        .find("\n```")
        .expect("the contract document's yaml block must be closed");
    rest[..body_end].to_owned()
}

/// Parses every `operations` entry into `(name, declared_planned)`, reading each `- name:` line
/// and the `status:` line that follows it (if any) up to the next `- name:` or the next
/// unindented top-level key. `declared_planned` is true only when a `status:` value starts with
/// `planned`, matching interface-001-AC-1's and AC-2's own wording.
fn parse_operations(yaml: &str) -> Vec<(String, bool)> {
    let mut operations = Vec::new();
    let mut in_operations = false;
    let mut current: Option<(String, bool)> = None;
    for line in yaml.lines() {
        if line == "operations:" {
            in_operations = true;
            continue;
        }
        if !in_operations {
            continue;
        }
        if !line.is_empty() && !line.starts_with(' ') {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(name) = trimmed.strip_prefix("- name: ") {
            if let Some(entry) = current.take() {
                operations.push(entry);
            }
            current = Some((name.trim().to_owned(), false));
        } else if let Some(status) = trimmed.strip_prefix("status: ") {
            if let Some((name, _)) = current.take() {
                current = Some((name, status.trim_start().starts_with("planned")));
            }
        }
    }
    if let Some(entry) = current.take() {
        operations.push(entry);
    }
    operations
}

/// Parses a `key: [a, b, c]` flow-sequence line into its comma-separated items. `marker` must
/// include the `: [` so a bare `key:` block-list line elsewhere in the document cannot match.
fn parse_flow_list(yaml: &str, marker: &str) -> Vec<String> {
    let line = yaml
        .lines()
        .find(|line| line.trim_start().contains(marker))
        .unwrap_or_else(|| panic!("contract yaml must declare a `{marker}...]` flow list"));
    let start = line
        .find('[')
        .expect("marker guarantees an opening bracket");
    let end = line[start..]
        .find(']')
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("`{marker}` flow list must be closed on one line"));
    line[start + 1..end]
        .split(',')
        .map(|item| item.trim().to_owned())
        .filter(|item| !item.is_empty())
        .collect()
}

fn sorted(mut items: Vec<String>) -> Vec<String> {
    items.sort();
    items
}

/// Every `operations` entry interface-001 declares without a `status: planned` caveat is a real
/// exported function under the exact name the contract gives it. The `use` above naming each of
/// `generate_bound_oracles`, `generate_tristate_harness`, `generate_i64_strategy`,
/// `generate_enum_strategy`, `generate_bound_strategy`, `generate_kani_bundle` and
/// `write_bundle_atomic` is itself the census: a renamed or removed export fails this file to
/// compile at all. `analyze_bound_coverage` is exercised the same way through its own import in
/// `tests/bound_coverage.rs`, so it is censused here by source text rather than re-imported. The
/// hardcoded list below is compared against the contract's own parsed `operations` vocabulary, so
/// a declared operation this list omits — implemented or not censused here — fails the assertion
/// rather than silently outrunning this file.
///
/// Trace: TC-028
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

    let implemented = sorted(
        [
            "generate_bound_oracles",
            "generate_tristate_harness",
            "generate_i64_strategy",
            "generate_enum_strategy",
            "generate_bound_strategy",
            "generate_kani_bundle",
            "write_bundle_atomic",
            "analyze_bound_coverage",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    );
    let declared_implemented = sorted(
        parse_operations(&contract_yaml())
            .into_iter()
            .filter_map(|(name, planned)| (!planned).then_some(name))
            .collect(),
    );
    assert_eq!(
        implemented, declared_implemented,
        "the census above must name exactly the contract's non-planned operations"
    );
}

/// Every `operations` entry interface-001 marks `status: planned` is absent from the public API,
/// so an implementation cannot silently outrun the status the contract declares for it. The
/// hardcoded list below is compared against the contract's own parsed `operations` vocabulary, so
/// a newly planned or newly un-planned operation this list omits fails the assertion.
///
/// Trace: TC-028
#[test]
fn it_001_planned_operations_are_not_exported() {
    let source = lib_source();
    let planned = ["generate_bundle", "analyze_coverage", "cli_generate"];
    for name in planned {
        assert!(
            !source.contains(name),
            "src/lib.rs must not name {name}: interface-001 declares it status: planned"
        );
    }

    let declared_planned = sorted(
        parse_operations(&contract_yaml())
            .into_iter()
            .filter_map(|(name, is_planned)| is_planned.then_some(name))
            .collect(),
    );
    assert_eq!(
        sorted(planned.into_iter().map(str::to_owned).collect()),
        declared_planned,
        "the census above must name exactly the contract's `status: planned` operations"
    );
}

/// `identity_envelope.required` names exactly the fields of `ProofAttestationBody`, and
/// `identity_envelope.results` names exactly the four `AttestationResult` variants: serializing
/// one full body yields exactly the eleven declared top-level field names, and the match below is
/// exhaustive over `AttestationResult`, so a fifth variant would fail this to compile rather than
/// pass silently. Both expected lists are parsed from the contract document itself, not
/// hand-copied.
///
/// Trace: TC-028
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
    let yaml = contract_yaml();
    let required = sorted(parse_flow_list(&yaml, "required: ["));
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
    let labels = [
        AttestationResult::Passed,
        AttestationResult::Failed,
        AttestationResult::Unavailable,
        AttestationResult::NotComputed,
    ]
    .into_iter()
    .map(label)
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let results = parse_flow_list(&yaml, "results: [");
    assert_eq!(sorted(labels), sorted(results));
}

/// `diagnostics.terminal_states` names exactly the six `GenerationTerminalState` variants. The
/// match is exhaustive, so a seventh variant this list does not name would fail to compile. The
/// expected list is parsed from the contract document itself, not hand-copied.
///
/// Trace: TC-028
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
    let labels = [
        GenerationTerminalState::Generated,
        GenerationTerminalState::Unsupported,
        GenerationTerminalState::InvalidInput,
        GenerationTerminalState::BackendUnavailable,
        GenerationTerminalState::IoFailed,
        GenerationTerminalState::Inconclusive,
    ]
    .into_iter()
    .map(label)
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let terminal_states = parse_flow_list(&contract_yaml(), "terminal_states: [");
    assert_eq!(sorted(labels), sorted(terminal_states));
}

/// `kani_obligation_execution_slice.pins` names exactly the six measured fields of
/// `KaniToolPins`: serializing one instance yields exactly those six keys. The expected list is
/// parsed from the contract document's own `pins: [...]` flow list, not hand-copied.
///
/// Trace: TC-028
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
    let declared_pins = sorted(parse_flow_list(&contract_yaml(), "pins: ["));
    assert_eq!(keys, declared_pins);
}
