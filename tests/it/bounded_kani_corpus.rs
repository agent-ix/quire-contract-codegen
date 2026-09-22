//! Public integration coverage for the bounded Kani corpus API.
//!
//! `--harness <name>` below is a substring filter over the fully qualified harness name, not an
//! exact match (`--exact` is not passed), so e.g. `--harness corpus_case_arithmetic` matches the
//! generated `corpus_case_arithmetic_<identity>` symbol. Each temporary crate here writes exactly
//! one harness, so the filter is effectively exact in this file today, but read literally it is a
//! prefix over the whole `arithmetic`/`graph`/`collection` family: a crate containing more than one
//! case of the same family would have every one of them selected by this same filter.

use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_bounded_kani_corpus_case, replay_codegen_counterexample, BoundedCorpusRequest,
};
use quire_contract_ir::{
    kani::{
        CapabilityDisposition, CapabilityEntry, CollectionQuery, DispatchIndex, FiniteInput,
        FiniteObject, FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile,
        ModuleDescriptor, PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds,
        SemanticFamily,
    },
    NumericOperator,
};

fn fixture() -> (
    KaniProfile,
    DispatchIndex,
    quire_contract_ir::kani::ValidatedFiniteInput,
) {
    let selection = ProfileSelection {
        profile: "kani-bounded/1".to_owned(),
        revision: "r1".to_owned(),
        executable_digest: "exe".to_owned(),
        options_digest: "opts".to_owned(),
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

/// Trace: FR-007-AC-1, FR-007-AC-2, FR-007-AC-3, FR-007-AC-5, TC-023.
#[test]
fn tc_023_public_corpus_uses_the_validated_profile_boundary() {
    let (profile, dispatch, input) = fixture();
    let arithmetic = generate_bounded_kani_corpus_case(
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
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![1, 2],
            max_items: 1,
            kind: QueryKind::ForAllNonNegative,
        }),
    )
    .unwrap_err();
    assert_eq!(exhausted.kind, KaniOutcomeKind::ResourceExhausted);
    assert_eq!(exhausted.boolean_claim(), None);

    let counterexample = generate_bounded_kani_corpus_case(
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2],
            max_items: 2,
            kind: QueryKind::ExistsEqual(7),
        }),
    )
    .unwrap();
    let packet = counterexample
        .counterexample
        .expect("a generated false collection case retains its packet");
    let agreement = replay_codegen_counterexample(packet, |native_input| {
        quire_contract_ir::kani::KaniOutcome::counterexample(
            native_input.source_id.clone(),
            native_input.profile.revision.clone(),
        )
    })
    .expect("the retained false classification replays through Contract IR");
    let quire_contract_ir::kani::ReplayAgreement::Input(agreement) = agreement else {
        panic!("a corpus-generated packet must settle as an Input replay agreement");
    };
    assert_eq!(agreement.native().boolean_claim(), Some(false));
    // The settled `Input`-arm agreement's own assignment content, not merely that it settled:
    // exactly the ordered population's two values, never `max_items` or the query's `expected`
    // oracle target (see the `arithmetic_assignments`/`collection_assignments` unit tests).
    assert_eq!(
        agreement.input(),
        &std::collections::BTreeMap::from([
            (
                "value_0".to_owned(),
                quire_contract_ir::kani::WitnessValue::Integer(2)
            ),
            (
                "value_1".to_owned(),
                quire_contract_ir::kani::WitnessValue::Integer(2)
            ),
        ])
    );
}

/// Trace: FR-007-AC-2, FR-007-AC-5, TC-023.
#[test]
fn tc_023_kani_executes_the_generated_arithmetic_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
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
    )
    .unwrap();
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
    let output = Command::new("cargo")
        .args(["kani", "--harness", "corpus_case_arithmetic"])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    assert!(
        output.status.success(),
        "generated arithmetic Kani proof failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: FR-007-AC-2, FR-007-AC-5, TC-023.
#[test]
fn tc_023_kani_executes_the_generated_graph_harness() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
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
    )
    .unwrap();
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
        "[package]\nname = \"bounded-kani-graph-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("generated corpus manifest should be writable");
    fs::write(
        directory.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("generated check-cfg declaration should be writable");
    let output = Command::new("cargo")
        .args(["kani", "--harness", "corpus_case_graph"])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    assert!(
        output.status.success(),
        "generated graph Kani proof failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: FR-007-AC-2, FR-007-AC-4, TC-023.
#[test]
fn tc_023_kani_counterexample_replays_through_contract_ir() {
    let (profile, dispatch, input) = fixture();
    let generated = generate_bounded_kani_corpus_case(
        &profile,
        &dispatch,
        &input,
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: "source".to_owned(),
            values: vec![2, 2],
            max_items: 2,
            kind: QueryKind::ExistsEqual(7),
        }),
    )
    .unwrap();
    let packet = generated
        .counterexample
        .clone()
        .expect("false corpus case must retain a replay packet");
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
        "[package]\nname = \"bounded-kani-counterexample-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("generated corpus manifest should be writable");
    fs::write(
        directory.0.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("generated check-cfg declaration should be writable");
    let output = Command::new("cargo")
        .args(["kani", "--harness", "corpus_case_collection"])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .current_dir(&directory.0)
        .output()
        .expect("cargo kani should launch");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.status.success(),
        "false generated corpus case must not be reported as a Kani proof"
    );
    // A nonzero exit alone does not prove the harness ran and was falsified: a harness-filter
    // mismatch ("error: no harnesses matched the harness filter") also exits nonzero under
    // Kani 0.67.0, and would pass the assertion above vacuously. Require the backend's own
    // verification-failed banner so this test cannot pass on a filter that matched nothing.
    assert!(
        text.contains("VERIFICATION:- FAILED"),
        "expected a genuine Kani verification failure (VERIFICATION:- FAILED), not merely a \
         nonzero exit, which a harness-filter mismatch also produces; got:\n{text}"
    );
    let agreement = replay_codegen_counterexample(packet, |native_input| {
        quire_contract_ir::kani::KaniOutcome::counterexample(
            native_input.source_id.clone(),
            native_input.profile.revision.clone(),
        )
    })
    .expect("the retained Kani counterexample must replay as native false");
    let quire_contract_ir::kani::ReplayAgreement::Input(agreement) = agreement else {
        panic!("a corpus-generated packet must settle as an Input replay agreement");
    };
    assert_eq!(agreement.native().boolean_claim(), Some(false));
    // Same assignment-content check as `tc_023_public_corpus_uses_the_validated_profile_boundary`,
    // against the Kani-executed harness's own retained packet rather than a freshly generated one.
    assert_eq!(
        agreement.input(),
        &std::collections::BTreeMap::from([
            (
                "value_0".to_owned(),
                quire_contract_ir::kani::WitnessValue::Integer(2)
            ),
            (
                "value_1".to_owned(),
                quire_contract_ir::kani::WitnessValue::Integer(2)
            ),
        ])
    );
}
