//! FR-022 per-occurrence requirement records (IR-304 slice (a)).

use ix_trace_rs::trace;
use std::collections::BTreeMap;

use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, generate_routed,
    negotiate_kani_obligations, routed_contract_lowering_profile, ContractDriverResultSelection,
    ContractRequestPosition, GenerationContexts, KaniGenerationContext, KaniObligationOutcome,
    KaniObligationRequest, KindOutput, ObligationDisposition, ObligationItem, RequirementGuard,
    RequirementOccurrence, RequirementRecordInput, RequirementRecordMismatch, RequirementRecords,
    RoutedGeneration, RoutedGenerationError, RoutedGenerationItem, RoutedRequirement,
    UnsupportedObligation,
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
    direct_outcome_guarded(code, guarded.then_some(code))
}

/// The direct FR-015 outcome for the node `code`, under one guard on the node `guard`.
fn direct_outcome_guarded(code: u32, guard: Option<u32>) -> KaniObligationOutcome {
    let mut builder = package::qsl_shaped_package();
    builder.expression_occurrences();
    let (checked, ids) = builder.admit_resolved();
    let node_id = ids.resolve(&package::code_id(code));
    let path_condition = guard
        .map(|guard| RequirementGuard {
            occurrence: occurrence(&ids.resolve(&package::code_id(guard)), 0),
            holds: true,
        })
        .into_iter()
        .collect::<Vec<_>>();
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
        path_condition: &path_condition,
        requirement_occurrence: None,
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
        Self::with_profile(checked, specs, &routed_contract_lowering_profile())
    }

    fn with_profile(
        checked: CheckedPackageV2,
        specs: Vec<ItemSpec>,
        profile: &quire_contract_model::CompleteLoweringProfileV2,
    ) -> Self {
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
        let lowered = checked.lower(&roots, profile);
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
    Guarded {
        f_conversion: conversion(&f_add),
        f_add,
        g_add,
        p,
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

    // routing the same node `OwnResult` with its own requirement is accepted
    let accepted = scenario
        .generate(&scenario.valid(), records.clone())
        .expect("OwnResult items with their own requirements are accepted");
    for index in 0..scenario.requirements.len() {
        assert!(matches!(
            kani_record(&accepted, index).0.disposition,
            ObligationDisposition::Supported { .. }
        ));
    }

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
#[trace("FR-022-AC-32", "TC-033")]
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
        assert_eq!(guarded["reason"], "unsupported_guard_form");
    }
}

// ---- IR-304 slice (b): guard lowering ----

type NodePredicate<'a> = &'a dyn Fn(&quire_contract_model::CheckedSemanticNodeV2) -> bool;
/// A guard: the node it reads, its `holds` and which of the node's occurrences it is.
type GuardSpec<'a> = (NodePredicate<'a>, bool, usize);

fn param_named(
    name: &'static str,
) -> impl Fn(&quire_contract_model::CheckedSemanticNodeV2) -> bool {
    move |node| {
        node.semantic_form.as_ref() == "parameter"
            && format!("{:?}", node.body).contains(&format!("value: \"{name}\""))
    }
}

fn op_is(identity: &'static str) -> impl Fn(&quire_contract_model::CheckedSemanticNodeV2) -> bool {
    move |node| operation(node) == Some(identity)
}

fn is_plus(addend: &'static str) -> impl Fn(&quire_contract_model::CheckedSemanticNodeV2) -> bool {
    move |node| {
        operation(node) == Some("quire.op.integer.add")
            && format!("{:?}", node.node_id.path).contains(&format!("\"{addend}\""))
    }
}

fn find_node(checked: &CheckedPackageV2, predicate: NodePredicate) -> CheckedNodeId {
    let matches = checked
        .resolved_graph()
        .nodes
        .iter()
        .filter(|node| predicate(node))
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "one node selected, got {matches:?}");
    matches[0].clone()
}

fn item_spec(
    checked: &CheckedPackageV2,
    application: NodePredicate,
    guards: &[GuardSpec],
) -> ItemSpec {
    let add = find_node(checked, application);
    let conversion = narrow_of(checked, &add).first().map(|node| (*node).clone());
    ItemSpec {
        application: conv_occurrence(checked, &add, 0),
        conversion: conversion.map(|node| conv_occurrence(checked, &node, 0)),
        path: guards
            .iter()
            .map(|(predicate, holds, ordinal)| RequirementGuard {
                occurrence: conv_occurrence(checked, &find_node(checked, *predicate), *ordinal),
                holds: *holds,
            })
            .collect(),
    }
}

