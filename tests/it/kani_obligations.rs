//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and Contract IR FR-036
//! backend negotiation.
//!
//! The default lane checks negotiation, refusal and harness shape without running Kani. The
//! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend, runs real
//! verified and seeded-failing harnesses one at a time, and writes execution evidence under
//! `CARGO_TARGET_TMPDIR/kani-obligation-evidence`.

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `exact_scalar_generation.rs` each pick independently, on the
// assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../exact_scalar_support/package.rs"]
pub(crate) mod package;
#[path = "kani_scalar_native_digest.rs"]
mod scalar_native_digest;
#[path = "kani_scalar_native_fixture.rs"]
mod scalar_native_fixture;
use crate::common::proof_ceilings::proof_ceilings_with_wall_clock;
use quire_contract_codegen as scalar_fixture_types;
use quire_contract_codegen::{
    generate_routed, Artifact, BackendDescriptor, Candidate, ContractDriverResultSelection,
    ContractRequestPosition, GenerationContexts, KaniGenerationContext, KindOutput, ProviderOrigin,
    RoutedGenerationItem,
};
use scalar_native_digest::independent_digest;
use scalar_native_fixture::{routed_native_scalar_with_artifacts, routed_scalar_with_artifacts};

use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::scratch_crate::{write_manifest, TemporaryDirectory};
use ix_trace_rs::trace;
use package::{
    application, bounded, code_id, corpus_package, golden_items, id, integer_add, key, op,
    reference, Bound, MISSING, MODEL, STATE, T_BOOLEAN, T_INTEGER, UNBOUNDED, V_BOOLEAN,
};
use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
use quire_contract_codegen::{
    classify_kani_run, derive_exact_scalar_items, execute_kani_obligation,
    execute_kani_scalar_native, generate_exact_scalar_oracles, negotiate_kani_obligations,
    routed_contract_lowering_profile, run_terminal_value, ClaimDisposition, ClaimMap,
    ExactScalarClaim, ExactScalarItem, ExactScalarOperation, ExactScalarRefusal,
    InvalidObligationItem, KaniExecutionRefusal, KaniExecutionRequest, KaniFailurePhase,
    KaniInconclusiveReason, KaniInstallation, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, KaniRunOutcome, KaniScalarNativeRequest, KaniScalarObligationHarness,
    KaniSolver, KaniTool, KaniToolError, ObligationDisposition, ObligationItem, ObligationKind,
    ObligationRecord, ObligationSubject, OperationProvenance, UnsupportedObligation,
    UpstreamBlocker, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageReadLimits, CheckedPackageV2, ContractDependencies,
    ContractPackageReadResult, ContractPackageV1, ContractPackageWireV1,
};
use serde_json::json;

/// Budget for the kani lane's real `cargo-kani` runs. Generous because CBMC is memory- and
/// time-heavy on these small obligations; this is a ceiling against a genuine hang, not a
/// performance target.
pub(crate) const REAL_KANI_TIMEOUT: Duration = Duration::from_secs(600);
fn request<'a>(
    items: &'a [ObligationItem<'a>],
    subject_path: &'a str,
) -> KaniObligationRequest<'a> {
    KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(REAL_KANI_TIMEOUT),
        items,
        subject_path,
        unwind: 4,
    }
}

fn emitted_scalar(
    outcome: KaniObligationOutcome,
) -> (Vec<ObligationRecord>, Vec<KaniScalarObligationHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted {
            records,
            scalar_harnesses,
            ..
        } => (records, scalar_harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

fn unsupported(record: &ObligationRecord) -> &UnsupportedObligation {
    match &record.disposition {
        ObligationDisposition::Unsupported { reason } => reason,
        other => panic!("expected unsupported, got {other:?}"),
    }
}

/// The admitted scalar harness source for the cover-last guard.
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let (scalar, claim_map, ids) = scalar_package();
    let node = ids.resolve(&code_id(1001));
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &node,
        claim_index: claim_index(&claim_map, &node),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (_, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    scalar_harnesses
        .into_iter()
        .map(|harness| ("scalar", harness.rust.contents))
        .collect()
}

// ---- V2 fixture --------------------------------------------------------------

/// An integer addition whose only bound admits no value.
const UNSATISFIABLE: u32 = 3001;
/// A frame clause.
const FRAME: u32 = 3002;
/// The object type `FRAME` frames.
const FRAMED_OBJECT: u32 = 3004;

fn scalar_package_with_occurrences(
    expression_occurrences: bool,
) -> (
    CheckedPackageV2,
    ClaimMap<ExactScalarClaim>,
    package::FixtureIds,
) {
    let mut builder = corpus_package();
    // IR-280's FR-322 application-node dependency join means this bound must
    // anchor on a node no other expression's differing bound also reaches
    // (see `PackageBuilder::dedicated_operand`'s doc): the plain shared
    // `V_INTEGER` would union this unsatisfiable
    // `[5, -5]` bound onto every other expression that still references it
    // (`UNBOUNDED` among them), turning its own `RequiresBound` into
    // `AmbiguousBound`/`UnsatisfiableBound` depending on load order.
    let unsatisfiable_anchor = {
        let bound_key = builder.bound(&Bound::Integer(5, -5));
        builder.dedicated_operand("integer", &[bound_key])
    };
    builder
        .application_bounded(
            UNSATISFIABLE,
            "expression",
            "binary",
            &key(T_INTEGER),
            application(
                "binary",
                op("quire.op.integer.add"),
                &key(T_INTEGER),
                // A `reference(V_INTEGER)` pair here would be byte-identical
                // to corpus code 1001's own `integer.add` body (same
                // operator/operation/result_type/arguments), and bounds
                // aren't part of the node-id preimage, so this node and
                // 1001 would collide on digest and IR's `validate_graph`
                // would refuse the whole package as a duplicate node id.
                vec![
                    reference(&unsatisfiable_anchor),
                    package::literal("integer", &UNSATISFIABLE.to_string()),
                ],
            ),
            &[Bound::Integer(5, -5)],
        )
        // Contract IR types a frame by the `model`/`object_type` node it frames.
        .code(
            FRAMED_OBJECT,
            "model",
            "object_type",
            &key(T_BOOLEAN),
            json!({"term": "aggregate", "members": []}),
        )
        .code(
            FRAME,
            "state",
            "frame",
            &key(FRAMED_OBJECT),
            json!({"term": "frame", "modifies": [], "creates": [], "deletes": [], "deletes_scoped": []}),
        );
    if expression_occurrences {
        builder.expression_occurrences();
        let (_, ids) = builder.admit_resolved();
        let unsatisfiable = builder.bound(&Bound::Integer(5, -5));
        builder.narrow(49_301, &key(UNSATISFIABLE), &unsatisfiable);
        builder.expression_occurrences();
    }
    let (package, ids) = builder.admit_resolved();
    let mut items = golden_items()
        .into_iter()
        .map(|mut item| {
            item.node_id = ids.resolve(&item.node_id);
            item
        })
        .collect::<Vec<_>>();
    for code in [UNSATISFIABLE, FRAME] {
        items.push(ExactScalarItem {
            node_id: ids.resolve(&code_id(code)),
            operation: integer_add(),
        });
    }
    let oracles = generate_exact_scalar_oracles(&package, &items).expect("claim map");
    (package, oracles.claim_map, ids)
}

pub(crate) fn scalar_package() -> (
    CheckedPackageV2,
    ClaimMap<ExactScalarClaim>,
    package::FixtureIds,
) {
    scalar_package_with_occurrences(false)
}

/// The same authored fixture with QSL expression occurrences for FR-049 routing.
pub(crate) fn routed_scalar_package() -> (
    CheckedPackageV2,
    ClaimMap<ExactScalarClaim>,
    package::FixtureIds,
) {
    scalar_package_with_occurrences(true)
}

/// Admit one Contract IR package for the exact scalar claim-map roots.
pub(crate) fn admitted_scalar_claims(
    checked: &CheckedPackageV2,
    claims: &ClaimMap<ExactScalarClaim>,
) -> ContractPackageV1 {
    let requested = claims
        .items
        .iter()
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    // Direct FR-015 claim controls need their admitted state/model nodes in the
    // package graph. The built-in routed profile intentionally excludes those
    // families before Kani settlement.
    let mut profile = routed_contract_lowering_profile();
    profile.reference.identity = "quire.codegen.kani-direct-test/v1".into();
    profile.supported_tags.extend([
        quire_contract_model::CheckedNodeTag::Model,
        quire_contract_model::CheckedNodeTag::Relation,
        quire_contract_model::CheckedNodeTag::State,
        quire_contract_model::CheckedNodeTag::Temporal,
        quire_contract_model::CheckedNodeTag::Protocol,
    ]);
    let lowered = checked.lower(&requested, &profile);
    match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => *package,
        other => panic!("Contract IR must admit scalar claims: {other:?}"),
    }
}

fn claim_index(
    claim_map: &ClaimMap<ExactScalarClaim>,
    node_id: &quire_contract_model::CheckedNodeId,
) -> usize {
    claim_map
        .items
        .iter()
        .position(|claim| &claim.node_id == node_id)
        .expect("test claim map contains selected node")
}

fn boolean_implies_scalar_from(
    package: CheckedPackageV2,
    node: CheckedNodeId,
) -> (
    KaniScalarObligationHarness,
    quire_contract_codegen::Artifact,
    String,
) {
    let [item] = derive_exact_scalar_items(&package, std::slice::from_ref(&node))
        .try_into()
        .expect("one derived scalar item");
    let oracles = generate_exact_scalar_oracles(&package, &[item.expect("implies derives")])
        .expect("Boolean oracle generation");
    let admitted = admitted_scalar_claims(&package, &oracles.claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &oracles.claim_map,
        node_id: &node,
        claim_index: claim_index(&oracles.claim_map, &node),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (_, mut harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    let oracle_crate = oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("Boolean oracle source")
        .contents
        .clone();
    let manifest = oracles
        .artifacts
        .into_iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("Boolean oracle manifest");
    let harness = harnesses.remove(0);
    let start = oracle_crate
        .find(&format!("pub fn {}(", harness.identity.oracle_symbol))
        .expect("the oracle crate has the selected function");
    let end = oracle_crate[start..]
        .find("\n}\n")
        .map(|offset| start + offset + 3)
        .expect("the selected function closes");
    (harness, manifest, oracle_crate[start..end].to_owned())
}

fn boolean_implies_scalar() -> (
    KaniScalarObligationHarness,
    quire_contract_codegen::Artifact,
    String,
) {
    const IMPLIES: u32 = 7_702;
    let mut builder = corpus_package();
    builder.application_code_with(
        IMPLIES,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.boolean.implies"),
            &key(T_BOOLEAN),
            vec![reference(&key(V_BOOLEAN)), reference(&key(V_BOOLEAN))],
        ),
        &[key(V_BOOLEAN)],
    );
    let (package, ids) = builder.admit_resolved();
    let node = ids.resolve(&code_id(IMPLIES));
    boolean_implies_scalar_from(package, node)
}

#[trace("FR-015-AC-118", "TC-025")]
#[test]
fn tc_025_boolean_implication_harness_keeps_subject_oracle_and_native_independent() {
    let (harness, _, oracle) = boolean_implies_scalar();
    let source = &harness.rust.contents;
    assert!(
        source.contains(&oracle),
        "the FR-014 oracle function is embedded byte-identical"
    );
    assert!(source.contains("let left: bool = kani::any();"));
    assert!(source.contains("let right: bool = kani::any();"));
    assert!(source.contains("let native = !left || right;"));
    assert!(source.contains(
        "let subject = quire_contract_runtime::operators::implies_short_circuit(left, || right);"
    ));
    assert!(source.contains("assert!(subject == native"));
    assert!(source.contains("Ok(rt::Outcome::Completed(value)) => *value == native"));
    assert!(source.contains("kani::cover!(completed"));
}

#[trace("FR-015-AC-118", "TC-025")]
#[test]
fn tc_025_boolean_implication_with_lazy_right_calls_its_oracle_with_an_outcome() {
    let package = package::derivation_package().admit();
    let node =
        package::fixture_ids_for(&package).resolve(&code_id(package::DERIVE_BOOLEAN_SHORT_CIRCUIT));
    let (harness, _, _) = boolean_implies_scalar_from(package, node);
    assert!(harness.rust.contents.contains(&format!(
        "{}(left, || rt::Outcome::Completed(right), &mut meter)",
        harness.identity.oracle_symbol
    )));
}

#[trace("FR-015-AC-118", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_boolean_implication_subject_mutation_is_falsified_with_false_left() {
    let (harness, manifest, oracle) = boolean_implies_scalar();
    let healthy = run_scalar_under_real_kani("scalar-implies-healthy", &harness, &manifest);
    assert_eq!(healthy.outcome, KaniRunOutcome::Verified, "{healthy:?}");
    assert_eq!(healthy.symbolic_arguments.len(), 2);
    assert!(healthy
        .symbolic_arguments
        .iter()
        .all(|argument| { argument.bounds == quire_contract_codegen::SymbolicBounds::Boolean }));
    let covers = healthy
        .checks
        .iter()
        .filter(|check| check.class == quire_contract_codegen::KaniCheckClass::Cover)
        .collect::<Vec<_>>();
    assert_eq!(covers.len(), 1, "one final cover in the generated source");
    assert_eq!(
        covers[0].status,
        quire_contract_codegen::KaniCheckStatus::Satisfied
    );

    let subject_call = "quire_contract_runtime::operators::implies_short_circuit(left, || right)";
    let parsed = syn::parse_file(&harness.rust.contents).expect("generated Rust parses");
    let call_count = parsed
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(module) => module.content.as_ref(),
            _ => None,
        })
        .flat_map(|(_, items)| items)
        .filter_map(|item| match item {
            syn::Item::Fn(function) => Some(function),
            _ => None,
        })
        .flat_map(|function| &function.block.stmts)
        .filter_map(|statement| match statement {
            syn::Stmt::Local(local) if matches!(&local.pat, syn::Pat::Ident(name) if name.ident == "subject") =>
                local.init.as_ref().map(|init| &*init.expr),
            _ => None,
        })
        .filter(|expression| {
            let syn::Expr::Call(call) = expression else { return false };
            let syn::Expr::Path(path) = &*call.func else { return false };
            path.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>()
                == ["quire_contract_runtime", "operators", "implies_short_circuit"]
                && call.args.len() == 2
        })
        .count();
    assert_eq!(call_count, 1, "exactly one parsed subject expression");
    assert_eq!(harness.rust.contents.matches(subject_call).count(), 1);
    let mut mutated = harness.clone();
    mutated.rust.contents = mutated.rust.contents.replacen(subject_call, "false", 1);
    assert!(
        mutated.rust.contents.contains(&oracle),
        "oracle source is unchanged"
    );
    assert!(mutated
        .rust
        .contents
        .contains("let native = !left || right;"));
    let evidence = run_scalar_under_real_kani("scalar-implies-mutated", &mutated, &manifest);
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("subject-only mutation must falsify: {evidence:?}");
    };
    assert!(counterexample.contains("kani::concrete_playback_run"));
    assert!(
        counterexample.contains("the compiled implication subject must equal native implication")
    );
    use syn::parse::Parser;
    let playback = syn::parse_file(counterexample).expect("Kani playback is Rust source");
    let first_input = playback
        .items
        .iter()
        .find_map(|item| {
            let syn::Item::Fn(function) = item else {
                return None;
            };
            function.block.stmts.iter().find_map(|statement| {
                let syn::Stmt::Local(local) = statement else {
                    return None;
                };
                let syn::Pat::Type(typed) = &local.pat else {
                    return None;
                };
                let syn::Pat::Ident(name) = &*typed.pat else {
                    return None;
                };
                if name.ident != "concrete_vals" {
                    return None;
                }
                let syn::Expr::Macro(values) = &*local.init.as_ref()?.expr else {
                    return None;
                };
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                    .parse2(values.mac.tokens.clone())
                    .ok()?
                    .first()
                    .cloned()
            })
        })
        .expect("playback contains symbolic inputs");
    let syn::Expr::Macro(left_bytes) = first_input else {
        panic!("first playback input is a byte vector: {counterexample}");
    };
    let left_bytes = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
        .parse2(left_bytes.mac.tokens)
        .expect("first input bytes parse");
    let [syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(byte),
        ..
    })] = left_bytes.iter().collect::<Vec<_>>().as_slice()
    else {
        panic!("one Boolean byte for left: {counterexample}");
    };
    assert_eq!(
        byte.base10_parse::<u8>().unwrap(),
        0,
        "left is false in playback"
    );
}

