//! Public integration coverage for the bounded Kani corpus API.
//!
//! `--harness <name>` below is a substring filter over the fully qualified harness name.
//! Each temporary crate writes exactly one graph harness.

use std::{fs, process::Command};

use crate::scratch_crate::TemporaryDirectory;
use ix_trace_rs::trace;
use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    classify_kani_run, generate_bounded_kani_corpus_case, parse_assertion_playback,
    prepare_finite_graph_reaches, select_counterexample, BoundedCorpusCase, BoundedCorpusRequest,
    CorpusProofDependencyGraph, EmittedCorpusIdentities, GraphLowering, GraphRequest,
    KaniInconclusiveReason, KaniRunOutcome, Playback, ProofDependencyKind, ProofDependencyRequest,
    ProofDependencyState, ProofReadiness, ReplayVerdict, WitnessDecodeLimits,
    CORPUS_PROOF_GRAPH_SCHEMA,
};
use quire_contract_ir::kani::{
    CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
    FiniteReference, KaniOutcome, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
    PopulationCompleteness, ProfileSelection, ResourceBounds, SemanticFamily,
};

use quire_contract_model::{std001_code, Std001Code};

fn typed_playback(transcript: &str, limits: WitnessDecodeLimits) -> Playback<'_> {
    parse_assertion_playback(
        transcript,
        "corpus-replay-test",
        "corpus",
        limits.bytes,
        limits.values,
    )
    .expect("the complete assertion playback parses")
}

/// A generated case can carry only an expectation; it has no run-authenticated
/// Kani outcome that the public terminal mapper could certify before Kani runs.
#[trace("FR-015-AC-91", "TC-023")]
#[test]
fn tc_023_pre_run_case_cannot_expose_a_certifiable_kani_outcome() {
    let source = include_str!("../../src/kani/generate/corpus/bounded_kani_corpus.rs");
    assert!(source.contains("pub expectation: BoundedCorpusExpectation"));
    assert!(!source.contains("pub outcome: KaniOutcome"));
    assert!(!source.contains("KaniOutcome::proved("));
    assert!(!source.contains("KaniOutcome::counterexample("));
}

fn fixture() -> (
    KaniProfile,
    DispatchIndex,
    quire_contract_ir::kani::ValidatedFiniteInput,
) {
    let selection = ProfileSelection {
        profile: "kani-bounded/1".to_owned(),
        revision: "r1".to_owned(),
        abi_revision: "abi".to_owned(),
    };
    let profile = KaniProfile::new(
        selection.clone(),
        vec![CapabilityEntry {
            construct: "finite-reference-graph".to_owned(),
            disposition: CapabilityDisposition::Supported {
                module: "graphs".to_owned(),
            },
        }],
    )
    .unwrap();
    let dispatch = DispatchIndex::new(vec![ModuleDescriptor {
        module_id: "graphs".to_owned(),
        family: SemanticFamily::ObjectsReferencesGraphs,
        abi_revision: "abi".to_owned(),
        constructs: vec!["finite-reference-graph".to_owned()],
    }])
    .unwrap();
    let input = FiniteInput {
        model_id: "model".to_owned(),
        source_id: "source".to_owned(),
        profile: selection,
        completeness: PopulationCompleteness::Complete,
        bounds: ResourceBounds {
            max_objects: 2,
            max_references: 1,
            max_input_bytes: 2,
        },
        input_bytes: 1,
        objects: vec![
            FiniteObject {
                identity: "a".to_owned(),
                type_id: "node".to_owned(),
                snapshot_id: "s".to_owned(),
            },
            FiniteObject {
                identity: "b".to_owned(),
                type_id: "node".to_owned(),
                snapshot_id: "s".to_owned(),
            },
        ],
        references: vec![FiniteReference {
            source_id: "a".to_owned(),
            field_id: "next".to_owned(),
            target_id: "b".to_owned(),
        }],
    }
    .validate()
    .unwrap();
    (profile, dispatch, input)
}

#[trace("FR-015-AC-57", "TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_executes_the_generated_graph_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert_eq!(
        classify_corpus(&generated, "corpus_case_graph"),
        KaniRunOutcome::Verified
    );
}

