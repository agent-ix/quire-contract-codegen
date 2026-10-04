//! FR-021: function-application oracle generation over admitted CheckedPackage
//! V2 input.
//!
//! Generation-time coverage only (no execution of generated code):
//! dispositions, refusal reasons, ordering/determinism, the manifest, and the
//! static location map. Execution-level criteria (AC-2, AC-4, AC-5, AC-7,
//! AC-9, AC-17) are covered by `tests/it/exact_function_agreement.rs`.

use crate::exact_scalar_generation::invokes_panicking_macro;
use crate::scratch_crate::runtime_dependency;
use quire_contract_codegen::{
    generate_exact_function_oracles, CallPointKind, ClaimDisposition, ExactFunctionItem,
    ExactFunctionRefusal, GeneratedExactFunctionClaim, UpstreamBlocker,
};
use quire_contract_model::CheckedPackageV2;

// Duplicated per consumer (also `exact_function_agreement.rs`) for structural consistency with
// the exact_scalar/composite_equality families (IR-237). Unlike those two, this package.rs holds
// no process-global state, so duplication here isn't load-bearing the way it is for them -- it's
// kept for uniformity across the tests/it/*_support/package.rs pattern, not to avoid a hazard.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
mod package;
use package::*;

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

/// Trace: FR-021-AC-23, TC-031.
///
/// The generator lowers twice, the declared functions' bodies and then the requested `call`
/// nodes. Read under a ceiling that admits the checked package and is one byte below the shorter
/// of the two lowered packages, both lowerings fail for bytes (Contract IR FR-038-AC-95): every
/// function is absent from `checked_package()` and the location map, and every item is refused as
/// `LoweringByteLimitExceeded` with the ceiling as `limit` and the call-node lowering's
/// `consumed`, which the fixture makes differ from the body lowering's, so the call-node-first
/// order is asserted through the call. Never `LoweringWorkExhausted`. The mapping of a record to
/// a refusal is asserted on hand-built records in the module's own tests; per-function isolation
/// is FR-021-AC-12's.
#[test]
fn tc_031_ac23_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    use quire_contract_model::{CheckedNodeId, CheckedPackageLimit, CompleteLoweringRecordV2};

    let builder = byte_ceiling_package();
    let functions = byte_ceiling_functions();
    let items = byte_ceiling_items();
    let body_nodes = functions
        .iter()
        .map(|function| function.node_id.clone())
        .collect::<Vec<_>>();
    let call_nodes = items
        .iter()
        .map(|item| item.call_node_id.clone())
        .collect::<Vec<_>>();
    let byte_refusal = |claim: &quire_contract_codegen::ExactFunctionClaim| match &claim.result {
        ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::LoweringByteLimitExceeded { limit, consumed },
        } => Some((*limit, *consumed)),
        _ => None,
    };

    let checked_length = u64::try_from(serde_json::to_vec(&builder.wire()).unwrap().len()).unwrap();
    let generous = builder.admit_with(limits_under(LARGEST_CEILING));
    let profile = measuring_profile(false);
    let body_length = lowered_package_length(&generous, &body_nodes, &profile);
    let call_length = lowered_package_length(&generous, &call_nodes, &profile);
    let ceiling = body_length.min(call_length) - 1;
    assert!(
        ceiling >= checked_length,
        "both lowered packages ({body_length}, {call_length} bytes) are longer than the checked \
         package ({checked_length} bytes), so a ceiling one byte below the shorter admits it"
    );

    // Each lowering alone, under the same ceiling: both fail for bytes, with different `consumed`.
    let package = builder.admit_with(limits_under(ceiling));
    let consumed_alone =
        |requested: &[CheckedNodeId]| match &package.lower(requested, &profile).records[..] {
            [CompleteLoweringRecordV2::Failed {
                limit_kind: CheckedPackageLimit::Bytes,
                limit,
                consumed,
                ..
            }, ..] => {
                assert_eq!(*limit, ceiling);
                *consumed
            }
            other => panic!("expected a byte-ceiling failure, got {other:?}"),
        };
    let body_consumed = consumed_alone(&body_nodes);
    let call_consumed = consumed_alone(&call_nodes);
    assert_ne!(body_consumed, call_consumed);

    let oracles = generate(&package, &functions, &items);
    assert_eq!(oracles.claim_map.items.len(), items.len());
    for claim in &oracles.claim_map.items {
        assert_eq!(
            byte_refusal(claim),
            Some((ceiling, call_consumed)),
            "the call-node lowering's refusal is checked first: {claim:?}"
        );
    }
    assert!(oracles.location_map.is_empty(), "no function is located");
    let lib = oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("src/lib.rs");
    assert!(
        !lib.contents.contains("byte_fn_"),
        "every function is absent from checked_package()"
    );

    // At the longer lowered package's own length neither lowering fails for bytes.
    let exact = builder.admit_with(limits_under(body_length.max(call_length)));
    assert!(generate(&exact, &functions, &items)
        .claim_map
        .items
        .iter()
        .all(|claim| byte_refusal(claim).is_none()));
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

    // Two items name this call node (`add_fn` and `eq_fn`); their order in the claim map follows
    // the declaring node ids' digests, so select the claim by the function it records.
    let claim = oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == code_id(ITEM_CALL_ADD))
        .find_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated) if generated.function == "add_fn" => {
                Some(generated)
            }
            _ => None,
        })
        .expect("a generated claim for add_fn over the call node");
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
/// split: `spec/oracle/matrix/tests.md`'s FR-018 row records the identical gap.)
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
    let expected_index = |node_id: &quire_contract_model::CheckedNodeId| {
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

/// Trace: FR-021-AC-21, TC-031. `IntegerOperator::Negate` is unary and out of
/// this V1's scope (only binary `Add`/`Subtract`/`Multiply` are supported): a
/// `Scalar` body declaring it is refused as `UnsupportedOperator` and the
/// function is absent from the emitted `checked_package()`.
#[test]
fn tc_031_ac21_unsupported_operator_refuses_unary_negate_scalar_body() {
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
    let lib = contents(&oracles, "src/lib.rs");
    assert!(
        !lib.contains("negate_fn"),
        "a refused Negate function must not appear in checked_package()"
    );
    assert!(
        !lib.contains("rt::FunctionDeclaration {"),
        "no function survives, so checked_package() declares none"
    );
}

/// `declaration` moved onto the node `code`.
fn on_node(
    mut declaration: quire_contract_codegen::ExactFunctionDeclaration,
    code: u32,
) -> quire_contract_codegen::ExactFunctionDeclaration {
    declaration.node_id = code_id(code);
    declaration
}

/// Every ordering of `items`.
fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let Some((first, rest)) = items.split_first() else {
        return vec![Vec::new()];
    };
    let mut all = Vec::new();
    for tail in permutations(rest) {
        for at in 0..=tail.len() {
            let mut order = tail.clone();
            order.insert(at, first.clone());
            all.push(order);
        }
    }
    all
}