pub(crate) fn boolean_operation_package() -> (CheckedPackageV2, CheckedNodeId) {
    const BOOLEAN_AND: u32 = 7_701;
    let mut builder = corpus_package();
    builder.application_code_with(
        BOOLEAN_AND,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.boolean.and"),
            &key(T_BOOLEAN),
            vec![reference(&key(V_BOOLEAN)), reference(&key(V_BOOLEAN))],
        ),
        &[key(V_BOOLEAN)],
    );
    let (package, ids) = builder.admit_resolved();
    (package, ids.resolve(&code_id(BOOLEAN_AND)))
}

fn scalar_records(
    package: &CheckedPackageV2,
    claim_map: &ClaimMap<ExactScalarClaim>,
    fixture_ids: &package::FixtureIds,
    codes: &[u32],
) -> Vec<ObligationRecord> {
    let ids = codes
        .iter()
        .map(|code| fixture_ids.resolve(&code_id(*code)))
        .collect::<Vec<_>>();
    let admitted = admitted_scalar_claims(package, claim_map);
    let items = ids
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map,
            node_id,
            claim_index: claim_index(claim_map, node_id),
            result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    negotiate_kani_obligations(&request(&items, "crate::subject"))
        .unwrap()
        .records()
        .to_vec()
}

// ---- default lane ------------------------------------------------------------

/// Symbolic ranges are the IR's inclusive domains, and every flag is part of the harness
/// identity.
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-1, TC-045
#[trace("FR-015-AC-2", "FR-015-AC-9", "FR-015-AC-10", "FR-015-AC-11", "TC-025")]
#[test]
fn tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity() {
    // A V2 claim's domain is read from its IR bound, inclusive at both ends. Node 1001 is
    // `quire.op.integer.add`, IR-confirmed and renderable, so it reaches a real Kani harness --
    // not a typed refusal. Both its operands are references to the corpus's `value` node whose
    // body is the literal `3`, so each is a constant and ranged at its own value (IR-302); the
    // per-operand bounded ranges are TC-033's.
    let (scalar, claim_map, ids) = scalar_package();
    let node_1001 = ids.resolve(&code_id(1001));
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &node_1001,
        claim_index: claim_index(&claim_map, &node_1001),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (records, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert_eq!(scalar_harnesses.len(), 1);
    let identity = &scalar_harnesses[0].identity;
    assert_eq!(identity.operation_identity, "quire.op.integer.add");
    assert_eq!(identity.solver, KaniSolver::Cadical);
    assert_eq!(identity.unwind, 4);
    assert_eq!(
        identity.ceilings,
        request(&items, "crate::subject").ceilings
    );
    assert!(identity.options.iter().any(|option| option == "--exact"));
    assert!(identity.options.iter().any(|option| option == "--unwind"));
    assert!(!scalar_harnesses[0].rust.contents.contains("kani::unwind"));
    let repeated =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap()).1;
    assert_eq!(repeated, scalar_harnesses);
    assert_eq!(
        identity
            .arguments
            .iter()
            .map(|argument| (argument.minimum, argument.maximum))
            .collect::<Vec<_>>(),
        [(3, 3), (3, 3)]
    );
}

