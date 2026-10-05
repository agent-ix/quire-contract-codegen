//! FR-024 (IR-460): a postcondition state-clause counterexample is put before QSL's
//! `replay_state_clause` as a request and a witness-arm envelope built by `StateClauseReplay`.
//!
//! The admitted package is the state-frame fixture of `kani_obligations_state_frame`, the same one
//! the operation-contract harness is generated from; the QSL twin is its hand-mirrored native
//! unit, here with two postcondition clauses on `deposit`. The invocation and snapshot documents
//! are built by the crate under test and read back from the request's byte provision, so each
//! assertion is about the bytes QSL receives.
//!
//! The default lane runs without Kani. The `kani` lane (`#[ignore]` here, run serially by
//! `make kani`, whose `kani_obligations` filter this module's name matches) replays the real
//! playback of the falsified operation-contract harness.

use std::cell::RefCell;

use qsl_replay::{
    replay_state_clause, CallSiteRefusal, Category, ClauseName, ClauseSelectionInput,
    DisagreementCause, DomainKey, EvaluatedValue, FiniteBound, Identifier, Integer,
    ReplayRequestWire, ReplayResult, ReplaySource, StateClauseReplayResult, Verdict, WireNodeId,
    WitnessEnvelope, WitnessSettlement,
};
use quire_contract_codegen::{
    DocumentLabel, OperationDeclaration, StateClauseModelFieldsCause, StateClauseReplay,
    StateClauseReplayError, StateClauseReplayInputs, StateObjectAddress,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::kani_obligations_state_frame::{
    falsified, fixture_declaring, fixture_with_unbounded_balance, model, native_twin::Twin,
    playback_state, prove, self_parameter, subject, Declares, Fixture,
};

/// The clause whose counterexample the tests replay: `balance` never drops.
const BALANCE: &str = "BalanceNeverDrops";
/// The second postcondition clause on `deposit`: `audit` never drops.
const AUDIT: &str = "AuditNeverDrops";

/// QSL's emitted checked package has an empty model body, yet replay takes the caller's field
/// order and both inclusive ranges from the admitted model declaration's accessor.
///
/// Trace: FR-024-AC-31, FR-024-AC-33
#[test]
fn tc_035_emitted_model_fields_bind_replay_before_playback() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let input = twin.state_clause_inputs(&package, &clause, BALANCE, (5, 7), (4, 7));
    let replay = StateClauseReplay::new(input.clone()).expect("model fields resolve");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), model::FIELDS.len());
    for (position, (domain, (_, (lower, upper)))) in domains.iter().zip(model::FIELDS).enumerate() {
        assert!(matches!(
            domain.domain(),
            DomainKey::Node { path, .. } if path == &[u32::try_from(position).expect("small")]
        ));
        assert_eq!(
            domain.bound(),
            &FiniteBound::integer_range(Integer::from(lower), Integer::from(upper)).expect("range")
        );
    }
    let result = replay.replay().expect("the debit replays");
    assert_eq!(
        witness_arm(&result).settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    let respecting = StateClauseReplay::new(twin.state_clause_inputs(
        &package,
        &clause,
        BALANCE,
        (5, 7),
        (6, 7),
    ))
    .expect("the same emitted package builds a respecting replay");
    let result = respecting.replay().expect("the respecting run replays");
    assert_eq!(
        witness_arm(&result).settlement(),
        WitnessSettlement::Inconclusive
    );
    assert!(matches!(
        witness_arm(&result).disagreement(),
        Some(DisagreementCause::Verdicts { .. })
    ));

    let mut absent = input.clone();
    absent.state_fields.push("ghost".to_owned());
    absent.playback.clear();
    let error = StateClauseReplay::new(absent)
        .err()
        .expect("model absence precedes binding");
    assert!(matches!(
        error,
        StateClauseReplayError::ModelFields {
            cause: StateClauseModelFieldsCause::Absent { field }, ..
        } if field == "ghost"
    ));

    let mut outside = input;
    outside.playback[1].1 = 1001;
    assert!(matches!(
        StateClauseReplay::new(outside),
        Err(StateClauseReplayError::OutOfDomain { field }) if field == "audit"
    ));
}

