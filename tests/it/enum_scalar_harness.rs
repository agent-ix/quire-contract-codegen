//! FR-049 enum scalar comparison harnesses over a genuine QSL source admitted through Contract IR.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_enum_scalar_harness, EnumCapability, EnumScalarHarness, EnumScalarOutcome,
    EnumScalarRefusal, EnumScalarRequest, KaniRunOutcome,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use super::admitted_enum_equality::application;
use super::admitted_enum_equality::checked;
use super::admitted_enum_fixture::compile;

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
ordered enum Level { LOW, MID, HIGH }\n\
enum Color { RED, GREEN, BLUE }\n\
function lt using v(a: Level, b: Level): Boolean pure { a < b }\n\
function le using v(a: Level, b: Level): Boolean pure { a <= b }\n\
function gt using v(a: Level, b: Level): Boolean pure { a > b }\n\
function ge using v(a: Level, b: Level): Boolean pure { a >= b }\n\
function eq using v(a: Level, b: Level): Boolean pure { a = b }\n\
function ne using v(a: Level, b: Level): Boolean pure { a != b }\n\
function color_eq using v(a: Color, b: Color): Boolean pure { a = b }\n\
function color_ne using v(a: Color, b: Color): Boolean pure { a != b }\n\
function not_enum using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a < b }\n";

struct Corpus {
    compiled: qsl_replay::AdmittedCompilation,
    package: CheckedPackageV2,
}

fn corpus() -> Corpus {
    let compiled = compile(SOURCE);
    let package = checked(&compiled, &[]);
    Corpus { compiled, package }
}

impl Corpus {
    fn node(&self, function: &str) -> CheckedNodeId {
        application(&self.package, function)
    }

    fn outcome(&self, function: &str, with_context: bool) -> EnumScalarOutcome {
        generate_enum_scalar_harness(&EnumScalarRequest {
            source: &self.package,
            context: with_context.then(|| self.compiled.context()),
            node_id: &self.node(function),
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            unwind: 1,
        })
    }

    fn harness(&self, function: &str) -> EnumScalarHarness {
        match self.outcome(function, true) {
            EnumScalarOutcome::Supported(harness) => *harness,
            EnumScalarOutcome::Refused(refusal) => panic!("{function} refused: {refusal:?}"),
        }
    }
}

const OPERATORS: [(&str, &str, &str); 6] = [
    ("lt", "quire.op.enum.lt", "<"),
    ("le", "quire.op.enum.le", "<="),
    ("gt", "quire.op.enum.gt", ">"),
    ("ge", "quire.op.enum.ge", ">="),
    ("eq", "quire.op.enum.eq", "=="),
    ("ne", "quire.op.enum.ne", "!="),
];

fn proof_body(harness: &EnumScalarHarness) -> &str {
    let source = &harness.harness.rust.contents;
    &source[source.find("#[kani::proof]").expect("a proof attribute")..]
}

/// Every operator over the ordered enum, and eq and ne over the unordered enum, settle to one
/// harness whose generated side is `compare_native_enum` over carriers and whose expected value is
/// the native operator over the indices, from the same admitted node.
#[trace("TC-063", "FR-049-AC-1")]
#[test]
fn every_operator_generates_one_native_carrier_harness_from_its_node() {
    let corpus = corpus();
    let mut cases =
        OPERATORS.map(|(function, identity, native)| (function, identity, native, true));
    cases[4].3 = true;
    let colors = [
        ("color_eq", "quire.op.enum.eq", "=="),
        ("color_ne", "quire.op.enum.ne", "!="),
    ]
    .map(|(function, identity, native)| (function, identity, native, false));
    for (function, identity, native, ordered) in cases.into_iter().chain(colors) {
        let generated = corpus.harness(function);
        let identity_record = &generated.harness.identity;
        assert_eq!(identity_record.node_id, corpus.node(function), "{function}");
        assert_eq!(identity_record.operation_identity, identity);
        let proof = proof_body(&generated);
        assert!(proof.contains("compare_native_enum(operator, &left_carrier, &right_carrier"));
        assert!(
            proof.contains(&format!("let native = left {native} right;")),
            "{proof}"
        );
        assert!(proof.contains(&format!("from_index(0_u64, {ordered}, left as u32, 3_u32)")));
        assert!(!proof.contains("EnumShape") && !proof.contains("from_member"));
    }
}

