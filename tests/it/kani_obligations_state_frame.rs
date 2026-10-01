//! FR-015 (IR-412): one `state_clause` postcondition over one integer field becomes an
//! operation-contract Kani harness and a frame-effect Kani harness, both scoped to the clause's
//! operation.
//!
//! The package is hand-built in the shape QSL lowers `post Deposit ... { self.balance >=
//! pre(self.balance) }` to: a `postcondition` `state_clause` whose condition is one
//! `quire.op.integer.ge` over `record.project(deref(self), balance)` and `pre` of the same read,
//! anchored on the `deposit` operation whose frame modifies `balance` only. The framed object
//! declares its own fields, so IR admits the field reads without a domain package.
//!
//! The default lane measures the generated source, identity and refusals. The `kani` lane
//! (`#[ignore]` here, run serially by `make kani`, whose `kani_obligations` filter this module's
//! name matches) proves the harnesses with the installed backend: a healthy subject verifies,
//! each seeded defect is falsified for the intended property, and regenerating the frame from a
//! mutated package turns a green proof red.

#[path = "../state_frame_support/model.rs"]
mod model;
#[path = "../state_frame_support/native_twin.rs"]
mod native_twin;
#[path = "../exact_scalar_support/package.rs"]
#[allow(clippy::duplicate_mod)]
mod package;
#[path = "../state_frame_support/subject.rs"]
// The Kani crates run every subject variant; the test binary executes only some natively.
#[allow(dead_code)]
mod subject;

use std::{fs, path::PathBuf, time::Duration};

use native_twin::{Counterexample, Tamper, Twin};
use package::{
    application, code_id, corpus_package, key, literal, member, op, op_full, parameter_body,
    reference, Bound, PackageBuilder, NODE_DOMAIN, T_BOOLEAN, T_INTEGER,
};
use qsl_replay::{
    CallSiteRefusal, DisagreementCause, FrameChange, FrameIdentityMismatch, ProofCategory,
    ReplayRefusal, ReplayResult, Verdict, WitnessSettlement,
};
use quire_contract_codegen::{
    decode_frame_witness, execute_kani_obligation, generate_state_frame_obligations,
    obligation_digest, FrameReplayError, FrameWitnessRefusal, IdentityRefusal,
    KaniExecutionRequest, KaniInstallation, KaniRunOutcome, ObligationBinding, ObligationKind,
    StateComparison, StateFrameHarness, StateFrameIdentity, StateFrameObligations,
    StateFrameProperty, StateFrameRefusal, StateFrameRequest, UnsupportedFrameEffect,
};
use quire_contract_ir::{CheckedNodeId, CheckedPackageV2, CompleteLoweringRecordV2};
use serde_json::{json, Value};

const OBJECT: u32 = 4001;
const REFERENCE: u32 = 4002;
const SELF: u32 = 4003;
const FRAME: u32 = 4004;
const ANCHOR: u32 = 4005;
const DEREF: u32 = 4006;
const OTHER: u32 = 4011;
const OTHER_OBJECT: u32 = 4012;
const RELATIONSHIP: u32 = 4013;

const STATE_FIELDS: [&str; 2] = [model::FIELDS[0].0, model::FIELDS[1].0];
const STATE_PATH: &str = "crate::subject::Account";
const SUBJECT_PATH: &str = "crate::subject::deposit";
const SUBJECT_SOURCE: &str = include_str!("../state_frame_support/subject.rs");

/// One variant of the fixture. Each variant owns its node codes, because the test binary's
/// fixture registry refuses one code bound to two different bodies.
struct Shape {
    variant: u32,
    clause: &'static str,
    modifies: &'static [&'static str],
    frame_effect: FrameEffect,
    condition: Condition,
    /// The range the object's `balance` member declares; `None` types it by the plain integer.
    balance_bound: Option<(i64, i64)>,
    audit_bound: (i64, i64),
}

/// What a frame does beyond granting fields.
#[derive(Clone, Copy, PartialEq)]
enum FrameEffect {
    None,
    Creates,
    Deletes,
    GrantsRelationship,
    GrantsForeignField,
}

#[derive(Clone, Copy)]
enum Condition {
    /// `post(self.balance) >= pre(self.balance)`.
    PostGePre,
    /// `post(self.balance) >= post(self.balance)`.
    PostGePost,
    /// `post(self.balance) >= pre(self.audit)`.
    BalanceAgainstAudit,
    /// `post(self.balance) >= pre(other.balance)`, `other` a second parameter of the object type.
    BalanceAgainstOther,
    /// `post(self.balance) >= 0`.
    BalanceAgainstLiteral,
    /// `post(self.balance) <= pre(self.balance)`.
    PostLePre,
    /// `not (post(self.balance) >= pre(self.balance))`.
    Negated,
}

impl Shape {
    const HEALTHY: Self = Self {
        variant: 0,
        clause: "postcondition",
        modifies: &model::GRANTED,
        frame_effect: FrameEffect::None,
        condition: Condition::PostGePre,
        balance_bound: Some(model::FIELDS[0].1),
        audit_bound: model::FIELDS[1].1,
    };

    fn code(&self, base: u32) -> u32 {
        50_000 + 1_000 * self.variant + base
    }
}

