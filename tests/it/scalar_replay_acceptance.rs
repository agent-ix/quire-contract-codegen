// SPDX-License-Identifier: AGPL-3.0-or-later
//! Residual acceptance checks over the routed scalar fixtures: setup-step precedence
//! (AC-2), single-member report binding (AC-6), QSL's stale-dependency refusal (AC-17), absent
//! retained body members (AC-18), invented membership refused before comparison (AC-12).
//! Native controls only: no Kani proof or backend falsification.

use super::scalar_playback::{replay_fixture, transcript, ReplayFixture};
use super::scalar_replay_binding::{context, observe, plan};
use ix_trace_rs::trace;
use qsl_replay::{
    Code, Domain, InconclusiveCause, NodeKey, OperandIdentity, OperatorClaim, OperatorParityReport,
    OperatorParityResult, Origin, ParityArgument, ParityPreimage, ReplayLimits, ReplayRefusal,
    ReplayRequestWire, Role, ScalarIdentityMismatch, ScalarOperand, ScalarOperation, TerminalValue,
};
use quire_contract_codegen::{
    CompletedScalarReplay, ScalarAuthorityError, ScalarBuildError, ScalarSentRequest,
    SCALAR_PLAYBACK_BYTES,
};
use quire_exact::{Integer, IntegerInterval};

fn sum_fixture() -> ReplayFixture {
    replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
    )
}

/// A completed `x + x` plan over `[2, 4]` with its genuine native observation.
fn completed_sum(fixture: &ReplayFixture) -> CompletedScalarReplay {
    let original = plan(fixture, &[2, 4]);
    let observation = observe(fixture, &original, false);
    original.complete(observation).unwrap()
}

/// The genuine request that plan sends to the QSL facade.
fn genuine_request(fixture: &ReplayFixture) -> (ReplayRequestWire, OperatorClaim, ReplayLimits) {
    completed_sum(fixture).request()
}

fn operands(operation: &ScalarOperation) -> Vec<&ScalarOperand> {
    match operation {
        ScalarOperation::Add { left, right }
        | ScalarOperation::Subtract { left, right }
        | ScalarOperation::Multiply { left, right } => vec![left, right],
        ScalarOperation::Negate { operand } => vec![operand],
    }
}

/// The obligation identity QSL's public mint gives `claim`, so a mutated claim stays
/// self-consistent with the identity it travels under.
fn remint(wire: &mut ReplayRequestWire, claim: &OperatorClaim) {
    let preimage = ParityPreimage {
        node: claim.node.clone(),
        occurrence: claim.occurrence.clone(),
        obligation_kind: claim.obligation_kind.clone(),
        arguments: operands(&claim.operation)
            .into_iter()
            .map(|operand| ParityArgument {
                identity: operand.identity.clone(),
                domain: Domain::Range(operand.range.clone()),
            })
            .collect(),
    };
    wire.obligation_identity = qsl_replay::parity_obligation(&preimage).expect("encodable");
}

fn playback_code<T: std::fmt::Debug>(result: Result<T, ScalarBuildError>) -> String {
    match result {
        Err(ScalarBuildError::Playback(failure)) => failure.code.to_owned(),
        other => panic!("expected a playback refusal, got {other:?}"),
    }
}

fn scalar_identity(report: &OperatorParityReport) -> &ScalarIdentityMismatch {
    let OperatorParityResult::Refused(refusal) = report.result() else {
        panic!("expected a refusal, got {:?}", report.result())
    };
    let ReplayRefusal::ScalarIdentity(mismatch) = &**refusal else {
        panic!("expected a scalar identity refusal, got {refusal:?}")
    };
    mismatch
}

