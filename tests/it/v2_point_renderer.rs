//! Authored point-qualified V2 clause identities and exact generated verdict points.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_v2_numeric_strategy, Population, V2NumericErrorCode, V2NumericPolicy,
    V2NumericRequest, V2SelectedClaimKey,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageV2, ExecutionPoint,
    ExecutionPointName,
};

use super::kani_obligations_state_frame::native_twin::Twin;

fn claim<'a>(package: &'a CheckedPackageV2, clause: &CheckedNodeId) -> &'a CheckedOccurrence {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == clause)
        .expect("selected QSL clause")
        .occurrences
        .iter()
        .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .expect("authentic claim")
}

fn request<'a>(
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
    occurrence: &'a CheckedOccurrence,
) -> V2NumericRequest<'a> {
    V2NumericRequest {
        package,
        key: V2SelectedClaimKey {
            clause,
            claim_owner: clause,
            occurrence,
        },
        population: Population::Broad,
        policy: V2NumericPolicy {
            minimum_accepted_cases: 0,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
        },
        process_package_id: None,
    }
}

fn emitted(
    twin: &Twin,
    name: &str,
    point: ExecutionPoint,
) -> (CheckedPackageV2, CheckedNodeId, CheckedOccurrence) {
    let (package, clauses) = twin.emitted_package_with_execution_points(&[(name, point)]);
    let clause = clauses.into_iter().next().expect("selected clause");
    let occurrence = claim(&package, &clause).clone();
    (package, clause, occurrence)
}

fn point_name(name: &str) -> ExecutionPointName {
    ExecutionPointName::new(name).expect("authored point name")
}

/// The point, including its variant and name, comes from the selected QSL clause.
#[trace("TC-020", "FR-011-AC-14", "FR-004-AC-26")]
#[test]
fn distinct_pointed_invariants_render_distinct_verdict_points_and_identities() {
    let twin = Twin::numeric_clause("invariant", "BalanceAtLeastHalf");
    let inputs = [
        (
            ExecutionPoint::Initialization {
                name: point_name("startup"),
            },
            "initialization:7:startup",
        ),
        (
            ExecutionPoint::Handler {
                name: point_name("tick"),
            },
            "handler:4:tick",
        ),
        (
            ExecutionPoint::Handler {
                name: point_name("tock"),
            },
            "handler:4:tock",
        ),
        (
            ExecutionPoint::Handler {
                name: point_name("tïck"),
            },
            "handler:5:tïck",
        ),
    ];
    let mut results = Vec::new();
    for (point, expected) in inputs {
        let (package, clause, occurrence) = emitted(&twin, "BalanceAtLeastHalf", point.clone());
        let generated = generate_v2_numeric_strategy(&request(&package, &clause, &occurrence))
            .expect("pointed invariant generates a numeric runner");
        assert_eq!(
            package.clause_execution_point(&clause, (&clause, &occurrence)),
            Ok(point)
        );
        assert_eq!(generated.identity.execution_point, expected);
        let source = &generated.artifacts.artifacts()[0].contents;
        assert!(source.contains(&format!("ExecutionPoint::new({expected:?})")));
        assert_eq!(generated.identity.clause, clause);
        assert_eq!(generated.identity.package_id, *package.package_id());
        results.push((
            clause,
            package.package_id().clone(),
            generated.identity.execution_point,
        ));
    }
    for pair in results.windows(2) {
        assert_ne!(pair[0].0, pair[1].0, "point changes node identity");
        assert_ne!(pair[0].1, pair[1].1, "point changes package identity");
        assert_ne!(
            pair[0].2, pair[1].2,
            "point changes runtime verdict context"
        );
    }
}

/// Pre and post points preserve their operation names in generated verdict construction.
#[trace("TC-020", "FR-011-AC-14")]
#[test]
fn pre_and_post_claims_render_their_accepted_operation_points() {
    for (kind, point, expected) in [
        (
            "pre",
            ExecutionPoint::Pre {
                operation: point_name("deposit"),
            },
            "pre:7:deposit",
        ),
        (
            "post",
            ExecutionPoint::Post {
                operation: point_name("deposit"),
            },
            "post:7:deposit",
        ),
    ] {
        let twin = Twin::numeric_clause(kind, "BalanceAtLeastHalf");
        let (package, clause, occurrence) = emitted(&twin, "BalanceAtLeastHalf", point);
        let generated = generate_v2_numeric_strategy(&request(&package, &clause, &occurrence))
            .expect("selected operation clause generates a numeric runner");
        assert_eq!(generated.identity.execution_point, expected);
        assert!(generated.artifacts.artifacts()[0]
            .contents
            .contains(&format!("ExecutionPoint::new({expected:?})")));
    }
}