/// The symbolic domain is exactly the declared members: `0..member_count` per operand, drawn
/// independently, with the count read from the owner's own declaration.
#[trace("TC-063", "FR-049-AC-2")]
#[test]
fn the_symbolic_domain_is_exactly_the_declared_members() {
    let corpus = corpus();
    let declared = |name: &str| {
        corpus
            .compiled
            .context()
            .enums()
            .find(|item| {
                item.qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq([name])
            })
            .expect("declared enum")
            .cases()
            .len()
    };
    for (function, enum_name) in [("lt", "Level"), ("color_eq", "Color")] {
        let generated = corpus.harness(function);
        let count = i64::try_from(declared(enum_name)).unwrap();
        let arguments = &generated.harness.identity.arguments;
        assert_eq!(
            arguments
                .iter()
                .map(|a| (a.identifier.as_str(), a.minimum, a.maximum))
                .collect::<Vec<_>>(),
            [("left", 0, count - 1), ("right", 0, count - 1)]
        );
        let proof = proof_body(&generated);
        for operand in ["left", "right"] {
            let bound = format!(
                "kani::assume({operand} >= 0_i64 && {operand} <= {}_i64);",
                count - 1
            );
            assert_eq!(proof.matches(&bound).count(), 1, "{proof}");
        }
        assert_eq!(proof.matches("kani::any()").count(), 2);
    }
}

/// The harness assumes nothing about the comparison outcome and requires `Completed`.
#[trace("TC-063", "FR-049-AC-4")]
#[test]
fn the_assertion_requires_completed_and_no_assumption_reads_the_outcome() {
    let corpus = corpus();
    let proof_text = corpus.harness("le");
    let proof = proof_body(&proof_text);
    assert!(proof.contains("Ok(rt::Outcome::Completed(value)) => *value == native, _ => false"));
    let outcome_at = proof.find("let outcome").expect("outcome");
    assert!(
        proof
            .match_indices("kani::assume(")
            .all(|(at, _)| at < outcome_at),
        "no assumption follows the comparison: {proof}"
    );
}

/// Exactly one completion cover, after all assertions, so a vacuous constraint cannot verify.
#[trace("TC-063", "FR-049-AC-7")]
#[test]
fn each_harness_ends_with_exactly_one_completion_cover() {
    let corpus = corpus();
    for function in ["lt", "eq", "color_ne"] {
        let generated = corpus.harness(function);
        let proof = proof_body(&generated);
        assert_eq!(proof.matches("kani::cover!(").count(), 1);
        let last_assert = proof.rfind("assert!(").expect("an assertion");
        assert!(proof.find("kani::cover!(").unwrap() > last_assert);
        let tail = proof.lines().rev().nth(2).expect("cover line");
        assert!(tail.starts_with("kani::cover!(cover_reached"), "{tail}");
    }
}

/// A node of another family keeps its own disposition; the enum row only settles enum nodes.
#[trace("TC-063", "FR-049-AC-9")]
#[test]
fn a_node_of_another_family_is_not_an_enum_comparison() {
    let corpus = corpus();
    assert!(matches!(
        corpus.outcome("not_enum", true),
        EnumScalarOutcome::Refused(EnumScalarRefusal::NotEnumComparison)
    ));
}

/// Without the owner route's admitted declaration the item is unsupported naming the capability
/// and the enum, and no harness exists.
#[trace("TC-063", "FR-049-AC-8")]
#[test]
fn a_missing_owner_declaration_is_unsupported_with_a_named_capability() {
    let corpus = corpus();
    let EnumScalarOutcome::Refused(EnumScalarRefusal::Unsupported {
        capability,
        enum_name,
        ..
    }) = corpus.outcome("lt", false)
    else {
        panic!("expected unsupported");
    };
    assert_eq!(capability, EnumCapability::OwnerAdmittedContext);
    assert_eq!(capability.name(), "enum.owner_admitted_context");
    assert!(enum_name.contains("Level"), "{enum_name}");
    // A sibling in the same corpus keeps its supported disposition.
    assert!(matches!(
        corpus.outcome("eq", true),
        EnumScalarOutcome::Supported(_)
    ));
}

