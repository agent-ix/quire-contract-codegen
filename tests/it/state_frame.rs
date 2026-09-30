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
//! (`#[ignore]` here, run serially by `make kani`) proves the harnesses with the installed
//! backend: a healthy subject verifies, each seeded defect is falsified for the intended
//! property, and regenerating the frame from a mutated package turns a green proof red.

#[path = "../exact_scalar_support/package.rs"]
#[allow(clippy::duplicate_mod)]
mod package;

use std::{fs, path::PathBuf, time::Duration};

use package::{
    application, code_id, corpus_package, key, literal, member, op, op_full, parameter_body,
    reference, Bound, PackageBuilder, NODE_DOMAIN, T_BOOLEAN,
};
use quire_contract_codegen::{
    execute_kani_obligation, generate_state_frame_obligations, KaniExecutionRequest,
    KaniInstallation, KaniRunOutcome, StateComparison, StateFrameHarness, StateFrameObligations,
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
const POST_BALANCE: u32 = 4007;
const PRE_BALANCE: u32 = 4008;

const STATE_FIELDS: [&str; 2] = ["balance", "audit"];
const STATE_PATH: &str = "crate::subject::Account";
const SUBJECT_PATH: &str = "crate::subject::deposit";

/// What the operation's state looks like to the subject.
const STATE_TYPE: &str =
    "#[derive(Clone)]\npub struct Account {\n    pub balance: i64,\n    pub audit: i64,\n}\n";
/// Credits one unit to `balance`, the field the frame grants.
const DEPOSIT: &str = "pub fn deposit(account: &mut Account) {\n    if account.balance < 1000 {\n        account.balance += 1;\n    }\n}\n";
/// Seeded defect: also rewrites `audit`, which the frame does not grant.
const DEPOSIT_TOUCHING_AUDIT: &str = "pub fn deposit(account: &mut Account) {\n    if account.balance < 1000 {\n        account.balance += 1;\n    }\n    account.audit = account.audit.wrapping_add(1);\n}\n";
/// Seeded defect: debits instead of crediting.
const DEPOSIT_DEBITING: &str = "pub fn deposit(account: &mut Account) {\n    account.balance = account.balance.wrapping_sub(1);\n}\n";

/// One variant of the fixture. Each variant owns its node codes, because the test binary's
/// fixture registry refuses one code bound to two different bodies.
struct Shape {
    variant: u32,
    clause: &'static str,
    modifies: &'static [&'static str],
    creates_object: bool,
    condition: Condition,
    audit_bound: (i64, i64),
}

#[derive(Clone, Copy)]
enum Condition {
    /// `post(balance) >= pre(balance)`.
    PostGePre,
    /// `post(balance) >= post(balance)`.
    PostGePost,
    /// `post(balance) >= pre(audit)`.
    BalanceAgainstAudit,
}

impl Shape {
    const HEALTHY: Self = Self {
        variant: 0,
        clause: "postcondition",
        modifies: &["balance"],
        creates_object: false,
        condition: Condition::PostGePre,
        audit_bound: (0, 1000),
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
            vec![reference(&deref)],
        ),
        &[deref, object.to_owned()],
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
    let bound_key = builder.bound(&Bound::Integer(0, 1000));
    let audit_key = builder.bound(&Bound::Integer(shape.audit_bound.0, shape.audit_bound.1));
    let object = key(OBJECT);
    builder.node_with(
        &object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": [
            member("balance", reference(&bound_key)),
            member("audit", reference(&audit_key)),
        ]}),
        &[bound_key.clone(), audit_key.clone()],
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
    let creates = if shape.creates_object {
        vec![node_ref(&object)]
    } else {
        vec![]
    };
    let modifies = shape
        .modifies
        .iter()
        .map(|name| json!({"kind": "field", "declaration": node_ref(&object), "name": name}))
        .collect::<Vec<_>>();
    builder.node_with(
        &key(FRAME),
        "state",
        "frame",
        &object,
        json!({"term": "frame", "modifies": modifies, "creates": creates, "deletes": []}),
        std::slice::from_ref(&object),
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
    let post_balance = field_read(&mut builder, POST_BALANCE, &object, "balance", &bound_key);
    let pre_balance = pre_read(&mut builder, PRE_BALANCE, &post_balance, &bound_key);
    let post_audit = field_read(&mut builder, shape.code(400), &object, "audit", &audit_key);
    let pre_audit = pre_read(&mut builder, shape.code(401), &post_audit, &audit_key);
    let (left, right) = match shape.condition {
        Condition::PostGePre => (&post_balance, &pre_balance),
        Condition::PostGePost => (&post_balance, &post_balance),
        Condition::BalanceAgainstAudit => (&post_balance, &pre_audit),
    };
    builder.application_bounded(
        shape.code(200),
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.integer.ge"),
            &key(T_BOOLEAN),
            vec![reference(left), reference(right)],
        ),
        &[],
    );
    let condition = code_id(shape.code(200)).digest.to_string();
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
                json!({"term": "aggregate", "members": [reference(&key(SELF))]}),
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

