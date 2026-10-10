//! FR-014: exact complete-V1 scalar oracle generation over admitted
//! CheckedPackage V2 input.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::common::panic_scan::{comments_stripped, non_test_code, panic_tokens_in};
use crate::scratch_crate::seed_lock;
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, BoundForm, ClaimDerivationRefusal,
    ClaimDisposition, DecimalOperator, ExactScalarItem, ExactScalarOperation, ExactScalarOracles,
    ExactScalarRefusal, GeneratedScalarClaim, IntegerOperator, OperationProvenance,
    OracleGenerationError, RationalOperator, ScalarForm, UpstreamBlocker, EXACT_SCALAR_CRATE_NAME,
    SCALAR_LOWERING_SUPPORTED_TAGS,
};
use quire_contract_model::{
    CheckedPackageRefusalCause, CheckedPackageRefusalCode, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use quire_contract_runtime::exact::{
    ComparisonOperator, DivisionProfile, QuantityTarget, RoundingMode,
};
use serde_json::{json, Value};

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `kani_obligations.rs` each pick independently, on the
// assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_scalar_support/package.rs"]
mod package;

use package::*;

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

/// Generate, and scan the `src/lib.rs` it produced for a panic site (FR-014-AC-39). Every crate
/// this module's TC-024 tests generate comes through here, so each one is scanned.
fn generate(package: &CheckedPackageV2, items: &[ExactScalarItem]) -> ExactScalarOracles {
    let oracles = generate_exact_scalar_oracles(package, items).expect("generation succeeds");
    let found = panic_tokens_in(contents(&oracles, "src/lib.rs"));
    assert!(found.is_empty(), "generated src/lib.rs holds {found:?}");
    oracles
}

fn contents<'o>(oracles: &'o ExactScalarOracles, path: &str) -> &'o str {
    &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("artifact {path}"))
        .contents
}

fn dispositions(
    oracles: &ExactScalarOracles,
) -> BTreeMap<String, &ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>> {
    oracles
        .claim_map
        .items
        .iter()
        .map(|claim| (claim.node_id.digest.to_string(), &claim.result))
        .collect()
}

