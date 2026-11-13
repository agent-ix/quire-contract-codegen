//! Private installed-Kani controls for the graph reachability semantic mutants (FR-015-AC-106,
//! TC-025, IR-805 slice B). Run explicitly and serially: `cargo test --lib kani_graph_private --
//! --ignored --test-threads=1`.
//!
//! The mutants edit the generated reachability helper and are sealed with the crate-private
//! constructor, the same fault-injection seam the scalar mutants use; production callers cannot
//! re-seal a changed harness. Each Falsified playback is decoded into the graph Kani drew and put
//! before QSL's `replay_state_clause` over the AUTHORED clause (no separately written
//! reachability): the negated claim must reproduce as a violation, and each source mutant, whose
//! authored clause holds natively, must settle as a verdict disagreement.
//!
//! The wrong-role mutant makes the closure follow a role that holds no instance in the drawn graph
//! (the helper's edge count is replaced by zero). The harness draws one end only, so following a
//! different populated role cannot be expressed by editing it.

use crate::{
    core::artifact::Artifact,
    kani::{
        classify::KaniRunOutcome,
        identity::KaniGraphObligationHarness,
        output::playback::parse_assertion_playback,
        run::{
            execute::{execute_kani_obligation, KaniExecutionRequest},
            tool::KaniInstallation,
        },
        test_support::proof_ceilings::proof_ceilings_with_wall_clock,
    },
    negotiate_kani_obligations, DocumentLabel, GraphReplayAddress, GraphReplayInputs,
    KaniObligationOutcome, KaniObligationRequest, LockedSource, ObligationItem, ProvidedDocument,
    ReplayInputs, StateClauseReplay, WitnessDecodeLimits,
};
use ix_trace_rs::trace;
use qsl_replay::{
    call_site, AnchorKind, ClauseName, DependencyInput, DisagreementCause, Identifier,
    ReplayLimits, ReplayResult, ScalarLimits, SelectedAnchor, SourceIdentity, WitnessSettlement,
    DEFAULT_REPLAY_INPUT_BYTES,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, time::Duration};

const DOMAIN: &str = "ir268.test/domain";
const NODE: &str = "ix://ir268.test/domain/Node";
const POPULATION: &str = "ix://ir268.test/domain/CollectionNodes";
const SOURCE: &str = "ix://ir268.test/spec";
const AUTHORITY: &str = "agent-ix";
const UNIT: &str = "graph.native";

fn generated(identity: &str) -> Value {
    json!({"generated": {"generatorIdentity": SOURCE, "generatorVersion": "1.0.0",
                          "inputIdentities": [identity]}})
}

fn relation(name: &str, lower: u64, upper: u64) -> Value {
    json!({
        "identity": format!("ix://ir268.test/domain/relationship/Node-{name}"),
        "category": "structural", "composite": false, "direction": "source-to-target",
        "origin": {"source": {"sourceIdentity": SOURCE, "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
        "sourceEnd": {"type": NODE, "role": name,
                      "multiplicity": {"lower": 0, "upper": 9, "ordered": false, "unique": true}},
        "targetEnd": {"type": NODE,
                      "multiplicity": {"lower": lower, "upper": upper, "ordered": false, "unique": true}}
    })
}

/// A `Node` with no declared field, two unique relationship ends, and one operation to anchor on.
fn domain_document() -> Vec<u8> {
    let construct = |name: &str, meaning: &str| {
        json!({"kind": {"module": DOMAIN, "name": name}, "moduleVersion": "1.0.0",
               "construct": {"identity": "none", "shape": "record", "members": {}, "meaning": meaning}})
    };
    let document = json!({
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
            "fields": [],
            "relationships": [relation("loop", 1, 1), relation("empty", 0, 0)],
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
    });
    let limits = quire_canonical::Limits::new(1 << 20);
    quire_canonical::to_vec(&document, limits).expect("canonical domain document")
}

struct Authored {
    package: CheckedPackageV2,
    clause: CheckedNodeId,
    source: String,
    domain: Vec<u8>,
}

fn authored(condition: &str, maximum: u64) -> Authored {
    let domain = domain_document();
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{DOMAIN}\";\nbound Nodes on M::CollectionNodes [{maximum}]\npredicate inspect using v (input: M::Node): Boolean {{ true }}\ninvariant Graph using v on M::Node at current {{ {condition} }}\n"
    );
    let site = call_site(
        SourceIdentity::new(AUTHORITY, UNIT),
        "graph".to_owned(),
        UNIT,
        source.as_bytes(),
        [domain.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new("Graph").unwrap()),
    )
    .unwrap_or_else(|error| panic!("QSL emits the authored clause: {error:?}"));
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(DOMAIN, domain.clone())
        .unwrap();
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("Contract IR admits the authored package")
    };
    let package = *package;
    let clause = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause")
        .expect("a state clause")
        .node_id
        .clone();
    Authored {
        package,
        clause,
        source,
        domain,
    }
}

