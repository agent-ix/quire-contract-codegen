use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use common::{packaged_attestation_schema, packaged_attestation_validator, seal_attestation};
use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    generate_boolean_oracle, generator_source_is_dirty, Artifact, AttestationContext,
    AttestationResult, GenerationErrorCode, GenerationTerminalState, OracleRequest,
    ProofAttestationBody, GENERATOR_SOURCE_REVISION, IR_CANDIDATE_REVISION,
    MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, CanonicalProfile, ClauseId, ComparisonOperator,
    DeclarationEnvironment, ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType,
    NumericOperator, OverflowPolicy, PackageId, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, ValueDeclaration, ValueDeclarationKind, ValueType,
};
use sha2::{Digest as _, Sha256};

mod common;

mod generated_boolean_oracle {
    include!("fixtures/generated_boolean_oracle.golden");
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn name(value: &str) -> SymbolName {
    SymbolName::new(value).unwrap()
}

fn span(start: u64, end: u64) -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("oracle-test").unwrap(),
        SourceRevision::new(1).unwrap(),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, start as u32 + 1, start).unwrap(),
        SourceLocation::new(source, 1, end as u32 + 1, end).unwrap(),
    )
    .unwrap()
}

fn requirement() -> RequirementRef {
    RequirementRef::new(
        PackageId::new("agent-ix/oracle-test").unwrap(),
        RequirementId::new("FR-001").unwrap(),
        RequirementRevision::new(7).unwrap(),
    )
}

/// The record digest this test's attestations bind to.
///
/// No change-assurance record is sealed for a unit test, so there is no digest to
/// name. The all-zero digest is the one 64-hexadecimal string no sealed record can
/// have, so it cannot be mistaken for a real binding.
const TEST_RECORD_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn attestation_context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: TEST_RECORD_DIGEST,
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}

fn expected_implementation_digest() -> String {
    let mut hasher = Sha256::new();
    for value in [
        include_bytes!("../src/oracle.rs").as_slice(),
        include_bytes!("../src/harness.rs").as_slice(),
        include_bytes!("../src/strategy.rs").as_slice(),
        include_bytes!("../build.rs").as_slice(),
        include_bytes!("../Cargo.lock").as_slice(),
        include_bytes!("../schemas/oracle-source-map-v1.schema.json").as_slice(),
        include_bytes!("../schemas/generated-rust-oracle-v1.schema.json").as_slice(),
        include_bytes!("../spec/functional/FR-001-deterministic-oracles.md").as_slice(),
        include_bytes!("../spec/functional/FR-002-tristate-proptest.md").as_slice(),
    ] {
        hasher.update(value.len().to_le_bytes());
        hasher.update(value);
    }
    let mut result = String::with_capacity(64);
    for byte in hasher.finalize() {
        let _ = write!(result, "{byte:02x}");
    }
    result
}

fn pre() -> ExecutionPoint {
    ExecutionPoint::Pre {
        operation: AnchorName::new("generate").unwrap(),
    }
}

fn handler() -> ExecutionPoint {
    ExecutionPoint::Handler {
        name: AnchorName::new("generate").unwrap(),
    }
}

fn boolean_environment(names: &[&str]) -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        requirement(),
        vec![],
        names
            .iter()
            .enumerate()
            .map(|(index, value)| {
                ValueDeclaration::new(
                    name(value),
                    ValueDeclarationKind::Input,
                    ValueType::Boolean,
                    span(index as u64, index as u64 + 1),
                )
            })
            .collect(),
        vec![],
    )
    .unwrap()
}

fn differential_environment() -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        requirement(),
        vec![],
        [
            ("a", ValueDeclarationKind::Input),
            ("b", ValueDeclarationKind::Input),
            ("s", ValueDeclarationKind::State),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (value, kind))| {
            ValueDeclaration::new(
                name(value),
                kind,
                ValueType::Boolean,
                span(index as u64, index as u64 + 1),
            )
        })
        .collect(),
        vec![],
    )
    .unwrap()
}

fn boolean(value: bool, at: u64) -> Expression {
    Expression::new(ExpressionKind::BooleanLiteral { value }, span(at, at + 1))
}

fn value(name_value: &str, at: u64) -> Expression {
    observed_value(name_value, StateObservation::Current, at)
}

fn observed_value(name_value: &str, observation: StateObservation, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::ValueReference {
            name: name(name_value),
            observation,
        },
        span(at, at + 1),
    )
}

fn boolean_not(operand: Expression, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::BooleanNot {
            operand: Box::new(operand),
        },
        span(at, at + 1),
    )
}

fn boolean_op(
    operator: BooleanOperator,
    left: Expression,
    right: Expression,
    at: u64,
) -> Expression {
    Expression::new(
        ExpressionKind::Boolean {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        },
        span(at, at + 1),
    )
}

fn balanced_and(name_value: &str, depth: u32, next_span: &mut u64) -> Expression {
    let at = *next_span;
    *next_span += 1;
    if depth == 0 {
        return value(name_value, at);
    }
    let left = balanced_and(name_value, depth - 1, next_span);
    let right = balanced_and(name_value, depth - 1, next_span);
    boolean_op(BooleanOperator::TotalAnd, left, right, at)
}

fn source_symbol(source: &str) -> &str {
    source
        .lines()
        .find_map(|line| line.strip_prefix("pub fn "))
        .and_then(|signature| signature.split('(').next())
        .unwrap()
}

