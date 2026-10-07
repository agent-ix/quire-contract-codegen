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

use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::scratch_crate::{runtime_dependency, write_manifest};
use package::{
    application, bounded, code_id, corpus_package, golden_items, id, integer_add, key, op,
    reference, Bound, MISSING, MISSING_ROUNDING, MODEL, STATE, T_BOOLEAN, T_INTEGER, UNBOUNDED,
};
use quire_contract_codegen::{
    classify_kani_run, execute_kani_obligation, generate_exact_scalar_oracles, generate_routed,
    negotiate_kani_obligations, BackendKind, Candidate, ClaimDisposition, ClaimMap,
    ExactScalarClaim, ExactScalarItem, ExactScalarOperation, ExactScalarRefusal,
    GenerationContexts, IntegerOperator, InvalidObligationItem, KaniExecutionRefusal,
    KaniExecutionRequest, KaniGenerationContext, KaniInconclusiveReason, KaniInstallation,
    KaniObligationError, KaniObligationHarness, KaniObligationOutcome, KaniObligationRequest,
    KaniRunOutcome, KaniScalarObligationHarness, KaniSolver, KaniTool, KaniToolError, KindOutput,
    ObligationDisposition, ObligationItem, ObligationKind, ObligationRecord, ObligationSubject,
    OperationProvenance, RoutedGenerationItem, UnsupportedObligation, UpstreamBlocker,
    MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
use quire_contract_model::{
    BoundPackage, CheckedPackageV2, ClauseId, ClauseKind, ClauseRef, RequirementRef,
    EXECUTABLE_PROJECTION_FORMAT,
};
use serde_json::{json, Value};

const PACKAGE: &str = "test/kani-obligations";
const PRECONDITION: &str = "amount-within-balance";
const POSTCONDITION: &str = "balance-never-grows";
const INVARIANT: &str = "balance-nonnegative";
const DEFINEDNESS: &str = "doubled-amount-fits";
const ASSERTION: &str = "amount-nonnegative";

/// Budget for the kani lane's real `cargo-kani` runs. Generous because CBMC is memory- and
/// time-heavy on these small obligations; this is a ceiling against a genuine hang, not a
/// performance target.
pub(crate) const REAL_KANI_TIMEOUT: Duration = Duration::from_secs(600);
/// Trace: FR-028-AC-1.
#[test]
fn changing_either_request_ceiling_changes_the_generated_proof_identity() {
    let package = bound_package(1000);
    let clause = clause(PRECONDITION);
    let items = [ObligationItem::BoundClause {
        package: &package,
        clause: &clause,
    }];
    let mut requested = request(&items, "crate::withdraw");
    let original = emitted(negotiate_kani_obligations(&requested).unwrap())
        .1
        .remove(0);
    requested.ceilings.memory_bytes =
        std::num::NonZeroU64::new(requested.ceilings.memory_bytes.get() / 2).unwrap();
    let memory_changed = emitted(negotiate_kani_obligations(&requested).unwrap())
        .1
        .remove(0);
    assert_ne!(original.identity, memory_changed.identity);
    assert_ne!(original.record.contents, memory_changed.record.contents);
    assert_eq!(memory_changed.identity.ceilings, requested.ceilings);
    requested.ceilings = original.identity.ceilings;
    requested.ceilings.wall_clock /= 2;
    let wall_changed = emitted(negotiate_kani_obligations(&requested).unwrap())
        .1
        .remove(0);
    assert_ne!(original.identity, wall_changed.identity);
    assert_ne!(original.record.contents, wall_changed.record.contents);
    assert_eq!(wall_changed.identity.ceilings, requested.ceilings);
}

// ---- V1 fixture --------------------------------------------------------------

fn span(line: u64) -> Value {
    let source = json!({"document":"kani-obligations", "revision":1});
    json!({"start":{"source":source,"line":line,"column":1,"byte_offset":line - 1},
        "end":{"source":source,"line":line,"column":2,"byte_offset":line}})
}

fn int(minimum: i64, maximum: i64) -> Value {
    json!({"kind":"integer","domain":"signed","minimum":minimum.to_string(),"maximum":maximum.to_string(),"overflow":"reject"})
}

fn owner() -> Value {
    json!({"package":PACKAGE,"requirement":"FR-200","revision":1})
}

fn read(name: &str, observation: &str, line: u64) -> Value {
    json!({"node":"value_reference","name":name,"observation":observation,"source":span(line)})
}

fn identity(kind: &str, name: &str, observation: &str) -> Value {
    json!({"node":"reference","identity":{"requirement":owner(),"kind":kind,"observation":observation,"path":[name]}})
}

struct ClauseFixture {
    id: &'static str,
    kind: &'static str,
    anchor: Value,
    line: u64,
    references: Vec<Value>,
    values: Vec<Value>,
    expression: Value,
}

fn amount(line: u64) -> Value {
    json!({"name":"amount","kind":"input","value_type":int(0, 1000),"source":span(line)})
}

fn balance(line: u64, maximum: i64) -> Value {
    json!({"name":"balance","kind":"state","value_type":int(0, maximum),"source":span(line)})
}

fn clauses(balance_maximum_in_invariant: i64) -> Vec<ClauseFixture> {
    let pre = json!({"kind":"pre","operation":"withdraw"});
    vec![
        ClauseFixture {
            id: PRECONDITION,
            kind: "precondition",
            anchor: pre.clone(),
            line: 10,
            references: vec![
                identity("input", "amount", "current"),
                identity("state", "balance", "current"),
            ],
            values: vec![amount(11), balance(12, 1000)],
            expression: json!({"node":"compare","operator":"less_equal",
                "left":read("amount", "current", 13),"right":read("balance", "current", 14),
                "source":span(11)}),
        },
        ClauseFixture {
            id: POSTCONDITION,
            kind: "postcondition",
            anchor: json!({"kind":"post","operation":"withdraw"}),
            line: 20,
            references: vec![
                identity("state", "balance", "post"),
                identity("state", "balance", "pre"),
            ],
            values: vec![balance(21, 1000)],
            expression: json!({"node":"compare","operator":"less_equal",
                "left":read("balance", "post", 22),"right":read("balance", "pre", 23),
                "source":span(21)}),
        },
        ClauseFixture {
            id: INVARIANT,
            kind: "invariant",
            anchor: json!({"kind":"handler","name":"withdraw"}),
            line: 30,
            references: vec![identity("state", "balance", "current")],
            values: vec![balance(31, balance_maximum_in_invariant)],
            expression: json!({"node":"compare","operator":"greater_equal",
                "left":read("balance", "current", 32),
                "right":{"node":"integer_literal","value":"0","value_type":int(0, balance_maximum_in_invariant),"source":span(33)},
                "source":span(31)}),
        },
        ClauseFixture {
            id: DEFINEDNESS,
            kind: "precondition",
            anchor: json!({"kind":"pre","operation":"deposit"}),
            line: 40,
            references: vec![identity("input", "amount", "current")],
            values: vec![amount(41)],
            // `amount != 0 && 1000 / amount <= 1000`: the division carries a guarded
            // non-zero-divisor obligation.
            expression: json!({"node":"boolean","operator":"short_circuit_and",
                "left":{"node":"compare","operator":"not_equal","left":read("amount", "current", 42),
                    "right":{"node":"integer_literal","value":"0","value_type":int(0, 1000),"source":span(43)},
                    "source":span(42)},
                "right":{"node":"compare","operator":"less_equal",
                    "left":{"node":"numeric","operator":"divide",
                        "left":{"node":"integer_literal","value":"1000","value_type":int(0, 1000),"source":span(44)},
                        "right":read("amount", "current", 45),"source":span(44)},
                    "right":{"node":"integer_literal","value":"1000","value_type":int(0, 1000),"source":span(46)},
                    "source":span(44)},
                "source":span(41)}),
        },
        ClauseFixture {
            id: ASSERTION,
            kind: "assertion",
            anchor: json!({"kind":"pre","operation":"audit"}),
            line: 50,
            references: vec![identity("input", "amount", "current")],
            values: vec![amount(51)],
            expression: json!({"node":"compare","operator":"greater_equal",
                "left":read("amount", "current", 52),
                "right":{"node":"integer_literal","value":"0","value_type":int(0, 1000),"source":span(53)},
                "source":span(51)}),
        },
    ]
}

const TRANSFER_SMALL: &str = "transfer-at-most-five";
const TRANSFER_LARGE: &str = "transfer-at-least-ten";
const TRANSFER_IMPOSSIBLE: &str = "transfer-balance-negative";
const REFUND_CAPPED: &str = "refund-within-fee";
const REFUND_NONNEGATIVE: &str = "refund-balance-nonnegative";

fn literal(value: i64, line: u64) -> Value {
    json!({"node":"integer_literal","value":value.to_string(),"value_type":int(0, 1000),"source":span(line)})
}

fn compare(operator: &str, left: Value, right: Value, line: u64) -> Value {
    json!({"node":"compare","operator":operator,"left":left,"right":right,"source":span(line)})
}

/// `transfer`: two preconditions, each satisfiable in `0..=1000` but not together, and a
/// postcondition no result satisfies. `refund`: a postcondition reading input `fee` that the
/// invariant on the same operation does not read; `refund_invariant_maximum` sets the invariant's
/// balance type so the two can disagree on a shared slot.
fn operation_group_clauses(refund_invariant_maximum: i64) -> Vec<ClauseFixture> {
    let transfer = json!({"kind":"pre","operation":"transfer"});
    let fee = json!({"name":"fee","kind":"input","value_type":int(0, 1000),"source":span(91)});
    vec![
        ClauseFixture {
            id: TRANSFER_SMALL,
            kind: "precondition",
            anchor: transfer.clone(),
            line: 60,
            references: vec![identity("input", "amount", "current")],
            values: vec![amount(61)],
            expression: compare(
                "less_equal",
                read("amount", "current", 62),
                literal(5, 63),
                61,
            ),
        },
        ClauseFixture {
            id: TRANSFER_LARGE,
            kind: "precondition",
            anchor: transfer,
            line: 70,
            references: vec![identity("input", "amount", "current")],
            values: vec![amount(71)],
            expression: compare(
                "greater_equal",
                read("amount", "current", 72),
                literal(10, 73),
                71,
            ),
        },
        ClauseFixture {
            id: TRANSFER_IMPOSSIBLE,
            kind: "postcondition",
            anchor: json!({"kind":"post","operation":"transfer"}),
            line: 80,
            references: vec![identity("state", "balance", "post")],
            values: vec![balance(81, 1000)],
            expression: compare("less", read("balance", "post", 82), literal(0, 83), 81),
        },
        ClauseFixture {
            id: REFUND_CAPPED,
            kind: "postcondition",
            anchor: json!({"kind":"post","operation":"refund"}),
            line: 90,
            references: vec![
                identity("state", "balance", "post"),
                identity("input", "fee", "current"),
            ],
            values: vec![balance(92, 1000), fee],
            expression: compare(
                "less_equal",
                read("balance", "post", 93),
                read("fee", "current", 94),
                91,
            ),
        },
        ClauseFixture {
            id: REFUND_NONNEGATIVE,
            kind: "invariant",
            anchor: json!({"kind":"handler","name":"refund"}),
            line: 100,
            references: vec![identity("state", "balance", "current")],
            values: vec![balance(101, refund_invariant_maximum)],
            expression: json!({"node":"compare","operator":"greater_equal",
                "left":read("balance", "current", 102),
                "right":{"node":"integer_literal","value":"0","value_type":int(0, refund_invariant_maximum),"source":span(103)},
                "source":span(101)}),
        },
    ]
}

fn operation_group_package(refund_invariant_maximum: i64) -> BoundPackage {
    let mut fixtures = clauses(1000);
    fixtures.extend(operation_group_clauses(refund_invariant_maximum));
    BoundPackage::from_json_bytes(&serde_json::to_vec(&projection_of(&fixtures)).unwrap())
        .unwrap_or_else(|diagnostics| panic!("fixture projection must bind: {diagnostics:?}"))
}

fn projection(balance_maximum_in_invariant: i64) -> Value {
    projection_of(&clauses(balance_maximum_in_invariant))
}

fn projection_of(fixtures: &[ClauseFixture]) -> Value {
    let package_clauses = fixtures
        .iter()
        .map(|clause| {
            let body = match clause.references.as_slice() {
                [single] => single.clone(),
                many => json!({"node":"composite","children":many}),
            };
            json!({"id":clause.id,"kind":clause.kind,"anchor":clause.anchor,
                "source":span(clause.line),"body":body})
        })
        .collect::<Vec<_>>();
    let bindings = fixtures
        .iter()
        .map(|clause| {
            json!({"clause":{"requirement":owner(),"clause":clause.id},
                "expression":{"owner":owner(),"types":[],"values":clause.values,"functions":[],
                    "expression":clause.expression,"expected_type":{"kind":"boolean"},
                    "execution_point":clause.anchor,"clause_root":true}})
        })
        .collect::<Vec<_>>();
    json!({
        "format":EXECUTABLE_PROJECTION_FORMAT,
        "package":{"id":PACKAGE,"schema_version":{"major":1,"minor":1},
            "source":{"document":"kani-obligations","revision":1},
            "requirements":[{"id":"FR-200","revision":1,"source":span(1),"clauses":package_clauses}]},
        "bindings":bindings
    })
}

pub(crate) fn bound_package(balance_maximum_in_invariant: i64) -> BoundPackage {
    BoundPackage::from_json_bytes(
        &serde_json::to_vec(&projection(balance_maximum_in_invariant)).unwrap(),
    )
    .unwrap_or_else(|diagnostics| panic!("fixture projection must bind: {diagnostics:?}"))
}

fn clause(id: &str) -> ClauseRef {
    ClauseRef::new(
        RequirementRef::parse(PACKAGE, "FR-200", 1).unwrap(),
        ClauseId::new(id).unwrap(),
    )
}

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

fn emitted(outcome: KaniObligationOutcome) -> (Vec<ObligationRecord>, Vec<KaniObligationHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted {
            records, harnesses, ..
        } => (records, harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
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

pub(crate) fn supported_contract_harnesses(
    package: &BoundPackage,
    subject: &str,
) -> Vec<KaniObligationHarness> {
    let refs = [
        clause(PRECONDITION),
        clause(POSTCONDITION),
        clause(INVARIANT),
    ];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause { package, clause })
        .collect::<Vec<_>>();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, subject)).unwrap());
    assert!(records
        .iter()
        .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })));
    harnesses
}

