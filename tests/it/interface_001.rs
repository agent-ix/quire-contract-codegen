//! interface-001: the API surface `spec/interface/interface-001-codegen-api.md` declares matches
//! what the crate actually exports and emits. Each test below is a small, independently checkable
//! census over the public surface or a generated value, in the style TC-013 uses for FR-006's
//! absence claims: it reads the crate's own declaration rather than trusting prose to stay in
//! sync with code.
//!
//! Every test also parses the contract document's own fenced YAML block and compares the parsed
//! vocabulary against the Rust vocabulary. For `GenerationTerminalState` and `AttestationResult`
//! the Rust side is `GenerationTerminalState::ALL` and `AttestationResult::ALL`, the census each
//! enum carries beside its own definition in `src/oracle.rs`, rather than a second hand-copied
//! literal living only in this file. A renamed `KaniToolPins` field or an `operations` entry added
//! without updating its export/status caveat drifts the parsed contract text away from this file's
//! own struct-field and string censuses and fails the assertion below. A seventh
//! `GenerationTerminalState` variant or a fifth `AttestationResult` variant fails the build first:
//! `label()` beside each enum is an exhaustive match, so the build breaks until the new variant is
//! named there. That alone never used to carry the new variant into `ALL` too — Rust has no stable
//! way to link an array's contents to an enum's variant set without a proc-macro crate this
//! workspace does not depend on — so a variant added and named in `label()` but never added to
//! `ALL` used to still pass every assertion below. `census_enum_variants` closes that gap from the
//! other direction: it counts the variant identifiers `src/oracle.rs`'s own enum declaration
//! carries between its braces and asserts that count equals `ALL.len()`, so `ALL`, `label()` and
//! the declaration itself cannot drift out of step with each other while still agreeing among
//! themselves. `quire coverage` cannot resolve these criteria as declared rows (see the contract
//! document's Open items), so `interface-001-AC-1` through `interface-001-AC-5` are traced
//! `Test (TC-028)` and backed by this file rather than by a `quire coverage`-verified row.

use std::{fs, path::Path};

use quire_contract_codegen::{
    generate_bound_oracles, generate_bound_strategy, generate_enum_strategy, generate_i64_strategy,
    generate_kani_bundle, generate_tristate_harness, write_bundle_atomic, AttestationCommand,
    AttestationEnvironment, AttestationResult, AttestationTool, GenerationTerminalState,
    KaniToolPins, ProofAttestationBody,
};

fn crate_source(relative_path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path))
        .unwrap_or_else(|_| panic!("{relative_path} must be readable from the crate root"))
}

fn lib_source() -> String {
    crate_source("src/lib.rs")
}

fn oracle_source() -> String {
    crate_source("src/oracle.rs")
}

/// Counts the variant identifiers declared between `pub enum <name> {` and its matching closing
/// `}` in `source`. `label()`'s own exhaustive match already fails the build on an added variant,
/// but nothing carries that variant into `ALL` too -- this closes that gap the other direction,
/// against the enum declaration itself rather than against `label()` or `ALL`, so the three
/// cannot drift into agreement with each other while all three disagree with the real variant
/// count.
///
/// Not a general Rust parser: it only counts bare unit variants (`Ident,`), which is the only
/// shape `GenerationTerminalState` and `AttestationResult` use, and it skips `//`-prefixed and
/// `#`-prefixed lines (comments, doc comments and attributes) entirely -- before looking for
/// either a variant or a brace on that line -- so a brace or comma mentioned in a comment or an
/// attribute cannot desynchronize its `{`/`}` depth tracking or be miscounted as a variant. A
/// future data-carrying variant (`Foo(String),` or `Foo { x: i32 },`) would be silently skipped
/// rather than counted -- correct today, since `ALL: [Self; N]` cannot hold a data-carrying
/// variant either, so both sides would already be wrong the same way -- but is not a shape either
/// enum uses now.
fn census_enum_variants(source: &str, name: &str) -> usize {
    let marker = format!("pub enum {name} {{");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("`pub enum {name} {{` must appear in src/oracle.rs"));
    let mut depth = 1i32;
    let mut count = 0usize;
    for line in source[start + marker.len()..].lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }
        for ch in trimmed.chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 {
            break;
        }
        let is_unit_variant = trimmed.strip_suffix(',').is_some_and(|ident| {
            !ident.is_empty() && ident.chars().all(|c| c.is_alphanumeric() || c == '_')
        });
        if is_unit_variant {
            count += 1;
        }
    }
    assert!(
        count > 0,
        "census_enum_variants found no variants for {name}; the parser likely desynchronized"
    );
    count
}

