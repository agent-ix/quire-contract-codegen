//! FR-016: the surviving replay adapter and function-contract identity behavior.
//!
//! A direct test-only `KaniObligationIdentity` supplies persisted argument bindings for
//! deterministic playback, domain, native replay and O-09 identity checks. It asserts no
//! generated harness or installed proof. QSL's `call_site` supplies the package and parameter
//! identities; its `replay` facade recompiles the source and evaluates the selected function.
//! The installed E7-E9 control awaits a genuine V2 generated harness under IR-1047.

use ix_trace_rs::trace;
use std::collections::BTreeMap;

use qsl_replay::{
    CallSiteRefusal, CanonicalAssignment, Code, DeclaredDomain, DependencyInputRefusal,
    DependencySelectionsCause, DisagreementCause, DomainKey, DomainKind, FiniteBound, Integer,
    ProofBound, ReplayLimits, ReplayRefusal, ReplayRequestRefusal, ReplaySource, ScalarLimits,
    Verdict, WitnessSettlement, DEFAULT_REPLAY_INPUT_BYTES,
};
use qsl_replay::{Witness, WitnessValue};
use quire_contract_codegen::{
    parse_assertion_playback, replay_counterexample, replay_counterexample_through,
    replay_falsification, DependencyLock, DependencyLockError, EvidenceFailureCause,
    FunctionPacketError, HarnessSymbol, KaniBindingRole, KaniIntegerBounds, KaniObligationIdentity,
    KaniPrimitiveType, KaniSolver, LockedSource, ModuleSymbol, ObligationBinding, ObligationKind,
    Playback, ReplayInputs, ReplayPackage, ReplayPackageError, ReplayParameter, ReplayVerdict,
    SpineReplayError, WitnessDecodeLimits,
};
use quire_contract_model::{
    ClauseId, ClauseRef, IntegerDomain, OverflowPolicy, RequirementRef, SourceSpan,
};
use quire_semantic_value::catalog::{Category, InternalFault};
use serde_json::json;

const PROFILE: &str = "profile v = \"quire.value.complete/v1\";\n";
const AUTHORITY: &str = "agent-ix";
const IDENTITY: &str = "test:skeleton-spine";
const NAMESPACE: &str = "git";
const REVISION: &str = "r1";
const FUNCTION: &str = "balance_never_grows";

/// The native twin of the clause over the subject: the QSL function the replay evaluates.
fn native_source(post_balance: &str) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function {FUNCTION} using v(amount_current: Int[0, 1000], balance_pre: Int[0, 1000]): \
         Boolean pure {{ {post_balance} <= balance_pre }}\n"
    )
}

/// A native twin whose clause is false at exactly one point, `(amount, balance)`, and true
/// everywhere else in the declared domain. Replaying a witness against it reproduces the
/// violation only when the witness carries those two values, so the verdict cannot be reached
/// by a constant, a literal or a witness other than the one the prover found.
fn point_source(amount: i64, balance: i64) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function {FUNCTION} using v(amount_current: Int[0, 1000], balance_pre: Int[0, 1000]): \
         Boolean pure {{ amount_current < {amount} or amount_current > {amount} \
         or balance_pre < {balance} or balance_pre > {balance} }}\n"
    )
}

const UNLIMITED: ScalarLimits = ScalarLimits {
    integer_bits: u64::MAX,
    decimal_digits: u64::MAX,
    scale_expansion: u64::MAX,
    text_input_bytes: u64::MAX,
    text_scalars: u64::MAX,
    normalized_scalars: u64::MAX,
    unit_edges: u64::MAX,
    value_occurrences: u64::MAX,
    work_units: u64::MAX,
    result_units: u64::MAX,
};

const FUNCTION_PLAYBACK_LIMITS: WitnessDecodeLimits = WitnessDecodeLimits {
    bytes: 4096,
    values: 2,
};

fn locked(identity: &str, bytes: &[u8]) -> LockedSource {
    LockedSource {
        authority: AUTHORITY.to_owned(),
        identity: identity.to_owned(),
        namespace: NAMESPACE.to_owned(),
        revision: REVISION.to_owned(),
        bytes: bytes.to_vec(),
    }
}

/// The native twin's test lock: finite playback limits and unlimited accounting stand-ins,
/// because this direct identity fixture has no installed proving run.
fn inputs(source: &str, dependencies: Vec<DependencyLock>) -> ReplayInputs {
    ReplayInputs {
        decode_limits: FUNCTION_PLAYBACK_LIMITS,
        package_name: "skeleton-spine".to_owned(),
        source: locked(IDENTITY, source.as_bytes()),
        dependencies,
        accounting_limits: UNLIMITED,
        stage_limits: BTreeMap::from([("s1.input_bytes".to_owned(), DEFAULT_REPLAY_INPUT_BYTES)]),
        declared_domains: Vec::new(),
        replay_limits: ReplayLimits::default(),
    }
}

/// Build a request for a selected function by binding the direct test identity to its parameters.
/// `ReplayPackage::request` derives the identity slot from that binding.
fn request_of(package: &ReplayPackage, source: ReplaySource) -> qsl_replay::ReplayRequestWire {
    let mut identity = spine_identity();
    let template = identity.arguments[0].clone();
    identity.arguments = package
        .parameters()
        .iter()
        .map(|parameter| quire_contract_codegen::ObligationBinding {
            identifier: parameter.argument.to_owned(),
            ..template.clone()
        })
        .collect();
    let witness = match &source {
        ReplaySource::Witness(witness) => witness.clone(),
        ReplaySource::Input(_) => {
            Witness::parse("<<<assertion|h|c|>>>").expect("the fixture witness parses")
        }
        ReplaySource::ObservedTemporal(_) | ReplaySource::ModelGraph(_) => {
            panic!("the fixture requires a function replay source")
        }
    };
    let package = package
        .clone()
        .with_kani_identity(&identity)
        .expect("the test's selected function binds its harness identity");
    let packet = package
        .backend_witness_packet(&identity, witness)
        .expect("every parameter has a bound harness argument");
    let mut request = package
        .request(&identity, packet)
        .expect("the selected function admits its backend packet");
    // Input-arm cases below exercise QSL's generic request behavior after CG's packet boundary.
    request.source = source;
    request
}