fn healthy() -> Fixture {
    fixture_declaring(Declares::Nothing)
}

fn built(
    twin: &Twin,
    fixture: &Fixture,
    clause: &str,
    pre: (i64, i64),
    post: (i64, i64),
) -> StateClauseReplay {
    StateClauseReplay::new(twin.state_clause_inputs(
        &fixture.package,
        &fixture.clause,
        clause,
        pre,
        post,
    ))
    .expect("the replay is built")
}

fn inputs<'a>(
    twin: &Twin,
    fixture: &'a Fixture,
    pre: (i64, i64),
    post: (i64, i64),
) -> StateClauseReplayInputs<'a> {
    twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, pre, post)
}

/// What `qsl_replay::replay_state_clause` returns for the request and envelope `replay` holds,
/// called directly.
fn direct(replay: StateClauseReplay) -> StateClauseReplayResult {
    let StateClauseReplay { wire, packet } = replay;
    let envelope = WitnessEnvelope::reconstruct(packet).expect("a complete packet reconstructs");
    replay_state_clause(wire, &envelope).expect("QSL settles the replay")
}

/// The violating run: the debiting subject takes `balance` from 5 to 4.
const VIOLATING: ((i64, i64), (i64, i64)) = ((5, 0), (4, 0));
/// The respecting run over the same pre state: the crediting subject takes `balance` to 6.
const RESPECTING: ((i64, i64), (i64, i64)) = ((5, 0), (6, 0));

fn witness_arm(result: &StateClauseReplayResult) -> &qsl_replay::WitnessArmResult {
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    arm
}

/// `StateClauseReplay::replay` returns what `replay_state_clause` returns for the same request
/// and envelope, for a violating and a respecting run whose results differ, and `replay_through`
/// hands its executor the request and envelope it built and returns the executor's value as it is.
///
/// Trace: FR-024-AC-11, TC-035
#[test]
fn tc_035_replay_returns_the_direct_result_and_replay_through_returns_the_executors_value() {
    let (twin, fixture) = (Twin::new(), healthy());
    let mut results = Vec::new();
    for (pre, post) in [VIOLATING, RESPECTING] {
        let through = built(&twin, &fixture, BALANCE, pre, post)
            .replay()
            .expect("the replay settles");
        assert_eq!(
            through,
            direct(built(&twin, &fixture, BALANCE, pre, post)),
            "replay is the direct call"
        );
        results.push(through);
    }
    let [violating, respecting] = &results[..] else {
        panic!("two results");
    };
    assert_ne!(violating, respecting, "the two runs settle differently");

    // The executor sees the request and envelope the replay built, and its value comes back.
    let seen = RefCell::new(None);
    let replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
    let expected_package = replay.wire.package_id.clone();
    let sentinel = respecting.clone();
    let returned = replay
        .replay_through(|wire: ReplayRequestWire, envelope| {
            *seen.borrow_mut() = Some((
                wire.package_id.clone(),
                envelope.family_payload().clause.clone(),
                envelope.clause_node(),
            ));
            Ok(sentinel)
        })
        .expect("the executor's value is returned");
    assert_eq!(&returned, respecting, "the sentinel replaces the verdict");
    assert_ne!(&returned, violating);
    let Some((package_id, clause, node)) = seen.take() else {
        panic!("the executor was called");
    };
    assert_eq!(package_id, expected_package);
    assert_eq!(clause.as_str(), BALANCE);
    assert_eq!(
        node,
        twin.clause_site(BALANCE)
            .expect("the clause is declared")
            .node
    );
}

