//! IR-213: nothing coupled `symbolic_arguments`'s `kani::any()` emission order
//! (`src/kani_obligations.rs:1450`) to the order persisted in `identity.arguments`. That coupling
//! is what `decode_falsification` (`src/kani_witness_join.rs`) depends on: it binds
//! `identity.arguments` *positionally* onto Kani's concrete playback bytes, which is only sound
//! if position *i* of the persisted arguments is position *i* of the emitted `kani::any()` calls.
//! It holds today because both sites walk the exact same `Vec` once, with no filter and no
//! re-sort — but nothing enforced that; `witness_schema` takes a bare `&[ObligationBinding]`
//! slice and cannot observe it, so a future reorder or filter in `symbolic_arguments` would break
//! the join with zero red: arity still matches, widths still match, and Kani's own byte-vs-comment
//! cross-check validates bytes against a comment, never identifiers.
//!
//! This test negotiates a real obligation (no Kani backend: this is generated source text versus
//! the persisted schema, not a proof run), parses the generated harness's own
//! `let <id>: ... = kani::any();` bindings out of `harness.rust.contents` with `syn` — the same
//! parser `render()` itself uses to validate generated source before returning it — and asserts
//! their identifiers, in the order they appear in the generated source, equal
//! `identity.arguments`'s identifiers in order. It does not assume or hardcode which identifiers
//! `symbolic_arguments` happens to emit today; it recognizes only the `kani::any()` call shape
//! itself and reports whatever bindings are actually there.
//!
//! Trace: IR-213.

use quire_contract_codegen::{
    negotiate_kani_obligations, AttestationContext, KaniObligationHarness, KaniObligationOutcome,
    KaniObligationRequest, KaniToolPins, ObligationDisposition, ObligationItem, ObligationKind,
    ObligationRecord, IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{
    BoundPackage, ClauseId, ClauseRef, RequirementRef, EXECUTABLE_PROJECTION_FORMAT,
};
use serde_json::{json, Value};
use syn::{
    visit::{self, Visit},
    Expr, Local, Pat,
};

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

// ---- minimal V1 fixture: withdraw's precondition, postcondition and invariant ----------------
//
// Same shape as `tests/kani_witness_join.rs`'s own fixture (duplicated rather than shared: Rust
// integration test binaries do not share modules across files, and unlike that test this one
// never touches a real Kani backend).

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
/// the postcondition's subject ABI is the union `unify_subject_signatures` computes (two
/// arguments: `amount_current`, `balance_pre`).
fn withdraw_harnesses(pins: &KaniToolPins) -> Vec<KaniObligationHarness> {
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

// ---- generic, honest extraction of kani::any() bindings from generated source -----------------

/// Collects every `kani::any()`-bound identifier a parsed file contains, in the order `syn`
/// visits them (source order for a single straight-line block, which is what every generated
/// harness function body is).
#[derive(Default)]
struct KaniAnyBindings {
    identifiers: Vec<String>,
}

impl<'ast> Visit<'ast> for KaniAnyBindings {
    fn visit_local(&mut self, node: &'ast Local) {
        if let Some(identifier) = kani_any_binding_identifier(node) {
            self.identifiers.push(identifier);
        }
        visit::visit_local(self, node);
    }
}

/// Returns the bound identifier when `local` is exactly `let <id>: <ty> = kani::any();`: a
/// zero-argument call to the two-segment path `kani::any`, bound to a single typed identifier
/// pattern. Every other `let` — bound to some other call, or to an untyped or destructured
/// pattern — returns `None`. This does not know or assume any of the identifiers
/// `symbolic_arguments` happens to emit today; it recognizes only the `kani::any()` call shape
/// itself, so it reports whatever bindings the generated source actually contains.
fn kani_any_binding_identifier(local: &Local) -> Option<String> {
    let Pat::Type(pat_type) = &local.pat else {
        return None;
    };
    let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
        return None;
    };
    let init = local.init.as_ref()?;
    let Expr::Call(call) = init.expr.as_ref() else {
        return None;
    };
    if !call.args.is_empty() {
        return None;
    }
    let Expr::Path(expr_path) = call.func.as_ref() else {
        return None;
    };
    let segments = expr_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    (segments == ["kani", "any"]).then(|| pat_ident.ident.to_string())
}

/// Parses `source` as a Rust file — the same parser `render()` (`src/kani_obligations.rs`) itself
/// uses to validate generated source before returning it — and returns every `kani::any()`-bound
/// identifier found, in the order it appears in the source.
fn kani_any_identifiers_in_source_order(source: &str) -> Vec<String> {
    let file = syn::parse_file(source).expect("generated harness source must parse as Rust");
    let mut bindings = KaniAnyBindings::default();
    bindings.visit_file(&file);
    bindings.identifiers
}

/// IR-213: asserts the invariant `decode_falsification` depends on, directly against the
/// generator's own output. The `kani::any()` bindings the postcondition harness's generated
/// source actually contains, in the order they appear, must equal `identity.arguments`'s
/// identifiers in order — not merely as a set, since a positional misbinding between two
/// same-typed arguments (e.g. `amount_current`/`balance_pre`, both `i64`) is exactly the failure
/// mode this join exists to exclude, and a set comparison cannot see it.
#[test]
fn ir_213_kani_any_emission_order_matches_persisted_identity_arguments() {
    let pins = KaniToolPins::pinned();
    let harnesses = withdraw_harnesses(&pins);
    let postcondition_harness = harnesses
        .iter()
        .find(|harness| harness.identity.kind == ObligationKind::Postcondition)
        .expect("withdraw_harnesses negotiates a postcondition harness");

    let declared_order = postcondition_harness
        .identity
        .arguments
        .iter()
        .map(|binding| binding.identifier.clone())
        .collect::<Vec<_>>();
    // The join this test protects is only interesting for two or more positions: a
    // single-argument harness could never observe a reorder or a dropped binding.
    assert!(
        declared_order.len() >= 2,
        "fixture must negotiate at least two arguments to make a positional check meaningful: {declared_order:?}"
    );

    let emitted_order = kani_any_identifiers_in_source_order(&postcondition_harness.rust.contents);

    assert_eq!(
        emitted_order, declared_order,
        "kani::any() emission order in the generated harness (left) must match \
         identity.arguments order (right); decode_falsification in src/kani_witness_join.rs \
         binds them positionally, so any divergence here is a silent misbind there"
    );
}
