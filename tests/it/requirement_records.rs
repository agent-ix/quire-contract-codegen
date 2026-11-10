//! FR-022 per-occurrence requirement records (IR-304 slice (a)).

use ix_trace_rs::trace;
use std::collections::BTreeMap;

use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, generate_routed,
    negotiate_kani_obligations, routed_contract_lowering_profile, ContractDriverResultSelection,
    ContractRequestPosition, GenerationContexts, KaniGenerationContext, KaniObligationOutcome,
    KaniObligationRequest, KindOutput, ObligationDisposition, ObligationItem, PathGuardRefusal,
    RequirementGuard, RequirementOccurrence, RequirementRecordInput, RequirementRecordMismatch,
    RequirementRecords, RoutedGeneration, RoutedGenerationError, RoutedGenerationItem,
    RoutedRequirement, UnsupportedObligation,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies,
    ContractOccurrenceKeyV1, ContractPackageReadResult, ContractPackageV1, ContractPackageWireV1,
};

use crate::kani_obligations::package;
use crate::routed_generation::{admitted_routing_input, route};

/// The persisted record of the harness a direct FR-015 caller gets for `x + 1` over
/// `x: Int[0, 9]` into `Int[0, 10]`.
fn direct_increment_record() -> String {
    let KaniObligationOutcome::Emitted {
        scalar_harnesses, ..
    } = direct_outcome(package::QSL_INC, false)
    else {
        panic!("the request is rejected");
    };
    let [harness] = scalar_harnesses.as_slice() else {
        panic!("one harness, got {scalar_harnesses:?}");
    };
    harness.record.contents.clone()
}

/// The direct FR-015 outcome for the QSL-shaped node `code`, under one guard when `guarded`.
fn direct_outcome(code: u32, guarded: bool) -> KaniObligationOutcome {
    let mut builder = package::qsl_shaped_package();
    builder.expression_occurrences();
    let (checked, ids) = builder.admit_resolved();
    let node_id = ids.resolve(&package::code_id(code));
    let path_condition = [RequirementGuard {
        occurrence: occurrence(&node_id, 0),
        holds: true,
    }];
    let path_condition = if guarded { &path_condition[..] } else { &[] };
    let routed = [route(0, &node_id)];
    let admitted = admitted_routing_input(&checked, &routed);
    let item = derive_exact_scalar_items(&checked, std::slice::from_ref(&node_id))
        .into_iter()
        .next()
        .expect("one requested node is derived")
        .expect("the increment has a source-derived descriptor");
    let claim_map = generate_exact_scalar_oracles(&checked, &[item])
        .expect("the increment renders")
        .claim_map;
    let selection: &ContractDriverResultSelection = admitted
        .positions
        .iter()
        .find_map(|position| match position {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => Some(result_selection),
            _ => None,
        })
        .expect("a driver position");
    let items = [ObligationItem::ScalarClaim {
        path_condition,
        source: &checked,
        package: &admitted.package,
        claim_map: &claim_map,
        node_id: &node_id,
        claim_index: 0,
        result_selection: selection,
    }];
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &items,
        subject_path: "crate::subject",
        unwind: 1,
    })
    .expect("FR-015 accepts the request")
}

#[trace("FR-022-AC-31", "TC-033")]
#[test]
fn direct_increment_record_equals_the_committed_golden() {
    let golden = include_str!("../fixtures/kani-scalar/direct-increment-record.json");
    assert_eq!(direct_increment_record(), golden);
}

fn qsl_package(name: &str, source: &str) -> CheckedPackageV2 {
    let path = format!("{name}.native");
    let compiled = compile_package(
        qsl_replay::CompileUnit {
            source: SourceIdentity::new("test", &path),
            package_name: name.to_owned(),
            path: &path,
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL compiles the source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("IR admits QSL output: {other:?}"),
    }
}

fn admit(
    checked: &CheckedPackageV2,
    lowered: &quire_contract_model::CompleteLoweringResultV2,
) -> ContractPackageV1 {
    match ContractPackageWireV1::read_with_source(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
        checked,
    ) {
        Ok(ContractPackageReadResult::Admitted(package)) => *package,
        other => panic!("IR admits the routed package: {other:?}"),
    }
}

fn operation(node: &quire_contract_model::CheckedSemanticNodeV2) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = &node.body else {
        return None;
    };
    operation.identity()
}

fn first_argument(node: &quire_contract_model::CheckedSemanticNodeV2) -> Option<&CheckedNodeId> {
    let CheckedTerm::Application { arguments, .. } = &node.body else {
        return None;
    };
    match arguments.first()? {
        CheckedTerm::Reference { target } => Some(target),
        _ => None,
    }
}

