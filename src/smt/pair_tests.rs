// SPDX-License-Identifier: AGPL-3.0-or-later

use std::num::NonZeroU64;
use std::{num::NonZeroUsize, time::Duration};

use ix_trace_rs::trace;
use quire_exact::Cancel;

use super::{
    classify_pair, run_solver_query, PairEvidence, QueryAssociation, QueryRun, ReachedLimit,
    SolverStatus,
};
use crate::kani::run::process::{ProcessLimits, ProcessOutcome};

type Association = QueryAssociation<&'static str, &'static str, &'static str, &'static str>;
type Run = QueryRun<&'static str, &'static str, &'static str, &'static str>;

fn base() -> Association {
    QueryAssociation {
        product: "product-a",
        item: "item-a",
        instance: "instance-v",
        depth: NonZeroU64::MIN,
        kind: "base",
        query_bytes: b"(check-sat)\n".to_vec(),
        attempt: NonZeroU64::MIN,
    }
}

fn step() -> Association {
    QueryAssociation {
        kind: "step",
        attempt: NonZeroU64::new(2).expect("two is nonzero"),
        ..base()
    }
}

fn record(association: Association, status: SolverStatus) -> Run {
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
        QueryAssociation {
            product: "product-b",
            ..base()
        },
        QueryAssociation {
            item: "item-b",
            ..base()
        },
        QueryAssociation {
            instance: "instance-w",
            ..base()
        },
        QueryAssociation {
            depth: NonZeroU64::new(2).expect("two is nonzero"),
            ..base()
        },
        QueryAssociation {
            kind: "step",
            ..base()
        },
        QueryAssociation {
            query_bytes: b"(check-sat) \n".to_vec(),
            ..base()
        },
        QueryAssociation {
            attempt: NonZeroU64::new(3).expect("three is nonzero"),
            ..base()
        },
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

fn process_limits(input_bytes: usize) -> ProcessLimits {
    ProcessLimits {
        input_bytes: NonZeroUsize::new(input_bytes).expect("finite input limit"),
        wall_clock: Duration::from_secs(5),
        memory_bytes: NonZeroU64::new(128 * 1024 * 1024).expect("finite memory limit"),
        output_bytes: NonZeroUsize::new(1024).expect("finite capture limit"),
    }
}

#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]
#[test]
fn completed_solver_run_keeps_exact_query_and_bounded_raw_response() {
    let association = QueryAssociation {
        query_bytes: b"unsat\n".to_vec(),
        ..base()
    };
    let execution = run_solver_query(
        association.clone(),
        "/usr/bin/cat".into(),
        vec![],
        process_limits(1024),
        Cancel::new(),
    )
    .expect("owned direct child");
    assert_eq!(execution.query.association, association);
    assert_eq!(execution.query.status, SolverStatus::Unsat);
    assert!(execution.process.started);
    assert!(matches!(
        execution.process.outcome,
        ProcessOutcome::Completed {
            success: true,
            exit_code: Some(0)
        }
    ));
    assert_eq!(
        execution.process.stdout.as_deref(),
        Some(b"unsat\n".as_slice())
    );
    assert_eq!(execution.process.stderr.as_deref(), Some(b"".as_slice()));
}

#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]
#[test]
fn excess_input_and_conflicting_status_never_become_unsat_evidence() {
    let blocked = run_solver_query(
        base(),
        "/usr/bin/cat".into(),
        vec![],
        process_limits(1),
        Cancel::new(),
    )
    .expect("input preflight");
    assert!(!blocked.process.started);
    assert_eq!(
        blocked.query.status,
        SolverStatus::Stopped(ReachedLimit::InputBytes)
    );
    assert!(matches!(
        blocked.process.outcome,
        ProcessOutcome::InputOverLimit
    ));

    let conflicting = run_solver_query(
        QueryAssociation {
            query_bytes: b"unsat\nsat\n".to_vec(),
            ..base()
        },
        "/usr/bin/cat".into(),
        vec![],
        process_limits(1024),
        Cancel::new(),
    )
    .expect("completed direct child");
    assert_eq!(conflicting.query.status, SolverStatus::MalformedResponse);
    assert_eq!(
        conflicting.process.stdout.as_deref(),
        Some(b"unsat\nsat\n".as_slice())
    );

    let missing = run_solver_query(
        base(),
        "/nonexistent/cg-ir974-solver".into(),
        vec![],
        process_limits(1024),
        Cancel::new(),
    )
    .expect("missing executable is a typed result");
    assert!(!missing.process.started);
    assert_eq!(missing.query.status, SolverStatus::MissingTool);

    let absent = run_solver_query(
        QueryAssociation {
            query_bytes: Vec::new(),
            ..base()
        },
        "/usr/bin/true".into(),
        vec![],
        process_limits(1024),
        Cancel::new(),
    )
    .expect("successful child with no response");
    assert_eq!(absent.query.status, SolverStatus::MalformedResponse);

    let failed = run_solver_query(
        QueryAssociation {
            query_bytes: Vec::new(),
            ..base()
        },
        "/usr/bin/false".into(),
        vec![],
        process_limits(1024),
        Cancel::new(),
    )
    .expect("nonzero exit is a typed result");
    assert_eq!(failed.query.status, SolverStatus::ProcessFailed);
    assert!(matches!(
        failed.process.outcome,
        ProcessOutcome::Completed {
            success: false,
            exit_code: Some(1)
        }
    ));
}