/// The payload names its clause and the `Invocation` arm, the envelope's identities are the
/// `ClauseSite`'s that `call_site` names, two clauses carry their own, and an undeclared clause is
/// refused by the call site when the request is built.
///
/// Trace: FR-024-AC-12, TC-035
#[test]
fn tc_035_the_envelope_carries_the_call_sites_clause_identities() {
    let (twin, fixture) = (Twin::new(), healthy());
    let mut nodes = Vec::new();
    for clause in [BALANCE, AUDIT] {
        let replay = built(&twin, &fixture, clause, RESPECTING.0, RESPECTING.1);
        let site = twin.clause_site(clause).expect("the clause is declared");
        let payload = replay.packet.family_payload.as_ref().expect("a payload");
        assert_eq!(payload.clause.as_str(), clause);
        assert_eq!(site.name.as_str(), clause);
        assert!(matches!(
            payload.observation,
            ClauseSelectionInput::Invocation { .. }
        ));
        assert!(payload.witness.is_none());
        assert_eq!(replay.packet.clause_node, Some(site.node));
        assert_eq!(
            replay.packet.occurrence_key.as_ref(),
            Some(&site.occurrence)
        );
        nodes.push(site.node);
    }
    assert_ne!(nodes[0], nodes[1], "each clause has its own node");

    let mut missing = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
    missing.clause = ClauseName(Identifier::new("NoSuchClause").expect("identifier"));
    let refusal = StateClauseReplay::new(missing)
        .err()
        .expect("an undeclared clause is refused");
    assert!(
        matches!(
            &refusal,
            StateClauseReplayError::CallSite(cause)
                if matches!(&**cause, CallSiteRefusal::UnknownClause { selection, .. }
                    if selection.0.as_str() == "NoSuchClause")
        ),
        "{refusal}"
    );
}

/// The documents of a request, by what each is.
struct Documents {
    invocation: (String, Value),
    pre: (String, Value),
    post: (String, Value),
}

/// Reads the invocation and the two snapshots from `wire`'s byte provision, each with the digest
/// the request addresses it by, after checking that digest is SHA-256 of the bytes provided.
fn documents(wire: &ReplayRequestWire) -> Documents {
    let mut found = (None, None, None);
    for (domain, hex, bytes) in &wire.byte_provision {
        if domain.as_deref() != Some("sha256-jcs") {
            continue;
        }
        let Ok(document) = serde_json::from_slice::<Value>(bytes) else {
            continue;
        };
        let entry = (hex.clone(), document.clone());
        match document.get("format").and_then(Value::as_str) {
            Some("quire.state.invocation/v1") => found.0 = Some(entry),
            Some("quire.state.snapshot/v1") => match document.get("observation") {
                Some(Value::String(role)) if role == "pre" => found.1 = Some(entry),
                Some(Value::String(role)) if role == "post" => found.2 = Some(entry),
                other => panic!("an unknown snapshot role: {other:?}"),
            },
            _ => continue,
        }
        let digest = Sha256::digest(bytes);
        let actual = digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(
            &actual, hex,
            "the request addresses a document by its bytes' digest"
        );
    }
    let (Some(invocation), Some(pre), Some(post)) = found else {
        panic!("the request provides the invocation and both snapshots");
    };
    Documents {
        invocation,
        pre,
        post,
    }
}

fn integers(snapshot: &Value) -> Value {
    snapshot
        .pointer("/populations/0/objects/0/fields")
        .cloned()
        .expect("the snapshot holds one object's fields")
}

/// A label as a document's identity member spells it.
fn identity_member(label: &DocumentLabel) -> Value {
    json!({
        "authority": label.authority,
        "identity": label.identity,
        "revision_namespace": label.revision_namespace,
        "revision": label.revision,
    })
}