fn node_ref(digest: &str) -> Value {
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

fn field_read(
    builder: &mut PackageBuilder,
    code: u32,
    object: &str,
    field: &str,
    bound: &str,
) -> String {
    let deref = code_id(DEREF).digest.to_string();
    field_read_through(builder, code, &deref, (object, field, bound))
}

/// `project(<deref>, field)` over `object`'s `field`, typed by `bound`.
fn field_read_through(
    builder: &mut PackageBuilder,
    code: u32,
    deref: &str,
    (object, field, bound): (&str, &str, &str),
) -> String {
    let member = json!({"kind": "field", "declaration": node_ref(object), "name": field});
    builder.application_code_with(
        code,
        "expression",
        "query",
        bound,
        application(
            "query",
            op_full("quire.op.record.project", vec![], None, Some(member)),
            bound,
            vec![reference(deref)],
        ),
        &[deref.to_owned(), object.to_owned()],
    );
    code_id(code).digest.to_string()
}

fn pre_read(builder: &mut PackageBuilder, code: u32, read: &str, bound: &str) -> String {
    builder.application_bounded(
        code,
        "expression",
        "pre_read",
        bound,
        application(
            "pre",
            op("quire.op.state.pre"),
            bound,
            vec![reference(read)],
        ),
        &[],
    );
    code_id(code).digest.to_string()
}

fn package_for(shape: &Shape) -> PackageBuilder {
    let mut builder = corpus_package();
    let balance_key = match shape.balance_bound {
        Some((minimum, maximum)) => builder.bound(&Bound::Integer(minimum, maximum)),
        None => key(T_INTEGER),
    };
    let audit_key = builder.bound(&Bound::Integer(shape.audit_bound.0, shape.audit_bound.1));
    // A second integer range keeps a bound reachable when `balance` names none.
    let object = key(OBJECT);
    builder.node_with(
        &object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": [
            member("balance", reference(&balance_key)),
            member("audit", reference(&audit_key)),
        ]}),
        &[balance_key.clone(), audit_key.clone()],
    );
    let other_object = key(OTHER_OBJECT);
    builder.node_with(
        &other_object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
        &[],
    );
    let relationship = key(RELATIONSHIP);
    builder.node_with(
        &relationship,
        "relation",
        "relationship",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
        &[key(T_BOOLEAN)],
    );
    builder.node_with(
        &key(REFERENCE),
        "composite_type",
        "reference",
        &key(REFERENCE),
        json!({"term": "aggregate", "members": [reference(&object)]}),
        std::slice::from_ref(&object),
    );
    builder.code(
        SELF,
        "value",
        "parameter",
        &key(REFERENCE),
        parameter_body("self", 0),
    );
    builder.code(
        OTHER,
        "value",
        "parameter",
        &key(REFERENCE),
        parameter_body("other", 1),
    );
    let field_entry = |declaration: &str, name: &str| json!({"kind": "field", "declaration": node_ref(declaration), "name": name});
    let mut modifies = shape
        .modifies
        .iter()
        .map(|name| field_entry(&object, name))
        .collect::<Vec<_>>();
    let mut frame_dependencies = vec![object.clone()];
    let (mut creates, mut deletes) = (vec![], vec![]);
    match shape.frame_effect {
        FrameEffect::None => {}
        FrameEffect::Creates => creates.push(node_ref(&object)),
        FrameEffect::Deletes => deletes.push(node_ref(&object)),
        FrameEffect::GrantsRelationship => {
            modifies.push(json!({"kind": "relationship", "declaration": node_ref(&relationship)}));
            frame_dependencies.push(relationship.clone());
        }
        FrameEffect::GrantsForeignField => {
            modifies.push(field_entry(&other_object, "balance"));
            frame_dependencies.push(other_object.clone());
        }
    }
    builder.node_with(
        &key(FRAME),
        "state",
        "frame",
        &object,
        json!({"term": "frame", "modifies": modifies, "creates": creates, "deletes": deletes}),
        &frame_dependencies,
    );
    let frame = key(FRAME);
    builder.node_with(
        &key(ANCHOR),
        "state",
        "operation_anchor",
        &object,
        json!({"term": "aggregate", "members": [
            member("context", reference(&object)),
            member("operation", literal("text", "deposit")),
            member("frame", reference(&frame)),
        ]}),
        &[object.clone(), frame],
    );
    builder.application_bounded(
        DEREF,
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(SELF))],
        ),
        &[],
    );
    builder.application_bounded(
        shape.code(420),
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(OTHER))],
        ),
        &[],
    );
    let post_balance = field_read(
        &mut builder,
        shape.code(410),
        &object,
        "balance",
        &balance_key,
    );
    let pre_balance = pre_read(&mut builder, shape.code(411), &post_balance, &balance_key);
    let post_audit = field_read(&mut builder, shape.code(400), &object, "audit", &audit_key);
    let pre_audit = pre_read(&mut builder, shape.code(401), &post_audit, &audit_key);
    let other = code_id(shape.code(420)).digest.to_string();
    let post_other = field_read_through(
        &mut builder,
        shape.code(421),
        &other,
        (&object, "balance", &balance_key),
    );
    let pre_other = pre_read(&mut builder, shape.code(422), &post_other, &balance_key);
    let zero = literal("integer", "0");
    let (left, right) = match shape.condition {
        Condition::PostGePre | Condition::PostLePre | Condition::Negated => {
            (reference(&post_balance), reference(&pre_balance))
        }
        Condition::PostGePost => (reference(&post_balance), reference(&post_balance)),
        Condition::BalanceAgainstAudit => (reference(&post_balance), reference(&pre_audit)),
        Condition::BalanceAgainstOther => (reference(&post_balance), reference(&pre_other)),
        Condition::BalanceAgainstLiteral => (reference(&post_balance), zero),
    };
    builder.application_bounded(
        shape.code(200),
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op(if matches!(shape.condition, Condition::PostLePre) {
                "quire.op.integer.le"
            } else {
                "quire.op.integer.ge"
            }),
            &key(T_BOOLEAN),
            vec![left, right],
        ),
        &[],
    );
    let mut condition = code_id(shape.code(200)).digest.to_string();
    if matches!(shape.condition, Condition::Negated) {
        builder.application_bounded(
            shape.code(210),
            "expression",
            "unary",
            &key(T_BOOLEAN),
            application(
                "unary",
                op("quire.op.boolean.not"),
                &key(T_BOOLEAN),
                vec![reference(&condition)],
            ),
            &[],
        );
        condition = code_id(shape.code(210)).digest.to_string();
    }
    let parameters = if matches!(shape.condition, Condition::BalanceAgainstOther) {
        vec![reference(&key(SELF)), reference(&key(OTHER))]
    } else {
        vec![reference(&key(SELF))]
    };
    builder.application_bounded(
        shape.code(300),
        "state",
        "state_clause",
        &key(T_BOOLEAN),
        application(
            "state_clause",
            op_full(
                "quire.op.state.clause",
                vec![],
                None,
                Some(json!({"kind": "state_clause", "clause": shape.clause})),
            ),
            &key(T_BOOLEAN),
            vec![
                json!({"term": "aggregate", "members": parameters}),
                reference(&key(ANCHOR)),
                reference(&condition),
            ],
        ),
        &[],
    );
    builder
}

