//! FR-014: exact complete-V1 scalar oracle generation over admitted
//! CheckedPackage V2 input.

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_exact_scalar_oracles, BoundForm, ExactScalarDisposition, ExactScalarGenerationError,
    ExactScalarItem, ExactScalarOperation, ExactScalarOracles, ExactScalarRefusal,
    OperationProvenance, ScalarForm, UpstreamBlocker, EXACT_SCALAR_CLAIM_MAP_VERSION,
    EXACT_SCALAR_CRATE_NAME, RUNTIME_REVISION,
};
use quire_contract_ir::CheckedPackageV2;
use quire_contract_runtime::exact::{ComparisonOperator, TextProfile};
use serde_json::Value;

#[path = "exact_scalar_support/package.rs"]
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
    match dispositions(oracles).get(&key(code)) {
        Some(ExactScalarDisposition::Refused { refusal }) => refusal.clone(),
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

fn symbol(code: u32) -> String {
    format!("oracle_{}", key(code))
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
            "rt::evaluate_integer(rt::IntegerOperation::Add(".to_owned(),
        ),
        (
            1002,
            "rt::evaluate_integer(rt::IntegerOperation::Negate(".to_owned(),
        ),
        (
            1003,
            "rt::evaluate_integer(rt::IntegerOperation::Subtract(".to_owned(),
        ),
        (
            1004,
            "rt::evaluate_integer(rt::IntegerOperation::Multiply(".to_owned(),
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
            "rt::evaluate_rational(rt::RationalOperation::Add(".to_owned(),
        ),
        (
            1032,
            "rt::evaluate_rational(rt::RationalOperation::Divide(".to_owned(),
        ),
        (
            1033,
            "rt::evaluate_rational(rt::RationalOperation::IntegerDivide(".to_owned(),
        ),
        (
            1034,
            "rt::evaluate_rational(rt::RationalOperation::Subtract(".to_owned(),
        ),
        (
            1035,
            "rt::evaluate_rational(rt::RationalOperation::Multiply(".to_owned(),
        ),
        (
            1036,
            "rt::evaluate_rational(rt::RationalOperation::Negate(".to_owned(),
        ),
        (
            1041,
            "rt::OrderingOperator::Less, rt::OrderingOperands::Integer(".to_owned(),
        ),
        (
            1042,
            "rt::OrderingOperator::LessOrEqual, rt::OrderingOperands::Decimal(".to_owned(),
        ),
        (
            1043,
            "rt::OrderingOperator::Greater, rt::OrderingOperands::Rational(".to_owned(),
        ),
        (
            1044,
            "rt::OrderingOperator::LessOrEqual, rt::OrderingOperands::Integer(".to_owned(),
        ),
        (
            1045,
            "rt::OrderingOperator::GreaterOrEqual, rt::OrderingOperands::Integer(".to_owned(),
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
    assert_eq!(
        generated,
        calls.iter().map(|(code, _)| key(*code)).collect::<Vec<_>>(),
        "exactly the corpus is generated"
    );
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
        (
            COMPOSITE,
            blocked(
                COMPOSITE,
                "composite_type",
                UpstreamBlocker::QuireSpecLanguage119,
            ),
        ),
        (
            FUNCTION,
            blocked(FUNCTION, "function", UpstreamBlocker::QuireSpecLanguage119),
        ),
        (
            CALLS_FUNCTION,
            blocked(FUNCTION, "function", UpstreamBlocker::QuireSpecLanguage119),
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
            !lib.contains(&key(*code)),
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

/// Trace: FR-014-AC-5, TC-024.
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

    assert_eq!(map.blocked, [UpstreamBlocker::OperationIdentityNotCarried]);
    assert_eq!(
        json["blocked"],
        serde_json::json!(["operation identity not carried by CheckedPackage V2"])
    );
    for (claim, entry) in map
        .items
        .iter()
        .zip(json["items"].as_array().expect("items"))
    {
        assert_eq!(
            claim.operation.provenance,
            OperationProvenance::CallerDeclared {
                blocked_on: UpstreamBlocker::OperationIdentityNotCarried,
            }
        );
        assert_eq!(
            entry["operation"]["provenance"],
            serde_json::json!({
                "kind": "caller_declared",
                "blocked_on": "operation identity not carried by CheckedPackage V2",
            })
        );
    }

    let lowered = package.lower(
        &corpus()
            .iter()
            .map(|expression| code_id(expression.code))
            .collect::<Vec<_>>(),
        &quire_contract_ir::CompleteLoweringProfileV2 {
            supported_tags: [
                quire_contract_ir::CheckedNodeTag::ScalarType,
                quire_contract_ir::CheckedNodeTag::BoundedDomain,
                quire_contract_ir::CheckedNodeTag::Value,
                quire_contract_ir::CheckedNodeTag::Expression,
            ]
            .into(),
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
            .filter(|entry| entry["node_id"]["digest"] == key(expression.code))
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(source_map.len(), 1);
        assert_eq!(
            serde_json::to_value(&generated.source_map).expect("map"),
            Value::Array(source_map)
        );
    }

    // Distinct descriptors produce distinct identities.
    let identities = map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_)))
        .map(|claim| claim.operation.identity.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), corpus().len());
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

/// Trace: FR-014-AC-11, TC-024.
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

    // Node 1011 is a truncating division whose IR is byte-identical to 1012's
    // apart from its key, so the law cannot be checked: a floor descriptor
    // generates, and the claim says the law is caller-declared.
    let wire = corpus_package().wire();
    let body = |code: u32| {
        wire["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["node_id"]["digest"] == key(code))
            .map(|node| {
                let mut node = node.clone();
                node.as_object_mut().expect("node").remove("node_id");
                node
            })
            .expect("node")
    };
    assert_eq!(body(1011), body(1012));
    assert_eq!(body(1011), body(1013));
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
            blocked_on: UpstreamBlocker::OperationIdentityNotCarried,
        }
    );
    assert_eq!(
        marked.claim_map.blocked,
        [UpstreamBlocker::OperationIdentityNotCarried]
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
    builder.bounded(
        3001,
        "expression",
        "binary",
        &key(T_INTEGER),
        application("binary", arguments),
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
    assert!(!contents(&oracles, "src/lib.rs").contains(&key(3001)));
}

/// Trace: FR-014-AC-9, TC-024.
#[test]
fn tc_024_generated_source_over_the_ceiling_is_refused_whole() {
    let mut builder = corpus_package();
    let codes = 10_000..13_000;
    for code in codes.clone() {
        builder.code(
            code,
            "expression",
            "binary",
            &key(T_BOOLEAN),
            application(
                "binary",
                vec![reference(ENUM_MEMBER), reference(ENUM_MEMBER)],
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
        dispositions(&oracles)[&key(LITERAL_OPERAND)],
        ExactScalarDisposition::Generated(_)
    ));
    let subtract = function_body(lib, &symbol(LITERAL_OPERAND));
    assert!(
        subtract.contains("rt::IntegerOperation::Subtract("),
        "{subtract}"
    );

    // The same shape with a text literal is refused with the literal's kind.
    let mut builder = corpus_package();
    builder.bounded(
        3002,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            vec![reference(&key(V_INTEGER)), literal("text", "3")],
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
