//! This repository's own generation producer (FR-006).
//!
//! It runs the bounded generation corpus through the public API and publishes
//! `codegen.generation-conformance/v1` on stdout: one row per case, each row
//! carrying the outcome, the Interface-001 terminal state the case reached, the
//! diagnostic code it produced when it produced one, and the number of declared
//! checks that actually held.
//!
//! Three things this file deliberately is not.
//!
//! It is not a judge of the crate. It states what the generator did. A case that
//! was expected to be rejected and was rejected is a `pass`, because the
//! rejection is the behaviour under test; a case that was expected to be
//! rejected and generated an artifact anyway is a `fail`.
//!
//! It is not a second copy of the test suite. The integration tests assert
//! properties of individual bundles in detail. This walks the corpus and emits a
//! machine-readable census, so that something downstream can attest to it
//! without reading a transcript. A producer whose only consumer is a human is
//! not a producer.
//!
//! It is not a verdict on the whole repository. `cargo test`, `cargo clippy`,
//! `cargo deny`, the MSRV build and the specification gates each report their
//! own fact. This one reports generation conformance and nothing else.
//!
//! Every row carries `checksDischarged` and `floor`. A row that holds every
//! check it ran but ran fewer than its declared floor is `vacuous`, not `pass`:
//! a case that simplified away is not a case that held. That distinction is what
//! stops a corpus from going green by getting smaller.

use std::fmt::Write as _;

use quire_contract_codegen::{
    generate_boolean_oracle, generate_i64_strategy, generate_tristate_harness,
    GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState, HarnessErrorCode,
    HarnessRequest, ManifestContext, OracleArtifactBundle, OracleRequest, SourceRegion,
    StrategyCampaign, StrategyConstraint, StrategyErrorCode, StrategyRequest,
    IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, OverflowPolicy,
    PackageId, RequirementId, RequirementRef, RequirementRevision, SourceDocumentId,
    SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation, SymbolName,
    TypedExpression, ValueDeclaration, ValueDeclarationKind, ValueType,
};
use serde::Serialize;
use sha2::{Digest as _, Sha256};

/// The protocol this producer publishes. Named, so a consumer that transcribes
/// it can refuse anything else rather than guess.
const PROTOCOL: &str = "codegen.generation-conformance/v1";

/// One corpus case's report.
///
/// `outcome` uses the shared producer vocabulary the adapter enumerates. The
/// domain's own richer answer is carried alongside it in `terminalState` and
/// `diagnosticCode` rather than collapsed into the outcome, because
/// `unsupported` and `invalid-input` are different facts about a rejection and
/// this repository is required to keep them apart.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    protocol: &'static str,
    symbol: String,
    outcome: &'static str,
    /// The Interface-001 terminal state the generator actually reached.
    terminal_state: Option<GenerationTerminalState>,
    expected_terminal_state: Option<GenerationTerminalState>,
    diagnostic_code: Option<String>,
    expected_diagnostic_code: Option<&'static str>,
    /// How many of this case's declared checks held.
    checks_discharged: usize,
    /// How many had to hold for the case to be a pass.
    floor: usize,
    detail: Vec<String>,
    trace_ids: Vec<&'static str>,
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}

// ---------------------------------------------------------------------------
// Corpus construction
// ---------------------------------------------------------------------------

fn requirement(id: &str, revision: u64) -> RequirementRef {
    RequirementRef::new(
        PackageId::new("agent-ix/codegen-conformance").unwrap(),
        RequirementId::new(id).unwrap(),
        RequirementRevision::new(revision).unwrap(),
    )
}

fn span(start: u64, end: u64) -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("codegen-conformance").unwrap(),
        SourceRevision::new(1).unwrap(),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, start as u32 + 1, start).unwrap(),
        SourceLocation::new(source, 1, end as u32 + 1, end).unwrap(),
    )
    .unwrap()
}

const ORACLE_REFS: [&str; 1] = ["FR-001"];
const HARNESS_REFS: [&str; 1] = ["FR-002"];

