// SPDX-License-Identifier: AGPL-3.0-or-later
//! IR-631 real-Kani half of FR-032-AC-7 and FR-032-AC-8 (TC-047): a falsified Kani run of the
//! routed native `x + 1` arithmetic mutant is planned from its real playback, observed natively on
//! the same staged bytes, replayed by the genuine QSL facade and settled as `Diverged` and `Failed`.
//!
//! The mutant is a crate-private construction: the generator never emits a falsifiable harness, so
//! this test re-seals one with the original replay body and package bound and the same mutation in
//! both the harness and the emitted oracle artifact. Production callers cannot do this.
#[allow(dead_code)]
#[path = "../common/requirements.rs"]
mod requirements;
use super::scalar_native_private::run_mutant_evidence;
use crate::kani::{
    identity::KaniScalarObligationHarness,
    run::scalar::{observe_staged_native_for_test, KaniScalarNativeRequest},
    run::tool::KaniInstallation,
    test_support::proof_ceilings::proof_ceilings_with_wall_clock,
};
use ix_trace_rs::trace;
use qsl_replay::{
    compile_package, DependencyInput, NativeOutcome, OperatorParityResult, ReplayLimits,
    SourceIdentity, StageLimits, TerminalValue,
};
use quire_contract_codegen::{
    generate_routed_native, routed_contract_lowering_profile, scalar_run_terminal_value, Artifact,
    BackendDescriptor, Candidate, ContractDriverResultSelection, ContractRequestPosition,
    GenerationContexts, KaniGenerationContext, KindOutput, LockedSource, OriginalScalarContext,
    ProviderOrigin, ReplayInputs, RoutedGenerationItem, ScalarDriverObservation, ScalarSentRequest,
    ScalarSettlement, WitnessDecodeLimits, SCALAR_PLAYBACK_BYTES,
};
use quire_contract_model::{
    CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies, ContractOccurrenceKeyV1,
    ContractPackageReadResult, ContractPackageWireV1,
};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc, time::Duration};

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction subject using v(x: Int[0, 9]): Int[0, 10] pure { x + 1 }\n";

fn application(body: &CheckedTerm, operation: &str) -> bool {
    matches!(body, CheckedTerm::Application { operation: found, .. } if found.identity() == Some(operation))
}

struct Fixture {
    harness: KaniScalarObligationHarness,
    checked: CheckedPackageV2,
    raw_package: qsl_replay::ReplayPackage,
    artifacts: Vec<Artifact>,
}