/// Compiles the hand-mirrored native twin `source` and locates `function` in it.
fn compile_native_twin(source: &str, function: &str) -> ReplayPackage {
    let package = ReplayPackage::new(inputs(source, Vec::new()), function)
        .expect("the native twin compiles and declares the function");
    if function == FUNCTION {
        package
            .with_kani_identity(&spine_identity())
            .expect("the retained proof identity binds the native twin")
    } else {
        package
    }
}

/// Replays `values` against `native`, the QSL source compiled and recompiled.
fn replay_against(
    native: &str,
    values: &[(String, WitnessValue)],
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    let package = compile_native_twin(native, FUNCTION);
    replay_falsification(
        "module::proof",
        "balance-never-grows",
        values,
        &package.parameters(),
        |source| request_of(&package, source),
        ReplayLimits::default(),
    )
}

fn values(amount: i64, balance: i64) -> Vec<(String, WitnessValue)> {
    vec![
        (
            "amount_current".to_owned(),
            WitnessValue::Integer(i128::from(amount)),
        ),
        (
            "balance_pre".to_owned(),
            WitnessValue::Integer(i128::from(balance)),
        ),
    ]
}

const VIOLATING_TWIN: &str = "balance_pre + amount_current";
const HEALTHY_TWIN: &str = "balance_pre - amount_current";

fn violation() -> Verdict {
    Verdict::from_category(Category::Violation)
}

fn success() -> Verdict {
    Verdict::from_category(Category::Success)
}

