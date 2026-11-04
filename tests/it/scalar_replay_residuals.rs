// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native replay authority residuals; no backend proof or fabricated success receipt.

use super::scalar_playback::{replay_fixture, transcript};
use super::scalar_replay_binding::{context, observe, plan};
use ix_trace_rs::trace;
use qsl_replay::{NativeOutcome, OperatorParityResult};
use quire_contract_codegen::{ScalarBuildError, ScalarSentRequest};
use quire_contract_model::{CheckedOccurrenceRole, CheckedScalarOperandChild};
use quire_exact::Integer;

#[trace("TC-047")]
#[test]
fn each_integer_operator_and_literal_keep_the_public_reader_body_and_real_route() {
    let cases = [
        (
            "quire.op.integer.add",
            "x: Int[0, 9], y: Int[0, 9]",
            "Int[0, 18]",
            "x + y",
            &[2, 7][..],
            9_i64,
        ),
        (
            "quire.op.integer.sub",
            "x: Int[0, 9], y: Int[0, 9]",
            "Int[-9, 9]",
            "x - y",
            &[7, 2][..],
            5,
        ),
        (
            "quire.op.integer.mul",
            "x: Int[0, 9], y: Int[0, 9]",
            "Int[0, 81]",
            "x * y",
            &[2, 7][..],
            14,
        ),
        (
            "quire.op.integer.negate",
            "x: Int[0, 9]",
            "Int[-9, 0]",
            "-x",
            &[3][..],
            -3,
        ),
        (
            "quire.op.integer.add",
            "x: Int[0, 9]",
            "Int[3, 12]",
            "x + 3",
            &[2, 3][..],
            5,
        ),
    ];
    for (operation, parameters, result, expression, values, expected) in cases {
        let fixture = replay_fixture(operation, parameters, result, expression);
        let body = fixture.harness.scalar_replay_body().unwrap();
        assert_eq!(body.operation_identity, operation);
        assert_eq!(body.operands.len(), values.len());
        let original_node = fixture.checked.graph().nodes.iter().find(|node| matches!(&node.body, quire_contract_model::CheckedTerm::Application { operation: entry, .. } if entry.identity() == Some(operation))).expect("independent admitted application");
        assert_eq!(body.node, original_node.node_id);
        assert!(original_node.occurrences.contains(&body.occurrence));
        let independent = fixture
            .checked
            .scalar_application_operands(&original_node.node_id, &body.occurrence)
            .unwrap();
        assert_eq!(
            body.operands, independent,
            "original admitted positional body: {operation}"
        );
        if expression == "x + 3" {
            assert!(
                matches!(&body.operands[1].child, CheckedScalarOperandChild::InlineLiteral { application, occurrence, ordinal: 1 } if application == &body.node && occurrence == &body.occurrence)
            );
            assert_eq!(
                (body.operands[1].range.lower, body.operands[1].range.upper),
                (3, 3)
            );
        } else if values.len() == 2 {
            assert_ne!(
                body.operands[0].child, body.operands[1].child,
                "distinct parameters stay distinct"
            );
        }
        let original = plan(&fixture, values);
        let observation = observe(&fixture, &original, false);
        assert_eq!(
            observation.outcome,
            NativeOutcome::Completed(Integer::from(expected)),
            "independent arithmetic oracle: {expression}"
        );
        let completed = original.complete(observation).unwrap();
        let (wire, claim, limits) = completed.request();
        let sent = ScalarSentRequest {
            wire: wire.clone(),
            claim: claim.clone(),
            replay_limits: limits,
        };
        let report = qsl_replay::replay_operator_parity(wire, claim, limits);
        assert!(
            matches!(report.result(), OperatorParityResult::Agrees { .. }),
            "genuine QSL route: {:?}",
            report.result()
        );
        assert_eq!(
            completed.settle(&sent, Some(&report)).unwrap(),
            report.terminal_value()
        );
    }
}

#[trace("TC-047")]
#[test]
fn source_span_changes_preserve_identity_but_native_results_remain_independently_bound() {
    let first = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
    );
    let shifted = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "\n   x + x",
    );
    let original_body = first.harness.scalar_replay_body().unwrap();
    let shifted_body = shifted.harness.scalar_replay_body().unwrap();
    assert_eq!(
        original_body, shifted_body,
        "source positions are not a new claim identity"
    );
    let regions = |fixture: &super::scalar_playback::ReplayFixture| {
        let body = fixture.harness.scalar_replay_body().unwrap();
        fixture
            .checked
            .source_map()
            .iter()
            .filter(|entry| {
                entry.node_id == body.node && entry.role == CheckedOccurrenceRole::Expression
            })
            .flat_map(|entry| {
                entry
                    .regions
                    .iter()
                    .map(|region| (region.start, region.end))
            })
            .collect::<Vec<_>>()
    };
    assert_ne!(
        regions(&first),
        regions(&shifted),
        "the public reader observes a real span change"
    );
    let original = plan(&first, &[2, 4]);
    let changed_values = plan(&first, &[1, 3]);
    let first_observation = observe(&first, &original, false);
    let second_observation = observe(&first, &changed_values, false);
    assert_eq!(
        first_observation.outcome,
        NativeOutcome::Completed(Integer::from(6_i64))
    );
    assert_eq!(
        second_observation.outcome,
        NativeOutcome::Completed(Integer::from(4_i64))
    );
    let (_, first_claim, _) = original.complete(first_observation).unwrap().request();
    let (_, second_claim, _) = changed_values
        .complete(second_observation)
        .unwrap()
        .request();
    assert_eq!(first_claim.node, second_claim.node);
    assert_eq!(first_claim.occurrence, second_claim.occurrence);
    assert_eq!(first_claim.obligation_kind, second_claim.obligation_kind);
    assert_ne!(first_claim.generated, second_claim.generated);
}

#[trace("TC-047")]
#[test]
fn missing_retained_operands_and_changed_occurrences_refuse_before_replay() {
    let fixture = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
    );
    let authority = context(&fixture);
    let retained = fixture.harness.scalar_replay_body().unwrap();
    let playback = transcript(&fixture.harness.identity, &[2, 4]);
    for mutation in 0..3 {
        let mut changed = retained.clone();
        match mutation {
            0 => changed.operands.clear(),
            1 => changed.occurrence.ordinal += 1,
            2 => changed.occurrence.role = CheckedOccurrenceRole::Claim,
            _ => unreachable!(),
        }
        assert!(
            matches!(
                authority.plan(
                    &fixture.harness,
                    &fixture.harness.identity,
                    &changed,
                    &fixture.artifacts,
                    &playback
                ),
                Err(ScalarBuildError::BodyMismatch)
            ),
            "missing or foreign body authority {mutation}"
        );
    }
}