fn manifest(requirement_refs: &'static [&'static str]) -> ManifestContext<'static> {
    ManifestContext {
        candidate_revision: IR_CANDIDATE_REVISION,
        contribution_method: "generated",
        reviewers: &["@codegen-conformance-reviewer"],
        result_status: "conclusive",
        result_summary: "bounded generation corpus case",
        requirement_refs,
    }
}

fn boolean_environment(
    owner: RequirementRef,
    names: &[(&str, ValueDeclarationKind)],
) -> DeclarationEnvironment {
    DeclarationEnvironment::new(
        owner,
        vec![],
        names
            .iter()
            .enumerate()
            .map(|(index, (value, kind))| {
                ValueDeclaration::new(
                    SymbolName::new(*value).unwrap(),
                    *kind,
                    ValueType::Boolean,
                    span(index as u64, index as u64 + 1),
                )
            })
            .collect(),
        vec![],
    )
    .unwrap()
}

fn bounded_integer() -> IntegerType {
    IntegerType::new(IntegerDomain::Signed, -10, 10, OverflowPolicy::Reject).unwrap()
}

fn reference(name: &str, observation: StateObservation, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::ValueReference {
            name: SymbolName::new(name).unwrap(),
            observation,
        },
        span(at, at + 1),
    )
}

fn boolean_literal(value: bool, at: u64) -> Expression {
    Expression::new(ExpressionKind::BooleanLiteral { value }, span(at, at + 1))
}

