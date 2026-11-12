// SPDX-License-Identifier: AGPL-3.0-or-later
//! Residual acceptance checks over the routed scalar fixtures: setup-step precedence
//! (AC-2), single-member report binding (AC-6), QSL's stale-dependency refusal (AC-17), absent
//! retained body members (AC-18), invented membership refused before comparison (AC-12).
//! Native controls only: no Kani proof or backend falsification.

use super::kani_obligations::boolean_implies_scalar;
use super::scalar_playback::{replay_fixture, routed_implies_fixture, transcript, ReplayFixture};
use super::scalar_replay_binding::{context, observe, plan};
use super::terminal_map::synthetic_evidence;
use ix_trace_rs::trace;
use qsl_replay::{
    Code, Domain, InconclusiveCause, NodeKey, OperandIdentity, OperatorClaim, OperatorParityReport,
    OperatorParityResult, Origin, ParityArgument, ParityPreimage, ReplayLimits, ReplayRefusal,
    ReplayRequestWire, Role, ScalarIdentityMismatch, ScalarOperand, ScalarOperation, TerminalValue,
};
use quire_contract_codegen::{
    scalar_run_terminal_value, CompletedScalarReplay, KaniRunOutcome, ScalarAuthorityError,
    ScalarBuildError, ScalarSentRequest, TerminalPairError, SCALAR_PLAYBACK_BYTES,
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
        Err(ScalarBuildError::UnavailableFamily(cause)) => playback_code::<T>(Err(*cause)),
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
/// adapter block/harness/arity/width/comment, operand domain, then capability availability.
/// Each defect alone gives its own refusal; each earlier defect wins over a later one present
/// with it; an absent capability refuses only when nothing earlier is wrong.
#[trace("TC-047", "FR-032-AC-1", "FR-032-AC-2")]
#[test]
fn setup_refusal_precedence_follows_the_stated_step_order() {
    let fixture = sum_fixture();
    let identity = &fixture.harness.identity;
    let body = fixture.harness.scalar_replay_body().unwrap();
    let run = |artifacts: &[quire_contract_codegen::Artifact], text: &str| {
        context(&fixture).plan(&fixture.harness, identity, body, artifacts, text)
    };
    let rename = |text: &str| {
        text.replace(
            &format!("{}::{}", identity.module_symbol, identity.harness_symbol),
            "other::harness",
        )
    };
    let pad = |text: &str| format!("{text}\n//{}", "x".repeat(SCALAR_PLAYBACK_BYTES));
    let good = transcript(identity, &[2, 4]);
    let oversize = pad(&good);
    let wrong_harness = rename(&good);
    let short = transcript(identity, &[10]);
    let long = transcript(identity, &[2, 4, 4]);
    let narrow =
        transcript(identity, &[2, 10]).replacen("[2, 0, 0, 0, 0, 0, 0, 0]", "[2, 0, 0, 0]", 1);
    let comment = transcript(identity, &[12, 4]).replacen("// 12\n", "// 2\n", 1);
    let out_of_domain = transcript(identity, &[10, 4]);
    assert!(run(&fixture.artifacts, &good).is_ok(), "control");

    // Each defect alone.
    let alone = [
        (&oversize, "cg_witness_decode_bytes_limit"),
        (&wrong_harness, "cg_witness_harness_identity_mismatch"),
        (&short, "kani_witness_arity_mismatch"),
        (&long, "cg_witness_decode_values_limit"),
        (&narrow, "kani_witness_width_mismatch"),
        (&comment, "kani_witness_comment_mismatch"),
        (&out_of_domain, "cg_scalar_witness_operand_out_of_domain"),
    ];
    for (text, code) in alone {
        assert_eq!(playback_code(run(&fixture.artifacts, text)), code);
    }

    // Each earlier playback defect wins over every later one present with it.
    let pairs = [
        (
            pad(&rename(&out_of_domain)),
            "cg_witness_decode_bytes_limit",
        ),
        (
            rename(&out_of_domain),
            "cg_witness_harness_identity_mismatch",
        ),
        (
            rename(&transcript(identity, &[10])),
            "cg_witness_harness_identity_mismatch",
        ),
        (
            transcript(identity, &[12, 10]).replacen("// 12\n", "// 2\n", 1),
            "kani_witness_comment_mismatch",
        ),
    ];
    for (text, code) in &pairs {
        assert_eq!(playback_code(run(&fixture.artifacts, text)), *code);
    }

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

    // Step 5: an absent capability (a routed Boolean `Implies` harness, which binds its own
    // artifact package) never hides an earlier refusal and is the refusal when nothing
    // earlier is wrong.
    let implies = routed_implies_fixture();
    let implies_identity = &implies.harness.identity;
    let unavailable = |artifacts: &[quire_contract_codegen::Artifact], text: &str| {
        context(&fixture).plan(&implies.harness, implies_identity, body, artifacts, text)
    };
    let implies_good = transcript(implies_identity, &[1, 1]);
    assert!(matches!(
        unavailable(&implies.artifacts, &implies_good),
        Err(ScalarBuildError::Authority(
            ScalarAuthorityError::Unavailable
        ))
    ));
    assert_eq!(
        playback_code(unavailable(&implies.artifacts, &pad(&implies_good))),
        "cg_witness_decode_bytes_limit"
    );
    assert_eq!(
        playback_code(unavailable(
            &implies.artifacts,
            &rename_implies(implies_identity, &implies_good)
        )),
        "cg_witness_harness_identity_mismatch"
    );
    assert!(
        matches!(
            unavailable(&implies.artifacts, "garbage"),
            Err(ScalarBuildError::UnavailableFamily(cause)) if matches!(*cause, ScalarBuildError::Playback(_))
        ),
        "a malformed block is an adapter refusal, not an absent capability"
    );
    let mut changed_implies = implies_identity.clone();
    changed_implies.node_id.ordinal += 1;
    assert!(matches!(
        context(&fixture).plan(
            &implies.harness,
            &changed_implies,
            body,
            &implies.artifacts,
            &implies_good
        ),
        Err(ScalarBuildError::UnavailableFamily(cause)) if matches!(*cause, ScalarBuildError::BodyMismatch)
    ));
    let mut changed_artifacts = implies.artifacts.clone();
    changed_artifacts[0].contents.push(' ');
    assert!(
        matches!(
            unavailable(&changed_artifacts, &pad(&implies_good)),
            Err(ScalarBuildError::UnavailableFamily(cause)) if matches!(*cause, ScalarBuildError::Artifacts)
        ),
        "changed artifacts of a routed harness refuse before the playback"
    );
}

/// A refusal on a family with no operator-parity route never settles: a falsified run of the
/// Boolean `Implies` harness yields no terminal record, whatever the playback is.
#[trace("TC-047", "FR-032-AC-24")]
#[test]
fn every_refusal_on_an_unavailable_family_settles_nothing() {
    let fixture = sum_fixture();
    let body = fixture.harness.scalar_replay_body().unwrap();
    let routed = routed_implies_fixture();
    let (unrouted, _, _) = boolean_implies_scalar();
    let falsified = synthetic_evidence(
        KaniRunOutcome::Falsified {
            counterexample: "#[test] fn kani_concrete_playback() {}".to_owned(),
        },
        0,
    );
    for (label, harness, artifacts) in [
        ("routed", &routed.harness, &routed.artifacts),
        ("unrouted", &unrouted, &fixture.artifacts),
    ] {
        let identity = &harness.identity;
        let good = transcript(identity, &[1, 1]);
        let oversize = format!("{good}\n//{}", "x".repeat(SCALAR_PLAYBACK_BYTES));
        let wrong = rename_implies(identity, &good);
        for text in [
            good.as_str(),
            oversize.as_str(),
            wrong.as_str(),
            "garbage",
            "",
        ] {
            let refusal = context(&fixture)
                .plan(harness, identity, body, artifacts, text)
                .expect_err("no plan for a family without a parity route");
            assert!(refusal.setup_settlement().is_none(), "{label}: {refusal:?}");
            assert_eq!(
                scalar_run_terminal_value(&falsified, refusal.setup_settlement()),
                Err(TerminalPairError::MissingSettlement),
                "{label}: {refusal:?}"
            );
        }
    }
}

fn rename_implies(
    identity: &quire_contract_codegen::ScalarObligationIdentity,
    text: &str,
) -> String {
    text.replace(
        &format!("{}::{}", identity.module_symbol, identity.harness_symbol),
        "other::harness",
    )
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

/// A caller body that lacks a retained operand is refused against the retained body, and a
/// retained record whose typed body lost its kind or an operand identity is refused by the
/// generator seal. The record check shows the seal only: `ScalarReplayBody.kind` is a typed
/// single-variant field and the record's `kind` text is never read for authority, so an absent
/// kind is not otherwise observable.
#[trace("TC-047", "FR-032-AC-18")]
#[test]
fn a_body_or_record_missing_a_retained_member_refuses_setup() {
    let fixture = sum_fixture();
    let body = fixture.harness.scalar_replay_body().unwrap();
    let playback = transcript(&fixture.harness.identity, &[2, 4]);
    for member in 0..2 {
        let mut missing = body.clone();
        match member {
            0 => missing.operands.clear(),
            1 => {
                missing.operands.pop();
            }
            _ => unreachable!(),
        }
        let refusal = context(&fixture)
            .plan(
                &fixture.harness,
                &fixture.harness.identity,
                &missing,
                &fixture.artifacts,
                &playback,
            )
            .expect_err("missing operand");
        assert!(
            matches!(refusal, ScalarBuildError::BodyMismatch),
            "{refusal:?}"
        );
        assert!(refusal.setup_settlement().is_none());
    }
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
            .expect_err("sealed record");
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

/// Operands are retained in draw order, not identifier order, and an enclosing parameter the
/// application does not read contributes no position.
#[trace("TC-047", "FR-032-AC-15")]
#[test]
fn draw_order_is_kept_and_unrelated_enclosing_parameters_are_excluded() {
    let fixture = replay_fixture(
        "quire.op.integer.sub",
        "x: Int[10, 20], y: Int[0, 9], z: Int[50, 60]",
        "Int[-20, -1]",
        "y - x",
    );
    let body = fixture.harness.scalar_replay_body().unwrap();
    let identity = &fixture.harness.identity;
    assert_eq!(
        identity.arguments.len(),
        2,
        "the unused parameter z is not drawn"
    );
    let ranges: Vec<_> = body
        .operands
        .iter()
        .map(|operand| (operand.range.lower, operand.range.upper))
        .collect();
    assert_eq!(
        ranges,
        [(0, 9), (10, 20)],
        "draw order y then x, not declaration or identifier order; z absent"
    );
    let admitted = fixture
        .checked
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.node_id == body.node)
        .unwrap();
    assert_eq!(
        body.operands,
        fixture
            .checked
            .scalar_application_operands(&admitted.node_id, &body.occurrence)
            .unwrap(),
        "the body is the IR reader's positional body"
    );
    let original = plan(&fixture, &[3, 15]);
    assert_eq!(original.operands(), [3, 15]);
    let observation = observe(&fixture, &original, false);
    assert_eq!(
        observation.outcome,
        qsl_replay::NativeOutcome::Completed(Integer::from(-12_i64))
    );
    let (wire, claim, limits) = original.complete(observation).unwrap().request();
    let ScalarOperation::Subtract { left, right } = &claim.operation else {
        panic!("subtraction")
    };
    assert_eq!((left.value, right.value), (3, 15));
    assert!(matches!(
        qsl_replay::replay_operator_parity(wire, claim, limits).result(),
        OperatorParityResult::Agrees { .. }
    ));
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