/// The precondition, V1 contract (postcondition and invariant) and scalar harness sources, for
/// the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let mut sources = supported_contract_harnesses(&bound_package(1000), "crate::withdraw")
        .into_iter()
        .map(|harness| {
            let family = match harness.identity.kind {
                ObligationKind::Precondition => "precondition",
                ObligationKind::Postcondition => "v1 contract postcondition",
                ObligationKind::Invariant => "v1 contract invariant",
                // The clause renderer never emits a frame harness; one here has a label no
                // family lists, so the guard's family-set check fails on it.
                ObligationKind::Frame => "unexpected frame harness",
            };
            (family, harness.rust.contents)
        })
        .collect::<Vec<_>>();
    let (scalar, claim_map, ids) = scalar_package();
    let node = ids.resolve(&code_id(1001));
    let items = [ObligationItem::ScalarClaim {
        package: &scalar,
        claim_map: &claim_map,
        node_id: &node,
    }];
    let (_, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    sources.extend(
        scalar_harnesses
            .into_iter()
            .map(|harness| ("scalar", harness.rust.contents)),
    );
    sources
}

// ---- V2 fixture --------------------------------------------------------------

/// An integer addition whose only bound admits no value.
const UNSATISFIABLE: u32 = 3001;
/// A frame clause.
const FRAME: u32 = 3002;
/// The object type `FRAME` frames.
const FRAMED_OBJECT: u32 = 3004;

