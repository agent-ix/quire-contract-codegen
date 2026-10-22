// SPDX-License-Identifier: AGPL-3.0-or-later

use ix_trace_rs::trace;

use super::{classify_pair, PairEvidence, QueryRun, ReachedLimit, SolverStatus};

type Association = (
    &'static str,
    &'static str,
    &'static str,
    u64,
    &'static str,
    &'static [u8],
);

fn base() -> Association {
    (
        "product-a",
        "item-a",
        "instance-v",
        1,
        "base",
        b"(check-sat)\n",
    )
}

fn step() -> Association {
    (
        "product-a",
        "item-a",
        "instance-v",
        1,
        "step",
        b"(check-sat)\n",
    )
}

fn record(association: Association, status: SolverStatus) -> QueryRun<Association> {
    QueryRun {
        association,
        status,
    }
}

#[trace("TC-059", "FR-045-AC-1", "FR-045-AC-2")]
#[test]
fn two_unsat_records_need_exact_base_and_step_association() {
    let base_run = record(base(), SolverStatus::Unsat);
    let step_run = record(step(), SolverStatus::Unsat);
    assert_eq!(
        classify_pair(&base(), &step(), Some(&base_run), Some(&step_run)),
        PairEvidence::InductionEligible
    );
    assert_eq!(
        classify_pair(&base(), &step(), Some(&step_run), Some(&base_run)),
        PairEvidence::AssociationMismatch
    );
    for changed_base in [
        (
            "product-b",
            "item-a",
            "instance-v",
            1,
            "base",
            b"(check-sat)\n" as &[u8],
        ),
        (
            "product-a",
            "item-b",
            "instance-v",
            1,
            "base",
            b"(check-sat)\n",
        ),
        (
            "product-a",
            "item-a",
            "instance-w",
            1,
            "base",
            b"(check-sat)\n",
        ),
        (
            "product-a",
            "item-a",
            "instance-v",
            2,
            "base",
            b"(check-sat)\n",
        ),
        (
            "product-a",
            "item-a",
            "instance-v",
            1,
            "step",
            b"(check-sat)\n",
        ),
        (
            "product-a",
            "item-a",
            "instance-v",
            1,
            "base",
            b"(check-sat) \n",
        ),
    ] {
        assert_eq!(
            classify_pair(
                &base(),
                &step(),
                Some(&record(changed_base, SolverStatus::Unsat)),
                Some(&step_run)
            ),
            PairEvidence::AssociationMismatch
        );
    }
}

#[trace("TC-059", "FR-045-AC-2")]
#[test]
fn solver_responses_do_not_become_terminal_proof_or_refutation() {
    let step_unsat = record(step(), SolverStatus::Unsat);
    assert_eq!(
        classify_pair(
            &base(),
            &step(),
            Some(&record(base(), SolverStatus::Sat)),
            Some(&step_unsat)
        ),
        PairEvidence::BaseSatCandidate
    );
    assert_eq!(
        classify_pair(
            &base(),
            &step(),
            Some(&record(base(), SolverStatus::Unsat)),
            Some(&record(step(), SolverStatus::Sat))
        ),
        PairEvidence::StepSatNotClosed
    );
    for status in [
        SolverStatus::Unknown,
        SolverStatus::MissingTool,
        SolverStatus::Stopped(ReachedLimit::WallClock),
        SolverStatus::MalformedResponse,
        SolverStatus::ProcessFailed,
    ] {
        assert_eq!(
            classify_pair(
                &base(),
                &step(),
                Some(&record(base(), status)),
                Some(&step_unsat)
            ),
            PairEvidence::NonProof(status)
        );
    }
    assert_eq!(
        classify_pair(&base(), &step(), None, Some(&step_unsat)),
        PairEvidence::MissingResponse
    );
    assert_eq!(
        classify_pair(
            &base(),
            &step(),
            Some(&record(base(), SolverStatus::Sat)),
            Some(&record(
                step(),
                SolverStatus::Stopped(ReachedLimit::MemoryBytes)
            ))
        ),
        PairEvidence::NonProof(SolverStatus::Stopped(ReachedLimit::MemoryBytes))
    );
}
