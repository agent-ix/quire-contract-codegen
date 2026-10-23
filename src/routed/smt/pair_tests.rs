// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    num::{NonZeroU64, NonZeroUsize},
    time::Duration,
};

use ix_trace_rs::trace;
use quire_exact::Cancel;

use super::{
    classify_pair, run_solver_query, PairEvidence, QueryAssociation, QuerySelection, ReachedLimit,
    SolverExecution, SolverStatus,
};
use crate::kani::run::process::{ProcessLimits, ProcessOutcome};

type Selection = QuerySelection<&'static str, &'static str, &'static str, &'static str>;
type Association = QueryAssociation<&'static str, &'static str, &'static str, &'static str>;
type Execution = SolverExecution<&'static str, &'static str, &'static str, &'static str>;

fn selection(kind: &'static str, query_bytes: &[u8]) -> Selection {
    QuerySelection {
        product: "product-a",
        item: "item-a",
        instance: "instance-v",
        depth: NonZeroU64::MIN,
        kind,
        query_bytes: query_bytes.to_vec(),
    }
}

fn limits(input_bytes: usize) -> ProcessLimits {
    ProcessLimits {
        input_bytes: NonZeroUsize::new(input_bytes).expect("nonzero"),
        wall_clock: Duration::from_secs(5),
        memory_bytes: NonZeroU64::new(128 * 1024 * 1024).expect("nonzero"),
        output_bytes: NonZeroUsize::new(1024).expect("nonzero"),
    }
}

fn run(selected: Selection) -> Execution {
    run_solver_query(
        selected,
        "/usr/bin/cat".into(),
        vec![],
        limits(1024),
        Cancel::new(),
    )
    .expect("bounded direct child")
}

fn association(execution: &Execution) -> &Association {
    &execution.query().association
}

// Full AC-1 needs QSL FR-315's encoder; these are CG runner checks only.
#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]
#[test]
fn two_real_unsat_processes_need_distinct_exact_associations() {
    let base = run(selection("base", b"unsat\n"));
    let step = run(selection("step", b"unsat\n"));
    assert!(base.process().started && step.process().started);
    assert_ne!(association(&base).attempt(), association(&step).attempt());
    assert_eq!(
        classify_pair(
            association(&base),
            association(&step),
            Some(&base),
            Some(&step)
        ),
        PairEvidence::InductionEligible
    );
    assert_eq!(
        classify_pair(
            association(&base),
            association(&step),
            Some(&step),
            Some(&base)
        ),
        PairEvidence::AssociationMismatch
    );
    let stale = run(selection("base", b"unsat\n"));
    assert_ne!(association(&base).attempt(), association(&stale).attempt());
    assert_eq!(
        classify_pair(
            association(&base),
            association(&step),
            Some(&stale),
            Some(&step)
        ),
        PairEvidence::AssociationMismatch
    );
    for changed in [
        Selection {
            product: "other",
            ..association(&base).selection.clone()
        },
        Selection {
            item: "other",
            ..association(&base).selection.clone()
        },
        Selection {
            instance: "other",
            ..association(&base).selection.clone()
        },
        Selection {
            depth: NonZeroU64::new(2).expect("nonzero"),
            ..association(&base).selection.clone()
        },
        Selection {
            kind: "step",
            ..association(&base).selection.clone()
        },
        Selection {
            query_bytes: b"unsat \n".to_vec(),
            ..association(&base).selection.clone()
        },
    ] {
        let expected = Association {
            selection: changed,
            ..association(&base).clone()
        };
        assert_eq!(
            classify_pair(&expected, association(&step), Some(&base), Some(&step)),
            PairEvidence::AssociationMismatch
        );
    }
}