pub(crate) fn scalar_package() -> (
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
            json!({"term": "frame", "modifies": [], "creates": [], "deletes": []}),
        );
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
    let items = ids
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package,
            claim_map,
            node_id,
        })
        .collect::<Vec<_>>();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    assert!(harnesses.is_empty(), "no V2 item may emit a harness");
    records
}

// ---- default lane ------------------------------------------------------------

/// Each precondition, postcondition and invariant clause is its own obligation, harness and
/// proof, with its clause, source span and assumed preconditions recorded.
///
/// Trace: FR-015-AC-1, FR-015-AC-7, FR-015-AC-8, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-3, TC-045
#[test]
fn tc_025_each_clause_lowers_to_a_separate_harness_with_exact_correspondence() {
    let package = bound_package(1000);
    let harnesses = supported_contract_harnesses(&package, "crate::withdraw");
    assert_eq!(harnesses.len(), 3);
    let expected = [
        (PRECONDITION, ObligationKind::Precondition, 10, 0),
        (POSTCONDITION, ObligationKind::Postcondition, 20, 1),
        (INVARIANT, ObligationKind::Invariant, 30, 2),
    ];
    for (harness, (id, kind, line, requires)) in harnesses.iter().zip(expected) {
        let identity = &harness.identity;
        assert_eq!(identity.kind, kind);
        assert_eq!(identity.clause, clause(id));
        assert_eq!(identity.source_span.start().line(), line);
        assert_eq!(identity.oracles[0].clause, clause(id));
        let source = &harness.rust.contents;
        assert_eq!(source.matches("#[kani::requires(").count(), requires);
        let is_precondition = kind == ObligationKind::Precondition;
        assert_eq!(
            source.matches("#[kani::proof]").count(),
            usize::from(is_precondition)
        );
        assert_eq!(
            source.matches("#[kani::proof_for_contract(").count(),
            usize::from(!is_precondition)
        );
        assert_eq!(
            source.matches("#[kani::ensures(").count(),
            usize::from(!is_precondition)
        );
        // Every harness ends with exactly one non-vacuity cover.
        assert_eq!(source.matches("kani::cover!(").count(), 1);
        if !is_precondition {
            let call = source.find("let _ = ").unwrap();
            let cover = source.find("kani::cover!(true, ").unwrap();
            assert!(cover > call, "the cover follows the contract call");
        }
        assert_eq!(identity.subject_path.is_some(), !is_precondition);
    }
    // The contract harnesses assume exactly the precondition sharing their anchor, and no
    // harness embeds any other obligation's oracle.
    let precondition_symbol = &harnesses[0].identity.oracles[0].symbol;
    for harness in &harnesses[1..] {
        let oracles = &harness.identity.oracles;
        assert_eq!(oracles.len(), 2);
        assert_eq!(oracles[1].clause, clause(PRECONDITION));
        assert_eq!(oracles[1].kind, ObligationKind::Precondition);
        assert!(harness.rust.contents.contains(&format!(
            "#[kani::requires({precondition_symbol}(amount_current, balance_pre))]"
        )));
    }
    for (index, harness) in harnesses.iter().enumerate() {
        for (other_index, other) in harnesses.iter().enumerate() {
            let other_symbol = &other.identity.oracles[0].symbol;
            let embeds = harness.rust.contents.contains(other_symbol.as_str());
            assert_eq!(
                embeds,
                other_index == index || other_index == 0,
                "{index} embeds {other_index}"
            );
        }
    }

    // Without its precondition a postcondition is refused rather than proved under fewer
    // assumptions than the contract states.
    let post = clause(POSTCONDITION);
    let items = [ObligationItem::BoundClause {
        package: &package,
        clause: &post,
    }];
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::withdraw")).unwrap());
    assert!(harnesses.is_empty());
    assert_eq!(records[0].kind, Some(ObligationKind::Postcondition));
    assert_eq!(
        unsupported(&records[0]),
        &UnsupportedObligation::PreconditionNotNegotiated {
            precondition: clause(PRECONDITION)
        }
    );
    assert_eq!(
        records[0].subject,
        ObligationSubject::BoundClause {
            clause: post,
            source_span: Some(
                package
                    .clauses()
                    .iter()
                    .find(|candidate| candidate.identity() == &clause(POSTCONDITION))
                    .unwrap()
                    .source()
                    .clone()
            ),
        }
    );
}