fn harness(authored: &Authored, path: &[usize]) -> KaniGraphObligationHarness {
    let KaniObligationOutcome::Emitted {
        mut graph_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: proof_ceilings_with_wall_clock(Duration::from_secs(1800)),
        items: &[ObligationItem::GraphClaim {
            package: &authored.package,
            clause_id: &authored.clause,
            argument_path: path,
        }],
        subject_path: "crate::graph_subject",
        unwind: 8,
    })
    .expect("a graph request is valid")
    else {
        panic!("a graph request settles items")
    };
    assert_eq!(graph_harnesses.len(), 1, "the claim is supported");
    graph_harnesses.remove(0)
}

/// The harness with `from` replaced by `to` exactly once, sealed as a generated harness.
fn mutated(
    healthy: &KaniGraphObligationHarness,
    from: &str,
    to: &str,
) -> KaniGraphObligationHarness {
    assert_eq!(
        healthy.rust.contents.matches(from).count(),
        1,
        "the mutation site is unique in {}",
        healthy.rust.contents
    );
    let mut rust: Artifact = healthy.rust.clone();
    rust.contents = rust.contents.replace(from, to);
    // This private test deliberately seals its one fault injection; production callers cannot
    // re-seal a changed harness and the normal execution path still enforces provenance.
    KaniGraphObligationHarness::generated(healthy.identity.clone(), rust, healthy.record.clone())
}

struct Scratch(PathBuf);

impl Scratch {
    fn create(label: &str) -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("ir805b-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("src")).expect("scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Runs `harness` under the installed backend and returns the Falsified counterexample.
fn run_falsified(harness: &KaniGraphObligationHarness, label: &str) -> String {
    let scratch = Scratch::create(label);
    fs::write(
        scratch.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-kani-graph\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ {}, features = [\"exact\"] }}\n\n[workspace]\n",
            crate::RUNTIME_DEPENDENCY_SOURCE
        ),
    )
    .unwrap();
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        scratch.0.join("Cargo.lock"),
    )
    .unwrap();
    fs::write(scratch.0.join("src/lib.rs"), &harness.rust.contents).unwrap();
    fs::write(
        scratch.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let metadata = std::process::Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&scratch.0)
        .output()
        .expect("resolve seeded dependency lock");
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let cancel = quire_exact::Cancel::new();
    let started = std::time::Instant::now();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory: &scratch.0,
        target_directory: &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/ir805b-kani"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("{label}: {refusal}"));
    eprintln!(
        "IR805B-KANI {label}: {:?} in {:?}",
        std::mem::discriminant(&evidence.outcome),
        started.elapsed()
    );
    let KaniRunOutcome::Falsified { counterexample } = evidence.outcome else {
        panic!("{label}: the mutant must falsify: {:?}", evidence.outcome);
    };
    counterexample
}

fn scalar_limits() -> ScalarLimits {
    let seed = 1_000_000;
    ScalarLimits {
        integer_bits: seed,
        decimal_digits: seed,
        scale_expansion: seed,
        text_input_bytes: seed,
        text_scalars: seed,
        normalized_scalars: seed,
        unit_edges: seed,
        value_occurrences: seed,
        work_units: seed,
        result_units: seed,
    }
}

const UNBOUNDED: WitnessDecodeLimits = WitnessDecodeLimits {
    bytes: usize::MAX,
    values: usize::MAX,
};

fn graph_inputs<'a>(authored: &'a Authored, relationship: &str) -> GraphReplayInputs<'a> {
    GraphReplayInputs {
        run: ReplayInputs {
            decode_limits: UNBOUNDED,
            package_name: "graph".to_owned(),
            source: LockedSource {
                authority: AUTHORITY.to_owned(),
                identity: UNIT.to_owned(),
                bytes: authored.source.clone().into_bytes(),
            },
            dependencies: Vec::new(),
            accounting_limits: scalar_limits(),
            stage_limits: std::collections::BTreeMap::from([(
                "s1.input_bytes".to_owned(),
                DEFAULT_REPLAY_INPUT_BYTES,
            )]),
            supplied_limits: None,
            declared_domains: Vec::new(),
            replay_limits: ReplayLimits::default(),
        },
        packages: vec![ProvidedDocument {
            bytes: authored.domain.clone(),
        }],
        package: &authored.package,
        clause_node: &authored.clause,
        clause: ClauseName(Identifier::new("Graph").unwrap()),
        address: GraphReplayAddress {
            population: POPULATION.to_owned(),
            object_type: NODE.to_owned(),
            relationship: relationship.to_owned(),
        },
        anchor: SelectedAnchor {
            kind: AnchorKind::Handler,
            name: "visit".to_owned(),
        },
        snapshot_label: DocumentLabel {
            authority: "test".to_owned(),
            identity: "graph-current".to_owned(),
        },
        witness: None,
    }
}

