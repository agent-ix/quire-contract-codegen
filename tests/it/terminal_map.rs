//! FR-029 and FR-030: the total maps from a Kani run outcome (FR-029) and from a Contract IR
//! outcome (FR-030), each paired with its replay settlement, to QSL's terminal value.
//!
//! The replay failures are built from the public types of this crate and of `qsl-replay`; a
//! repeated dependency identity is the refusal `ReplayPackage::new` returns for a real lock.
//! The IR outcomes are built with IR's own constructors and the `Std001Code` that `qsl-replay`
//! re-exports, so the `Declined` code arm is built through `qsl-replay` alone.

use ix_trace_rs::trace;
use qsl_replay::{
    std001_code, AdmissionFailure, CallSiteRefusal, Certification, ClauseName, Code, DeclineCode,
    DependencyInput, DependencyInputRefusal, DigestDomain, DigestRecord, DisagreementCause,
    FieldName, FrameCounterexample, Identifier, IncompleteCause, InconclusiveCause,
    MalformedTranscript, OperationName, PopulationName, ProofBasis, ProofRefusalCause,
    QualifiedName, ReplayLimits, ReplayRefusal, ReplayTargetMembers, ReportedInconclusiveCause,
    ScalarLimits, SourceIdentity, StateClauseCounterexample, Std001Code, SuppliedLibrary,
    TerminalValue, UnavailabilityCause, Verdict, Witness, WitnessEnvelope, WitnessFailure,
    WitnessPacket,
};
use quire_contract_codegen::{
    ir_outcome_terminal_value, run_terminal_value, DecodeFailure, DependencyLock,
    DependencyLockError, DocumentError, EvidenceFailureCause, FrameReplayError,
    KaniExecutionEvidence, KaniFailureCause, KaniInconclusiveReason, KaniRunOutcome, KaniSolver,
    LockedSource, MemoryMechanism, MemoryObservation, ModelError, ObligationIdentityError,
    OperationDeclaration, PreStateFault, ProofCeilings, ReplayInputs, ReplayPackage,
    ReplayPackageError, ReplaySettlement, ReplayVerdict, ScopeMember, SpineReplayError,
    StateClauseReplay, StateClauseReplayError, StateFrameRefusal, TerminalPairError,
};
use quire_contract_ir::kani::{KaniCancellationSource, KaniOutcome, KaniOutcomeKind};
use quire_exact::CancelCause;
use quire_semantic_value::catalog::{Category, InternalFault};

use crate::kani_obligations_state_frame::{emitted_fixture, native_twin::Twin};

/// Explicitly synthetic evidence for mapping tables, independent of launcher fixtures.
pub(crate) fn synthetic_evidence(
    outcome: KaniRunOutcome,
    success_checks: u32,
) -> KaniExecutionEvidence {
    KaniExecutionEvidence {
        ceilings: ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(1_073_741_824).unwrap(),
            wall_clock: std::time::Duration::from_secs(30),
        },
        memory: MemoryObservation {
            mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
            peak_resident_bytes: None,
        },
        symbolic_arguments: Vec::new(),
        kind: None,
        harness_path: "synthetic::mapping".into(),
        launcher_path: "synthetic".into(),
        arguments: Vec::new(),
        unwind: 4,
        solver: KaniSolver::Cadical,
        exit_code: None,
        outcome,
        success_checks,
        checks: Vec::new(),
        batch: None,
        diagnostics: None,
        failure_cause: None,
    }
}

/// Every inconclusive reason the classifier has today.
const REASONS: [KaniInconclusiveReason; 7] = [
    KaniInconclusiveReason::FailedWithoutCounterexample,
    KaniInconclusiveReason::NoVerdict,
    KaniInconclusiveReason::MissingCoverSummary,
    KaniInconclusiveReason::VacuousProof,
    KaniInconclusiveReason::UnwindBoundExhausted,
    KaniInconclusiveReason::TimedOut,
    KaniInconclusiveReason::MemoryExhausted,
];

fn falsified() -> KaniRunOutcome {
    KaniRunOutcome::Falsified {
        counterexample: "#[test] fn kani_concrete_playback() {}".to_owned(),
    }
}

fn inconclusive(reason: KaniInconclusiveReason) -> KaniRunOutcome {
    KaniRunOutcome::Inconclusive { reason }
}

/// The outcomes that take no settlement, one per reason and one per other kind.
fn settlement_free_outcomes() -> Vec<KaniRunOutcome> {
    let mut outcomes = vec![
        KaniRunOutcome::Verified,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1,
        },
    ];
    outcomes.extend(REASONS.map(inconclusive));
    outcomes
}

/// Both direct run-map cancellation sources preserve their exact cause and refuse replay pairs.
#[trace("FR-029-AC-15", "FR-029-AC-29", "FR-040-AC-7")]
#[test]
fn direct_cancelled_run_maps_exact_source_and_rejects_settlement() {
    for (source, exact) in [
        (KaniCancellationSource::Requested, CancelCause::Requested),
        (KaniCancellationSource::Deadline, CancelCause::Deadline),
    ] {
        let outcome = KaniRunOutcome::Cancelled { source };
        assert_eq!(
            run_terminal_value(&synthetic_evidence(outcome.clone(), 0), None),
            Ok(TerminalValue::Incomplete(IncompleteCause::Cancelled {
                source: exact
            }))
        );
        assert_eq!(
            run_terminal_value(
                &synthetic_evidence(outcome.clone(), 0),
                Some(ReplaySettlement::Reproduced)
            ),
            Err(TerminalPairError::UnexpectedSettlement)
        );
    }
}

fn map_run(outcome: &KaniRunOutcome, settlement: Option<ReplaySettlement<'_>>) -> TerminalValue {
    run_terminal_value(&synthetic_evidence(outcome.clone(), 3), settlement)
        .expect("a well-paired outcome maps")
}

fn map_settled<'a>(settlement: impl Into<ReplaySettlement<'a>>) -> TerminalValue {
    map_run(&falsified(), Some(settlement.into()))
}

fn replayed(code: Code) -> TerminalValue {
    TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(code))
}

fn library(identity: &str, owner: &str) -> SuppliedLibrary {
    SuppliedLibrary {
        identity: identity.to_owned(),
        source: SourceIdentity::new("agent-ix", owner, "git", "r1"),
        path: owner.to_owned(),
        bytes: b"a dependency source".to_vec(),
    }
}

/// QSL's own refusal of a dependency input with a repeated identity.
fn duplicate_identity() -> DependencyInputRefusal {
    DependencyInput::new([library("test/units", "one"), library("test/units", "two")])
        .map(|_| ())
        .expect_err("a repeated identity is refused")
}

/// QSL's own refusal of a library under the empty identity.
fn empty_identity() -> DependencyInputRefusal {
    DependencyInput::new([library("", "one")])
        .map(|_| ())
        .expect_err("an empty identity is refused")
}

fn identifier(name: &str) -> Identifier {
    Identifier::new(name).expect("an identifier")
}

/// QSL's own refusal of two libraries sharing one source owner.
fn shared_owner() -> DependencyInputRefusal {
    DependencyInput::new([library("test/a", "one"), library("test/b", "one")])
        .map(|_| ())
        .expect_err("a shared owner is refused")
}

