//! FR-049 enum scalar comparison harnesses over a genuine QSL source admitted through Contract IR.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    decode_enum_falsification, generate_enum_scalar_harness, parse_assertion_playback,
    EnumCapability, EnumScalarHarness, EnumScalarOutcome, EnumScalarRefusal, EnumScalarRequest,
    KaniRunOutcome,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use super::admitted_enum_equality::application;
use super::admitted_enum_equality::checked;
use super::admitted_enum_fixture::compile;
use super::routed_generation::{admitted_routing_input, kani_parts, route};

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
function not_enum using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a < b }\n\
function bool_and using v(a: Boolean, b: Boolean): Boolean pure { a and b }\n\
function text_eq using v(a: Text[0, 16; nfc], b: Text[0, 16; nfc]): Boolean pure { a = b }\n\
function seq_eq using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { a = b }\n";

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
        self.outcome_with(
            function,
            with_context,
            crate::common::proof_ceilings::proof_ceilings(),
        )
    }

    fn outcome_with(
        &self,
        function: &str,
        with_context: bool,
        ceilings: quire_contract_codegen::ProofCeilings,
    ) -> EnumScalarOutcome {
        generate_enum_scalar_harness(&EnumScalarRequest {
            source: &self.package,
            context: with_context.then(|| self.compiled.context()),
            node_id: &self.node(function),
            ceilings,
            unwind: 1,
            requirement_occurrence: None,
            has_path_condition: false,
            ordinal: 0,
        })
    }

    /// The corpus case through the routed path: QSL, Contract IR, `generate_routed_admitted`.
    fn routed(
        &self,
        functions: &[&str],
        admitted: bool,
    ) -> quire_contract_codegen::RoutedGeneration {
        let items = functions
            .iter()
            .enumerate()
            .map(|(index, function)| route(index, &self.node(function)))
            .collect::<Vec<_>>();
        let input = admitted_routing_input(&self.package, &items);
        let mut records = std::collections::BTreeMap::new();
        let items = items
            .into_iter()
            .zip(&input.positions)
            .map(|(item, position)| {
                let quire_contract_codegen::ContractRequestPosition::DriverItem {
                    result_selection,
                    node_id,
                } = position
                else {
                    panic!("a driver position, got {position:?}");
                };
                let ordinal = self
                    .package
                    .source_map()
                    .iter()
                    .find(|entry| {
                        &entry.node_id == node_id
                            && entry.role == quire_contract_model::CheckedOccurrenceRole::Expression
                    })
                    .expect("the node has an expression occurrence")
                    .ordinal;
                let occurrence = quire_contract_codegen::RequirementOccurrence {
                    node_id: node_id.clone(),
                    role: quire_contract_model::CheckedOccurrenceRole::Expression,
                    ordinal,
                };
                let result_bound = self
                    .package
                    .resolved_graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == node_id)
                    .expect("the node")
                    .semantic_type
                    .clone();
                records.insert(
                    occurrence.clone(),
                    quire_contract_codegen::RequirementRecordInput {
                        result_bound: result_bound.clone(),
                        conversion_context: None,
                        path_condition: Vec::new(),
                    },
                );
                item.with_scalar_selection(&input.package, result_selection.clone())
                    .with_requirement(quire_contract_codegen::RoutedRequirement {
                        occurrence,
                        result_bound,
                        conversion_context: None,
                        path_condition: Vec::new(),
                    })
            })
            .collect::<Vec<_>>();
        let records = quire_contract_codegen::RequirementRecords::new(records);
        let contexts = quire_contract_codegen::GenerationContexts {
            kani: Some(quire_contract_codegen::KaniGenerationContext {
                ceilings: crate::common::proof_ceilings::proof_ceilings(),
                subject_path: "crate::subject",
                unwind: 1,
                requirement_records: &records,
            }),
        };
        if admitted {
            quire_contract_codegen::generate_routed_admitted(
                input.source,
                &input.package,
                &input.positions,
                &items,
                &contexts,
                self.compiled.context(),
                false,
            )
        } else {
            quire_contract_codegen::generate_routed(
                input.source,
                &input.package,
                &input.positions,
                &items,
                &contexts,
            )
        }
        .expect("the routed call generates")
    }

    /// The routed harness of one corpus function and the manifest its crate builds under.
    fn routed_harness(
        &self,
        function: &str,
    ) -> (
        quire_contract_codegen::KaniScalarObligationHarness,
        quire_contract_codegen::Artifact,
    ) {
        let generation = self.routed(&[function], true);
        let manifest = generation
            .oracle_artifacts
            .as_ref()
            .and_then(|artifacts| artifacts.iter().find(|a| a.path == "Cargo.toml"))
            .expect("the oracle crate manifest")
            .clone();
        let (record, harness) = kani_parts(&generation.items[0].output);
        (
            harness
                .unwrap_or_else(|| panic!("a harness, got {:?}", record.disposition))
                .clone(),
            manifest,
        )
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
    let cases = OPERATORS.map(|(function, identity, native)| (function, identity, native, true));
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
        assert!(domain_corresponds(proof, count), "{proof}");
        // Each way the symbolic domain can stop matching the member list is caught by the same
        // correspondence check: one extra index, the last member dropped, a constant operand.
        let widened = proof.replacen(
            &format!("left <= {}_i64", count - 1),
            &format!("left <= {count}_i64"),
            1,
        );
        let dropped = proof.replacen(
            &format!("right <= {}_i64", count - 1),
            &format!("right <= {}_i64", count - 2),
            1,
        );
        let constant = proof.replacen("let left: i64 = kani::any();", "let left: i64 = 1;", 1);
        for (name, mutant) in [
            ("extra index", widened),
            ("dropped member", dropped),
            ("constant operand", constant),
        ] {
            assert_ne!(mutant, proof, "{name} changes the harness");
            assert!(!domain_corresponds(&mutant, count), "{name} is caught");
        }
    }
}