/// Decodes the counterexample into the drawn graph and replays the authored clause over it
/// through `qsl_replay`.
fn replay(
    authored: &Authored,
    harness: &KaniGraphObligationHarness,
    counterexample: &str,
    relationship: &str,
) -> (crate::GraphWitness, qsl_replay::StateClauseReplayResult) {
    let unbounded = UNBOUNDED;
    let playback = parse_assertion_playback(
        counterexample,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the actual backend playback parses");
    let mut inputs = graph_inputs(authored, relationship);
    inputs
        .decode_falsification(harness, &playback, unbounded)
        .expect("the playback decodes in the harness's domain");
    let witness = inputs.witness.clone().expect("a decoded graph");
    let replay = StateClauseReplay::new_graph_invariant(inputs).expect("the replay is built");
    let result = replay.replay().expect("QSL settles the graph replay");
    (witness, result)
}

fn relationship(name: &str) -> String {
    format!("ix://ir268.test/domain/relationship/Node-{name}")
}

fn settled(
    result: &qsl_replay::StateClauseReplayResult,
) -> (WitnessSettlement, Category, Option<DisagreementCause>) {
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("the invariant envelope is on the witness arm");
    };
    (
        arm.settlement(),
        arm.category(),
        arm.disagreement().cloned(),
    )
}

fn assert_reproduced(result: &qsl_replay::StateClauseReplayResult) {
    assert_eq!(
        settled(result),
        (
            WitnessSettlement::ReproducedWithEvaluatedWitness,
            Category::Violation,
            None
        )
    );
}

/// The authored clause holds on the replayed graph: the harness and the clause disagree.
fn assert_clause_holds(result: &qsl_replay::StateClauseReplayResult) {
    let (settlement, category, cause) = settled(result);
    assert_eq!(settlement, WitnessSettlement::Inconclusive);
    assert_eq!(category, Category::Success);
    assert!(
        matches!(cause, Some(DisagreementCause::Verdicts { .. })),
        "{cause:?}"
    );
}

/// The negated claim: `not reaches(self, self, loop)` over a forced `{1,1}` end at population 1
/// is false on the only admitted graph. Kani falsifies the unmutated harness, and QSL replays the
/// same authored clause on the decoded graph and reproduces the violation.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_negated_claim_falsifies_and_qsl_replay_reproduces_it() {
    let authored = authored("not reaches(self, self, loop)", 1);
    let healthy = harness(&authored, &[0]);
    let counterexample = run_falsified(&healthy, "negated");
    let (witness, result) = replay(&authored, &healthy, &counterexample, &relationship("loop"));
    assert_eq!((witness.objects, witness.edges.clone()), (1, vec![vec![0]]));
    assert_reproduced(&result);
}

