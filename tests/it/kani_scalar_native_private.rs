//! Private orchestration controls for scalar-native cleanup after real execution.
#[path = "../exact_scalar_support/package.rs"]
mod package;
use crate::{
    core::artifact::Artifact,
    core::contract_package::{
        routed_contract_lowering_profile, ContractDriverResultSelection, ContractRequestPosition,
    },
    kani::{
        identity::KaniScalarObligationHarness,
        run::{
            scalar::{execute_with_cleanup, KaniScalarNativeRequest},
            tool::KaniInstallation,
        },
    },
    routed::{
        capability::{BackendDescriptor, Candidate, ProviderOrigin},
        generate::{
            generate_routed, GenerationContexts, KaniGenerationContext, KindOutput,
            RoutedGenerationItem,
        },
    },
};
use ix_trace_rs::trace;
use package::Bound;
use quire_contract_model::{
    CheckedPackageReadLimits, ContractDependencies, ContractPackageReadResult,
    ContractPackageWireV1,
};
use std::{fs, path::PathBuf, time::Duration};
fn routed_scalar_with_artifacts(
    mut builder: package::PackageBuilder,
    operation_code: u32,
    narrow_code: u32,
    result_bound: Bound,
    wall_clock: Duration,
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
            node_id,
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
                ContractRequestPosition::ContextRoot { .. } => unreachable!("driver position"),
            },
        )],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(
                    wall_clock,
                ),
                subject_path: "crate::subject",
                unwind: 3,
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

fn fixture(wall_clock: Duration) -> (KaniScalarObligationHarness, Vec<Artifact>) {
    routed_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_190,
        Bound::Integer(0, 9),
        wall_clock,
    )
}

fn limits() -> quire_contract_runtime::exact::ScalarLimits {
    quire_contract_runtime::exact::ScalarLimits {
        integer_bits: 101,
        decimal_digits: 102,
        scale_expansion: 103,
        text_input_bytes: 104,
        text_scalars: 105,
        normalized_scalars: 106,
        unit_edges: 107,
        value_occurrences: 108,
        work_units: 109,
        result_units: 110,
    }
}

#[trace("FR-017-AC-45", "FR-017-AC-48", "FR-017-AC-50")]
#[test]
#[ignore = "real Kani and native cleanup controls: run this private lib control explicitly and serially"]
fn cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal() {
    let (harness, artifacts) = fixture(Duration::from_secs(600));
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("installed Kani");
    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/private-scalar-cleanup");
    let request = KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &[4, 1],
        scalar_limits: limits(),
        target_directory: &target,
    };
    let mut success_path = None;
    let success = execute_with_cleanup(&request, |path| {
        success_path = Some(path.to_owned());
        assert!(path.join("Cargo.lock").exists());
        fs::remove_dir_all(path)
    })
    .expect("real successful receipt");
    assert!(
        !success_path.unwrap().exists(),
        "removal must finish before the receipt returns"
    );
    assert!(success.cleanup_diagnostic().is_none());
    let mut failure_path = None;
    let failure = execute_with_cleanup(&request, |path| {
        failure_path = Some(path.to_owned());
        assert!(path.join("Cargo.lock").exists());
        Err(std::io::Error::other("injected snapshot removal failure"))
    })
    .expect("cleanup failure must preserve a real receipt");
    assert!(failure
        .cleanup_diagnostic()
        .unwrap()
        .contains("injected snapshot removal failure"));
    assert!(
        !failure_path.unwrap().exists(),
        "Drop backstop retries a transient removal failure"
    );
    assert_eq!(
        success.proof_evidence().outcome,
        crate::kani::classify::KaniRunOutcome::Verified
    );
    assert_eq!(
        success.proof_evidence().outcome,
        failure.proof_evidence().outcome
    );
    assert_eq!(success.native_outcome(), failure.native_outcome());
    assert_eq!(
        success.proof_content_digest(),
        failure.proof_content_digest()
    );
    assert_eq!(success.staged_entries(), failure.staged_entries());
    assert_eq!(success.meter_observation().limits(), &limits());
    assert_eq!(
        success.meter_observation().limits(),
        failure.meter_observation().limits()
    );
    // The actual returned entries, independently encoded after both snapshots disappeared.
    use sha2::{Digest, Sha256};
    let artifacts = failure.staged_entries().iter().map(|(path, bytes)| {
        serde_json::json!({"path": path, "contents": std::str::from_utf8(bytes).unwrap()})
    }).collect::<Vec<_>>();
    let jcs = serde_json::to_vec(&serde_json::json!({"schema": "agent-ix/cg-kani-scalar-proof-content/v2", "artifacts": artifacts})).unwrap();
    let digest: [u8; 32] = Sha256::digest(jcs).into();
    assert_eq!(failure.proof_content_digest().as_bytes(), &digest);
}