/// A falsifying input that satisfies the proved precondition (`amount <= balance`) replays
/// through QSL's `replay` and settles the same violation: the twin carrying the injected
/// violation evaluates the clause to false at the input. QSL evaluated it; nothing here
/// supplies the verdict.
#[trace("FR-016-AC-9", "TC-026")]
#[test]
fn tc_026_a_falsifying_input_replays_through_qsl_to_the_same_violation() {
    let result =
        replay_against(&native_source(VIOLATING_TWIN), &values(1, 5)).expect("the replay settles");
    assert_eq!(
        result.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(result.category(), Category::Violation);
    assert!(result.disagreement().is_none());
    assert!(
        result.charges().work_units > 0,
        "QSL charged the evaluation"
    );
}

/// The verdict is QSL's evaluation at the witness point: the same twin holds the clause at
/// `amount = 0`, so a witness that is not a counterexample settles `inconclusive` with the
/// proved violation and the replayed success both named.
#[trace("FR-016-AC-9", "FR-016-AC-10", "TC-026")]
#[test]
fn tc_026_the_replayed_verdict_depends_on_the_witness_value() {
    let result =
        replay_against(&native_source(VIOLATING_TWIN), &values(0, 5)).expect("the replay settles");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    let cause = result.disagreement().expect("both verdicts are named");
    assert_eq!(
        (cause.proved(), cause.replayed()),
        (Some(violation()), Some(success()))
    );
}

/// The healthy twin holds the clause at a real counterexample's input, so QSL settles
/// `inconclusive` naming both verdicts: replay never repairs a disagreement into a
/// reproduction.
#[trace("FR-016-AC-4", "FR-016-AC-10", "TC-026")]
#[test]
fn tc_026_a_healthy_native_twin_does_not_reproduce_the_violation() {
    let result =
        replay_against(&native_source(HEALTHY_TWIN), &values(1, 5)).expect("the replay settles");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    let cause = result.disagreement().expect("both verdicts are named");
    assert_eq!(
        (cause.proved(), cause.replayed()),
        (Some(violation()), Some(success()))
    );
}

/// A decoded value no replay parameter binds is refused before any call.
#[trace("FR-016-AC-11", "TC-026")]
#[test]
fn tc_026_a_value_with_no_parameter_is_refused() {
    let mut extra = values(1, 5);
    extra.push(("stray".to_owned(), WitnessValue::Integer(1)));
    let refusal = replay_against(&native_source(VIOLATING_TWIN), &extra)
        .expect_err("`stray` names no parameter");
    assert!(
        matches!(&refusal, SpineReplayError::UnboundArgument { argument } if argument == "stray"),
        "{refusal}"
    );
}

/// A Boolean value replays as the integer 1 or 0: `flag(b) = b` is false for `false`, which
/// reproduces the violation, and true for `true`, which does not.
#[trace("FR-016-AC-9", "TC-026")]
#[test]
fn tc_026_a_boolean_value_replays_as_zero_or_one() {
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function flag using v(b: Boolean): Boolean pure {{ b }}\n"
    );
    let package = compile_native_twin(&source, "flag");
    let replay = |value: bool| {
        replay_falsification(
            "module::proof",
            "flag",
            &[("b".to_owned(), WitnessValue::Boolean(value))],
            &package.parameters(),
            |witness| request_of(&package, witness),
            ReplayLimits::default(),
        )
        .expect("the replay settles")
    };
    assert_eq!(
        replay(false).settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(replay(true).settlement(), WitnessSettlement::Inconclusive);
}

/// Every refusal the adapter has is its own typed error: a delimiter in a transcript field, a
/// node id that makes the transcript inadmissible, a request QSL refuses, and a request whose
/// source is not a witness.
#[trace("FR-016-AC-11", "TC-026")]
#[test]
fn tc_026_each_adapter_refusal_is_its_own_typed_error() {
    let native = native_source(VIOLATING_TWIN);
    let package = compile_native_twin(&native, FUNCTION);
    let parameters = package.parameters();
    let build = |witness| request_of(&package, witness);

    let delimiter = replay_falsification(
        "a|b",
        "c",
        &values(1, 5),
        &parameters,
        build,
        ReplayLimits::default(),
    );
    assert!(matches!(delimiter, Err(SpineReplayError::FieldDelimiter)));

    for value in [
        WitnessValue::Text("x".to_owned()),
        WitnessValue::Uuid("550e8400-e29b-41d4-a716-446655440000".to_owned()),
        WitnessValue::Timestamp("0".to_owned()),
    ] {
        let unsupported = replay_falsification(
            "h",
            "c",
            &[("amount_current".to_owned(), value)],
            &parameters,
            build,
            ReplayLimits::default(),
        );
        assert!(matches!(
            unsupported,
            Err(SpineReplayError::UnsupportedWitnessValue { argument })
                if argument == "amount_current"
        ));
    }

    let bad_node = [ReplayParameter {
        argument: "amount_current",
        node_id: "x>>>y",
    }];
    let transcript = replay_falsification(
        "h",
        "c",
        &[("amount_current".to_owned(), WitnessValue::Integer(1))],
        &bad_node,
        build,
        ReplayLimits::default(),
    );
    assert!(matches!(transcript, Err(SpineReplayError::Transcript(_))));

    let refused = replay_falsification(
        "h",
        "c",
        &values(1, 5),
        &parameters,
        |witness| {
            let mut wire = request_of(&package, witness);
            wire.sources.clear();
            wire
        },
        ReplayLimits::default(),
    );
    assert!(matches!(refused, Err(SpineReplayError::Refused(_))));

    let wrong_arm = replay_falsification(
        "h",
        "c",
        &values(1, 5),
        &parameters,
        |_| {
            let input = package
                .parameters()
                .iter()
                .zip([1_i64, 5])
                .map(|(parameter, value)| CanonicalAssignment {
                    parameter: qsl_replay::NodeKey::new(
                        "skeleton-spine",
                        vec![quire_exact::PathSegment::Text(
                            parameter.argument.to_owned(),
                        )],
                        0,
                    )
                    .expect("a structural node id"),
                    value: WitnessValue::Integer(i128::from(value)),
                })
                .collect();
            request_of(&package, ReplaySource::Input(input))
        },
        ReplayLimits::default(),
    );
    assert!(matches!(wrong_arm, Err(SpineReplayError::WrongArm)));

    let reader = replay_falsification(
        "h",
        "c",
        &values(1, 5),
        &parameters,
        build,
        ReplayLimits::default().with_input_bytes(1),
    );
    assert!(matches!(
        reader,
        Err(SpineReplayError::Refused(refusal))
            if matches!(*refusal, ReplayRefusal::Request(ReplayRequestRefusal::BoundExceeded(_)))
    ));
}

/// One dependency selection of the proved lock, with its own source.
fn dependency_lock() -> DependencyLock {
    DependencyLock {
        identity: "test/units".to_owned(),
        source: locked("lib-units", b"a dependency source"),
    }
}

/// The request carries each selected dependency identity and its exact source reference in
/// ascending identity order. Byte provision is keyed by source identity even when bytes repeat.
#[trace("FR-016-AC-29", "TC-026")]
#[test]
fn tc_026_the_request_package_reference_carries_the_lock_dependencies() {
    let native = native_source(VIOLATING_TWIN);
    let lock = dependency_lock();
    let mut earlier = dependency_lock();
    earlier.identity = "test/aaa".to_owned();
    earlier.source = locked("lib-shared", native.as_bytes());
    let mut shared = dependency_lock();
    shared.identity = "test/mmm".to_owned();
    shared.source = locked("lib-mmm", b"a dependency source");
    let mut lock_inputs = inputs(&native, vec![lock.clone(), shared, earlier]);
    let declared = DeclaredDomain::new(
        ProofBound::new(
            DomainKey::Node {
                node: qsl_replay::NodeKey::new(
                    "skeleton-spine",
                    vec![quire_exact::PathSegment::Text("domain".to_owned())],
                    0,
                )
                .expect("a structural node id"),
                path: Vec::new(),
            },
            Some(DomainKind::Integer),
            FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64))
                .expect("nonempty range"),
        )
        .expect("integer range pairs with integer"),
    );
    lock_inputs.declared_domains.push(declared.clone());
    let expected_stage_limits = lock_inputs.stage_limits.clone();
    let package = ReplayPackage::new(lock_inputs, FUNCTION).expect("the twin compiles");
    let wire = request_of(&package, ReplaySource::Input(Vec::new()));

    assert_eq!(wire.stage_limits, expected_stage_limits);
    assert_eq!(wire.declared_domains, vec![declared]);

    let identities: Vec<_> = wire
        .dependencies
        .iter()
        .map(|d| d.identity.as_str())
        .collect();
    assert_eq!(identities, ["test/aaa", "test/mmm", "test/units"]);
    // The backend member is the identity string alone.
    assert_eq!(wire.backend, "kani");
    // Destructure without `..` to keep the selected wire shape explicit.
    let qsl_replay::DependencyEntryWire { identity, sources } = &wire.dependencies[2];
    assert_eq!(identity, "test/units");
    let [source] = sources.as_slice() else {
        panic!("the dependency's own lock source: {sources:?}")
    };
    assert_eq!(
        source,
        &(
            AUTHORITY.to_owned(),
            "lib-units".to_owned(),
            NAMESPACE.to_owned(),
            REVISION.to_owned()
        )
    );
    for (entry, expected_identity, expected_source) in [
        (&wire.dependencies[0], "test/aaa", "lib-shared"),
        (&wire.dependencies[1], "test/mmm", "lib-mmm"),
        (&wire.dependencies[2], "test/units", "lib-units"),
    ] {
        assert_eq!(entry.identity, expected_identity);
        assert_eq!(
            entry.sources,
            vec![(
                AUTHORITY.to_owned(),
                expected_source.to_owned(),
                NAMESPACE.to_owned(),
                REVISION.to_owned()
            )]
        );
    }
    let supplied: BTreeMap<_, _> = wire
        .byte_provision
        .iter()
        .map(|(authority, identity, bytes)| {
            ((authority.as_str(), identity.as_str()), bytes.as_slice())
        })
        .collect();
    assert_eq!(
        supplied.len(),
        4,
        "each source identity has one byte provision"
    );
    assert_eq!(supplied[&(AUTHORITY, IDENTITY)], native.as_bytes());
    assert_eq!(supplied[&(AUTHORITY, "lib-shared")], native.as_bytes());
    for identity in ["lib-mmm", "lib-units"] {
        assert_eq!(supplied[&(AUTHORITY, identity)], b"a dependency source");
    }

    let without = request_of(
        &compile_native_twin(&native, FUNCTION),
        ReplaySource::Input(Vec::new()),
    );
    assert!(without.dependencies.is_empty());
}

