//! Execute the same emitted guards with genuine checked-owner inputs.
use ix_trace_rs::trace;
use qsl_replay::{
    acquire_checked_closure, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_codegen::{CheckedFunctionResources, CheckedFunctionSource};
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};
use quire_eval::{BodyFailure, CallFailure, FamilyOutcome};
use quire_exact::{
    CancelCause, CheckedInvariantCause, FieldValue, Integer, LimitKind, Meter, Outcome, Refusal,
    ScalarLimits, Value, ValueType,
};
use quire_semantic_value::{
    declaration::{
        DeclarationCause, EnvironmentFailure, EnvironmentLimitKind, TypeEnvironmentLimits,
    },
    location::{Location, Origin},
    object_closure::ObjectClosure,
};

mod healthy {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/healthy.rs"));
}
mod wrong {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/wrong.rs"));
}
mod undefined {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/undefined.rs"));
}
mod refused {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/refused.rs"));
}
mod incomplete {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/incomplete.rs"));
}
mod duplicate {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/duplicate.rs"));
}
mod constructor_allocation {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/constructor_allocation.rs"
    ));
}
mod constructor_capacity {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/constructor_capacity.rs"
    ));
}
mod membership_allocation {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/membership_allocation.rs"
    ));
}
mod membership_capacity {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/membership_capacity.rs"
    ));
}
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture.rs"));

fn identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "ir595-agreement")
}
fn acquired() -> qsl_replay::CheckedClosureCompilation {
    acquire_checked_closure(
        CompileUnit {
            source: identity(),
            package_name: "ir595-agreement".into(),
            path: "ir595-agreement.native",
            bytes: SOURCE.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("real checked closure")
}
fn package() -> CheckedPackageV2 {
    let acquired = acquired();
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        acquired.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("IR admission: {other:?}"),
    }
}
fn selection(
    acquired: &qsl_replay::CheckedClosureCompilation,
    package: &CheckedPackageV2,
    name: &str,
) -> qsl_replay::CallableSelection {
    let function = &package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.node_tag.as_ref() == "function"
                && node
                    .declaration
                    .as_ref()
                    .is_some_and(|d| d.qualified_name == [Box::<str>::from(name)])
        })
        .expect("actual callable")
        .node_id;
    quire_contract_codegen::select_checked_callable(package, package, function, acquired)
        .expect("same acquisition and owner")
}
fn source(
    dependencies: &DependencyInput,
    resources: CheckedFunctionResources,
) -> CheckedFunctionSource<'_> {
    CheckedFunctionSource {
        source: identity(),
        path: "ir595-agreement.native",
        bytes: SOURCE.as_bytes(),
        packages: &[],
        dependencies,
        limits: StageLimits::default(),
        replay_limits: ReplayLimits::default(),
        body_resources: resources,
    }
}
fn nominal_value() -> Value {
    let package = package();
    let acquired = acquired();
    let selected = selection(&acquired, &package, "eq");
    let views =
        quire_contract_codegen::checked_function_declarations(&package, &acquired, &selected)
            .unwrap();
    assert_eq!(
        views.len(),
        3,
        "Holder, Choice and Pair appear once despite repeated references"
    );
    let named = |name: &str| {
        views
            .iter()
            .find(|v| {
                v.qualified_name()
                    .iter()
                    .map(quire_exact::Identifier::as_str)
                    .eq([name])
            })
            .unwrap()
    };
    let types = healthy::inspect_environment(
        &package,
        &acquired,
        &selected,
        &CheckedFunctionResources::default(),
    )
    .expect("owner declarations construct successfully");
    assert_eq!(types.composites().count(), 3);
    let sv_shape = types
        .composite(named("Holder").key().clone())
        .expect("same Holder key");
    let quire_semantic_value::declaration::CompositeShape::Record(fields) = sv_shape.shape() else {
        panic!("checked record form");
    };
    assert_eq!(
        fields
            .iter()
            .map(|field| (field.name(), field.presence()))
            .collect::<Vec<_>>(),
        vec![
            ("pair", quire_exact::Presence::Required),
            ("shape", quire_exact::Presence::Required),
            ("flag", quire_exact::Presence::Optional)
        ],
        "native declaration assembly must preserve each actual owner presence"
    );

    let pair = types
        .tuple(
            named("Pair").key().clone(),
            vec![Value::Boolean(true), Value::Integer(Integer::from(1_i64))],
        )
        .unwrap();
    let choice = named("Choice");
    let qsl_replay::NominalDeclarationMember::UnionMember { variant, .. } = &choice.members()[0]
    else {
        panic!("actual Empty union member");
    };
    let empty = types
        .union(choice.key().clone(), variant.clone(), vec![])
        .unwrap();
    types
        .record(
            named("Holder").key().clone(),
            vec![
                ("pair", FieldValue::Present(pair)),
                ("shape", FieldValue::Present(empty)),
            ],
        )
        .unwrap()
}
fn objects() -> ObjectClosure {
    ObjectClosure::new(
        &quire_semantic_value::declaration::TypeEnvironment::default(),
        [],
        &[],
    )
    .expect("empty object closure")
}
fn meter() -> Meter {
    Meter::new(ScalarLimits {
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
    })
}
fn counters(meter: &Meter) -> Vec<u64> {
    [
        LimitKind::IntegerBits,
        LimitKind::DecimalDigits,
        LimitKind::ScaleExpansion,
        LimitKind::TextInputBytes,
        LimitKind::TextScalars,
        LimitKind::NormalizedScalars,
        LimitKind::UnitEdges,
        LimitKind::ValueOccurrences,
        LimitKind::WorkUnits,
        LimitKind::ResultUnits,
    ]
    .map(|kind| meter.consumed(kind))
    .to_vec()
}
fn invariant(evaluation: &quire_eval::Evaluation, expected: CheckedInvariantCause) {
    assert!(
        matches!(&evaluation.outcome,
        FamilyOutcome::Evaluated(Outcome::Refused(Refusal::CheckedInvariant { cause })) if *cause == expected),
        "exact typed cause: {:?}",
        evaluation.outcome
    );
}