fn malformed() -> MalformedTranscript {
    Witness::parse("not a transcript".to_owned()).expect_err("the transcript is malformed")
}

fn non_fault_refusal() -> ReplayRefusal {
    ReplayRefusal::SourceCount(2)
}

/// `verified` with three SUCCESS checks is a certified `Proved` with three checks. A
/// `falsified` run with a reproduced replay is `Refuted`.
#[trace("FR-029-AC-1", "TC-040")]
#[test]
fn tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes() {
    assert_eq!(
        map_run(&KaniRunOutcome::Verified, None),
        TerminalValue::Proved {
            basis: ProofBasis::Checks { success_checks: 3 },
            certification: Certification::Certified,
        }
    );
    assert_eq!(
        map_run(&falsified(), Some(ReplaySettlement::Reproduced)),
        TerminalValue::Refuted
    );
}

/// The vacuous-proof reason and `cover-unsatisfied` each map to a certified `Proved` with
/// zero checks, which QSL reads as category `inconclusive` with the vacuity cause.
#[trace("FR-029-AC-2", "TC-040")]
#[test]
fn tc_040_a_vacuous_proof_and_an_unsatisfied_cover_are_proved_with_zero_checks() {
    for outcome in [
        inconclusive(KaniInconclusiveReason::VacuousProof),
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 2,
        },
    ] {
        let value = map_run(&outcome, None);
        assert_eq!(
            value,
            TerminalValue::Proved {
                basis: ProofBasis::Checks { success_checks: 0 },
                certification: Certification::Certified,
            }
        );
        assert_eq!(value.category(), Category::Inconclusive);
        assert_eq!(
            value.vacuous_proof_cause(),
            Some(ReportedInconclusiveCause::KaniVacuousProof)
        );
    }
}

#[trace("FR-029-AC-3")]
#[test]
fn observed_memory_exhaustion_maps_to_incomplete_resource_exhausted() {
    assert_eq!(
        map_run(&inconclusive(KaniInconclusiveReason::MemoryExhausted), None),
        TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
    );
}

/// The timed-out reason maps to `Incomplete(TimedOut)` and the exhausted-unwind-bound reason to
/// `Incomplete(ResourceExhausted)`.
#[trace("TC-040")]
#[test]
fn tc_040_a_timeout_and_an_exhausted_unwind_bound_are_incomplete() {
    assert_eq!(
        map_run(&inconclusive(KaniInconclusiveReason::TimedOut), None),
        TerminalValue::Incomplete(IncompleteCause::TimedOut)
    );
    assert_eq!(
        map_run(
            &inconclusive(KaniInconclusiveReason::UnwindBoundExhausted),
            None
        ),
        TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
    );
}

/// The no-verdict reason maps to `Failed`.
#[trace("FR-029-AC-4", "TC-040")]
#[test]
fn tc_040_a_run_with_no_verdict_is_failed() {
    assert_eq!(
        map_run(&inconclusive(KaniInconclusiveReason::NoVerdict), None),
        TerminalValue::Failed
    );
}

/// The failure-without-counterexample and missing-cover-summary reasons each map to `Failed`.
#[trace("FR-029-AC-5", "TC-040")]
#[test]
fn tc_040_a_missing_counterexample_or_cover_summary_is_failed() {
    for reason in [
        KaniInconclusiveReason::FailedWithoutCounterexample,
        KaniInconclusiveReason::MissingCoverSummary,
    ] {
        assert_eq!(map_run(&inconclusive(reason), None), TerminalValue::Failed);
    }
}

/// No outcome, with any reason and any settlement, maps to `Tested`.
#[trace("FR-029-AC-6", "TC-040")]
#[test]
fn tc_040_no_outcome_maps_to_tested() {
    let mut values: Vec<TerminalValue> = settlement_free_outcomes()
        .iter()
        .map(|outcome| map_run(outcome, None))
        .collect();
    with_settlements(|settlement| values.push(map_settled(settlement)));
    assert!(values.len() > REASONS.len());
    assert!(
        values.iter().all(|value| *value != TerminalValue::Tested),
        "{values:?}"
    );
}

/// A pair outside the domain is a typed refusal, not a value: a falsified run needs its settlement
/// and no other outcome has one.
#[trace("FR-029-AC-15", "TC-040")]
#[test]
fn tc_040_a_settlement_accompanies_a_falsified_outcome_only() {
    assert_eq!(
        run_terminal_value(&synthetic_evidence(falsified(), 0), None),
        Err(TerminalPairError::MissingSettlement)
    );
    for outcome in settlement_free_outcomes() {
        assert_eq!(
            run_terminal_value(
                &synthetic_evidence(outcome.clone(), 3),
                Some(ReplaySettlement::Reproduced)
            ),
            Err(TerminalPairError::UnexpectedSettlement),
            "{outcome:?}"
        );
    }
}

/// A replay disagreement of each cause maps to `Inconclusive(ReplayParity)` carrying that cause.
#[trace("FR-029-AC-8", "TC-040")]
#[test]
fn tc_040_a_replay_disagreement_carries_its_cause() {
    let (violation, success) = (
        Verdict::from_category(Category::Violation),
        Verdict::from_category(Category::Success),
    );
    for cause in [
        DisagreementCause::Verdicts {
            proved: violation,
            replayed: success,
        },
        DisagreementCause::NoValue {
            proved: violation,
            replayed: violation,
        },
        DisagreementCause::Witness {
            proved: violation,
            replayed: violation,
            given: None,
            derived: None,
            failure: WitnessFailure::Mismatch,
        },
    ] {
        let verdict = ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement: qsl_replay::WitnessSettlement::Inconclusive,
            category: Category::Violation,
            disagreement: Some(cause.clone()),
        });
        assert_eq!(
            map_run(&falsified(), Some((&verdict).into())),
            TerminalValue::Inconclusive(InconclusiveCause::ReplayParity(cause))
        );
    }
}

/// A non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying its own code, bare
/// and as the cause of a spine replay failure.
#[trace("FR-029-AC-9", "TC-040")]
#[test]
fn tc_040_a_replay_refusal_carries_its_code() {
    let refusal = non_fault_refusal();
    let expected = replayed(refusal.code());
    assert_eq!(
        map_run(&falsified(), Some(ReplaySettlement::Refused(&refusal))),
        expected
    );
    let spine = SpineReplayError::Refused(Box::new(non_fault_refusal()));
    assert_eq!(map_settled(&spine), expected);
    let frame = FrameReplayError::Refused {
        refusal: Box::new(non_fault_refusal()),
        unranged: Vec::new(),
    };
    assert_eq!(map_settled(&frame), expected);
}

/// Every failure this repository raises that carries no QSL catalog code maps to `Failed`, so no
/// `ReplayRefused` value carries a code no QSL refusal value supplied.
#[trace("FR-029-AC-11", "TC-040")]
#[test]
fn tc_040_each_failure_this_repository_raises_is_failed() {
    for_each_cg_failure(|settlement| {
        assert_eq!(
            map_run(&falsified(), Some(settlement)),
            TerminalValue::Failed
        );
    });
}

