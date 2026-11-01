// SPDX-License-Identifier: AGPL-3.0-or-later
//! Typed report conversion; real backend qualification lives in the integration lane.
use super::*;
use crate::kani::{abi::KaniSolver, run::{execute::KaniExecutionEvidence,memory::{MemoryObservation,MemoryMechanism}},output::report::{KaniCheckResult,KaniCheckLocation},classify::KaniInconclusiveReason};
use ix_trace_rs::trace;
#[path = "../../../../tests/it/constrained_method/support.rs"]
mod support;
fn evidence(method: &ConstrainedMethodIdentity) -> ConstrainedBackendEvidence {
    let check = |id, class, line, status| KaniCheckResult { id,class,location:KaniCheckLocation {file:"src/lib.rs".into(),line:Some(line)},status};
    ConstrainedBackendEvidence {
        method:method.clone(), safety_assertion_line:method.safety_assertion_line,ordinary_cover_line:method.ordinary_cover_line,boundary_cover_line:method.boundary_cover_line,
        backend: KaniExecutionEvidence {
            ceilings:method.ceilings,memory:MemoryObservation {mechanism:MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,peak_resident_bytes:None},symbolic_arguments:vec![],kind:None,harness_path:"constrained::search".into(),launcher_path:"unit-conversion".into(),arguments:vec![],unwind:method.unwind,solver:KaniSolver::Cadical,exit_code:Some(0),outcome:KaniRunOutcome::Verified,success_checks:1,checks:vec![
                check(1,KaniCheckClass::from("assertion".to_owned()),method.safety_assertion_line,KaniCheckStatus::Success),
                check(2,KaniCheckClass::Cover,method.ordinary_cover_line,KaniCheckStatus::Satisfied),
                check(3,KaniCheckClass::Cover,method.boundary_cover_line.unwrap(),KaniCheckStatus::Unsatisfiable),
            ],batch:None,diagnostics:None,failure_cause:None,
        }
    }
}
#[trace("TC-062", "FR-048-AC-6", "FR-048-AC-7", "FR-048-AC-8")]
#[test]
fn conversion_preserves_original_causes_and_rejects_malformed_or_cross_method_evidence() {
    let f=support::fixture(&support::document(),"constrained-conversion","self.n <= 3",&["self.n < 2"]);
    let initial=support::initial(0);let bindings=support::bindings(&f);let request=support::request(&f,&initial,&bindings);let method=support::emit_method(&request).unwrap();
    let healthy=qsl_replay::CallOutcome::Completed(qsl_replay::CallValue::Boolean(true));
    let observation=ConstrainedOwnerObservation {method:method.identity(),safety:Ok(&healthy),constraint:Some(Ok(&healthy)),playback:None};
    let mut backend=evidence(method.identity());
    assert_eq!(classify_constrained_method(&backend,&observation).unwrap().status(),ConstrainedMethodStatus::BoundedSuccess);
    backend.backend.checks.reverse();
    assert_eq!(classify_constrained_method(&backend,&observation).unwrap().status(),ConstrainedMethodStatus::BoundedSuccess);
    for missing in 0..3 {
        let mut malformed=evidence(method.identity());malformed.backend.checks.remove(missing);
        let result=classify_constrained_method(&malformed,&observation).unwrap();
        assert_eq!(result.status(),ConstrainedMethodStatus::Inconclusive);
        assert_eq!(result.boundary(),Some(ConstraintBoundary::Unknown));
    }
    for reason in [KaniInconclusiveReason::TimedOut,KaniInconclusiveReason::MemoryExhausted,KaniInconclusiveReason::UnwindBoundExhausted,KaniInconclusiveReason::MissingCoverSummary] {
        backend.backend.outcome=KaniRunOutcome::Inconclusive {reason};
        let result=classify_constrained_method(&backend,&observation).unwrap();
        assert_eq!(result.status(),ConstrainedMethodStatus::Inconclusive);
        assert_eq!(result.evidence().nonproof_reason(),Some(reason));
        assert_eq!(result.boundary(),Some(ConstraintBoundary::Unknown));
    }

    let partial = support::fixture(&support::document(),"constrained-partial","self.n <= 3",&["self.n mod self.aux = 0"]);
    let mut zero = request.evaluator_limits;
    zero.work_units = 0;
    let incomplete = support::owner_evaluation(&f,"EvalSafety",0,false,0,zero);
    assert!(matches!(incomplete,qsl_replay::CallOutcome::Incomplete(_)));
    let undefined = support::owner_evaluation(&partial,"EvalConstraint0",0,false,0,request.evaluator_limits);
    assert!(matches!(undefined,qsl_replay::CallOutcome::Undefined { .. }));
    let refusal = qsl_replay::CallOutcome::Refused(qsl_replay::CallRefusal::Family {
        code: quire_semantic_value::catalog::InternalFault::new("unit-refusal","unit-payload").catalog_code(), location:None,
    });
    backend.backend.outcome=KaniRunOutcome::Falsified {counterexample:"unit conversion report".into()};
    for (original, status) in [(&undefined,ConstrainedMethodStatus::EvaluationViolation),(&refusal,ConstrainedMethodStatus::Refused),(&incomplete,ConstrainedMethodStatus::Incomplete)] {
        let stopped=ConstrainedOwnerObservation {safety:Ok(&healthy),constraint:Some(Ok(original)),playback:Some("unit conversion report"),..observation};
        let result=classify_constrained_method(&backend,&stopped).unwrap();
        assert_eq!(result.status(),status);
        assert!(matches!(result.owner().constraint,Some(Ok(retained)) if std::ptr::eq(retained,original)));
    }
    let fault=quire_eval::CallFailure::Fault(quire_semantic_value::catalog::InternalFault::new("owner-stage","owner-invariant"));
    let failed=ConstrainedOwnerObservation {safety:Err(&fault),playback:backend.playback(),..observation};
    let result=classify_constrained_method(&backend,&failed).unwrap();
    assert_eq!(result.status(),ConstrainedMethodStatus::RuntimeInvariant);
    assert!(matches!(result.owner().safety, Err(original) if std::ptr::eq(original, &fault)));
    let code=result.fault_catalog_code().unwrap();assert_eq!(code.code(),"runtime_invariant");assert_eq!(code.cause(),"established-invariant-broken");
    let mut other=method.identity().clone();other.maximum_steps+=1;
    let changed=ConstrainedOwnerObservation {method:&other,..observation};
    assert!(matches!(classify_constrained_method(&backend,&changed),Err(ConstrainedBindingRefusal::Method)));
    let changed=ConstrainedOwnerObservation {playback:Some("substituted"),..observation};
    assert!(matches!(classify_constrained_method(&backend,&changed),Err(ConstrainedBindingRefusal::Playback)));
}
