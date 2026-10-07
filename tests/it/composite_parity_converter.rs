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

struct PublicCompositeFixture {
    wire: ReplayRequestWire,
    claim: CompositeParityClaim,
}

fn sequence_value(value: i64) -> WitnessValue {
    WitnessValue::Sequence(vec![WitnessValue::ExactInteger(Integer::from(value))])
}

fn falsified_evidence() -> FalsifiedParity {
    let shadow = EqualityOutcome {
        equal: false,
        pair_count: 2,
    };
    FalsifiedParity {
        operands: [sequence_value(1), sequence_value(2)],
        shadow,
        native: NativeParityObservation::Completed(shadow),
        refinement: Refinement::NotExhausted,
    }
}

fn public_report(
    wire: ReplayRequestWire,
    claim: CompositeParityClaim,
    evidence: FalsifiedParity,
    replay_limits: ReplayLimits,
) -> (CompositeIdentity, qsl_replay::CompositeParityReport) {
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, replay_limits);
    (sent, report)
}

fn public_composite_fixture() -> PublicCompositeFixture {
    use qsl_replay::{
        compile_package, parity_obligation, BoundEntries, ByteDigest, DependencyInput, Domain,
        DomainKey, FiniteBound, OperandIdentity, ParityArgument, ParityPreimage, ProofBound,
        SourceIdentity, StageLimits,
    };
    use quire_contract_model::{
        CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
        CheckedPackageV2ReadResult,
    };
    let source = b"language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction f using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { a = b }\n";
    let compiled = compile_package(
        SourceIdentity::new("test", "ir666", "git", "r1"),
        "ir666.native",
        source,
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("compile package");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("read package")
    };
    let nodes = &package.graph().nodes;
    let named = |name: &str| {
        nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == name
                })
            })
            .expect("named declaration in compiled package")
    };
    let function = named("f");
    let binding = |name: &str| {
        function.body["members"]
            .as_array()
            .unwrap()
            .iter()
            .find(|member| member["name"] == name)
            .unwrap()["value"]
            .clone()
    };
    let parameters = binding("parameters")["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| WireNodeId::from_hex(entry["target"]["digest"].as_str().unwrap()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(parameters.len(), 2);
    let application = nodes
        .iter()
        .find(|node| {
            node.node_tag.as_ref() == "expression"
                && node.body["operation"]["identity"] == "quire.op.structural.eq"
        })
        .expect("compiled structural equality");
    let node = WireNodeId::from_hex(&application.node_id.digest).unwrap();
    let occurrence = &application.occurrences[0];
    let origin = Origin::new(Role::new("expression"), occurrence.ordinal);
    let bounds = parameters
        .iter()
        .flat_map(|parameter| {
            [
                ProofBound {
                    domain: DomainKey::Node {
                        node: *parameter,
                        path: Vec::new(),
                    },
                    bound: FiniteBound::cardinality(3),
                },
                ProofBound {
                    domain: DomainKey::Node {
                        node: *parameter,
                        path: vec![0],
                    },
                    bound: FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64))
                        .unwrap(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let claim = CompositeParityClaim {
        node,
        occurrence: origin.clone(),
        operator: EqualityOperator::Equal,
        obligation_kind: "bounded_shadow".to_owned(),
        harness_bounds: bounds.clone(),
        limits: limits(),
        content_identity: DigestRecord::mint(DigestDomain::Sha256Jcs, [3; 32]),
    };
    let preimage = ParityPreimage {
        node,
        occurrence: origin,
        obligation_kind: claim.obligation_kind.clone(),
        arguments: parameters
            .iter()
            .map(|parameter| {
                let parameter_bounds = bounds
                    .iter()
                    .filter(|bound| {
                        matches!(bound.domain, DomainKey::Node { node, .. } if node == *parameter)
                    })
                    .cloned()
                    .collect();
                ParityArgument {
                    identity: OperandIdentity::GraphChild(*parameter),
                    domain: Domain::Bounds(BoundEntries::new(parameter_bounds).unwrap()),
                }
            })
            .collect(),
    };
    let obligation = parity_obligation(&preimage).expect("the public preimage encodes");
    let digest = DigestRecord::mint(
        DigestDomain::SourceBytesV1,
        ByteDigest::of(source).as_bytes(),
    );
    let wire = ReplayRequestWire {
        profile_selections: Vec::new(),
        package_id: (
            Some(compiled.package_id().domain().as_str().to_owned()),
            compiled.package_id().hex(),
        ),
        source_digests: vec![(
            "test".to_owned(),
            "ir666".to_owned(),
            "git".to_owned(),
            "r1".to_owned(),
            Some(digest.domain().as_str().to_owned()),
            digest.hex(),
        )],
        dependencies: Vec::new(),
        selected_function: qsl_replay::QualifiedName::new(vec![Identifier::new("f").unwrap()])
            .unwrap(),
        source: ReplaySource::Input(Vec::new()),
        obligation_identity: *obligation.as_bytes(),
        backend: "test-backend".to_owned(),
        state_environment: StateEnvironment::new(Vec::new()),
        accounting_limits: limits(),
        stage_limits: Default::default(),
        declared_domains: Vec::new(),
        byte_provision: vec![(
            Some(digest.domain().as_str().to_owned()),
            digest.hex(),
            source.to_vec(),
        )],
    };
    PublicCompositeFixture { wire, claim }
}

/// A valid public claim lets retained disagreement win over native faults and invalid operands;
/// either native fault kind wins over admission and both limit stages without disagreement.
/// Trace: FR-029-AC-20, FR-030-AC-15, FR-033-AC-12, FR-033-AC-13, TC-041, TC-048
#[test]
fn tc_041_f1_and_f2_preserve_precedence_and_native_causes() {
    let fixture = public_composite_fixture();
    let mut evidence = falsified_evidence();
    evidence.refinement = Refinement::Disagreed;
    evidence.operands[0] = sequence_value(12);
    evidence.native = NativeParityObservation::ExecutionFault(qsl_replay::NativeCause::new(
        "native failed".to_owned(),
    ));
    let mut wire = fixture.wire.clone();
    wire.accounting_limits.value_occurrences = 0;
    let mut claim = fixture.claim.clone();
    claim.limits.value_occurrences = 0;
    let (sent, report) = public_report(wire, claim, evidence, ReplayLimits::default());
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(bound.result(), CompositeParityResult::Disagreed));
    assert_eq!(bound.terminal_value(), TerminalValue::Failed);

    for native in [
        NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
            "native limit".to_owned(),
        )),
        NativeParityObservation::ExecutionFault(qsl_replay::NativeCause::new(
            "native fault".to_owned(),
        )),
    ] {
        let mut evidence = falsified_evidence();
        evidence.refinement = Refinement::CeilingReached;
        evidence.operands[0] = sequence_value(12);
        evidence.native = native.clone();
        let mut wire = fixture.wire.clone();
        wire.accounting_limits.value_occurrences = 0;
        let mut claim = fixture.claim.clone();
        claim.limits.value_occurrences = 0;
        let (sent, report) = public_report(wire, claim, evidence, ReplayLimits::default());
        let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
        let CompositeParityResult::GeneratedFault { native: actual } = bound.result() else {
            panic!("expected F-2, got {:?}", bound.result());
        };
        assert_eq!(actual, &native);
        assert_eq!(bound.terminal_value(), TerminalValue::Failed);
    }
}

