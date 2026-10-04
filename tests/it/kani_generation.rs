use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::scratch_crate::{runtime_dependency, write_manifest};
use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    classify_kani_run, generate_boolean_oracle, generate_kani_bundle, GenerationErrorCode,
    GenerationTerminalState, KaniBindingRole, KaniDiagnostic, KaniErrorCode,
    KaniInconclusiveReason, KaniPrimitiveType, KaniRequest, KaniRunOutcome, KaniSolver,
    OracleRequest, ProofDependencyGraph, ProofDependencyKind, ProofDependencyRequest,
    ProofDependencyState, ProofReadiness, MAX_GENERATED_SOURCE_BYTES, MAX_OBLIGATION_UNWIND,
};
use quire_contract_model::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, OverflowPolicy,
    PackageId, RequirementId, RequirementRef, RequirementRevision, SourceDocumentId,
    SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation, SymbolName,
    ValueDeclaration, ValueDeclarationKind, ValueType,
};

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

fn clauses(
    environment: &DeclarationEnvironment,
) -> (
    quire_contract_model::TypedExpression,
    quire_contract_model::TypedExpression,
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
    precondition: &'a quire_contract_model::TypedExpression,
    postcondition: &'a quire_contract_model::TypedExpression,
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
        unwind: 2,
        solver: KaniSolver::Cadical,
        dependencies,
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
    write_manifest(
        &directory.0,
        &format!(
            "[package]\nname = \"generated-kani-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n# `exact` is required here because RT's own `#[cfg(kani)] mod verification` unconditionally\n# imports `crate::exact` (verification/kani.rs), independent of whether this fixture's subject\n# uses exact-scalar types. Building this generated crate under `cargo kani` without the feature\n# fails with E0432 on RT's own module, not on anything this generator emitted.\n{}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
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

/// The harness sources `generate_kani_bundle` emits, for the cover-last guard (FR-015-AC-58): a
/// bundle with no dependency and one whose census assumes and stubs, which add statements before
/// the contract call.
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let census = [
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
    vec![
        ("v1 bundle", fixture_bundle(&[]).rust.contents),
        (
            "v1 bundle with an assumed and a stubbed dependency",
            fixture_bundle(&census).rust.contents,
        ),
    ]
}

/// A bundle whose requires clause (`input && false`) no bounded argument satisfies.
fn unsatisfiable_requires_bundle() -> quire_contract_codegen::KaniArtifactBundle {
    let environment = environment();
    let (_, postcondition) = clauses(&environment);
    let precondition = environment
        .check_expression(
            &boolean_and(
                observed("input", StateObservation::Current, 10),
                Expression::new(
                    ExpressionKind::BooleanLiteral { value: false },
                    span(12, 13),
                ),
                10,
            ),
            &ValueType::Boolean,
            &handler(),
            true,
        )
        .expect("unsatisfiable precondition fixture should type-check");
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
        &[],
    ))
    .expect("unsatisfiable-requires bundle should generate")
}

/// Runs `bundle` over `subject` under the installed backend with the options its own graph
/// records plus the report export, and classifies the run as production does.
fn classify_bundle(
    bundle: &quire_contract_codegen::KaniArtifactBundle,
    subject: &str,
) -> KaniRunOutcome {
    let graph: ProofDependencyGraph =
        serde_json::from_str(&bundle.proof_graph.contents).expect("graph should deserialize");
    let directory = write_generated_crate(bundle, subject);
    let report_path = directory.0.join("report.json");
    let output = Command::new("cargo")
        .arg("kani")
        .args(&graph.options)
        .args(["-Z", "unstable-options", "--export-json"])
        .arg(&report_path)
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = fs::read(&report_path).ok();
    classify_kani_run(output.status.success(), report.as_deref(), &text, None)
        .unwrap_or_else(|refusal| panic!("the run was refused as {refusal:?}:\n{text}"))
        .outcome
}

const HEALTHY_BUNDLE_SUBJECT: &str = "/// Customer transition under proof.\npub fn subject(input: bool, pre_state: bool) -> bool { input || pre_state }";

/// A bundle whose requires some bounded argument satisfies and whose `ensures` holds for every
/// such argument is `Verified` (its cover after the contract call is satisfied), a bundle whose
/// requires no bounded argument satisfies is `CoverUnsatisfied` with no satisfied cover of one,
/// and a broken `ensures` is `Falsified` with the cover unreachable. A bundle without its cover
/// carries no cover summary, so the cover is what makes the first of these a proof.
///
/// Trace: FR-015-AC-54, FR-015-AC-56, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_classifies_the_v1_bundle_verified_vacuous_and_falsified() {
    let healthy = fixture_bundle(&[]);
    assert_eq!(
        classify_bundle(&healthy, HEALTHY_BUNDLE_SUBJECT),
        KaniRunOutcome::Verified
    );

    let uncovered = quire_contract_codegen::KaniArtifactBundle {
        rust: quire_contract_codegen::Artifact::new(
            healthy.rust.path.clone(),
            healthy
                .rust
                .contents
                .lines()
                .filter(|line| !line.contains("kani::cover!"))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        proof_graph: healthy.proof_graph.clone(),
    };
    assert_eq!(
        classify_bundle(&uncovered, HEALTHY_BUNDLE_SUBJECT),
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MissingCoverSummary
        },
        "a bundle harness with no cover must not classify as Verified"
    );

    assert_eq!(
        classify_bundle(
            &unsatisfiable_requires_bundle(),
            "/// Customer transition under proof.\npub fn subject(input: bool) -> bool { input }"
        ),
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );

    let broken = classify_bundle(
        &healthy,
        "/// Violates the ensures clause.\npub fn subject(_input: bool, _pre_state: bool) -> bool { false }",
    );
    assert!(
        matches!(broken, KaniRunOutcome::Falsified { .. }),
        "a broken ensures must be falsified, got {broken:?}"
    );
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
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
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
    let first = generate_kani_bundle(&request).expect("mixed numeric Kani bundle should generate");
    let second =
        generate_kani_bundle(&request).expect("repeated mixed numeric Kani bundle should generate");
    assert_eq!(first, second);

    let graph: ProofDependencyGraph =
        serde_json::from_str(&first.proof_graph.contents).expect("v2 graph should deserialize");
    assert_eq!(graph.schema_version, "quire.kani-proof-graph/v2");
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
    write_manifest(
        &directory.0,
        &format!(
            "[package]\nname = \"generated-kani-oracle-corpus\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\n\n[workspace]\n",
            runtime_dependency(&[])
        ),
    );
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
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_proves_identity_and_prints_numeric_counterexamples() {
    let version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .expect("cargo-kani must be installed for the numeric adapter test");
    assert!(version.status.success(), "cargo-kani version query failed");

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
    let state_bundle =
        generate_kani_bundle(&state_request).expect("state identity bundle should generate");
    let state_graph: ProofDependencyGraph =
        serde_json::from_str(&state_bundle.proof_graph.contents)
            .expect("state graph should deserialize");
    assert_eq!(state_graph.subject_arguments.len(), 1);
    assert_eq!(state_graph.subject_results.len(), 1);

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

    value.subject_path = "not::a::valid::path::";
    let diagnostic = &generate_kani_bundle(&value).expect_err("subject should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidIdentity);

    value.subject_path = "crate::subject";
    value.unwind = 0;
    let diagnostic = &generate_kani_bundle(&value).expect_err("unwind should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidUnwind);

    value.unwind = MAX_OBLIGATION_UNWIND + 1;
    let diagnostic = &generate_kani_bundle(&value)
        .expect_err("unwind above the declared bound should be rejected")[0];
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
        })
        .expect("executable precondition should generate");
        let executable_postcondition = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &postcondition_clause,
            expression: &postcondition,
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
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_executes_the_generated_contract_proof() {
    let version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .expect("cargo-kani must be installed for the adapter test");
    assert!(version.status.success(), "cargo-kani version query failed");

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

/// The `i64` values Kani's concrete playback assigns the harness's symbolic inputs, in order, of
/// the failing check. A harness with a non-vacuity cover also prints the cover's own playback
/// ("Check for `cover`"), which is a satisfying valuation and not the counterexample, so that
/// block is skipped.
fn playback_values(output: &std::process::Output) -> Vec<i64> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut in_cover_block = false;
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| {
            if line.starts_with("Concrete playback unit test") {
                in_cover_block = false;
            } else if line.starts_with("/// Check for `cover`") {
                in_cover_block = true;
            }
            !in_cover_block && line.starts_with("vec![") && !line.contains("concrete_vals")
        })
        .map(|line| {
            let bytes = line
                .trim_start_matches("vec![")
                .trim_end_matches("],")
                .split(',')
                .map(|byte| byte.trim().parse::<u8>().expect("a playback byte"))
                .collect::<Vec<_>>();
            i64::from_le_bytes(bytes.try_into().expect("eight bytes per i64"))
        })
        .collect()
}

/// `bundle` with its generated source replaced by `contents`, keeping the graph and its options.
fn mutated(
    bundle: &quire_contract_codegen::KaniArtifactBundle,
    from: &str,
    to: &str,
) -> quire_contract_codegen::KaniArtifactBundle {
    assert_eq!(
        bundle.rust.contents.matches(from).count(),
        1,
        "the mutation applies to exactly one site of:\n{}",
        bundle.rust.contents
    );
    let mut mutant = bundle.clone();
    mutant.rust.contents = bundle.rust.contents.replacen(from, to, 1);
    mutant
}

/// The exemplar's controls, run with real Kani through `generate_kani_bundle`: QSL's IT-011 and
/// IT-010-SC-05 clause (`amount < 1000` implies `amount + 1 <= 1000` over `0..=1000`) verifies;
/// the mutant that appends ` + 1_i64` to the addition (`left + right + 1`) is falsified with the
/// counterexample `amount_current = 999`; the equivalent mutant that drops the `+ 1` verifies. The
/// bundle holds exactly one syntactic addition, the shape the mutation targets. This is the
/// in-repo equivalent of the QSL exemplar run, which needs a checkout of the integration repository
/// and is recorded separately (FR-031-AC-11).
///
/// Trace: TC-044, FR-031-AC-11
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_exemplar_verifies_and_its_addition_mutant_is_falsified_at_999() {
    const SUBJECT: &str =
        "/// Customer transition under proof.\npub fn subject(_amount_current: i64) {}";
    let bundle = crate::oracle_arithmetic::exemplar()
        .bundle()
        .expect("the exemplar bundle generates");
    let oracle = crate::oracle_arithmetic::post_oracle(&bundle.rust.contents);
    let mut scanned = crate::oracle_arithmetic::Scan::default();
    syn::visit::Visit::visit_item_fn(&mut scanned, &oracle);
    assert_eq!(scanned.count("+"), 1, "exactly one addition to extend");

    let healthy = execute_kani(&bundle, SUBJECT);
    assert!(
        healthy.status.success(),
        "the healthy exemplar must verify:\n{}\n{}",
        String::from_utf8_lossy(&healthy.stdout),
        String::from_utf8_lossy(&healthy.stderr)
    );
    assert!(String::from_utf8_lossy(&healthy.stdout).contains("VERIFICATION:- SUCCESSFUL"));

    // `left + right + 1`: the oracle's addition gains ` + 1_i64`.
    let non_equivalent = mutated(
        &bundle,
        "(\n1_i64\n)\n)\n<=\n",
        "(\n1_i64\n)\n+\n1_i64\n)\n<=\n",
    );
    let falsified = execute_kani(&non_equivalent, SUBJECT);
    assert!(
        !falsified.status.success(),
        "the non-equivalent mutant must be falsified:\n{}",
        String::from_utf8_lossy(&falsified.stdout)
    );
    assert_eq!(
        playback_values(&falsified),
        [999],
        "{}",
        String::from_utf8_lossy(&falsified.stdout)
    );

    // Dropping the `+ 1` leaves `amount <= 1000`, which the clause's antecedent already implies.
    let equivalent = mutated(&bundle, "\n+\n(\n1_i64\n)", "");
    let verified = execute_kani(&equivalent, SUBJECT);
    assert!(
        verified.status.success(),
        "the equivalent mutant must verify:\n{}\n{}",
        String::from_utf8_lossy(&verified.stdout),
        String::from_utf8_lossy(&verified.stderr)
    );
}

/// Overflow in the bundle oracle is a falsifiable Kani property, never a wrapped value: the
/// oracle of `x < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained,
/// fails Kani's "attempt to multiply with overflow" check, and the counterexample's playback value
/// satisfies `x < 5` and overflows `x * 2` (no particular value is named); a harness that assumes
/// the declared domain on `x` verifies the same oracle.
///
/// Trace: TC-044, FR-031-AC-20
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value() {
    let bundle = crate::oracle_arithmetic::o4_multiply(ComparisonOperator::LessEqual, 10)
        .bundle()
        .expect("the bundle generates");
    let oracle = source_symbol(
        bundle
            .rust
            .contents
            .lines()
            .find(|line| line.starts_with("pub fn ") && line.contains("_post("))
            .expect("the postcondition oracle"),
    );
    let oracle = oracle.to_owned();
    let subject = format!(
        "/// Customer transition under proof.\npub fn subject(_x_current: i64) {{}}\n\n#[cfg(kani)]\nmod overflow_probe {{\n    use super::*;\n\n    #[kani::proof]\n    fn unconstrained() {{\n        let x: i64 = kani::any();\n        let _ = {oracle}(x);\n    }}\n\n    #[kani::proof]\n    fn domain_assumed() {{\n        let x: i64 = kani::any();\n        kani::assume(x >= 0_i64 && x <= 10_i64);\n        let _ = {oracle}(x);\n    }}\n}}\n"
    );
    let run = |harness: &str| {
        let directory = write_generated_crate(&bundle, &subject);
        Command::new("cargo")
            .args([
                "kani",
                "-Z",
                "function-contracts",
                "-Z",
                "concrete-playback",
                "--harness",
                harness,
                "--exact",
                "--unwind",
                "2",
                "--solver",
                "cadical",
                "--output-format",
                "regular",
                "--concrete-playback",
                "print",
            ])
            .env("CARGO_TARGET_DIR", directory.0.join("target"))
            .current_dir(&directory.0)
            .output()
            .expect("cargo kani should launch")
    };

    let unconstrained = run("overflow_probe::unconstrained");
    let stdout = String::from_utf8_lossy(&unconstrained.stdout);
    assert!(!unconstrained.status.success(), "{stdout}");
    assert!(
        stdout.contains("attempt to multiply with overflow"),
        "the failing check names the multiplication:\n{stdout}"
    );
    let values = playback_values(&unconstrained);
    let [x] = values[..] else {
        panic!("one symbolic input in the playback, found {values:?}\n{stdout}");
    };
    assert!(x < 5, "the playback value passes the guard: {x}");
    assert!(x.checked_mul(2).is_none(), "and overflows `x * 2`: {x}");

    let assumed = run("overflow_probe::domain_assumed");
    assert!(
        assumed.status.success(),
        "the declared domain discharges the overflow check:\n{}\n{}",
        String::from_utf8_lossy(&assumed.stdout),
        String::from_utf8_lossy(&assumed.stderr)
    );
}