/// A branch outside the shortest route consumes the depth-first expansion budget before the
/// target branch; the generated oracle and native admission must agree at the boundary.
#[trace("TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_branching_graph_oracle_preserves_native_expansion_budget() {
    let (profile, dispatch, input) = fixture();
    let mut offered = input.input().clone();
    offered.bounds.max_objects = 5;
    offered.bounds.max_references = 4;
    offered
        .objects
        .extend(["c", "d", "target"].map(|identity| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        }));
    offered.references = [("a", "c"), ("b", "d"), ("c", "target"), ("a", "b")]
        .into_iter()
        .map(|(source_id, target_id)| FiniteReference {
            source_id: source_id.to_owned(),
            field_id: "next".to_owned(),
            target_id: target_id.to_owned(),
        })
        .collect();
    let input = offered.validate().expect("branching finite input");
    let request = GraphRequest {
        source_id: "source".to_owned(),
        start_id: "a".to_owned(),
        target_id: "target".to_owned(),
        field_id: "next".to_owned(),
        max_expansions: 4,
    };
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(request.clone()),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .expect("four expansions admit the target branch");
    assert_eq!(
        classify_corpus(&generated, "corpus_case_graph"),
        KaniRunOutcome::Verified
    );

    let exhausted = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            max_expansions: 3,
            ..request
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .expect_err("the third expansion enters the other branch");
    assert_eq!(exhausted.code().as_str(), "kani_graph_expansion_exhausted");
}

/// Raw identity ordering must survive Rust literal escaping in the generated graph oracle.
/// At bound two, visiting `a0` before `a\n` would miss the target.
#[trace("TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_escaped_graph_identity_keeps_native_and_oracle_order() {
    let (profile, dispatch, input) = fixture();
    let mut offered = input.input().clone();
    offered.bounds.max_objects = 4;
    offered.bounds.max_references = 3;
    offered
        .objects
        .extend(["a\n", "a0", "target"].map(|identity| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        }));
    offered.objects.retain(|object| object.identity != "b");
    offered.references = [("a", "a\n"), ("a", "a0"), ("a\n", "target")]
        .into_iter()
        .map(|(source_id, target_id)| FiniteReference {
            source_id: source_id.to_owned(),
            field_id: "next".to_owned(),
            target_id: target_id.to_owned(),
        })
        .collect();
    let input = offered
        .validate()
        .expect("escaped finite identity is valid");
    let request = GraphRequest {
        source_id: "source".to_owned(),
        start_id: "a".to_owned(),
        target_id: "target".to_owned(),
        field_id: "next".to_owned(),
        max_expansions: 2,
    };
    let native = prepare_finite_graph_reaches(&profile, &dispatch, &input, request.clone())
        .expect("the newline branch reaches the target on its second expansion");
    assert!(native.reachable);
    assert_eq!(native.expanded, ["a", "a\n"]);
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(request),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .expect("native reachability admits the bounded case");
    assert!(generated.expectation.expected_assertion_holds());
    assert_eq!(
        classify_corpus(&generated, "corpus_case_graph"),
        KaniRunOutcome::Verified
    );
}

/// A false graph case is `Falsified` with empty-valued assertion playback.
#[trace("FR-015-AC-57", "TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_falsifies_a_false_graph_harness() {
    let (profile, dispatch, input) = fixture();
    let false_graph = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "b".to_owned(),
            target_id: "a".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert!(!false_graph.expectation.expected_assertion_holds());
    assert_empty_valued_falsification(classify_corpus(&false_graph, "corpus_case_graph"));
}

/// Without its cover a corpus harness carries no cover summary, so the cover is what lets a
/// true case classify `Verified`.
#[trace("FR-015-AC-55", "TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_reads_a_corpus_harness_without_its_cover_as_missing_the_cover_summary() {
    let (profile, dispatch, input) = fixture();
    let mut generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    generated.artifacts.kani_harness.contents = generated
        .artifacts
        .kani_harness
        .contents
        .lines()
        .filter(|line| !line.contains("kani::cover!"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        classify_corpus(&generated, "corpus_case_graph"),
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MissingCoverSummary
        }
    );
}