/// Checks the three documents against the address, the labels (invocation, pre, post) and the
/// `(balance, audit)` decimal strings of the pre and post snapshots.
fn assert_documents(
    documents: &Documents,
    address: &StateObjectAddress,
    labels: &[DocumentLabel; 3],
    [(pre_balance, pre_audit), (post_balance, post_audit)]: [(&str, &str); 2],
) {
    let Documents {
        invocation,
        pre,
        post,
    } = documents;
    let [invocation_label, pre_label, post_label] = labels;
    for (document, observation, label, balance, audit) in [
        (&pre.1, "pre", pre_label, pre_balance, pre_audit),
        (&post.1, "post", post_label, post_balance, post_audit),
    ] {
        assert_eq!(document["observation"], observation);
        assert_eq!(document["identity"], identity_member(label));
        assert_eq!(document["model"]["identity"], "test/bank");
        let populations = document["populations"].as_array().expect("populations");
        assert_eq!(populations.len(), 1);
        assert_eq!(populations[0]["population"], address.population.as_str());
        assert_eq!(populations[0]["complete"], true);
        let objects = populations[0]["objects"].as_array().expect("objects");
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0]["key"], address.key.as_str());
        assert_eq!(objects[0]["type"], address.object_type.as_str());
        assert_eq!(
            integers(document),
            json!({"balance": {"integer": balance}, "audit": {"integer": audit}})
        );
    }

    assert_eq!(invocation.1["identity"], identity_member(invocation_label));
    assert_eq!(invocation.1["model"], pre.1["model"]);
    assert_eq!(invocation.1["context"], address.object_type.as_str());
    assert_eq!(invocation.1["operation"], "deposit");
    assert_eq!(
        invocation.1["self"],
        json!({"population": address.population, "key": address.key})
    );
    assert_eq!(invocation.1["parameters"], json!({}));
    assert_eq!(invocation.1["result"], Value::Null);
    assert_eq!(invocation.1["created"], json!([]));
    assert_eq!(invocation.1["deleted"], json!([]));
    for (link, snapshot, label) in [("pre", pre, pre_label), ("post", post, post_label)] {
        assert_eq!(invocation.1[link]["identity"], identity_member(label));
        assert_eq!(
            invocation.1[link]["digest"],
            format!("sha256-jcs:{}", snapshot.0)
        );
    }
}

/// The snapshots hold the playback's and the supplied values, the invocation holds the stated
/// members with both digests, and a changed value changes only its own snapshot.
///
/// Trace: FR-024-AC-13, TC-035
#[test]
fn tc_035_the_documents_hold_the_playback_the_post_state_and_their_digests() {
    let (twin, fixture) = (Twin::new(), healthy());
    let default = inputs(&twin, &fixture, (5, 7), (4, 8));
    let (address, labels) = (
        default.object.clone(),
        [
            default.invocation_label.clone(),
            default.pre_label.clone(),
            default.post_label.clone(),
        ],
    );
    let wire = StateClauseReplay::new(default)
        .expect("the replay is built")
        .wire;
    let Documents {
        invocation,
        pre,
        post,
    } = documents(&wire);
    assert_documents(
        &Documents {
            invocation: invocation.clone(),
            pre: pre.clone(),
            post: post.clone(),
        },
        &address,
        &labels,
        [("5", "7"), ("4", "8")],
    );

    // The address and every member of the three labels are carried as given, each its own value:
    // a document that wrote a literal, or swapped two label members, would not match.
    let label = |tag: &str| DocumentLabel {
        authority: format!("authority-{tag}"),
        identity: format!("identity-{tag}"),
        revision_namespace: format!("namespace-{tag}"),
        revision: format!("revision-{tag}"),
    };
    let other_address = StateObjectAddress {
        population: "ix://test/bank/other-accounts".to_owned(),
        key: "account-two".to_owned(),
        object_type: "ix://test/bank/Savings".to_owned(),
    };
    let other_labels = [label("invocation"), label("pre"), label("post")];
    let mut varied = inputs(&twin, &fixture, (1, 2), (3, 4));
    varied.object = other_address.clone();
    varied.invocation_label = other_labels[0].clone();
    varied.pre_label = other_labels[1].clone();
    varied.post_label = other_labels[2].clone();
    let varied = StateClauseReplay::new(varied)
        .expect("the replay is built")
        .wire;
    assert_documents(
        &documents(&varied),
        &other_address,
        &other_labels,
        [("1", "2"), ("3", "4")],
    );

    // One changed playback value changes the pre snapshot and nothing of the post snapshot.
    let changed_pre = documents(&built(&twin, &fixture, BALANCE, (6, 7), (4, 8)).wire);
    assert_ne!(changed_pre.pre.0, pre.0);
    assert_ne!(changed_pre.pre.1, pre.1);
    assert_eq!(changed_pre.post.0, post.0);
    // One changed post-state value changes the post snapshot and nothing of the pre snapshot.
    let changed_post = documents(&built(&twin, &fixture, BALANCE, (5, 7), (4, 9)).wire);
    assert_ne!(changed_post.post.0, post.0);
    assert_ne!(changed_post.post.1, post.1);
    assert_eq!(changed_post.pre.0, pre.0);
}