/// The correspondence check of FR-049-AC-2: both operands are drawn symbolically and each is
/// bounded to exactly the indices of the declared members.
fn domain_corresponds(proof: &str, count: i64) -> bool {
    proof.matches("kani::any()").count() == 2
        && ["left", "right"].iter().all(|operand| {
            proof
                .matches(&format!(
                    "kani::assume({operand} >= 0_i64 && {operand} <= {}_i64);",
                    count - 1
                ))
                .count()
                == 1
        })
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

// ---- witness decode (FR-049-AC-10) ----

fn enum_transcript(harness: &EnumScalarHarness, draws: &[i64]) -> String {
    super::scalar_playback::transcript(&harness.harness.identity, draws)
}

fn decode(
    harness: &EnumScalarHarness,
    transcript: &str,
) -> Result<quire_contract_codegen::EnumWitness, quire_contract_codegen::DecodeFailure> {
    let identity = &harness.harness.identity;
    let playback = parse_assertion_playback(
        transcript,
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .map_err(quire_contract_codegen::DecodeFailure::from)?;
    decode_enum_falsification(identity, &playback)
}

/// Each enum operand is one physical `i64` binding, grouped to its parameter with the declaration
/// and member bounds, and a valid pair decodes to the declared members at those positions.
#[trace("TC-063", "FR-049-AC-10")]
#[test]
fn a_valid_binding_pair_decodes_to_the_declared_members() {
    let corpus = corpus();
    let generated = corpus.harness("lt");
    let identity = &generated.harness.identity;
    assert_eq!(
        identity
            .arguments
            .iter()
            .map(|a| (a.identifier.as_str(), a.minimum, a.maximum))
            .collect::<Vec<_>>(),
        [("left", 0, 2), ("right", 0, 2)]
    );
    let witness = decode(&generated, &enum_transcript(&generated, &[2, 0])).expect("a witness");
    assert_eq!((witness.left_index, witness.right_index), (2, 0));
    assert_eq!(
        (witness.left_member.as_str(), witness.right_member.as_str()),
        ("HIGH", "LOW")
    );
    assert_eq!(witness.operator, "quire.op.enum.lt");
}

/// An out-of-range, missing, duplicated or foreign binding is refused with a typed cause and
/// never mapped to a member.
#[trace("TC-063", "FR-049-AC-10")]
#[test]
fn perturbed_bindings_are_refused_with_typed_causes() {
    let corpus = corpus();
    let generated = corpus.harness("lt");
    for (draws, code) in [
        (&[3_i64, 0][..], "cg_scalar_witness_operand_out_of_domain"),
        (&[0, -1][..], "cg_scalar_witness_operand_out_of_domain"),
        (&[1][..], "kani_witness_arity_mismatch"),
        (&[0, 1, 2][..], "kani_witness_arity_mismatch"),
    ] {
        let error = decode(&generated, &enum_transcript(&generated, draws))
            .expect_err("a perturbed binding must refuse");
        assert_eq!(error.code, code, "{draws:?}");
    }
    let foreign = enum_transcript(&generated, &[0, 1]).replace(
        generated.harness.identity.harness_symbol.as_str(),
        "another_harness",
    );
    let error = decode(&generated, &foreign).expect_err("a foreign harness binding must refuse");
    assert_eq!(error.code, "cg_witness_harness_identity_mismatch");
    let narrow = enum_transcript(&generated, &[0, 1]).replacen(
        "vec![0, 0, 0, 0, 0, 0, 0, 0]",
        "vec![0, 0, 0, 0]",
        1,
    );
    let error = decode(&generated, &narrow).expect_err("a narrow binding must refuse");
    assert_eq!(error.code, "kani_witness_width_mismatch");
    let mut integer = generated.harness.identity.clone();
    integer.enum_scalar = None;
    let text = enum_transcript(&generated, &[0, 1]);
    let playback = parse_assertion_playback(
        &text,
        integer.harness_symbol.as_str(),
        integer.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    let error = decode_enum_falsification(&integer, &playback)
        .expect_err("an identity without an enum schema is not an enum witness");
    assert_eq!(error.code, "cg_scalar_witness_enum_identity_missing");
}

/// A persisted enum schema whose bounds disagree with its member list is refused before any draw.
#[trace("TC-063", "FR-049-AC-10")]
#[test]
fn an_enum_schema_with_bounds_off_the_member_list_is_refused() {
    let corpus = corpus();
    let mut generated = corpus.harness("lt");
    generated.harness.identity.arguments[1].maximum = 5;
    let error = decode(&generated, &enum_transcript(&generated, &[0, 1]))
        .expect_err("a widened bound must refuse");
    assert_eq!(error.code, "cg_scalar_witness_schema_enum_mismatch");
}

// ---- owner replay (FR-049-AC-5, FR-049-AC-10) ----

/// The selected corpus function recompiled by QSL's replay facade, as the owner route holds it.
fn owner_package(function: &str) -> quire_contract_codegen::ReplayPackage {
    let unlimited = qsl_replay::ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    };
    quire_contract_codegen::ReplayPackage::new(
        quire_contract_codegen::ReplayInputs {
            decode_limits: quire_contract_codegen::WitnessDecodeLimits {
                bytes: 1 << 20,
                values: 16,
            },
            package_name: "admitted-enums".to_owned(),
            source: quire_contract_codegen::LockedSource {
                authority: "test".to_owned(),
                identity: "admitted-enums.native".to_owned(),
                bytes: SOURCE.as_bytes().to_vec(),
            },
            dependencies: Vec::new(),
            accounting_limits: unlimited,
            stage_limits: std::collections::BTreeMap::from([(
                "s1.input_bytes".to_owned(),
                qsl_replay::DEFAULT_REPLAY_INPUT_BYTES,
            )]),
            supplied_limits: None,
            declared_domains: Vec::new(),
            replay_limits: qsl_replay::ReplayLimits::default(),
        },
        function,
    )
    .expect("the owner package recompiles the corpus source")
}

/// The owner's key for the declaration an enum harness's operands are declared with.
fn owner_declaration(corpus: &Corpus, generated: &EnumScalarHarness) -> qsl_replay::NodeKey {
    let wanted = &generated
        .harness
        .identity
        .enum_scalar
        .as_ref()
        .expect("an enum identity")
        .qualified_name;
    corpus
        .compiled
        .context()
        .enums()
        .find(|item| {
            item.qualified_name()
                .iter()
                .map(|part| part.as_str())
                .eq(wanted.iter().map(String::as_str))
        })
        .expect("the owner declares the enum")
        .key()
        .clone()
}

/// Decodes `draws` against `function`'s enum harness and replays the members through the owner.
fn owner_result(function: &str, draws: &[i64]) -> (quire_contract_codegen::EnumWitness, bool) {
    let corpus = corpus();
    let generated = corpus.harness(function);
    let witness = decode(&generated, &enum_transcript(&generated, draws)).expect("decodes");
    let owner = quire_contract_codegen::replay_enum_witness(
        &owner_package(function),
        &owner_declaration(&corpus, &generated),
        &witness,
    )
    .expect("the owner route evaluates the members");
    (witness, owner)
}

/// A decoded counterexample is replayed through `qsl_replay` over the declaration's members, and
/// the owner's result is the native operator over the ranks, whichever the operator and order.
#[trace("TC-063", "FR-049-AC-5")]
#[trace("TC-063", "FR-049-AC-10")]
#[test]
fn a_decoded_witness_replays_through_the_owner_route_as_the_declared_members() {
    for (function, native) in [
        ("lt", (|l, r| l < r) as fn(i64, i64) -> bool),
        ("le", |l, r| l <= r),
        ("gt", |l, r| l > r),
        ("ge", |l, r| l >= r),
        ("eq", |l, r| l == r),
        ("ne", |l, r| l != r),
    ] {
        for (left, right) in [(0, 2), (2, 0), (1, 1)] {
            let (witness, owner) = owner_result(function, &[left, right]);
            assert_eq!((witness.left_index, witness.right_index), (left, right));
            assert_eq!(owner, native(left, right), "{function}({left}, {right})");
        }
    }
    let (witness, owner) = owner_result("color_eq", &[0, 0]);
    assert_eq!(witness.left_member, witness.right_member);
    assert!(owner, "the unordered enum's equal members compare equal");
}

/// A member the owner does not declare is refused by the owner route and yields no result.
#[trace("TC-063", "FR-049-AC-10")]
#[test]
fn a_member_the_owner_does_not_declare_is_refused_by_the_owner_route() {
    let corpus = corpus();
    let generated = corpus.harness("lt");
    let mut witness = decode(&generated, &enum_transcript(&generated, &[0, 1])).expect("decodes");
    witness.left_member = "NOT_A_MEMBER".to_owned();
    let refusal = quire_contract_codegen::replay_enum_witness(
        &owner_package("lt"),
        &owner_declaration(&corpus, &generated),
        &witness,
    );
    assert!(refusal.is_err(), "{refusal:?}");
}

// ---- installed Kani lane ----

/// Run `function`'s harness, with `site` replaced by `with` when a mutation is given.
fn run(
    name: &str,
    function: &str,
    mutation: Option<(&str, &str)>,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let corpus = corpus();
    let (mut harness, manifest) = corpus.routed_harness(function);
    if let Some((site, with)) = mutation {
        assert!(harness.rust.contents.contains(site), "{site}");
        harness.rust.contents = harness.rust.contents.replace(site, with);
    }
    let started = std::time::Instant::now();
    let evidence = crate::kani_obligations::run_scalar_under_real_kani(name, &harness, &manifest);
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

/// A comparison that stops (`Incomplete`) is never `Verified`, and the failing check names the
/// `Incomplete` outcome rather than a disagreement with the native result.
#[trace("TC-063", "FR-049-AC-4")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_an_incomplete_comparison_is_not_verified() {
    let evidence = run(
        "enum-incomplete",
        "lt",
        Some(("work_units: u64::MAX,", "work_units: 0,")),
    );
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("an Incomplete comparison must not verify: {evidence:?}");
    };
    assert!(
        counterexample.contains("stopped Incomplete"),
        "the Incomplete type is the failing check: {counterexample}"
    );
}

/// A run-time `Err(IllTyped)` is never `Verified` and the failing check names its cause.
#[trace("TC-063", "FR-049-AC-4")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_an_ill_typed_comparison_is_not_verified_and_keeps_its_cause() {
    for (name, function, site, with, cause) in [
        (
            "enum-ill-unordered",
            "lt",
            "from_index(0_u64, true,",
            "from_index(0_u64, false,",
            "UnorderedEnumOrdering",
        ),
        (
            "enum-ill-distinct",
            "eq",
            "right_carrier = rt::NativeEnum::from_index(0_u64,",
            "right_carrier = rt::NativeEnum::from_index(1_u64,",
            "DistinctEnumDeclarations",
        ),
    ] {
        let evidence = run(name, function, Some((site, with)));
        let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
            panic!("{name} must not verify: {evidence:?}");
        };
        assert!(
            counterexample.contains(cause),
            "{name} keeps {cause}: {counterexample}"
        );
    }
}

