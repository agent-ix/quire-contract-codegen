//! Classification of one Kani run from the report Kani exported for it (FR-017).

use quire_contract_ir::kani::{KaniOutcome, KaniOutcomeKind};
use quire_contract_model::Std001Code;
use serde::Serialize;

use crate::kani::{
    identity::ObligationKind,
    output::{
        playback::counterexample_playback,
        report::{
            KaniCheckResult, KaniHarnessReport, KaniHarnessStatus, KaniMemberReport,
            KaniReportRefusal,
        },
    },
};

/// Why a run proves nothing. Every variant but [`TimedOut`](Self::TimedOut) and [`MemoryExhausted`](Self::MemoryExhausted) is a completed run
/// the backend printed no usable verdict for; `TimedOut` is not a completed run at all — it is
/// killed before it ever prints one.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniInconclusiveReason {
    /// Kani reported failure but printed no concrete playback for a failed property.
    FailedWithoutCounterexample,
    /// Kani exported no verdict: a build, launcher or solver failure that ended the process
    /// before a report was written, or a process that failed beside a report of success.
    NoVerdict,
    /// Kani reported success with no cover property in the report, so non-vacuity was not
    /// observed.
    MissingCoverSummary,
    /// Kani's report lists zero successful checks: no check in the obligation actually held, so
    /// a `Proved`-looking run proved nothing. This settles
    /// [`quire_contract_ir::kani::KaniOutcomeKind::Inconclusive`]'s `kani_vacuous_proof` cause
    /// for the execution path, using the check count this module itself reads from the
    /// backend's own report — never a generation-time value — so it is an execution outcome this
    /// module observed, not a generation-time classification reported as one (FR-017-CON-2).
    VacuousProof,
    /// A loop-unwinding check failed: the loop bound was exhausted before the property
    /// could be decided, so no failure is a counterexample.
    UnwindBoundExhausted,
    /// The run did not conclude within
    /// [`ProofCeilings::wall_clock`](crate::kani::identity::ProofCeilings::wall_clock).
    /// The launcher and every
    /// process it forked were killed; no verdict, failed-check count or playback is available
    /// because none was ever printed.
    TimedOut,
    /// The observed backend-tree resident memory exceeded the harness identity ceiling.
    MemoryExhausted,
}

/// The backend-reported outcome of one run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum KaniRunOutcome {
    /// Every property held and every cover was satisfied, so the harness's assumptions,
    /// requires and IR bounds are jointly satisfiable.
    Verified,
    /// A property other than a loop-unwinding check failed and Kani printed a concrete
    /// counterexample for it.
    Falsified {
        /// The concrete-playback test Kani printed for the failed check, verbatim.
        counterexample: String,
    },
    /// Vacuous: no property failed, but a cover was not satisfied within the bounds. For a
    /// precondition harness the precondition is unsatisfiable; for a contract harness the
    /// requires and IR bounds are jointly unsatisfiable, so the ensures was never checked.
    CoverUnsatisfied {
        /// Satisfied cover properties.
        satisfied: u64,
        /// Total cover properties.
        total: u64,
    },
    /// The run established nothing.
    Inconclusive {
        /// Why.
        reason: KaniInconclusiveReason,
    },
}

/// A classified run: the outcome and the check count it was classified from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassifiedRun {
    /// The outcome.
    pub outcome: KaniRunOutcome,
    /// The count
    /// [`KaniExecutionEvidence::success_checks`](crate::kani::run::execute::KaniExecutionEvidence::success_checks)
    /// documents.
    pub success_checks: u32,
    /// The checks
    /// [`KaniExecutionEvidence::checks`](crate::kani::run::execute::KaniExecutionEvidence::checks)
    /// documents.
    pub checks: Vec<KaniCheckResult>,
}