/// Captured Kani counterexamples for false corpus cases reproduce through each case's QSL source.
#[trace("FR-015-AC-57", "TC-023")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_real_corpus_playback_replays_through_its_qsl_source() {
    let (profile, dispatch, input) = fixture();
    let graph = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "b".to_owned(),
            target_id: "a".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    let playback = falsification_playback(classify_corpus(&graph, "corpus_case_graph"));
    let limits = WitnessDecodeLimits {
        bytes: playback.len(),
        values: 0,
    };
    let parsed = typed_playback(&playback, limits);
    assert_eq!(
        graph.replay_counterexample(&parsed, limits).unwrap(),
        ReplayVerdict::Reproduced,
    );
}

/// The caller parses the complete captured text; the public corpus replay entry rechecks its
/// source length and value count before native execution, even if parsing used wider bounds.
#[trace("TC-026", "FR-016-AC-6")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_026_corpus_replay_refuses_poisoned_suffixes_at_both_decode_bounds() {
    let (profile, dispatch, input) = fixture();
    let case = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "b".to_owned(),
            target_id: "a".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    let playback = falsification_playback(classify_corpus(&case, "corpus_case_graph"));
    let limits = WitnessDecodeLimits {
        bytes: playback.len(),
        values: 0,
    };
    let parsed = typed_playback(&playback, limits);
    assert_eq!(
        case.replay_counterexample(&parsed, limits).unwrap(),
        ReplayVerdict::Reproduced,
    );
    let poisoned = format!("{playback}\ntrailing captured output");
    let wider_bytes = WitnessDecodeLimits {
        bytes: poisoned.len(),
        values: 0,
    };
    let parsed = typed_playback(&poisoned, wider_bytes);
    assert_eq!(
        parse_assertion_playback(
            &poisoned,
            "corpus-replay-test",
            "corpus",
            limits.bytes,
            limits.values
        )
        .expect_err("complete retained text exceeds the caller byte ceiling")
        .code
        .as_str(),
        "cg_witness_decode_bytes_limit"
    );
    let verdict = case
        .replay_counterexample(&parsed, limits)
        .expect("over-bound evidence yields a verdict");
    assert!(matches!(
        verdict,
        ReplayVerdict::EvidenceFailure(quire_contract_codegen::EvidenceFailureCause::Decode(ref cause))
        if cause.code == "cg_witness_decode_bytes_limit"
    ));
    let control = case
        .replay_counterexample(&parsed, wider_bytes)
        .expect("the complete source fits the byte bound");
    assert_eq!(control, ReplayVerdict::Reproduced);
    let poisoned_value = playback.replace(
        "let concrete_vals: Vec<Vec<u8>> = vec![\n    ];",
        "let concrete_vals: Vec<Vec<u8>> = vec![\n        // 0\n        vec![0, 0, 0, 0, 0, 0, 0, 0],\n    ];",
    );
    assert_ne!(poisoned_value, playback);
    let zero_values = WitnessDecodeLimits {
        bytes: poisoned_value.len(),
        values: 0,
    };
    let one_value = WitnessDecodeLimits {
        bytes: poisoned_value.len(),
        values: 1,
    };
    let parsed = typed_playback(&poisoned_value, one_value);
    assert_eq!(
        parse_assertion_playback(
            &poisoned_value,
            "corpus-replay-test",
            "corpus",
            zero_values.bytes,
            zero_values.values,
        )
        .expect_err("complete retained text exceeds the caller value ceiling")
        .code
        .as_str(),
        "cg_witness_decode_values_limit"
    );
    let verdict = case
        .replay_counterexample(&parsed, zero_values)
        .expect("over-bound value yields a verdict");
    assert!(matches!(
        verdict,
        ReplayVerdict::EvidenceFailure(quire_contract_codegen::EvidenceFailureCause::Decode(ref cause))
        if cause.code == "cg_witness_decode_values_limit"
    ));
    let control = case
        .replay_counterexample(&parsed, one_value)
        .expect("extra value yields a decode verdict");
    assert!(matches!(
        control,
        ReplayVerdict::EvidenceFailure(quire_contract_codegen::EvidenceFailureCause::Decode(ref cause))
        if cause.code == "kani_witness_arity_mismatch"
    ));
}

