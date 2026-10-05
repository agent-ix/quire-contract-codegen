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
pub(crate) mod model;
#[path = "../state_frame_support/native_twin.rs"]
pub(crate) mod native_twin;
#[path = "../exact_scalar_support/package.rs"]
#[allow(clippy::duplicate_mod)]
mod package;
#[path = "../state_frame_support/subject.rs"]
// The Kani crates run every subject variant; the test binary executes only some natively.
#[allow(dead_code)]
pub(crate) mod subject;

use std::{fs, path::PathBuf, time::Duration};

use native_twin::{
    playback_text, Invocation, Run, Tamper, Twin, CLAUSES, INVOCATION_DOCUMENT, PRE_DOCUMENT,
};
use package::{
    application, code_id, corpus_package, key, literal, member, op, op_full, parameter_body,
    reference, Bound, PackageBuilder, FUNCTION, NODE_DOMAIN, T_BOOLEAN, T_INTEGER,
};
use qsl_replay::{
    CallSiteRefusal, Category, DisagreementCause, FrameChange, FrameIdentityMismatch,
    OperationSite, ReplayRefusal, ReplayResult, ReplaySource, Verdict, WitnessSettlement,
};
use quire_contract_codegen::{
    execute_kani_obligation, generate_state_frame_obligations, negotiate_kani_obligations,
    BoundNotResolvedCause, FrameReplay, FrameReplayError, InvalidObligationItem,
    KaniExecutionRequest, KaniInstallation, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, KaniRunOutcome, ModuleSymbol, ObligationDisposition, ObligationItem,
    ObligationKind, ObligationRecord, PreStateFault, ScopeMember, StateComparison,
    StateFieldDomain, StateFrameHarness, StateFrameIdentity, StateFrameLoweringRefusal,
    StateFrameObligations, StateFrameProperty, StateFrameRefusal, StateFrameRequest,
    StateFrameRole, UnsupportedFrameEffect, UnsupportedObligation, MAX_OBLIGATION_ITEMS,
    MAX_OBLIGATION_UNWIND,
};
use quire_contract_codegen::{HarnessSymbol, StateFrameRecordError};
use quire_contract_model::{CheckedNodeId, CheckedPackageReadLimits, CheckedPackageV2};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const OBJECT: u32 = 4001;
const REFERENCE: u32 = 4002;
const SELF: u32 = 4003;
const FRAME: u32 = 4004;
const ANCHOR: u32 = 4005;
const DEREF: u32 = 4006;
const OTHER: u32 = 4011;
const OTHER_OBJECT: u32 = 4012;
const RELATIONSHIP: u32 = 4013;
const RESULT: u32 = 4014;

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
    /// Whether the clause binds a `result` parameter, as FR-341 binds one exactly when the
    /// operation declares a result.
    result: bool,
    /// The name of the object's `balance` member, which the clause's reads carry in the graph.
    condition_field: &'static str,
    /// What the object's `balance` member holds, when it is not the `balance_bound` reference.
    member: Member,
    /// Types `audit` by the plain integer too, so with an unbounded `balance` no bound is
    /// reachable from the clause and lowering itself refuses it for a missing bound.
    audit_unbounded: bool,
    /// `0` shares the framed object, frame, anchor and `self` of every other shape of scope `0`,
    /// as two clauses of one operation do. Any other value gives the shape nodes of its own, so
    /// shapes of different scopes whose objects or frames differ share one package.
    scope: u32,
    /// Makes a `function` node, outside the lowering profile, reachable from the framed object,
    /// so lowering refuses the clause for an unsupported family.
    reaches_function: bool,
}

/// What the operation a fixture's clause anchors declares beyond `self`, for the state-clause
/// replay, which supports only an operation that declares neither.
#[derive(Clone, Copy)]
pub(crate) enum Declares {
    /// No parameter and no result.
    Nothing,
    /// One parameter, `other`.
    Parameter,
    /// A result.
    Result,
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
    /// `(post(self.balance) >= pre(self.balance)) and (post(self.balance) >= pre(self.balance))`:
    /// an operator outside the six integer comparisons.
    Conjunction,
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
        result: false,
        condition_field: "balance",
        member: Member::Declared,
        audit_unbounded: false,
        scope: 0,
        reaches_function: false,
    };

    fn code(&self, base: u32) -> u32 {
        50_000 + 1_000 * self.variant + base
    }

    /// The code of a node the shape shares with its scope: the object, its frame, anchor and
    /// parameters, and the dereference of `self`.
    fn scoped(&self, base: u32) -> u32 {
        if self.scope == 0 {
            base
        } else {
            80_000 + 1_000 * self.scope + base
        }
    }
}

fn node_ref(digest: &str) -> Value {
    json!({"domain": NODE_DOMAIN, "digest": digest})
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
    add_shape(&mut builder, shape);
    builder
}

/// A package holding every shape, each over its own object, frame, anchor and clause.
fn package_of(shapes: &[&Shape]) -> PackageBuilder {
    let mut builder = corpus_package();
    for shape in shapes {
        add_shape(&mut builder, shape);
    }
    builder
}

/// Adds the shape's nodes to `builder`. Every node code is the shape's own, so two shapes with
/// different variants share a package.
fn add_shape(builder: &mut PackageBuilder, shape: &Shape) {
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
    // A second integer range keeps a bound reachable when `balance` names none, unless the shape
    // asks for none to be.
    let audit_key = if shape.audit_unbounded {
        key(T_INTEGER)
    } else {
        builder.bound(&Bound::Integer(shape.audit_bound.0, shape.audit_bound.1))
    };
    let object = key(shape.scoped(OBJECT));
    let mut object_dependencies = vec![balance_key.clone(), audit_key.clone()];
    if shape.reaches_function {
        object_dependencies.push(code_id(FUNCTION).digest.to_string());
    }
    builder.node_with(
        &object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": [
            member(shape.condition_field, balance_value),
            member("audit", reference(&audit_key)),
        ]}),
        &object_dependencies,
    );
    let other_object = key(shape.scoped(OTHER_OBJECT));
    builder.node_with(
        &other_object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
        &[],
    );
    let relationship = key(shape.scoped(RELATIONSHIP));
    builder.node_with(
        &relationship,
        "relation",
        "relationship",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
        &[key(T_BOOLEAN)],
    );
    let reference_type = key(shape.scoped(REFERENCE));
    builder.node_with(
        &reference_type,
        "composite_type",
        "reference",
        &reference_type,
        json!({"term": "aggregate", "members": [reference(&object)]}),
        std::slice::from_ref(&object),
    );
    builder.code(
        shape.scoped(SELF),
        "value",
        "parameter",
        &reference_type,
        parameter_body("self", 0),
    );
    builder.code(
        shape.scoped(OTHER),
        "value",
        "parameter",
        &reference_type,
        parameter_body("other", 1),
    );
    builder.code(
        shape.scoped(RESULT),
        "value",
        "parameter",
        &key(T_BOOLEAN),
        parameter_body("result", 1),
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
    let frame = key(shape.scoped(FRAME));
    let anchor = key(shape.scoped(ANCHOR));
    builder.node_with(
        &frame,
        "state",
        "frame",
        &object,
        json!({"term": "frame", "modifies": modifies, "creates": creates, "deletes": deletes}),
        &frame_dependencies,
    );
    builder.node_with(
        &anchor,
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
        shape.scoped(DEREF),
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(shape.scoped(SELF)))],
        ),
        &[],
    );
    let deref = code_id(shape.scoped(DEREF)).digest.to_string();
    builder.application_bounded(
        shape.code(420),
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(shape.scoped(OTHER)))],
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
    let post_balance = field_read_through(
        builder,
        shape.code(410),
        &deref,
        (&object, shape.condition_field, &read_key),
    );
    let pre_balance = pre_read(builder, shape.code(411), &post_balance, &read_key);
    let post_audit = field_read_through(
        builder,
        shape.code(400),
        &deref,
        (&object, "audit", &audit_key),
    );
    let pre_audit = pre_read(builder, shape.code(401), &post_audit, &audit_key);
    let other = code_id(shape.code(420)).digest.to_string();
    let post_other = field_read_through(
        builder,
        shape.code(421),
        &other,
        (&object, shape.condition_field, &read_key),
    );
    let pre_other = pre_read(builder, shape.code(422), &post_other, &read_key);
    let zero = literal("integer", "0");
    let (left, right) = match shape.condition {
        Condition::PostGePre
        | Condition::PostLePre
        | Condition::Negated
        | Condition::Conjunction => (reference(&post_balance), reference(&pre_balance)),
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
    if matches!(shape.condition, Condition::Conjunction) {
        builder.application_bounded(
            shape.code(210),
            "expression",
            "binary",
            &key(T_BOOLEAN),
            application(
                "binary",
                op("quire.op.boolean.and"),
                &key(T_BOOLEAN),
                vec![reference(&condition), reference(&condition)],
            ),
            &[],
        );
        condition = code_id(shape.code(210)).digest.to_string();
    }
    // FR-341 binder order: `self`, then `result` when present, then the operation's parameters.
    let mut parameters = vec![reference(&key(shape.scoped(SELF)))];
    if shape.result {
        parameters.push(reference(&key(shape.scoped(RESULT))));
    }
    if matches!(shape.condition, Condition::BalanceAgainstOther) {
        parameters.push(reference(&key(shape.scoped(OTHER))));
    }
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
                reference(&anchor),
                reference(&condition),
            ],
        ),
        &[],
    );
}