fn refusal_of(oracles: &ExactScalarOracles, code: u32) -> ExactScalarRefusal {
    match dispositions(oracles).get(code_id(code).digest.as_ref()) {
        Some(ClaimDisposition::Refused { refusal }) => refusal.clone(),
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

/// The generated oracle function name of node `code`, as the claim map records it.
pub(super) fn symbol(oracles: &ExactScalarOracles, code: u32) -> String {
    match dispositions(oracles).get(code_id(code).digest.as_ref()) {
        Some(ClaimDisposition::Generated(generated)) => generated.symbol.clone(),
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

/// The generator's current output for the whole corpus request, which
/// `exact_scalar_agreement` builds and executes.
pub(super) fn corpus_oracles() -> ExactScalarOracles {
    generate(&corpus_package().admit(), &golden_items())
}

/// Trace: FR-014-AC-2, TC-024. UNBACKED today: the six text admissions (FR-014-AC-2's text
/// admission case) are refused by Contract IR at admission, so no oracle is generated for them
/// and no agreement or generation test can run over them. `quire.op.numeric.convert` takes
/// `exact_numeric`, no catalogued `convert` identity takes `text`, and QSL lowers no text
/// admission; it is unblocked by a catalogued text-admission operation. This test pins the exact
/// refusal of each node (code, cause, pointer, locus), so the behaviour is recorded and a change
/// in either IR or the corpus shows up here.
#[test]
fn tc_024_text_admission_corpus_is_refused_by_ir_today() {
    let refused = refused_corpus();
    assert_eq!(refused.len(), TEXT_ADMISSIONS.len(), "the six admissions");
    for expression in refused {
        let code = expression.code;
        let (result, wire, ids) = refused_corpus_package(code).read_resolved();
        let node_id = ids.resolve(&code_id(code));
        let position = wire["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
            .unwrap_or_else(|| panic!("node {code} is in the wire"));
        let CheckedPackageV2ReadResult::Refused(refusal) = result else {
            panic!("node {code}: IR now admits it ({result:?}); move it back into the corpus");
        };
        assert_eq!(
            refusal.code,
            CheckedPackageRefusalCode::IllTyped,
            "node {code}"
        );
        assert_eq!(
            refusal.cause,
            Some(CheckedPackageRefusalCause::OperatorIneligible),
            "node {code}"
        );
        assert_eq!(
            refusal.path.as_ref().map(|path| path.as_str().to_owned()),
            Some(format!("/semantic_graph/nodes/{position}/body/arguments/0")),
            "node {code}"
        );
        assert_eq!(refusal.locus, Some(node_id), "node {code}");
    }
}

/// Trace: FR-014-AC-4, TC-024.
///
/// The corpus crate is exactly `Cargo.toml`, `src/lib.rs` and `claim-map.json`;
/// the claim map on disk is the serialized in-memory claim map; and every
/// generated claim's oracle symbol is defined in `src/lib.rs`.
#[test]
fn tc_024_corpus_crate_artifacts_are_complete_and_self_consistent() {
    let oracles = corpus_oracles();
    let paths = oracles
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        paths,
        BTreeSet::from(["Cargo.toml", "claim-map.json", "src/lib.rs"])
    );
    let on_disk: Value = serde_json::from_str(contents(&oracles, "claim-map.json")).unwrap();
    assert_eq!(on_disk, serde_json::to_value(&oracles.claim_map).unwrap());
    let lib = contents(&oracles, "src/lib.rs");
    let mut generated = 0_usize;
    for claim in &oracles.claim_map.items {
        if let ClaimDisposition::Generated(generated_claim) = &claim.result {
            let symbol = format!("pub fn {}(", generated_claim.symbol);
            assert!(lib.contains(&symbol), "{symbol} is not defined");
            generated += 1;
        }
    }
    // 59 admitted expressions: the original 65 less the six text admissions IR refuses today.
    assert!(generated >= 59, "the corpus generates every family");
}

/// Trace: FR-014-AC-4, TC-024.
#[test]
fn tc_024_generation_is_byte_deterministic_across_runs_and_request_orders() {
    let package = corpus_package().admit();
    let items = golden_items();
    let first = generate(&package, &items);
    assert_eq!(generate(&package, &items), first);
    assert_eq!(generate(&corpus_package().admit(), &items), first);

    let mut reversed = items.clone();
    reversed.reverse();
    assert_eq!(generate(&package, &reversed), first);

    // A deterministic interleaving that moves every item.
    let (even, odd): (Vec<_>, Vec<_>) = items
        .iter()
        .cloned()
        .enumerate()
        .partition(|(position, _)| position % 2 == 0);
    let interleaved = odd
        .into_iter()
        .chain(even)
        .map(|(_, item)| item)
        .collect::<Vec<_>>();
    assert_ne!(
        interleaved
            .iter()
            .map(|item| &item.node_id)
            .collect::<Vec<_>>(),
        items.iter().map(|item| &item.node_id).collect::<Vec<_>>()
    );
    assert_eq!(generate(&package, &interleaved), first);
}

/// Trace: FR-014-AC-1, FR-014-AC-2, TC-024.
#[test]
fn tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    let mut calls = vec![
        (
            1001,
            "rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::Add(".to_owned(),
        ),
        (
            1002,
            "rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::Negate(".to_owned(),
        ),
        (
            1003,
            "rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::Subtract(".to_owned(),
        ),
        (
            1004,
            "rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::Multiply(".to_owned(),
        ),
        (
            1011,
            "rt::divide(rt::DivisionProfile::Truncating,".to_owned(),
        ),
        (1012, "rt::divide(rt::DivisionProfile::Floor,".to_owned()),
        (
            1013,
            "rt::divide(rt::DivisionProfile::Euclidean,".to_owned(),
        ),
        (
            1014,
            "rt::divide(rt::DivisionProfile::Truncating,".to_owned(),
        ),
        (1021, "rt::modulo(".to_owned()),
        (
            1031,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Add(".to_owned(),
        ),
        (
            1032,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Divide(".to_owned(),
        ),
        (
            1033,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Divide(&left, &right)"
                .to_owned(),
        ),
        (
            1034,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Subtract(".to_owned(),
        ),
        (
            1035,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Multiply(".to_owned(),
        ),
        (
            1036,
            "rt::evaluate_rational_arithmetic(rt::RationalArithmetic::Negate(".to_owned(),
        ),
        (
            1041,
            "rt::OrderingOperator::Less, rt::OrderedOperands::Integers(".to_owned(),
        ),
        (
            1042,
            "rt::OrderingOperator::LessOrEqual, rt::OrderedOperands::Decimals(".to_owned(),
        ),
        (
            1043,
            "rt::OrderingOperator::Greater, rt::OrderedOperands::Rationals(".to_owned(),
        ),
        (
            1044,
            "rt::OrderingOperator::LessOrEqual, rt::OrderedOperands::Integers(".to_owned(),
        ),
        (
            1045,
            "rt::OrderingOperator::GreaterOrEqual, rt::OrderedOperands::Integers(".to_owned(),
        ),
        (
            1051,
            "rt::evaluate_decimal(rt::DecimalOperation::Add(".to_owned(),
        ),
        (
            1052,
            "rt::evaluate_decimal(rt::DecimalOperation::Divide(".to_owned(),
        ),
        (
            1053,
            "rt::evaluate_decimal(rt::DecimalOperation::Round(".to_owned(),
        ),
        (
            1054,
            "rt::evaluate_decimal(rt::DecimalOperation::Subtract(".to_owned(),
        ),
        (
            1055,
            "rt::evaluate_decimal(rt::DecimalOperation::Multiply(".to_owned(),
        ),
        (
            1056,
            "rt::evaluate_decimal(rt::DecimalOperation::Negate(".to_owned(),
        ),
        (1061, "rt::evaluate_ieee(rt::IeeeOperation::Add(".to_owned()),
        (
            1062,
            "rt::evaluate_ieee(rt::IeeeOperation::Divide(".to_owned(),
        ),
        (
            1063,
            "rt::compare_ieee(rt::IeeeComparison::TotalOrder,".to_owned(),
        ),
        (1064, "rt::convert_ieee_width(".to_owned()),
        (
            1065,
            "rt::evaluate_ieee(rt::IeeeOperation::Subtract(".to_owned(),
        ),
        (
            1066,
            "rt::evaluate_ieee(rt::IeeeOperation::Multiply(".to_owned(),
        ),
        (
            1067,
            "rt::compare_ieee(rt::IeeeComparison::NumericEqual,".to_owned(),
        ),
        (
            1068,
            "rt::compare_ieee(rt::IeeeComparison::BitIdentical,".to_owned(),
        ),
        (
            1081,
            "rt::evaluate_quantity(rt::QuantityOperation::Add(".to_owned(),
        ),
        (
            1082,
            "rt::evaluate_quantity(rt::QuantityOperation::Multiply(".to_owned(),
        ),
        (
            1083,
            "rt::evaluate_quantity(rt::QuantityOperation::Power(".to_owned(),
        ),
        (1086, "rt::QuantityTarget::Decimal(".to_owned()),
        (1087, "rt::QuantityTarget::Integer {".to_owned()),
        (
            1088,
            "rt::evaluate_quantity(rt::QuantityOperation::Subtract(".to_owned(),
        ),
        (
            1089,
            "rt::evaluate_quantity(rt::QuantityOperation::Divide(".to_owned(),
        ),
    ];
    for (index, operator) in ComparisonOperator::ALL.into_iter().enumerate() {
        calls.push((
            TEXT_COMPARISONS[index],
            format!("rt::compare_text(rt::ComparisonOperator::{operator:?},"),
        ));
        calls.push((
            ENUM_COMPARISONS[index],
            format!("rt::compare_enum(rt::ComparisonOperator::{operator:?},"),
        ));
        calls.push((
            QUANTITY_COMPARISONS[index],
            format!("rt::compare_quantity(rt::ComparisonOperator::{operator:?},"),
        ));
    }
    calls.sort();
    let generated = dispositions(&oracles)
        .into_iter()
        .filter(|(_, result)| matches!(result, ClaimDisposition::Generated(_)))
        .map(|(digest, _)| digest)
        .collect::<Vec<_>>();
    let mut expected = calls
        .iter()
        .map(|(code, _)| code_id(*code).digest.to_string())
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(generated, expected, "exactly the corpus is generated");
    assert_eq!(corpus().len(), calls.len());
    for (code, call) in &calls {
        let body = function_body(lib, &symbol(&oracles, *code));
        assert!(
            body.contains(call.as_str()),
            "{code} does not call `{call}`:\n{body}"
        );
        assert!(body.contains("meter"), "{code} does not meter");
    }
    assert_eq!(lib.matches("\npub fn oracle_").count(), calls.len());
}

/// The text of one generated function, from its signature to the closing brace.
fn function_body<'l>(lib: &'l str, symbol: &str) -> &'l str {
    let start = lib
        .find(&format!("pub fn {symbol}("))
        .unwrap_or_else(|| panic!("{symbol} is generated"));
    let end = lib[start..].find("\n}\n").expect("function end") + start + 3;
    &lib[start..end]
}

/// Trace: FR-014-AC-1, FR-014-AC-3, FR-014-AC-7, FR-014-AC-10, TC-024.
#[test]
fn tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged() {
    let (package, ids) = corpus_package().admit_resolved();
    let resolve = |code| ids.resolve(&code_id(code));
    let items = golden_items()
        .into_iter()
        .map(|mut item| {
            item.node_id = ids.resolve(&item.node_id);
            item
        })
        .collect::<Vec<_>>();
    let oracles = generate(&package, &items);
    let lib = contents(&oracles, "src/lib.rs");
    // IR's lowering checks reachable unsupported nodes in admitted node-id order. The
    // rekeyed clause sorts before its formula, so it is the first unsupported temporal node.
    assert!(resolve(TEMPORAL) < resolve(TEMPORAL_FORMULA));
    let blocked = |code, tag, issue| ExactScalarRefusal::BlockedOnUpstream {
        unsupported_node_id: resolve(code),
        node_tag: tag,
        issue,
    };
    let unsupported = |code, tag| ExactScalarRefusal::Unsupported {
        unsupported_node_id: resolve(code),
        node_tag: tag,
    };
    let expected = [
        (COMPOSITE, unsupported(COMPOSITE, "composite_type")),
        (
            FUNCTION,
            blocked(
                FUNCTION,
                "function",
                UpstreamBlocker::QuireContractRuntime34,
            ),
        ),
        (
            CALLS_FUNCTION,
            blocked(
                FUNCTION,
                "function",
                UpstreamBlocker::QuireContractRuntime34,
            ),
        ),
        (
            MODEL,
            blocked(MODEL, "model", UpstreamBlocker::QuireSpecLanguage120),
        ),
        (
            RELATION,
            blocked(RELATION, "relation", UpstreamBlocker::QuireSpecLanguage120),
        ),
        (STATE, unsupported(STATE, "state")),
        (TEMPORAL, unsupported(TEMPORAL, "temporal")),
        (PROTOCOL, unsupported(PROTOCOL, "protocol")),
        (MISSING, ExactScalarRefusal::InvalidInput),
        (
            V_BOOLEAN,
            ExactScalarRefusal::NotExpression { node_tag: "value" },
        ),
        (
            UNBOUNDED,
            ExactScalarRefusal::RequiresBound {
                unbounded_type: code_id(T_INTEGER),
            },
        ),
        (
            MATHEMATICAL,
            ExactScalarRefusal::BoundMismatch {
                bound: id(&INT.key()),
                form: BoundForm::IntegerRange,
            },
        ),
        (
            MISSING_ROUNDING,
            ExactScalarRefusal::MissingBound {
                bounded_type: code_id(T_FLOAT32),
                expected_form: BoundForm::FloatRounding,
            },
        ),
        (
            AMBIGUOUS,
            ExactScalarRefusal::AmbiguousBound {
                bounded_type: code_id(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
        ),
        (
            UNREADABLE,
            ExactScalarRefusal::UnreadableBound {
                bound: id(&unreadable_bound().key()),
            },
        ),
        (
            WRONG_BOUND_FORM,
            ExactScalarRefusal::MissingBound {
                bounded_type: code_id(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
        ),
        (
            DOMAIN_MISMATCH,
            ExactScalarRefusal::BoundMismatch {
                bound: id(&INT5.key()),
                form: BoundForm::IntegerRange,
            },
        ),
        (
            QUANTITY_EXACT,
            ExactScalarRefusal::BoundMismatch {
                bound: id(&RAT.key()),
                form: BoundForm::RationalRange,
            },
        ),
        (
            EXPRESSION_OPERAND,
            ExactScalarRefusal::OperandUnsupported {
                position: 0,
                term: "aggregate".to_owned(),
            },
        ),
        (
            LITERAL_QUANTITY,
            ExactScalarRefusal::UnitlessLiteralOperand { position: 1 },
        ),
        (
            UNTYPED_OPERAND,
            ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: None,
            },
        ),
        (DUPLICATED, ExactScalarRefusal::DuplicateRequest),
        (
            WRONG_BODY,
            ExactScalarRefusal::BodyMismatch {
                expected_operator: "binary",
                expected_arguments: 2,
            },
        ),
        (
            WRONG_OPERAND,
            ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: Some("decimal".to_owned()),
            },
        ),
        // Restores the coverage `WRONG_OPERAND` lost when it moved from a
        // reference to a literal operand (see its own comment above): a
        // reference operand that resolves to a concrete but wrong
        // `ScalarForm`, refused through `check_operand`'s `"reference"` arm
        // (`reference_form` -> a graph lookup -> `type_form`), not its
        // `"literal"` arm (`value_kind` -> `ScalarForm::from_literal_kind`).
        (
            WRONG_OPERAND_REFERENCE,
            ExactScalarRefusal::OperandTypeMismatch {
                position: 0,
                expected: ScalarForm::Rational,
                found: Some("integer".to_owned()),
            },
        ),
        (
            WRONG_RESULT,
            ExactScalarRefusal::ResultTypeMismatch {
                expected: ScalarForm::Decimal,
                found: Some("integer".to_owned()),
            },
        ),
        (
            WRONG_ARITY,
            ExactScalarRefusal::FormMismatch {
                expected: "unary",
                found: "binary".to_owned(),
            },
        ),
    ];
    for (code, refusal) in &expected {
        let node_id = resolve(*code);
        match dispositions(&oracles).get(node_id.digest.as_ref()) {
            Some(ClaimDisposition::Refused { refusal: actual }) => {
                assert_eq!(actual, refusal, "node {code}");
            }
            other => panic!("node {code} is not refused: {other:?}"),
        }
        assert!(
            !lib.contains(node_id.digest.as_ref()),
            "refused node {code} left code behind"
        );
    }
    // One disposition per distinct requested node.
    assert_eq!(
        oracles.claim_map.items.len(),
        corpus().len() + expected.len()
    );

    // Each generated function is byte-identical with or without the refused items.
    let generated_only = corpus()
        .into_iter()
        .map(|expression| ExactScalarItem {
            node_id: resolve(expression.code),
            operation: expression.operation,
        })
        .collect::<Vec<_>>();
    let alone = generate(&package, &generated_only);
    assert_eq!(contents(&alone, "src/lib.rs"), lib);
    let generated_claims = |oracles: &ExactScalarOracles| {
        oracles
            .claim_map
            .items
            .iter()
            .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(generated_claims(&alone), generated_claims(&oracles));

    // A refusal of one copy of a duplicated sibling does not depend on its
    // descriptors' arrival order, and a single copy of the same node generates.
    let single = generate(
        &package,
        &[ExactScalarItem {
            node_id: resolve(DUPLICATED),
            operation: golden_items()
                .into_iter()
                .find(|item| item.node_id == code_id(DUPLICATED))
                .expect("duplicated item")
                .operation,
        }],
    );
    assert!(matches!(
        single.claim_map.items[0].result,
        ClaimDisposition::Generated(_)
    ));
}

/// Trace: FR-014-AC-5, FR-014-AC-11, FR-014-AC-14, TC-024.
///
/// AC-14 is bound here for its provenance conjunct only: the loop below asserts that every
/// `Refused` claim is `CallerDeclared` with a typed blocked item. It asserts nothing about the
/// identity such a claim reports, so AC-14's identity clause stays unbacked and its row in
/// `spec/oracle/matrix/tests.md` says so.
///
/// Discharges FR-014-AC-11 over the whole golden corpus, typed and on the wire. Every
/// golden CORPUS item's descriptor agrees with its node -- `golden_items()` as a whole also
/// carries deliberate disagreements such as `DOMAIN_MISMATCH`, `WRONG_RESULT` and
/// `WRONG_ARITY`, which are refused -- so in THIS corpus every `Generated` claim is
/// `IrConfirmed`. That is a property of the corpus, not an invariant of the generator: node 1011
/// under a floor descriptor is `Generated` and `CallerDeclared` (AC-13), asserted in
/// `tc_024_a_mislabelled_descriptor_is_refused_where_bounds_disagree_and_marked_otherwise`.
/// Adding a deliberately mislabelled item to `golden_items()` would break the assertion below
/// for a reason unrelated to AC-5 or AC-11; widen AC-12/AC-13 coverage in their own tests.
#[test]
fn tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item() {
    let wire = corpus_package().wire();
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let map = &oracles.claim_map;
    let lib = contents(&oracles, "src/lib.rs");
    assert_eq!(map.package_id, *package.package_id());
    let json: Value = serde_json::from_str(contents(&oracles, "claim-map.json")).expect("json");
    assert_eq!(json, serde_json::to_value(map).expect("typed map"));

    // No blocker applies to every entry: a Generated claim's operation is IR-confirmed, and an
    // unconfirmed claim, in any of the cases `OperationProvenance::CallerDeclared` enumerates,
    // carries the blocker on its own `operation.provenance` rather than at the map
    // level. `blocked` has one writer in the crate (`oracle/scalar/mod.rs`'s `Vec::new()`), so this
    // pair is a regression guard against that literal changing, not a criterion an implementation
    // can violate; the falsifiable contract is the per-item provenance asserted below.
    assert_eq!(map.blocked, []);
    assert_eq!(json["blocked"], serde_json::json!([]));
    for (claim, entry) in map
        .items
        .iter()
        .zip(json["items"].as_array().expect("items"))
    {
        match &claim.result {
            ClaimDisposition::Generated(_) => {
                assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
                assert_eq!(
                    entry["operation"]["provenance"],
                    serde_json::json!({"kind": "ir_confirmed"})
                );
            }
            ClaimDisposition::Refused { .. } => {
                assert_eq!(
                    claim.operation.provenance,
                    OperationProvenance::CallerDeclared {
                        blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                    }
                );
                assert_eq!(
                    entry["operation"]["provenance"],
                    serde_json::json!({
                        "kind": "caller_declared",
                        "blocked_on": "operation identity not consumed by codegen's generators",
                    })
                );
            }
        }
    }

    // This list is hand-written and independently constructed -- not read from
    // `scalar_profile()` (`src/oracle/scalar/mod.rs`, private) -- so this cross-check calls IR's
    // `lower` directly rather than the generator's own profile, and stays meaningful rather than
    // circular. `LITERAL_OPERAND` reaches the `claim` nodes `corpus_package` wires as its
    // dependencies (issue #100), so this list must admit `Claim` too, or this cross-check would
    // mark that corpus node `Unsupported` while the real generator (which also admits `Claim`)
    // lowers it -- a divergence between the two, not a property of either.
    //
    // Nothing kept this list in agreement with `scalar_profile()`'s own `supported_tags` (cg#134):
    // before PR #132 (issue #100) this list had four tags against production's six, and after it,
    // this list still omitted `Correspondence`, which production still admitted despite zero
    // corpus nodes ever carrying it (cg#133, now removed from production instead). The assertion
    // below closes that gap: it compares this list's *content* against
    // `SCALAR_LOWERING_SUPPORTED_TAGS`, the tag set `scalar_profile()` itself builds from, so
    // either list changing without the other now fails here, while the lowering profile below
    // still never calls `scalar_profile()`.
    let claim_supported_tags: BTreeSet<quire_contract_model::CheckedNodeTag> = [
        quire_contract_model::CheckedNodeTag::ScalarType,
        quire_contract_model::CheckedNodeTag::BoundedDomain,
        quire_contract_model::CheckedNodeTag::Value,
        quire_contract_model::CheckedNodeTag::Expression,
        quire_contract_model::CheckedNodeTag::Claim,
    ]
    .into();
    assert_eq!(
        claim_supported_tags,
        BTreeSet::from(SCALAR_LOWERING_SUPPORTED_TAGS),
        "this test's independently-constructed tag list has drifted from scalar_profile()'s own \
         supported_tags (cg#134) -- update whichever one is stale"
    );

    let lowered = package.lower(
        &corpus()
            .iter()
            .map(|expression| code_id(expression.code))
            .collect::<Vec<_>>(),
        &quire_contract_model::CompleteLoweringProfileV2 {
            supported_tags: claim_supported_tags,
            require_bounds: true,
            work_limit: u64::MAX,
        },
    );
    for (expression, record) in corpus().iter().zip(&lowered.records) {
        let quire_contract_model::CompleteLoweringRecordV2::Lowered { node } = record else {
            panic!("corpus node {} lowers", expression.code);
        };
        let claim = map
            .items
            .iter()
            .find(|claim| claim.node_id == code_id(expression.code))
            .expect("claim");
        assert!(!claim.operation.identity.is_empty());
        let ClaimDisposition::Generated(generated) = &claim.result else {
            panic!("corpus node {} generates", expression.code);
        };
        assert!(
            generated.symbol.starts_with("oracle_")
                && lib.contains(&format!("pub fn {}(", generated.symbol)),
            "{} names no generated function",
            generated.symbol
        );
        assert_eq!(generated.ir_id, node.ir_id);
        assert_eq!(generated.semantic_form, expression.form);
        assert_eq!(generated.semantic_type, node.semantic_type);
        assert_eq!(generated.claims, node.claims);
        assert_eq!(generated.bounds, node.bounds);
        // Every checked bound is one the expression declares and reaches.
        let declared = expression
            .bounds
            .iter()
            .map(|bound| id(&bound.key()))
            .collect::<Vec<_>>();
        assert!(generated
            .checked_bounds
            .iter()
            .all(|bound| declared.contains(bound) && node.bounds.contains(bound)));
        assert_eq!(
            generated.checked_bounds.is_empty(),
            checks_no_bound(&expression.operation),
            "node {}",
            expression.code
        );
        assert_eq!(generated.dependencies, node.dependencies);
        let source_map = wire["source_map"]
            .as_array()
            .expect("source map")
            .iter()
            .filter(|entry| entry["node_id"]["digest"] == *code_id(expression.code).digest)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(source_map.len(), 1);
        assert_eq!(
            serde_json::to_value(&generated.source_map).expect("map"),
            Value::Array(source_map)
        );
    }

    // The reported identity is the node's own catalogued operation, not a per-instance string:
    // codes 1011 and 1014 are both `division(_, Truncating, ...)`, differing only in bound
    // (`[-1000,1000]` vs `[-5,5]`) -- codegen's own descriptor-derived string used to embed the
    // bound and so differed between them; the catalog does not, and both now report the same
    // `quire.op.integer.div`.
    let identity = |code| {
        map.items
            .iter()
            .find(|claim| claim.node_id == code_id(code))
            .expect("claim")
            .operation
            .identity
            .clone()
    };
    assert_eq!(identity(1011), "quire.op.integer.div");
    assert_eq!(identity(1014), "quire.op.integer.div");
}

/// Operations whose parameters the IR carries no bound for.
fn checks_no_bound(operation: &ExactScalarOperation) -> bool {
    matches!(
        operation,
        ExactScalarOperation::Ordering { .. }
            | ExactScalarOperation::IeeeComparison { .. }
            | ExactScalarOperation::TextComparison { .. }
            | ExactScalarOperation::EnumComparison { .. }
            | ExactScalarOperation::QuantityArithmetic { .. }
            | ExactScalarOperation::QuantityComparison { .. }
    )
}

/// Trace: FR-014-AC-4, TC-024.
#[test]
fn tc_024_claim_map_entries_ascend_by_node_id_domain_then_digest() {
    let package = corpus_package().admit();
    let other_domain: quire_contract_model::CheckedNodeId = serde_json::from_value(
        serde_json::json!({"domain": "quire.a-earlier-domain/v1", "digest": "f".repeat(64)}),
    )
    .expect("node id");
    let mut items = golden_items();
    items.push(ExactScalarItem {
        node_id: other_domain.clone(),
        operation: package::integer_add(),
    });
    let oracles = generate(&package, &items);
    let ids = oracles
        .claim_map
        .items
        .iter()
        .map(|claim| {
            (
                claim.node_id.domain.to_string(),
                claim.node_id.digest.to_string(),
            )
        })
        .collect::<Vec<_>>();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "entries ascend by domain, then digest");
    assert_eq!(oracles.claim_map.items[0].node_id, other_domain);
    // Its digest is the greatest requested, so it sorts first only by its domain.
    assert_eq!(
        oracles.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactScalarRefusal::InvalidInput
        }
    );
}

/// Trace: FR-014-AC-13, TC-024.
///
/// Node 1011 is a truncating division whose own `operation.laws` does not name the floor
/// definition. A floor descriptor implies the same catalogued identity (`quire.op.integer.div`),
/// so this is not AC-12's different-operation case, and the item's disposition is `Generated`, so
/// it is neither half of AC-14's never-inspected-or-refused case. It is the third state: the item
/// generates the oracle its descriptor
/// names and is marked `caller_declared` because the law disagrees. Deleting the law check would
/// leave AC-11, AC-12 and AC-14 satisfied and this one violated.
#[test]
fn tc_024_a_mislabelled_descriptor_is_refused_where_bounds_disagree_and_marked_otherwise() {
    let package = corpus_package().admit();
    let floor = |lower, upper| ExactScalarOperation::IntegerDivision {
        profile: quire_contract_runtime::exact::DivisionProfile::Floor,
        domain: bounded(lower, upper),
    };
    // Node 1014 is bounded [-5, 5]: a descriptor over [-1000, 1000] is refused.
    let refused = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(1014),
            operation: floor(-1000, 1000),
        }],
    );
    assert_eq!(
        refused.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactScalarRefusal::BoundMismatch {
                bound: id(&INT5.key()),
                form: BoundForm::IntegerRange,
            }
        }
    );

    // Node 1011 is a truncating division; its own `operation.laws` names the
    // truncating law definition, unlike 1012's (floor) and 1013's
    // (euclidean). `check_item`'s shape checks alone cannot see this --
    // every `DivisionProfile` gives the same binary `&rt::Integer` shape --
    // so a floor descriptor still generates against node 1011's truncating
    // body: `law_confirmed` catches the disagreement between the
    // descriptor's law and the node's own, and this generator does not
    // silently prefer either. The claim still generates (from the
    // descriptor, exactly as before this generator ever read
    // `operation.identity`), but its law is caller-declared, not confirmed.
    let marked = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(1011),
            operation: floor(-1000, 1000),
        }],
    );
    let claim = &marked.claim_map.items[0];
    assert!(matches!(claim.result, ClaimDisposition::Generated(_)));
    assert_eq!(
        claim.operation.provenance,
        OperationProvenance::CallerDeclared {
            blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
        }
    );
    assert_eq!(marked.claim_map.blocked, []);
}