/// F-3, F-4, F-5 and F-6 remain separate typed rows with the exact QSL terminal value.
/// Trace: FR-029-AC-20, FR-030-AC-16, FR-033-AC-13, TC-041, TC-048
#[test]
fn tc_041_f3_to_f6_keep_operand_and_limit_stages_distinct() {
    let fixture = public_composite_fixture();
    let mut refused = falsified_evidence();
    refused.operands[0] = sequence_value(12);
    refused.refinement = Refinement::CeilingReached;
    let (sent, report) = public_report(
        fixture.wire.clone(),
        fixture.claim.clone(),
        refused,
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::RefusedInput(refusal) = bound.result() else {
        panic!("expected F-3, got {:?}", bound.result());
    };
    assert_eq!(refusal.index, 0);
    assert_eq!(refusal.code(), qsl_replay::Code::InvalidRuntimeInput);
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );

    let mut ceiling = falsified_evidence();
    ceiling.refinement = Refinement::CeilingReached;
    let mut wire = fixture.wire.clone();
    wire.accounting_limits.value_occurrences = 0;
    let (sent, report) = public_report(
        wire,
        fixture.claim.clone(),
        ceiling.clone(),
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::Incomplete {
        stage: qsl_replay::IncompleteStage::Admission(incomplete),
    } = bound.result()
    else {
        panic!("expected F-4 admission, got {:?}", bound.result());
    };
    assert_eq!(incomplete.limit, 0);
    assert_eq!(incomplete.limit_kind.as_str(), "value_occurrences");
    assert_eq!(incomplete.consumed, 0);
    assert_eq!(incomplete.next_charge, Integer::from(2_i64));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted)
    );

    let mut claim = fixture.claim.clone();
    claim.limits.value_occurrences = 0;
    let (sent, report) = public_report(
        fixture.wire.clone(),
        claim,
        ceiling.clone(),
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::Incomplete {
        stage: qsl_replay::IncompleteStage::ExactEvaluation(incomplete),
    } = bound.result()
    else {
        panic!("expected F-5 exact evaluation, got {:?}", bound.result());
    };
    assert_eq!(incomplete.limit, 0);
    assert_eq!(incomplete.limit_kind.as_str(), "value_occurrences");
    assert_eq!(incomplete.consumed, 0);
    assert_eq!(incomplete.next_charge, Integer::from(2_i64));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted)
    );

    let (sent, report) = public_report(
        fixture.wire,
        fixture.claim,
        ceiling,
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(
        bound.result(),
        CompositeParityResult::Incomplete {
            stage: qsl_replay::IncompleteStage::RefinementCeiling
        }
    ));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted)
    );
}

