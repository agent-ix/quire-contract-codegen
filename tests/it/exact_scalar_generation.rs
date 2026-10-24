//! FR-014: exact complete-V1 scalar oracle generation over admitted
//! CheckedPackage V2 input.

use ix_trace_rs::trace;
use quire_contract_codegen as scalar_fixture_types;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    process::Command,
    sync::OnceLock,
};

use crate::common::panic_scan::{comments_stripped, non_test_code, panic_tokens_in};
use crate::scratch_crate::{seed_lock, TemporaryDirectory};
use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, plan_exact_scalar_oracles, BoundForm,
    ClaimDerivationRefusal, ClaimDisposition, DecimalOperator, ExactScalarItem,
    ExactScalarOperation, ExactScalarOracles, ExactScalarRefusal, GeneratedScalarClaim,
    OperationProvenance, OracleGenerationError, RationalOperator, ScalarForm, UpstreamBlocker,
    EXACT_SCALAR_CRATE_NAME, SCALAR_LOWERING_SUPPORTED_TAGS, SCALAR_LOWERING_WORK_LIMIT,
};
use quire_contract_model::{
    derive_application_node_id, derive_structural_candidates, ApplicationNodeDraft,
    ApplicationOperator, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageRefusalCause, CheckedPackageRefusalCode, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedSemanticNodeV2, CheckedTerm, StructuralCandidateError,
};
use quire_exact::{ComparisonOperator, DivisionProfile, Integer, IntegerDomain, IntegerInterval};
use serde_json::{json, Value};

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `kani_obligations.rs` each pick independently, on the
// assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_scalar_support/package.rs"]
pub(super) mod package;

use package::*;

/// Generate, and scan the `src/lib.rs` it produced for a panic site (FR-014-AC-39). Every crate
/// this module's TC-024 tests generate comes through here, so each one is scanned.
fn generate(package: &CheckedPackageV2, items: &[ExactScalarItem]) -> ExactScalarOracles {
    let ids = fixture_ids_for(package);
    let items = resolved_items(items, &ids);
    let oracles = generate_exact_scalar_oracles(package, &items).expect("generation succeeds");
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

fn claim_id(oracles: &ExactScalarOracles, code: u32) -> quire_contract_model::CheckedNodeId {
    fixture_ids_for_digest(oracles.claim_map.package_id.digest.as_ref()).resolve(&code_id(code))
}

fn assert_refusal_with_ids(
    actual: &ExactScalarRefusal,
    expected: &ExactScalarRefusal,
    ids: &FixtureIds,
) {
    let mut expected = serde_json::to_value(expected).expect("expected refusal serializes");
    ids.resolve_value(&mut expected);
    assert_eq!(
        serde_json::to_value(actual).expect("actual refusal serializes"),
        expected
    );
}

#[derive(Debug)]
struct AdmittedRefusal {
    actual: ExactScalarRefusal,
    ids: FixtureIds,
}

impl PartialEq<ExactScalarRefusal> for AdmittedRefusal {
    fn eq(&self, expected: &ExactScalarRefusal) -> bool {
        let mut expected = serde_json::to_value(expected).expect("expected refusal serializes");
        self.ids.resolve_value(&mut expected);
        serde_json::to_value(&self.actual).expect("actual refusal serializes") == expected
    }
}

fn refusal_of(oracles: &ExactScalarOracles, code: u32) -> ExactScalarRefusal {
    let ids = fixture_ids_for_digest(oracles.claim_map.package_id.digest.as_ref());
    let node_id = ids.resolve(&code_id(code));
    match dispositions(oracles).get(node_id.digest.as_ref()) {
        Some(ClaimDisposition::Refused { refusal }) => refusal.clone(),
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

/// The generated oracle function name of node `code`, as the claim map records it.
pub(super) fn symbol(oracles: &ExactScalarOracles, code: u32) -> String {
    let ids = fixture_ids_for_digest(oracles.claim_map.package_id.digest.as_ref());
    let node_id = ids.resolve(&code_id(code));
    match dispositions(oracles).get(node_id.digest.as_ref()) {
        Some(ClaimDisposition::Generated(generated)) => generated.symbol.clone(),
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

/// The generator's current output for the whole corpus request, which
/// `exact_scalar_agreement` builds and executes.
pub(super) fn corpus_oracles() -> ExactScalarOracles {
    let (package, ids) = corpus_package().admit_resolved();
    generate(&package, &resolved_items(&golden_items(), &ids))
}

fn resolved_items(items: &[ExactScalarItem], ids: &FixtureIds) -> Vec<ExactScalarItem> {
    items
        .iter()
        .cloned()
        .map(|mut item| {
            item.node_id = ids.resolve(&item.node_id);
            item
        })
        .collect()
}

/// UNBACKED today: the six text admissions (FR-014-AC-2's text
/// admission case) are refused by Contract IR at admission, so no oracle is generated for them
/// and no agreement or generation test can run over them. `quire.op.numeric.convert` takes
/// `exact_numeric`, no catalogued `convert` identity takes `text`, and QSL lowers no text
/// admission; it is unblocked by a catalogued text-admission operation. This test pins the exact
/// refusal of each node (code, cause, pointer, locus), so the behaviour is recorded and a change
/// in either IR or the corpus shows up here.
#[trace("FR-014-AC-2", "TC-024")]
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
            "node {code}: {refusal:?}"
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

/// The aggregate operand is an explicit reader negative.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_aggregate_scalar_operand_is_refused_by_reader() {
    let (result, wire, ids) = expression_operand_package().read_resolved();
    let node_id = ids.resolve(&code_id(EXPRESSION_OPERAND));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("aggregate operand node");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("aggregate operand unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/body/arguments/0"))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// A literal's kind must agree with its declared type.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_mistyped_scalar_literal_is_refused_by_reader() {
    let (result, wire, ids) = wrong_operand_package().read_resolved();
    let node_id = ids.resolve(&code_id(WRONG_OPERAND));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("mistyped literal node");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("mistyped literal unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/arguments/1/type"
        ))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// The reader rejects a quantity-typed scalar literal.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_quantity_literal_operand_is_refused_by_reader() {
    let (result, wire, ids) = literal_quantity_package().read_resolved();
    let node_id = ids.resolve(&code_id(LITERAL_QUANTITY));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("quantity literal node");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("quantity literal unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/arguments/1/type"
        ))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// A noncanonical endpoint has no structural candidate.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_noncanonical_integer_endpoint_is_refused_by_reader() {
    let (_, mut wire, ids) = corpus_package().read_resolved();
    let bound = ids.resolve(&id(&INT5.key()));
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let position = nodes
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(bound.digest.as_ref()))
        .expect("integer bound");
    let members = nodes[position]["body"]["members"]
        .as_array_mut()
        .expect("bound members");
    let maximum = members
        .iter_mut()
        .find(|member| member["name"] == "max")
        .expect("max");
    maximum["value"]["value"] = json!("05");
    let draft: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(wire["semantic_graph"]["nodes"].clone()).expect("draft nodes");
    assert!(matches!(
        derive_structural_candidates(
            &draft,
            quire_contract_model::CheckedPackageReadLimits::bounded().bytes,
            quire_contract_model::CheckedPackageReadLimits::bounded().work,
        ),
        Err(StructuralCandidateError::InvalidClosedShape { position: refused, node_id })
            if refused == position && node_id == bound
    ));
    let result = read_mutated_wire(wire);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("noncanonical endpoint unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/node_id"))
    );
    assert_eq!(refusal.locus, Some(bound));
    assert_eq!(refusal.expected_node_id(), None);
}

/// A valid application body under a different node key
/// is refused by the reader with the original admitted key as its expected identity.
#[trace("FR-037-AC-4", "TC-052")]
#[test]
fn tc_052_reader_refuses_stale_application_key_in_admitted_positive_wire() {
    let (admitted, mut wire, ids) = bounded_increment_package().read_resolved();
    assert!(matches!(admitted, CheckedPackageV2ReadResult::Admitted(_)));
    let expected = ids.resolve(&code_id(BOUNDED_INCREMENT));
    let stale = id(&"f".repeat(64));
    assert_ne!(stale, expected);
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let position = nodes
        .iter()
        .position(|node| node["node_id"]["digest"] == expected.digest.as_ref())
        .expect("admitted increment application");
    assert_eq!(nodes[position]["body"]["term"], "application");
    assert!(nodes.iter().all(|node| node["node_id"] != json!(stale)));
    nodes[position]["node_id"] = json!(stale);
    let source_map = wire["source_map"].as_array_mut().expect("source map");
    let matching = source_map
        .iter_mut()
        .filter(|entry| entry["node_id"] == json!(expected))
        .collect::<Vec<_>>();
    assert_eq!(matching.len(), 1);
    for entry in matching {
        entry["node_id"] = json!(stale);
    }

    let result = read_mutated_wire(wire);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("stale application key unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/node_id"))
    );
    assert_eq!(refusal.locus.as_ref(), Some(&stale));
    assert_eq!(refusal.expected_node_id(), Some(&expected));
}

/// Changing a valid application literal while retaining
/// its admitted key must be refused before any fixture rekey can conceal the mismatch.
#[trace("FR-037-AC-4", "TC-052")]
#[test]
fn tc_052_reader_refuses_application_body_mutation_under_prior_key() {
    let (admitted, mut wire, ids) = bounded_increment_package().read_resolved();
    assert!(matches!(admitted, CheckedPackageV2ReadResult::Admitted(_)));
    let prior = ids.resolve(&code_id(BOUNDED_INCREMENT));
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let position = nodes
        .iter()
        .position(|node| node["node_id"]["digest"] == prior.digest.as_ref())
        .expect("admitted increment application");
    assert_eq!(nodes[position]["body"]["arguments"][1]["value"], "1");
    nodes[position]["body"]["arguments"][1]["value"] = json!("2");
    let draft: CheckedSemanticNodeV2 =
        serde_json::from_value(nodes[position].clone()).expect("mutated typed application");
    let expected = derive_application_node_id(
        ApplicationNodeDraft::new(&draft),
        None,
        CheckedPackageReadLimits::bounded().bytes,
    )
    .expect("public candidate for valid mutated body")
    .candidate_node_id()
    .clone();
    assert_ne!(expected, prior);

    let result = read_mutated_wire(wire);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("application body mutation unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/node_id"))
    );
    assert_eq!(refusal.locus.as_ref(), Some(&prior));
    assert_eq!(refusal.expected_node_id(), Some(&expected));
}

/// A mismatched decimal mode is reader-ineligible.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_mismatched_decimal_mode_is_refused_by_reader() {
    let (_, mut wire, ids) = corpus_package().read_resolved();
    let node_id = ids.resolve(&code_id(1051));
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let position = nodes
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("decimal add");
    nodes[position]["body"]["operation"]["mode"]["value"] = json!("toward-zero");
    let (result, candidate) = read_application_mutated_wire(wire, &node_id);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("decimal mode mismatch unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperationModeTypeMismatch)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/operation/mode/value"
        ))
    );
    assert_eq!(refusal.locus, Some(candidate));
}

/// The corpus crate carries its manifest, source, coverage map and claim map;
/// the claim map on disk is the serialized in-memory claim map; and every
/// generated claim's oracle symbol is defined in `src/lib.rs`.
#[trace("FR-014-AC-4", "TC-024")]
#[test]
fn tc_024_corpus_crate_artifacts_are_complete_and_self_consistent() {
    let (package, ids) = corpus_package().admit_resolved();
    let items = resolved_items(&golden_items(), &ids);
    let oracles = generate(&package, &items);
    let paths = oracles
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        paths,
        BTreeSet::from([
            "Cargo.toml",
            "claim-map.json",
            "source-maps/lib.json",
            "src/lib.rs"
        ])
    );
    let on_disk: Value = serde_json::from_str(contents(&oracles, "claim-map.json")).unwrap();
    assert_eq!(on_disk, serde_json::to_value(&oracles.claim_map).unwrap());
    let lib = contents(&oracles, "src/lib.rs");
    let mut generated = BTreeSet::new();
    for claim in &oracles.claim_map.items {
        if let ClaimDisposition::Generated(generated_claim) = &claim.result {
            let symbol = format!("pub fn {}(", generated_claim.symbol);
            assert!(lib.contains(&symbol), "{symbol} is not defined");
            generated.insert(claim.node_id.clone());
        }
    }
    let expected = corpus()
        .iter()
        .map(|expression| ids.resolve(&code_id(expression.code)))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        generated, expected,
        "every admitted expression generates exactly once"
    );
}