/// Hands `check` the settlement of each failure this repository raises that carries no QSL
/// catalog code.
fn for_each_cg_failure(check: impl Fn(ReplaySettlement<'_>)) {
    let spine = [
        SpineReplayError::UnboundArgument {
            argument: "x".to_owned(),
        },
        SpineReplayError::UnsupportedWitnessValue {
            argument: "x".to_owned(),
        },
        SpineReplayError::FieldDelimiter,
        SpineReplayError::Transcript(malformed()),
        SpineReplayError::WrongArm,
        SpineReplayError::Identity(ObligationIdentityError::UnboundArgument {
            argument: "x".to_owned(),
        }),
    ];
    for error in &spine {
        check(error.into());
    }

    let empty_name = QualifiedName::new(Vec::new()).expect_err("an empty qualified name");
    let envelope = WitnessEnvelope::reconstruct(
        WitnessPacket::<FrameCounterexample> {
            obligation_identity: None,
            occurrence_key: None,
            clause_node: None,
            replay_target: ReplayTargetMembers::default(),
            package_id: None,
            source_digests: None,
            profile_selections: None,
            run_limits: None,
            declared_domains: None,
            backend: None,
            trace_position: None,
            source: None,
            family_payload: None,
        },
        ReplayLimits::default(),
    )
    .map(|_| ())
    .expect_err("an envelope with no members is refused");
    let frame = [
        FrameReplayError::Transcript(malformed()),
        FrameReplayError::Envelope(envelope),
        FrameReplayError::Name(empty_name),
        FrameReplayError::NotAFrame,
        FrameReplayError::FieldSetMismatch {
            state_fields: vec!["a".to_owned()],
            granted: Vec::new(),
            checked: Vec::new(),
        },
        FrameReplayError::Decode(DecodeFailure {
            code: "kani_witness_arity_mismatch".to_owned(),
            source_id: String::new(),
            context: String::new(),
        }),
        FrameReplayError::OutOfDomain {
            field: "a".to_owned(),
            value: 1,
        },
        FrameReplayError::PreState(PreStateFault::InvocationUnreadable),
        FrameReplayError::ScopeMismatch {
            member: ScopeMember::Frame,
            harness: "h".to_owned(),
            named: "n".to_owned(),
        },
        FrameReplayError::Identity(ObligationIdentityError::UnboundArgument {
            argument: "a".to_owned(),
        }),
    ];
    for error in &frame {
        check(error.into());
    }

    let invalid_function = ReplayPackageError::InvalidFunction {
        function: "1x".to_owned(),
    };
    check((&invalid_function).into());

    // A playback outside the harness proof bound, and a playback that does not type against the
    // persisted bindings.
    let domain = EvidenceFailureCause::Domain {
        argument: "x".to_owned(),
    };
    let decode = EvidenceFailureCause::Decode(DecodeFailure {
        code: "cg_witness_harness_identity_mismatch".to_owned(),
        source_id: String::new(),
        context: String::new(),
    });
    for cause in [domain, decode, reproduced_without_violation()] {
        let verdict = ReplayVerdict::EvidenceFailure(cause);
        check((&verdict).into());
    }
}

/// A replay reproduced in a category other than `violation`: no QSL result states it, and only
/// this crate's public `EvidenceFailureCause::Verdict` can.
fn reproduced_without_violation() -> EvidenceFailureCause {
    EvidenceFailureCause::Verdict {
        settlement: qsl_replay::WitnessSettlement::ReproducedWithEvaluatedWitness,
        category: Category::Success,
        disagreement: None,
    }
}

/// Hands `check` the settlement of each fault wrapper the run-outcome map's fault criterion names,
/// built from QSL's constructible `InternalFault`, and returns how many it handed over: the
/// call-site fault's own reading (`ReplaySettlement::Fault`), `ReplayRefusal::Fault` and
/// `ReplayRefusal::Admission(AdmissionFailure::Fault)` bare and as the cause of each replay
/// error that wraps a `ReplayRefusal`, and `CallSiteRefusal::Fault` bare and wrapped in
/// `ReplayPackageError`, `FrameReplayError` and `StateClauseReplayError`.
fn for_each_fault(check: impl Fn(ReplaySettlement<'_>)) -> usize {
    let fault = || InternalFault::new("replay", "broken");
    // `ReplayRefusal` is not `Clone`, so each wrapper gets a fresh one.
    let replay_faults: [fn() -> ReplayRefusal; 2] = [
        || ReplayRefusal::Fault(InternalFault::new("replay", "broken")),
        || {
            ReplayRefusal::Admission(AdmissionFailure::Fault(InternalFault::new(
                "admission",
                "broken",
            )))
        },
    ];
    let call_site = || CallSiteRefusal::Fault(fault());
    let mut exercised = 0;
    let mut check = |settlement: ReplaySettlement<'_>| {
        exercised += 1;
        check(settlement);
    };

    check(ReplaySettlement::Fault);
    for refusal in replay_faults {
        check(ReplaySettlement::Refused(&refusal()));
        check((&SpineReplayError::Refused(Box::new(refusal()))).into());
        check(
            (&FrameReplayError::Refused {
                refusal: Box::new(refusal()),
                unranged: Vec::new(),
            })
                .into(),
        );
        check((&StateClauseReplayError::Refused(Box::new(refusal()))).into());
    }
    let bare = call_site();
    check((&bare).into());
    check((&ReplayPackageError::CallSite(Box::new(call_site()))).into());
    check((&FrameReplayError::CallSite(Box::new(call_site()))).into());
    check((&StateClauseReplayError::CallSite(Box::new(call_site()))).into());
    exercised
}

/// The fault reading is `Failed` for every fault wrapper the criterion names, walked through the
/// whole error, and a fault is neither `Refuted` nor a refusal carrying a code.
#[trace("FR-029-AC-10", "TC-040")]
#[test]
fn tc_040_a_fault_in_any_replay_wrapper_is_failed() {
    let exercised = for_each_fault(|settlement| {
        assert_eq!(map_settled(settlement), TerminalValue::Failed);
    });
    assert_eq!(exercised, 1 + 2 * 4 + 4, "every named wrapper is exercised");
}

/// Across every replay settlement other than reproduced, a falsified run is not `Refuted`, and a
/// reproduced replay in a category other than `violation` is not read as a reproduction either.
#[trace("FR-029-AC-12", "TC-040")]
#[test]
fn tc_040_only_a_reproduced_replay_refutes() {
    let mut exercised = 0;
    with_settlements(|settlement| {
        exercised += 1;
        assert_ne!(map_settled(settlement), TerminalValue::Refuted);
    });
    assert_eq!(exercised, 5, "every reading but reproduced is exercised");
    let reproduced_without_violation =
        ReplayVerdict::EvidenceFailure(reproduced_without_violation());
    assert_ne!(
        map_settled(&reproduced_without_violation),
        TerminalValue::Refuted
    );
    assert_eq!(
        map_settled(&ReplayVerdict::Reproduced),
        TerminalValue::Refuted
    );
}

/// Hands `check` one settlement of each reading other than reproduced.
fn with_settlements(mut check: impl FnMut(ReplaySettlement<'_>)) {
    let cause = DisagreementCause::NoValue {
        proved: Verdict::from_category(Category::Violation),
        replayed: Verdict::from_category(Category::Success),
    };
    let refusal = non_fault_refusal();
    check(ReplaySettlement::Disagreement(&cause));
    check(ReplaySettlement::Refused(&refusal));
    check(ReplaySettlement::SetupRefused(Code::InvalidPackage));
    check(ReplaySettlement::Fault);
    check(ReplaySettlement::CgDefect);
}

/// A non-fault `CallSiteRefusal` and a `DependencyLockError::Input`, each bare and wrapped in
/// `ReplayPackageError` and `FrameReplayError`, map to `Inconclusive(ReplayRefused)` carrying the
/// code QSL's refusal supplies, and never to `Declined`.
#[trace("FR-029-AC-13", "TC-040")]
#[test]
fn tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined() {
    for_each_setup_refusal(|code, settlement| {
        assert_eq!(map_run(&falsified(), Some(settlement)), replayed(code));
    });
}

/// Hands `check` the code QSL's refusal supplies and the settlement of each non-fault
/// `CallSiteRefusal` and `DependencyLockError::Input`, bare and wrapped.
fn for_each_setup_refusal(check: impl Fn(Code, ReplaySettlement<'_>)) {
    let call_sites = || {
        let package = DigestRecord::mint(DigestDomain::PackageSemanticV2, [1; 32]);
        [
            CallSiteRefusal::Compile {
                code: Code::InvalidIdentifier,
                message: "does not compile".to_owned(),
            },
            CallSiteRefusal::ModelIntake {
                alias: "m".to_owned(),
                code: Code::InvalidPackage,
                message: "no package".to_owned(),
            },
            CallSiteRefusal::DependencyInput(shared_owner()),
            CallSiteRefusal::Import {
                code: Code::StaleDependency,
                message: "unresolved".to_owned(),
            },
            CallSiteRefusal::Dependency {
                path: Vec::new(),
                code: Code::InvalidIdentifier,
                message: "library".to_owned(),
            },
            CallSiteRefusal::UnknownFunction {
                selection: QualifiedName::new(vec![identifier("f")]).expect("a name"),
                package,
            },
            CallSiteRefusal::UnknownOperation {
                selection: OperationName {
                    model: identifier("m"),
                    object: identifier("o"),
                    operation: identifier("op"),
                },
                package,
            },
            CallSiteRefusal::UnknownClause {
                selection: ClauseName(identifier("c")),
                package,
            },
            CallSiteRefusal::UnknownField {
                selection: FieldName {
                    model: identifier("m"),
                    object: identifier("o"),
                    field: identifier("f"),
                },
                package,
            },
            CallSiteRefusal::UnknownPopulation {
                selection: PopulationName {
                    model: identifier("m"),
                    population: identifier("p"),
                },
                package,
            },
        ]
    };
    for refusal in call_sites() {
        let code = refusal.code();
        check(code, (&refusal).into());
        let package = ReplayPackageError::CallSite(Box::new(refusal));
        check(code, (&package).into());
    }
    for refusal in call_sites() {
        let code = refusal.code();
        let frame = FrameReplayError::CallSite(Box::new(refusal));
        check(code, (&frame).into());
    }

    // The empty identity is the refusal whose code is not `invalid_package`, so a conversion that
    // ignores the refusal's own code cannot pass.
    let empty = empty_identity();
    assert_eq!(empty.code(), Code::InvalidIdentifier);
    for refusal in [duplicate_identity(), shared_owner(), empty] {
        let code = refusal.code();
        let lock = DependencyLockError::Input(refusal.clone());
        check(code, (&lock).into());
        let package = ReplayPackageError::Dependencies(DependencyLockError::Input(refusal.clone()));
        check(code, (&package).into());
        let frame = FrameReplayError::Dependencies(DependencyLockError::Input(refusal));
        check(code, (&frame).into());
    }
}

/// A lock whose only defect is one library identity selected twice is refused by
/// `ReplayPackage::new` with QSL's `DuplicateIdentity`, and a falsified run with that refusal is
/// `Inconclusive(ReplayRefused)` carrying `invalid_package`.
#[trace("FR-029-AC-14", "TC-040")]
#[test]
fn tc_040_a_repeated_dependency_identity_is_replay_refused_with_invalid_package() {
    let refusal = repeated_identity_refusal();
    assert_eq!(
        map_settled(&refusal),
        replayed(Code::InvalidPackage),
        "{refusal}"
    );
}

/// The refusal `ReplayPackage::new` returns for a lock whose only defect is a repeated identity,
/// checked to be QSL's `DuplicateIdentity`.
fn repeated_identity_refusal() -> ReplayPackageError {
    let unlimited = ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    };
    let locked = |identity: &str| LockedSource {
        authority: "agent-ix".to_owned(),
        identity: identity.to_owned(),
        namespace: "git".to_owned(),
        revision: "r1".to_owned(),
        bytes: b"a source".to_vec(),
    };
    let dependency = |source: &str| DependencyLock {
        identity: "test/units".to_owned(),
        package_id: DigestRecord::mint(DigestDomain::PackageSemanticV2, [7; 32]),
        source: locked(source),
    };
    let inputs = ReplayInputs {
        decode_limits: crate::unbounded_decode_limits(),
        source: locked("unit"),
        dependencies: vec![dependency("lib-one"), dependency("lib-two")],
        accounting_limits: unlimited,
        stage_limits: Default::default(),
        declared_domains: Vec::new(),
        replay_limits: ReplayLimits::default(),
    };
    let refusal = ReplayPackage::new(inputs, "f").expect_err("the repeated identity is refused");
    let ReplayPackageError::Dependencies(DependencyLockError::Input(input)) = &refusal else {
        panic!("expected a refusal from QSL's dependency input, got {refusal}");
    };
    assert!(
        matches!(input, DependencyInputRefusal::DuplicateIdentity { .. }),
        "{input}"
    );
    refusal
}

/// The state-clause replay path settles as the other replay paths do (FR-029-AC-16): a
/// reproduced result is `Refuted` and an inconclusive one `Inconclusive(ReplayParity)` carrying
/// its cause, a refusal or a call-site or dependency refusal is read by its own code, and each
/// failure this repository raises that carries no QSL code is `Failed`.
#[trace("FR-029-AC-16", "TC-040")]
#[test]
fn tc_040_the_state_clause_replay_reads_as_fr029_ac16() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, "BalanceNeverDrops");
    let run = |pre, post| {
        StateClauseReplay::new(twin.state_clause_inputs(
            &fixture.package,
            &fixture.clause,
            "BalanceNeverDrops",
            pre,
            post,
        ))
        .expect("the replay is built")
        .replay()
        .expect("the replay settles")
    };
    // Both directions of the result: a debiting run reproduces, a crediting run is a disagreement
    // carrying QSL's own cause.
    assert_eq!(map_settled(&run((5, 0), (4, 0))), TerminalValue::Refuted);
    let cause = DisagreementCause::Verdicts {
        proved: Verdict::from_category(Category::Violation),
        replayed: Verdict::from_category(Category::Success),
    };
    assert_eq!(
        map_settled(&run((5, 0), (6, 0))),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayParity(cause))
    );

    // The failures this repository raises, none of which carries a QSL code.
    let empty_packet = || WitnessPacket::<StateClauseCounterexample> {
        obligation_identity: None,
        occurrence_key: None,
        clause_node: None,
        replay_target: ReplayTargetMembers::default(),
        package_id: None,
        source_digests: None,
        profile_selections: None,
        run_limits: None,
        declared_domains: None,
        backend: None,
        trace_position: None,
        source: None,
        family_payload: None,
    };
    let envelope = WitnessEnvelope::reconstruct(empty_packet(), ReplayLimits::default())
        .map(|_| ())
        .expect_err("an envelope with no members is refused");
    let defects = [
        StateClauseReplayError::Name(
            QualifiedName::new(Vec::new()).expect_err("an empty qualified name"),
        ),
        StateClauseReplayError::Transcript(malformed()),
        StateClauseReplayError::Envelope(envelope),
        StateClauseReplayError::Document(DocumentError::Model(ModelError::Unreadable)),
        StateClauseReplayError::Document(DocumentError::Clause {
            refusal: StateFrameRefusal::UnwindOutOfRange { unwind: 0 },
        }),
        StateClauseReplayError::MissingField {
            field: "balance".to_owned(),
        },
        StateClauseReplayError::UndeclaredField {
            field: "ghost".to_owned(),
        },
        StateClauseReplayError::DuplicateField {
            field: "balance".to_owned(),
        },
        StateClauseReplayError::OutOfDomain {
            field: "balance".to_owned(),
        },
        StateClauseReplayError::UnsupportedOperationShape {
            operation: OperationName {
                model: identifier("m"),
                object: identifier("o"),
                operation: identifier("op"),
            },
            declaration: OperationDeclaration::default(),
        },
    ];
    for defect in &defects {
        assert_eq!(map_settled(defect), TerminalValue::Failed, "{defect}");
    }

    // The QSL refusals, read as the frame path reads them: by their own code, never `Failed`,
    // `Incomplete` or `Declined`.
    let refusal = non_fault_refusal();
    let expected = replayed(refusal.code());
    assert_eq!(
        map_settled(&StateClauseReplayError::Refused(Box::new(refusal))),
        expected
    );
    let package = DigestRecord::mint(DigestDomain::PackageSemanticV2, [1; 32]);
    let unknown = CallSiteRefusal::UnknownClause {
        selection: ClauseName(identifier("c")),
        package,
    };
    let code = unknown.code();
    assert_eq!(
        map_settled(&StateClauseReplayError::CallSite(Box::new(unknown))),
        replayed(code)
    );
    let input = shared_owner();
    let code = input.code();
    assert_eq!(
        map_settled(&StateClauseReplayError::Dependencies(
            DependencyLockError::Input(input)
        )),
        replayed(code)
    );
}