fn occurrences(checked: &CheckedPackageV2, node: &CheckedNodeId) -> Vec<u64> {
    checked
        .source_map()
        .iter()
        .filter(|entry| &entry.node_id == node && entry.role == CheckedOccurrenceRole::Expression)
        .map(|entry| entry.ordinal)
        .collect()
}

fn occurrence(node: &CheckedNodeId, ordinal: u64) -> RequirementOccurrence {
    RequirementOccurrence {
        node_id: node.clone(),
        role: CheckedOccurrenceRole::Expression,
        ordinal,
    }
}

fn ir_key(occurrence: &RequirementOccurrence) -> ContractOccurrenceKeyV1 {
    ContractOccurrenceKeyV1 {
        node_id: occurrence.node_id.clone(),
        role: occurrence.role.clone(),
        ordinal: occurrence.ordinal,
    }
}

fn narrow_of<'a>(
    checked: &'a CheckedPackageV2,
    application: &CheckedNodeId,
) -> Vec<&'a CheckedNodeId> {
    checked
        .resolved_graph()
        .nodes
        .iter()
        .filter(|node| {
            operation(node) == Some("quire.op.numeric.narrow")
                && first_argument(node) == Some(application)
        })
        .map(|node| &node.node_id)
        .collect()
}

fn semantic_type(checked: &CheckedPackageV2, node: &CheckedNodeId) -> CheckedNodeId {
    checked
        .resolved_graph()
        .nodes
        .iter()
        .find(|candidate| &candidate.node_id == node)
        .expect("node in the source graph")
        .semantic_type
        .clone()
}

/// One routed item: its application occurrence, optional direct narrow and guards. Each guard's
/// node is a context root of the item.
struct ItemSpec {
    application: RequirementOccurrence,
    conversion: Option<RequirementOccurrence>,
    path: Vec<RequirementGuard>,
}

/// An admitted package with its lowering positions, routed items (selection bound, no
/// requirement) and the requirement the driver projects for each.
struct Scenario {
    checked: CheckedPackageV2,
    package: ContractPackageV1,
    positions: Vec<ContractRequestPosition>,
    routes: Vec<RoutedGenerationItem>,
    requirements: Vec<RoutedRequirement>,
}

impl Scenario {
    fn new(checked: CheckedPackageV2, specs: Vec<ItemSpec>) -> Self {
        let mut positions = Vec::new();
        for spec in &specs {
            positions.push(ContractRequestPosition::DriverItem {
                node_id: spec.application.node_id.clone(),
                result_selection: ContractDriverResultSelection::OwnResult {
                    application: ir_key(&spec.application),
                    conversion: spec.conversion.as_ref().map(ir_key),
                },
            });
            for root in spec
                .conversion
                .iter()
                .map(|conversion| &conversion.node_id)
                .chain(spec.path.iter().map(|guard| &guard.occurrence.node_id))
            {
                positions.push(ContractRequestPosition::ContextRoot {
                    node_id: root.clone(),
                });
            }
        }
        let roots = positions
            .iter()
            .map(|position| position.node_id().clone())
            .collect::<Vec<_>>();
        let lowered = checked.lower(&roots, &routed_contract_lowering_profile());
        let package = admit(&checked, &lowered);
        let mut routes = Vec::new();
        let mut requirements = Vec::new();
        for (index, spec) in specs.iter().enumerate() {
            let selection = ContractDriverResultSelection::OwnResult {
                application: ir_key(&spec.application),
                conversion: spec.conversion.as_ref().map(ir_key),
            };
            routes.push(
                route(index, &spec.application.node_id).with_scalar_selection(&package, selection),
            );
            requirements.push(RoutedRequirement {
                occurrence: spec.application.clone(),
                result_bound: semantic_type(
                    &checked,
                    &spec
                        .conversion
                        .as_ref()
                        .unwrap_or(&spec.application)
                        .node_id,
                ),
                conversion_context: spec.conversion.clone(),
                path_condition: spec.path.clone(),
            });
        }
        Self {
            checked,
            package,
            positions,
            routes,
            requirements,
        }
    }

    /// The records QSL writes for the scenario's requirements.
    fn records(&self) -> BTreeMap<RequirementOccurrence, RequirementRecordInput> {
        self.requirements
            .iter()
            .map(|requirement| {
                (
                    requirement.occurrence.clone(),
                    RequirementRecordInput {
                        result_bound: requirement.result_bound.clone(),
                        conversion_context: requirement.conversion_context.clone(),
                        path_condition: requirement.path_condition.clone(),
                    },
                )
            })
            .collect()
    }

