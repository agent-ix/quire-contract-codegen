//! FR-015 separate bounded Kani obligations, FR-017 pinned execution, and Contract IR FR-036
//! backend negotiation.
//!
//! The default lane checks negotiation, refusal and harness shape without running Kani. The
//! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend, runs real
//! verified and seeded-failing harnesses one at a time, and writes execution evidence under
//! `CARGO_TARGET_TMPDIR/kani-obligation-evidence`.

#[path = "exact_scalar_support/package.rs"]
mod package;

use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use package::{
    application, code_id, corpus_package, golden_items, integer_add, key, reference, Bound,
    MISSING, MISSING_ROUNDING, MODEL, STATE, T_BOOLEAN, T_INTEGER, UNBOUNDED, V_INTEGER,
};
use quire_contract_codegen::{
    execute_kani_obligation, generate_exact_scalar_oracles, negotiate_kani_obligations,
    AttestationContext, DerivedDomain, ExactScalarClaimMap, ExactScalarDisposition,
    ExactScalarItem, InvalidObligationItem, KaniExecutionRefusal, KaniExecutionRequest,
    KaniInconclusiveReason, KaniInstallation, KaniObligationError, KaniObligationHarness,
    KaniObligationOutcome, KaniObligationRequest, KaniPinField, KaniRunOutcome, KaniTool,
    KaniToolError, KaniToolPins, ObligationDisposition, ObligationItem, ObligationKind,
    ObligationRecord, ObligationSubject, UnsupportedObligation, UpstreamBlocker,
    IR_CANDIDATE_REVISION, KANI_BACKEND_VERSION, KANI_OBLIGATION_PROFILE, MAX_OBLIGATION_ITEMS,
    MAX_OBLIGATION_UNWIND, RUNTIME_REVISION,
};
use quire_contract_ir::{
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

/// Budget for the pinned lane's real `cargo-kani` runs. Generous because CBMC is memory- and
/// time-heavy on these small obligations; this is a ceiling against a genuine hang, not a
/// performance target.
const REAL_KANI_TIMEOUT: Duration = Duration::from_secs(600);
/// Placeholder budget for tests that refuse before any process is spawned (a pin drift, a
/// missing backend component, or a harness the crate does not contain): the value is never
/// consulted, since `execute_kani_obligation` returns before reaching the launcher.
const UNUSED_TIMEOUT: Duration = Duration::from_secs(60);

fn context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

fn pins() -> KaniToolPins {
    KaniToolPins::pinned()
}

// ---- V1 fixture --------------------------------------------------------------

fn span(line: u64) -> Value {
    let source = json!({"document":"kani-obligations", "revision":1});
    json!({"start":{"source":source,"line":line,"column":1,"byte_offset":line - 1},
        "end":{"source":source,"line":line,"column":2,"byte_offset":line}})
}

fn int(minimum: i64, maximum: i64) -> Value {
    json!({"kind":"integer","domain":"signed","minimum":minimum,"maximum":maximum,"overflow":"reject"})
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
                "right":{"node":"integer_literal","value":0,"value_type":int(0, balance_maximum_in_invariant),"source":span(33)},
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
                    "right":{"node":"integer_literal","value":0,"value_type":int(0, 1000),"source":span(43)},
                    "source":span(42)},
                "right":{"node":"compare","operator":"less_equal",
                    "left":{"node":"numeric","operator":"divide",
                        "left":{"node":"integer_literal","value":1000,"value_type":int(0, 1000),"source":span(44)},
                        "right":read("amount", "current", 45),"source":span(44)},
                    "right":{"node":"integer_literal","value":1000,"value_type":int(0, 1000),"source":span(46)},
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
                "right":{"node":"integer_literal","value":0,"value_type":int(0, 1000),"source":span(53)},
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
    json!({"node":"integer_literal","value":value,"value_type":int(0, 1000),"source":span(line)})
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
                "right":{"node":"integer_literal","value":0,"value_type":int(0, refund_invariant_maximum),"source":span(103)},
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

fn bound_package(balance_maximum_in_invariant: i64) -> BoundPackage {
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
    pins: &'a KaniToolPins,
    subject_path: &'a str,
) -> KaniObligationRequest<'a> {
    KaniObligationRequest {
        items,
        subject_path,
        pins,
        unwind: 4,
        attestation: context(),
    }
}

fn emitted(outcome: KaniObligationOutcome) -> (Vec<ObligationRecord>, Vec<KaniObligationHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted { records, harnesses } => (records, harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

fn unsupported(record: &ObligationRecord) -> &UnsupportedObligation {
    match &record.disposition {
        ObligationDisposition::Unsupported { reason } => reason,
        other => panic!("expected unsupported, got {other:?}"),
    }
}

fn supported_contract_harnesses(
    package: &BoundPackage,
    pins: &KaniToolPins,
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
        emitted(negotiate_kani_obligations(&request(&items, pins, subject)).unwrap());
    assert!(records
        .iter()
        .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })));
    harnesses
}