/// A lock whose only defect is one library identity selected twice has no admitted request: QSL's
/// `DependencyInput::new` refuses it as `DuplicateIdentity` (code `invalid_package`), and it
/// arrives as `DependencyLockError::Input`. This crate carries no duplicate check of its own.
#[trace("FR-016-AC-24", "TC-026")]
#[test]
fn tc_026_a_lock_repeating_a_dependency_is_refused() {
    let refusal = ReplayPackage::new(
        inputs(
            &native_source(VIOLATING_TWIN),
            vec![dependency_lock(), dependency_lock()],
        ),
        FUNCTION,
    )
    .expect_err("the repeated identity is refused");
    let ReplayPackageError::Dependencies(DependencyLockError::Input(input)) = &refusal else {
        panic!("expected a refusal from QSL's dependency input, got {refusal}");
    };
    let DependencyInputRefusal::DuplicateIdentity { identity, .. } = input else {
        panic!("expected DuplicateIdentity, got {input}");
    };
    assert_eq!(identity.to_string(), "test/units");
    assert_eq!(input.code(), Code::InvalidPackage);
    assert_eq!(input.cause(), Some("conflicting-definition"));
}

const UNITS_IDENTITY: &str = "test:units";
const BIG: &str = "function big using v(x: Int[0, 9]): Boolean pure { x > 5 }\n";

/// A library `test/units` holding `body`, as a lock source.
fn units_library(body: &str) -> LockedSource {
    let source = format!("language \"ix:native\" edition \"1-draft\";\n{PROFILE}{body}");
    locked(UNITS_IDENTITY, source.as_bytes())
}

/// A unit `q(x) = u::big(x)` importing `library` by identity alone, and the lock selecting
/// that library's source.
fn importing_inputs(library: &LockedSource) -> (ReplayInputs, DependencyLock) {
    let unit = format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         import \"test/units\" as u;\n\
         function q using v(x: Int[0, 9]): Boolean pure {{ u::big(x) }}\n"
    );
    let lock = DependencyLock {
        identity: "test/units".to_owned(),
        source: library.clone(),
    };
    (inputs(&unit, vec![lock.clone()]), lock)
}

fn replay_q(
    package: &ReplayPackage,
    x: i64,
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    replay_falsification(
        "module::proof",
        "q",
        &[("x".to_owned(), WitnessValue::Integer(i128::from(x)))],
        &package.parameters(),
        |source| request_of(package, source),
        ReplayLimits::default(),
    )
}

