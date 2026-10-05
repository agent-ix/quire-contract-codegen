//! Public integration coverage for the bounded Kani corpus API.
//!
//! `--harness <name>` below is a substring filter over the fully qualified harness name, not an
//! exact match (`--exact` is not passed), so e.g. `--harness corpus_case_arithmetic` matches the
//! generated `corpus_case_arithmetic_<digest>` symbol. Each temporary crate here writes exactly
//! one harness, so the filter is effectively exact in this file today, but read literally it is a
//! prefix over the whole `arithmetic`/`graph`/`collection` family: a crate containing more than one
//! case of the same family would have every one of them selected by this same filter.

use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::{
    classify_kani_run, generate_bounded_kani_corpus_case, BoundedCorpusRequest,
    CorpusProofDependencyGraph, EmittedCorpusIdentities, KaniInconclusiveReason, KaniRunOutcome,
    ProofDependencyKind, ProofDependencyRequest, ProofDependencyState, ProofReadiness,
    CORPUS_PROOF_GRAPH_SCHEMA,
};
use quire_contract_ir::kani::{
    CapabilityDisposition, CapabilityEntry, CollectionQuery, DispatchIndex, FiniteInput,
    FiniteObject, FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
    PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, SemanticFamily,
};
use quire_contract_model::NumericOperator;

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
        vec![
            CapabilityEntry {
                construct: "checked-arithmetic".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "arithmetic".to_owned(),
                },
            },
            CapabilityEntry {
                construct: "bounded-collection-query".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "collections".to_owned(),
                },
            },
            CapabilityEntry {
                construct: "finite-reference-graph".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "graphs".to_owned(),
                },
            },
        ],
    )
    .unwrap();
    let dispatch = DispatchIndex::new(vec![
        ModuleDescriptor {
            module_id: "arithmetic".to_owned(),
            family: SemanticFamily::DefinednessArithmetic,
            abi_revision: "abi".to_owned(),
            constructs: vec!["checked-arithmetic".to_owned()],
        },
        ModuleDescriptor {
            module_id: "collections".to_owned(),
            family: SemanticFamily::CollectionsQueries,
            abi_revision: "abi".to_owned(),
            constructs: vec!["bounded-collection-query".to_owned()],
        },
        ModuleDescriptor {
            module_id: "graphs".to_owned(),
            family: SemanticFamily::ObjectsReferencesGraphs,
            abi_revision: "abi".to_owned(),
            constructs: vec!["finite-reference-graph".to_owned()],
        },
    ])
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

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must follow the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "quire-codegen-bounded-kani-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(path.join("src")).expect("temporary source directory should exist");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Trace: TC-023.
#[test]
fn tc_023_public_corpus_uses_the_validated_profile_boundary() {
    let (profile, dispatch, input) = fixture();
    let mut emitted = EmittedCorpusIdentities::new();
    let arithmetic = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 1,
            right: 1,
            minimum: 0,
            maximum: 2,
        }),
        &[],
        &mut emitted,
    )
    .unwrap();
    assert_eq!(arithmetic.outcome.boolean_claim(), Some(true));
    assert!(arithmetic
        .artifacts
        .oracle
        .contents
        .contains("1i128.checked_add(1i128)"));
    assert!(!arithmetic
        .artifacts
        .kani_harness
        .contents
        .contains("kani::assume"));

    let exhausted = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![1, 2],
            max_items: 1,
            kind: QueryKind::ForAllNonNegative,
        }),
        &[],
        &mut emitted,
    )
    .unwrap_err();
    let exhausted = exhausted
        .outcome()
        .expect("a bound refusal is a typed outcome");
    assert_eq!(exhausted.kind, KaniOutcomeKind::ResourceExhausted);
    assert_eq!(exhausted.boolean_claim(), None);

    let counterexample = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2],
            max_items: 2,
            kind: QueryKind::ExistsEqual(7),
        }),
        &[],
        &mut emitted,
    )
    .unwrap();
    assert_eq!(counterexample.outcome.boolean_claim(), Some(false));
}

/// Trace: FR-015-AC-57, TC-023.
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_executes_the_generated_arithmetic_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 1,
            right: 1,
            minimum: 0,
            maximum: 2,
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert_eq!(
        classify_corpus(&generated, "corpus_case_arithmetic"),
        KaniRunOutcome::Verified
    );
}

/// Trace: FR-015-AC-57, TC-023.
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_executes_the_generated_graph_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
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

/// A true collection case classifies `Verified`, and a false graph case is `Falsified` with the
/// assertion's playback, which draws no value, so it is empty-valued.
///
/// Trace: FR-015-AC-57, TC-023.
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_verifies_a_true_collection_and_falsifies_a_false_graph_harness() {
    let (profile, dispatch, input) = fixture();
    let true_collection = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2, 7],
            max_items: 3,
            kind: QueryKind::ExistsEqual(7),
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert_eq!(
        classify_corpus(&true_collection, "corpus_case_collection"),
        KaniRunOutcome::Verified
    );
    let false_graph = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
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
    assert_eq!(false_graph.outcome.boolean_claim(), Some(false));
    assert_empty_valued_falsification(classify_corpus(&false_graph, "corpus_case_graph"));
}