/// Setup steps refuse in the stated order: context/identity/schema, selected playback bytes,
/// adapter block/harness/arity/width, then operand domain. Each defect alone gives its own
/// refusal; each earlier defect wins over every later one present with it.
#[trace("TC-047", "FR-032-AC-2")]
#[test]
fn setup_refusal_precedence_follows_the_stated_step_order() {
    let fixture = sum_fixture();
    let identity = &fixture.harness.identity;
    let body = fixture.harness.scalar_replay_body().unwrap();
    let run = |artifacts: &[quire_contract_codegen::Artifact], text: &str| {
        context(&fixture).plan(&fixture.harness, identity, body, artifacts, text)
    };
    let good = transcript(identity, &[2, 4]);
    let wrong_harness = good.replace(
        &format!("{}::{}", identity.module_symbol, identity.harness_symbol),
        "other::harness",
    );
    let oversize = format!("{wrong_harness}\n//{}", "x".repeat(SCALAR_PLAYBACK_BYTES));
    let out_of_domain = transcript(identity, &[10, 4]);
    let short_and_out_of_domain = transcript(identity, &[10]);
    let narrow_and_out_of_domain =
        transcript(identity, &[2, 10]).replacen("[2, 0, 0, 0, 0, 0, 0, 0]", "[2, 0, 0, 0]", 1);
    assert!(run(&fixture.artifacts, &good).is_ok(), "control");

    // Each defect alone.
    assert_eq!(
        playback_code(run(&fixture.artifacts, &oversize)),
        "cg_witness_decode_bytes_limit"
    );
    assert_eq!(
        playback_code(run(&fixture.artifacts, &wrong_harness)),
        "cg_witness_harness_identity_mismatch"
    );
    assert_eq!(
        playback_code(run(&fixture.artifacts, &short_and_out_of_domain)),
        "kani_witness_arity_mismatch"
    );
    assert_eq!(
        playback_code(run(&fixture.artifacts, &narrow_and_out_of_domain)),
        "kani_witness_width_mismatch"
    );
    assert_eq!(
        playback_code(run(&fixture.artifacts, &out_of_domain)),
        "cg_scalar_witness_operand_out_of_domain"
    );

    // Playback bytes beat the adapter checks that the same transcript would also fail.
    assert_eq!(
        playback_code(run(&fixture.artifacts, &oversize)),
        "cg_witness_decode_bytes_limit",
        "oversize and wrong harness together"
    );
    // The adapter's checks beat the operand domain.
    assert_eq!(
        playback_code(run(
            &fixture.artifacts,
            &wrong_harness.replace("// 2\n", "// 12\n")
        )),
        "cg_witness_harness_identity_mismatch"
    );

    // Context/identity/schema defects beat every playback defect.
    let mut changed = identity.clone();
    changed.node_id.ordinal += 1;
    assert!(matches!(
        context(&fixture).plan(
            &fixture.harness,
            &changed,
            body,
            &fixture.artifacts,
            &oversize
        ),
        Err(ScalarBuildError::BodyMismatch)
    ));
    let mut changed_artifacts = fixture.artifacts.clone();
    changed_artifacts[0].contents.push(' ');
    assert!(matches!(
        run(&changed_artifacts, &oversize),
        Err(ScalarBuildError::Artifacts)
    ));
}