#[trace("TC-057", "FR-042-AC-2", "FR-021-AC-38", "FR-021-AC-40")]
#[test]
fn result_membership_uses_real_signature_and_preserves_stopped_arms_and_charges() {
    let package = package();
    let dependencies = DependencyInput::default();
    let args = || vec![Value::Boolean(true), Value::Boolean(true)];
    let mut baseline = meter();
    let healthy = healthy::oracle_bool_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        args(),
        objects(),
        &mut baseline,
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        &healthy.outcome,
        FamilyOutcome::Evaluated(Outcome::Completed(Value::Boolean(true)))
    ));
    let expected = counters(&baseline);
    let mut actual = meter();
    let mismatch = wrong::oracle_bool_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        args(),
        objects(),
        &mut actual,
    )
    .unwrap()
    .unwrap();
    invariant(&mismatch, CheckedInvariantCause::GeneratedUnexpectedOutcome);
    assert_eq!(
        mismatch.location,
        Some(Location::root(Origin::Body {
            function: "bool_eq".into(),
            index: 1
        }))
    );
    assert_eq!(counters(&actual), expected);
    let mut actual = meter();
    let stopped = undefined::oracle_bool_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        args(),
        objects(),
        &mut actual,
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        &stopped.outcome,
        FamilyOutcome::Evaluated(Outcome::Undefined(quire_exact::Undefined::DivisionByZero))
    ));
    assert_eq!(stopped.location, healthy.location);
    assert_eq!(counters(&actual), expected);
    let mut actual = meter();
    let stopped = refused::oracle_bool_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        args(),
        objects(),
        &mut actual,
    )
    .unwrap()
    .unwrap();
    invariant(
        &stopped,
        CheckedInvariantCause::GeneratedDescriptorReconstructionFailed,
    );
    assert_eq!(stopped.location, healthy.location);
    assert_eq!(counters(&actual), expected);
    let mut actual = meter();
    let stopped = incomplete::oracle_bool_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        args(),
        objects(),
        &mut actual,
    )
    .unwrap()
    .unwrap();
    let FamilyOutcome::Evaluated(Outcome::Incomplete(record)) = &stopped.outcome else {
        panic!("unchanged incomplete arm");
    };
    assert_eq!(
        record,
        &quire_exact::Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 9,
            consumed: 9,
            next_charge: Integer::from(1_i64),
            charge_point: quire_exact::ChargePoint::BooleanResultRetain,
        }
    );
    assert_eq!(stopped.location, healthy.location);
    assert_eq!(counters(&actual), expected);
}