#[trace("FR-014-AC-4", "TC-024")]
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

#[trace("FR-014-AC-1", "FR-014-AC-2", "TC-024")]
#[trace("TC-001", "NFR-002-AC-1", "NFR-002-AC-2")]
#[test]
fn tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    let subjects = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
        .map(|claim| format!("{}/{}", claim.node_id.domain, claim.node_id.digest))
        .collect::<Vec<String>>();
    let subject_refs = subjects.iter().map(String::as_str).collect::<Vec<_>>();
    crate::generated_headers::assert_header("src/lib.rs", lib, &subject_refs);
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
            "rt::divide(rt::DivisionProfile::Truncating, rt::DivisionMember::Quotient,".to_owned(),
        ),
        (
            1012,
            "rt::divide(rt::DivisionProfile::Floor, rt::DivisionMember::Quotient,".to_owned(),
        ),
        (
            1013,
            "rt::divide(rt::DivisionProfile::Euclidean, rt::DivisionMember::Quotient,".to_owned(),
        ),
        (
            1014,
            "rt::divide(rt::DivisionProfile::Truncating, rt::DivisionMember::Quotient,".to_owned(),
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
        (1140, "quantity_owner::convert_quantity(".to_owned()),
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
            "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::Add("
                .to_owned(),
        ),
        (
            1082,
            "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::Multiply("
                .to_owned(),
        ),
        (
            1083,
            "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::Power("
                .to_owned(),
        ),
        (
            1088,
            "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::Subtract("
                .to_owned(),
        ),
        (
            1089,
            "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::Divide("
                .to_owned(),
        ),
    ];
    for (index, operator) in ComparisonOperator::ALL.into_iter().enumerate() {
        calls.push((
            TEXT_COMPARISONS[index],
            format!("rt::compare_text(rt::ComparisonOperator::{operator:?},"),
        ));
        calls.push((
            ENUM_COMPARISONS[index],
            format!("enum_owner::compare_enum(rt::ComparisonOperator::{operator:?},"),
        ));
        calls.push((
            QUANTITY_COMPARISONS[index],
            format!("quantity_owner::compare_quantity(rt::ComparisonOperator::{operator:?},"),
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
        .map(|(code, _)| claim_id(&oracles, *code).digest.to_string())
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

#[trace("FR-014-AC-1", "FR-014-AC-3", "FR-014-AC-7", "FR-014-AC-10", "TC-024")]
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
                unbounded_type: resolve(T_INTEGER),
            },
        ),
        (
            MATHEMATICAL,
            ExactScalarRefusal::BoundMismatch {
                bound: ids.resolve(&id(&INT.key())),
                form: BoundForm::IntegerRange,
            },
        ),
        (
            AMBIGUOUS,
            ExactScalarRefusal::AmbiguousBound {
                bounded_type: resolve(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
        ),
        (
            WRONG_BOUND_FORM,
            ExactScalarRefusal::MissingBound {
                bounded_type: resolve(T_INTEGER),
                expected_form: BoundForm::IntegerRange,
            },
        ),
        (
            DOMAIN_MISMATCH,
            ExactScalarRefusal::BoundMismatch {
                bound: ids.resolve(&id(&INT5.key())),
                form: BoundForm::IntegerRange,
            },
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
    let expected_identity = |items: &[ExactScalarItem]| {
        let mut roots = items
            .iter()
            .map(|item| item.node_id.clone())
            .collect::<Vec<_>>();
        roots.sort();
        roots.dedup();
        let profile = quire_contract_model::CompleteLoweringProfileV2 {
            reference: quire_contract_model::LoweringProfileRef {
                authority: "agent-ix".into(),
                identity: "quire.codegen.exact-scalar/v1".into(),
            },
            supported_tags: BTreeSet::from(SCALAR_LOWERING_SUPPORTED_TAGS),
            require_bounds: true,
            work_limit: SCALAR_LOWERING_WORK_LIMIT,
        };
        package
            .lower(&roots, &profile)
            .package
            .wire()
            .expect("direct scalar roots lower")
            .identity()
            .clone()
    };
    let full_identity = expected_identity(&items);
    let alone_identity = expected_identity(&generated_only);
    assert_ne!(
        full_identity, alone_identity,
        "request roots bind package identity"
    );
    let full_claims = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .collect::<Vec<_>>();
    let alone_claims = alone
        .claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .collect::<Vec<_>>();
    assert_eq!(alone_claims.len(), full_claims.len());
    for (alone, full) in alone_claims.iter().zip(&full_claims) {
        assert_eq!(alone.node_id, full.node_id);
        assert_eq!(alone.operation, full.operation);
        let (ClaimDisposition::Generated(left), ClaimDisposition::Generated(right)) =
            (&alone.result, &full.result)
        else {
            panic!("selected claims are generated");
        };
        assert_eq!(left.contract_package_identity, alone_identity);
        assert_eq!(right.contract_package_identity, full_identity);
        assert_eq!(left.symbol, right.symbol);
        assert_eq!(left.package_id, right.package_id);
        assert_eq!(left.semantic_form, right.semantic_form);
        assert_eq!(left.semantic_type, right.semantic_type);
        assert_eq!(left.source_map, right.source_map);
        assert_eq!(left.claims, right.claims);
        assert_eq!(left.bounds, right.bounds);
        assert_eq!(left.checked_bounds, right.checked_bounds);
        assert_eq!(left.dependencies, right.dependencies);
        assert_eq!(left.oracle_source, right.oracle_source);
    }

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
#[trace("FR-014-AC-5", "FR-014-AC-11", "FR-014-AC-14", "TC-024")]
#[test]
fn tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item() {
    let wire = corpus_package().wire();
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let fixture_ids = fixture_ids_for(&package);
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

    let profile = quire_contract_model::CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.exact-scalar/v1".into(),
        },
        supported_tags: claim_supported_tags,
        require_bounds: true,
        work_limit: SCALAR_LOWERING_WORK_LIMIT,
    };
    let mut complete_request = golden_items()
        .iter()
        .map(|item| fixture_ids.resolve(&item.node_id))
        .collect::<Vec<_>>();
    complete_request.sort();
    complete_request.dedup();
    let expected_package_identity = package
        .lower(&complete_request, &profile)
        .package
        .wire()
        .expect("full direct request lowers")
        .identity()
        .clone();
    let lowered = package.lower(
        &corpus()
            .iter()
            .map(|expression| fixture_ids.resolve(&code_id(expression.code)))
            .collect::<Vec<_>>(),
        &profile,
    );
    for (expression, record) in corpus().iter().zip(&lowered.records) {
        let quire_contract_model::CompleteLoweringRecordV2::Lowered { node } = record else {
            panic!("corpus node {} lowers", expression.code);
        };
        let claim = map
            .items
            .iter()
            .find(|claim| claim.node_id == fixture_ids.resolve(&code_id(expression.code)))
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
        assert_eq!(generated.node_id, claim.node_id);
        assert_eq!(generated.package_id, *package.package_id());
        assert_eq!(
            generated.contract_package_identity,
            expected_package_identity
        );
        assert!(serde_json::to_value(generated)
            .expect("generated claim")
            .get("ir_id")
            .is_none());
        assert_eq!(generated.semantic_form, expression.form);
        assert_eq!(generated.semantic_type, node.semantic_type);
        assert_eq!(generated.claims, node.claims);
        assert_eq!(generated.bounds, node.bounds);
        // Every checked bound is one the expression declares and reaches.
        let declared = expression
            .bounds
            .iter()
            .map(|bound| fixture_ids.resolve(&id(&bound.key())))
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
            .filter(|entry| {
                entry["node_id"]["digest"] == *fixture_ids.resolve(&code_id(expression.code)).digest
            })
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
            .find(|claim| claim.node_id == fixture_ids.resolve(&code_id(code)))
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
            | ExactScalarOperation::QuantityConversion
    )
}

#[trace("FR-014-AC-4", "TC-024")]
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

/// Node 1011 is a truncating division whose own `operation.laws` does not name the floor
/// definition. A floor descriptor implies the same catalogued identity (`quire.op.integer.div`),
/// so this is not AC-12's different-operation case, and the item's disposition is `Generated`, so
/// it is neither half of AC-14's never-inspected-or-refused case. It is the third state: the item
/// generates the oracle its descriptor
/// names and is marked `caller_declared` because the law disagrees. Deleting the law check would
/// leave AC-11, AC-12 and AC-14 satisfied and this one violated.
#[trace("FR-014-AC-13", "TC-024")]
#[test]
fn tc_024_a_mislabelled_descriptor_is_refused_where_bounds_disagree_and_marked_otherwise() {
    let package = corpus_package().admit();
    let floor = |lower, upper| ExactScalarOperation::IntegerDivision {
        profile: DivisionProfile::Floor,
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
                bound: fixture_ids_for(&package).resolve(&id(&INT5.key())),
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

/// `Shape::of` gives `Add`, `Subtract`, `Multiply`, `IntegerDivision` and `IntegerModulo` the
/// identical binary `&rt::Integer` shape, so `check_item`'s shape checks alone cannot distinguish
/// an `Add` descriptor from a `Multiply` one over the same bounds. Node 1004 is
/// `quire.op.integer.mul` over `[-1000,1000]`; naming `Add` against it, with the identical
/// `[-1000,1000]` domain, passes every shape and bound check `check_item` makes. Confirming it
/// anyway -- the defect this ticket exists to close -- would render an oracle that adds where the
/// catalog says multiply and a harness whose `operation_identity` says `mul`.
///
///
/// Backs AC-12: an `Add` descriptor against node 1004's catalogued `quire.op.integer.mul`, over
/// the identical `[-1000,1000]` bound and the identical binary `&rt::Integer` shape, is not
/// confirmed. The mismatched case is asserted first; the agreeing case second, which is AC-11
/// for that one node. AC-11 over the whole corpus is discharged by
/// `tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item`.
#[trace("FR-014-AC-12", "FR-038-AC-3", "TC-024")]
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
    let mismatched_source = contents(&mismatched, "src/lib.rs");
    let mismatched_body = function_body(mismatched_source, &symbol(&mismatched, 1004));
    assert!(mismatched_body.contains("rt::IntegerArithmetic::Add("));
    assert!(!mismatched_body.contains("rt::IntegerArithmetic::Multiply("));

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
    let matched_source = contents(&matched, "src/lib.rs");
    let matched_body = function_body(matched_source, &symbol(&matched, 1004));
    assert!(matched_body.contains("rt::IntegerArithmetic::Multiply("));
    assert!(!matched_body.contains("rt::IntegerArithmetic::Add("));
}

/// Wrong arity and result form/target refuse before source is emitted, while binary operand
/// order and the descriptor-selected runtime callee remain visible in generated code.
#[trace("FR-038-AC-3", "FR-014-AC-8", "FR-014-AC-13", "FR-014-AC-18", "TC-024")]
#[test]
fn tc_024_catalog_consumer_negative_cases_refuse_wrong_shapes_and_preserve_order_and_target() {
    let (package, _) = corpus_package().admit_resolved();

    let subtraction = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(1003),
            operation: ExactScalarOperation::IntegerArithmetic {
                operator: IntegerOperator::Subtract,
                domain: bounded(-1000, 1000),
            },
        }],
    );
    let subtraction_source = contents(&subtraction, "src/lib.rs");
    let subtraction_body = function_body(subtraction_source, &symbol(&subtraction, 1003));
    assert!(subtraction_body.contains("Subtract(left, right)"));
    assert!(!subtraction_body.contains("Subtract(right, left)"));

    let ordering = derive_one(&package, 1041).expect("integer less-than descriptor");
    let ordering_oracles = generate(&package, &[ordering]);
    let ordering_source = contents(&ordering_oracles, "src/lib.rs");
    let ordering_body = function_body(ordering_source, &symbol(&ordering_oracles, 1041));
    assert!(ordering_body.contains(
        "rt::order_numbers(rt::OrderingOperator::Less, rt::OrderedOperands::Integers(left, right), meter)"
    ));
    assert!(!ordering_body.contains(
        "rt::order_numbers(rt::OrderingOperator::Less, rt::OrderedOperands::Integers(right, left), meter)"
    ));

    let golden = generate(&package, &golden_items());
    assert_eq!(
        refusal_of(&golden, WRONG_ARITY),
        ExactScalarRefusal::FormMismatch {
            expected: "unary",
            found: "binary".to_owned(),
        }
    );
    assert_eq!(
        refusal_of(&golden, WRONG_RESULT),
        ExactScalarRefusal::ResultTypeMismatch {
            expected: ScalarForm::Decimal,
            found: Some("integer".to_owned()),
        }
    );
    let golden_source = contents(&golden, "src/lib.rs");
    assert!(!golden_source.contains(code_id(WRONG_ARITY).digest.as_ref()));
    assert!(!golden_source.contains(code_id(WRONG_RESULT).digest.as_ref()));
}