/// Trace: FR-014-AC-13, TC-024.
///
/// AC-13's other conjunct: a descriptor whose implied identity matches the node's, and whose
/// rounding matches the node's own IR `decimal_range` bound (so `check_item` passes it), but
/// whose descriptor disagrees with that same node's separately catalogued `operation.mode`.
/// `MODE_MISMATCH` (node 2033) exists exactly for this: its `operation.mode` rounding is
/// `"toward-zero"`, but its IR bound rounding is `"nearest-even"` -- the two are independently
/// settable, and every other corpus node happens to keep them equal, so this is the only node
/// this crate can currently exercise the disagreement against. A descriptor whose own rounding is
/// `"nearest-even"` agrees with the bound (passing `check_item`) and disagrees with
/// `operation.mode`, so `operation_confirmed`'s `if let Some((kind, value)) = catalogued.mode`
/// branch runs and returns `false` on a genuine mismatch, not merely an absent member. The item
/// still generates -- exactly as the law conjunct does above -- and is marked `caller_declared`.
#[test]
fn tc_024_a_mismatched_rounding_mode_is_refused_where_the_bound_still_agrees() {
    let package = corpus_package().admit();
    let matching_bound = ExactScalarOperation::DecimalArithmetic {
        operator: DecimalOperator::Add,
        target: decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven),
    };
    let marked = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(MODE_MISMATCH),
            operation: matching_bound,
        }],
    );
    let claim = &marked.claim_map.items[0];
    assert!(matches!(claim.result, ClaimDisposition::Generated(_)));
    assert_eq!(
        claim.operation.provenance,
        OperationProvenance::CallerDeclared {
            blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
        }
    );
    assert_eq!(marked.claim_map.blocked, []);
}