/// A package whose routed items are the `+` applications selected by predicate, each converted to
/// its declared result and routed under its guards.
fn guard_items(name: &str, source: &str, items: &[(NodePredicate, &[GuardSpec])]) -> Scenario {
    let checked = qsl_package(name, source);
    let specs = items
        .iter()
        .map(|(application, guards)| item_spec(&checked, *application, guards))
        .collect();
    Scenario::new(checked, specs)
}

/// One routed item under `guards`, each the first occurrence of its node.
fn guard_scenario(
    name: &str,
    source: &str,
    application: NodePredicate,
    guards: &[(NodePredicate, bool)],
) -> Scenario {
    let guards = guards
        .iter()
        .map(|(predicate, holds)| (*predicate, *holds, 0))
        .collect::<Vec<_>>();
    guard_items(name, source, &[(application, &guards)])
}

fn guard_fn(signature: &str, body: &str) -> String {
    let returns = if body.contains("100") {
        "Int[0, 200]"
    } else {
        "Int[0, 10]"
    };
    format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction t using v({signature}): {returns} pure {{ {body} }}\n"
    )
}

type Settled = (
    serde_json::Value,
    Option<quire_contract_codegen::KaniScalarObligationHarness>,
);

/// The disposition JSON and harness of the scenario's item at `index`.
fn settle_at(scenario: &Scenario, index: usize) -> Settled {
    let generation = scenario
        .generate(&scenario.valid(), scenario.records())
        .expect("valid records route");
    let (record, harness) = kani_record(&generation, index);
    (
        serde_json::to_value(&record.disposition).expect("serializes"),
        harness.cloned(),
    )
}

fn settle(scenario: &Scenario) -> Settled {
    settle_at(scenario, 0)
}

fn args(harness: &quire_contract_codegen::KaniScalarObligationHarness) -> Vec<(i64, i64)> {
    harness
        .identity
        .arguments
        .iter()
        .map(|argument| (argument.minimum, argument.maximum))
        .collect()
}

/// The guard assumptions of a generated harness: every `kani::assume` that is not an
/// operand-range or input-domain bound, in source order.
fn guard_assumptions(source: &str) -> Vec<String> {
    source
        .lines()
        .map(str::trim)
        .filter(|line| {
            line.starts_with("kani::assume(") && !(line.contains(" >= ") && line.contains(" <= "))
        })
        .map(str::to_owned)
        .collect()
}

/// The harness of `scenario`'s first item, with its guard assumptions asserted to precede the
/// oracle call.
fn guarded_harness(
    scenario: &Scenario,
) -> (
    quire_contract_codegen::KaniScalarObligationHarness,
    Vec<String>,
) {
    let (disposition, harness) = settle(scenario);
    let harness = harness.unwrap_or_else(|| panic!("a harness, got {disposition}"));
    let text = harness.rust.contents.clone();
    let assumptions = guard_assumptions(&text);
    let call = text.find("let outcome = ").expect("the oracle call");
    for assumption in &assumptions {
        assert_eq!(text.matches(assumption.as_str()).count(), 1, "{assumption}");
        assert!(text.find(assumption.as_str()).expect("present") < call);
    }
    (harness, assumptions)
}

fn json<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).expect("serializes")
}