/// Hold the genuine report fixed per member: every other retained member equal, change exactly
/// one of the obligation, node, kind, operator, operand identity, and require a binding refusal.
#[trace("TC-047", "FR-032-AC-6")]
#[test]
fn each_single_retained_member_change_refuses_the_genuine_report() {
    let fixture = sum_fixture();
    let (wire, claim, limits) = genuine_request(&fixture);
    let sent = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    let completed = completed_sum(&fixture);
    let genuine = qsl_replay::replay_operator_parity(wire.clone(), claim.clone(), limits);
    assert!(completed.settlement(&sent, Some(&genuine)).is_ok());

    let node = |package: &str| {
        NodeKey::new(
            package,
            vec![quire_exact::PathSegment::Text("0".to_owned())],
            0,
        )
        .unwrap()
    };
    for member in 0..5 {
        let mut changed_wire = wire.clone();
        let mut changed = claim.clone();
        match member {
            0 => {
                changed_wire.obligation_identity = qsl_replay::ObligationIdentity::new(
                    claim.node.clone(),
                    None,
                    wire.obligation_identity.ordinal() + 1,
                )
            }
            1 => changed.node = node("other-package"),
            2 => changed.obligation_kind = "other_opaque_kind".to_owned(),
            3 => {
                let ScalarOperation::Add { left, right } = changed.operation.clone() else {
                    panic!("addition")
                };
                changed.operation = ScalarOperation::Subtract { left, right };
            }
            4 => {
                let ScalarOperation::Add { left, .. } = &mut changed.operation else {
                    panic!("addition")
                };
                left.identity = OperandIdentity::GraphChild(claim.node.clone());
            }
            _ => unreachable!(),
        }
        let report = qsl_replay::replay_operator_parity(changed_wire, changed, limits);
        assert_ne!(report.claim(), genuine.claim(), "member {member} differs");
        assert!(
            matches!(
                completed.settlement(&sent, Some(&report)),
                Err(ScalarBuildError::Report)
            ),
            "member {member}"
        );
    }
}

/// QSL's own typed refusal: a claimed package, path or producer ordinal that is not the
/// recompiled package's, or an identity QSL does not recompute, is `stale_dependency` before any
/// scalar comparison. The body-only edit under an equal node is the converter's `BodyMismatch`,
/// asserted in `scalar_replay_binding.rs`.
#[trace("TC-047", "FR-032-AC-17")]
#[test]
fn qsl_refuses_a_changed_node_identity_as_stale_dependency() {
    let fixture = sum_fixture();
    let (wire, claim, limits) = genuine_request(&fixture);
    let control = qsl_replay::replay_operator_parity(wire.clone(), claim.clone(), limits);
    assert!(matches!(
        control.result(),
        OperatorParityResult::Agrees { .. }
    ));

    let source = claim.node.clone();
    let nodes = [
        NodeKey::new("other-package", source.path(), source.ordinal()).unwrap(),
        NodeKey::new(
            source.package(),
            [
                source.path(),
                vec![quire_exact::PathSegment::Text("other".into())],
            ]
            .concat(),
            source.ordinal(),
        )
        .unwrap(),
        NodeKey::new(source.package(), source.path(), source.ordinal() + 1000).unwrap(),
    ];
    for (member, changed_node) in nodes.into_iter().enumerate() {
        let mut changed = claim.clone();
        changed.node = changed_node;
        let report = qsl_replay::replay_operator_parity(wire.clone(), changed, limits);
        assert!(
            matches!(
                scalar_identity(&report),
                ScalarIdentityMismatch::Node { .. }
            ),
            "node member {member}: {:?}",
            report.result()
        );
        assert_eq!(
            report.terminal_value(),
            TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(Code::StaleDependency)),
            "node member {member}"
        );
    }

    let mut stale = wire.clone();
    stale.obligation_identity = qsl_replay::ObligationIdentity::new(
        wire.obligation_identity.node().clone(),
        None,
        wire.obligation_identity.ordinal() + 1,
    );
    let report = qsl_replay::replay_operator_parity(stale, claim, limits);
    assert!(matches!(
        scalar_identity(&report),
        ScalarIdentityMismatch::Obligation { .. }
    ));
    assert_eq!(
        report.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(Code::StaleDependency))
    );
}

