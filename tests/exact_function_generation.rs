//! FR-021: function-application oracle generation over admitted CheckedPackage
//! V2 input.
//!
//! Generation-time coverage only (no execution of generated code):
//! dispositions, refusal reasons, ordering/determinism, the manifest, and the
//! static location map. Execution-level criteria (AC-2, AC-4, AC-5, AC-7,
//! AC-9, AC-17) are covered by `tests/exact_function_agreement.rs`.

use std::{fs, path::PathBuf};

use quire_contract_codegen::{
    generate_exact_function_oracles, CallPointKind, ExactFunctionDisposition, ExactFunctionItem,
    ExactFunctionRefusal, ExactFunctionUpstreamBlocker,
};
use quire_contract_ir::CheckedPackageV2;

#[path = "exact_function_support/package.rs"]
mod package;
use package::*;

const BLESS: &str = "QUIRE_CODEGEN_BLESS";

/// The main FR-021 corpus: one Scalar, one CompositeEquality and one Call
/// function (TC-031 step 1(a)); one function each refused for a `reference`
/// operand (AC-10), a model node (AC-11), a state node (AC-11), an
/// undischargeable capability (AC-6), and a dangling nested call (AC-12);
/// and one function wholly unrelated to any of the refused ones, to prove
/// isolation (AC-12).
fn main_functions() -> Vec<quire_contract_codegen::ExactFunctionDeclaration> {
    vec![
        function_add("add_fn"),
        function_eq("eq_fn"),
        function_call_nested("call_fn", "add_fn"),
        function_ref_param("ref_fn"),
        function_model_param("model_fn"),
        function_state_param("state_fn"),
        function_capability("capability_fn"),
        function_dangling_call("dangling_fn"),
        function_unrelated("unrelated_fn"),
    ]
}

fn main_items() -> Vec<ExactFunctionItem> {
    vec![
        item(ITEM_CALL_ADD, "add_fn"),
        item(ITEM_CALL_ADD, "eq_fn"), // same call node, different function (AC-13 key)
        item(ITEM_CALL_EQ, "eq_fn"),
        item(ITEM_CALL_NESTED, "call_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
        item(ITEM_CALL_UNKNOWN_FUNCTION, "does_not_exist_anywhere"),
    ]
}

fn generate(
    package: &CheckedPackageV2,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    items: &[ExactFunctionItem],
) -> quire_contract_codegen::ExactFunctionOracles {
    generate_exact_function_oracles(package, functions, items).expect("generation succeeds")
}

fn disposition_for(
    oracles: &quire_contract_codegen::ExactFunctionOracles,
    call_code: u32,
) -> &ExactFunctionDisposition {
    let node_id = code_id(call_code);
    &oracles
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == node_id)
        .unwrap_or_else(|| panic!("no claim for call node {call_code}"))
        .result
}

/// Trace: FR-021-AC-1, TC-031. Every requested item receives exactly one
/// generated or typed-refused disposition; a refused item (naming an
/// unknown function) contributes nothing, while its siblings generate
/// unchanged.
#[test]
fn tc_031_ac1_every_item_gets_one_disposition_and_refusal_does_not_cascade() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    assert_eq!(oracles.claim_map.items.len(), main_items().len());

    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ExactFunctionDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ExactFunctionDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ExactFunctionDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ExactFunctionDisposition::Generated(_)
    ));

    let unknown = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == code_id(ITEM_CALL_UNKNOWN_FUNCTION))
        .collect::<Vec<_>>();
    assert_eq!(unknown.len(), 1);
    assert!(matches!(
        &unknown[0].result,
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownFunction { name }
        } if name == "does_not_exist_anywhere"
    ));
}