/// Runs the corpus case's oracle and harness as one crate under the installed backend and
/// classifies the run as production does.
///
/// `-Z concrete-playback` and `--concrete-playback print` are required for the classifier to see
/// a playback block at all: without them Kani never prints one, even for a genuine
/// falsification, and every run classifies Inconclusive rather than Falsified. `adapter_options`
/// (src/kani/abi.rs) always includes these two for every production harness, plus `--exact`,
/// `--unwind` and `--solver`, which this invocation does not replicate: `--harness` is an
/// effective exact match here (the crate holds exactly one harness), so the omission is inert.
/// `--export-json` is what the verdict is read from.
///
/// A nonzero exit alone does not prove a falsification: a harness-filter mismatch and CBMC's
/// out-of-memory abort also exit nonzero with no property decided (IR-220), so the outcome is
/// read through the classifier and the callers assert it.
fn classify_corpus(
    generated: &quire_contract_codegen::BoundedCorpusCase,
    harness_filter: &str,
) -> KaniRunOutcome {
    let directory = TemporaryDirectory::new("quire-codegen-bounded-kani");
    fs::write(
        directory.path.join("src/lib.rs"),
        format!(
            "{}\n{}",
            generated.artifacts.oracle.contents, generated.artifacts.kani_harness.contents
        ),
    )
    .expect("generated corpus source should be writable");
    fs::write(
        directory.path.join("Cargo.toml"),
        "[package]\nname = \"bounded-kani-corpus-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("generated corpus manifest should be writable");
    fs::write(
        directory.path.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("generated check-cfg declaration should be writable");
    let report_path = directory.path.join("report.json");
    let output = Command::new("cargo")
        .args([
            "kani",
            "-Z",
            "concrete-playback",
            "-Z",
            "unstable-options",
            "--export-json",
        ])
        .arg(&report_path)
        .args(["--harness", harness_filter, "--concrete-playback", "print"])
        .env("CARGO_TARGET_DIR", directory.path.join("target"))
        .current_dir(&directory.path)
        .output()
        .expect("cargo kani should launch");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = fs::read(&report_path).ok();
    classify_kani_run(
        output.status.success(),
        report.as_deref(),
        select_counterexample(&text, None),
        None,
    )
    .unwrap_or_else(|refusal| panic!("the run was refused as {refusal:?}:\n{text}"))
    .outcome
}

/// A corpus case draws no input, so a falsification's playback carries no value.
fn assert_empty_valued_falsification(outcome: KaniRunOutcome) {
    let _ = falsification_playback(outcome);
}

fn falsification_playback(outcome: KaniRunOutcome) -> String {
    let KaniRunOutcome::Falsified { counterexample } = outcome else {
        panic!("expected a falsified corpus case, got {outcome:?}");
    };
    assert!(
        counterexample.contains("let concrete_vals: Vec<Vec<u8>> = vec![];")
            || counterexample.contains("let concrete_vals: Vec<Vec<u8>> = vec![\n    ];"),
        "the playback of a case that draws no input must be empty-valued:\n{counterexample}"
    );
    counterexample
}

/// The graph corpus harness ends with exactly one cover, after the oracle's assertion.
#[trace("FR-015-AC-55", "TC-023")]
#[test]
fn tc_023_graph_corpus_ends_its_harness_with_one_cover_after_its_assertion() {
    for (family, harness) in guard_sources() {
        let assertion = harness.find("assert!(corpus_oracle());");
        let cover = harness.find("kani::cover!(");
        assert!(
            assertion
                .zip(cover)
                .is_some_and(|(assert, cover)| assert < cover),
            "{family}: the cover must follow the oracle assertion:\n{harness}"
        );
        assert_eq!(harness.matches("kani::cover!(").count(), 1, "{family}");
    }
}

/// The harness of one true graph case, for the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    vec![("corpus graph", generated.artifacts.kani_harness.contents)]
}

/// The graph family's emitted `proof_graph` artifact is a real `CORPUS_PROOF_GRAPH_SCHEMA`
/// document -- not merely a JSON blob this crate's own `CorpusProofDependencyGraph` type happens
/// to deserialize -- validated against the published corpus schema file.
#[trace("TC-023", "FR-028-AC-1")]
#[test]
fn tc_023_proof_graph_artifact_validates_against_its_published_schema() {
    let (profile, dispatch, input) = fixture();
    let request = BoundedCorpusRequest::Graph(GraphRequest {
        source_id: "source".to_owned(),
        start_id: "a".to_owned(),
        target_id: "b".to_owned(),
        field_id: "next".to_owned(),
        max_expansions: 2,
    });
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        request,
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    let instance: serde_json::Value =
        serde_json::from_str(&generated.artifacts.proof_graph.contents)
            .expect("the corpus proof-dependency graph must parse as JSON");
    // The old struct-order encoder wrote `schemaVersion`
    // second; canonical bytes sort `dependencies` before it and end with one newline.
    let graph_text = &generated.artifacts.proof_graph.contents;
    assert!(
        graph_text.find("\"dependencies\"").unwrap()
            < graph_text.find("\"schemaVersion\"").unwrap()
    );
    assert!(graph_text.ends_with("}\n"));
    assert!(!graph_text.ends_with("}\n\n"));
    validate(
        include_str!("../../schemas/kani-corpus-proof-graph-v1.schema.json"),
        &instance,
    );
    let graph: CorpusProofDependencyGraph = serde_json::from_str(
        &generated.artifacts.proof_graph.contents,
    )
    .expect("the corpus proof-dependency graph must deserialize as CorpusProofDependencyGraph");
    assert_eq!(graph.schema_version, CORPUS_PROOF_GRAPH_SCHEMA);
    assert!(graph.dependencies.is_empty());
    assert_eq!(graph.readiness, ProofReadiness::Ready);
}

/// The published finite corpus schema accepts graph cases and rejects retired families.
#[trace("FR-015-AC-110", "FR-028-AC-1", "TC-023")]
#[test]
fn tc_023_proof_graph_schema_rejects_retired_corpus_families() {
    let schema: serde_json::Value = serde_json::from_str(include_str!(
        "../../schemas/kani-corpus-proof-graph-v1.schema.json"
    ))
    .unwrap();
    let validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .unwrap();
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    let graph: serde_json::Value =
        serde_json::from_str(&generated.artifacts.proof_graph.contents).unwrap();
    assert!(validator.is_valid(&graph));
    for (field, retired) in [
        ("family", "collection"),
        ("family", "arithmetic"),
        ("construct", "bounded-collection-query"),
        ("construct", "checked-arithmetic"),
        ("identity", "collection_0000000000000000000000000000000000000000000000000000000000000000"),
        ("proofId", "corpus_case_arithmetic_0000000000000000000000000000000000000000000000000000000000000000"),
    ] {
        let mut retired_graph = graph.clone();
        retired_graph[field] = serde_json::json!(retired);
        assert!(!validator.is_valid(&retired_graph), "{field}={retired} remained admitted");
    }
}

/// A declared `Required` dependency's edge must also validate against the published schema, not
/// only the empty-census shape the test above exercises.
#[trace("TC-023", "FR-028-AC-1")]
#[test]
fn tc_023_proof_graph_with_a_declared_dependency_validates_against_its_published_schema() {
    let (profile, dispatch, input) = fixture();
    let dependency = ProofDependencyRequest {
        proof_id: "upstream-lemma",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Passed,
        original_path: None,
        replacement_path: None,
    };
    let generated = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[dependency],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    let instance: serde_json::Value =
        serde_json::from_str(&generated.artifacts.proof_graph.contents)
            .expect("the corpus proof-dependency graph must parse as JSON");
    validate(
        include_str!("../../schemas/kani-corpus-proof-graph-v1.schema.json"),
        &instance,
    );
}

fn validate(schema: &str, instance: &serde_json::Value) {
    let schema: serde_json::Value =
        serde_json::from_str(schema).expect("repository schema should parse");
    let validator = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("repository schema should compile");
    let errors = validator
        .validate(instance)
        .err()
        .map(|values| values.map(|error| error.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();
    assert!(errors.is_empty(), "schema errors: {errors:?}");
}

fn direct_graph(max_expansions: usize) -> GraphRequest {
    GraphRequest {
        source_id: "source:direct".into(),
        start_id: "a".into(),
        target_id: "b".into(),
        field_id: "next".into(),
        max_expansions,
    }
}

fn assert_direct_refusal(
    outcome: &KaniOutcome,
    kind: KaniOutcomeKind,
    code: Std001Code,
    source: &str,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.code(), code);
    assert_eq!(outcome.source_id(), source);
    assert_eq!(outcome.context(), "r1");
    assert_eq!(outcome.boolean_claim(), None);
}

#[trace(
    "FR-015-AC-123",
    "FR-015-AC-124",
    "TC-001",
    "NFR-002-AC-1",
    "NFR-002-AC-2"
)]
#[test]
fn public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes() {
    let (profile, dispatch, input) = fixture();
    let graph: Result<GraphLowering, KaniOutcome> =
        prepare_finite_graph_reaches(&profile, &dispatch, &input, direct_graph(1));
    let graph = graph.expect("the target edge needs only its source expansion");
    assert!(graph.reachable);
    assert_eq!(graph.expanded, ["a"]);
    let mut emitted = EmittedCorpusIdentities::new();
    let corpus: Result<BoundedCorpusCase, KaniOutcome> = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(2)),
        &[],
        &mut emitted,
    );
    let corpus = corpus.expect("all corpus artifacts are retained");
    for artifact in [
        &corpus.artifacts.oracle,
        &corpus.artifacts.strategy,
        &corpus.artifacts.kani_harness,
    ] {
        let case = artifact
            .path
            .strip_prefix("corpus/")
            .unwrap()
            .split('.')
            .next()
            .unwrap();
        crate::generated_headers::assert_header(&artifact.path, &artifact.contents, &[case]);
    }
    assert!(corpus.expectation.expected_assertion_holds());
    assert_eq!(corpus.expectation.expected_success_checks(), 1);
    assert_eq!(corpus.expectation.source_id(), "source:direct");
    assert_eq!(corpus.expectation.context(), "r1");
    assert!(corpus
        .artifacts
        .strategy
        .contents
        .contains("CORPUS_CASE: bool = true"));
    assert!(corpus.artifacts.kani_harness.contents.contains(
        "kani::cover!(true, \"the harness runs to its end past the oracle assertion\");"
    ));
    let duplicate: Result<BoundedCorpusCase, KaniOutcome> = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(2)),
        &[],
        &mut emitted,
    );
    assert_direct_refusal(
        &duplicate.unwrap_err(),
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_corpus_identity_collision"),
        "source:direct",
    );
}

