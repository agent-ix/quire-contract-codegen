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
//!
//! The exit code carries that same census: `0` when every row is a `pass`, `1`
//! when any row is a `fail`, and `2` when no row failed but one is `vacuous`,
//! which is inconclusive rather than either verdict. The rows themselves are
//! unchanged and still go to stdout; the failing and vacuous ones are named on
//! stderr. This judges generation conformance and still nothing else.
//!
//! `ci` reaches this producer twice, and only one of them is a judgement. The
//! `conformance` target runs it standalone, and nothing else looks at that
//! invocation, so its exit code is the whole gate. `assurance-inputs` runs it
//! redirected into the chain's intake and deliberately tolerates status 1 and
//! 2, because the chain classifies the rows itself and cannot report a defect
//! whose bytes never reached it.

use std::fmt::Write as _;
use std::process;

use quire_contract_codegen::{
    generate_boolean_oracle, generate_i64_strategy, generate_tristate_harness, AttestationContext,
    AttestationResult, GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
    HarnessErrorCode, HarnessRequest, OracleArtifactBundle, OracleRequest, ProofAttestationBody,
    SourceRegion, StrategyCampaign, StrategyConstraint, StrategyErrorCode, StrategyRequest,
    IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES,
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

/// The record digest this corpus's attestations bind to.
///
/// A bounded corpus is not a change under review, so no change-assurance record is
/// sealed for it and there is no digest to name. The all-zero digest is used
/// because it is the one 64-hexadecimal string no sealed record can have: a
/// plausible-looking value here would be a false binding, and this one cannot be
/// mistaken for a real one.
const CORPUS_RECORD_DIGEST: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

fn attestation() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: CORPUS_RECORD_DIGEST,
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

/// Reads one emitted attestation and reports whether it is a well-formed
/// `ProofAttestationV1` body bound to the context the caller supplied.
///
/// This is a shape check and deliberately not a conformance check against Quoin's
/// packaged schema: the producer runs before the chain and shells out to nothing.
/// `tests/oracle_generation.rs` seals these bytes through the real
/// `quoin change-assurance seal-attestation` and validates the sealed result
/// against the packaged schema.
fn attestation_is_a_shared_body(contents: &str) -> bool {
    let Ok(body) = serde_json::from_str::<ProofAttestationBody>(contents) else {
        return false;
    };
    body.schema_version == 1
        && body.record_type == "proof_attestation"
        && body.result == AttestationResult::Passed
        && body.record_digest == CORPUS_RECORD_DIGEST
        && body.candidate_revision == IR_CANDIDATE_REVISION
        && body.proof_id.starts_with("PROOF-codegen-")
        && body.attestation_id.starts_with(&body.proof_id)
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
        7,
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
        attestation: attestation(),
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
                    && first.rust_attestation.sha256
                        == sha256(first.rust_attestation.contents.as_bytes())
                    && first.source_map_attestation.sha256
                        == sha256(first.source_map_attestation.contents.as_bytes()),
            );
            case.check(
                "both generated artifacts carry a shared proof-attestation body",
                attestation_is_a_shared_body(&first.rust_attestation.contents)
                    && attestation_is_a_shared_body(&first.source_map_attestation.contents),
            );
        }
        Err(diagnostics) => {
            case.terminal_state = diagnostics.first().map(|item| item.terminal_state);
            // Serialized, not `Debug`, so this field carries one vocabulary in
            // every arm that publishes it — including this one, which only fires
            // when a supported clause unexpectedly failed and a reader is going
            // through the corpus to find out why.
            case.diagnostic_code = diagnostics
                .first()
                .and_then(|item| serde_json::to_value(item.code).ok())
                .and_then(|value| value.as_str().map(str::to_owned));
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
        5,
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
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 1,
        attestation: attestation(),
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
                    && first.attestation.sha256 == sha256(first.attestation.contents.as_bytes()),
            );
            case.check(
                "the generated artifact carries a shared proof-attestation body",
                attestation_is_a_shared_body(&first.attestation.contents),
            );
        }
        Err(diagnostics) => {
            case.terminal_state = diagnostics.first().map(|item| item.terminal_state);
            // Serialized, not `Debug`, so this field carries one vocabulary in
            // every arm that publishes it — including this one, which only fires
            // when a supported clause unexpectedly failed and a reader is going
            // through the corpus to find out why.
            case.diagnostic_code = diagnostics
                .first()
                .and_then(|item| serde_json::to_value(item.code).ok())
                .and_then(|value| value.as_str().map(str::to_owned));
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
        4,
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
        attestation: attestation(),
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
                    && first.attestation.sha256 == sha256(first.attestation.contents.as_bytes()),
            );
            case.check(
                "the generated artifact carries a shared proof-attestation body",
                attestation_is_a_shared_body(&first.attestation.contents),
            );
        }
        Err(diagnostic) => {
            case.terminal_state = Some(diagnostic.terminal_state);
            // Serialized, not `Debug`, so this field carries one vocabulary
            // whichever arm publishes it.
            case.diagnostic_code = serde_json::to_value(diagnostic.code)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned));
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
        attestation: attestation(),
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-003: numeric negation remains outside the obligation-free comparison
/// slice, so it is `unsupported` — a construct the generator declines to lower
/// rather than one it rejects as wrong.
///
/// Arithmetic used to be named here too, and no longer is: #40 (`583502b`) made
/// integer arithmetic lowerable. That commit shipped no spec change, so
/// `FR-001:48`, `interface-001:114` and FR-003-AC-3 still require arithmetic to
/// refuse. This case follows the code, and the divergence is IR-235's.
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
    let integer =
        IntegerType::new(IntegerDomain::Signed, -10, 10, OverflowPolicy::Saturate).unwrap();
    let literal = |value, start| {
        Expression::new(
            ExpressionKind::IntegerLiteral {
                value,
                value_type: integer.clone(),
            },
            span(start, start + 1),
        )
    };
    // An `ExpressionKind::Numeric { operator: NumericOperator::Add, .. }` used
    // to stand here and no longer refuses: #40 (`583502b`) made integer
    // arithmetic lowerable and, in that same commit, moved
    // `tc_003_unsupported_expression_and_root_map_to_declared_terminal_states`'s
    // expected span off the addition and onto a `NumericNegate`. Negation is
    // the form still outside the slice, so it is the one this entry names.
    let negation = Expression::new(
        ExpressionKind::NumericNegate {
            operand: Box::new(literal(1, 30)),
        },
        span(30, 32),
    );
    let comparison = Expression::new(
        ExpressionKind::Compare {
            operator: ComparisonOperator::Equal,
            left: Box::new(negation),
            right: Box::new(literal(-1, 32)),
        },
        span(30, 33),
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
        attestation: attestation(),
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-003: an invalid caller-supplied attestation binding is rejected before any
/// artifact exists, so a generated artifact cannot arrive bound to nothing.
fn oracle_rejects_invalid_attestation_context() -> Case {
    let mut case = Case::new(
        "rejection::invalid-attestation-context",
        2,
        vec!["FR-001-AC-4", "NFR-002-AC-1", "NFR-002-AC-3", "TC-003"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("invalid_attestation_context");
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
        attestation: AttestationContext {
            record_digest: "not-a-digest",
            candidate_revision: "not-a-revision",
        },
    };
    record_oracle_rejection(&mut case, generate_boolean_oracle(&request));
    case
}

/// TC-004: distinguish colliding clause identities from an oversized generated
/// harness; the size fixture uses distinct identities so it reaches its own guard.
fn harness_rejects_invalid_input(duplicate_clause: bool) -> Case {
    let (symbol, expected_code, expected_state) = if duplicate_clause {
        (
            "rejection::duplicate-clause-identity",
            HarnessErrorCode::DuplicateClauseIdentity,
            GenerationTerminalState::InvalidInput,
        )
    } else {
        (
            "rejection::harness-source-limit",
            HarnessErrorCode::ResourceLimitExceeded,
            GenerationTerminalState::Unsupported,
        )
    };
    let mut case = Case::new(
        symbol,
        3,
        vec!["FR-002", "FR-002-AC-4", "NFR-002-AC-3", "TC-004"],
    );
    case.expected_terminal_state = Some(expected_state);
    case.expected_diagnostic_code = Some(if duplicate_clause {
        "duplicate_clause_identity"
    } else {
        "resource_limit_exceeded"
    });
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
    let other_clause = ClauseId::new("clause-other").unwrap();
    let execution_point = if duplicate_clause {
        "generate".to_owned()
    } else {
        "x".repeat(MAX_GENERATED_SOURCE_BYTES)
    };
    let request = HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &clause,
        postcondition_clause: if duplicate_clause {
            &clause
        } else {
            &other_clause
        },
        precondition: &precondition,
        postcondition: &postcondition,
        execution_point: &execution_point,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 1,
        attestation: attestation(),
    };
    match generate_tristate_harness(&request) {
        Ok(_) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            case.check("the invalid harness input was rejected", false);
        }
        Err(diagnostics) => {
            let first = diagnostics.first();
            case.terminal_state = first.map(|item| item.terminal_state);
            // Serialize rather than `Debug`. `HarnessErrorCode` is
            // `rename_all = "snake_case"` and its doc calls it the stable
            // reason, so the wire identity is `resource_limit_exceeded`; a
            // `Debug` rendering is a different string that Rust does not promise
            // to keep. Publishing it left this protocol field carrying two
            // vocabularies at once — snake_case from the oracle rows, PascalCase
            // from these — and made the check below blind to a `serde(rename)`,
            // which is exactly the declaration-versus-reality drift it exists to
            // catch.
            case.diagnostic_code = first
                .and_then(|item| serde_json::to_value(item.code).ok())
                .and_then(|value| value.as_str().map(str::to_owned));
            case.check(
                "the rejection carries the declared diagnostic code",
                first.map(|item| item.code) == Some(expected_code),
            );
            // The row publishes `expectedDiagnosticCode` for downstream
            // consumers, and until #122 nothing read it: the check above
            // compares the typed `expected_code` declared in the same tuple, so
            // the published string was a second declaration of the same fact
            // that could drift from it silently. Measured: setting it to
            // `"NotTheRealCode"` left the row `pass` with that string in its own
            // output. Assert the published field too, so the row cannot state an
            // expectation it did not test.
            case.check(
                "the published expected diagnostic code is the code produced",
                case.expected_diagnostic_code.is_some()
                    && case.diagnostic_code.as_deref() == case.expected_diagnostic_code,
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
        3,
        vec!["FR-002", "FR-002-AC-4", "NFR-002-AC-3", "TC-004"],
    );
    case.expected_terminal_state = Some(GenerationTerminalState::InvalidInput);
    case.expected_diagnostic_code = Some("invalid_range");
    let owner = requirement("FR-002", 1);
    let request = StrategyRequest {
        requirement: &owner,
        strategy_id: "reversed-range",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 8,
            maximum: -8,
        },
        campaign: StrategyCampaign::Broad,
        attestation: attestation(),
    };
    match generate_i64_strategy(&request) {
        Ok(_) => {
            case.terminal_state = Some(GenerationTerminalState::Generated);
            case.check("the reversed range was rejected", false);
        }
        Err(diagnostic) => {
            case.terminal_state = Some(diagnostic.terminal_state);
            // Serialized, not `Debug`, for the reason given in the harness path.
            case.diagnostic_code = serde_json::to_value(diagnostic.code)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned));
            case.check(
                "the rejection carries the declared diagnostic code",
                diagnostic.code == StrategyErrorCode::InvalidRange,
            );
            // Same defect as the harness path, and the same fix (#122): the
            // check above names the variant inline, so the published
            // `expectedDiagnosticCode` was never read.
            case.check(
                "the published expected diagnostic code is the code produced",
                case.expected_diagnostic_code.is_some()
                    && case.diagnostic_code.as_deref() == case.expected_diagnostic_code,
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
        10,
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
            GenerationErrorCode::InvalidAttestationContext,
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
        oracle_rejects_invalid_attestation_context().into_row(),
        harness_rejects_invalid_input(true).into_row(),
        harness_rejects_invalid_input(false).into_row(),
        strategy_rejects_invalid_range().into_row(),
    ];
    rows.push(diagnostic_census(&rows).into_row());

    // Serialized once, then both published and classified from the same
    // strings. The exit code is derived from the bytes this producer actually
    // emitted, not from a `rows` binding that the exit call happens to be
    // handed -- see `exit_code`.
    let published: Vec<String> = rows
        .iter()
        .map(|row| serde_json::to_string(row).expect("a row serializes"))
        .collect();

    for line in &published {
        println!("{line}");
    }

    // `Makefile:233` lists `conformance` in `ci`, so this producer's own rows
    // have to set its exit code. Printing a `fail` row and returning unit made
    // that entry structurally incapable of failing: `rejection::unsupported-
    // expression` sat red inside a green `make conformance`, and because the
    // assurance chain asserts a zero exit, that one unjudged row suppressed
    // four `shared_assurance` tests at once (#77).
    //
    // The three codes keep the row vocabulary's distinction instead of
    // collapsing it, and they agree with the chain's `ROW_RESULTS`, which maps
    // `fail` to `failed` and `vacuous` to `not_computed`, and with its
    // `RESULT_PRECEDENCE`, where both outrank `passed` because the strongest
    // thing observed is what the run has to be reported as. `vacuous` is not a
    // failure and is not a pass either, so it exits 2 as inconclusive.
    //
    // Only `make conformance` acts on these codes. `make assurance-inputs`
    // redirects this producer into the chain's intake and tolerates 1 and 2 on
    // purpose; see the comment on that target.
    let failed: Vec<&Row> = rows.iter().filter(|row| row.outcome == "fail").collect();
    let vacuous: Vec<&Row> = rows.iter().filter(|row| row.outcome == "vacuous").collect();

    for row in failed.iter().chain(vacuous.iter()) {
        eprintln!(
            "{}: {} ({} of {} declared checks discharged)",
            row.outcome, row.symbol, row.checks_discharged, row.floor
        );
        for item in row
            .detail
            .iter()
            .filter(|item| item.starts_with("FAILED: "))
        {
            eprintln!("  {item}");
        }
    }

    if !failed.is_empty() {
        eprintln!(
            "generation conformance: {} of {} rows failed",
            failed.len(),
            rows.len()
        );
    }
    if !vacuous.is_empty() {
        eprintln!(
            "generation conformance: {} of {} rows are vacuous",
            vacuous.len(),
            rows.len()
        );
    }

    // A single exit path (#125). Three separate `process::exit` calls would
    // let a unit test of the classification prove nothing about the 1 and 2
    // arms actually reaching the process's exit status -- only the 0 arm
    // (falling off the end of `main`) would ever have been end-to-end. With
    // one call, `exit_code`'s return value *is* the process's exit code, so a
    // test of `exit_code` is a test of what `main` exits with, and the
    // separate end-to-end test below (`built_example_binary_...`) exercises
    // this exact statement by running the compiled binary.
    process::exit(exit_code(&published));
}

/// Classify the lines this producer published into its exit code: `0` when
/// every published row is a `pass`, `1` when any is a `fail`, `2` when none
/// failed but one is `vacuous`.
///
/// It reads the published strings rather than the `Row` values because the
/// binding handed to the exit call is not necessarily the one that was
/// printed, and #77 is precisely what that gap costs. Measured during review
/// of this change, against an earlier version that classified `&[Row]`:
/// inserting `rows.retain(|row| row.outcome == "pass")` between the print
/// loop and the exit call left all seven tests in this file green at exit 0
/// while every failing row still went to stdout -- the producer rendered
/// structurally incapable of a non-zero exit, which is #77 verbatim, and the
/// defect this contract exists to prevent. Deriving from the published bytes
/// closes it: whatever reaches stdout is what is classified, so the two
/// cannot disagree. A run that drops rows before publishing is a different
/// defect and is caught elsewhere -- `tests/shared_assurance.rs` requires at
/// least ten rows in the emitted JSONL.
///
/// This is the same discipline `scripts/assurance_chain.py` applies one layer
/// out, deriving every attested result from the producer's own bytes rather
/// than from anything the producer says about itself.
///
/// Precedence is fail, then vacuous, then pass -- a run with both a failing
/// and a vacuous row exits 1, not 2, because a fail is the stronger claim and
/// the row vocabulary is already ordered that way in the comment above (the
/// chain's `RESULT_PRECEDENCE`, where both outrank `passed`). Checking
/// vacuous first would silently invert that.
///
/// A line that is not an object carrying a string `outcome` is a defect in
/// this producer rather than a verdict about the corpus, so it panics instead
/// of falling through to the `0` arm. A silent fall-through there would make
/// a serialization change read as a clean run.
fn exit_code(published: &[String]) -> i32 {
    let outcomes: Vec<String> = published
        .iter()
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .ok()
                .and_then(|value| {
                    value
                        .get("outcome")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| panic!("a published row must carry a string `outcome`: {line}"))
        })
        .collect();

    if outcomes.iter().any(|outcome| outcome == "fail") {
        1
    } else if outcomes.iter().any(|outcome| outcome == "vacuous") {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smallest honest construction of a `Row` for these tests: `exit_code`
    /// only reads `outcome`, so every other field takes an inert placeholder.
    /// This helper lives here, in the test module, rather than becoming a
    /// `Default` impl on `Row` that production code has no other reason to
    /// carry.
    fn row(outcome: &'static str) -> Row {
        Row {
            protocol: PROTOCOL,
            symbol: "test-row".to_owned(),
            outcome,
            terminal_state: None,
            expected_terminal_state: None,
            diagnostic_code: None,
            expected_diagnostic_code: None,
            checks_discharged: 0,
            floor: 0,
            detail: Vec::new(),
            trace_ids: Vec::new(),
        }
    }

    /// One published line, serialized through the same `serde_json::to_string`
    /// `main` publishes with, so these tests classify the shape the producer
    /// actually emits rather than a hand-written approximation of it.
    fn published(outcome: &'static str) -> String {
        serde_json::to_string(&row(outcome)).expect("a row serializes")
    }

    /// A published line whose `outcome` is not a string is a defect in this
    /// producer, not a verdict about the corpus, so it must be loud. Without
    /// this, a serialization change that renamed or retyped the field would
    /// make every row unreadable and the whole run classify as `0`.
    #[test]
    #[should_panic(expected = "must carry a string `outcome`")]
    fn a_line_without_a_string_outcome_panics_rather_than_reading_as_a_pass() {
        let _ = exit_code(&[r#"{"outcome":7}"#.to_owned()]);
    }

    /// The source of this file, for the tripwire below. `include_str!`
    /// resolves relative to this file's own directory, so it embeds these
    /// bytes at compile time rather than reading a path at run time that a
    /// different working directory would move.
    const OWN_SOURCE: &str = include_str!("generation_conformance.rs");

    /// A tripwire and a floor, not a proof -- said plainly because the
    /// distinction is the whole subject of this file.
    ///
    /// Every other test here exercises `exit_code`, and `main` reaching the
    /// process's exit status through it is what makes those tests mean
    /// anything. That link is the one thing they cannot check: replacing
    /// `main`'s single call with `process::exit(0)` leaves every one of them
    /// green, because the real bounded corpus passes today, so the end-to-end
    /// test observes 0 either way and the classifier is simply never
    /// consulted. Measured with this assertion deleted and that mutation in
    /// place: exit 0, 7 of 7 passing. With it restored, the same tree is exit
    /// 101 and this is the only test that fails.
    ///
    /// Nothing available here closes that by behaviour. A corpus row that
    /// deliberately fails would, and does not exist; manufacturing one inside
    /// the producer would be a fault-injection affordance in an evidence
    /// producer, which costs more than it buys. So this reads the text of
    /// this file instead and requires the single exit to be the classifier's
    /// value.
    ///
    /// What it catches: a constant substituted for the call, and a second
    /// exit path added anywhere outside the test module. The second half of
    /// that needed a correction. An earlier version scanned only from `fn
    /// main` to the test module, and review showed the obvious bypass --
    /// `fn bail_out() -> ! { process::exit(0) }` declared *after* `mod
    /// tests`, called from `main` -- left all seven tests green at exit 0
    /// while the classifier was never consulted for any non-empty run. The
    /// scan below therefore covers everything before the test module and
    /// separately refuses a top-level item after it.
    ///
    /// What it does not catch: an early `return` in `main`. A behavioural
    /// check would be better the moment a failing fixture exists.
    ///
    /// The pre-module slice cannot match this test's own expected strings,
    /// because they live inside the module the slice stops at.
    #[test]
    fn main_reaches_the_process_exit_status_only_through_the_classifier() {
        const MODULE: &str = "\n#[cfg(test)]";
        let module_at = OWN_SOURCE
            .find(MODULE)
            .expect("this file declares a test module at the start of a line");
        assert!(
            OWN_SOURCE
                .find("\nfn main() {")
                .is_some_and(|at| at < module_at),
            "fn main must be declared before the test module for the scan below \
             to cover it"
        );
        let before_module = &OWN_SOURCE[..module_at];

        let exits = before_module.matches("process::exit(").count();
        assert_eq!(
            exits, 1,
            "everything outside the test module must reach the process's exit \
             status exactly once, so that a test of `exit_code` is a test of what \
             this producer exits with; found {exits} calls"
        );
        assert!(
            before_module.contains("process::exit(exit_code(&published));"),
            "the single exit must pass the classifier's value. A literal here \
             would leave every test in this module green while the exit code \
             stopped depending on the published rows at all"
        );

        // A top-level item after the test module is outside the slice above,
        // which is exactly where the bypass review found was planted. Items
        // inside the module are indented, so a column-zero `fn` here is one
        // that escaped the scan.
        let after_module = &OWN_SOURCE[module_at + MODULE.len()..];
        assert!(
            !after_module.contains("\nfn "),
            "a top-level `fn` is declared after the test module, where the exit \
             census above cannot see it. Move it before the module so it is \
             scanned"
        );
    }

    #[test]
    fn all_rows_passing_exits_zero() {
        let rows = vec![published("pass"), published("pass")];
        assert_eq!(exit_code(&rows), 0);
    }

    #[test]
    fn one_failing_row_exits_one() {
        let rows = vec![published("pass"), published("fail")];
        assert_eq!(exit_code(&rows), 1);
    }

    #[test]
    fn one_vacuous_row_exits_two() {
        let rows = vec![published("pass"), published("vacuous")];
        assert_eq!(exit_code(&rows), 2);
    }

    /// The boundary the ticket (#125) names explicitly: a run carrying both a
    /// failing row and a vacuous row exits 1, not 2. Checking vacuous before
    /// fail in `exit_code` would flip this test red while leaving the
    /// single-outcome tests above green, which is exactly why this case has
    /// to be asserted on its own rather than assumed from the other two.
    #[test]
    fn failing_and_vacuous_together_exits_one_not_two() {
        let rows = vec![published("pass"), published("fail"), published("vacuous")];
        assert_eq!(exit_code(&rows), 1);
    }

    /// An empty row set exits 0 under the same rule as any other run with no
    /// failing and no vacuous row -- `exit_code` has no special case for "no
    /// rows" and should not grow one: a corpus that produced zero rows is a
    /// different defect (the producer not running at all, or every case being
    /// filtered out upstream) that this function has no information to
    /// detect.
    ///
    /// That is a real gap, and it is closed elsewhere rather than here, so
    /// this test is not blessing it. `tests/shared_assurance.rs` requires at
    /// least ten rows in the emitted JSONL, which is what actually refuses a
    /// corpus that shrank to nothing. The end-to-end test below does not
    /// close it and an earlier version of this comment wrongly said it did:
    /// that test asserts exit 0, and an empty corpus produces exit 0 too.
    #[test]
    fn no_rows_exits_zero() {
        let rows: Vec<String> = Vec::new();
        assert_eq!(exit_code(&rows), 0);
    }

    /// Proves the wire is real: this executes the compiled example binary --
    /// not the classifier, not this test harness -- against the real bounded
    /// corpus and asserts the process's own exit status.
    ///
    /// `cargo test` builds this file's `#[cfg(test)] mod tests` into a
    /// harness binary (`current_exe()` while this test runs), separately from
    /// the plain runnable example binary `make conformance` and
    /// `make assurance-inputs` invoke via `cargo run --example
    /// generation_conformance`. `env!("CARGO_BIN_EXE_<name>")` only resolves
    /// `[[bin]]` targets, not examples, so there is no compile-time constant
    /// for the plain binary's path. Both binaries land as siblings in the
    /// same `<target>/debug/examples/` directory regardless of any
    /// `CARGO_TARGET_DIR` override, so this walks there from the harness
    /// binary's own `current_exe()` rather than reconstructing the path from
    /// `CARGO_MANIFEST_DIR` and an assumed profile name, which would silently
    /// stop tracking a `CARGO_TARGET_DIR` override or a non-debug profile.
    ///
    /// Measured: on a from-scratch `CARGO_TARGET_DIR`, `cargo test --example
    /// generation_conformance` alone builds only the harness binary above,
    /// not the plain one -- the plain binary only appears once something
    /// (`make conformance`, `make assurance-inputs`, or `cargo run --example
    /// generation_conformance`/`cargo build --example generation_conformance`
    /// directly) has built it first.
    ///
    /// The build below is unconditional, and that is the whole of what makes
    /// this test mean anything. An earlier version ran it only when the file
    /// was absent, which let the test execute whatever binary happened to be
    /// on disk. Measured during review, twice: with the pass arm of
    /// `exit_code` mutated to return 3, a stale binary left from an earlier
    /// build reported `ok` while the classifier tests went red around it, and
    /// deleting that one file turned the same tree red. A third instance
    /// occurred by accident in a full-suite run, which reported this test
    /// `ok` in 0.06s against a binary containing a symbol the source on disk
    /// no longer had. `cargo test` rebuilds this harness when the source
    /// changes; it does not rebuild the plain binary, so freshness is not
    /// something this test may assume. The ambient shared `CARGO_TARGET_DIR`
    /// on a developer machine can also hold another branch's binary
    /// entirely.
    ///
    /// Rebuilding costs nothing when the binary is already current -- cargo
    /// is incremental, and `make test` and `make ci` have already built it
    /// via `assurance-inputs` by the time this runs. It is ordinary
    /// compilation of the same already-reviewed source, not an evidence
    /// producer manufacturing its own input.
    #[test]
    fn built_example_binary_exits_zero_against_the_real_corpus() {
        let harness = std::env::current_exe().expect("current_exe resolves for a running test");
        let examples_dir = harness
            .parent()
            .expect("a test binary always has a parent directory");
        let plain_binary_name = if cfg!(windows) {
            "generation_conformance.exe"
        } else {
            "generation_conformance"
        };
        let plain_binary = examples_dir.join(plain_binary_name);
        {
            let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
            let status = std::process::Command::new(&cargo)
                .args(["build", "--locked", "--example", "generation_conformance"])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .status()
                .unwrap_or_else(|err| {
                    panic!("failed to run {cargo:?} build --example generation_conformance: {err}")
                });
            assert!(
                status.success(),
                "building the plain example binary failed: {status}"
            );
            assert!(
                plain_binary.is_file(),
                "cargo build reported success but {} still does not exist",
                plain_binary.display()
            );
        }
        let output = std::process::Command::new(&plain_binary)
            .output()
            .unwrap_or_else(|err| panic!("failed to execute {}: {err}", plain_binary.display()));
        assert_eq!(
            output.status.code(),
            Some(0),
            "expected the real bounded corpus to exit 0 (every row a pass); stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
