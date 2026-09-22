use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::common;

use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    generate_boolean_oracle, generate_kani_bundle, AttestationContext, AttestationResult,
    GenerationErrorCode, GenerationTerminalState, KaniBindingRole, KaniDiagnostic, KaniErrorCode,
    KaniPrimitiveType, KaniRequest, KaniSolver, OracleRequest, ProofAttestationBody,
    ProofDependencyGraph, ProofDependencyKind, ProofDependencyRequest, ProofDependencyState,
    ProofReadiness, IR_CANDIDATE_REVISION, KANI_ADAPTER_PROFILE, KANI_BACKEND_VERSION,
    MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, OverflowPolicy,
    PackageId, RequirementId, RequirementRef, RequirementRevision, SourceDocumentId,
    SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation, SymbolName,
    ValueDeclaration, ValueDeclarationKind, ValueType,
};
use sha2::{Digest as _, Sha256};

const BACKEND_SHA256: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must follow the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&path).expect("temporary test directory should be writable");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn name(value: &str) -> SymbolName {
    SymbolName::new(value).expect("fixture symbol should be valid")
}

fn span(start: u64, end: u64) -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("kani-test").expect("fixture source should be valid"),
        SourceRevision::new(1).expect("fixture revision should be valid"),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, start as u32 + 1, start)
            .expect("fixture start should be valid"),
        SourceLocation::new(source, 1, end as u32 + 1, end).expect("fixture end should be valid"),
    )
    .expect("fixture span should be valid")
}

fn requirement() -> RequirementRef {
    RequirementRef::new(
        PackageId::new("agent-ix/kani-test").expect("fixture package should be valid"),
        RequirementId::new("FR-003").expect("fixture requirement should be valid"),
        RequirementRevision::new(4).expect("fixture revision should be valid"),
    )
}

fn environment() -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        requirement(),
        vec![],
        vec![
            ValueDeclaration::new(
                name("input"),
                ValueDeclarationKind::Input,
                ValueType::Boolean,
                span(0, 1),
            ),
            ValueDeclaration::new(
                name("state"),
                ValueDeclarationKind::State,
                ValueType::Boolean,
                span(2, 3),
            ),
        ],
        vec![],
    )
    .expect("fixture environment should be valid")
}

fn integer_type(minimum: i64, maximum: i64) -> IntegerType {
    IntegerType::new(
        IntegerDomain::Signed,
        minimum,
        maximum,
        OverflowPolicy::Reject,
    )
    .expect("fixture integer bounds should be valid")
}

fn numeric_environment(
    values: &[(&str, ValueDeclarationKind, ValueType)],
) -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        requirement(),
        vec![],
        values
            .iter()
            .enumerate()
            .map(|(index, (value, kind, value_type))| {
                ValueDeclaration::new(
                    name(value),
                    *kind,
                    value_type.clone(),
                    span(index as u64, index as u64 + 1),
                )
            })
            .collect(),
        vec![],
    )
    .expect("numeric fixture environment should be valid")
}

fn handler() -> ExecutionPoint {
    ExecutionPoint::Handler {
        name: AnchorName::new("generate").expect("fixture anchor should be valid"),
    }
}

fn observed(name_value: &str, observation: StateObservation, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::ValueReference {
            name: name(name_value),
            observation,
        },
        span(at, at + 1),
    )
}

fn integer(value: i64, value_type: &IntegerType, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::IntegerLiteral {
            value,
            value_type: value_type.clone(),
        },
        span(at, at + 1),
    )
}

fn compare(
    operator: ComparisonOperator,
    left: Expression,
    right: Expression,
    at: u64,
) -> Expression {
    Expression::new(
        ExpressionKind::Compare {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        },
        span(at, at + 1),
    )
}

fn boolean_and(left: Expression, right: Expression, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::Boolean {
            operator: BooleanOperator::ShortCircuitAnd,
            left: Box::new(left),
            right: Box::new(right),
        },
        span(at, at + 1),
    )
}

fn source_symbol(source: &str) -> &str {
    source
        .lines()
        .find_map(|line| line.strip_prefix("pub fn "))
        .and_then(|signature| signature.split('(').next())
        .expect("generated oracle should contain one public function")
}

fn comparison_truth(operator: ComparisonOperator, left: i64, right: i64) -> bool {
    match operator {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => left < right,
        ComparisonOperator::LessEqual => left <= right,
        ComparisonOperator::Greater => left > right,
        ComparisonOperator::GreaterEqual => left >= right,
    }
}

fn boolean_or(left: Expression, right: Expression, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::Boolean {
            operator: BooleanOperator::ShortCircuitOr,
            left: Box::new(left),
            right: Box::new(right),
        },
        span(at, at + 1),
    )
}

fn attestation_context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

fn clauses(
    environment: &DeclarationEnvironment,
) -> (
    quire_contract_ir::TypedExpression,
    quire_contract_ir::TypedExpression,
) {
    let precondition = boolean_or(
        observed("input", StateObservation::Current, 10),
        observed("state", StateObservation::Pre, 11),
        10,
    );
    let postcondition = observed("state", StateObservation::Post, 20);
    (
        environment
            .check_expression(&precondition, &ValueType::Boolean, &handler(), true)
            .expect("precondition fixture should type-check"),
        environment
            .check_expression(&postcondition, &ValueType::Boolean, &handler(), true)
            .expect("postcondition fixture should type-check"),
    )
}

fn request<'a>(
    environment: &'a DeclarationEnvironment,
    precondition: &'a quire_contract_ir::TypedExpression,
    postcondition: &'a quire_contract_ir::TypedExpression,
    precondition_clause: &'a ClauseId,
    postcondition_clause: &'a ClauseId,
    dependencies: &'a [ProofDependencyRequest<'a>],
) -> KaniRequest<'a> {
    KaniRequest {
        requirement: environment.owner(),
        precondition_clause,
        postcondition_clause,
        precondition,
        postcondition,
        proof_id: "proof-boolean-transition",
        subject_path: "crate::subject",
        backend_version: KANI_BACKEND_VERSION,
        backend_executable_sha256: BACKEND_SHA256,
        unwind: 2,
        solver: KaniSolver::Cadical,
        dependencies,
        attestation: attestation_context(),
    }
}