/// Without its cover a corpus harness carries no cover summary, so the cover is what lets a
/// true case classify `Verified`.
///
/// Trace: FR-015-AC-55, TC-023.
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_reads_a_corpus_harness_without_its_cover_as_missing_the_cover_summary() {
    let (profile, dispatch, input) = fixture();
    let mut generated = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 1,
            right: 1,
            minimum: 0,
            maximum: 2,
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
        classify_corpus(&generated, "corpus_case_arithmetic"),
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MissingCoverSummary
        }
    );
}

/// Trace: FR-015-AC-57, TC-023.
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_023_kani_falsifies_the_generated_false_collection_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2],
            max_items: 2,
            kind: QueryKind::ExistsEqual(7),
        }),
        &[],
        &mut EmittedCorpusIdentities::new(),
    )
    .unwrap();
    assert_empty_valued_falsification(classify_corpus(&generated, "corpus_case_collection"));
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
    let directory = TemporaryDirectory::new();
    fs::write(
        directory.0.join("src/lib.rs"),
        format!(
            "{}\n{}",
            generated.artifacts.oracle.contents, generated.artifacts.kani_harness.contents
        ),
    )
    .expect("generated corpus source should be writable");
    fs::write(
        directory.0.join("Cargo.toml"),
        "[package]\nname = \"bounded-kani-corpus-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("generated corpus manifest should be writable");
    fs::write(
        directory.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("generated check-cfg declaration should be writable");
    let report_path = directory.0.join("report.json");
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
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = fs::read(&report_path).ok();
    classify_kani_run(output.status.success(), report.as_deref(), &text, None)
        .unwrap_or_else(|refusal| panic!("the run was refused as {refusal:?}:\n{text}"))
        .outcome
}

/// A corpus case draws no input, so a falsification's playback carries no value.
fn assert_empty_valued_falsification(outcome: KaniRunOutcome) {
    let KaniRunOutcome::Falsified { counterexample } = outcome else {
        panic!("expected a falsified corpus case, got {outcome:?}");
    };
    assert!(
        counterexample.contains("let concrete_vals: Vec<Vec<u8>> = vec![];")
            || counterexample.contains("let concrete_vals: Vec<Vec<u8>> = vec![\n    ];"),
        "the playback of a case that draws no input must be empty-valued:\n{counterexample}"
    );
}

/// Every corpus family's harness ends with exactly one cover, after the oracle's assertion.
///
/// Trace: FR-015-AC-55, TC-023.
#[test]
fn tc_023_every_corpus_family_ends_its_harness_with_one_cover_after_its_assertion() {
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

/// The harness of one true case of each corpus family, for the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let (profile, dispatch, input) = fixture();
    let requests = [
        (
            "corpus arithmetic",
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
                source_id: "source",
                operator: NumericOperator::Add,
                left: 1,
                right: 1,
                minimum: 0,
                maximum: 2,
            }),
        ),
        (
            "corpus graph",
            BoundedCorpusRequest::Graph(GraphRequest {
                source_id: "source".to_owned(),
                start_id: "a".to_owned(),
                target_id: "b".to_owned(),
                field_id: "next".to_owned(),
                max_expansions: 2,
            }),
        ),
        (
            "corpus collection",
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
        ),
    ];
    requests
        .into_iter()
        .map(|(family, request)| {
            let generated = generate_bounded_kani_corpus_case(
                quire_contract_codegen::ProofCeilings {
                    memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
                    wall_clock: std::time::Duration::from_secs(600),
                },
                &profile,
                &dispatch,
                &input,
                request,
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            (family, generated.artifacts.kani_harness.contents)
        })
        .collect()
}

/// Every supported family's emitted `proof_graph` artifact is a real `CORPUS_PROOF_GRAPH_SCHEMA`
/// document -- not merely a JSON blob this crate's own `CorpusProofDependencyGraph` type happens
/// to deserialize -- validated against the published schema file the same way
/// `tests/it/kani_generation.rs` validates `quire.kani-proof-graph/v2` graphs against
/// `schemas/kani-proof-graph-v2.schema.json`.
///
/// Trace: TC-023.
#[test]
fn tc_023_proof_graph_artifact_validates_against_its_published_schema() {
    let (profile, dispatch, input) = fixture();
    let cases = [
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 1,
            right: 1,
            minimum: 0,
            maximum: 2,
        }),
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "b".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 2,
        }),
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2, 7],
            max_items: 3,
            kind: QueryKind::ExistsEqual(7),
        }),
    ];
    for request in cases {
        let generated = generate_bounded_kani_corpus_case(
            quire_contract_codegen::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
                wall_clock: std::time::Duration::from_secs(600),
            },
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
}

/// A declared `Required` dependency's edge must also validate against the published schema, not
/// only the empty-census shape the test above exercises.
///
/// Trace: TC-023.
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
        quire_contract_codegen::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(600),
        },
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
            source_id: "source",
            operator: NumericOperator::Add,
            left: 1,
            right: 1,
            minimum: 0,
            maximum: 2,
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