    /// The routed items carrying `requirements` (`None` leaves an item without one).
    fn bound(&self, requirements: &[Option<RoutedRequirement>]) -> Vec<RoutedGenerationItem> {
        self.routes
            .iter()
            .cloned()
            .zip(requirements)
            .map(|(route, requirement)| match requirement {
                Some(requirement) => route.with_requirement(requirement.clone()),
                None => route,
            })
            .collect()
    }

    fn valid(&self) -> Vec<RoutedGenerationItem> {
        self.bound(
            &self
                .requirements
                .iter()
                .cloned()
                .map(Some)
                .collect::<Vec<_>>(),
        )
    }

    fn generate(
        &self,
        routed: &[RoutedGenerationItem],
        records: BTreeMap<RequirementOccurrence, RequirementRecordInput>,
    ) -> Result<RoutedGeneration, RoutedGenerationError> {
        let records = RequirementRecords::new(records);
        generate_routed(
            &self.checked,
            &self.package,
            &self.positions,
            routed,
            &GenerationContexts {
                kani: Some(KaniGenerationContext {
                    ceilings: crate::common::proof_ceilings::proof_ceilings(),
                    subject_path: "crate::subject",
                    unwind: 1,
                    requirement_records: &records,
                }),
            },
        )
    }

    fn refusal(
        &self,
        routed: &[RoutedGenerationItem],
        records: BTreeMap<RequirementOccurrence, RequirementRecordInput>,
    ) -> RoutedGenerationError {
        self.generate(routed, records)
            .expect_err("the whole call is refused")
    }
}

const RR16: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    function g using v(p: Int[0, 10]): Boolean pure { true }\n\
    function h using v(q: Int[0, 20]): Boolean pure { true }\n\
    function f using v(x: Int[0, 9]): Boolean pure { g(x + 1) and h(x + 1) }\n";

/// The checked RR-16 package: one `x + 1` node with two occurrences, directly narrowed to
/// `Int[0, 10]` and `Int[0, 20]`.
fn rr16() -> Scenario {
    let checked = qsl_package("rr16", RR16);
    let add = checked
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| operation(node) == Some("quire.op.integer.add"))
        .expect("the shared addition")
        .node_id
        .clone();
    let ordinals = occurrences(&checked, &add);
    let [first, second] = ordinals.as_slice() else {
        panic!("RR-16 retains two occurrences of one node");
    };
    let mut conversions = narrow_of(&checked, &add)
        .into_iter()
        .map(|conversion| {
            let bound = semantic_type(&checked, conversion);
            let text = format!("{:?}", bound.path);
            (text, conversion.clone())
        })
        .collect::<Vec<_>>();
    conversions.sort();
    let [(_, ten), (_, twenty)] = conversions.as_slice() else {
        panic!("RR-16 has two direct narrows");
    };
    let ten_ordinal = occurrences(&checked, ten)[0];
    let twenty_ordinal = occurrences(&checked, twenty)[0];
    Scenario::new(
        checked,
        vec![
            ItemSpec {
                application: occurrence(&add, *first),
                conversion: Some(occurrence(ten, ten_ordinal)),
                path: Vec::new(),
            },
            ItemSpec {
                application: occurrence(&add, *second),
                conversion: Some(occurrence(twenty, twenty_ordinal)),
                path: Vec::new(),
            },
        ],
    )
}

fn record_refusal(
    request_index: usize,
    occurrence: Option<RequirementOccurrence>,
    mismatch: RequirementRecordMismatch,
) -> RoutedGenerationError {
    RoutedGenerationError::RequirementRecord {
        request_index,
        occurrence,
        mismatch: Box::new(mismatch),
    }
}

fn unboxed(error: &RoutedGenerationError) -> Option<(usize, RequirementRecordMismatch)> {
    match error {
        RoutedGenerationError::RequirementRecord {
            request_index,
            mismatch,
            ..
        } => Some((*request_index, (**mismatch).clone())),
        _ => None,
    }
}

fn kani_record(
    generation: &RoutedGeneration,
    request_index: usize,
) -> (
    &quire_contract_codegen::ObligationRecord,
    Option<&quire_contract_codegen::KaniScalarObligationHarness>,
) {
    let item = generation
        .items
        .iter()
        .find(|item| item.request_index == request_index)
        .expect("a routed item at the request index");
    crate::routed_generation::kani_parts(&item.output)
}

