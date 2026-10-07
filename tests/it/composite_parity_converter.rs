//! FR-033/FR-030: bind public QSL composite reports before reading their terminal values.

use qsl_replay::{
    replay_composite_parity, settle_verified_shadow, CompositeEvidence, CompositeIdentity,
    CompositeParityClaim, CompositeParityResult, DigestDomain, DigestRecord, EqualityOperator,
    EqualityOutcome, FalsifiedParity, Identifier, InconclusiveCause, Integer,
    NativeParityObservation, ObligationIdentity, Origin, Refinement, ReplayLimits, ReplayRefusal,
    ReplayRequest, ReplayRequestRefusal, ReplayRequestWire, ReplaySource, Role, ScalarLimits,
    StateEnvironment, TerminalValue, VerifiedShadow, VerifiedShadowResult, WireNodeId,
    WitnessValue,
};
use quire_contract_codegen::{
    composite_parity_terminal_value, verified_shadow_terminal_value, CompositeReportError,
};

fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 1_000,
        decimal_digits: 1_000,
        scale_expansion: 1_000,
        text_input_bytes: 1_000,
        text_scalars: 1_000,
        normalized_scalars: 1_000,
        unit_edges: 1_000,
        value_occurrences: 1_000,
        work_units: 1_000,
        result_units: 1_000,
    }
}

fn claim() -> CompositeParityClaim {
    CompositeParityClaim {
        node: WireNodeId::from_digest([2; 32]),
        occurrence: Origin::new(Role::new("body"), 0),
        operator: EqualityOperator::Equal,
        obligation_kind: "bounded_shadow".to_owned(),
        harness_bounds: Vec::new(),
        limits: limits(),
        content_identity: DigestRecord::mint(DigestDomain::Sha256Jcs, [3; 32]),
    }
}

fn wire() -> ReplayRequestWire {
    let package = DigestRecord::mint(DigestDomain::PackageSemanticV2, [4; 32]);
    ReplayRequestWire {
        profile_selections: Vec::new(),
        package_id: (Some(package.domain().as_str().to_owned()), package.hex()),
        source_digests: Vec::new(),
        dependencies: Vec::new(),
        selected_function: qsl_replay::QualifiedName::new(vec![Identifier::new("f").unwrap()])
            .unwrap(),
        source: ReplaySource::Input(Vec::new()),
        obligation_identity: [1; 32],
        backend: "test-backend".to_owned(),
        state_environment: StateEnvironment::new(Vec::new()),
        accounting_limits: limits(),
        stage_limits: Default::default(),
        declared_domains: Vec::new(),
        byte_provision: Vec::new(),
    }
}

fn falsified() -> FalsifiedParity {
    let shadow = EqualityOutcome {
        equal: false,
        pair_count: 1,
    };
    FalsifiedParity {
        operands: [
            WitnessValue::ExactInteger(Integer::from(1_i64)),
            WitnessValue::ExactInteger(Integer::from(2_i64)),
        ],
        shadow,
        native: NativeParityObservation::Completed(shadow),
        refinement: Refinement::Disagreed,
    }
}