/// Classifies one run from the report Kani exported for it. Every generated harness, of every
/// kind, carries exactly the covers that witness its assumptions are satisfiable, so success
/// without every cover satisfied is vacuous and never `Verified`.
///
/// `report` is the exported report's bytes and `text` the run's console output, used only to find
/// the playback of a falsifying check. A run that exited unsuccessfully and exported no report
/// never reached a verdict (a build, launcher or solver failure), which is
/// [`KaniInconclusiveReason::NoVerdict`]. A run that exited successfully and exported none, or
/// any report [`KaniReportRefusal`] describes, is a refusal: the run is not read as
/// inconclusive, because that would let a change in Kani's output pass as a result.
///
/// Before consulting the covers, a report listing no successful check is routed through
/// [`KaniOutcome::proved_from_checks`] -- the one implementation of "a proof backed by zero
/// checks proved nothing" that this module and `quire-contract-ir` share
/// (agent-ix/quire-contract-codegen#99). A verdict of `Inconclusive` under
/// [`KaniOutcomeKind::Inconclusive`]'s `kani_vacuous_proof` cause is not reported as-is — that
/// would violate FR-017-CON-2, which forbids reporting a generation-time classification as an
/// execution outcome — it is mapped into this module's own [`KaniInconclusiveReason::VacuousProof`].
/// The count it classifies is read from this run's own report, never from generation time.
///
/// `pub` so a test asserting "this run proves falsification" can route through the same
/// classifier production uses (IR-220), instead of re-implementing the check and misreading an
/// inconclusive run — CBMC out-of-memory among them — as a decided failure.
///
/// A precondition harness asserts nothing: its one property is its non-vacuity cover, so the
/// zero-checks rule does not apply to it and the satisfied cover is its one successful check.
pub fn classify_kani_run(
    exited_successfully: bool,
    report: Option<&[u8]>,
    text: &str,
    kind: Option<ObligationKind>,
) -> Result<ClassifiedRun, KaniReportRefusal> {
    let Some(report) = report else {
        return if exited_successfully {
            Err(KaniReportRefusal::Missing)
        } else {
            Ok(ClassifiedRun {
                outcome: inconclusive(KaniInconclusiveReason::NoVerdict),
                success_checks: 0,
                checks: Vec::new(),
            })
        };
    };
    let report = KaniHarnessReport::parse(report)?;
    let outcome = classify_report(
        exited_successfully,
        &report,
        || counterexample_playback(text, None),
        kind,
    );
    Ok(classified(outcome, report, kind))
}

/// Classifies one member of a batch from its own entry of the report Kani exported for the
/// process (FR-017-AC-22).
///
/// `process_succeeded` is the success the member may claim: the process exited 0, or it exited
/// non-zero and at least one member's entry states failure. Kani exits 1 for any failed harness
/// and for its own errors alike, so a success beside a non-zero exit is believed only when some
/// entry accounts for that exit. `playback` is the console block headed for this member's path,
/// found by the caller; it is asked for only when this member's entry names a failed property.
///
/// An entry that states failure with no checks and was stopped by Kani's own per-harness timeout
/// is inconclusive as timed out, never falsified.
pub(crate) fn classify_member(
    process_succeeded: bool,
    member: &KaniMemberReport,
    playback: impl FnOnce() -> Option<String>,
    kind: Option<ObligationKind>,
) -> ClassifiedRun {
    let report = member.report.clone();
    let outcome = if member.timed_out
        && report.status == KaniHarnessStatus::Failure
        && report.checks.is_empty()
    {
        inconclusive(KaniInconclusiveReason::TimedOut)
    } else {
        classify_report(process_succeeded, &report, playback, kind)
    };
    classified(outcome, report, kind)
}

/// The run `outcome` was reached for, with the check count and the checks of its `report`.
fn classified(
    outcome: KaniRunOutcome,
    report: KaniHarnessReport,
    kind: Option<ObligationKind>,
) -> ClassifiedRun {
    let success_checks =
        report
            .property_successes()
            .saturating_add(if kind == Some(ObligationKind::Precondition) {
                report.covers_satisfied()
            } else {
                0
            });
    ClassifiedRun {
        outcome,
        success_checks,
        checks: report.checks,
    }
}

/// The classification rule (codegen#55), over the typed report only. `playback` finds the
/// falsifying playback, and is asked for only when the report names a failed property.
fn classify_report(
    exited_successfully: bool,
    report: &KaniHarnessReport,
    playback: impl FnOnce() -> Option<String>,
    kind: Option<ObligationKind>,
) -> KaniRunOutcome {
    match report.status {
        KaniHarnessStatus::Success if exited_successfully => classify_success(report, kind),
        KaniHarnessStatus::Success => inconclusive(KaniInconclusiveReason::NoVerdict),
        KaniHarnessStatus::Failure if report.failed_unwinding() => {
            inconclusive(KaniInconclusiveReason::UnwindBoundExhausted)
        }
        KaniHarnessStatus::Failure => match report.failed_property().then(playback).flatten() {
            Some(counterexample) => KaniRunOutcome::Falsified { counterexample },
            None => inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample),
        },
    }
}

