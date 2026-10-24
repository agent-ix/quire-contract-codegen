//! Shared admitted scalar routing fixture for the integration and private library controls.
use super::{
    generate_routed, routed_contract_lowering_profile, Artifact, BackendDescriptor, Candidate,
    ContractDriverResultSelection, ContractRequestPosition, GenerationContexts,
    KaniGenerationContext, KaniScalarObligationHarness, KindOutput, ProviderOrigin,
    RoutedGenerationItem,
};
use super::{package, proof_ceilings_with_wall_clock};
use package::Bound;
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
    builder.narrow(
        narrow_code,
        package::code_id(operation_code).digest.as_ref(),
        &bound,
    );
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
    let positions = [
        ContractRequestPosition::DriverItem {
            node_id: node_id.clone(),
            result_selection: ContractDriverResultSelection::OwnResult {
                application: occurrence(&node_id),
                conversion: Some(occurrence(&narrow_id)),
            },
        },
        ContractRequestPosition::ContextRoot { node_id: narrow_id },
    ];
    let generation = generate_routed(
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
        )],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: proof_ceilings_with_wall_clock(wall_clock),
                subject_path: "crate::subject",
                unwind: 3,
            }),
        },
    )
    .expect("routed generation succeeds");
    if route == ScalarRoute::Native {
        let claim_map = generation.claim_map.as_ref().expect("checked claim map");
        let claim_index = claim_map
            .items
            .iter()
            .position(|claim| claim.node_id == node_id)
            .expect("selected checked claim");
        let ContractRequestPosition::DriverItem {
            result_selection, ..
        } = &positions[0]
        else {
            panic!("selected driver result");
        };
        let item = quire_contract_codegen::ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map,
            node_id: &node_id,
            claim_index,
            result_selection,
        };
        let quire_contract_codegen::KaniObligationOutcome::Emitted {
            mut scalar_harnesses,
            records,
            ..
        } = quire_contract_codegen::negotiate_kani_native_obligations(
            &quire_contract_codegen::KaniObligationRequest {
                ceilings: proof_ceilings_with_wall_clock(wall_clock),
                items: &[item],
                subject_path: "crate::subject",
                unwind: 3,
            },
        )
        .expect("native negotiation")
        else {
            panic!("original scalar claim is admitted");
        };
        assert_eq!(scalar_harnesses.len(), 1, "{records:?}");
        let manifest = generation
            .oracle_artifacts
            .expect("dependency manifest")
            .into_iter()
            .find(|artifact| artifact.path == "Cargo.toml")
            .expect("dependency manifest");
        return (scalar_harnesses.remove(0), vec![manifest]);
    }
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
