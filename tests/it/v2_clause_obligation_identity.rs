//! FR-015-AC-48: a V2 clause claim carries QSL's producer-issued `ObligationIdentity` unchanged
//! and its separately stated obligation record is cross-checked against the independently
//! selected clause before any harness is generated.
//!
//! Every record here is written by hand from the fixture source. None is read back from the
//! generator, so a cross-check that ignored the clause would fail these tests.

use ix_trace_rs::trace;
use qsl_replay::{ModelIdentity, ObligationIdentity};
use quire_contract_codegen::{
    negotiate_kani_native_obligations, negotiate_kani_obligations, KaniObligationOutcome,
    KaniObligationRequest, KaniV2ClauseObligationHarness, ObligationDisposition, ObligationItem,
    ObligationKind, RequirementOccurrence, UnsupportedObligation, V2ClauseClaimField,
    V2ClauseObligationClaim, V2ClauseObligationRecord, V2ClauseParameter, V2ClauseRead,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use quire_exact::{NodeKey, PathSegment};

use super::kani_obligations_state_frame::native_twin::Twin;

/// The account object type that declares `balance` and `audit` (range `0..=1000`).
fn account_declaration(package: &CheckedPackageV2) -> CheckedNodeId {
    package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| &*node.semantic_form == "object_type")
        .expect("the account object declaration")
        .node_id
        .clone()
}

fn occurrence(package: &CheckedPackageV2, clause: &CheckedNodeId) -> RequirementOccurrence {
    let entry = package
        .source_map()
        .iter()
        .find(|entry| &entry.node_id == clause)
        .expect("the clause has a source occurrence");
    RequirementOccurrence {
        node_id: entry.node_id.clone(),
        role: entry.role.clone(),
        ordinal: entry.ordinal,
    }
}

fn parameter(name: &str, minimum: i64, maximum: i64) -> V2ClauseParameter {
    V2ClauseParameter {
        name: name.to_owned(),
        minimum,
        maximum,
    }
}

fn read(declaring: &CheckedNodeId, field: &str, minimum: i64, maximum: i64) -> V2ClauseRead {
    V2ClauseRead {
        declaring_node_id: declaring.clone(),
        field_name: field.to_owned(),
        minimum,
        maximum,
        element: None,
    }
}

struct Fixture {
    package: CheckedPackageV2,
    clause: CheckedNodeId,
    kind: ObligationKind,
    /// The claim a correct independent producer states for this clause.
    claim: V2ClauseObligationClaim,
}

fn claim_of(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    kind: ObligationKind,
    parameters: Vec<V2ClauseParameter>,
    state_reads: Vec<V2ClauseRead>,
) -> V2ClauseObligationClaim {
    let node = crate::common::v2_claim::resolved_key(package, clause);
    V2ClauseObligationClaim {
        obligation: ObligationIdentity::new(node, None, 0),
        record: V2ClauseObligationRecord {
            occurrence: occurrence(package, clause),
            kind,
            parameters,
            state_reads,
        },
    }
}

fn fixture(twin: Twin, name: &str, kind: ObligationKind) -> Fixture {
    let (package, clause) = twin.emitted_package(name);
    let account = account_declaration(&package);
    // The sources: `pre OtherBelowLimit { other < 1000 }` over parameter `other`;
    // `post ... { result = (amount < 1000 implies ...) }` over parameter `amount`; and
    // `invariant ... at current { self.balance >= 0 }` reading `balance`. Each is declared
    // over the `0..=1000` range.
    let (parameters, state_reads) = match kind {
        ObligationKind::Precondition => (vec![parameter("other", 0, 1000)], Vec::new()),
        ObligationKind::Postcondition => (vec![parameter("amount", 0, 1000)], Vec::new()),
        ObligationKind::Invariant => (Vec::new(), vec![read(&account, "balance", 0, 1000)]),
        ObligationKind::Frame => unreachable!("no frame fixture"),
    };
    let claim = claim_of(&package, &clause, kind, parameters, state_reads);
    Fixture {
        package,
        clause,
        kind,
        claim,
    }
}

fn parameter_fixtures() -> Vec<Fixture> {
    vec![
        fixture(
            Twin::with_bounded_other_precondition(),
            "OtherBelowLimit",
            ObligationKind::Precondition,
        ),
        fixture(
            Twin::with_bounded_amount_implication(),
            "BalanceNeverDrops",
            ObligationKind::Postcondition,
        ),
    ]
}