fn asserts_bound(
    harness: &quire_contract_codegen::KaniScalarObligationHarness,
    upper: i64,
) -> bool {
    harness.rust.contents.contains(&format!(
        "exact >= i128::from(0_i64) && exact <= i128::from({upper}_i64);"
    ))
}

#[trace("FR-022-AC-26", "TC-033")]
#[test]
fn each_rr16_occurrence_pairs_with_its_own_record_and_bound() {
    let scenario = rr16();
    let routed = scenario.valid();
    // A raw cross-frame comparison would refuse this valid input: the admitted result-bound
    // nodes are in the represented frame, the records' are in the V2 frame.
    let admitted = scenario
        .package
        .wire()
        .semantic_graph
        .nodes
        .iter()
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    for requirement in &scenario.requirements {
        assert!(!admitted.contains(&requirement.result_bound));
    }
    let generation = scenario
        .generate(&routed, scenario.records())
        .expect("both occurrences route");
    for (index, upper) in [(0, 10), (1, 20)] {
        let (record, harness) = kani_record(&generation, index);
        assert!(
            matches!(record.disposition, ObligationDisposition::Supported { .. }),
            "{record:?}"
        );
        assert!(asserts_bound(harness.expect("a harness"), upper));
    }
    let reversed = routed.iter().rev().cloned().collect::<Vec<_>>();
    assert_eq!(
        scenario
            .generate(&reversed, scenario.records())
            .expect("reversed routing"),
        generation
    );

    let mut exchanged = scenario.requirements.clone();
    let (left, right) = exchanged.split_at_mut(1);
    std::mem::swap(&mut left[0], &mut right[0]);
    let exchanged = scenario.bound(&exchanged.into_iter().map(Some).collect::<Vec<_>>());
    assert_eq!(
        scenario.refusal(&exchanged, scenario.records()),
        record_refusal(
            0,
            Some(scenario.requirements[1].occurrence.clone()),
            RequirementRecordMismatch::ApplicationMismatch {
                selected: scenario.requirements[0].occurrence.clone(),
            }
        )
    );
}