#[trace("FR-022-AC-27", "TC-033")]
#[test]
fn boolean_guards_lower_to_assumptions_in_path_order() {
    let source = guard_fn("p: Boolean, x: Int[0, 9]", "if p then x + 1 else x + 1");
    let routed = |name: &str, holds| {
        let scenario = guard_scenario(name, &source, &is_plus("1"), &[(&param_named("p"), holds)]);
        let (harness, assumptions) = guarded_harness(&scenario);
        (scenario, harness, assumptions)
    };
    let (then, then_harness, assumptions) = routed("ir304-then", true);
    assert_eq!(assumptions, ["kani::assume(guard_0);"]);
    let p = find_node(&then.checked, &param_named("p"));
    assert_eq!(then_harness.identity.guard_inputs.len(), 1);
    assert_eq!(then_harness.identity.guard_inputs[0].parameter, p);
    assert_eq!(
        then_harness.identity.path_condition,
        then.requirements[0].path_condition
    );
    assert_eq!(
        then_harness.identity.requirement_occurrence,
        Some(then.requirements[0].occurrence.clone())
    );
    let (_, _, assumptions) = routed("ir304-else", false);
    assert_eq!(assumptions, ["kani::assume(!guard_0);"]);

    // A nested path assumes the outer guard first and draws its inputs in the same order.
    let nested_source = guard_fn(
        "p: Boolean, q: Boolean, x: Int[0, 9]",
        "if p then (if q then x + 1 else 0) else 0",
    );
    let nested = |name: &str, guards: &[(NodePredicate, bool)]| {
        let scenario = guard_scenario(name, &nested_source, &is_plus("1"), guards);
        let (harness, assumptions) = guarded_harness(&scenario);
        (scenario, harness, assumptions)
    };
    let (scenario, harness, assumptions) = nested(
        "ir304-nested",
        &[(&param_named("p"), true), (&param_named("q"), false)],
    );
    assert_eq!(
        assumptions,
        ["kani::assume(guard_0);", "kani::assume(!guard_1);"]
    );
    let (p, q) = (
        find_node(&scenario.checked, &param_named("p")),
        find_node(&scenario.checked, &param_named("q")),
    );
    let inputs = &harness.identity.guard_inputs;
    assert_eq!((&inputs[0].parameter, &inputs[1].parameter), (&p, &q));
    assert_eq!(inputs[0].identifier, "guard_0");
    assert_eq!(inputs[1].identifier, "guard_1");
    assert_eq!(
        harness.identity.path_condition,
        scenario.requirements[0].path_condition
    );

    // Exchanging the path conditions exchanges the assumptions and the inputs they read.
    let (scenario, harness, assumptions) = nested(
        "ir304-exchanged",
        &[(&param_named("q"), false), (&param_named("p"), true)],
    );
    assert_eq!(
        assumptions,
        ["kani::assume(!guard_0);", "kani::assume(guard_1);"]
    );
    let inputs = &harness.identity.guard_inputs;
    assert_eq!(
        (&inputs[0].parameter, &inputs[1].parameter),
        (&q_of(&scenario), &p_of(&scenario))
    );

    // The unguarded item has no guard input or assumption.
    let unguarded = guard_scenario("ir304-plain", &source, &is_plus("1"), &[]);
    let (harness, assumptions) = guarded_harness(&unguarded);
    assert!(assumptions.is_empty());
    assert!(harness.identity.guard_inputs.is_empty());
}

fn p_of(scenario: &Scenario) -> CheckedNodeId {
    find_node(&scenario.checked, &param_named("p"))
}

fn q_of(scenario: &Scenario) -> CheckedNodeId {
    find_node(&scenario.checked, &param_named("q"))
}

#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn unlowerable_guards_settle_per_item_with_their_reason() {
    let reason = |signature: &str, body: &str, plus: &'static str, comparison: &'static str| {
        let scenario = guard_scenario(
            "ir304-refusal",
            &guard_fn(signature, body),
            &is_plus(plus),
            &[(&op_is(comparison), true)],
        );
        let (disposition, harness) = settle(&scenario);
        assert!(harness.is_none(), "no harness for a refused guard");
        assert_eq!(disposition["disposition"], "unsupported");
        assert_eq!(disposition["reason"]["code"], "path_guard_not_lowered");
        assert_eq!(
            disposition["reason"]["guard"],
            json(&scenario.requirements[0].path_condition[0].occurrence)
        );
        (scenario, disposition["reason"]["reason"].clone())
    };

    let (_, form) = reason(
        "x: Int[0, 9]",
        "if x + 1 > 5 then x + 100 else 0",
        "100",
        "quire.op.integer.gt",
    );
    assert_eq!(form, "unsupported_guard_form");

    let (scenario, unbounded) = reason(
        "n: Integer, x: Int[0, 9]",
        "if n >= 0 then x + 1 else 0",
        "1",
        "quire.op.integer.ge",
    );
    assert_eq!(
        unbounded,
        serde_json::json!({"unbounded_guard_operand": {"parameter": json(&find_node(&scenario.checked, &param_named("n")))}})
    );

    let (_, literal) = reason(
        "x: Int[0, 9]",
        "if x < 100000000000000000000000 then x + 1 else 0",
        "1",
        "quire.op.integer.lt",
    );
    let literal_not_i64 =
        serde_json::json!({"guard_literal_not_i64": {"value": "100000000000000000000000"}});
    assert_eq!(literal, literal_not_i64);

    // The `=` literal QSL widens through a conversion keeps its reason.
    let (_, widened) = reason(
        "x: Int[0, 1000000000000000000000000], z: Int[0, 9]",
        "if x = 100000000000000000000000 then z + 1 else 0",
        "1",
        "quire.op.integer.eq",
    );
    assert_eq!(widened, literal_not_i64);
}