fn read_fixture() -> Fixture {
    fixture(
        Twin::with_balance_invariant(),
        "BalanceNonnegative",
        ObligationKind::Invariant,
    )
}

fn generate(
    fixture: &Fixture,
    claim: &V2ClauseObligationClaim,
    unwind: u32,
    subject_path: &str,
) -> KaniObligationOutcome {
    let item = ObligationItem::V2Clause {
        package: &fixture.package,
        clause: &fixture.clause,
        claim,
        dependencies: None,
    };
    let request = KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: std::slice::from_ref(&item),
        subject_path,
        unwind,
    };
    if fixture.kind == ObligationKind::Precondition {
        negotiate_kani_obligations(&request)
    } else {
        negotiate_kani_native_obligations(&request)
    }
    .expect("a well-formed request")
}

fn harness(outcome: KaniObligationOutcome) -> KaniV2ClauseObligationHarness {
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        records,
        ..
    } = outcome
    else {
        panic!("the matching claim emits");
    };
    <[_; 1]>::try_from(v2_clause_harnesses)
        .unwrap_or_else(|_| panic!("one harness: {records:?}"))
        .into_iter()
        .next()
        .expect("one harness")
}

fn mismatch_of(outcome: &KaniObligationOutcome) -> Option<V2ClauseClaimField> {
    match &outcome.records()[0].disposition {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseClaimMismatch { field, .. },
        } => Some(*field),
        _ => None,
    }
}