/// Each single change refuses the whole call at the changed item, with the other occurrence's
/// record of the same node present throughout.
#[trace("FR-022-AC-28", "TC-033")]
#[test]
fn each_single_record_change_refuses_the_whole_call() {
    let scenario = rr16();
    let [first, second] = scenario.requirements.as_slice() else {
        panic!("two requirements");
    };
    let refused = |index: usize,
                   requirement: Option<RoutedRequirement>,
                   records: BTreeMap<RequirementOccurrence, RequirementRecordInput>,
                   mismatch: RequirementRecordMismatch| {
        let mut requirements = scenario
            .requirements
            .iter()
            .cloned()
            .map(Some)
            .collect::<Vec<_>>();
        let occurrence = requirement.as_ref().map(|r| r.occurrence.clone());
        requirements[index] = requirement;
        assert_eq!(
            scenario.refusal(&scenario.bound(&requirements), records),
            RoutedGenerationError::RequirementRecord {
                request_index: index,
                occurrence,
                mismatch: Box::new(mismatch),
            }
        );
    };
    // removing the requirement
    refused(
        1,
        None,
        scenario.records(),
        RequirementRecordMismatch::RequirementMissing,
    );
    // removing the record
    let mut records = scenario.records();
    records.remove(&first.occurrence);
    refused(
        0,
        Some(first.clone()),
        records,
        RequirementRecordMismatch::RecordMissing,
    );
    // an occurrence that has a record but is not the authenticated application
    let mut other = first.clone();
    other.occurrence = second.occurrence.clone();
    refused(
        0,
        Some(other),
        scenario.records(),
        RequirementRecordMismatch::ApplicationMismatch {
            selected: first.occurrence.clone(),
        },
    );
    // the item's result bound
    let mut item_bound = first.clone();
    item_bound.result_bound = second.result_bound.clone();
    refused(
        0,
        Some(item_bound),
        scenario.records(),
        RequirementRecordMismatch::ResultBoundMismatch {
            item: second.result_bound.clone(),
            record: first.result_bound.clone(),
            selected: Some(first.result_bound.clone()),
        },
    );
    // the record's result bound
    let mut records = scenario.records();
    records
        .get_mut(&first.occurrence)
        .expect("first record")
        .result_bound = second.result_bound.clone();
    refused(
        0,
        Some(first.clone()),
        records,
        RequirementRecordMismatch::ResultBoundMismatch {
            item: first.result_bound.clone(),
            record: second.result_bound.clone(),
            selected: Some(first.result_bound.clone()),
        },
    );
    // the item's conversion context
    let mut item_conversion = first.clone();
    item_conversion.conversion_context = second.conversion_context.clone();
    refused(
        0,
        Some(item_conversion),
        scenario.records(),
        RequirementRecordMismatch::ConversionContextMismatch {
            item: second.conversion_context.clone(),
            record: first.conversion_context.clone(),
            selected: first.conversion_context.clone(),
        },
    );
    // the record's conversion context
    let mut records = scenario.records();
    records
        .get_mut(&first.occurrence)
        .expect("first record")
        .conversion_context = second.conversion_context.clone();
    refused(
        0,
        Some(first.clone()),
        records,
        RequirementRecordMismatch::ConversionContextMismatch {
            item: first.conversion_context.clone(),
            record: second.conversion_context.clone(),
            selected: first.conversion_context.clone(),
        },
    );
    // a guard added to the record only
    let guard = RequirementGuard {
        occurrence: occurrence(&first.occurrence.node_id, 7),
        holds: true,
    };
    let mut records = scenario.records();
    records
        .get_mut(&first.occurrence)
        .expect("first record")
        .path_condition = vec![guard.clone()];
    refused(
        0,
        Some(first.clone()),
        records,
        RequirementRecordMismatch::PathConditionMismatch {
            item: Vec::new(),
            record: vec![guard],
        },
    );
    // item and record agree with each other but not with the authenticated selection
    let mut agreeing = first.clone();
    agreeing.result_bound = second.result_bound.clone();
    let mut records = scenario.records();
    records
        .get_mut(&first.occurrence)
        .expect("first record")
        .result_bound = second.result_bound.clone();
    refused(
        0,
        Some(agreeing),
        records,
        RequirementRecordMismatch::ResultBoundMismatch {
            item: second.result_bound.clone(),
            record: second.result_bound.clone(),
            selected: Some(first.result_bound.clone()),
        },
    );
    let mut agreeing = first.clone();
    agreeing.conversion_context = second.conversion_context.clone();
    let mut records = scenario.records();
    records
        .get_mut(&first.occurrence)
        .expect("first record")
        .conversion_context = second.conversion_context.clone();
    refused(
        0,
        Some(agreeing),
        records,
        RequirementRecordMismatch::ConversionContextMismatch {
            item: second.conversion_context.clone(),
            record: second.conversion_context.clone(),
            selected: first.conversion_context.clone(),
        },
    );
    // same-length path conditions that differ in the guard's value, occurrence or order
    let at = |ordinal: u64, holds: bool| RequirementGuard {
        occurrence: occurrence(&first.occurrence.node_id, ordinal),
        holds,
    };
    for (item_path, record_path) in [
        (vec![at(7, true)], vec![at(7, false)]),
        (vec![at(7, true)], vec![at(8, true)]),
        (
            vec![at(7, true), at(8, true)],
            vec![at(8, true), at(7, true)],
        ),
    ] {
        let mut with_path = first.clone();
        with_path.path_condition = item_path.clone();
        let mut records = scenario.records();
        records
            .get_mut(&first.occurrence)
            .expect("first record")
            .path_condition = record_path.clone();
        refused(
            0,
            Some(with_path),
            records,
            RequirementRecordMismatch::PathConditionMismatch {
                item: item_path,
                record: record_path,
            },
        );
    }
    // two items fail: the lower request index is reported
    let mut records = scenario.records();
    records.remove(&first.occurrence);
    records.remove(&second.occurrence);
    let error = scenario.refusal(&scenario.valid(), records);
    assert!(matches!(
        unboxed(&error),
        Some((0, RequirementRecordMismatch::RecordMissing))
    ));
}