pub(crate) struct Fixture {
    pub(crate) package: CheckedPackageV2,
    pub(crate) clause: CheckedNodeId,
    object: CheckedNodeId,
    anchor: CheckedNodeId,
    frame: CheckedNodeId,
}

/// The node ids a shape's nodes have in whatever package holds it.
struct Ids {
    clause: CheckedNodeId,
    object: CheckedNodeId,
    anchor: CheckedNodeId,
    frame: CheckedNodeId,
}

fn ids(shape: &Shape) -> Ids {
    let id = |digest: String| -> CheckedNodeId {
        serde_json::from_value(node_ref(&digest)).expect("node id")
    };
    Ids {
        clause: code_id(shape.code(300)),
        object: id(key(shape.scoped(OBJECT))),
        anchor: id(key(shape.scoped(ANCHOR))),
        frame: id(key(shape.scoped(FRAME))),
    }
}

fn fixture(shape: &Shape) -> Fixture {
    let package = package_for(shape).admit();
    let Ids {
        clause,
        object,
        anchor,
        frame,
    } = ids(shape);
    Fixture {
        package,
        clause,
        object,
        anchor,
        frame,
    }
}

/// The healthy fixture with `balance` typed by the plain integer: its member declares no range.
pub(crate) fn fixture_with_unbounded_balance() -> Fixture {
    fixture(&Shape {
        variant: 32,
        balance_bound: None,
        ..Shape::HEALTHY
    })
}

/// The node of the clause's `self` parameter, the node a state field's domain is declared on.
pub(crate) fn self_parameter() -> CheckedNodeId {
    code_id(SELF)
}

/// The healthy fixture, or one whose clause's operation declares `declares`.
pub(crate) fn fixture_declaring(declares: Declares) -> Fixture {
    fixture(&match declares {
        Declares::Nothing => Shape::HEALTHY,
        Declares::Parameter => Shape {
            variant: 30,
            condition: Condition::BalanceAgainstOther,
            ..Shape::HEALTHY
        },
        Declares::Result => Shape {
            variant: 31,
            result: true,
            ..Shape::HEALTHY
        },
    })
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

/// The check text of the frame harness's assertion over `audit`, as Kani prints it.
const FORBIDDEN_CHECK: &str =
    "operation `deposit` changed `audit`, which its frame does not modify";

/// The healthy fixture's frame harness identity, with the scope's anchor and frame the ones QSL
/// names for `deposit` in the twin's unit.
fn frame_harness(twin: &Twin) -> StateFrameIdentity {
    twin.aligned(
        &generate(&fixture(&Shape::HEALTHY)).frame.identity,
        "deposit",
    )
}

/// The run of `harness` whose playback binds `values`, in the harness's draw order.
fn run_of(harness: &StateFrameIdentity, values: [i64; 2]) -> Run {
    Run {
        playback: playback_text(harness, FORBIDDEN_CHECK, &values),
        harness: harness.clone(),
    }
}

/// The forbidden-write run at the pre state `(balance, audit) = (5, 0)`.
fn forbidden_run(twin: &Twin) -> Run {
    run_of(&frame_harness(twin), [5, 0])
}

/// A run whose harness is scoped to `operation`, for a replay that names that operation. Its
/// anchor and frame are the ones QSL names for the operation when the unit names it.
fn run_for_operation(twin: &Twin, operation: &str) -> Run {
    let mut harness = generate(&fixture(&Shape::HEALTHY)).frame.identity;
    harness.scope.operation = operation.to_owned();
    let harness = if twin.operation_site(operation).is_ok() {
        twin.aligned(&harness, operation)
    } else {
        harness
    };
    run_of(&harness, [5, 0])
}

/// The identity the replay of `operation` over `run` puts in the request, the invocation at
/// `(5, 0)` named for every operation.
fn minted(twin: &Twin, operation: &str, run: &Run) -> [u8; 32] {
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    twin.try_frame_replay(operation, &invocation, "account", "audit", run)
        .unwrap_or_else(|error| panic!("the replay of `{operation}` builds: {error}"))
        .wire
        .obligation_identity
}

/// The SHA-256 of the hand-written RFC 8785 text of the frame preimage of `site`: the frame node
/// as `function`, its occurrence as `declaration`, the `frame` kind and no arguments.
fn hand_written_identity(site: &OperationSite) -> [u8; 32] {
    let origin = site.frame_occurrence.origin();
    let text = format!(
        "{{\"arguments\":[],\"declaration\":{{\"node\":\"{}\",\"ordinal\":{},\"role\":\"{}\"}},\
         \"function\":\"{}\",\"kind\":\"frame\"}}",
        site.frame_occurrence.node(),
        origin.ordinal(),
        origin.role(),
        site.frame,
    );
    Sha256::digest(text.as_bytes()).into()
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
        cause(21, Member::Literal, None),
        (field, BoundNotResolvedCause::ValueNotReference)
    );
    // `MemberAbsent` has no engine-level case: IR refuses a read of a name its object does not
    // declare when it admits the package, so the mapping test builds it directly.
}

// ---- the StateFrame arm of negotiation (IR-461) -----------------------------

/// Every shape the arm's tests negotiate, in one admitted package, each over nodes of its own.
/// The index of a shape in this list is its `const` below.
const PRE: usize = 0;
const OK: usize = 1;
const LITERAL_MEMBER: usize = 2;
const TWO_FIELDS: usize = 3;
const RELATIONSHIP_FRAME: usize = 4;
const GRANTS_ALL: usize = 5;
const UNBOUNDED_MEMBER: usize = 6;
const NO_BOUND_REACHABLE: usize = 7;
const REACHES_FUNCTION: usize = 8;
const NEGATION: usize = 9;
const LITERAL_OPERAND: usize = 10;
const CONJUNCTION: usize = 11;
const OTHER_PARAMETER: usize = 12;
const SAME_SIDE: usize = 13;
const CREATES: usize = 14;
const DELETES: usize = 15;
const FOREIGN_FIELD: usize = 16;
const RATIONAL_BOUND: usize = 17;
const WIDE_RANGE: usize = 18;
const KEYWORD_READ: usize = 19;
const SPARE_A: usize = 20;
const SPARE_B: usize = 21;
const SPARE_C: usize = 22;
const NEGATION_GRANTS_ALL: usize = 23;

fn arm_shapes() -> Vec<Shape> {
    let edits: Vec<fn(Shape) -> Shape> = vec![
        |shape| Shape {
            clause: "precondition",
            ..shape
        },
        |shape| shape,
        |shape| Shape {
            member: Member::Literal,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstAudit,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::GrantsRelationship,
            ..shape
        },
        |shape| Shape {
            modifies: &["audit", "balance"],
            ..shape
        },
        |shape| Shape {
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            balance_bound: None,
            audit_unbounded: true,
            ..shape
        },
        |shape| Shape {
            reaches_function: true,
            ..shape
        },
        |shape| Shape {
            condition: Condition::Negated,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstLiteral,
            ..shape
        },
        |shape| Shape {
            condition: Condition::Conjunction,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstOther,
            ..shape
        },
        |shape| Shape {
            condition: Condition::PostGePost,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::Creates,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::Deletes,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::GrantsForeignField,
            ..shape
        },
        |shape| Shape {
            member: Member::RationalBound,
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            member: Member::WideRange,
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            condition_field: "type",
            ..shape
        },
        |shape| shape,
        |shape| shape,
        |shape| shape,
        |shape| Shape {
            condition: Condition::Negated,
            modifies: &["audit", "balance"],
            ..shape
        },
    ];
    edits
        .into_iter()
        .enumerate()
        .map(|(index, edit)| {
            let scope = u32::try_from(index + 1).expect("a small index");
            edit(Shape {
                variant: 40 + scope,
                scope,
                ..Shape::HEALTHY
            })
        })
        .collect()
}

/// A package of `shapes`, with the ids of each shape's nodes.
struct World {
    package: CheckedPackageV2,
    ids: Vec<Ids>,
}

fn world_of(shapes: &[Shape]) -> World {
    // The corpus package alone nearly fills the default byte ceiling, so a package of many shapes
    // is read under a larger one.
    let limits = CheckedPackageReadLimits {
        bytes: 8 << 20,
        ..CheckedPackageReadLimits::bounded()
    };
    World {
        package: package_of(&shapes.iter().collect::<Vec<_>>()).admit_with(limits),
        ids: shapes.iter().map(ids).collect(),
    }
}

/// The package of every arm shape, admitted once.
fn world() -> &'static World {
    static WORLD: std::sync::OnceLock<World> = std::sync::OnceLock::new();
    WORLD.get_or_init(|| world_of(&arm_shapes()))
}