fn integer_literal(value: i64, at: u64) -> Expression {
    Expression::new(
        ExpressionKind::IntegerLiteral {
            value,
            value_type: bounded_integer(),
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

fn pre() -> ExecutionPoint {
    ExecutionPoint::Pre {
        operation: AnchorName::new("generate").unwrap(),
    }
}

/// A post-state observation is only well typed at a handler, so a postcondition
/// is checked at one. Using the pre-state anchor for both clauses is how a
/// corpus quietly stops covering the post-state half of a harness.
fn handler() -> ExecutionPoint {
    ExecutionPoint::Handler {
        name: AnchorName::new("generate").unwrap(),
    }
}

fn typed_at(
    environment: &DeclarationEnvironment,
    expression: &Expression,
    value_type: &ValueType,
    execution_point: &ExecutionPoint,
    boolean_context: bool,
) -> Result<TypedExpression, String> {
    environment
        .check_expression(expression, value_type, execution_point, boolean_context)
        .map_err(|error| {
            let text = format!("{error:?}").replace('\n', " ");
            text.chars().take(200).collect()
        })
}

fn typed(
    environment: &DeclarationEnvironment,
    expression: &Expression,
    value_type: &ValueType,
    boolean_context: bool,
) -> Result<TypedExpression, String> {
    typed_at(environment, expression, value_type, &pre(), boolean_context)
}

// ---------------------------------------------------------------------------
// Case accumulation
// ---------------------------------------------------------------------------

/// A case's accumulated result.
struct Case {
    symbol: String,
    checks: Vec<String>,
    floor: usize,
    terminal_state: Option<GenerationTerminalState>,
    expected_terminal_state: Option<GenerationTerminalState>,
    diagnostic_code: Option<String>,
    expected_diagnostic_code: Option<&'static str>,
    trace_ids: Vec<&'static str>,
    failures: Vec<String>,
}

impl Case {
    fn new(symbol: &str, floor: usize, trace_ids: Vec<&'static str>) -> Self {
        Self {
            symbol: symbol.to_owned(),
            checks: Vec::new(),
            floor,
            terminal_state: None,
            expected_terminal_state: None,
            diagnostic_code: None,
            expected_diagnostic_code: None,
            trace_ids,
            failures: Vec::new(),
        }
    }

    fn check(&mut self, name: &str, held: bool) {
        if held {
            self.checks.push(name.to_owned());
        } else {
            self.failures.push(name.to_owned());
        }
    }

    fn into_row(self) -> Row {
        // The order of these three arms is the whole point of the type.
        //
        // A declared check that did not hold is a failure, whatever else is
        // true. A case that held every check it ran but ran fewer than its
        // declared floor is vacuous — it did not fail, and it did not
        // demonstrate what it claims to demonstrate either. Only a case that met
        // its floor with no failures is a pass.
        let discharged = self.checks.len();
        let outcome = if !self.failures.is_empty() {
            "fail"
        } else if discharged < self.floor {
            "vacuous"
        } else {
            "pass"
        };
        let mut detail = self.checks;
        detail.extend(self.failures.iter().map(|item| format!("FAILED: {item}")));
        Row {
            protocol: PROTOCOL,
            symbol: self.symbol,
            outcome,
            terminal_state: self.terminal_state,
            expected_terminal_state: self.expected_terminal_state,
            diagnostic_code: self.diagnostic_code,
            expected_diagnostic_code: self.expected_diagnostic_code,
            checks_discharged: discharged,
            floor: self.floor,
            detail,
            trace_ids: self.trace_ids,
        }
    }
}

/// Record a rejection that returned the oracle slice's diagnostic list.
///
/// Two checks, and the second is the one that matters: the diagnostic's own
/// `terminal_state` field must be the state its `code` declares. A diagnostic
/// that carries a code from one category and a state from another is how an
/// unsupported input starts reading as an invalid one.
fn record_oracle_rejection(
    case: &mut Case,
    outcome: Result<OracleArtifactBundle, Vec<GenerationDiagnostic>>,
) {
    match outcome {
        Ok(_) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            case.check("the rejected input was rejected", false);
        }
        Err(diagnostics) => {
            let Some(diagnostic) = diagnostics.first() else {
                case.check("the rejection carries a diagnostic", false);
                return;
            };
            let code = serde_json::to_value(diagnostic.code)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default();
            case.terminal_state = Some(diagnostic.terminal_state);
            case.diagnostic_code = Some(code.clone());
            case.check(
                "the rejection carries the declared diagnostic code",
                Some(code.as_str()) == case.expected_diagnostic_code,
            );
            case.check(
                "the diagnostic's terminal state is the one its code declares",
                diagnostic.terminal_state == diagnostic.code.terminal_state()
                    && Some(diagnostic.terminal_state) == case.expected_terminal_state,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------------

/// TC-001: a supported Boolean clause lowers to a deterministic, traceable,
/// SPDX-identified oracle whose source map parses as the declared region list.
fn oracle_generated() -> Case {
    let mut case = Case::new(
        "generation::boolean-oracle",
        6,
        vec![
            "FR-001",
            "FR-001-AC-1",
            "FR-001-AC-3",
            "NFR-001-AC-1",
            "NFR-002-AC-1",
            "NFR-002-AC-2",
            "TC-001",
        ],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::Generated);
    let owner = requirement("FR-001", 7);
    let environment = boolean_environment(owner, &[("enabled", ValueDeclarationKind::Input)]);
    let expression = boolean_op(
        BooleanOperator::Implication,
        reference("enabled", StateObservation::Current, 3),
        boolean_literal(false, 4),
        3,
    );
    let typed_expression = match typed(&environment, &expression, &ValueType::Boolean, true) {
        Ok(value) => value,
        Err(reason) => {
            case.check(&format!("the clause types: {reason}"), false);
            return case;
        }
    };
    let clause = ClauseId::new("clause-main").unwrap();
    let request = OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed_expression,
        manifest: manifest(&ORACLE_REFS),
    };

    match generate_boolean_oracle(&request) {
        Ok(first) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            let second = generate_boolean_oracle(&request);
            case.check(
                "regeneration is byte-identical",
                second
                    .as_ref()
                    .map(|value| value == &first)
                    .unwrap_or(false),
            );
            case.check(
                "generated Rust carries the dual-licence SPDX identity",
                first
                    .rust
                    .contents
                    .contains("// SPDX-License-Identifier: MIT OR Apache-2.0\n"),
            );
            case.check(
                "generated Rust names its requirement and revision",
                first.rust.contents.contains("FR-001@7"),
            );
            case.check(
                "generated Rust names its clause",
                first.rust.contents.contains("clause-main"),
            );
            case.check(
                "the source map parses as the declared region list",
                serde_json::from_str::<Vec<SourceRegion>>(&first.source_map.contents)
                    .map(|regions| !regions.is_empty())
                    .unwrap_or(false),
            );
            case.check(
                "each artifact's recorded digest is the digest of that artifact",
                first.rust.sha256 == sha256(first.rust.contents.as_bytes())
                    && first.source_map.sha256 == sha256(first.source_map.contents.as_bytes())
                    && first.manifest.sha256 == sha256(first.manifest.contents.as_bytes()),
            );
        }
        Err(diagnostics) => {
            case.terminal_state = diagnostics.first().map(|item| item.terminal_state);
            case.diagnostic_code = diagnostics.first().map(|item| format!("{:?}", item.code));
            case.check(
                &format!("a supported clause generated: {diagnostics:?}"),
                false,
            );
        }
    }
    case
}

/// TC-004: a typed pre/postcondition pair lowers to a tri-state harness with a
/// proptest adapter.
fn harness_generated() -> Case {
    let mut case = Case::new(
        "generation::tristate-harness",
        4,
        vec!["FR-002", "FR-002-AC-1", "NFR-002-AC-2", "TC-004"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::Generated);
    let owner = requirement("FR-002", 1);
    let environment = boolean_environment(
        owner,
        &[
            ("enabled", ValueDeclarationKind::Input),
            ("state", ValueDeclarationKind::State),
        ],
    );
    let precondition_expression = boolean_op(
        BooleanOperator::ShortCircuitAnd,
        reference("enabled", StateObservation::Current, 2),
        reference("state", StateObservation::Pre, 3),
        2,
    );
    let postcondition_expression = reference("state", StateObservation::Post, 4);
    let precondition = typed(
        &environment,
        &precondition_expression,
        &ValueType::Boolean,
        true,
    );
    let postcondition = typed_at(
        &environment,
        &postcondition_expression,
        &ValueType::Boolean,
        &handler(),
        true,
    );
    let (precondition, postcondition) = match (precondition, postcondition) {
        (Ok(first), Ok(second)) => (first, second),
        (first, second) => {
            case.check(&format!("the clauses type: {first:?} / {second:?}"), false);
            return case;
        }
    };
    let precondition_clause = ClauseId::new("clause-pre").unwrap();
    let postcondition_clause = ClauseId::new("clause-post").unwrap();
    let request = HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &precondition_clause,
        postcondition_clause: &postcondition_clause,
        precondition: &precondition,
        postcondition: &postcondition,
        execution_point: "generate",
        manifest: manifest(&HARNESS_REFS),
    };
    match generate_tristate_harness(&request) {
        Ok(first) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            let second = generate_tristate_harness(&request);
            case.check(
                "regeneration is byte-identical",
                second
                    .as_ref()
                    .map(|value| value == &first)
                    .unwrap_or(false),
            );
            case.check(
                "generated Rust carries the dual-licence SPDX identity",
                first
                    .rust
                    .contents
                    .contains("// SPDX-License-Identifier: MIT OR Apache-2.0\n"),
            );
            case.check(
                "the harness exposes a proptest adapter",
                first.rust.contents.contains("_proptest"),
            );
            case.check(
                "each artifact's recorded digest is the digest of that artifact",
                first.rust.sha256 == sha256(first.rust.contents.as_bytes())
                    && first.manifest.sha256 == sha256(first.manifest.contents.as_bytes()),
            );
        }
        Err(diagnostics) => {
            case.terminal_state = diagnostics.first().map(|item| item.terminal_state);
            case.diagnostic_code = diagnostics.first().map(|item| format!("{:?}", item.code));
            case.check(
                &format!("a supported clause pair generated: {diagnostics:?}"),
                false,
            );
        }
    }
    case
}

/// TC-004: a bounded inclusive range lowers to a shaped proptest strategy.
fn strategy_generated() -> Case {
    let mut case = Case::new(
        "generation::i64-strategy",
        3,
        vec!["FR-002", "FR-002-AC-2", "NFR-002-AC-2", "TC-004"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::Generated);
    let owner = requirement("FR-002", 1);
    let request = StrategyRequest {
        requirement: &owner,
        strategy_id: "bounded-range",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: -8,
            maximum: 8,
        },
        campaign: StrategyCampaign::Boundary,
        manifest: manifest(&HARNESS_REFS),
    };
    match generate_i64_strategy(&request) {
        Ok(first) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            let second = generate_i64_strategy(&request);
            case.check(
                "regeneration is byte-identical",
                second
                    .as_ref()
                    .map(|value| value == &first)
                    .unwrap_or(false),
            );
            case.check(
                "generated Rust carries the dual-licence SPDX identity",
                first
                    .rust
                    .contents
                    .contains("// SPDX-License-Identifier: MIT OR Apache-2.0\n"),
            );
            case.check(
                "each artifact's recorded digest is the digest of that artifact",
                first.rust.sha256 == sha256(first.rust.contents.as_bytes())
                    && first.manifest.sha256 == sha256(first.manifest.contents.as_bytes()),
            );
        }
        Err(diagnostic) => {
            case.terminal_state = Some(diagnostic.terminal_state);
            case.diagnostic_code = Some(format!("{:?}", diagnostic.code));
            case.check(
                &format!("a bounded range generated: {}", diagnostic.message),
                false,
            );
        }
    }
    case
}

/// TC-003: a non-Boolean clause root is invalid input, not an unsupported
/// construct. The two states have different meanings for a caller and this case
/// pins which one this input reaches.
fn oracle_rejects_non_boolean_root() -> Case {
    let mut case = Case::new(
        "rejection::non-boolean-root",
        2,
        vec!["FR-001", "FR-001-AC-4", "NFR-002-AC-3", "TC-003"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("non_boolean_root");
    let owner = requirement("FR-001", 7);
    let environment = DeclarationEnvironment::new(owner, vec![], vec![], vec![]).unwrap();
    let root = integer_literal(1, 33);
    let integer = bounded_integer();
    let typed_root = match typed(&environment, &root, &ValueType::integer(integer), false) {
        Ok(value) => value,
        Err(reason) => {
            case.check(&format!("the clause types: {reason}"), false);
            return case;
        }
    };
    let clause = ClauseId::new("wrong-root").unwrap();
    let request = OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed_root,
        manifest: manifest(&ORACLE_REFS),
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-003: a comparison is outside the bounded first slice, so it is
/// `unsupported` — a construct the generator declines to lower rather than one
/// it rejects as wrong.
fn oracle_rejects_unsupported_expression() -> Case {
    let mut case = Case::new(
        "rejection::unsupported-expression",
        2,
        vec![
            "FR-001",
            "FR-001-AC-4",
            "FR-003-AC-3",
            "NFR-002-AC-3",
            "TC-003",
        ],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::Unsupported);
    case.expected_diagnostic_code = Some("unsupported_expression");
    let owner = requirement("FR-001", 7);
    let environment = DeclarationEnvironment::new(owner, vec![], vec![], vec![]).unwrap();
    let comparison = Expression::new(
        ExpressionKind::Compare {
            operator: ComparisonOperator::Equal,
            left: Box::new(integer_literal(1, 30)),
            right: Box::new(integer_literal(1, 31)),
        },
        span(30, 32),
    );
    let typed_expression = match typed(&environment, &comparison, &ValueType::Boolean, true) {
        Ok(value) => value,
        Err(reason) => {
            case.check(&format!("the clause types: {reason}"), false);
            return case;
        }
    };
    let clause = ClauseId::new("unsupported").unwrap();
    let request = OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed_expression,
        manifest: manifest(&ORACLE_REFS),
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-003: an invalid caller-supplied manifest context is rejected before any
/// artifact exists, so provenance cannot be omitted by supplying nothing.
fn oracle_rejects_invalid_manifest_context() -> Case {
    let mut case = Case::new(
        "rejection::invalid-manifest-context",
        2,
        vec!["FR-001-AC-4", "NFR-002-AC-1", "NFR-002-AC-3", "TC-003"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("invalid_manifest_context");
    let owner = requirement("FR-001", 7);
    let environment = boolean_environment(owner, &[("enabled", ValueDeclarationKind::Input)]);
    let expression = reference("enabled", StateObservation::Current, 3);
    let typed_expression = match typed(&environment, &expression, &ValueType::Boolean, true) {
        Ok(value) => value,
        Err(reason) => {
            case.check(&format!("the clause types: {reason}"), false);
            return case;
        }
    };
    let clause = ClauseId::new("clause-main").unwrap();
    let request = OracleRequest {
        requirement: environment.owner(),
        clause: &clause,
        expression: &typed_expression,
        manifest: ManifestContext {
            candidate_revision: "not-a-revision",
            contribution_method: "unspecified",
            reviewers: &[],
            result_status: "complete",
            result_summary: "",
            requirement_refs: &[],
        },
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-004: a harness whose two clause identities collide is rejected, because a
/// harness that cannot attribute a failure to a clause is worse than none.
fn harness_rejects_duplicate_clause_identity() -> Case {
    let mut case = Case::new(
        "rejection::duplicate-clause-identity",
        2,
        vec!["FR-002", "FR-002-AC-4", "NFR-002-AC-3", "TC-004"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("DuplicateClauseIdentity");
    let owner = requirement("FR-002", 1);
    let environment = boolean_environment(
        owner,
        &[
            ("enabled", ValueDeclarationKind::Input),
            ("state", ValueDeclarationKind::State),
        ],
    );
    let precondition = typed(
        &environment,
        &reference("enabled", StateObservation::Current, 2),
        &ValueType::Boolean,
        true,
    );
    let postcondition = typed_at(
        &environment,
        &reference("state", StateObservation::Post, 3),
        &ValueType::Boolean,
        &handler(),
        true,
    );
    let (precondition, postcondition) = match (precondition, postcondition) {
        (Ok(first), Ok(second)) => (first, second),
        (first, second) => {
            case.check(&format!("the clauses type: {first:?} / {second:?}"), false);
            return case;
        }
    };
    let clause = ClauseId::new("clause-same").unwrap();
    let request = HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &clause,
        postcondition_clause: &clause,
        precondition: &precondition,
        postcondition: &postcondition,
        execution_point: "generate",
        manifest: manifest(&HARNESS_REFS),
    };
    match generate_tristate_harness(&request) {
        Ok(_) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            case.check("the colliding clause identities were rejected", false);
        }
        Err(diagnostics) => {
            let first = diagnostics.first();
            case.terminal_state = first.map(|item| item.terminal_state);
            case.diagnostic_code = first.map(|item| format!("{:?}", item.code));
            case.check(
                "the rejection carries the declared diagnostic code",
                first.map(|item| item.code) == Some(HarnessErrorCode::DuplicateClauseIdentity),
            );
            case.check(
                "the rejection carries the declared terminal state",
                first.map(|item| item.terminal_state) == case.expected_terminal_state,
            );
        }
    }
    case
}

/// TC-004: a reversed range is invalid input.
fn strategy_rejects_invalid_range() -> Case {
    let mut case = Case::new(
        "rejection::invalid-range",
        2,
        vec!["FR-002", "FR-002-AC-4", "NFR-002-AC-3", "TC-004"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("InvalidRange");
    let owner = requirement("FR-002", 1);
    let request = StrategyRequest {
        requirement: &owner,
        strategy_id: "reversed-range",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 8,
            maximum: -8,
        },
        campaign: StrategyCampaign::Broad,
        manifest: manifest(&HARNESS_REFS),
    };
    match generate_i64_strategy(&request) {
        Ok(_) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            case.check("the reversed range was rejected", false);
        }
        Err(diagnostic) => {
            case.terminal_state = Some(diagnostic.terminal_state);
            case.diagnostic_code = Some(format!("{:?}", diagnostic.code));
            case.check(
                "the rejection carries the declared diagnostic code",
                diagnostic.code == StrategyErrorCode::InvalidRange,
            );
            case.check(
                "the rejection carries the declared terminal state",
                Some(diagnostic.terminal_state) == case.expected_terminal_state,
            );
        }
    }
    case
}

/// The row that keeps the corpus honest about its own reach.
///
/// A corpus can go green by getting smaller. This counts what the cases above
/// actually produced — distinct diagnostic codes, and both non-success terminal
/// states this bounded slice can reach — against a floor. A corpus that stops
/// producing one of them falls below the floor and the row reads `vacuous`,
/// which is not `pass`.
fn diagnostic_census(rows: &[Row]) -> Case {
    let mut case = Case::new(
        "census::diagnostic-vocabulary",
        9,
        vec!["FR-001-AC-4", "NFR-002-AC-3", "TC-003", "TC-006"],
    );
    // Only a case that passed demonstrates anything. A case that failed and
    // produced a diagnostic on the way out produced it by accident, and counting
    // it would let a broken corpus inflate its own census — which is the exact
    // shape of a gate that goes green by getting worse.
    let mut produced: Vec<&str> = rows
        .iter()
        .filter(|row| row.outcome == "pass")
        .filter_map(|row| row.diagnostic_code.as_deref())
        .collect();
    produced.sort_unstable();
    produced.dedup();
    for code in &produced {
        case.check(&format!("the corpus produced diagnostic {code}"), true);
    }
    for state in [
        GenerationTerminalState::InvalidInput,
        GenerationTerminalState::Unsupported,
        GenerationTerminalState::Generated,
    ] {
        case.check(
            &format!("the corpus reached terminal state {state:?}"),
            rows.iter()
                .any(|row| row.outcome == "pass" && row.terminal_state == Some(state)),
        );
    }
    // Every declared code must still map to the terminal state it declares.
    // Cheap, and it is exactly the thing that would silently change if a
    // category were re-pointed at a different state.
    let declared = [
        (
            GenerationErrorCode::NonBooleanRoot,
            GenerationTerminalState::InvalidInput,
        ),
        (
            GenerationErrorCode::NameCollision,
            GenerationTerminalState::InvalidInput,
        ),
        (
            GenerationErrorCode::InvalidManifestContext,
            GenerationTerminalState::InvalidInput,
        ),
        (
            GenerationErrorCode::UnsupportedExpression,
            GenerationTerminalState::Unsupported,
        ),
        (
            GenerationErrorCode::UnsupportedDependency,
            GenerationTerminalState::Unsupported,
        ),
        (
            GenerationErrorCode::UnsupportedObligations,
            GenerationTerminalState::Unsupported,
        ),
        (
            GenerationErrorCode::ResourceLimitExceeded,
            GenerationTerminalState::Unsupported,
        ),
        (
            GenerationErrorCode::InvalidGeneratedSyntax,
            GenerationTerminalState::Inconclusive,
        ),
        (
            GenerationErrorCode::SerializationFailed,
            GenerationTerminalState::Inconclusive,
        ),
    ];
    case.check(
        "every declared generation error code keeps its declared terminal state",
        declared
            .iter()
            .all(|(code, state)| code.terminal_state() == *state),
    );
    case
}

fn main() {
    let mut rows: Vec<Row> = vec![
        oracle_generated().into_row(),
        harness_generated().into_row(),
        strategy_generated().into_row(),
        oracle_rejects_non_boolean_root().into_row(),
        oracle_rejects_unsupported_expression().into_row(),
        oracle_rejects_invalid_manifest_context().into_row(),
        harness_rejects_duplicate_clause_identity().into_row(),
        strategy_rejects_invalid_range().into_row(),
    ];
    rows.push(diagnostic_census(&rows).into_row());

    for row in &rows {
        println!("{}", serde_json::to_string(row).expect("a row serializes"));
    }
}