// ---- default lane ------------------------------------------------------------

/// A postcondition clause yields two separate harnesses whose identity is scoped to the
/// operation, anchor, frame and object, whose bounds and granted and forbidden fields are read
/// from the IR, and each carries exactly one non-vacuity cover.
///
/// Trace: FR-015-AC-26, TC-025
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
            minimum: 0,
            maximum: 1000,
        }
    );
    assert_eq!(
        generated.frame.identity.property,
        StateFrameProperty::Frame {
            granted: vec!["balance".to_owned()],
            checked: vec!["audit".to_owned()],
        }
    );

    let post = &generated.postcondition.rust.contents;
    assert!(post.contains("kani::assume(pre.balance >= 0_i64 && pre.balance <= 1000_i64);"));
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
/// Trace: FR-015-AC-26, TC-025
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

/// Every shape outside one integer comparison of pre and post reads of one field, every frame
/// effect with no finite encoding, and every unreadable or unbounded clause is refused by name,
/// with no harness.
///
/// Trace: FR-015-AC-27, TC-025
#[test]
fn tc_025_shapes_without_a_finite_encoding_are_refused_by_name() {
    assert!(matches!(
        refusal(
            &Shape { variant: 2, clause: "precondition", ..Shape::HEALTHY },
            &STATE_FIELDS
        ),
        StateFrameRefusal::NotAPostcondition { clause } if clause == "precondition"
    ));
    assert_eq!(
        refusal(
            &Shape {
                variant: 3,
                condition: Condition::PostGePost,
                ..Shape::HEALTHY
            },
            &STATE_FIELDS
        ),
        StateFrameRefusal::ObservationsSameSide
    );
    assert_eq!(
        refusal(
            &Shape {
                variant: 4,
                condition: Condition::BalanceAgainstAudit,
                ..Shape::HEALTHY
            },
            &STATE_FIELDS
        ),
        StateFrameRefusal::ObservationsDiffer {
            left: "balance".to_owned(),
            right: "audit".to_owned()
        }
    );
    assert!(matches!(
        refusal(
            &Shape {
                variant: 5,
                creates_object: true,
                ..Shape::HEALTHY
            },
            &STATE_FIELDS
        ),
        StateFrameRefusal::FrameEffectUnsupported {
            effect: UnsupportedFrameEffect::Creates,
            ..
        }
    ));
    // The object's two fields are typed by two different ranges: no single bound.
    assert_eq!(
        refusal(
            &Shape {
                variant: 6,
                audit_bound: (0, 50),
                ..Shape::HEALTHY
            },
            &STATE_FIELDS
        ),
        StateFrameRefusal::BoundNotResolved { distinct: 2 }
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

/// A request that is not well formed, and a node that is not a clause, are refused before any
/// harness is generated.
///
/// Trace: FR-015-AC-27, TC-025
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

/// Runs `harness` over `subject` with the real prover.
fn prove(harness: &StateFrameHarness, subject: &str) -> KaniRunOutcome {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\npub mod subject {{\n{STATE_TYPE}{subject}}}\n",
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

fn falsified(outcome: KaniRunOutcome, reason: &str) {
    let KaniRunOutcome::Falsified { counterexample } = outcome else {
        panic!("expected a falsified proof for `{reason}`, got {outcome:?}");
    };
    assert!(
        counterexample.contains(reason),
        "the counterexample must name `{reason}`:\n{counterexample}"
    );
}

/// The operation contract verifies for a healthy subject and is falsified, for the postcondition
/// and no other reason, when the subject is mutated to debit.
///
/// Trace: FR-015-AC-28, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it() {
    let generated = generate(&fixture(&Shape::HEALTHY));
    assert_eq!(
        prove(&generated.postcondition, DEPOSIT),
        KaniRunOutcome::Verified
    );
    falsified(
        prove(&generated.postcondition, DEPOSIT_DEBITING),
        "postcondition `post.balance >= pre.balance` failed",
    );
}

/// An effect the frame allows verifies, an effect it forbids is falsified naming the forbidden
/// field, and regenerating the frame from a package whose `modifies` is mutated to grant nothing
/// turns the allowed proof red.
///
/// Trace: FR-015-AC-28, TC-025
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies() {
    let generated = generate(&fixture(&Shape::HEALTHY));
    // ALLOWED: only `balance` changes, and the frame grants it.
    assert_eq!(prove(&generated.frame, DEPOSIT), KaniRunOutcome::Verified);
    // FORBIDDEN: `audit` changes too.
    falsified(
        prove(&generated.frame, DEPOSIT_TOUCHING_AUDIT),
        "changed `audit`, which its frame does not modify",
    );
    // MUTATION CONTROL: the same allowed subject against a frame that grants nothing.
    let emptied = generate(&fixture(&Shape {
        variant: 1,
        modifies: &[],
        ..Shape::HEALTHY
    }));
    falsified(
        prove(&emptied.frame, DEPOSIT),
        "changed `balance`, which its frame does not modify",
    );
}