/// The scalar harness states a property the oracle does not itself guarantee: the outcome agrees
/// with the clause's operation evaluated natively in `i128` over the same operands, completing with
/// exactly that value when it lies in the checked domain and refusing when it does not. Asserting
/// only that a completed value lies in the domain restated the bound check the oracle makes before
/// returning `Completed`, so the proof held whatever the arithmetic did. `outcome.is_ok()` is still
/// not matched: it is true for `Ok(Refused(_))` too. This is a source-inspection check of the
/// rendered text; the real-Kani tests below discharge it and falsify a mutated oracle.
#[trace(
    "FR-015-AC-37",
    "FR-015-AC-7",
    "TC-025",
    "TC-001",
    "NFR-002-AC-1",
    "NFR-002-AC-2"
)]
#[test]
fn tc_025_scalar_harness_asserts_the_native_arithmetic_relation() {
    let (scalar, claim_map, ids) = scalar_package();
    let node_1001 = ids.resolve(&code_id(1001));
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &node_1001,
        claim_index: claim_index(&claim_map, &node_1001),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (_, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    let source = &scalar_harnesses[0].rust.contents;
    let node_path = node_1001
        .path
        .iter()
        .map(|segment| match segment {
            quire_contract_model::CheckedNodePathSegment::Name(name) => {
                quire_exact::PathSegment::Text(name.to_string())
            }
            quire_contract_model::CheckedNodePathSegment::Node(_) => {
                panic!("the scalar fixture has a direct authored path")
            }
        })
        .collect();
    let node_key =
        quire_exact::NodeKey::new(node_1001.package.to_string(), node_path, node_1001.ordinal)
            .expect("admitted structural node key");
    crate::generated_headers::assert_header(
        &scalar_harnesses[0].rust.path,
        source,
        &[&format!(
            "// Obligation: exact-scalar {} `quire.op.integer.add`",
            node_key
        )],
    );
    assert!(
        source.contains("let exact: i128 = i128::from(left_native) + i128::from(right_native);"),
        "the expected result must be the native i128 evaluation of the clause's operation over \
         the raw operands, not a value read back from the oracle: {source}"
    );
    assert!(
        source.contains(
            "let admitted = exact >= i128::from(-1000_i64) && exact <= i128::from(1000_i64);"
        ),
        "the checked domain must come from the result bound's own literal bounds: {source}"
    );
    assert!(
        source.contains(
            "Ok(rt::Outcome::Completed(value)) => admitted && *value == rt::Integer::from(exact),"
        ),
        "a completed value must equal the native result, not merely lie in the domain: {source}"
    );
    assert!(
        source.contains("Ok(rt::Outcome::Refused(_)) => !admitted,"),
        "a refusal is correct exactly when the native result leaves the domain: {source}"
    );
    assert!(
        source.contains("_ => false,"),
        "every other outcome (Undefined, Incomplete, Err) must fail the proof, so an oracle that \
         stops on some inputs cannot verify: {source}"
    );
    assert!(
        !source.contains("domain.contains("),
        "regression to the tautology that re-ran the oracle's own bound check: {source}"
    );
    assert!(
        source.contains("assert!(sound,"),
        "soundness must be an assertion Kani can falsify, not only a cover: {source}"
    );
    assert!(
        source
            .rfind("assert!(")
            .zip(source.find("kani::cover!("))
            .is_some_and(|(assertion, cover)| assertion < cover),
        "the cover must follow the last assertion (FR-015-AC-7, IR-451): {source}"
    );
    assert!(
        !source.contains("assert!(completed,"),
        "regression to the old, false, unconditional-totality assertion: {source}"
    );
    assert!(
        source.contains("kani::cover!(completed,"),
        "the cover must gate on outcome reachability, as the FR-015-AC-7 non-vacuity guard, \
         never on a constant: {source}"
    );
    assert!(
        !source.contains("kani::cover!(true, "),
        "the cover must not be trivially reachable: {source}"
    );
    assert!(
        !source.contains("kani::cover!(outcome.is_ok(), "),
        "regression to the vacuous Result::is_ok() check: {source}"
    );
    // The assumption is scoped by the same ranges
    // `tc_025_symbolic_bounds_equal_ir_domains_and_every_pin_is_identity` checks via
    // `identity.arguments` above (each operand a literal `3`, pinned at its value): this is the
    // corresponding source-level check.
    assert!(source.contains("kani::assume(left >= 3_i64 && left <= 3_i64);"));
    assert!(source.contains("kani::assume(right >= 3_i64 && right <= 3_i64);"));
}

/// Each of the four rendered integer operations states its own native `i128` expression, written
/// out here independently of the generator. A slip in one (operands swapped for subtraction,
/// multiplication rendered as addition, a dropped negation) would make the first real Kani run of
/// that operation falsify a correct oracle, and the real-Kani lane runs only addition.
#[trace("FR-015-AC-37", "TC-025")]
#[test]
fn tc_025_every_rendered_operation_states_its_own_native_relation() {
    const EXACT: [(&str, &str); 4] = [
        (
            "quire.op.integer.add",
            "let exact: i128 = i128::from(left_native) + i128::from(right_native);",
        ),
        (
            "quire.op.integer.sub",
            "let exact: i128 = i128::from(left_native) - i128::from(right_native);",
        ),
        (
            "quire.op.integer.mul",
            "let exact: i128 = i128::from(left_native) * i128::from(right_native);",
        ),
        (
            "quire.op.integer.negate",
            "let exact: i128 = -i128::from(operand_native);",
        ),
    ];
    let (scalar, claim_map, _) = scalar_package();
    let generated = claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .collect::<Vec<_>>();
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = generated
        .iter()
        .map(|claim| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &claim.node_id,
            claim_index: claim_index(&claim_map, &claim.node_id),
            result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let (_, harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    for (identity, line) in EXACT {
        let rendered = harnesses
            .iter()
            .filter(|harness| harness.identity.operation_identity == identity)
            .collect::<Vec<_>>();
        assert!(!rendered.is_empty(), "{identity}: the corpus renders it");
        for harness in rendered {
            let source = &harness.rust.contents;
            assert_eq!(
                source.matches("let exact: i128 = ").count(),
                1,
                "{identity}: {source}"
            );
            assert!(
                source.contains(line),
                "{identity}: expected `{line}` in {source}"
            );
        }
    }
}

/// Unbounded, non-finite, model-dependent, frame and definedness-bearing items are typed
/// refusals with no harness. These generation-time classifications (`ObligationDisposition`,
/// `UnsupportedObligation`) are never available to `execute_kani_obligation`, which requires a
/// a generated harness: an item this test refuses never produces one, so there is no way to
/// report the refusal as an execution outcome (`KaniRunOutcome`) instead of what it is.
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-2, TC-045
#[trace("FR-015-AC-3", "FR-017-CON-2", "TC-025", "TC-027")]
#[test]
fn tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses() {
    let (scalar, claim_map, ids) = scalar_package();
    let records = scalar_records(&scalar, &claim_map, &ids, &[UNBOUNDED, MODEL, STATE, FRAME]);
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    for (record, code) in records.iter().zip([UNBOUNDED, MODEL, STATE, FRAME]) {
        let mut singleton = scalar_records(&scalar, &claim_map, &ids, &[code]).remove(0);
        singleton.request_index = record.request_index;
        assert_eq!(
            *record, singleton,
            "batch classification must preserve each item"
        );
        let node_id = ids.resolve(&code_id(code));
        let expected = admitted
            .wire()
            .source_map
            .nodes
            .iter()
            .filter(|entry| entry.node_id == node_id)
            .flat_map(|entry| entry.occurrences.iter().cloned())
            .collect::<Vec<_>>();
        assert!(matches!(
            &record.subject,
            ObligationSubject::CheckedNode { node_id: actual_id, source_map }
                if actual_id == &node_id && source_map == &expected
        ));
    }
    assert_eq!(
        records[0].disposition,
        ObligationDisposition::RequiresBound {
            unbounded_type: ids.resolve(&code_id(T_INTEGER))
        }
    );
    assert_eq!(
        unsupported(&records[1]),
        &UnsupportedObligation::BlockedOnUpstream {
            node_id: ids.resolve(&code_id(MODEL)),
            node_tag: "model",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }
    );
    assert_eq!(
        unsupported(&records[2]),
        &UnsupportedObligation::NoFiniteEncoding {
            node_id: ids.resolve(&code_id(STATE)),
            node_tag: "state",
        }
    );
    assert_eq!(records[3].kind, Some(ObligationKind::Frame));
    let model = ids.resolve(&code_id(FRAMED_OBJECT));
    let frame = ids.resolve(&code_id(FRAME));
    // The frame is typed by FRAMED_OBJECT, and IR names the first unsupported member in
    // admitted node-id order. Both possible first members retain their own typed refusal.
    let first_refusal = if model < frame {
        UnsupportedObligation::BlockedOnUpstream {
            node_id: model,
            node_tag: "model",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }
    } else {
        UnsupportedObligation::NoFiniteEncoding {
            node_id: frame.clone(),
            node_tag: "state",
        }
    };
    assert_eq!(unsupported(&records[3]), &first_refusal);
    assert!(matches!(
        &records[3].subject,
        ObligationSubject::CheckedNode { node_id, source_map } if node_id == &ids.resolve(&code_id(FRAME)) && !source_map.is_empty()
    ));
}

/// A source-authored node can have several distinct occurrences. The indexed lookup must keep
/// every occurrence in its admitted wire order, including when neighboring items share a graph.
#[trace("FR-015-AC-3", "TC-025")]
#[test]
fn tc_025_scalar_source_lookup_preserves_multiple_occurrences_per_node() {
    let (builder, _, _) = package::source_backed_byte_chain(2);
    let (checked, ids) = builder.admit_resolved();
    let nodes = (0..2)
        .map(|offset| ids.resolve(&code_id(package::BYTE_CHAIN_BASE + offset)))
        .collect::<Vec<_>>();
    let claims = nodes
        .iter()
        .map(|node_id| ExactScalarItem {
            node_id: node_id.clone(),
            operation: integer_add(),
        })
        .collect::<Vec<_>>();
    let claim_map = generate_exact_scalar_oracles(&checked, &claims)
        .expect("source-backed scalar claims")
        .claim_map;
    let admitted = admitted_scalar_claims(&checked, &claim_map);
    let items = nodes
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id,
            claim_index: claim_index(&claim_map, node_id),
            result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let records = negotiate_kani_obligations(&request(&items, "crate::subject"))
        .unwrap()
        .records()
        .to_vec();
    assert_eq!(records.len(), nodes.len());
    for (record, node_id) in records.iter().zip(&nodes) {
        let expected = admitted
            .wire()
            .source_map
            .nodes
            .iter()
            .filter(|entry| &entry.node_id == node_id)
            .flat_map(|entry| entry.occurrences.iter().cloned())
            .collect::<Vec<_>>();
        assert!(
            expected.len() >= 2,
            "fixture must distinguish occurrence order"
        );
        assert_ne!(expected[0], expected[1], "occurrences must be distinct");
        assert!(matches!(
            &record.subject,
            ObligationSubject::CheckedNode { node_id: actual_id, source_map }
                if actual_id == node_id && source_map == &expected
        ));
    }
}

/// A quantity conversion node declared by the caller reaches the Kani obligation engine only as
/// Exact conversion claims remain unrendered for Kani and emit no harness.
#[trace("FR-014-AC-2", "FR-014-AC-51", "FR-015-AC-15", "TC-025")]
#[test]
fn tc_025_a_quantity_conversion_claim_is_unrendered_with_no_harness() {
    let (scalar, _, ids) = scalar_package();
    let declared = package::QUANTITY_CONVERSIONS
        .iter()
        .map(|code| ExactScalarItem {
            node_id: ids.resolve(&code_id(*code)),
            operation: ExactScalarOperation::QuantityConversion,
        })
        .collect::<Vec<_>>();
    let claim_map = generate_exact_scalar_oracles(&scalar, &declared)
        .expect("claim map")
        .claim_map;
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let node_ids = package::QUANTITY_CONVERSIONS
        .iter()
        .map(|code| ids.resolve(&code_id(*code)))
        .collect::<Vec<_>>();
    let items = node_ids
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id,
            claim_index: claim_index(&claim_map, node_id),
            result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let KaniObligationOutcome::Emitted {
        records,
        scalar_harnesses,
        state_frame_harnesses,
        collection_harnesses,
        v2_clause_harnesses,
    } = negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap()
    else {
        panic!("a refused claim settles per item");
    };
    // The whole artifact set is empty: no harness of any family is emitted for the obligations.
    assert!(scalar_harnesses.is_empty(), "scalar harnesses");
    assert!(state_frame_harnesses.is_empty(), "state-frame harnesses");
    assert!(collection_harnesses.is_empty(), "collection harnesses");
    assert!(v2_clause_harnesses.is_empty(), "V2 clause harnesses");
    // Control: the same negotiation over a supported claim does populate `scalar_harnesses`, so the
    // empty vectors above are a measured absence, not a vacuous one.
    let (control_package, control_claims, control_ids) = scalar_package();
    let control_admitted = admitted_scalar_claims(&control_package, &control_claims);
    let control_node = control_ids.resolve(&code_id(1001));
    let control_item = ObligationItem::ScalarClaim {
        package: &control_admitted,
        claim_map: &control_claims,
        node_id: &control_node,
        claim_index: claim_index(&control_claims, &control_node),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    };
    let (_, control_harnesses) = emitted_scalar(
        negotiate_kani_obligations(&request(
            std::slice::from_ref(&control_item),
            "crate::subject",
        ))
        .unwrap(),
    );
    assert_eq!(control_harnesses.len(), 1);
    assert_eq!(records.len(), package::QUANTITY_CONVERSIONS.len());
    for record in &records {
        assert!(
            matches!(unsupported(record), UnsupportedObligation::OperationNotRendered {
            operation_identity, ..
        } if operation_identity == "quire.op.quantity.convert")
        );
    }
}

/// A rendered `Generated` claim, and the [`ClaimMap`] it came from, reused by both
/// `UnknownNodeKind` tests below to hand-assemble a claim map entry `generate_exact_scalar_oracles`
/// would never itself produce.
const RENDERED_OPERATIONS: [&str; 4] = [
    "quire.op.integer.add",
    "quire.op.integer.sub",
    "quire.op.integer.mul",
    "quire.op.integer.negate",
];

/// IR-81: a V2 scalar-claim item's graph node that is present, but is neither the one recognized
/// `state`/`frame` role pair nor `expression`-tagged (the only family
/// `generate_exact_scalar_oracles` ever lowers to a `Generated` claim), must not be silently
/// accounted with a null `kind` and a `Supported` disposition if it ever reaches one. The real
/// generator already refuses the other `state`-tagged nodes before that point -- `STATE` lands on
/// `NoFiniteEncoding`, while `FRAME` reaches its framed model's upstream blocker -- so this
/// drives the gap directly: the admitted `state/snapshot` node's claim is replaced with a
/// `Generated` claim, a shape the scalar generator would never itself produce for a
/// non-`expression` node. This simulates a future widening of that gate or a caller-assembled
/// claim map without inventing a malformed state body.
#[trace("FR-015-AC-14", "TC-025")]
#[test]
fn tc_025_a_present_node_with_an_unrecognized_kind_is_refused_rather_than_silently_supported() {
    // The corpus's admitted snapshot is a real non-expression kind. A
    // transition would require its own closed body, unrelated to this guard.
    let (package, ids) = corpus_package().admit_resolved();
    let node_id = ids.resolve(&code_id(STATE));

    let oracles = generate_exact_scalar_oracles(&package, &package::golden_items_for(&package))
        .expect("claim map");
    let (operation, generated) = oracles
        .claim_map
        .items
        .iter()
        .find_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated)
                if RENDERED_OPERATIONS.contains(&claim.operation.identity.as_str()) =>
            {
                Some((claim.operation.clone(), generated.clone()))
            }
            _ => None,
        })
        .expect("the corpus renders at least one integer-arithmetic claim");
    let mut claim_map = oracles.claim_map;
    let existing = claim_map
        .items
        .iter_mut()
        .find(|claim| claim.node_id == node_id)
        .expect("the admitted snapshot has a scalar claim");
    *existing = ExactScalarClaim {
        node_id: node_id.clone(),
        operation,
        result: ClaimDisposition::Generated(generated),
    };

    let admitted = admitted_scalar_claims(&package, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &node_id,
        claim_index: claim_index(&claim_map, &node_id),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let outcome = negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap();
    let KaniObligationOutcome::Emitted {
        records,
        scalar_harnesses,
        ..
    } = outcome
    else {
        panic!("unexpected rejection");
    };
    assert!(
        scalar_harnesses.is_empty(),
        "an unrecognized node kind must emit no harness"
    );
    assert_eq!(records[0].kind, None);
    assert_eq!(
        unsupported(&records[0]),
        &UnsupportedObligation::UnknownNodeKind {
            node_id: node_id.clone(),
            node_tag: "state".to_owned(),
            semantic_form: "snapshot".to_owned(),
        }
    );
}

/// A scalar refusal of the byte ceiling or of an unrecognised lowering limit kind is reported as
/// `OracleRefused` carrying that refusal unchanged, field for field, as a work-ceiling refusal
/// is: never another `UnsupportedObligation` variant and never `LoweringWorkExhausted`. Each
/// refusal is placed in a claim map by hand, as `ClaimMap` and `ExactScalarClaim` are fully
/// `pub`; the generator's own byte-ceiling refusal is FR-014-AC-40's.
#[trace("FR-015-AC-50", "TC-025")]
#[test]
fn tc_025_a_byte_ceiling_and_an_unrecognised_lowering_refusal_are_oracle_refused_unchanged() {
    let (package, claim_map, ids) = scalar_package();
    for refusal in [
        ExactScalarRefusal::LoweringByteLimitExceeded {
            limit: 1_000,
            consumed: 1_001,
        },
        ExactScalarRefusal::LoweringLimitUnrecognised {
            limit_kind: "nodes",
            limit: 128,
            consumed: 129,
        },
    ] {
        let mut hand_built = claim_map.clone();
        let claim = hand_built
            .items
            .iter_mut()
            .find(|claim| claim.node_id == ids.resolve(&code_id(UNBOUNDED)))
            .expect("the corpus claims the unbounded node");
        claim.result = ClaimDisposition::Refused {
            refusal: refusal.clone(),
        };
        let records = scalar_records(&package, &hand_built, &ids, &[UNBOUNDED]);
        assert_eq!(
            unsupported(&records[0]),
            &UnsupportedObligation::OracleRefused { refusal }
        );
    }
}

/// IR-81 follow-up (PR review F2): the module doc's own claim that a node absent from the graph
/// "never reaches `Outcome::LoweredScalar` at all, since `claim_map.items` cannot name one the
/// graph does not also carry" is only true for a claim map `generate_exact_scalar_oracles` itself
/// produced. `ClaimMap`/`ExactScalarClaim` are fully `pub`, so a hand-assembled claim
/// map can name a `node_id` that has an entry in `claim_map.items` (so it does not hit the
/// pre-existing "no entry in the claim map" `UnknownNode` ground at all) but no entry in
/// `package.graph()` at all.
///
/// `code_id(MISSING)` -- this fixture module's own dedicated guaranteed-absent-from-the-graph
/// sentinel -- is deliberately *not* reused here: `golden_items()` already chains
/// `refused_items()`, which names `MISSING` itself as a `Refused { refusal: InvalidInput }` claim
/// (the pre-existing "node not in the admitted graph" ground fired by
/// `generate_exact_scalar_oracles` itself), so `code_id(MISSING)` already has a real entry in
/// `oracles.claim_map.items` before this test would append a second one, and `Iterator::find`
/// would silently match that pre-existing entry first -- exercising the pre-existing ground rather
/// than the new one. `NODE_ABSENT_FROM_GRAPH` is instead a code no `PackageBuilder` constructor
/// or `refused_items()` registers at all, built directly with `id(&key(..))` rather than
/// `code_id` (which panics for an unregistered, non-`MISSING` code), so its only entry in
/// `claim_map.items` is the one this test appends: a real rendered arithmetic claim's operation
/// and bounds, so the claim's own bound lookups still resolve and it would otherwise reach
/// `Outcome::LoweredScalar` -- proving this is the "node absent from the graph" ground, not the
/// "no finite encoding" or "unrecognized tag/form" one.
#[trace("FR-015-AC-14", "TC-025")]
#[test]
fn tc_025_a_claim_naming_a_node_absent_from_the_graph_is_refused_not_silently_supported() {
    const NODE_ABSENT_FROM_GRAPH: u32 = 8888;
    let package = corpus_package().admit();
    let missing_node_id = id(&key(NODE_ABSENT_FROM_GRAPH));

    let oracles = generate_exact_scalar_oracles(&package, &package::golden_items_for(&package))
        .expect("claim map");
    let (operation, generated) = oracles
        .claim_map
        .items
        .iter()
        .find_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated)
                if RENDERED_OPERATIONS.contains(&claim.operation.identity.as_str()) =>
            {
                Some((claim.operation.clone(), generated.clone()))
            }
            _ => None,
        })
        .expect("the corpus renders at least one integer-arithmetic claim");
    let mut claim_map = oracles.claim_map;
    claim_map.items.push(ExactScalarClaim {
        node_id: missing_node_id.clone(),
        operation,
        result: ClaimDisposition::Generated(generated),
    });

    let admitted = admitted_scalar_claims(&package, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &missing_node_id,
        claim_index: claim_index(&claim_map, &missing_node_id),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let outcome = negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap();
    let KaniObligationOutcome::Rejected { records } = &outcome else {
        panic!(
            "a claim naming a node absent from the graph must not be silently supported: \
             {outcome:?}"
        );
    };
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, None);
    assert_eq!(
        records[0].disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::UnknownNode
        }
    );
}