/// Trace: TC-001, NFR-002-AC-1
///
/// NFR-002-AC-1 declares its verification method as Test (TC-001), and this is
/// that test: it asserts that each emitted proof attestation records the tool
/// identity and exact generator revision, the configuration digest over the
/// lowering implementation, the input digest and canonical profile, the backend
/// discriminator, the bounded parameters, the output path with its media type and
/// its schema identity and digest, and the build environment.
///
/// Notes on the two identities asserted differently from the deprecated envelope
/// are in the body, not here: prose in this block is read by the coverage census as
/// trace ids, and an identifier-shaped word written here becomes an unmatched tag.
#[test]
fn tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid() {
    let environment = boolean_environment(&["enabled"]);
    let expression = boolean_op(
        BooleanOperator::Implication,
        value("enabled", 3),
        boolean(false, 4),
        3,
    );
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let clause = ClauseId::new("clause-main").unwrap();
    let request = OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: attestation_context(),
    };

    let first = generate_boolean_oracle(&request).unwrap();
    let second = generate_boolean_oracle(&request).unwrap();

    assert_eq!(first, second);
    assert!(first
        .rust
        .contents
        .starts_with("// SPDX-License-Identifier: MIT OR Apache-2.0\n"));
    assert!(first.rust.contents.contains("FR-001@7"));
    assert!(first.rust.contents.contains("clause-main"));
    assert!(first.rust.contents.contains("enabled_current: bool"));
    assert!(first.rust.contents.contains("implies_short_circuit"));
    assert_eq!(
        first.rust.contents,
        include_str!("fixtures/generated_boolean_oracle.golden")
    );
    assert!(!generated_boolean_oracle::oracle_fr_001_7_clause_main_id_57741ce71ac28bb13911940353c3f67827c9429e1c21cc1becae9119641ab101(true));
    assert!(generated_boolean_oracle::oracle_fr_001_7_clause_main_id_57741ce71ac28bb13911940353c3f67827c9429e1c21cc1becae9119641ab101(false));
    assert_eq!(
        generated_boolean_oracle::ORACLE_FR_001_7_CLAUSE_MAIN_ID_57741CE71AC28BB13911940353C3F67827C9429E1C21CC1BECAE9119641AB101_IDENTITY
            .requirement
            .as_str(),
        "FR-001"
    );
    assert_eq!(
        generated_boolean_oracle::ORACLE_FR_001_7_CLAUSE_MAIN_ID_57741CE71AC28BB13911940353C3F67827C9429E1C21CC1BECAE9119641AB101_CLAUSE.as_str(),
        "clause-main"
    );

    let source_map: Vec<quire_contract_codegen::SourceRegion> =
        serde_json::from_str(&first.source_map.contents).unwrap();
    let source_map_value: serde_json::Value =
        serde_json::from_str(&first.source_map.contents).unwrap();
    let source_map_schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/oracle-source-map-v1.schema.json")).unwrap();
    let source_map_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&source_map_schema)
        .unwrap();
    assert!(source_map_validator.validate(&source_map_value).is_ok());
    assert!(source_map_validator
        .validate(&serde_json::json!({"not": "a source map"}))
        .is_err());
    let consequent = source_map
        .iter()
        .find(|region| region.role == "implication_consequent")
        .unwrap();
    assert_eq!((consequent.start_line, consequent.end_line), (21, 21));
    assert_eq!(
        first
            .rust
            .contents
            .lines()
            .nth(consequent.start_line as usize - 1),
        Some("false")
    );

    let attestation: ProofAttestationBody =
        serde_json::from_str(&first.rust_attestation.contents).unwrap();
    let attestation_value: serde_json::Value =
        serde_json::from_str(&first.rust_attestation.contents).unwrap();
    let source_map_attestation: ProofAttestationBody =
        serde_json::from_str(&first.source_map_attestation.contents).unwrap();

    // A field the shared shape does not declare is refused rather than carried.
    let mut extended = attestation_value.clone();
    extended["unexpected"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<ProofAttestationBody>(extended).is_err());

    // The shared discriminators, and the two fields Quoin derives, which a body
    // must not state. `seal-attestation` refuses a body carrying either, and the
    // control for that refusal is at the end of this test.
    assert_eq!(attestation.schema_version, 1);
    assert_eq!(attestation.record_type, "proof_attestation");
    assert!(attestation_value.get("digest").is_none());
    assert!(attestation_value.get("retained_output").is_none());

    // Identity. One attestation per generated artifact, each naming its own proof
    // obligation, and both derived from the same request.
    assert_eq!(attestation.proof_id, "PROOF-codegen-generated-rust-oracle");
    assert_eq!(
        source_map_attestation.proof_id,
        "PROOF-codegen-oracle-source-map"
    );
    assert_ne!(attestation.proof_id, source_map_attestation.proof_id);
    assert!(attestation
        .attestation_id
        .starts_with("PROOF-codegen-generated-rust-oracle:"));
    assert_eq!(attestation.record_digest, TEST_RECORD_DIGEST);
    assert_eq!(attestation.candidate_revision, IR_CANDIDATE_REVISION);
    assert_eq!(attestation.result, AttestationResult::Passed);
    assert_eq!(
        attestation.observed_at, source_map_attestation.observed_at,
        "both attestations describe one generation and must share its time"
    );
    // `observed_at` is the generator's own source-commit time, frozen at build so
    // that regeneration is byte-identical. It is asserted against that commit
    // rather than merely against itself, and against the RFC 3339 shape the shared
    // schema requires -- the local validator below does not check `format` by
    // default, so a malformed value would otherwise reach the CLI unchallenged.
    let commit_time = Command::new("git")
        .args(["show", "-s", "--format=%cI", "HEAD"])
        .output()
        .unwrap();
    assert!(commit_time.status.success());
    assert_eq!(
        attestation.observed_at,
        String::from_utf8(commit_time.stdout).unwrap().trim()
    );

    // The tool. `tool.version` is the exact generator revision, which is the form
    // the shared schema's `immutable_version` admits alongside a semantic version,
    // and it is the strongest identity available here.
    let git_head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(git_head.status.success());
    assert_eq!(
        attestation.tool.version,
        String::from_utf8(git_head.stdout).unwrap().trim()
    );
    assert_eq!(attestation.tool.version, GENERATOR_SOURCE_REVISION);
    assert_eq!(attestation.tool.identity, "agent-ix/quire-contract-codegen");
    assert_eq!(
        attestation.tool.configuration_digest,
        expected_implementation_digest()
    );

    // The environment, including the two build-identity facts the deprecated
    // envelope carried as namespaced extensions.
    assert_eq!(
        attestation.environment.dependencies_digest,
        sha256(include_bytes!("../Cargo.lock"))
    );
    assert!(!attestation.environment.source_dirty);
    assert!(!generator_source_is_dirty());
    assert!(attestation.environment.source_revision_available);

    // The command. Everything the envelope hid behind `parameters_digest`, the
    // `backend` discriminator, the `inputs[]` entry and the codegen extension is
    // written here in full, so each is asserted by value rather than by digest.
    let argv = &attestation.command.argv;
    assert_eq!(attestation.command.working_directory, ".");
    let flag = |name: &str| -> Option<String> {
        argv.iter()
            .position(|item| item == name)
            .and_then(|at| argv.get(at + 1))
            .cloned()
    };
    // `argv[0]` is the crate name and `tool.identity` is the owning repository. The
    // two differ on purpose: an argv is a program name, and the repository slug is
    // where a reader resolves `tool.version` from.
    assert_eq!(argv[0], "quire-contract-codegen");
    assert_eq!(argv[1], "generate_boolean_oracle");
    assert_eq!(flag("--requirement").as_deref(), Some("FR-001@7"));
    assert_eq!(flag("--clause").as_deref(), Some("clause-main"));
    assert_eq!(flag("--backend").as_deref(), Some("none"));
    assert_eq!(
        flag("--ir-revision").as_deref(),
        Some(IR_CANDIDATE_REVISION)
    );
    assert_eq!(
        flag("--runtime-revision").as_deref(),
        Some(RUNTIME_REVISION)
    );
    // Input identity. The deprecated envelope carried this as an `inputs[]` entry
    // whose `content_digest` nothing checked, beside a `schema.digest` that was the
    // SHA-256 of the profile *name* rather than of any schema. Both surviving
    // values are recomputed here from the same canonical expression the generator
    // lowered, so an input digest naming different bytes is caught rather than
    // merely being present.
    let canonical = typed.canonical_expression(CanonicalProfile::V1).unwrap();
    assert_eq!(
        flag("--canonical-profile").as_deref(),
        Some(CanonicalProfile::V1.as_str())
    );
    assert_eq!(
        flag("--input-digest"),
        Some(sha256(canonical.bytes().as_slice()))
    );
    assert_eq!(
        flag("--expression-canonical-digest"),
        Some(canonical.digest().to_string())
    );
    // One generation, two artifacts, one input: the source-map attestation must
    // name the same input as the Rust one, or the pair does not describe a single
    // derivation.
    let source_map_input = {
        let items = &source_map_attestation.command.argv;
        items
            .iter()
            .position(|item| item == "--input-digest")
            .and_then(|at| items.get(at + 1))
            .cloned()
    };
    assert_eq!(flag("--input-digest"), source_map_input);
    assert_eq!(
        flag("--maximum-source-bytes"),
        Some(MAX_GENERATED_SOURCE_BYTES.to_string())
    );
    assert_eq!(flag("--output").as_deref(), Some(first.rust.path.as_str()));
    assert_eq!(flag("--output-media-type").as_deref(), Some("text/x-rust"));
    assert_eq!(
        flag("--output-schema").as_deref(),
        Some("quire.codegen.rust-oracle/v1")
    );
    assert_eq!(
        flag("--output-schema-digest"),
        Some(sha256(include_bytes!(
            "../schemas/generated-rust-oracle-v1.schema.json"
        )))
    );
    let source_map_argv = &source_map_attestation.command.argv;
    let source_map_flag = |name: &str| -> Option<String> {
        source_map_argv
            .iter()
            .position(|item| item == name)
            .and_then(|at| source_map_argv.get(at + 1))
            .cloned()
    };
    assert_eq!(
        source_map_flag("--output").as_deref(),
        Some(first.source_map.path.as_str())
    );
    assert_eq!(
        source_map_flag("--output-schema").as_deref(),
        Some("quire.codegen.oracle-source-map/v1")
    );
    assert_eq!(
        source_map_flag("--output-media-type").as_deref(),
        Some("application/json")
    );
    assert_eq!(
        source_map_flag("--output-schema-digest"),
        Some(sha256(include_bytes!(
            "../schemas/oracle-source-map-v1.schema.json"
        )))
    );

    // What the shared shape does not carry, asserted absent rather than left to a
    // reader's memory. Reviewer identity belongs to the ix-flow decision event a
    // verification receipt binds; requirement references belong to the record's own
    // proof obligations; the free-text result summary and the contribution method
    // have no field in any of the three packaged schemas and are dropped outright.
    // `summary` was missing from this list in the form an adversarial review read,
    // which left one of the four documented drops unprobed.
    for absent in [
        "reviewer",
        "contribution",
        "summary",
        "requirement_refs",
        "requirementRefs",
    ] {
        assert!(
            !first.rust_attestation.contents.contains(absent),
            "the emitted attestation still carries {absent}"
        );
    }

    // The caller's binding is carried through, measured differentially rather than
    // by comparing it to a constant.
    //
    // Every accepting site in this repository supplies the all-zero record digest
    // and `IR_CANDIDATE_REVISION`, both of which the generator already has. An
    // adversarial review replaced `context.record_digest` with `"0".repeat(64)` and
    // `context.candidate_revision` with `IR_CANDIDATE_REVISION` -- the caller's
    // binding ignored completely -- and every test in this repository stayed green
    // and the conformance corpus passed 9 of 9. A value check cannot see that. Two
    // generations with two different bindings can, whatever constants either uses.
    let other = AttestationContext {
        record_digest: "ab".repeat(32).leak(),
        candidate_revision: "9".repeat(48).leak(),
    };
    assert_ne!(other.record_digest, TEST_RECORD_DIGEST);
    assert_ne!(other.candidate_revision, IR_CANDIDATE_REVISION);
    let rebound = generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: other,
    })
    .unwrap();
    let rebound_attestation: ProofAttestationBody =
        serde_json::from_str(&rebound.rust_attestation.contents).unwrap();
    assert_eq!(rebound_attestation.record_digest, other.record_digest);
    assert_eq!(
        rebound_attestation.candidate_revision,
        other.candidate_revision
    );
    // And only those two fields moved: the binding is carried, not mixed into the
    // derivation. The artifacts themselves are identical, which is also the reason
    // the attestation paths collide and neither overwrites anything it should not.
    assert_eq!(rebound.rust, first.rust);
    assert_eq!(rebound.source_map, first.source_map);
    assert_eq!(rebound.rust_attestation.path, first.rust_attestation.path);
    assert_eq!(rebound_attestation.command, attestation.command);
    assert_eq!(rebound_attestation.tool, attestation.tool);
    assert_eq!(rebound_attestation.environment, attestation.environment);
    assert_eq!(
        rebound_attestation.attestation_id,
        attestation.attestation_id
    );

    // The generated Rust still validates against its own domain output contract.
    let rust_schema: serde_json::Value = serde_json::from_str(include_str!(
        "../schemas/generated-rust-oracle-v1.schema.json"
    ))
    .unwrap();
    let rust_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&rust_schema)
        .unwrap();
    assert!(rust_validator
        .validate(&serde_json::Value::String(first.rust.contents.clone()))
        .is_ok());
    assert!(rust_validator
        .validate(&serde_json::Value::String("fn wrong() {}".to_owned()))
        .is_err());

    // And the whole point: the emitted bodies are the shared shape, measured by
    // sealing them through the pinned Quoin CLI and validating what comes back
    // against the schema Quoin itself publishes.
    let directory = TemporaryDirectory::new("quire-codegen-attestation");
    let schema = packaged_attestation_schema();
    let validator = packaged_attestation_validator(&schema);

    for (body, output) in [
        (&first.rust_attestation, &first.rust),
        (&first.source_map_attestation, &first.source_map),
    ] {
        let (code, stdout, stderr) = seal_attestation(&body.contents, output, &directory.0);
        assert_eq!(
            code, 0,
            "quoin refused the emitted attestation body for {}: {stderr}",
            output.path
        );
        let sealed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        assert!(
            validator.validate(&sealed).is_ok(),
            "the sealed attestation for {} does not validate against the packaged schema: {}",
            output.path,
            stdout
        );
        // The sealed form binds the exact bytes of the artifact it accompanies.
        // That binding is Quoin's to make, and this is where it is measured.
        assert_eq!(
            sealed["retained_output"]["size_bytes"].as_u64().unwrap() as usize,
            output.contents.len()
        );

        // A control for the binding. One appended byte must move the retained
        // digest and the size; without this the assertion above is satisfied by a
        // sealer that hashes nothing.
        let mutated = Artifact {
            path: output.path.clone(),
            contents: format!("{}\n", output.contents),
            sha256: output.sha256.clone(),
        };
        let (code, mutated_stdout, stderr) =
            seal_attestation(&body.contents, &mutated, &directory.0);
        assert_eq!(code, 0, "sealing over the mutated bytes failed: {stderr}");
        let mutated_sealed: serde_json::Value = serde_json::from_str(&mutated_stdout).unwrap();
        assert_ne!(
            sealed["retained_output"]["digest"], mutated_sealed["retained_output"]["digest"],
            "one appended byte did not change the retained-output digest"
        );

        // Sealing the same body over the same bytes twice is the same binding.
        // Without this, "one appended byte moves the digest" is also satisfied by a
        // sealer that returns a fresh random value every call.
        let (code, repeated_stdout, stderr) =
            seal_attestation(&body.contents, output, &directory.0);
        assert_eq!(code, 0, "re-sealing the same body failed: {stderr}");
        let repeated: serde_json::Value = serde_json::from_str(&repeated_stdout).unwrap();
        assert_eq!(sealed, repeated, "sealing is not deterministic");
    }

    // The two artifacts of one bundle must not seal to the same retained binding.
    let (_, rust_sealed, _) =
        seal_attestation(&first.rust_attestation.contents, &first.rust, &directory.0);
    let (_, map_sealed, _) = seal_attestation(
        &first.source_map_attestation.contents,
        &first.source_map,
        &directory.0,
    );
    let rust_sealed: serde_json::Value = serde_json::from_str(&rust_sealed).unwrap();
    let map_sealed: serde_json::Value = serde_json::from_str(&map_sealed).unwrap();
    assert_ne!(
        rust_sealed["retained_output"]["digest"], map_sealed["retained_output"]["digest"],
        "the two artifacts of one bundle sealed to the same retained digest"
    );
    assert_eq!(rust_sealed["retained_output"]["media_type"], "text/x-rust");
    assert_eq!(
        map_sealed["retained_output"]["media_type"],
        "application/json"
    );

    // The negative control for the validator itself. A schema that accepts
    // everything would have made every assertion above vacuous.
    let (_, stdout, _) =
        seal_attestation(&first.rust_attestation.contents, &first.rust, &directory.0);
    let mut broken: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    broken["record_digest"] = serde_json::Value::String("not-a-digest".to_owned());
    assert!(
        validator.validate(&broken).is_err(),
        "the packaged schema accepted a record_digest that is not a digest"
    );
    let mut untimed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    untimed["observed_at"] = serde_json::Value::String("Mon Aug 31 12:00:00 2026 +0000".to_owned());
    assert!(
        validator.validate(&untimed).is_err(),
        "the local validator accepted an observed_at that is not RFC 3339; the CLI \
         refuses one, so a build emitting it would fail only downstream"
    );

    // And the reason the emitted body omits `digest` and `retained_output`: Quoin
    // refuses a body that states either. This is what makes the omission the
    // shared contract rather than a local shortcut.
    let mut oversupplied: serde_json::Value = attestation_value.clone();
    oversupplied["retained_output"] = serde_json::json!({
        "media_type": "text/x-rust",
        "digest": "0".repeat(64),
        "size_bytes": 1,
    });
    let (code, _, stderr) = seal_attestation(
        &serde_json::to_string(&oversupplied).unwrap(),
        &first.rust,
        &directory.0,
    );
    assert_ne!(
        code, 0,
        "quoin accepted a body supplying retained_output, so omitting it is not a contract"
    );
    assert!(
        !stderr.trim().is_empty(),
        "quoin refused the oversupplied body without saying why"
    );
}

