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
//! IR-805 slice E adds the multi-reachability controls: the transitivity claim over a `{0,3}` end
//! (`TRANSITIVITY`), whose three reachability applications are quantifier-bound, is `Verified` by
//! the unmutated harness, its harness mutant (the transitive closure removed, so reachability is
//! the selected edges only) is Falsified and the unchanged authored clause holds natively on the
//! decoded graph, and its claim mutant (the conclusion reversed) is Falsified and QSL reaches the
//! same `Violation` verdict (its evaluated-witness settlement is not reached; see that test).
//!
//! There is NO wrong-role mutant. The harness draws one end only, so a closure that follows
//! another role cannot be expressed by editing it, and a closure that follows no edge is the
//! rejected-self-loop mutant again. AC-106's "another field or role" clause is NOT evidenced.
//!
//! These are lib tests, so `make kani-gate` (which runs `--test it` with the Kani filters) does not
//! run them; run them with the command above inside the flock and the memory scope.

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
    CheckedNodeId, CheckedOccurrence, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult,
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

fn relation(name: &str, lower: u64, upper: u64, unique: bool) -> Value {
    json!({
        "identity": format!("ix://ir268.test/domain/relationship/Node-{name}"),
        "category": "structural", "composite": false, "direction": "source-to-target",
        "origin": {"source": {"sourceIdentity": SOURCE, "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
        "sourceEnd": {"type": NODE, "role": name,
                      "multiplicity": {"lower": 0, "upper": 9, "ordered": false, "unique": unique}},
        "targetEnd": {"type": NODE,
                      "multiplicity": {"lower": lower, "upper": upper, "ordered": false, "unique": unique}}
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
            "relationships": [
                relation("loop", 1, 1, true),
                relation("empty", 0, 0, true),
                relation("kids", 0, 3, true),
                relation("litter", 0, 3, false)
            ],
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
    occurrence: CheckedOccurrence,
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
    let node = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause")
        .expect("a state clause");
    let (clause, occurrence) = (node.node_id.clone(), node.occurrences[0].clone());
    Authored {
        package,
        clause,
        occurrence,
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
            node_id: &authored.clause,
            occurrence: &authored.occurrence,
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
    let KaniRunOutcome::Falsified { counterexample } = run_outcome(harness, label) else {
        panic!("{label}: the run must falsify");
    };
    counterexample
}

fn run_outcome(harness: &KaniGraphObligationHarness, label: &str) -> KaniRunOutcome {
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
    evidence.outcome
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
    GraphReplayInputs::new(
        ReplayInputs {
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
        vec![ProvidedDocument {
            bytes: authored.domain.clone(),
        }],
        &authored.package,
        &authored.clause,
        ClauseName(Identifier::new("Graph").unwrap()),
        GraphReplayAddress {
            population: POPULATION.to_owned(),
            object_type: NODE.to_owned(),
            relationship: relationship.to_owned(),
        },
        SelectedAnchor {
            kind: AnchorKind::Handler,
            name: "visit".to_owned(),
        },
        DocumentLabel {
            authority: "test".to_owned(),
            identity: "graph-current".to_owned(),
        },
    )
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
    let witness = inputs.witness().cloned().expect("a decoded graph");
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

/// No Kani: a hand-authored graph is put before QSL and the authored clause judges it. A forced
/// self-loop violates `not reaches(self, self, loop)` and satisfies `reaches(self, self, loop)`.
/// The witness is set through a test seam: a public caller can only obtain one by decoding a
/// playback.
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
        inputs.with_witness(witness.clone());
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
    inputs.with_witness(crate::GraphWitness {
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

const GRAPH_CHECK: &str = "\"the symbolic graph satisfies the admitted clause condition\"";

/// Hand-authored Kani playback text for `harness`, one little-endian `i64` per draw.
fn playback_text(harness: &KaniGraphObligationHarness, check: &str, values: &[i64]) -> String {
    let entries: String = values
        .iter()
        .map(|value| {
            let bytes = value
                .to_le_bytes()
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("        // {value}\n        vec![{bytes}],\n")
        })
        .collect();
    format!(
        "/// Test generated for harness `{}::{}`\n/// Check for `assertion`: \"{check}\"\n#[test]\nfn kani_concrete_playback_check_1() {{\n    let concrete_vals: Vec<Vec<u8>> = vec![\n{entries}    ];\n    kani::concrete_playback_run(concrete_vals, check);\n}}\n",
        harness.identity.module_symbol.as_str(),
        harness.identity.harness_symbol.as_str(),
    )
}

fn decode(
    inputs: &mut GraphReplayInputs<'_>,
    harness: &KaniGraphObligationHarness,
    check: &str,
    values: &[i64],
) -> Result<(), crate::StateClauseReplayError> {
    let text = playback_text(harness, check, values);
    let playback = parse_assertion_playback(
        &text,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the hand-authored playback parses");
    inputs.decode_falsification(harness, &playback, UNBOUNDED)
}

/// Slot layout for a population of 3 and an end of upper bound 3: `object_count`, `start`,
/// `target`, then `degree` and three edge slots per row.
fn slots(count: i64, start: i64, target: i64, rows: [[i64; 4]; 3]) -> Vec<i64> {
    let mut slots = vec![count, start, target];
    slots.extend(rows.into_iter().flatten());
    slots
}

const EMPTY_ROW: [i64; 4] = [0, 0, 0, 0];

/// Every refusal of `decode_falsification`, on a unique `{0,3}` end at population 3, and the `{1,1}`
/// lower bound at population 1.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn decode_falsification_refuses_every_playback_the_harness_could_not_draw() {
    use crate::StateClauseReplayError as Error;
    let authored = authored("not reaches(self, self, kids)", 3);
    let harness = harness(&authored, &[0]);
    let fresh = || graph_inputs(&authored, &relationship("kids"));
    let ok = slots(1, 0, 0, [[1, 0, 0, 0], EMPTY_ROW, EMPTY_ROW]);
    let mut inputs = fresh();
    decode(&mut inputs, &harness, GRAPH_CHECK, &ok).expect("a drawable graph decodes");
    let witness = inputs.witness().unwrap();
    assert_eq!((witness.objects, witness.start), (1, 0));
    assert_eq!(witness.edges, vec![vec![0]]);
    // A second decode is refused.
    assert!(matches!(
        decode(&mut inputs, &harness, GRAPH_CHECK, &ok),
        Err(Error::PlaybackAlreadySupplied)
    ));
    let refused = |values: Vec<i64>| decode(&mut fresh(), &harness, GRAPH_CHECK, &values);
    for (label, values) in [
        ("empty population", slots(0, 0, 0, [EMPTY_ROW; 3])),
        ("count above N", slots(4, 0, 0, [EMPTY_ROW; 3])),
        ("negative count", slots(-1, 0, 0, [EMPTY_ROW; 3])),
        ("start out of range", slots(1, 1, 1, [EMPTY_ROW; 3])),
        (
            "degree above upper",
            slots(1, 0, 0, [[4, 0, 0, 0], EMPTY_ROW, EMPTY_ROW]),
        ),
        (
            "target out of range",
            slots(1, 0, 0, [[1, 1, 0, 0], EMPTY_ROW, EMPTY_ROW]),
        ),
        (
            "repeated target on a unique end",
            slots(2, 0, 0, [[2, 1, 1, 0], EMPTY_ROW, EMPTY_ROW]),
        ),
    ] {
        assert!(
            matches!(refused(values), Err(Error::GraphWitnessNotInDomain)),
            "{label}"
        );
    }
    assert!(matches!(
        refused(slots(2, 0, 1, [EMPTY_ROW; 3])),
        Err(Error::GraphStartIsNotTarget)
    ));
    // The assertion must be the harness's own.
    assert!(matches!(
        decode(&mut fresh(), &harness, "\"another assertion\"", &ok),
        Err(Error::AssertionMismatch)
    ));
    // A harness whose public fields changed since generation is refused.
    let mut tampered = harness.clone();
    tampered.rust.contents.push_str("\n// changed");
    assert!(matches!(
        decode(&mut fresh(), &tampered, GRAPH_CHECK, &ok),
        Err(Error::Decode(_))
    ));
    // A harness of another clause is refused.
    let other_node = authored
        .package
        .resolved_graph()
        .nodes
        .iter()
        .map(|node| &node.node_id)
        .find(|node| **node != authored.clause)
        .expect("another node");
    let mut elsewhere = fresh();
    elsewhere.clause_node = other_node;
    assert!(matches!(
        decode(&mut elsewhere, &harness, GRAPH_CHECK, &ok),
        Err(Error::HarnessMismatch { .. })
    ));
    // The edge lower bound: a `{1,1}` end draws one edge per live object.
    let forced = self::authored("not reaches(self, self, loop)", 1);
    let forced_harness = self::harness(&forced, &[0]);
    let mut inputs = graph_inputs(&forced, &relationship("loop"));
    assert!(matches!(
        decode(&mut inputs, &forced_harness, GRAPH_CHECK, &[1, 0, 0, 0, 0]),
        Err(Error::GraphWitnessNotInDomain)
    ));
    decode(&mut inputs, &forced_harness, GRAPH_CHECK, &[1, 0, 0, 1, 0])
        .expect("the forced self-loop decodes");
}

fn snapshot_instances(replay: &StateClauseReplay) -> Value {
    replay
        .wire
        .documents
        .iter()
        .filter_map(|bytes| serde_json::from_slice::<Value>(bytes).ok())
        .find(|document| document["format"] == "quire.state.snapshot/v1")
        .expect("the snapshot")["relationships"][0]["instances"]
        .clone()
}

/// The reverse direction swaps each instance's source and target, and QSL admits both
/// snapshots of a two-cycle; a non-unique end writes a repeated target as repeated instances,
/// which QSL either admits or refuses by itself (recorded below).
#[trace("TC-025", "FR-015-AC-106")]
#[test]
fn the_reverse_direction_and_a_non_unique_end_reach_the_snapshot() {
    let cycle = |forward| crate::GraphWitness {
        objects: 2,
        start: 0,
        edges: vec![vec![1], vec![0]],
        forward,
    };
    let authored = authored("not reaches(self, self, kids)", 3);
    let mut results = Vec::new();
    for forward in [true, false] {
        let mut inputs = graph_inputs(&authored, &relationship("kids"));
        inputs.with_witness(cycle(forward));
        let replay = StateClauseReplay::new_graph_invariant(inputs).expect("built");
        let instances = snapshot_instances(&replay);
        assert_eq!(
            instances[0]["source"]["key"],
            if forward { "n0" } else { "n1" }
        );
        assert_eq!(
            instances[0]["target"]["key"],
            if forward { "n1" } else { "n0" }
        );
        results.push(settled(&replay.replay().expect("QSL settles")));
    }
    // A cycle reaches itself in either direction: the same violation.
    assert_eq!(results[0], results[1]);
    assert_eq!(results[0].1, Category::Violation);

    let authored = self::authored("not reaches(self, self, litter)", 3);
    let mut inputs = graph_inputs(&authored, &relationship("litter"));
    inputs.with_witness(crate::GraphWitness {
        objects: 2,
        start: 0,
        edges: vec![vec![1, 1], vec![]],
        forward: true,
    });
    let replay = StateClauseReplay::new_graph_invariant(inputs).expect("built");
    assert_eq!(snapshot_instances(&replay).as_array().unwrap().len(), 2);
    // Object 1 has no edge, so object 0 does not reach itself: the clause holds.
    assert_clause_holds(&replay.replay().expect("QSL settles the repeated instances"));
}

/// Healthy controls on this fixture for the claims the mutants falsify: the unmutated harness
/// verifies, so each Falsified mutant run is the mutation's doing.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_healthy_controls_verify_on_the_slice_b_fixture() {
    for (label, condition, maximum, path) in [
        (
            "healthy-empty",
            "not reaches(self, self, empty)",
            3,
            vec![0],
        ),
        ("healthy-loop", "reaches(self, self, loop)", 1, vec![]),
    ] {
        let authored = authored(condition, maximum);
        let healthy = harness(&authored, &path);
        assert!(
            matches!(run_outcome(&healthy, label), KaniRunOutcome::Verified),
            "{label}"
        );
    }
}

/// The transitivity claim over the `{0,3}` unique `kids` end at population 3, written with three
/// quantifier-bound objects. The harness closure is transitive, so the healthy harness verifies.
const TRANSITIVITY: &str = "forall(a in M::CollectionNodes : forall(b in M::CollectionNodes : forall(c in M::CollectionNodes : (reaches(a, b, kids) and reaches(b, c, kids)) implies reaches(a, c, kids))))";

/// `reaches(a, c)`: the conclusion application, by argument path through the three binders.
const CONCLUSION: [usize; 4] = [1, 1, 1, 1];

/// The conclusion reversed, `reaches(c, a)`: a claim mutant that a chain graph violates.
const REVERSED: &str = "forall(a in M::CollectionNodes : forall(b in M::CollectionNodes : forall(c in M::CollectionNodes : (reaches(a, b, kids) and reaches(b, c, kids)) implies reaches(c, a, kids))))";

/// Healthy transitivity: a `{0,3}` end admits both reachable and unreachable graphs, and the claim
/// is false on a harness whose reachability is not transitive, so `Verified` carries meaning.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_healthy_transitivity_verifies_over_a_zero_to_three_end() {
    let authored = authored(TRANSITIVITY, 3);
    let healthy = harness(&authored, &CONCLUSION);
    assert_eq!(healthy.identity.edge.upper, 3);
    assert!(
        matches!(
            run_outcome(&healthy, "healthy-transitivity"),
            KaniRunOutcome::Verified
        ),
        "healthy transitivity"
    );
}

/// Harness mutant: the closure step is removed, so reachability is the selected edges only and a
/// two-step path with no direct edge falsifies transitivity. The authored clause is unchanged, and
/// QSL's replay of it on the decoded graph holds (the native closure is transitive), so the
/// verdicts disagree and the spurious counterexample is the detection.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_transitivity_without_closure_mutant_falsifies_and_qsl_replay_disagrees() {
    let authored = authored(TRANSITIVITY, 3);
    let healthy = harness(&authored, &CONCLUSION);
    let mutant = mutated(
        &healthy,
        "paths[from][to] = paths[from][to] || (paths[from][via] && paths[via][to]);",
        "paths[from][to] = paths[from][to];",
    );
    let counterexample = run_falsified(&mutant, "transitivity-no-closure");
    let (witness, result) = replay(&authored, &mutant, &counterexample, &relationship("kids"));
    // A path of two edges with no direct edge: some object reaches another through a third.
    let edge = |from: usize, to: usize| witness.edges[from].contains(&to);
    assert!(
        (0..witness.objects).any(|a| (0..witness.objects)
            .any(|b| (0..witness.objects).any(|c| edge(a, b) && edge(b, c) && !edge(a, c)))),
        "{witness:?}"
    );
    assert_clause_holds(&result);
}

/// Claim mutant: the conclusion is reversed, so the authored clause itself is false on a chain
/// graph. Kani falsifies the unmutated harness over the mutated claim, and QSL replays that
/// mutant's own authored clause on the decoded graph and reaches the same `Violation` verdict.
/// The clause is universally quantified, so QSL also derives a separating witness (the binder
/// objects that violate it) that the harness's playback does not carry: the settlement is
/// `Inconclusive` with a witness cause (`given` absent, `derived` present), not
/// `ReproducedWithEvaluatedWitness`. Verdict agreement is evidenced; the evaluated-witness
/// settlement for a quantified claim is not.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "real Kani qualification: run serially inside the flock and the 24G scope"]
fn real_kani_reversed_transitivity_claim_falsifies_and_qsl_replay_agrees_on_the_verdict() {
    let authored = authored(REVERSED, 3);
    let healthy = harness(&authored, &CONCLUSION);
    let counterexample = run_falsified(&healthy, "transitivity-reversed");
    let (_, result) = replay(&authored, &healthy, &counterexample, &relationship("kids"));
    let (settlement, category, cause) = settled(&result);
    assert_eq!(settlement, WitnessSettlement::Inconclusive);
    assert_eq!(category, Category::Violation);
    let Some(DisagreementCause::Witness {
        proved,
        replayed,
        given: None,
        derived: Some(_),
        ..
    }) = cause
    else {
        panic!("a witness-cause disagreement with only the derived witness: {cause:?}")
    };
    assert_eq!(
        (proved.category(), replayed.category()),
        (Category::Violation, Category::Violation)
    );
}