/// The scalar nodes shared by the admitted work-exhaustion fixture and its
/// reader-negative aggregate-operand control.
fn minimal_integer_builder() -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    builder.code(
        T_INTEGER,
        "scalar_type",
        "integer",
        &key(T_INTEGER),
        json!({"term": "aggregate", "members": []}),
    );
    let bound_key = builder.bound(&INT);
    builder.code(
        V_INTEGER,
        "value",
        "literal",
        &bound_key,
        literal("integer", "3"),
    );
    builder
}

/// A binary Integer application whose first referenced value holds a wide
/// aggregate body. The reader can type both arguments as Integer, and lowering
/// visits the aggregate's terms and successor edges through that reference.
fn work_exhausted_builder(references: usize) -> PackageBuilder {
    let body = json!({
        "term": "aggregate",
        "members": (0..references)
            .map(|_| reference(&key(V_INTEGER)))
            .collect::<Vec<_>>(),
    });
    let mut builder = minimal_integer_builder();
    builder.code(3003, "value", "literal", &key(T_INTEGER), body);
    builder.application_code_with(
        3001,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.add"),
            &key(T_INTEGER),
            vec![reference(&key(3003)), reference(&key(V_INTEGER))],
        ),
        &[key(3003), key(V_INTEGER)],
    );
    builder
}

/// The stress shape reaches the reader before
/// the lowering work ceiling is applied.
#[trace("FR-014-AC-15", "TC-024")]
#[test]
fn tc_024_work_exhaustion_shape_is_reader_admitted_at_small_size() {
    let (result, _, _) = work_exhausted_builder(2).read_resolved();
    assert!(
        matches!(result, CheckedPackageV2ReadResult::Admitted(_)),
        "small work-exhaustion shape was refused: {result:?}"
    );
}

/// A binary integer operator cannot take an
/// aggregate directly in an operand slot.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_aggregate_integer_operand_is_refused_by_reader() {
    let mut builder = minimal_integer_builder();
    builder.application_code_with(
        3004,
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
                    "term": "aggregate",
                    "members": [reference(&key(V_INTEGER)), reference(&key(V_INTEGER))],
                }),
            ],
        ),
        &[key(V_INTEGER)],
    );
    let (result, wire, ids) = builder.read_resolved();
    let node_id = ids.resolve(&code_id(3004));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("aggregate operand node");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("aggregate operand unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/body/arguments/1"))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// A package/item pair whose lone application (code 3001) exceeds
/// `SCALAR_LOWERING_WORK_LIMIT` during lowering. Each reference term costs a
/// body term and a successor edge, so 40,000 of them exceed the 65,536 units
/// scalar lowering allows. Both stress tests construct the same minimal shape.
fn work_exhausted_fixture() -> (CheckedPackageV2, ExactScalarItem) {
    let builder = work_exhausted_builder(40_000);
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

#[trace("FR-014-AC-3", "FR-014-AC-15", "TC-024")]
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

/// A call whose lowered contract package is longer than the checked package, read under a ceiling
/// that admits the checked package and is one byte below that lowered package, fails every
/// requested record for bytes (Contract IR FR-038-AC-95): every item is refused as
/// `LoweringByteLimitExceeded` with the ceiling as `limit` and one shared `consumed` above it,
/// never as `LoweringWorkExhausted`, and no function is generated. The per-node case and each
/// other limit kind are asserted on hand-built records in the module's own tests.
#[trace("FR-014-AC-40", "TC-024")]
#[test]
fn tc_024_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    use sha2::{Digest, Sha256};

    const REQUESTS: u32 = 16;
    let (builder, source, sites) = source_backed_byte_chain(REQUESTS);
    let raw = builder.raw_wire();
    let source_ref = raw["lock"]["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .find(|item| item["identity"] == "ir764-ac40-source")
        .expect("authored source is locked");
    assert_eq!(
        source_ref["digest"],
        format!("{:x}", Sha256::digest(source.as_bytes()))
    );
    for (code, spans) in &sites {
        let entries = raw["source_map"]
            .as_array()
            .expect("source map")
            .iter()
            .filter(|entry| entry["node_id"]["digest"] == code_id(*code).digest.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), spans.len());
        for (ordinal, (entry, (start, end))) in entries.iter().zip(spans).enumerate() {
            assert_eq!(entry["ordinal"], ordinal);
            assert_eq!(entry["regions"][0]["source"], *source_ref);
            assert_eq!(entry["regions"][0]["start"], *start);
            assert_eq!(entry["regions"][0]["end"], *end);
            assert_eq!(&source[*start..*end], format!("n{code}"));
        }
    }
    let (measured, ids) = builder.admit_with_resolved(limits_under(LARGEST_CEILING));
    // Every source-backed addition is independently requested; the shared lowering must admit
    // all of them and make one byte-refusal charge visible to every item.
    let requested = (0..REQUESTS)
        .map(|offset| ids.resolve(&code_id(BYTE_CHAIN_BASE + offset)))
        .collect::<Vec<_>>();
    let items = requested
        .iter()
        .map(|node_id| ExactScalarItem {
            node_id: node_id.clone(),
            operation: integer_add_0_9(),
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
        &measured,
        &requested,
        &measuring_profile(true, "quire.codegen.exact-scalar/v1"),
    );
    let ceiling = lowered_length - 1;
    assert!(
        ceiling >= checked_length,
        "the lowered package ({lowered_length} bytes) is longer than the checked package \
         ({checked_length} bytes), so a ceiling one byte below it admits the checked package"
    );
    // At the lowered package's own length no record fails for bytes: the ceiling is exact.
    let exact = generate_under(lowered_length);
    let mut exact_nodes = exact
        .claim_map
        .items
        .iter()
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    exact_nodes.sort();
    let mut expected_nodes = requested.clone();
    expected_nodes.sort();
    assert_eq!(exact_nodes, expected_nodes);
    assert!(exact
        .claim_map
        .items
        .iter()
        .all(|claim| byte_refusal(&claim.result).is_none()));
    assert!(exact
        .claim_map
        .items
        .iter()
        .all(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));

    let oracles = generate_under(ceiling);
    let mut refused_nodes = oracles
        .claim_map
        .items
        .iter()
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    refused_nodes.sort();
    assert_eq!(refused_nodes, expected_nodes);
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
        ExactScalarRefusal::PackageAccounting { .. } => "PackageAccounting",
        ExactScalarRefusal::SelectedDependencyUnsupported { .. } => "SelectedDependencyUnsupported",
        ExactScalarRefusal::DuplicateRequest => "DuplicateRequest",
        ExactScalarRefusal::InvalidInput => "InvalidInput",
        ExactScalarRefusal::Unsupported { .. } => "Unsupported",
        ExactScalarRefusal::BlockedOnUpstream { .. } => "BlockedOnUpstream",
        ExactScalarRefusal::RequiresBound { .. } => "RequiresBound",
        ExactScalarRefusal::MissingBound { .. } => "MissingBound",
        ExactScalarRefusal::IncompleteContext => "IncompleteContext",
        ExactScalarRefusal::AmbiguousBound { .. } => "AmbiguousBound",
        ExactScalarRefusal::UnreadableBound { .. } => "UnreadableBound",
        ExactScalarRefusal::BoundMismatch { .. } => "BoundMismatch",
        ExactScalarRefusal::OperandUnsupported { .. } => "OperandUnsupported",
        ExactScalarRefusal::OperandRangeLimitExceeded { .. } => "OperandRangeLimitExceeded",
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
        ExactScalarRefusal::BooleanRightOperandMismatch { .. } => "BooleanRightOperandMismatch",
        ExactScalarRefusal::MissingOperationIdentity { .. } => "MissingOperationIdentity",
        ExactScalarRefusal::NoDerivableClaim { .. } => "NoDerivableClaim",
    }
}

/// One fixture per `ExactScalarRefusal` variant, each checked against
/// `refusal_variant_name`'s exhaustive match (see its own doc). What this
/// test adds is not novel exhaustiveness protection for `ExactScalarRefusal`
/// -- `src/kani/generate/negotiate.rs` already has its own wildcard-free match over
/// the same enum in production code, so a twenty-first variant would already
/// break that build today whether or not this test exists. What this test
/// adds is the variant<->fixture pairing: proof that every variant names a
/// real, driven scenario (or, for the four noted below, a scenario
/// deliberately undrivable), not just a name `refusal_variant_name` happens
/// to mention.
///
/// Sixteen variants are driven through the real admitted-package generation
/// pipeline -- fifteen share the golden corpus's one `generate` call (see
/// `tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged`,
/// which asserts the full typed reason for each of these same codes; this
/// test only needs the variant, not the reason's payload), and
/// `LoweringWorkExhausted` needs `work_exhausted_fixture`'s own package.
///
/// Four variants are documented, on both sides of the crate boundary, as
/// unreachable through any package this crate's public API can admit, so no
/// admitted-package fixture for them exists to drive:
/// `MissingOperationIdentity` (`catalogued_operation_identity`'s own doc:
/// "this generator should never receive a node for which this member is
/// absent" -- quire-contract-ir's `validate_operations` guarantees it before
/// admission) and `InvalidBody`/`BodyIncomplete` (quire-contract-ir's own
/// `CompleteLoweringRecordV2` doc: "A package the V2 reader admitted never
/// yields this" / "Like `InvalidBody`, an admitted package never yields
/// this"). `UnreadableBound` is unreachable because Contract IR's strict
/// reader rejects the noncanonical endpoint before CG sees the package;
/// `tc_024_unreadable_integer_bound_is_a_strict_reader_refusal` asserts that
/// public boundary. Real function-path coverage of the other three exists as
/// unit tests inside the crate that call the real private functions this
/// integration-test crate cannot reach:
/// `tc_024_invalid_and_incomplete_bodies_are_typed_refusals` drives
/// `InvalidBody`/`BodyIncomplete` through the real `lowered()`, and
/// `tc_024_catalogued_operation_identity_refuses_a_node_whose_operation_has_no_identity`
/// drives `MissingOperationIdentity` through the real
/// `catalogued_operation_identity` (both in `src/oracle/scalar/mod.rs`, both
/// private to `quire_contract_codegen` and so unreachable from here). The
/// four assertions below construct each variant directly instead; they only
/// exercise `refusal_variant_name`'s own match arms for these four variants
/// -- they are not evidence that production code produces them, which is the
/// two unit tests' job.
#[trace("FR-014-AC-1", "FR-014-AC-3", "FR-014-AC-11", "FR-014-AC-14", "TC-024")]
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
        (WRONG_BOUND_FORM, "MissingBound"),
        (AMBIGUOUS, "AmbiguousBound"),
        (MATHEMATICAL, "BoundMismatch"),
        (V_BOOLEAN, "NotExpression"),
        (WRONG_ARITY, "FormMismatch"),
        (WRONG_BODY, "BodyMismatch"),
        (WRONG_RESULT, "ResultTypeMismatch"),
        (WRONG_OPERAND_REFERENCE, "OperandTypeMismatch"),
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
    assert_eq!(
        refusal_variant_name(&ExactScalarRefusal::UnreadableBound {
            bound: id(&unreadable_bound().key()),
        }),
        "UnreadableBound"
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

/// An integer-range endpoint spelled `05` is refused by the authoritative
/// checked-package reader before a CG claim can exist.
#[trace("FR-014-AC-10", "TC-024")]
#[test]
fn tc_024_unreadable_integer_bound_is_a_strict_reader_refusal() {
    let mut builder = PackageBuilder::default();
    builder.code(
        T_INTEGER,
        "scalar_type",
        "integer",
        &key(T_INTEGER),
        json!({"term": "aggregate", "members": []}),
    );
    let unreadable = unreadable_bound();
    builder.bound(&unreadable);
    let (outcome, _, _) = builder.read_resolved();
    let CheckedPackageV2ReadResult::Refused(refusal) = outcome else {
        panic!("noncanonical integer endpoint must not admit");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(refusal.locus, Some(id(&unreadable.key())));
    assert!(refusal.expected_node_id().is_none());
}

/// A non-exact IEEE rounding mode over bare Float32 is refused by the reader
/// before CG can classify the missing rounding bound.
#[trace("FR-014-AC-10", "TC-024")]
#[test]
fn tc_024_missing_rounding_mode_is_a_strict_reader_refusal() {
    let (outcome, wire, ids) = missing_rounding_package().read_resolved();
    let node = ids.resolve(&code_id(MISSING_ROUNDING));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture nodes")
        .iter()
        .position(|item| item["node_id"] == serde_json::to_value(&node).expect("node id"))
        .expect("authored addition");
    let CheckedPackageV2ReadResult::Refused(refusal) = outcome else {
        panic!("bare Float32 with non-exact mode must not admit");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperationModeTypeMismatch)
    );
    assert_eq!(refusal.locus, Some(node));
    let expected_path = format!("/semantic_graph/nodes/{position}/body/operation/mode/value");
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str()),
        Some(expected_path.as_str())
    );
    assert!(refusal.expected_node_id().is_none());
}