/// `Shape::of` gives `Add`, `Subtract`, `Multiply`, `IntegerDivision` and `IntegerModulo` the
/// identical binary `&rt::Integer` shape, so `check_item`'s shape checks alone cannot distinguish
/// an `Add` descriptor from a `Multiply` one over the same bounds. Node 1004 is
/// `quire.op.integer.mul` over `[-1000,1000]`; naming `Add` against it, with the identical
/// `[-1000,1000]` domain, passes every shape and bound check `check_item` makes. Confirming it
/// anyway -- the defect this ticket exists to close -- would render an oracle that adds where the
/// catalog says multiply and a harness whose `operation_identity` says `mul`.
///
/// Trace: FR-014-AC-12, TC-024.
///
/// Backs AC-12: an `Add` descriptor against node 1004's catalogued `quire.op.integer.mul`, over
/// the identical `[-1000,1000]` bound and the identical binary `&rt::Integer` shape, is not
/// confirmed. The mismatched case is asserted first; the agreeing case second, which is AC-11
/// for that one node. AC-11 over the whole corpus is discharged by
/// `tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item`.
#[test]
fn tc_024_operator_confusion_within_one_shape_is_not_silently_confirmed() {
    let package = corpus_package().admit();
    let mismatched = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(1004),
            operation: ExactScalarOperation::IntegerArithmetic {
                operator: IntegerOperator::Add,
                domain: bounded(-1000, 1000),
            },
        }],
    );
    let claim = &mismatched.claim_map.items[0];
    assert!(matches!(claim.result, ClaimDisposition::Generated(_)));
    assert_eq!(claim.operation.identity, "integer.add domain=[-1000,1000]");
    assert_eq!(
        claim.operation.provenance,
        OperationProvenance::CallerDeclared {
            blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
        },
        "an Add descriptor over a catalogued mul node must not be marked IrConfirmed"
    );

    // The matching descriptor over the same node does confirm, reading the node's own
    // catalogued identity rather than re-deriving codegen's own descriptor string.
    let matched = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(1004),
            operation: ExactScalarOperation::IntegerArithmetic {
                operator: IntegerOperator::Multiply,
                domain: bounded(-1000, 1000),
            },
        }],
    );
    let matched_claim = &matched.claim_map.items[0];
    assert_eq!(matched_claim.operation.identity, "quire.op.integer.mul");
    assert_eq!(
        matched_claim.operation.provenance,
        OperationProvenance::IrConfirmed
    );
}

/// A package/item pair whose lone node (code 3001) exceeds
/// `SCALAR_LOWERING_WORK_LIMIT` during lowering. Each reference argument
/// costs a body term and a successor edge, so 40,000 of them exceed the
/// 65,536 units scalar lowering allows. Shared by
/// `tc_024_lowering_work_exhaustion_is_a_typed_refusal` and
/// `tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively`, each
/// of which still calls this function -- and so still pays to build and admit
/// this 40,000-node package -- independently; only the fixture-construction
/// code is shared here, not the runtime cost of paying for it twice.
fn work_exhausted_fixture() -> (CheckedPackageV2, ExactScalarItem) {
    let arguments = (0..40_000)
        .map(|_| reference(&key(V_INTEGER)))
        .collect::<Vec<_>>();
    let mut builder = corpus_package();
    builder.application_bounded(
        3001,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.add"),
            &key(T_INTEGER),
            // `quire.op.integer.add` takes exactly 2 operands, so the
            // 40,000 stress references live nested inside an `aggregate`
            // wrapper as the second operand rather than as 40,000 direct
            // operands: IR-216's operand-arity check
            // (`check_operands`/`entry.rest`) refuses any node whose
            // `body.arguments.len()` disagrees with its catalogued
            // identity's own operand count, but IR's *lowering* successor-
            // edge walk (`CheckedPackageV2::lower`, exercised below, wholly
            // separate from admission-time operation validation) still
            // recurses into a nested term tree regardless of catalog
            // arity, so the stress case is preserved.
            vec![
                reference(&key(V_INTEGER)),
                json!({"term": "aggregate", "members": arguments}),
            ],
        ),
        &[INT],
    );
    let limits = quire_contract_model::CheckedPackageReadLimits {
        bytes: 16 * 1024 * 1024,
        ..quire_contract_model::CheckedPackageReadLimits::bounded()
    };
    (
        builder.admit_with(limits),
        ExactScalarItem {
            node_id: code_id(3001),
            operation: package::integer_add(),
        },
    )
}

/// Trace: FR-014-AC-3, FR-014-AC-15, TC-024.
#[test]
fn tc_024_lowering_work_exhaustion_is_a_typed_refusal() {
    let (package, item) = work_exhausted_fixture();
    let oracles = generate(&package, &[item]);
    let ClaimDisposition::Refused {
        refusal: ExactScalarRefusal::LoweringWorkExhausted { limit, consumed },
    } = &oracles.claim_map.items[0].result
    else {
        panic!("expected exhaustion: {:?}", oracles.claim_map.items[0]);
    };
    assert_eq!(*limit, quire_contract_codegen::SCALAR_LOWERING_WORK_LIMIT);
    assert!(consumed > limit);
    assert!(!contents(&oracles, "src/lib.rs").contains(code_id(3001).digest.as_ref()));
}

/// Trace: FR-014-AC-40, TC-024.
///
/// A call whose lowered contract package is longer than the checked package, read under a ceiling
/// that admits the checked package and is one byte below that lowered package, fails every
/// requested record for bytes (Contract IR FR-038-AC-95): every item is refused as
/// `LoweringByteLimitExceeded` with the ceiling as `limit` and one shared `consumed` above it,
/// never as `LoweringWorkExhausted`, and no function is generated. The per-node case and each
/// other limit kind are asserted on hand-built records in the module's own tests.
#[test]
fn tc_024_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    const CHAIN: u32 = 150;

    let mut builder = corpus_package();
    integer_add_chain(&mut builder, CHAIN);
    let (_, ids) = builder.admit_resolved();
    let requested = (0..CHAIN)
        .map(|offset| ids.resolve(&code_id(CHAIN_BASE + offset)))
        .collect::<Vec<_>>();
    let items = requested
        .iter()
        .map(|node_id| ExactScalarItem {
            node_id: node_id.clone(),
            operation: package::integer_add(),
        })
        .collect::<Vec<_>>();
    let generate_under = |ceiling: u64| {
        generate_exact_scalar_oracles(&builder.admit_with(limits_under(ceiling)), &items)
            .expect("generation succeeds")
    };
    let byte_refusal =
        |result: &ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>| match result {
            ClaimDisposition::Refused {
                refusal: ExactScalarRefusal::LoweringByteLimitExceeded { limit, consumed },
            } => Some((*limit, *consumed)),
            _ => None,
        };

    let checked_length = u64::try_from(serde_json::to_vec(&builder.wire()).unwrap().len()).unwrap();
    let lowered_length = lowered_package_length(
        &builder.admit_with(limits_under(LARGEST_CEILING)),
        &requested,
        &measuring_profile(true),
    );
    let ceiling = lowered_length - 1;
    assert!(
        ceiling >= checked_length,
        "the lowered package ({lowered_length} bytes) is longer than the checked package \
         ({checked_length} bytes), so a ceiling one byte below it admits the checked package"
    );
    // At the lowered package's own length no record fails for bytes: the ceiling is exact.
    assert!(generate_under(lowered_length)
        .claim_map
        .items
        .iter()
        .all(|claim| byte_refusal(&claim.result).is_none()));

    let oracles = generate_under(ceiling);
    assert_eq!(oracles.claim_map.items.len(), items.len());
    let mut consumed_seen = BTreeSet::new();
    for claim in &oracles.claim_map.items {
        let Some((limit, consumed)) = byte_refusal(&claim.result) else {
            panic!("expected a byte-ceiling refusal: {claim:?}");
        };
        assert_eq!(limit, ceiling);
        assert!(
            consumed > ceiling,
            "{consumed} is above the ceiling {ceiling}"
        );
        consumed_seen.insert(consumed);
    }
    assert_eq!(
        consumed_seen.len(),
        1,
        "one shared `consumed`: {consumed_seen:?}"
    );
    assert!(
        !contents(&oracles, "src/lib.rs").contains("pub fn oracle_"),
        "no function is generated"
    );
}

/// Every `ExactScalarRefusal` variant's name, matched with **no `_` arm**.
/// This is the guard IR-229 exists for: FR-014-AC-11 and its two Behavior
/// bullets used to name `check_item`'s checks in prose, three separate
/// enumerations that PR #113 alone found stale five times over. Prose can go
/// stale silently; this match cannot -- add a twenty-first `ExactScalarRefusal`
/// variant and this function stops compiling until a matching arm exists for
/// it. That compile failure is the only part the compiler enforces: it forces
/// the arm, not a fixture. A correct new arm with no corresponding entry in
/// `tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively` below
/// compiles and passes fine -- pairing the new variant with real coverage
/// there is a human convention this match cannot enforce, and a reviewer must
/// still check for it.
fn refusal_variant_name(refusal: &ExactScalarRefusal) -> &'static str {
    match refusal {
        ExactScalarRefusal::DuplicateRequest => "DuplicateRequest",
        ExactScalarRefusal::InvalidInput => "InvalidInput",
        ExactScalarRefusal::Unsupported { .. } => "Unsupported",
        ExactScalarRefusal::BlockedOnUpstream { .. } => "BlockedOnUpstream",
        ExactScalarRefusal::RequiresBound { .. } => "RequiresBound",
        ExactScalarRefusal::MissingBound { .. } => "MissingBound",
        ExactScalarRefusal::AmbiguousBound { .. } => "AmbiguousBound",
        ExactScalarRefusal::UnreadableBound { .. } => "UnreadableBound",
        ExactScalarRefusal::BoundMismatch { .. } => "BoundMismatch",
        ExactScalarRefusal::OperandUnsupported { .. } => "OperandUnsupported",
        ExactScalarRefusal::UnitlessLiteralOperand { .. } => "UnitlessLiteralOperand",
        ExactScalarRefusal::InvalidBody { .. } => "InvalidBody",
        ExactScalarRefusal::BodyIncomplete { .. } => "BodyIncomplete",
        ExactScalarRefusal::LoweringWorkExhausted { .. } => "LoweringWorkExhausted",
        ExactScalarRefusal::LoweringByteLimitExceeded { .. } => "LoweringByteLimitExceeded",
        ExactScalarRefusal::LoweringLimitUnrecognised { .. } => "LoweringLimitUnrecognised",
        ExactScalarRefusal::NotExpression { .. } => "NotExpression",
        ExactScalarRefusal::FormMismatch { .. } => "FormMismatch",
        ExactScalarRefusal::BodyMismatch { .. } => "BodyMismatch",
        ExactScalarRefusal::ResultTypeMismatch { .. } => "ResultTypeMismatch",
        ExactScalarRefusal::OperandTypeMismatch { .. } => "OperandTypeMismatch",
        ExactScalarRefusal::MissingOperationIdentity { .. } => "MissingOperationIdentity",
        ExactScalarRefusal::NoDerivableClaim { .. } => "NoDerivableClaim",
    }
}