// ---- V2 fixture --------------------------------------------------------------

/// An integer addition whose only bound admits no value.
const UNSATISFIABLE: u32 = 3001;
/// A frame clause.
const FRAME: u32 = 3002;

fn scalar_package() -> (CheckedPackageV2, ExactScalarClaimMap) {
    let mut builder = corpus_package();
    builder
        .bounded(
            UNSATISFIABLE,
            "expression",
            "binary",
            &key(T_INTEGER),
            application(
                "binary",
                vec![reference(&key(V_INTEGER)), reference(&key(V_INTEGER))],
            ),
            &[Bound::Integer(5, -5)],
        )
        .code(
            FRAME,
            "state",
            "frame",
            &key(T_BOOLEAN),
            json!({"term": "aggregate", "members": []}),
        );
    let package = builder.admit();
    let mut items = golden_items();
    for code in [UNSATISFIABLE, FRAME] {
        items.push(ExactScalarItem {
            node_id: code_id(code),
            operation: integer_add(),
        });
    }
    let oracles = generate_exact_scalar_oracles(&package, &items).expect("claim map");
    (package, oracles.claim_map)
}

fn scalar_records(
    package: &CheckedPackageV2,
    claim_map: &ExactScalarClaimMap,
    codes: &[u32],
) -> Vec<ObligationRecord> {
    let ids = codes.iter().map(|code| code_id(*code)).collect::<Vec<_>>();
    let items = ids
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package,
            claim_map,
            node_id,
        })
        .collect::<Vec<_>>();
    let pins = pins();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::subject")).unwrap());
    assert!(harnesses.is_empty(), "no V2 item may emit a harness");
    records
}

// ---- default lane ------------------------------------------------------------