/// The corpus Integer is the anonymous structural authority required by the reader.
#[trace("FR-014-AC-10", "TC-024")]
#[test]
fn tc_024_anonymous_integer_range_generates_one_addition() {
    const OPERAND: u32 = 39_011;
    const ADD: u32 = 39_012;
    let mut builder = PackageBuilder::default();
    builder.code(
        T_INTEGER,
        "scalar_type",
        "integer",
        &key(T_INTEGER),
        json!({"term": "aggregate", "members": []}),
    );
    let integer = key(T_INTEGER);
    let typed_literal = |value: &str| {
        json!({
            "term": "literal",
            "type": id(&integer),
            "value_kind": "integer",
            "value": value,
        })
    };
    builder.code(OPERAND, "value", "literal", &integer, typed_literal("3"));
    builder.application_bounded(
        ADD,
        "expression",
        "binary",
        &integer,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer,
            vec![reference(&key(OPERAND)), typed_literal("2")],
        ),
        &[Bound::Integer(0, 9)],
    );

    let (checked, ids) = builder.admit_resolved();
    let operation = ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(0, 9),
    };
    let item = ExactScalarItem {
        node_id: ids.resolve(&code_id(ADD)),
        operation,
    };
    let oracles = generate(&checked, &[item]);
    assert!(matches!(
        &oracles.claim_map.items[0].result,
        ClaimDisposition::Generated(_)
    ));
    assert!(contents(&oracles, "src/lib.rs").contains("rt::IntegerArithmetic::Add"));
}

#[trace("FR-014-AC-10", "TC-024")]
#[test]
fn tc_024_corpus_reader_admits_anonymous_integer_ranges() {
    let (outcome, _, ids) = corpus_package().read_resolved();
    let CheckedPackageV2ReadResult::Admitted(package) = outcome else {
        panic!("corpus must admit before CG generation: {outcome:?}");
    };
    let anonymous = ids.resolve(&id(&key(T_INTEGER)));
    let int5 = ids.resolve(&id(&INT5.key()));
    let range = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == int5)
        .expect("admitted INT5");
    assert_eq!(range.semantic_type, anonymous);

    let rounded = ids.resolve(&code_id(1053));
    let result_type = ids.resolve(&code_id(T_DECIMAL_AWAY));
    let conversion = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == rounded)
        .expect("admitted nearest-away decimal conversion");
    assert_eq!(conversion.semantic_type, result_type);
    let CheckedTerm::Application {
        result_type: body_type,
        operation,
        ..
    } = &conversion.body
    else {
        panic!("admitted conversion is an application");
    };
    assert_eq!(body_type, &result_type);
    assert_eq!(
        operation["mode"],
        json!({"kind": "rounding", "value": "nearest-away"})
    );
}

#[trace("FR-014-AC-9", "TC-024")]
#[test]
fn tc_024_generated_source_over_the_ceiling_is_refused_whole() {
    // One canonical range is shared by every admitted application. The range is represented
    // once in the wire but its endpoints appear in each emitted operation identity and body.
    let lower = format!("-{}", "9".repeat(2_500));
    let upper = "9".repeat(2_500);
    let interval = IntegerInterval::new(
        lower.parse::<Integer>().expect("canonical lower"),
        upper.parse::<Integer>().expect("canonical upper"),
    )
    .expect("nonempty interval");
    let bound = Bound::Raw {
        form: "integer_range",
        bounded: "integer",
        body: json!({
            "term": "aggregate",
            "members": [
                member("min", literal("integer", &lower)),
                member("max", literal("integer", &upper)),
            ],
        }),
        foreign: vec![],
    };
    let mut builder = PackageBuilder::default();
    builder.code(
        T_INTEGER,
        "scalar_type",
        "integer",
        &key(T_INTEGER),
        json!({"term": "aggregate", "members": []}),
    );
    let bound_key = builder.bound(&bound);
    builder.code(
        V_INTEGER,
        "value",
        "literal",
        &bound_key,
        literal("integer", "3"),
    );
    let codes = 10_000..10_205;
    for code in codes.clone() {
        builder.application_code_with(
            code,
            "expression",
            "binary",
            &key(T_INTEGER),
            application(
                "binary",
                op("quire.op.integer.add"),
                &key(T_INTEGER),
                vec![
                    reference(&key(V_INTEGER)),
                    literal("integer", &code.to_string()),
                ],
            ),
            &[key(V_INTEGER)],
        );
    }
    let (result, wire, ids) = builder.read_resolved();
    let CheckedPackageV2ReadResult::Admitted(package) = result else {
        panic!("compact source-ceiling fixture refused: {result:?}");
    };
    let wire_bytes = serde_json::to_vec(&wire).expect("fixture wire").len();
    assert!(
        wire_bytes <= quire_contract_model::CheckedPackageReadLimits::bounded().bytes as usize,
        "fixture wire {wire_bytes} exceeds bounded reader limit"
    );
    let items = codes
        .map(|code| ExactScalarItem {
            node_id: ids.resolve(&code_id(code)),
            operation: ExactScalarOperation::IntegerArithmetic {
                operator: IntegerOperator::Add,
                domain: IntegerDomain::Bounded(interval.clone()),
            },
        })
        .collect::<Vec<_>>();
    assert!(matches!(
        generate_exact_scalar_oracles(&package, &items),
        Err(OracleGenerationError::SourceTooLarge { bytes, .. })
            if bytes > quire_contract_codegen::MAX_GENERATED_SOURCE_BYTES
    ));
}

#[trace("FR-014-AC-3", "FR-014-AC-9", "TC-024")]
#[test]
fn tc_024_literal_operands_are_classified_by_value_kind_and_constants_stop_typed() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    // Node 1003's right operand is an integer literal, so it type-checks as an
    // integer and the subtraction generates like any other.
    assert!(matches!(
        dispositions(&oracles)[claim_id(&oracles, LITERAL_OPERAND).digest.as_ref()],
        ClaimDisposition::Generated(_)
    ));
    let subtract = function_body(lib, &symbol(&oracles, LITERAL_OPERAND));
    assert!(
        subtract.contains("rt::IntegerArithmetic::Subtract("),
        "{subtract}"
    );

    // The reader refuses the same shape with a text literal at the operand's type.
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
    let (result, wire, ids) = builder.read_resolved();
    let node_id = ids.resolve(&code_id(3002));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("text literal node");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("text literal unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/arguments/1/type"
        ))
    );
    assert_eq!(refusal.locus, Some(node_id));

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

#[trace("FR-014-AC-8", "FR-014-AC-9", "TC-024")]
#[test]
fn tc_024_generated_crate_is_unpublished_charge_free_and_compiles() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{EXACT_SCALAR_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains("features = [\"exact\"]"));
    assert!(!manifest.contains("test-support"));
    assert!(!manifest.contains("[[example]]"));

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
    fs::create_dir_all(directory.path.join("src")).unwrap();
    for artifact in &oracles.artifacts {
        fs::create_dir_all(
            directory
                .path
                .join(&artifact.path)
                .parent()
                .expect("artifact parent"),
        )
        .unwrap();
        fs::write(directory.path.join(&artifact.path), &artifact.contents).unwrap();
    }
    seed_lock(&directory.path);
    let output = Command::new(env!("CARGO"))
        .args(["build", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", directory.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&directory.path)
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
#[trace("FR-014-AC-17", "TC-024")]
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
        dispositions(&oracles).get(claim_id(&oracles, BOUNDED_INCREMENT).digest.as_ref()),
        Some(ClaimDisposition::Generated(_))
    ));
}

fn add_over(low: i64, high: i64) -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(low, high),
    }
}

/// An admitted direct temporal formula request keeps its node
/// identity and settles as a typed unsupported refusal without producing oracle code.
#[trace("FR-014-AC-44", "TC-024")]
#[test]
fn tc_024_ac44_direct_temporal_formula_is_unsupported() {
    let package = qspec_package("positive-all-families.json");
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "formula"
                && matches!(
                    &node.body,
                    CheckedTerm::Application {
                        operator: ApplicationOperator::TemporalFormula,
                        ..
                    }
                )
        })
        .expect("QSpec fixture has an admitted temporal_formula root");
    let requested_id = node.node_id.clone();
    let oracles = generate_exact_scalar_oracles(
        &package,
        &[ExactScalarItem {
            node_id: requested_id.clone(),
            operation: add_over(-100, 100),
        }],
    )
    .expect("direct unsupported request settles in claim map");
    assert_eq!(oracles.claim_map.items.len(), 1);
    let claim = &oracles.claim_map.items[0];
    assert_eq!(claim.node_id, requested_id);
    assert!(
        matches!(
            &claim.result,
            ClaimDisposition::Refused { refusal: ExactScalarRefusal::Unsupported { unsupported_node_id, node_tag: "temporal" } }
                if unsupported_node_id == &requested_id
        ),
        "expected direct typed temporal refusal, got {:?}",
        claim.result
    );
    assert!(!oracles
        .claim_map
        .items
        .iter()
        .any(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));
}

fn qspec_package(fixture: &str) -> CheckedPackageV2 {
    let root =
        PathBuf::from(std::env::var_os("QSPEC_DIR").expect("QSPEC_DIR points at published QSpec"));
    let bytes = fs::read(
        root.join("proposals/checked-package-v2/fixtures")
            .join(fixture),
    )
    .expect("read authoritative QSpec fixture");
    let wire: Value = serde_json::from_slice(&bytes).expect("QSpec fixture JSON");
    let canonical = serde_json::to_vec(&wire).expect("canonical QSpec fixture bytes");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    crate::checked_package_evidence::retain_selected_extensions(&root, &wire, &mut evidence);
    if let Some(selection) = wire["lock"]["model_selections"]
        .as_array()
        .and_then(|rows| rows.first())
    {
        let identity = selection["identity"]
            .as_str()
            .expect("model selection identity");
        let digest = selection["digest"]
            .as_str()
            .expect("model selection digest");
        let document = match identity {
            "acme/orders" => "domain-package-acme-orders.json",
            other => panic!("no published model document for QSpec selection {other}"),
        };
        evidence
            .insert_domain_package_document(
                digest,
                fs::read(root.join("proposals/checked-package-v2").join(document))
                    .expect("read selected QSpec model package"),
            )
            .expect("retain selected QSpec model package");
    }
    match CheckedPackageV2::read(&canonical, CheckedPackageReadLimits::bounded(), &evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("authoritative QSpec fixture must be admitted: {other:?}"),
    }
}

/// A directly requested case keeps its operation and node identity
/// while producing no generated claim.
#[trace("FR-014-AC-44", "TC-024")]
#[test]
fn tc_024_ac44_direct_control_case_is_unsupported() {
    let package = qspec_package("positive-union-nodes.json");
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "case"
                && matches!(
                    &node.body,
                    CheckedTerm::Application {
                        operator: ApplicationOperator::Case,
                        ..
                    }
                )
        })
        .expect("QSpec fixture has an admitted case root");
    let requested_id = node.node_id.clone();
    let oracles = generate_exact_scalar_oracles(
        &package,
        &[ExactScalarItem {
            node_id: requested_id.clone(),
            operation: add_over(-100, 100),
        }],
    )
    .expect("direct unsupported request settles in claim map");
    let claim = &oracles.claim_map.items[0];
    assert_eq!(claim.node_id, requested_id);
    assert_eq!(claim.operation.identity, "integer.add domain=[-100,100]");
    assert!(
        matches!(
            &claim.result,
            ClaimDisposition::Refused { refusal: ExactScalarRefusal::Unsupported { unsupported_node_id, node_tag: "expression" } }
                if unsupported_node_id == &requested_id
        ),
        "expected direct typed control.case refusal, got {:?}",
        claim.result
    );
    assert!(!oracles
        .claim_map
        .items
        .iter()
        .any(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));
}