#[trace("FR-017-AC-42")]
#[test]
fn bounded_single_run_standins_preserve_cancelled_and_timed_out_evidence() {
    use crate::kani::{
        classify::{KaniInconclusiveReason, KaniRunOutcome},
        run::scalar::{KaniScalarNativeError, KaniScalarNativeErrorClass},
    };
    use std::os::unix::fs::PermissionsExt;
    for cancelled in [true, false] {
        let (harness, artifacts) = fixture(Duration::from_secs(3));
        let control = crate::kani::test_support::discover_scratch(if cancelled {
            "scalar-cancel"
        } else {
            "scalar-timeout"
        });
        fs::create_dir_all(&control).unwrap();
        let started = control.join("started");
        let launcher = control.join("standin");
        fs::write(
            &launcher,
            format!(
                "#!/bin/sh\nprintf started > '{}'\nexec /bin/sleep 30\n",
                started.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o700)).unwrap();
        let installation = KaniInstallation::new(launcher);
        let cancel = quire_exact::Cancel::new();
        let target = control.join("target");
        let error = std::thread::scope(|scope| {
            let watcher = cancelled.then(|| {
                scope.spawn(|| {
                    let deadline = std::time::Instant::now() + Duration::from_secs(15);
                    while !started.exists() {
                        assert!(
                            std::time::Instant::now() < deadline,
                            "stand-in did not signal startup"
                        );
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    cancel.cancel(quire_exact::CancelCause::Requested);
                })
            });
            let result =
                crate::kani::run::scalar::execute_kani_scalar_native(&KaniScalarNativeRequest {
                    cancel: &cancel,
                    installation: &installation,
                    harness: &harness,
                    oracle_artifacts: &artifacts,
                    operands: &[4, 1],
                    scalar_limits: limits(),
                    target_directory: &target,
                });
            if let Some(watcher) = watcher {
                watcher.join().unwrap();
            }
            result.unwrap_err()
        });
        assert!(started.exists(), "bounded Kani stand-in actually started");
        assert_eq!(error.code(), "proof_not_verified");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
        let KaniScalarNativeError::ProofNotVerified(evidence) = error else {
            panic!("expected single-run evidence")
        };
        let expected = if cancelled {
            KaniRunOutcome::Cancelled {
                source: quire_contract_ir::kani::KaniCancellationSource::Requested,
            }
        } else {
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut,
            }
        };
        assert_eq!(evidence.outcome, expected);
        assert!(!target.join("kani-scalar-native").exists());
        fs::remove_dir_all(control).unwrap();
    }
}

#[trace("FR-017-AC-42")]
#[test]
#[ignore = "real Kani paired false-bound control: run this private lib control explicitly and serially"]
fn seeded_false_bound_harness_produces_a_paired_completion_refusal() {
    use crate::kani::{
        classify::KaniRunOutcome,
        generate::false_bound_harness_for_test,
        run::scalar::{
            execute_kani_scalar_native, KaniScalarNativeError, KaniScalarNativeErrorClass,
        },
    };
    let (original, artifacts) = fixture(Duration::from_secs(600));
    let harness = false_bound_harness_for_test(&original, &artifacts);
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("installed Kani");
    let target =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/private-scalar-false-bound");
    let error = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &[4, 1],
        scalar_limits: limits(),
        target_directory: &target,
    })
    .unwrap_err();
    assert_eq!(error.code(), "proof_not_verified");
    assert_eq!(error.class(), KaniScalarNativeErrorClass::CompletionRefusal);
    let KaniScalarNativeError::ProofNotVerified(evidence) = error else {
        panic!("expected real proof evidence")
    };
    assert!(
        matches!(evidence.outcome, KaniRunOutcome::Falsified { .. }),
        "{evidence:?}"
    );
    assert!(
        !target.join("kani-scalar-native").exists(),
        "falsification prevents native invocation"
    );
}