struct Fixture {
    package: CheckedPackageV2,
    clause: CheckedNodeId,
    object: CheckedNodeId,
    anchor: CheckedNodeId,
    frame: CheckedNodeId,
}

fn fixture(shape: &Shape) -> Fixture {
    let package = package_for(shape).admit();
    let id = |digest: String| -> CheckedNodeId {
        serde_json::from_value(node_ref(&digest)).expect("node id")
    };
    Fixture {
        package,
        clause: code_id(shape.code(300)),
        object: id(key(OBJECT)),
        anchor: id(key(ANCHOR)),
        frame: id(key(FRAME)),
    }
}

fn request<'a>(fixture: &'a Fixture, fields: &'a [&'a str]) -> StateFrameRequest<'a> {
    StateFrameRequest {
        package: &fixture.package,
        clause: &fixture.clause,
        state_path: STATE_PATH,
        state_fields: fields,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    }
}

fn generate(fixture: &Fixture) -> StateFrameObligations {
    generate_state_frame_obligations(&request(fixture, &STATE_FIELDS))
        .unwrap_or_else(|refusal| panic!("the fixture must generate: {refusal}"))
}

fn refusal(shape: &Shape, fields: &[&str]) -> StateFrameRefusal {
    let fixture = fixture(shape);
    generate_state_frame_obligations(&request(&fixture, fields))
        .expect_err("the request must be refused")
}

/// Each binding of `harness` with the range assumed of it.
fn domains(harness: &StateFrameHarness) -> Vec<(String, Option<(i64, i64)>)> {
    harness
        .identity
        .arguments
        .iter()
        .map(|binding| {
            (
                binding.identifier.clone(),
                binding
                    .integer_bounds
                    .as_ref()
                    .map(|bounds| (bounds.minimum, bounds.maximum)),
            )
        })
        .collect()
}

/// A `cargo kani --concrete-playback print` transcript of `obligation`'s harness whose symbolic
/// bindings take `values`, in the order the harness draws them. Only the default lane builds one;
/// the kani lane decodes the prover's own.
fn playback(obligation: &StateFrameIdentity, values: &[(&str, i64)]) -> String {
    let entries = obligation
        .arguments
        .iter()
        .map(|binding| {
            let (_, value) = values
                .iter()
                .find(|(name, _)| *name == binding.identifier)
                .expect("a value for every binding");
            let bytes = value
                .to_le_bytes()
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("        // {value}\n        vec![{bytes}],\n")
        })
        .collect::<String>();
    format!(
        "/// Test generated for harness `{module}::{harness}`\n\
/// Check for `assertion`: \"a frame effect\"\n\
#[test]\n\
fn kani_concrete_playback_check_1() {{\n\
    let concrete_vals: Vec<Vec<u8>> = vec![\n{entries}    ];\n\
    kani::concrete_playback_run(concrete_vals, check);\n\
}}\n",
        module = obligation.module_symbol,
        harness = obligation.harness_symbol,
    )
}

/// The frame obligation of the healthy fixture and a playback of it at `(balance, audit)`.
fn counterexample(balance: i64, audit: i64) -> Counterexample {
    let obligation = generate(&fixture(&Shape::HEALTHY)).frame.identity;
    let transcript = playback(&obligation, &[("balance", balance), ("audit", audit)]);
    let witness = decode_frame_witness(&obligation, &transcript).expect("the playback decodes");
    Counterexample {
        obligation,
        witness,
    }
}

// ---- default lane ------------------------------------------------------------

