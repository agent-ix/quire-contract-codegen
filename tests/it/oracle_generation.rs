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
    generate_boolean_oracle, GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
    OracleRequest, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, NumericOperator,
    OverflowPolicy, PackageId, RationalType, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, ValueDeclaration, ValueDeclarationKind, ValueType,
};

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

fn integer_environment(value_type: &IntegerType) -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        requirement(),
        vec![],
        [
            ("x", ValueDeclarationKind::Input),
            ("version", ValueDeclarationKind::State),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (value, kind))| {
            ValueDeclaration::new(
                name(value),
                kind,
                ValueType::integer(value_type.clone()),
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

fn integer_literal(value: i64, value_type: &IntegerType, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::IntegerLiteral {
            value,
            value_type: value_type.clone(),
        },
        span(at, at + 1),
    )
}

fn comparison(
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

/// Trace: TC-001, TC-006, FR-001-AC-5
#[test]
fn tc_006_generated_oracle_probes_qualify_against_native_llvm_export() {
    use quire_contract_codegen::{
        classify_clause, parse_llvm_coverage, ClauseCoverage, SourceRegion,
    };
    let directory = TemporaryDirectory::new("quire-native-vacuity");
    let environment = boolean_environment(&["a", "b"]);
    let implication = || {
        boolean_op(
            BooleanOperator::Implication,
            value("a", 2),
            value("b", 3),
            1,
        )
    };
    let expressions = [
        implication(),
        value("a", 1),
        boolean_op(
            BooleanOperator::TotalAnd,
            implication(),
            boolean_op(
                BooleanOperator::Implication,
                value("b", 5),
                value("a", 6),
                4,
            ),
            0,
        ),
        implication(),
        boolean_op(
            BooleanOperator::Implication,
            value("a", 0),
            boolean_op(
                BooleanOperator::Implication,
                value("b", 1),
                value("a", 2),
                3,
            ),
            4,
        ),
        implication(),
    ];
    let mut generated = Vec::new();
    let mut modules = String::new();
    let mut calls = String::new();
    for (index, expression) in expressions.iter().enumerate() {
        let typed = environment
            .check_expression(expression, &ValueType::Boolean, &pre(), true)
            .unwrap();
        let clause = ClauseId::new(format!("coverage-{index}")).unwrap();
        let bundle = generate_boolean_oracle(&OracleRequest {
            requirement: &requirement(),
            clause: &clause,
            expression: &typed,
        })
        .unwrap();
        let path = directory.0.join(&bundle.rust.path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &bundle.rust.contents).unwrap();
        writeln!(
            modules,
            "#[path = {:?}] mod case_{index};",
            bundle.rust.path.strip_prefix("src/").unwrap()
        )
        .unwrap();
        let symbol = source_symbol(&bundle.rust.contents);
        match index {
            0 => writeln!(calls, "assert!(case_0::{symbol}(std::hint::black_box(false), std::hint::black_box(true)));"),
            1 => writeln!(calls, "assert!(case_1::{symbol}(std::hint::black_box(true)));"),
            2 => writeln!(calls, "assert!(!case_2::{symbol}(std::hint::black_box(true), std::hint::black_box(false)));"),
            4 => writeln!(calls, "assert!(case_4::{symbol}(std::hint::black_box(true), std::hint::black_box(false)));"),
            5 => writeln!(calls, "assert!(case_5::{symbol}(std::hint::black_box(true), std::hint::black_box(true)));"),
            _ => Ok(()),
        }.unwrap();
        generated.push(bundle);
    }
    fs::write(
        directory.0.join("src/lib.rs"),
        format!("#![allow(dead_code)]\n{modules}\n#[test] fn native_run() {{ {calls} }}"),
    )
    .unwrap();
    fs::write(directory.0.join("Cargo.toml"), format!("[package]\nname = \"native-vacuity\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n")).unwrap();
    let output_path = directory.0.join("coverage.json");
    let sysroot = Command::new("rustc")
        .args(["+stable", "--print", "sysroot"])
        .output()
        .unwrap();
    assert!(sysroot.status.success());
    let tools = PathBuf::from(String::from_utf8(sysroot.stdout).unwrap().trim())
        .join("lib/rustlib/x86_64-unknown-linux-gnu/bin");
    for tool in ["llvm-cov", "llvm-profdata"] {
        assert!(
            tools.join(tool).is_file(),
            "qualified llvm-tools must already be installed"
        );
    }
    let output = Command::new("cargo")
        .args([
            "+stable",
            "llvm-cov",
            "--offline",
            "--json",
            "--output-path",
        ])
        .arg(&output_path)
        .current_dir(&directory.0)
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .env("LLVM_COV", tools.join("llvm-cov"))
        .env("LLVM_PROFDATA", tools.join("llvm-profdata"))
        .env("CARGO_PROFILE_TEST_OPT_LEVEL", "0")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "native producer failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let bytes = fs::read(output_path).unwrap();
    let coverage = parse_llvm_coverage(&bytes, directory.0.to_str().unwrap()).unwrap();
    for (index, (bundle, expected)) in generated
        .iter()
        .zip([
            ClauseCoverage::Vacuous,
            ClauseCoverage::Exercised,
            ClauseCoverage::PartiallyExercised,
            ClauseCoverage::Unexecuted,
            ClauseCoverage::PartiallyExercised,
            ClauseCoverage::Exercised,
        ])
        .enumerate()
    {
        let map: Vec<SourceRegion> = serde_json::from_str(&bundle.source_map.contents).unwrap();
        let envelope = map.iter().find(|region| region.role == "clause").unwrap();
        assert_eq!(
            envelope.expected_consequents,
            Some([1, 0, 2, 1, 2, 1][index])
        );
        assert_eq!(
            map.len(),
            envelope.expected_consequents.unwrap() as usize + 2
        );
        let evaluation = map
            .iter()
            .find(|region| region.role == "oracle_evaluation")
            .unwrap();
        let observe = |region: &SourceRegion| {
            coverage
                .observe(&region.artifact_path, region.probe.unwrap())
                .unwrap()
        };
        let consequents = map
            .iter()
            .filter(|region| region.role == "implication_consequent")
            .map(observe)
            .collect::<Vec<_>>();
        assert_eq!(
            classify_clause(
                observe(evaluation),
                envelope.expected_consequents.unwrap(),
                &consequents
            )
            .unwrap(),
            expected
        );
    }
}

/// Trace: TC-001, FR-001-AC-1, FR-001-AC-3
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
    // The symbol names the requirement, revision and clause the request carried, and the
    // generated oracle, compiled and run, evaluates `enabled -> false` to `!enabled`.
    let symbol = source_symbol(&first.rust.contents);
    assert_eq!(symbol, "oracle_fr_001_7_clause_main");
    assert!(first.rust.contents.contains(&format!(
        "pub fn {symbol}(enabled_current: bool) -> bool {{"
    )));
    run_generated_program(
        "generated-oracle-tc001",
        &format!(
            "#![deny(missing_docs)]\n//! TC-001 generated oracle.\n{}fn main() {{\n    \
             assert!({symbol}(false));\n    assert!(!{symbol}(true));\n}}\n",
            first.rust.contents
        ),
    );
    assert!(first
        .rust
        .contents
        .contains("quire_contract_runtime::RequirementId::new(\"FR-001\")"));
    assert!(first
        .rust
        .contents
        .contains("quire_contract_runtime::RevisionId::new(\"7\")"));
    assert!(first
        .rust
        .contents
        .contains("quire_contract_runtime::ClauseId::new(\"clause-main\")"));

    let source_map: Vec<quire_contract_codegen::SourceRegion> =
        serde_json::from_str(&first.source_map.contents).unwrap();
    let source_map_value: serde_json::Value =
        serde_json::from_str(&first.source_map.contents).unwrap();
    let source_map_schema: serde_json::Value = serde_json::from_str(include_str!(
        "../../schemas/oracle-source-map-v1.schema.json"
    ))
    .unwrap();
    let source_map_validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&source_map_schema)
        .unwrap();
    assert!(source_map_validator.validate(&source_map_value).is_ok());
    let mut missing_probe = source_map_value.clone();
    let evaluation_row = missing_probe
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|row| row["role"] == "oracle_evaluation")
        .unwrap();
    evaluation_row.as_object_mut().unwrap().remove("probe");
    assert!(source_map_validator.validate(&missing_probe).is_err());
    let mut missing_census = source_map_value.clone();
    missing_census[0]
        .as_object_mut()
        .unwrap()
        .remove("expectedConsequents");
    assert!(source_map_validator.validate(&missing_census).is_err());
    assert_eq!(source_map[0].expected_consequents, Some(1));
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

    // The generated Rust still validates against its own domain output contract.
    let rust_schema: serde_json::Value = serde_json::from_str(include_str!(
        "../../schemas/generated-rust-oracle-v1.schema.json"
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

#[derive(Clone, Copy)]
enum IntegerOperand {
    Literal(i64),
    Input,
    State(StateObservation),
}

impl IntegerOperand {
    fn expression(self, value_type: &IntegerType, at: u64) -> Expression {
        match self {
            Self::Literal(value) => integer_literal(value, value_type, at),
            Self::Input => observed_value("x", StateObservation::Current, at),
            Self::State(observation) => observed_value("version", observation, at),
        }
    }

    fn evaluate(self, input: i64, current: i64, pre: i64, post: i64) -> i64 {
        match self {
            Self::Literal(value) => value,
            Self::Input => input,
            Self::State(StateObservation::Current) => current,
            Self::State(StateObservation::Pre) => pre,
            Self::State(StateObservation::Post) => post,
        }
    }
}

#[derive(Clone, Copy)]
struct IntegerComparison {
    operator: ComparisonOperator,
    left: IntegerOperand,
    right: IntegerOperand,
}

impl IntegerComparison {
    fn expression(self, value_type: &IntegerType, at: u64) -> Expression {
        comparison(
            self.operator,
            self.left.expression(value_type, at + 1),
            self.right.expression(value_type, at + 2),
            at,
        )
    }

    fn evaluate(self, input: i64, current: i64, pre: i64, post: i64) -> bool {
        let left = self.left.evaluate(input, current, pre, post);
        let right = self.right.evaluate(input, current, pre, post);
        match self.operator {
            ComparisonOperator::Equal => left == right,
            ComparisonOperator::NotEqual => left != right,
            ComparisonOperator::Less => left < right,
            ComparisonOperator::LessEqual => left <= right,
            ComparisonOperator::Greater => left > right,
            ComparisonOperator::GreaterEqual => left >= right,
        }
    }

    const fn token(self) -> &'static str {
        match self.operator {
            ComparisonOperator::Equal => "==",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::Less => "<",
            ComparisonOperator::LessEqual => "<=",
            ComparisonOperator::Greater => ">",
            ComparisonOperator::GreaterEqual => ">=",
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

    run_generated_program("generated-oracle-differential", &generated_program);
}

/// Writes `program` as the `main.rs` of a scratch binary crate named `crate_name` that depends
/// on the runtime, then builds and runs it with warnings denied, asserting it exits cleanly.
fn run_generated_program(crate_name: &str, program: &str) {
    let directory = TemporaryDirectory::new(&format!("quire-codegen-{crate_name}"));
    let source_directory = directory.0.join("src");
    fs::create_dir_all(&source_directory).unwrap();
    fs::write(source_directory.join("main.rs"), program).unwrap();
    fs::write(
        directory.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{crate_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
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
        "{crate_name} did not compile and execute against runtime {RUNTIME_REVISION}: {}",
        String::from_utf8_lossy(&execution.stderr)
    );
}

/// TC-002
/// FR-001-AC-2
/// FR-001-AC-8
#[test]
fn tc_002_integer_and_state_comparisons_are_deterministic_compile_and_match_the_model() {
    let value_type =
        IntegerType::new(IntegerDomain::Signed, -2, 2, OverflowPolicy::Reject).unwrap();
    let environment = integer_environment(&value_type);
    let models = [
        IntegerComparison {
            operator: ComparisonOperator::Equal,
            left: IntegerOperand::Input,
            right: IntegerOperand::Literal(0),
        },
        IntegerComparison {
            operator: ComparisonOperator::NotEqual,
            left: IntegerOperand::State(StateObservation::Current),
            right: IntegerOperand::Input,
        },
        IntegerComparison {
            operator: ComparisonOperator::Less,
            left: IntegerOperand::State(StateObservation::Pre),
            right: IntegerOperand::State(StateObservation::Post),
        },
        IntegerComparison {
            operator: ComparisonOperator::LessEqual,
            left: IntegerOperand::State(StateObservation::Post),
            right: IntegerOperand::Literal(2),
        },
        IntegerComparison {
            operator: ComparisonOperator::Greater,
            left: IntegerOperand::Input,
            right: IntegerOperand::State(StateObservation::Pre),
        },
        IntegerComparison {
            operator: ComparisonOperator::GreaterEqual,
            left: IntegerOperand::State(StateObservation::Current),
            right: IntegerOperand::State(StateObservation::Post),
        },
    ];
    let values = [-3_i64, -2, 0, 2, 3];
    let mut generated_program =
        String::from("#![deny(warnings)]\n//! Numeric generated-oracle differential corpus.\n");
    let mut assertions = String::from("fn main() {\n");

    for (index, model) in models.into_iter().enumerate() {
        let expression = model.expression(&value_type, 2_000 + index as u64 * 3);
        let typed = environment
            .check_expression(&expression, &ValueType::Boolean, &handler(), true)
            .unwrap();
        assert!(typed.obligations().is_empty());
        let clause = ClauseId::new(format!("integer-differential-{index}")).unwrap();
        let request = OracleRequest {
            requirement: environment.owner(),
            clause: &clause,
            expression: &typed,
        };
        let bundle = generate_boolean_oracle(&request).unwrap();
        let repeated = generate_boolean_oracle(&request).unwrap();
        assert_eq!(
            bundle, repeated,
            "numeric generation changed for case {index}"
        );
        assert!(bundle.rust.contents.contains(": i64"));
        assert!(bundle
            .rust
            .contents
            .contains(&format!("\n{}\n", model.token())));
        assert!(!bundle.rust.contents.contains(": bool"));
        let symbol = source_symbol(&bundle.rust.contents).to_owned();
        generated_program.push_str(&bundle.rust.contents);

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
        for input in values {
            for current in values {
                for pre in values {
                    for post in values {
                        let arguments = parameters
                            .iter()
                            .map(|(parameter, observation)| {
                                match (*parameter, *observation) {
                                    ("x", StateObservation::Current) => input,
                                    ("version", StateObservation::Current) => current,
                                    ("version", StateObservation::Pre) => pre,
                                    ("version", StateObservation::Post) => post,
                                    other => panic!("unexpected numeric parameter {other:?}"),
                                }
                                .to_string()
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        assertions.push_str(&format!(
                            "assert_eq!({symbol}({arguments}), {});\n",
                            model.evaluate(input, current, pre, post)
                        ));
                    }
                }
            }
        }
    }
    assertions.push_str("}\n");
    generated_program.push_str(&assertions);

    let directory = TemporaryDirectory::new("quire-codegen-numeric-differential");
    let source_directory = directory.0.join("src");
    fs::create_dir_all(&source_directory).unwrap();
    fs::write(source_directory.join("main.rs"), generated_program).unwrap();
    fs::write(
        directory.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-numeric-oracle-differential\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .unwrap();
    let execution = Command::new("cargo")
        .args(["run", "--offline", "--quiet", "--target-dir"])
        .arg(directory.0.join("target-codex-backends"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert!(
        execution.status.success(),
        "generated numeric corpus did not compile and execute against runtime {RUNTIME_REVISION}: {}",
        String::from_utf8_lossy(&execution.stderr)
    );
}

/// TC-001
/// FR-001-AC-5
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

/// TC-003
/// FR-001-AC-4
#[test]
fn tc_003_unsupported_expression_and_root_map_to_declared_terminal_states() {
    let integer =
        IntegerType::new(IntegerDomain::Signed, -10, 10, OverflowPolicy::Saturate).unwrap();
    let environment = DeclarationEnvironment::new(requirement(), vec![], vec![], vec![]).unwrap();
    let addition_span = span(30, 32);
    let addition = Expression::new(
        ExpressionKind::Numeric {
            operator: NumericOperator::Add,
            left: Box::new(integer_literal(1, &integer, 30)),
            right: Box::new(integer_literal(1, &integer, 31)),
        },
        addition_span,
    );
    let first_unsupported_span = span(35, 36);
    let negation = Expression::new(
        ExpressionKind::NumericNegate {
            operand: Box::new(integer_literal(1, &integer, 35)),
        },
        first_unsupported_span.clone(),
    );
    let expression = boolean_op(
        BooleanOperator::TotalAnd,
        comparison(
            ComparisonOperator::Equal,
            addition,
            integer_literal(2, &integer, 32),
            29,
        ),
        comparison(
            ComparisonOperator::Equal,
            negation,
            integer_literal(-1, &integer, 36),
            34,
        ),
        29,
    );
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    assert!(typed.obligations().is_empty());
    let clause = ClauseId::new("unsupported").unwrap();
    let diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
    })
    .unwrap_err()[0];
    assert_eq!(diagnostic.code, GenerationErrorCode::UnsupportedExpression);
    assert_eq!(
        diagnostic.terminal_state,
        GenerationTerminalState::Unsupported
    );
    assert_eq!(
        diagnostic.source_span.as_ref(),
        Some(&first_unsupported_span),
        "the first unsupported node in authored preorder must win"
    );
    let encoded = serde_json::to_string(diagnostic).unwrap();
    let decoded: GenerationDiagnostic = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        &decoded, diagnostic,
        "the exact IR span must survive the API wire form"
    );
    let mut legacy_value = serde_json::to_value(diagnostic).unwrap();
    legacy_value.as_object_mut().unwrap().remove("sourceSpan");
    let legacy_decoded: GenerationDiagnostic = serde_json::from_value(legacy_value).unwrap();
    let mut expected_legacy = diagnostic.clone();
    expected_legacy.source_span = None;
    assert_eq!(
        legacy_decoded, expected_legacy,
        "diagnostics serialized before sourceSpan existed must remain readable"
    );

    let root_span = span(33, 34);
    let integer_root = Expression::new(
        ExpressionKind::IntegerLiteral {
            value: 1,
            value_type: integer.clone(),
        },
        root_span.clone(),
    );
    let typed_root = environment
        .check_expression(&integer_root, &ValueType::integer(integer), &pre(), false)
        .unwrap();
    let root_clause = ClauseId::new("wrong-root").unwrap();
    let root_diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &root_clause,
        expression: &typed_root,
    })
    .unwrap_err()[0];
    assert_eq!(root_diagnostic.code, GenerationErrorCode::NonBooleanRoot);
    assert_eq!(
        root_diagnostic.terminal_state,
        GenerationTerminalState::InvalidInput
    );
    assert_eq!(root_diagnostic.source_span.as_ref(), Some(&root_span));

    assert_eq!(
        GenerationErrorCode::InvalidGeneratedSyntax.terminal_state(),
        GenerationTerminalState::Inconclusive
    );
    assert_eq!(
        GenerationErrorCode::SerializationFailed.terminal_state(),
        GenerationTerminalState::Inconclusive
    );
}

/// TC-003
/// FR-001-AC-4
#[test]
fn tc_003_unsupported_dependency_reports_the_first_reference_span() {
    let rational = RationalType::new(-10, 10, 10).unwrap();
    let environment = DeclarationEnvironment::new(
        requirement(),
        vec![],
        vec![ValueDeclaration::new(
            name("ratio"),
            ValueDeclarationKind::Input,
            ValueType::rational(rational.clone()),
            span(40, 41),
        )],
        vec![],
    )
    .unwrap();
    let reference_span = span(41, 42);
    let reference = Expression::new(
        ExpressionKind::ValueReference {
            name: name("ratio"),
            observation: StateObservation::Current,
        },
        reference_span.clone(),
    );
    let literal = Expression::new(
        ExpressionKind::RationalLiteral {
            numerator: 1,
            denominator: 2,
            value_type: rational,
        },
        span(42, 43),
    );
    let expression = comparison(ComparisonOperator::Equal, reference, literal, 40);
    let typed = environment
        .check_expression(&expression, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let clause = ClauseId::new("unsupported-rational-dependency").unwrap();
    let diagnostic = &generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
    })
    .unwrap_err()[0];
    assert_eq!(diagnostic.code, GenerationErrorCode::UnsupportedDependency);
    assert_eq!(diagnostic.source_span.as_ref(), Some(&reference_span));
}

/// TC-003.
#[test]
fn tc_003_dependency_normalization_is_injective_and_artifact_names_are_bounded() {
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
    })
    .unwrap();
    assert!(bundle.rust.contents.contains("enabled_2dflag_current"));
    assert!(bundle.rust.contents.contains("enabled_5fflag_current"));

    let literal_environment = boolean_environment(&[]);
    let literal = boolean(true, 42);
    let typed_literal = literal_environment
        .check_expression(&literal, &ValueType::Boolean, &pre(), true)
        .unwrap();
    let long_clause = ClauseId::new("x".repeat(400)).unwrap();
    let long_bundle = generate_boolean_oracle(&OracleRequest {
        requirement: literal_environment.owner(),
        clause: &long_clause,
        expression: &typed_literal,
    })
    .unwrap();
    for path in [&long_bundle.rust.path, &long_bundle.source_map.path] {
        assert!(path.rsplit('/').next().unwrap().len() <= 255, "{path}");
    }
}

/// TC-003
/// FR-001-AC-4
#[test]
fn tc_023_native_proven_numeric_obligations_render_without_assumptions() {
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
    let bundle = generate_boolean_oracle(&OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed,
    })
    .expect("native-proven nonzero divisor and checked range may render");
    assert!(bundle.rust.contents.contains('/'));
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
