//! Private installed-Kani controls for field-edge reachability claims, `quire.op.model.reaches_field`
//! over `Reference`, `Option` and bounded `Collection` fields (FR-015-AC-98, -105, -106, TC-025,
//! IR-805 slice D). Run explicitly and serially: `cargo test --lib kani_graph_field_private --
//! --ignored --test-threads=1`.
//!
//! The Kani controls need QSL to link `reaches_field` to the population of the edge type
//! (QSL-1904): without that link the generator refuses the claim and `harness` below panics, so
//! they are `#[ignore]` with the rest of the installed-Kani lane. The seeded mutants edit the
//! generated reachability helper like slice B's and are sealed with the same crate-private
//! constructor. Each Falsified playback is decoded into the graph Kani drew and put before QSL's
//! `replay_state_clause` over the AUTHORED clause: the graph is carried as the field's own value
//! on each object (`{"reference": ..}`, `{"present": ..}` or `{"absent": {}}`, a collection
//! literal), the form the field's member type states, not a relationship entry.
//!
//! No wrong-field mutant edits the harness: it draws one edge only, so a closure following
//! another field cannot be expressed by editing it. Wrong multiplicity is evidenced instead by the
//! same authored claim over a field whose member type admits a missing edge.

use super::kani_graph_private::{
    assert_clause_holds, assert_reproduced, authored_over, generated, graph_inputs, mutated,
    replay, run_falsified, run_outcome, settled, transitivity, Authored, CLOSURE_STEP, CONCLUSION,
    DOMAIN, NODE, POPULATION, SOURCE,
};
use crate::{
    kani::generate::graph::{field_edge, GraphRefusal},
    kani::generate::outcome::GraphUnsupportedReason as Reason,
    GraphWitness, StateClauseReplay,
};
use ix_trace_rs::trace;
use qsl_replay::{DisagreementCause, WitnessSettlement};
use quire_contract_model::{CheckedNodeId, CheckedTerm};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};

fn field(name: &str, lower: u64, upper: u64, ordered: bool, unique: bool, presence: &str) -> Value {
    json!({
        "identity": format!("{NODE}/{name}"), "name": name, "typeRef": NODE,
        "presence": presence, "nullable": false, "defaultKind": "none",
        "origin": generated(&format!("{NODE}/{name}")),
        "multiplicity": {"lower": lower, "upper": upper, "ordered": ordered, "unique": unique}
    })
}

/// A `Node` whose only fields are reference edges: `next` (`Reference`), `maybe` (`Option`),
/// `many` (a `Sequence` of zero to two) and `empty` (a `Sequence` of zero to zero), and one
/// operation to anchor on. An object with no other field needs no value this replay path would
/// have to invent.
fn domain() -> Vec<u8> {
    encode(&domain_value())
}

fn encode(document: &Value) -> Vec<u8> {
    let limits = quire_canonical::Limits::new(1 << 20);
    quire_canonical::to_vec(document, limits).expect("canonical domain document")
}