#[derive(Clone)]
enum ModelExpression {
    Literal(bool),
    A,
    B,
    SCurrent,
    SPre,
    SPost,
    Not(Box<Self>),
    Binary(BooleanOperator, Box<Self>, Box<Self>),
}

impl ModelExpression {
    fn evaluate(&self, a: bool, b: bool, s_current: bool, s_pre: bool, s_post: bool) -> bool {
        match self {
            Self::Literal(value) => *value,
            Self::A => a,
            Self::B => b,
            Self::SCurrent => s_current,
            Self::SPre => s_pre,
            Self::SPost => s_post,
            Self::Not(value) => !value.evaluate(a, b, s_current, s_pre, s_post),
            Self::Binary(operator, left, right) => {
                let left = left.evaluate(a, b, s_current, s_pre, s_post);
                let right = right.evaluate(a, b, s_current, s_pre, s_post);
                match operator {
                    BooleanOperator::ShortCircuitAnd | BooleanOperator::TotalAnd => left && right,
                    BooleanOperator::ShortCircuitOr | BooleanOperator::TotalOr => left || right,
                    BooleanOperator::Implication => !left || right,
                }
            }
        }
    }
}

fn model_expression(model: &ModelExpression, next_span: &mut u64) -> Expression {
    let at = *next_span;
    *next_span += 1;
    match model {
        ModelExpression::Literal(value) => boolean(*value, at),
        ModelExpression::A => value("a", at),
        ModelExpression::B => value("b", at),
        ModelExpression::SCurrent => observed_value("s", StateObservation::Current, at),
        ModelExpression::SPre => observed_value("s", StateObservation::Pre, at),
        ModelExpression::SPost => observed_value("s", StateObservation::Post, at),
        ModelExpression::Not(operand) => {
            let operand = model_expression(operand, next_span);
            boolean_not(operand, at)
        }
        ModelExpression::Binary(operator, left, right) => {
            let left = model_expression(left, next_span);
            let right = model_expression(right, next_span);
            boolean_op(*operator, left, right, at)
        }
    }
}

