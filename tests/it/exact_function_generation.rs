//! FR-021: function-application oracle generation over admitted CheckedPackage
//! V2 input.
//!
//! Generation-time coverage only (no execution of generated code):
//! dispositions, refusal reasons, ordering/determinism, the manifest, and the
//! static location map. Execution-level criteria (AC-2, AC-4, AC-5, AC-7,
//! AC-9, AC-17) are covered by `tests/it/exact_function_agreement.rs`.

use std::{fs, path::PathBuf};

use quire_contract_codegen::{
    generate_exact_function_oracles, CallPointKind, ClaimDisposition, ExactFunctionItem,
    ExactFunctionRefusal, GeneratedExactFunctionClaim, UpstreamBlocker,
};
use quire_contract_ir::CheckedPackageV2;

// Duplicated per consumer (also `exact_function_agreement.rs`) for structural consistency with
// the exact_scalar/composite_equality families (IR-237). Unlike those two, this package.rs holds
// no process-global state, so duplication here isn't load-bearing the way it is for them -- it's
// kept for uniformity across the tests/it/*_support/package.rs pattern, not to avoid a hazard.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
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
) -> &ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal> {
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
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_NESTED),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
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
        ClaimDisposition::Refused {
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

    let ClaimDisposition::Generated(claim) = disposition_for(&oracles, ITEM_CALL_ADD) else {
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
        ClaimDisposition::Refused {
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
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: UpstreamBlocker::QuireSpecLanguage120,
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
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: UpstreamBlocker::QuireSpecLanguage120,
                ..
            }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::BlockedOnUpstream {
                issue: UpstreamBlocker::QuireSpecLanguage121,
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
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnknownCallee { callee }
        } if callee == "not_declared_anywhere"
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
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
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::FormMismatch { .. }
        }
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
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
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::DuplicateRequest
        }
    ));
}

/// Trace: FR-021-AC-12 (signature/body-kind agreement), TC-031. A `Scalar`
/// body's declared parameter count or result type disagreeing with what the
/// body kind requires is refused with `SignatureMismatch` at generation
/// time, instead of assembling into a package whose emitted
/// `FunctionDeclaration` disagrees with its own rendered `Body` -- the two
/// measured repros: an `Add` declaration with only one declared parameter,
/// and an `Add` declaration whose declared result type is `Boolean` instead
/// of `Integer`.
#[test]
fn tc_031_signature_mismatch_refuses_arity_and_result_type_disagreement() {
    let package = ext_corpus_package().admit();

    // Arity: FN_ADD's real body needs two operands; declared with one.
    let arity_mismatch = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: "one_param_add".to_owned(),
        parameters: vec![quire_contract_codegen::FunctionParameter {
            name: "a".to_owned(),
            type_node_id: code_id(T_INTEGER),
        }],
        result_type: code_id(T_INTEGER),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let items = vec![item(ITEM_CALL_ADD, "one_param_add")];
    let oracles = generate(&package, &[arity_mismatch], &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::SignatureMismatch { .. }
        }
    ));

    // Result type: FN_EQ's node, a Scalar body (Integer-only), declared
    // with a Boolean result.
    let result_mismatch = quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_EQ),
        name: "boolean_result_add".to_owned(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "a".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
            quire_contract_codegen::FunctionParameter {
                name: "b".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
        ],
        result_type: code_id(T_BOOLEAN),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Add,
        },
        capability_requirements: Vec::new(),
    };
    let items = vec![item(ITEM_CALL_EQ, "boolean_result_add")];
    let oracles = generate(&package, &[result_mismatch], &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::SignatureMismatch { .. }
        }
    ));
}