/// Trace: FR-014-AC-1, FR-014-AC-3, FR-014-AC-11, FR-014-AC-14, TC-024.
///
/// One fixture per `ExactScalarRefusal` variant, each checked against
/// `refusal_variant_name`'s exhaustive match (see its own doc). What this
/// test adds is not novel exhaustiveness protection for `ExactScalarRefusal`
/// -- `src/kani/generate/negotiate.rs` already has its own wildcard-free match over
/// the same enum in production code, so a twenty-first variant would already
/// break that build today whether or not this test exists. What this test
/// adds is the variant<->fixture pairing: proof that every variant names a
/// real, driven scenario (or, for the three noted below, a scenario
/// deliberately undrivable), not just a name `refusal_variant_name` happens
/// to mention.
///
/// Seventeen variants are driven through the real admitted-package generation
/// pipeline -- sixteen share the golden corpus's one `generate` call (see
/// `tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged`,
/// which asserts the full typed reason for each of these same codes; this
/// test only needs the variant, not the reason's payload), and
/// `LoweringWorkExhausted` needs `work_exhausted_fixture`'s own package.
///
/// Three variants are documented, on both sides of the crate boundary, as
/// unreachable through any package this crate's public API can admit, so no
/// admitted-package fixture for them exists to drive:
/// `MissingOperationIdentity` (`catalogued_operation_identity`'s own doc:
/// "this generator should never receive a node for which this member is
/// absent" -- quire-contract-ir's `validate_operations` guarantees it before
/// admission) and `InvalidBody`/`BodyIncomplete` (quire-contract-ir's own
/// `CompleteLoweringRecordV2` doc: "A package the V2 reader admitted never
/// yields this" / "Like `InvalidBody`, an admitted package never yields
/// this"). Real production-path coverage of these three already exists, as
/// unit tests inside the crate that call the real private functions this
/// integration-test crate cannot reach:
/// `tc_024_invalid_and_incomplete_bodies_are_typed_refusals` drives
/// `InvalidBody`/`BodyIncomplete` through the real `lowered()`, and
/// `tc_024_catalogued_operation_identity_refuses_a_node_whose_operation_has_no_identity`
/// drives `MissingOperationIdentity` through the real
/// `catalogued_operation_identity` (both in `src/oracle/scalar/mod.rs`, both
/// private to `quire_contract_codegen` and so unreachable from here). The
/// three assertions below construct each variant directly instead; they only
/// exercise `refusal_variant_name`'s own match arms for these three variants
/// -- they are not evidence that production code produces them, which is the
/// two unit tests' job.
#[test]
fn tc_024_every_exact_scalar_refusal_variant_is_matched_exhaustively() {
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());

    let from_the_golden_corpus = [
        (DUPLICATED, "DuplicateRequest"),
        (MISSING, "InvalidInput"),
        (COMPOSITE, "Unsupported"),
        (FUNCTION, "BlockedOnUpstream"),
        (UNBOUNDED, "RequiresBound"),
        (MISSING_ROUNDING, "MissingBound"),
        (AMBIGUOUS, "AmbiguousBound"),
        (UNREADABLE, "UnreadableBound"),
        (MATHEMATICAL, "BoundMismatch"),
        (EXPRESSION_OPERAND, "OperandUnsupported"),
        (LITERAL_QUANTITY, "UnitlessLiteralOperand"),
        (V_BOOLEAN, "NotExpression"),
        (WRONG_ARITY, "FormMismatch"),
        (WRONG_BODY, "BodyMismatch"),
        (WRONG_RESULT, "ResultTypeMismatch"),
        (WRONG_OPERAND, "OperandTypeMismatch"),
    ];
    for (code, expected) in from_the_golden_corpus {
        assert_eq!(
            refusal_variant_name(&refusal_of(&oracles, code)),
            expected,
            "node {code}"
        );
    }

    let (work_exhausted_package, work_exhausted_item) = work_exhausted_fixture();
    let work_exhausted_oracles = generate(&work_exhausted_package, &[work_exhausted_item]);
    assert_eq!(
        refusal_variant_name(&refusal_of(&work_exhausted_oracles, 3001)),
        "LoweringWorkExhausted"
    );

    // Unreachable through admission (see this test's own doc): constructed
    // directly rather than fabricated through a fixture that cannot exist.
    // These three assertions exercise `refusal_variant_name`'s own match
    // arms for these variants; the real production-path coverage is the two
    // unit tests named in this test's doc comment above, not these lines.
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::MissingOperationIdentity {
            node_id: code_id(V_BOOLEAN),
        }),
        "MissingOperationIdentity"
    );
    // `NoDerivableClaim` is recorded by the routed generation arm and never by
    // `generate_exact_scalar_oracles`; TC-033 drives it through `generate_routed`.
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::NoDerivableClaim {
            reason: ClaimDerivationRefusal::MissingOperationIdentity,
        }),
        "NoDerivableClaim"
    );
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::InvalidBody {
            body_node_id: code_id(V_BOOLEAN),
        }),
        "InvalidBody"
    );
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::BodyIncomplete {
            body_node_id: code_id(V_BOOLEAN),
        }),
        "BodyIncomplete"
    );
    // The byte-ceiling refusal is driven through a whole call by
    // `tc_024_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal`; an
    // unrecognised limit kind only through the hand-built records of the module's own
    // `tc_024_a_failed_record_is_refused_by_its_limit_kind`. These lines exercise the name arms.
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::LoweringByteLimitExceeded {
            limit: 1,
            consumed: 2,
        }),
        "LoweringByteLimitExceeded"
    );
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::LoweringLimitUnrecognised {
            limit_kind: "nodes",
            limit: 1,
            consumed: 2,
        }),
        "LoweringLimitUnrecognised"
    );
}

/// Trace: FR-014-AC-9, TC-024.
#[test]
fn tc_024_generated_source_over_the_ceiling_is_refused_whole() {
    let mut builder = corpus_package();
    let codes = 10_000..14_000;
    for code in codes.clone() {
        // Both operands are `code`-keyed literals, not `reference(ENUM_MEMBER)`
        // twice over: every one of these 4,000 nodes would otherwise share
        // one preimage (same tag/form/type/body) and collide on a single
        // `node_id`. A literal operand also bypasses IR's operand-family
        // check entirely, so `enum.eq`'s `enum_kind` expectation is never
        // actually exercised here -- this fixture's point is source size,
        // not enum-family conformance.
        builder.application_code(
            code,
            "expression",
            "binary",
            &key(T_BOOLEAN),
            application(
                "binary",
                op("quire.op.enum.eq"),
                &key(T_BOOLEAN),
                vec![
                    literal("enum", &code.to_string()),
                    literal("enum", &code.to_string()),
                ],
            ),
        );
    }
    let limits = quire_contract_model::CheckedPackageReadLimits {
        bytes: 16 * 1024 * 1024,
        ..quire_contract_model::CheckedPackageReadLimits::bounded()
    };
    let (package, ids) = builder.admit_with_resolved(limits);
    let items = codes
        .map(|code| ExactScalarItem {
            node_id: ids.resolve(&code_id(code)),
            operation: ExactScalarOperation::EnumComparison {
                operator: ComparisonOperator::Equal,
            },
        })
        .collect::<Vec<_>>();
    assert!(matches!(
        generate_exact_scalar_oracles(&package, &items),
        Err(OracleGenerationError::SourceTooLarge { bytes, .. })
            if bytes > quire_contract_codegen::MAX_GENERATED_SOURCE_BYTES
    ));
}

/// Trace: FR-014-AC-3, FR-014-AC-9, TC-024.
#[test]
fn tc_024_literal_operands_are_classified_by_value_kind_and_constants_stop_typed() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    // Node 1003's right operand is an integer literal, so it type-checks as an
    // integer and the subtraction generates like any other.
    assert!(matches!(
        dispositions(&oracles)[code_id(LITERAL_OPERAND).digest.as_ref()],
        ClaimDisposition::Generated(_)
    ));
    let subtract = function_body(lib, &symbol(&oracles, LITERAL_OPERAND));
    assert!(
        subtract.contains("rt::IntegerArithmetic::Subtract("),
        "{subtract}"
    );

    // The same shape with a text literal is refused with the literal's kind. Contract IR resolves
    // a literal operand's family through `literal.type` and does not cross-check it against
    // `value_kind`, so the literal is typed by the integer type to admit and carries the `text`
    // value kind CG classifies it by.
    // A deliberate negative fixture: type and `value_kind` disagree, and QSL builds no text
    // literal, so it never emits this shape.
    let mut builder = corpus_package();
    builder.application_bounded(
        3002,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.add"),
            &key(T_INTEGER),
            vec![
                reference(&key(V_INTEGER)),
                json!({
                    "term": "literal",
                    "type": {"domain": NODE_DOMAIN, "digest": key(T_INTEGER)},
                    "value_kind": "text",
                    "value": "3",
                }),
            ],
        ),
        &[INT],
    );
    let refused = generate(
        &builder.admit(),
        &[ExactScalarItem {
            node_id: code_id(3002),
            operation: package::integer_add(),
        }],
    );
    assert_eq!(
        refused.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: Some("text".to_owned()),
            }
        }
    );

    // A decimal target the runtime refuses is an invalid constant, like every
    // other generated constant.
    let add = function_body(lib, &symbol(&oracles, 1051));
    assert!(
        add.contains("rt::DecimalType::new(")
            && add.contains(".map_err(|_| OracleStop::InvalidConstant)?"),
        "{add}"
    );
    let decimal_constants = lib
        .lines()
        .filter(|line| line.contains("rt::DecimalType::new("))
        .collect::<Vec<_>>();
    assert!(!decimal_constants.is_empty());
    for line in decimal_constants {
        assert!(
            line.contains("map_err(|_| OracleStop::InvalidConstant)?")
                && !line.contains("OracleStop::IllTyped"),
            "{line}"
        );
    }
}