/// TC-002.
#[test]
fn tc_002_supported_boolean_grammar_compiles_and_matches_an_independent_evaluator() {
    let environment = differential_environment();
    let leaves = vec![
        ModelExpression::Literal(false),
        ModelExpression::Literal(true),
        ModelExpression::A,
        ModelExpression::B,
        ModelExpression::SCurrent,
        ModelExpression::SPre,
        ModelExpression::SPost,
    ];
    let operators = [
        BooleanOperator::ShortCircuitAnd,
        BooleanOperator::ShortCircuitOr,
        BooleanOperator::TotalAnd,
        BooleanOperator::TotalOr,
        BooleanOperator::Implication,
    ];
    let mut corpus = leaves.clone();
    corpus.extend(
        leaves
            .iter()
            .cloned()
            .map(|value| ModelExpression::Not(Box::new(value))),
    );
    for operator in operators {
        for left in &leaves {
            for right in &leaves {
                corpus.push(ModelExpression::Binary(
                    operator,
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ));
            }
        }
    }
    corpus.push(ModelExpression::Not(Box::new(ModelExpression::Binary(
        BooleanOperator::Implication,
        Box::new(ModelExpression::Binary(
            BooleanOperator::TotalAnd,
            Box::new(ModelExpression::A),
            Box::new(ModelExpression::B),
        )),
        Box::new(ModelExpression::Binary(
            BooleanOperator::ShortCircuitOr,
            Box::new(ModelExpression::B),
            Box::new(ModelExpression::Literal(false)),
        )),
    ))));
    for operator in operators {
        corpus.push(ModelExpression::Binary(
            operator,
            Box::new(ModelExpression::Not(Box::new(ModelExpression::SPre))),
            Box::new(ModelExpression::Binary(
                BooleanOperator::ShortCircuitOr,
                Box::new(ModelExpression::A),
                Box::new(ModelExpression::SPost),
            )),
        ));
    }
    assert_eq!(corpus.len(), 265);

    let mut generated_program =
        String::from("#![deny(missing_docs)]\n//! Differential generated-oracle corpus.\n");
    let mut operator_sources = BTreeSet::new();
    let mut next_span = 100;
    for (index, model) in corpus.iter().enumerate() {
        let expression = model_expression(model, &mut next_span);
        let typed = environment
            .check_expression(&expression, &ValueType::Boolean, &handler(), true)
            .unwrap();
        let clause = ClauseId::new(format!("differential-{index:03}")).unwrap();
        let bundle = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &clause,
            expression: &typed,
            attestation: attestation_context(),
        })
        .unwrap();
        let symbol = source_symbol(&bundle.rust.contents).to_owned();
        if let ModelExpression::Binary(operator, _, _) = model {
            let expected_function = match operator {
                BooleanOperator::ShortCircuitAnd => "and_short_circuit",
                BooleanOperator::ShortCircuitOr => "or_short_circuit",
                BooleanOperator::TotalAnd => "and_total",
                BooleanOperator::TotalOr => "or_total",
                BooleanOperator::Implication => "implies_short_circuit",
            };
            assert!(bundle.rust.contents.contains(&format!(
                "quire_contract_runtime::operators::{expected_function}("
            )));
        }
        for function in [
            "and_short_circuit",
            "or_short_circuit",
            "and_total",
            "or_total",
            "implies_short_circuit",
        ] {
            if bundle.rust.contents.contains(function) {
                operator_sources.insert(function);
            }
        }
        if matches!(model, ModelExpression::Not(_)) {
            assert!(bundle.rust.contents.contains("!(\n"));
        }
        let expected_state_parameter = match model {
            ModelExpression::SCurrent => Some("s_current: bool"),
            ModelExpression::SPre => Some("s_pre: bool"),
            ModelExpression::SPost => Some("s_post: bool"),
            _ => None,
        };
        if let Some(parameter) = expected_state_parameter {
            assert!(bundle.rust.contents.contains(parameter));
        }
        generated_program.push_str(&bundle.rust.contents);
        assert!(bundle.rust.contents.contains(&symbol));
    }
    assert_eq!(operator_sources.len(), 5);
    generated_program.push_str("fn main() {\n");
    next_span = 100;
    for (index, model) in corpus.iter().enumerate() {
        let expression = model_expression(model, &mut next_span);
        let typed = environment
            .check_expression(&expression, &ValueType::Boolean, &handler(), true)
            .unwrap();
        let clause = ClauseId::new(format!("differential-{index:03}")).unwrap();
        let bundle = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &clause,
            expression: &typed,
            attestation: attestation_context(),
        })
        .unwrap();
        let symbol = source_symbol(&bundle.rust.contents);
        let parameters = typed
            .dependencies()
            .iter()
            .map(|dependency| {
                (
                    dependency.path()[0].as_str(),
                    dependency
                        .observation()
                        .unwrap_or(StateObservation::Current),
                )
            })
            .collect::<Vec<_>>();
        for a in [false, true] {
            for b in [false, true] {
                for s_current in [false, true] {
                    for s_pre in [false, true] {
                        for s_post in [false, true] {
                            let arguments = parameters
                                .iter()
                                .map(
                                    |(parameter, observation)| match (*parameter, *observation) {
                                        ("a", StateObservation::Current) => a.to_string(),
                                        ("b", StateObservation::Current) => b.to_string(),
                                        ("s", StateObservation::Current) => s_current.to_string(),
                                        ("s", StateObservation::Pre) => s_pre.to_string(),
                                        ("s", StateObservation::Post) => s_post.to_string(),
                                        other => {
                                            panic!("unexpected differential parameter {other:?}")
                                        }
                                    },
                                )
                                .collect::<Vec<_>>()
                                .join(", ");
                            generated_program.push_str(&format!(
                                "assert_eq!({symbol}({arguments}), {});\n",
                                model.evaluate(a, b, s_current, s_pre, s_post)
                            ));
                        }
                    }
                }
            }
        }
    }
    generated_program.push_str("}\n");

    let directory = TemporaryDirectory::new("quire-codegen-differential");
    let source_directory = directory.0.join("src");
    fs::create_dir_all(&source_directory).unwrap();
    let source_path = source_directory.join("main.rs");
    fs::write(&source_path, generated_program).unwrap();
    fs::write(
        directory.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-oracle-differential\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .unwrap();
    let execution = Command::new("cargo")
        .args(["run", "--offline", "--quiet"])
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert!(
        execution.status.success(),
        "generated corpus did not compile and execute against runtime {RUNTIME_REVISION}: {}",
        String::from_utf8_lossy(&execution.stderr)
    );
}

