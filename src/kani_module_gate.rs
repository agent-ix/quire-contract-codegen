//! The claimed-module proof gate over one Kani run (ADR-011 section 2.3).
//!
//! A gate publishes the modules it claims. It passes only when the run verified and, for each
//! claimed module, the prover's transcript lists at least one check with status `SUCCESS` whose
//! location is inside that module. Every other status (`UNREACHABLE`, `UNDETERMINED`, a cover's
//! `SATISFIED`) discharges nothing, so a module the prover compiled but reached no proposition in
//! is reported as [`ModuleStatus::Unreached`] and fails the gate.
//!
//! The mutation controls are the caller's: a defect injected inside a claimed module must turn
//! this gate red, which it does through [`ModuleGateFailure::NotVerified`].

use crate::kani_execution::KaniRunOutcome;
use crate::kani_transcript::{KaniCheckStatus, KaniTranscript};

/// Whether the prover discharged a check inside one claimed module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleStatus {
    /// At least one check located in the module has status `SUCCESS`.
    Discharged {
        /// How many such checks the transcript lists.
        success_checks: usize,
    },
    /// No check located in the module has status `SUCCESS`.
    Unreached,
}

/// One claimed module and its status in the transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimedModuleReport {
    /// The claimed module's Rust path, as Kani prints it in a check's location.
    pub module: String,
    /// What the transcript shows inside it.
    pub status: ModuleStatus,
}

/// Why the gate is red.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModuleGateFailure {
    /// The gate claims no module, so it discharges nothing.
    NoClaims,
    /// The run did not verify, so no claimed module is proved.
    NotVerified(KaniRunOutcome),
    /// The run verified, but the prover reached no proposition inside these claimed modules.
    Unreached(Vec<String>),
}

/// Applies the gate to one run: `outcome` is the run's classification and `transcript` its
/// printed text. On success, returns one report per claimed module, in claim order.
///
/// # Errors
///
/// [`ModuleGateFailure`] when no module is claimed, the run did not verify, or a claimed module
/// has no discharged check.
pub fn claimed_module_gate(
    claimed: &[&str],
    outcome: &KaniRunOutcome,
    transcript: &str,
) -> Result<Vec<ClaimedModuleReport>, ModuleGateFailure> {
    if claimed.is_empty() {
        return Err(ModuleGateFailure::NoClaims);
    }
    if *outcome != KaniRunOutcome::Verified {
        return Err(ModuleGateFailure::NotVerified(outcome.clone()));
    }
    let transcript = KaniTranscript::parse(transcript);
    let reports = claimed
        .iter()
        .map(|module| {
            let nested = format!("{module}::");
            let success_checks = transcript
                .checks
                .iter()
                .filter(|check| check.status == KaniCheckStatus::Success)
                .filter_map(|check| check.function.as_deref())
                .filter(|function| *function == *module || function.starts_with(&nested))
                .count();
            ClaimedModuleReport {
                module: (*module).to_owned(),
                status: if success_checks == 0 {
                    ModuleStatus::Unreached
                } else {
                    ModuleStatus::Discharged { success_checks }
                },
            }
        })
        .collect::<Vec<_>>();
    let unreached = reports
        .iter()
        .filter(|report| report.status == ModuleStatus::Unreached)
        .map(|report| report.module.clone())
        .collect::<Vec<_>>();
    if unreached.is_empty() {
        Ok(reports)
    } else {
        Err(ModuleGateFailure::Unreached(unreached))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRANSCRIPT: &str = "Check 1: subject::f.overflow.1\n\t - Status: SUCCESS\n\t - Location: src/lib.rs:3:5 in function subject::f\nCheck 2: gen::h.cover.1\n\t - Status: SATISFIED\n\t - Location: src/lib.rs:9:5 in function gen::h\nCheck 3: dead::g.assertion.1\n\t - Status: UNREACHABLE\n\t - Location: src/lib.rs:14:5 in function dead::g\n";

    /// A module with a `SUCCESS` check is discharged; a cover's `SATISFIED` and an `UNREACHABLE`
    /// check discharge nothing, and a module absent from the transcript is unreached.
    ///
    /// Trace: FR-023-AC-1, FR-023-AC-2, TC-034
    #[test]
    fn tc_034_only_success_checks_inside_a_module_discharge_it() {
        assert_eq!(
            claimed_module_gate(&["subject"], &KaniRunOutcome::Verified, TRANSCRIPT).unwrap(),
            [ClaimedModuleReport {
                module: "subject".to_owned(),
                status: ModuleStatus::Discharged { success_checks: 1 }
            }]
        );
        for unreached in ["gen", "dead", "absent"] {
            assert_eq!(
                claimed_module_gate(
                    &["subject", unreached],
                    &KaniRunOutcome::Verified,
                    TRANSCRIPT
                ),
                Err(ModuleGateFailure::Unreached(vec![unreached.to_owned()])),
                "{unreached}"
            );
        }
    }

    /// A claim on `sub` does not match `subject`'s checks: modules match whole path segments.
    ///
    /// Trace: FR-023-AC-4, TC-034
    #[test]
    fn tc_034_a_module_claim_matches_whole_path_segments() {
        assert_eq!(
            claimed_module_gate(&["sub"], &KaniRunOutcome::Verified, TRANSCRIPT),
            Err(ModuleGateFailure::Unreached(vec!["sub".to_owned()]))
        );
    }

    /// A run that did not verify, or a gate with no claims, is red however many checks succeed.
    ///
    /// Trace: FR-023-AC-1, FR-023-AC-4, TC-034
    #[test]
    fn tc_034_an_unverified_run_or_an_empty_claim_list_is_red() {
        let falsified = KaniRunOutcome::Falsified {
            counterexample: String::new(),
        };
        assert_eq!(
            claimed_module_gate(&["subject"], &falsified, TRANSCRIPT),
            Err(ModuleGateFailure::NotVerified(falsified))
        );
        assert_eq!(
            claimed_module_gate(&[], &KaniRunOutcome::Verified, TRANSCRIPT),
            Err(ModuleGateFailure::NoClaims)
        );
    }
}