/// A postcondition clause yields two separate harnesses whose identity is scoped to the
/// operation, anchor, frame and object, whose bounds and granted and forbidden fields are read
/// from the IR, and each carries exactly one non-vacuity cover.
///
/// Trace: FR-015-AC-26, FR-015-AC-27, FR-015-AC-28, TC-025
#[test]
fn tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness() {
    let fixture = fixture(&Shape::HEALTHY);
    let generated = generate(&fixture);
    let scope = &generated.postcondition.identity.scope;
    assert_eq!(scope.operation, "deposit");
    assert_eq!(scope.object, fixture.object);
    assert_eq!(scope.anchor, fixture.anchor);
    assert_eq!(scope.frame, fixture.frame);
    assert_eq!(&generated.frame.identity.scope, scope);
    assert_eq!(generated.postcondition.identity.clause, fixture.clause);

    assert_eq!(
        generated.postcondition.identity.property,
        StateFrameProperty::Postcondition {
            field: "balance".to_owned(),
            comparison: StateComparison::Ge,
            left_is_pre: false,
        }
    );
    // Every field the object's IR bounds is assumed in range, so a counterexample is a state
    // the model admits. The bindings are ascending by field name, the order the harness draws
    // them in.
    for harness in [&generated.postcondition, &generated.frame] {
        assert_eq!(
            domains(harness),
            vec![
                ("audit".to_owned(), Some((0, 1000))),
                ("balance".to_owned(), Some((0, 1000)))
            ]
        );
        let source = &harness.rust.contents;
        assert!(source.contains("kani::assume(pre.balance >= 0_i64 && pre.balance <= 1000_i64);"));
        assert!(source.contains("kani::assume(pre.audit >= 0_i64 && pre.audit <= 1000_i64);"));
    }
    assert_eq!(
        generated.frame.identity.property,
        StateFrameProperty::Frame {
            granted: vec!["balance".to_owned()],
            checked: vec!["audit".to_owned()],
        }
    );

    let post = &generated.postcondition.rust.contents;
    assert!(post.contains("let mut post = pre.clone();"));
    assert!(post.contains("crate::subject::deposit(&mut post);"));
    assert!(post.contains("post.balance >= pre.balance"));
    let frame = &generated.frame.rust.contents;
    assert!(frame.contains("assert!(post.audit == pre.audit,"));
    assert!(!frame.contains("post.balance == pre.balance"));
    for source in [post, frame] {
        assert_eq!(source.matches("#[kani::proof]").count(), 1);
        assert_eq!(source.matches("kani::cover!(").count(), 1);
    }
    assert_eq!(post.matches("assert!(").count(), 1);
    assert_eq!(frame.matches("assert!(").count(), 1);

    // The persisted record carries the same scoped identity.
    let record: Value = serde_json::from_str(&generated.frame.record.contents).expect("record");
    assert_eq!(record["identity"]["scope"]["operation"], "deposit");
    assert_eq!(record["identity"]["property"]["kind"], "frame");
    assert_eq!(
        record["identity"]["scope"]["frame"]["digest"],
        json!(fixture.frame.digest)
    );

    // Equal inputs regenerate byte-identically, and the unwind bound is part of the identity.
    assert_eq!(generate(&fixture), generated);
    let unwound = generate_state_frame_obligations(&StateFrameRequest {
        unwind: 5,
        ..request(&fixture, &STATE_FIELDS)
    })
    .expect("generates");
    assert_ne!(
        unwound.frame.identity.options,
        generated.frame.identity.options
    );
}

/// A frame that grants nothing checks every field, so a different frame is a different proof.
///
/// Trace: FR-015-AC-28, TC-025
#[test]
fn tc_025_the_frame_harness_follows_the_frame_node_not_the_caller() {
    let emptied = Shape {
        variant: 1,
        modifies: &[],
        ..Shape::HEALTHY
    };
    let generated = generate(&fixture(&emptied));
    assert_eq!(
        generated.frame.identity.property,
        StateFrameProperty::Frame {
            granted: vec![],
            checked: vec!["balance".to_owned(), "audit".to_owned()],
        }
    );
    let frame = &generated.frame.rust.contents;
    assert!(frame.contains("assert!(post.balance == pre.balance,"));
    assert!(frame.contains("assert!(post.audit == pre.audit,"));
}

/// Every shape outside one integer comparison of pre and post reads of one field through
/// `self`, and every frame effect with no finite encoding, is refused by name, with no harness.
/// Each row fails if the refusal it names is removed: the shape would otherwise generate.
///
/// Trace: FR-015-AC-29, TC-025
#[test]
fn tc_025_shapes_without_a_finite_encoding_are_refused_by_name() {
    let refused = |shape: Shape| refusal(&shape, &STATE_FIELDS);
    let with = |variant, edit: fn(Shape) -> Shape| {
        refused(edit(Shape {
            variant,
            ..Shape::HEALTHY
        }))
    };
    let condition_not_supported = |refusal: StateFrameRefusal| {
        matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
    };
    let unsupported_effect = |refusal: StateFrameRefusal, expected: UnsupportedFrameEffect| {
        matches!(
            refusal,
            StateFrameRefusal::FrameEffectUnsupported { effect, .. } if effect == expected
        )
    };

    // Not a postcondition.
    assert!(matches!(
        with(2, |shape| Shape { clause: "precondition", ..shape }),
        StateFrameRefusal::NotAPostcondition { clause } if clause == "precondition"
    ));
    // The condition.
    assert_eq!(
        with(3, |shape| Shape {
            condition: Condition::PostGePost,
            ..shape
        }),
        StateFrameRefusal::ObservationsSameSide
    );
    assert_eq!(
        with(4, |shape| Shape {
            condition: Condition::BalanceAgainstAudit,
            ..shape
        }),
        StateFrameRefusal::ObservationsDiffer {
            left: "balance".to_owned(),
            right: "audit".to_owned()
        }
    );
    assert!(condition_not_supported(with(10, |shape| Shape {
        condition: Condition::Negated,
        ..shape
    })));
    assert!(condition_not_supported(with(11, |shape| Shape {
        condition: Condition::BalanceAgainstLiteral,
        ..shape
    })));
    // A read through a second parameter of the object type is not a read of `self`.
    assert!(condition_not_supported(with(12, |shape| Shape {
        condition: Condition::BalanceAgainstOther,
        ..shape
    })));
    // The frame.
    assert!(unsupported_effect(
        with(5, |shape| Shape {
            frame_effect: FrameEffect::Creates,
            ..shape
        }),
        UnsupportedFrameEffect::Creates
    ));
    assert!(unsupported_effect(
        with(7, |shape| Shape {
            frame_effect: FrameEffect::Deletes,
            ..shape
        }),
        UnsupportedFrameEffect::Deletes
    ));
    assert!(unsupported_effect(
        with(8, |shape| Shape {
            frame_effect: FrameEffect::GrantsRelationship,
            ..shape
        }),
        UnsupportedFrameEffect::Relationship
    ));
    assert!(unsupported_effect(
        with(9, |shape| Shape {
            frame_effect: FrameEffect::GrantsForeignField,
            ..shape
        }),
        UnsupportedFrameEffect::ForeignField
    ));
    // The clause's own field declares no integer range.
    assert_eq!(
        with(13, |shape| Shape {
            balance_bound: None,
            ..shape
        }),
        StateFrameRefusal::BoundNotResolved {
            field: "balance".to_owned()
        }
    );
    // A field the frame grants or the clause reads that the caller's state lacks.
    assert_eq!(
        refusal(&Shape::HEALTHY, &["audit"]),
        StateFrameRefusal::UnknownStateField {
            field: "balance".to_owned()
        }
    );
    // A frame granting every state field forbids nothing.
    assert!(matches!(
        refusal(&Shape::HEALTHY, &["balance"]),
        StateFrameRefusal::NothingForbidden { .. }
    ));
}