fn no_harness(outcome: &KaniObligationOutcome) -> bool {
    match outcome {
        KaniObligationOutcome::Emitted {
            v2_clause_harnesses,
            collection_harnesses,
            ..
        } => v2_clause_harnesses.is_empty() && collection_harnesses.is_empty(),
        KaniObligationOutcome::Rejected { .. } => true,
    }
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
    let mut all = parameter_fixtures();
    all.push(read_fixture());
    for fixture in all {
        let node = fixture.claim.obligation.node().clone();
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
                let claim = with_obligation(&fixture.claim, identity.clone());
                harness(generate(&fixture, &claim, 3, "crate::subject::deposit")).obligation
            })
            .collect::<Vec<_>>();
        assert_eq!(carried, variants, "{:?}: carried unchanged", fixture.kind);
        for (index, left) in carried.iter().enumerate() {
            for right in &carried[index + 1..] {
                assert_ne!(left, right, "{:?}: changed member", fixture.kind);
            }
        }
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn unwind_subject_and_ordinal_leave_the_other_identity_alone() {
    let mut all = parameter_fixtures();
    all.push(read_fixture());
    for fixture in all {
        let kind = fixture.kind;
        let claim = &fixture.claim;
        let first = harness(generate(&fixture, claim, 3, "crate::subject::deposit"));
        let unwind = harness(generate(&fixture, claim, 4, "crate::subject::deposit"));
        let subject = harness(generate(&fixture, claim, 3, "crate::subject::other"));
        assert_eq!(first.obligation, unwind.obligation, "{kind:?}");
        assert_eq!(first.obligation, subject.obligation, "{kind:?}");
        assert_ne!(first.identity, unwind.identity, "{kind:?}: tracks unwind");
        let reordinal = with_obligation(
            claim,
            ObligationIdentity::new(claim.obligation.node().clone(), None, 7),
        );
        let moved = harness(generate(&fixture, &reordinal, 3, "crate::subject::deposit"));
        assert_ne!(first.obligation, moved.obligation, "{kind:?}");
        assert_eq!(first.record.contents, moved.record.contents, "{kind:?}");
        assert_eq!(first.identity, moved.identity, "{kind:?}");
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn a_changed_source_span_alone_changes_neither_the_identity_nor_the_record() {
    let plain = fixture(
        Twin::with_bounded_other_precondition(),
        "OtherBelowLimit",
        ObligationKind::Precondition,
    );
    let shifted = fixture(
        Twin::with_bounded_other_precondition().with_blank_lines_before("pre OtherBelowLimit", 3),
        "OtherBelowLimit",
        ObligationKind::Precondition,
    );
    let regions = |fixture: &Fixture| {
        fixture
            .package
            .source_map()
            .iter()
            .find(|entry| entry.node_id == fixture.clause)
            .expect("clause occurrence")
            .regions
            .clone()
    };
    assert_ne!(regions(&plain), regions(&shifted), "the span moved");
    assert_eq!(
        plain.claim.record, shifted.claim.record,
        "occurrence unchanged"
    );
    let before = harness(generate(&plain, &plain.claim, 3, "crate::subject::deposit"));
    let after = harness(generate(
        &shifted,
        &plain.claim,
        3,
        "crate::subject::deposit",
    ));
    assert_eq!(before.obligation, after.obligation);
    assert_eq!(before.identity, after.identity);
    assert_eq!(before.record.contents, after.record.contents);
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn a_correct_claim_is_accepted_whatever_identifiers_the_harness_generates() {
    for fixture in parameter_fixtures() {
        assert!(
            !fixture.claim.record.parameters.is_empty(),
            "{:?}",
            fixture.kind
        );
        let emitted = harness(generate(
            &fixture,
            &fixture.claim,
            3,
            "crate::subject::deposit",
        ));
        let claimed = &fixture.claim.record.parameters[0].name;
        assert!(
            emitted
                .identity
                .arguments
                .iter()
                .all(|argument| &argument.identifier != claimed),
            "{:?}: the generated identifier differs from the declared name",
            fixture.kind
        );
    }
    let invariant = read_fixture();
    assert!(!invariant.claim.record.state_reads.is_empty());
    let emitted = harness(generate(
        &invariant,
        &invariant.claim,
        3,
        "crate::subject::deposit",
    ));
    assert!(emitted
        .identity
        .state_fields
        .iter()
        .all(|field| field.identifier != "balance"));
}

fn refuses(fixture: &Fixture, changed: &V2ClauseObligationClaim, field: V2ClauseClaimField) {
    assert_eq!(
        changed.obligation, fixture.claim.obligation,
        "identity fixed"
    );
    let outcome = generate(fixture, changed, 3, "crate::subject::deposit");
    assert_eq!(mismatch_of(&outcome), Some(field), "{:?}", fixture.kind);
    assert!(
        no_harness(&outcome),
        "{:?}: {field:?} emitted",
        fixture.kind
    );
}

fn common_mismatches(fixture: &Fixture) {
    let mut occurrence = fixture.claim.clone();
    occurrence.record.occurrence.ordinal += 1;
    refuses(fixture, &occurrence, V2ClauseClaimField::Occurrence);
    let mut wrong_kind = fixture.claim.clone();
    wrong_kind.record.kind = if fixture.kind == ObligationKind::Invariant {
        ObligationKind::Precondition
    } else {
        ObligationKind::Invariant
    };
    refuses(fixture, &wrong_kind, V2ClauseClaimField::Kind);
    let mut extra_parameter = fixture.claim.clone();
    extra_parameter
        .record
        .parameters
        .push(parameter("absent", 0, 1));
    refuses(fixture, &extra_parameter, V2ClauseClaimField::Parameters);
    let mut extra_read = fixture.claim.clone();
    extra_read
        .record
        .state_reads
        .push(read(&fixture.clause, "absent", 0, 1));
    refuses(fixture, &extra_read, V2ClauseClaimField::StateReads);
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn a_parameter_mismatch_refuses_before_any_harness() {
    for fixture in parameter_fixtures() {
        common_mismatches(&fixture);
        let first = fixture.claim.record.parameters[0].clone();
        let mut widened = fixture.claim.clone();
        widened.record.parameters[0].maximum = first.maximum + 1;
        refuses(&fixture, &widened, V2ClauseClaimField::Parameters);
        let mut narrowed = fixture.claim.clone();
        narrowed.record.parameters[0].minimum = first.minimum + 1;
        refuses(&fixture, &narrowed, V2ClauseClaimField::Parameters);
        let mut renamed = fixture.claim.clone();
        renamed.record.parameters[0].name = format!("{}_renamed", first.name);
        refuses(&fixture, &renamed, V2ClauseClaimField::Parameters);
        let mut missing = fixture.claim.clone();
        missing.record.parameters.clear();
        refuses(&fixture, &missing, V2ClauseClaimField::Parameters);
    }
}

#[trace("TC-025", "FR-015-AC-48")]
#[test]
fn a_state_read_mismatch_refuses_before_any_harness() {
    let fixture = read_fixture();
    common_mismatches(&fixture);
    let first = fixture.claim.record.state_reads[0].clone();
    let mut widened = fixture.claim.clone();
    widened.record.state_reads[0].maximum = first.maximum + 1;
    refuses(&fixture, &widened, V2ClauseClaimField::StateReads);
    let mut renamed = fixture.claim.clone();
    renamed.record.state_reads[0].field_name = "audit".to_owned();
    refuses(&fixture, &renamed, V2ClauseClaimField::StateReads);
    // The same field name under another declaring node is another read.
    let mut elsewhere = fixture.claim.clone();
    elsewhere.record.state_reads[0].declaring_node_id = fixture.clause.clone();
    refuses(&fixture, &elsewhere, V2ClauseClaimField::StateReads);
    let mut missing = fixture.claim.clone();
    missing.record.state_reads.clear();
    refuses(&fixture, &missing, V2ClauseClaimField::StateReads);
}

// FR-015-AC-135 and FR-015-AC-136: the claim's `NodeKey` is the selected clause node's key.
//
// The expected keys below are written out by hand from the package, node path and ordinal of
// the two fixture clauses. They are not read from the resolver CG uses, so a mutated resolver
// cannot change both sides of a comparison.

const TWIN: &str = "test:state-frame-twin";

fn text(value: &str) -> PathSegment {
    PathSegment::Text(value.to_owned())
}

fn key(package: &str, path: Vec<PathSegment>, ordinal: u64) -> NodeKey {
    NodeKey::new(package, path, ordinal).expect("hand-written node key")
}

fn twin_key(path: Vec<PathSegment>) -> NodeKey {
    key(TWIN, path, 0)
}

/// The hand-written key of the precondition `name` on operation `operation`, whose condition is
/// `self.<field> >= literal` over a field declared `minimum..=maximum`.
fn clause_key(
    operation: &str,
    literal: &str,
    minimum: &str,
    maximum: &str,
    ordinal: u64,
) -> NodeKey {
    let node = PathSegment::Node;
    let account = key(
        "test/bank",
        vec![
            text("model"),
            text("object_type"),
            text("ix://test/bank/Account"),
        ],
        0,
    );
    let integer = twin_key(vec![text("scalar_type"), text("integer")]);
    let reference = twin_key(vec![
        text("composite_type"),
        text("reference"),
        node(account.clone()),
    ]);
    let this = twin_key(vec![
        text("value"),
        text("parameter"),
        text("name"),
        node(twin_key(vec![text("scalar_type"), text("text")])),
        text("self"),
        text("level"),
        node(integer.clone()),
        text("0"),
        node(reference),
    ]);
    let deref = twin_key(vec![
        text("expression"),
        text("deref"),
        node(this),
        text("quire.op.model.deref"),
        text("deref"),
        node(account.clone()),
    ]);
    let domain = twin_key(vec![
        text("bounded_domain"),
        text("integer_range"),
        text("min"),
        node(integer.clone()),
        text(minimum),
        text("max"),
        text(maximum),
    ]);
    let query = twin_key(vec![
        text("expression"),
        text("query"),
        node(deref),
        text("quire.op.record.project"),
        text("false"),
        text("query"),
        node(domain),
    ]);
    let literal = twin_key(vec![
        text("value"),
        text("literal"),
        node(integer),
        text(literal),
    ]);
    let condition = twin_key(vec![
        text("expression"),
        text("binary"),
        node(query),
        node(literal),
        text("quire.op.integer.ge"),
        text("binary"),
        node(twin_key(vec![text("scalar_type"), text("boolean")])),
    ]);
    let anchor = twin_key(vec![
        text("state"),
        text("operation_anchor"),
        node(account),
        text(operation),
    ]);
    key(
        TWIN,
        vec![
            text("state"),
            text("state_clause"),
            text("precondition"),
            node(anchor),
            node(condition),
        ],
        ordinal,
    )
}

/// `ClauseA`: `pre` on `deposit` over `balance` (`-5..=10`).
fn key_a() -> NodeKey {
    clause_key("deposit", "0", "-5", "10", 0)
}

/// `ClauseB`: `pre` on `transfer` over `audit` (`50..=60`).
fn key_b() -> NodeKey {
    clause_key("transfer", "55", "50", "60", 0)
}

struct TwoClauses {
    package: CheckedPackageV2,
    a: CheckedNodeId,
    b: CheckedNodeId,
}

fn two_clauses() -> TwoClauses {
    let twin = Twin::with_preconditions_on_two_operations();
    let (package, a) = twin.emitted_package("ClauseA");
    let (other, b) = twin.emitted_package("ClauseB");
    assert_eq!(package.name(), other.name(), "one package");
    TwoClauses { package, a, b }
}

impl TwoClauses {
    /// A claim for `clause`, its record written by hand from the fixture source.
    fn claim(
        &self,
        clause: &CheckedNodeId,
        node: NodeKey,
        state_read: V2ClauseRead,
    ) -> V2ClauseObligationClaim {
        V2ClauseObligationClaim {
            obligation: ObligationIdentity::new(node, None, 0),
            record: V2ClauseObligationRecord {
                occurrence: occurrence(&self.package, clause),
                kind: ObligationKind::Precondition,
                parameters: Vec::new(),
                state_reads: vec![state_read],
            },
        }
    }

    fn read_a(&self, maximum: i64) -> V2ClauseRead {
        read(&account_declaration(&self.package), "balance", -5, maximum)
    }

    fn read_b(&self) -> V2ClauseRead {
        read(&account_declaration(&self.package), "audit", 50, 60)
    }

    /// Request both clauses, `ClauseB` listed first.
    fn request(
        &self,
        claim_a: &V2ClauseObligationClaim,
        claim_b: &V2ClauseObligationClaim,
    ) -> KaniObligationOutcome {
        let items = [
            ObligationItem::V2Clause {
                package: &self.package,
                clause: &self.b,
                claim: claim_b,
                dependencies: None,
            },
            ObligationItem::V2Clause {
                package: &self.package,
                clause: &self.a,
                claim: claim_a,
                dependencies: None,
            },
        ];
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &items,
            subject_path: "crate::subject::deposit",
            unwind: 3,
        })
        .expect("a well-formed request")
    }
}

/// The `ClauseA` record is the second, `ClauseB` the first: B is listed first.
fn disposition(outcome: &KaniObligationOutcome, listed: usize) -> &ObligationDisposition {
    &outcome.records()[listed].disposition
}

fn is_supported(outcome: &KaniObligationOutcome, listed: usize) -> bool {
    matches!(
        disposition(outcome, listed),
        ObligationDisposition::Supported { .. }
    )
}

fn node_key_refusal(outcome: &KaniObligationOutcome, listed: usize) -> Option<&CheckedNodeId> {
    match disposition(outcome, listed) {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseNodeKeyMismatch { node_id },
        } => Some(node_id),
        _ => None,
    }
}

fn emitted_obligations(outcome: &KaniObligationOutcome) -> Vec<ObligationIdentity> {
    match outcome {
        KaniObligationOutcome::Emitted {
            v2_clause_harnesses,
            ..
        } => v2_clause_harnesses
            .iter()
            .map(|harness| harness.obligation.clone())
            .collect(),
        KaniObligationOutcome::Rejected { .. } => Vec::new(),
    }
}

#[trace("TC-025", "FR-015-AC-135")]
#[test]
fn each_clause_claim_carrying_its_own_resolved_key_is_admitted() {
    let two = two_clauses();
    let claim_a = two.claim(&two.a, key_a(), two.read_a(10));
    let claim_b = two.claim(&two.b, key_b(), two.read_b());
    let outcome = two.request(&claim_a, &claim_b);
    assert!(is_supported(&outcome, 0), "B: {:?}", outcome.records()[0]);
    assert!(is_supported(&outcome, 1), "A: {:?}", outcome.records()[1]);
    assert_eq!(
        emitted_obligations(&outcome),
        vec![claim_b.obligation.clone(), claim_a.obligation.clone()],
        "the carried identity equals the supplied one"
    );
}

#[trace("TC-025", "FR-015-AC-135")]
#[test]
fn a_claim_carrying_another_clauses_key_is_refused_naming_only_the_clause() {
    let two = two_clauses();
    // A's own, correct record under B's key.
    let swapped = two.claim(&two.a, key_b(), two.read_a(10));
    let claim_b = two.claim(&two.b, key_b(), two.read_b());
    let outcome = two.request(&swapped, &claim_b);
    assert_eq!(node_key_refusal(&outcome, 1), Some(&two.a), "A is named");
    assert!(is_supported(&outcome, 0), "B is unaffected");
    assert_eq!(
        emitted_obligations(&outcome),
        vec![claim_b.obligation.clone()],
        "no harness for A"
    );
    // The refusal names the node and holds neither key nor CG's selection.
    let ObligationDisposition::Unsupported {
        reason: UnsupportedObligation::V2ClauseNodeKeyMismatch { node_id },
    } = disposition(&outcome, 1)
    else {
        panic!("refused");
    };
    assert_eq!(node_id, &two.a);
    let rendered = format!("{:?}", disposition(&outcome, 1));
    for forbidden in [
        "test/bank",
        "operation_anchor",
        "transfer",
        "audit",
        "balance",
        "selected",
    ] {
        assert!(!rendered.contains(forbidden), "{forbidden}: {rendered}");
    }
}

#[trace("TC-025", "FR-015-AC-135")]
#[test]
fn a_claim_key_differing_only_in_its_node_ordinal_is_refused() {
    let two = two_clauses();
    let reordinal = two.claim(
        &two.a,
        clause_key("deposit", "0", "-5", "10", 1),
        two.read_a(10),
    );
    let claim_b = two.claim(&two.b, key_b(), two.read_b());
    let outcome = two.request(&reordinal, &claim_b);
    assert_eq!(node_key_refusal(&outcome, 1), Some(&two.a));
    assert!(is_supported(&outcome, 0));
    assert_eq!(emitted_obligations(&outcome).len(), 1);
}

#[trace("TC-025", "FR-015-AC-136")]
#[test]
fn the_key_refusal_precedes_the_record_comparison() {
    let two = two_clauses();
    // B's key and a record that also differs from the clause (A's read domain widened).
    let both = two.claim(&two.a, key_b(), two.read_a(11));
    let claim_b = two.claim(&two.b, key_b(), two.read_b());
    let outcome = two.request(&both, &claim_b);
    assert_eq!(
        node_key_refusal(&outcome, 1),
        Some(&two.a),
        "{:?}",
        outcome.records()[1]
    );
    // The record alone differs under A's own key, so the control for this changed record is
    // the record refusal.
    let record_only = two.claim(&two.a, key_a(), two.read_a(11));
    let outcome = two.request(&record_only, &claim_b);
    assert!(matches!(
        disposition(&outcome, 1),
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseClaimMismatch {
                field: V2ClauseClaimField::StateReads,
                ..
            }
        }
    ));
}

