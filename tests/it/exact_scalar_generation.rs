//! FR-014: exact complete-V1 scalar oracle generation over admitted
//! CheckedPackage V2 input.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_exact_scalar_oracles, BoundForm, ExactScalarDisposition, ExactScalarGenerationError,
    ExactScalarItem, ExactScalarOperation, ExactScalarOracles, ExactScalarRefusal, IntegerOperator,
    OperationProvenance, ScalarForm, UpstreamBlocker, EXACT_SCALAR_CLAIM_MAP_VERSION,
    EXACT_SCALAR_CRATE_NAME, RUNTIME_REVISION, SCALAR_LOWERING_SUPPORTED_TAGS,
};
use quire_contract_ir::CheckedPackageV2;
use quire_contract_runtime::exact::{ComparisonOperator, TextProfile};
use serde_json::{json, Value};

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `kani_obligations.rs` each pick independently, on the
// assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_scalar_support/package.rs"]
mod package;

use package::*;

/// Set to regenerate the committed golden files instead of comparing them.
const BLESS: &str = "QUIRE_CODEGEN_BLESS";

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

fn generate(package: &CheckedPackageV2, items: &[ExactScalarItem]) -> ExactScalarOracles {
    generate_exact_scalar_oracles(package, items).expect("generation succeeds")
}

fn contents<'o>(oracles: &'o ExactScalarOracles, path: &str) -> &'o str {
    &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("artifact {path}"))
        .contents
}

fn dispositions(oracles: &ExactScalarOracles) -> BTreeMap<String, &ExactScalarDisposition> {
    oracles
        .claim_map
        .items
        .iter()
        .map(|claim| (claim.node_id.digest.to_string(), &claim.result))
        .collect()
}

fn refusal_of(oracles: &ExactScalarOracles, code: u32) -> ExactScalarRefusal {
    match dispositions(oracles).get(code_id(code).digest.as_ref()) {
        Some(ExactScalarDisposition::Refused { refusal }) => refusal.clone(),
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

fn symbol(code: u32) -> String {
    format!("oracle_{}", code_id(code).digest)
}

/// Trace: FR-014-AC-4, TC-024.
#[test]
fn tc_024_generation_matches_the_committed_golden_files() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/exact_scalar");
    for (artifact, golden) in [
        ("Cargo.toml", "Cargo.toml.golden"),
        ("src/lib.rs", "lib.rs.golden"),
        ("claim-map.json", "claim-map.json.golden"),
    ] {
        let path = fixtures.join(golden);
        if std::env::var_os(BLESS).is_some() {
            fs::write(&path, contents(&oracles, artifact)).expect("write golden");
        }
        let expected = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}; set {BLESS}=1", path.display()));
        assert_eq!(
            contents(&oracles, artifact),
            expected,
            "{artifact} drifted from {golden}"
        );
    }
    for artifact in &oracles.artifacts {
        assert_eq!(
            artifact.sha256,
            sha256_hex(artifact.contents.as_bytes()),
            "{}",
            artifact.path
        );
    }
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
    for (code, profile) in TEXT_ADMISSIONS.into_iter().zip(TextProfile::ALL) {
        calls.push((code, format!("rt::TextProfile::{profile:?}")));
    }
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
        .filter(|(_, result)| matches!(result, ExactScalarDisposition::Generated(_)))
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
        let body = function_body(lib, &symbol(*code));
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
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    let blocked = |code, tag, issue| ExactScalarRefusal::BlockedOnUpstream {
        unsupported_node_id: code_id(code),
        node_tag: tag,
        issue,
    };
    let unsupported = |code, tag| ExactScalarRefusal::Unsupported {
        unsupported_node_id: code_id(code),
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
                term: "application".to_owned(),
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
        assert_eq!(refusal_of(&oracles, *code), *refusal, "node {code}");
        assert!(
            !lib.contains(code_id(*code).digest.as_ref()),
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
            node_id: code_id(expression.code),
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
            .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_)))
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(generated_claims(&alone), generated_claims(&oracles));

    // A refusal of one copy of a duplicated sibling does not depend on its
    // descriptors' arrival order, and a single copy of the same node generates.
    let single = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(DUPLICATED),
            operation: golden_items()
                .into_iter()
                .find(|item| item.node_id == code_id(DUPLICATED))
                .expect("duplicated item")
                .operation,
        }],
    );
    assert!(matches!(
        single.claim_map.items[0].result,
        ExactScalarDisposition::Generated(_)
    ));
}

