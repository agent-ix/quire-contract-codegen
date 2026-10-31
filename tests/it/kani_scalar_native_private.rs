//! Private orchestration controls for scalar-native cleanup after real execution.
#[path = "../exact_scalar_support/package.rs"]
mod package;
use crate::{
    core::artifact::Artifact,
    kani::{
        classify::KaniRunOutcome,
        identity::KaniScalarObligationHarness,
        run::{
            execute::{execute_kani_obligation, KaniExecutionRequest},
            scalar::{execute_with_cleanup, KaniScalarNativeRequest},
            tool::KaniInstallation,
        },
        test_support::proof_ceilings::proof_ceilings_with_wall_clock,
    },
};
use crate::{
    core::contract_package::{
        routed_contract_lowering_profile, ContractDriverResultSelection, ContractRequestPosition,
    },
    oracle::scalar as scalar_fixture_types,
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
#[path = "kani_scalar_native_meter.rs"]
mod native_meter;
#[path = "kani_scalar_native_fixture.rs"]
mod scalar_native_fixture;
use scalar_native_fixture::{routed_native_scalar_with_artifacts, routed_scalar_with_artifacts};
use std::{fs, path::PathBuf, time::Duration};

struct NativeMutationScratch(PathBuf);

impl NativeMutationScratch {
    fn create() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!(
                "ir705-native-mutant-{}-{nonce}",
                std::process::id()
            ));
        fs::create_dir_all(path.join("src")).expect("scratch directory");
        Self(path)
    }
}

impl Drop for NativeMutationScratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[trace("FR-015-AC-126", "TC-025")]
#[test]
#[ignore = "real Kani qualification: run after independent review at the original 16 GiB ceiling"]
fn native_tc025_arithmetic_mutant_falsifies() {
    let (healthy, artifacts) = routed_native_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_100,
        Bound::Integer(0, 9),
        Duration::from_secs(600),
    );
    let manifest = artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("original dependency manifest");
    let original = "nx::NativeIntegerOperation::Add {";
    assert_eq!(healthy.rust.contents.matches(original).count(), 1);
    let mut rust = healthy.rust.clone();
    rust.contents = rust
        .contents
        .replace(original, "nx::NativeIntegerOperation::Subtract {");
    // This private test deliberately seals its one fault injection; production callers cannot
    // re-seal a changed harness and the normal execution path still enforces provenance.
    let mutant = KaniScalarObligationHarness::generated(
        healthy.identity.clone(),
        rust,
        healthy.record.clone(),
    );
    let counterexample = run_mutant(&mutant, manifest);
    let playback = crate::kani::output::playback::parse_assertion_playback(
        &counterexample,
        mutant.identity.harness_symbol.as_str(),
        mutant.identity.module_symbol.as_str(),
        crate::replay::scalar::SCALAR_PLAYBACK_BYTES,
        usize::MAX,
    )
    .expect("actual backend playback is valid");
    let witness = crate::replay::scalar::decode_scalar_falsification(&mutant.identity, &playback)
        .expect("actual backend witness decodes against original intervals");
    let [left, right] = witness.as_slice() else {
        panic!("two original symbolic integer draws: {witness:?}");
    };
    assert!((0..=9).contains(left));
    assert_eq!(*right, 1);
}

#[trace("FR-015-AC-126", "TC-027")]
#[test]
#[ignore = "real Kani qualification: run after independent review at the original 16 GiB ceiling"]
fn native_original_tc027_false_bound_falsifies() {
    let (healthy, artifacts) = routed_native_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_100,
        Bound::Integer(0, 9),
        Duration::from_secs(600),
    );
    let manifest = artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("original dependency manifest");
    let original = "exact <= i128::from(9_i64);";
    assert_eq!(healthy.rust.contents.matches(original).count(), 1);
    let mut rust = healthy.rust.clone();
    rust.contents = rust
        .contents
        .replacen(original, "exact <= i128::from(5_i64);", 1);
    let mutant = KaniScalarObligationHarness::generated(
        healthy.identity.clone(),
        rust,
        healthy.record.clone(),
    );
    let counterexample = run_mutant(&mutant, manifest);
    let playback = crate::kani::output::playback::parse_assertion_playback(
        &counterexample,
        mutant.identity.harness_symbol.as_str(),
        mutant.identity.module_symbol.as_str(),
        crate::replay::scalar::SCALAR_PLAYBACK_BYTES,
        usize::MAX,
    )
    .expect("actual backend playback is valid");
    let witness = crate::replay::scalar::decode_scalar_falsification(&mutant.identity, &playback)
        .expect("actual backend witness decodes against original intervals");
    let [left, right] = witness.as_slice() else {
        panic!("two original symbolic integer draws: {witness:?}");
    };
    assert!((0..=9).contains(left));
    assert_eq!(*right, 1);
    assert!(i128::from(*left) + 1 > 5);
}