fn classify_success(report: &KaniHarnessReport, kind: Option<ObligationKind>) -> KaniRunOutcome {
    if kind != Some(ObligationKind::Precondition) {
        let checks_outcome = KaniOutcome::proved_from_checks(
            usize::try_from(report.property_successes()).unwrap_or(usize::MAX),
            "kani_execution::classify_kani_run",
            "report_checks",
        );
        if checks_outcome.kind == KaniOutcomeKind::Inconclusive
            && checks_outcome.code == Std001Code::KANI_VACUOUS_PROOF
        {
            return inconclusive(KaniInconclusiveReason::VacuousProof);
        }
    }
    let (satisfied, total) = (report.covers_satisfied(), report.covers_total());
    match (satisfied, total) {
        (_, 0) => inconclusive(KaniInconclusiveReason::MissingCoverSummary),
        (satisfied, total) if satisfied == total => KaniRunOutcome::Verified,
        (satisfied, total) => KaniRunOutcome::CoverUnsatisfied {
            satisfied: u64::from(satisfied),
            total: u64::from(total),
        },
    }
}

fn inconclusive(reason: KaniInconclusiveReason) -> KaniRunOutcome {
    KaniRunOutcome::Inconclusive { reason }
}

#[cfg(test)]
mod synthetic {
    use super::*;
    use crate::kani::test_support::{report, COVER_NO, COVER_OK, PASSED};

    const COVER_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `cover`: \"contract assumptions are jointly satisfiable\"\n\n#[test]\nfn kani_concrete_playback_h_1() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";
    const ASSERTION_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `assertion`: \"|post_state: &i64| *post_state <= 5\"\n\n#[test]\nfn kani_concrete_playback_h_2() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![8, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";

    fn classify(
        exited_successfully: bool,
        report: Option<&[u8]>,
        text: &str,
        kind: Option<ObligationKind>,
    ) -> KaniRunOutcome {
        classify_kani_run(exited_successfully, report, text, kind)
            .expect("a readable report")
            .outcome
    }

