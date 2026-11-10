//! Shared admitted scalar routing fixture for the integration and private library controls.
use super::{
    generate_routed, routed_contract_lowering_profile, Artifact, BackendDescriptor, Candidate,
    ContractDriverResultSelection, ContractRequestPosition, GenerationContexts,
    KaniGenerationContext, KaniScalarObligationHarness, KindOutput, ProviderOrigin,
    RequirementOccurrence, RequirementRecordInput, RequirementRecords, RoutedGenerationItem,
    RoutedRequirement,
};
use super::{package, proof_ceilings_with_wall_clock};
use package::Bound;
use quire_contract_codegen::generate_routed_native;
use quire_contract_model::{
    CheckedPackageReadLimits, ContractDependencies, ContractPackageReadResult,
    ContractPackageWireV1,
};
use std::time::Duration;
pub(super) fn routed_scalar_with_artifacts(
    builder: package::PackageBuilder,
    operation_code: u32,
    narrow_code: u32,
    result_bound: Bound,
    wall_clock: Duration,
) -> (KaniScalarObligationHarness, Vec<Artifact>) {
    scalar_with_policy(
        builder,
        operation_code,
        narrow_code,
        result_bound,
        wall_clock,
        ScalarRoute::General,
    )
}

// This fixture module is also included by library tests that only call the general route.
#[allow(dead_code)]
pub(super) fn routed_native_scalar_with_artifacts(
    builder: package::PackageBuilder,
    operation_code: u32,
    narrow_code: u32,
    result_bound: Bound,
    wall_clock: Duration,
) -> (KaniScalarObligationHarness, Vec<Artifact>) {
    scalar_with_policy(
        builder,
        operation_code,
        narrow_code,
        result_bound,
        wall_clock,
        ScalarRoute::Native,
    )
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ScalarRoute {
    General,
    Native,
}

fn scalar_with_policy(
    mut builder: package::PackageBuilder,
    operation_code: u32,
    narrow_code: u32,
    result_bound: Bound,
    wall_clock: Duration,
    route: ScalarRoute,
) -> (KaniScalarObligationHarness, Vec<Artifact>) {
    let bound = builder.bound(&result_bound);
    builder.narrow(narrow_code, &package::key(operation_code), &bound);
    builder.expression_occurrences();
    let (package, ids) = builder.admit_resolved();
    let node_id = ids.resolve(&package::code_id(operation_code));
    let narrow_id = ids.resolve(&package::code_id(narrow_code));
    let roots = [node_id.clone(), narrow_id.clone()];
    let lowered = package.lower(&roots, &routed_contract_lowering_profile());
    let admitted = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("IR must admit the routed scalar package: {other:?}"),
    };
    let occurrence = |node: &quire_contract_model::CheckedNodeId| {
        let entries = package
            .source_map()
            .iter()
            .filter(|entry| {
                &entry.node_id == node
                    && entry.role == quire_contract_model::CheckedOccurrenceRole::Expression
            })
            .collect::<Vec<_>>();
        let [entry] = entries.as_slice() else {
            panic!("one caller-authored expression occurrence");
        };
        quire_contract_model::ContractOccurrenceKeyV1 {
            node_id: node.clone(),
            role: entry.role.clone(),
            ordinal: entry.ordinal,
        }
    };
    let to_requirement =
        |key: &quire_contract_model::ContractOccurrenceKeyV1| RequirementOccurrence {
            node_id: key.node_id.clone(),
            role: key.role.clone(),
            ordinal: key.ordinal,
        };
    let application = occurrence(&node_id);
    let conversion = occurrence(&narrow_id);
    let requirement = RoutedRequirement {
        occurrence: to_requirement(&application),
        result_bound: package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| node.node_id == narrow_id)
            .expect("the authored narrow")
            .semantic_type
            .clone(),
        conversion_context: Some(to_requirement(&conversion)),
        path_condition: Vec::new(),
    };
    let records = RequirementRecords::new(std::collections::BTreeMap::from([(
        requirement.occurrence.clone(),
        RequirementRecordInput {
            result_bound: requirement.result_bound.clone(),
            conversion_context: requirement.conversion_context.clone(),
            path_condition: Vec::new(),
        },
    )]));
    let positions = [
        ContractRequestPosition::DriverItem {
            node_id: node_id.clone(),
            result_selection: ContractDriverResultSelection::OwnResult {
                application,
                conversion: Some(conversion),
            },
        },
        ContractRequestPosition::ContextRoot { node_id: narrow_id },
    ];
    let generate = if route == ScalarRoute::Native {
        generate_routed_native
    } else {
        generate_routed
    };
    let generation = generate(
        &package,
        &admitted,
        &positions,
        &[RoutedGenerationItem::from_descriptor(
            0,
            node_id.clone(),
            Candidate {
                identity: "kani".to_owned(),
            },
            &BackendDescriptor {
                identity: "kani".to_owned(),
                advertised: Vec::new(),
                origin: ProviderOrigin::Linked,
                domains: None,
                bounds: Default::default(),
            },
        )
        .expect("linked Kani descriptor")
        .with_scalar_selection(
            &admitted,
            match &positions[0] {
                ContractRequestPosition::DriverItem {
                    result_selection, ..
                } => result_selection.clone(),
                ContractRequestPosition::Excluded { .. }
                | ContractRequestPosition::ContextRoot { .. } => unreachable!("driver position"),
            },
        )
        .with_requirement(requirement)],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: proof_ceilings_with_wall_clock(wall_clock),
                subject_path: "crate::subject",
                unwind: 3,
                requirement_records: &records,
            }),
        },
    )
    .expect("routed generation succeeds");
    let artifacts = generation.oracle_artifacts.expect("a Kani group ran");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let KindOutput::Kani { harness, .. } = &item.output else {
        panic!("expected Kani output");
    };
    (
        harness.as_deref().cloned().expect("the item is supported"),
        artifacts,
    )
}