/// A unit that imports a locked dependency compiles with it and replays through
/// `qsl_replay::replay`: the dependency's `big` is evaluated, so `x = 3` (not big) reproduces the
/// proved violation and `x = 7` (big) does not.
#[trace("FR-016-AC-14", "TC-026")]
#[test]
fn tc_026_a_unit_importing_a_locked_dependency_replays_to_a_reproduced_verdict() {
    let (lock_inputs, _) = importing_inputs(&units_library(BIG));
    let package =
        ReplayPackage::new(lock_inputs, "q").expect("the unit compiles with its dependency");
    let wire = request_of(&package, ReplaySource::Input(Vec::new()));
    assert_eq!(wire.dependencies.len(), 1);

    let reproduced = replay_q(&package, 3).expect("the replay settles");
    assert_eq!(
        reproduced.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(reproduced.category(), Category::Violation);

    let held = replay_q(&package, 7).expect("the replay settles");
    assert_eq!(held.settlement(), WitnessSettlement::Inconclusive);
}

/// The dependency is compiled from the lock's own source, not assumed: a unit whose import names
/// no supplied library is refused at the call site.
#[trace("FR-016-AC-16", "TC-026")]
#[test]
fn tc_026_an_import_with_no_locked_dependency_is_refused() {
    let (mut lock_inputs, _) = importing_inputs(&units_library(BIG));
    lock_inputs.dependencies.clear();
    let refusal =
        ReplayPackage::new(lock_inputs, "q").expect_err("no library satisfies the import");
    assert!(
        matches!(&refusal, ReplayPackageError::CallSite(cause) if matches!(**cause, CallSiteRefusal::Import { .. })),
        "{refusal}"
    );
}

/// CG compares each request dependency with the package admitted at the call site before replay.
/// A request mutation cannot silently select a different imported source or dependency ordering.
#[trace("FR-016-AC-30", "TC-026")]
#[test]
fn tc_026_a_changed_dependency_selection_is_refused_before_replay() {
    let (mut lock_inputs, _) = importing_inputs(&units_library(BIG));
    let mut extra = dependency_lock();
    extra.identity = "test/extra".to_owned();
    extra.source = locked("lib-extra", b"an unselected dependency source");
    lock_inputs.dependencies.push(extra);
    let package =
        ReplayPackage::new(lock_inputs, "q").expect("the unit compiles with its dependency");
    let base = request_of(&package, ReplaySource::Input(Vec::new()));
    assert_eq!(base.dependencies.len(), 2);

    let mut missing = base.clone();
    missing.dependencies.pop();
    let mut extra = base.clone();
    extra.dependencies.push(qsl_replay::DependencyEntryWire {
        identity: "test/other".to_owned(),
        sources: vec![(
            AUTHORITY.to_owned(),
            "lib-other".to_owned(),
            NAMESPACE.to_owned(),
            REVISION.to_owned(),
        )],
    });
    let mut reordered = base.clone();
    reordered.dependencies.swap(0, 1);
    let mut changed = base.clone();
    changed.dependencies[1].sources[0].1 = "lib-units-other".to_owned();
    changed.byte_provision.push((
        AUTHORITY.to_owned(),
        "lib-units-other".to_owned(),
        units_library(BIG).bytes,
    ));

    for (case, wire, position) in [
        ("missing", missing, 1),
        ("extra", extra, 2),
        ("reordered", reordered, 0),
        ("changed source", changed, 1),
    ] {
        let expected = base.dependencies.get(position).cloned();
        let actual = wire.dependencies.get(position).cloned();
        let mut calls = 0;
        let refusal = package
            .replay_request_through(wire, |_| {
                calls += 1;
                panic!("{case} dependency selection must stop before QSL replay")
            })
            .expect_err("the admitted dependency selection must match exactly");
        let SpineReplayError::DependencySelectionMismatch(cause) = refusal else {
            panic!("{case} returned a different refusal: {refusal}")
        };
        assert_eq!(cause.position, position, "{case}");
        assert_eq!(cause.expected, expected, "{case}");
        assert_eq!(cause.actual, actual, "{case}");
        assert_eq!(calls, 0, "{case} reached QSL replay");
    }

    let mut calls = 0;
    let control = package.replay_request_through(base, |_| {
        calls += 1;
        Err(ReplayRefusal::Fault(InternalFault::new(
            "replay",
            "injected-control-fault",
        )))
    });
    assert_eq!(calls, 1, "the unchanged admitted selection reaches replay");
    assert!(
        matches!(control, Err(SpineReplayError::Refused(_))),
        "the injected executor fault must remain typed: {control:?}"
    );
}

/// Kind and argument domains remain on the retained proving-run record. They do not rekey QSL's
/// producer identity, so a packet cannot use that same identity to change either claim member.
#[trace("FR-016-AC-27", "FR-016-AC-28", "TC-026")]
#[test]
fn tc_026_a_held_producer_identity_rejects_changed_claim_members() {
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let original = spine_identity();
    let witness = Witness::parse("<<<assertion|h|c|>>>").expect("the fixture witness parses");
    let packet = package
        .backend_witness_packet(&original, witness)
        .expect("the original claim binds to the selected function");
    let control = package
        .request(&original, packet.clone())
        .expect("the matching claim admits its packet");
    assert_eq!(
        control.obligation_identity,
        package
            .obligation_identity(&original)
            .expect("QSL issued the selected identity")
    );

    let mut changed_kind = original.clone();
    changed_kind.kind = ObligationKind::Precondition;
    let mut changed_domain = original.clone();
    changed_domain.arguments[0]
        .integer_bounds
        .as_mut()
        .expect("the fixture argument is bounded")
        .maximum -= 1;
    for (name, changed) in [("kind", changed_kind), ("domain", changed_domain)] {
        assert_eq!(
            package
                .obligation_identity(&changed)
                .expect("the binding remains finite"),
            package
                .obligation_identity(&original)
                .expect("QSL issued the identity"),
            "{name} is a record member, not an identity component"
        );
        assert!(
            matches!(
                package.request(&changed, packet.clone()),
                Err(FunctionPacketError::ProvingRunMismatch)
            ),
            "{name} must refuse before a replay request exists"
        );
    }
}

/// A lock selecting a library the unit does not import compiles, but QSL refuses the request
/// naming it as unselected.
#[trace("FR-016-AC-18", "TC-026")]
#[test]
fn tc_026_qsl_refuses_a_dependency_the_unit_does_not_select() {
    let package = ReplayPackage::new(
        inputs(&native_source(VIOLATING_TWIN), vec![dependency_lock()]),
        FUNCTION,
    )
    .expect("the twin compiles");
    let refusal = replay_falsification(
        "module::proof",
        "balance-never-grows",
        &values(1, 5),
        &package.parameters(),
        |source| request_of(&package, source),
        ReplayLimits::default(),
    )
    .expect_err("QSL refuses the unselected dependency");
    assert!(
        matches!(
            &refusal,
            SpineReplayError::Refused(cause)
                if matches!(
                    cause.as_ref(),
                    ReplayRefusal::DependencySelections(DependencySelectionsCause::Unselected { identity })
                        if identity.as_str() == "test/units"
                )
        ),
        "{refusal}"
    );
}

/// Two lock libraries whose sources share an owner are no dependency input, so the package is
/// refused before the call site compiles anything.
#[trace("FR-016-AC-19", "TC-026")]
#[test]
fn tc_026_libraries_sharing_a_source_owner_are_refused() {
    let first = dependency_lock();
    let mut second = dependency_lock();
    second.identity = "test/other".to_owned();
    let refusal = ReplayPackage::new(
        inputs(&native_source(VIOLATING_TWIN), vec![first, second]),
        FUNCTION,
    )
    .expect_err("one source owner per library");
    assert!(
        matches!(
            &refusal,
            ReplayPackageError::Dependencies(DependencyLockError::Input(_))
        ),
        "{refusal}"
    );
}

/// A lock library whose source has the unit's own owner is refused by QSL's call site.
#[trace("FR-016-AC-20", "TC-026")]
#[test]
fn tc_026_a_library_sharing_the_units_source_owner_is_refused() {
    let mut lock = dependency_lock();
    lock.source = locked(IDENTITY, b"a dependency source");
    let refusal = ReplayPackage::new(inputs(&native_source(VIOLATING_TWIN), vec![lock]), FUNCTION)
        .expect_err("the unit and a library never share an owner");
    assert!(
        matches!(
            &refusal,
            ReplayPackageError::CallSite(cause)
                if matches!(**cause, CallSiteRefusal::DependencyInput(_))
        ),
        "{refusal}"
    );
}

/// Playback for a different retained proving-run function cannot be replayed through the
/// previously bound QSL twin merely because its argument names have the same shape.
#[trace("FR-024-AC-43", "FR-024-AC-46", "TC-035")]
#[test]
fn tc_035_function_replay_refuses_another_proving_run_identity_before_execution() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let mut other = harness.clone();
    other.contract_symbol = Some("other_function_contract".to_owned());
    other.subject_path = Some("crate::subject::other_function".to_owned());
    let packet = package
        .backend_witness_packet(
            &harness,
            Witness::parse("<<<assertion|h|c|>>>").expect("the fixture witness parses"),
        )
        .expect("the retained proof identity binds the twin");
    assert!(matches!(
        package.request(&other, packet),
        Err(quire_contract_codegen::FunctionPacketError::ProvingRunMismatch)
    ));
    let mut calls = 0;
    let verdict = replay_counterexample_through(
        &other,
        &typed_playback(&playback(&harness, 1, 5)),
        &package,
        |_| {
            calls += 1;
            panic!("a different proving-run identity must not reach QSL");
        },
    )
    .expect("a mismatched identity is an evidence failure");
    assert_eq!(
        verdict,
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::PackageIdentity)
    );
    assert_eq!(calls, 0);
}