/// A refused guarded item sits beside a supported sibling in one call: the refusal is per item.
#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn a_refused_guard_leaves_its_supported_sibling_untouched() {
    let source =
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n\
        function a using v(x: Int[0, 9]): Int[0, 11] pure { x + 2 }\n\
        function g using v(x: Int[0, 9]): Int[0, 9] pure { x }\n\
        function t using v(x: Int[0, 9]): Int[0, 12] pure { if g(x) > 5 then x + 3 else 0 }\n";
    let guard: &[GuardSpec] = &[(&op_is("quire.op.integer.gt"), true, 0)];
    let scenario = guard_items(
        "ir304-sibling",
        source,
        &[(&is_plus("2"), &[]), (&is_plus("3"), guard)],
    );
    let (supported, harness) = settle_at(&scenario, 0);
    assert_eq!(supported["disposition"], "supported");
    assert!(harness
        .expect("the unguarded sibling keeps its harness")
        .identity
        .guard_inputs
        .is_empty());
    let (refused, harness) = settle_at(&scenario, 1);
    assert!(harness.is_none());
    assert_eq!(refused["reason"]["code"], "path_guard_not_lowered");
    assert_eq!(refused["reason"]["reason"], "unsupported_guard_form");
    assert_eq!(
        refused["reason"]["guard"],
        json(&scenario.requirements[1].path_condition[0].occurrence)
    );
}

#[trace("FR-022-AC-33", "TC-033")]
#[test]
fn integer_guards_narrow_the_operand_domains() {
    let source = guard_fn("x: Int[0, 9]", "if x > 5 then x + 100 else 0");
    let narrowed = |name: &str, holds| {
        let scenario = guard_scenario(
            name,
            &source,
            &is_plus("100"),
            &[(&op_is("quire.op.integer.gt"), holds)],
        );
        let (harness, assumptions) = guarded_harness(&scenario);
        assert!(asserts_bound(&harness, 200));
        (harness, assumptions)
    };
    let (harness, assumptions) = narrowed("ir304-narrow", true);
    assert_eq!(args(&harness), vec![(6, 9), (100, 100)]);
    assert_eq!(assumptions, ["kani::assume(left_native > 5_i64);"]);
    let (harness, assumptions) = narrowed("ir304-narrow-else", false);
    assert_eq!(args(&harness), vec![(0, 5), (100, 100)]);
    assert_eq!(assumptions, ["kani::assume(!(left_native > 5_i64));"]);

    let two = guard_scenario(
        "ir304-two",
        &guard_fn(
            "x: Int[0, 9], y: Int[0, 9]",
            "if x < y then x + 1 else x + 1",
        ),
        &is_plus("1"),
        &[(&op_is("quire.op.integer.lt"), true)],
    );
    let (harness, assumptions) = guarded_harness(&two);
    assert_eq!(args(&harness), vec![(0, 9), (1, 1)]);
    assert_eq!(assumptions, ["kani::assume(left_native < guard_0);"]);
    assert_eq!(harness.identity.guard_inputs.len(), 1);
    assert_eq!(
        harness.identity.guard_inputs[0].domain,
        quire_contract_codegen::ScalarGuardDomain::Integer {
            minimum: 0,
            maximum: 9
        }
    );
    assert_eq!(
        harness.identity.guard_inputs[0].parameter,
        find_node(&two.checked, &param_named("y"))
    );
}