/// A second admitted package, holding one shape the first also holds.
fn other_world() -> &'static World {
    static WORLD: std::sync::OnceLock<World> = std::sync::OnceLock::new();
    WORLD.get_or_init(|| world_of(&arm_shapes()[OK..=OK]))
}

/// The subject the request's own `subject_path` names, which no `StateFrame` item reads.
const REQUEST_SUBJECT: &str = "crate::request::unrelated";

/// The fields of a `StateFrame` item, so a test can change one and keep the rest.
#[derive(Clone, Copy)]
struct Spec<'a> {
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
    role: StateFrameRole,
    state_path: &'a str,
    state_fields: &'a [&'a str],
    subject_path: &'a str,
}

impl<'a> Spec<'a> {
    fn item(self) -> ObligationItem<'a> {
        ObligationItem::StateFrame {
            package: self.package,
            clause: self.clause,
            role: self.role,
            state_path: self.state_path,
            state_fields: self.state_fields,
            subject_path: self.subject_path,
        }
    }
}

fn spec(shape: usize, role: StateFrameRole) -> Spec<'static> {
    Spec {
        package: &world().package,
        clause: &world().ids[shape].clause,
        role,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
    }
}

fn item(shape: usize, role: StateFrameRole) -> ObligationItem<'static> {
    spec(shape, role).item()
}

fn negotiate(items: &[ObligationItem<'_>]) -> KaniObligationOutcome {
    negotiate_kani_obligations(&KaniObligationRequest {
        items,
        subject_path: REQUEST_SUBJECT,
        unwind: 4,
    })
    .expect("a well-formed request")
}

/// The record `item` has in a request holding it alone, placed at `request_index`.
fn alone(item: ObligationItem<'_>, request_index: usize) -> ObligationRecord {
    let record = negotiate(&[item]).records()[0].clone();
    ObligationRecord {
        request_index,
        ..record
    }
}

fn refusal_of(record: &ObligationRecord) -> &StateFrameRefusal {
    match &record.disposition {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::StateFrameRefused { refusal },
        } => refusal,
        other => panic!("expected unsupported state_frame_refused, got {other:?}"),
    }
}

fn is_supported(record: &ObligationRecord) -> bool {
    matches!(record.disposition, ObligationDisposition::Supported { .. })
}

fn emitted_state_frame(
    outcome: KaniObligationOutcome,
) -> (Vec<ObligationRecord>, Vec<StateFrameHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted {
            records,
            state_frame_harnesses,
            ..
        } => (records, state_frame_harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

/// One record per item in request order, `postcondition` for the contract role and `frame` for
/// the frame role, whatever an earlier item's refusal: a request of three refused items has three
/// records, and each later item's record is the record it has alone. An implementation that
/// returns at the first refusal reads one record, and fails the counts here.
///
/// Trace: FR-015-AC-59, TC-025
#[test]
fn tc_025_every_state_frame_item_has_a_record_whatever_an_earlier_item_refused() {
    use StateFrameRole::{Contract, Frame};
    let items = [
        item(PRE, Contract),
        item(OK, Contract),
        item(OK, Frame),
        item(UNBOUNDED_MEMBER, Contract),
        item(LITERAL_MEMBER, Contract),
        item(TWO_FIELDS, Contract),
        item(RELATIONSHIP_FRAME, Frame),
        item(GRANTS_ALL, Frame),
        item(GRANTS_ALL, Contract),
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    assert_eq!(records.len(), items.len(), "one record per item");
    assert_eq!(
        records
            .iter()
            .map(|record| record.request_index)
            .collect::<Vec<_>>(),
        (0..items.len()).collect::<Vec<_>>()
    );
    let kinds = records.iter().map(|record| record.kind).collect::<Vec<_>>();
    let (post, frame) = (
        Some(ObligationKind::Postcondition),
        Some(ObligationKind::Frame),
    );
    assert_eq!(
        kinds,
        [post, post, frame, post, post, post, frame, frame, post]
    );
    assert!(matches!(
        refusal_of(&records[0]),
        StateFrameRefusal::NotAPostcondition { clause } if clause == "precondition"
    ));
    let supported = records.iter().map(is_supported).collect::<Vec<_>>();
    assert_eq!(
        supported,
        [false, true, true, false, false, false, false, false, true]
    );
    assert!(matches!(
        records[3].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    assert_eq!(harnesses.len(), 3, "one harness per supported record");
    // Each later item's record is the record it has alone.
    for (index, item) in items.iter().enumerate().skip(1) {
        assert_eq!(records[index], alone(*item, index), "item {index}");
    }

    // Three refused items are three records.
    let refused = [
        item(PRE, Contract),
        item(LITERAL_MEMBER, Contract),
        item(TWO_FIELDS, Contract),
    ];
    let outcome = negotiate(&refused);
    assert_eq!(outcome.records().len(), 3);
    assert!(outcome.records().iter().all(|record| !is_supported(record)));

    // A later invalid item is a record too, and the request is rejected with every record.
    let bad_path = Spec {
        state_path: "not a path",
        ..spec(OK, Contract)
    }
    .item();
    let outcome = negotiate(&[item(PRE, Contract), bad_path]);
    let KaniObligationOutcome::Rejected { records } = outcome else {
        panic!("an invalid item rejects the request");
    };
    assert_eq!(records.len(), 2);
    assert_eq!(records[1], alone(bad_path, 1));
}

/// A supported item's harness leaves in `state_frame_harnesses`, in request order and
/// byte-identical to the harness `generate_state_frame_obligations` returns for its clause and
/// role; the record carries its symbol. The harness calls the item's own subject path, and the
/// request's `subject_path` is not read.
///
/// This does not back FR-015-AC-60, which stays planned and is not traced here: the comparison
/// with `generate_state_frame_role` is not testable from `tests/it` (the function is
/// crate-private and no V2 package builder exists under `src/`); byte identity holds by
/// construction, and this test compares against the public entry for a clause whose roles both
/// succeed.
///
/// Trace: TC-025
#[test]
fn tc_025_a_supported_state_frame_item_returns_the_harness_the_engine_generates() {
    use StateFrameRole::{Contract, Frame};
    let items = [item(OK, Frame), item(PRE, Contract), item(OK, Contract)];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let engine = generate_state_frame_obligations(&StateFrameRequest {
        package: &world().package,
        clause: &world().ids[OK].clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the OK shape generates");
    // Request order: the frame item came first.
    assert_eq!(
        harnesses,
        vec![engine.frame.clone(), engine.postcondition.clone()]
    );
    for (record, harness) in [&records[0], &records[2]].into_iter().zip(&harnesses) {
        assert_eq!(
            record.disposition,
            ObligationDisposition::Supported {
                harness_symbol: harness.identity.harness_symbol.clone()
            }
        );
    }
    assert!(
        !is_supported(&records[1]),
        "the refused item has no harness"
    );

    // The harness calls the item's own subject, never the request's.
    let own_subject = Spec {
        subject_path: "crate::subject::withdraw",
        ..spec(OK, Contract)
    }
    .item();
    let (_, harnesses) = emitted_state_frame(negotiate(&[own_subject]));
    let source = &harnesses[0].rust.contents;
    assert!(source.contains("crate::subject::withdraw(&mut post)"));
    assert!(!source.contains(REQUEST_SUBJECT));
    assert_eq!(
        harnesses[0].identity.subject_path,
        "crate::subject::withdraw"
    );
}

/// A clause whose field's member is typed by a plain integer is `requires_bound` with that
/// member's `value.target`; a clause whose lowering is a requires-bound record is
/// `requires_bound` with the record's type. Neither emits a harness, and the other role of the
/// first, whose frame needs no bound, still does.
///
/// Trace: FR-015-AC-61, TC-025
#[test]
fn tc_025_a_state_frame_item_with_no_bound_is_requires_bound() {
    use StateFrameRole::{Contract, Frame};
    let unbounded_integer = package::id(&key(T_INTEGER));
    let items = [
        item(UNBOUNDED_MEMBER, Contract),
        item(NO_BOUND_REACHABLE, Contract),
        item(NO_BOUND_REACHABLE, Frame),
        item(UNBOUNDED_MEMBER, Frame),
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let requires_bound = |record: &ObligationRecord| match &record.disposition {
        ObligationDisposition::RequiresBound { unbounded_type } => unbounded_type.clone(),
        other => panic!("expected requires_bound, got {other:?}"),
    };
    assert_eq!(requires_bound(&records[0]), unbounded_integer);
    assert_eq!(requires_bound(&records[1]), unbounded_integer);
    assert_eq!(requires_bound(&records[2]), unbounded_integer);
    assert!(is_supported(&records[3]));
    // Only the frame of the clause whose member alone is unbounded has a harness.
    assert_eq!(harnesses.len(), 1);
    assert_eq!(
        harnesses[0].identity.clause,
        world().ids[UNBOUNDED_MEMBER].clause
    );
}

/// A lowering record for an unsupported family is `unsupported` `NoFiniteEncoding` naming the
/// record's node and family, and a node that is not a `state_clause` is `UnknownNodeKind`; neither
/// has a harness.
///
/// Trace: FR-015-AC-62, TC-025
#[test]
fn tc_025_a_state_frame_item_outside_the_encoding_has_a_named_reason() {
    use StateFrameRole::{Contract, Frame};
    let frame_node = Spec {
        clause: &world().ids[OK].frame,
        ..spec(OK, Contract)
    }
    .item();
    let items = [
        item(REACHES_FUNCTION, Contract),
        item(REACHES_FUNCTION, Frame),
        frame_node,
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let no_finite_encoding = ObligationDisposition::Unsupported {
        reason: UnsupportedObligation::NoFiniteEncoding {
            node_id: code_id(FUNCTION),
            node_tag: "function",
        },
    };
    assert_eq!(records[0].disposition, no_finite_encoding);
    assert_eq!(records[1].disposition, no_finite_encoding);
    assert_eq!(
        records[2].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::UnknownNodeKind {
                node_id: world().ids[OK].frame.clone(),
                node_tag: "state".to_owned(),
                semantic_form: "frame".to_owned(),
            }
        }
    );
    assert!(harnesses.is_empty());
}

/// Every shape the arm does not render is `unsupported` `StateFrameRefused` carrying the
/// engine's refusal, never `NoFiniteEncoding`, and has no harness. Each row names the role whose
/// harness the ground refuses; the rows that ground both roles are asked of both. A row fails if
/// its refusal moves to another reason or to another variant.
///
/// Trace: FR-015-AC-63, TC-025
#[test]
fn tc_025_a_state_frame_item_the_arm_does_not_render_carries_the_engines_refusal() {
    use StateFrameRole::{Contract, Frame};
    type Check = fn(&StateFrameRefusal) -> bool;
    let rows: Vec<(&str, usize, StateFrameRole, Check)> = vec![
        ("negation", NEGATION, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        ("literal operand", LITERAL_OPERAND, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        (
            "operator outside the six",
            CONJUNCTION,
            Contract,
            |refusal| matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. }),
        ),
        ("another parameter", OTHER_PARAMETER, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        ("two fields", TWO_FIELDS, Contract, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::ObservationsDiffer { left, right }
                    if left == "balance" && right == "audit"
            )
        }),
        ("one side twice", SAME_SIDE, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ObservationsSameSide)
        }),
        (
            "member bound not an integer range",
            RATIONAL_BOUND,
            Contract,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::BoundNotResolved {
                        cause: BoundNotResolvedCause::NotIntegerRange { .. },
                        ..
                    }
                )
            },
        ),
        ("endpoint outside i64", WIDE_RANGE, Contract, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::BoundNotResolved {
                    cause: BoundNotResolvedCause::EndpointOutsideI64 { .. },
                    ..
                }
            )
        }),
        (
            "member value not a reference",
            LITERAL_MEMBER,
            Contract,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::BoundNotResolved {
                        cause: BoundNotResolvedCause::ValueNotReference,
                        ..
                    }
                )
            },
        ),
        ("frame creates", CREATES, Frame, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::FrameEffectUnsupported {
                    effect: UnsupportedFrameEffect::Creates,
                    ..
                }
            )
        }),
        ("frame deletes", DELETES, Frame, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::FrameEffectUnsupported {
                    effect: UnsupportedFrameEffect::Deletes,
                    ..
                }
            )
        }),
        (
            "frame grants a relationship",
            RELATIONSHIP_FRAME,
            Frame,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::FrameEffectUnsupported {
                        effect: UnsupportedFrameEffect::Relationship,
                        ..
                    }
                )
            },
        ),
        (
            "frame grants a foreign field",
            FOREIGN_FIELD,
            Frame,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::FrameEffectUnsupported {
                        effect: UnsupportedFrameEffect::ForeignField,
                        ..
                    }
                )
            },
        ),
        ("frame grants every field", GRANTS_ALL, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::NothingForbidden { .. })
        }),
        // Grounds of the clause itself refuse both roles.
        ("a precondition, contract", PRE, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::NotAPostcondition { .. })
        }),
        ("a precondition, frame", PRE, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::NotAPostcondition { .. })
        }),
        (
            "a malformed read, contract",
            KEYWORD_READ,
            Contract,
            |refusal| matches!(refusal, StateFrameRefusal::MalformedClause { .. }),
        ),
        ("a malformed read, frame", KEYWORD_READ, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::MalformedClause { .. })
        }),
    ];
    let items = rows
        .iter()
        .map(|(_, shape, role, _)| item(*shape, *role))
        .collect::<Vec<_>>();
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    assert_eq!(records.len(), rows.len());
    for ((label, _, _, check), record) in rows.iter().zip(&records) {
        assert!(
            check(refusal_of(record)),
            "{label}: got {:?}",
            record.disposition
        );
    }
    assert!(harnesses.is_empty(), "no refused item has a harness");
    // The effect a frame refusal names is the frame's own node.
    let StateFrameRefusal::FrameEffectUnsupported { frame, .. } = refusal_of(&records[9]) else {
        panic!("a frame effect");
    };
    assert_eq!(frame, &world().ids[CREATES].frame);
}