/// Every V2 scalar claim FR-014 generates has an IR-confirmed operation. The four
/// `IntegerArithmetic` identities this generator knows how to render (`quire.op.integer.{add,sub,
/// mul,negate}`) each reach a real Kani harness; every other confirmed family is refused as
/// `OperationNotRendered` -- known, honestly-named debt, never the old `CallerDeclaredOperation`
/// lie (that provenance is unreachable here: every one of these claims' nodes was successfully
/// lowered and checked, from an admitted package whose IR already confirmed the identity).
#[trace("FR-015-AC-6", "TC-025")]
#[test]
fn tc_025_every_confirmed_operation_is_rendered_or_honestly_refused() {
    const RENDERED: [&str; 4] = [
        "quire.op.integer.add",
        "quire.op.integer.sub",
        "quire.op.integer.mul",
        "quire.op.integer.negate",
    ];
    let (scalar, claim_map, _) = scalar_package();
    let generated = claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .cloned()
        .collect::<Vec<_>>();
    assert!(generated.len() > 40, "the corpus generates every family");
    assert!(
        generated
            .iter()
            .all(|claim| claim.operation.provenance == OperationProvenance::IrConfirmed),
        "every generated V2 claim's operation is IR-confirmed"
    );
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = generated
        .iter()
        .map(|claim| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &claim.node_id,
            claim_index: claim_index(&claim_map, &claim.node_id),
            result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let (records, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    assert_eq!(records.len(), generated.len());
    let expected_rendered = generated
        .iter()
        .filter(|claim| RENDERED.contains(&claim.operation.identity.as_str()))
        .count();
    assert!(
        expected_rendered > 0,
        "the corpus exercises integer arithmetic"
    );
    assert_eq!(scalar_harnesses.len(), expected_rendered);
    for (record, claim) in records.iter().zip(&generated) {
        if RENDERED.contains(&claim.operation.identity.as_str()) {
            assert!(
                matches!(record.disposition, ObligationDisposition::Supported { .. }),
                "{}: {:?}",
                claim.operation.identity,
                record.disposition
            );
        } else {
            assert_eq!(
                unsupported(record),
                &UnsupportedObligation::OperationNotRendered {
                    operation_identity: claim.operation.identity.clone(),
                    derived_domains: match unsupported(record) {
                        UnsupportedObligation::OperationNotRendered {
                            derived_domains, ..
                        } => derived_domains.clone(),
                        other => panic!("{other:?}"),
                    },
                }
            );
        }
    }
}

/// An admitted division claim remains distinct from operand-lowering refusals:
/// its confirmed operation has no scalar harness renderer.
#[trace("FR-015-AC-109", "TC-023")]
#[test]
fn admitted_integer_division_is_unrendered_and_emits_no_harness() {
    let (scalar, claim_map, _) = scalar_package();
    let division = claim_map
        .items
        .iter()
        .find(|claim| {
            claim.operation.identity == "quire.op.integer.div"
                && matches!(claim.result, ClaimDisposition::Generated(_))
        })
        .expect("the admitted corpus generates an integer division oracle");
    assert_eq!(
        division.operation.provenance,
        OperationProvenance::IrConfirmed
    );
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &claim_map,
        node_id: &division.node_id,
        claim_index: claim_index(&claim_map, &division.node_id),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (records, harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    assert_eq!(records.len(), 1);
    assert!(matches!(unsupported(&records[0]),
        UnsupportedObligation::OperationNotRendered { operation_identity, .. }
            if operation_identity == "quire.op.integer.div"
    ));
    assert!(harnesses.is_empty(), "unrendered division emits no harness");
}

/// FR-015-AC-6 ("an obligation over a `caller_declared` oracle is refused with a typed reason and
/// no harness") stayed true in code once IR-217 widened `IrConfirmed`, but the test above that is
/// still traced to it was repurposed to check the render-or-honestly-refuse split over an
/// all-confirmed corpus, so no test anywhere produced `UnsupportedObligation::CallerDeclaredOperation`
/// any more. This restores a real case: node 1004 is `quire.op.integer.mul`; naming `Add` against
/// it, with the identical `[-1000,1000]` domain, passes every `check_item` shape and bound check
/// (see `exact_scalar_generation.rs`'s own
/// `tc_024_operator_confusion_within_one_shape_is_not_silently_confirmed`) but is not confirmed,
/// so it must reach FR-015 negotiation as a typed refusal with no harness, never a proof over a
/// mismatched operator.
#[trace("FR-015-AC-6", "FR-038-AC-3", "TC-025")]
#[test]
fn tc_025_a_caller_declared_operation_is_refused_with_no_harness() {
    let (package, ids) = corpus_package().admit_resolved();
    let mismatched = ids.resolve(&code_id(1004));
    let oracles = generate_exact_scalar_oracles(
        &package,
        &[ExactScalarItem {
            node_id: mismatched.clone(),
            operation: ExactScalarOperation::IntegerArithmetic {
                operator: IntegerOperator::Add,
                domain: bounded(-1000, 1000),
            },
        }],
    )
    .expect("claim map");
    let claim = &oracles.claim_map.items[0];
    assert_eq!(
        claim.operation.provenance,
        OperationProvenance::CallerDeclared {
            blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
        }
    );
    let admitted = admitted_scalar_claims(&package, &oracles.claim_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &oracles.claim_map,
        node_id: &mismatched,
        claim_index: claim_index(&oracles.claim_map, &mismatched),
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let (records, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    assert!(scalar_harnesses.is_empty());
    match unsupported(&records[0]) {
        UnsupportedObligation::CallerDeclaredOperation {
            operation_identity,
            blocked_on,
            ..
        } => {
            assert_eq!(operation_identity, "integer.add domain=[-1000,1000]");
            assert_eq!(*blocked_on, UpstreamBlocker::OperationIdentityNotConsumed);
        }
        other => panic!("expected CallerDeclaredOperation, got {other:?}"),
    }
}

/// One invalid item rejects the request after every item is accounted, and exposes no bytes.
/// The whole-request refusals below (`KaniObligationError`) are generation-time classifications
/// too: a rejected `negotiate_kani_obligations` call returns no harness at all,
/// so `execute_kani_obligation` has nothing to run and nothing to misreport as an execution
/// outcome in place of the refusal it actually is.
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-4, TC-045
#[trace("FR-015-AC-12", "FR-017-CON-2", "TC-025", "TC-027")]
#[test]
fn tc_025_every_item_is_accounted_before_any_harness_is_exposed() {
    let (scalar, claim_map, ids) = scalar_package();
    let admitted = admitted_scalar_claims(&scalar, &claim_map);
    let supported = ids.resolve(&code_id(1001));
    let missing = code_id(MISSING);
    let unbounded = ids.resolve(&code_id(UNBOUNDED));
    let items = [
        ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &supported,
            claim_index: claim_index(&claim_map, &supported),
            result_selection: &ContractDriverResultSelection::NotApplicable,
        },
        ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &supported,
            claim_index: claim_index(&claim_map, &supported),
            result_selection: &ContractDriverResultSelection::NotApplicable,
        },
        ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &missing,
            claim_index: claim_map.items.len(),
            result_selection: &ContractDriverResultSelection::NotApplicable,
        },
        ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map: &claim_map,
            node_id: &unbounded,
            claim_index: claim_index(&claim_map, &unbounded),
            result_selection: &ContractDriverResultSelection::NotApplicable,
        },
    ];
    let outcome = negotiate_kani_obligations(&request(&items, "crate::withdraw")).unwrap();
    let KaniObligationOutcome::Rejected { records } = &outcome else {
        panic!("an invalid item must reject the request: {outcome:?}");
    };
    assert_eq!(records.len(), items.len());
    assert_eq!(
        records
            .iter()
            .map(|record| record.request_index)
            .collect::<Vec<_>>(),
        (0..items.len()).collect::<Vec<_>>()
    );
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert_eq!(
        records[1].disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::DuplicateItem { first_index: 0 }
        }
    );
    assert_eq!(
        records[2].disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::UnknownNode
        }
    );
    assert!(matches!(
        records[3].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));

    // A claim map from another package is a mismatch, not a lookup.
    let (_, foreign_map) = {
        let mut builder = corpus_package();
        builder.code(
            4001,
            "value",
            "literal",
            &key(T_INTEGER),
            package::literal("integer", "4"),
        );
        let foreign = builder.admit();
        let map = generate_exact_scalar_oracles(&foreign, &golden_items())
            .unwrap()
            .claim_map;
        (foreign, map)
    };
    let node = code_id(1001);
    let admitted = admitted_scalar_claims(&scalar, &foreign_map);
    let items = [ObligationItem::ScalarClaim {
        package: &admitted,
        claim_map: &foreign_map,
        node_id: &node,
        // PackageMismatch takes precedence over claim selection.
        claim_index: 0,
        result_selection: &quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
    }];
    let outcome = negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap();
    assert_eq!(
        outcome.records()[0].disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::PackageMismatch
        }
    );

    for (items, subject, unwind, expected) in [
        (
            &[][..],
            "crate::withdraw",
            4,
            KaniObligationError::EmptyRequest,
        ),
        (
            &items[..],
            "not a path",
            4,
            KaniObligationError::InvalidSubjectPath,
        ),
        (
            &items[..],
            "crate::withdraw",
            0,
            KaniObligationError::InvalidUnwind { unwind: 0 },
        ),
        (
            &items[..],
            "crate::withdraw",
            MAX_OBLIGATION_UNWIND + 1,
            KaniObligationError::InvalidUnwind {
                unwind: MAX_OBLIGATION_UNWIND + 1,
            },
        ),
    ] {
        let mut value = request(items, subject);
        value.unwind = unwind;
        assert_eq!(negotiate_kani_obligations(&value), Err(expected));
    }

    // The item ceiling is a request-level refusal too: nothing is accounted.
    let over_limit = vec![items[0]; MAX_OBLIGATION_ITEMS + 1];
    assert_eq!(
        negotiate_kani_obligations(&request(&over_limit, "crate::withdraw")),
        Err(KaniObligationError::TooManyItems {
            count: MAX_OBLIGATION_ITEMS + 1
        })
    );
}