/// The precondition harness's non-vacuity cover argument is the `precondition_holds` binding,
/// and that binding is itself the precondition oracle applied to the harness's own symbolic
/// arguments — not a literal. A regression to `kani::cover!(true, ...)`, or to a
/// `precondition_holds` that is not derived from the oracle call, makes this fail: the cover
/// would then be trivially reachable regardless of whether the precondition is satisfiable.
///
/// Trace: FR-015-AC-7, TC-025
#[test]
fn tc_025_precondition_cover_argument_is_derived_from_the_precondition_expression() {
    let package = bound_package(1000);
    let harnesses = supported_contract_harnesses(&package, "crate::withdraw");
    let precondition = &harnesses[0];
    assert_eq!(precondition.identity.kind, ObligationKind::Precondition);
    let source = &precondition.rust.contents;
    let symbol = &precondition.identity.oracles[0].symbol;

    // The cover's argument names the `precondition_holds` binding, never a constant.
    assert!(
        source.contains("kani::cover!(precondition_holds, "),
        "cover must be gated on the precondition_holds binding: {source}"
    );
    assert!(
        !source.contains("kani::cover!(true, "),
        "cover must not be trivially reachable: {source}"
    );

    // `precondition_holds` is bound to the precondition oracle applied to the harness's
    // symbolic arguments, not to a constant. This pins the identifier order rather than
    // merely asserting the oracle symbol occurs somewhere in the source.
    let expected_binding =
        format!("let precondition_holds = {symbol}(amount_current, balance_pre);");
    assert!(
        source.contains(&expected_binding),
        "expected {expected_binding:?} in {source}"
    );
    assert!(
        !source.contains("let precondition_holds = true;"),
        "precondition_holds must not be a hardcoded literal: {source}"
    );
}