/// Every comparison narrows the operand by exactly the values that falsify it; `ne` trims only an
/// endpoint it names.
#[trace("FR-022-AC-33", "TC-033")]
#[test]
fn every_comparison_narrows_by_the_values_that_falsify_it() {
    // (operator, name, literal, holds, narrowed range of `x`). QSL widens an `=` or `!=` literal
    // through a conversion, which the lowering reads through to the literal.
    let cases = [
        ("<", "lt", "5", true, (0, 4)),
        ("<", "lt", "5", false, (5, 9)),
        ("<=", "le", "5", true, (0, 5)),
        ("<=", "le", "5", false, (6, 9)),
        (">", "gt", "5", true, (6, 9)),
        (">=", "ge", "5", true, (5, 9)),
        (">=", "ge", "5", false, (0, 4)),
        ("<", "lt", "0", false, (0, 9)),
        ("=", "eq", "5", true, (5, 5)),
        ("=", "eq", "5", false, (0, 9)),
        ("=", "eq", "0", false, (1, 9)),
        ("!=", "ne", "0", true, (1, 9)),
        ("!=", "ne", "9", true, (0, 8)),
        ("!=", "ne", "5", true, (0, 9)),
        ("!=", "ne", "9", false, (9, 9)),
    ];
    for (index, (symbol, name, rhs, holds, expected)) in cases.into_iter().enumerate() {
        let signature = "x: Int[0, 9]";
        let source = guard_fn(
            signature,
            &format!("if x {symbol} {rhs} then x + 100 else 0"),
        );
        let comparison: &'static str = match name {
            "lt" => "quire.op.integer.lt",
            "le" => "quire.op.integer.le",
            "gt" => "quire.op.integer.gt",
            "ge" => "quire.op.integer.ge",
            "eq" => "quire.op.integer.eq",
            _ => "quire.op.integer.ne",
        };
        let scenario = guard_scenario(
            &format!("ir304-table-{index}"),
            &source,
            &is_plus("100"),
            &[(&op_is(comparison), holds)],
        );
        let (disposition, harness) = settle(&scenario);
        let harness = harness.unwrap_or_else(|| panic!("{name} {rhs} {holds}: {disposition}"));
        assert_eq!(
            args(&harness),
            vec![expected, (100, 100)],
            "x {name} {rhs}, holds={holds}"
        );
    }
}

fn domain_of(low: i64, high: i64) -> serde_json::Value {
    serde_json::json!({"domain": "integer", "minimum": low, "maximum": high})
}

/// A dead branch names the guard that emptied the domain, the parameter and its declared domain.
#[trace("FR-022-AC-34", "TC-033")]
#[test]
fn contradictory_guards_settle_path_condition_unsatisfiable() {
    let dead = |name: &str, body: &str, guards: &[GuardSpec]| {
        let scenario = guard_items(
            &format!("ir304-{name}"),
            &guard_fn("x: Int[0, 9]", body),
            &[(&is_plus("100"), guards)],
        );
        let (disposition, harness) = settle(&scenario);
        assert!(harness.is_none());
        let reason = disposition["reason"].clone();
        assert_eq!(
            reason["code"], "path_condition_unsatisfiable",
            "{disposition}"
        );
        assert_eq!(
            reason["parameter"],
            json(&find_node(&scenario.checked, &param_named("x")))
        );
        assert_eq!(reason["declared"], domain_of(0, 9));
        (scenario, reason)
    };
    for (name, body, comparison) in [
        (
            "dead-gt",
            "if x > 20 then x + 100 else 0",
            "quire.op.integer.gt",
        ),
        (
            "dead-lt",
            "if 20 < x then x + 100 else 0",
            "quire.op.integer.lt",
        ),
    ] {
        let (scenario, reason) = dead(name, body, &[(&op_is(comparison), true, 0)]);
        assert_eq!(
            reason["guard"],
            json(&scenario.requirements[0].path_condition[0].occurrence)
        );
    }
    // The second guard empties a domain the first only narrowed: it is the one reported.
    let (scenario, reason) = dead(
        "dead-second",
        "if x >= 5 then (if x < 3 then x + 100 else 0) else 0",
        &[
            (&op_is("quire.op.integer.ge"), true, 0),
            (&op_is("quire.op.integer.lt"), true, 0),
        ],
    );
    assert_eq!(
        reason["guard"],
        json(&scenario.requirements[0].path_condition[1].occurrence)
    );
    assert_ne!(
        scenario.requirements[0].path_condition[0],
        scenario.requirements[0].path_condition[1]
    );

    // the same node without a guard is `result_bound_unreachable`
    let KaniObligationOutcome::Emitted { records, .. } =
        direct_outcome(package::QSL_PLUS_HUNDRED, false)
    else {
        panic!("the request is rejected");
    };
    let disposition = serde_json::to_value(&records[0].disposition).expect("serializes");
    assert_eq!(disposition["reason"]["code"], "result_bound_unreachable");
}