#[trace("TC-057", "FR-042-AC-3", "FR-021-AC-39")]
#[test]
fn authentic_owner_declarations_construct_once_and_duplicate_keeps_original_diagnostic() {
    let package = package();
    let acquired = acquired();
    let selected = selection(&acquired, &package, "eq");
    assert_eq!(selected.signature().result(), &ValueType::Boolean);
    let error = duplicate::inspect_environment(&package, &acquired, &selected, &Default::default())
        .unwrap_err();
    let EnvironmentFailure::Refused(invalid) = error else {
        panic!("genuine constructor refusal");
    };
    assert_eq!(invalid.cause, DeclarationCause::DuplicateKey);
    let originals =
        quire_contract_codegen::checked_function_declarations(&package, &acquired, &selected)
            .unwrap();
    let original_name = originals[0]
        .qualified_name()
        .iter()
        .map(quire_exact::Identifier::as_str)
        .collect::<Vec<_>>()
        .join(".");
    assert_eq!(invalid.declaration, original_name);
    let value = nominal_value();
    let dependencies = DependencyInput::default();
    let mut actual = meter();
    let rejection = duplicate::oracle_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        vec![value.clone(), value.clone()],
        objects(),
        &mut actual,
    )
    .unwrap()
    .unwrap();
    invariant(
        &rejection,
        CheckedInvariantCause::GeneratedEnvironmentRejected,
    );
    assert_eq!(
        rejection.location,
        Some(Location::root(Origin::TypeDeclaration {
            name: invalid.declaration
        }))
    );
    let root_charge = counters(&actual);
    let mut baseline = meter();
    let healthy = healthy::oracle_eq(
        source(&dependencies, Default::default()),
        &package,
        &package,
        vec![value.clone(), value],
        objects(),
        &mut baseline,
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        &healthy.outcome,
        FamilyOutcome::Evaluated(Outcome::Completed(Value::Boolean(true)))
    ));
    assert!(
        root_charge[8] < counters(&baseline)[8],
        "no equality operation charged after constructor rejection"
    );
    assert_eq!(root_charge[9], 0, "no retained body result");
}

fn environment_failure(
    result: Result<quire_eval::Evaluation, CallFailure>,
) -> (EnvironmentFailure, Option<CancelCause>) {
    let Err(CallFailure::Body(failure)) = result else {
        panic!("typed body failure: {result:?}");
    };
    let BodyFailure::QsvEnvironment {
        failure,
        cancellation,
    } = *failure
    else {
        panic!("not an internal fault");
    };
    (failure, cancellation)
}

#[trace("TC-057", "FR-042-AC-3", "FR-021-AC-39")]
#[test]
fn real_constructor_work_and_storage_stops_never_become_declaration_rejections() {
    let package = package();
    let dependencies = DependencyInput::default();
    let value = nominal_value();
    for cancelled in [false, true] {
        let mut resources = CheckedFunctionResources::default();
        resources.environment_limits = TypeEnvironmentLimits {
            work_units: 0,
            ..resources.environment_limits
        };
        if cancelled {
            resources.cancel.cancel(CancelCause::Requested);
        }
        let result = healthy::oracle_eq(
            source(&dependencies, resources),
            &package,
            &package,
            vec![value.clone(), value.clone()],
            objects(),
            &mut meter(),
        )
        .unwrap();
        let (failure, cancellation) = environment_failure(result);
        assert!(
            matches!(failure, EnvironmentFailure::Limit(limit) if limit.kind() == EnvironmentLimitKind::WorkUnits)
        );
        assert_eq!(cancellation, cancelled.then_some(CancelCause::Requested));
    }
    for capacity in [false, true] {
        let result = if capacity {
            constructor_capacity::oracle_eq(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![value.clone(), value.clone()],
                objects(),
                &mut meter(),
            )
            .unwrap()
        } else {
            constructor_allocation::oracle_eq(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![value.clone(), value.clone()],
                objects(),
                &mut meter(),
            )
            .unwrap()
        };
        let (failure, cancellation) = environment_failure(result);
        let request = match (capacity, failure) {
            (true, EnvironmentFailure::Capacity(request))
            | (false, EnvironmentFailure::Allocation(request)) => request,
            (_, other) => panic!("distinct constructor storage stop: {other:?}"),
        };
        assert!(request.amount() > 0, "actual QSV reservation request");
        assert_eq!(cancellation, None);
    }
}