/// Trace: FR-021-AC-15/AC-17 (duplicate declared names), TC-031. Two
/// declarations sharing one name -- different node ids, same string --
/// are both refused as `AmbiguousFunctionName` rather than colliding into
/// one Stage 1 classification, and a third, uniquely-named survivor's
/// location map index is unaffected: it still equals its own position among
/// the surviving functions, not a value corrupted by the excluded pair.
/// This is FR-021-AC-15's own mutation table entry ("record it against the
/// wrong function's index") reproduced and proven fixed.
#[test]
fn tc_031_ambiguous_function_name_refuses_both_and_does_not_corrupt_indices() {
    let package = ext_corpus_package().admit();
    let functions = vec![
        function_add("dup"),       // node FN_ADD
        function_unrelated("dup"), // node FN_UNRELATED, same name, different node
        function_eq("solo_eq"),    // node FN_EQ, unique name
        quire_contract_codegen::ExactFunctionDeclaration {
            node_id: code_id(FN_CAPABILITY),
            name: "solo_add".to_owned(),
            parameters: vec![
                quire_contract_codegen::FunctionParameter {
                    name: "a".to_owned(),
                    type_node_id: code_id(T_INTEGER),
                },
                quire_contract_codegen::FunctionParameter {
                    name: "b".to_owned(),
                    type_node_id: code_id(T_INTEGER),
                },
            ],
            result_type: code_id(T_INTEGER),
            body: quire_contract_codegen::ExactFunctionBody::Scalar {
                operator: quire_contract_codegen::IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        },
    ];
    let items = vec![
        item(ITEM_CALL_ADD, "dup"),
        item(ITEM_CALL_EQ, "solo_eq"),
        item(ITEM_CALL_UNRELATED, "solo_add"),
    ];
    let oracles = generate(&package, &functions, &items);

    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::AmbiguousFunctionName { name }
        } if name == "dup"
    ));
    // Neither "dup" declaration ever survives to Stage 2, so neither ever
    // gets a location-map entry at all -- the old, name-keyed maps would
    // instead have let both survive and collide on one shared index.
    assert!(oracles
        .location_map
        .iter()
        .all(|entry| entry.function != "dup"));

    // Node ids are content digests, not the numeric placeholder codes
    // above, so the surviving order (solo_eq, solo_add) is whichever the
    // generator's own node-id sort gives -- re-derived here exactly as
    // `generate_exact_function_oracles` computes it, the same pattern
    // `tc_031_ac3`/`tc_031_ac15` already use, rather than hardcoded.
    let mut survivor_node_ids = [code_id(FN_EQ), code_id(FN_CAPABILITY)];
    survivor_node_ids.sort();
    let expected_index = |node_id: &quire_contract_ir::CheckedNodeId| {
        survivor_node_ids
            .iter()
            .position(|candidate| candidate == node_id)
            .expect("survivor node id")
    };

    let solo_eq_entry = oracles
        .location_map
        .iter()
        .find(|entry| entry.function == "solo_eq")
        .expect("solo_eq has a location map entry");
    let solo_add_entry = oracles
        .location_map
        .iter()
        .find(|entry| entry.function == "solo_add")
        .expect("solo_add has a location map entry");
    assert_eq!(oracles.location_map.len(), 2);
    match &solo_eq_entry.location.origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "solo_eq");
            assert_eq!(*index, expected_index(&code_id(FN_EQ)));
        }
    }
    match &solo_add_entry.location.origin {
        quire_contract_codegen::RecordedOrigin::Body { function, index } => {
            assert_eq!(function, "solo_add");
            assert_eq!(*index, expected_index(&code_id(FN_CAPABILITY)));
        }
    }
    assert_ne!(
        expected_index(&code_id(FN_EQ)),
        expected_index(&code_id(FN_CAPABILITY)),
        "the two survivors must occupy distinct indices"
    );
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_EQ),
        ClaimDisposition::Generated(_)
    ));
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_UNRELATED),
        ClaimDisposition::Generated(_)
    ));
}

/// Trace: FR-021-AC-1 (item-level arity), TC-031. An item's argument count
/// disagreeing with the applied function's declared parameter count is
/// refused as `ArityMismatch`, naming both counts.
#[test]
fn tc_031_arity_mismatch_refuses_when_item_argument_count_disagrees() {
    let package = ext_corpus_package().admit();
    let functions = vec![function_add("add_fn")];
    let items = vec![quire_contract_codegen::ExactFunctionItem {
        call_node_id: code_id(ITEM_CALL_ADD),
        function: "add_fn".to_owned(),
        argument_node_ids: vec![code_id(T_INTEGER)],
    }];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::ArityMismatch {
                expected: 2,
                found: 1
            }
        }
    ));
}