fn domain_value() -> Value {
    let construct = |name: &str, meaning: &str| {
        json!({"kind": {"module": DOMAIN, "name": name}, "moduleVersion": "1.0.0",
               "construct": {"identity": "none", "shape": "record", "members": {}, "meaning": meaning}})
    };
    json!({
        "contractVersion": "2.0.0",
        "canonicalization": {"algorithm": "rfc8785-v1"},
        "source": {"identity": SOURCE, "version": "1.0.0", "dialect": "spec-bundle"},
        "package": {"identity": DOMAIN, "version": "1.0.0", "mappingVersions": [], "profileVersions": []},
        "constructs": [
            construct("entity", "quire.meaning.model.object-type/v1"),
            construct("population", "quire.meaning.model.population/v1")
        ],
        "occurrences": [], "extensions": [],
        "types": [{
            "identity": NODE, "displayName": "Node",
            "kind": {"module": DOMAIN, "name": "entity"},
            "roles": [], "origin": generated(NODE),
            "constraints": [], "extensions": [], "unknownPolicy": "reject", "supertypes": [],
            "fields": [
                field("next", 1, 1, false, true, "required"),
                field("maybe", 1, 1, false, true, "optional"),
                field("many", 0, 2, true, false, "required"),
                field("empty", 0, 0, true, false, "required")
            ],
            "relationships": [],
            "operations": [{
                "identity": format!("{NODE}/visit"), "name": "visit",
                "params": [], "pre": [], "post": [],
                "origin": {"source": {"sourceIdentity": SOURCE, "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
                "frame": {"modifies": [], "creates": [], "deletes": []}
            }]
        }],
        "populations": [{
            "identity": POPULATION, "displayName": "CollectionNodes",
            "kind": {"module": DOMAIN, "name": "population"},
            "members": [NODE], "extent": "closed", "origin": generated(POPULATION)
        }]
    })
}

/// The declaring type of the claim's `reaches_field` member.
fn declaring_type(authored: &Authored) -> CheckedNodeId {
    authored
        .package
        .resolved_graph()
        .nodes
        .iter()
        .find_map(|node| match &node.body {
            CheckedTerm::Application { operation, .. }
                if operation.identity() == Some("quire.op.model.reaches_field") =>
            {
                operation.member_declaration().ok().flatten().cloned()
            }
            _ => None,
        })
        .expect("the field member's declaring type")
}

/// The generated harness of a field-edge claim. The generator needs QSL to link the
/// `reaches_field` application to its population (QSL-1904); on a package without the link it
/// refuses, so these installed-Kani controls stop here with that reason instead of an opaque
/// "claim is supported" failure.
fn harness(authored: &Authored, path: &[usize]) -> crate::KaniGraphObligationHarness {
    let application = authored
        .package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| {
            matches!(&node.body, CheckedTerm::Application { operation, .. }
                if operation.identity() == Some("quire.op.model.reaches_field"))
        })
        .expect("a field-edge application")
        .node_id
        .clone();
    assert!(
        matches!(
            authored.package.reaches_population(&application),
            Ok(Some(_))
        ),
        "this control needs QSL to link reaches_field to its population (QSL-1904)"
    );
    super::kani_graph_private::harness(authored, path)
}

fn authored(condition: &str, maximum: u64) -> Authored {
    authored_over(domain(), condition, maximum, &["Graph"])
}

/// Every object reaches some object through `end`: true exactly when every live object has an
/// edge, which only a field whose member type forces one guarantees.
fn every_object_has_an_edge(end: &str) -> String {
    format!(
        "forall(a in M::CollectionNodes : exists(b in M::CollectionNodes : reaches(a, b, {end})))"
    )
}

/// The claim's reachability application, by argument path through the `forall` and `exists` binders.
const EVERY_OBJECT_PATH: [usize; 2] = [1, 1];

fn assert_verified(authored: &Authored, path: &[usize], label: &str) {
    let healthy = harness(authored, path);
    assert!(
        matches!(
            run_outcome(&healthy, label),
            crate::kani::classify::KaniRunOutcome::Verified
        ),
        "{label}"
    );
}

/// The authored clause over each drawn graph, no Kani: a hand-authored graph is put before QSL
/// and the clause judges it, for each member type's own snapshot form. A forced `Reference` to
/// itself violates `not reaches(self, self, next)`, an absent `Option` satisfies it, a present
/// `Option` self edge violates it and a `Sequence` of two edges has one of each target.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn qsl_replay_judges_a_concrete_field_graph_by_the_authored_clause() {
    let object = |_: &str, authored: &Authored| declaring_type(authored);
    for (field, edges, violated) in [
        ("next", vec![vec![0]], true),
        ("maybe", vec![vec![]], false),
        ("maybe", vec![vec![0]], true),
        ("many", vec![vec![0, 0]], true),
        ("many", vec![vec![]], false),
    ] {
        let authored = authored(&format!("not reaches(self, self, {field})"), 1);
        let mut inputs = graph_inputs(&authored, "unused");
        inputs.with_field_witness(
            GraphWitness {
                objects: 1,
                start: 0,
                edges,
                forward: true,
            },
            object(field, &authored),
            field,
        );
        let result = StateClauseReplay::new_graph_invariant(inputs)
            .expect("the replay is built")
            .replay()
            .expect("QSL settles the graph replay");
        if violated {
            assert_reproduced(&result);
        } else {
            assert_clause_holds(&result);
        }
    }
}

/// The negated claim: `not reaches(self, self, next)` over a forced single `Reference` at
/// population 1 is false on the only admitted graph. Kani falsifies the unmutated harness, and
/// QSL replays the same authored clause on the decoded graph and reproduces the violation.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_negated_field_claim_falsifies_and_qsl_replay_reproduces_it() {
    let authored = authored("not reaches(self, self, next)", 1);
    let healthy = harness(&authored, &[0]);
    let counterexample = run_falsified(&healthy, "field-negated");
    let (witness, result) = replay(&authored, &healthy, &counterexample, "unused");
    assert_eq!((witness.objects, witness.edges.clone()), (1, vec![vec![0]]));
    assert_reproduced(&result);
}