/// The first failing check of one item is the one reported, and an earlier loop's failure at a
/// higher index precedes a later loop's at a lower one.
#[trace("FR-022-AC-37", "TC-033")]
#[test]
fn record_checks_follow_the_fixed_order() {
    let scenario = rr16();
    let [first, second] = scenario.requirements.as_slice() else {
        panic!("two requirements");
    };
    // ScalarSelectionMismatch at the higher index precedes RecordMissing at the lower.
    let mut routed = scenario.valid();
    routed[1] = scenario.routes[1]
        .clone()
        .with_scalar_selection(
            &scenario.package,
            ContractDriverResultSelection::NotApplicable,
        )
        .with_requirement(second.clone());
    let mut records = scenario.records();
    records.remove(&first.occurrence);
    assert_eq!(
        scenario.refusal(&routed, records),
        RoutedGenerationError::ScalarSelectionMismatch { request_index: 1 }
    );
    let guard = RequirementGuard {
        occurrence: occurrence(&first.occurrence.node_id, 7),
        holds: true,
    };
    // a wrong result bound together with a wrong path condition: ResultBoundMismatch
    let mut wrong = first.clone();
    wrong.result_bound = second.result_bound.clone();
    wrong.path_condition = vec![guard];
    let routed = scenario.bound(&[Some(wrong), Some(second.clone())]);
    assert!(matches!(
        unboxed(&scenario.refusal(&routed, scenario.records())),
        Some((0, RequirementRecordMismatch::ResultBoundMismatch { .. }))
    ));
    // the other recorded occurrence's key together with a wrong conversion context
    let mut wrong = first.clone();
    wrong.occurrence = second.occurrence.clone();
    wrong.conversion_context = second.conversion_context.clone();
    let routed = scenario.bound(&[Some(wrong), Some(second.clone())]);
    assert!(matches!(
        unboxed(&scenario.refusal(&routed, scenario.records())),
        Some((0, RequirementRecordMismatch::ApplicationMismatch { .. }))
    ));
    // an ordinal with no record
    let mut wrong = first.clone();
    wrong.occurrence = occurrence(&first.occurrence.node_id, 99);
    let routed = scenario.bound(&[Some(wrong.clone()), Some(second.clone())]);
    assert_eq!(
        scenario.refusal(&routed, scenario.records()),
        record_refusal(
            0,
            Some(wrong.occurrence),
            RequirementRecordMismatch::RecordMissing
        )
    );
}

const GUARDED: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    function f using v(p: Boolean, x: Int[0, 9]): Int[0, 10] pure { if p then x + 1 else x + 1 }\n\
    function g using v(x: Int[0, 9]): Int[0, 10] pure { x + 1 }\n\
    function h using v(x: Int[0, 9]): Int[0, 200] pure { if x > 5 then x + 100 else 0 }\n\
    function k using v(p: Boolean, x: Int[0, 9], m: Integer): Integer pure { if p then x + m else 0 }\n\
    function c using v(flag: Boolean, n: Int[0, 9]): Boolean pure { if flag then true else n >= 0 }\n";

struct Guarded {
    f_add: CheckedNodeId,
    f_conversion: CheckedNodeId,
    p: CheckedNodeId,
    g_add: CheckedNodeId,
    g_conversion: CheckedNodeId,
    h_add: CheckedNodeId,
    h_conversion: CheckedNodeId,
    gt: CheckedNodeId,
    checked: CheckedPackageV2,
}

fn guarded() -> Guarded {
    let checked = qsl_package("ir304-guarded", GUARDED);
    let nodes = &checked.resolved_graph().nodes;
    let adds = nodes
        .iter()
        .filter(|node| operation(node) == Some("quire.op.integer.add"))
        .collect::<Vec<_>>();
    let by_text = |needle: &str| {
        adds.iter()
            .filter(|node| format!("{:?}", node.node_id.path).contains(needle))
            .map(|node| node.node_id.clone())
            .collect::<Vec<_>>()
    };
    let h_add = by_text("\"100\"").remove(0);
    let f_add = adds
        .iter()
        .find(|node| occurrences(&checked, &node.node_id).len() == 2)
        .expect("f's shared addition")
        .node_id
        .clone();
    let g_add = adds
        .iter()
        .map(|node| node.node_id.clone())
        .find(|id| {
            *id != h_add
                && *id != f_add
                && !narrow_of(&checked, id).is_empty()
                && format!(
                    "{:?}",
                    semantic_type(&checked, narrow_of(&checked, id)[0]).path
                )
                .contains("\"10\"")
        })
        .expect("g's addition");
    let conversion = |add: &CheckedNodeId| narrow_of(&checked, add).remove(0).clone();
    let p = nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "parameter"
                && occurrences(&checked, &node.node_id).len() >= 2
                && format!("{:?}", node.body).contains("value: \"p\"")
        })
        .expect("f's Boolean guard parameter")
        .node_id
        .clone();
    let gt = nodes
        .iter()
        .find(|node| operation(node) == Some("quire.op.integer.gt"))
        .expect("h's comparison")
        .node_id
        .clone();
    Guarded {
        f_conversion: conversion(&f_add),
        g_conversion: conversion(&g_add),
        h_conversion: conversion(&h_add),
        f_add,
        g_add,
        h_add,
        p,
        gt,
        checked,
    }
}

fn conv_occurrence(
    checked: &CheckedPackageV2,
    node: &CheckedNodeId,
    ordinal: usize,
) -> RequirementOccurrence {
    occurrence(node, occurrences(checked, node)[ordinal])
}

