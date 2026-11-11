//! FR-015-AC-48: a V2 clause claim carries QSL's producer-issued `ObligationIdentity` unchanged
//! and is cross-checked against the independently selected clause before any harness.

use ix_trace_rs::trace;
use qsl_replay::{ModelIdentity, ObligationIdentity};
use quire_contract_codegen::{
    negotiate_kani_native_obligations, negotiate_kani_obligations, KaniObligationOutcome,
    KaniObligationRequest, KaniV2ClauseObligationHarness, ObligationDisposition, ObligationItem,
    ObligationKind, ScalarObligationArgument, UnsupportedObligation, V2ClauseClaimField,
    V2ClauseObligationClaim, V2ClauseStateField,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use super::kani_obligations_state_frame::native_twin::Twin;
use crate::common::v2_claim::claim_for;

fn fixtures() -> Vec<(CheckedPackageV2, CheckedNodeId, ObligationKind)> {
    [
        (
            Twin::with_bounded_other_precondition(),
            "OtherBelowLimit",
            ObligationKind::Precondition,
        ),
        (
            Twin::with_bounded_amount_implication(),
            "BalanceNeverDrops",
            ObligationKind::Postcondition,
        ),
        (
            Twin::with_balance_invariant(),
            "BalanceNonnegative",
            ObligationKind::Invariant,
        ),
    ]
    .into_iter()
    .map(|(twin, name, kind)| {
        let (package, clause) = twin.emitted_package(name);
        (package, clause, kind)
    })
    .collect()
}

fn generate(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    claim: &V2ClauseObligationClaim,
    kind: ObligationKind,
    unwind: u32,
    subject_path: &str,
) -> KaniObligationOutcome {
    let item = ObligationItem::V2Clause {
        package,
        clause,
        claim,
        dependencies: None,
    };
    let request = KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: std::slice::from_ref(&item),
        subject_path,
        unwind,
    };
    if kind == ObligationKind::Precondition {
        negotiate_kani_obligations(&request)
    } else {
        negotiate_kani_native_obligations(&request)
    }
    .expect("a well-formed request")
}

fn harness(outcome: KaniObligationOutcome) -> KaniV2ClauseObligationHarness {
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the matching claim emits");
    };
    let [harness] = <[_; 1]>::try_from(v2_clause_harnesses).expect("one harness");
    harness
}