/// A state field with no playback value, or no post-state value, is refused by name and no
/// default is supplied.
///
/// Trace: FR-024-AC-15, TC-035
#[test]
fn tc_035_a_missing_field_is_refused_by_name() {
    let (twin, fixture) = (Twin::new(), healthy());
    for field in ["balance", "audit"] {
        let mut no_playback = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        no_playback.playback.retain(|(name, _)| name != field);
        let mut no_post = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        no_post.post_state.retain(|(name, _)| name != field);
        for missing in [no_playback, no_post] {
            let error = StateClauseReplay::new(missing)
                .err()
                .expect("a missing field is refused");
            assert!(
                matches!(&error, StateClauseReplayError::MissingField { field: named } if named == field),
                "{field}: {error}"
            );
        }
    }
}

/// A name the framed object does not declare, and a field bound twice, are refused by name on
/// the playback and on the post state alike: no binding is dropped or taken first-wins.
///
/// Trace: FR-024-AC-15, TC-035
#[test]
fn tc_035_an_undeclared_or_repeated_binding_is_refused_by_name() {
    let (twin, fixture) = (Twin::new(), healthy());
    for on_post_state in [false, true] {
        let side = |candidate: &mut StateClauseReplayInputs<'_>, name: &str, value: i64| {
            let values = if on_post_state {
                &mut candidate.post_state
            } else {
                &mut candidate.playback
            };
            values.push((name.to_owned(), value));
        };
        let mut undeclared = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        side(&mut undeclared, "ghost", 1);
        let error = StateClauseReplay::new(undeclared)
            .err()
            .expect("an undeclared name is refused");
        assert!(
            matches!(&error, StateClauseReplayError::UndeclaredField { field } if field == "ghost"),
            "{error}"
        );

        let mut repeated = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        side(&mut repeated, "audit", 9);
        let error = StateClauseReplay::new(repeated)
            .err()
            .expect("a repeated field is refused");
        assert!(
            matches!(&error, StateClauseReplayError::DuplicateField { field } if field == "audit"),
            "{error}"
        );
    }
}