/// An enum comparison node routed through `generate_routed_admitted` is generated per FR-049, the
/// same call without the admitted context keeps `OperationNotRendered`, and a node of another
/// family produces an identical output either way.
#[trace("TC-063", "FR-049-AC-1")]
#[trace("TC-063", "FR-049-AC-9")]
#[test]
fn the_routed_path_generates_enum_nodes_and_leaves_other_families_byte_identical() {
    let corpus = corpus();
    let functions = [
        "not_enum", "bool_and", "text_eq", "seq_eq", "lt", "eq", "color_eq",
    ];
    let legacy = corpus.routed(&functions, false);
    let admitted = corpus.routed(&functions, true);
    for (index, function) in functions.iter().enumerate().take(4) {
        assert_eq!(
            legacy.items[index].output, admitted.items[index].output,
            "{function}: another family is unchanged"
        );
    }
    for (index, item) in legacy.items.iter().enumerate().skip(4) {
        let (record, harness) = kani_parts(&item.output);
        assert!(harness.is_none());
        assert!(
            matches!(
                &record.disposition,
                quire_contract_codegen::ObligationDisposition::Unsupported {
                    reason: quire_contract_codegen::UnsupportedObligation::OperationNotRendered { .. }
                }
            ),
            "{record:?}"
        );
        let (record, harness) = kani_parts(&admitted.items[index].output);
        let harness = harness.expect("the enum node is generated");
        assert!(matches!(
            &record.disposition,
            quire_contract_codegen::ObligationDisposition::Supported { harness_symbol }
                if *harness_symbol == harness.identity.harness_symbol
        ));
        assert_eq!(harness.identity.node_id, corpus.node(functions[index]));
    }
}