/// An invalid item is a record of its own with its own code, the request is `Rejected` with every
/// item's record, and a supported item of the request is accounted but its bytes are withheld.
///
/// Trace: FR-015-AC-64, TC-025
#[test]
fn tc_025_an_invalid_state_frame_item_rejects_the_request_with_every_record() {
    use StateFrameRole::{Contract, Frame};
    let absent = package::id(&key(9999));
    let items = [
        item(OK, Contract),
        Spec {
            state_path: "not a path",
            ..spec(SPARE_A, Contract)
        }
        .item(),
        Spec {
            subject_path: "not a path",
            ..spec(SPARE_A, Frame)
        }
        .item(),
        Spec {
            state_fields: &["balance", "bad-name"],
            ..spec(SPARE_B, Contract)
        }
        .item(),
        Spec {
            state_fields: &["audit"],
            ..spec(SPARE_C, Contract)
        }
        .item(),
        Spec {
            clause: &absent,
            ..spec(OK, Frame)
        }
        .item(),
        item(OK, Contract),
        Spec {
            package: &other_world().package,
            clause: &other_world().ids[0].clause,
            ..spec(OK, Frame)
        }
        .item(),
    ];
    let KaniObligationOutcome::Rejected { records } = negotiate(&items) else {
        panic!("an invalid item rejects the request, and a rejection carries no harness");
    };
    assert_eq!(records.len(), items.len());
    let invalid = |record: &ObligationRecord| match &record.disposition {
        ObligationDisposition::InvalidRequest { reason } => reason.clone(),
        other => panic!("expected invalid_request, got {other:?}"),
    };
    // Each engine-invalid item alone, beside a supported one, rejects the request: the engine's
    // invalid grounds are not only the arm's duplicate and mixed-package grounds.
    for engine_invalid in &items[1..=5] {
        let outcome = negotiate(&[item(OK, Contract), *engine_invalid]);
        assert!(
            matches!(outcome, KaniObligationOutcome::Rejected { .. }),
            "{:?}",
            outcome.records()[1]
        );
    }
    assert!(is_supported(&records[0]), "accounted though withheld");
    assert_eq!(
        invalid(&records[1]),
        InvalidObligationItem::InvalidStatePath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[2]),
        InvalidObligationItem::InvalidStatePath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[3]),
        InvalidObligationItem::InvalidStateField {
            name: "bad-name".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[4]),
        InvalidObligationItem::InvalidStateField {
            name: "balance".to_owned()
        }
    );
    assert_eq!(invalid(&records[5]), InvalidObligationItem::UnknownNode);
    assert_eq!(
        invalid(&records[6]),
        InvalidObligationItem::DuplicateItem { first_index: 0 }
    );
    assert_eq!(
        invalid(&records[7]),
        InvalidObligationItem::MixedStatePackages
    );
}

/// A request holding `StateFrame` items is refused whole, with the existing error and no record,
/// for no items, too many, an unparsable request subject path (checked though no `StateFrame`
/// item reads it) and an unwind bound outside the range.
///
/// Trace: FR-015-AC-65, TC-025
#[test]
fn tc_025_a_state_frame_request_is_refused_whole_for_the_requests_own_faults() {
    let one = [item(OK, StateFrameRole::Contract)];
    let refuse = |items: &[ObligationItem<'_>], subject_path: &str, unwind: u32| {
        negotiate_kani_obligations(&KaniObligationRequest {
            items,
            subject_path,
            unwind,
        })
        .expect_err("the request is refused")
    };
    assert_eq!(
        refuse(&[], REQUEST_SUBJECT, 4),
        KaniObligationError::EmptyRequest
    );
    let too_many = vec![one[0]; MAX_OBLIGATION_ITEMS + 1];
    assert_eq!(
        refuse(&too_many, REQUEST_SUBJECT, 4),
        KaniObligationError::TooManyItems {
            count: MAX_OBLIGATION_ITEMS + 1
        }
    );
    assert_eq!(
        refuse(&one, "not a path", 4),
        KaniObligationError::InvalidSubjectPath
    );
    assert_eq!(
        refuse(&one, REQUEST_SUBJECT, 0),
        KaniObligationError::InvalidUnwind { unwind: 0 }
    );
    assert_eq!(
        refuse(&one, REQUEST_SUBJECT, MAX_OBLIGATION_UNWIND + 1),
        KaniObligationError::InvalidUnwind {
            unwind: MAX_OBLIGATION_UNWIND + 1
        }
    );
}