/// A direct identity fixture for the surviving replay API. No generated V1 harness is implied.
fn spine_identity() -> KaniObligationIdentity {
    identity_fixture(ObligationKind::Postcondition, "balance-never-grows", 20)
}

fn identity_fixture(kind: ObligationKind, clause: &str, line: u32) -> KaniObligationIdentity {
    let span: SourceSpan = serde_json::from_value(json!({
        "start": {"source": {"document": "skeleton-spine", "revision": 1}, "line": line, "column": 1, "byte_offset": line},
        "end": {"source": {"document": "skeleton-spine", "revision": 1}, "line": line, "column": 2, "byte_offset": line + 1}
    }))
    .expect("valid test source span");
    let bounds = KaniIntegerBounds {
        domain: IntegerDomain::Signed,
        minimum: 0,
        maximum: 1000,
        overflow: OverflowPolicy::Reject,
    };
    let argument = |identifier: &str| ObligationBinding {
        identifier: identifier.to_owned(),
        role: KaniBindingRole::Argument,
        primitive_type: KaniPrimitiveType::I64,
        integer_bounds: Some(bounds.clone()),
        dependencies: Vec::new(),
    };
    KaniObligationIdentity {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        kind,
        clause: ClauseRef::new(
            RequirementRef::parse("test/skeleton-spine", "FR-200", 1).unwrap(),
            ClauseId::new(clause).unwrap(),
        ),
        source_span: span,
        oracles: Vec::new(),
        module_symbol: ModuleSymbol::try_from("kani_skeleton_spine").unwrap(),
        harness_symbol: HarnessSymbol::try_from("proof").unwrap(),
        contract_symbol: Some("contract".to_owned()),
        subject_path: Some("crate::subject::withdraw".to_owned()),
        arguments: vec![argument("amount_current"), argument("balance_pre")],
        results: Vec::new(),
        solver: KaniSolver::Cadical,
        unwind: 4,
        options: Vec::new(),
    }
}

/// Parses a complete synthetic transcript under the same finite limits as the replay package.
fn typed_playback(transcript: &str) -> Playback<'_> {
    parse_assertion_playback(
        transcript,
        "function-replay-test",
        "function",
        FUNCTION_PLAYBACK_LIMITS.bytes,
        FUNCTION_PLAYBACK_LIMITS.values,
    )
    .expect("the complete assertion playback parses")
}

/// A Kani playback transcript for `harness` carrying the two argument values in persisted
/// order (`amount_current`, `balance_pre`).
fn playback(harness: &KaniObligationIdentity, amount: i64, balance: i64) -> String {
    let values = [amount, balance]
        .map(|value| {
            let bytes = value.to_le_bytes().map(|byte| byte.to_string()).join(", ");
            format!("        // {value}\n        vec![{bytes}],\n")
        })
        .concat();
    format!(
        "/// Test generated for harness `{}::{}`\n\
         /// Check for `assertion`: \"synthetic assertion\"\n\
         #[test]\n\
         fn kani_concrete_playback_synthetic() {{\n\
             let concrete_vals: Vec<Vec<u8>> = vec![\n{values}    ];\n\
             kani::concrete_playback_run(concrete_vals, synthetic);\n\
         }}\n",
        harness.module_symbol, harness.harness_symbol
    )
}

/// A counterexample in domain that the violating twin falsifies is a reproduced violation.
#[trace("FR-016-AC-3", "TC-026")]
#[test]
fn tc_026_an_in_domain_counterexample_the_twin_falsifies_is_reproduced() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let verdict = replay_counterexample(
        &harness,
        &typed_playback(&playback(&harness, 1, 5)),
        &package,
    )
    .expect("the replay settles");
    assert_eq!(verdict, ReplayVerdict::Reproduced);
}