#[trace("TC-057", "FR-042-AC-2", "FR-021-AC-40")]
#[test]
fn primitive_membership_has_no_storage_failure_execution_credit() {
    use std::sync::atomic::Ordering;
    let package = package();
    let dependencies = DependencyInput::default();
    for capacity in [false, true] {
        let mut actual = meter();
        let result = if capacity {
            membership_capacity::REQUESTS.store(0, Ordering::SeqCst);
            membership_capacity::oracle_bool_eq(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![Value::Boolean(true), Value::Boolean(true)],
                objects(),
                &mut actual,
            )
            .unwrap()
            .unwrap()
        } else {
            membership_allocation::REQUESTS.store(0, Ordering::SeqCst);
            membership_allocation::oracle_bool_eq(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![Value::Boolean(true), Value::Boolean(true)],
                objects(),
                &mut actual,
            )
            .unwrap()
            .unwrap()
        };
        assert!(matches!(
            &result.outcome,
            FamilyOutcome::Evaluated(Outcome::Completed(Value::Boolean(true)))
        ));
        let result = if capacity {
            membership_capacity::oracle_add(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![
                    Value::Integer(Integer::from(1_i64)),
                    Value::Integer(Integer::from(2_i64)),
                ],
                objects(),
                &mut actual,
            )
            .unwrap()
            .unwrap()
        } else {
            membership_allocation::oracle_add(
                source(&dependencies, Default::default()),
                &package,
                &package,
                vec![
                    Value::Integer(Integer::from(1_i64)),
                    Value::Integer(Integer::from(2_i64)),
                ],
                objects(),
                &mut actual,
            )
            .unwrap()
            .unwrap()
        };
        assert!(
            matches!(&result.outcome, FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(value))) if *value == Integer::from(3_i64))
        );
        let requests = if capacity {
            membership_capacity::REQUESTS.load(Ordering::SeqCst)
        } else {
            membership_allocation::REQUESTS.load(Ordering::SeqCst)
        };
        assert_eq!(
            requests, 0,
            "primitive membership makes no actual reservation: no storage-failure execution credit"
        );
        assert!(
            actual.consumed(LimitKind::WorkUnits) > 0,
            "real bodies executed"
        );
    }
}

#[trace("TC-057", "FR-042-AC-1")]
#[test]
fn native_argument_shape_guard_uses_its_exact_cause_without_a_body_charge() {
    let package = package();
    let acquired = acquired();
    let selected = selection(&acquired, &package, "bool_eq");
    let mut actual = meter();
    let quire_eval::Transition::Done(rejected) =
        healthy::invoke_native_body(&package, &acquired, &selected, vec![], &mut actual).unwrap()
    else {
        panic!("terminal body guard");
    };
    invariant(
        &rejected,
        CheckedInvariantCause::GeneratedArgumentShapeMismatch,
    );
    assert_eq!(counters(&actual), vec![0; 10]);
}

#[trace("TC-057", "FR-042-AC-1")]
#[test]
fn native_operand_kind_guard_uses_its_exact_cause_without_a_body_charge() {
    let package = package();
    let acquired = acquired();
    let selected = selection(&acquired, &package, "bool_eq");
    let mut actual = meter();
    let quire_eval::Transition::Done(rejected) = healthy::invoke_native_body(
        &package,
        &acquired,
        &selected,
        vec![Value::String("wrong kind".into()), Value::Boolean(true)],
        &mut actual,
    )
    .unwrap() else {
        panic!("terminal body guard");
    };
    invariant(
        &rejected,
        CheckedInvariantCause::GeneratedOperandKindUnsupported,
    );
    assert_eq!(counters(&actual), vec![0; 10]);
}