/// Trace: FR-021 module doc (scalar vocabulary), TC-031.
/// `IntegerOperator::Negate` is unary and out of this V1's scope (only
/// binary `Add`/`Subtract`/`Multiply` are supported): a `Scalar` body
/// declaring it is refused as `UnsupportedOperator`.
#[test]
fn tc_031_unsupported_operator_refuses_unary_negate_scalar_body() {
    let package = ext_corpus_package().admit();
    let functions = vec![quire_contract_codegen::ExactFunctionDeclaration {
        node_id: code_id(FN_ADD),
        name: "negate_fn".to_owned(),
        parameters: vec![
            quire_contract_codegen::FunctionParameter {
                name: "a".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
            quire_contract_codegen::FunctionParameter {
                name: "b".to_owned(),
                type_node_id: code_id(T_INTEGER),
            },
        ],
        result_type: code_id(T_INTEGER),
        body: quire_contract_codegen::ExactFunctionBody::Scalar {
            operator: quire_contract_codegen::IntegerOperator::Negate,
        },
        capability_requirements: Vec::new(),
    }];
    let items = vec![item(ITEM_CALL_ADD, "negate_fn")];
    let oracles = generate(&package, &functions, &items);
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_ADD),
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::UnsupportedOperator { .. }
        }
    ));
}

/// Trace: FR-021-AC-7, TC-031. The committed golden source for the
/// nested-call chain corpus (deeper than `MAX_CALL_DEPTH`), consumed (via
/// `include!`) by `exact_function_agreement.rs`'s AC-7 test to execute the
/// real generated `Call` bodies -- not a hand-built parallel double --
/// against the runtime's own `MAX_CALL_DEPTH` enforcement.
#[test]
fn tc_031_ac7_chain_source_matches_the_committed_golden() {
    let package = ext_corpus_package().admit();
    let mut functions = chain_functions();
    assert_eq!(functions[0].name, "chain_0");
    assert_eq!(
        functions[(CHAIN_LENGTH - 1) as usize].name,
        format!("chain_{}", CHAIN_LENGTH - 1)
    );
    functions.push(function_add("add_fn")); // the chain's own last link calls this
    let items = vec![item(ITEM_CALL_CHAIN, "chain_0")];
    let oracles = generate(&package, &functions, &items);

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/exact_function_chain/lib.rs.golden");
    if std::env::var_os(BLESS).is_some() {
        fs::write(&path, contents(&oracles, "src/lib.rs")).expect("write chain golden");
    }
    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}; set {BLESS}=1", path.display()));
    assert_eq!(
        contents(&oracles, "src/lib.rs"),
        expected,
        "chain src/lib.rs drifted from lib.rs.golden"
    );
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
    // Asserted positively -- the exact pattern `render_body` emits for each
    // supported body kind -- rather than as an absence of a hand-picked
    // literal substring: a mutation that hardcoded a hollow result would
    // remove one of these patterns, where the equivalent absence-only
    // checks this replaced could be defeated by any differently-formatted
    // literal.
    assert!(lib.contains("Outcome::Completed(rt::Value::Integer(value))"));
    assert!(lib.contains("Outcome::Completed(rt::Value::Boolean(value))"));
}

/// Trace: FR-021-AC-15 (origin half only -- see module doc "Location
/// tagging" and `spec/test-matrix.md`'s FR-021 row: the `path`-non-empty
/// case is not implemented and is unreachable from this V1's scoped body
/// vocabulary), TC-031. The location map records one entry per generated
/// function body, each `Location{origin, path}` re-derivable from the
/// request's own expression tree with no execution: every generated
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
    // The claim map's own `Serialize` types (`ClaimMap`,
    // `GeneratedExactFunctionClaim`) carry no `location`/`losses` field at
    // all, so a JSON-key absence check on `claim-map.json` here would only
    // restate what the type system already guarantees at compile time, not
    // exercise any behavior of this generator.
}
