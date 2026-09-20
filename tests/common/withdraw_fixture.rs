//! The `withdraw` precondition/postcondition/invariant fixture, negotiated as a real bound
//! obligation rather than hand-built `KaniObligationHarness` values.
//!
//! `tests/kani_obligations.rs` and `tests/kani_witness_join.rs` each keep their own near-identical
//! copy of a `withdraw` fixture rather than using this one: touching either was out of scope for
//! the change that added this module (`tests/kani_witness_join.rs`'s copy in particular pins the
//! exact `amount_current`/`balance_pre` subject signature its seeded-defect subject function is
//! written against, so switching it here would require rewriting that subject too). This is the
//! version new tests should share.
//!
//! Unlike those two copies, the precondition clause here also declares a `priority` input with no
//! integer bounds, so every harness's argument list carries one `Boolean` binding (no
//! `kani::assume`) alongside the bounded `i64` ones — the shape `tests/kani_argument_order.rs`
//! needs to exercise the axis its check is uniquely sensitive to.

use quire_contract_codegen::{
    negotiate_kani_obligations, AttestationContext, KaniObligationHarness, KaniObligationOutcome,
    KaniObligationRequest, KaniToolPins, ObligationDisposition, ObligationItem, ObligationRecord,
    IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{
    BoundPackage, ClauseId, ClauseRef, RequirementRef, EXECUTABLE_PROJECTION_FORMAT,
};
use serde_json::{json, Value};

const PACKAGE: &str = "test/kani-argument-order";
const PRECONDITION: &str = "amount-within-balance";
const POSTCONDITION: &str = "balance-never-grows";
const INVARIANT: &str = "balance-nonnegative";

fn context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

fn span(line: u64) -> Value {
    let source = json!({"document":"kani-argument-order", "revision":1});
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

fn identity_ref(kind: &str, name: &str, observation: &str) -> Value {
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

/// An input with no bounds at all: `abi()` (`src/kani_obligations.rs`) gives a `Boolean`-typed
/// dependency `integer_bounds: None`, unlike every `Integer`-typed one in this fixture.
fn priority(line: u64) -> Value {
    json!({"name":"priority","kind":"input","value_type":{"kind":"boolean"},"source":span(line)})
}

fn clauses() -> Vec<ClauseFixture> {
    let pre = json!({"kind":"pre","operation":"withdraw"});
    vec![
        ClauseFixture {
            id: PRECONDITION,
            kind: "precondition",
            anchor: pre.clone(),
            line: 10,
            references: vec![
                identity_ref("input", "amount", "current"),
                identity_ref("state", "balance", "current"),
                identity_ref("input", "priority", "current"),
            ],
            values: vec![amount(11), balance(12, 1000), priority(15)],
            // `(amount <= balance) && priority`: `priority` contributes nothing to what the
            // precondition means (any bound would do), only an unbounded Boolean argument.
            expression: json!({"node":"boolean","operator":"short_circuit_and",
                "left":{"node":"compare","operator":"less_equal",
                    "left":read("amount", "current", 13),"right":read("balance", "current", 14),
                    "source":span(11)},
                "right":read("priority", "current", 16),
                "source":span(9)}),
        },
        ClauseFixture {
            id: POSTCONDITION,
            kind: "postcondition",
            anchor: json!({"kind":"post","operation":"withdraw"}),
            line: 20,
            references: vec![
                identity_ref("state", "balance", "post"),
                identity_ref("state", "balance", "pre"),
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
            references: vec![identity_ref("state", "balance", "current")],
            values: vec![balance(31, 1000)],
            expression: json!({"node":"compare","operator":"greater_equal",
                "left":read("balance", "current", 32),
                "right":{"node":"integer_literal","value":0,"value_type":int(0, 1000),"source":span(33)},
                "source":span(31)}),
        },
    ]
}

fn projection() -> Value {
    let fixtures = clauses();
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
            "source":{"document":"kani-argument-order","revision":1},
            "requirements":[{"id":"FR-200","revision":1,"source":span(1),"clauses":package_clauses}]},
        "bindings":bindings
    })
}

fn bound_package() -> BoundPackage {
    BoundPackage::from_json_bytes(&serde_json::to_vec(&projection()).unwrap())
        .unwrap_or_else(|diagnostics| panic!("fixture projection must bind: {diagnostics:?}"))
}

fn clause(id: &str) -> ClauseRef {
    ClauseRef::new(
        RequirementRef::parse(PACKAGE, "FR-200", 1).unwrap(),
        ClauseId::new(id).unwrap(),
    )
}

fn emitted(outcome: KaniObligationOutcome) -> (Vec<ObligationRecord>, Vec<KaniObligationHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted { records, harnesses } => (records, harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

/// The precondition, postcondition and invariant harnesses for `withdraw`, negotiated together so
/// the postcondition's and invariant's subject ABI is the union `unify_subject_signatures`
/// computes (three arguments: `amount_current`, `balance_pre`, `priority_current`).
pub fn withdraw_harnesses(pins: &KaniToolPins) -> Vec<KaniObligationHarness> {
    let package = bound_package();
    let refs = [
        clause(PRECONDITION),
        clause(POSTCONDITION),
        clause(INVARIANT),
    ];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let request = KaniObligationRequest {
        items: &items,
        subject_path: "crate::withdraw",
        pins,
        unwind: 4,
        attestation: context(),
    };
    let (records, harnesses) = emitted(negotiate_kani_obligations(&request).unwrap());
    assert!(
        records
            .iter()
            .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })),
        "{records:#?}"
    );
    harnesses
}
