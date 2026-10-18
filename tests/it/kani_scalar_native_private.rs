//! Private orchestration controls for scalar-native cleanup after real execution.
#[path = "../exact_scalar_support/package.rs"]
mod package;
use crate::{
    core::artifact::Artifact,
    kani::{
        identity::KaniScalarObligationHarness,
        run::{
            scalar::{execute_with_cleanup, KaniScalarNativeRequest},
            tool::KaniInstallation,
        },
        test_support::{
            proof_ceilings::proof_ceilings_with_wall_clock,
            scalar_native_digest::independent_digest,
        },
    },
};
use ix_trace_rs::trace;
use package::Bound;
#[path = "kani_scalar_native_fixture.rs"]
mod scalar_native_fixture;
use scalar_native_fixture::routed_scalar_with_artifacts;
use std::{fs, path::PathBuf, time::Duration};

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
    assert_eq!(
        failure.proof_content_digest().as_bytes(),
        &independent_digest(failure.staged_entries())
    );
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

/// Real proof execution followed by a bounded native-helper protocol stand-in. The actual
/// process emits OracleStop; neither proof evidence nor the returned outer error is constructed.
#[trace("FR-017-AC-42")]
#[test]
#[ignore = "real Kani with bounded native-helper stop stand-in: run this private lib control explicitly and serially"]
fn bounded_paired_runner_propagates_the_native_helpers_oracle_stop() {
    use crate::kani::run::scalar::{
        execute_kani_scalar_native, KaniScalarNativeError, KaniScalarNativeErrorClass,
        ScalarOracleStopKind,
    };
    use std::os::unix::fs::PermissionsExt;
    let (harness, artifacts) = fixture(Duration::from_secs(600));
    let control = crate::kani::test_support::discover_scratch("paired-oracle-stop");
    fs::create_dir_all(&control).unwrap();
    let helper = control.join("native-helper");
    let observed_arguments = control.join("native-arguments");
    fs::write(&helper, format!("#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf 'IR791_NATIVE_V1\\toracle_stop\\tinvalid_constant\\nIR791_LIMITS_V1\\t101\\t102\\t103\\t104\\t105\\t106\\t107\\t108\\t109\\t110\\nIR791_METER_V1\\t0\\t0\\t0\\t0\\t0\\t0\\t0\\t0\\t0\\t0\\n'\n", observed_arguments.display())).unwrap();
    fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
    let installation = KaniInstallation::discover()
        .expect("installed real Kani")
        .with_scalar_native_launcher(helper);
    let cancel = quire_exact::Cancel::new();
    let target =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/private-scalar-oracle-stop");
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
    assert!(matches!(
        error,
        KaniScalarNativeError::OracleStop(ScalarOracleStopKind::InvalidConstant)
    ));
    assert_eq!(error.code(), "oracle_stop");
    assert_eq!(error.class(), KaniScalarNativeErrorClass::CompletionRefusal);
    assert_eq!(
        fs::read_to_string(observed_arguments)
            .unwrap()
            .lines()
            .collect::<Vec<_>>(),
        [
            "run",
            "--quiet",
            "--locked",
            "--bin",
            "ir791_native",
            "--",
            "4",
            "1",
            "101",
            "102",
            "103",
            "104",
            "105",
            "106",
            "107",
            "108",
            "109",
            "110"
        ]
    );
    fs::remove_dir_all(control).unwrap();
}