/// `if p then (if p then 0 else x + 1) else 0`: the inner occurrence of `p` contradicts the outer.
#[trace("FR-022-AC-34", "TC-033")]
#[test]
fn a_contradicted_boolean_guard_names_the_inner_occurrence_and_its_domain() {
    let scenario = guard_items(
        "ir304-dead-boolean",
        &guard_fn(
            "p: Boolean, x: Int[0, 9]",
            "if p then (if p then 0 else x + 1) else 0",
        ),
        &[(
            &is_plus("1"),
            &[(&param_named("p"), true, 0), (&param_named("p"), false, 1)],
        )],
    );
    let path = &scenario.requirements[0].path_condition;
    assert_ne!(path[0].occurrence, path[1].occurrence);
    let (disposition, harness) = settle(&scenario);
    assert!(harness.is_none());
    let reason = &disposition["reason"];
    assert_eq!(
        reason["code"], "path_condition_unsatisfiable",
        "{disposition}"
    );
    assert_eq!(reason["guard"], json(&path[1].occurrence));
    assert_eq!(reason["parameter"], json(&p_of(&scenario)));
    assert_eq!(reason["declared"], serde_json::json!({"domain": "boolean"}));
}

#[trace("FR-022-AC-31", "TC-033")]
#[test]
fn routed_identity_carries_the_occurrence_and_the_direct_one_does_not() {
    let scenario = rr16();
    let generation = scenario
        .generate(&scenario.valid(), scenario.records())
        .expect("both occurrences route");
    let mut occurrences = Vec::new();
    for index in 0..2 {
        let harness = kani_record(&generation, index).1.expect("a harness");
        assert_eq!(
            harness.identity.requirement_occurrence.as_ref(),
            Some(&scenario.requirements[index].occurrence)
        );
        assert!(harness.identity.path_condition.is_empty());
        assert!(harness.identity.guard_inputs.is_empty());
        occurrences.push(harness.identity.requirement_occurrence.clone());
    }
    assert_ne!(occurrences[0], occurrences[1]);
    let record = direct_increment_record();
    assert_eq!(
        record,
        include_str!("../fixtures/kani-scalar/direct-increment-record.json")
    );
    assert!(!record.contains("requirementOccurrence"));
}

#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn a_guard_whose_context_root_is_dropped_refuses_the_whole_call() {
    let mut scenario = guard_scenario(
        "ir304-context",
        &guard_fn("p: Boolean, x: Int[0, 9]", "if p then x + 1 else x + 1"),
        &is_plus("1"),
        &[(&param_named("p"), true)],
    );
    // A record and requirement that agree on a guard whose node the package has no context root
    // for: every earlier record check passes.
    let unrooted = scenario
        .checked
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| param_named("x")(node))
        .expect("the parameter x")
        .node_id
        .clone();
    let guard = RequirementGuard {
        occurrence: conv_occurrence(&scenario.checked, &unrooted, 0),
        holds: true,
    };
    scenario.requirements[0].path_condition.push(guard.clone());
    let error = scenario.refusal(&scenario.valid(), scenario.records());
    assert_eq!(
        unboxed(&error),
        Some((
            0,
            RequirementRecordMismatch::GuardContextMissing {
                guard: guard.occurrence
            }
        ))
    );
}