/// The envelope's `declared_domains` are one per ranged state field, on `self`'s node at the
/// field's child index with the declared range, and the witness transcript names the operation,
/// the clause and each playback value. QSL's replay reads neither, so only these assertions
/// pin them. A field whose member declares no range is carried and is not range-checked or
/// given a domain.
///
/// Trace: FR-024-AC-12, TC-035
#[test]
fn tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback() {
    let (twin, fixture) = (Twin::new(), healthy());
    let candidate = inputs(&twin, &fixture, (5, 7), (4, 8));
    let operation = candidate.operation.to_string();
    let replay = StateClauseReplay::new(candidate).expect("the replay is built");

    let Some(ReplaySource::Witness(witness)) = &replay.packet.source else {
        panic!("the envelope is on the witness arm");
    };
    assert_eq!(
        witness.transcript(),
        format!("<<<assertion|{operation}|{BALANCE}|balance=5;audit=7>>>")
    );

    let parameter = WireNodeId::from_hex(&self_parameter().digest).expect("a node id");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), model::FIELDS.len());
    for (position, (domain, (_, (minimum, maximum)))) in
        domains.iter().zip(model::FIELDS).enumerate()
    {
        assert_eq!(
            domain.domain(),
            &DomainKey::Node {
                node: parameter,
                path: vec![u32::try_from(position).expect("small")],
            }
        );
        assert_eq!(
            domain.bound(),
            &FiniteBound::integer_range(Integer::from(minimum), Integer::from(maximum))
                .expect("a range")
        );
    }

    // `balance` declares no range: any playback value is carried, and only `audit` has a domain.
    let unbounded = fixture_with_unbounded_balance();
    let mut candidate = inputs(&twin, &unbounded, (5, 7), (4, 8));
    candidate.playback[0].1 = 1_000_000;
    let replay = StateClauseReplay::new(candidate).expect("an unranged field is not checked");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), 1);
    assert!(matches!(
        domains[0].domain(),
        DomainKey::Node { path, .. } if path == &[1]
    ));
}

/// The mutated subject's counterexample settles `reproduced-with-evaluated-witness` in category
/// `violation` with an evaluated `false`; the unmutated subject's run over the same pre state
/// settles `inconclusive` with cause `Verdicts`; both envelopes are on the `Witness` arm with a
/// payload `witness` of none.
///
/// Trace: FR-024-AC-16, TC-035
#[test]
fn tc_035_a_violating_run_reproduces_and_a_respecting_run_is_inconclusive() {
    let (twin, fixture) = (Twin::new(), healthy());
    for (pre, post) in [VIOLATING, RESPECTING] {
        let replay = built(&twin, &fixture, BALANCE, pre, post);
        assert!(matches!(
            replay.packet.source,
            Some(ReplaySource::Witness(_))
        ));
        let payload = replay.packet.family_payload.as_ref().expect("a payload");
        assert!(payload.witness.is_none());
    }

    let violating = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1)
        .replay()
        .expect("the replay settles");
    let arm = witness_arm(&violating);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    assert_eq!(arm.value(), Some(EvaluatedValue::Boolean(false)));

    let respecting = built(&twin, &fixture, BALANCE, RESPECTING.0, RESPECTING.1)
        .replay()
        .expect("the replay settles");
    let arm = witness_arm(&respecting);
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
        })
    );
}

/// A playback value outside its field's declared range is refused by name, and the range's two
/// endpoints are admitted. The out-of-range value is put in the playback alone, and separately in
/// the post state alone, so the test says which side the range is read from: the playback. The
/// post state is whatever the subject ran to; how a post state outside the range should settle
/// is a pending QSL ruling (see the replay spec's Current state), and this test pins no answer to
/// it.
///
/// Trace: FR-024-AC-17, TC-035
#[test]
fn tc_035_a_value_outside_its_declared_range_is_out_of_domain() {
    let (twin, fixture) = (Twin::new(), healthy());
    for (field, (minimum, maximum)) in model::FIELDS {
        // Only the playback holds `value` for `field`; the post state keeps the field as it was.
        let in_playback = |value: i64| {
            let mut candidate = inputs(&twin, &fixture, (5, 5), (5, 5));
            if let Some(entry) = candidate
                .playback
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = value;
            }
            candidate
        };
        // Only the post state holds `value` for `field`; the playback is in range.
        let in_post_state = |value: i64| {
            let mut candidate = inputs(&twin, &fixture, (5, 5), (5, 5));
            if let Some(entry) = candidate
                .post_state
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = value;
            }
            candidate
        };
        for admitted in [minimum, maximum] {
            // A replayed run changes only what the frame grants, so the post state holds the
            // same value.
            let mut candidate = in_playback(admitted);
            if let Some(entry) = candidate
                .post_state
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = admitted;
            }
            let replay = StateClauseReplay::new(candidate)
                .unwrap_or_else(|error| panic!("{field}={admitted} is admitted: {error}"));
            replay.replay().expect("an admitted value replays");
        }
        for refused in [minimum - 1, maximum + 1] {
            let error = StateClauseReplay::new(in_playback(refused))
                .err()
                .unwrap_or_else(|| panic!("{field}={refused} is outside the range"));
            assert!(
                matches!(&error, StateClauseReplayError::OutOfDomain { field: named } if named == field),
                "{field}={refused}: {error}"
            );
            // The same value on the post-state side is not the playback's out-of-domain error.
            StateClauseReplay::new(in_post_state(refused))
                .unwrap_or_else(|error| panic!("{field}={refused} in the post state: {error}"));
        }
    }
}