/// A directly requested case must keep its identity through the public function path.
#[trace("FR-021-AC-33", "TC-031")]
#[test]
fn tc_031_ac33_direct_control_case_is_unsupported() {
    use quire_contract_codegen::ContractRequestPosition;
    use quire_contract_codegen::{
        generate_exact_function_oracles, ExactFunctionItem, ExactFunctionRefusal,
    };
    use quire_contract_model::{
        CheckedNodeTag, CompleteLoweringProfileV2, ContractDependencies, ContractPackageReadResult,
        ContractPackageWireV1, LoweringProfileRef,
    };
    let package = qspec_package("positive-union-nodes.json");
    let case = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "case"
                && matches!(
                    &node.body,
                    CheckedTerm::Application {
                        operator: ApplicationOperator::Case,
                        ..
                    }
                )
        })
        .expect("admitted case root");
    let id = case.node_id.clone();
    let lowered = package.lower(
        std::slice::from_ref(&id),
        &CompleteLoweringProfileV2 {
            reference: LoweringProfileRef {
                authority: "agent-ix".into(),
                identity: "quire.codegen.exact-function/v1".into(),
            },
            supported_tags: [
                CheckedNodeTag::ScalarType,
                CheckedNodeTag::CompositeType,
                CheckedNodeTag::BoundedDomain,
                CheckedNodeTag::Value,
                CheckedNodeTag::Expression,
                CheckedNodeTag::Claim,
                CheckedNodeTag::Correspondence,
            ]
            .into(),
            require_bounds: false,
            work_limit: 65_536,
        },
    );
    let contract = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().unwrap(),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => *package,
        other => panic!("lowered function request should be admitted: {other:?}"),
    };
    let output = generate_exact_function_oracles(
        &contract,
        &[ContractRequestPosition::DriverItem {
            node_id: id.clone(),
            result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        }],
        &[],
        &[],
        &[0],
        &[ExactFunctionItem {
            call_node_id: id.clone(),
            function: "missing".into(),
            argument_node_ids: vec![],
        }],
    )
    .unwrap();
    assert!(
        matches!(&output.claim_map.items[0].result,
        ClaimDisposition::Refused { refusal: ExactFunctionRefusal::Unsupported { unsupported_node_id, node_tag: "expression" } } if unsupported_node_id == &id),
        "{:?}",
        output.claim_map.items[0].result
    );
}

fn two_parameter_admitted() -> &'static (CheckedPackageV2, FixtureIds) {
    static ADMITTED: OnceLock<(CheckedPackageV2, FixtureIds)> = OnceLock::new();
    ADMITTED.get_or_init(|| two_parameter_package().admit_resolved())
}