#[trace("TC-026", "FR-016-AC-6")]
#[test]
fn tc_026_function_replay_refuses_poisoned_suffixes_at_both_decode_bounds() {
    let harness = spine_identity();
    let transcript = playback(&harness, 1, 5);
    let source = native_source(VIOLATING_TWIN);
    let limits = WitnessDecodeLimits {
        bytes: transcript.len(),
        values: 2,
    };
    let make_package = |limits| {
        let mut run = inputs(&source, Vec::new());
        run.decode_limits = limits;
        ReplayPackage::new(run, FUNCTION)
            .expect("native twin compiles")
            .with_kani_identity(&harness)
            .expect("the retained proof identity binds the twin")
    };
    let package = make_package(limits);
    let parsed = parse_assertion_playback(
        &transcript,
        harness.harness_symbol.as_str(),
        harness.module_symbol.as_str(),
        limits.bytes,
        limits.values,
    )
    .expect("the complete captured playback fits the proving-run bounds");
    assert_eq!(
        replay_counterexample(&harness, &parsed, &package).expect("at bounds"),
        ReplayVerdict::Reproduced
    );
    let mut separate = inputs(&source, Vec::new());
    separate.decode_limits = limits;
    separate.replay_limits = ReplayLimits::default().with_input_bytes(1);
    let separate = ReplayPackage::new(separate, FUNCTION)
        .expect("native twin compiles")
        .with_kani_identity(&harness)
        .expect("the retained proof identity binds the twin");
    let mut calls = 0;
    let refused =
        replay_counterexample_through(&harness, &typed_playback(&transcript), &separate, |_| {
            calls += 1;
            Err(ReplayRefusal::Fault(InternalFault::new(
                "replay",
                "stop after decode",
            )))
        });
    assert!(
        matches!(refused, Err(SpineReplayError::Packet(_))),
        "QSL's input_bytes bound must not limit CG decoding: {refused:?}"
    );
    assert_eq!(
        calls, 0,
        "packet admission must reject the small QSL input_bytes bound before execution"
    );
    for (poisoned, limits, code) in [
        (
            format!("{transcript}\ntrailing captured output"),
            limits,
            "cg_witness_decode_bytes_limit",
        ),
        (
            transcript.replace(
                "    ];",
                "        // 0\n        vec![0, 0, 0, 0, 0, 0, 0, 0],\n    ];",
            ),
            WitnessDecodeLimits {
                bytes: FUNCTION_PLAYBACK_LIMITS.bytes,
                values: 2,
            },
            "cg_witness_decode_values_limit",
        ),
    ] {
        let bounded_parse = parse_assertion_playback(
            &poisoned,
            harness.harness_symbol.as_str(),
            harness.module_symbol.as_str(),
            limits.bytes,
            limits.values,
        );
        assert_eq!(
            bounded_parse
                .expect_err("the output reader must enforce the requested bound")
                .code
                .as_str(),
            code
        );
        // A wider but finite caller parse isolates replay's independent limit check.
        let wider_limits = WitnessDecodeLimits {
            bytes: poisoned.len(),
            values: 3,
        };
        let parsed = parse_assertion_playback(
            &poisoned,
            harness.harness_symbol.as_str(),
            harness.module_symbol.as_str(),
            wider_limits.bytes,
            wider_limits.values,
        )
        .expect("the complete transcript parses under wider finite limits");
        let package = make_package(limits);
        let verdict = replay_counterexample(&harness, &parsed, &package)
            .expect("over-bound evidence yields a verdict");
        assert!(
            matches!(
                verdict,
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(ref cause))
                    if cause.code == code
            ),
            "{verdict:?}"
        );
        let through = replay_counterexample_through(&harness, &parsed, &package, |_| {
            panic!("over-bound playback must not reach the executor")
        })
        .expect("over-bound evidence yields a verdict");
        assert!(
            matches!(
                through,
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(ref cause))
                    if cause.code == code
            ),
            "{through:?}"
        );
        let wider = make_package(wider_limits);
        let control = replay_counterexample(&harness, &parsed, &wider)
            .expect("poisoned suffix produces a decode verdict");
        if code == "cg_witness_decode_bytes_limit" {
            assert_eq!(control, ReplayVerdict::Reproduced);
        } else {
            assert!(
                matches!(
                    control,
                    ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(ref cause))
                        if cause.code == "kani_witness_arity_mismatch"
                ),
                "{control:?}"
            );
        }
    }
    let duplicated = format!("{transcript}{transcript}");
    let refused = parse_assertion_playback(
        &duplicated,
        harness.harness_symbol.as_str(),
        harness.module_symbol.as_str(),
        duplicated.len(),
        limits.values,
    )
    .expect_err("two assertion blocks cannot become typed playback");
    assert_eq!(
        refused.code.as_str(),
        "kani_witness_multiple_assertions_refused"
    );
}

/// An executor fault crosses the public counterexample replay path as the same typed refusal.
#[trace("FR-016-AC-12", "TC-026")]
#[test]
fn tc_026_an_executor_fault_remains_a_typed_replay_refusal() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let fault = InternalFault::new("replay", "injected-executor-fault");
    let mut calls = 0;
    let result = replay_counterexample_through(
        &harness,
        &typed_playback(&playback(&harness, 1, 5)),
        &package,
        |_| {
            calls += 1;
            Err(ReplayRefusal::Fault(fault.clone()))
        },
    );
    assert_eq!(calls, 1, "the decoded counterexample reached the executor");
    assert!(
        matches!(result, Err(SpineReplayError::Refused(refusal))
            if matches!(refusal.as_ref(), ReplayRefusal::Fault(returned) if *returned == fault)),
        "the fault must remain a typed replay refusal"
    );
}

/// A decoded value outside its argument's domain is evidence failure, and the replay never
/// runs: the same value inside the domain reproduces against the same twin.
#[trace("FR-016-AC-2", "TC-026")]
#[test]
fn tc_026_an_out_of_domain_counterexample_is_evidence_failure() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let verdict = replay_counterexample(
        &harness,
        &typed_playback(&playback(&harness, 1, 5000)),
        &package,
    )
    .expect("the verdict is reached");
    assert_eq!(
        verdict,
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Domain {
            argument: "balance_pre".to_owned()
        })
    );
}

/// The public replay entry point admits both endpoints of each argument's declared domain and
/// refuses the adjacent values before invoking its executor.
#[trace("TC-035", "FR-024-AC-3")]
#[test]
fn tc_035_public_function_domain_boundaries_stop_before_replay() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    for (argument, amount, balance, expected_domain_failure) in [
        ("amount_current", 0, 5, false),
        ("amount_current", 1000, 5, false),
        ("amount_current", -1, 5, true),
        ("amount_current", 1001, 5, true),
        ("balance_pre", 5, 0, false),
        ("balance_pre", 5, 1000, false),
        ("balance_pre", 5, -1, true),
        ("balance_pre", 5, 1001, true),
    ] {
        let mut calls = 0;
        let result = replay_counterexample_through(
            &harness,
            &typed_playback(&playback(&harness, amount, balance)),
            &package,
            |wire| {
                calls += 1;
                qsl_replay::replay(wire, ReplayLimits::default())
            },
        );
        if expected_domain_failure {
            assert_eq!(calls, 0, "{argument}={amount},{balance} reached replay");
            assert_eq!(
                result.expect("domain failure is an evidence verdict"),
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Domain {
                    argument: argument.to_owned(),
                })
            );
        } else {
            assert_eq!(calls, 1, "{argument}={amount},{balance}: {result:?}");
            assert!(result.is_ok(), "{argument}={amount},{balance}: {result:?}");
        }
    }
}