// FR-030: the map from a Contract IR outcome.

/// An IR outcome of `kind` carrying `code`, built by IR's own constructors.
fn ir(kind: KaniOutcomeKind, code: Std001Code) -> KaniOutcome {
    match kind {
        KaniOutcomeKind::Proved => {
            KaniOutcome::proved(1, "source", "context").expect("one proved check")
        }
        KaniOutcomeKind::Counterexample => KaniOutcome::counterexample("source", "context"),
        other => KaniOutcome::non_success(other, code, "source", "context")
            .expect("a non-success kind builds"),
    }
}

/// Every kind IR's outcome has, each with a cause code that kind could carry.
fn ir_outcomes() -> Vec<KaniOutcome> {
    use KaniOutcomeKind as Kind;
    vec![
        ir(Kind::Proved, Std001Code::KANI_PROVED),
        ir(Kind::Counterexample, Std001Code::KANI_COUNTEREXAMPLE),
        ir(Kind::Refused, Std001Code::KANI_CAPABILITY_MISSING),
        ir(Kind::InvalidInput, Std001Code::KANI_IDENTITY_INVALID),
        ir(
            Kind::IncompleteInput,
            Std001Code::KANI_POPULATION_INCOMPLETE,
        ),
        ir(Kind::Unavailable, Std001Code::KANI_CHECKER_ABSENT),
        ir(Kind::TimedOut, Std001Code::KANI_BOUND_EXHAUSTED),
        ir(Kind::ResourceExhausted, Std001Code::KANI_BOUND_EXHAUSTED),
        ir(
            Kind::Cancelled {
                source: KaniCancellationSource::Requested,
            },
            Std001Code::KANI_BOUND_EXHAUSTED,
        ),
        ir(
            Kind::Cancelled {
                source: KaniCancellationSource::Deadline,
            },
            Std001Code::KANI_BOUND_EXHAUSTED,
        ),
        ir(Kind::Inconclusive, Std001Code::KANI_VACUOUS_PROOF),
    ]
}