/// A real QSL prepare refusal remains a bound report and carries QSL's own code before F-1.
/// Trace: FR-029-AC-28, FR-030-AC-17, FR-033-AC-9, TC-048
#[test]
fn tc_048_bound_prepare_refusal_passes_through_qsl_terminal() {
    let wire = wire();
    ReplayRequest::decode(wire.clone(), ReplayLimits::default())
        .expect("the wire request itself is valid at the caller's ordinary limit");
    let claim = claim();
    let evidence = falsified();
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(
        wire,
        claim,
        evidence,
        ReplayLimits::default().with_input_bytes(1),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::Refused(refusal) = bound.result else {
        panic!("QSL prepare should refuse before F-1: {:?}", bound.result);
    };
    assert!(matches!(
        &**refusal,
        ReplayRefusal::Request(ReplayRequestRefusal::BoundExceeded(_))
    ));
    assert_eq!(
        bound.terminal_value,
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
    assert_eq!(bound.terminal_value, report.terminal_value());
}

/// Each retained identity member is checked against the real QSL report, including the native
/// cause and refinement. A missing report has no terminal value.
/// Trace: FR-033-AC-9, FR-030-AC-17, TC-048
#[test]
fn tc_048_falsified_report_requires_every_sent_identity_member() {
    let wire = wire();
    let claim = claim();
    let evidence = falsified();
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    assert!(composite_parity_terminal_value(&sent, Some(&report)).is_ok());
    assert_eq!(
        composite_parity_terminal_value(&sent, None).unwrap_err(),
        CompositeReportError::MissingReport
    );

    let mut changed = Vec::new();
    let mut other = sent.clone();
    other.obligation = ObligationIdentity::from_digest([8; 32]);
    changed.push(other);
    let mut other = sent.clone();
    other.node = WireNodeId::from_digest([8; 32]);
    changed.push(other);
    let mut other = sent.clone();
    other.occurrence = Origin::new(Role::new("body"), 1);
    changed.push(other);
    let mut other = sent.clone();
    other.operator = EqualityOperator::NotEqual;
    changed.push(other);
    let mut other = sent.clone();
    other.obligation_kind = "other".to_owned();
    changed.push(other);
    let mut other = sent.clone();
    other.harness_bounds.push(qsl_replay::ProofBound {
        domain: qsl_replay::DomainKey::Node {
            node: sent.node,
            path: Vec::new(),
        },
        bound: qsl_replay::FiniteBound::cardinality(1),
    });
    changed.push(other);
    let mut other = sent.clone();
    other.limits.work_units += 1;
    changed.push(other);
    let mut other = sent.clone();
    other.content_identity = DigestRecord::mint(DigestDomain::Sha256Jcs, [8; 32]);
    changed.push(other);
    let CompositeEvidence::Falsified(_) = &sent.evidence else {
        panic!("fixture is falsified");
    };
    for edit in [
        |e: &mut FalsifiedParity| e.operands[0] = WitnessValue::ExactInteger(Integer::from(3_i64)),
        |e: &mut FalsifiedParity| e.operands[1] = WitnessValue::ExactInteger(Integer::from(4_i64)),
        |e: &mut FalsifiedParity| e.shadow.equal = true,
        |e: &mut FalsifiedParity| e.shadow.pair_count += 1,
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Completed(EqualityOutcome {
                equal: true,
                pair_count: 1,
            })
        },
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Completed(EqualityOutcome {
                equal: false,
                pair_count: 2,
            })
        },
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
                "stopped".to_owned(),
            ))
        },
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
                "other stop".to_owned(),
            ))
        },
        |e: &mut FalsifiedParity| e.refinement = Refinement::CeilingReached,
    ] {
        let mut other = sent.clone();
        let CompositeEvidence::Falsified(ref mut evidence) = other.evidence else {
            unreachable!()
        };
        edit(evidence);
        changed.push(other);
    }
    for other in changed {
        assert_eq!(
            composite_parity_terminal_value(&other, Some(&report)).unwrap_err(),
            CompositeReportError::ClaimMismatch
        );
    }
}

/// The report binds to the identity of a request changed before send, with no earlier unsent
/// identity influencing the result.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_changed_request_binds_to_its_actual_sent_identity() {
    let mut wire = wire();
    let mut claim = claim();
    let evidence = falsified();
    let unsent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    wire.obligation_identity = [9; 32];
    claim.limits.work_units += 1;
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    assert!(composite_parity_terminal_value(&sent, Some(&report)).is_ok());
    assert_eq!(
        composite_parity_terminal_value(&unsent, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// Native fault descriptors are part of the full observation even when QSL's common step
/// refuses before F-2 can be considered.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_native_cause_is_part_of_the_bound_observation() {
    let wire = wire();
    let claim = claim();
    let mut evidence = falsified();
    evidence.native = NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
        "actual native stop".to_owned(),
    ));
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    assert!(composite_parity_terminal_value(&sent, Some(&report)).is_ok());
    let mut other = sent.clone();
    let CompositeEvidence::Falsified(ref mut other_evidence) = other.evidence else {
        panic!("fixture is falsified");
    };
    other_evidence.native = NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
        "different stop".to_owned(),
    ));
    assert_eq!(
        composite_parity_terminal_value(&other, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// Verified reports use their own observation shape and preserve QSL's settlement.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_verified_report_requires_success_count_and_refinement() {
    let wire = wire();
    let claim = claim();
    let verified = VerifiedShadow {
        success_checks: 3,
        refinement: Refinement::Exhausted,
    };
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Verified(verified),
    );
    let report = settle_verified_shadow(wire, claim, verified, ReplayLimits::default());
    let bound = verified_shadow_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(bound.result, VerifiedShadowResult::Refused(_)));
    assert_eq!(bound.terminal_value, report.terminal_value());
    assert_eq!(
        verified_shadow_terminal_value(&sent, None).unwrap_err(),
        CompositeReportError::MissingReport
    );
    for evidence in [
        VerifiedShadow {
            success_checks: 4,
            ..verified
        },
        VerifiedShadow {
            refinement: Refinement::NotExhausted,
            ..verified
        },
    ] {
        let mut other = sent.clone();
        other.evidence = CompositeEvidence::Verified(evidence);
        assert_eq!(
            verified_shadow_terminal_value(&other, Some(&report)).unwrap_err(),
            CompositeReportError::ClaimMismatch
        );
    }
}