/// TC-001.
#[test]
fn tc_001_every_implication_has_an_exact_unaliased_consequent_region() {
    let environment = boolean_environment(&["a", "b", "implies_short_circuit"]);
    let expression = boolean_op(
        BooleanOperator::TotalAnd,
        boolean_op(
            BooleanOperator::Implication,
            value("a", 10),
            value("b", 11),
            10,
        ),
        boolean_op(
            BooleanOperator::Implication,
            value("b", 12),
            value("a", 13),
            12,
        ),
        10,
    );
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let clause = ClauseId::new("two-implications").unwrap();
    let bundle = generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: attestation_context(),
    })
    .unwrap();
    let regions: Vec<quire_contract_codegen::SourceRegion> =
        serde_json::from_str(&bundle.source_map.contents).unwrap();
    let consequences = regions
        .iter()
        .filter(|region| region.role == "implication_consequent")
        .collect::<Vec<_>>();
    assert_eq!(consequences.len(), 2);
    let lines = bundle.rust.contents.lines().collect::<Vec<_>>();
    assert_eq!(
        consequences
            .iter()
            .map(|region| {
                assert_eq!(region.start_line, region.end_line);
                lines[region.start_line as usize - 1]
            })
            .collect::<Vec<_>>(),
        vec!["b_current", "a_current"]
    );

    let alias = boolean_op(
        BooleanOperator::Implication,
        value("implies_short_circuit", 20),
        boolean(false, 21),
        20,
    );
    let typed_alias = environment
        .check_expression(&alias, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let alias_clause = ClauseId::new("marker-alias").unwrap();
    let alias_bundle = generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &alias_clause,
        expression: &typed_alias,
        attestation: attestation_context(),
    })
    .unwrap();
    let alias_regions: Vec<quire_contract_codegen::SourceRegion> =
        serde_json::from_str(&alias_bundle.source_map.contents).unwrap();
    let consequent = alias_regions
        .iter()
        .find(|region| region.role == "implication_consequent")
        .unwrap();
    assert_eq!(
        alias_bundle
            .rust
            .contents
            .lines()
            .nth(consequent.start_line as usize - 1),
        Some("false")
    );
}