/// A missing launcher is a typed refusal naming it, and nothing runs.
#[trace("FR-017-AC-2", "TC-027")]
#[test]
fn tc_027_a_missing_launcher_is_refused_before_anything_runs() {
    let cancel = quire_exact::Cancel::new();
    let (harness, manifest) = routed_scalar_increment();
    let scratch = write_scalar_crate("missing-launcher", &manifest, &harness.rust.contents);
    let directory = &scratch.path;
    let installation = KaniInstallation::new(directory.join("cargo-kani"));
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: directory,
        target_directory: &directory.join("target"),
        home_directory: None,
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::Tool(KaniToolError::Io {
            tool: KaniTool::Launcher,
            ..
        })
    ));
    assert!(!directory.join("target").exists(), "nothing ran");
}

/// Invalid launcher file kinds and execute permissions are typed faults before dispatch.
#[trace("TC-027")]
#[cfg(unix)]
#[test]
fn tc_027_non_executable_launchers_are_refused_before_anything_runs() {
    let cancel = quire_exact::Cancel::new();
    use std::os::unix::fs::PermissionsExt;

    let (harness, manifest) = routed_scalar_increment();
    for directory_launcher in [false, true] {
        let scratch = write_scalar_crate("bad-launcher", &manifest, &harness.rust.contents);
        let directory = &scratch.path;
        let launcher = directory.join("cargo-kani");
        if directory_launcher {
            fs::create_dir(&launcher).unwrap();
        } else {
            fs::write(&launcher, "#!/bin/sh\nexit 0\n").unwrap();
            fs::set_permissions(&launcher, fs::Permissions::from_mode(0o644)).unwrap();
        }
        let installation = KaniInstallation::new(launcher);
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: (&harness).into(),
            crate_directory: directory,
            target_directory: &directory.join("target"),
            home_directory: None,
        })
        .unwrap_err();
        match refusal {
            KaniExecutionRefusal::Tool(KaniToolError::Io {
                tool: KaniTool::Launcher,
                path,
                error,
            }) => {
                assert_eq!(path, installation.launcher);
                assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
            }
            other => panic!("expected a typed launcher permission fault, got {other:?}"),
        }
        assert!(!directory.join("target").exists(), "nothing ran");
    }
}

// ---- kani lane ---------------------------------------------------------------

/// One item that does not compile: it reads a name nothing defines. The V2 scalar subject
/// resolves, so this is the only compile error.
const BROKEN_ITEM: &str = "pub fn broken() -> i64 {\n    undefined_name\n}\n";

/// A launcher that exits 1 with a usage error on stderr, exports no report and prints nothing on
/// stdout, installed at `path` once it can be started.
#[cfg(unix)]
fn write_failing_launcher(path: &Path) {
    write_script_launcher(path, "printf 'error: unexpected argument\\n' >&2\nexit 1\n");
}

/// A launcher script that runs `body` (after exiting at once when probed), installed at `path`
/// once it can be started.
#[cfg(unix)]
fn write_script_launcher(path: &Path, body: &str) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        path,
        format!("#!/bin/sh\n[ \"$1\" = --probe ] && exit 0\n{body}"),
    )
    .unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    for _ in 0..500 {
        match std::process::Command::new(path).arg("--probe").status() {
            Err(error) if error.kind() == std::io::ErrorKind::ExecutableFileBusy => {
                std::thread::sleep(Duration::from_millis(10));
            }
            _ => break,
        }
    }
}

/// The FR-029 terminal value of a `NoVerdict` evidence that carries a diagnostic record is
/// `Failed`, and the record rides the public evidence and its wire form.
///
#[trace("FR-017-AC-32", "FR-017-AC-36", "TC-027", "TC-043")]
#[cfg(unix)]
#[test]
fn tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics() {
    let cancel = quire_exact::Cancel::new();
    let (harness, manifest) = routed_scalar_increment();
    let scratch = write_scalar_crate("no-verdict", &manifest, &harness.rust.contents);
    let directory = &scratch.path;
    let launcher = directory.join("cargo-kani");
    write_failing_launcher(&launcher);
    let installation = KaniInstallation::new(launcher);
    let mut evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: directory,
        target_directory: &directory.join("target"),
        home_directory: None,
    })
    .unwrap();

    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict
        }
    );
    let record = evidence
        .diagnostics
        .take()
        .expect("a NoVerdict run has a record");
    assert_eq!(record.stderr.text, "error: unexpected argument\n");
    assert_eq!(record.phase, KaniFailurePhase::Launcher);
    let terminal = run_terminal_value(&evidence, None).unwrap();
    assert_eq!(terminal, qsl_replay::TerminalValue::Failed);
    evidence.diagnostics = Some(record);
    let wire = serde_json::to_value(&evidence).unwrap();
    assert_eq!(wire["diagnostics"]["phase"], "launcher");
    assert_eq!(wire["diagnostics"]["stderr"]["omittedBytes"], 0);
}