fn fixture_bundle(
    dependencies: &[ProofDependencyRequest<'_>],
) -> quire_contract_codegen::KaniArtifactBundle {
    let environment = environment();
    let (precondition, postcondition) = clauses(&environment);
    let precondition_clause =
        ClauseId::new("precondition").expect("fixture clause should be valid");
    let postcondition_clause =
        ClauseId::new("postcondition").expect("fixture clause should be valid");
    generate_kani_bundle(&request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        dependencies,
    ))
    .expect("fixture bundle should generate")
}

fn validate(schema: &str, instance: &serde_json::Value) {
    let schema: serde_json::Value =
        serde_json::from_str(schema).expect("repository schema should parse");
    let validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("repository schema should compile");
    let errors = validator
        .validate(instance)
        .err()
        .map(|values| values.map(|error| error.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    assert!(errors.is_empty(), "schema errors: {errors:?}");
}

fn write_generated_crate(
    bundle: &quire_contract_codegen::KaniArtifactBundle,
    subject: &str,
) -> TemporaryDirectory {
    let directory = TemporaryDirectory::new("quire-generated-kani");
    fs::create_dir_all(directory.0.join("src")).expect("source directory should be writable");
    fs::write(
        directory.0.join("src/lib.rs"),
        format!(
            "{}\n{subject}\n\n/// Assumed dependency predicate.\npub fn dependency_predicate() -> bool {{ true }}\n\n/// Original dependency implementation.\npub fn original() -> bool {{ false }}\n\n/// Replacement dependency implementation.\npub fn replacement() -> bool {{ true }}\n",
            bundle.rust.contents,
        ),
    )
    .expect("generated source should be writable");
    fs::write(
        directory.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-kani-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .expect("generated manifest should be writable");
    fs::write(
        directory.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("generated check-cfg declaration should be writable");
    directory
}

fn execute_kani(
    bundle: &quire_contract_codegen::KaniArtifactBundle,
    subject: &str,
) -> std::process::Output {
    let graph: ProofDependencyGraph =
        serde_json::from_str(&bundle.proof_graph.contents).expect("graph should deserialize");
    let directory = write_generated_crate(bundle, subject);
    Command::new("cargo")
        .arg("kani")
        .args(&graph.options)
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch")
}

fn cargo_kani_sha256() -> String {
    let executable_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);
    let executable = env::var_os("PATH")
        .into_iter()
        .flat_map(|value| env::split_paths(&value).collect::<Vec<_>>())
        .map(|directory| directory.join(&executable_name))
        .find(|candidate| candidate.is_file())
        .expect("cargo-kani must be discoverable on PATH");
    let bytes = fs::read(executable).expect("cargo-kani executable should be readable");
    format!("{:x}", Sha256::digest(bytes))
}

/// TC-003
/// TC-005
/// TC-007
#[test]
fn kani_bundle_is_deterministic_schema_valid_and_stable_rust_compiles() {
    let dependencies = [ProofDependencyRequest {
        proof_id: "proof-required",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Passed,
        original_path: None,
        replacement_path: None,
    }];
    let first = fixture_bundle(&dependencies);
    let second = fixture_bundle(&dependencies);
    assert_eq!(first, second);
    assert!(first.rust.contents.contains(KANI_ADAPTER_PROFILE));
    assert!(first.rust.contents.contains("// BEGIN framing"));
    assert!(first.rust.contents.contains("// BEGIN binding"));
    assert!(first.rust.contents.contains("// BEGIN contract"));
    assert!(first.rust.contents.contains("// BEGIN proof harness"));
    assert!(first.rust.contents.contains("#[kani::requires("));
    assert!(first.rust.contents.contains("#[kani::ensures("));
    assert!(first.rust.contents.contains("#[kani::proof_for_contract("));

    let graph: ProofDependencyGraph =
        serde_json::from_str(&first.proof_graph.contents).expect("graph should deserialize");
    assert_eq!(graph.readiness, ProofReadiness::Ready);
    assert_eq!(graph.dependencies.len(), 1);
    assert!(graph
        .options
        .windows(2)
        .any(|pair| pair[0] == "--harness" && pair[1].starts_with("kani_fr_003_4_")));
    assert!(graph.options.iter().any(|option| option == "--exact"));
    validate(
        include_str!("../../schemas/kani-proof-graph-v2.schema.json"),
        &serde_json::from_str(&first.proof_graph.contents).expect("graph JSON should parse"),
    );
    validate(
        include_str!("../../schemas/generated-rust-kani-v2.schema.json"),
        &serde_json::Value::String(first.rust.contents.clone()),
    );

    assert_eq!(graph.proof_execution_state, "not_run");
    assert_eq!(graph.backend_executable_sha256, BACKEND_SHA256);
    let schema = common::packaged_attestation_schema();
    let validator = common::packaged_attestation_validator(&schema);
    let sealed_directory = TemporaryDirectory::new("quire-kani-attestations");
    for (attestation, artifact, proof_id) in [
        (
            &first.rust_attestation,
            &first.rust,
            "PROOF-codegen-generated-rust-kani-proof",
        ),
        (
            &first.proof_graph_attestation,
            &first.proof_graph,
            "PROOF-codegen-kani-proof-dependency-graph",
        ),
    ] {
        let body: ProofAttestationBody =
            serde_json::from_str(&attestation.contents).expect("attestation should deserialize");
        assert_eq!(body.proof_id, proof_id);
        assert_eq!(body.result, AttestationResult::Passed);
        assert!(body
            .command
            .argv
            .windows(2)
            .any(|pair| { pair[0] == "--proof-execution-state" && pair[1] == "not_run" }));
        let sealed = common::seal_and_validate(
            &attestation.contents,
            artifact,
            &sealed_directory.0,
            &validator,
        );
        assert_eq!(sealed["proof_id"], proof_id);
    }

    let directory = write_generated_crate(
        &first,
        "/// Customer transition under proof.\npub fn subject(input: bool, pre_state: bool) -> bool { input || pre_state }",
    );
    let compilation = Command::new("cargo")
        .args(["check", "--offline", "--quiet"])
        .env("RUSTFLAGS", "-Dwarnings")
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo check should launch");
    assert!(
        compilation.status.success(),
        "generated Rust did not compile warning-clean: {}",
        String::from_utf8_lossy(&compilation.stderr)
    );
}

/// TC-014
/// FR-003-AC-2
/// FR-003-AC-4
/// FR-003-AC-5
/// FR-003-AC-8
#[test]
fn numeric_state_bindings_are_normalized_bounded_and_schema_valid() {
    let bounded = integer_type(0, 1000);
    let environment = numeric_environment(&[
        ("flag", ValueDeclarationKind::Input, ValueType::Boolean),
        (
            "amount",
            ValueDeclarationKind::Input,
            ValueType::integer(bounded.clone()),
        ),
        ("enabled", ValueDeclarationKind::State, ValueType::Boolean),
        (
            "version",
            ValueDeclarationKind::State,
            ValueType::integer(bounded.clone()),
        ),
    ]);
    let precondition = environment
        .check_expression(
            &boolean_and(
                observed("flag", StateObservation::Current, 100),
                boolean_and(
                    observed("enabled", StateObservation::Current, 101),
                    boolean_and(
                        compare(
                            ComparisonOperator::GreaterEqual,
                            observed("amount", StateObservation::Current, 102),
                            integer(0, &bounded, 103),
                            102,
                        ),
                        compare(
                            ComparisonOperator::GreaterEqual,
                            observed("version", StateObservation::Pre, 104),
                            integer(0, &bounded, 105),
                            104,
                        ),
                        103,
                    ),
                    101,
                ),
                100,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("mixed primitive precondition should type-check");
    let postcondition = environment
        .check_expression(
            &boolean_and(
                observed("enabled", StateObservation::Post, 200),
                compare(
                    ComparisonOperator::Equal,
                    observed("version", StateObservation::Post, 201),
                    observed("version", StateObservation::Pre, 202),
                    201,
                ),
                200,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("mixed primitive postcondition should type-check");
    let precondition_clause =
        ClauseId::new("numeric-precondition").expect("fixture clause should be valid");
    let postcondition_clause =
        ClauseId::new("numeric-postcondition").expect("fixture clause should be valid");
    let mut request = request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    request.proof_id = "proof-mixed-numeric-state";
    let backend_digest = cargo_kani_sha256();
    request.backend_executable_sha256 = &backend_digest;
    let first = generate_kani_bundle(&request).expect("mixed numeric Kani bundle should generate");
    let second =
        generate_kani_bundle(&request).expect("repeated mixed numeric Kani bundle should generate");
    assert_eq!(first, second);

    let graph: ProofDependencyGraph =
        serde_json::from_str(&first.proof_graph.contents).expect("v2 graph should deserialize");
    assert_eq!(graph.schema_version, "quire.kani-proof-graph/v2");
    assert_eq!(graph.adapter_profile, KANI_ADAPTER_PROFILE);
    assert_eq!(graph.backend_executable_sha256, backend_digest);
    assert_eq!(
        graph
            .subject_arguments
            .iter()
            .map(|binding| binding.identifier.as_str())
            .collect::<Vec<_>>(),
        [
            "amount_current",
            "flag_current",
            "enabled_current",
            "version_pre"
        ]
    );
    assert_eq!(
        graph
            .subject_results
            .iter()
            .map(|binding| binding.identifier.as_str())
            .collect::<Vec<_>>(),
        ["enabled_post", "version_post"]
    );
    for binding in &graph.subject_arguments {
        assert_eq!(binding.role, KaniBindingRole::Argument);
        assert!(!binding.source_spans.is_empty());
    }
    for binding in &graph.subject_results {
        assert_eq!(binding.role, KaniBindingRole::Result);
        assert!(!binding.source_spans.is_empty());
    }
    let integer_bindings = graph
        .subject_arguments
        .iter()
        .chain(&graph.subject_results)
        .filter(|binding| binding.primitive_type == KaniPrimitiveType::I64)
        .collect::<Vec<_>>();
    assert_eq!(integer_bindings.len(), 3);
    for binding in integer_bindings {
        let bounds = binding
            .integer_bounds
            .as_ref()
            .expect("every i64 binding retains checked bounds");
        assert_eq!((bounds.minimum, bounds.maximum), (0, 1000));
        assert_eq!(bounds.domain, IntegerDomain::Signed);
        assert_eq!(bounds.overflow, OverflowPolicy::Reject);
        assert!(!(bounds.minimum..=bounds.maximum).contains(&-1));
        assert!((bounds.minimum..=bounds.maximum).contains(&0));
        assert!((bounds.minimum..=bounds.maximum).contains(&1000));
        assert!(!(bounds.minimum..=bounds.maximum).contains(&1001));
    }
    assert_eq!(first.rust.contents.matches("kani::assume(").count(), 2);
    assert!(first
        .rust
        .contents
        .contains("kani::assume(amount_current >= 0_i64 && amount_current <= 1000_i64);"));
    assert!(first
        .rust
        .contents
        .contains("kani::assume(version_pre >= 0_i64 && version_pre <= 1000_i64);"));
    assert!(first
        .rust
        .contents
        .contains("post_state.1 >= 0_i64 && post_state.1 <= 1000_i64"));
    assert!(graph
        .options
        .windows(2)
        .any(|pair| pair == ["-Z", "concrete-playback"]));
    assert!(graph
        .options
        .windows(2)
        .any(|pair| pair == ["--output-format", "regular"]));
    assert!(graph
        .options
        .windows(2)
        .any(|pair| pair == ["--concrete-playback", "print"]));
    validate(
        include_str!("../../schemas/kani-proof-graph-v2.schema.json"),
        &serde_json::from_str(&first.proof_graph.contents).expect("graph JSON should parse"),
    );
    validate(
        include_str!("../../schemas/generated-rust-kani-v2.schema.json"),
        &serde_json::Value::String(first.rust.contents.clone()),
    );

    let directory = write_generated_crate(
        &first,
        "/// Mixed primitive transition under proof.\npub fn subject(amount: i64, flag: bool, enabled: bool, version: i64) -> (bool, i64) { let _ = (amount, enabled); (flag, version) }",
    );
    let compilation = Command::new("cargo")
        .args(["check", "--offline", "--quiet"])
        .env("RUSTFLAGS", "-Dwarnings")
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo check should launch");
    assert!(
        compilation.status.success(),
        "mixed generated Rust did not compile warning-clean: {}",
        String::from_utf8_lossy(&compilation.stderr)
    );

    let verification = execute_kani(
        &first,
        "/// Mixed primitive transition under proof.\npub fn subject(amount: i64, flag: bool, enabled: bool, version: i64) -> (bool, i64) { let _ = (amount, enabled); (flag, version) }",
    );
    assert!(
        verification.status.success(),
        "mixed generated Kani proof failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verification.stdout),
        String::from_utf8_lossy(&verification.stderr)
    );
}

/// TC-014
/// FR-003-AC-2
/// FR-003-AC-5
/// FR-003-AC-7
#[test]
fn every_integer_comparison_supports_a_zero_result_bounded_subject() {
    let bounded = integer_type(0, 1000);
    let environment = numeric_environment(&[(
        "value",
        ValueDeclarationKind::Input,
        ValueType::integer(bounded.clone()),
    )]);
    let precondition = environment
        .check_expression(
            &Expression::new(
                ExpressionKind::BooleanLiteral { value: true },
                span(300, 301),
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("constant precondition should type-check");
    let precondition_clause =
        ClauseId::new("comparison-precondition").expect("fixture clause should be valid");
    for (index, operator) in [
        ComparisonOperator::Equal,
        ComparisonOperator::NotEqual,
        ComparisonOperator::Less,
        ComparisonOperator::LessEqual,
        ComparisonOperator::Greater,
        ComparisonOperator::GreaterEqual,
    ]
    .into_iter()
    .enumerate()
    {
        let at = 400 + index as u64 * 4;
        let postcondition = environment
            .check_expression(
                &compare(
                    operator,
                    observed("value", StateObservation::Current, at),
                    integer(1000, &bounded, at + 1),
                    at,
                ),
                &ValueType::Boolean,
                &handler(),
                true,
            )
            .expect("integer comparison should type-check");
        let postcondition_clause = ClauseId::new(format!("comparison-postcondition-{index}"))
            .expect("fixture clause should be valid");
        let mut request = request(
            &environment,
            &precondition,
            &postcondition,
            &precondition_clause,
            &postcondition_clause,
            &[],
        );
        request.proof_id = "proof-zero-result-comparison";
        let bundle = generate_kani_bundle(&request)
            .expect("every supported integer comparison should generate");
        let graph: ProofDependencyGraph = serde_json::from_str(&bundle.proof_graph.contents)
            .expect("comparison graph should deserialize");
        assert_eq!(graph.subject_arguments.len(), 1);
        assert!(graph.subject_results.is_empty());
        assert!(bundle
            .rust
            .contents
            .contains("fn call_subject(value_current: i64) -> ()"));
        assert!(bundle.rust.contents.contains("|_post_state: &()|"));
    }
}

/// TC-007
/// TC-014
/// FR-003-AC-2
/// FR-003-AC-5
/// FR-003-AC-7
#[test]
fn generated_numeric_oracles_execute_the_shared_inside_and_outside_corpus() {
    let bounded = integer_type(0, 1000);
    let environment = numeric_environment(&[(
        "value",
        ValueDeclarationKind::Input,
        ValueType::integer(bounded.clone()),
    )]);
    let precondition = environment
        .check_expression(
            &Expression::new(
                ExpressionKind::BooleanLiteral { value: true },
                span(500, 501),
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("constant precondition should type-check");
    let precondition_clause =
        ClauseId::new("corpus-precondition").expect("fixture clause should be valid");
    let mut generated_program =
        String::from("#![deny(warnings)]\n//! Executed Kani/oracle numeric parity corpus.\n");
    let mut assertions = String::from("fn main() {\n");
    for (index, operator) in [
        ComparisonOperator::Equal,
        ComparisonOperator::NotEqual,
        ComparisonOperator::Less,
        ComparisonOperator::LessEqual,
        ComparisonOperator::Greater,
        ComparisonOperator::GreaterEqual,
    ]
    .into_iter()
    .enumerate()
    {
        let at = 520 + index as u64 * 4;
        let postcondition = environment
            .check_expression(
                &compare(
                    operator,
                    observed("value", StateObservation::Current, at),
                    integer(1000, &bounded, at + 1),
                    at,
                ),
                &ValueType::Boolean,
                &handler(),
                true,
            )
            .expect("corpus comparison should type-check");
        let postcondition_clause = ClauseId::new(format!("corpus-postcondition-{index}"))
            .expect("fixture clause should be valid");
        let oracle = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &postcondition_clause,
            expression: &postcondition,
            attestation: attestation_context(),
        })
        .expect("executable numeric oracle should generate");
        let mut kani_request = request(
            &environment,
            &precondition,
            &postcondition,
            &precondition_clause,
            &postcondition_clause,
            &[],
        );
        let proof_id = format!("proof-corpus-{index}");
        kani_request.proof_id = &proof_id;
        let kani =
            generate_kani_bundle(&kani_request).expect("numeric Kani bundle should generate");
        assert!(
            kani.rust.contents.contains(&oracle.rust.contents),
            "Kani must embed the executable oracle source byte-for-byte"
        );
        let symbol = source_symbol(&oracle.rust.contents);
        generated_program.push_str(&oracle.rust.contents);
        for value in [-1_i64, 0, 1, 999, 1000, 1001] {
            let expected = comparison_truth(operator, value, 1000);
            let in_domain = (0..=1000).contains(&value);
            assertions.push_str(&format!(
                "    assert_eq!({symbol}({value}), {expected}); // in_kani_domain={in_domain}\n"
            ));
        }
    }
    assertions.push_str("}\n");
    generated_program.push_str(&assertions);

    let directory = TemporaryDirectory::new("quire-kani-oracle-corpus");
    fs::create_dir_all(directory.0.join("src"))
        .expect("generated corpus source directory should be writable");
    fs::write(directory.0.join("src/main.rs"), generated_program)
        .expect("generated corpus source should be writable");
    fs::write(
        directory.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-kani-oracle-corpus\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .expect("generated corpus manifest should be writable");
    let execution = Command::new("cargo")
        .args(["run", "--offline", "--quiet", "--target-dir"])
        .arg(directory.0.join("target-codex-backends"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&directory.0)
        .output()
        .expect("generated corpus should execute");
    assert!(
        execution.status.success(),
        "generated numeric oracle corpus failed:\n{}",
        String::from_utf8_lossy(&execution.stderr)
    );
}

/// TC-014
/// FR-003-AC-2
/// FR-003-AC-8
#[test]
fn declaration_and_dependency_order_do_not_change_the_normalized_bundle() {
    let bounded = integer_type(0, 1000);
    let first_environment = numeric_environment(&[
        (
            "version",
            ValueDeclarationKind::State,
            ValueType::integer(bounded.clone()),
        ),
        ("flag", ValueDeclarationKind::Input, ValueType::Boolean),
    ]);
    let second_environment = numeric_environment(&[
        ("flag", ValueDeclarationKind::Input, ValueType::Boolean),
        (
            "version",
            ValueDeclarationKind::State,
            ValueType::integer(bounded.clone()),
        ),
    ]);
    let expression_pair = || {
        (
            boolean_and(
                observed("flag", StateObservation::Current, 900),
                compare(
                    ComparisonOperator::GreaterEqual,
                    observed("version", StateObservation::Pre, 901),
                    integer(0, &bounded, 902),
                    901,
                ),
                900,
            ),
            compare(
                ComparisonOperator::Equal,
                observed("version", StateObservation::Post, 910),
                observed("version", StateObservation::Pre, 911),
                910,
            ),
        )
    };
    let (first_pre_source, first_post_source) = expression_pair();
    let (second_pre_source, second_post_source) = expression_pair();
    let first_precondition = first_environment
        .check_expression(&first_pre_source, &ValueType::Boolean, &handler(), true)
        .expect("first ordered precondition should type-check");
    let first_postcondition = first_environment
        .check_expression(&first_post_source, &ValueType::Boolean, &handler(), true)
        .expect("first ordered postcondition should type-check");
    let second_precondition = second_environment
        .check_expression(&second_pre_source, &ValueType::Boolean, &handler(), true)
        .expect("second ordered precondition should type-check");
    let second_postcondition = second_environment
        .check_expression(&second_post_source, &ValueType::Boolean, &handler(), true)
        .expect("second ordered postcondition should type-check");
    let precondition_clause =
        ClauseId::new("normalized-precondition").expect("fixture clause should be valid");
    let postcondition_clause =
        ClauseId::new("normalized-postcondition").expect("fixture clause should be valid");
    let mut first_request = request(
        &first_environment,
        &first_precondition,
        &first_postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    first_request.proof_id = "proof-normalized-order";
    let mut second_request = request(
        &second_environment,
        &second_precondition,
        &second_postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    second_request.proof_id = "proof-normalized-order";
    assert_eq!(
        generate_kani_bundle(&first_request).expect("first ordered bundle should generate"),
        generate_kani_bundle(&second_request).expect("second ordered bundle should generate")
    );
}

/// TC-014
/// FR-003-AC-2
/// FR-003-AC-5
/// FR-003-AC-6
/// FR-003-AC-7
#[test]
fn pinned_kani_proves_identity_and_prints_numeric_counterexamples() {
    let version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .expect("cargo-kani must be installed for the numeric adapter test");
    assert!(version.status.success(), "cargo-kani version query failed");
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        format!("cargo-kani {KANI_BACKEND_VERSION}")
    );

    let bounded = integer_type(0, 1000);
    let state_environment = numeric_environment(&[(
        "version",
        ValueDeclarationKind::State,
        ValueType::integer(bounded.clone()),
    )]);
    let state_precondition = state_environment
        .check_expression(
            &Expression::new(
                ExpressionKind::BooleanLiteral { value: true },
                span(700, 701),
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("state precondition should type-check");
    let state_postcondition = state_environment
        .check_expression(
            &compare(
                ComparisonOperator::Equal,
                observed("version", StateObservation::Post, 710),
                observed("version", StateObservation::Pre, 711),
                710,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("state postcondition should type-check");
    let state_precondition_clause =
        ClauseId::new("state-precondition").expect("fixture clause should be valid");
    let state_postcondition_clause =
        ClauseId::new("state-postcondition").expect("fixture clause should be valid");
    let mut state_request = request(
        &state_environment,
        &state_precondition,
        &state_postcondition,
        &state_precondition_clause,
        &state_postcondition_clause,
        &[],
    );
    state_request.proof_id = "proof-config-version";
    let backend_digest = cargo_kani_sha256();
    state_request.backend_executable_sha256 = &backend_digest;
    let state_bundle =
        generate_kani_bundle(&state_request).expect("state identity bundle should generate");
    let state_graph: ProofDependencyGraph =
        serde_json::from_str(&state_bundle.proof_graph.contents)
            .expect("state graph should deserialize");
    assert_eq!(state_graph.subject_arguments.len(), 1);
    assert_eq!(state_graph.subject_results.len(), 1);
    assert_eq!(state_graph.backend_executable_sha256, backend_digest);

    let healthy = execute_kani(
        &state_bundle,
        "/// Identity ConfigVersion subject.\npub fn subject(version_pre: i64) -> i64 { version_pre }",
    );
    assert!(
        healthy.status.success(),
        "identity Kani proof failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&healthy.stdout),
        String::from_utf8_lossy(&healthy.stderr)
    );

    let wrong_signature = execute_kani(
        &state_bundle,
        "/// Deliberately incompatible customer signature.\npub fn subject(version_pre: bool) -> bool { version_pre }",
    );
    assert!(
        !wrong_signature.status.success(),
        "an incompatible external subject signature must not prove"
    );
    let signature_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&wrong_signature.stdout),
        String::from_utf8_lossy(&wrong_signature.stderr)
    );
    assert!(
        signature_output.contains("mismatched types"),
        "signature failure was not reported by Rust/Kani:\n{signature_output}"
    );

    let changed = execute_kani(
        &state_bundle,
        "/// Violating ConfigVersion subject.\npub fn subject(version_pre: i64) -> i64 { version_pre + 1 }",
    );
    assert!(
        !changed.status.success(),
        "changed state must falsify the contract"
    );
    let changed_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&changed.stdout),
        String::from_utf8_lossy(&changed.stderr)
    );
    assert!(
        changed_output.contains("Concrete playback unit test")
            && changed_output.contains("let concrete_vals: Vec<Vec<u8>>")
            && changed_output.contains("kani::concrete_playback_run"),
        "changed-state failure did not print concrete playback:\n{changed_output}"
    );

    let input_environment = numeric_environment(&[(
        "value",
        ValueDeclarationKind::Input,
        ValueType::integer(bounded.clone()),
    )]);
    let comparison_precondition = input_environment
        .check_expression(
            &Expression::new(
                ExpressionKind::BooleanLiteral { value: true },
                span(800, 801),
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("comparison precondition should type-check");
    let comparison_postcondition = input_environment
        .check_expression(
            &compare(
                ComparisonOperator::Less,
                observed("value", StateObservation::Current, 810),
                integer(1000, &bounded, 811),
                810,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("comparison postcondition should type-check");
    let comparison_precondition_clause =
        ClauseId::new("plain-precondition").expect("fixture clause should be valid");
    let comparison_postcondition_clause =
        ClauseId::new("plain-postcondition").expect("fixture clause should be valid");
    let mut comparison_request = request(
        &input_environment,
        &comparison_precondition,
        &comparison_postcondition,
        &comparison_precondition_clause,
        &comparison_postcondition_clause,
        &[],
    );
    comparison_request.proof_id = "proof-plain-integer-comparison";
    let comparison_bundle =
        generate_kani_bundle(&comparison_request).expect("plain comparison bundle should generate");
    let comparison = execute_kani(
        &comparison_bundle,
        "/// Unit-returning comparison subject.\npub fn subject(value_current: i64) { let _ = value_current; }",
    );
    assert!(
        !comparison.status.success(),
        "value 1000 must falsify the strict comparison"
    );
    let comparison_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&comparison.stdout),
        String::from_utf8_lossy(&comparison.stderr)
    );
    assert!(
        comparison_output.contains("Concrete playback")
            && comparison_output.contains("value_current")
            && comparison_output.contains("1000"),
        "comparison failure did not print the in-domain counterexample:\n{comparison_output}"
    );
}

/// TC-005
/// FR-003-AC-1
#[test]
fn proof_dependency_graph_derives_readiness_and_preserves_source_sites() {
    let missing = [ProofDependencyRequest {
        proof_id: "proof-missing",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Missing,
        original_path: None,
        replacement_path: None,
    }];
    let missing_graph: ProofDependencyGraph =
        serde_json::from_str(&fixture_bundle(&missing).proof_graph.contents)
            .expect("missing graph should deserialize");
    assert_eq!(missing_graph.readiness, ProofReadiness::Incomplete);

    let failed = [ProofDependencyRequest {
        proof_id: "proof-failed",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Failed,
        original_path: None,
        replacement_path: None,
    }];
    let failed_graph: ProofDependencyGraph =
        serde_json::from_str(&fixture_bundle(&failed).proof_graph.contents)
            .expect("failed graph should deserialize");
    assert_eq!(failed_graph.readiness, ProofReadiness::Incomplete);

    let conditional = [
        ProofDependencyRequest {
            proof_id: "proof-assumed",
            kind: ProofDependencyKind::Assumed,
            state: ProofDependencyState::Assumed,
            original_path: Some("crate::dependency_predicate"),
            replacement_path: None,
        },
        ProofDependencyRequest {
            proof_id: "proof-stubbed",
            kind: ProofDependencyKind::Stubbed,
            state: ProofDependencyState::Stubbed,
            original_path: Some("crate::original"),
            replacement_path: Some("crate::replacement"),
        },
    ];
    let bundle = fixture_bundle(&conditional);
    let reversed = [conditional[1], conditional[0]];
    assert_eq!(
        bundle,
        fixture_bundle(&reversed),
        "dependency census order must not alter generated artifacts"
    );
    let graph: ProofDependencyGraph =
        serde_json::from_str(&bundle.proof_graph.contents).expect("graph should deserialize");
    assert_eq!(graph.readiness, ProofReadiness::Conditional);
    assert!(graph
        .options
        .windows(2)
        .any(|pair| pair == ["-Z", "stubbing"]));
    for edge in &graph.dependencies {
        let source_site = edge
            .source_site
            .as_deref()
            .expect("assumed and stubbed edges require source sites");
        assert_eq!(bundle.rust.contents.matches(source_site).count(), 1);
    }
    assert_eq!(bundle.rust.contents.matches("kani::assume(").count(), 1);
    assert_eq!(bundle.rust.contents.matches("#[kani::stub(").count(), 1);

    let graph_schema: serde_json::Value = serde_json::from_str(include_str!(
        "../../schemas/kani-proof-graph-v2.schema.json"
    ))
    .expect("graph schema should parse");
    let graph_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&graph_schema)
        .expect("graph schema should compile");
    let mut laundered_graph: serde_json::Value =
        serde_json::from_str(&bundle.proof_graph.contents).expect("graph JSON should parse");
    laundered_graph["dependencies"][0]["state"] = serde_json::json!("passed");
    assert!(graph_validator.validate(&laundered_graph).is_err());
}

/// TC-003
/// FR-003-AC-3
#[test]
fn invalid_kani_requests_return_structured_non_generated_states() {
    let environment = environment();
    let (precondition, postcondition) = clauses(&environment);
    let precondition_clause =
        ClauseId::new("precondition").expect("fixture clause should be valid");
    let postcondition_clause =
        ClauseId::new("postcondition").expect("fixture clause should be valid");
    let mut value = request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );

    value.backend_version = "0.66.0";
    let diagnostic = &generate_kani_bundle(&value).expect_err("version should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::UnsupportedBackendVersion);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::BackendUnavailable
    );

    value.backend_version = KANI_BACKEND_VERSION;
    value.backend_executable_sha256 = "not-a-digest";
    let diagnostic =
        &generate_kani_bundle(&value).expect_err("backend digest should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidIdentity);
    assert_eq!(diagnostic.path, "backend_executable_sha256");

    value.backend_executable_sha256 = BACKEND_SHA256;
    value.subject_path = "not::a::valid::path::";
    let diagnostic = &generate_kani_bundle(&value).expect_err("subject should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidIdentity);

    value.subject_path = "crate::subject";
    value.attestation = AttestationContext {
        record_digest: "not-a-digest",
        candidate_revision: IR_CANDIDATE_REVISION,
    };
    let diagnostic = &generate_kani_bundle(&value).expect_err("context should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidAttestationContext);

    value.attestation = attestation_context();
    value.unwind = 0;
    let diagnostic = &generate_kani_bundle(&value).expect_err("unwind should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidUnwind);

    let invalid_dependency = [ProofDependencyRequest {
        proof_id: "missing-assumption-path",
        kind: ProofDependencyKind::Assumed,
        state: ProofDependencyState::Assumed,
        original_path: None,
        replacement_path: None,
    }];
    let invalid_dependency_request = request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &invalid_dependency,
    );
    let diagnostic = &generate_kani_bundle(&invalid_dependency_request)
        .expect_err("invalid dependency should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidDependency);

    let duplicate_dependencies = [
        ProofDependencyRequest {
            proof_id: "duplicate-proof",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        },
        ProofDependencyRequest {
            proof_id: "duplicate-proof",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        },
    ];
    let duplicate_dependency_request = request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &duplicate_dependencies,
    );
    let diagnostic = &generate_kani_bundle(&duplicate_dependency_request)
        .expect_err("duplicate dependency identity should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidDependency);
    assert_eq!(diagnostic.path, "dependencies[1].proof_id");

    let unsupported_precondition = environment
        .check_expression(
            &boolean_or(
                observed("input", StateObservation::Current, 50),
                observed("state", StateObservation::Post, 51),
                50,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("current-state fixture should type-check");
    let unsupported_request = request(
        &environment,
        &unsupported_precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    let diagnostic = &generate_kani_bundle(&unsupported_request)
        .expect_err("post-state precondition should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::UnsupportedBinding);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::Unsupported
    );

    let wide = integer_type(0, 1000);
    let narrow = integer_type(0, 10);
    let wide_environment = numeric_environment(&[(
        "value",
        ValueDeclarationKind::Input,
        ValueType::integer(wide.clone()),
    )]);
    let narrow_environment = numeric_environment(&[(
        "value",
        ValueDeclarationKind::Input,
        ValueType::integer(narrow.clone()),
    )]);
    let wide_precondition = wide_environment
        .check_expression(
            &compare(
                ComparisonOperator::GreaterEqual,
                observed("value", StateObservation::Current, 600),
                integer(0, &wide, 601),
                600,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("wide precondition should type-check");
    let narrow_postcondition = narrow_environment
        .check_expression(
            &compare(
                ComparisonOperator::LessEqual,
                observed("value", StateObservation::Current, 610),
                integer(10, &narrow, 611),
                610,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("narrow postcondition should type-check");
    let conflicting_request = request(
        &wide_environment,
        &wide_precondition,
        &narrow_postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    let diagnostic = &generate_kani_bundle(&conflicting_request)
        .expect_err("conflicting domains should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::UnsupportedBinding);

    let signed = IntegerType::new(IntegerDomain::Signed, -1000, 1000, OverflowPolicy::Saturate)
        .expect("fixture integer bounds should be valid");
    let unsupported_environment = numeric_environment(&[]);
    let unsupported_span = span(70, 71);
    let unsupported_precondition = unsupported_environment
        .check_expression(
            &compare(
                ComparisonOperator::Equal,
                Expression::new(
                    ExpressionKind::NumericNegate {
                        operand: Box::new(integer(1, &signed, 71)),
                    },
                    unsupported_span.clone(),
                ),
                integer(-1, &signed, 72),
                70,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("unsupported lowering fixture should still type-check");
    let supported_postcondition = unsupported_environment
        .check_expression(
            &Expression::new(ExpressionKind::BooleanLiteral { value: true }, span(73, 74)),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("supported postcondition fixture should type-check");
    let unsupported_clause_request = request(
        &unsupported_environment,
        &unsupported_precondition,
        &supported_postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    let diagnostic = &generate_kani_bundle(&unsupported_clause_request)
        .expect_err("unsupported clause construct should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::ClauseGenerationFailed);
    assert_eq!(
        diagnostic.generation_code,
        Some(GenerationErrorCode::UnsupportedExpression)
    );
    assert_eq!(diagnostic.source_span.as_ref(), Some(&unsupported_span));
    let mut legacy_value = serde_json::to_value(diagnostic).expect("diagnostic should serialize");
    legacy_value
        .as_object_mut()
        .expect("diagnostic wire value should be an object")
        .remove("sourceSpan");
    let legacy_diagnostic: KaniDiagnostic =
        serde_json::from_value(legacy_value).expect("legacy diagnostic should deserialize");
    let mut expected_legacy = diagnostic.clone();
    expected_legacy.source_span = None;
    assert_eq!(legacy_diagnostic, expected_legacy);

    let oversized_proof_id = "x".repeat(MAX_GENERATED_SOURCE_BYTES + 1);
    let mut oversized_request = request(
        &environment,
        &precondition,
        &postcondition,
        &precondition_clause,
        &postcondition_clause,
        &[],
    );
    oversized_request.proof_id = &oversized_proof_id;
    let diagnostic = &generate_kani_bundle(&oversized_request)
        .expect_err("oversized generated source should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::ResourceLimitExceeded);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::Unsupported
    );
    assert_eq!(diagnostic.path, "generated.rust");
}

/// TC-007
#[test]
fn generated_kani_predicates_are_the_exact_executable_oracles_for_the_boolean_corpus() {
    let environment = environment();
    let postcondition = environment
        .check_expression(
            &observed("state", StateObservation::Post, 200),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("postcondition fixture should type-check");
    let postcondition_clause =
        ClauseId::new("postcondition").expect("fixture clause should be valid");
    for (index, operator) in [
        BooleanOperator::ShortCircuitAnd,
        BooleanOperator::ShortCircuitOr,
        BooleanOperator::TotalAnd,
        BooleanOperator::TotalOr,
        BooleanOperator::Implication,
    ]
    .into_iter()
    .enumerate()
    {
        let at = 300 + index as u64 * 3;
        let precondition = environment
            .check_expression(
                &Expression::new(
                    ExpressionKind::Boolean {
                        operator,
                        left: Box::new(observed("input", StateObservation::Current, at)),
                        right: Box::new(observed("state", StateObservation::Pre, at + 1)),
                    },
                    span(at, at + 2),
                ),
                &ValueType::Boolean,
                &handler(),
                true,
            )
            .expect("Boolean corpus clause should type-check");
        let precondition_clause =
            ClauseId::new(format!("precondition-{index}")).expect("fixture clause should be valid");
        let executable_precondition = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &precondition_clause,
            expression: &precondition,
            attestation: attestation_context(),
        })
        .expect("executable precondition should generate");
        let executable_postcondition = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &postcondition_clause,
            expression: &postcondition,
            attestation: attestation_context(),
        })
        .expect("executable postcondition should generate");
        let bundle = generate_kani_bundle(&request(
            &environment,
            &precondition,
            &postcondition,
            &precondition_clause,
            &postcondition_clause,
            &[],
        ))
        .expect("Kani corpus bundle should generate");
        assert!(bundle
            .rust
            .contents
            .contains(&executable_precondition.rust.contents));
        assert!(bundle
            .rust
            .contents
            .contains(&executable_postcondition.rust.contents));
    }
}

/// TC-007
#[test]
fn pinned_kani_executes_the_generated_contract_proof() {
    let version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .expect("cargo-kani must be installed for the pinned adapter test");
    assert!(version.status.success(), "cargo-kani version query failed");
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        format!("cargo-kani {KANI_BACKEND_VERSION}")
    );

    let bundle = fixture_bundle(&[]);
    let graph: ProofDependencyGraph =
        serde_json::from_str(&bundle.proof_graph.contents).expect("graph should deserialize");
    let directory = write_generated_crate(
        &bundle,
        "/// Customer transition under proof.\npub fn subject(input: bool, pre_state: bool) -> bool { input || pre_state }",
    );
    let listing = Command::new("cargo")
        .args([
            "kani",
            "list",
            "-Z",
            "function-contracts",
            "--format",
            "json",
        ])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani list should launch");
    assert!(
        listing.status.success(),
        "Kani harness listing failed: {}",
        String::from_utf8_lossy(&listing.stderr)
    );
    let listing_json = fs::read_to_string(directory.0.join("kani-list.json"))
        .expect("Kani JSON listing should be retained");
    let verification = Command::new("cargo")
        .arg("kani")
        .args(&graph.options)
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    assert!(
        verification.status.success(),
        "generated Kani proof failed:\nlisting:\n{}\nstdout:\n{}\nstderr:\n{}",
        listing_json,
        String::from_utf8_lossy(&verification.stdout),
        String::from_utf8_lossy(&verification.stderr)
    );
    let conditional_dependencies = [
        ProofDependencyRequest {
            proof_id: "proof-assumed",
            kind: ProofDependencyKind::Assumed,
            state: ProofDependencyState::Assumed,
            original_path: Some("crate::dependency_predicate"),
            replacement_path: None,
        },
        ProofDependencyRequest {
            proof_id: "proof-stubbed",
            kind: ProofDependencyKind::Stubbed,
            state: ProofDependencyState::Stubbed,
            original_path: Some("crate::original"),
            replacement_path: Some("crate::replacement"),
        },
    ];
    let conditional_bundle = fixture_bundle(&conditional_dependencies);
    let conditional_graph: ProofDependencyGraph =
        serde_json::from_str(&conditional_bundle.proof_graph.contents)
            .expect("conditional graph should deserialize");
    assert_eq!(conditional_graph.readiness, ProofReadiness::Conditional);
    let conditional_directory = write_generated_crate(
        &conditional_bundle,
        "/// Customer transition under proof.\npub fn subject(input: bool, pre_state: bool) -> bool { input || pre_state }",
    );
    let conditional_verification = Command::new("cargo")
        .arg("kani")
        .args(&conditional_graph.options)
        .env("CARGO_TARGET_DIR", conditional_directory.0.join("target"))
        .current_dir(&conditional_directory.0)
        .output()
        .expect("conditional cargo kani should launch");
    assert!(
        conditional_verification.status.success(),
        "conditional generated Kani proof failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&conditional_verification.stdout),
        String::from_utf8_lossy(&conditional_verification.stderr)
    );
}