/// TC-003.
#[test]
fn tc_003_unsupported_expression_and_root_map_to_declared_terminal_states() {
    let integer = IntegerType::new(IntegerDomain::Signed, -10, 10, OverflowPolicy::Reject).unwrap();
    let environment = DeclarationEnvironment::new(requirement(), vec![], vec![], vec![]).unwrap();
    let expression = Expression::new(
        ExpressionKind::Compare {
            operator: ComparisonOperator::Equal,
            left: Box::new(Expression::new(
                ExpressionKind::IntegerLiteral {
                    value: 1,
                    value_type: integer.clone(),
                },
                span(30, 31),
            )),
            right: Box::new(Expression::new(
                ExpressionKind::IntegerLiteral {
                    value: 1,
                    value_type: integer.clone(),
                },
                span(31, 32),
            )),
        },
        span(30, 32),
    );
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let clause = ClauseId::new("unsupported").unwrap();
    let diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: attestation_context(),
    })
    .unwrap_err()[0];
    assert_eq!(diagnostic.code, GenerationErrorCode::UnsupportedExpression);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::Unsupported
    );

    let integer_root = Expression::new(
        ExpressionKind::IntegerLiteral {
            value: 1,
            value_type: integer.clone(),
        },
        span(33, 34),
    );
    let typed_root = environment
        .check_expression(&integer_root, &ValueType::integer(integer), &pre(), false)
        .unwrap();
    let root_clause = ClauseId::new("wrong-root").unwrap();
    let root_diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &root_clause,
        expression: &typed_root,
        attestation: attestation_context(),
    })
    .unwrap_err()[0];
    assert_eq!(root_diagnostic.code, GenerationErrorCode::NonBooleanRoot);
    assert_eq!(
        root_diagnostic.terminal_state,
        GenerationTerminalState::InvalidInput
    );

    assert_eq!(
        GenerationErrorCode::InvalidGeneratedSyntax.terminal_state(),
        GenerationTerminalState::Inconclusive
    );
    assert_eq!(
        GenerationErrorCode::SerializationFailed.terminal_state(),
        GenerationTerminalState::Inconclusive
    );

    // Every rule the attestation binding enforces, probed one at a time.
    //
    // The previous form set six bad fields at once, which shows only that *some*
    // rule fired. A single surviving rule satisfied it while the other five were
    // gone, so each rule now gets a context that is valid apart from the one thing
    // it is about, and the valid context is required to be accepted so that the
    // probes are not all passing for the wrong reason.
    let valid = attestation_context();
    assert!(
        generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &clause,
            expression: &typed,
            attestation: valid,
        })
        .is_err(),
        "this clause is unsupported, so it must be rejected for that and not for its binding"
    );
    for (name, invalid) in [
        (
            "a record digest that is not hexadecimal",
            AttestationContext {
                record_digest: "not-a-digest",
                ..valid
            },
        ),
        (
            "a record digest of the wrong length",
            AttestationContext {
                record_digest: "0123456789abcdef",
                ..valid
            },
        ),
        (
            "an uppercase record digest",
            AttestationContext {
                record_digest: &"A".repeat(64),
                ..valid
            },
        ),
        (
            "an empty record digest",
            AttestationContext {
                record_digest: "",
                ..valid
            },
        ),
        (
            "a candidate revision that is not a revision",
            AttestationContext {
                candidate_revision: "not-a-revision",
                ..valid
            },
        ),
        (
            "a candidate revision that is too short",
            AttestationContext {
                candidate_revision: "abcdef",
                ..valid
            },
        ),
        (
            "an empty candidate revision",
            AttestationContext {
                candidate_revision: "",
                ..valid
            },
        ),
        (
            "an over-length record digest",
            AttestationContext {
                record_digest: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0",
                ..valid
            },
        ),
        (
            "an over-length candidate revision",
            AttestationContext {
                candidate_revision:
                    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0",
                ..valid
            },
        ),
    ] {
        // A supported clause, so the only thing that can reject it is the binding.
        // With an unsupported clause every probe would pass on the clause's own
        // rejection and none of them would be about the binding at all.
        let supported_environment = boolean_environment(&["enabled"]);
        let supported = supported_environment
            .check_expression(&value("enabled", 60), &ValueType::Boolean, &pre(), true)
            .unwrap();
        let supported_clause = ClauseId::new("binding-probe").unwrap();
        let supported_request = |attestation| OracleRequest {
            requirement: supported_environment.owner(),
            clause: &supported_clause,
            expression: &supported,
            attestation,
        };
        assert!(
            generate_boolean_oracle(&supported_request(valid)).is_ok(),
            "the probe clause must generate under a valid binding, or every probe \
             below passes for the wrong reason"
        );
        let diagnostics = generate_boolean_oracle(&supported_request(invalid))
            .err()
            .unwrap_or_else(|| panic!("{name} was accepted"));
        assert_eq!(
            diagnostics[0].code,
            GenerationErrorCode::InvalidAttestationContext,
            "{name} produced {:?}",
            diagnostics[0].code
        );
        assert_eq!(
            diagnostics[0].terminal_state,
            GenerationTerminalState::InvalidInput,
            "{name} reached {:?}",
            diagnostics[0].terminal_state
        );
        assert_eq!(diagnostics[0].path, "attestation.context", "{name}");
    }
}