fn counterexample() -> KaniOutcome {
    ir(
        KaniOutcomeKind::Counterexample,
        Std001Code::KANI_COUNTEREXAMPLE,
    )
}

/// An outcome that takes no settlement, using any check count retained in IR.
fn map_ir(outcome: &KaniOutcome) -> TerminalValue {
    ir_outcome_terminal_value(outcome, None).expect("a well-paired outcome maps")
}

fn map_ir_settled<'a>(settlement: impl Into<ReplaySettlement<'a>>) -> TerminalValue {
    ir_outcome_terminal_value(&counterexample(), Some(settlement.into()))
        .expect("a counterexample with a settlement maps")
}

/// Every pair the map's input can express maps to one value: nine kinds with no settlement and a
/// counterexample with each of the six readings.
#[trace("FR-030-AC-1", "TC-041")]
#[test]
fn tc_041_every_expressible_pair_maps_to_one_value() {
    let mut values = Vec::new();
    for outcome in ir_outcomes() {
        if outcome.kind() == KaniOutcomeKind::Counterexample {
            values.push(map_ir_settled(ReplaySettlement::Reproduced));
        } else {
            values.push(map_ir(&outcome));
        }
    }
    with_settlements(|settlement| values.push(map_ir_settled(settlement)));
    // Ten source-bearing outcomes with no settlement and six counterexample readings.
    assert_eq!(values.len(), 10 + 6);
}

/// A pair outside the map's input is a typed refusal, not a value: a counterexample needs its
/// settlement and no other kind has one.
#[trace("FR-030-AC-14", "TC-041")]
#[test]
fn tc_041_a_settlement_accompanies_a_counterexample_only() {
    for outcome in ir_outcomes() {
        let bare = ir_outcome_terminal_value(&outcome, None);
        let settled = ir_outcome_terminal_value(&outcome, Some(ReplaySettlement::Reproduced));
        if outcome.kind() == KaniOutcomeKind::Counterexample {
            assert_eq!(bare, Err(TerminalPairError::MissingSettlement));
            assert_eq!(settled, Ok(TerminalValue::Refuted));
        } else {
            assert_eq!(
                settled,
                Err(TerminalPairError::UnexpectedSettlement),
                "{:?}",
                outcome.kind()
            );
            assert!(bare.is_ok(), "{:?}", outcome.kind());
        }
    }
}