#[trace("FR-015-AC-124")]
#[test]
fn direct_graph_errors_preserve_profile_dispatch_and_bound_precedence() {
    let (profile, dispatch, input) = fixture();
    let refused = KaniProfile::new(
        profile.selection().clone(),
        vec![CapabilityEntry {
            construct: "finite-reference-graph".into(),
            disposition: CapabilityDisposition::Refused {
                code: std001_code!("graph_not_qualified"),
            },
        }],
    )
    .unwrap();
    let empty = DispatchIndex::new(vec![]).unwrap();
    assert_direct_refusal(
        &prepare_finite_graph_reaches(&refused, &empty, &input, direct_graph(0)).unwrap_err(),
        KaniOutcomeKind::Refused,
        std001_code!("graph_not_qualified"),
        "source:direct",
    );
    assert_direct_refusal(
        &prepare_finite_graph_reaches(&profile, &empty, &input, direct_graph(0)).unwrap_err(),
        KaniOutcomeKind::Refused,
        std001_code!("kani_dispatch_unowned"),
        "source:direct",
    );
    let mismatched = KaniProfile::new(
        profile.selection().clone(),
        vec![CapabilityEntry {
            construct: "finite-reference-graph".into(),
            disposition: CapabilityDisposition::Supported {
                module: "foreign".into(),
            },
        }],
    )
    .unwrap();
    assert_direct_refusal(
        &prepare_finite_graph_reaches(&mismatched, &dispatch, &input, direct_graph(0)).unwrap_err(),
        KaniOutcomeKind::Refused,
        std001_code!("kani_dispatch_profile_mismatch"),
        "source:direct",
    );
    assert_direct_refusal(
        &prepare_finite_graph_reaches(&profile, &dispatch, &input, direct_graph(0)).unwrap_err(),
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_graph_bound_invalid"),
        "source:direct",
    );
    let unknown = GraphRequest {
        start_id: "foreign".into(),
        ..direct_graph(0)
    };
    assert_direct_refusal(
        &prepare_finite_graph_reaches(&profile, &dispatch, &input, unknown.clone()).unwrap_err(),
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_graph_bound_invalid"),
        "source:direct",
    );
    assert_direct_refusal(
        &prepare_finite_graph_reaches(
            &profile,
            &dispatch,
            &input,
            GraphRequest {
                max_expansions: 1,
                ..unknown
            },
        )
        .unwrap_err(),
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_graph_identity_invalid"),
        "source:direct",
    );
    assert_direct_refusal(
        &prepare_finite_graph_reaches(
            &profile,
            &dispatch,
            &input,
            GraphRequest {
                target_id: "a".into(),
                ..direct_graph(1)
            },
        )
        .unwrap_err(),
        KaniOutcomeKind::ResourceExhausted,
        std001_code!("kani_graph_expansion_exhausted"),
        "source:direct",
    );
}