/// Each precondition, postcondition and invariant clause is its own obligation, harness and
/// proof, with its clause, source span, IR digests and assumed preconditions recorded.
///
/// Trace: FR-015-AC-1, FR-015-AC-7, FR-015-AC-8, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-3, TC-045
#[test]
fn tc_025_each_clause_lowers_to_a_separate_harness_with_exact_correspondence() {
    let package = bound_package(1000);
    let pins = pins();
    let harnesses = supported_contract_harnesses(&package, &pins, "crate::withdraw");
    assert_eq!(harnesses.len(), 3);
    let expected = [
        (PRECONDITION, ObligationKind::Precondition, 10, 0),
        (POSTCONDITION, ObligationKind::Postcondition, 20, 1),
        (INVARIANT, ObligationKind::Invariant, 30, 2),
    ];
    let package_clauses = package.clauses();
    for (harness, (id, kind, line, requires)) in harnesses.iter().zip(expected) {
        let identity = &harness.identity;
        assert_eq!(identity.kind, kind);
        assert_eq!(identity.clause, clause(id));
        assert_eq!(identity.source_span.start().line(), line);
        let ir = package_clauses
            .iter()
            .find(|candidate| candidate.identity() == &clause(id))
            .unwrap();
        assert_eq!(
            identity.expression_digest,
            ir.expression_digest().to_string()
        );
        assert_eq!(
            identity.declaration_digest,
            ir.declaration_digest().to_string()
        );
        assert_eq!(identity.bound_package_digest, package.digest().to_string());
        assert_eq!(identity.adapter_profile, KANI_OBLIGATION_PROFILE);
        assert_eq!(identity.oracles[0].clause, clause(id));
        let source = &harness.rust.contents;
        assert!(source.contains(&format!(
            "// Obligation identity sha256: {}",
            harness.identity_sha256
        )));
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
        let record: Value = serde_json::from_str(&harness.record.contents).unwrap();
        assert_eq!(record["identitySha256"], harness.identity_sha256);
        assert_eq!(record["rustSha256"], harness.rust.sha256);
        assert_eq!(record["identity"]["kind"], json!(kind));
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
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::withdraw")).unwrap());
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
    let pins = pins();
    let harnesses = supported_contract_harnesses(&package, &pins, "crate::withdraw");
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

/// Symbolic ranges are the IR's inclusive domains, and every pin, flag and revision is part of
/// the harness identity.
///
/// Trace: FR-015-AC-2, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-1, TC-045
#[test]
fn tc_025_symbolic_bounds_equal_ir_domains_and_every_pin_is_identity() {
    let package = bound_package(1000);
    let pins = pins();
    let harnesses = supported_contract_harnesses(&package, &pins, "crate::withdraw");
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
    assert_eq!(identity.runtime_revision, RUNTIME_REVISION);
    assert_eq!(identity.ir_revision, IR_CANDIDATE_REVISION);
    assert_eq!(identity.pins, pins);
    assert_eq!(identity.solver, "cadical");
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

    // The committed pins are the identity's pins; any other backend is refused before
    // negotiation, field by field.
    assert_eq!(identity.pins, KaniToolPins::pinned());
    let refs = [clause(PRECONDITION), clause(POSTCONDITION)];
    let single = [ObligationItem::BoundClause {
        package: &package,
        clause: &refs[0],
    }];
    let digest = "3".repeat(64);
    for (field, value) in [
        (KaniPinField::KaniVersion, "0.68.0"),
        (KaniPinField::LauncherSha256, digest.as_str()),
        (KaniPinField::DriverSha256, digest.as_str()),
        (KaniPinField::CbmcVersion, "6.8.1"),
        (KaniPinField::RustToolchain, "nightly-2025-11-22"),
        (KaniPinField::TargetTriple, "aarch64-unknown-linux-gnu"),
    ] {
        let mut changed = pins.clone();
        let slot = match field {
            KaniPinField::KaniVersion => &mut changed.kani_version,
            KaniPinField::LauncherSha256 => &mut changed.launcher_sha256,
            KaniPinField::DriverSha256 => &mut changed.driver_sha256,
            KaniPinField::CbmcVersion => &mut changed.cbmc_version,
            KaniPinField::RustToolchain => &mut changed.rust_toolchain,
            KaniPinField::TargetTriple => &mut changed.target_triple,
        };
        *slot = value.to_owned();
        let Err(KaniObligationError::UnpinnedBackend {
            field: refused,
            expected,
            supplied,
        }) = negotiate_kani_obligations(&request(&single, &changed, "crate::withdraw"))
        else {
            panic!("{field:?} must be refused");
        };
        assert_eq!(refused, field);
        assert_ne!(expected, supplied);
    }

    // The unwind bound and the subject change the identity.
    let seen = [postcondition.identity_sha256.clone()];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let mut unwound = request(&items, &pins, "crate::withdraw");
    unwound.unwind = 5;
    let (_, other) = emitted(negotiate_kani_obligations(&unwound).unwrap());
    assert!(!seen.contains(&other[1].identity_sha256));
    let (_, other) =
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::other")).unwrap());
    assert!(!seen.contains(&other[1].identity_sha256));
    // Regeneration is byte-identical.
    assert_eq!(
        supported_contract_harnesses(&package, &pins, "crate::withdraw"),
        harnesses
    );

    // A V2 claim's domain is read from its IR bound, inclusive at both ends.
    let (scalar, claim_map) = scalar_package();
    let records = scalar_records(&scalar, &claim_map, &[1001]);
    let UnsupportedObligation::CallerDeclaredOperation {
        derived_domains, ..
    } = unsupported(&records[0])
    else {
        panic!("expected a caller-declared refusal: {records:?}");
    };
    assert!(matches!(
        derived_domains.as_slice(),
        [DerivedDomain::IntegerRange { lower, upper, .. }] if lower == "-1000" && upper == "1000"
    ));
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
    let (scalar, claim_map) = scalar_package();
    let records = scalar_records(
        &scalar,
        &claim_map,
        &[UNBOUNDED, MISSING_ROUNDING, MODEL, STATE, FRAME],
    );
    assert_eq!(
        records[0].disposition,
        ObligationDisposition::RequiresBound {
            unbounded_type: code_id(T_INTEGER)
        }
    );
    assert!(matches!(
        records[1].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    assert_eq!(
        unsupported(&records[2]),
        &UnsupportedObligation::BlockedOnUpstream {
            node_id: code_id(MODEL),
            node_tag: "model",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }
    );
    assert_eq!(
        unsupported(&records[3]),
        &UnsupportedObligation::NoFiniteEncoding {
            node_id: code_id(STATE),
            node_tag: "state",
        }
    );
    assert_eq!(records[4].kind, Some(ObligationKind::Frame));
    assert_eq!(
        unsupported(&records[4]),
        &UnsupportedObligation::NoFiniteEncoding {
            node_id: code_id(FRAME),
            node_tag: "state",
        }
    );
    assert!(matches!(
        &records[4].subject,
        ObligationSubject::CheckedNode { node_id, source_map } if node_id == &code_id(FRAME) && !source_map.is_empty()
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
    let pins = pins();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::deposit")).unwrap());
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

/// The only assumptions are the IR bounds of symbolic arguments; nothing constrains results,
/// and clause predicates appear only as the contract's own requires and ensures.
///
/// Trace: FR-015-AC-4, TC-025
#[test]
fn tc_025_assumptions_constrain_only_arguments_to_their_ir_bounds() {
    let package = bound_package(1000);
    let pins = pins();
    for harness in supported_contract_harnesses(&package, &pins, "crate::withdraw") {
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
    let pins = pins();
    let harness = transfer_harness(&package, &pins);
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

fn transfer_harness(package: &BoundPackage, pins: &KaniToolPins) -> KaniObligationHarness {
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
        emitted(negotiate_kani_obligations(&request(&items, pins, "crate::transfer")).unwrap());
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
    let pins = pins();
    let refs = [clause(REFUND_CAPPED), clause(REFUND_NONNEGATIVE)];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let (_, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::refund")).unwrap());
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
        emitted(negotiate_kani_obligations(&request(&alone, &pins, "crate::refund")).unwrap());
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
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::refund")).unwrap());
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
            emitted(negotiate_kani_obligations(&request(&alone, &pins, "crate::refund")).unwrap());
        assert_eq!(harnesses.len(), 1);
    }
}

/// A bound whose lower limit exceeds its upper limit is refused, not emitted as an empty proof.
///
/// Trace: FR-015-AC-5, TC-025
/// Upstream: agent-ix/quire-contract-ir FR-036-AC-2, TC-045
#[test]
fn tc_025_unsatisfiable_bounds_are_refused() {
    let (scalar, claim_map) = scalar_package();
    let records = scalar_records(&scalar, &claim_map, &[UNSATISFIABLE]);
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

/// Every V2 scalar claim FR-014 generates is caller-declared and refused, with no harness.
///
/// Trace: FR-015-AC-6, TC-025
#[test]
fn tc_025_every_caller_declared_operation_is_refused() {
    let (scalar, claim_map) = scalar_package();
    let generated = claim_map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_)))
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    assert!(generated.len() > 40, "the corpus generates every family");
    let items = generated
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &scalar,
            claim_map: &claim_map,
            node_id,
        })
        .collect::<Vec<_>>();
    let pins = pins();
    let (records, harnesses) =
        emitted(negotiate_kani_obligations(&request(&items, &pins, "crate::subject")).unwrap());
    assert!(harnesses.is_empty());
    assert_eq!(records.len(), generated.len());
    for (record, claim) in records.iter().zip(
        claim_map
            .items
            .iter()
            .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_))),
    ) {
        assert_eq!(
            unsupported(record),
            &UnsupportedObligation::CallerDeclaredOperation {
                operation_identity: claim.operation.identity.clone(),
                blocked_on: UpstreamBlocker::OperationIdentityNotCarried,
                derived_domains: match unsupported(record) {
                    UnsupportedObligation::CallerDeclaredOperation {
                        derived_domains, ..
                    } => derived_domains.clone(),
                    other => panic!("{other:?}"),
                },
            }
        );
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
    let (scalar, claim_map) = scalar_package();
    let pins = pins();
    let refs = [
        clause(PRECONDITION),
        clause(POSTCONDITION),
        clause("no-such-clause"),
        clause(DEFINEDNESS),
    ];
    let missing = code_id(MISSING);
    let unbounded = code_id(UNBOUNDED);
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
    let outcome = negotiate_kani_obligations(&request(&items, &pins, "crate::withdraw")).unwrap();
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
    let outcome = negotiate_kani_obligations(&request(&items, &pins, "crate::subject")).unwrap();
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
        let mut value = request(items, &pins, subject);
        value.unwind = unwind;
        assert_eq!(negotiate_kani_obligations(&value), Err(expected));
    }

    // The item ceiling is a request-level refusal too: nothing is accounted.
    let over_limit = vec![items[0]; MAX_OBLIGATION_ITEMS + 1];
    assert_eq!(
        negotiate_kani_obligations(&request(&over_limit, &pins, "crate::withdraw")),
        Err(KaniObligationError::TooManyItems {
            count: MAX_OBLIGATION_ITEMS + 1
        })
    );
}

/// A missing backend is a typed refusal before anything runs.
///
/// Trace: FR-017-AC-2, TC-027
#[test]
fn tc_027_an_unmeasurable_backend_is_refused_before_running() {
    let package = bound_package(1000);
    let pins = pins();
    let harness = supported_contract_harnesses(&package, &pins, "crate::withdraw").remove(1);
    let directory = scratch("missing-backend");
    let installation = KaniInstallation {
        launcher: directory.join("cargo-kani"),
        kani_home: directory.join("kani-home"),
    };
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: &harness,
        crate_directory: &directory,
        target_directory: &directory.join("target"),
        timeout: UNUSED_TIMEOUT,
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::Tool(KaniToolError::Missing {
            tool: KaniTool::Launcher,
            ..
        })
    ));
    let _ = fs::remove_dir_all(directory);
}