/// Slice (a): a non-empty path condition settles per item as `path_guard_not_lowered` with
/// `guard_lowering_not_implemented` naming its first guard, whatever the guard's form, and an
/// unguarded sibling keeps its harness.
#[trace("FR-022-AC-38", "TC-033")]
#[test]
fn guarded_items_settle_unsupported_and_the_unguarded_sibling_keeps_its_harness() {
    let guarded = guarded();
    let f_guard = RequirementGuard {
        occurrence: conv_occurrence(&guarded.checked, &guarded.p, 0),
        holds: true,
    };
    let h_guard = RequirementGuard {
        occurrence: conv_occurrence(&guarded.checked, &guarded.gt, 0),
        holds: true,
    };
    let specs = vec![
        ItemSpec {
            application: conv_occurrence(&guarded.checked, &guarded.f_add, 0),
            conversion: Some(conv_occurrence(&guarded.checked, &guarded.f_conversion, 0)),
            path: vec![f_guard.clone()],
        },
        ItemSpec {
            application: conv_occurrence(&guarded.checked, &guarded.h_add, 0),
            conversion: Some(conv_occurrence(&guarded.checked, &guarded.h_conversion, 0)),
            path: vec![h_guard.clone()],
        },
        ItemSpec {
            application: conv_occurrence(&guarded.checked, &guarded.g_add, 0),
            conversion: Some(conv_occurrence(&guarded.checked, &guarded.g_conversion, 0)),
            path: Vec::new(),
        },
    ];
    let scenario = Scenario::new(guarded.checked, specs);
    let generation = scenario
        .generate(&scenario.valid(), scenario.records())
        .expect("valid records route");
    for (index, guard) in [(0, &f_guard), (1, &h_guard)] {
        let (record, harness) = kani_record(&generation, index);
        assert_eq!(
            record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::PathGuardNotLowered {
                    guard: guard.occurrence.clone(),
                    reason: PathGuardRefusal::GuardLoweringNotImplemented,
                }
            }
        );
        assert!(harness.is_none());
        let wire = serde_json::to_value(&record.disposition).expect("serializes");
        assert_eq!(wire["reason"]["code"], "path_guard_not_lowered");
        assert_eq!(wire["reason"]["reason"], "guard_lowering_not_implemented");
    }
    let (record, harness) = kani_record(&generation, 2);
    assert!(matches!(
        record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert!(harness.is_some());
}

/// A `NotApplicable` item authenticates no occurrence, so a node with records is refused with or
/// without a requirement, and a node without records keeps today's unguarded behaviour.
#[trace("FR-022-AC-36", "TC-033")]
#[test]
fn not_applicable_items_with_records_are_refused() {
    let guarded = guarded();
    let guard_p = |ordinal: usize, holds: bool| RequirementGuard {
        occurrence: conv_occurrence(&guarded.checked, &guarded.p, ordinal),
        holds,
    };
    let specs = vec![
        ItemSpec {
            application: conv_occurrence(&guarded.checked, &guarded.f_add, 0),
            conversion: Some(conv_occurrence(&guarded.checked, &guarded.f_conversion, 0)),
            path: vec![guard_p(0, true)],
        },
        ItemSpec {
            application: conv_occurrence(&guarded.checked, &guarded.f_add, 1),
            conversion: Some(conv_occurrence(&guarded.checked, &guarded.f_conversion, 1)),
            path: vec![guard_p(1, false)],
        },
    ];
    let f_add = guarded.f_add.clone();
    let g_add = guarded.g_add.clone();
    let scenario = Scenario::new(guarded.checked, specs);
    let records = scenario.records();
    assert_eq!(records.len(), 2);

    // the same node, routed `NotApplicable`
    let not_applicable = |requirement: Option<RoutedRequirement>| {
        let positions = vec![ContractRequestPosition::DriverItem {
            node_id: f_add.clone(),
            result_selection: ContractDriverResultSelection::NotApplicable,
        }];
        let lowered = scenario.checked.lower(
            std::slice::from_ref(&f_add),
            &routed_contract_lowering_profile(),
        );
        let package = admit(&scenario.checked, &lowered);
        let mut item = route(0, &f_add)
            .with_scalar_selection(&package, ContractDriverResultSelection::NotApplicable);
        if let Some(requirement) = requirement {
            item = item.with_requirement(requirement);
        }
        let records = RequirementRecords::new(records.clone());
        generate_routed(
            &scenario.checked,
            &package,
            &positions,
            &[item],
            &GenerationContexts {
                kani: Some(KaniGenerationContext {
                    ceilings: crate::common::proof_ceilings::proof_ceilings(),
                    subject_path: "crate::subject",
                    unwind: 1,
                    requirement_records: &records,
                }),
            },
        )
    };
    assert_eq!(
        not_applicable(None).expect_err("records exist for the node"),
        record_refusal(0, None, RequirementRecordMismatch::OccurrenceNotSelected)
    );
    assert_eq!(
        not_applicable(Some(scenario.requirements[0].clone()))
            .expect_err("a requirement does not select an occurrence"),
        record_refusal(
            0,
            Some(scenario.requirements[0].occurrence.clone()),
            RequirementRecordMismatch::OccurrenceNotSelected
        )
    );

    // a node with no record and no requirement keeps its unguarded behaviour
    let lowered = scenario.checked.lower(
        std::slice::from_ref(&g_add),
        &routed_contract_lowering_profile(),
    );
    let package = admit(&scenario.checked, &lowered);
    let control = route(0, &g_add)
        .with_scalar_selection(&package, ContractDriverResultSelection::NotApplicable);
    let records = RequirementRecords::new(records);
    let generation = generate_routed(
        &scenario.checked,
        &package,
        &[ContractRequestPosition::DriverItem {
            node_id: g_add.clone(),
            result_selection: ContractDriverResultSelection::NotApplicable,
        }],
        &[control],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: crate::common::proof_ceilings::proof_ceilings(),
                subject_path: "crate::subject",
                unwind: 1,
                requirement_records: &records,
            }),
        },
    )
    .expect("a node with no records is not refused");
    assert!(matches!(
        &generation.items[0].output,
        KindOutput::Kani { .. }
    ));
}