/// Healthy: a forced `Reference` makes every object reach some object, over every population of
/// zero to three. The same claim over the `Option` field `maybe` is false (an object with no
/// edge), which is the member type's multiplicity being read, not assumed: Kani falsifies it and
/// QSL reaches the same `Violation` verdict on the decoded graph (the claim is quantified, so
/// QSL's settlement is `Inconclusive` with only the derived witness).
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_the_field_multiplicity_decides_the_every_object_claim() {
    assert_verified(
        &authored(&every_object_has_an_edge("next"), 3),
        &EVERY_OBJECT_PATH,
        "field-every-object-next",
    );
    let authored = authored(&every_object_has_an_edge("maybe"), 3);
    let healthy = harness(&authored, &EVERY_OBJECT_PATH);
    assert_eq!(healthy.identity.edge.lower, 0);
    let counterexample = run_falsified(&healthy, "field-every-object-maybe");
    let (witness, result) = replay(&authored, &healthy, &counterexample, "unused");
    assert!(witness.edges.iter().any(Vec::is_empty), "{witness:?}");
    let (settlement, category, cause) = settled(&result);
    assert_eq!(
        (settlement, category),
        (WitnessSettlement::Inconclusive, Category::Violation)
    );
    assert!(
        matches!(
            cause,
            Some(DisagreementCause::Witness {
                given: None,
                derived: Some(_),
                ..
            })
        ),
        "{cause:?}"
    );
}

/// Zero-length mutant: every object reaches itself, so an isolated `start = target` reads
/// reachable and `not reaches(self, self, empty)` fails over the always-empty field. The
/// authored clause holds natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_zero_length_path_mutant_falsifies_a_field_claim_and_qsl_replay_disagrees() {
    let authored = authored("not reaches(self, self, empty)", 3);
    let healthy = harness(&authored, &[0]);
    assert_verified(&authored, &[0], "field-empty-healthy");
    let mutant = mutated(
        &healthy,
        "let mut paths = [[false; 3]; 3];",
        "let mut paths = [[false; 3]; 3];\n for own in 0..3 { paths[own][own] = true; }",
    );
    let counterexample = run_falsified(&mutant, "field-zero-length");
    let (witness, result) = replay(&authored, &mutant, &counterexample, "unused");
    assert!(witness.edges.iter().all(Vec::is_empty), "{witness:?}");
    assert_clause_holds(&result);
}

/// Rejected-self-loop mutant: the closure drops an edge from an object to itself, so the forced
/// `Reference` self edge at population 1 reads unreachable and `reaches(self, self, next)` fails.
/// The authored clause holds natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_rejected_self_loop_mutant_falsifies_a_field_claim_and_qsl_replay_disagrees() {
    let authored = authored("reaches(self, self, next)", 1);
    assert_verified(&authored, &[], "field-self-loop-healthy");
    let healthy = harness(&authored, &[]);
    let mutant = mutated(
        &healthy,
        "paths[row][edges[row][edge] as usize] = true;",
        "if edges[row][edge] as usize != row { paths[row][edges[row][edge] as usize] = true; }",
    );
    let counterexample = run_falsified(&mutant, "field-self-loop");
    let (witness, result) = replay(&authored, &mutant, &counterexample, "unused");
    assert_eq!(witness.edges, vec![vec![0]]);
    assert_clause_holds(&result);
}

/// Healthy transitivity over each member type at population 3: it holds on every graph, whatever
/// the edge draw, so `Verified` evidences the closure and the lowering of each field's domain.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_healthy_transitivity_verifies_over_each_field_member_type() {
    for field in ["next", "maybe", "many"] {
        assert_verified(
            &authored(&transitivity(field), 3),
            &CONCLUSION,
            &format!("field-transitivity-{field}"),
        );
    }
}

/// Harness mutant over a bounded `Sequence` field: the closure step is removed, so a two-step
/// path with no direct edge falsifies transitivity while the authored clause holds natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_transitivity_without_closure_mutant_falsifies_a_field_claim_and_qsl_replay_disagrees()
{
    let authored = authored(&transitivity("many"), 3);
    let healthy = harness(&authored, &CONCLUSION);
    let mutant = mutated(&healthy, CLOSURE_STEP, "paths[from][to] = paths[from][to];");
    let counterexample = run_falsified(&mutant, "field-transitivity-no-closure");
    let (_, result) = replay(&authored, &mutant, &counterexample, "unused");
    assert_clause_holds(&result);
}