#[trace("TC-025", "FR-015-AC-136")]
#[test]
fn a_key_mismatch_precedes_a_lowering_refusal() {
    // The second parameter is unbounded, so lowering refuses this clause whatever its claim.
    let (package, clause) =
        Twin::with_unbounded_second_parameter_precondition().emitted_package("TwoParameters");
    let item = |claim| {
        let items = [ObligationItem::V2Clause {
            package: &package,
            clause: &clause,
            claim,
            dependencies: None,
        }];
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &items,
            subject_path: "crate::subject::deposit",
            unwind: 3,
        })
        .expect("a well-formed request")
    };
    // Control: the resolved key reaches lowering, which refuses.
    let own = crate::common::v2_claim::claim_for(&package, &clause);
    assert!(matches!(
        disposition(&item(&own), 0),
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseConditionNotRendered { .. }
        }
    ));
    // Another clause's hand-written key: the key refusal comes first.
    let other = V2ClauseObligationClaim {
        obligation: ObligationIdentity::new(key_a(), None, 0),
        record: own.record.clone(),
    };
    assert_eq!(node_key_refusal(&item(&other), 0), Some(&clause));
}

#[trace("TC-025", "FR-015-AC-136")]
#[test]
fn a_key_mismatch_precedes_a_parameter_domain_mismatch() {
    for fixture in parameter_fixtures() {
        let mut changed =
            with_obligation(&fixture.claim, ObligationIdentity::new(key_b(), None, 0));
        changed.record.parameters[0].maximum += 1;
        let outcome = generate(&fixture, &changed, 3, "crate::subject::deposit");
        assert_eq!(
            node_key_refusal(&outcome, 0),
            Some(&fixture.clause),
            "{:?}",
            fixture.kind
        );
        assert_eq!(mismatch_of(&outcome), None, "{:?}", fixture.kind);
        // Control: the same changed record under the resolved key is the record refusal.
        let mut own = fixture.claim.clone();
        own.record.parameters[0].maximum += 1;
        let outcome = generate(&fixture, &own, 3, "crate::subject::deposit");
        assert_eq!(
            mismatch_of(&outcome),
            Some(V2ClauseClaimField::Parameters),
            "{:?}",
            fixture.kind
        );
    }
}