/// Trace: FR-014-AC-5, FR-014-AC-11, FR-014-AC-14, TC-024.
///
/// AC-14 is bound here for its provenance conjunct only: the loop below asserts that every
/// `Refused` claim is `CallerDeclared` with a typed blocked item. It asserts nothing about the
/// identity such a claim reports, so AC-14's identity clause stays unbacked and its row in
/// `spec/test-matrix.md` says so.
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
    assert_eq!(map.version, EXACT_SCALAR_CLAIM_MAP_VERSION);
    assert_eq!(map.runtime_revision, RUNTIME_REVISION);
    assert_eq!(map.package_id, *package.package_id());
    let json: Value = serde_json::from_str(contents(&oracles, "claim-map.json")).expect("json");
    assert_eq!(json, serde_json::to_value(map).expect("typed map"));

    // No blocker applies to every entry: a Generated claim's operation is IR-confirmed, and an
    // unconfirmed claim, in any of the cases `OperationProvenance::CallerDeclared` enumerates,
    // carries the blocker on its own `operation.provenance` rather than at the map
    // level. `blocked` has one writer in the crate (`exact_scalar.rs`'s `Vec::new()`), so this
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
            ExactScalarDisposition::Generated(_) => {
                assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
                assert_eq!(
                    entry["operation"]["provenance"],
                    serde_json::json!({"kind": "ir_confirmed"})
                );
            }
            ExactScalarDisposition::Refused { .. } => {
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
    // `scalar_profile()` (`src/exact_scalar.rs`, private) -- so this cross-check calls IR's
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
    let claim_supported_tags: BTreeSet<quire_contract_ir::CheckedNodeTag> = [
        quire_contract_ir::CheckedNodeTag::ScalarType,
        quire_contract_ir::CheckedNodeTag::BoundedDomain,
        quire_contract_ir::CheckedNodeTag::Value,
        quire_contract_ir::CheckedNodeTag::Expression,
        quire_contract_ir::CheckedNodeTag::Claim,
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
        &quire_contract_ir::CompleteLoweringProfileV2 {
            supported_tags: claim_supported_tags,
            require_bounds: true,
            work_limit: u64::MAX,
        },
    );
    for (expression, record) in corpus().iter().zip(&lowered.records) {
        let quire_contract_ir::CompleteLoweringRecordV2::Lowered { node } = record else {
            panic!("corpus node {} lowers", expression.code);
        };
        let claim = map
            .items
            .iter()
            .find(|claim| claim.node_id == code_id(expression.code))
            .expect("claim");
        assert!(!claim.operation.identity.is_empty());
        let ExactScalarDisposition::Generated(generated) = &claim.result else {
            panic!("corpus node {} generates", expression.code);
        };
        assert_eq!(generated.symbol, symbol(expression.code));
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
    let other_domain: quire_contract_ir::CheckedNodeId = serde_json::from_value(
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
        ExactScalarDisposition::Refused {
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
        ExactScalarDisposition::Refused {
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
    assert!(matches!(claim.result, ExactScalarDisposition::Generated(_)));
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
    assert!(matches!(claim.result, ExactScalarDisposition::Generated(_)));
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

/// Trace: FR-014-AC-3, TC-024.
#[test]
fn tc_024_lowering_work_exhaustion_is_a_typed_refusal() {
    // Each reference argument costs a body term and a successor edge, so
    // 40,000 of them exceed the 65,536 units scalar lowering allows.
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
    let limits = quire_contract_ir::CheckedPackageReadLimits {
        bytes: 16 * 1024 * 1024,
        ..quire_contract_ir::CheckedPackageReadLimits::bounded()
    };
    let oracles = generate(
        &builder.admit_with(limits),
        &[ExactScalarItem {
            node_id: code_id(3001),
            operation: package::integer_add(),
        }],
    );
    let ExactScalarDisposition::Refused {
        refusal: ExactScalarRefusal::LoweringWorkExhausted { limit, consumed },
    } = &oracles.claim_map.items[0].result
    else {
        panic!("expected exhaustion: {:?}", oracles.claim_map.items[0]);
    };
    assert_eq!(*limit, quire_contract_codegen::SCALAR_LOWERING_WORK_LIMIT);
    assert!(consumed > limit);
    assert!(!contents(&oracles, "src/lib.rs").contains(code_id(3001).digest.as_ref()));
}

/// Trace: FR-014-AC-9, TC-024.
#[test]
fn tc_024_generated_source_over_the_ceiling_is_refused_whole() {
    let mut builder = corpus_package();
    let codes = 10_000..13_000;
    for code in codes.clone() {
        // Both operands are `code`-keyed literals, not `reference(ENUM_MEMBER)`
        // twice over: every one of these 3,000 nodes would otherwise share
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
    let limits = quire_contract_ir::CheckedPackageReadLimits {
        bytes: 16 * 1024 * 1024,
        ..quire_contract_ir::CheckedPackageReadLimits::bounded()
    };
    let items = codes
        .map(|code| ExactScalarItem {
            node_id: code_id(code),
            operation: ExactScalarOperation::EnumComparison {
                operator: ComparisonOperator::Equal,
            },
        })
        .collect::<Vec<_>>();
    let package = builder.admit_with(limits);
    assert!(matches!(
        generate_exact_scalar_oracles(&package, &items),
        Err(ExactScalarGenerationError::SourceTooLarge { bytes })
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
        ExactScalarDisposition::Generated(_)
    ));
    let subtract = function_body(lib, &symbol(LITERAL_OPERAND));
    assert!(
        subtract.contains("rt::IntegerArithmetic::Subtract("),
        "{subtract}"
    );

    // The same shape with a text literal is refused with the literal's kind.
    // Both `INT` and `TEXT` must be reachable: Contract IR's lowering walks
    // `literal.type` on the injected text operand too, and with
    // `require_bounds` on (FR-014's own generator turns it on) an unbounded
    // `text` scalar type would refuse the request before this crate's own
    // operand-type check ever ran.
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
            vec![reference(&key(V_INTEGER)), literal("text", "3")],
        ),
        &[INT, TEXT],
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
        ExactScalarDisposition::Refused {
            refusal: ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: Some("text".to_owned()),
            }
        }
    );

    // A decimal target the runtime refuses is an invalid constant, like every
    // other generated constant.
    let add = function_body(lib, &symbol(1051));
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
fn tc_024_generated_crate_is_unpublished_pinned_charge_free_and_compiles() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{EXACT_SCALAR_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&format!("rev = \"{RUNTIME_REVISION}\"")));
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
        "generated crate did not compile against runtime {RUNTIME_REVISION}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest as _;
    format!("{:x}", sha2::Sha256::digest(bytes))
}