    /// Only a successful report with every cover satisfied is verified; each other shape of run
    /// settles on its own outcome or reason.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-5, TC-027
    #[test]
    fn tc_027_run_classification_never_defaults_to_verified() {
        let verified = report("Success", &[PASSED, COVER_OK]);
        assert_eq!(
            classify(true, Some(&verified), COVER_PLAYBACK, None),
            KaniRunOutcome::Verified
        );
        // A contract whose requires are jointly unsatisfiable: every check but one is reachable,
        // and the cover after the contract call is not.
        assert_eq!(
            classify(
                true,
                Some(&report(
                    "Success",
                    &[PASSED, ("Unreachable", "assertion"), COVER_NO]
                )),
                "",
                None
            ),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            classify(
                true,
                Some(&report("Success", &[PASSED, COVER_OK, COVER_NO])),
                "",
                None
            ),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
        // The failure's counterexample is the assertion playback, not the cover playback.
        let falsified = report("Failure", &[("Failure", "assertion"), COVER_OK]);
        assert!(matches!(
            classify(false, Some(&falsified), &format!("{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"), None),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`") && !counterexample.contains("Check for `cover`")
        ));
        for (success, report, text, expected) in [
            (
                false,
                Some(falsified.clone()),
                COVER_PLAYBACK,
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
            // A failed run with no failed property is not a counterexample, whatever it printed.
            (
                false,
                Some(report("Failure", &[("Undetermined", "assertion"), COVER_OK])),
                ASSERTION_PLAYBACK,
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
            // No report and a failed exit: a build or launcher failure.
            (
                false,
                None,
                "error[E0308]: mismatched types",
                KaniInconclusiveReason::NoVerdict,
            ),
            // A report of success from a process that exited unsuccessfully is not success.
            (false, Some(verified.clone()), "", KaniInconclusiveReason::NoVerdict),
            (
                true,
                Some(report("Success", &[PASSED])),
                "",
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            // CBMC's own out-of-memory abort writes no report and exits unsuccessfully, with zero
            // properties ever decided (IR-220): it is never falsified.
            (
                false,
                None,
                "Runtime Convert SSA: 15.3149s\nOut of memory\n\nCBMC failed with status 6\nVERIFICATION:- FAILED\n",
                KaniInconclusiveReason::NoVerdict,
            ),
        ] {
            assert_eq!(
                classify(success, report.as_deref(), text, None),
                KaniRunOutcome::Inconclusive { reason: expected },
                "{text}"
            );
        }
    }

    /// A report listing no successful check is inconclusive under `VacuousProof`, never verified,
    /// even with every cover satisfied.
    ///
    /// Trace: FR-017-AC-13, TC-027
    #[test]
    fn a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied(
    ) {
        let vacuous = report("Success", &[COVER_OK]);
        let result = classify_kani_run(true, Some(&vacuous), "", None).unwrap();
        assert_eq!(
            result.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            },
            "zero successful checks must not be Verified merely because covers were satisfied"
        );
        assert_eq!(result.success_checks, 0);
        // Checks that are not successes do not count toward the proof. (A success report that lists
        // an undetermined or unknown check is refused instead; see `output::report`.)
        assert_eq!(
            classify(
                true,
                Some(&report(
                    "Success",
                    &[("Unreachable", "assertion"), COVER_OK]
                )),
                "",
                None
            ),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            },
        );
        // One successful check is enough for the cover to decide.
        let proved = classify_kani_run(
            true,
            Some(&report("Success", &[PASSED, PASSED, COVER_OK])),
            "",
            None,
        )
        .unwrap();
        assert_eq!(proved.outcome, KaniRunOutcome::Verified);
        assert_eq!(proved.success_checks, 2);
    }

    /// An exhausted unwind bound is inconclusive even when Kani prints a playback and a
    /// property also failed; a succeeded unwinding check is not a failure.
    ///
    /// Trace: FR-017-AC-5, TC-027
    #[test]
    fn tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified() {
        let unwound = report(
            "Failure",
            &[
                ("Failure", "unwind"),
                ("Undetermined", "assertion"),
                ("Failure", "assertion"),
                COVER_OK,
            ],
        );
        assert_eq!(
            classify(
                false,
                Some(&unwound),
                &format!("{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"),
                None
            ),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
        let listed = report("Success", &[("Success", "unwind"), PASSED, COVER_OK]);
        assert_eq!(
            classify(true, Some(&listed), COVER_PLAYBACK, None),
            KaniRunOutcome::Verified
        );
    }

    /// A run that exits successfully but exports no report, and every report this crate cannot
    /// read exactly, is a refusal rather than an outcome: the result is never
    /// `Inconclusive` by default.
    ///
    /// Trace: FR-017-AC-18, TC-027
    #[test]
    fn tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive() {
        assert_eq!(
            classify_kani_run(true, None, "VERIFICATION:- SUCCESSFUL", None),
            Err(KaniReportRefusal::Missing)
        );
        for exited in [true, false] {
            assert!(matches!(
                classify_kani_run(exited, Some(b"VERIFICATION:- SUCCESSFUL"), "", None),
                Err(KaniReportRefusal::Malformed { .. })
            ));
        }
    }
}

#[cfg(test)]
mod real_capture {
    use super::*;
    use crate::kani::output::report::{KaniCheckClass, KaniCheckLocation, KaniCheckStatus};

    /// A real Kani capture of one `--exact` harness: the exported report, its stdout, and
    /// whether it exited successfully.
    struct Capture {
        report: &'static str,
        stdout: &'static str,
        exited_successfully: bool,
    }

    macro_rules! capture {
        ($name:literal) => {
            Capture {
                report: include_str!(concat!("../../tests/fixtures/kani-report/", $name, ".json")),
                stdout: include_str!(concat!(
                    "../../tests/fixtures/kani-report/",
                    $name,
                    ".stdout"
                )),
                exited_successfully: include_str!(concat!(
                    "../../tests/fixtures/kani-report/",
                    $name,
                    ".exit"
                ))
                .trim()
                    == "0",
            }
        };
    }

    fn classified(capture: &Capture) -> KaniRunOutcome {
        classify_kani_run(
            capture.exited_successfully,
            Some(capture.report.as_bytes()),
            capture.stdout,
            None,
        )
        .expect("a real Kani report is readable")
        .outcome
    }

    fn report(capture: &Capture) -> KaniHarnessReport {
        KaniHarnessReport::parse(capture.report.as_bytes()).expect("a real Kani report is readable")
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_success_with_a_satisfied_cover_is_verified() {
        let capture = capture!("verified-satisfied-cover");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Success);
        assert_eq!(parsed.property_successes(), 1);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 1));
        assert!(!parsed.failed_property() && !parsed.failed_unwinding());
        assert_eq!(classified(&capture), KaniRunOutcome::Verified);
    }

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_failure_carries_the_assertion_playback_not_the_cover_one() {
        let capture = capture!("falsified-with-playback");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Failure);
        assert!(parsed.failed_property() && !parsed.failed_unwinding());
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 1));
        assert!(matches!(
            classified(&capture),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`")
                    && !counterexample.contains("Check for `cover`")
                    && counterexample.contains("assertion failed: x < 5")
        ));
    }

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_unwinding_failure_is_inconclusive_not_falsified() {
        let capture = capture!("unwind-exhausted");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Failure);
        assert!(parsed.failed_unwinding());
        assert!(
            !parsed.failed_property(),
            "the undetermined checks are not failures"
        );
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
    }