/// A guard whose context root the package admits but accounts as not `lowered` (here its closure
/// reaches a function the profile does not support) refuses that item with the guard named, and
/// leaves its unguarded sibling alone.
#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn a_guard_root_the_package_does_not_lower_is_not_lowered() {
    let source =
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n\
        function a using v(x: Int[0, 9]): Int[0, 11] pure { x + 2 }\n\
        function g using v(x: Int[0, 9]): Int[0, 9] pure { x }\n\
        function t using v(x: Int[0, 9]): Int[0, 12] pure { if g(x) > 5 then x + 3 else 0 }\n";
    // The guard's closure reaches a function the profile does not support, so its root is
    // accounted for but not `lowered`.
    let mut profile = routed_contract_lowering_profile();
    profile
        .supported_tags
        .remove(&quire_contract_model::CheckedNodeTag::Function);
    {
        let checked = qsl_package("ir304-unlowered-root", source);
        let guard: &[GuardSpec] = &[(&op_is("quire.op.integer.gt"), true, 0)];
        let items: [(NodePredicate, &[GuardSpec]); 2] =
            [(&is_plus("2"), &[]), (&is_plus("3"), guard)];
        let specs = items
            .iter()
            .map(|(application, guards)| item_spec(&checked, application, guards))
            .collect();
        let scenario = Scenario::with_profile(checked, specs, &profile);
        let (supported, _) = settle_at(&scenario, 0);
        assert_eq!(supported["disposition"], "supported");
        let (refused, harness) = settle_at(&scenario, 1);
        assert!(harness.is_none());
        assert_eq!(refused["reason"]["code"], "path_guard_not_lowered");
        assert_eq!(refused["reason"]["reason"], "guard_root_not_lowered");
        assert_eq!(
            refused["reason"]["guard"],
            json(&scenario.requirements[1].path_condition[0].occurrence)
        );
    }
}

/// A direct caller's guard names a node that is in the admitted package (inside the application's
/// closure) but was not requested as a root, so its accounting is not `lowered`.
#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn a_guard_inside_the_closure_but_not_a_requested_root_is_not_lowered() {
    let scenario = guard_scenario(
        "ir304-direct",
        &guard_fn("x: Int[0, 9]", "x + 1"),
        &is_plus("1"),
        &[],
    );
    let checked = &scenario.checked;
    let node_id = find_node(checked, &is_plus("1"));
    let item = derive_exact_scalar_items(checked, std::slice::from_ref(&node_id))
        .into_iter()
        .next()
        .expect("one requested node is derived")
        .expect("the increment has a source-derived descriptor");
    let claim_map = generate_exact_scalar_oracles(checked, &[item])
        .expect("the increment renders")
        .claim_map;
    let selection = scenario
        .positions
        .iter()
        .find_map(|position| match position {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => Some(result_selection),
            _ => None,
        })
        .expect("a driver position");
    let x = find_node(checked, &param_named("x"));
    let path_condition = [RequirementGuard {
        occurrence: conv_occurrence(checked, &x, 0),
        holds: true,
    }];
    let items = [ObligationItem::ScalarClaim {
        path_condition: &path_condition,
        requirement_occurrence: None,
        source: checked,
        package: &scenario.package,
        claim_map: &claim_map,
        node_id: &node_id,
        claim_index: 0,
        result_selection: selection,
    }];
    let KaniObligationOutcome::Emitted { records, .. } =
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &items,
            subject_path: "crate::subject",
            unwind: 1,
        })
        .expect("FR-015 accepts the request")
    else {
        panic!("the request is rejected");
    };
    let disposition = json(&records[0].disposition);
    assert_eq!(disposition["reason"]["code"], "path_guard_not_lowered");
    assert_eq!(disposition["reason"]["reason"], "guard_root_not_lowered");
    assert_eq!(
        disposition["reason"]["guard"],
        json(&path_condition[0].occurrence)
    );
}

#[trace("FR-022-AC-32", "TC-033")]
#[test]
fn a_guard_on_a_node_that_is_no_lowered_root_is_not_lowered() {
    let KaniObligationOutcome::Emitted { records, .. } =
        direct_outcome_guarded(package::QSL_INC, Some(package::QSL_LITERAL_HUNDRED))
    else {
        panic!("the request is rejected");
    };
    let disposition = serde_json::to_value(&records[0].disposition).expect("serializes");
    assert_eq!(disposition["reason"]["code"], "path_guard_not_lowered");
    assert_eq!(disposition["reason"]["reason"], "guard_root_not_lowered");
}