/// Each field is assumed in the range its own member declares, so two fields with different
/// ranges each keep theirs, and the clause field never takes another field's.
///
/// Trace: FR-015-AC-27, TC-025
#[test]
fn tc_025_each_field_is_assumed_in_its_own_declared_range() {
    let shape = Shape {
        variant: 6,
        balance_bound: Some((5, 900)),
        audit_bound: (0, 50),
        ..Shape::HEALTHY
    };
    let generated = generate(&fixture(&shape));
    let expected = vec![
        ("audit".to_owned(), Some((0, 50))),
        ("balance".to_owned(), Some((5, 900))),
    ];
    assert_eq!(domains(&generated.postcondition), expected);
    assert_eq!(domains(&generated.frame), expected);
    let post = &generated.postcondition.rust.contents;
    assert!(post.contains("kani::assume(pre.balance >= 5_i64 && pre.balance <= 900_i64);"));
    assert!(post.contains("kani::assume(pre.audit >= 0_i64 && pre.audit <= 50_i64);"));
}

/// Two clauses of one operation share an anchor and frame, yet each harness lands at its own
/// module and record path.
///
/// Trace: FR-015-AC-26, TC-025
#[test]
fn tc_025_two_clauses_of_one_operation_never_share_a_frame_record_path() {
    let first = generate(&fixture(&Shape::HEALTHY));
    let second = generate(&fixture(&Shape {
        variant: 14,
        condition: Condition::PostLePre,
        ..Shape::HEALTHY
    }));
    assert_eq!(first.frame.identity.scope, second.frame.identity.scope);
    assert_ne!(first.frame.identity.clause, second.frame.identity.clause);
    assert_ne!(first.frame.record.path, second.frame.record.path);
    assert_ne!(first.frame.rust.path, second.frame.rust.path);
}

/// A request that is not well formed, and a node that is not a clause, are refused before any
/// harness is generated.
///
/// Trace: FR-015-AC-29, TC-025
#[test]
fn tc_025_malformed_requests_and_non_clause_nodes_are_refused() {
    let fixture = fixture(&Shape::HEALTHY);
    let refused = |request: StateFrameRequest<'_>| {
        generate_state_frame_obligations(&request).expect_err("refused")
    };
    assert_eq!(
        refused(StateFrameRequest {
            unwind: 0,
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::UnwindOutOfRange { unwind: 0 }
    );
    assert_eq!(
        refused(StateFrameRequest {
            subject_path: "not a path",
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::InvalidPath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        refused(request(&fixture, &["balance", "balance"])),
        StateFrameRefusal::InvalidField {
            name: "balance".to_owned()
        }
    );
    let frame_node = refused(StateFrameRequest {
        clause: &fixture.frame,
        ..request(&fixture, &STATE_FIELDS)
    });
    assert!(matches!(
        frame_node,
        StateFrameRefusal::NotAStateClause { ref semantic_form, .. } if semantic_form == "frame"
    ));
    let absent = package::id(&key(9999));
    assert!(matches!(
        refused(StateFrameRequest {
            clause: &absent,
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::NotLowered { record }
            if matches!(*record, CompleteLoweringRecordV2::InvalidInput { .. })
    ));
}

/// `replay_frame` refuses an envelope whose `clause_node` or `occurrence_key` is not its
/// payload's, naming both; the same run with an agreeing envelope replays, so each refusal is the
/// difference and not the run.
///
/// Trace: TC-025
#[test]
fn tc_025_replay_frame_refuses_an_envelope_that_disagrees_with_its_payload() {
    let twin = Twin::new();
    let counterexample = counterexample(5, 0);
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    assert!(twin
        .replay_tampered(
            &counterexample,
            &invocation,
            "account",
            "audit",
            Tamper::Nothing
        )
        .is_ok());

    let clause = twin.replay_tampered(
        &counterexample,
        &invocation,
        "account",
        "audit",
        Tamper::ClauseNode,
    );
    assert!(matches!(
        clause,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeFrame { .. })
    ));
    let occurrence = twin.replay_tampered(
        &counterexample,
        &invocation,
        "account",
        "audit",
        Tamper::Occurrence,
    );
    assert!(matches!(
        occurrence,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeOccurrence { .. })
    ));
}

/// The frame-replay payload's identities are QSL's answer for the operation, and the envelope
/// names the payload's own frame node and frame occurrence.
///
/// Trace: FR-015-AC-33, FR-015-AC-34, TC-025
#[test]
fn tc_025_the_frame_replay_envelope_names_the_payloads_frame_and_occurrence() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let replay = twin.frame_replay(&counterexample(5, 0), &invocation, "account", "audit");
    let payload = replay.packet.family_payload.as_ref().expect("a payload");
    assert_ne!(payload.anchor, payload.frame);
    assert_eq!(payload.occurrence.node(), payload.frame);
    assert_eq!(replay.packet.clause_node, Some(payload.frame));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&payload.occurrence)
    );
}