fn two_parameter_disposition(
    code: u32,
    operation: ExactScalarOperation,
) -> (
    ExactScalarOracles,
    ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>,
) {
    let (package, ids) = two_parameter_admitted();
    let node_id = ids.resolve(&code_id(code));
    let oracles = generate(
        package,
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

fn two_parameter_refusal(code: u32, operation: ExactScalarOperation) -> AdmittedRefusal {
    let (oracles, disposition) = two_parameter_disposition(code, operation);
    match disposition {
        ClaimDisposition::Refused { refusal } => AdmittedRefusal {
            actual: refusal,
            ids: fixture_ids_for_digest(oracles.claim_map.package_id.digest.as_ref()),
        },
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

fn two_parameter_checked_bounds(
    code: u32,
    operation: ExactScalarOperation,
) -> (Vec<quire_contract_model::CheckedNodeId>, FixtureIds) {
    let (oracles, disposition) = two_parameter_disposition(code, operation);
    match disposition {
        ClaimDisposition::Generated(generated) => (
            generated.checked_bounds,
            fixture_ids_for_digest(oracles.claim_map.package_id.digest.as_ref()),
        ),
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

fn two_parameter_derive(code: u32) -> Result<ExactScalarItem, AdmittedRefusal> {
    let (package, ids) = two_parameter_admitted();
    derive_one_resolved(package, ids, code).map_err(|actual| AdmittedRefusal {
        actual,
        ids: ids.clone(),
    })
}

/// Two distinct bounded parameters, `Int[0, 9]` and `Int[10, 20]`, with a result typed `[0, 29]`
/// generate: the descriptor is compared with the result bound only, and the claim's checked
/// bounds are the result bound first, then each operand's own. Derivation returns the descriptor
/// over the result bound. Before IR-298 the three reachable `integer_range` bounds over Integer
/// were `AmbiguousBound`.
#[trace("FR-014-AC-20", "TC-024")]
#[test]
fn tc_024_ac20_two_distinct_bounded_parameters_generate_against_the_result_bound() {
    let (checked, ids) = two_parameter_checked_bounds(TWO_PARAMETER_SUM, add_over(0, 29));
    assert_eq!(
        checked,
        [
            ids.resolve(&id(&Bound::Integer(0, 29).key())),
            ids.resolve(&id(&Bound::Integer(0, 9).key())),
            ids.resolve(&id(&Bound::Integer(10, 20).key())),
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
#[trace("FR-014-AC-21", "TC-024")]
#[test]
fn tc_024_ac21_a_scalar_typed_result_takes_the_one_bound_no_operand_types() {
    let (checked, ids) = two_parameter_checked_bounds(TWO_PARAMETER_ATTACHED, add_over(0, 29));
    assert_eq!(checked[0], ids.resolve(&id(&Bound::Integer(0, 29).key())));
}

/// A scalar-typed result whose reachable bounds are all operands' own has no result bound.
#[trace("FR-014-AC-22", "TC-024")]
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
#[trace("FR-014-AC-23", "TC-024")]
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
#[trace("FR-014-AC-24", "TC-024")]
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
        let (checked, ids) = two_parameter_checked_bounds(code, operation);
        assert_eq!(
            checked,
            expected
                .iter()
                .map(|bound| ids.resolve(&id(&bound.key())))
                .collect::<Vec<_>>(),
            "node {code}"
        );
    }
}

/// A `reference` operand typed by a plain scalar type owns no bound: where another operand owns
/// one, or the result is typed by one, it is `RequiresBound` naming its type, in generation and in
/// derivation. A literal operand is never refused, and a package whose operands own no bound and
/// whose result is scalar-typed keeps the one-bound reading.
#[trace("FR-014-AC-25", "TC-024")]
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
        let Err(derived) = two_parameter_derive(code) else {
            panic!("node {code}: derivation unexpectedly succeeded");
        };
        assert_eq!(derived, expected, "node {code}: derivation refusal");
    }
    assert!(matches!(
        two_parameter_disposition(TWO_PARAMETER_LITERAL, add_over(0, 29)).1,
        ClaimDisposition::Generated(_)
    ));
    // A bounded parameter beside a plain-typed reference to a `value` node whose body is a literal
    // is a constant operand, not a plain-typed one (FR-014-AC-26), so it generates.
    let (package, ids) = corpus_package().admit_resolved();
    let bounded_operand = ids.resolve(&code_id(BOUNDED_OPERAND));
    let oracles = generate(
        &package,
        &[ExactScalarItem {
            node_id: bounded_operand.clone(),
            operation: integer_add(),
        }],
    );
    assert!(matches!(
        dispositions(&oracles).get(bounded_operand.digest.as_ref()),
        Some(ClaimDisposition::Generated(_))
    ));
}

fn qsl_admitted() -> &'static (CheckedPackageV2, FixtureIds) {
    static ADMITTED: OnceLock<(CheckedPackageV2, FixtureIds)> = OnceLock::new();
    ADMITTED.get_or_init(|| qsl_shaped_package().admit_resolved())
}

fn qsl_disposition(
    code: u32,
    operation: ExactScalarOperation,
) -> ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal> {
    let (package, ids) = qsl_admitted();
    let node_id = ids.resolve(&code_id(code));
    let oracles = generate(
        package,
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
) -> (Vec<quire_contract_model::CheckedNodeId>, FixtureIds) {
    let (package, ids) = qsl_admitted();
    let item = ExactScalarItem {
        node_id: ids.resolve(&code_id(code)),
        operation,
    };
    let oracles = generate(package, &[item]);
    match dispositions(&oracles).get(ids.resolve(&code_id(code)).digest.as_ref()) {
        Some(ClaimDisposition::Generated(generated)) => {
            (generated.checked_bounds.clone(), ids.clone())
        }
        other => panic!("node {code} is not generated: {other:?}"),
    }
}

fn bound_ids(bounds: &[Bound], ids: &FixtureIds) -> Vec<quire_contract_model::CheckedNodeId> {
    bounds
        .iter()
        .map(|bound| ids.resolve(&id(&bound.key())))
        .collect()
}

/// Generation and derivation over a QSL-shaped node both refuse it with `expected`.
fn assert_qsl_refused(code: u32, operation: ExactScalarOperation, expected: ExactScalarRefusal) {
    let (package, ids) = qsl_admitted();
    match qsl_disposition(code, operation) {
        ClaimDisposition::Refused { refusal } => assert_refusal_with_ids(&refusal, &expected, ids),
        other => panic!("node {code} is not refused: {other:?}"),
    }
    let Err(derived) = derive_one_resolved(package, ids, code) else {
        panic!("node {code}: derivation unexpectedly succeeded");
    };
    assert_refusal_with_ids(&derived, &expected, ids);
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
#[trace("FR-014-AC-26", "TC-024")]
#[test]
fn tc_024_ac26_a_reference_to_a_literal_value_node_is_a_literal_operand() {
    let (checked, ids) = qsl_checked_bounds(QSL_INC, add_over(0, 10));
    assert_eq!(
        checked,
        bound_ids(&[Bound::Integer(0, 10), Bound::Integer(0, 9)], &ids)
    );
    let (package, ids) = qsl_admitted();
    assert_eq!(
        derive_one_resolved(package, ids, QSL_INC)
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
#[trace("FR-014-AC-27", "TC-024")]
#[test]
fn tc_024_ac27_a_scalar_typed_result_takes_the_bound_of_its_narrowing_conversion() {
    let (package, ids) = qsl_admitted();
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
            derive_one_resolved(package, ids, code)
                .expect("derives")
                .operation,
            operation,
            "node {code}"
        );
        let (checked, ids) = qsl_checked_bounds(code, operation);
        assert_eq!(checked, bound_ids(&expected, &ids), "node {code}");
    }
    match qsl_disposition(QSL_ADD, add_over(0, 9)) {
        ClaimDisposition::Refused { refusal } => assert_refusal_with_ids(
            &refusal,
            &ExactScalarRefusal::BoundMismatch {
                bound: id(&Bound::Integer(10, 29).key()),
                form: BoundForm::IntegerRange,
            },
            ids,
        ),
        other => panic!("not refused: {other:?}"),
    }
}

/// The referenced node's body decides: `n` is a plain-Integer `value` node of form `literal`
/// whose body is not a literal, so `x + n` is `RequiresBound`.
#[trace("FR-014-AC-28", "TC-024")]
#[test]
fn tc_024_ac28_a_literal_form_node_without_a_literal_body_is_not_a_literal() {
    assert_qsl_refused(
        QSL_NOT_LITERAL_OPERAND,
        add_over(0, 29),
        requires_integer_bound(),
    );
}

/// `x + p` narrowed to `Int[0, 29]`, `p` a plain-Integer parameter, is `RequiresBound`.
#[trace("FR-014-AC-29", "TC-024")]
#[test]
fn tc_024_ac29_a_plain_parameter_beside_a_bounded_one_requires_a_bound() {
    assert_qsl_refused(QSL_PLAIN_OPERAND, add_over(0, 29), requires_integer_bound());
}

/// `q + q` narrowed to `Int[0, 29]`, no operand owning a bound, is `RequiresBound`: the narrowing
/// bounds the result as a `bounded_domain` result type does.
#[trace("FR-014-AC-30", "TC-024")]
#[test]
fn tc_024_ac30_plain_operands_under_a_narrowing_require_a_bound() {
    assert_qsl_refused(QSL_PLAIN_PAIR, add_over(0, 29), requires_integer_bound());
}

/// A literal operand of an integer operation that is not an `integer` literal is refused by its
/// own kind, whatever the node referencing it is typed: `x + "a"`, the text literal in a
/// plain-Integer-typed `value` node.
#[trace("FR-014-AC-31", "TC-024")]
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
#[trace("FR-014-AC-32", "TC-024")]
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
#[trace("FR-014-AC-33", "TC-024")]
#[test]
fn tc_024_ac33_a_narrowing_of_another_form_or_base_type_is_missing_bound() {
    assert_qsl_refused(
        QSL_WRONG_FORM_NARROWED,
        add_over(10, 29),
        ExactScalarRefusal::MissingBound {
            bounded_type: code_id(T_INTEGER),
            expected_form: BoundForm::IntegerRange,
        },
    );
    let (result, wire, _) = qsl_wrong_base_package().read_resolved();
    let invalid = id(&wrong_base_bound().key());
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"] == invalid.digest.as_ref())
        .expect("wrong-base bound node");
    let nodes: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(wire["semantic_graph"]["nodes"].clone()).expect("nodes parse");
    assert!(matches!(
        derive_structural_candidates(
            &nodes,
            quire_contract_model::CheckedPackageReadLimits::bounded().bytes,
            quire_contract_model::CheckedPackageReadLimits::bounded().work,
        ),
        Err(StructuralCandidateError::InvalidClosedShape { position: rejected, node_id })
            if rejected == position && node_id == invalid
    ));
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("wrong-base narrowing unexpectedly admitted");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str()),
        Some(format!("/semantic_graph/nodes/{position}/node_id").as_str())
    );
    assert_eq!(refusal.locus, Some(invalid));
}

/// `x + 2` is narrowed to `Int[0, 11]` and is also an operand of `(x + 2) + x`: the narrow's
/// bound does not bound that other use, so it is `AmbiguousBound`. `x + 1`, consumed only by its
/// narrowing, generates.
#[trace("FR-014-AC-34", "TC-024")]
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
    derive_exact_scalar_items(package, &[fixture_ids_for(package).resolve(&code_id(code))])
        .pop()
        .expect("one result per node id")
}

/// Deriving from the package alone yields, for every golden-corpus node, the descriptor the
/// fixture declares, and generating from the derived items gives the same claim map as
/// generating from the declared ones, every claim `ir_confirmed`.
#[trace("FR-014-AC-18", "FR-014-AC-19", "TC-024")]
#[test]
fn tc_024_derivation_equals_every_golden_corpus_descriptor() {
    let package = corpus_package().admit();
    let corpus = corpus();
    let fixture_ids = fixture_ids_for(&package);
    let ids = corpus
        .iter()
        .map(|expression| fixture_ids.resolve(&code_id(expression.code)))
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
        assert_eq!(item.node_id, fixture_ids.resolve(&code_id(expression.code)));
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
/// `numeric.convert_rounding`) and the laws (`integer.div`).
#[trace("FR-014-AC-18", "TC-024")]
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
#[trace("FR-014-AC-18", "FR-014-AC-22", "FR-014-AC-35", "TC-024")]
#[test]
fn tc_024_derivation_refuses_what_it_cannot_derive_with_a_typed_reason() {
    let package = derivation_package().admit();
    assert_eq!(
        derive_one(&package, DERIVE_REM),
        Err(no_claim(ClaimDerivationRefusal::OperationNotDerivable {
            operation_identity: "quire.op.integer.rem".to_owned()
        }))
    );
    assert!(
        derive_one(&package, DERIVE_INTEGER_EQ).is_ok(),
        "integer equality must derive a confirmed oracle"
    );
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
    // A node whose closure carries two bounds of one form on its scalar result type, neither the
    // own bound of an operand, is ambiguous, so it derives no domain. A node over two bounded
    // parameters is not: each operand names its own bound (IR-298, FR-014-AC-20).
    assert!(matches!(
        bound(AMBIGUOUS),
        ExactScalarRefusal::AmbiguousBound { .. }
    ));
}

/// Every checked Boolean and integer equality identity derives, confirms and emits the runtime
/// operator rather than retaining the old operation-not-derivable disposition.
#[trace("FR-014-AC-35", "TC-024")]
#[test]
fn tc_024_boolean_and_integer_equality_nodes_generate_confirmed_runtime_oracles() {
    let package = derivation_package().admit();
    let codes = [
        DERIVE_BOOLEAN_AND,
        DERIVE_BOOLEAN_OR,
        DERIVE_BOOLEAN_NOT,
        DERIVE_BOOLEAN_IMPLIES,
        DERIVE_BOOLEAN_EQ,
        DERIVE_BOOLEAN_NE,
        DERIVE_BOOLEAN_SHORT_CIRCUIT,
        DERIVE_BOOLEAN_LAZY_AND,
        DERIVE_BOOLEAN_LAZY_OR,
        DERIVE_INTEGER_EQ,
        DERIVE_INTEGER_NE,
        1041,
        1044,
        DERIVE_INTEGER_GT,
        1045,
    ];
    let items = codes
        .iter()
        .map(|code| derive_one(&package, *code).expect("derives"))
        .collect::<Vec<_>>();
    let oracles = generate(&package, &items);
    for code in codes {
        let id = claim_id(&oracles, code);
        let claim = oracles
            .claim_map
            .items
            .iter()
            .find(|claim| claim.node_id == id)
            .expect("one claim");
        assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
        assert!(matches!(claim.result, ClaimDisposition::Generated(_)));
    }
    let source = contents(&oracles, "src/lib.rs");
    assert!(source.contains("rt::evaluate_boolean("));
    assert!(source.contains("residue::evaluate_boolean_short_circuit("));
    assert!(source.contains(".check_equality("));
    assert!(source.contains("checked.evaluate("));
}

/// A caller cannot relabel a decided right operand as a stopping oracle.
#[trace("FR-014-AC-35", "TC-024")]
#[test]
fn tc_024_boolean_descriptor_rejects_a_wrong_right_evaluation_mode() {
    let package = derivation_package().admit();
    let mut item = derive_one(&package, DERIVE_BOOLEAN_AND).expect("derives");
    item.operation = ExactScalarOperation::Boolean {
        operator: quire_contract_codegen::BooleanOperator::And,
        right_may_stop: true,
    };
    let oracles = generate(&package, &[item]);
    assert_eq!(
        refusal_of(&oracles, DERIVE_BOOLEAN_AND),
        ExactScalarRefusal::BooleanRightOperandMismatch {
            declared_may_stop: true,
            actual_may_stop: false
        },
    );
}

/// Generate the admitted Boolean/equality corpus for the direct-runtime scratch crate.
pub(super) fn boolean_oracles() -> ExactScalarOracles {
    let package = derivation_package().admit();
    let codes = [
        DERIVE_BOOLEAN_AND,
        DERIVE_BOOLEAN_OR,
        DERIVE_BOOLEAN_NOT,
        DERIVE_BOOLEAN_IMPLIES,
        DERIVE_BOOLEAN_EQ,
        DERIVE_BOOLEAN_NE,
        DERIVE_BOOLEAN_SHORT_CIRCUIT,
        DERIVE_BOOLEAN_LAZY_AND,
        DERIVE_BOOLEAN_LAZY_OR,
        DERIVE_INTEGER_EQ,
        DERIVE_INTEGER_NE,
        1041,
        1044,
        DERIVE_INTEGER_GT,
        1045,
    ];
    let items = codes
        .iter()
        .map(|code| derive_one(&package, *code).expect("derives"))
        .collect::<Vec<_>>();
    generate(&package, &items)
}

/// The coverage map names executable tokens in the emitted file and counts implication
/// consequents from the admitted node, with the evaluation entry outside their regions.
#[trace("FR-014-AC-36", "TC-024")]
#[test]
fn tc_024_boolean_oracle_source_map_probes_the_entry_and_implication_consequent() {
    let package = derivation_package().admit();
    let item = derive_one(&package, DERIVE_BOOLEAN_IMPLIES).expect("implies derives");
    let oracles = generate(&package, &[item]);
    let symbol = symbol(&oracles, DERIVE_BOOLEAN_IMPLIES);
    let source = contents(&oracles, "src/lib.rs");
    let regions: Vec<Value> =
        serde_json::from_str(contents(&oracles, "source-maps/lib.json")).expect("coverage map");
    assert_eq!(regions.len(), 2);
    let entry = &regions[0];
    assert_eq!(entry["role"], "oracle_evaluation");
    assert_eq!(entry["expectedConsequents"], 1);
    let entry_line = entry["probe"]["line"].as_u64().expect("entry line") as usize;
    assert!(source
        .lines()
        .nth(entry_line - 1)
        .expect("line")
        .contains(&format!("pub fn {symbol}")));
    let probe_token = |source: &str, region: &Value| {
        let probe = &region["probe"];
        let line = probe["line"].as_u64().expect("probe line") as usize;
        let start = probe["startColumn"].as_u64().expect("probe start") as usize;
        let end = probe["endColumn"].as_u64().expect("probe end") as usize;
        let text = source.lines().nth(line - 1).expect("probe line exists");
        text.get(start - 1..end - 1)
            .expect("probe spans a token")
            .to_owned()
    };
    assert_eq!(probe_token(source, entry), "pub");
    let consequent = &regions[1];
    assert_eq!(consequent["role"], "implication_consequent");
    let consequent_line = consequent["probe"]["line"]
        .as_u64()
        .expect("consequent line") as usize;
    assert_ne!(entry_line, consequent_line);
    assert!(source
        .lines()
        .nth(consequent_line - 1)
        .expect("line")
        .contains("right"));
    assert_eq!(probe_token(source, consequent), "right");
    assert_eq!(consequent["startLine"], consequent["endLine"]);
    assert_ne!(entry["startLine"], consequent["startLine"]);

    let lazy = derive_one(&package, DERIVE_BOOLEAN_SHORT_CIRCUIT).expect("lazy implies derives");
    let lazy_oracles = generate(&package, &[lazy]);
    let lazy_source = contents(&lazy_oracles, "src/lib.rs");
    let lazy_regions: Vec<Value> =
        serde_json::from_str(contents(&lazy_oracles, "source-maps/lib.json"))
            .expect("lazy coverage map");
    let lazy_line = lazy_regions[1]["probe"]["line"]
        .as_u64()
        .expect("lazy consequent line") as usize;
    assert_eq!(lazy_regions[1]["role"], "implication_consequent");
    assert_eq!(probe_token(lazy_source, &lazy_regions[0]), "pub");
    assert_eq!(probe_token(lazy_source, &lazy_regions[1]), "right");
    assert_eq!(
        lazy_source.lines().nth(lazy_line - 1).expect("line").trim(),
        "right()",
        "the probe must execute only when the lazy consequent is called"
    );

    let mut mismatched = derive_one(&package, DERIVE_BOOLEAN_AND).expect("and derives");
    mismatched.operation = ExactScalarOperation::Boolean {
        operator: quire_contract_codegen::BooleanOperator::Implies,
        right_may_stop: false,
    };
    let caller_declared = generate(&package, &[mismatched]);
    let regions: Vec<Value> =
        serde_json::from_str(contents(&caller_declared, "source-maps/lib.json"))
            .expect("coverage map");
    assert_eq!(
        regions.len(),
        2,
        "an implied oracle still renders a consequent region"
    );
    assert_eq!(
        regions[0]["expectedConsequents"], 0,
        "the node is an and node"
    );

    let all = boolean_oracles();
    let all_source = contents(&all, "src/lib.rs");
    let all_regions: Vec<Value> =
        serde_json::from_str(contents(&all, "source-maps/lib.json")).expect("all coverage");
    let generated = all
        .claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .collect::<Vec<_>>();
    let entries = all_regions
        .iter()
        .filter(|region| region["role"] == "oracle_evaluation")
        .collect::<Vec<_>>();
    let ids = fixture_ids_for_digest(all.claim_map.package_id.digest.as_ref());
    let implication_nodes = [DERIVE_BOOLEAN_IMPLIES, DERIVE_BOOLEAN_SHORT_CIRCUIT]
        .map(|code| ids.resolve(&code_id(code)));
    assert_eq!(
        entries.len(),
        generated.len(),
        "one entry per generated oracle"
    );
    for claim in generated {
        let node = serde_json::to_value(&claim.node_id).expect("node identity");
        let own_entries = entries
            .iter()
            .filter(|region| region["nodeId"] == node)
            .collect::<Vec<_>>();
        assert_eq!(
            own_entries.len(),
            1,
            "one entry for {}",
            claim.node_id.digest
        );
        let entry = own_entries[0];
        let ClaimDisposition::Generated(generated) = &claim.result else {
            unreachable!("generated claims were selected above")
        };
        let entry_line = entry["startLine"].as_u64().expect("entry line") as usize;
        assert!(all_source
            .lines()
            .nth(entry_line - 1)
            .expect("entry line exists")
            .contains(&format!("pub fn {}", generated.symbol)));
        assert_eq!(probe_token(all_source, entry), "pub");
        assert_eq!(entry["startLine"], entry["endLine"]);
        let claim_regions = all_regions
            .iter()
            .filter(|region| region["nodeId"] == node && region["role"] == "implication_consequent")
            .collect::<Vec<_>>();
        let expected_consequents = implication_nodes
            .iter()
            .filter(|id| &claim.node_id == *id)
            .count();
        assert_eq!(
            entry["expectedConsequents"]
                .as_u64()
                .expect("declared consequent count") as usize,
            expected_consequents,
            "the admitted node fixes the consequent count for {}",
            claim.node_id.digest
        );
        assert_eq!(
            claim_regions.len(),
            expected_consequents,
            "no consequent region is dropped or duplicated for {}",
            claim.node_id.digest
        );
        for region in claim_regions {
            assert_eq!(probe_token(all_source, region), "right");
            assert_ne!(entry["startLine"], region["startLine"]);
        }
    }
}

/// The shared scan names every token the two
/// criteria list, in each delimiter form and with whitespace before the `!`, and does not name
/// the allowed lookalikes, so a scan test built on it can fail on each and only on those.
#[trace("FR-014-AC-39", "FR-018-AC-19", "TC-024", "TC-029")]
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

/// `non_test_code` removes the items a
/// `#[cfg(test)]` attribute governs by structure, wherever they sit, and keeps everything else:
/// an earlier mention of the attribute in a comment or a literal cuts nothing, code after a test
/// module is kept, braces in literals do not end the module early, and comments are dropped
/// without mistaking `//` inside a string for one.
#[trace("FR-014-AC-39", "FR-018-AC-19", "TC-024", "TC-029")]
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

/// The scalar generator's non-test code holds no panic site, and
/// neither does the `src/lib.rs` of the corpus crate (every other crate this module generates
/// is scanned inside [`generate`] as it is produced).
#[trace("FR-014-AC-39", "TC-024")]
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

/// The non-test code of the three oracle generators reads no `failed` record's limit kind: it
/// names no `CheckedPackageLimit`, accesses no `.limit_kind`, and meets a `Failed` record only at
/// an arm that binds nothing (`Failed { .. }`) and hands the record to
/// `classify_lowering_failure`. The scalar generator may name `limit_kind` only inside the
/// unrecognised-lowering refusal and its conversion; operand-range resource refusals use their
/// distinct `resource_kind` field. The shared module names `CheckedPackageLimit` and is the only
/// generator module that reads a failed lowering record's limit kind.
#[trace("FR-014-AC-42", "TC-024")]
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

        let arms = code.matches("CompleteLoweringRecordV2::Failed").count();
        let compact = code
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        assert!(
            compact.contains("LoweringLimitUnrecognised{limit_kind:&'staticstr,"),
            "{path} carries the field on the unrecognised lowering refusal"
        );
        if path != "src/oracle/scalar/mod.rs" {
            let expected_uses = if path == "src/oracle/equality/mod.rs" {
                1
            } else {
                assert!(
                    compact.contains("LoweringFailure::LimitUnrecognised{limit_kind,limit,consumed,}=>schedule::ScheduleRefusal::LoweringLimitUnrecognised{limit_kind,"),
                    "{path} transfers the field only into the typed schedule refusal"
                );
                3
            };
            assert_eq!(
                compact.matches("limit_kind").count(),
                expected_uses,
                "{path} names `limit_kind` only in its typed lowering refusal and conversion"
            );
            assert_eq!(
                arms, 0,
                "{path} consumes admitted package accounting instead"
            );
            assert!(
                code.contains("PackageItem::Refused") && code.contains("PackageAccounting"),
                "{path} retains the admitted package refusal path"
            );
            continue;
        }
        assert_eq!(
            compact.matches("limit_kind").count(),
            3,
            "{path} names `limit_kind` only in the typed lowering refusal and its conversion"
        );
        assert!(
            compact.contains("LoweringLimitUnrecognised{limit_kind:&'staticstr,"),
            "{path} carries the field on the unrecognised lowering refusal"
        );
        assert!(
            compact.contains("LoweringFailure::LimitUnrecognised{limit_kind,limit,consumed,}=>Self::LoweringLimitUnrecognised{limit_kind,"),
            "{path} binds the field only while converting the unrecognised lowering cause"
        );
        assert_eq!(arms, 1, "{path} has one direct-lowering `Failed` arm");
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

/// FR-021-AC-21 (the source half: the function generator holds zero invocations;
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
#[trace("FR-021-AC-21", "TC-031")]
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

/// The whole-file scan of the function generator counts a
/// `#[cfg(test)]` module: a banned macro inside one survives `comments_stripped` and is named,
/// where `non_test_code` would drop it, and a banned word inside a comment is not counted.
#[trace("FR-021-AC-21", "TC-031")]
#[test]
fn tc_031_ac21_the_whole_file_scan_counts_a_test_module_and_not_a_comment() {
    let source = "fn real() {}\n// todo!()\n#[cfg(test)]\nmod probe_tests {\n    fn p() { unreachable!(\"probe\") }\n}\n";
    let whole = comments_stripped(source);
    assert!(whole.lines().any(invokes_panicking_macro));
    assert_eq!(panic_tokens_in(&whole), vec!["unreachable!".to_owned()]);
    assert!(!non_test_code(source).lines().any(invokes_panicking_macro));
}

/// Exact conversions derive a unit-free descriptor and preserve the admitted target identity.
#[trace("FR-014-AC-48", "FR-014-AC-5", "TC-024")]
#[test]
fn quantity_conversion_derives_and_preserves_target_provenance() {
    let (package, ids) = derivation_package().admit_resolved();
    let items = QUANTITY_CONVERSIONS
        .iter()
        .map(|code| {
            let node_id = ids.resolve(&code_id(*code));
            let operation = derive_exact_scalar_items(&package, std::slice::from_ref(&node_id))
                .pop()
                .expect("one item")
                .expect("derives")
                .operation;
            assert_eq!(operation, ExactScalarOperation::QuantityConversion);
            ExactScalarItem { node_id, operation }
        })
        .collect::<Vec<_>>();
    let generated = generate_exact_scalar_oracles(&package, &items).expect("generates");
    for claim in &generated.claim_map.items {
        assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
        let ClaimDisposition::Generated(generated) = &claim.result else {
            panic!("{claim:?}")
        };
        assert!(generated.checked_bounds.is_empty());
        let node = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == claim.node_id)
            .expect("node");
        let CheckedTerm::Application { operation, .. } = &node.body else {
            panic!("quantity conversion is an application");
        };
        let target: quire_contract_model::CheckedNodeId =
            serde_json::from_value(operation["member"]["declaration"].clone()).expect("target");
        assert_eq!(generated.quantity_conversion_target.as_ref(), Some(&target));
        assert_eq!(
            serde_json::to_value(generated).unwrap()["quantity_conversion_target"],
            serde_json::to_value(target).unwrap()
        );
    }
    let targets = generated
        .claim_map
        .items
        .iter()
        .map(|claim| {
            let ClaimDisposition::Generated(generated) = &claim.result else {
                panic!("generated")
            };
            generated
                .quantity_conversion_target
                .clone()
                .expect("target identity")
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        targets.len(),
        2,
        "distinct admitted target declarations remain distinct"
    );
    let source = contents(&generated, "src/lib.rs");
    assert!(source.contains("unit: &quantity_owner::IdentifiedUnit"));
    assert!(source.contains(
        "quantity_owner::convert_quantity(operand, unit.unit(), &quantity_owner::QuantityTarget::Exact, meter)"
    ));
}

/// The Contract IR package of a request for `nodes`, and its driver positions in the same order.
fn contract_request(
    package: &CheckedPackageV2,
    nodes: &[quire_contract_model::CheckedNodeId],
) -> (
    quire_contract_model::ContractPackageV1,
    Vec<quire_contract_codegen::ContractRequestPosition>,
) {
    use quire_contract_model::{
        CheckedNodeTag, CompleteLoweringProfileV2, ContractDependencies, ContractPackageReadResult,
        ContractPackageWireV1, LoweringProfileRef,
    };
    let lowered = package.lower(
        nodes,
        &CompleteLoweringProfileV2 {
            reference: LoweringProfileRef {
                authority: "agent-ix".into(),
                identity: "quire.codegen.exact-function/v1".into(),
            },
            supported_tags: [
                CheckedNodeTag::ScalarType,
                CheckedNodeTag::CompositeType,
                CheckedNodeTag::BoundedDomain,
                CheckedNodeTag::Value,
                CheckedNodeTag::Expression,
                CheckedNodeTag::Function,
                CheckedNodeTag::Claim,
                CheckedNodeTag::Correspondence,
            ]
            .into(),
            require_bounds: false,
            work_limit: 65_536,
        },
    );
    let contract = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().unwrap(),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => *package,
        other => panic!("the conversion request should be admitted: {other:?}"),
    };
    let positions = nodes
        .iter()
        .map(
            |node_id| quire_contract_codegen::ContractRequestPosition::DriverItem {
                node_id: node_id.clone(),
                result_selection:
                    quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
            },
        )
        .collect();
    (contract, positions)
}

/// Conversion function bodies retain the earlier form refusal under either declared body kind.
#[trace("FR-014-AC-2", "TC-024")]
#[test]
fn tc_024_function_generator_refuses_a_function_whose_body_is_a_quantity_conversion() {
    use quire_contract_codegen::{
        generate_exact_function_oracles, ExactFunctionBody, ExactFunctionDeclaration,
        ExactFunctionItem, ExactFunctionRefusal,
    };
    let (package, ids) = conversion_operand_package().admit_resolved();
    let conversion = ids.resolve(&code_id(QUANTITY_CONVERSIONS[0]));
    let call = ids.resolve(&code_id(CALL_CONVERTING_FUNCTION));
    let unit = ids.resolve(&id(&unit_type()));
    let (contract, positions) = contract_request(&package, &[conversion.clone(), call.clone()]);
    for body in [
        ExactFunctionBody::Scalar {
            operator: IntegerOperator::Add,
        },
        ExactFunctionBody::CompositeEquality {
            operator: quire_contract_codegen::EqualityOperatorKind::Equal,
        },
    ] {
        let output = generate_exact_function_oracles(
            &contract,
            &positions,
            &[0],
            &[ExactFunctionDeclaration {
                node_id: conversion.clone(),
                name: CONVERTING_NAME.to_owned(),
                parameters: Vec::new(),
                result_type: unit.clone(),
                body: body.clone(),
                capability_requirements: Vec::new(),
            }],
            &[1],
            &[ExactFunctionItem {
                call_node_id: call.clone(),
                function: CONVERTING_NAME.to_owned(),
                argument_node_ids: Vec::new(),
            }],
        )
        .expect("generates");
        assert_eq!(output.claim_map.items.len(), 1, "{body:?}");
        assert_eq!(
            output.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::FormMismatch {
                    found: "conversion".to_owned()
                }
            },
            "{body:?}"
        );
        assert!(output.location_map.is_empty(), "{body:?}: a location entry");
    }
}

/// The composite-equality generator accepts only a `binary` expression, so a quantity conversion
/// node is refused as `FormMismatch` with its `conversion` form before any descriptor is read and
/// no oracle is produced for it.
#[trace("FR-014-AC-2", "TC-024")]
#[test]
fn tc_024_equality_generator_refuses_a_quantity_conversion_node_before_any_artifact() {
    use quire_contract_codegen::{
        generate_composite_equality_oracles, CompositeEqualityItem, CompositeEqualityRefusal,
        EqualityOperandDescriptor, EqualityOperatorKind,
    };
    let (package, ids) = derivation_package().admit_resolved();
    let node = ids.resolve(&code_id(QUANTITY_CONVERSIONS[0]));
    let integer = ids.resolve(&code_id(T_INTEGER));
    let (contract, positions) = contract_request(&package, std::slice::from_ref(&node));
    let output = generate_composite_equality_oracles(
        &contract,
        None,
        &positions,
        &[0],
        &[CompositeEqualityItem {
            node_id: node,
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(integer.clone()),
            right: EqualityOperandDescriptor::typed(integer),
        }],
        Default::default(),
    )
    .expect("generates");
    assert_eq!(output.claim_map.items.len(), 1);
    assert_eq!(
        output.claim_map.items[0].result,
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::FormMismatch {
                found: "conversion".to_owned()
            }
        }
    );
    assert!(
        !output
            .claim_map
            .items
            .iter()
            .any(|claim| matches!(claim.result, ClaimDisposition::Generated(_))),
        "no oracle is generated"
    );
}

/// Function schedules refuse a function declared over a quantity conversion body with the named
/// capability, as a refused function, whatever result type is declared: no schedule, no module
/// and no artifact is produced, and the request as a whole still settles.
#[trace("FR-014-AC-2", "TC-024")]
#[test]
fn tc_024_function_schedules_refuse_quantity_bodies_per_function() {
    use quire_contract_codegen::{
        generate_function_schedules, generate_function_schedules_with_limits,
        FunctionScheduleDeclaration, FunctionScheduleLimits, FunctionScheduleRefusal,
    };
    let (package, ids) = conversion_operand_package().admit_resolved();
    let function = ids.resolve(&code_id(CONVERSION_FUNCTION));
    let body = ids.resolve(&code_id(QUANTITY_CONVERSIONS[0]));
    let unit = ids.resolve(&id(&unit_type()));
    let integer = ids.resolve(&code_id(T_INTEGER));
    for result_type_node_id in [unit, integer] {
        let declaration = FunctionScheduleDeclaration {
            function_node_id: function.clone(),
            body_node_id: body.clone(),
            name: "converting".to_owned(),
            parameters: Vec::new(),
            result_type_node_id: result_type_node_id.clone(),
        };
        for generated in [
            generate_function_schedules(&package, std::slice::from_ref(&declaration), &[], 1_000),
            generate_function_schedules_with_limits(
                &package,
                std::slice::from_ref(&declaration),
                &[],
                FunctionScheduleLimits::default(),
            ),
        ] {
            let generated = generated.unwrap_or_else(|error| {
                panic!("result {result_type_node_id:?}: the request settles: {error:?}")
            });
            assert!(generated.functions.is_empty(), "no schedule");
            assert!(generated.modules.is_empty(), "no module artifact");
            assert_eq!(
                generated.refused_functions.len(),
                1,
                "result {result_type_node_id:?}"
            );
            assert_eq!(
                generated.refused_functions[0].refusal,
                FunctionScheduleRefusal::UnsupportedOperation(
                    if result_type_node_id == ids.resolve(&id(&unit_type())) {
                        function.clone()
                    } else {
                        body.clone()
                    }
                ),
                "result {result_type_node_id:?}"
            );
        }
    }
}

/// A quantity conversion argument retains the argument's generic type refusal.
#[trace("FR-014-AC-2", "TC-024")]
#[test]
fn tc_024_function_generator_refuses_a_call_passing_a_quantity_conversion_as_an_argument() {
    use quire_contract_codegen::{
        generate_exact_function_oracles, ExactFunctionBody, ExactFunctionDeclaration,
        ExactFunctionItem, ExactFunctionRefusal, FunctionParameter,
    };
    let (package, ids) = conversion_operand_package().admit_resolved();
    let adder_body = ids.resolve(&code_id(1001));
    let call = ids.resolve(&code_id(CALL_OVER_CONVERSION));
    let conversion = ids.resolve(&code_id(QUANTITY_CONVERSIONS[0]));
    let integer_value = ids.resolve(&code_id(V_INTEGER));
    let integer = ids.resolve(&code_id(T_INTEGER));
    let (contract, positions) = contract_request(&package, &[adder_body.clone(), call.clone()]);
    let expected = |_node_id| ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::UnsupportedOperandType {
            type_node_id: ids.resolve(&id(&unit_type())),
        },
    };
    let output = generate_exact_function_oracles(
        &contract,
        &positions,
        &[0],
        &[ExactFunctionDeclaration {
            node_id: adder_body,
            name: ADDER_NAME.to_owned(),
            parameters: vec![
                FunctionParameter {
                    name: "a".to_owned(),
                    type_node_id: integer.clone(),
                },
                FunctionParameter {
                    name: "b".to_owned(),
                    type_node_id: integer.clone(),
                },
            ],
            result_type: integer,
            body: ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        }],
        &[1],
        &[ExactFunctionItem {
            call_node_id: call,
            function: ADDER_NAME.to_owned(),
            argument_node_ids: vec![conversion.clone(), integer_value],
        }],
    )
    .expect("generates");
    assert_eq!(output.claim_map.items.len(), 1);
    assert_eq!(output.claim_map.items[0].result, expected(conversion));
    assert!(
        !output
            .claim_map
            .items
            .iter()
            .any(|claim| matches!(claim.result, ClaimDisposition::Generated(_))),
        "no oracle is generated"
    );
}