/// Two enum nodes of the same operator in one routed request keep distinct harness names.
#[trace("TC-063", "FR-049-AC-1")]
#[test]
fn two_enum_nodes_of_one_operator_in_one_request_do_not_collide() {
    let corpus = corpus();
    let generation = corpus.routed(&["eq", "color_eq"], true);
    let symbols = generation
        .items
        .iter()
        .map(|item| {
            let (_, harness) = kani_parts(&item.output);
            harness.expect("supported").identity.harness_symbol.clone()
        })
        .collect::<Vec<_>>();
    assert_ne!(symbols[0], symbols[1]);
}

/// The counterexample of a falsified mutant is decoded to declared members and replayed through
/// the owner route: the owner's result is the native one, so the mutant's disagreement with the
/// native operator is reproduced against the declared semantics.
#[trace("TC-063", "FR-049-AC-5")]
#[trace("TC-063", "FR-049-AC-10")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_a_falsifying_counterexample_replays_through_the_owner_route() {
    let corpus = corpus();
    let generated = corpus.harness("lt");
    let evidence = run(
        "enum-replay",
        "lt",
        Some((
            "rt::ComparisonOperator::Less",
            "rt::ComparisonOperator::LessOrEqual",
        )),
    );
    let KaniRunOutcome::Falsified { counterexample } = evidence.outcome else {
        panic!("the mutant must falsify: {evidence:?}");
    };
    let witness = decode(&generated, &counterexample).expect("the counterexample decodes");
    assert_eq!(
        witness.left_index, witness.right_index,
        "only equal members distinguish < from <="
    );
    let owner = quire_contract_codegen::replay_enum_witness(
        &owner_package("lt"),
        &owner_declaration(&corpus, &generated),
        &witness,
    )
    .expect("the owner route evaluates the members");
    assert!(
        !owner,
        "the owner says lt(m, m) is false; the mutant claimed true"
    );
}