fn run_mutant(mutant: &KaniScalarObligationHarness, manifest: &Artifact) -> String {
    let scratch = NativeMutationScratch::create();
    fs::write(scratch.0.join("Cargo.toml"), &manifest.contents).unwrap();
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        scratch.0.join("Cargo.lock"),
    )
    .unwrap();
    fs::write(scratch.0.join("src/lib.rs"), &mutant.rust.contents).unwrap();
    fs::write(
        scratch.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let metadata = std::process::Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&scratch.0)
        .output()
        .expect("resolve seeded dependency lock");
    assert!(
        metadata.status.success(),
        "seeded dependency lock failed: {}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let cancel = quire_exact::Cancel::new();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: mutant.into(),
        crate_directory: &scratch.0,
        target_directory: &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/ir705-native-scalar-kani"),
        home_directory: None,
    })
    .expect("the private mutation preserves the execution module's bounded launch");
    let KaniRunOutcome::Falsified { counterexample } = evidence.outcome else {
        panic!("the original-domain native mutant must falsify: {evidence:?}");
    };
    assert!(
        counterexample.contains("Check for `assertion`")
            && counterexample.contains("the oracle must complete with exactly the native quire.op.integer.add result when it lies in the checked domain, and refuse otherwise"),
        "the generated sound assertion must fail: {counterexample}"
    );
    counterexample
}

pub(crate) fn fixture(wall_clock: Duration) -> (KaniScalarObligationHarness, Vec<Artifact>) {
    routed_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_190,
        Bound::Integer(0, 9),
        wall_clock,
    )
}

fn limits() -> quire_exact::ScalarLimits {
    quire_exact::ScalarLimits {
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
    assert_eq!(success.staged_entries(), failure.staged_entries());
    assert_eq!(success.meter_observation().limits(), &limits());
    assert_eq!(
        success.meter_observation().limits(),
        failure.meter_observation().limits()
    );
}

#[trace("FR-017-AC-42")]
#[test]
fn bounded_single_run_preserves_cancelled_with_start_signal_and_timeout_without_signal() {
    use crate::kani::{
        classify::{KaniInconclusiveReason, KaniRunOutcome},
        run::scalar::{KaniScalarNativeError, KaniScalarNativeErrorClass},
    };
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicBool, Ordering};
    struct CompletionGuard<'a>(&'a AtomicBool);

    impl Drop for CompletionGuard<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }

    for cancelled in [true, false] {
        let (harness, artifacts) = fixture(Duration::from_secs(30));
        let control = crate::core::test_support::discover_scratch(if cancelled {
            "scalar-cancel"
        } else {
            "scalar-timeout"
        });
        fs::create_dir_all(&control).unwrap();
        let started = control.join("started");
        let launcher = control.join("standin");
        let script = if cancelled {
            format!(
                "#!/bin/sh\nprintf started > '{}'\nexec /bin/sleep 60\n",
                started.display()
            )
        } else {
            "#!/bin/sh\nexec /bin/sleep 60\n".to_owned()
        };
        fs::write(&launcher, script).unwrap();
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o700)).unwrap();
        let installation = KaniInstallation::with_test_only_patient_pidfd_confirmation(launcher);
        let cancel = quire_exact::Cancel::new();
        let target = control.join("target");
        let completed = AtomicBool::new(false);
        let error = std::thread::scope(|scope| {
            let completion_guard = CompletionGuard(&completed);
            let watcher = cancelled.then(|| {
                scope.spawn(|| {
                    while !started.exists() && !completed.load(Ordering::Acquire) {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    if started.exists() {
                        cancel.cancel(quire_exact::CancelCause::Requested);
                        true
                    } else {
                        false
                    }
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
            drop(completion_guard);
            if let Some(watcher) = watcher {
                assert!(
                    watcher.join().unwrap(),
                    "proof returned before stand-in startup: {result:?}"
                );
            }
            result.unwrap_err()
        });
        // Timeout evidence must survive even when the backend never emits the
        // startup signal; the cancellation watcher has asserted its signal.
        assert_eq!(error.code(), "proof_not_verified", "{error:?}");
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
    let control = crate::core::test_support::discover_scratch("paired-oracle-stop");
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
            "--manifest-path",
            "observer/Cargo.toml",
            "--example",
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

#[trace(
    "FR-017-AC-29",
    "FR-017-AC-46",
    "FR-017-AC-48",
    "FR-017-AC-52",
    "FR-017-AC-53",
    "TC-027"
)]
#[test]
#[ignore = "installed Kani codegen and native compiler control: run explicitly and serially; no verification"]
fn staged_routed_native_companion_retains_custody_and_dev_only_feature_boundary() {
    let (harness, artifacts) = fixture(Duration::from_secs(600));
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("installed Kani");
    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    crate::kani::run::scalar::qualify_staged_native_for_test(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &[4, 1],
        scalar_limits: limits(),
        target_directory: &target,
    });
}