/// An unsupported conversion argument refuses its schedule item.
#[trace("FR-014-AC-2", "TC-024")]
#[test]
fn tc_024_function_schedule_item_with_a_conversion_argument_is_refused() {
    use quire_contract_codegen::{
        generate_function_schedules, FunctionScheduleDeclaration, FunctionScheduleDisposition,
        FunctionScheduleItem, FunctionScheduleParameter, FunctionScheduleRefusal,
    };
    let (package, ids) = conversion_operand_package().admit_resolved();
    let integer = ids.resolve(&code_id(T_INTEGER));
    let declaration = FunctionScheduleDeclaration {
        function_node_id: ids.resolve(&code_id(REFERENCE_FUNCTION)),
        body_node_id: ids.resolve(&code_id(1001)),
        name: ADDER_NAME.to_owned(),
        parameters: vec![
            FunctionScheduleParameter {
                value_node_id: ids.resolve(&code_id(ADDER_LEFT)),
                type_node_id: integer.clone(),
            },
            FunctionScheduleParameter {
                value_node_id: ids.resolve(&code_id(ADDER_RIGHT)),
                type_node_id: integer.clone(),
            },
        ],
        result_type_node_id: integer,
    };
    let item = FunctionScheduleItem {
        call_node_id: ids.resolve(&code_id(CALL_REFERENCE_OVER_CONVERSION)),
        function: ADDER_NAME.to_owned(),
    };
    let generated = generate_function_schedules(&package, &[declaration], &[item], 1_000)
        .expect("the request settles");
    assert_eq!(generated.claims.len(), 1);
    assert!(
        matches!(
            &generated.claims[0].result,
            FunctionScheduleDisposition::Refused {
                refusal: FunctionScheduleRefusal::MissingNode(type_node_id),
                ..
            } if type_node_id == &ids.resolve(&id(&unit_type()))
        ),
        "{:?}",
        generated.claims[0].result
    );
    assert!(
        !generated
            .claims
            .iter()
            .any(|claim| matches!(claim.result, FunctionScheduleDisposition::Scheduled { .. })),
        "no schedule"
    );
}