/// TC-003.
#[test]
fn tc_003_normalization_is_injective_for_dependencies_and_clause_artifacts() {
    let environment = boolean_environment(&["enabled-flag", "enabled_flag"]);
    let expression = boolean_op(
        BooleanOperator::TotalAnd,
        value("enabled-flag", 40),
        value("enabled_flag", 41),
        40,
    );
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let clause = ClauseId::new("colliding-dependencies").unwrap();
    let bundle = generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: attestation_context(),
    })
    .unwrap();
    assert!(bundle.rust.contents.contains("enabled_2dflag_current"));
    assert!(bundle.rust.contents.contains("enabled_5fflag_current"));

    let literal_environment = boolean_environment(&[]);
    let literal = boolean(true, 42);
    let typed_literal = literal_environment
        .check_expression(&literal, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let mut rust_paths = BTreeSet::new();
    let mut map_paths = BTreeSet::new();
    let mut rust_attestation_paths = BTreeSet::new();
    let mut source_map_attestation_paths = BTreeSet::new();
    for clause_value in ["Clause-A", "clause_a", "clause-a", "CLAUSE.A"] {
        let clause = ClauseId::new(clause_value).unwrap();
        let bundle = generate_boolean_oracle(&OracleRequest {
            requirement: literal_environment.owner(),
            clause: &clause,
            expression: &typed_literal,
            attestation: attestation_context(),
        })
        .unwrap();
        rust_paths.insert(bundle.rust.path);
        map_paths.insert(bundle.source_map.path);
        rust_attestation_paths.insert(bundle.rust_attestation.path);
        source_map_attestation_paths.insert(bundle.source_map_attestation.path);
    }
    assert_eq!(rust_paths.len(), 4);
    assert_eq!(map_paths.len(), 4);
    assert_eq!(rust_attestation_paths.len(), 4);
    assert_eq!(source_map_attestation_paths.len(), 4);
    // The two attestations of one bundle must not claim the same path either. A
    // single generation now emits two of them, so collision within a bundle is a
    // new way for one to overwrite the other.
    assert!(rust_attestation_paths.is_disjoint(&source_map_attestation_paths));

    let long_clause = ClauseId::new("x".repeat(400)).unwrap();
    let long_bundle = generate_boolean_oracle(&OracleRequest {
        requirement: literal_environment.owner(),
        clause: &long_clause,
        expression: &typed_literal,
        attestation: attestation_context(),
    })
    .unwrap();
    for path in [
        &long_bundle.rust.path,
        &long_bundle.source_map.path,
        &long_bundle.rust_attestation.path,
        &long_bundle.source_map_attestation.path,
    ] {
        assert!(path.rsplit('/').next().unwrap().len() <= 255, "{path}");
    }
}

/// TC-003.
#[test]
fn tc_003_discharged_obligations_are_explicitly_rejected() {
    let integer = IntegerType::new(IntegerDomain::Signed, -10, 10, OverflowPolicy::Reject).unwrap();
    let environment = DeclarationEnvironment::new(
        requirement(),
        vec![],
        vec![ValueDeclaration::new(
            name("divisor"),
            ValueDeclarationKind::Input,
            ValueType::integer(integer.clone()),
            span(50, 51),
        )],
        vec![],
    )
    .unwrap();
    let divisor = Expression::new(
        ExpressionKind::ValueReference {
            name: name("divisor"),
            observation: StateObservation::Current,
        },
        span(51, 52),
    );
    let integer_literal = |value, at| {
        Expression::new(
            ExpressionKind::IntegerLiteral {
                value,
                value_type: integer.clone(),
            },
            span(at, at + 1),
        )
    };
    let nonzero = Expression::new(
        ExpressionKind::Compare {
            operator: ComparisonOperator::NotEqual,
            left: Box::new(divisor.clone()),
            right: Box::new(integer_literal(0, 52)),
        },
        span(51, 53),
    );
    let division = Expression::new(
        ExpressionKind::Numeric {
            operator: NumericOperator::Divide,
            left: Box::new(integer_literal(10, 53)),
            right: Box::new(divisor),
        },
        span(53, 55),
    );
    let bounded = Expression::new(
        ExpressionKind::Compare {
            operator: ComparisonOperator::LessEqual,
            left: Box::new(division),
            right: Box::new(integer_literal(10, 55)),
        },
        span(53, 56),
    );
    let guarded = boolean_op(BooleanOperator::ShortCircuitAnd, nonzero, bounded, 51);
    let typed = environment
        .check_expression(&guarded, &ValueType::Boolean, &pre(), true)
        .unwrap();
    assert!(!typed.obligations().is_empty());
    let clause = ClauseId::new("guarded-division").unwrap();
    let diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        attestation: attestation_context(),
    })
    .unwrap_err()[0];
    assert_eq!(diagnostic.code, GenerationErrorCode::UnsupportedObligations);
    assert_eq!(diagnostic.path, "expression.obligations");
}