/// The output reader refuses a missing block before replay; typed playback with the wrong arity
/// or byte width remains a decode evidence failure carrying the decoder's own cause code.
#[trace("FR-016-AC-1", "TC-026")]
#[test]
fn tc_026_malformed_counterexamples_stop_at_output_or_decode() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let one_value = playback(&harness, 1, 5).replacen(
        "        // 5\n        vec![5, 0, 0, 0, 0, 0, 0, 0],\n",
        "",
        1,
    );
    let short_value =
        playback(&harness, 1, 5).replacen("vec![5, 0, 0, 0, 0, 0, 0, 0]", "vec![5, 0]", 1);
    let missing = parse_assertion_playback(
        "not a playback block",
        "function-replay-test",
        "function",
        "not a playback block".len(),
        FUNCTION_PLAYBACK_LIMITS.values,
    );
    assert_eq!(
        missing
            .expect_err("missing playback must stop at the output reader")
            .code
            .as_str(),
        "kani_witness_harness_missing"
    );
    for (transcript, code) in [
        (one_value, "kani_witness_arity_mismatch"),
        (short_value, "kani_witness_width_mismatch"),
    ] {
        let verdict = replay_counterexample(&harness, &typed_playback(&transcript), &package)
            .expect("the verdict is reached");
        let ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(failure)) = &verdict else {
            panic!("expected a decode failure for {transcript:?}: {verdict:?}");
        };
        assert_eq!(failure.code, code, "{transcript:?}");
    }
}

/// A transcript recorded for another harness is a decode evidence failure and is never
/// replayed: the same values under the right harness reproduce.
#[trace("FR-016-AC-5", "TC-026")]
#[test]
fn tc_026_a_counterexample_for_another_harness_is_never_replayed() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let other = playback(&harness, 1, 5).replace(harness.harness_symbol.as_str(), "sibling");
    let verdict = replay_counterexample(&harness, &typed_playback(&other), &package)
        .expect("the verdict is reached");
    assert!(
        matches!(
            &verdict,
            ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(failure))
                if failure.code == "cg_witness_harness_identity_mismatch"
        ),
        "{verdict:?}"
    );
}

/// A replay that runs and does not reproduce the violation is evidence failure naming QSL's
/// settlement, never a clause success.
#[trace("FR-016-AC-4", "TC-026")]
#[test]
fn tc_026_a_counterexample_the_twin_holds_is_evidence_failure() {
    let harness = spine_identity();
    let package = compile_native_twin(&native_source(HEALTHY_TWIN), FUNCTION);
    let verdict = replay_counterexample(
        &harness,
        &typed_playback(&playback(&harness, 1, 5)),
        &package,
    )
    .expect("the replay settles");
    assert_eq!(verdict, not_reproduced());
}

/// The replay verdict is a function of the exact witness values: the twin that is false only at
/// `(1, 5)` is reproduced by the transcript carrying `(1, 5)` and by no neighbouring transcript,
/// so no constant or substituted witness can stand in for the decoded one.
#[trace("FR-016-AC-9", "TC-026")]
#[test]
fn tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values() {
    let harness = spine_identity();
    let package = compile_native_twin(&point_source(1, 5), FUNCTION);
    let replay = |amount: i64, balance: i64| {
        replay_counterexample(
            &harness,
            &typed_playback(&playback(&harness, amount, balance)),
            &package,
        )
        .expect("the replay settles")
    };
    assert_eq!(replay(1, 5), ReplayVerdict::Reproduced);
    for (amount, balance) in [(0, 5), (2, 5), (1, 4), (1, 6), (5, 1)] {
        assert_eq!(
            replay(amount, balance),
            not_reproduced(),
            "({amount}, {balance}) is not the witness point"
        );
    }
    // The independent reader agrees with the synthetic transcript's own values.
    assert_eq!(
        printed_values(&playback(&harness, 1, 5)),
        (vec![1, 5], vec![1, 5])
    );
}

/// The verdict of a replay that ran and settled `inconclusive` because the twin holds the clause:
/// the proving run's verdict was a violation and the replay's a success.
fn not_reproduced() -> ReplayVerdict {
    ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
        settlement: WitnessSettlement::Inconclusive,
        category: Category::Success,
        disagreement: Some(DisagreementCause::Verdicts {
            proved: violation(),
            replayed: success(),
        }),
    })
}

/// The integers a Kani playback block prints, read twice and with no use of the decoder under
/// test: the `// {value}` comment line above each `vec![..]` entry, and the entry's own bytes read
/// as a little-endian `i64`. Returns `(comments, bytes)` in block order.
fn printed_values(transcript: &str) -> (Vec<i64>, Vec<i64>) {
    let mut comments = Vec::new();
    let mut bytes = Vec::new();
    for line in transcript.lines().map(str::trim) {
        if let Some(text) = line.strip_prefix("// ") {
            if let Ok(value) = text.parse::<i64>() {
                comments.push(value);
            }
        } else if let Some(inner) = line
            .strip_prefix("vec![")
            .and_then(|rest| rest.strip_suffix("],"))
        {
            let raw: Vec<u8> = inner
                .split(',')
                .map(|byte| byte.trim().parse().expect("a byte"))
                .collect();
            bytes.push(i64::from_le_bytes(raw.try_into().expect("eight bytes")));
        }
    }
    (comments, bytes)
}