#[trace("FR-015-AC-124")]
#[test]
fn direct_corpus_refusals_preserve_earliest_failure_and_identity_transactions() {
    let (profile, dispatch, input) = fixture();
    let dependency = ProofDependencyRequest {
        proof_id: "duplicate",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Passed,
        original_path: None,
        replacement_path: None,
    };
    let empty = DispatchIndex::new(vec![]).unwrap();
    let mut emitted = EmittedCorpusIdentities::new();
    let bad: Result<BoundedCorpusCase, KaniOutcome> = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &empty,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(0)),
        &[dependency, dependency],
        &mut emitted,
    );
    assert_direct_refusal(
        &bad.unwrap_err(),
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_corpus_dependency_invalid"),
        "source:direct",
    );
    let mismatch = KaniProfile::new(
        ProfileSelection {
            revision: "r2".into(),
            ..profile.selection().clone()
        },
        vec![],
    )
    .unwrap();
    let mismatch_error = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &mismatch,
        &empty,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(0)),
        &[dependency, dependency],
        &mut emitted,
    )
    .unwrap_err();
    assert_eq!(mismatch_error.kind(), KaniOutcomeKind::InvalidInput);
    assert_eq!(
        mismatch_error.code(),
        std001_code!("kani_profile_input_mismatch")
    );
    assert_eq!(mismatch_error.source_id(), "source:direct");
    assert_eq!(mismatch_error.context(), "r2");
    assert_eq!(mismatch_error.boolean_claim(), None);
    let oversized_source = "x".repeat(1 << 20);
    let oversized = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &empty,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: oversized_source.clone(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        &[],
        &mut emitted,
    )
    .unwrap_err();
    assert_direct_refusal(
        &oversized,
        KaniOutcomeKind::InvalidInput,
        std001_code!("kani_corpus_identity_unencodable"),
        &oversized_source,
    );
    let bound = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(GraphRequest {
            target_id: "a".into(),
            max_expansions: 1,
            ..direct_graph(2)
        }),
        &[],
        &mut emitted,
    )
    .unwrap_err();
    assert_direct_refusal(
        &bound,
        KaniOutcomeKind::ResourceExhausted,
        std001_code!("kani_graph_expansion_exhausted"),
        "source:direct",
    );
    // Dispatch is not part of the case identity: retrying the exact request with
    // its admitted dispatch proves that a failure did not reserve its identity.
    let unowned = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &empty,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(2)),
        &[],
        &mut emitted,
    )
    .unwrap_err();
    assert_direct_refusal(
        &unowned,
        KaniOutcomeKind::Refused,
        std001_code!("kani_dispatch_unowned"),
        "source:direct",
    );
    let accepted = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(2)),
        &[],
        &mut emitted,
    )
    .expect("the repaired case still admits in the same registry");
    let independent = generate_bounded_kani_corpus_case(
        crate::common::proof_ceilings::proof_ceilings(),
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Graph(direct_graph(2)),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert_eq!(
        accepted, independent,
        "earlier failures do not change successful bytes or identity"
    );
}