/// An FR-014 refusal or a missing renderer is reported for that cause, whatever the guards.
#[trace("FR-022-AC-37", "TC-033")]
#[test]
fn claim_refusals_precede_the_guard_refusal() {
    let guarded = guarded();
    let nodes = &guarded.checked.resolved_graph().nodes;
    let k_add = nodes
        .iter()
        .find(|node| {
            operation(node) == Some("quire.op.integer.add")
                && narrow_of(&guarded.checked, &node.node_id).is_empty()
                && occurrences(&guarded.checked, &node.node_id).len() == 1
        })
        .expect("k's plain-Integer addition")
        .node_id
        .clone();
    let ge = nodes
        .iter()
        .find(|node| operation(node) == Some("quire.op.integer.ge"))
        .expect("c's comparison")
        .node_id
        .clone();
    let flag = nodes
        .iter()
        .find(|node| format!("{:?}", node.body).contains("value: \"flag\""))
        .expect("c's guard parameter")
        .node_id
        .clone();
    let p = guarded.p.clone();
    let checked = qsl_package("ir304-guarded", GUARDED);
    let guard = |node: &CheckedNodeId, holds: bool| RequirementGuard {
        occurrence: conv_occurrence(&checked, node, 0),
        holds,
    };
    let specs = vec![
        ItemSpec {
            application: conv_occurrence(&checked, &k_add, 0),
            conversion: None,
            path: vec![guard(&p, true)],
        },
        ItemSpec {
            application: conv_occurrence(&checked, &ge, 0),
            conversion: None,
            path: vec![guard(&flag, false)],
        },
    ];
    let scenario = Scenario::new(checked, specs);
    let generation = scenario
        .generate(&scenario.valid(), scenario.records())
        .expect("valid records route");
    assert!(matches!(
        kani_record(&generation, 0).0.disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    assert!(matches!(
        &kani_record(&generation, 1).0.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::OperationNotRendered { .. }
        }
    ));
}

/// The guard refusal is reported ahead of `domain_not_representable_in_i64` and
/// `result_bound_unreachable`, which the same node without a guard is settled as.
#[trace("FR-022-AC-38", "TC-033")]
#[test]
fn guard_refusal_precedes_the_bound_and_reachability_refusals() {
    for (code, plain) in [
        (package::QSL_PLUS_HUNDRED, "result_bound_unreachable"),
        (package::QSL_PLUS_HUGE, "domain_not_representable_in_i64"),
    ] {
        let reason = |guarded: bool| {
            let KaniObligationOutcome::Emitted { records, .. } = direct_outcome(code, guarded)
            else {
                panic!("the request is rejected");
            };
            let [record] = records.as_slice() else {
                panic!("one record");
            };
            serde_json::to_value(&record.disposition).expect("serializes")["reason"].clone()
        };
        assert_eq!(reason(false)["code"], plain);
        let guarded = reason(true);
        assert_eq!(guarded["code"], "path_guard_not_lowered");
        assert_eq!(guarded["reason"], "guard_lowering_not_implemented");
    }
}