/// Writes an executable shell script at `path`, creating parent directories as needed.
fn write_executable(path: &Path, script: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, script).unwrap();
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

/// Builds a complete, working fake Kani installation: a launcher that answers
/// `kani --version`, and a release tree with a driver, CBMC, a toolchain file and a `rustc`
/// that answers `-vV`. Every component here measures successfully, so a test that wants to
/// exercise one component's failure overwrites or removes exactly that one component after
/// calling this, leaving the rest of the chain intact up to that point.
fn fake_installation(name: &str) -> (KaniInstallation, PathBuf) {
    let directory = scratch(name);
    let kani_home = directory.join("kani-home");
    let release = kani_home.join("kani-0.67.0");
    write_executable(
        &directory.join("cargo-kani"),
        "#!/bin/sh\necho 'cargo-kani 0.67.0'\n",
    );
    write_executable(&release.join("bin/kani-driver"), "#!/bin/sh\nexit 0\n");
    write_executable(
        &release.join("bin/cbmc"),
        "#!/bin/sh\necho '6.8.0 (cbmc-6.8.0)'\n",
    );
    fs::create_dir_all(&release).unwrap();
    fs::write(
        release.join("rust-toolchain-version"),
        "nightly-2025-11-21-x86_64-unknown-linux-gnu\n",
    )
    .unwrap();
    write_executable(
        &release.join("toolchain/bin/rustc"),
        "#!/bin/sh\necho 'host: x86_64-unknown-linux-gnu'\n",
    );
    let installation = KaniInstallation {
        launcher: directory.join("cargo-kani"),
        kani_home,
    };
    (installation, release)
}

