use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    generate_boolean_oracle, generate_kani_bundle, GenerationTerminalState, KaniDerivationManifest,
    KaniErrorCode, KaniRequest, KaniSolver, ManifestContext, OracleRequest, ProofDependencyGraph,
    ProofDependencyKind, ProofDependencyRequest, ProofDependencyState, ProofReadiness,
    IR_CANDIDATE_REVISION, KANI_ADAPTER_PROFILE, KANI_BACKEND_VERSION, RUNTIME_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, DeclarationEnvironment, ExecutionPoint, Expression,
    ExpressionKind, PackageId, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, ValueDeclaration, ValueDeclarationKind, ValueType,
};

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

fn manifest_context() -> ManifestContext<'static> {
    ManifestContext {
        candidate_revision: IR_CANDIDATE_REVISION,
        contribution_method: "generated",
        reviewers: &["@kani-test-reviewer"],
        result_status: "pending",
        result_summary: "proof execution has not run",
        requirement_refs: &["FR-003", "TC-005", "TC-007"],
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
        manifest: manifest_context(),
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
) -> TemporaryDirectory {
    let directory = TemporaryDirectory::new("quire-generated-kani");
    fs::create_dir_all(directory.0.join("src")).expect("source directory should be writable");
    fs::write(
        directory.0.join("src/lib.rs"),
        format!(
            "{}\n/// Customer transition under proof.\npub fn subject(input: bool, pre_state: bool) -> bool {{ input || pre_state }}\n\n/// Assumed dependency predicate.\npub fn dependency_predicate() -> bool {{ true }}\n\n/// Original dependency implementation.\npub fn original() -> bool {{ false }}\n\n/// Replacement dependency implementation.\npub fn replacement() -> bool {{ true }}\n",
            bundle.rust.contents
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

/// TC-003 / TC-005 / TC-007.
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
        include_str!("../schemas/kani-proof-graph-v1.schema.json"),
        &serde_json::from_str(&first.proof_graph.contents).expect("graph JSON should parse"),
    );
    validate(
        include_str!("../schemas/generated-rust-kani-v1.schema.json"),
        &serde_json::Value::String(first.rust.contents.clone()),
    );

    let manifest: KaniDerivationManifest =
        serde_json::from_str(&first.manifest.contents).expect("manifest should deserialize");
    validate(
        include_str!("../schemas/pgm01-derivation-evidence-envelope-v1.schema.json"),
        &serde_json::from_str(&first.manifest.contents).expect("manifest JSON should parse"),
    );
    assert_eq!(manifest.backend.kind, "tool");
    assert_eq!(manifest.backend.version, KANI_BACKEND_VERSION);
    assert_eq!(manifest.result.status, "pending");
    let extension = &manifest.extensions["dev.agent-ix.codegen.kani"];
    assert_eq!(extension.proof_execution_state, "not-run");
    assert_eq!(extension.dependency_readiness, ProofReadiness::Ready);

    let directory = write_generated_crate(&first);
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

/// TC-005.
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

    let graph_schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/kani-proof-graph-v1.schema.json"))
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

/// TC-003.
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
    value.subject_path = "not::a::valid::path::";
    let diagnostic = &generate_kani_bundle(&value).expect_err("subject should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidIdentity);

    value.subject_path = "crate::subject";
    value.manifest.result_status = "conclusive";
    let diagnostic = &generate_kani_bundle(&value).expect_err("status should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::InvalidManifestContext);

    value.manifest = manifest_context();
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

    let unsupported_precondition = environment
        .check_expression(
            &boolean_or(
                observed("input", StateObservation::Current, 50),
                observed("state", StateObservation::Current, 51),
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
        .expect_err("unsupported binding should be rejected")[0];
    assert_eq!(diagnostic.code, KaniErrorCode::UnsupportedBinding);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::Unsupported
    );
}

/// TC-007.
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
            manifest: manifest_context(),
        })
        .expect("executable precondition should generate");
        let executable_postcondition = generate_boolean_oracle(&OracleRequest {
            requirement: environment.owner(),
            clause: &postcondition_clause,
            expression: &postcondition,
            manifest: manifest_context(),
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

/// TC-007.
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
    let directory = write_generated_crate(&bundle);
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
    assert!(String::from_utf8_lossy(&verification.stdout).contains("VERIFICATION:- SUCCESSFUL"));

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
    let conditional_directory = write_generated_crate(&conditional_bundle);
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