/// Exact verdict and pair-count divergence are distinct F-7 inputs; agreement carries the full
/// composite claim and equality outcome, and none of the three becomes Refuted.
/// Trace: FR-029-AC-20, FR-030-AC-17, FR-033-AC-7, TC-041, TC-048
#[test]
fn tc_041_f7_compares_verdict_and_pair_count_then_preserves_agreement() {
    let fixture = public_composite_fixture();
    for shadow in [
        EqualityOutcome {
            equal: true,
            pair_count: 2,
        },
        EqualityOutcome {
            equal: false,
            pair_count: 3,
        },
    ] {
        let mut evidence = falsified_evidence();
        evidence.shadow = shadow;
        let (sent, report) = public_report(
            fixture.wire.clone(),
            fixture.claim.clone(),
            evidence,
            ReplayLimits::default(),
        );
        let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
        let CompositeParityResult::Diverged {
            exact,
            shadow: kept,
            ..
        } = bound.result()
        else {
            panic!("expected F-7 divergence, got {:?}", bound.result());
        };
        assert_eq!(
            *exact,
            EqualityOutcome {
                equal: false,
                pair_count: 2
            }
        );
        assert_eq!(*kept, shadow);
        assert_eq!(bound.terminal_value(), TerminalValue::Failed);
    }
    let (sent, report) = public_report(
        fixture.wire,
        fixture.claim,
        falsified_evidence(),
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::Agrees { agreement, .. } = bound.result() else {
        panic!("expected F-7 agreement, got {:?}", bound.result());
    };
    assert!(
        matches!(agreement.claim(), qsl_replay::ScalarClaim::CompositeEquality(claim) if **claim == sent)
    );
    assert_eq!(
        agreement.outcome(),
        &qsl_replay::ScalarOutcome::Equality(EqualityOutcome {
            equal: false,
            pair_count: 2
        })
    );
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ScalarAgrees(agreement.clone()))
    );
}

/// The verified entry retains its distinct V-row outcomes through the same full-identity check.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_verified_public_rows_keep_their_qsl_terminal_values() {
    let fixture = public_composite_fixture();
    for (verified, expected) in [
        (
            VerifiedShadow {
                success_checks: 3,
                refinement: Refinement::Disagreed,
            },
            TerminalValue::Failed,
        ),
        (
            VerifiedShadow {
                success_checks: 3,
                refinement: Refinement::CeilingReached,
            },
            TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted),
        ),
        (
            VerifiedShadow {
                success_checks: 0,
                refinement: Refinement::Exhausted,
            },
            TerminalValue::Proved { success_checks: 0 },
        ),
        (
            VerifiedShadow {
                success_checks: 3,
                refinement: Refinement::Exhausted,
            },
            TerminalValue::Proved { success_checks: 3 },
        ),
        (
            VerifiedShadow {
                success_checks: 3,
                refinement: Refinement::NotExhausted,
            },
            TerminalValue::Tested,
        ),
    ] {
        let sent = CompositeIdentity::new(
            ObligationIdentity::from_digest(fixture.wire.obligation_identity),
            &fixture.claim,
            CompositeEvidence::Verified(verified),
        );
        let report = settle_verified_shadow(
            fixture.wire.clone(),
            fixture.claim.clone(),
            verified,
            ReplayLimits::default(),
        );
        let bound = verified_shadow_terminal_value(&sent, Some(&report)).unwrap();
        match verified.refinement {
            Refinement::Disagreed => {
                assert!(matches!(bound.result(), VerifiedShadowResult::Disagreed))
            }
            Refinement::CeilingReached => assert!(matches!(
                bound.result(),
                VerifiedShadowResult::Incomplete {
                    stage: qsl_replay::IncompleteStage::RefinementCeiling
                }
            )),
            Refinement::Exhausted if verified.success_checks == 0 => {
                assert!(matches!(bound.result(), VerifiedShadowResult::Vacuous))
            }
            Refinement::Exhausted => assert!(matches!(
                bound.result(),
                VerifiedShadowResult::Proved { success_checks: 3 }
            )),
            Refinement::NotExhausted => {
                assert!(matches!(bound.result(), VerifiedShadowResult::Tested))
            }
        }
        assert_eq!(bound.terminal_value(), expected);
    }
}