/// Real Kani on a crate that holds the generated harness byte for byte beside an item that does
/// not compile (the `make kani` shape of a non-compiling harness): the run exits unsuccessfully,
/// exports no report and is `NoVerdict` with phase `compile`, the rustc error code and Kani's
/// `Failed to execute cargo` line in the stdout excerpt, and Cargo's `could not compile` line,
/// with the crate path redacted, in the stderr excerpt.
///
#[trace("FR-017-AC-32", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile() {
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let (harness, manifest) = routed_scalar_increment();
    let source = format!("{}\n{BROKEN_ITEM}", harness.rust.contents);
    let scratch = write_scalar_crate("non-compiling", &manifest, &source);
    let crate_directory = &scratch.path;
    let target = crate_directory.join("target");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: (&harness).into(),
        crate_directory,
        target_directory: &target,
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("a run that started is not refused: {refusal}"));

    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict
        }
    );
    assert_ne!(evidence.exit_code, Some(0));
    assert!(evidence.checks.is_empty());
    let record = evidence.diagnostics.expect("a NoVerdict run has a record");
    assert_eq!(record.phase, KaniFailurePhase::Compile, "{record:?}");
    assert_eq!(
        record.stdout.text.matches("error[E0425]").count(),
        1,
        "{}",
        record.stdout.text
    );
    assert!(
        record
            .stdout
            .text
            .contains("cannot find value `undefined_name`"),
        "{}",
        record.stdout.text
    );
    for text in [&record.stdout.text, &record.stderr.text] {
        assert!(
            !text.contains("cannot find function `subject`")
                && !text.contains("cannot find function subject"),
            "the scalar subject resolves, so the only error is the broken item: {text}"
        );
    }
    assert!(
        record
            .stdout
            .text
            .contains("error: Failed to execute cargo"),
        "{}",
        record.stdout.text
    );
    assert!(
        record.stderr.text.contains("error: could not compile"),
        "{}",
        record.stderr.text
    );
    let crate_text = crate_directory.display().to_string();
    assert!(
        record.stderr.text.contains("<CRATE_DIR>"),
        "{}",
        record.stderr.text
    );
    for text in [&record.stdout.text, &record.stderr.text] {
        assert!(!text.contains(&crate_text), "{text}");
    }
}

/// Bounded launcher stand-ins preserve failure classification while resource causes determine
/// the terminal value from the retained execution evidence.
///
#[trace("FR-017-AC-40", "FR-029-AC-32", "FR-029-AC-33", "TC-027")]
#[cfg(unix)]
#[test]
fn cbmc_failure_terminal_value_reads_the_retained_cause() {
    let cancel = quire_exact::Cancel::new();
    let (harness, manifest) = routed_scalar_increment();
    let scratch = write_scalar_crate("cbmc-failure", &manifest, &harness.rust.contents);
    let directory = &scratch.path;
    fs::create_dir_all(directory.join("target")).unwrap();
    for (status, expected) in [
        ("exit_code_64", qsl_replay::TerminalValue::Failed),
        (
            "out_of_memory",
            qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted),
        ),
        (
            "timeout",
            qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::TimedOut),
        ),
        ("cbmc_exploded", qsl_replay::TerminalValue::Failed),
    ] {
        let report = json!({
            "metadata": { "version": "1.0" },
            "error_details": [{ "harness_id": "m::h", "has_errors": true, "exit_status": status }],
            "verification_results": {
                "results": [{ "harness_id": "m::h", "status": "Failure", "checks": [] }]
            },
        });
        let exported = directory.join("report.json");
        fs::write(&exported, report.to_string()).unwrap();
        let launcher = directory.join("cargo-kani");
        write_script_launcher(
            &launcher,
            &format!(
                "for last; do :; done\ncp '{}' \"$last\"\nprintf 'CBMC failed\\n'\nexit 1\n",
                exported.display()
            ),
        );
        let installation = KaniInstallation::new(launcher);
        let evidence = execute_kani_obligation(&KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: (&harness).into(),
            crate_directory: directory,
            target_directory: &directory.join("target"),
            home_directory: None,
        })
        .unwrap();
        assert_eq!(
            evidence.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
        );
        assert!(
            serde_json::to_value(&evidence).unwrap()["failureCause"].is_object(),
            "{status}"
        );
        let terminal = run_terminal_value(&evidence, None).unwrap();
        assert_eq!(terminal, expected, "{status}");
    }
}

/// Real Kani on a crate whose manifest hands CBMC an option it does not know, with the harness's
/// option vector unchanged: the run exits unsuccessfully but exports a report, and is
/// `FailedWithoutCounterexample` with the cause `exit_code` 64 and a diagnostic record whose
/// stderr excerpt holds CBMC's own message.
///
#[trace("FR-017-AC-40", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause() {
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let (harness, manifest) = routed_scalar_increment();
    let scratch = write_scalar_crate("unknown-cbmc-option", &manifest, &harness.rust.contents);
    let crate_directory = &scratch.path;
    let manifest = crate_directory.join("Cargo.toml");
    let mut text = fs::read_to_string(&manifest).unwrap();
    text.push_str(
        "\n[package.metadata.kani]\nunstable = { unstable-options = true }\nflags = { cbmc-args = [\"--no-such-option\"] }\n",
    );
    fs::write(&manifest, text).unwrap();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: (&harness).into(),
        crate_directory,
        target_directory: &crate_directory.join("target"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("a run that started is not refused: {refusal}"));

    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        }
    );
    assert_ne!(evidence.exit_code, Some(0));
    let wire = serde_json::to_value(&evidence).unwrap();
    assert_eq!(
        wire["failureCause"],
        json!({ "kind": "exit_code", "code": 64 }),
        "{wire}"
    );
    assert_eq!(
        run_terminal_value(&evidence, None),
        Ok(qsl_replay::TerminalValue::Failed)
    );
    let record = evidence
        .diagnostics
        .expect("a CBMC failure carries the record");
    assert!(
        record
            .stderr
            .text
            .contains("Unknown option: --no-such-option"),
        "{}",
        record.stderr.text
    );
}

/// The routed scalar harness for `x + 1` over `Int[0, 9]`, and the oracle crate's `Cargo.toml`, both
/// exactly as `generate_routed` returns them: the inputs the driver assembles a run from.
pub(crate) fn routed_scalar_increment() -> (
    KaniScalarObligationHarness,
    quire_contract_codegen::Artifact,
) {
    let (harness, artifacts) = routed_scalar_increment_with_artifacts();
    let manifest = artifacts
        .into_iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("the oracle crate returns its manifest");
    (harness, manifest)
}

fn routed_scalar_increment_with_artifacts() -> (
    KaniScalarObligationHarness,
    Vec<quire_contract_codegen::Artifact>,
) {
    routed_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_100,
        Bound::Integer(0, 9),
        REAL_KANI_TIMEOUT,
    )
}

fn native_scalar_increment() -> (KaniScalarObligationHarness, Artifact) {
    let (harness, artifacts) = routed_native_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_100,
        Bound::Integer(0, 9),
        REAL_KANI_TIMEOUT,
    );
    let manifest = artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("the original dependency manifest");
    let production = artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("the routed production native oracle");
    assert!(
        harness.rust.contents.contains(&production.contents),
        "the proof must embed the routed production oracle byte for byte"
    );
    (harness, manifest.clone())
}

#[trace("FR-015-AC-125", "TC-027")]
#[test]
fn native_scalar_increment_keeps_original_profile_and_checked_capacity() {
    let (native, _) = native_scalar_increment();
    let (general, _) = routed_scalar_increment();
    assert_eq!(native.identity.node_id, general.identity.node_id);
    assert_eq!(
        native.identity.operation_identity,
        general.identity.operation_identity
    );
    assert_eq!(native.identity.arguments, general.identity.arguments);
    assert_eq!(native.identity.unwind, general.identity.unwind);
    assert_eq!(native.identity.ceilings, general.identity.ceilings);
    assert_eq!(native.identity.solver, KaniSolver::Cadical);
    assert_eq!(
        native.identity.integer_representation,
        quire_contract_codegen::ScalarIntegerRepresentation::Native
    );
    let selected = native
        .identity
        .native_integer_selection
        .as_ref()
        .expect("native selection is recorded");
    let [operation] = selected.operations.as_slice() else {
        panic!("one checked Add plan");
    };
    assert_eq!(operation.node_id, native.identity.node_id);
    assert_eq!(operation.operation_identity, "quire.op.integer.add");
    let endpoints = |interval: &quire_contract_codegen::NativeIntegerIntervalDecision| {
        (
            interval.lower.parse::<i128>().unwrap(),
            interval.upper.parse::<i128>().unwrap(),
        )
    };
    assert_eq!(
        operation.operands.iter().map(endpoints).collect::<Vec<_>>(),
        [(0, 9), (1, 1)]
    );
    assert_eq!(endpoints(operation.capacity.as_ref().unwrap()), (1, 10));
    assert_eq!(
        endpoints(operation.semantic_domain.as_ref().unwrap()),
        (0, 9)
    );
    assert!(native.rust.contents.contains("nx::NativeIntegerPlan::new"));
    assert!(native
        .rust
        .contents
        .contains("nx::evaluate_native_integer_arithmetic"));
    assert!(!general.rust.contents.contains("nx::NativeIntegerPlan::new"));
    let record: serde_json::Value =
        serde_json::from_str(&native.record.contents).expect("persisted harness record");
    let bound = &record["identity"]["nativeIntegerSelection"];
    assert_eq!(
        bound["policy"], "i128-arithmetic-signed32-multiply/v1",
        "the persisted identity must bind the selected policy"
    );
    assert_eq!(
        bound,
        &serde_json::to_value(selected).expect("selected typed policy"),
        "the persisted identity must bind the actual admission decisions"
    );
}

#[trace("FR-015-AC-125", "TC-027")]
#[test]
fn native_scalar_generated_proof_body_typechecks() {
    let (harness, manifest) = native_scalar_increment();
    assert_eq!(harness.rust.contents.matches("#[kani::proof]").count(), 1);
    assert_eq!(harness.rust.contents.matches("#[cfg(kani)]").count(), 1);
    let stubs = r#"extern crate self as kani;
pub fn any<T: Default>() -> T { T::default() }
pub fn assume(_: bool) {}
#[macro_export]
macro_rules! cover { ($condition:expr, $message:expr) => { let _ = ($condition, $message); }; }
"#;
    let source = format!(
        "{stubs}{}",
        harness
            .rust
            .contents
            .replace("#[kani::proof]", "#[test]")
            .replace("#[cfg(kani)]", "")
    );
    let scratch = write_scalar_crate("native-scalar-cfg", &manifest, &source);
    let metadata = std::process::Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&scratch.path)
        .output()
        .expect("resolve seeded native scalar lock");
    assert!(
        metadata.status.success(),
        "seeded native scalar lock: {}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let output = std::process::Command::new("cargo")
        .args(["check", "--offline", "--locked", "--tests", "--quiet"])
        .env("RUSTFLAGS", "-Dwarnings")
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&scratch.path)
        .output()
        .expect("typecheck the generated proof body");
    assert!(
        output.status.success(),
        "native scalar proof body failed typecheck: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-015-AC-126", "TC-027")]