/// Trace: FR-021-AC-3, TC-031. The claim-map entry for a generated item
/// records the applied function's name and its `Origin::Body{function,
/// index}` equal to the request's own declared-function ordering (functions
/// sorted by declaring node id, digest domain then digest) -- read from the
/// generator's own bookkeeping, not from the generated source text.
#[test]
fn tc_031_ac3_claim_records_function_and_origin_from_the_request() {
    let package = ext_corpus_package().admit();
    let functions = main_functions();
    let oracles = generate(&package, &functions, &main_items());

    let mut ordered: Vec<_> = functions.iter().collect();
    ordered.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    let survivor_names: Vec<&str> = ordered
        .iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "add_fn" | "eq_fn" | "call_fn" | "unrelated_fn"
            )
        })
        .map(|f| f.name.as_str())
        .collect();

    let ExactFunctionDisposition::Generated(claim) = disposition_for(&oracles, ITEM_CALL_ADD)
    else {
        panic!("expected generated");
    };
    assert_eq!(claim.function, "add_fn");
    let expected_index = survivor_names
        .iter()
        .position(|name| *name == "add_fn")
        .unwrap();
    match &claim.function_origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "add_fn");
            assert_eq!(*index, expected_index);
        }
    }
}

/// Trace: FR-021-AC-6, TC-031. A function whose declared operator
/// requirements name a capability no registered backend can discharge is
/// marked unsupported at generation time, naming the capability; no oracle
/// is emitted for any item naming it, and no `capability_fn` item was even
/// requested here, so this test drives it as a direct second request.
#[test]
fn tc_031_ac6_undischargeable_capability_refuses_before_any_item() {
    let package = ext_corpus_package().admit();
    let functions = vec![function_capability("capability_fn")];
    let items = vec![item(ITEM_CALL_ADD, "capability_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::UnsupportedCapability { capability }
        } if capability == "quire.capability.undischargeable-in-v1"
    ));
}

/// Trace: FR-021-AC-8, TC-031 (Inspection). Only `CheckMode::Linked` is ever
/// admitted: the generated source's `checked_package()` names
/// `rt::CheckMode::Linked` and never `rt::CheckMode::Kernel`.
#[test]
fn tc_031_ac8_generated_source_never_names_check_mode_kernel() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    let lib = contents(&oracles, "src/lib.rs");
    assert!(lib.contains("rt::CheckMode::Linked"));
    assert!(!lib.contains("CheckMode::Kernel"));
}

/// Trace: FR-021-AC-10, TC-031. A function whose declared parameter type
/// reaches a `reference` composite form at any depth is refused as blocked
/// on quire-spec-language#120 for every item naming it.
#[test]
fn tc_031_ac10_reference_parameter_blocked_on_qsl_120() {
    let package = ext_corpus_package().admit();
    let functions = vec![function_ref_param("ref_fn")];
    let items = vec![item(ITEM_CALL_ADD, "ref_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: ExactFunctionUpstreamBlocker::QuireSpecLanguage120,
                ..
            }
        }
    ));
}

/// Trace: FR-021-AC-11, TC-031. Model nodes are blocked on
/// quire-spec-language#120 and state nodes on quire-spec-language#121 --
/// two distinct blockers. (The corpus carries no `relation`/`protocol`
/// node, matching FR-018's own TC-029 precedent for this same family
/// split: `spec/test-matrix.md`'s FR-018 row records the identical gap.)
#[test]
fn tc_031_ac11_model_and_state_are_distinct_blockers() {
    let package = ext_corpus_package().admit();
    let functions = vec![
        function_model_param("model_fn"),
        function_state_param("state_fn"),
    ];
    let items = vec![
        item(ITEM_CALL_ADD, "model_fn"),
        item(ITEM_CALL_EQ, "state_fn"),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: ExactFunctionUpstreamBlocker::QuireSpecLanguage120,
                ..
            }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: ExactFunctionUpstreamBlocker::QuireSpecLanguage121,
                ..
            }
        }
    ));
}