/// The QSL-emitted `x + 1` over `Int[0, 9]` through the native routed generation.
fn native_fixture() -> Fixture {
    let compiled = compile_package(
        qsl_replay::CompileUnit {
            source: SourceIdentity::new("test", "scalar-playback.native"),
            package_name: "scalar-playback".to_owned(),
            path: "scalar-playback.native",
            bytes: SOURCE.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL compiles the authored scalar source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let CheckedPackageV2ReadResult::Admitted(checked) = CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("IR admits the emitted package")
    };
    let nodes = &checked.resolved_graph().nodes;
    let node_id = nodes
        .iter()
        .find(|node| application(&node.body, "quire.op.integer.add"))
        .expect("one authored addition")
        .node_id
        .clone();
    let conversion = nodes
        .iter()
        .find(|node| {
            application(&node.body, "quire.op.numeric.narrow")
                && matches!(&node.body, CheckedTerm::Application { arguments, .. }
                    if matches!(arguments.first(), Some(CheckedTerm::Reference { target }) if target == &node_id))
        })
        .expect("authored result narrowing")
        .node_id
        .clone();
    let occurrence = |id: &quire_contract_model::CheckedNodeId| {
        let entry = checked
            .source_map()
            .iter()
            .find(|entry| &entry.node_id == id && entry.role == CheckedOccurrenceRole::Expression)
            .expect("one authored expression occurrence");
        ContractOccurrenceKeyV1 {
            node_id: id.clone(),
            role: entry.role.clone(),
            ordinal: entry.ordinal,
        }
    };
    let selection = ContractDriverResultSelection::OwnResult {
        application: occurrence(&node_id),
        conversion: Some(occurrence(&conversion)),
    };
    let positions = vec![
        ContractRequestPosition::DriverItem {
            node_id: node_id.clone(),
            result_selection: selection.clone(),
        },
        ContractRequestPosition::ContextRoot {
            node_id: conversion.clone(),
        },
    ];
    let lowered = checked.lower(
        &[node_id.clone(), conversion.clone()],
        &routed_contract_lowering_profile(),
    );
    let ContractPackageReadResult::Admitted(admitted) = ContractPackageWireV1::read(
        lowered.package.canonical_bytes().unwrap(),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) else {
        panic!("IR admits the routed package")
    };
    let requirement = requirements::requirement_for(&checked, &selection).expect("own result");
    let records = requirements::records(&BTreeMap::from([(0, requirement.clone())]));
    let route = RoutedGenerationItem::from_descriptor(
        0,
        node_id,
        Candidate {
            identity: "kani".to_owned(),
        },
        &BackendDescriptor {
            identity: "kani".to_owned(),
            advertised: Vec::new(),
            origin: ProviderOrigin::Linked,
            domains: None,
            bounds: None,
        },
    )
    .expect("linked Kani route")
    .with_scalar_selection(&admitted, selection)
    .with_requirement(requirement);
    let generated = generate_routed_native(
        &checked,
        &admitted,
        &positions,
        &[route],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: proof_ceilings_with_wall_clock(Duration::from_secs(600)),
                subject_path: "crate::subject",
                unwind: 3,
                requirement_records: &records,
            }),
        },
    )
    .expect("native routed scalar generation");
    let [item] = generated.items.as_slice() else {
        panic!("one routed item")
    };
    let KindOutput::Kani {
        harness: Some(harness),
        ..
    } = &item.output
    else {
        panic!("routed scalar harness: {:?}", item.output)
    };
    Fixture {
        harness: (**harness).clone(),
        checked: (*checked).clone(),
        raw_package: qsl_replay::ReplayPackage::read(compiled.bytes(), u64::MAX).unwrap(),
        artifacts: generated.oracle_artifacts.expect("retained oracle package"),
    }
}

fn locked_source() -> LockedSource {
    LockedSource {
        authority: "test".to_owned(),
        identity: "scalar-playback.native".to_owned(),
        bytes: SOURCE.as_bytes().to_vec(),
    }
}