#[test]
fn native_scalar_false_bound_control_keeps_original_draws() {
    let (healthy, _) = native_scalar_increment();
    let original = "exact <= i128::from(9_i64);";
    assert_eq!(healthy.rust.contents.matches(original).count(), 1);
    let mutant = healthy
        .rust
        .contents
        .replacen(original, "exact <= i128::from(5_i64);", 1);
    assert_eq!(healthy.identity.arguments[0].minimum, 0);
    assert_eq!(healthy.identity.arguments[0].maximum, 9);
    assert_eq!(healthy.identity.arguments[1].minimum, 1);
    assert_eq!(healthy.identity.arguments[1].maximum, 1);
    assert_eq!(healthy.identity.unwind, 3);
    assert_eq!(healthy.identity.solver, KaniSolver::Cadical);
    assert_eq!(
        healthy.identity.ceilings.memory_bytes.get(),
        16 * 1024 * 1024 * 1024
    );
    assert!(mutant.contains("exact <= i128::from(5_i64);"));
    assert!(mutant.contains("Some(nx::NativeIntegerInterval::new(0_i128, 9_i128)"));
}

#[trace("FR-015-AC-126", "TC-027")]
#[test]
#[ignore = "kani lane: real Kani qualification at the original 16 GiB ceiling"]
fn native_original_tc027_increment_verifies() {
    let (harness, manifest) = native_scalar_increment();
    let evidence = run_scalar_under_real_kani("native-original-tc027", &harness, &manifest);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{evidence:?}");
}

fn routed_scalar_binary_subtraction_with_artifacts() -> (
    KaniScalarObligationHarness,
    Vec<quire_contract_codegen::Artifact>,
) {
    routed_scalar_with_artifacts(
        package::bounded_binary_subtraction_package(),
        package::BOUNDED_BINARY_SUBTRACTION,
        49_102,
        Bound::Integer(-9, 9),
        REAL_KANI_TIMEOUT,
    )
}

/// Writes the crate the way the driver does: the returned `Cargo.toml` and `library` as
/// `src/lib.rs`.
fn write_scalar_crate(
    name: &str,
    manifest: &quire_contract_codegen::Artifact,
    library: &str,
) -> TemporaryDirectory {
    let scratch = TemporaryDirectory::new(&format!("quire-kani-obligations-scalar-{name}"));
    let directory = &scratch.path;
    write_manifest(directory, &manifest.contents);
    fs::write(directory.join("src/lib.rs"), library).unwrap();
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    scratch
}

/// A routed scalar harness carries a non-vacuity cover. A run with every cover satisfied is
/// verified; one with an unsatisfied cover is cover-unsatisfied. A success text with no cover
/// summary is inconclusive.
#[trace("FR-017-AC-4", "FR-017-AC-11", "FR-017-CON-1", "TC-027")]
#[test]
fn tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness() {
    let (scalar, _manifest) = routed_scalar_increment();
    let covers = |source: &str| source.matches("kani::cover!(").count();
    assert_eq!(covers(&scalar.rust.contents), 1, "one non-vacuity cover");
    let total = covers(&scalar.rust.contents);
    let report = |checks: Vec<serde_json::Value>| {
        serde_json::to_vec(&serde_json::json!({
            "metadata": { "version": "1.0" },
            "verification_results": { "results": [{
                "harness_id": "h", "status": "Success", "checks": checks
            }] }
        }))
        .unwrap()
    };
    let check = |status: &str, category: &str| {
        serde_json::json!({
            "id": 1,
            "status": status,
            "category": category,
            "location": { "file": "src/lib.rs", "line": "10", "column": "5" },
        })
    };
    let classify = |checks: Vec<serde_json::Value>| {
        classify_kani_run(true, Some(&report(checks)), None, None)
            .unwrap()
            .outcome
    };
    let covers = |status: &str| {
        let mut checks = vec![check("Success", "assertion")];
        checks.extend((0..total).map(|_| check(status, "cover")));
        checks
    };
    assert_eq!(classify(covers("Satisfied")), KaniRunOutcome::Verified);
    assert_eq!(
        classify(covers("Unsatisfiable")),
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: u64::try_from(total).unwrap()
        }
    );
    assert_eq!(
        classify(vec![check("Success", "assertion")]),
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MissingCoverSummary
        }
    );
}

/// Runs `library` (a routed scalar harness's source, possibly mutated) through the execution
/// module in the crate the driver assembles, under the real Kani backend and the lane budget.
pub(crate) fn run_scalar_under_real_kani(
    name: &str,
    harness: &KaniScalarObligationHarness,
    manifest: &quire_contract_codegen::Artifact,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let scratch = write_scalar_crate(name, manifest, &harness.rust.contents);
    let crate_directory = &scratch.path;
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-scalar"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("{refusal}"));
    evidence
}

pub(crate) fn verify_routed_scalar_harness_real_kani() {
    let cancel = quire_exact::Cancel::new();
    let (harness, manifest) = routed_scalar_increment();
    let evidence = run_scalar_under_real_kani("scalar-real", &harness, &manifest);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.kind, None);
    assert_eq!(evidence.harness_path, harness.rust.path);
    assert_eq!(
        evidence.launcher_path,
        KaniInstallation::discover()
            .unwrap()
            .launcher
            .display()
            .to_string()
    );
    assert_eq!(evidence.arguments.len(), harness.identity.options.len() + 1);
    assert_eq!(
        evidence.arguments[1..=harness.identity.options.len()],
        harness.identity.options[..]
    );
    assert_eq!(evidence.unwind, harness.identity.unwind);
    assert_eq!(evidence.solver, harness.identity.solver);
    assert_eq!(evidence.exit_code, Some(0));

    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let scratch = write_scalar_crate("scalar-real-missing", &manifest, "//! empty\n");
    let crate_directory = &scratch.path;
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: (&harness).into(),
        crate_directory,
        target_directory: &crate_directory.join("target"),
        home_directory: None,
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::HarnessNotInCrate { .. }
    ));
    assert!(!crate_directory.join("target").exists(), "nothing ran");
}

/// A retained routed scalar is proved and invoked with normal and constrained native limits.
#[trace(
    "FR-017-AC-27",
    "FR-017-AC-28",
    "FR-017-AC-29",
    "FR-017-AC-30",
    "FR-017-AC-45",
    "FR-017-AC-46",
    "FR-017-AC-47",
    "FR-017-AC-48",
    "FR-017-AC-50"
)]
#[test]
#[ignore = "kani lane: run serially through make kani"]
fn tc_027_attested_scalar_receipt_binds_real_proof_and_native_result() {
    use quire_exact::{ChargePoint, LimitKind, Outcome};
    let (harness, artifacts) = routed_scalar_increment_with_artifacts();
    let operands = [4_i64, 1_i64];
    let limits = attested_scalar_limits();
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-scalar-attested");
    let receipt = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &operands,
        scalar_limits: limits,
        target_directory: &target,
    })
    .expect("verified proof and typed native receipt");
    assert_eq!(receipt.proof_evidence().outcome, KaniRunOutcome::Verified);
    assert_eq!(receipt.operands(), operands);
    assert_eq!(receipt.scalar_limits(), &limits);
    assert_retained_scalar_content(&receipt, &harness, &artifacts);
    assert!(
        matches!(receipt.native_outcome(), Outcome::Completed(value) if value.to_string() == "5")
    );
    assert!(receipt
        .meter_observation()
        .consumed()
        .iter()
        .any(|count| *count > 0));
    let mut constrained_limits = limits;
    constrained_limits.result_units = 0;
    let constrained_receipt = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &operands,
        scalar_limits: constrained_limits,
        target_directory: &target,
    })
    .expect("verified proof and typed native incomplete receipt at zero result budget");

    assert_eq!(
        constrained_receipt.proof_evidence().outcome,
        KaniRunOutcome::Verified
    );
    assert_eq!(constrained_receipt.operands(), operands);
    assert_eq!(constrained_receipt.scalar_limits(), &constrained_limits);
    assert_retained_scalar_content(&constrained_receipt, &harness, &artifacts);
    assert_eq!(
        receipt.proof_content_digest(),
        constrained_receipt.proof_content_digest()
    );
    assert_eq!(
        receipt.proof_content_identity(),
        constrained_receipt.proof_content_identity(),
        "native request limits do not change the staged proof content"
    );
    let Outcome::Incomplete(incomplete) = constrained_receipt.native_outcome() else {
        panic!("zero result_units must stop integer result retention");
    };
    assert_eq!(incomplete.limit_kind, LimitKind::ResultUnits);
    assert_eq!(incomplete.limit, 0);
    assert_eq!(incomplete.consumed, 0);
    assert_eq!(incomplete.next_charge, 1_i64.into());
    assert_eq!(
        incomplete.charge_point,
        ChargePoint::IntegerArithmeticResultRetain
    );
    assert_eq!(constrained_receipt.meter_observation().consumed()[9], 0);
    assert!(!constrained_receipt
        .meter_observation()
        .admitted_charges()
        .contains(&ChargePoint::IntegerArithmeticResultRetain));
}

/// The routed subtraction fixture retains two independently bounded symbolic operands.
#[trace("FR-017-AC-31")]
#[test]
fn tc_027_attested_scalar_subtraction_fixture_has_two_bounded_operands() {
    let (harness, _) = routed_scalar_binary_subtraction_with_artifacts();
    assert_eq!(harness.identity.arguments.len(), 2);
    assert!(harness
        .identity
        .arguments
        .iter()
        .all(|argument| argument.minimum == 0 && argument.maximum == 9));
}

#[trace("FR-017-AC-31", "FR-017-AC-47", "FR-017-AC-50")]
#[test]
#[ignore = "kani lane: run serially through make kani"]
fn tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders() {
    use quire_exact::Outcome;

    let (harness, artifacts) = routed_scalar_binary_subtraction_with_artifacts();
    assert_eq!(harness.identity.arguments.len(), 2);
    assert!(harness
        .identity
        .arguments
        .iter()
        .all(|argument| argument.minimum == 0 && argument.maximum == 9));
    let limits = attested_scalar_limits();
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-scalar-attested-orders");

    let left_first = [4_i64, 1_i64];
    let left_first_receipt = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &left_first,
        scalar_limits: limits,
        target_directory: &target,
    })
    .expect("verified proof and typed native receipt for [4, 1]");

    let right_first = [1_i64, 4_i64];
    let right_first_receipt = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &artifacts,
        operands: &right_first,
        scalar_limits: limits,
        target_directory: &target,
    })
    .expect("verified proof and typed native receipt for [1, 4]");

    assert_eq!(
        left_first_receipt.proof_evidence().outcome,
        KaniRunOutcome::Verified
    );
    assert_eq!(
        right_first_receipt.proof_evidence().outcome,
        KaniRunOutcome::Verified
    );
    assert_eq!(left_first_receipt.operands(), left_first);
    assert_eq!(right_first_receipt.operands(), right_first);
    assert_eq!(left_first_receipt.scalar_limits(), &limits);
    assert_eq!(right_first_receipt.scalar_limits(), &limits);
    assert_retained_scalar_content(&left_first_receipt, &harness, &artifacts);
    assert_retained_scalar_content(&right_first_receipt, &harness, &artifacts);
    assert_eq!(
        left_first_receipt.proof_content_digest(),
        right_first_receipt.proof_content_digest()
    );
    assert_eq!(
        left_first_receipt.proof_content_identity(),
        right_first_receipt.proof_content_identity(),
        "request operands are retained in each receipt but are not staged proof content"
    );
    assert!(
        matches!(left_first_receipt.native_outcome(), Outcome::Completed(value) if value.to_string() == "3")
    );
    assert!(
        matches!(right_first_receipt.native_outcome(), Outcome::Completed(value) if value.to_string() == "-3")
    );
}