/// `Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with their own cause, and the
/// outcome's STD-001 code is carried in `DeclineCode::Std001`: not respelled as a QSL catalog code
/// and not refused when STD-001 does not register it.
#[trace("FR-030-AC-2", "TC-041")]
#[test]
fn tc_041_a_refusal_before_the_run_is_declined_with_its_kind_and_ir_code() {
    use KaniOutcomeKind as Kind;
    let unregistered = std001_code!("kani_corpus_identity_collision");
    assert!(!unregistered.is_registered());
    let cases = [
        (
            Kind::Refused,
            ProofRefusalCause::Refused,
            Std001Code::KANI_CAPABILITY_MISSING,
        ),
        (
            Kind::InvalidInput,
            ProofRefusalCause::InvalidInput,
            Std001Code::KANI_IDENTITY_INVALID,
        ),
        (
            Kind::IncompleteInput,
            ProofRefusalCause::IncompleteInput,
            Std001Code::KANI_POPULATION_INCOMPLETE,
        ),
        (Kind::Refused, ProofRefusalCause::Refused, unregistered),
    ];
    for (kind, cause, code) in cases {
        assert_eq!(
            map_ir(&ir(kind, code)),
            TerminalValue::Declined {
                cause,
                code: DeclineCode::Std001(code),
            }
        );
    }
}

/// `TimedOut`, `ResourceExhausted` and `Cancelled` map to `Incomplete` with their own cause.
#[trace("FR-030-AC-3", "TC-041")]
#[test]
fn tc_041_a_limit_or_a_cancellation_is_incomplete_with_its_cause() {
    use KaniOutcomeKind as Kind;
    for (kind, cause) in [
        (Kind::TimedOut, IncompleteCause::TimedOut),
        (Kind::ResourceExhausted, IncompleteCause::ResourceExhausted),
        (
            Kind::Cancelled {
                source: KaniCancellationSource::Requested,
            },
            IncompleteCause::Cancelled {
                source: CancelCause::Requested,
            },
        ),
        (
            Kind::Cancelled {
                source: KaniCancellationSource::Deadline,
            },
            IncompleteCause::Cancelled {
                source: CancelCause::Deadline,
            },
        ),
    ] {
        assert_eq!(
            map_ir(&ir(kind, Std001Code::KANI_BOUND_EXHAUSTED)),
            TerminalValue::Incomplete(cause)
        );
    }
}

/// `Proved` carries the transcript's SUCCESS check count, zero included, and a `Counterexample`
/// with a reproduced replay is `Refuted`.
#[trace("FR-030-AC-1", "FR-030-AC-4", "FR-030-AC-18", "TC-041")]
#[test]
fn tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes() {
    let proved = KaniOutcome::proved(3, "source", "context").expect("three proved checks");
    assert_eq!(proved.success_checks(), Some(3));
    assert_eq!(
        ir_outcome_terminal_value(&proved, None),
        Ok(TerminalValue::Proved {
            basis: ProofBasis::Checks { success_checks: 3 },
            certification: Certification::Certified,
        })
    );
    assert_eq!(
        map_ir_settled(ReplaySettlement::Reproduced),
        TerminalValue::Refuted
    );
    let zero = KaniOutcome::proved(0, "source", "context").unwrap_err();
    assert_eq!(*zero.requested_kind(), KaniOutcomeKind::Proved);
    assert_eq!(zero.requested_code(), Std001Code::KANI_PROVED);
    assert_eq!(zero.rejected_success_checks(), Some(0));
    #[cfg(target_pointer_width = "64")]
    {
        let above_qsl = u32::MAX as usize + 1;
        let proved = KaniOutcome::proved(above_qsl, "source", "context")
            .expect("IR accepts a positive count above QSL's range");
        assert_eq!(
            ir_outcome_terminal_value(&proved, None),
            Err(TerminalPairError::ProvedCountOutOfRange {
                success_checks: above_qsl,
            })
        );
    }
}

/// `Inconclusive` with `kani_vacuous_proof` is a certified `Proved` with zero checks whatever
/// count the caller passes; IR refuses any other `Inconclusive` cause.
#[trace("FR-030-AC-5", "FR-030-AC-18", "TC-041")]
#[test]
fn tc_041_a_vacuous_proof_is_proved_with_zero_checks_and_other_inconclusive_is_refused() {
    let outcome = |code| ir(KaniOutcomeKind::Inconclusive, code);
    assert_eq!(
        map_ir(&outcome(Std001Code::KANI_VACUOUS_PROOF)),
        TerminalValue::Proved {
            basis: ProofBasis::Checks { success_checks: 0 },
            certification: Certification::Certified,
        }
    );
    for code in [
        Std001Code::KANI_BOUND_INVALID,
        std001_code!("kani_no_interpretation"),
    ] {
        let error =
            KaniOutcome::non_success(KaniOutcomeKind::Inconclusive, code, "source", "context")
                .unwrap_err();
        assert_eq!(*error.requested_kind(), KaniOutcomeKind::Inconclusive);
        assert_eq!(error.requested_code(), code);
        assert_eq!(error.rejected_success_checks(), None);
    }
}

/// No pair maps to `Tested`.
#[trace("FR-030-AC-6", "TC-041")]
#[test]
fn tc_041_no_outcome_maps_to_tested() {
    let mut values: Vec<TerminalValue> = ir_outcomes()
        .iter()
        .filter(|outcome| outcome.kind() != KaniOutcomeKind::Counterexample)
        .map(map_ir)
        .collect();
    values.push(map_ir_settled(ReplaySettlement::Reproduced));
    with_settlements(|settlement| values.push(map_ir_settled(settlement)));
    assert_eq!(values.len(), 10 + 1 + 5);
    assert!(
        values.iter().all(|value| *value != TerminalValue::Tested),
        "{values:?}"
    );
}

/// The map is one `match` over the pair with no wildcard arm: inspection of the function's source.
/// The scrutinee is a two-tuple, every arm's first element names only `KaniOutcomeKind` variants
/// (no `_` and no binding), no arm's pattern or tuple element is a catch-all, and the variants the
/// arms name are all ten of the kind, so a kind IR adds fails to compile in the library.
#[trace("FR-030-AC-7", "TC-041")]
#[test]
fn tc_041_the_map_is_one_match_over_the_pair_with_no_wildcard_arm() {
    use syn::{visit::Visit, Expr, ExprMatch, Item, Pat};

    #[derive(Default)]
    struct Matches<'a>(Vec<&'a ExprMatch>);
    impl<'a> Visit<'a> for Matches<'a> {
        fn visit_expr_match(&mut self, node: &'a ExprMatch) {
            self.0.push(node);
            syn::visit::visit_expr_match(self, node);
        }
    }

    fn catch_all(pattern: &Pat) -> bool {
        match pattern {
            Pat::Wild(_) => true,
            // `None` parses as an identifier pattern; any other bare name binds everything.
            Pat::Ident(ident) => ident.subpat.is_none() && ident.ident != "None",
            Pat::Or(or) => or.cases.iter().any(catch_all),
            Pat::Tuple(tuple) => tuple.elems.iter().any(catch_all),
            Pat::Paren(paren) => catch_all(&paren.pat),
            _ => false,
        }
    }

    /// The `KaniOutcomeKind` variants a first-tuple-element pattern names, or `None` when it names
    /// anything else.
    fn kinds(pattern: &Pat, out: &mut Vec<String>) -> Option<()> {
        match pattern {
            Pat::Path(path) => {
                let segments: Vec<String> = path
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect();
                let [kind, variant] = segments.as_slice() else {
                    return None;
                };
                (kind == "Kind").then(|| out.push(variant.clone()))
            }
            Pat::Struct(structure) => {
                let names: Vec<_> = structure
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect();
                let [kind, variant] = names.as_slice() else {
                    return None;
                };
                (kind == "Kind").then(|| out.push(variant.clone()))
            }
            Pat::Or(or) => or.cases.iter().try_for_each(|case| kinds(case, out)),
            _ => None,
        }
    }

    let sources = crate::layout::source_files();
    let source = sources
        .get("kani/terminal.rs")
        .expect("the terminal map's source");
    let file = syn::parse_file(source).expect("terminal.rs parses");
    let map = file
        .items
        .iter()
        .find_map(|item| match item {
            Item::Fn(function) if function.sig.ident == "ir_outcome_terminal_value" => {
                Some(function)
            }
            _ => None,
        })
        .expect("the map is a top-level function");
    let mut found = Matches::default();
    found.visit_block(&map.block);
    let [pair_match] = found.0.as_slice() else {
        panic!("the map is exactly one match, found {}", found.0.len());
    };
    let Expr::Tuple(scrutinee) = &*pair_match.expr else {
        panic!("the match scrutinee is the pair");
    };
    assert_eq!(scrutinee.elems.len(), 2);
    let mut named = Vec::new();
    for arm in &pair_match.arms {
        assert!(!catch_all(&arm.pat), "a catch-all arm");
        let Pat::Tuple(pair) = &arm.pat else {
            panic!("every arm destructures the pair");
        };
        let first = pair.elems.first().expect("a pair has a first element");
        assert!(
            kinds(first, &mut named).is_some(),
            "an arm's kind is not a named `KaniOutcomeKind` variant"
        );
    }
    named.sort();
    named.dedup();
    let mut expected: Vec<String> = [
        "Proved",
        "Counterexample",
        "Refused",
        "InvalidInput",
        "IncompleteInput",
        "Unavailable",
        "TimedOut",
        "ResourceExhausted",
        "Cancelled",
        "Inconclusive",
    ]
    .map(str::to_owned)
    .into();
    expected.sort();
    assert_eq!(named, expected);
}