/// TC-001.
#[test]
fn tc_001_deep_expression_output_is_linear_and_bounded() {
    std::thread::Builder::new()
        .name("deep-oracle-test".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let environment = boolean_environment(&["a"]);
            let mut expression = value("a", 1000);
            for offset in 0..200 {
                expression = boolean_not(expression, 1001 + offset);
            }
            let typed = environment
                .check_expression(&expression, &ValueType::Boolean, &pre(), true)
                .unwrap();
            let clause = ClauseId::new("deep-not-chain").unwrap();
            let bundle = generate_boolean_oracle(&OracleRequest {
                requirement: environment.owner(),
                clause: &clause,
                expression: &typed,
                attestation: attestation_context(),
            })
            .unwrap();
            assert!(bundle.rust.contents.len() < 16_384);
            assert!(bundle.rust.contents.len() <= MAX_GENERATED_SOURCE_BYTES);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// TC-003.
#[test]
fn tc_003_source_size_limit_rejects_without_a_partial_bundle() {
    std::thread::Builder::new()
        .name("resource-limit-test".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(|| {
            let long_name = "state".repeat(100);
            let environment = boolean_environment(&[&long_name]);
            let mut next_span = 2_000;
            let expression = balanced_and(&long_name, 11, &mut next_span);
            let typed = environment
                .check_expression(&expression, &ValueType::Boolean, &pre(), true)
                .unwrap();
            let clause = ClauseId::new("resource-limit").unwrap();
            let diagnostics = generate_boolean_oracle(&OracleRequest {
                requirement: environment.owner(),
                clause: &clause,
                expression: &typed,
                attestation: attestation_context(),
            })
            .unwrap_err();
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(
                diagnostics[0].code,
                GenerationErrorCode::ResourceLimitExceeded
            );
            assert_eq!(
                diagnostics[0].terminal_state,
                GenerationTerminalState::Unsupported
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

/// TC-001.
#[test]
fn tc_001_build_metadata_degrades_explicitly_outside_git() {
    let directory = TemporaryDirectory::new("quire-codegen-archive-build");
    let build_binary = directory.0.join("build-script");
    let compilation = Command::new("rustc")
        .args(["--edition=2021", "build.rs", "-o"])
        .arg(&build_binary)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(
        compilation.status.success(),
        "build script did not compile: {}",
        String::from_utf8_lossy(&compilation.stderr)
    );
    let archive_revision = "1234567890abcdef1234567890abcdef12345678";
    let execution = Command::new(&build_binary)
        .current_dir(&directory.0)
        .env("RUSTC", "rustc")
        .env("TARGET", "x86_64-unknown-linux-gnu")
        .env("CARGO_CFG_TARGET_OS", "linux")
        .env("QUIRE_CODEGEN_ARCHIVE_REVISION", archive_revision)
        .env("QUIRE_CODEGEN_ARCHIVE_RECORDED_AT", "2026-08-31T00:00:00Z")
        .output()
        .unwrap();
    assert!(
        execution.status.success(),
        "archive build metadata failed: {}",
        String::from_utf8_lossy(&execution.stderr)
    );
    let output = String::from_utf8(execution.stdout).unwrap();
    assert!(output.contains(&format!(
        "cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION={archive_revision}"
    )));
    assert!(output.contains("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION_AVAILABLE=true"));
    assert!(output.contains("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_DIRTY=true"));

    let unavailable = Command::new(&build_binary)
        .current_dir(&directory.0)
        .env("RUSTC", "rustc")
        .env("TARGET", "x86_64-unknown-linux-gnu")
        .env("CARGO_CFG_TARGET_OS", "linux")
        .output()
        .unwrap();
    assert!(unavailable.status.success());
    let output = String::from_utf8(unavailable.stdout).unwrap();
    assert!(output.contains(
        "cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION=0000000000000000000000000000000000000000"
    ));
    assert!(output.contains("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION_AVAILABLE=false"));
    assert!(output.contains("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_DIRTY=true"));

    // The archive timestamp now reaches `observed_at`, which the shared schema
    // declares `format: date-time` and which `seal-attestation` enforces. Its
    // sibling `QUIRE_CODEGEN_ARCHIVE_REVISION` was validated all along; this one was
    // not, so an archive build supplying `git log`'s default date format produced a
    // generator every one of whose artifacts Quoin refuses — discoverable only in a
    // downstream repository, at seal time.
    //
    // Both directions are asserted, and then both are put through the real CLI,
    // because "the build script printed a plausible string" is not the claim. The
    // claim is that whatever it prints can be sealed.
    let recorded_at = |value: Option<&str>| -> String {
        let mut command = Command::new(&build_binary);
        command
            .current_dir(&directory.0)
            .env("RUSTC", "rustc")
            .env("TARGET", "x86_64-unknown-linux-gnu")
            .env("CARGO_CFG_TARGET_OS", "linux")
            .env("QUIRE_CODEGEN_ARCHIVE_REVISION", archive_revision)
            .env_remove("QUIRE_CODEGEN_ARCHIVE_RECORDED_AT");
        if let Some(value) = value {
            command.env("QUIRE_CODEGEN_ARCHIVE_RECORDED_AT", value);
        }
        let run = command.output().unwrap();
        assert!(run.status.success());
        String::from_utf8(run.stdout)
            .unwrap()
            .lines()
            .find_map(|line| {
                line.strip_prefix("cargo:rustc-env=QUIRE_CODEGEN_RECORDED_AT=")
                    .map(str::to_owned)
            })
            .expect("the build script states a recorded time")
    };

    assert_eq!(
        recorded_at(Some("2026-08-31T00:00:00Z")),
        "2026-08-31T00:00:00Z"
    );
    assert_eq!(
        recorded_at(Some("2026-08-31T00:00:00-07:00")),
        "2026-08-31T00:00:00-07:00"
    );
    for rejected in [
        "Mon Aug 31 12:00:00 2026 +0000",
        "2026-08-31",
        "2026-08-31T00:00:00",
        "not-a-time",
        "",
    ] {
        assert_eq!(
            recorded_at(Some(rejected)),
            "1970-01-01T00:00:00Z",
            "the build script accepted {rejected:?} as a recorded time"
        );
    }
    assert_eq!(recorded_at(None), "1970-01-01T00:00:00Z");

    // And the values it does emit are ones the CLI will seal. A real emitted body
    // with each substituted in, through `quoin change-assurance seal-attestation`.
    let probe_environment = boolean_environment(&["enabled"]);
    let probe_typed = probe_environment
        .check_expression(&value("enabled", 70), &ValueType::Boolean, &pre(), true)
        .unwrap();
    let probe_clause = ClauseId::new("recorded-at-probe").unwrap();
    let probe = generate_boolean_oracle(&OracleRequest {
        requirement: probe_environment.owner(),
        clause: &probe_clause,
        expression: &probe_typed,
        attestation: attestation_context(),
    })
    .unwrap();
    let with_time = |time: &str| -> String {
        let mut body: serde_json::Value =
            serde_json::from_str(&probe.rust_attestation.contents).unwrap();
        body["observed_at"] = serde_json::Value::String(time.to_owned());
        serde_json::to_string(&body).unwrap()
    };
    for accepted in [
        "2026-08-31T00:00:00Z",
        "2026-08-31T00:00:00-07:00",
        "1970-01-01T00:00:00Z",
    ] {
        let (code, _, stderr) = seal_attestation(&with_time(accepted), &probe.rust, &directory.0);
        assert_eq!(
            code, 0,
            "quoin refused a recorded time the build emits: {stderr}"
        );
    }
    // The control: the value the build script now refuses is one the CLI refuses
    // too. Without it, the loop above is satisfied by a CLI that accepts anything.
    let (code, _, _) = seal_attestation(
        &with_time("Mon Aug 31 12:00:00 2026 +0000"),
        &probe.rust,
        &directory.0,
    );
    assert_ne!(
        code, 0,
        "the CLI accepted a non-RFC-3339 observed_at, so the build-script guard \
         guards nothing"
    );
}
