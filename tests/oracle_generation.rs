use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    generate_boolean_oracle, generator_source_is_dirty, DerivationManifest, GenerationErrorCode,
    GenerationTerminalState, ManifestContext, OracleRequest, GENERATOR_SOURCE_REVISION,
    IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, NumericOperator,
    OverflowPolicy, PackageId, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, ValueDeclaration, ValueDeclarationKind, ValueType,
};
use sha2::{Digest as _, Sha256};

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

fn manifest_context() -> ManifestContext<'static> {
    ManifestContext {
        candidate_revision: IR_CANDIDATE_REVISION,
        contribution_method: "generated",
        reviewers: &["@oracle-test-reviewer"],
        result_status: "conclusive",
        result_summary: "test oracle generated from a supported Boolean expression",
        requirement_refs: &["FR-001"],
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
        include_bytes!("../schemas/pgm01-derivation-evidence-envelope-v1.schema.json").as_slice(),
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
/// that test: it asserts the emitted manifest records the producer's source
/// revision, the schema digests of both outputs, the backend, the parameter and
/// dependency digests, the output content digests, and the generator's own
/// executable digest. Manifest `inputs` are required by the envelope schema the
/// same assertion validates against, rather than asserted field by field. The tag
/// used to sit on the retained-evidence compatibility test instead, which read
/// identity out of historical records rather than out of an artifact this crate
/// emits. Those records are deleted; the tag moves to the test the requirement
/// always named.
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
        manifest: manifest_context(),
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

    let manifest: DerivationManifest = serde_json::from_str(&first.manifest.contents).unwrap();
    let manifest_value: serde_json::Value = serde_json::from_str(&first.manifest.contents).unwrap();
    let schema: serde_json::Value = serde_json::from_str(include_str!(
        "../schemas/pgm01-derivation-evidence-envelope-v1.schema.json"
    ))
    .unwrap();
    let validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .unwrap();
    let validation_errors = validator
        .validate(&manifest_value)
        .err()
        .map(|errors| errors.map(|error| error.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    assert!(validation_errors.is_empty(), "{validation_errors:?}");
    let mut malformed_manifest = manifest_value.clone();
    malformed_manifest
        .as_object_mut()
        .unwrap()
        .remove("producer");
    assert!(validator.validate(&malformed_manifest).is_err());
    let mut extended_manifest = manifest_value.clone();
    extended_manifest["unexpected"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<DerivationManifest>(extended_manifest).is_err());
    assert_eq!(
        sha256(include_bytes!(
            "../schemas/pgm01-derivation-evidence-envelope-v1.schema.json"
        )),
        "0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256"
    );
    assert_eq!(manifest.schema_version, "quire.derivation-evidence/v1");
    assert_eq!(manifest.backend.kind, "none");
    let git_head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(git_head.status.success());
    assert_eq!(
        manifest.producer.source_revision,
        String::from_utf8(git_head.stdout).unwrap().trim()
    );
    assert_eq!(manifest.producer.source_revision, GENERATOR_SOURCE_REVISION);
    assert_eq!(
        manifest.producer.executable_digest.value,
        expected_implementation_digest()
    );
    let parameters = format!(
        "ir={IR_CANDIDATE_REVISION}\nruntime={RUNTIME_REVISION}\nmaximumSourceBytes={MAX_GENERATED_SOURCE_BYTES}\n"
    );
    assert_eq!(
        manifest.parameters_digest.value,
        sha256(parameters.as_bytes())
    );
    assert_eq!(
        manifest.environment.dependencies_digest.value,
        sha256(include_bytes!("../Cargo.lock"))
    );
    assert_eq!(manifest.result.status, "conclusive");
    assert_eq!(manifest.result.requirement_refs, ["FR-001"]);
    assert_eq!(
        manifest.provenance.candidate_revision,
        IR_CANDIDATE_REVISION
    );
    assert_eq!(manifest.provenance.contribution_method, "generated");
    assert_eq!(manifest.provenance.reviewers, ["@oracle-test-reviewer"]);
    assert_eq!(manifest.outputs[0].content_digest.value, first.rust.sha256);
    assert_eq!(
        manifest.outputs[1].content_digest.value,
        first.source_map.sha256
    );
    let extension = &manifest.extensions["dev.agent-ix.codegen"];
    assert_eq!(extension.terminal_state, GenerationTerminalState::Generated);
    assert!(!extension.generator_source_dirty);
    assert!(!generator_source_is_dirty());
    assert!(extension.generator_source_revision_available);
    assert_eq!(extension.ir_revision, IR_CANDIDATE_REVISION);
    assert_eq!(extension.runtime_revision, RUNTIME_REVISION);
    assert_eq!(extension.maximum_source_bytes, MAX_GENERATED_SOURCE_BYTES);
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
    assert_eq!(
        manifest.outputs[0].schema.digest.value,
        sha256(include_bytes!(
            "../schemas/generated-rust-oracle-v1.schema.json"
        ))
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
            manifest: manifest_context(),
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
            manifest: manifest_context(),
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
        manifest: manifest_context(),
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
        manifest: manifest_context(),
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
        manifest: manifest_context(),
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
        manifest: manifest_context(),
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

    let invalid_manifest = ManifestContext {
        candidate_revision: "not-a-revision",
        contribution_method: "unspecified",
        reviewers: &[],
        result_status: "complete",
        result_summary: "",
        requirement_refs: &[],
    };
    let context_diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
        manifest: invalid_manifest,
    })
    .unwrap_err()[0];
    assert_eq!(
        context_diagnostic.code,
        GenerationErrorCode::InvalidManifestContext
    );
    assert_eq!(
        context_diagnostic.terminal_state,
        GenerationTerminalState::InvalidInput
    );
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
        manifest: manifest_context(),
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
    let mut manifest_paths = BTreeSet::new();
    for clause_value in ["Clause-A", "clause_a", "clause-a", "CLAUSE.A"] {
        let clause = ClauseId::new(clause_value).unwrap();
        let bundle = generate_boolean_oracle(&OracleRequest {
            requirement: literal_environment.owner(),
            clause: &clause,
            expression: &typed_literal,
            manifest: manifest_context(),
        })
        .unwrap();
        rust_paths.insert(bundle.rust.path);
        map_paths.insert(bundle.source_map.path);
        manifest_paths.insert(bundle.manifest.path);
    }
    assert_eq!(rust_paths.len(), 4);
    assert_eq!(map_paths.len(), 4);
    assert_eq!(manifest_paths.len(), 4);

    let long_clause = ClauseId::new("x".repeat(400)).unwrap();
    let long_bundle = generate_boolean_oracle(&OracleRequest {
        requirement: literal_environment.owner(),
        clause: &long_clause,
        expression: &typed_literal,
        manifest: manifest_context(),
    })
    .unwrap();
    for path in [
        &long_bundle.rust.path,
        &long_bundle.source_map.path,
        &long_bundle.manifest.path,
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
        manifest: manifest_context(),
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
                manifest: manifest_context(),
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
                manifest: manifest_context(),
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
}