/// `Unavailable` with `kani_checker_absent` is `Unsupported(SolverAbsent)` and
/// `kani_backend_absent` is `Unsupported(BackendAbsent)`; IR refuses other causes.
#[trace("FR-030-AC-8", "TC-041")]
#[test]
fn tc_041_an_absent_solver_or_backend_is_unsupported_with_its_cause() {
    let outcome = |code| ir(KaniOutcomeKind::Unavailable, code);
    assert_eq!(
        map_ir(&outcome(Std001Code::KANI_CHECKER_ABSENT)),
        TerminalValue::Unsupported(UnavailabilityCause::SolverAbsent)
    );
    assert_eq!(
        map_ir(&outcome(Std001Code::KANI_BACKEND_ABSENT)),
        TerminalValue::Unsupported(UnavailabilityCause::BackendAbsent)
    );
    let invalid = KaniOutcome::non_success(
        KaniOutcomeKind::Unavailable,
        Std001Code::KANI_CAPABILITY_MISSING,
        "source",
        "context",
    )
    .unwrap_err();
    assert_eq!(*invalid.requested_kind(), KaniOutcomeKind::Unavailable);
    assert_eq!(
        invalid.requested_code(),
        Std001Code::KANI_CAPABILITY_MISSING
    );
    assert_eq!(invalid.rejected_success_checks(), None);
}

/// A `Counterexample` with a replay disagreement is `Inconclusive(ReplayParity)` carrying its
/// cause, and with a non-fault `ReplayRefusal` is `Inconclusive(ReplayRefused)` carrying that
/// refusal's `code()`.
#[trace("FR-030-AC-9", "TC-041")]
#[test]
fn tc_041_a_counterexample_disagreement_or_refusal_is_inconclusive_with_its_cause() {
    let cause = DisagreementCause::NoValue {
        proved: Verdict::from_category(Category::Violation),
        replayed: Verdict::from_category(Category::Success),
    };
    assert_eq!(
        map_ir_settled(ReplaySettlement::Disagreement(&cause)),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayParity(cause.clone()))
    );
    let refusal = non_fault_refusal();
    assert_eq!(
        map_ir_settled(ReplaySettlement::Refused(&refusal)),
        replayed(refusal.code())
    );
}

/// A counterexample settles as `Failed` with a fault walked through every wrapper the run-outcome
/// map's fault criterion lists and with each failure this repository raises that carries no QSL
/// catalog code.
#[trace("FR-030-AC-10", "TC-041")]
#[test]
fn tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed() {
    let faults = for_each_fault(|settlement| {
        assert_eq!(
            ir_outcome_terminal_value(&counterexample(), Some(settlement)),
            Ok(TerminalValue::Failed)
        );
    });
    assert_eq!(faults, 1 + 2 * 4 + 4, "every named wrapper is exercised");
    for_each_cg_failure(|settlement| {
        assert_eq!(
            ir_outcome_terminal_value(&counterexample(), Some(settlement)),
            Ok(TerminalValue::Failed)
        );
    });
}

/// Across every replay settlement other than reproduced, a counterexample is not `Refuted`.
#[trace("FR-030-AC-11", "TC-041")]
#[test]
fn tc_041_only_a_reproduced_replay_refutes_a_counterexample() {
    let mut exercised = 0;
    with_settlements(|settlement| {
        exercised += 1;
        assert_ne!(map_ir_settled(settlement), TerminalValue::Refuted);
    });
    assert_eq!(exercised, 5, "every reading but reproduced is exercised");
    assert_ne!(
        map_ir_settled(&ReplayVerdict::EvidenceFailure(
            reproduced_without_violation()
        )),
        TerminalValue::Refuted
    );
}

/// A counterexample with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, bare and
/// wrapped, is `Inconclusive(ReplayRefused)` carrying the code QSL's refusal supplies and never
/// `Declined`.
#[trace("FR-030-AC-12", "TC-041")]
#[test]
fn tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined() {
    for_each_setup_refusal(|code, settlement| {
        let value = map_ir_settled(settlement);
        assert_eq!(value, replayed(code));
        assert!(!matches!(value, TerminalValue::Declined { .. }));
    });
}

/// A lock whose only defect is one repeated library identity is refused by `ReplayPackage::new`
/// with QSL's `DuplicateIdentity`, and a counterexample with that refusal is
/// `Inconclusive(ReplayRefused)` carrying `invalid_package`.
#[trace("FR-030-AC-13", "TC-041")]
#[test]
fn tc_041_a_repeated_dependency_identity_after_a_counterexample_is_replay_refused() {
    let refusal = repeated_identity_refusal();
    assert_eq!(
        map_ir_settled(&refusal),
        replayed(Code::InvalidPackage),
        "{refusal}"
    );
}

/// Explicit columns: absent, properties_failed, timeout, out_of_memory, exit_code, unknown.
/// Matching the closed cause type makes an added variant a compile error in this inventory.
fn cause_column(cause: Option<&KaniFailureCause>) -> usize {
    match cause {
        None => 0,
        Some(KaniFailureCause::PropertiesFailed) => 1,
        Some(KaniFailureCause::Timeout) => 2,
        Some(KaniFailureCause::OutOfMemory) => 3,
        Some(KaniFailureCause::ExitCode { .. }) => 4,
        Some(KaniFailureCause::Unknown { .. }) => 5,
    }
}

