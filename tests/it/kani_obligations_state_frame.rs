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

use native_twin::{Tamper, Twin};
use package::{
    application, code_id, corpus_package, key, literal, member, op, op_full, parameter_body,
    reference, Bound, PackageBuilder, NODE_DOMAIN, T_BOOLEAN, T_INTEGER,
};
use qsl_replay::{
    CallSiteRefusal, Category, DisagreementCause, FrameChange, FrameIdentityMismatch,
    ReplayRefusal, ReplayResult, Verdict, WitnessSettlement,
};
use quire_contract_codegen::{
    execute_kani_obligation, generate_state_frame_obligations, BoundNotResolvedCause,
    FrameReplayError, KaniExecutionRequest, KaniInstallation, KaniRunOutcome, StateComparison,
    StateFieldDomain, StateFrameHarness, StateFrameLoweringRefusal, StateFrameObligations,
    StateFrameProperty, StateFrameRefusal, StateFrameRequest, UnsupportedFrameEffect,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
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
    /// The name of the object's `balance` member, which the clause's reads carry in the graph.
    condition_field: &'static str,
    /// What the object's `balance` member holds, when it is not the `balance_bound` reference.
    member: Member,
}

/// The object's `balance` member, for the grounds on which it gives the clause no `i64` range.
#[derive(Clone, Copy, PartialEq)]
enum Member {
    /// A reference to the `balance_bound` range or plain integer type.
    Declared,
    /// A reference to a bound that is not an `integer_range`.
    RationalBound,
    /// A reference to an `integer_range` whose maximum is one past `i64::MAX`.
    WideRange,
    /// A literal value, not a reference.
    Literal,
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
        condition_field: "balance",
        member: Member::Declared,
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
    let balance_key = match (shape.member, shape.balance_bound) {
        (Member::RationalBound, _) => builder.bound(&Bound::Rational(1, 2, 1, 2)),
        (Member::WideRange, _) => builder.bound(&Bound::Raw {
            form: "integer_range",
            bounded: "integer",
            body: json!({"term": "aggregate", "members": [
                member("min", literal("integer", "0")),
                member("max", literal("integer", "9223372036854775808")),
            ]}),
            foreign: vec![],
        }),
        (Member::Literal, _) => key(T_INTEGER),
        (Member::Declared, Some((minimum, maximum))) => {
            builder.bound(&Bound::Integer(minimum, maximum))
        }
        (Member::Declared, None) => key(T_INTEGER),
    };
    let balance_value = match shape.member {
        Member::Literal => literal("integer", "0"),
        Member::Declared | Member::RationalBound | Member::WideRange => reference(&balance_key),
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
            member(shape.condition_field, balance_value),
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
    // The clause's reads are typed by an integer range even where the member gives none, so the
    // comparison is well typed and the refusal is the engine's, not IR's.
    let read_key = if shape.member == Member::Declared {
        balance_key.clone()
    } else {
        audit_key.clone()
    };
    let post_balance = field_read(
        &mut builder,
        shape.code(410),
        &object,
        shape.condition_field,
        &read_key,
    );
    let pre_balance = pre_read(&mut builder, shape.code(411), &post_balance, &read_key);
    let post_audit = field_read(&mut builder, shape.code(400), &object, "audit", &audit_key);
    let pre_audit = pre_read(&mut builder, shape.code(401), &post_audit, &audit_key);
    let other = code_id(shape.code(420)).digest.to_string();
    let post_other = field_read_through(
        &mut builder,
        shape.code(421),
        &other,
        (&object, shape.condition_field, &read_key),
    );
    let pre_other = pre_read(&mut builder, shape.code(422), &post_other, &read_key);
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

/// The state-clause and frame-effect harness sources, for the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let generated = generate(&fixture(&Shape::HEALTHY));
    vec![
        ("state clause", generated.postcondition.rust.contents),
        ("frame effect", generated.frame.rust.contents),
    ]
}

fn refusal(shape: &Shape, fields: &[&str]) -> StateFrameRefusal {
    let fixture = fixture(shape);
    generate_state_frame_obligations(&request(&fixture, fields))
        .expect_err("the request must be refused")
}

// ---- default lane ------------------------------------------------------------

/// A postcondition clause yields two separate harnesses whose identity is scoped to the
/// operation, anchor, frame and object, whose bounds and granted and forbidden fields are read
/// from the IR, and each carries exactly one non-vacuity cover.
///
/// Trace: FR-015-AC-7, FR-015-AC-26, FR-015-AC-27, FR-015-AC-28, TC-025
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
    // the model admits.
    let domain = |field: &str| StateFieldDomain {
        field: field.to_owned(),
        minimum: 0,
        maximum: 1000,
    };
    for harness in [&generated.postcondition, &generated.frame] {
        assert_eq!(
            harness.identity.domains,
            vec![domain("balance"), domain("audit")]
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
        // The cover ends the harness, after every assertion: an assertion that fails does not
        // return, so the cover cannot share a failing valuation (IR-451).
        let (assertion, cover) = (source.rfind("assert!("), source.find("kani::cover!("));
        assert!(
            assertion.zip(cover).is_some_and(|(a, c)| a < c),
            "the cover must follow the last assertion:\n{source}"
        );
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
    // The clause's own field is typed by a plain integer type, no bound: the target is the
    // unbounded type.
    assert_eq!(
        with(13, |shape| Shape {
            balance_bound: None,
            ..shape
        }),
        StateFrameRefusal::BoundNotResolved {
            field: "balance".to_owned(),
            cause: BoundNotResolvedCause::UnboundedType {
                target: package::id(&key(T_INTEGER))
            },
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
    let domain = |field: &str, minimum, maximum| StateFieldDomain {
        field: field.to_owned(),
        minimum,
        maximum,
    };
    let expected = vec![domain("balance", 5, 900), domain("audit", 0, 50)];
    assert_eq!(generated.postcondition.identity.domains, expected);
    assert_eq!(generated.frame.identity.domains, expected);
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
        StateFrameRefusal::NotLowered { refusal }
            if matches!(*refusal, StateFrameLoweringRefusal::InvalidInput { .. })
    ));
}

/// A field name read from the graph that is not a Rust identifier is a malformed clause at the
/// node: not `InvalidField` and not `ConditionNotSupported`. IR admits a name that is an ASCII
/// identifier but a Rust keyword, so a keyword reaches the engine in a clause's read and in a
/// frame's grant, and each is refused there. A field name the caller supplies that is not an identifier is still
/// `InvalidField`, naming it. Each assertion fails if the refusal moves to the other code.
///
/// Trace: FR-015-AC-67, TC-025
#[test]
fn tc_025_a_non_identifier_graph_field_name_is_a_malformed_clause() {
    let read = Shape {
        variant: 15,
        condition_field: "not-an-identifier",
        ..Shape::HEALTHY
    };
    assert_eq!(
        refusal(&read, &STATE_FIELDS),
        StateFrameRefusal::MalformedClause {
            at: code_id(read.code(410))
        }
    );
    // A Rust keyword is not an identifier to `syn` yet is one to IR's own check of a frame's
    // granted name, so IR admits it and the engine alone refuses it: in a read at the read's
    // node, and in a frame's grant at the frame node.
    let keyword_read = Shape {
        variant: 16,
        condition_field: "type",
        ..Shape::HEALTHY
    };
    assert_eq!(
        refusal(&keyword_read, &STATE_FIELDS),
        StateFrameRefusal::MalformedClause {
            at: code_id(keyword_read.code(410))
        }
    );
    let keyword_grant = Shape {
        variant: 17,
        condition_field: "type",
        modifies: &["type"],
        ..Shape::HEALTHY
    };
    assert_eq!(
        refusal(&keyword_grant, &STATE_FIELDS),
        StateFrameRefusal::MalformedClause {
            at: fixture(&keyword_grant).frame
        }
    );
    // The caller's own field list is the one place `InvalidField` is still named.
    assert_eq!(
        refusal(&Shape::HEALTHY, &["balance", "not-an-identifier"]),
        StateFrameRefusal::InvalidField {
            name: "not-an-identifier".to_owned()
        }
    );
}

/// The engine names the exact ground on which the object gives the clause's field no `i64`
/// range, with the member's `value.target` node when it has one: the record carries the cause,
/// so a swapped ground ships in it. Each assertion fails if its ground is reported as another.
///
/// Trace: FR-015-AC-66, TC-025
#[test]
fn tc_025_the_engine_names_the_ground_a_field_has_no_integer_range() {
    let cause = |variant, member, balance_bound| {
        let shape = Shape {
            variant,
            member,
            balance_bound,
            ..Shape::HEALTHY
        };
        match refusal(&shape, &STATE_FIELDS) {
            StateFrameRefusal::BoundNotResolved { field, cause } => (field, cause),
            other => panic!("expected BoundNotResolved, got {other:?}"),
        }
    };
    let bound_of = |bound: Bound| package::id(&bound.key());
    let field = "balance".to_owned();
    assert_eq!(
        cause(18, Member::RationalBound, None),
        (
            field.clone(),
            BoundNotResolvedCause::NotIntegerRange {
                bound: bound_of(Bound::Rational(1, 2, 1, 2))
            }
        )
    );
    assert_eq!(
        cause(19, Member::WideRange, None),
        (
            field.clone(),
            BoundNotResolvedCause::EndpointOutsideI64 {
                bound: bound_of(Bound::Raw {
                    form: "integer_range",
                    bounded: "integer",
                    body: json!({"term": "aggregate", "members": [
                        member("min", literal("integer", "0")),
                        member("max", literal("integer", "9223372036854775808")),
                    ]}),
                    foreign: vec![],
                })
            }
        )
    );
    assert_eq!(
        cause(20, Member::Literal, None),
        (field, BoundNotResolvedCause::ValueNotReference)
    );
    // `MemberAbsent` has no engine-level case: IR refuses a read of a name its object does not
    // declare when it admits the package, so the mapping test builds it directly.
}

/// `replay_frame` refuses an envelope whose `clause_node` or `occurrence_key` is not its
/// payload's, naming both; the same run with an agreeing envelope replays, so each refusal is the
/// difference and not the run.
///
/// Trace: TC-025
#[test]
fn tc_025_replay_frame_refuses_an_envelope_that_disagrees_with_its_payload() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    assert!(twin
        .replay_tampered(&invocation, "account", "audit", Tamper::Nothing)
        .is_ok());

    let clause = twin.replay_tampered(&invocation, "account", "audit", Tamper::ClauseNode);
    assert!(matches!(
        clause,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeFrame { .. })
    ));
    let occurrence = twin.replay_tampered(&invocation, "account", "audit", Tamper::Occurrence);
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
    let replay = twin.frame_replay(&invocation, "account", "audit");
    let payload = replay.packet.family_payload.as_ref().expect("a payload");
    assert_ne!(payload.anchor, payload.frame);
    assert_eq!(payload.occurrence.node(), payload.frame);
    assert_eq!(replay.packet.clause_node, Some(payload.frame));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&payload.occurrence)
    );
}

