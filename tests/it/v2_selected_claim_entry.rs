//! Public V2 strategy selection of an admitted state-clause claim.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    select_v2_claim_execution_point, V2SelectedClaimKey, V2SelectedClaimRequest,
};
use quire_contract_model::{
    CheckedExecutionPointRefusal, CheckedOccurrence, CheckedOccurrenceRole, ExecutionPoint,
    ExecutionPointName,
};

use super::kani_obligations_state_frame::native_twin::Twin;

/// The complete key selects the admitted point; every changed key member is refused.
#[trace("TC-020", "FR-011-AC-13")]
#[test]
fn v2_selected_claim_entry_keeps_point_and_exact_refusal_key() {
    let twin = Twin::build(
        &["balance"],
        &[
            ("BalanceNeverDrops", "deposit", "balance"),
            ("TransferBalanceNeverDrops", "transfer", "balance"),
        ],
        0,
    );
    let (package, deposit_clause) = twin.emitted_package("BalanceNeverDrops");
    let (second_package, transfer_clause) = twin.emitted_package("TransferBalanceNeverDrops");
    assert_eq!(package.package_id(), second_package.package_id());

    let deposit = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == deposit_clause)
        .expect("deposit clause in package");
    let transfer = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == transfer_clause)
        .expect("transfer clause in package");
    let deposit_claim = deposit
        .occurrences
        .iter()
        .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .expect("authentic deposit claim");
    let transfer_claim = transfer
        .occurrences
        .iter()
        .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .expect("authentic transfer claim");

    let deposit_point = select_v2_claim_execution_point(V2SelectedClaimRequest {
        package: &package,
        key: V2SelectedClaimKey {
            clause: &deposit_clause,
            claim_owner: &deposit_clause,
            occurrence: deposit_claim,
        },
    })
    .expect("authentic deposit claim selects its point");
    let transfer_point = select_v2_claim_execution_point(V2SelectedClaimRequest {
        package: &package,
        key: V2SelectedClaimKey {
            clause: &transfer_clause,
            claim_owner: &transfer_clause,
            occurrence: transfer_claim,
        },
    })
    .expect("authentic transfer claim selects its point");
    assert_eq!(
        deposit_point,
        ExecutionPoint::Post {
            operation: ExecutionPointName::new("deposit").expect("point name"),
        }
    );
    assert_eq!(
        transfer_point,
        ExecutionPoint::Post {
            operation: ExecutionPointName::new("transfer").expect("point name"),
        }
    );
    assert_ne!(deposit_point, transfer_point);

    let wrong_role = CheckedOccurrence {
        role: CheckedOccurrenceRole::Expression,
        ordinal: deposit_claim.ordinal,
    };
    let absent_ordinal = CheckedOccurrence {
        role: CheckedOccurrenceRole::Claim,
        ordinal: deposit_claim.ordinal + 1,
    };
    for (owner, occurrence) in [
        (&deposit_clause, &wrong_role),
        (&deposit_clause, &absent_ordinal),
        (&transfer_clause, deposit_claim),
    ] {
        let diagnostic = select_v2_claim_execution_point(V2SelectedClaimRequest {
            package: &package,
            key: V2SelectedClaimKey {
                clause: &deposit_clause,
                claim_owner: owner,
                occurrence,
            },
        })
        .expect_err("changed key must refuse without an artifact");
        assert_eq!(
            diagnostic.refusal.code,
            CheckedExecutionPointRefusal::MissingClaimOccurrence
        );
        assert_eq!(diagnostic.refusal.clause, deposit_clause);
        assert_eq!(diagnostic.refusal.claim_owner, *owner);
        assert_eq!(diagnostic.refusal.claim, *occurrence);
    }
}