fn cause_probes() -> Vec<Option<KaniFailureCause>> {
    vec![
        None,
        Some(KaniFailureCause::PropertiesFailed),
        Some(KaniFailureCause::Timeout),
        Some(KaniFailureCause::OutOfMemory),
        Some(KaniFailureCause::ExitCode { code: i32::MIN }),
        Some(KaniFailureCause::ExitCode { code: 0 }),
        Some(KaniFailureCause::ExitCode { code: 64 }),
        Some(KaniFailureCause::ExitCode { code: i32::MAX }),
        Some(KaniFailureCause::Unknown {
            exit_status: "timeout out_of_memory".into(),
        }),
    ]
}

/// Independent literal specification rows; the compiler checks every outcome and reason.
fn ordinary_row(outcome: &KaniRunOutcome, count: u32) -> Option<[TerminalValue; 6]> {
    use KaniInconclusiveReason as R;
    use TerminalValue::{Failed, Incomplete};
    let proved = |success_checks| TerminalValue::Proved {
        basis: ProofBasis::Checks { success_checks },
        certification: Certification::Certified,
    };
    let repeated = |value: TerminalValue| std::array::from_fn(|_| value.clone());
    Some(match outcome {
        KaniRunOutcome::Verified => repeated(proved(count)),
        KaniRunOutcome::CoverUnsatisfied { .. } => repeated(proved(0)),
        KaniRunOutcome::Falsified { .. } => return None,
        KaniRunOutcome::Cancelled { source } => repeated(Incomplete(IncompleteCause::Cancelled {
            source: match source {
                KaniCancellationSource::Requested => CancelCause::Requested,
                KaniCancellationSource::Deadline => CancelCause::Deadline,
            },
        })),
        KaniRunOutcome::Inconclusive { reason } => match reason {
            R::VacuousProof => repeated(proved(0)),
            R::TimedOut => repeated(Incomplete(IncompleteCause::TimedOut)),
            R::MemoryExhausted | R::UnwindBoundExhausted => {
                repeated(Incomplete(IncompleteCause::ResourceExhausted))
            }
            R::NoVerdict | R::MissingCoverSummary => repeated(Failed),
            R::FailedWithoutCounterexample => [
                Failed,
                Failed,
                Incomplete(IncompleteCause::TimedOut),
                Incomplete(IncompleteCause::ResourceExhausted),
                Failed,
                Failed,
            ],
        },
    })
}

/// Exhaust the closed replay vocabulary independently of the run-map implementation.
fn assert_settlement_row(evidence: &KaniExecutionEvidence, settlement: ReplaySettlement<'_>) {
    let expected = match &settlement {
        ReplaySettlement::Reproduced => TerminalValue::Refuted,
        ReplaySettlement::Disagreement(cause) => {
            TerminalValue::Inconclusive(InconclusiveCause::ReplayParity((*cause).clone()))
        }
        ReplaySettlement::Refused(refusal) => TerminalValue::from_replay_refusal(refusal),
        ReplaySettlement::SetupRefused(code) => replayed(*code),
        ReplaySettlement::Fault | ReplaySettlement::CgDefect => TerminalValue::Failed,
    };
    assert_eq!(run_terminal_value(evidence, Some(settlement)), Ok(expected));
}

/// Cross every closed outcome/reason with every cause, payload probes and varied evidence counts.
#[trace(
    "FR-029-AC-32",
    "FR-029-AC-33",
    "FR-029-AC-34",
    "FR-029-AC-35",
    "FR-029-AC-15",
    "TC-040"
)]
#[test]
fn cause_matrix_preserves_literal_terminal_rows_and_pair_refusals() {
    let mut outcomes = settlement_free_outcomes();
    outcomes.extend([
        falsified(),
        KaniRunOutcome::Cancelled {
            source: KaniCancellationSource::Requested,
        },
        KaniRunOutcome::Cancelled {
            source: KaniCancellationSource::Deadline,
        },
    ]);
    // Compile the intentionally breaking root-exported evidence API too.
    let map: fn(
        &KaniExecutionEvidence,
        Option<ReplaySettlement<'_>>,
    ) -> Result<TerminalValue, TerminalPairError> = run_terminal_value;
    for count in [0, 7, 19] {
        for outcome in &outcomes {
            for cause in cause_probes() {
                let mut evidence = synthetic_evidence(outcome.clone(), count);
                evidence.failure_cause = cause;
                let before = evidence.clone();
                if let Some(row) = ordinary_row(outcome, count) {
                    let value = map(&evidence, None).unwrap();
                    assert_eq!(
                        value,
                        row[cause_column(evidence.failure_cause.as_ref())],
                        "{evidence:?}"
                    );
                    if matches!(
                        outcome,
                        KaniRunOutcome::CoverUnsatisfied { .. }
                            | KaniRunOutcome::Inconclusive {
                                reason: KaniInconclusiveReason::VacuousProof
                            }
                    ) {
                        assert_eq!(value.category(), Category::Inconclusive);
                        assert_eq!(
                            value.vacuous_proof_cause(),
                            Some(ReportedInconclusiveCause::KaniVacuousProof)
                        );
                    }
                    assert_eq!(
                        map(&evidence, Some(ReplaySettlement::Reproduced)),
                        Err(TerminalPairError::UnexpectedSettlement)
                    );
                    with_settlements(|settlement| {
                        assert_eq!(
                            map(&evidence, Some(settlement)),
                            Err(TerminalPairError::UnexpectedSettlement)
                        )
                    });
                } else {
                    assert_eq!(
                        map(&evidence, None),
                        Err(TerminalPairError::MissingSettlement)
                    );
                    assert_eq!(
                        map(&evidence, Some(ReplaySettlement::Reproduced)),
                        Ok(TerminalValue::Refuted)
                    );
                    with_settlements(|settlement| assert_settlement_row(&evidence, settlement));
                    for_each_fault(|settlement| {
                        assert_eq!(map(&evidence, Some(settlement)), Ok(TerminalValue::Failed))
                    });
                    for_each_cg_failure(|settlement| {
                        assert_eq!(map(&evidence, Some(settlement)), Ok(TerminalValue::Failed))
                    });
                    for_each_setup_refusal(|code, settlement| {
                        assert_eq!(map(&evidence, Some(settlement)), Ok(replayed(code)))
                    });
                    let refusal = non_fault_refusal();
                    assert_eq!(
                        map(&evidence, Some(ReplaySettlement::Refused(&refusal))),
                        Ok(replayed(refusal.code()))
                    );
                    for cause in [
                        DisagreementCause::Verdicts {
                            proved: Verdict::from_category(Category::Violation),
                            replayed: Verdict::from_category(Category::Success),
                        },
                        DisagreementCause::NoValue {
                            proved: Verdict::from_category(Category::Violation),
                            replayed: Verdict::from_category(Category::Violation),
                        },
                        DisagreementCause::Witness {
                            proved: Verdict::from_category(Category::Violation),
                            replayed: Verdict::from_category(Category::Violation),
                            given: None,
                            derived: None,
                            failure: WitnessFailure::Mismatch,
                        },
                    ] {
                        assert_eq!(
                            map(&evidence, Some(ReplaySettlement::Disagreement(&cause))),
                            Ok(TerminalValue::Inconclusive(
                                InconclusiveCause::ReplayParity(cause.clone())
                            ))
                        );
                    }
                }
                assert_eq!(evidence, before, "terminal projection retains raw evidence");
            }
        }
    }
}