/// One FR-021-AC-22 fixture: declarations of which `pair_names` share the declaring node
/// `FN_ADD`, and the requested `(call node, function)` items.
struct DuplicateNodeFixture {
    functions: Vec<quire_contract_codegen::ExactFunctionDeclaration>,
    items: Vec<(u32, &'static str)>,
    /// Names held by a declaration sharing `FN_ADD`.
    pair_names: Vec<&'static str>,
    /// Every function name that must be absent from `checked_package()` and the location map.
    absent_names: Vec<&'static str>,
    /// Items (call node codes) that must be refused `DuplicateDeclaringNode`.
    duplicate_items: Vec<u32>,
    /// Items over a function with its own node id and name; each must equal the entry the same
    /// request produces with every declaration in `absent_names` removed.
    distinct_items: Vec<u32>,
}

impl DuplicateNodeFixture {
    /// Check the fixture under every permutation of the declaration order (which includes both
    /// orders of the pair).
    fn check(&self, package: &CheckedPackageV2) {
        let requested: Vec<ExactFunctionItem> = self
            .items
            .iter()
            .map(|(code, function)| item(*code, function))
            .collect();
        let baseline_functions: Vec<_> = self
            .functions
            .iter()
            .filter(|declaration| !self.absent_names.contains(&declaration.name.as_str()))
            .cloned()
            .collect();
        let baseline_items: Vec<ExactFunctionItem> = self
            .items
            .iter()
            .filter(|(code, _)| self.distinct_items.contains(code))
            .map(|(code, function)| item(*code, function))
            .collect();
        let baseline = generate(package, &baseline_functions, &baseline_items);

        let mut first: Option<quire_contract_codegen::ExactFunctionOracles> = None;
        for order in permutations(&self.functions) {
            let oracles = generate(package, &order, &requested);
            match &first {
                Some(first) => assert_eq!(first, &oracles, "request order changed the output"),
                None => first = Some(oracles.clone()),
            }
            self.check_one(&oracles, &baseline);
        }
    }

