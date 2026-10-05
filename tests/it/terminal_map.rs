//! FR-029: the total map from a Kani run outcome, paired with its replay settlement, to QSL's
//! terminal value.
//!
//! The replay failures are built from the public types of this crate and of `qsl-replay`; a
//! repeated dependency identity is the refusal `ReplayPackage::new` returns for a real lock.

use qsl_replay::{
    CallSiteRefusal, Category, ClauseName, Code, DependencyInput, DependencyInputRefusal,
    DigestDomain, DigestRecord, DisagreementCause, FrameCounterexample, Identifier,
    IncompleteCause, InconclusiveCause, MalformedTranscript, OperationName, QualifiedName,
    ReplayRefusal, ReportedInconclusiveCause, ScalarLimits, SourceIdentity, StageLimits,
    SuppliedLibrary, TerminalValue, Verdict, Witness, WitnessEnvelope, WitnessFailure,
    WitnessPacket,
};
use quire_contract_codegen::{
    run_terminal_value, DecodeFailure, DependencyLock, DependencyLockError, EvidenceFailureCause,
    FrameReplayError, KaniInconclusiveReason, KaniRunOutcome, LockedSource,
    ObligationIdentityError, ReplayInputs, ReplayPackage, ReplayPackageError, ReplaySettlement,
    ReplayVerdict, SpineReplayError, TerminalPairError,
};

/// Every inconclusive reason the classifier has today.
const REASONS: [KaniInconclusiveReason; 6] = [
    KaniInconclusiveReason::FailedWithoutCounterexample,
    KaniInconclusiveReason::NoVerdict,
    KaniInconclusiveReason::MissingCoverSummary,
    KaniInconclusiveReason::VacuousProof,
    KaniInconclusiveReason::UnwindBoundExhausted,
    KaniInconclusiveReason::TimedOut,
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

fn map_run(outcome: &KaniRunOutcome, settlement: Option<ReplaySettlement<'_>>) -> TerminalValue {
    run_terminal_value(outcome, 3, settlement).expect("a well-paired outcome maps")
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

/// `verified` with three SUCCESS checks is `Proved { success_checks: 3 }`, and `falsified` with
/// a reproduced replay is `Refuted`.
///
/// Trace: FR-029-AC-1, TC-040
#[test]
fn tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes() {
    assert_eq!(
        map_run(&KaniRunOutcome::Verified, None),
        TerminalValue::Proved { success_checks: 3 }
    );
    assert_eq!(
        map_run(&falsified(), Some(ReplaySettlement::Reproduced)),
        TerminalValue::Refuted
    );
}

/// The vacuous-proof reason and `cover-unsatisfied` each map to `Proved { success_checks: 0 }`,
/// which QSL reads as category `inconclusive` with the vacuity cause.
///
/// Trace: FR-029-AC-2, TC-040
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
        assert_eq!(value, TerminalValue::Proved { success_checks: 0 });
        assert_eq!(value.category(), Category::Inconclusive);
        assert_eq!(
            value.vacuous_proof_cause(),
            Some(ReportedInconclusiveCause::KaniVacuousProof)
        );
    }
}

/// The timed-out reason maps to `Incomplete(TimedOut)` and the exhausted-unwind-bound reason to
/// `Incomplete(ResourceExhausted)`.
///
/// Trace: TC-040
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
///
/// Trace: FR-029-AC-4, TC-040
#[test]
fn tc_040_a_run_with_no_verdict_is_failed() {
    assert_eq!(
        map_run(&inconclusive(KaniInconclusiveReason::NoVerdict), None),
        TerminalValue::Failed
    );
}

/// The failure-without-counterexample and missing-cover-summary reasons each map to `Failed`.
///
/// Trace: FR-029-AC-5, TC-040
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
///
/// Trace: FR-029-AC-6, TC-040
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
///
/// Trace: FR-029-AC-15, TC-040
#[test]
fn tc_040_a_settlement_accompanies_a_falsified_outcome_only() {
    assert_eq!(
        run_terminal_value(&falsified(), 0, None),
        Err(TerminalPairError::MissingSettlement)
    );
    for outcome in settlement_free_outcomes() {
        assert_eq!(
            run_terminal_value(&outcome, 3, Some(ReplaySettlement::Reproduced)),
            Err(TerminalPairError::UnexpectedSettlement),
            "{outcome:?}"
        );
    }
}

/// A replay disagreement of each cause maps to `Inconclusive(ReplayParity)` carrying that cause.
///
/// Trace: FR-029-AC-8, TC-040
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
///
/// Trace: FR-029-AC-9, TC-040
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
    let frame = FrameReplayError::Refused(Box::new(non_fault_refusal()));
    assert_eq!(map_settled(&frame), expected);
}