/// Every backend component besides the launcher is refused with a typed reason naming that
/// component, and every `KaniToolError` kind besides `Missing` (already exercised above by the
/// launcher) is reachable: `Io` (a file that exists but cannot be read or executed), `Failed`
/// (a component that runs and exits unsuccessfully) and `UnexpectedOutput` (a component whose
/// output this module cannot parse). Each case starts from a fully working fake installation
/// and breaks exactly the one component under test, so the refusal is attributable to that
/// component and not to some other part of the chain failing first.
///
/// Trace: FR-017-AC-2, TC-027
#[test]
fn tc_027_every_backend_component_is_refused_with_its_own_typed_reason() {
    let package = bound_package(1000);
    let pins = pins();
    let harness = supported_contract_harnesses(&package, &pins, "crate::withdraw").remove(1);

    // RustToolchain, Missing: the release directory exists but its toolchain file does not.
    // Reached right after the launcher's version is read, before the driver or CBMC are ever
    // touched.
    {
        let (installation, release) = fake_installation("component-rust-toolchain-missing");
        fs::remove_file(release.join("rust-toolchain-version")).unwrap();
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            installation: &installation,
            harness: &harness,
            crate_directory: &installation.kani_home,
            target_directory: &installation.kani_home.join("target"),
            timeout: UNUSED_TIMEOUT,
        })
        .unwrap_err();
        assert!(
            matches!(
                refusal,
                KaniExecutionRefusal::Tool(KaniToolError::Missing {
                    tool: KaniTool::RustToolchain,
                    ..
                })
            ),
            "got {refusal}"
        );
        let _ = fs::remove_dir_all(installation.kani_home.parent().unwrap());
    }

    // Rustc, UnexpectedOutput: rustc runs and exits successfully, but prints no `host: ` line,
    // so its target triple cannot be read.
    {
        let (installation, release) = fake_installation("component-rustc-unexpected-output");
        write_executable(
            &release.join("toolchain/bin/rustc"),
            "#!/bin/sh\necho 'not the expected shape'\n",
        );
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            installation: &installation,
            harness: &harness,
            crate_directory: &installation.kani_home,
            target_directory: &installation.kani_home.join("target"),
            timeout: UNUSED_TIMEOUT,
        })
        .unwrap_err();
        assert!(
            matches!(
                refusal,
                KaniExecutionRefusal::Tool(KaniToolError::UnexpectedOutput {
                    tool: KaniTool::Rustc,
                    ..
                })
            ),
            "got {refusal}"
        );
        let _ = fs::remove_dir_all(installation.kani_home.parent().unwrap());
    }

    // Driver, Io: the driver exists as a regular file but is not readable, so hashing it fails
    // with an underlying I/O error rather than a missing-file refusal.
    {
        let (installation, release) = fake_installation("component-driver-io");
        let driver = release.join("bin/kani-driver");
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&driver, fs::Permissions::from_mode(0o000)).unwrap();
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            installation: &installation,
            harness: &harness,
            crate_directory: &installation.kani_home,
            target_directory: &installation.kani_home.join("target"),
            timeout: UNUSED_TIMEOUT,
        })
        .unwrap_err();
        fs::set_permissions(&driver, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(
            matches!(
                refusal,
                KaniExecutionRefusal::Tool(KaniToolError::Io {
                    tool: KaniTool::Driver,
                    ..
                })
            ),
            "got {refusal}"
        );
        let _ = fs::remove_dir_all(installation.kani_home.parent().unwrap());
    }

    // Cbmc, Failed: CBMC exists, is executable and runs, but exits unsuccessfully.
    {
        let (installation, release) = fake_installation("component-cbmc-failed");
        write_executable(&release.join("bin/cbmc"), "#!/bin/sh\nexit 7\n");
        let refusal = execute_kani_obligation(&KaniExecutionRequest {
            installation: &installation,
            harness: &harness,
            crate_directory: &installation.kani_home,
            target_directory: &installation.kani_home.join("target"),
            timeout: UNUSED_TIMEOUT,
        })
        .unwrap_err();
        assert!(
            matches!(
                refusal,
                KaniExecutionRefusal::Tool(KaniToolError::Failed {
                    tool: KaniTool::Cbmc,
                    ..
                })
            ),
            "got {refusal}"
        );
        let _ = fs::remove_dir_all(installation.kani_home.parent().unwrap());
    }
}