/// The single-clause entry names the refusal it always named first for a clause both roles
/// refuse: the condition's shape before a frame granting every field. The first assertion fails
/// if the roles are simply run frame-first; the second pins the refusal for a missing granted
/// field and does not tell the orders apart.
///
/// Trace: FR-015-AC-29, TC-025
#[test]
fn tc_025_the_single_clause_entry_keeps_its_first_refusal_when_both_roles_refuse() {
    let refuse = |shape: usize, fields: &[&str]| {
        generate_state_frame_obligations(&StateFrameRequest {
            package: &world().package,
            clause: &world().ids[shape].clause,
            state_path: STATE_PATH,
            state_fields: fields,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        })
        .expect_err("both roles refuse")
    };
    // A negation (contract) on a frame granting every field (frame).
    assert!(matches!(
        refuse(NEGATION_GRANTS_ALL, &STATE_FIELDS),
        StateFrameRefusal::ConditionNotSupported { .. }
    ));
    // A granted field the state lacks (frame) precedes the check that nothing is forbidden.
    assert_eq!(
        refuse(GRANTS_ALL, &["audit"]),
        StateFrameRefusal::UnknownStateField {
            field: "balance".to_owned()
        }
    );
}

/// The two roles of a clause settle independently: a frame granting every field leaves the
/// contract item `supported`, a negation leaves the frame item `supported`, and a refused
/// lowering refuses both alike. `generate_state_frame_obligations` returns the refusal of the
/// failing role, and a supported harness is the role's own.
///
/// This does not back FR-015-AC-68, which stays planned and is not traced here: the comparison
/// with `generate_state_frame_role` is not testable from `tests/it` (the function is
/// crate-private and no V2 package builder exists under `src/`); byte identity holds by
/// construction, and the harnesses are checked only for their role and clause.
///
/// Trace: TC-025
#[test]
fn tc_025_the_two_roles_of_a_state_clause_settle_independently() {
    use StateFrameRole::{Contract, Frame};
    let engine = |shape: usize| {
        generate_state_frame_obligations(&StateFrameRequest {
            package: &world().package,
            clause: &world().ids[shape].clause,
            state_path: STATE_PATH,
            state_fields: &STATE_FIELDS,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        })
        .expect_err("the clause has a role the engine refuses")
    };
    let items = [
        item(GRANTS_ALL, Contract),
        item(GRANTS_ALL, Frame),
        item(NEGATION, Contract),
        item(NEGATION, Frame),
        item(REACHES_FUNCTION, Contract),
        item(REACHES_FUNCTION, Frame),
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    assert_eq!(
        records.iter().map(is_supported).collect::<Vec<_>>(),
        [true, false, false, true, false, false]
    );
    // Each supported role's harness is its role's, for its own clause.
    assert_eq!(harnesses.len(), 2);
    assert!(matches!(
        harnesses[0].identity.property,
        StateFrameProperty::Postcondition { .. }
    ));
    assert_eq!(harnesses[0].identity.clause, world().ids[GRANTS_ALL].clause);
    assert!(matches!(
        harnesses[1].identity.property,
        StateFrameProperty::Frame { .. }
    ));
    assert_eq!(harnesses[1].identity.clause, world().ids[NEGATION].clause);
    // The engine's single entry returns the refusal of the role that fails.
    assert_eq!(&engine(GRANTS_ALL), refusal_of(&records[1]));
    assert_eq!(&engine(NEGATION), refusal_of(&records[2]));
    // A refused lowering refuses both items alike.
    assert_eq!(records[4].disposition, records[5].disposition);
    assert!(matches!(
        engine(REACHES_FUNCTION),
        StateFrameRefusal::NotLowered { .. }
    ));
}

/// The shape of the real-Kani test whose only valuation falsifies the contract. It owns fixture
/// variant 20.
fn single_valuation_shape() -> Shape {
    Shape {
        variant: 20,
        balance_bound: Some((0, 0)),
        audit_bound: (0, 0),
        ..Shape::HEALTHY
    }
}

/// Each variant owns its node codes, and the fixture registry panics when one code is bound to
/// two bodies in a process. The ignored real-Kani test's fixture is built here with the default
/// lane's, so a default-lane variant that reuses its variant fails every run, not only
/// `--include-ignored`.
///
/// Trace: TC-025
#[test]
fn tc_025_the_kani_lanes_fixture_variant_is_not_shared_with_a_default_lane_variant() {
    let single = fixture(&single_valuation_shape());
    assert_eq!(single.clause, code_id(single_valuation_shape().code(300)));
}

/// `replay_frame` refuses an envelope whose `clause_node` or `occurrence_key` is not its
/// payload's, naming both; the same run with an agreeing envelope replays, so each refusal is the
/// difference and not the run.
///
/// Trace: TC-025
#[test]
fn tc_025_replay_frame_refuses_an_envelope_that_disagrees_with_its_payload() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    assert!(twin
        .replay_tampered(&invocation, "account", "audit", &run, Tamper::Nothing)
        .is_ok());

    let clause = twin.replay_tampered(&invocation, "account", "audit", &run, Tamper::ClauseNode);
    assert!(matches!(
        clause,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeFrame { .. })
    ));
    let occurrence =
        twin.replay_tampered(&invocation, "account", "audit", &run, Tamper::Occurrence);
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
    let replay = twin.frame_replay(&invocation, "account", "audit", &forbidden_run(&twin));
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
    let run = forbidden_run(&twin);
    let forbidden = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&forbidden, "account", "audit", &run)
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
        .frame_replay(&granted, "account", "balance", &run)
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
        let run = run_for_operation(&twin, operation);
        let refusal = twin
            .try_frame_replay(operation, &invocation, "account", "audit", &run)
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

// ---- frame obligation identity, witness and ties (FR-024, IR-459) -------------

/// The refusal of a replay that must be refused.
fn refused(result: Result<FrameReplay, FrameReplayError>) -> FrameReplayError {
    match result {
        Ok(_) => panic!("the replay must be refused"),
        Err(error) => error,
    }
}

/// The frame replay of `operation` over `run` at the pre state `(5, 0)`.
fn replay_of(twin: &Twin, operation: &str, run: &Run) -> Result<FrameReplay, FrameReplayError> {
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    twin.try_frame_replay(operation, &invocation, "account", "audit", run)
}

/// The request's and the envelope's obligation identity equal the SHA-256 of the hand-written
/// preimage of the site `qsl_replay::call_site` returns for the operation, and the envelope's
/// `clause_node` and `occurrence_key` are the same `function` and `declaration` members.
///
/// Trace: FR-024-AC-22, FR-024-AC-24, TC-035
#[test]
fn tc_035_the_request_and_the_envelope_carry_the_identity_minted_from_the_site() {
    let twin = Twin::new();
    let site = twin
        .operation_site("deposit")
        .expect("the operation is located");
    let replay = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");
    let expected = hand_written_identity(&site);
    assert_eq!(replay.wire.obligation_identity, expected);
    assert_eq!(replay.packet.obligation_identity, Some(expected));
    assert_eq!(replay.packet.clause_node, Some(site.frame));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&site.frame_occurrence)
    );
    assert_ne!(expected, [1; 32], "no stand-in digest reaches the request");
}

/// A harness whose frame grants nothing, replayed over a unit whose frame grants nothing, gives
/// a different identity in the request and in the envelope; each is the minted identity of its
/// own site.
///
/// Trace: FR-024-AC-24, TC-035
#[test]
fn tc_035_a_changed_grant_changes_the_identity_in_the_request_and_the_envelope() {
    let twin = Twin::new();
    let base = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");

    let granting_nothing = Twin::build(&[], &CLAUSES, 0);
    let emptied = fixture(&Shape {
        variant: 1,
        modifies: &[],
        ..Shape::HEALTHY
    });
    let harness = granting_nothing.aligned(&generate(&emptied).frame.identity, "deposit");
    let changed =
        replay_of(&granting_nothing, "deposit", &run_of(&harness, [5, 0])).expect("builds");

    assert_ne!(
        changed.wire.obligation_identity,
        base.wire.obligation_identity
    );
    assert_ne!(
        changed.packet.obligation_identity,
        base.packet.obligation_identity
    );
    let site = granting_nothing
        .operation_site("deposit")
        .expect("the operation is located");
    assert_eq!(
        changed.wire.obligation_identity,
        hand_written_identity(&site)
    );
    assert_eq!(
        changed.packet.obligation_identity,
        Some(changed.wire.obligation_identity)
    );
}