/// Every failure this repository raises that carries no QSL catalog code maps to `Failed`, so no
/// `ReplayRefused` value carries a code no QSL refusal value supplied.
///
/// Trace: FR-029-AC-11, TC-040
#[test]
fn tc_040_each_failure_this_repository_raises_is_failed() {
    let spine = [
        SpineReplayError::UnboundArgument {
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
        assert_eq!(map_settled(error), TerminalValue::Failed, "{error}");
    }

    let empty_name = QualifiedName::new(Vec::new()).expect_err("an empty qualified name");
    let envelope = WitnessEnvelope::reconstruct(WitnessPacket::<FrameCounterexample> {
        obligation_identity: None,
        occurrence_key: None,
        clause_node: None,
        selected_function: None,
        package_id: None,
        source_digests: None,
        profile_selections: None,
        run_limits: None,
        declared_domains: None,
        backend: None,
        trace_position: None,
        source: None,
        family_payload: None,
    })
    .map(|_| ())
    .expect_err("an envelope with no members is refused");
    let frame = [
        FrameReplayError::Transcript(malformed()),
        FrameReplayError::Envelope(envelope),
        FrameReplayError::Name(empty_name),
    ];
    for error in &frame {
        assert_eq!(map_settled(error), TerminalValue::Failed, "{error}");
    }

    let invalid_function = ReplayPackageError::InvalidFunction {
        function: "1x".to_owned(),
    };
    assert_eq!(map_settled(&invalid_function), TerminalValue::Failed);

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
        assert_eq!(map_settled(&verdict), TerminalValue::Failed);
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

/// The fault reading is `Failed`. QSL's `InternalFault` cannot be built here, so FR-029-AC-10 is
/// not tagged; the reading this crate owns is asserted all the same.
///
/// Trace: TC-040
#[test]
fn tc_040_a_fault_settlement_is_failed() {
    assert_eq!(map_settled(ReplaySettlement::Fault), TerminalValue::Failed);
}

/// Across every replay settlement other than reproduced, a falsified run is not `Refuted`, and a
/// reproduced replay in a category other than `violation` is not read as a reproduction either.
///
/// Trace: FR-029-AC-12, TC-040
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
///
/// Trace: FR-029-AC-13, TC-040
#[test]
fn tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined() {
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
        ]
    };
    for refusal in call_sites() {
        let expected = replayed(refusal.code());
        assert_eq!(map_settled(&refusal), expected, "{refusal}");
        let package = ReplayPackageError::CallSite(Box::new(refusal));
        assert_eq!(map_settled(&package), expected, "{package}");
    }
    for refusal in call_sites() {
        let expected = replayed(refusal.code());
        let frame = FrameReplayError::CallSite(Box::new(refusal));
        assert_eq!(map_settled(&frame), expected, "{frame}");
    }

    // The empty identity is the refusal whose code is not `invalid_package`, so a conversion that
    // ignores the refusal's own code cannot pass.
    let empty = empty_identity();
    assert_eq!(empty.code(), Code::InvalidIdentifier);
    for refusal in [duplicate_identity(), shared_owner(), empty] {
        let expected = replayed(refusal.code());
        let lock = DependencyLockError::Input(refusal.clone());
        assert_eq!(map_settled(&lock), expected);
        let package = ReplayPackageError::Dependencies(DependencyLockError::Input(refusal.clone()));
        assert_eq!(map_settled(&package), expected);
        let frame = FrameReplayError::Dependencies(DependencyLockError::Input(refusal));
        assert_eq!(map_settled(&frame), expected);
    }
}

/// A lock whose only defect is one library identity selected twice is refused by
/// `ReplayPackage::new` with QSL's `DuplicateIdentity`, and a falsified run with that refusal is
/// `Inconclusive(ReplayRefused)` carrying `invalid_package`.
///
/// Trace: FR-029-AC-14, TC-040
#[test]
fn tc_040_a_repeated_dependency_identity_is_replay_refused_with_invalid_package() {
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
        source: locked("unit"),
        dependencies: vec![dependency("lib-one"), dependency("lib-two")],
        accounting_limits: unlimited,
        stage_limits: StageLimits {
            s1: unlimited,
            s2: unlimited,
            s3: unlimited,
            s4: unlimited,
        },
    };
    let refusal = ReplayPackage::new(inputs, "f").expect_err("the repeated identity is refused");
    let ReplayPackageError::Dependencies(DependencyLockError::Input(input)) = &refusal else {
        panic!("expected a refusal from QSL's dependency input, got {refusal}");
    };
    assert!(
        matches!(input, DependencyInputRefusal::DuplicateIdentity { .. }),
        "{input}"
    );
    assert_eq!(
        map_settled(&refusal),
        replayed(Code::InvalidPackage),
        "{refusal}"
    );
}