fn assert_retained_scalar_content(
    receipt: &quire_contract_codegen::KaniScalarNativeReceipt,
    harness: &KaniScalarObligationHarness,
    artifacts: &[quire_contract_codegen::Artifact],
) {
    let entries = receipt.staged_entries();
    assert_eq!(
        entries
            .iter()
            .map(|(path, _)| path.as_str())
            .collect::<Vec<_>>(),
        vec![
            "Cargo.lock",
            "Cargo.toml",
            "claim-map.json",
            &harness.record.path,
            "observer/Cargo.lock",
            "observer/Cargo.toml",
            "source-maps/lib.json",
            "src/bin/ir791_native.rs",
            "src/lib.rs",
            "src/oracle-artifacts-lib.rs"
        ]
    );
    let expected = independent_digest(entries);
    for index in 0..entries.len() {
        for mutation in 0..5 {
            let mut changed = entries.to_vec();
            match mutation {
                0 => {
                    changed.remove(index);
                }
                1 => changed[index].0.push('x'),
                2 => changed.swap(index, (index + 1) % entries.len()),
                3 => changed.insert(index, changed[index].clone()),
                4 => changed[index].1[0] ^= 1,
                _ => unreachable!(),
            }
            assert_ne!(
                independent_digest(&changed),
                expected,
                "member {index}, mutation {mutation}"
            );
        }
    }
    let digest: &qsl_replay::DigestRecord = receipt.proof_content_digest();
    assert_eq!(digest.domain().as_str(), "sha256-jcs");
    assert_eq!(digest.as_bytes(), &expected);
    assert_eq!(receipt.proof_content_identity().as_bytes(), &expected);
    for artifact in artifacts {
        let path = if artifact.path == "src/lib.rs" {
            "src/oracle-artifacts-lib.rs"
        } else {
            &artifact.path
        };
        assert_eq!(
            &entries.iter().find(|(name, _)| name == path).unwrap().1,
            artifact.contents.as_bytes()
        );
    }
    assert_eq!(
        &entries
            .iter()
            .find(|(name, _)| name == "src/lib.rs")
            .unwrap()
            .1,
        harness.rust.contents.as_bytes()
    );
    assert_eq!(
        &entries
            .iter()
            .find(|(name, _)| name == &harness.record.path)
            .unwrap()
            .1,
        harness.record.contents.as_bytes()
    );
    assert_eq!(
        receipt.meter_observation().limits(),
        receipt.scalar_limits()
    );
    assert!(receipt.cleanup_diagnostic().is_none());
}

fn attested_scalar_limits() -> quire_exact::ScalarLimits {
    quire_exact::ScalarLimits {
        integer_bits: 10000,
        decimal_digits: 10001,
        scale_expansion: 10002,
        text_input_bytes: 10003,
        text_scalars: 10004,
        normalized_scalars: 10005,
        unit_edges: 10006,
        value_occurrences: 10007,
        work_units: 10008,
        result_units: 10009,
    }
}

/// Mismatched staged paths, retained scalar identity and operands refuse before either child starts.
#[trace("FR-017-AC-28", "FR-017-AC-29")]
#[test]
fn tc_027_attested_scalar_rejects_preflight_mismatches_without_launch() {
    use quire_contract_codegen::{KaniScalarNativeError, KaniScalarNativeRequest};

    let (harness, valid_artifacts) = routed_scalar_increment_with_artifacts();
    let cancel = quire_exact::Cancel::new();
    let control_scratch = TemporaryDirectory::new("quire-kani-obligations-preflight");
    let control_directory = &control_scratch.path;
    let launcher = control_directory.join("fake-cargo-kani");
    let marker = control_directory.join("launched");
    fs::write(
        &launcher,
        format!("#!/bin/sh\n: > '{}'\nexit 97\n", marker.display()),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let installation = KaniInstallation::new(launcher);
    let target = control_directory.join("target");
    let limits = attested_scalar_limits();
    let operands = [4_i64, 1_i64];
    let assert_not_launched = || assert!(!marker.exists(), "preflight failure launched a child");

    let mut changed_path_artifacts = valid_artifacts.clone();
    changed_path_artifacts
        .iter_mut()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("generated oracle library")
        .path = "src/renamed.rs".to_owned();
    let changed_path = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &changed_path_artifacts,
        operands: &operands,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert_eq!(changed_path.as_ref().unwrap_err().code(), "invalid_request");
    assert_eq!(
        changed_path.as_ref().unwrap_err().class(),
        quire_contract_codegen::KaniScalarNativeErrorClass::CompletionRefusal
    );
    assert!(matches!(
        changed_path,
        Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar package changed after routed generation"
        ))
    ));
    assert_not_launched();

    let mut changed_bytes_artifacts = valid_artifacts.clone();
    let source = changed_bytes_artifacts
        .iter_mut()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("generated oracle library");
    source.contents = source.contents.replacen("pub fn ", "fn ", 1);
    let changed_bytes = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &changed_bytes_artifacts,
        operands: &operands,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert!(matches!(
        changed_bytes,
        Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar package changed after routed generation"
        ))
    ));
    assert_not_launched();

    let mut changed_scalar_identity = harness.clone();
    changed_scalar_identity
        .identity
        .operation_identity
        .push_str(".changed");
    let changed_identity = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &changed_scalar_identity,
        oracle_artifacts: &valid_artifacts,
        operands: &operands,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert!(matches!(
        changed_identity,
        Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar harness changed after generation"
        ))
    ));
    assert_not_launched();

    let mut changed_oracle_symbol = harness.clone();
    let renamed_harness = changed_oracle_symbol.rust.contents.replacen(
        &harness.identity.oracle_symbol,
        "wrong_oracle",
        1,
    );
    assert_ne!(renamed_harness, changed_oracle_symbol.rust.contents);
    changed_oracle_symbol.rust.contents = renamed_harness;
    let wrong_symbol = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &changed_oracle_symbol,
        oracle_artifacts: &valid_artifacts,
        operands: &operands,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert!(matches!(
        wrong_symbol,
        Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar harness changed after generation"
        ))
    ));
    assert_not_launched();

    let one_operand = [4_i64];
    let wrong_operand_count = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &valid_artifacts,
        operands: &one_operand,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert!(matches!(
        wrong_operand_count,
        Err(KaniScalarNativeError::InvalidRequest(
            "operand count differs from harness identity"
        ))
    ));
    assert_not_launched();

    let out_of_bounds_operands = [i64::MAX, 1_i64];
    let out_of_bounds = execute_kani_scalar_native(&KaniScalarNativeRequest {
        cancel: &cancel,
        installation: &installation,
        harness: &harness,
        oracle_artifacts: &valid_artifacts,
        operands: &out_of_bounds_operands,
        scalar_limits: limits,
        target_directory: &target,
    });
    assert!(matches!(
        out_of_bounds,
        Err(KaniScalarNativeError::InvalidRequest(
            "operand lies outside its retained bounds"
        ))
    ));
    assert_not_launched();
    assert!(!target.exists(), "preflight refusal created a build target");
}

/// The same harness with its checked domain narrowed to `[0, 5]` asserts a bound the oracle does
/// not enforce (it completes up to 9), so the real backend falsifies it with a counterexample.
#[trace("FR-017-AC-7", "FR-017-AC-11", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified() {
    let (mut harness, manifest) = routed_scalar_increment();
    let domain_upper = "exact <= i128::from(9_i64);";
    assert_eq!(harness.rust.contents.matches(domain_upper).count(), 1);
    harness.rust.contents = harness
        .rust
        .contents
        .replace(domain_upper, "exact <= i128::from(5_i64);");
    let evidence = run_scalar_under_real_kani("scalar-real-violating", &harness, &manifest);
    assert!(
        matches!(evidence.outcome, KaniRunOutcome::Falsified { .. }),
        "got {:?}",
        evidence.outcome
    );
}

/// Mutation control for the arithmetic property: the embedded oracle's `add` is replaced by
/// `subtract`, the one defect the old in-domain assertion could not see (a subtraction result that
/// stays in the domain is still `Completed`). The healthy harness verifies, and the mutated one is
/// falsified by a counterexample on the harness's own assertion, not by a build error: the mutated
/// source still compiles, and the failing check is the generated `sound` assertion, whose message
/// the counterexample names.
#[trace("FR-015-AC-37", "TC-025", "FR-017-AC-11")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic() {
    let (harness, manifest) = routed_scalar_increment();
    let healthy = run_scalar_under_real_kani("scalar-arith-healthy", &harness, &manifest);
    assert_eq!(healthy.outcome, KaniRunOutcome::Verified);

    let call = "rt::IntegerArithmetic::Add(left, right)";
    let mut mutated = harness.clone();
    assert_eq!(mutated.rust.contents.matches(call).count(), 1);
    mutated.rust.contents = mutated
        .rust
        .contents
        .replace(call, "rt::IntegerArithmetic::Subtract(left, right)");
    let evidence = run_scalar_under_real_kani("scalar-arith-mutated", &mutated, &manifest);
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("the mutated arithmetic must be falsified: {evidence:?}");
    };
    assert!(counterexample.contains(mutated.identity.harness_symbol.as_str()));
    assert_ne!(evidence.exit_code, Some(0));
    let message = "the oracle must complete with exactly the native quire.op.integer.add result";
    assert!(
        counterexample.contains("Check for `assertion`") && counterexample.contains(message),
        "the failing check must be the generated `sound` assertion, got {counterexample}"
    );
}

#[test]
#[trace("TC-027", "FR-017-AC-51")]
fn tc_027_scalar_limits_is_available_at_codegen_root_with_request_type_identity() {
    use quire_contract_codegen::{KaniScalarNativeRequest, ScalarLimits};

    // This function's return type proves that the public root export is the
    // exact type of the existing request field, without naming Runtime.
    fn request_limits<'a>(request: &'a KaniScalarNativeRequest<'a>) -> &'a ScalarLimits {
        &request.scalar_limits
    }

    let limits = ScalarLimits {
        integer_bits: 64,
        decimal_digits: 20,
        scale_expansion: 20,
        text_input_bytes: 1024,
        text_scalars: 1024,
        normalized_scalars: 1024,
        unit_edges: 100,
        value_occurrences: 100,
        work_units: 1000,
        result_units: 1000,
    };
    let _typed_accessor = request_limits;
    let _kernel_limits: quire_exact::ScalarLimits = limits;
}