/// The record carries the declaration, the ordered member list and flag, the operator, the
/// encoding, the unwind bound and every option.
#[trace("TC-063", "FR-049-AC-11")]
#[test]
fn the_record_retains_declaration_operator_encoding_unwind_and_options() {
    let corpus = corpus();
    let generated = corpus.harness("ge");
    let record: serde_json::Value =
        serde_json::from_str(&generated.harness.record.contents).expect("a JSON record");
    let identity = &record["identity"];
    assert_eq!(
        identity["enumScalar"]["members"],
        serde_json::json!(["LOW", "MID", "HIGH"])
    );
    assert_eq!(identity["enumScalar"]["ordered"], true);
    assert_eq!(identity["enumScalar"]["operator"], "quire.op.enum.ge");
    assert_eq!(
        identity["enumScalar"]["declaration"]["path"],
        serde_json::json!(["Level"])
    );
    assert_eq!(identity["unwind"], 1);
    assert_eq!(identity["solver"], "cadical");
    assert_eq!(
        identity["options"].as_array().unwrap().len(),
        generated.harness.identity.options.len()
    );
    assert!(generated
        .harness
        .identity
        .options
        .contains(&"--unwind".to_owned()));
}

// ---- installed Kani lane ----

/// Run `function`'s harness, with `site` replaced by `with` when a mutation is given.
fn run(
    name: &str,
    function: &str,
    mutation: Option<(&str, &str)>,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let corpus = corpus();
    let mut generated = corpus.harness(function);
    if let Some((site, with)) = mutation {
        assert_eq!(
            generated.harness.rust.contents.matches(site).count(),
            1,
            "{site}"
        );
        generated.harness.rust.contents = generated.harness.rust.contents.replace(site, with);
    }
    let started = std::time::Instant::now();
    let evidence = crate::kani_obligations::run_scalar_under_real_kani(
        name,
        &generated.harness,
        &generated.artifacts[0],
    );
    eprintln!(
        "ENUMVERDICT {name} outcome={:?} wall_ms={} unwind={} solver={:?} argv={:?}",
        evidence.outcome,
        started.elapsed().as_millis(),
        evidence.unwind,
        evidence.solver,
        evidence.arguments
    );
    evidence
}

fn healthy(function: &str) {
    let evidence = run(&format!("enum-{function}"), function, None);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{evidence:?}");
}

fn falsified(function: &str, name: &str, site: &str, with: &str) {
    let evidence = run(name, function, Some((site, with)));
    assert!(
        matches!(evidence.outcome, KaniRunOutcome::Falsified { .. }),
        "{evidence:?}"
    );
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_lt_verifies() {
    healthy("lt");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_le_verifies() {
    healthy("le");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_gt_verifies() {
    healthy("gt");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_ge_verifies() {
    healthy("ge");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_eq_verifies() {
    healthy("eq");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_ne_verifies() {
    healthy("ne");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_unordered_eq_verifies() {
    healthy("color_eq");
}

#[trace("TC-063", "FR-049-AC-5")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_unordered_ne_verifies() {
    healthy("color_ne");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_lt_rendered_as_le_is_falsified() {
    falsified("lt", "enum-mut-lt-le", "left < right", "left <= right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_le_rendered_as_lt_is_falsified() {
    falsified("le", "enum-mut-le-lt", "left <= right", "left < right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_gt_rendered_as_ge_is_falsified() {
    falsified("gt", "enum-mut-gt-ge", "left > right", "left >= right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_ge_rendered_as_gt_is_falsified() {
    falsified("ge", "enum-mut-ge-gt", "left >= right", "left > right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_eq_rendered_as_ne_is_falsified() {
    falsified("eq", "enum-mut-eq-ne", "left == right", "left != right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_ne_rendered_as_eq_is_falsified() {
    falsified("ne", "enum-mut-ne-eq", "left != right", "left == right");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_reversed_ordering_is_falsified_for_lt() {
    falsified("lt", "enum-mut-rev-lt", "left < right", "right < left");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_reversed_ordering_is_falsified_for_le() {
    falsified("le", "enum-mut-rev-le", "left <= right", "right <= left");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_reversed_ordering_is_falsified_for_gt() {
    falsified("gt", "enum-mut-rev-gt", "left > right", "right > left");
}

#[trace("TC-063", "FR-049-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_reversed_ordering_is_falsified_for_ge() {
    falsified("ge", "enum-mut-rev-ge", "left >= right", "right >= left");
}

/// A comparison that stops (`Incomplete`) is never `Verified`: an exhausted work budget fails the
/// assertion that requires `Completed`.
#[trace("TC-063", "FR-049-AC-4")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_an_incomplete_comparison_is_not_verified() {
    falsified(
        "lt",
        "enum-incomplete",
        "work_units: u64::MAX,",
        "work_units: 0,",
    );
}