/// Trace: FR-021-AC-12, TC-031. A declared function whose own body fails to
/// lower -- here, a nested `call` naming an absent callee -- refuses every
/// item bound to that function, without changing `unrelated_fn`'s item.
#[test]
fn tc_031_ac12_dangling_callee_refuses_only_its_own_items() {
    let package = ext_corpus_package().admit();
    let functions = vec![
        function_dangling_call("dangling_fn"),
        function_unrelated("unrelated_fn"),
    ];
    let items = vec![
        item(ITEM_CALL_ADD, "dangling_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownCallee { callee }
        } if callee == "not_declared_anywhere"
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ExactFunctionDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-12, TC-031 (second case). A function whose own body
/// node's real form disagrees with its declared body kind ("a form none of
/// this generator's classifiers admits") is refused with `FormMismatch`,
/// isolated to its own item.
#[test]
fn tc_031_ac12_form_mismatch_refuses_only_its_own_item() {
    let package = ext_corpus_package().admit();
    let functions = vec![
        function_form_mismatch("bad_fn"),
        function_unrelated("unrelated_fn"),
    ];
    let items = vec![
        item(ITEM_CALL_NESTED, "bad_fn"),
        item(ITEM_CALL_UNRELATED, "unrelated_fn"),
    ];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::FormMismatch { .. }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ExactFunctionDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-1 (duplicate handling, matching FR-014/FR-018). One
/// node id requested twice under one binding refuses every copy.
#[test]
fn tc_031_ac1_duplicate_request_refuses_every_copy() {
    let package = ext_corpus_package().admit();
    let functions = vec![function_add("add_fn")];
    let items = vec![item(ITEM_CALL_ADD, "add_fn"), item(ITEM_CALL_ADD, "add_fn")];
    let oracles = generate(&package, &functions, &items);
    assert_eq!(oracles.claim_map.items.len(), 1);
    assert!(matches!(
        &oracles.claim_map.items[0].result,
        ExactFunctionDisposition::Refused {
            refusal: ExactFunctionRefusal::DuplicateRequest
        }
    ));
}

pub fn contents(oracles: &quire_contract_codegen::ExactFunctionOracles, path: &str) -> String {
    oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("no artifact {path}"))
        .contents
        .clone()
}

/// Trace: FR-021-AC-13, TC-031. Generated bytes are identical across
/// repeated runs and across permutations of the request order; claim-map
/// entries are ordered by the item key (call node id, then the applied
/// function's declaring node id, then each argument operand's source node
/// id).
#[test]
fn tc_031_ac13_generation_is_deterministic_across_runs_and_permutations() {
    let package = ext_corpus_package().admit();
    let functions = main_functions();
    let items = main_items();

    let first = generate(&package, &functions, &items);
    let second = generate(&package, &functions, &items);
    assert_eq!(
        contents(&first, "src/lib.rs"),
        contents(&second, "src/lib.rs")
    );
    assert_eq!(
        contents(&first, "claim-map.json"),
        contents(&second, "claim-map.json")
    );

    let mut permuted_functions = functions.clone();
    permuted_functions.reverse();
    let mut permuted_items = items.clone();
    permuted_items.reverse();
    let third = generate(&package, &permuted_functions, &permuted_items);
    assert_eq!(
        contents(&first, "src/lib.rs"),
        contents(&third, "src/lib.rs")
    );
    assert_eq!(
        contents(&first, "claim-map.json"),
        contents(&third, "claim-map.json")
    );
}

/// Trace: FR-021-AC-13, TC-031: the committed golden crate.
#[test]
fn tc_031_ac13_generation_matches_the_committed_golden_files() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/exact_function");
    for (artifact, golden) in [
        ("Cargo.toml", "Cargo.toml.golden"),
        ("src/lib.rs", "lib.rs.golden"),
        ("claim-map.json", "claim-map.json.golden"),
        ("location-map.json", "location-map.json.golden"),
    ] {
        let path = fixtures.join(golden);
        if std::env::var_os(BLESS).is_some() {
            fs::write(&path, contents(&oracles, artifact)).expect("write golden");
        }
        let expected = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}; set {BLESS}=1", path.display()));
        assert_eq!(
            contents(&oracles, artifact),
            expected,
            "{artifact} drifted from {golden}"
        );
    }
}