fn scalar(name: &str, lower: u64, upper: u64, ordered: bool, presence: &str) -> Value {
    let mut member = field(name, lower, upper, ordered, false, presence);
    member["typeRef"] = json!("ix://quire/native/Boolean");
    member
}

/// The fixture's `Node` with one field of each shape the member-type mapping reads, and a second
/// object type to refer to.
fn edge_domain() -> Vec<u8> {
    let mut document = domain_value();
    let other = "ix://ir268.test/domain/Other".to_owned();
    let mut other_type = document["types"][0].clone();
    other_type["identity"] = json!(other);
    other_type["displayName"] = json!("Other");
    other_type["fields"] = json!([]);
    other_type["operations"] = json!([]);
    let mut unbounded = field("loose", 0, 0, true, false, "required");
    unbounded["multiplicity"] = json!({"lower": 0, "ordered": true, "unique": false});
    let mut across = field("across", 1, 1, false, true, "required");
    across["typeRef"] = json!(other);
    let fields = document["types"][0]["fields"].as_array_mut().unwrap();
    fields.extend([
        field("set", 0, 2, false, true, "required"),
        field("oset", 0, 2, true, true, "required"),
        field("bag", 0, 2, false, false, "required"),
        unbounded,
        across,
        scalar("count", 1, 1, false, "required"),
        scalar("optscalar", 1, 1, false, "optional"),
        scalar("flags", 0, 2, true, "required"),
    ]);
    document["types"].as_array_mut().unwrap().push(other_type);
    encode(&document)
}

/// A field edge's lower, upper, ordered, unique and field name.
type Shape = (u64, Option<u64>, bool, bool, Option<String>);

fn edge_of(authored: &Authored, name: &str) -> Result<Shape, GraphRefusal> {
    let node = declaring_type(authored);
    field_edge(&authored.package, &node, &node, name).map(|edge| {
        (
            edge.lower,
            edge.upper,
            edge.ordered,
            edge.unique,
            edge.field.map(|name| name.to_string()),
        )
    })
}

/// The pure field-edge mapping over a QSL-emitted, IR-admitted package, with the declaring type
/// and name read from the claim's own `reaches_field` member: it runs with or without QSL's
/// population link, which `field_edge` does not read.
#[trace("TC-025", "FR-015-AC-98")]
#[trace("TC-025", "FR-015-AC-105")]
#[test]
fn the_member_type_decides_the_field_edge_domain() {
    let authored = authored_over(
        edge_domain(),
        "not reaches(self, self, next)",
        3,
        &["Graph"],
    );
    for (name, lower, upper, ordered, unique) in [
        ("next", 1, 1, false, true),
        ("maybe", 0, 1, false, true),
        ("many", 0, 2, true, false),
        ("set", 0, 2, false, true),
        ("oset", 0, 2, true, true),
        ("bag", 0, 2, false, false),
    ] {
        assert_eq!(
            edge_of(&authored, name),
            Ok((lower, Some(upper), ordered, unique, Some(name.to_owned()))),
            "{name}"
        );
    }
}

/// An unbounded `Collection`, an absent field and a field that is not a reference edge each
/// refuse with their own typed reason and no edge, and a reference to another object type is not
/// an edge of the population's type.
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn field_edge_refusals_are_typed_and_name_their_input() {
    let authored = authored_over(
        edge_domain(),
        "not reaches(self, self, next)",
        3,
        &["Graph"],
    );
    let node = declaring_type(&authored);
    assert_eq!(
        edge_of(&authored, "loose"),
        Err(GraphRefusal::RequiresBound(node.clone()))
    );
    assert_eq!(
        edge_of(&authored, "absent"),
        Err(GraphRefusal::Unsupported(Reason::FieldNotAdmitted))
    );
    for name in ["count", "optscalar", "flags"] {
        assert_eq!(
            edge_of(&authored, name),
            Err(GraphRefusal::Unsupported(Reason::FieldEdgeNotReference)),
            "{name}"
        );
    }
    assert_eq!(
        edge_of(&authored, "across"),
        Err(GraphRefusal::Unsupported(Reason::PopulationTypeMismatch))
    );
    // A population of another object type is not the field's.
    let clause = authored.clause.clone();
    assert_eq!(
        field_edge(&authored.package, &clause, &node, "next").map(|_| ()),
        Err(GraphRefusal::Unsupported(Reason::PopulationTypeMismatch))
    );
}