/// The identity is measured through `qsl_replay::call_site`: it changes with the frame's grants
/// and between two operations of one object whose frames are equal text, and it does not change
/// when the unit is shifted by blank lines or when a second clause names the same operation.
///
/// Trace: FR-024-AC-21, TC-035
#[test]
fn tc_035_the_frame_identity_follows_what_call_site_names() {
    let identity =
        |twin: &Twin, operation: &str| minted(twin, operation, &run_for_operation(twin, operation));
    let twin = Twin::new();
    let base = identity(&twin, "deposit");

    // Two units that differ only in the frame's `modifies` grants.
    let both = Twin::build(&["balance", "audit"], &CLAUSES, 0);
    assert_ne!(base, identity(&both, "deposit"), "grants");

    // Two operations of one object whose frames are equal text.
    let two_operations = Twin::build(
        &model::GRANTED,
        &[CLAUSES[0], ("TransferKeepsBalance", "transfer", "balance")],
        0,
    );
    assert_ne!(
        identity(&two_operations, "deposit"),
        identity(&two_operations, "transfer"),
        "operations with equal frame text"
    );

    // The same operation in a unit shifted by blank lines.
    let shifted = Twin::build(&model::GRANTED, &CLAUSES, 3);
    assert_eq!(base, identity(&shifted, "deposit"), "blank lines");

    // Two postcondition clauses of one operation: one frame, whatever the harness's clause.
    let first = twin.clause_site("BalanceNeverDrops").expect("a clause");
    let second = twin.clause_site("AuditNeverDrops").expect("a clause");
    assert_ne!(first.node, second.node, "the clauses are two nodes");
    let mut harness = frame_harness(&twin);
    harness.clause = harness.scope.object.clone();
    assert_eq!(base, minted(&twin, "deposit", &run_of(&harness, [5, 0])));

    // A unit with one postcondition clause on `deposit` and the unit with two mint one identity.
    let one_clause = Twin::build(&model::GRANTED, &[CLAUSES[0]], 0);
    assert_eq!(
        base,
        identity(&one_clause, "deposit"),
        "one clause against two on one operation"
    );
}

/// The edge the spec states and does not hide: occurrence ordinals run over the operations the
/// unit's clauses name, so a clause added on an operation that sorts earlier moves this
/// operation's frame occurrence. The identity of `transfer` is read before and after a clause on
/// `deposit` is added, and it differs.
///
/// Trace: FR-024-AC-21, TC-035
#[test]
fn tc_035_a_clause_on_an_earlier_sorting_operation_moves_the_frame_occurrence() {
    let identity = |twin: &Twin| minted(twin, "transfer", &run_for_operation(twin, "transfer"));
    let transfer = ("TransferKeepsBalance", "transfer", "balance");
    let alone = Twin::build(&model::GRANTED, &[transfer], 0);
    let after = Twin::build(&model::GRANTED, &[CLAUSES[0], transfer], 0);
    assert_ne!(identity(&alone), identity(&after));
}

/// The identity does not change when any one of the harness's own members changes: its clause
/// node, module symbol, harness symbol, state path, subject path, unwind bound, options, the
/// ranges of its fields or the order of its state fields. The solver is not edited: `KaniSolver`
/// has one variant (`Cadical`), so no other solver can be built, and the identity function takes
/// no solver.
///
/// Trace: FR-024-AC-21, TC-035
#[test]
fn tc_035_the_frame_identity_names_none_of_the_harness_members() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    let base = minted(&twin, "deposit", &run_of(&harness, [5, 0]));
    type Edit = Box<dyn Fn(&mut StateFrameIdentity)>;
    let edits: Vec<(&str, Edit)> = vec![
        (
            "clause node",
            Box::new(|h| h.clause = h.scope.object.clone()),
        ),
        (
            "module symbol",
            Box::new(|h| h.module_symbol = ModuleSymbol::try_from("renamed_module").unwrap()),
        ),
        (
            "harness symbol",
            Box::new(|h| h.harness_symbol = HarnessSymbol::try_from("renamed_check").unwrap()),
        ),
        (
            "state path",
            Box::new(|h| h.state_path = "crate::other::State".to_owned()),
        ),
        (
            "subject path",
            Box::new(|h| h.subject_path = "crate::other::operate".to_owned()),
        ),
        ("unwind", Box::new(|h| h.unwind += 3)),
        (
            "options",
            Box::new(|h| h.options.push("--extra".to_owned())),
        ),
        (
            "ranges",
            Box::new(|h| {
                for domain in &mut h.domains {
                    domain.minimum -= 10;
                    domain.maximum += 10;
                }
            }),
        ),
    ];
    for (name, edit) in edits {
        let mut edited = harness.clone();
        edit(&mut edited);
        assert_ne!(edited, harness, "{name} is an edit");
        assert_eq!(
            base,
            minted(&twin, "deposit", &run_of(&edited, [5, 0])),
            "{name}"
        );
    }
    let mut reordered = harness.clone();
    reordered.state_fields.reverse();
    assert_eq!(
        base,
        minted(&twin, "deposit", &run_of(&reordered, [0, 5])),
        "state field order"
    );
}

/// A harness whose property is a postcondition is `NotAFrame`, and a frame whose granted and
/// checked fields are not exactly the state fields is `FieldSetMismatch`, whether a field is
/// neither granted nor checked, granted and checked, or no state field at all. Each is refused
/// before the call site: the operation the unit does not name would otherwise be a
/// `CallSite` refusal, as it is for the harness that passes the checks.
///
/// Trace: FR-024-AC-22, TC-035
#[test]
fn tc_035_a_non_frame_and_a_mismatched_field_set_are_refused_before_the_call_site() {
    let twin = Twin::new();
    let mut base = frame_harness(&twin);
    base.scope.operation = "withdraw".to_owned();
    let refusal = |harness: &StateFrameIdentity| {
        refused(replay_of(&twin, "withdraw", &run_of(harness, [5, 0])))
    };
    assert!(
        matches!(refusal(&base), FrameReplayError::CallSite(_)),
        "the control reaches the call site"
    );

    let mut postcondition = base.clone();
    postcondition.property = generate(&fixture(&Shape::HEALTHY))
        .postcondition
        .identity
        .property;
    assert!(matches!(
        refusal(&postcondition),
        FrameReplayError::NotAFrame
    ));

    let frame = |granted: &[&str], checked: &[&str]| {
        let mut harness = base.clone();
        harness.property = StateFrameProperty::Frame {
            granted: granted.iter().map(|field| (*field).to_owned()).collect(),
            checked: checked.iter().map(|field| (*field).to_owned()).collect(),
        };
        harness
    };
    for (granted, checked) in [
        (&["balance"][..], &[][..]),
        (&["balance"], &["audit", "balance"]),
        (&["balance"], &["audit", "other"]),
        (&["balance", "audit"], &["audit"]),
    ] {
        let error = refusal(&frame(granted, checked));
        let FrameReplayError::FieldSetMismatch {
            state_fields,
            granted: held,
            checked: also,
        } = error
        else {
            panic!("{granted:?} / {checked:?}: {error}");
        };
        assert_eq!(state_fields, ["balance", "audit"]);
        assert_eq!(held, granted);
        assert_eq!(also, checked);
    }
    assert!(
        matches!(
            refusal(&frame(&["audit"], &["balance"])),
            FrameReplayError::CallSite(_)
        ),
        "the same fields in the other split are a frame over the state"
    );
}

/// The harness record carries `state_fields` in the order the harness draws them, and the
/// identity read back from the record equals the generated one. A harness regenerated from equal
/// inputs has a byte-identical record, and a record without `state_fields` is not read as a
/// frame identity.
///
/// Trace: FR-024-AC-23, TC-035
#[test]
fn tc_035_the_record_carries_the_draw_order_and_a_record_without_it_is_not_read() {
    let fixture = fixture(&Shape::HEALTHY);
    let generated = generate(&fixture);
    let record: Value = serde_json::from_str(&generated.frame.record.contents).expect("record");
    assert_eq!(
        record["identity"]["state_fields"],
        json!(["balance", "audit"])
    );
    assert_eq!(generate(&fixture).frame.record, generated.frame.record);
    assert_eq!(
        StateFrameIdentity::from_record(&generated.frame.record.contents).expect("a frame record"),
        generated.frame.identity
    );

    let reversed = generate_state_frame_obligations(&StateFrameRequest {
        state_fields: &["audit", "balance"],
        ..request(&fixture, &STATE_FIELDS)
    })
    .expect("the fields generate in either order");
    let reversed_record: Value =
        serde_json::from_str(&reversed.frame.record.contents).expect("record");
    assert_eq!(
        reversed_record["identity"]["state_fields"],
        json!(["audit", "balance"])
    );
    assert_ne!(reversed.frame.record, generated.frame.record);

    let mut without = record.clone();
    without["identity"]
        .as_object_mut()
        .expect("an identity object")
        .remove("state_fields");
    assert!(StateFrameIdentity::from_record(&without.to_string()).is_err());
    let mut extra = record;
    extra["identity"]["unknown"] = json!(1);
    assert!(StateFrameIdentity::from_record(&extra.to_string()).is_err());
    assert!(StateFrameIdentity::from_record("{}").is_err());
}