/// A retained body or record that lacks its kind or an operand identity gives a typed setup
/// refusal with no plan, no identity and no settlement.
#[trace("TC-047", "FR-032-AC-18")]
#[test]
fn absent_retained_kind_or_operand_identity_refuses_setup() {
    let fixture = sum_fixture();
    let body = fixture.harness.scalar_replay_body().unwrap();
    for member in 0..2 {
        let mut harness = fixture.harness.clone();
        let mut record: serde_json::Value =
            serde_json::from_str(&harness.record.contents).expect("proving record");
        let retained = record
            .get_mut("scalarReplayBody")
            .and_then(serde_json::Value::as_object_mut)
            .expect("the record retains the typed body");
        match member {
            0 => assert!(retained.remove("kind").is_some(), "kind retained"),
            1 => {
                let operand = retained["operands"][0]
                    .as_object_mut()
                    .expect("operand entry");
                assert!(operand.remove("child").is_some(), "operand identity");
            }
            _ => unreachable!(),
        }
        harness.record.contents = record.to_string();
        let refusal = context(&fixture)
            .plan(
                &harness,
                &harness.identity,
                body,
                &fixture.artifacts,
                &transcript(&harness.identity, &[2, 4]),
            )
            .expect_err("absent authority");
        assert!(
            matches!(
                refusal,
                ScalarBuildError::Authority(ScalarAuthorityError::GeneratedArtifactChanged)
            ),
            "member {member}: {refusal:?}"
        );
        assert!(refusal.setup_settlement().is_none());
    }
}

/// A self-consistent claim, reminted under the public owning mint, that invents an occurrence
/// or graph child, or misstates an inline literal's value or singleton range, is refused by
/// QSL's membership checks, never by the identity comparison and never compared.
#[trace("TC-047", "FR-032-AC-12")]
#[test]
fn invented_membership_is_refused_before_identity_comparison() {
    let fixture = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[3, 12]",
        "x + 3",
    );
    let original = plan(&fixture, &[2, 3]);
    let observation = observe(&fixture, &original, false);
    let (wire, claim, limits) = original.complete(observation).unwrap().request();
    let mut control_wire = wire.clone();
    remint(&mut control_wire, &claim);
    assert!(
        matches!(
            qsl_replay::replay_operator_parity(control_wire, claim.clone(), limits).result(),
            OperatorParityResult::Agrees { .. }
        ),
        "the reminted genuine claim is accepted"
    );

    let literal = |claim: &mut OperatorClaim| match &mut claim.operation {
        ScalarOperation::Add { right, .. } => right.clone(),
        other => panic!("addition, got {other:?}"),
    };
    assert!(matches!(
        literal(&mut claim.clone()).identity,
        OperandIdentity::InlineLiteral
    ));
    for member in 0..4 {
        let mut changed = claim.clone();
        match member {
            0 => {
                changed.occurrence =
                    Origin::new(Role::new("expression"), claim.occurrence.ordinal() + 1)
            }
            1 => {
                let ScalarOperation::Add { left, .. } = &mut changed.operation else {
                    panic!("addition")
                };
                left.identity = OperandIdentity::GraphChild(claim.node.clone());
            }
            2 => {
                let ScalarOperation::Add { right, .. } = &mut changed.operation else {
                    panic!("addition")
                };
                right.value = 4;
                right.range = IntegerInterval::spanning(Integer::from(4_i64), Integer::from(4_i64));
            }
            3 => {
                let ScalarOperation::Add { right, .. } = &mut changed.operation else {
                    panic!("addition")
                };
                right.range = IntegerInterval::spanning(Integer::from(3_i64), Integer::from(4_i64));
            }
            _ => unreachable!(),
        }
        let mut changed_wire = wire.clone();
        remint(&mut changed_wire, &changed);
        let report = qsl_replay::replay_operator_parity(changed_wire, changed, limits);
        let mismatch = scalar_identity(&report);
        let expected = match member {
            0 => matches!(mismatch, ScalarIdentityMismatch::Occurrence { .. }),
            1 => matches!(mismatch, ScalarIdentityMismatch::OperandChild { .. }),
            _ => matches!(mismatch, ScalarIdentityMismatch::LiteralValue { .. }),
        };
        assert!(expected, "member {member}: {mismatch:?}");
        assert!(!matches!(
            mismatch,
            ScalarIdentityMismatch::Obligation { .. }
        ));
    }
}