#[trace("TC-059", "FR-045-AC-2")]
#[test]
fn responses_remain_nonterminal_evidence() {
    let base_sat = run(selection("base", b"sat\n"));
    let base_unsat = run(selection("base", b"unsat\n"));
    let base_unknown = run(selection("base", b"unknown\n"));
    let step_unsat = run(selection("step", b"unsat\n"));
    let step_sat = run(selection("step", b"sat\n"));
    let step_unknown = run(selection("step", b"unknown\n"));
    let classify = |base: &Execution, step: &Execution| {
        classify_pair(association(base), association(step), Some(base), Some(step))
    };
    assert_eq!(
        classify(&base_sat, &step_unsat),
        PairEvidence::BaseSatCandidate
    );
    assert_eq!(
        classify(&base_unsat, &step_sat),
        PairEvidence::StepSatNotClosed
    );
    assert_eq!(
        classify(&base_unknown, &step_unsat),
        PairEvidence::NonProof(SolverStatus::Unknown)
    );
    assert_eq!(
        classify(&base_sat, &step_unknown),
        PairEvidence::NonProof(SolverStatus::Unknown)
    );
    assert_eq!(
        classify_pair(
            association(&base_unsat),
            association(&step_unsat),
            None,
            Some(&step_unsat)
        ),
        PairEvidence::MissingResponse
    );
    let stopped = run_solver_query(
        selection("base", b"unsat\n"),
        "/usr/bin/cat".into(),
        vec![],
        limits(1),
        Cancel::new(),
    )
    .expect("typed input refusal");
    assert_eq!(
        classify_pair(
            association(&stopped),
            association(&step_unsat),
            Some(&stopped),
            Some(&step_unsat)
        ),
        PairEvidence::NonProof(SolverStatus::Stopped(ReachedLimit::InputBytes))
    );
}

#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]
#[test]
fn completed_run_retains_exact_selection_limits_and_raw_output() {
    let selected = selection("base", b"unsat\n");
    let execution = run(selected.clone());
    assert_eq!(execution.query().association.selection, selected);
    assert_eq!(execution.query().status, SolverStatus::Unsat);
    assert!(execution.process().started);
    assert!(matches!(
        execution.process().outcome,
        ProcessOutcome::Completed {
            success: true,
            exit_code: Some(0)
        }
    ));
    assert_eq!(
        execution.process().stdout.as_deref(),
        Some(b"unsat\n".as_slice())
    );
    assert_eq!(execution.process().stderr.as_deref(), Some(b"".as_slice()));
    assert_eq!(
        execution.process().executable.to_str(),
        Some("/usr/bin/cat")
    );
    assert_eq!(execution.process().limits, limits(1024));
}

#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]
#[test]
fn excess_input_and_malformed_output_never_become_unsat() {
    let blocked = run_solver_query(
        selection("base", &vec![b'x'; 1024 * 1024]),
        "/usr/bin/cat".into(),
        vec![],
        limits(1),
        Cancel::new(),
    )
    .expect("input preflight");
    assert!(!blocked.process().started);
    assert_eq!(
        blocked.query().status,
        SolverStatus::Stopped(ReachedLimit::InputBytes)
    );
    assert!(matches!(
        blocked.process().outcome,
        ProcessOutcome::InputOverLimit
    ));
    for output in [
        b"unsat\nsat\n".as_slice(),
        b"unsat\njunk\n",
        b"unsat\n(error \"bad\")\n",
    ] {
        let malformed = run(selection("base", output));
        assert_eq!(malformed.query().status, SolverStatus::MalformedResponse);
        assert_eq!(malformed.process().stdout.as_deref(), Some(output));
    }
    let model_bytes = b"unsat\n(model (define-fun x () String \"(\"))\n";
    let model = run(selection("base", model_bytes));
    assert_eq!(model.query().status, SolverStatus::Unsat);
    assert_eq!(
        model.process().stdout.as_deref(),
        Some(model_bytes.as_slice())
    );
    let missing = run_solver_query(
        selection("base", b"unsat\n"),
        "/nonexistent/cg-ir974-solver".into(),
        vec![],
        limits(1024),
        Cancel::new(),
    )
    .expect("missing executable is typed");
    assert!(!missing.process().started);
    assert_eq!(missing.query().status, SolverStatus::MissingTool);
    let absent = run_solver_query(
        selection("base", b""),
        "/usr/bin/true".into(),
        vec![],
        limits(1024),
        Cancel::new(),
    )
    .expect("successful child with no response");
    assert_eq!(absent.query().status, SolverStatus::MalformedResponse);
    let failed = run_solver_query(
        selection("base", b""),
        "/usr/bin/false".into(),
        vec![],
        limits(1024),
        Cancel::new(),
    )
    .expect("nonzero exit is typed");
    assert_eq!(failed.query().status, SolverStatus::ProcessFailed);
    assert!(matches!(
        failed.process().outcome,
        ProcessOutcome::Completed {
            success: false,
            exit_code: Some(1)
        }
    ));
}