/// The frame replay's request and envelope both carry the `obligation_identity` the caller
/// supplied in `FrameReplayInputs` (the twin supplies `[1; 32]`), not a placeholder.
///
/// Trace: TC-025
#[test]
fn tc_025_the_frame_replay_request_and_envelope_carry_the_supplied_obligation_identity() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let replay = twin.frame_replay(&invocation, "account", "audit");
    assert_eq!(replay.wire.obligation_identity, [1; 32]);
    assert_eq!(replay.packet.obligation_identity, Some([1; 32]));
}

/// `FrameReplay::replay` returns QSL's result without Kani: a forbidden write settles a reproduced
/// violation that names the written field, and a write the frame grants is a respected frame.
///
/// Trace: FR-015-AC-36, TC-025
#[test]
fn tc_025_frame_replay_settles_a_forbidden_and_a_granted_write() {
    let twin = Twin::new();
    let forbidden = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&forbidden, "account", "audit")
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    let Some(FrameChange::FieldWrite { object, field, .. }) =
        result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!((object.as_str(), field.as_str()), ("account", "audit"));

    let granted = twin.invocation("account", (5, 0), (6, 0));
    let result = twin
        .frame_replay(&granted, "account", "balance")
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
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
    for operation in ["transfer", "withdraw"] {
        let refusal = twin
            .try_frame_replay(operation, &invocation, "account", "audit")
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

/// The `(balance, audit)` values Kani's concrete playback assigns the harness's two symbolic
/// `i64` fields, in the order the harness declares them.
fn playback_state(counterexample: &str) -> (i64, i64) {
    let values = counterexample
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("vec![") && !line.contains("concrete_vals"))
        .map(|line| {
            let bytes = line
                .trim_start_matches("vec![")
                .trim_end_matches("],")
                .split(',')
                .map(|byte| byte.trim().parse::<u8>().expect("a playback byte"))
                .collect::<Vec<_>>();
            i64::from_le_bytes(bytes.try_into().expect("eight bytes per i64"))
        })
        .collect::<Vec<_>>();
    let [balance, audit] = values[..] else {
        panic!("two symbolic fields in the playback, found {values:?}");
    };
    (balance, audit)
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

/// A violation whose valuation the cover must share is still a counterexample. Both state fields
/// are bounded to the single value 0, so the one valuation the harness can draw is the failing
/// one and the non-vacuity cover takes it too, whatever model the solver picks; a harness whose
/// cover preceded its assertion would print only the cover's playback here and classify as a
/// failure with no counterexample (IR-451).
///
/// Trace: FR-015-AC-7, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_a_violation_at_the_only_valuation_is_a_counterexample_not_the_covers_playback()
{
    let single = fixture(&Shape {
        variant: 20,
        balance_bound: Some((0, 0)),
        audit_bound: (0, 0),
        ..Shape::HEALTHY
    });
    let counterexample = falsified(
        prove(&generate_over(&single, "deposit_debiting").postcondition),
        "postcondition `post.balance >= pre.balance` failed",
    );
    assert_eq!(playback_state(&counterexample), (0, 0));
    let counterexample = falsified(
        prove(&generate_over(&single, "deposit_touching_audit").frame),
        "changed `audit`, which its frame does not modify",
    );
    assert_eq!(playback_state(&counterexample), (0, 0));
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
    let (balance, audit) = playback_state(&counterexample);
    let mut account = subject::Account { balance, audit };
    subject::deposit_touching_audit(&mut account);
    assert_ne!(
        account.audit, audit,
        "the native run reproduces the forbidden write"
    );
    let invocation = twin.invocation(
        "account",
        (balance, audit),
        (account.balance, account.audit),
    );
    let result = twin
        .replay(&invocation, "account", "audit")
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
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
        .replay(&invocation, "account", "balance")
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
        })
    );
    assert!(result.found().is_none());
}