/// The playback is decoded against the harness's draw order, not the order of its ranges or its
/// grants: a harness that draws `audit` first reads the first playback value as `audit`, and the
/// pre snapshot tie sees that.
///
/// Trace: FR-024-AC-23, FR-024-AC-25, TC-035
#[test]
fn tc_035_the_playback_is_decoded_in_the_harnesss_draw_order() {
    let fixture = fixture(&Shape::HEALTHY);
    let twin = Twin::new();
    let reversed = generate_state_frame_obligations(&StateFrameRequest {
        state_fields: &["audit", "balance"],
        ..request(&fixture, &STATE_FIELDS)
    })
    .expect("generates")
    .frame
    .identity;
    let harness = twin.aligned(&reversed, "deposit");
    // Draw order `audit`, `balance`: the playback `[0, 5]` is the pre state `balance = 5`,
    // `audit = 0` the invocation holds.
    let run = run_of(&harness, [0, 5]);
    let replay = replay_of(&twin, "deposit", &run).expect("the playback ties to the pre state");
    let ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("a frame replay is a witness replay");
    };
    assert!(
        witness.transcript().ends_with("|audit=0;balance=5>>>"),
        "{}",
        witness.transcript()
    );
    // The same values read in declaration order are the pre state `balance = 0`, `audit = 5`.
    let swapped = refused(replay_of(&twin, "deposit", &run_of(&harness, [5, 0])));
    assert!(
        matches!(
            swapped,
            FrameReplayError::PreState(PreStateFault::Differs { ref field, decoded: 5, snapshot: 0 })
                if field == "audit"
        ),
        "{swapped}"
    );
}

/// A field with no declared range is listed in `state_fields`, has no entry in `domains`, is
/// decoded and is neither range-checked nor refused: its value far outside any range the twin's
/// package declares is carried into the transcript.
///
/// Trace: FR-024-AC-23, FR-024-AC-26, TC-035
#[test]
fn tc_035_a_field_with_no_declared_range_is_decoded_and_not_checked() {
    let twin = Twin::new();
    let unranged_audit = fixture(&Shape {
        variant: 90,
        audit_unbounded: true,
        ..Shape::HEALTHY
    });
    let harness = twin.aligned(&generate(&unranged_audit).frame.identity, "deposit");
    assert_eq!(harness.state_fields, ["balance", "audit"]);
    assert_eq!(
        harness
            .domains
            .iter()
            .map(|domain| domain.field.as_str())
            .collect::<Vec<_>>(),
        ["balance"],
        "only the ranged field has a domain"
    );
    let unranged = 5_000_000_000_i64;
    let invocation = twin.invocation("account", (5, unranged), (5, unranged));
    let replay = twin
        .try_frame_replay(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run_of(&harness, [5, unranged]),
        )
        .unwrap_or_else(|error| panic!("an unranged value is not refused: {error}"));
    let ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("a frame replay is a witness replay");
    };
    assert!(
        witness
            .transcript()
            .ends_with("|balance=5;audit=5000000000>>>"),
        "{}",
        witness.transcript()
    );
}

/// The transcript `Witness::parse` is given is the one rendering function's, over the decoded
/// values, the harness path and the check text the decode names: it is not a fixed text, and a
/// different playback gives a different transcript.
///
/// Trace: FR-024-AC-25, TC-035
#[test]
fn tc_035_the_frame_witness_is_rendered_from_the_decoded_playback() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    for (balance, audit) in [(5, 0), (7, 3)] {
        let invocation = twin.invocation("account", (balance, audit), (balance, audit + 1));
        let replay = twin
            .try_frame_replay(
                "deposit",
                &invocation,
                "account",
                "audit",
                &run_of(&harness, [balance, audit]),
            )
            .expect("builds");
        let expected = format!(
            "<<<assertion|{}|{FORBIDDEN_CHECK}|balance={balance};audit={audit}>>>",
            harness.harness_path()
        );
        let ReplaySource::Witness(witness) = &replay.wire.source else {
            panic!("a frame replay is a witness replay");
        };
        assert_eq!(witness.transcript(), expected);
        assert_eq!(replay.packet.source, Some(replay.wire.source.clone()));
    }
}

/// The playback of another harness, a playback with the wrong number of values and one with a
/// wrong-width value each return a typed decode refusal carrying the decoder's cause, and the
/// call site is not reached: the operation the unit does not name would otherwise be a
/// `CallSite` refusal, as it is for the playback that decodes.
///
/// Trace: FR-024-AC-25, TC-035
#[test]
fn tc_035_a_playback_that_does_not_decode_is_refused_before_the_call_site() {
    let twin = Twin::new();
    let mut harness = frame_harness(&twin);
    harness.scope.operation = "withdraw".to_owned();
    let good = run_of(&harness, [5, 0]);
    assert!(
        matches!(
            refused(replay_of(&twin, "withdraw", &good)),
            FrameReplayError::CallSite(_)
        ),
        "the control reaches the call site"
    );

    let mut sibling = harness.clone();
    sibling.module_symbol = ModuleSymbol::try_from("sibling_module").unwrap();
    let other_harness = Run {
        playback: playback_text(&sibling, FORBIDDEN_CHECK, &[5, 0]),
        harness: harness.clone(),
    };
    let short = Run {
        playback: playback_text(&harness, FORBIDDEN_CHECK, &[5]),
        harness: harness.clone(),
    };
    let wrong_width = Run {
        playback: good
            .playback
            .replacen("vec![5, 0, 0, 0, 0, 0, 0, 0]", "vec![5, 0, 0, 0]", 1),
        harness: harness.clone(),
    };
    assert_ne!(wrong_width.playback, good.playback);
    for (run, code) in [
        (&other_harness, "cg_witness_harness_identity_mismatch"),
        (&short, "kani_witness_arity_mismatch"),
        (&wrong_width, "kani_witness_width_mismatch"),
    ] {
        let error = refused(replay_of(&twin, "withdraw", run));
        let FrameReplayError::Decode(cause) = &error else {
            panic!("{code}: {error}");
        };
        assert_eq!(cause.code, code, "{error}");
    }
}

/// A decoded value at either end of its field's declared range is admitted, and the values one
/// below and one above are refused naming the field and the value, before the call site (the
/// operation the unit does not name would otherwise be a `CallSite` refusal).
///
/// Trace: FR-024-AC-26, TC-035
#[test]
fn tc_035_a_decoded_value_outside_its_declared_range_is_refused_at_the_endpoints() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    assert_eq!(
        harness
            .domains
            .iter()
            .map(|domain| domain.field.as_str())
            .collect::<Vec<_>>(),
        harness.state_fields,
        "every field of the fixture has a range, in draw order"
    );
    let mut unlocatable = harness.clone();
    unlocatable.scope.operation = "withdraw".to_owned();
    for (position, domain) in harness.domains.iter().enumerate() {
        let at = |value: i64| {
            let mut values = [0, 0];
            values[position] = value;
            values
        };
        for admitted in [domain.minimum, domain.maximum] {
            let values = at(admitted);
            let invocation =
                twin.invocation("account", (values[0], values[1]), (values[0], values[1]));
            twin.try_frame_replay(
                "deposit",
                &invocation,
                "account",
                "audit",
                &run_of(&harness, values),
            )
            .unwrap_or_else(|error| panic!("{} = {admitted} is admitted: {error}", domain.field));
        }
        for outside in [domain.minimum - 1, domain.maximum + 1] {
            let error = refused(replay_of(
                &twin,
                "withdraw",
                &run_of(&unlocatable, at(outside)),
            ));
            assert!(
                matches!(
                    &error,
                    FrameReplayError::OutOfDomain { field, value }
                        if *field == domain.field && *value == outside
                ),
                "{} = {outside}: {error}",
                domain.field
            );
        }
    }
}

/// The pre snapshot the invocation names must hold the decoded values. The invocation of the
/// playback's own pre state settles `reproduced-with-evaluated-witness`, `violation`, naming the
/// written field; the invocation of a different pre state, in either field, is refused naming the
/// field and both values.
///
/// Trace: FR-024-AC-27, TC-035
#[test]
fn tc_035_the_invocation_of_the_playbacks_own_pre_state_replays_and_another_is_refused() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let own = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&own, "account", "audit", &run)
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
    let Some(FrameChange::FieldWrite { field, .. }) = result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!(field.as_str(), "audit");

    for (pre, field, snapshot) in [((6, 0), "balance", 6), ((5, 1), "audit", 1)] {
        let other = twin.invocation("account", pre, (6, 1));
        let error = refused(twin.try_frame_replay("deposit", &other, "account", "audit", &run));
        assert!(
            matches!(
                &error,
                FrameReplayError::PreState(PreStateFault::Differs {
                    field: named,
                    decoded,
                    snapshot: held,
                }) if named == field && *held == snapshot && *decoded == if field == "balance" { 5 } else { 0 }
            ),
            "{pre:?}: {error}"
        );
    }
}