    /// Kani reports a harness whose every check is unreachable as a success with zero successful
    /// checks, so it is vacuous before its cover is consulted.
    ///
    /// Trace: FR-017-AC-12, FR-017-AC-13, TC-027
    #[test]
    fn tc_027_real_kani_a_run_with_no_successful_check_is_a_vacuous_proof() {
        let capture = capture!("vacuous-cover");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Success);
        assert_eq!(parsed.property_successes(), 0);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (0, 1));
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            }
        );
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_partly_satisfied_covers_are_cover_unsatisfied() {
        let capture = capture!("partial-cover");
        let parsed = report(&capture);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 2));
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_success_without_a_cover_is_inconclusive() {
        let capture = capture!("no-cover");
        assert_eq!(report(&capture).covers_total(), 0);
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary
            }
        );
    }

    /// A run's verdict is the report's, whatever the console says: the success banner printed
    /// beside a failing report decides nothing.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_the_console_banner_never_decides_the_verdict() {
        let capture = capture!("falsified-with-playback");
        let outcome = classify_kani_run(
            false,
            Some(capture.report.as_bytes()),
            "VERIFICATION:- SUCCESSFUL",
            None,
        )
        .expect("readable")
        .outcome;
        assert_eq!(
            outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
        );
        let verified = capture!("verified-satisfied-cover");
        let outcome = classify_kani_run(
            true,
            Some(verified.report.as_bytes()),
            "VERIFICATION:- FAILED",
            None,
        )
        .expect("readable")
        .outcome;
        assert_eq!(outcome, KaniRunOutcome::Verified);
    }

    fn mutated(edit: impl FnOnce(&mut serde_json::Value)) -> Vec<u8> {
        let mut report: serde_json::Value =
            serde_json::from_str(capture!("verified-satisfied-cover").report).unwrap();
        edit(&mut report);
        serde_json::to_vec(&report).unwrap()
    }

    /// A report that differs from the schema this module reads is refused with its own typed
    /// cause and never classified.
    ///
    /// Trace: FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_report_that_changed_shape_is_refused_not_classified() {
        let version = mutated(|report| report["metadata"]["version"] = "2.0".into());
        assert_eq!(
            KaniHarnessReport::parse(&version),
            Err(KaniReportRefusal::UnsupportedVersion {
                found: "2.0".to_owned()
            })
        );
        let status = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["status"] = "Proved".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&status),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let harness_status = mutated(|report| {
            report["verification_results"]["results"][0]["status"] = "Timeout".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&harness_status),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let renamed = mutated(|report| {
            let results = report["verification_results"].take();
            report["verification"] = results;
            report
                .as_object_mut()
                .unwrap()
                .remove("verification_results");
        });
        assert!(matches!(
            KaniHarnessReport::parse(&renamed),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        assert!(matches!(
            KaniHarnessReport::parse(b"VERIFICATION:- SUCCESSFUL"),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        for refusal in [&version, &status, &harness_status, &renamed] {
            let refused = classify_kani_run(true, Some(refusal), "VERIFICATION:- SUCCESSFUL", None);
            assert!(refused.is_err(), "a refused report is never a verdict");
        }
    }

    /// A report holding any number of harness results but one is refused: a run selects one
    /// `--exact` harness, so another count means the selection or the report is not what was
    /// asked for.
    ///
    /// Trace: FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_report_without_exactly_one_harness_is_refused() {
        let none = mutated(|report| {
            report["verification_results"]["results"] = serde_json::json!([]);
        });
        assert_eq!(
            KaniHarnessReport::parse(&none),
            Err(KaniReportRefusal::HarnessCount { found: 0 })
        );
        let two = mutated(|report| {
            let first = report["verification_results"]["results"][0].clone();
            report["verification_results"]["results"]
                .as_array_mut()
                .unwrap()
                .push(first);
        });
        assert_eq!(
            KaniHarnessReport::parse(&two),
            Err(KaniReportRefusal::HarnessCount { found: 2 })
        );
    }

    /// A report whose harness says success but which lists a check that failed, errored, was
    /// undetermined or unknown, or an unwinding assertion that failed, contradicts itself and is
    /// refused with its own cause, for every obligation kind. It is never `Verified`.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified() {
        for status in ["Failure", "Error", "Undetermined", "Unknown"] {
            for category in ["assertion", "unwind", "cover"] {
                let report = mutated(|report| {
                    let checks = &mut report["verification_results"]["results"][0]["checks"];
                    let last = checks.as_array().unwrap().len() - 1;
                    checks[last]["status"] = status.into();
                    checks[last]["category"] = category.into();
                });
                let found = KaniHarnessReport::parse(&report);
                assert!(
                    matches!(found, Err(KaniReportRefusal::Inconsistent { .. })),
                    "{category} {status}: {found:?}"
                );
                for kind in [None, Some(ObligationKind::Precondition)] {
                    let run = classify_kani_run(true, Some(&report), "", kind);
                    assert!(
                        matches!(run, Err(KaniReportRefusal::Inconsistent { .. })),
                        "{category} {status} {kind:?}: {run:?}"
                    );
                }
            }
        }
        let held = mutated(|_| {});
        assert!(KaniHarnessReport::parse(&held).is_ok());
    }

    /// The per-check view of a real run names every check with its id, class, source location and
    /// status, so a consumer can attribute each successful check to a source line.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status() {
        let capture = capture!("falsified-with-playback");
        let run = classify_kani_run(
            capture.exited_successfully,
            Some(capture.report.as_bytes()),
            capture.stdout,
            None,
        )
        .unwrap();
        let at = |line| KaniCheckLocation {
            file: "src/lib.rs".to_owned(),
            line: Some(line),
        };
        assert_eq!(
            run.checks,
            [
                KaniCheckResult {
                    id: 1,
                    class: KaniCheckClass::Cover,
                    location: at(15),
                    status: KaniCheckStatus::Satisfied,
                },
                KaniCheckResult {
                    id: 2,
                    class: KaniCheckClass::from("assertion".to_owned()),
                    location: at(16),
                    status: KaniCheckStatus::Failure,
                },
            ]
        );
        let unwound = capture!("unwind-exhausted");
        let run = classify_kani_run(
            unwound.exited_successfully,
            Some(unwound.report.as_bytes()),
            unwound.stdout,
            None,
        )
        .unwrap();
        assert!(run
            .checks
            .iter()
            .any(|check| check.class == KaniCheckClass::Unwind
                && check.status == KaniCheckStatus::Failure
                && check.location.line == Some(24)));
    }

    /// A location Kani leaves unknown has no line; a line that is neither a number nor Kani's
    /// word for none is a malformed report, not a dropped location.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused() {
        let unknown = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["location"]["line"] =
                "unknown".into();
        });
        let parsed = KaniHarnessReport::parse(&unknown).unwrap();
        assert_eq!(parsed.checks[0].location.line, None);
        let bad = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["location"]["line"] =
                "12a".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&bad),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let no_location = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]
                .as_object_mut()
                .unwrap()
                .remove("location");
        });
        assert!(matches!(
            KaniHarnessReport::parse(&no_location),
            Err(KaniReportRefusal::Malformed { .. })
        ));
    }

    /// A check category outside the two this module names is a property: an unknown category must
    /// still count against a proof, never be dropped.
    ///
    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_an_unnamed_check_category_is_a_property() {
        let report = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["category"] =
                "pointer_dereference".into();
            report["verification_results"]["results"][0]["checks"][0]["status"] = "Failure".into();
            report["verification_results"]["results"][0]["status"] = "Failure".into();
        });
        let parsed = KaniHarnessReport::parse(&report).unwrap();
        assert!(parsed.failed_property());
        assert_eq!(
            parsed.checks[0].class,
            KaniCheckClass::from("pointer_dereference".to_owned())
        );
    }
}