fn with_obligation(
    claim: &V2ClauseObligationClaim,
    obligation: ObligationIdentity,
) -> V2ClauseObligationClaim {
    V2ClauseObligationClaim {
        obligation,
        record: claim.record.clone(),
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn producer_identity_is_carried_unchanged_and_each_member_is_distinct() {
    for (package, clause, kind) in fixtures() {
        let base = claim_for(&package, &clause);
        let node = base.obligation.node().clone();
        let model = ModelIdentity::new("model-a");
        let variants = [
            ObligationIdentity::new(node.clone(), None, 0),
            ObligationIdentity::new(node.clone(), None, 1),
            ObligationIdentity::new(node.clone(), Some(model.clone()), 0),
            ObligationIdentity::new(node.clone(), Some(ModelIdentity::new("model-b")), 0),
            ObligationIdentity::new(node.clone(), Some(model), 1),
        ];
        let carried = variants
            .iter()
            .map(|identity| {
                let claim = with_obligation(&base, identity.clone());
                harness(generate(
                    &package,
                    &clause,
                    &claim,
                    kind,
                    3,
                    "crate::subject::deposit",
                ))
                .obligation
            })
            .collect::<Vec<_>>();
        assert_eq!(carried, variants, "{kind:?}: carried unchanged");
        for (index, left) in carried.iter().enumerate() {
            for right in &carried[index + 1..] {
                assert_ne!(
                    left, right,
                    "{kind:?}: a changed member is a different identity"
                );
            }
        }
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn span_unwind_and_subject_leave_the_identity_and_the_ordinal_leaves_the_harness_record() {
    for (package, clause, kind) in fixtures() {
        let claim = claim_for(&package, &clause);
        let first = harness(generate(
            &package,
            &clause,
            &claim,
            kind,
            3,
            "crate::subject::deposit",
        ));
        let unwind = harness(generate(
            &package,
            &clause,
            &claim,
            kind,
            4,
            "crate::subject::deposit",
        ));
        let subject = harness(generate(
            &package,
            &clause,
            &claim,
            kind,
            3,
            "crate::subject::other",
        ));
        assert_eq!(first.obligation, unwind.obligation, "{kind:?}");
        assert_eq!(first.obligation, subject.obligation, "{kind:?}");
        assert_ne!(
            first.identity, unwind.identity,
            "{kind:?}: harness identity tracks unwind"
        );
        let reordinal = with_obligation(
            &claim,
            ObligationIdentity::new(claim.obligation.node().clone(), None, 7),
        );
        let moved = harness(generate(
            &package,
            &clause,
            &reordinal,
            kind,
            3,
            "crate::subject::deposit",
        ));
        assert_ne!(first.obligation, moved.obligation, "{kind:?}");
        assert_eq!(
            first.record.contents, moved.record.contents,
            "{kind:?}: the harness identity record is distinct from the producer identity"
        );
        assert_eq!(first.identity, moved.identity, "{kind:?}");
    }
}

fn mismatch_of(outcome: &KaniObligationOutcome) -> Option<V2ClauseClaimField> {
    match &outcome.records()[0].disposition {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseClaimMismatch { field, .. },
        } => Some(*field),
        _ => None,
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn a_record_mismatch_refuses_before_any_harness_with_the_identity_unchanged() {
    for (package, clause, kind) in fixtures() {
        let claim = claim_for(&package, &clause);
        let control = generate(
            &package,
            &clause,
            &claim,
            kind,
            3,
            "crate::subject::deposit",
        );
        assert_eq!(
            mismatch_of(&control),
            None,
            "{kind:?}: the matching claim is admitted"
        );
        assert!(matches!(control, KaniObligationOutcome::Emitted { .. }));
        let mut occurrence = claim.clone();
        occurrence.record.occurrence.ordinal += 1;
        let mut wrong_kind = claim.clone();
        wrong_kind.record.kind = if kind == ObligationKind::Invariant {
            ObligationKind::Precondition
        } else {
            ObligationKind::Invariant
        };
        let mut extra_argument = claim.clone();
        extra_argument
            .record
            .arguments
            .push(ScalarObligationArgument {
                identifier: "parameter_absent".to_owned(),
                minimum: 0,
                maximum: 1,
            });
        let mut extra_read = claim.clone();
        extra_read.record.state_reads.push(V2ClauseStateField {
            declaring_node_id: clause.clone(),
            field_name: "absent".to_owned(),
            identifier: "current_absent".to_owned(),
            minimum: 0,
            maximum: 1,
        });
        let mut cases = vec![
            (occurrence, V2ClauseClaimField::Occurrence),
            (wrong_kind, V2ClauseClaimField::Kind),
            (extra_argument, V2ClauseClaimField::Arguments),
            (extra_read, V2ClauseClaimField::StateReads),
        ];
        if let Some(first) = claim.record.arguments.first() {
            let mut widened = claim.clone();
            widened.record.arguments[0] = ScalarObligationArgument {
                maximum: first.maximum + 1,
                ..first.clone()
            };
            cases.push((widened, V2ClauseClaimField::Arguments));
        }
        if let Some(first) = claim.record.state_reads.first() {
            let mut widened = claim.clone();
            widened.record.state_reads[0] = V2ClauseStateField {
                minimum: first.minimum - 1,
                ..first.clone()
            };
            cases.push((widened, V2ClauseClaimField::StateReads));
        }
        for (changed, field) in cases {
            assert_eq!(
                changed.obligation, claim.obligation,
                "structural identity unchanged"
            );
            let outcome = generate(
                &package,
                &clause,
                &changed,
                kind,
                3,
                "crate::subject::deposit",
            );
            assert_eq!(mismatch_of(&outcome), Some(field), "{kind:?}");
            assert!(
                matches!(&outcome, KaniObligationOutcome::Emitted { v2_clause_harnesses, .. }
                    if v2_clause_harnesses.is_empty())
                    || matches!(outcome, KaniObligationOutcome::Rejected { .. }),
                "{kind:?}: no harness for {field:?}"
            );
        }
    }
}