/// Trace: FR-014-AC-8, FR-014-AC-9, TC-024.
#[test]
fn tc_024_generated_crate_is_unpublished_charge_free_and_compiles() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{EXACT_SCALAR_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains("features = [\"exact\"]"));

    let lib = contents(&oracles, "src/lib.rs");
    for forbidden in [
        "ChargePoint",
        "charge(",
        "Resource::",
        "WorkLimits",
        "panic!",
        "unwrap(",
        "expect(",
        "unreachable!",
        "todo!",
        "unsafe {",
        "unsafe fn",
    ] {
        assert!(
            !lib.contains(forbidden),
            "generated source contains `{forbidden}`"
        );
    }

    let directory = TemporaryDirectory::new("quire-exact-scalar-oracles");
    fs::create_dir_all(directory.0.join("src")).unwrap();
    for artifact in &oracles.artifacts {
        fs::write(directory.0.join(&artifact.path), &artifact.contents).unwrap();
    }
    seed_lock(&directory.0);
    let output = Command::new(env!("CARGO"))
        .args(["build", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated crate did not compile against the runtime: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A `reference` operand typed by an `integer_range` `bounded_domain` (QSL's `Int[0, 9]`
/// parameter) is an Integer operand and generates; before IR-297 it was refused as
/// `OperandTypeMismatch { found: None }`.
///
/// Trace: FR-014-AC-17, TC-024.
#[test]
fn tc_024_ac17_an_operand_typed_by_a_bounded_domain_generates() {
    let oracles = generate(
        &bounded_increment_package().admit(),
        &[ExactScalarItem {
            node_id: code_id(BOUNDED_INCREMENT),
            operation: integer_add_0_9(),
        }],
    );
    assert!(matches!(
        dispositions(&oracles).get(code_id(BOUNDED_INCREMENT).digest.as_ref()),
        Some(ClaimDisposition::Generated(_))
    ));
}

fn add_over(low: i64, high: i64) -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(low, high),
    }
}

fn two_parameter_disposition(
    code: u32,
    operation: ExactScalarOperation,
) -> (
    ExactScalarOracles,
    ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>,
) {
    let (package, ids) = two_parameter_package().admit_resolved();
    let node_id = ids.resolve(&code_id(code));
    let oracles = generate(
        &package,
        &[ExactScalarItem {
            node_id: node_id.clone(),
            operation,
        }],
    );
    let disposition = dispositions(&oracles)
        .get(node_id.digest.as_ref())
        .map(|disposition| (*disposition).clone())
        .expect("one claim per item");
    (oracles, disposition)
}

fn two_parameter_refusal(code: u32, operation: ExactScalarOperation) -> ExactScalarRefusal {
    match two_parameter_disposition(code, operation).1 {
        ClaimDisposition::Refused { refusal } => refusal,
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

fn two_parameter_checked_bounds(
    code: u32,
    operation: ExactScalarOperation,
) -> Vec<quire_contract_model::CheckedNodeId> {
    match two_parameter_disposition(code, operation).1 {
        ClaimDisposition::Generated(generated) => generated.checked_bounds,
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

fn two_parameter_derive(code: u32) -> Result<ExactScalarItem, ExactScalarRefusal> {
    let (package, ids) = two_parameter_package().admit_resolved();
    derive_one_resolved(&package, &ids, code)
}

/// Two distinct bounded parameters, `Int[0, 9]` and `Int[10, 20]`, with a result typed `[0, 29]`
/// generate: the descriptor is compared with the result bound only, and the claim's checked
/// bounds are the result bound first, then each operand's own. Derivation returns the descriptor
/// over the result bound. Before IR-298 the three reachable `integer_range` bounds over Integer
/// were `AmbiguousBound`.
///
/// Trace: FR-014-AC-20, TC-024
#[test]
fn tc_024_ac20_two_distinct_bounded_parameters_generate_against_the_result_bound() {
    assert_eq!(
        two_parameter_checked_bounds(TWO_PARAMETER_SUM, add_over(0, 29)),
        [
            id(&Bound::Integer(0, 29).key()),
            id(&Bound::Integer(0, 9).key()),
            id(&Bound::Integer(10, 20).key()),
        ]
    );
    assert_eq!(
        two_parameter_derive(TWO_PARAMETER_SUM)
            .expect("derives")
            .operation,
        add_over(0, 29)
    );
    // The descriptor is the result bound: one over an operand's bound is a mismatch, not a pick.
    assert_eq!(
        two_parameter_refusal(TWO_PARAMETER_SUM, add_over(0, 9)),
        ExactScalarRefusal::BoundMismatch {
            bound: id(&Bound::Integer(0, 29).key()),
            form: BoundForm::IntegerRange,
        }
    );
}

/// A scalar-typed result whose reachable bounds are the operands' own two and one more resolves to
/// the one that is no operand's.
///
/// Trace: FR-014-AC-21, TC-024
#[test]
fn tc_024_ac21_a_scalar_typed_result_takes_the_one_bound_no_operand_types() {
    assert_eq!(
        two_parameter_checked_bounds(TWO_PARAMETER_ATTACHED, add_over(0, 29))[0],
        id(&Bound::Integer(0, 29).key())
    );
}

/// A scalar-typed result whose reachable bounds are all operands' own has no result bound.
///
/// Trace: FR-014-AC-22, TC-024
#[test]
fn tc_024_ac22_a_scalar_typed_result_with_only_operand_bounds_is_ambiguous() {
    assert_eq!(
        two_parameter_refusal(TWO_PARAMETER_NO_RESULT, add_over(0, 29)),
        ExactScalarRefusal::AmbiguousBound {
            bounded_type: code_id(T_INTEGER),
            expected_form: BoundForm::IntegerRange,
        }
    );
}

/// An operand, or the result, typed by a bound of another form is `MissingBound`.
///
/// Trace: FR-014-AC-23, TC-024
#[test]
fn tc_024_ac23_an_operand_or_result_typed_by_a_bound_of_another_form_is_refused() {
    for code in [
        TWO_PARAMETER_WRONG_FORM_OPERAND,
        TWO_PARAMETER_WRONG_FORM_RESULT,
    ] {
        assert_eq!(
            two_parameter_refusal(code, add_over(0, 29)),
            ExactScalarRefusal::MissingBound {
                bounded_type: code_id(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
            "node {code}"
        );
    }
}

/// Operand bounds need not lie inside the result bound: `a + wide` over `[0, 9]` and `[0, 50]`
/// with a `[0, 29]` result, `-e` over `[1, 9]` with a `[-9, -1]` result and `e * f` over `[1, 9]`
/// and `[100, 200]` with a `[100, 1800]` result all generate, derive the same descriptor, and
/// record each operand's own bound after the result bound.
///
/// Trace: FR-014-AC-24, TC-024
#[test]
fn tc_024_ac24_operand_bounds_need_not_lie_inside_the_result_bound() {
    let negate = ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Negate,
        domain: bounded(-9, -1),
    };
    let product = ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Multiply,
        domain: bounded(100, 1800),
    };
    for (code, operation, expected) in [
        (
            TWO_PARAMETER_WIDE,
            add_over(0, 29),
            vec![
                Bound::Integer(0, 29),
                Bound::Integer(0, 9),
                Bound::Integer(0, 50),
            ],
        ),
        (
            BOUNDED_NEGATE,
            negate,
            vec![Bound::Integer(-9, -1), Bound::Integer(1, 9)],
        ),
        (
            BOUNDED_PRODUCT,
            product,
            vec![
                Bound::Integer(100, 1800),
                Bound::Integer(1, 9),
                Bound::Integer(100, 200),
            ],
        ),
    ] {
        assert_eq!(
            two_parameter_derive(code).expect("derives").operation,
            operation,
            "node {code}: derivation and generation agree"
        );
        assert_eq!(
            two_parameter_checked_bounds(code, operation),
            expected
                .iter()
                .map(|bound| id(&bound.key()))
                .collect::<Vec<_>>(),
            "node {code}"
        );
    }
}

/// A `reference` operand typed by a plain scalar type owns no bound: where another operand owns
/// one, or the result is typed by one, it is `RequiresBound` naming its type, in generation and in
/// derivation. A literal operand is never refused, and a package whose operands own no bound and
/// whose result is scalar-typed keeps the one-bound reading.
///
/// Trace: FR-014-AC-25, TC-024
#[test]
fn tc_024_ac25_a_plain_typed_reference_operand_beside_bounded_typing_requires_a_bound() {
    // The scalar-typed result's one reachable bound is `[0, 9]`, so its descriptor is over that.
    for (code, operation) in [
        (TWO_PARAMETER_UNBOUNDED_OPERAND, add_over(0, 29)),
        (TWO_PARAMETER_PLAIN_PAIR, add_over(0, 29)),
        (TWO_PARAMETER_PLAIN_SCALAR_RESULT, add_over(0, 9)),
    ] {
        let expected = ExactScalarRefusal::RequiresBound {
            unbounded_type: code_id(T_INTEGER),
        };
        assert_eq!(
            two_parameter_refusal(code, operation),
            expected,
            "node {code}"
        );
        assert_eq!(
            two_parameter_derive(code),
            Err(expected),
            "node {code}: derivation refuses what generation refuses"
        );
    }
    assert!(matches!(
        two_parameter_disposition(TWO_PARAMETER_LITERAL, add_over(0, 29)).1,
        ClaimDisposition::Generated(_)
    ));
    // A bounded parameter beside a plain-typed reference to a `value` node whose body is a literal
    // is a constant operand, not a plain-typed one (FR-014-AC-26), so it generates.
    let oracles = generate(
        &corpus_package().admit(),
        &[ExactScalarItem {
            node_id: code_id(BOUNDED_OPERAND),
            operation: integer_add(),
        }],
    );
    assert!(matches!(
        dispositions(&oracles).get(code_id(BOUNDED_OPERAND).digest.as_ref()),
        Some(ClaimDisposition::Generated(_))
    ));
}

fn qsl_disposition(
    code: u32,
    operation: ExactScalarOperation,
) -> ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal> {
    let (package, ids) = qsl_shaped_package().admit_resolved();
    let node_id = ids.resolve(&code_id(code));
    let oracles = generate(
        &package,
        &[ExactScalarItem {
            node_id: node_id.clone(),
            operation,
        }],
    );
    dispositions(&oracles)
        .get(node_id.digest.as_ref())
        .map(|disposition| (*disposition).clone())
        .expect("one claim per item")
}

fn derive_one_resolved(
    package: &CheckedPackageV2,
    ids: &FixtureIds,
    code: u32,
) -> Result<ExactScalarItem, ExactScalarRefusal> {
    derive_exact_scalar_items(package, &[ids.resolve(&code_id(code))])
        .pop()
        .expect("one result per node id")
}

/// The checked bounds of a QSL-shaped node that generates.
fn qsl_checked_bounds(
    code: u32,
    operation: ExactScalarOperation,
) -> Vec<quire_contract_model::CheckedNodeId> {
    match qsl_disposition(code, operation) {
        ClaimDisposition::Generated(generated) => generated.checked_bounds,
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

fn bound_ids(bounds: &[Bound]) -> Vec<quire_contract_model::CheckedNodeId> {
    bounds.iter().map(|bound| id(&bound.key())).collect()
}

/// Generation and derivation over a QSL-shaped node both refuse it with `expected`.
fn assert_qsl_refused(code: u32, operation: ExactScalarOperation, expected: ExactScalarRefusal) {
    match qsl_disposition(code, operation) {
        ClaimDisposition::Refused { refusal } => assert_eq!(refusal, expected, "node {code}"),
        other => panic!("node {code} is not refused: {other:?}"),
    }
    let (package, ids) = qsl_shaped_package().admit_resolved();
    assert_eq!(
        derive_one_resolved(&package, &ids, code),
        Err(expected),
        "node {code}: derivation refuses what generation refuses"
    );
}

fn requires_integer_bound() -> ExactScalarRefusal {
    ExactScalarRefusal::RequiresBound {
        unbounded_type: code_id(T_INTEGER),
    }
}

fn ambiguous_integer_bound() -> ExactScalarRefusal {
    ExactScalarRefusal::AmbiguousBound {
        bounded_type: code_id(T_INTEGER),
        expected_form: BoundForm::IntegerRange,
    }
}

/// QSL emits the literal of `x + 1` as a `reference` to its own `value` node, whose body is the
/// literal and whose type is the plain Integer type: that operand is a literal, so `x + 1` over
/// `x: Int[0, 9]` narrowed to `Int[0, 10]` generates with checked bounds `[0, 10]` then `[0, 9]`
/// and derives the descriptor over `[0, 10]`.
///
/// Trace: FR-014-AC-26, TC-024
#[test]
fn tc_024_ac26_a_reference_to_a_literal_value_node_is_a_literal_operand() {
    assert_eq!(
        qsl_checked_bounds(QSL_INC, add_over(0, 10)),
        bound_ids(&[Bound::Integer(0, 10), Bound::Integer(0, 9)])
    );
    let (package, ids) = qsl_shaped_package().admit_resolved();
    assert_eq!(
        derive_one_resolved(&package, &ids, QSL_INC)
            .expect("derives")
            .operation,
        add_over(0, 10)
    );
}

/// QSL wraps arithmetic in a bounded context in a narrowing `conversion` typed by the declared
/// bound, and the arithmetic node's own result type is the plain Integer type: its result bound is
/// the conversion's. `x + y` over `[0, 9]` and `[10, 20]` narrowed to `[10, 29]` checks `[10, 29]`,
/// `[0, 9]`, `[10, 20]`, and a descriptor over an operand's bound is a mismatch against `[10, 29]`;
/// `-z` over `[0, 9]` narrowed to `[-9, 0]` checks `[-9, 0]` then `[0, 9]`.
///
/// Trace: FR-014-AC-27, TC-024
#[test]
fn tc_024_ac27_a_scalar_typed_result_takes_the_bound_of_its_narrowing_conversion() {
    let (package, ids) = qsl_shaped_package().admit_resolved();
    let negate = ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Negate,
        domain: bounded(-9, 0),
    };
    for (code, operation, expected) in [
        (
            QSL_ADD,
            add_over(10, 29),
            vec![
                Bound::Integer(10, 29),
                Bound::Integer(0, 9),
                Bound::Integer(10, 20),
            ],
        ),
        (
            QSL_NEGATE,
            negate,
            vec![Bound::Integer(-9, 0), Bound::Integer(0, 9)],
        ),
    ] {
        assert_eq!(
            derive_one_resolved(&package, &ids, code)
                .expect("derives")
                .operation,
            operation,
            "node {code}"
        );
        assert_eq!(
            qsl_checked_bounds(code, operation),
            bound_ids(&expected),
            "node {code}"
        );
    }
    match qsl_disposition(QSL_ADD, add_over(0, 9)) {
        ClaimDisposition::Refused { refusal } => assert_eq!(
            refusal,
            ExactScalarRefusal::BoundMismatch {
                bound: id(&Bound::Integer(10, 29).key()),
                form: BoundForm::IntegerRange,
            }
        ),
        other => panic!("not refused: {other:?}"),
    }
}

/// The referenced node's body decides: `n` is a plain-Integer `value` node of form `literal`
/// whose body is not a literal, so `x + n` is `RequiresBound`.
///
/// Trace: FR-014-AC-28, TC-024
#[test]
fn tc_024_ac28_a_literal_form_node_without_a_literal_body_is_not_a_literal() {
    assert_qsl_refused(
        QSL_NOT_LITERAL_OPERAND,
        add_over(0, 29),
        requires_integer_bound(),
    );
}

/// `x + p` narrowed to `Int[0, 29]`, `p` a plain-Integer parameter, is `RequiresBound`.
///
/// Trace: FR-014-AC-29, TC-024
#[test]
fn tc_024_ac29_a_plain_parameter_beside_a_bounded_one_requires_a_bound() {
    assert_qsl_refused(QSL_PLAIN_OPERAND, add_over(0, 29), requires_integer_bound());
}

/// `q + q` narrowed to `Int[0, 29]`, no operand owning a bound, is `RequiresBound`: the narrowing
/// bounds the result as a `bounded_domain` result type does.
///
/// Trace: FR-014-AC-30, TC-024
#[test]
fn tc_024_ac30_plain_operands_under_a_narrowing_require_a_bound() {
    assert_qsl_refused(QSL_PLAIN_PAIR, add_over(0, 29), requires_integer_bound());
}

/// A literal operand of an integer operation that is not an `integer` literal is refused by its
/// own kind, whatever the node referencing it is typed: `x + "a"`, the text literal in a
/// plain-Integer-typed `value` node.
///
/// Trace: FR-014-AC-31, TC-024
#[test]
fn tc_024_ac31_a_non_integer_literal_operand_is_an_operand_type_mismatch() {
    assert_qsl_refused(
        QSL_PLUS_TEXT,
        add_over(0, 10),
        ExactScalarRefusal::OperandTypeMismatch {
            position: 1,
            expected: ScalarForm::Integer,
            found: Some("text".to_owned()),
        },
    );
}

/// Two narrowing conversions to different bounds leave the result role unresolved.
///
/// Trace: FR-014-AC-32, TC-024
#[test]
fn tc_024_ac32_two_distinct_narrowings_are_ambiguous() {
    assert_qsl_refused(
        QSL_TWICE_NARROWED,
        add_over(10, 29),
        ambiguous_integer_bound(),
    );
}

/// A narrowing to a bound of another form, or of the form over another scalar type, is
/// `MissingBound`.
///
/// Trace: FR-014-AC-33, TC-024
#[test]
fn tc_024_ac33_a_narrowing_of_another_form_or_base_type_is_missing_bound() {
    for code in [QSL_WRONG_FORM_NARROWED, QSL_WRONG_BASE_NARROWED] {
        assert_qsl_refused(
            code,
            add_over(10, 29),
            ExactScalarRefusal::MissingBound {
                bounded_type: code_id(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
        );
    }
}

/// `x + 2` is narrowed to `Int[0, 11]` and is also an operand of `(x + 2) + x`: the narrow's
/// bound does not bound that other use, so it is `AmbiguousBound`. `x + 1`, consumed only by its
/// narrowing, generates.
///
/// Trace: FR-014-AC-34, TC-024
#[test]
fn tc_024_ac34_a_node_narrowed_and_consumed_otherwise_is_ambiguous() {
    assert_qsl_refused(QSL_SHARED, add_over(0, 11), ambiguous_integer_bound());
    assert!(matches!(
        qsl_disposition(QSL_INC, add_over(0, 10)),
        ClaimDisposition::Generated(_)
    ));
}

fn no_claim(reason: ClaimDerivationRefusal) -> ExactScalarRefusal {
    ExactScalarRefusal::NoDerivableClaim { reason }
}

fn derive_one(
    package: &CheckedPackageV2,
    code: u32,
) -> Result<ExactScalarItem, ExactScalarRefusal> {
    derive_exact_scalar_items(package, &[code_id(code)])
        .pop()
        .expect("one result per node id")
}

/// Deriving from the package alone yields, for every golden-corpus node, the descriptor the
/// fixture declares, and generating from the derived items gives the same claim map as
/// generating from the declared ones, every claim `ir_confirmed`.
///
/// Trace: FR-014-AC-18, FR-014-AC-19, TC-024
#[test]
fn tc_024_derivation_equals_every_golden_corpus_descriptor() {
    let package = corpus_package().admit();
    let corpus = corpus();
    // 59 admitted expressions: the original 65 less the six text admissions IR refuses today.
    assert!(corpus.len() >= 59, "the corpus is the whole golden set");
    let ids = corpus
        .iter()
        .map(|expression| code_id(expression.code))
        .collect::<Vec<_>>();
    let derived = derive_exact_scalar_items(&package, &ids);
    assert_eq!(
        derived.len(),
        ids.len(),
        "one result per node id, in input order"
    );
    let mut items = Vec::new();
    for (expression, derived) in corpus.iter().zip(derived) {
        let item = derived.unwrap_or_else(|refusal| {
            panic!(
                "corpus node {} does not derive: {refusal:?}",
                expression.code
            )
        });
        assert_eq!(item.node_id, code_id(expression.code));
        assert_eq!(
            item.operation, expression.operation,
            "node {} derives to its declared descriptor",
            expression.code
        );
        items.push(item);
    }
    let from_derived = generate(&package, &items).claim_map;
    let declared = corpus
        .iter()
        .map(|expression| ExactScalarItem {
            node_id: code_id(expression.code),
            operation: expression.operation.clone(),
        })
        .collect::<Vec<_>>();
    assert_eq!(from_derived, generate(&package, &declared).claim_map);
    assert!(from_derived.items.iter().all(|claim| {
        matches!(claim.result, ClaimDisposition::Generated(_))
            && claim.operation.provenance == OperationProvenance::IrConfirmed
    }));
}

/// The operations with more than one descriptor for one identity are told apart by what the node
/// carries: the operand forms (`rational.div`), the result form (`numeric.convert`,
/// `numeric.convert_rounding`, `quantity.convert`) and the laws (`integer.div`).
///
/// Trace: FR-014-AC-18, TC-024
#[test]
fn tc_024_each_overloaded_identity_derives_by_its_own_selector() {
    let package = derivation_package().admit();
    let operation = |code| derive_one(&package, code).map(|item| item.operation);
    // rational.div: both rationals divide, both integers integer-divide, a mix is neither.
    assert!(matches!(
        operation(1032),
        Ok(ExactScalarOperation::RationalArithmetic {
            operator: RationalOperator::Divide,
            ..
        })
    ));
    assert!(matches!(
        operation(1033),
        Ok(ExactScalarOperation::RationalArithmetic {
            operator: RationalOperator::IntegerDivide,
            ..
        })
    ));
    assert_eq!(
        operation(DERIVE_MIXED_DIV),
        Err(no_claim(ClaimDerivationRefusal::OperandFormsNotDerivable {
            operation_identity: "quire.op.rational.div".to_owned()
        }))
    );
    // numeric.convert_rounding to a decimal is a rounding conversion.
    assert!(matches!(
        operation(1053),
        Ok(ExactScalarOperation::DecimalArithmetic {
            operator: DecimalOperator::Round,
            ..
        })
    ));
    // numeric.convert to an integer is neither a rounding nor an admission. (A conversion to a
    // text type is an admission, but IR refuses those nodes today: see
    // `tc_024_text_admission_corpus_is_refused_by_ir_today`.)
    assert_eq!(
        operation(DERIVE_CONVERT_TO_INTEGER),
        Err(no_claim(ClaimDerivationRefusal::OperandFormsNotDerivable {
            operation_identity: "quire.op.numeric.convert".to_owned()
        }))
    );
    // quantity.convert takes its target from the result form; an integer target takes its
    // rounding from the node's mode, a rational result is an exact conversion.
    assert!(matches!(
        operation(1086),
        Ok(ExactScalarOperation::QuantityConversion {
            target: QuantityTarget::Decimal(_)
        })
    ));
    assert!(matches!(
        operation(1087),
        Ok(ExactScalarOperation::QuantityConversion {
            target: QuantityTarget::Integer {
                rounding: RoundingMode::TowardZero,
                ..
            }
        })
    ));
    assert!(matches!(
        operation(QUANTITY_EXACT),
        Ok(ExactScalarOperation::QuantityConversion {
            target: QuantityTarget::Exact
        })
    ));
    // integer.div takes its profile from the laws: the three corpus nodes differ only by it.
    let profiles = [1011, 1012, 1013].map(|code| match operation(code) {
        Ok(ExactScalarOperation::IntegerDivision { profile, .. }) => profile,
        other => panic!("node {code} does not derive a division: {other:?}"),
    });
    assert_eq!(
        profiles,
        [
            DivisionProfile::Truncating,
            DivisionProfile::Floor,
            DivisionProfile::Euclidean
        ]
    );
}

/// The refused set: identities outside the derivable set, nodes that are not applications, and
/// refused bounds each return their own typed refusal and no descriptor.
///
/// Trace: FR-014-AC-18, FR-014-AC-22, TC-024
#[test]
fn tc_024_derivation_refuses_what_it_cannot_derive_with_a_typed_reason() {
    let package = derivation_package().admit();
    for (code, identity) in [
        (DERIVE_REM, "quire.op.integer.rem"),
        (DERIVE_INTEGER_EQ, "quire.op.integer.eq"),
    ] {
        assert_eq!(
            derive_one(&package, code),
            Err(no_claim(ClaimDerivationRefusal::OperationNotDerivable {
                operation_identity: identity.to_owned()
            }))
        );
    }
    assert_eq!(
        derive_one(&package, V_BOOLEAN),
        Err(no_claim(ClaimDerivationRefusal::NotApplication {
            node_tag: "value".to_owned()
        }))
    );
    // Lowering and bound refusals are the ones `generate_exact_scalar_oracles` gives, so FR-015
    // classifies them as it always did.
    let bound = |code| match derive_one(&package, code) {
        Err(refusal) => refusal,
        other => panic!("node {code}: expected a refusal, got {other:?}"),
    };
    assert_eq!(bound(MISSING), ExactScalarRefusal::InvalidInput);
    assert!(matches!(
        bound(FUNCTION),
        ExactScalarRefusal::BlockedOnUpstream { .. }
    ));
    assert!(matches!(
        bound(UNBOUNDED),
        ExactScalarRefusal::RequiresBound { .. }
    ));
    assert!(matches!(
        bound(MISSING_ROUNDING),
        ExactScalarRefusal::MissingBound { .. }
    ));
    assert!(matches!(
        bound(UNREADABLE),
        ExactScalarRefusal::UnreadableBound { .. }
    ));
    // A node whose closure carries two bounds of one form on its scalar result type, neither the
    // own bound of an operand, is ambiguous, so it derives no domain. A node over two bounded
    // parameters is not: each operand names its own bound (IR-298, FR-014-AC-20).
    assert!(matches!(
        bound(AMBIGUOUS),
        ExactScalarRefusal::AmbiguousBound { .. }
    ));
}

/// Trace: FR-014-AC-39, FR-018-AC-19, TC-024, TC-029. The shared scan names every token the two
/// criteria list, in each delimiter form and with whitespace before the `!`, and does not name
/// the allowed lookalikes, so a scan test built on it can fail on each and only on those.
#[test]
fn tc_024_ac39_the_panic_scan_names_every_banned_token_and_no_lookalike() {
    for banned in [
        "x.unwrap()",
        "x.unwrap ()",
        "x\n    .unwrap()",
        "x.map(Option::unwrap)",
        "Result::expect(x, \"m\")",
        "use std::option::Option::unwrap;",
        "x.expect(\"m\")",
        "x.expect_err(\"m\")",
        "x.unwrap_err()",
        "x.unwrap_err_unchecked()",
        "std::panic::panic_any(1)",
        "std::panic::resume_unwind(b)",
        "x.unwrap_unchecked()",
        "panic!(\"m\")",
        "panic![]",
        "panic!{}",
        "panic !(\"m\")",
        "std::panic!(\"m\")",
        "unreachable!()",
        "todo!()",
        "unimplemented!()",
        "assert!(a)",
        "assert_eq!(a, b)",
        "assert_ne!(a, b)",
        "debug_assert!(a)",
        "debug_assert_eq!(a, b)",
        "debug_assert_ne!(a, b)",
        "std::process::abort()",
        "use std::process::abort;\nfn f() { abort() }",
        "use std::process::{abort};",
    ] {
        assert!(
            !panic_tokens_in(banned).is_empty(),
            "the scan missed `{banned}`"
        );
    }
    for allowed in [
        "x.unwrap_or_else(f)",
        "x.unwrap_or(1)",
        "x.unwrap_or_default()",
        "let assertion = 1;",
        "fn panic_free() {}",
        "a != b && todo != 1",
        "handle.abort()",
    ] {
        assert_eq!(
            panic_tokens_in(allowed),
            Vec::<String>::new(),
            "the scan named `{allowed}`"
        );
    }
}

/// Trace: FR-014-AC-39, FR-018-AC-19, TC-024, TC-029. `non_test_code` removes the items a
/// `#[cfg(test)]` attribute governs by structure, wherever they sit, and keeps everything else:
/// an earlier mention of the attribute in a comment or a literal cuts nothing, code after a test
/// module is kept, braces in literals do not end the module early, and comments are dropped
/// without mistaking `//` inside a string for one.
#[test]
fn tc_024_ac39_non_test_code_strips_test_items_by_structure() {
    let source = "\
// the module below is #[cfg(test)]
fn before() { a.unwrap() }
/* #[cfg(test)] fn commented() { todo!() } */
fn literal() -> &'static str { \"#[cfg(test)] http://x\" }
#[cfg(test)]
mod tests {
    fn t() { let s = \"}\"; let c = '{'; todo!() }
}
fn after() { b.expect(1) }
#[cfg(test)]
use helper::only_in_tests;
fn last() { c.unwrap_or(1) }
";
    let code = non_test_code(source);
    for kept in [
        "fn before()",
        "a.unwrap()",
        "fn literal()",
        "http://x",
        "fn after()",
        "b.expect(1)",
        "fn last()",
    ] {
        assert!(code.contains(kept), "`{kept}` was dropped from:\n{code}");
    }
    for dropped in [
        "mod tests",
        "todo!",
        "only_in_tests",
        "commented",
        "the module below",
    ] {
        assert!(!code.contains(dropped), "`{dropped}` survived in:\n{code}");
    }
    assert_eq!(
        panic_tokens_in(&code),
        vec!["expect".to_owned(), "unwrap".to_owned()]
    );
}

/// Trace: FR-014-AC-39, TC-024. The scalar generator's non-test code holds no panic site, and
/// neither does the `src/lib.rs` of the corpus crate (every other crate this module generates
/// is scanned inside [`generate`] as it is produced).
#[test]
fn tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(root.join("src/oracle/scalar/mod.rs")).expect("read generator");
    assert_eq!(
        panic_tokens_in(&non_test_code(&source)),
        Vec::<String>::new(),
        "src/oracle/scalar/mod.rs"
    );
    let oracles = corpus_oracles();
    assert_eq!(
        panic_tokens_in(contents(&oracles, "src/lib.rs")),
        Vec::<String>::new(),
        "the TC-024 corpus crate"
    );
}

/// Trace: FR-014-AC-42, TC-024.
///
/// The non-test code of the three oracle generators reads no `failed` record's limit kind: it
/// names no `CheckedPackageLimit`, accesses no `.limit_kind`, and meets a `Failed` record only at
/// an arm that binds nothing (`Failed { .. }`) and hands the record to
/// `classify_lowering_failure`. The refusal enums' own `limit_kind` field, which the criterion's
/// "holds no `limit_kind`" cannot mean literally, is the one place the name may appear, and only
/// in the unrecognised-kind refusal and the conversion into it. The shared module, which does
/// read the kind, is the control: it names `CheckedPackageLimit`.
#[test]
fn tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let code_of = |path: &str| {
        let source = fs::read_to_string(root.join(path)).expect("read generator source");
        non_test_code(&comments_stripped(&source))
    };

    let shared = code_of("src/oracle/mod.rs");
    assert!(
        shared.contains("CheckedPackageLimit") && shared.contains("fn classify_lowering_failure"),
        "the shared module owns the classifier"
    );

    for path in [
        "src/oracle/scalar/mod.rs",
        "src/oracle/equality/mod.rs",
        "src/oracle/function/mod.rs",
    ] {
        let code = code_of(path);
        assert!(
            !code.contains("CheckedPackageLimit"),
            "{path} names a limit kind type"
        );
        assert!(
            !code.contains(".limit_kind"),
            "{path} accesses a limit kind"
        );

        let lines = code.lines().collect::<Vec<_>>();
        for (at, line) in lines.iter().enumerate() {
            if line.contains("limit_kind") {
                let near = lines[at.saturating_sub(3)..=at]
                    .iter()
                    .any(|earlier| earlier.contains("LimitUnrecognised"));
                assert!(
                    near,
                    "{path}:{} names `limit_kind` outside the refusal: {line}",
                    at + 1
                );
            }
        }

        let arms = code.matches("CompleteLoweringRecordV2::Failed").count();
        assert_eq!(arms, 1, "{path} has one `Failed` arm");
        let arm = &code[code
            .find("CompleteLoweringRecordV2::Failed")
            .expect("the arm")..];
        let arm = &arm[..arm.len().min(160)];
        assert!(
            arm.starts_with("CompleteLoweringRecordV2::Failed { .. }")
                && arm.contains("classify_lowering_failure("),
            "{path}'s `Failed` arm must bind nothing and call the classifier: {arm}"
        );
    }
}

/// The panicking macros no generator or generated oracle may invoke.
const PANICKING_MACROS: [&str; 4] = ["unreachable", "panic", "todo", "unimplemented"];

/// True when `text` invokes one of [`PANICKING_MACROS`]: the name, optional whitespace, then `!`,
/// whatever delimiter follows and whatever path precedes it. Shared with the function-oracle
/// emitted-source scan in `exact_function_generation`.
pub(crate) fn invokes_panicking_macro(text: &str) -> bool {
    PANICKING_MACROS.iter().any(|name| {
        text.match_indices(name).any(|(at, _)| {
            let before_is_ident = text[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            let rest = text[at + name.len()..].trim_start();
            !before_is_ident && rest.starts_with('!')
        })
    })
}

/// Trace: FR-021-AC-21, TC-031 (the source half: the function generator holds zero invocations;
/// the emitted source is AC-19's, in `exact_function_generation`); IR-352 (partial, the scalar
/// and equality generators). Evidence that no generator panics on an unknown Contract Runtime
/// variant. The RT enums are `#[non_exhaustive]` and foreign to this crate, so a test cannot build
/// an unknown variant to drive `OracleGenerationError::UnknownRuntimeVariant`, and every mapper
/// takes the RT enum directly (there is no CG-local seam to inject one through). The evidence is
/// therefore the source itself: the exact-scalar and composite-equality generators minus comments
/// and any `#[cfg(test)]` item wherever it sits (`non_test_code`, as their ACs say), and the
/// function generator whole, minus comments only (`comments_stripped`, as FR-021-AC-21 says),
/// contain no `unreachable!`, `panic!`, `todo!` or `unimplemented!` invocation (any delimiter,
/// any path prefix) and no excused arm. The equality generator's two former `unreachable!` arms,
/// for `ValueType::Quantity` and `ValueType::Reference`, now return a typed render error
/// (FR-018-AC-18).
#[test]
fn oracle_generators_have_no_panicking_arms() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut offending = Vec::new();
    for (file, whole_file) in [
        ("src/oracle/scalar/mod.rs", false),
        ("src/oracle/equality/mod.rs", false),
        ("src/oracle/function/mod.rs", true),
    ] {
        let source = fs::read_to_string(root.join(file)).expect("read generator source");
        let counted = if whole_file {
            comments_stripped(&source)
        } else {
            non_test_code(&source)
        };
        for line in counted.lines() {
            if invokes_panicking_macro(line) {
                offending.push(format!("{file}: {}", line.trim()));
            }
        }
    }
    assert!(
        offending.is_empty(),
        "panicking arms in generator source: {offending:#?}"
    );
}

/// Trace: FR-021-AC-21, TC-031. The whole-file scan of the function generator counts a
/// `#[cfg(test)]` module: a banned macro inside one survives `comments_stripped` and is named,
/// where `non_test_code` would drop it, and a banned word inside a comment is not counted.
#[test]
fn tc_031_ac21_the_whole_file_scan_counts_a_test_module_and_not_a_comment() {
    let source = "fn real() {}\n// todo!()\n#[cfg(test)]\nmod probe_tests {\n    fn p() { unreachable!(\"probe\") }\n}\n";
    let whole = comments_stripped(source);
    assert!(whole.lines().any(invokes_panicking_macro));
    assert_eq!(panic_tokens_in(&whole), vec!["unreachable!".to_owned()]);
    assert!(!non_test_code(source).lines().any(invokes_panicking_macro));
}