/// Symbolic ranges are the IR's inclusive domains, and every flag is part of the harness
/// identity.
///
/// Trace: FR-015-AC-2, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-1, TC-045
#[test]
fn tc_025_symbolic_bounds_equal_ir_domains_and_every_option_is_identity() {
    let package = bound_package(1000);
    let harnesses = supported_contract_harnesses(&package, "crate::withdraw");
    let postcondition = &harnesses[1];
    let identity = &postcondition.identity;
    let bounds = identity
        .arguments
        .iter()
        .chain(&identity.results)
        .map(|binding| {
            let bounds = binding.integer_bounds.as_ref().unwrap();
            (binding.identifier.as_str(), bounds.minimum, bounds.maximum)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        bounds,
        [
            ("amount_current", 0, 1000),
            ("balance_pre", 0, 1000),
            ("balance_post", 0, 1000)
        ]
    );
    let source = &postcondition.rust.contents;
    assert!(source.contains("kani::assume(amount_current >= 0_i64 && amount_current <= 1000_i64);"));
    assert!(source.contains("kani::assume(balance_pre >= 0_i64 && balance_pre <= 1000_i64);"));
    assert!(source.contains("|post_state: &i64| (*post_state >= 0_i64 && *post_state <= 1000_i64)"));
    // The unwind bound reaches the backend through the option vector alone; no harness source
    // states a bound of its own, so the identity is the only place it can be read from.
    for harness in &harnesses {
        assert!(!harness.rust.contents.contains("kani::unwind"));
    }
    assert_eq!(identity.solver, KaniSolver::Cadical);
    assert_eq!(identity.unwind, 4);
    let exact = format!("{}::{}", identity.module_symbol, identity.harness_symbol);
    // The complete ordered option vector, not merely a set of substrings present somewhere in
    // it: order, length and completeness are all part of FR-015-AC-9, so a truncated or
    // reordered vector must fail here even though every flag it kept would still pass a
    // membership check.
    let expected_options: Vec<String> = [
        "-Z",
        "function-contracts",
        "-Z",
        "concrete-playback",
        "--harness",
        exact.as_str(),
        "--exact",
        "--unwind",
        "4",
        "--solver",
        "cadical",
        "--output-format",
        "regular",
        "--concrete-playback",
        "print",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(identity.options, expected_options);
    // A stub is an assumption this obligation path does not admit, so no option enables one.
    assert!(
        !identity
            .options
            .iter()
            .any(|option| option.contains("stub")),
        "{:?}",
        identity.options
    );

    let refs = [clause(PRECONDITION), clause(POSTCONDITION)];

    // The unwind bound and the subject change the identity.
    let seen = [postcondition.identity.clone()];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let mut unwound = request(&items, "crate::withdraw");
    unwound.unwind = 5;
    let (_, other) = emitted(negotiate_kani_obligations(&unwound).unwrap());
    assert!(!seen.contains(&other[1].identity));
    let (_, other) = emitted(negotiate_kani_obligations(&request(&items, "crate::other")).unwrap());
    assert!(!seen.contains(&other[1].identity));
    // Regeneration is byte-identical.
    assert_eq!(
        supported_contract_harnesses(&package, "crate::withdraw"),
        harnesses
    );

    // A V2 claim's domain is read from its IR bound, inclusive at both ends. Node 1001 is
    // `quire.op.integer.add`, IR-confirmed and renderable, so it reaches a real Kani harness --
    // not a typed refusal. Both its operands are references to the corpus's `value` node whose
    // body is the literal `3`, so each is a constant and ranged at its own value (IR-302); the
    // per-operand bounded ranges are TC-033's.
    let (scalar, claim_map, ids) = scalar_package();
    let node_1001 = ids.resolve(&code_id(1001));
    let items = [ObligationItem::ScalarClaim {
        package: &scalar,
        claim_map: &claim_map,
        node_id: &node_1001,
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
///
/// Trace: FR-015-AC-37, FR-015-AC-7, TC-025
#[test]
fn tc_025_scalar_harness_asserts_the_native_arithmetic_relation() {
    let (scalar, claim_map, ids) = scalar_package();
    let node_1001 = ids.resolve(&code_id(1001));
    let items = [ObligationItem::ScalarClaim {
        package: &scalar,
        claim_map: &claim_map,
        node_id: &node_1001,
    }];
    let (_, scalar_harnesses) =
        emitted_scalar(negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap());
    let source = &scalar_harnesses[0].rust.contents;
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
///
/// Trace: FR-015-AC-37, TC-025
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
    let items = generated
        .iter()
        .map(|claim| ObligationItem::ScalarClaim {
            package: &scalar,
            claim_map: &claim_map,
            node_id: &claim.node_id,
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
/// `KaniObligationHarness`: an item this test refuses never produces one, so there is no way to
/// report the refusal as an execution outcome (`KaniRunOutcome`) instead of what it is.
///
/// Trace: FR-015-AC-3, FR-017-CON-2, TC-025, TC-027
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-2, TC-045
#[test]
fn tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses() {
    let (scalar, claim_map, ids) = scalar_package();
    let records = scalar_records(
        &scalar,
        &claim_map,
        &ids,
        &[UNBOUNDED, MISSING_ROUNDING, MODEL, STATE, FRAME],
    );
    assert_eq!(
        records[0].disposition,
        ObligationDisposition::RequiresBound {
            unbounded_type: ids.resolve(&code_id(T_INTEGER))
        }
    );
    assert!(matches!(
        records[1].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    assert_eq!(
        unsupported(&records[2]),
        &UnsupportedObligation::BlockedOnUpstream {
            node_id: ids.resolve(&code_id(MODEL)),
            node_tag: "model",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }
    );
    assert_eq!(
        unsupported(&records[3]),
        &UnsupportedObligation::NoFiniteEncoding {
            node_id: ids.resolve(&code_id(STATE)),
            node_tag: "state",
        }
    );
    assert_eq!(records[4].kind, Some(ObligationKind::Frame));
    assert!(ids.resolve(&code_id(FRAMED_OBJECT)) < ids.resolve(&code_id(FRAME)));
    // The frame is typed by FRAMED_OBJECT. IR names that model first in admitted node-id
    // order, before the rekeyed frame node, so this item carries the model blocker.
    assert_eq!(
        unsupported(&records[4]),
        &UnsupportedObligation::BlockedOnUpstream {
            node_id: ids.resolve(&code_id(FRAMED_OBJECT)),
            node_tag: "model",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }
    );
    assert!(matches!(
        &records[4].subject,
        ObligationSubject::CheckedNode { node_id, source_map } if node_id == &ids.resolve(&code_id(FRAME)) && !source_map.is_empty()
    ));

    let package = bound_package(1000);
    let refs = [clause(DEFINEDNESS), clause(ASSERTION)];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::deposit")).unwrap());
    assert!(harnesses.is_empty());
    assert!(matches!(
        unsupported(&records[0]),
        UnsupportedObligation::DefinednessNotEncoded { obligations } if *obligations > 0
    ));
    assert_eq!(records[1].kind, None);
    assert_eq!(
        unsupported(&records[1]),
        &UnsupportedObligation::ClauseKindNotObligation {
            kind: ClauseKind::Assertion
        }
    );
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
/// drives the gap directly: `transition`
/// is a real, admitted `state` form distinct from `frame` (`quire-contract-ir`'s own
/// `CheckedNodeTag::State::forms()`), and its claim-map entry is hand-appended as `Generated`,
/// the shape `generate_exact_scalar_oracles` would never itself produce for a non-`expression`
/// node, to simulate what a future widening of that gate -- or a claim map assembled by another
/// caller -- could hand this function.
///
/// Trace: FR-015-AC-14, TC-025
#[test]
fn tc_025_a_present_node_with_an_unrecognized_kind_is_refused_rather_than_silently_supported() {
    const UNRECOGNIZED_STATE_FORM: u32 = 3003;
    let mut builder = corpus_package();
    builder.code(
        UNRECOGNIZED_STATE_FORM,
        "state",
        "transition",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
    );
    let (package, ids) = builder.admit_resolved();
    let node_id = ids.resolve(&code_id(UNRECOGNIZED_STATE_FORM));

    let oracles = generate_exact_scalar_oracles(&package, &golden_items()).expect("claim map");
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
        node_id: node_id.clone(),
        operation,
        result: ClaimDisposition::Generated(generated),
    });

    let items = [ObligationItem::ScalarClaim {
        package: &package,
        claim_map: &claim_map,
        node_id: &node_id,
    }];
    let outcome = negotiate_kani_obligations(&request(&items, "crate::subject")).unwrap();
    let KaniObligationOutcome::Emitted {
        records,
        harnesses,
        scalar_harnesses,
        ..
    } = outcome
    else {
        panic!("unexpected rejection");
    };
    assert!(
        harnesses.is_empty(),
        "an unrecognized node kind must emit no V1 harness"
    );
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
            semantic_form: "transition".to_owned(),
        }
    );
}

/// A scalar refusal of the byte ceiling or of an unrecognised lowering limit kind is reported as
/// `OracleRefused` carrying that refusal unchanged, field for field, as a work-ceiling refusal
/// is: never another `UnsupportedObligation` variant and never `LoweringWorkExhausted`. Each
/// refusal is placed in a claim map by hand, as `ClaimMap` and `ExactScalarClaim` are fully
/// `pub`; the generator's own byte-ceiling refusal is FR-014-AC-40's.
///
/// Trace: FR-015-AC-50, TC-025
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
///
/// Trace: FR-015-AC-14, TC-025
#[test]
fn tc_025_a_claim_naming_a_node_absent_from_the_graph_is_refused_not_silently_supported() {
    const NODE_ABSENT_FROM_GRAPH: u32 = 8888;
    let package = corpus_package().admit();
    let missing_node_id = id(&key(NODE_ABSENT_FROM_GRAPH));

    let oracles = generate_exact_scalar_oracles(&package, &golden_items()).expect("claim map");
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

    let items = [ObligationItem::ScalarClaim {
        package: &package,
        claim_map: &claim_map,
        node_id: &missing_node_id,
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

/// The only assumptions are the IR bounds of symbolic arguments; nothing constrains results,
/// and clause predicates appear only as the contract's own requires and ensures.
///
/// Trace: FR-015-AC-4, TC-025
#[test]
fn tc_025_assumptions_constrain_only_arguments_to_their_ir_bounds() {
    let package = bound_package(1000);
    for harness in supported_contract_harnesses(&package, "crate::withdraw") {
        let identity = &harness.identity;
        let assumptions = harness
            .rust
            .contents
            .lines()
            .map(str::trim)
            .filter(|line| line.contains("kani::assume"))
            .collect::<Vec<_>>();
        let expected = identity
            .arguments
            .iter()
            .map(|binding| {
                let bounds = binding.integer_bounds.as_ref().unwrap();
                format!(
                    "kani::assume({id} >= {}_i64 && {id} <= {}_i64);",
                    bounds.minimum,
                    bounds.maximum,
                    id = binding.identifier
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(assumptions, expected, "{}", harness.rust.contents);
        for result in &identity.results {
            assert!(!assumptions
                .iter()
                .any(|line| line.contains(&result.identifier)));
        }
        let requires = harness
            .rust
            .contents
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("#[kani::requires("))
            .count();
        let expected_requires = match identity.kind {
            ObligationKind::Precondition => 0,
            ObligationKind::Postcondition => identity.oracles.len() - 1,
            ObligationKind::Invariant => identity.oracles.len(),
            ObligationKind::Frame => unreachable!("no frame harness"),
        };
        assert_eq!(requires, expected_requires);
    }
}

/// Preconditions that each hold but cannot hold together are all assumed by the contract
/// harness, which therefore ends with a cover reachable only when every `requires` and argument
/// bound holds at once. Whether it is satisfied is decided by Kani (the kani lane reports this
/// harness `CoverUnsatisfied`); here the harness must carry the cover after the call and embed
/// both preconditions, so nothing about it can be reported verified without that cover.
///
/// Trace: FR-015-AC-4, FR-015-AC-5, FR-015-AC-7, FR-015-AC-8, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-2, TC-045
#[test]
fn tc_025_jointly_unsatisfiable_preconditions_leave_a_cover_that_decides_vacuity() {
    let package = operation_group_package(1000);
    let harness = transfer_harness(&package);
    let source = &harness.rust.contents;
    let requires = source
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("#[kani::requires("))
        .collect::<Vec<_>>();
    assert_eq!(requires.len(), 2, "{source}");
    let assumed = harness.identity.oracles[1..]
        .iter()
        .map(|oracle| oracle.clause.clone())
        .collect::<Vec<_>>();
    // In the package's canonical clause order.
    assert_eq!(assumed, [clause(TRANSFER_LARGE), clause(TRANSFER_SMALL)]);
    let body = &source[source.find("#[kani::proof_for_contract(").unwrap()..];
    let call = body.find("let _ = ").unwrap();
    let cover = body
        .find("kani::cover!(true, \"contract requires and IR bounds are jointly satisfiable\");")
        .unwrap();
    assert!(call < cover, "{body}");
}

fn transfer_harness(package: &BoundPackage) -> KaniObligationHarness {
    let refs = [
        clause(TRANSFER_SMALL),
        clause(TRANSFER_LARGE),
        clause(TRANSFER_IMPOSSIBLE),
    ];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause { package, clause })
        .collect::<Vec<_>>();
    let (records, mut harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::transfer")).unwrap());
    assert!(records
        .iter()
        .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })));
    harnesses.remove(2)
}

/// Contract obligations on one operation share one subject signature, the union of their
/// slots, even when one reads an input the other does not; a union that gives one slot two
/// bounds refuses every obligation in the group with a typed reason and no harness.
///
/// Trace: FR-015-AC-1, TC-025
#[test]
fn tc_025_contract_harnesses_on_one_operation_share_one_subject_signature() {
    let package = operation_group_package(1000);
    let refs = [clause(REFUND_CAPPED), clause(REFUND_NONNEGATIVE)];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let (_, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::refund")).unwrap());
    assert_eq!(harnesses.len(), 2);
    let signature = |harness: &KaniObligationHarness| {
        (
            harness
                .identity
                .arguments
                .iter()
                .map(|binding| binding.identifier.clone())
                .collect::<Vec<_>>(),
            harness
                .identity
                .results
                .iter()
                .map(|binding| binding.identifier.clone())
                .collect::<Vec<_>>(),
        )
    };
    let expected = (
        vec!["balance_pre".to_owned(), "fee_current".to_owned()],
        vec!["balance_post".to_owned()],
    );
    assert_eq!(signature(&harnesses[0]), expected);
    assert_eq!(signature(&harnesses[1]), expected);
    for harness in &harnesses {
        assert!(harness
            .rust
            .contents
            .contains("crate::refund(balance_pre, fee_current)"));
    }
    // Alone, the postcondition's signature would omit the invariant's pre-state argument.
    let alone = [ObligationItem::BoundClause {
        package: &package,
        clause: &refs[0],
    }];
    let (_, harnesses) =
        emitted(negotiate_kani_obligations(&request(&alone, "crate::refund")).unwrap());
    assert_eq!(
        signature(&harnesses[0]),
        (
            vec!["fee_current".to_owned()],
            vec!["balance_post".to_owned()]
        )
    );

    // The invariant binds `balance` to 0..=999 and the postcondition to 0..=1000: each alone is
    // supported, together they are refused.
    let conflicting = operation_group_package(999);
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &conflicting,
            clause,
        })
        .collect::<Vec<_>>();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, "crate::refund")).unwrap());
    assert!(harnesses.is_empty());
    for record in &records {
        assert_eq!(
            unsupported(record),
            &UnsupportedObligation::SubjectSignatureConflict {
                operation: "refund".to_owned(),
                identifier: "balance_post".to_owned(),
            }
        );
    }
    for clause in &refs {
        let alone = [ObligationItem::BoundClause {
            package: &conflicting,
            clause,
        }];
        let (_, harnesses) =
            emitted(negotiate_kani_obligations(&request(&alone, "crate::refund")).unwrap());
        assert_eq!(harnesses.len(), 1);
    }
}

/// A bound whose lower limit exceeds its upper limit is refused, not emitted as an empty proof.
///
/// Trace: FR-015-AC-5, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-2, TC-045
#[test]
fn tc_025_unsatisfiable_bounds_are_refused() {
    let (scalar, claim_map, ids) = scalar_package();
    let records = scalar_records(&scalar, &claim_map, &ids, &[UNSATISFIABLE]);
    assert_eq!(
        unsupported(&records[0]),
        &UnsupportedObligation::UnsatisfiableBound {
            bound: package::id(&Bound::Integer(5, -5).key()),
            form: "integer_range".to_owned(),
            lower: "5".to_owned(),
            upper: "-5".to_owned(),
        }
    );
    // V1 integer types cannot carry an inverted range: IR refuses the projection itself.
    let inverted = serde_json::to_vec(&projection(-1)).unwrap();
    assert!(BoundPackage::from_json_bytes(&inverted).is_err());
}

/// Every V2 scalar claim FR-014 generates has an IR-confirmed operation. The four
/// `IntegerArithmetic` identities this generator knows how to render (`quire.op.integer.{add,sub,
/// mul,negate}`) each reach a real Kani harness; every other confirmed family is refused as
/// `OperationNotRendered` -- known, honestly-named debt, never the old `CallerDeclaredOperation`
/// lie (that provenance is unreachable here: every one of these claims' nodes was successfully
/// lowered and checked, from an admitted package whose IR already confirmed the identity).
///
/// Trace: FR-015-AC-6, TC-025
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
    let items = generated
        .iter()
        .map(|claim| ObligationItem::ScalarClaim {
            package: &scalar,
            claim_map: &claim_map,
            node_id: &claim.node_id,
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
///
/// Trace: FR-015-AC-6, TC-025
#[test]
fn tc_025_a_caller_declared_operation_is_refused_with_no_harness() {
    let package = corpus_package().admit();
    let mismatched = code_id(1004);
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
    let items = [ObligationItem::ScalarClaim {
        package: &package,
        claim_map: &oracles.claim_map,
        node_id: &mismatched,
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
/// too: a rejected `negotiate_kani_obligations` call returns no `KaniObligationHarness` at all,
/// so `execute_kani_obligation` has nothing to run and nothing to misreport as an execution
/// outcome in place of the refusal it actually is.
///
/// Trace: FR-015-AC-12, FR-017-CON-2, TC-025, TC-027
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-4, TC-045
#[test]
fn tc_025_every_item_is_accounted_before_any_harness_is_exposed() {
    let package = bound_package(1000);
    let other_package = bound_package(999);
    let (scalar, claim_map, ids) = scalar_package();
    let refs = [
        clause(PRECONDITION),
        clause(POSTCONDITION),
        clause("no-such-clause"),
        clause(DEFINEDNESS),
    ];
    let missing = code_id(MISSING);
    let unbounded = ids.resolve(&code_id(UNBOUNDED));
    let items = [
        ObligationItem::BoundClause {
            package: &package,
            clause: &refs[0],
        },
        ObligationItem::BoundClause {
            package: &package,
            clause: &refs[1],
        },
        ObligationItem::BoundClause {
            package: &package,
            clause: &refs[0],
        },
        ObligationItem::BoundClause {
            package: &package,
            clause: &refs[2],
        },
        ObligationItem::BoundClause {
            package: &other_package,
            clause: &refs[3],
        },
        ObligationItem::ScalarClaim {
            package: &scalar,
            claim_map: &claim_map,
            node_id: &missing,
        },
        ObligationItem::ScalarClaim {
            package: &scalar,
            claim_map: &claim_map,
            node_id: &unbounded,
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
    assert!(matches!(
        records[1].disposition,
        ObligationDisposition::Supported { .. }
    ));
    let invalid = |index: usize| match &records[index].disposition {
        ObligationDisposition::InvalidRequest { reason } => reason.clone(),
        other => panic!("{index}: {other:?}"),
    };
    assert_eq!(
        invalid(2),
        InvalidObligationItem::DuplicateItem { first_index: 0 }
    );
    assert_eq!(invalid(3), InvalidObligationItem::UnknownClause);
    assert_eq!(invalid(4), InvalidObligationItem::MixedBoundPackages);
    assert_eq!(invalid(5), InvalidObligationItem::UnknownNode);
    assert!(matches!(
        records[6].disposition,
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
    let items = [ObligationItem::ScalarClaim {
        package: &scalar,
        claim_map: &foreign_map,
        node_id: &node,
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
///
/// Trace: FR-017-AC-2, TC-027
#[test]
fn tc_027_a_missing_launcher_is_refused_before_anything_runs() {
    let package = bound_package(1000);
    let harness = supported_contract_harnesses(&package, "crate::withdraw").remove(1);
    let directory = write_crate(&harness, HEALTHY_SUBJECT);
    let installation = KaniInstallation {
        launcher: directory.join("cargo-kani"),
    };
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: &directory,
        target_directory: &directory.join("target"),
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
    let _ = fs::remove_dir_all(directory);
}

/// Invalid launcher file kinds and execute permissions are typed faults before dispatch.
///
/// Trace: TC-027
#[cfg(unix)]
#[test]
fn tc_027_non_executable_launchers_are_refused_before_anything_runs() {
    use std::os::unix::fs::PermissionsExt;

    let package = bound_package(1000);
    let harness = supported_contract_harnesses(&package, "crate::withdraw").remove(1);
    for directory_launcher in [false, true] {
        let directory = write_crate(&harness, HEALTHY_SUBJECT);
        let launcher = directory.join("cargo-kani");
        if directory_launcher {
            fs::create_dir(&launcher).unwrap();
        } else {
            fs::write(&launcher, "#!/bin/sh\nexit 0\n").unwrap();
            fs::set_permissions(&launcher, fs::Permissions::from_mode(0o644)).unwrap();
        }
        let installation = KaniInstallation { launcher };
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            installation: &installation,
            harness: (&harness).into(),
            crate_directory: &directory,
            target_directory: &directory.join("target"),
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
        fs::remove_dir_all(directory).unwrap();
    }
}

// ---- kani lane ---------------------------------------------------------------

pub(crate) fn scratch(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "quire-kani-obligations-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(path.join("src")).unwrap();
    path
}

const HEALTHY_SUBJECT: &str = "/// Withdraws `amount` from `balance`.\n#[must_use]\npub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n    balance_pre - amount_current\n}\n";
const SEEDED_FAILING_SUBJECT: &str = "/// Seeded defect: credits instead of debiting.\n#[must_use]\npub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n    balance_pre + amount_current\n}\n";
const VACUOUS_SUBJECT: &str = "/// Returns a balance no postcondition result satisfies.\n#[must_use]\npub fn transfer(amount_current: i64) -> i64 {\n    amount_current\n}\n";

pub(crate) fn write_crate(harness: &KaniObligationHarness, subject: &str) -> PathBuf {
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\n{subject}",
            harness.rust.contents
        ),
    )
    .unwrap();
    write_manifest(
        &directory,
        &format!(
            "[package]\nname = \"generated-kani-obligation\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    directory
}

fn run(
    installation: &KaniInstallation,
    harness: &KaniObligationHarness,
    subject: &str,
    evidence_directory: &Path,
    label: &str,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let crate_directory = write_crate(harness, subject);
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation,
        harness: harness.into(),
        crate_directory: &crate_directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-obligations"),
    })
    .unwrap_or_else(|refusal| panic!("{label}: {refusal}"));
    fs::write(
        evidence_directory.join(format!("{label}.json")),
        serde_json::to_string_pretty(&evidence).unwrap(),
    )
    .unwrap();
    fs::write(
        evidence_directory.join(format!("{label}.harness.rs")),
        &harness.rust.contents,
    )
    .unwrap();
    let _ = fs::remove_dir_all(crate_directory);
    evidence
}

/// Real Kani runs: the precondition, postcondition and invariant of a healthy subject verify
/// separately, a seeded defect is falsified with a concrete counterexample, jointly unsatisfiable
/// requires are reported vacuous rather than verified, and a real run given a budget it cannot
/// meet is reported timed out rather than left to block or misreported as `NoVerdict`.
///
/// Trace: FR-015-AC-1, FR-015-AC-4, TC-025, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7,
/// FR-017-CON-1, FR-017-CON-2, TC-027
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let evidence_directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-obligation-evidence");
    fs::create_dir_all(&evidence_directory).unwrap();
    let package = bound_package(1000);
    let harnesses = supported_contract_harnesses(&package, "crate::withdraw");

    for (harness, label) in harnesses
        .iter()
        .zip(["precondition", "postcondition", "invariant"])
    {
        let evidence = run(
            &installation,
            harness,
            HEALTHY_SUBJECT,
            &evidence_directory,
            &format!("verified-{label}"),
        );
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{label}");
        assert_eq!(evidence.exit_code, Some(0));
        assert_eq!(
            evidence.arguments[1..=harness.identity.options.len()],
            harness.identity.options[..]
        );
    }

    let evidence = run(
        &installation,
        &harnesses[1],
        SEEDED_FAILING_SUBJECT,
        &evidence_directory,
        "falsified-postcondition",
    );
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("the seeded defect must be falsified: {evidence:?}");
    };
    assert!(counterexample.contains("kani::concrete_playback_run"));
    assert!(counterexample.contains(harnesses[1].identity.harness_symbol.as_str()));
    assert_ne!(evidence.exit_code, Some(0));

    // Jointly unsatisfiable requires with a postcondition no result satisfies: every check
    // passes vacuously, and the cover after the contract call reports it.
    let group = operation_group_package(1000);
    let vacuous = transfer_harness(&group);
    let evidence = run(
        &installation,
        &vacuous,
        VACUOUS_SUBJECT,
        &evidence_directory,
        "vacuous-postcondition",
    );
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        },
        "jointly unsatisfiable requires must never verify"
    );

    // A budget a real run cannot meet is the typed timed-out inconclusive result, not
    // `NoVerdict` and not success: this harness has never been built before in this crate
    // directory, so 1ms cannot possibly be enough even to compile it, let alone run CBMC. The
    // proof that the run was actually killed, rather than merely misclassified after being
    // allowed to run to completion, is the mutation test against
    // `run_launcher_with_timeout`'s deadline check in `src/kani/run/launch.rs`: disabling that
    // check turns this same assertion red, because the run then completes for real and
    // verifies. This call's own wall-clock elapsed time is not that proof — a real run that
    // happened to finish quickly would satisfy an elapsed-time bound too — so none is asserted
    // here.
    let mut harness = harnesses[0].clone();
    harness.identity.ceilings.wall_clock = Duration::from_millis(1);
    let harness = &harness;
    let crate_directory = write_crate(harness, HEALTHY_SUBJECT);
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: harness.into(),
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
    })
    .unwrap_or_else(|refusal| {
        panic!("a run that started must not surface as a refusal: {refusal}")
    });
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    assert_eq!(evidence.exit_code, None);
    let _ = fs::remove_dir_all(crate_directory);

    // Evidence about a harness the crate does not contain is evidence about nothing.
    let harness = supported_contract_harnesses(&package, "crate::withdraw").remove(1);
    let crate_directory = write_crate(&harness, HEALTHY_SUBJECT);
    fs::write(
        crate_directory.join("src/lib.rs"),
        format!("//! Generated obligation check crate.\n\n{HEALTHY_SUBJECT}"),
    )
    .unwrap();
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::HarnessNotInCrate { .. }
    ));
    assert!(!crate_directory.join("target").exists(), "nothing ran");
    let _ = fs::remove_dir_all(crate_directory);
}

/// The routed scalar harness for `x + 1` over `Int[0, 9]`, and the oracle crate's `Cargo.toml`, both
/// exactly as `generate_routed` returns them: the inputs the driver assembles a run from.
fn routed_scalar_increment() -> (
    KaniScalarObligationHarness,
    quire_contract_codegen::Artifact,
) {
    let package = package::bounded_increment_package().admit();
    let node_id = package::code_id(package::BOUNDED_INCREMENT);
    let generation = generate_routed(
        &package,
        &[RoutedGenerationItem {
            request_index: 0,
            node_id,
            backend: Candidate {
                identity: "kani".to_owned(),
            },
            kind: BackendKind::Kani,
        }],
        &GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
                    REAL_KANI_TIMEOUT,
                ),
                subject_path: "crate::subject",
                unwind: 3,
            }),
        },
    )
    .expect("routed generation succeeds");
    let manifest = generation
        .oracle_artifacts
        .expect("a Kani group ran")
        .into_iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .expect("the oracle crate returns its manifest");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let KindOutput::Kani { harness, .. } = &item.output;
    (harness.clone().expect("the item is supported"), manifest)
}

/// Writes the crate the way the driver does: the returned `Cargo.toml` and `library` as
/// `src/lib.rs`.
fn write_scalar_crate(
    name: &str,
    manifest: &quire_contract_codegen::Artifact,
    library: &str,
) -> PathBuf {
    let directory = scratch(name);
    write_manifest(&directory, &manifest.contents);
    fs::write(directory.join("src/lib.rs"), library).unwrap();
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    directory
}

/// A routed scalar harness carries the same non-vacuity cover shape a contract harness does, so
/// a run classifies identically: every cover satisfied is verified, an unsatisfied one is
/// cover-unsatisfied, and success text with no cover summary is inconclusive.
///
/// Trace: FR-017-AC-4, FR-017-AC-11, TC-027
#[test]
fn tc_027_a_routed_scalar_harness_run_classifies_like_a_contract_harness() {
    let (scalar, _manifest) = routed_scalar_increment();
    let package = bound_package(1000);
    let contract = supported_contract_harnesses(&package, "crate::withdraw").remove(1);
    let covers = |source: &str| source.matches("kani::cover!(").count();
    assert_eq!(covers(&scalar.rust.contents), 1, "one non-vacuity cover");
    assert_eq!(
        covers(&contract.rust.contents),
        1,
        "the contract kind's shape"
    );
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
        classify_kani_run(true, Some(&report(checks)), "", None)
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
fn run_scalar_under_real_kani(
    name: &str,
    harness: &KaniScalarObligationHarness,
    manifest: &quire_contract_codegen::Artifact,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let crate_directory = write_scalar_crate(name, manifest, &harness.rust.contents);
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: harness.into(),
        crate_directory: &crate_directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-scalar"),
    })
    .unwrap_or_else(|refusal| panic!("{refusal}"));
    let _ = fs::remove_dir_all(&crate_directory);
    evidence
}

/// The routed `x + 1` over `Int[0, 9]` harness runs through the execution module in the crate the
/// driver assembles (`Cargo.toml` from `oracle_artifacts`, the harness source as `src/lib.rs`)
/// under the real Kani backend and is verified; its evidence carries the scalar identity. A
/// crate whose `src/lib.rs` lacks the harness is refused with no run.
///
/// Trace: FR-017-AC-7, FR-017-AC-11, TC-027
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_a_routed_scalar_harness_verifies() {
    let (harness, manifest) = routed_scalar_increment();
    let evidence = run_scalar_under_real_kani("scalar-real", &harness, &manifest);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.kind, None);
    assert_eq!(evidence.harness_path, harness.rust.path);
    assert_eq!(
        evidence.arguments[1..=harness.identity.options.len()],
        harness.identity.options[..]
    );
    assert_eq!(evidence.unwind, harness.identity.unwind);

    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let crate_directory = write_scalar_crate("scalar-real-missing", &manifest, "//! empty\n");
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::HarnessNotInCrate { .. }
    ));
    assert!(!crate_directory.join("target").exists(), "nothing ran");
    let _ = fs::remove_dir_all(crate_directory);
}

/// The same harness with its checked domain narrowed to `[0, 5]` asserts a bound the oracle does
/// not enforce (it completes up to 9), so the real backend falsifies it with a counterexample.
///
/// Trace: FR-017-AC-7, FR-017-AC-11, TC-027
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
///
/// Trace: FR-015-AC-37, TC-025, FR-017-AC-11
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