/// A real QSL prepare refusal remains a bound report and carries QSL's own code before F-1.
/// Trace: FR-029-AC-28, FR-030-AC-17, FR-033-AC-9, TC-041, TC-048
#[test]
fn tc_041_bound_prepare_refusal_precedes_disagreed() {
    let fixture = public_composite_fixture();
    let wire = fixture.wire;
    ReplayRequest::decode(wire.clone(), ReplayLimits::default())
        .expect("the wire request itself is valid at the caller's ordinary limit");
    let claim = fixture.claim;
    let mut evidence = falsified_evidence();
    evidence.refinement = Refinement::Disagreed;
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
    let CompositeParityResult::Refused(refusal) = bound.result() else {
        panic!("QSL prepare should refuse before F-1: {:?}", bound.result());
    };
    assert!(matches!(
        &**refusal,
        ReplayRefusal::Request(ReplayRequestRefusal::BoundExceeded(_))
    ));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
    assert_eq!(bound.terminal_value(), report.terminal_value());
}

/// Each retained identity member is checked against the real QSL report, including the native
/// cause and refinement. A missing report has no terminal value.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_falsified_report_requires_every_sent_identity_member() {
    let fixture = public_composite_fixture();
    let wire = fixture.wire;
    let claim = fixture.claim;
    let evidence = falsified_evidence();
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(
        bound.result(),
        CompositeParityResult::Agrees { .. }
    ));
    assert!(matches!(
        bound.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ScalarAgrees(_))
    ));
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
    other.occurrence = Origin::new(Role::new("expression"), 1);
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
        |e: &mut FalsifiedParity| e.operands[0] = sequence_value(3),
        |e: &mut FalsifiedParity| e.operands[1] = sequence_value(4),
        |e: &mut FalsifiedParity| e.shadow.equal = true,
        |e: &mut FalsifiedParity| e.shadow.pair_count += 1,
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Completed(EqualityOutcome {
                equal: true,
                pair_count: 2,
            })
        },
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Completed(EqualityOutcome {
                equal: false,
                pair_count: 3,
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
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::ExecutionFault(qsl_replay::NativeCause::new(
                "execution fault".to_owned(),
            ))
        },
        |e: &mut FalsifiedParity| {
            e.native = NativeParityObservation::Refused(qsl_replay::Code::InvalidRuntimeInput)
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
    let mut other = sent.clone();
    other.evidence = CompositeEvidence::Verified(VerifiedShadow {
        success_checks: 3,
        refinement: Refinement::Exhausted,
    });
    assert_eq!(
        composite_parity_terminal_value(&other, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// The report binds to the identity of a request changed before send, with no earlier unsent
/// identity influencing the result.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_changed_request_binds_to_its_actual_sent_identity() {
    let fixture = public_composite_fixture();
    let wire = fixture.wire;
    let mut claim = fixture.claim;
    let evidence = falsified_evidence();
    let unsent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    claim.limits.work_units += 1;
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(
        bound.result(),
        CompositeParityResult::Agrees { .. }
    ));
    assert_eq!(
        composite_parity_terminal_value(&unsent, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// Native fault descriptors are part of the full observation on a generated-fault report.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_native_cause_is_part_of_the_bound_observation() {
    let fixture = public_composite_fixture();
    let wire = fixture.wire;
    let claim = fixture.claim;
    let mut evidence = falsified_evidence();
    evidence.native = NativeParityObservation::Incomplete(qsl_replay::NativeCause::new(
        "actual native stop".to_owned(),
    ));
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(wire.obligation_identity),
        &claim,
        CompositeEvidence::Falsified(Box::new(evidence.clone())),
    );
    let report = replay_composite_parity(wire, claim, evidence, ReplayLimits::default());
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    let CompositeParityResult::GeneratedFault { native } = bound.result() else {
        panic!("expected F-2, got {:?}", bound.result());
    };
    assert!(
        matches!(native, NativeParityObservation::Incomplete(cause) if cause.as_str() == "actual native stop")
    );
    assert_eq!(bound.terminal_value(), TerminalValue::Failed);
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

    let fixture = public_composite_fixture();
    let mut evidence = falsified_evidence();
    evidence.native = NativeParityObservation::ExecutionFault(qsl_replay::NativeCause::new(
        "actual execution fault".to_owned(),
    ));
    let (sent, report) = public_report(
        fixture.wire.clone(),
        fixture.claim.clone(),
        evidence,
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(
        matches!(bound.result(), CompositeParityResult::GeneratedFault { native: NativeParityObservation::ExecutionFault(cause) } if cause.as_str() == "actual execution fault")
    );
    let mut other = sent.clone();
    let CompositeEvidence::Falsified(ref mut other_evidence) = other.evidence else {
        panic!("fixture is falsified");
    };
    other_evidence.native = NativeParityObservation::ExecutionFault(qsl_replay::NativeCause::new(
        "changed execution fault".to_owned(),
    ));
    assert_eq!(
        composite_parity_terminal_value(&other, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );

    let mut evidence = falsified_evidence();
    evidence.native = NativeParityObservation::Refused(qsl_replay::Code::InvalidRuntimeInput);
    let (sent, report) = public_report(
        fixture.wire,
        fixture.claim,
        evidence,
        ReplayLimits::default(),
    );
    let bound = composite_parity_terminal_value(&sent, Some(&report)).unwrap();
    assert!(matches!(
        bound.result(),
        CompositeParityResult::Agrees { .. }
    ));
    let mut other = sent;
    let CompositeEvidence::Falsified(ref mut other_evidence) = other.evidence else {
        panic!("fixture is falsified");
    };
    other_evidence.native = NativeParityObservation::Refused(qsl_replay::Code::InvalidRequest);
    assert_eq!(
        composite_parity_terminal_value(&other, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// Verified reports use their own observation shape and preserve QSL's settlement.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_verified_report_requires_success_count_and_refinement() {
    let fixture = public_composite_fixture();
    let wire = fixture.wire;
    let claim = fixture.claim;
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
    assert!(matches!(
        bound.result(),
        VerifiedShadowResult::Proved { success_checks: 3 }
    ));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Proved { success_checks: 3 }
    );
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
    let mut other = sent;
    other.evidence = CompositeEvidence::Falsified(Box::new(falsified_evidence()));
    assert_eq!(
        verified_shadow_terminal_value(&other, Some(&report)).unwrap_err(),
        CompositeReportError::ClaimMismatch
    );
}

/// A verified prepare refusal is a bound QSL report, with the same refusal code QSL emitted.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_verified_prepare_refusal_passes_through() {
    let fixture = public_composite_fixture();
    let verified = VerifiedShadow {
        success_checks: 3,
        refinement: Refinement::Disagreed,
    };
    let sent = CompositeIdentity::new(
        ObligationIdentity::from_digest(fixture.wire.obligation_identity),
        &fixture.claim,
        CompositeEvidence::Verified(verified),
    );
    let report = settle_verified_shadow(
        fixture.wire,
        fixture.claim,
        verified,
        ReplayLimits::default().with_input_bytes(1),
    );
    let bound = verified_shadow_terminal_value(&sent, Some(&report)).unwrap();
    let VerifiedShadowResult::Refused(refusal) = bound.result() else {
        panic!("expected QSL prepare refusal, got {:?}", bound.result());
    };
    assert!(matches!(
        &**refusal,
        ReplayRefusal::Request(ReplayRequestRefusal::BoundExceeded(_))
    ));
    assert_eq!(
        bound.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
}

/// The two CG binding refusals stay distinct and human-readable.
/// Trace: FR-033-AC-9, TC-048
#[test]
fn tc_048_binding_refusals_have_distinct_display() {
    assert_eq!(
        CompositeReportError::MissingReport.to_string(),
        "composite parity report is missing"
    );
    assert_eq!(
        CompositeReportError::ClaimMismatch.to_string(),
        "composite parity report claim differs from sent claim"
    );
}