/// `FrameReplay::replay` returns QSL's result without Kani: a forbidden write settles a reproduced
/// violation that names the written field, and a write the frame grants is a respected frame.
///
/// Trace: FR-015-AC-36, TC-025
#[test]
fn tc_025_frame_replay_settles_a_forbidden_and_a_granted_write() {
    let twin = Twin::new();
    let counterexample = counterexample(5, 0);
    let forbidden = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&counterexample, &forbidden, "account", "audit")
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), ProofCategory::Violation);
    let Some(FrameChange::FieldWrite { object, field, .. }) =
        result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!((object.as_str(), field.as_str()), ("account", "audit"));

    let granted = twin.invocation("account", (5, 0), (6, 0));
    let result = twin
        .frame_replay(&counterexample, &granted, "account", "balance")
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(DisagreementCause::Verdicts {
            proved: Verdict::from_category(ProofCategory::Violation),
            replayed: Verdict::from_category(ProofCategory::Success),
        })
    );
    assert!(result.found().is_none());
}

/// An operation the unit names no frame for is refused by the call site before any request is
/// built, whether the domain package declares it (`transfer`, which no clause names) or not
/// (`withdraw`).
///
/// Trace: FR-015-AC-35, TC-025
#[test]
fn tc_025_an_operation_with_no_frame_is_refused_when_the_request_is_built() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    let counterexample = counterexample(5, 0);
    for operation in ["transfer", "withdraw"] {
        let refusal = twin
            .try_frame_replay(operation, &counterexample, &invocation, "account", "audit")
            .err()
            .unwrap_or_else(|| panic!("the unit names no frame for `{operation}`"));
        assert!(
            matches!(
                &refusal,
                FrameReplayError::CallSite(cause)
                    if matches!(
                        &**cause,
                        CallSiteRefusal::UnknownOperation { selection, .. }
                            if selection.operation.as_str() == operation
                    )
            ),
            "{operation}: {refusal}"
        );
    }
}

/// One clause yields a postcondition obligation and a frame obligation, and they are two
/// identities: the kind is part of what an identity commits to, so the clause node alone never
/// names an obligation.
///
/// Trace: FR-015-AC-37, TC-025
#[test]
fn tc_025_two_kinds_of_one_clause_are_two_identities() {
    let generated = generate(&fixture(&Shape::HEALTHY));
    let (post, frame) = (&generated.postcondition.identity, &generated.frame.identity);
    assert_eq!(post.clause, frame.clause);
    assert_eq!(post.arguments, frame.arguments);
    assert_eq!(
        (post.kind, frame.kind),
        (ObligationKind::Postcondition, ObligationKind::Frame)
    );
    assert_ne!(post.obligation_identity, frame.obligation_identity);
    // Equal inputs mint the same identity, and the stored digest is the digest of the contents.
    assert_eq!(
        generate(&fixture(&Shape::HEALTHY))
            .frame
            .identity
            .obligation_identity,
        frame.obligation_identity
    );
    assert_eq!(
        obligation_digest(&frame.clause, frame.kind, &frame.arguments),
        Ok(frame.obligation_identity)
    );
}

/// The identity commits to the clause, the kind and each binding's identifier and
/// declared range: changing any one changes it. The unwind bound and the generated symbols are
/// not in the preimage.
///
/// Trace: FR-015-AC-37, TC-025
#[test]
fn tc_025_the_identity_changes_with_the_clause_the_kind_and_each_binding() {
    let frame = generate(&fixture(&Shape::HEALTHY)).frame.identity;
    let digest = |clause: &_, kind, arguments: &[ObligationBinding]| {
        obligation_digest(clause, kind, arguments).expect("ascending arguments")
    };
    let original = digest(&frame.clause, frame.kind, &frame.arguments);
    assert_eq!(original, frame.obligation_identity);

    let other_clause = generate(&fixture(&Shape {
        variant: 14,
        condition: Condition::PostLePre,
        ..Shape::HEALTHY
    }))
    .frame
    .identity;
    assert_ne!(other_clause.clause, frame.clause);
    assert_ne!(other_clause.obligation_identity, original);
    assert_ne!(
        digest(
            &frame.clause,
            ObligationKind::Postcondition,
            &frame.arguments
        ),
        original
    );

    let mut mutated = frame.arguments.clone();
    mutated[1].integer_bounds.as_mut().expect("a range").maximum += 1;
    assert_ne!(
        digest(&frame.clause, frame.kind, &mutated),
        original,
        "domain"
    );
    let mut mutated = frame.arguments.clone();
    mutated[1].integer_bounds = None;
    assert_ne!(
        digest(&frame.clause, frame.kind, &mutated),
        original,
        "unbounded"
    );
    let mut mutated = frame.arguments.clone();
    mutated[1].identifier = "balance_".to_owned();
    assert_ne!(
        digest(&frame.clause, frame.kind, &mutated),
        original,
        "identifier"
    );
    assert_ne!(
        digest(&frame.clause, frame.kind, &frame.arguments[..1]),
        original,
        "a dropped binding"
    );

    // Neither the unwind bound nor the harness symbols decide which obligation this is.
    let unwound = generate_state_frame_obligations(&StateFrameRequest {
        unwind: 5,
        ..request(&fixture(&Shape::HEALTHY), &STATE_FIELDS)
    })
    .expect("generates")
    .frame
    .identity;
    assert_ne!(unwound.options, frame.options);
    assert_eq!(unwound.obligation_identity, original);
}