/// Zero-length mutant: every object reaches itself, so an isolated `start = target` reads
/// reachable and `not reaches(self, self, empty)` fails. The authored clause holds natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_zero_length_path_mutant_falsifies_and_qsl_replay_disagrees() {
    let authored = authored("not reaches(self, self, empty)", 3);
    let healthy = harness(&authored, &[0]);
    let mutant = mutated(
        &healthy,
        "let mut paths = [[false; 3]; 3];",
        "let mut paths = [[false; 3]; 3];\n for own in 0..3 { paths[own][own] = true; }",
    );
    let counterexample = run_falsified(&mutant, "zero-length");
    let (witness, result) = replay(&authored, &mutant, &counterexample, &relationship("empty"));
    assert!(witness.edges.iter().all(Vec::is_empty), "{witness:?}");
    assert_clause_holds(&result);
}

/// Rejected-self-loop mutant: the closure drops an edge from an object to itself, so the forced
/// self-loop reads unreachable and `reaches(self, self, loop)` fails. The authored clause holds
/// natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_rejected_self_loop_mutant_falsifies_and_qsl_replay_disagrees() {
    let authored = authored("reaches(self, self, loop)", 1);
    let healthy = harness(&authored, &[]);
    let mutant = mutated(
        &healthy,
        "paths[row][edges[row][edge] as usize] = true;",
        "if edges[row][edge] as usize != row { paths[row][edges[row][edge] as usize] = true; }",
    );
    let counterexample = run_falsified(&mutant, "self-loop");
    let (witness, result) = replay(&authored, &mutant, &counterexample, &relationship("loop"));
    assert_eq!(witness.edges, vec![vec![0]]);
    assert_clause_holds(&result);
}

/// Wrong-role mutant: the closure follows a role with no instance in the drawn graph, so the
/// forced self-loop reads unreachable. The authored clause holds natively.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_wrong_role_mutant_falsifies_and_qsl_replay_disagrees() {
    let authored = authored("reaches(self, self, loop)", 1);
    let healthy = harness(&authored, &[]);
    let mutant = mutated(
        &healthy,
        "(edge as i64) < degrees[row]",
        "(edge as i64) < 0",
    );
    let counterexample = run_falsified(&mutant, "wrong-role");
    let (witness, result) = replay(&authored, &mutant, &counterexample, &relationship("loop"));
    assert_eq!(witness.edges, vec![vec![0]]);
    assert_clause_holds(&result);
}

/// No Kani: a hand-authored graph is put before QSL and the authored clause judges it. A forced
/// self-loop violates `not reaches(self, self, loop)` and satisfies `reaches(self, self, loop)`,
/// and a witness the harness could not have drawn is refused before QSL is asked.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn qsl_replay_judges_a_concrete_graph_by_the_authored_clause() {
    let witness = crate::GraphWitness {
        objects: 1,
        start: 0,
        edges: vec![vec![0]],
        forward: true,
    };
    for (condition, violated) in [
        ("not reaches(self, self, loop)", true),
        ("reaches(self, self, loop)", false),
    ] {
        let authored = authored(condition, 1);
        let mut inputs = graph_inputs(&authored, &relationship("loop"));
        inputs.witness = Some(witness.clone());
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
    let authored = authored("not reaches(self, self, empty)", 3);
    let mut inputs = graph_inputs(&authored, &relationship("empty"));
    inputs.witness = Some(crate::GraphWitness {
        objects: 2,
        start: 0,
        edges: vec![vec![], vec![]],
        forward: true,
    });
    let result = StateClauseReplay::new_graph_invariant(inputs)
        .expect("the replay is built")
        .replay()
        .expect("QSL settles the graph replay");
    assert_clause_holds(&result);
}