/// An operation that declares a parameter, and one that declares a result, are refused by shape
/// before anything else is read; one that declares neither is not.
///
/// Trace: FR-024-AC-19, TC-035
#[test]
fn tc_035_an_operation_declaring_a_parameter_or_a_result_is_refused_by_shape() {
    let twin = Twin::new();
    let operation = twin
        .state_clause_inputs(
            &healthy().package,
            &healthy().clause,
            BALANCE,
            (5, 0),
            (6, 0),
        )
        .operation;
    for (declares, declaration) in [
        (
            Declares::Parameter,
            OperationDeclaration {
                parameters: vec!["other".to_owned()],
                result: false,
            },
        ),
        (
            Declares::Result,
            OperationDeclaration {
                parameters: Vec::new(),
                result: true,
            },
        ),
    ] {
        let fixture = fixture_declaring(declares);
        // A playback with a field missing would be a different refusal: the shape is read first.
        let mut candidate = inputs(&twin, &fixture, (5, 0), (6, 0));
        candidate.playback.clear();
        let error = StateClauseReplay::new(candidate)
            .err()
            .expect("the shape is refused");
        assert!(
            matches!(
                &error,
                StateClauseReplayError::UnsupportedOperationShape { operation: named, declaration: found }
                    if *named == operation && *found == declaration
            ),
            "{error}"
        );
    }
    let supported = healthy();
    assert!(StateClauseReplay::new(inputs(&twin, &supported, (5, 0), (6, 0))).is_ok());
}

// ---- kani lane ---------------------------------------------------------------

/// The falsified operation-contract harness of a postcondition state clause over a subject
/// mutated to debit is replayed from its real Kani playback through `replay_state_clause`, which
/// reproduces the violation. The subject is the debit that stops at the floor of `balance`'s range
/// (`deposit_debiting_within_range`): the wrapping debit's real playback is the floor, whose native
/// post state is -1, outside the range, and QSL's snapshot admission refuses it (`invalid-value`).
/// How a violation that yields such a post state should settle is a pending QSL ruling and is not
/// decided here (see the replay spec's Current state).
///
/// Trace: FR-024-AC-18, TC-035
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_035_real_kani_state_clause_counterexample_replays_through_qsl() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let generated = quire_contract_codegen::generate_state_frame_obligations(
        &quire_contract_codegen::StateFrameRequest {
            package: &package,
            clause: &clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting_within_range",
            unwind: 4,
        },
    )
    .expect("emitted package generates");
    let counterexample = falsified(
        prove(&generated.postcondition),
        "postcondition `post.balance >= pre.balance` failed",
    );
    let (balance, audit) = playback_state(&counterexample);
    let mut account = subject::Account { balance, audit };
    subject::deposit_debiting_within_range(&mut account);
    assert_ne!(
        account.balance, balance,
        "the native run reproduces the debit"
    );
    let replay = StateClauseReplay::new(twin.state_clause_inputs(
        &package,
        &clause,
        BALANCE,
        (balance, audit),
        (account.balance, account.audit),
    ))
    .expect("emitted package replays");
    assert!(matches!(
        replay.packet.source,
        Some(ReplaySource::Witness(_))
    ));
    let result = replay.replay().expect("the replay settles");
    let arm = witness_arm(&result);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
}