#[trace(
    "TC-047",
    "FR-032-AC-3",
    "FR-032-AC-4",
    "FR-032-AC-5",
    "FR-032-AC-7",
    "FR-032-AC-8"
)]
#[test]
#[ignore = "real Kani qualification: run serially under flock and the 24 GiB scope"]
fn a_real_falsified_native_mutant_plans_observes_and_settles_diverged_failed() {
    let healthy = native_fixture();
    let original = "nx::NativeIntegerOperation::Add {";
    let mutated = "nx::NativeIntegerOperation::Subtract {";
    assert_eq!(healthy.harness.rust.contents.matches(original).count(), 1);
    let mut artifacts = healthy.artifacts.clone();
    let library = artifacts
        .iter_mut()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("emitted oracle library");
    assert_eq!(library.contents.matches(original).count(), 1);
    library.contents = library.contents.replace(original, mutated);
    let mut rust = healthy.harness.rust.clone();
    rust.contents = rust.contents.replace(original, mutated);

    // Re-seal: valid provenance, the original replay body and original package bound, and the
    // mutation in both the harness and the emitted oracle artifact.
    let mut mutant = KaniScalarObligationHarness::generated_with_scalar_limits(
        healthy.harness.identity.clone(),
        rust,
        healthy.harness.record.clone(),
        healthy.harness.original_scalar_limits().unwrap(),
    );
    let body = healthy.harness.scalar_replay_body().unwrap().clone();
    mutant.bind_scalar_replay_body(
        Ok(body.clone()),
        Arc::clone(healthy.harness.scalar_original_package().unwrap()),
    );
    mutant.bind_artifact_package(&artifacts);

    // Real Kani falsifies the sealed mutant.
    let manifest = artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .unwrap();
    let evidence = run_mutant_evidence(&mutant, manifest);
    let crate::kani::classify::KaniRunOutcome::Falsified { counterexample } = &evidence.outcome
    else {
        panic!("falsified: {evidence:?}")
    };

    // `plan` decodes the real playback against the original identity.
    let source = locked_source();
    let context = OriginalScalarContext::new(
        healthy.checked.clone(),
        healthy.raw_package.clone(),
        &source,
        ReplayInputs {
            decode_limits: WitnessDecodeLimits {
                bytes: SCALAR_PLAYBACK_BYTES,
                values: 2,
            },
            package_name: "scalar-playback".to_owned(),
            source: source.clone(),
            dependencies: vec![],
            accounting_limits: mutant.original_scalar_limits().unwrap(),
            stage_limits: BTreeMap::new(),
            supplied_limits: None,
            declared_domains: vec![],
            replay_limits: ReplayLimits::default(),
        },
        "subject",
        CheckedPackageReadLimits::bounded(),
    )
    .expect("original source context");
    let plan = context
        .plan(&mutant, &mutant.identity, &body, &artifacts, counterexample)
        .expect("the real playback plans against the re-sealed mutant");
    let operands = plan.operands().to_vec();
    assert_eq!(operands.len(), 2);
    assert!((0..=9).contains(&operands[0]));
    assert_eq!(operands[1], 1);

    // The driver's native observation of the same staged bytes, at the decoded vector.
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let cancel = quire_exact::Cancel::new();
    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/ir631-native-observe");
    let (native, executed) = observe_staged_native_for_test(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &mutant,
        oracle_artifacts: &artifacts,
        operands: &operands,
        scalar_limits: plan.scalar_limits(),
        target_directory: &target,
    })
    .expect("the staged mutant runs natively");
    let outcome = match native {
        quire_exact::Outcome::Completed(value) => NativeOutcome::Completed(value),
        quire_exact::Outcome::Refused(quire_exact::Refusal::IntegerOutOfDomain { .. }) => {
            NativeOutcome::RefusedOutOfRange
        }
        quire_exact::Outcome::Undefined(_) => NativeOutcome::Undefined,
        quire_exact::Outcome::Incomplete(_) => NativeOutcome::Incomplete,
        other => panic!("unexpected native outcome {other:?}"),
    };
    // x - 1 differs from the exact x + 1 at every admitted vector.
    assert_ne!(
        outcome,
        NativeOutcome::Completed(quire_exact::Integer::from(operands[0] + 1))
    );
    let completed = plan
        .complete(ScalarDriverObservation {
            staged_entries: executed,
            identity: mutant.identity.clone(),
            harness: mutant.rust.clone(),
            record: mutant.record.clone(),
            operands,
            limits: mutant.original_scalar_limits().unwrap(),
            source,
            package: healthy.checked.clone(),
            outcome,
        })
        .expect("the observation is of the planned staged bytes");

    // The genuine QSL facade and the settlement converter.
    let (wire, claim, limits) = completed.request();
    let sent = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    let report = qsl_replay::replay_operator_parity(wire, claim, limits);
    assert!(
        matches!(report.result(), OperatorParityResult::Diverged { .. }),
        "the mutant diverges from exact addition: {:?}",
        report.result()
    );
    let settlement = completed.settlement(&sent, Some(&report)).unwrap();
    assert!(matches!(
        &settlement,
        ScalarSettlement::Parity(result) if matches!(result, OperatorParityResult::Diverged { .. })
    ));
    assert_eq!(
        scalar_run_terminal_value(&evidence, Some(settlement)).unwrap(),
        TerminalValue::Failed
    );
}