#[trace("FR-022-AC-37", "TC-033")]
#[test]
fn an_unlowered_second_guard_precedes_unsatisfiability() {
    let scenario = guard_scenario(
        "ir304-j",
        &guard_fn(
            "x: Int[0, 9]",
            "if x > 20 then (if x + 1 > 5 then x + 100 else 0) else 0",
        ),
        &is_plus("100"),
        &[
            (
                &|node| {
                    operation(node) == Some("quire.op.integer.gt")
                        && !format!("{:?}", node.body).contains("add")
                },
                true,
            ),
            (
                &|node| {
                    operation(node) == Some("quire.op.integer.gt")
                        && format!("{:?}", node.body).contains("add")
                },
                true,
            ),
        ],
    );
    let (disposition, harness) = settle(&scenario);
    assert!(harness.is_none());
    assert_eq!(
        disposition["reason"]["code"], "path_guard_not_lowered",
        "{disposition}"
    );
}

/// The native-route harness of the FR-022-AC-35 scenario `if x OP y then x + 1 else 0` with
/// `x: Int[5, 9]` and `y: Int[0, 4]` into `Int[0, 10]`, guarded by `(x OP y, holds=true)`.
fn ac35_native_harness(
    comparison: &str,
    identity: &'static str,
) -> quire_contract_codegen::KaniScalarObligationHarness {
    let scenario = guard_scenario(
        "ac35",
        &guard_fn(
            "x: Int[5, 9], y: Int[0, 4]",
            &format!("if x {comparison} y then x + 1 else 0"),
        ),
        &is_plus("1"),
        &[(&op_is(identity), true)],
    );
    let records = RequirementRecords::new(scenario.records());
    let generation = quire_contract_codegen::generate_routed_native(
        &scenario.checked,
        &scenario.package,
        &scenario.positions,
        &scenario.valid(),
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: crate::common::proof_ceilings::proof_ceilings(),
                subject_path: "crate::subject",
                unwind: 3,
                requirement_records: &records,
            }),
        },
    )
    .expect("the guarded item routes");
    let (record, harness) = kani_record(&generation, 0);
    harness
        .unwrap_or_else(|| panic!("a harness, got {:?}", record.disposition))
        .clone()
}

/// The routed scalar harness computes on native integers, never drops the meter or the outcome
/// and ends with the one cover read from a plain `bool` (FR-022-AC-35, FR-015-AC-7).
#[trace("FR-022-AC-35", "TC-033")]
#[test]
fn the_guarded_native_harness_is_thin_and_ends_with_its_cover() {
    for (comparison, identity) in [("<", "quire.op.integer.lt"), (">", "quire.op.integer.gt")] {
        let harness = ac35_native_harness(comparison, identity);
        let source = &harness.rust.contents;
        let proof = &source[source.find("#[kani::proof]").expect("the proof attribute")..];
        assert_eq!(
            harness.identity.integer_representation,
            quire_contract_codegen::ScalarIntegerRepresentation::Native
        );
        assert!(
            !proof.contains("rt::Integer"),
            "no arbitrary-precision value: {proof}"
        );
        assert!(
            proof.contains("nx::NativeInteger::new(i128::from(left))"),
            "{proof}"
        );
        assert!(proof.contains("value.value() == exact"), "{proof}");
        let assumption = format!("kani::assume(left_native {comparison} guard_0);");
        assert_eq!(proof.matches(&assumption).count(), 1, "{proof}");
        assert!(
            proof.find(&assumption) < proof.find("let outcome = "),
            "{proof}"
        );
        let tail = proof
            .lines()
            .rev()
            .take(6)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        assert!(
            tail[0].starts_with("let cover_reached: bool = completed;"),
            "{tail:?}"
        );
        assert_eq!(tail[1], "core::mem::forget(meter);");
        assert_eq!(tail[2], "core::mem::forget(outcome);");
        assert!(
            tail[3].starts_with("kani::cover!(cover_reached, "),
            "{tail:?}"
        );
        assert_eq!(proof.matches("kani::cover!(").count(), 1);
    }
}