// ---- runner classification bound to the enum harness (FR-049-AC-7, FR-049-AC-11) ----

/// Runs the enum harness through the runner against a stand-in launcher running `body`.
#[cfg(unix)]
fn run_with_launcher(
    name: &str,
    body: &str,
    ceilings: quire_contract_codegen::ProofCeilings,
) -> quire_contract_codegen::KaniExecutionEvidence {
    use std::os::unix::fs::PermissionsExt;

    let corpus = corpus();
    let EnumScalarOutcome::Supported(generated) = corpus.outcome_with("lt", true, ceilings) else {
        panic!("lt must be supported");
    };
    let scratch = crate::kani_obligations::write_scalar_crate(
        name,
        &generated.artifacts[0],
        &generated.harness.rust.contents,
    );
    let directory = &scratch.path;
    std::fs::create_dir_all(directory.join("target")).unwrap();
    let launcher = directory.join("cargo-kani");
    std::fs::write(
        &launcher,
        format!("#!/bin/sh\n[ \"$1\" = --probe ] && exit 0\n{body}"),
    )
    .unwrap();
    std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o755)).unwrap();
    let cancel = quire_exact::Cancel::new();
    quire_contract_codegen::execute_kani_obligation(&quire_contract_codegen::KaniExecutionRequest {
        cancel: &cancel,
        installation: &quire_contract_codegen::KaniInstallation::new(launcher),
        harness: (&generated.harness).into(),
        crate_directory: directory,
        target_directory: &directory.join("target"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("{refusal}"))
}

/// The enum harness's single non-vacuity cover is what the classifier counts: with every other
/// check holding, an unsatisfiable cover is `CoverUnsatisfied`, an absent cover summary and an
/// exhausted unwinding check are inconclusive, and a satisfied cover verifies.
#[trace("TC-063", "FR-049-AC-7")]
#[test]
fn the_classifier_reads_the_enum_harness_cover_as_unsatisfied_missing_or_unwound() {
    let corpus = corpus();
    let generated = corpus.harness("lt");
    let covers = generated
        .harness
        .rust
        .contents
        .matches("kani::cover!(")
        .count();
    assert_eq!(covers, 1);
    let check = |status: &str, category: &str| {
        serde_json::json!({
            "id": 1, "status": status, "category": category,
            "location": { "file": "src/lib.rs", "line": "10", "column": "5" },
        })
    };
    let classify = |status: &str, checks: Vec<serde_json::Value>| {
        let report = serde_json::to_vec(&serde_json::json!({
            "metadata": { "version": "1.0" },
            "verification_results": { "results": [{
                "harness_id": "h", "status": status, "checks": checks
            }] }
        }))
        .unwrap();
        quire_contract_codegen::classify_kani_run(status == "Success", Some(&report), None, None)
            .unwrap()
            .outcome
    };
    let with_cover = |cover: &str| vec![check("Success", "assertion"), check(cover, "cover")];
    assert_eq!(
        classify("Success", with_cover("Satisfied")),
        KaniRunOutcome::Verified
    );
    assert_eq!(
        classify("Success", with_cover("Unsatisfiable")),
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );
    assert_eq!(
        classify("Success", vec![check("Success", "assertion")]),
        KaniRunOutcome::Inconclusive {
            reason: quire_contract_codegen::KaniInconclusiveReason::MissingCoverSummary
        }
    );
    assert_eq!(
        classify(
            "Failure",
            vec![
                check("Failure", "unwind"),
                check("Success", "assertion"),
                check("Satisfied", "cover")
            ]
        ),
        KaniRunOutcome::Inconclusive {
            reason: quire_contract_codegen::KaniInconclusiveReason::UnwindBoundExhausted
        }
    );
}

/// A launcher that never concludes inside the harness's wall clock is `TimedOut` with an
/// `Incomplete` terminal value, and one that dies without a report is `NoVerdict` with `Failed`;
/// neither is ever `Verified`, for the enum harness the runner executes.
#[trace("TC-063", "FR-049-AC-7")]
#[trace("TC-063", "FR-049-AC-11")]
#[cfg(unix)]
#[test]
fn a_timeout_and_a_missing_verdict_settle_the_enum_harness_inconclusive() {
    use quire_contract_codegen::{run_terminal_value, KaniInconclusiveReason};
    let timed = run_with_launcher(
        "enum-timeout",
        "exec sleep 30\n",
        crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            std::time::Duration::from_millis(300),
        ),
    );
    assert_eq!(
        timed.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    assert_eq!(
        run_terminal_value(&timed, None).unwrap(),
        qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::TimedOut)
    );
    assert_eq!(
        timed.ceilings.wall_clock,
        std::time::Duration::from_millis(300)
    );
    let silent = run_with_launcher(
        "enum-no-verdict",
        "printf 'Out of memory\\n' >&2\nexit 1\n",
        crate::common::proof_ceilings::proof_ceilings(),
    );
    assert_eq!(
        silent.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict
        }
    );
    assert_eq!(
        run_terminal_value(&silent, None).unwrap(),
        qsl_replay::TerminalValue::Failed
    );
    assert_eq!(silent.unwind, 1);
    assert_eq!(silent.solver, quire_contract_codegen::KaniSolver::Cadical);
}

/// One extra index makes the carrier constructor reject the draw, so Kani falsifies it.
#[trace("TC-063", "FR-049-AC-2")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_one_extra_index_is_falsified() {
    falsified("lt", "enum-extra-index", "left <= 2_i64", "left <= 3_i64");
}