/// A quantity equality that reads a conversion node as an operand is refused by the
/// composite-equality generator as an unsupported quantity, with no oracle: the operand is read
/// through to the converted quantity type, which has no equality encoding.
#[trace("FR-014-AC-2", "FR-018-AC-15", "TC-024")]
#[test]
fn tc_024_equality_over_a_conversion_operand_is_refused_as_an_unsupported_quantity() {
    use quire_contract_codegen::{
        generate_composite_equality_oracles, CompositeEqualityItem, CompositeEqualityRefusal,
        EqualityOperandDescriptor, EqualityOperatorKind,
    };
    let (package, ids) = conversion_operand_package().admit_resolved();
    let node = ids.resolve(&code_id(EQUALITY_OVER_CONVERSION));
    let unit = ids.resolve(&id(&unit_type()));
    let (contract, positions) = contract_request(&package, std::slice::from_ref(&node));
    let output = generate_composite_equality_oracles(
        &contract,
        None,
        &positions,
        &[0],
        &[CompositeEqualityItem {
            node_id: node,
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(unit.clone()),
            right: EqualityOperandDescriptor::typed(unit),
        }],
        Default::default(),
    )
    .expect("generates");
    assert_eq!(output.claim_map.items.len(), 1);
    assert!(
        matches!(
            &output.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::Unsupported {
                    node_tag: "quantity",
                    ..
                }
            }
        ),
        "{:?}",
        output.claim_map.items[0].result
    );
}

/// Caller-declared exact conversion descriptors are confirmed against the admitted node.
#[trace("FR-014-AC-48", "TC-024")]
#[test]
fn caller_declared_quantity_conversion_is_confirmed() {
    let (package, ids) = derivation_package().admit_resolved();
    for code in QUANTITY_CONVERSIONS {
        let items = [ExactScalarItem {
            node_id: ids.resolve(&code_id(code)),
            operation: ExactScalarOperation::QuantityConversion,
        }];
        let plan = plan_exact_scalar_oracles(&package, &items).expect("plans");
        assert_eq!(
            plan.claims()[0].operation.provenance,
            OperationProvenance::IrConfirmed
        );
        assert!(matches!(
            plan.claims()[0].result,
            ClaimDisposition::Generated(_)
        ));
    }
}

#[trace("FR-014-AC-50", "TC-024")]
#[test]
fn admitted_compound_conversion_target_refuses_without_changing_sibling() {
    let (package, ids) = compound_conversion_package().admit_resolved();
    let nodes = [ids.resolve(&code_id(9902)), ids.resolve(&code_id(1001))];
    let derived = derive_exact_scalar_items(&package, &nodes);
    assert_eq!(
        derived[0],
        Err(no_claim(ClaimDerivationRefusal::OperandFormsNotDerivable {
            operation_identity: "quire.op.quantity.convert".to_owned(),
        }))
    );
    assert!(derived[1].is_ok(), "supported sibling derives");
    let generated = generate_exact_scalar_oracles(&package, &[derived[1].clone().unwrap()])
        .expect("sibling generates");
    assert!(matches!(
        generated.claim_map.items[0].result,
        ClaimDisposition::Generated(_)
    ));
}

#[trace("FR-014-AC-2", "FR-014-AC-50", "TC-024")]
#[test]
fn unit_function_signatures_refuse_per_function_and_preserve_supported_siblings() {
    use quire_contract_codegen::{
        generate_function_schedules, FunctionScheduleDeclaration, FunctionScheduleParameter,
        FunctionScheduleRefusal,
    };
    let mut builder = conversion_operand_package();
    builder.node_with(
        &key(9910),
        "function",
        "pure_function",
        &key(T_INTEGER),
        serde_json::json!({"term":"reference", "target":id(&key(ADDER_RIGHT))}),
        &[key(ADDER_RIGHT)],
    );
    let (package, ids) = builder.admit_resolved();
    let integer = ids.resolve(&code_id(T_INTEGER));
    let unit = ids.resolve(&id(&unit_type()));
    let supported = FunctionScheduleDeclaration {
        function_node_id: ids.resolve(&id(&key(9910))),
        body_node_id: ids.resolve(&code_id(ADDER_RIGHT)),
        name: "identity".to_owned(),
        parameters: vec![FunctionScheduleParameter {
            value_node_id: ids.resolve(&code_id(ADDER_RIGHT)),
            type_node_id: integer.clone(),
        }],
        result_type_node_id: integer.clone(),
    };
    for parameter_is_unit in [false, true] {
        let unsupported = FunctionScheduleDeclaration {
            function_node_id: ids.resolve(&code_id(CONVERSION_FUNCTION)),
            body_node_id: ids.resolve(&code_id(QUANTITY_CONVERSIONS[0])),
            name: CONVERTING_NAME.to_owned(),
            parameters: if parameter_is_unit {
                vec![FunctionScheduleParameter {
                    value_node_id: ids.resolve(&id(&key(V_QUANTITY))),
                    type_node_id: unit.clone(),
                }]
            } else {
                vec![]
            },
            result_type_node_id: if parameter_is_unit {
                integer.clone()
            } else {
                unit.clone()
            },
        };
        let output = generate_function_schedules(
            &package,
            &[unsupported.clone(), supported.clone()],
            &[],
            1000,
        )
        .expect("request settles");
        assert_eq!(output.functions.len(), 1);
        assert_eq!(
            output.functions[0].function_node_id,
            supported.function_node_id
        );
        assert_eq!(output.refused_functions.len(), 1);
        assert_eq!(
            output.refused_functions[0].refusal,
            FunctionScheduleRefusal::UnsupportedOperation(unsupported.function_node_id)
        );
    }
}