/// The bindings are ascending by field name, and harness order is binding order: `audit` is
/// drawn first although `balance` is declared first.
///
/// Trace: FR-015-AC-37, TC-025
#[test]
fn tc_025_bindings_are_ascending_and_the_harness_draws_them_in_that_order() {
    let fixture = fixture(&Shape::HEALTHY);
    let frame = generate(&fixture).frame;
    let names = frame
        .identity
        .arguments
        .iter()
        .map(|binding| binding.identifier.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["audit", "balance"]);
    let source = &frame.rust.contents;
    assert!(
        source.contains("S { audit: kani::any(), balance: kani::any() }")
            || source.contains("Account { audit: kani::any(), balance: kani::any() }"),
        "{source}"
    );

    // Arguments out of order are no identity: the harness order would not be the committed one.
    let mut swapped = frame.identity.arguments.clone();
    swapped.swap(0, 1);
    assert_eq!(
        obligation_digest(&frame.identity.clause, frame.identity.kind, &swapped),
        Err(IdentityRefusal::ArgumentsNotAscending {
            identifier: "audit".to_owned()
        })
    );
}

/// A playback decodes into the values of its obligation's bindings; a value outside the range
/// the harness assumed, a playback of another harness, a non-frame obligation and an identity
/// that is not its own contents' are each refused before an envelope is built.
///
/// Trace: FR-015-AC-38, TC-025
#[test]
fn tc_025_a_playback_that_is_not_a_witness_of_the_obligation_is_refused() {
    let generated = generate(&fixture(&Shape::HEALTHY));
    let frame = &generated.frame.identity;
    let witness = decode_frame_witness(frame, &playback(frame, &[("balance", 7), ("audit", 9)]))
        .expect("an in-domain playback decodes");
    assert_eq!(witness.obligation(), frame.obligation_identity);
    assert_eq!(witness.integer("balance"), Some(7));
    assert_eq!(witness.integer("audit"), Some(9));
    // The bounds are inclusive.
    assert!(
        decode_frame_witness(frame, &playback(frame, &[("balance", 1000), ("audit", 0)])).is_ok()
    );

    for (balance, audit, argument) in [
        (1001, 0, "balance"),
        (5, -1, "audit"),
        (i64::MIN, 0, "balance"),
    ] {
        assert_eq!(
            decode_frame_witness(
                frame,
                &playback(frame, &[("balance", balance), ("audit", audit)])
            ),
            Err(FrameWitnessRefusal::OutOfDomain {
                argument: argument.to_owned()
            })
        );
    }

    // The postcondition harness's playback, decoded against the frame obligation.
    let post = &generated.postcondition.identity;
    let sibling = playback(post, &[("balance", 7), ("audit", 9)]);
    assert!(matches!(
        decode_frame_witness(frame, &sibling),
        Err(FrameWitnessRefusal::Decode(failure))
            if failure.code == "cg_witness_harness_identity_mismatch"
    ));
    assert_eq!(
        decode_frame_witness(post, &sibling),
        Err(FrameWitnessRefusal::NotAFrame {
            kind: ObligationKind::Postcondition
        })
    );

    // An identity whose bindings were edited after it was minted.
    let mut stale = frame.clone();
    stale.arguments[0]
        .integer_bounds
        .as_mut()
        .expect("a range")
        .maximum = i64::MAX;
    assert_eq!(
        decode_frame_witness(&stale, &playback(&stale, &[("balance", 7), ("audit", 9)])),
        Err(FrameWitnessRefusal::IdentityStale(None))
    );
}

/// The envelope carries the obligation's identity digest and the decoded values, each named by
/// its binding, and a request whose obligation is not the one the witness was
/// decoded under is refused when it is built.
///
/// Trace: FR-015-AC-38, TC-025
#[test]
fn tc_025_the_envelope_carries_the_obligation_identity_and_the_decoded_values() {
    let twin = Twin::new();
    let counterexample = counterexample(5, 3);
    let invocation = twin.invocation("account", (5, 3), (6, 4));
    let replay = twin.frame_replay(&counterexample, &invocation, "account", "audit");
    assert_eq!(
        replay.packet.obligation_identity,
        Some(*counterexample.obligation.obligation_identity.as_bytes())
    );
    let Some(qsl_replay::ReplaySource::Witness(witness)) = &replay.packet.source else {
        panic!("the envelope carries the transcript");
    };
    assert_eq!(
        witness.concrete_values(),
        vec![("audit".to_owned(), 3), ("balance".to_owned(), 5)],
        "audit then balance, each named by its binding"
    );

    let other = generate(&self::fixture(&Shape {
        variant: 14,
        condition: Condition::PostLePre,
        ..Shape::HEALTHY
    }))
    .frame
    .identity;
    let mismatched = Counterexample {
        obligation: other.clone(),
        witness: counterexample.witness.clone(),
    };
    let refusal = twin
        .try_frame_replay("deposit", &mismatched, &invocation, "account", "audit")
        .err()
        .expect("the witness belongs to another obligation");
    assert!(matches!(
        refusal,
        FrameReplayError::ObligationMismatch { witness, obligation }
            if witness == counterexample.obligation.obligation_identity
                && obligation == other.obligation_identity
    ));
}

// ---- kani lane ---------------------------------------------------------------