/// Two QSL claims of one content-addressed pointed node select its one point.
#[trace("TC-020", "FR-011-AC-14", "FR-004-AC-26")]
#[test]
fn two_authentic_claims_of_one_pointed_node_share_the_point() {
    let twin = Twin::build(
        &["balance"],
        &[
            ("BalanceFirst", "deposit", "balance"),
            ("BalanceSecond", "deposit", "balance"),
        ],
        0,
    );
    let point = ExecutionPoint::Post {
        operation: point_name("deposit"),
    };
    let (package, clauses) = twin.emitted_package_with_execution_points(&[
        ("BalanceFirst", point.clone()),
        ("BalanceSecond", point),
    ]);
    assert_eq!(clauses.len(), 2);
    assert_eq!(
        clauses[0], clauses[1],
        "equal bodies and points share one node"
    );
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == clauses[0])
        .expect("shared QSL node");
    let claims = node
        .occurrences
        .iter()
        .filter(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .collect::<Vec<_>>();
    assert_eq!(
        claims.len(),
        2,
        "both source claims must survive deduplication"
    );
    assert_ne!(claims[0], claims[1]);
    let generated = claims
        .iter()
        .map(|occurrence| {
            generate_v2_numeric_strategy(&request(&package, &node.node_id, occurrence))
                .expect("authentic claim generates the pointed runner")
        })
        .collect::<Vec<_>>();
    assert_eq!(generated[0].identity.execution_point, "post:7:deposit");
    assert_eq!(generated[1].identity.execution_point, "post:7:deposit");
    assert_ne!(
        generated[0].identity.occurrence,
        generated[1].identity.occurrence
    );
}

/// Changed claim members and an unpointed sibling all refuse before artifact publication.
#[trace("TC-020", "FR-011-AC-15", "FR-004-AC-26")]
#[test]
fn changed_or_unpointed_selection_returns_typed_no_artifact_refusal() {
    let twin = Twin::build(
        &["balance"],
        &[
            ("BalanceNeverDrops", "deposit", "balance"),
            ("TransferBalanceNeverDrops", "transfer", "balance"),
        ],
        0,
    );
    let (package, clauses) = twin.emitted_package_with_execution_points(&[(
        "BalanceNeverDrops",
        ExecutionPoint::Post {
            operation: point_name("deposit"),
        },
    )]);
    let pointed = &clauses[0];
    let authentic = claim(&package, pointed);
    let wrong_role = CheckedOccurrence {
        role: CheckedOccurrenceRole::Expression,
        ordinal: authentic.ordinal,
    };
    let wrong_ordinal = CheckedOccurrence {
        role: CheckedOccurrenceRole::Claim,
        ordinal: authentic.ordinal + 1,
    };
    for occurrence in [&wrong_role, &wrong_ordinal] {
        let diagnostic = generate_v2_numeric_strategy(&request(&package, pointed, occurrence))
            .expect_err("inauthentic occurrence refuses");
        assert_eq!(diagnostic.code, V2NumericErrorCode::MissingClaimOccurrence);
        assert_eq!(diagnostic.occurrence, *occurrence);
    }
    let unpointed = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause" && node.node_id != *pointed)
        .expect("separate unpointed QSL clause");
    let other_claim = claim(&package, &unpointed.node_id);
    let wrong_owner = V2NumericRequest {
        key: V2SelectedClaimKey {
            claim_owner: &unpointed.node_id,
            occurrence: other_claim,
            ..request(&package, pointed, authentic).key
        },
        ..request(&package, pointed, authentic)
    };
    let diagnostic = generate_v2_numeric_strategy(&wrong_owner)
        .expect_err("another node cannot own the selected claim");
    assert_eq!(diagnostic.code, V2NumericErrorCode::MissingClaimOccurrence);
    let diagnostic =
        generate_v2_numeric_strategy(&request(&package, &unpointed.node_id, other_claim))
            .expect_err("unpointed sibling cannot borrow the selected point");
    assert_eq!(diagnostic.code, V2NumericErrorCode::MissingExecutionPoint);
    assert_eq!(diagnostic.clause, unpointed.node_id);
}