/// An invocation or pre snapshot that is not provided, is not of the shape `state_clause`
/// writes, lacks the object, or lacks a field or holds a non-integer for it is refused naming what
/// is missing. Every provided document here hashes to its digest (`Invocation::edited`
/// re-addresses it), so these refusals are the tie's own.
///
/// Trace: FR-024-AC-27, TC-035
#[test]
fn tc_035_a_pre_state_that_cannot_be_read_from_the_invocation_is_refused_by_name() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let base = twin.invocation("account", (5, 0), (6, 1));
    let fault = |invocation: &Invocation| {
        let error = refused(twin.try_frame_replay("deposit", invocation, "account", "audit", &run));
        match error {
            FrameReplayError::PreState(fault) => fault,
            other => panic!("not a pre-state refusal: {other}"),
        }
    };
    assert_eq!(
        fault(&base.without(INVOCATION_DOCUMENT)),
        PreStateFault::InvocationNotProvided
    );
    // An invocation that does not name its pre snapshot or address its object is not the
    // document `state_clause` writes.
    for member in ["pre", "self"] {
        assert_eq!(
            fault(&base.edited(INVOCATION_DOCUMENT, |document| {
                document.as_object_mut().expect("object").remove(member);
            })),
            PreStateFault::InvocationUnreadable,
            "{member}"
        );
    }
    assert!(matches!(
        fault(&base.without(PRE_DOCUMENT)),
        PreStateFault::PreNotProvided { .. }
    ));
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"] = json!("none");
        })),
        PreStateFault::PreUnreadable
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["key"] = json!("another");
        })),
        PreStateFault::ObjectMissing {
            population: "ix://test/bank/accounts".to_owned(),
            key: "account".to_owned(),
        }
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["fields"]
                .as_object_mut()
                .expect("fields")
                .remove("audit");
        })),
        PreStateFault::FieldMissing {
            field: "audit".to_owned()
        }
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["fields"]["balance"] =
                json!({"integer": "five"});
        })),
        PreStateFault::FieldNotInteger {
            field: "balance".to_owned()
        }
    );
}

/// A provided document whose bytes do not match the digest it is addressed by is QSL's refusal,
/// with its own code, and not a pre-state refusal: the tie reads the documents only after QSL's
/// request decode has checked them. The same replay over documents that hash to their digests
/// builds.
///
/// Trace: FR-024-AC-27, TC-035
#[test]
fn tc_035_a_document_that_does_not_match_its_digest_is_qsls_refusal_not_a_pre_state_one() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let base = twin.invocation("account", (5, 0), (6, 1));
    for index in [INVOCATION_DOCUMENT, PRE_DOCUMENT] {
        let mismatched = base.replaced(index, br#"{"unrelated":true}"#);
        let error =
            refused(twin.try_frame_replay("deposit", &mismatched, "account", "audit", &run));
        assert!(
            matches!(
                &error,
                FrameReplayError::Refused(refusal)
                    if matches!(**refusal, ReplayRefusal::Request(_))
            ),
            "{index}: {error}"
        );
    }
    twin.try_frame_replay("deposit", &base, "account", "audit", &run)
        .map(|_| ())
        .unwrap_or_else(|error| panic!("the matching documents build: {error}"));
}

/// A state field listed twice is refused: the decoder refuses the schema, and a record that lists
/// one is not read as a frame identity.
///
/// Trace: FR-024-AC-23, TC-035
#[test]
fn tc_035_a_repeated_state_field_is_refused_by_the_decoder_and_the_record() {
    let twin = Twin::new();
    let mut harness = frame_harness(&twin);
    harness.state_fields = vec!["balance".to_owned(), "balance".to_owned()];
    harness.property = StateFrameProperty::Frame {
        granted: vec!["balance".to_owned()],
        checked: vec!["balance".to_owned()],
    };
    let error = refused(replay_of(&twin, "deposit", &run_of(&harness, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::Decode(cause)
                if cause.code == "cg_witness_schema_duplicate_binding" && cause.context == "balance"
        ),
        "{error}"
    );

    let generated = generate(&fixture(&Shape::HEALTHY)).frame;
    let mut record: Value = serde_json::from_str(&generated.record.contents).expect("record");
    record["identity"]["state_fields"] = json!(["balance", "balance"]);
    let error = StateFrameIdentity::from_record(&record.to_string())
        .expect_err("a repeated field is not a draw order");
    assert!(
        matches!(&error, StateFrameRecordError::RepeatedField { field } if field == "balance"),
        "{error}"
    );
}

/// A harness scoped to another operation than the one requested is a `ScopeMismatch` naming
/// `operation`, before the call site (the requested operation the unit does not name would
/// otherwise be a `CallSite` refusal). A harness whose anchor or frame is not the one the call
/// site names is a `ScopeMismatch` naming that member after it.
///
/// Trace: FR-024-AC-28, TC-035
#[test]
fn tc_035_a_harness_scope_that_is_not_the_requested_operation_is_refused_by_member() {
    let twin = Twin::new();
    let fixture = fixture(&Shape::HEALTHY);
    let site = twin.operation_site("deposit").expect("located");
    let aligned = frame_harness(&twin);

    let error = refused(replay_of(&twin, "withdraw", &run_of(&aligned, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Operation, harness, named }
                if harness == "deposit" && named == "withdraw"
        ),
        "{error}"
    );

    // The fixture's own node ids are not the ones QSL names for the twin's compiled unit.
    let unaligned = generate(&fixture).frame.identity;
    let error = refused(replay_of(&twin, "deposit", &run_of(&unaligned, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Anchor, harness, named }
                if *harness == *fixture.anchor.digest && *named == site.anchor.to_string()
        ),
        "{error}"
    );

    let mut other_frame = aligned.clone();
    other_frame.scope.frame = fixture.frame.clone();
    let error = refused(replay_of(&twin, "deposit", &run_of(&other_frame, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Frame, harness, named }
                if *harness == *fixture.frame.digest && *named == site.frame.to_string()
        ),
        "{error}"
    );
    replay_of(&twin, "deposit", &run_of(&aligned, [5, 0])).expect("the aligned scope builds");
}

/// The frame envelope declares no domain: `declared_domains` is present and empty.
///
/// Trace: FR-024-AC-29, TC-035
#[test]
fn tc_035_the_frame_envelope_declares_no_domain() {
    let twin = Twin::new();
    let replay = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");
    assert_eq!(replay.packet.declared_domains, Some(Vec::new()));
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
pub(crate) fn generate_over(fixture: &Fixture, subject: &str) -> StateFrameObligations {
    let subject_path = format!("crate::subject::{subject}");
    generate_state_frame_obligations(&StateFrameRequest {
        subject_path: &subject_path,
        ..request(fixture, &STATE_FIELDS)
    })
    .unwrap_or_else(|refusal| panic!("the fixture must generate: {refusal}"))
}

/// Runs `harness` with the real prover over the subject module every generated harness names.
pub(crate) fn prove(harness: &StateFrameHarness) -> KaniRunOutcome {
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
pub(crate) fn falsified(outcome: KaniRunOutcome, reason: &str) -> String {
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
pub(crate) fn playback_state(counterexample: &str) -> (i64, i64) {
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
    let single = fixture(&single_valuation_shape());
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
/// `FrameReplay::new` and `replay` from its harness's identity and its real playback text alone:
/// no obligation identity and no transcript are supplied, the decoded pre state is tied to the
/// invocation of the native run, and QSL reproduces the violation on the written field.
///
/// Trace: FR-015-AC-32, FR-024-AC-30, TC-025, TC-035
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_035_real_kani_frame_counterexample_replays_through_qsl() {
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
    let run = Run {
        harness: twin.aligned(&forbidden.frame.identity, "deposit"),
        playback: counterexample,
    };
    let result = twin
        .frame_replay(&invocation, "account", "audit", &run)
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
}

/// The allowed effect proves, and its run replays as a frame the invocation respects. A verified
/// harness yields no playback, so the run carries a hand-built one at the pre state `(5, 0)`; the
/// forbidden counterexample is replayed from its real playback by
/// `tc_035_real_kani_frame_counterexample_replays_through_qsl`.
///
/// Trace: FR-015-AC-32, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_the_allowed_frame_effect_replays_as_a_respected_frame() {
    let fixture = fixture(&Shape::HEALTHY);
    let twin = Twin::new();

    let allowed = generate_over(&fixture, "deposit");
    assert_eq!(prove(&allowed.frame), KaniRunOutcome::Verified);
    let mut account = subject::Account {
        balance: 5,
        audit: 0,
    };
    subject::deposit(&mut account);
    let invocation = twin.invocation("account", (5, 0), (account.balance, account.audit));
    let harness = twin.aligned(&allowed.frame.identity, "deposit");
    let run = run_of(&harness, [5, 0]);
    let result = twin
        .replay(&invocation, "account", "balance", &run)
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