fn contract_source() -> String {
    crate_source("spec/interface/interface-001-codegen-api.md")
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
/// Trace: interface-001-AC-2, TC-028
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
/// one full body yields exactly the eleven declared top-level field names, and the results side
/// compares each variant's own serde wire form -- what the envelope actually emits -- against the
/// contract, not `AttestationResult::label`'s independent hand-maintained string. Both expected
/// lists are parsed from the contract document itself, not hand-copied; the results side reads
/// `AttestationResult::ALL`, the census declared beside the enum in `src/oracle.rs`, rather than a
/// second hand-typed list local to this file.
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
    let yaml = contract_yaml();
    let required = sorted(parse_flow_list(&yaml, "required: ["));
    assert_eq!(keys, required);

    // The contract says this names the envelope the generator *emits* -- so this compares the
    // serde wire form (what `write_bundle_atomic`'s callers actually see on the wire), not
    // `AttestationResult::label`'s independent hand-maintained string. A `#[serde(rename = ...)]`
    // on one variant would drift the emitted JSON from the contract; comparing `label()` would
    // not catch that, since it is a second, unconnected source of the same vocabulary.
    // `AttestationResult::ALL` is still the census declared beside the enum, so the variant list
    // itself is read from there rather than hand-copied into this file.
    let labels = AttestationResult::ALL
        .into_iter()
        .map(|result| {
            let wire_form = serde_json::to_value(result)
                .expect("AttestationResult must serialize")
                .as_str()
                .expect("AttestationResult serializes as a bare string")
                .to_owned();
            // label() has no other caller once this file compares the wire form instead: without
            // this, a variant whose label() and #[serde(rename_all)] output silently diverged
            // would never be caught by anything. Keeping the two in lockstep here is what makes
            // it safe for label()'s exhaustive match to stay the crate's sole build-time guard
            // against an unnamed new variant (src/oracle.rs's own doc comment on ALL explains why
            // that guard still matters).
            assert_eq!(
                result.label(),
                wire_form,
                "AttestationResult::label() must agree with its own #[serde(rename_all)] output"
            );
            wire_form
        })
        .collect::<Vec<_>>();
    let results = parse_flow_list(&yaml, "results: [");
    assert_eq!(sorted(labels), sorted(results));
    assert_eq!(
        census_enum_variants(&oracle_source(), "AttestationResult"),
        AttestationResult::ALL.len(),
        "AttestationResult::ALL must name every variant src/oracle.rs declares"
    );
}

/// `diagnostics.terminal_states` names exactly the six `GenerationTerminalState` variants, as the
/// contract's serde wire form -- what a `GenerationDiagnostic` actually emits -- not
/// `GenerationTerminalState::label`'s independent hand-maintained string, which a
/// `#[serde(rename = ...)]` could drift away from silently. The Rust side reads
/// `GenerationTerminalState::ALL`, the census declared beside the enum in `src/oracle.rs`, rather
/// than a second hand-typed list local to this file; the expected list is parsed from the contract
/// document itself, not hand-copied.
///
/// Trace: interface-001-AC-4, TC-028
#[test]
fn it_001_terminal_states_are_exactly_the_declared_six() {
    let labels = GenerationTerminalState::ALL
        .into_iter()
        .map(|state| {
            let wire_form = serde_json::to_value(state)
                .expect("GenerationTerminalState must serialize")
                .as_str()
                .expect("GenerationTerminalState serializes as a bare string")
                .to_owned();
            // See the matching comment in it_001_identity_envelope_matches_the_emitted_attestation_body:
            // label() has no other caller once this file compares the wire form instead, so this
            // keeps it from silently drifting unnoticed.
            assert_eq!(
                state.label(),
                wire_form,
                "GenerationTerminalState::label() must agree with its own #[serde(rename_all)] output"
            );
            wire_form
        })
        .collect::<Vec<_>>();
    let terminal_states = parse_flow_list(&contract_yaml(), "terminal_states: [");
    assert_eq!(sorted(labels), sorted(terminal_states));
    assert_eq!(
        census_enum_variants(&oracle_source(), "GenerationTerminalState"),
        GenerationTerminalState::ALL.len(),
        "GenerationTerminalState::ALL must name every variant src/oracle.rs declares"
    );
}

/// `kani_obligation_execution_slice.pins` names exactly the six measured fields of
/// `KaniToolPins`: serializing one instance yields exactly those six keys. The expected list is
/// parsed from the contract document's own `pins: [...]` flow list, not hand-copied.
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
    let declared_pins = sorted(parse_flow_list(&contract_yaml(), "pins: ["));
    assert_eq!(keys, declared_pins);
}