/// Trace: FR-021-AC-14, TC-031. The generated crate declares
/// `publish = false`, pins the runtime revision with the `exact` feature,
/// contains no charge amount and no literal `Outcome`/`Value` constant
/// standing in for a runtime result, and forbids unsafe code.
#[test]
fn tc_031_ac14_manifest_and_source_shape() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(quire_contract_codegen::RUNTIME_REVISION));
    assert!(manifest.contains("features = [\"exact\"]"));
    assert!(manifest.contains("unsafe_code = \"forbid\""));

    let lib = contents(&oracles, "src/lib.rs");
    // No literal `Outcome::Completed(...)` construction standing in for a
    // runtime result: every `Outcome::Completed` in this source is a match
    // arm reconstructing a *runtime-computed* value, never a bare constant.
    assert!(!lib.contains("Outcome::Completed(rt::Value::Integer(rt::Integer::"));
    assert!(!lib.contains("Outcome::Completed(rt::Value::Boolean(true))"));
    assert!(!lib.contains("Outcome::Completed(rt::Value::Boolean(false))"));
}

/// Trace: FR-021-AC-15, TC-031. The location map records one entry per
/// generated function body, each `Location{origin, path}` re-derivable from
/// the request's own expression tree with no execution: every generated
/// function in this V1 has a body that is one classified node at its own
/// root, so `path` is always empty and `origin` always equals the
/// generator's own function ordering.
#[test]
fn tc_031_ac15_location_map_round_trips_to_the_request_structurally() {
    let package = ext_corpus_package().admit();
    let functions = main_functions();
    let oracles = generate(&package, &functions, &main_items());

    let mut ordered: Vec<_> = functions.iter().collect();
    ordered.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    let survivor_names: Vec<&str> = ordered
        .iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "add_fn" | "eq_fn" | "call_fn" | "unrelated_fn"
            )
        })
        .map(|f| f.name.as_str())
        .collect();

    assert_eq!(oracles.location_map.len(), survivor_names.len());
    for entry in &oracles.location_map {
        assert_eq!(entry.location.path, Vec::<usize>::new());
        let expected_index = survivor_names
            .iter()
            .position(|name| *name == entry.function)
            .unwrap_or_else(|| panic!("{} not a survivor", entry.function));
        match &entry.location.origin {
            quire_contract_codegen::RecordedOrigin::Body { function, index } => {
                assert_eq!(function, &entry.function);
                assert_eq!(*index, expected_index);
            }
        }
        let expected_kind = match entry.function.as_str() {
            "add_fn" => CallPointKind::ScalarOperator,
            "eq_fn" => CallPointKind::EqualityEvaluation,
            "call_fn" => CallPointKind::NestedCall,
            "unrelated_fn" => CallPointKind::ScalarOperator,
            other => panic!("unexpected function {other}"),
        };
        assert_eq!(entry.call_point, expected_kind);
    }
}

/// Trace: FR-021-AC-16, TC-031 (Inspection). No generated code or claim map
/// reads, branches on, or asserts non-emptiness of `Evaluation.location` or
/// `Evaluation.losses`: the generated oracle discards both by construction
/// (`.map(|evaluation| evaluation.outcome)`), and this test greps the
/// generated source and the serialized claim/location maps to confirm no
/// occurrence of either field name slipped in some other way.
#[test]
fn tc_031_ac16_no_generated_code_reads_location_or_losses() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    let lib = contents(&oracles, "src/lib.rs");
    // The header comment documents (in prose) that `Evaluation.location`/
    // `.losses` are never read; check for an actual field *read* --
    // `evaluation.location`/`evaluation.losses`, lowercase, the shape a real
    // access would take -- rather than banning the substring outright,
    // which the header's own documentation legitimately contains.
    assert!(!lib.contains("evaluation.location"));
    assert!(!lib.contains("evaluation.losses"));
    assert!(lib.contains(".map(|evaluation| evaluation.outcome)"));

    let claim_map = contents(&oracles, "claim-map.json");
    assert!(!claim_map.contains("\"location\""));
    assert!(!claim_map.contains("\"losses\""));
}