/// A harness identity that differs from the committed pins is refused before the backend is
/// ever measured. The installation here points at paths that do not exist, so if `observe()`
/// ran at all before the identity comparison, it would surface as `KaniToolError::Missing`
/// rather than as pin drift; getting `PinDrift` back is proof the backend was never touched.
///
/// Trace: FR-017-AC-1, TC-027
#[test]
fn tc_027_harness_identity_pin_drift_is_refused_before_the_backend_is_measured() {
    let package = bound_package(1000);
    let pins = pins();
    let mut harness = supported_contract_harnesses(&package, &pins, "crate::withdraw").remove(1);
    harness.identity.pins.driver_sha256 = "0".repeat(64);
    let directory = scratch("identity-pin-drift");
    let installation = KaniInstallation {
        launcher: directory.join("no-such-cargo-kani"),
        kani_home: directory.join("no-such-kani-home"),
    };
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: &harness,
        crate_directory: &directory,
        target_directory: &directory.join("target"),
        timeout: UNUSED_TIMEOUT,
    })
    .unwrap_err();
    assert!(
        matches!(
            refusal,
            KaniExecutionRefusal::PinDrift {
                field: KaniPinField::DriverSha256,
                ..
            }
        ),
        "the backend must not be measured before the identity pin check: got {refusal}"
    );
    assert!(!directory.join("target").exists(), "nothing ran");
    let _ = fs::remove_dir_all(directory);
}

/// FR-017's execution surface computes no aggregate verdict over runs and retains no evidence
/// of its own: no function in `src/kani_execution.rs` — the file FR-017 owns — takes more than
/// one run's evidence or outcome, and nothing in it writes a file. Retention, audit and
/// aggregation stay Quoin's; the caller receives one run's evidence and owns what happens to it.
///
/// This is a substring census, a tripwire and floor rather than a proof: it catches the literal
/// forms named below but not an equivalent rewrite, such as `impl IntoIterator<Item =
/// KaniExecutionEvidence>`, a `[KaniExecutionEvidence; 2]` array parameter, a type alias that
/// hides `Vec<...>` behind another name, `fs::copy` used in place of `fs::write`, `use std::fs::write
/// as emit`, or splitting the execution surface across a second module this test does not read.
/// Closing those gaps needs a stronger check than a grep; until then this test is the floor FR-017
/// stands on, not a guarantee nothing under it can shift.
///
/// Trace: FR-017-AC-8, FR-017-AC-9, TC-027
#[test]
fn tc_027_no_aggregate_verdict_and_no_retained_evidence_of_its_own() {
    let full_source =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/kani_execution.rs"))
            .expect("src/kani_execution.rs must be readable from the crate root");
    // Only the shipped execution surface, not its own `#[cfg(test)]` module: tests build fake
    // on-disk installations to exercise backend discovery and measurement, which is neither an
    // aggregate verdict nor evidence retention by the surface itself.
    let source = full_source
        .split_once("\n#[cfg(test)]\n")
        .map_or(full_source.as_str(), |(production, _)| production);

    // No aggregate verdict: nothing accepts a collection of runs' evidence or outcomes.
    for forbidden in [
        "Vec<KaniExecutionEvidence>",
        "&[KaniExecutionEvidence]",
        "Vec<KaniRunOutcome>",
        "&[KaniRunOutcome]",
    ] {
        assert!(
            !source.contains(forbidden),
            "src/kani_execution.rs must compute no aggregate verdict over runs: found {forbidden:?}"
        );
    }

    // No evidence of its own: nothing in this module writes a file. The backend process it
    // launches writes its own build artifacts; this module only ever reads them back.
    for forbidden in ["fs::write", "File::create", "OpenOptions"] {
        assert!(
            !source.contains(forbidden),
            "src/kani_execution.rs must retain no evidence of its own: found {forbidden:?}"
        );
    }
}