    fn check_one(
        &self,
        oracles: &quire_contract_codegen::ExactFunctionOracles,
        baseline: &quire_contract_codegen::ExactFunctionOracles,
    ) {
        let lib = contents(oracles, "src/lib.rs");
        let location_json = contents(oracles, "location-map.json");
        for name in &self.absent_names {
            assert!(
                !lib.contains(&format!("{name:?}")),
                "{name} is in src/lib.rs"
            );
            assert!(
                !location_json.contains(&format!("{name:?}")),
                "{name} is in location-map.json"
            );
            assert!(oracles
                .location_map
                .iter()
                .all(|entry| entry.function != *name));
        }
        for code in &self.duplicate_items {
            assert!(
                matches!(
                    disposition_for(oracles, *code),
                    ClaimDisposition::Refused {
                        refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id }
                    } if *node_id == code_id(FN_ADD)
                ),
                "item {code} is not DuplicateDeclaringNode: {:?}",
                disposition_for(oracles, *code)
            );
        }
        // No claim records a pair member's identity, and no generated claim shares an oracle
        // symbol or origin index with another.
        let mut symbols = Vec::new();
        let mut indices = Vec::new();
        for claim in &oracles.claim_map.items {
            if let ClaimDisposition::Generated(generated) = &claim.result {
                assert!(!self.pair_names.contains(&generated.function.as_str()));
                symbols.push(generated.oracle_symbol.clone());
                indices.push(format!("{:?}", generated.function_origin));
            }
        }
        let distinct = (symbols.len(), indices.len());
        symbols.sort();
        symbols.dedup();
        indices.sort();
        indices.dedup();
        assert_eq!((symbols.len(), indices.len()), distinct);
        for code in &self.distinct_items {
            assert!(
                matches!(
                    disposition_for(oracles, *code),
                    ClaimDisposition::Generated(_)
                ),
                "distinct item {code} was not generated"
            );
            assert_eq!(
                disposition_for(oracles, *code),
                disposition_for(baseline, *code),
                "item {code} differs from the request without the duplicates"
            );
        }
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (i). Two declarations sharing one declaring node
/// id and both admissible are both refused, whichever order they arrive in: neither enters
/// `checked_package()`, the source or `location-map.json`, and each item naming one is
/// `DuplicateDeclaringNode`, never `UnknownFunction`. A distinct-node function is generated
/// unchanged.
#[test]
fn tc_031_ac22_fixture_i_both_bodies_admissible() {
    let package = ext_corpus_package().admit();
    DuplicateNodeFixture {
        functions: vec![
            function_add("pair_one"),
            function_add("pair_two"),
            function_eq("solo"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "pair_one"),
            (ITEM_CALL_EQ, "pair_two"),
            (ITEM_CALL_UNRELATED, "solo"),
        ],
        pair_names: vec!["pair_one", "pair_two"],
        absent_names: vec!["pair_one", "pair_two"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (ii). One declaration of the pair would be
/// refused in Stage 1 (an undischargeable capability) and its same-node sibling is admissible:
/// the sibling does not survive, and neither item reports `UnknownFunction`, nor the refused
/// one's own Stage 1 reason.
#[test]
fn tc_031_ac22_fixture_ii_one_body_refused_and_sibling_admissible() {
    let package = ext_corpus_package().admit();
    DuplicateNodeFixture {
        functions: vec![
            on_node(function_capability("pair_one"), FN_ADD),
            function_add("pair_two"),
            function_eq("solo"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "pair_one"),
            (ITEM_CALL_EQ, "pair_two"),
            (ITEM_CALL_UNRELATED, "solo"),
        ],
        pair_names: vec!["pair_one", "pair_two"],
        absent_names: vec!["pair_one", "pair_two"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (iii). The pair also shares one name, and a third
/// declaration with its own node id holds the same name: the third is refused as
/// `AmbiguousFunctionName` and is absent from `checked_package()` and the location map, while
/// items naming the shared name are `DuplicateDeclaringNode`, never `AmbiguousFunctionName`.
#[test]
fn tc_031_ac22_fixture_iii_shared_name_takes_the_node_id_refusal() {
    let package = ext_corpus_package().admit();
    DuplicateNodeFixture {
        functions: vec![
            function_add("shared"),
            function_add("shared"),
            function_unrelated("shared"),
            function_eq("solo"),
        ],
        items: vec![(ITEM_CALL_ADD, "shared"), (ITEM_CALL_UNRELATED, "solo")],
        pair_names: vec!["shared"],
        absent_names: vec!["shared"],
        duplicate_items: vec![ITEM_CALL_ADD],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    }
    .check(&package);
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (iv). A declaration whose nested `call` names a
/// member of the pair is refused as `UnknownCallee`, and a distinct-node function and an item
/// naming each function are handled: the pair's items `DuplicateDeclaringNode`, the caller's
/// item `UnknownCallee`, the distinct function's generated.
#[test]
fn tc_031_ac22_fixture_iv_nested_call_to_a_refused_duplicate_is_unknown_callee() {
    let package = ext_corpus_package().admit();
    let fixture = DuplicateNodeFixture {
        functions: vec![
            function_add("pair_one"),
            function_add("pair_two"),
            function_call_nested("caller", "pair_one"),
            function_eq("solo"),
        ],
        items: vec![
            (ITEM_CALL_ADD, "pair_one"),
            (ITEM_CALL_EQ, "pair_two"),
            (ITEM_CALL_NESTED, "caller"),
            (ITEM_CALL_UNRELATED, "solo"),
        ],
        pair_names: vec!["pair_one", "pair_two"],
        absent_names: vec!["pair_one", "pair_two", "caller"],
        duplicate_items: vec![ITEM_CALL_ADD, ITEM_CALL_EQ],
        distinct_items: vec![ITEM_CALL_UNRELATED],
    };
    fixture.check(&package);

    let requested: Vec<ExactFunctionItem> = fixture
        .items
        .iter()
        .map(|(code, function)| item(*code, function))
        .collect();
    for order in permutations(&fixture.functions) {
        let oracles = generate(&package, &order, &requested);
        assert!(matches!(
            disposition_for(&oracles, ITEM_CALL_NESTED),
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::UnknownCallee { callee }
            } if callee == "pair_one"
        ));
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (v). One name is held by two duplicate groups, on
/// `FN_ADD` and on `FN_EQ`: an item naming it is refused `DuplicateDeclaringNode` carrying the
/// smaller of the two node ids, under every order of the declarations.
#[test]
fn tc_031_ac22_fixture_v_two_duplicate_groups_report_the_smallest_node_id() {
    let package = ext_corpus_package().admit();
    let smallest = std::cmp::min(code_id(FN_ADD), code_id(FN_EQ));
    let functions = vec![
        function_add("shared"),
        function_add("shared"),
        function_eq("shared"),
        function_eq("shared"),
    ];
    let requested = vec![item(ITEM_CALL_ADD, "shared")];
    for order in permutations(&functions) {
        let oracles = generate(&package, &order, &requested);
        match disposition_for(&oracles, ITEM_CALL_ADD) {
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id },
            } => assert_eq!(*node_id, smallest),
            other => panic!("expected DuplicateDeclaringNode, got {other:?}"),
        }
        assert!(oracles.location_map.is_empty());
    }
}

/// Trace: FR-021-AC-22, TC-031 step 9 fixture (vi). Two items on one call node naming the two
/// members of a duplicate pair, and one such item requested twice, each get
/// `DuplicateDeclaringNode`: the refusal takes precedence over the duplicate-request collapse.
#[test]
fn tc_031_ac22_fixture_vi_same_call_node_items_over_a_pair_each_keep_the_node_refusal() {
    let package = ext_corpus_package().admit();
    let functions = vec![function_add("pair_one"), function_add("pair_two")];
    let requested = vec![
        item(ITEM_CALL_ADD, "pair_one"),
        item(ITEM_CALL_ADD, "pair_two"),
        item(ITEM_CALL_EQ, "pair_one"),
        item(ITEM_CALL_EQ, "pair_one"),
    ];
    for order in permutations(&functions) {
        let oracles = generate(&package, &order, &requested);
        assert_eq!(oracles.claim_map.items.len(), 3);
        for claim in &oracles.claim_map.items {
            assert!(
                matches!(
                    &claim.result,
                    ClaimDisposition::Refused {
                        refusal: ExactFunctionRefusal::DuplicateDeclaringNode { node_id }
                    } if *node_id == code_id(FN_ADD)
                ),
                "{:?}",
                claim.result
            );
        }
    }
}

/// The dispositions of `oracles`' claim-map entries, in claim-map order.
fn entries(
    oracles: &quire_contract_codegen::ExactFunctionOracles,
) -> Vec<&ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal>> {
    oracles
        .claim_map
        .items
        .iter()
        .map(|claim| &claim.result)
        .collect()
}

fn unknown(name: &str) -> ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal> {
    ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::UnknownFunction {
            name: name.to_owned(),
        },
    }
}

/// The entries for `names` requested on `ITEM_CALL_ADD` in each of the two request orders (the
/// order given, then reversed), asserted identical to each other, and returned once.
fn both_orders(
    package: &CheckedPackageV2,
    functions: &[quire_contract_codegen::ExactFunctionDeclaration],
    names: [&str; 2],
) -> quire_contract_codegen::ExactFunctionOracles {
    let forward = generate(
        package,
        functions,
        &[item(ITEM_CALL_ADD, names[0]), item(ITEM_CALL_ADD, names[1])],
    );
    let reverse = generate(
        package,
        functions,
        &[item(ITEM_CALL_ADD, names[1]), item(ITEM_CALL_ADD, names[0])],
    );
    assert_eq!(forward, reverse, "request order changed the output");
    forward
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (i). One unknown name on a call node is one
/// `UnknownFunction` entry naming it.
#[test]
fn tc_031_ac24_case_i_one_unknown_name_is_one_entry() {
    let package = ext_corpus_package().admit();
    let oracles = generate(
        &package,
        &[function_add("add_fn")],
        &[item(ITEM_CALL_ADD, "zz_unknown")],
    );
    assert_eq!(entries(&oracles), vec![&unknown("zz_unknown")]);
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (ii). Two different unknown names on one call node
/// with equal arguments are two entries, `aa_unknown` then `zz_unknown`, never `DuplicateRequest`,
/// identical under both request orders.
#[test]
fn tc_031_ac24_case_ii_two_unknown_names_are_two_entries_in_name_order() {
    let package = ext_corpus_package().admit();
    let oracles = both_orders(
        &package,
        &[function_add("add_fn")],
        ["zz_unknown", "aa_unknown"],
    );
    assert_eq!(
        entries(&oracles),
        vec![&unknown("aa_unknown"), &unknown("zz_unknown")]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (iii). The same unknown name requested twice is one
/// `DuplicateRequest` entry.
#[test]
fn tc_031_ac24_case_iii_the_same_unknown_name_twice_is_one_duplicate_request() {
    let package = ext_corpus_package().admit();
    let oracles = generate(
        &package,
        &[function_add("add_fn")],
        &[
            item(ITEM_CALL_ADD, "zz_unknown"),
            item(ITEM_CALL_ADD, "zz_unknown"),
        ],
    );
    assert_eq!(
        entries(&oracles),
        vec![&ClaimDisposition::Refused {
            refusal: ExactFunctionRefusal::DuplicateRequest
        }]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (iv). A declared `add_fn` and `zz_unknown` on one
/// call node are two entries, the unknown one first, each equal to the entry its item gets when
/// requested alone.
#[test]
fn tc_031_ac24_case_iv_known_and_unknown_order_unknown_first_and_match_solo() {
    let package = ext_corpus_package().admit();
    let functions = [function_add("add_fn")];
    let oracles = both_orders(&package, &functions, ["add_fn", "zz_unknown"]);
    let solo_add = generate(&package, &functions, &[item(ITEM_CALL_ADD, "add_fn")]);
    let solo_unknown = generate(&package, &functions, &[item(ITEM_CALL_ADD, "zz_unknown")]);
    assert_eq!(solo_unknown.claim_map.items.len(), 1);
    assert_eq!(solo_add.claim_map.items.len(), 1);
    assert!(matches!(
        &solo_add.claim_map.items[0].result,
        ClaimDisposition::Generated(_)
    ));
    assert_eq!(
        oracles.claim_map.items,
        [
            solo_unknown.claim_map.items[0].clone(),
            solo_add.claim_map.items[0].clone()
        ]
    );
    assert_eq!(oracles.claim_map.items[0].result, unknown("zz_unknown"));
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (v). Names order byte-wise and case-sensitively:
/// `Zz_unknown` (`Z` is 0x5A) before `aa_unknown` (`a` is 0x61), identical under both orders.
#[test]
fn tc_031_ac24_case_v_names_order_by_bytes_case_sensitively() {
    let package = ext_corpus_package().admit();
    let oracles = both_orders(
        &package,
        &[function_add("add_fn")],
        ["aa_unknown", "Zz_unknown"],
    );
    assert_eq!(
        entries(&oracles),
        vec![&unknown("Zz_unknown"), &unknown("aa_unknown")]
    );
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (vi). Items on one call node naming the two members
/// of a duplicate-node pair are two entries ordered by function name. With `m_multi` and `z_pair`
/// on the larger node N2 and `m_multi` and `q_extra` on the smaller node N1, `m_multi` (whose
/// refusal carries N1) orders before `z_pair` (N2); with `z_multi` and `a_pair`, `a_pair` (N2)
/// orders before `z_multi` (N1). Identical under both request orders and every declaration order.
#[test]
fn tc_031_ac24_case_vi_duplicate_node_pair_members_order_by_name() {
    let package = ext_corpus_package().admit();
    let (n1, n2) = if code_id(FN_ADD) < code_id(FN_EQ) {
        (FN_ADD, FN_EQ)
    } else {
        (FN_EQ, FN_ADD)
    };
    let node_refusal = |code: u32| ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::DuplicateDeclaringNode {
            node_id: code_id(code),
        },
    };
    for (shared, pair_member, extra, expected) in [
        ("m_multi", "z_pair", "q_extra", [n1, n2]),
        ("z_multi", "a_pair", "q_extra", [n2, n1]),
    ] {
        let functions = vec![
            on_node(function_add(shared), n2),
            on_node(function_add(pair_member), n2),
            on_node(function_add(shared), n1),
            on_node(function_add(extra), n1),
        ];
        // The member named `shared` is requested alongside `pair_member`; the order of the two
        // refusals is the order of their names, not of the request.
        let (first, second) = if shared < pair_member {
            (shared, pair_member)
        } else {
            (pair_member, shared)
        };
        for order in permutations(&functions) {
            let oracles = both_orders(&package, &order, [first, second]);
            assert_eq!(
                entries(&oracles),
                vec![&node_refusal(expected[0]), &node_refusal(expected[1])],
                "{shared} and {pair_member}"
            );
        }
    }
}

/// Trace: FR-021-AC-24, TC-031 step 12 case (vii). The name tie-break applies only to items equal
/// on the earlier key fields: items whose declaring node ids differ are ordered by declaring node
/// id first. With `a_pair` and `z_pair` on N2, `z_pair` and `q_extra` on N1, and `a_pair` alone on
/// the larger N3, `a_pair` resolves to N3 and `z_pair` to N2, so `z_pair` (refusal N1) orders
/// before `a_pair` (refusal N2) although `a_pair` is the smaller name, under both request orders
/// and every declaration order.
#[test]
fn tc_031_ac24_case_vii_differing_declaring_node_ids_order_before_names() {
    let package = ext_corpus_package().admit();
    let mut codes = [FN_ADD, FN_EQ, FN_CALL_NESTED];
    codes.sort_by_key(|code| code_id(*code));
    let [n1, n2, n3] = codes;
    let functions = vec![
        on_node(function_add("a_pair"), n2),
        on_node(function_add("z_pair"), n2),
        on_node(function_add("z_pair"), n1),
        on_node(function_add("q_extra"), n1),
        on_node(function_add("a_pair"), n3),
    ];
    let node_refusal = |code: u32| ClaimDisposition::Refused {
        refusal: ExactFunctionRefusal::DuplicateDeclaringNode {
            node_id: code_id(code),
        },
    };
    for order in permutations(&functions) {
        let oracles = both_orders(&package, &order, ["a_pair", "z_pair"]);
        assert_eq!(
            entries(&oracles),
            vec![&node_refusal(n1), &node_refusal(n2)]
        );
    }
}

/// Trace: FR-021-AC-19, TC-031. The emitted `src/lib.rs` of the main corpus
/// (scalar `add_fn`, equality `eq_fn`, nested-call `call_fn`) and of the chain
/// corpus contains no `.unwrap(`, no `.expect(` and no panicking macro in any
/// delimiter form.
#[test]
fn tc_031_ac19_emitted_function_oracle_source_has_no_panicking_path() {
    for (corpus, oracles) in [("main", main_oracles()), ("chain", chain_oracles())] {
        let lib = contents(&oracles, "src/lib.rs");
        assert!(lib.contains("pub fn checked_package()"), "{corpus} corpus");
        for forbidden in [".unwrap(", ".expect("] {
            assert!(
                !lib.contains(forbidden),
                "{corpus} corpus emits {forbidden}"
            );
        }
        assert!(
            !invokes_panicking_macro(&lib),
            "{corpus} corpus emits a panicking macro"
        );
    }
    let lib = contents(&main_oracles(), "src/lib.rs");
    assert!(lib.contains("rt::TypeEnvironment::default()"));
}

/// Trace: FR-021-AC-20, TC-031. In the emitted bodies of `add_fn` and `eq_fn`
/// the `match` over the runtime operator's `Result<Outcome<_>, Refusal>` has
/// the five arms in order and ends with a catch-all `Ok(_)` arm valued
/// `Outcome::Refused(Refusal::CheckedInvariant)`. The unknown variant cannot
/// be built from a test crate (`rt::Outcome` is `#[non_exhaustive]`), so the
/// arm's text is the evidence.
#[test]
fn tc_031_ac20_unknown_outcome_variant_refuses_checked_invariant() {
    let lib = contents(&main_oracles(), "src/lib.rs");
    for (function, completed) in [
        (
            "add_fn",
            "Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Integer(value)),",
        ),
        (
            "eq_fn",
            "Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Boolean(value)),",
        ),
    ] {
        let name = format!("name: {function:?}.to_owned(),");
        let start = lib
            .find(&name)
            .unwrap_or_else(|| panic!("{function} is not declared"));
        let rest = &lib[start + name.len()..];
        let body = &rest[..rest.find("rt::FunctionDeclaration {").unwrap_or(rest.len())];

        let arms = [
            completed,
            "Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),",
            "Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),",
            "Ok(rt::Outcome::Incomplete(incomplete)) => rt::Outcome::Incomplete(incomplete),",
            "Err(refusal) => rt::Outcome::Refused(refusal),",
            "Ok(_) => rt::Outcome::Refused(rt::Refusal::CheckedInvariant),",
        ];
        let mut from = 0;
        for arm in arms {
            let at = body[from..]
                .find(arm)
                .unwrap_or_else(|| panic!("{function}: arm `{arm}` missing or out of order"));
            from += at + arm.len();
        }
        let tail: String = body[from..].split_whitespace().collect();
        assert!(
            tail.starts_with('}'),
            "{function}: the catch-all must be the final arm, found `{tail}`"
        );
    }
}

/// The generator's current output for the main corpus, which
/// `exact_function_agreement` builds and executes.
pub(super) fn main_oracles() -> quire_contract_codegen::ExactFunctionOracles {
    generate(
        &ext_corpus_package().admit(),
        &main_functions(),
        &main_items(),
    )
}

/// The generator's current output for the nested-call chain corpus (deeper
/// than `MAX_CALL_DEPTH`), which `exact_function_agreement` executes for AC-7.
pub(super) fn chain_oracles() -> quire_contract_codegen::ExactFunctionOracles {
    let package = ext_corpus_package().admit();
    let mut functions = chain_functions();
    functions.push(function_add("add_fn")); // the chain's own last link calls this
    generate(&package, &functions, &[item(ITEM_CALL_CHAIN, "chain_0")])
}

/// Trace: FR-021-AC-7, TC-031. The chain corpus is `CHAIN_LENGTH` (> 128)
/// functions `chain_0 .. chain_139`, each generated body calling the next and
/// the last calling `add_fn`, and its one item generates -- so the AC-7
/// execution case in `exact_function_agreement` really does enter a generated
/// chain deeper than `MAX_CALL_DEPTH`.
#[test]
fn tc_031_ac7_chain_corpus_generates_a_call_chain_deeper_than_max_call_depth() {
    let chain = chain_functions();
    const {
        assert!(
            CHAIN_LENGTH as u64 > quire_contract_runtime::exact::MAX_CALL_DEPTH,
            "the chain must exceed MAX_CALL_DEPTH"
        );
    }
    assert_eq!(chain.len(), CHAIN_LENGTH as usize);
    assert_eq!(chain[0].name, "chain_0");
    assert_eq!(chain[69].name, "chain_69");
    assert_eq!(
        chain[(CHAIN_LENGTH - 1) as usize].name,
        format!("chain_{}", CHAIN_LENGTH - 1)
    );

    let oracles = chain_oracles();
    assert!(matches!(
        disposition_for(&oracles, ITEM_CALL_CHAIN),
        ClaimDisposition::Generated(_)
    ));
    let lib = contents(&oracles, "src/lib.rs");
    for link in 0..CHAIN_LENGTH - 1 {
        let call = format!("frame.call(\"chain_{}\", args)", link + 1);
        assert!(lib.contains(&call), "chain_{link} does not emit {call}");
    }
    assert!(lib.contains("frame.call(\"add_fn\", args)"));
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
    let from_a_fresh_package = main_oracles();

    let mut permuted_functions = functions.clone();
    permuted_functions.reverse();
    let mut permuted_items = items.clone();
    permuted_items.reverse();
    let permuted = generate(&package, &permuted_functions, &permuted_items);

    for other in [&second, &from_a_fresh_package, &permuted] {
        assert_eq!(&first, other);
        for path in [
            "Cargo.toml",
            "src/lib.rs",
            "claim-map.json",
            "location-map.json",
        ] {
            assert_eq!(contents(&first, path), contents(other, path), "{path}");
        }
    }
}

/// Trace: FR-021-AC-14, TC-031. The generated crate declares
/// `publish = false`, names the runtime by branch with the `exact` feature,
/// contains no charge amount and no literal `Outcome`/`Value` constant
/// standing in for a runtime result, and forbids unsafe code.
#[test]
fn tc_031_ac14_manifest_and_source_shape() {
    let package = ext_corpus_package().admit();
    let oracles = generate(&package, &main_functions(), &main_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&runtime_dependency(&["exact"])));
    assert!(!manifest.contains("rev ="));
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
/// tagging" and `spec/oracle/matrix/tests.md`'s FR-021 row: the `path`-non-empty
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