const KANI_TIMEOUT: Duration = Duration::from_secs(600);

fn scratch(name: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("state-frame-{name}-{}-{nonce}", std::process::id()));
    fs::create_dir_all(path.join("src")).expect("scratch");
    path
}

/// The obligations of `fixture` over the subject function named `subject` in `subject.rs`.
fn generate_over(fixture: &Fixture, subject: &str) -> StateFrameObligations {
    let subject_path = format!("crate::subject::{subject}");
    generate_state_frame_obligations(&StateFrameRequest {
        subject_path: &subject_path,
        ..request(fixture, &STATE_FIELDS)
    })
    .unwrap_or_else(|refusal| panic!("the fixture must generate: {refusal}"))
}

/// Runs `harness` with the real prover over the subject module every generated harness names.
fn prove(harness: &StateFrameHarness) -> KaniRunOutcome {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\npub mod subject {{\n{SUBJECT_SOURCE}}}\n",
            harness.rust.contents
        ),
    )
    .expect("lib");
    fs::write(
        directory.join("Cargo.toml"),
        "[package]\nname = \"generated-state-frame\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("manifest");
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("build script");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: harness.into(),
        crate_directory: &directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-frame-kani"),
        timeout: KANI_TIMEOUT,
    })
    .unwrap_or_else(|refusal| panic!("the run must start: {refusal}"));
    let _ = fs::remove_dir_all(directory);
    evidence.outcome
}

/// The counterexample of a falsified proof, which must name `reason`.
fn falsified(outcome: KaniRunOutcome, reason: &str) -> String {
    let KaniRunOutcome::Falsified { counterexample } = outcome else {
        panic!("expected a falsified proof for `{reason}`, got {outcome:?}");
    };
    assert!(
        counterexample.contains(reason),
        "the counterexample must name `{reason}`:\n{counterexample}"
    );
    counterexample
}

/// The operation contract verifies for a healthy subject and is falsified, for the postcondition
/// and no other reason, when the subject is mutated to debit.
///
/// Trace: FR-015-AC-30, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it() {
    let fixture = fixture(&Shape::HEALTHY);
    assert_eq!(
        prove(&generate_over(&fixture, "deposit").postcondition),
        KaniRunOutcome::Verified
    );
    falsified(
        prove(&generate_over(&fixture, "deposit_debiting").postcondition),
        "postcondition `post.balance >= pre.balance` failed",
    );
}

/// An effect the frame allows verifies, an effect it forbids is falsified naming the forbidden
/// field, and regenerating the frame from a package whose `modifies` is mutated to grant nothing
/// turns the allowed proof red.
///
/// Trace: FR-015-AC-31, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies() {
    let fixture = fixture(&Shape::HEALTHY);
    // ALLOWED: only `balance` changes, and the frame grants it.
    assert_eq!(
        prove(&generate_over(&fixture, "deposit").frame),
        KaniRunOutcome::Verified
    );
    // FORBIDDEN: `audit` changes too.
    falsified(
        prove(&generate_over(&fixture, "deposit_touching_audit").frame),
        "changed `audit`, which its frame does not modify",
    );
    // MUTATION CONTROL: the same allowed subject against a frame that grants nothing.
    let emptied = self::fixture(&Shape {
        variant: 1,
        modifies: &[],
        ..Shape::HEALTHY
    });
    falsified(
        prove(&generate_over(&emptied, "deposit").frame),
        "changed `balance`, which its frame does not modify",
    );
}

/// The forbidden frame counterexample Kani finds is executed natively, then replayed through
/// QSL's `replay_frame`, which reproduces the violation on the forbidden field; the allowed
/// effect's run replays as a frame the invocation respects.
///
/// Trace: FR-015-AC-32, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_frame_counterexamples_replay_natively_through_qsl() {
    let fixture = fixture(&Shape::HEALTHY);
    let twin = Twin::new();

    let forbidden = generate_over(&fixture, "deposit_touching_audit");
    let counterexample = falsified(
        prove(&forbidden.frame),
        "changed `audit`, which its frame does not modify",
    );
    let witness =
        decode_frame_witness(&forbidden.frame.identity, &counterexample).expect("a real playback");
    let balance = witness.integer("balance").expect("a decoded balance");
    let audit = witness.integer("audit").expect("a decoded audit");
    let mut account = subject::Account { balance, audit };
    subject::deposit_touching_audit(&mut account);
    assert_ne!(
        account.audit, audit,
        "the native run reproduces the forbidden write"
    );
    let counterexample = Counterexample {
        obligation: forbidden.frame.identity.clone(),
        witness,
    };
    let invocation = twin.invocation(
        "account",
        (balance, audit),
        (account.balance, account.audit),
    );
    let result = twin
        .replay(&counterexample, &invocation, "account", "audit")
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), ProofCategory::Violation);
    let Some(FrameChange::FieldWrite { object, field, .. }) =
        result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!((object.as_str(), field.as_str()), ("account", "audit"));

    // ALLOWED: the granted write proves, and its run is a frame the invocation respects.
    let allowed = generate_over(&fixture, "deposit");
    assert_eq!(prove(&allowed.frame), KaniRunOutcome::Verified);
    let mut account = subject::Account {
        balance: 5,
        audit: 0,
    };
    subject::deposit(&mut account);
    let invocation = twin.invocation("account", (5, 0), (account.balance, account.audit));
    let result = twin
        .replay(&counterexample, &invocation, "account", "balance")
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(DisagreementCause::Verdicts {
            proved: Verdict::from_category(ProofCategory::Violation),
            replayed: Verdict::from_category(ProofCategory::Success),
        })
    );
    assert!(result.found().is_none());
}