/// Concatenates every `.rs` file under `directory`, recursively, so a source census below can
/// find text that might live in any module rather than one hardcoded path.
fn concatenated_source(directory: &Path) -> String {
    let mut entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", directory.display()))
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    entries.sort();
    let mut combined = String::new();
    for path in entries {
        if path.is_dir() {
            combined.push_str(&concatenated_source(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            combined
                .push_str(&fs::read_to_string(&path).unwrap_or_else(|error| {
                    panic!("{} must be readable: {error}", path.display())
                }));
        }
    }
    combined
}

/// FR-017-CON-2 is backed by `tc_025_unbounded_non_finite_and_blocked_items_are_refused_without_harnesses`
/// and `tc_025_every_item_is_accounted_before_any_harness_is_exposed`, which are a sound but
/// type-level argument: every generation-time refusal (`ObligationDisposition`,
/// `UnsupportedObligation`, `KaniObligationError`, `KaniObligationOutcome`) produces no
/// `KaniObligationHarness`, so `execute_kani_obligation` structurally has nothing to convert.
/// Neither test calls `execute_kani_obligation` or constructs a `KaniRunOutcome`, so this census
/// — in the style of FR-017-AC-8/AC-9's own census above — is what would actually fail if a
/// future change added a conversion from the generation-time vocabulary to the execution-time
/// one (`KaniRunOutcome`, `KaniInconclusiveReason`, `KaniExecutionRefusal`,
/// `KaniExecutionEvidence`) and let a generation-time classification start reporting itself as
/// an execution outcome.
///
/// This is a substring census, a tripwire and floor rather than a proof: it catches the literal
/// `From<G> for E` forms named below but not an equivalent rewrite, such as an inherent
/// `impl UnsupportedObligation { fn into_outcome(self) -> KaniRunOutcome }`, a free function, a
/// `TryFrom` or `Into` implementation, or a rustfmt wrap that puts `for` on its own line. Closing
/// those gaps needs a stronger check than a grep; until then this test is the floor FR-017-CON-2
/// stands on, not a guarantee nothing under it can shift.
///
/// Trace: FR-017-CON-2, TC-027
#[test]
fn tc_027_no_conversion_exists_between_generation_and_execution_vocabularies() {
    let source = concatenated_source(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"));
    let generation_types = [
        "ObligationDisposition",
        "UnsupportedObligation",
        "KaniObligationError",
        "KaniObligationOutcome",
    ];
    let execution_types = [
        "KaniRunOutcome",
        "KaniInconclusiveReason",
        "KaniExecutionRefusal",
        "KaniExecutionEvidence",
    ];
    for generation in generation_types {
        for execution in execution_types {
            let forbidden = format!("From<{generation}> for {execution}");
            assert!(
                !source.contains(&forbidden),
                "FR-017-CON-2 forbids reporting a generation-time classification as an \
                 execution outcome, but found a conversion: {forbidden}"
            );
        }
    }
}

// ---- kani lane ---------------------------------------------------------------

fn scratch(name: &str) -> PathBuf {
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

fn write_crate(harness: &KaniObligationHarness, subject: &str) -> PathBuf {
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\n{subject}",
            harness.rust.contents
        ),
    )
    .unwrap();
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-kani-obligation\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .unwrap();
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
        harness,
        crate_directory: &crate_directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-obligations"),
        timeout: REAL_KANI_TIMEOUT,
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

/// Real pinned Kani runs: the precondition, postcondition and invariant of a healthy subject
/// verify separately under the committed backend pins, a seeded defect is falsified with a
/// concrete counterexample, jointly unsatisfiable requires are reported vacuous rather than
/// verified, drifted pins refuse before running, and a real run given a budget it cannot meet is
/// reported timed out rather than left to block or misreported as `NoVerdict`.
///
/// Trace: FR-015-AC-1, FR-015-AC-2, FR-015-AC-4, TC-025, FR-017-AC-1, FR-017-AC-3, FR-017-AC-4,
/// FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-CON-1, FR-017-CON-2, TC-027, FR-007-AC-3,
/// TC-023
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_pinned_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let pins = installation.observe().expect("the backend is measurable");
    assert_eq!(pins.kani_version, KANI_BACKEND_VERSION);
    assert_eq!(
        pins,
        KaniToolPins::pinned(),
        "the installed backend is the committed one"
    );
    let evidence_directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-obligation-evidence");
    fs::create_dir_all(&evidence_directory).unwrap();
    let package = bound_package(1000);
    let harnesses = supported_contract_harnesses(&package, &pins, "crate::withdraw");

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
        assert_eq!(evidence.observed_pins, pins);
        assert_eq!(evidence.exit_code, Some(0));
        assert_eq!(evidence.arguments[1..], harness.identity.options[..]);
        assert_eq!(evidence.oracle_digest, harness.identity.oracle_digest);
        assert_eq!(
            evidence.cargo_lock_sha256.as_ref().map(String::len),
            Some(64)
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
    assert!(counterexample.contains(&harnesses[1].identity.harness_symbol));
    assert_ne!(evidence.exit_code, Some(0));

    // Jointly unsatisfiable requires with a postcondition no result satisfies: every check
    // passes vacuously, and the cover after the contract call reports it.
    let group = operation_group_package(1000);
    let vacuous = transfer_harness(&group, &pins);
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
    // directory, so 1ms cannot possibly be enough even to compile it, let alone run CBMC.
    // Bounding the wall time this call itself takes is proof the run was actually killed rather
    // than merely misclassified after being allowed to run to completion. This runs ahead of the
    // `Cargo.lock`-as-directory case below on purpose: that case fails for a reason unrelated to
    // this change (agent-ix/quire-contract-codegen#58) on current `cargo`, independent of this
    // harness and independent of this branch (reproduced identically on `origin/main`), and a
    // later panic in the same test function must not prevent this assertion from running.
    let harness = &harnesses[0];
    let crate_directory = write_crate(harness, HEALTHY_SUBJECT);
    let started = Instant::now();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness,
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
        timeout: Duration::from_millis(1),
    })
    .unwrap_or_else(|refusal| {
        panic!("a run that started must not surface as a refusal: {refusal}")
    });
    assert!(
        started.elapsed() < REAL_KANI_TIMEOUT,
        "a timed-out run must not block for anywhere near a real verification's duration"
    );
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    assert_eq!(evidence.exit_code, None);
    assert_eq!(evidence.cargo_lock_sha256, None);
    let _ = fs::remove_dir_all(crate_directory);

    // A lockfile that cannot be read after the backend has already run is missing evidence
    // about that run, never grounds to discard its own verdict: the outcome below is still
    // this healthy subject's real `Verified` classification, not a pre-run refusal and not
    // degraded to inconclusive. `Cargo.lock` is occupied by a directory before the run, so the
    // launcher still runs (a real process starts) but the post-run digest read fails.
    let harness = &harnesses[0];
    let crate_directory = write_crate(harness, HEALTHY_SUBJECT);
    fs::create_dir_all(crate_directory.join("Cargo.lock")).unwrap();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness,
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
        timeout: REAL_KANI_TIMEOUT,
    })
    .unwrap_or_else(|refusal| {
        panic!("a run that already happened must not surface as a refusal: {refusal}")
    });
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.cargo_lock_sha256, None);
    let _ = fs::remove_dir_all(crate_directory);

    // A harness whose identity names another driver is refused before anything runs.
    let mut stale = supported_contract_harnesses(&package, &pins, "crate::withdraw").remove(1);
    stale.identity.pins.driver_sha256 = "0".repeat(64);
    let crate_directory = write_crate(&stale, HEALTHY_SUBJECT);
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: &stale,
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
        timeout: UNUSED_TIMEOUT,
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::PinDrift {
            field: KaniPinField::DriverSha256,
            ..
        }
    ));
    assert!(!crate_directory.join("target").exists(), "nothing ran");
    let _ = fs::remove_dir_all(crate_directory);

    // Evidence about a harness the crate does not contain is evidence about nothing.
    let harness = supported_contract_harnesses(&package, &pins, "crate::withdraw").remove(1);
    let crate_directory = write_crate(&harness, HEALTHY_SUBJECT);
    fs::write(
        crate_directory.join("src/lib.rs"),
        format!("//! Generated obligation check crate.\n\n{HEALTHY_SUBJECT}"),
    )
    .unwrap();
    let refusal = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: &harness,
        crate_directory: &crate_directory,
        target_directory: &crate_directory.join("target"),
        timeout: UNUSED_TIMEOUT,
    })
    .unwrap_err();
    assert!(matches!(
        refusal,
        KaniExecutionRefusal::HarnessNotInCrate { .. }
    ));
    assert!(!crate_directory.join("target").exists(), "nothing ran");
    let _ = fs::remove_dir_all(crate_directory);
}
