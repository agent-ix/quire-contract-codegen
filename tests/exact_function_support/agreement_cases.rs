//! Executed FR-021 corpus: generated bodies and independent FR-485 session application.
//! The parent IT test writes the generated crate and the source literal used here.

mod generated {
    pub use quire_exact_function_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use qsl_replay::{
    acquire_checked_closure, ApplicationEnvironment, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_codegen::{
    checked_function_wire_id, checked_package_digest, CheckedFunctionError, CheckedFunctionSource,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use quire_eval::FamilyOutcome;
use quire_exact::{
    ChargePoint, InjectedDenial, Integer, LimitKind, Meter, Outcome, ScalarLimits, Value,
};
use quire_semantic_value::object_closure::ObjectClosure;
use std::num::NonZeroU64;

fn source_identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "ir595-agreement", "git", "1")
}

fn source<'a>(dependencies: &'a DependencyInput) -> CheckedFunctionSource<'a> {
    CheckedFunctionSource {
        source: source_identity(),
        path: "ir595-agreement.native",
        bytes: generated::SOURCE.as_bytes(),
        packages: &[],
        dependencies,
        limits: StageLimits::default(),
        replay_limits: ReplayLimits::default(),
    }
}

fn package() -> CheckedPackageV2 {
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        generated::SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the exact source used by the generator");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let read = CheckedPackageV2::read(
        acquired.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("Contract IR admits the same QSL bytes: {read:?}");
    };
    *package
}

fn unrelated_package() -> CheckedPackageV2 {
    const OTHER: &str = concat!(
        "language \"ix:native\" edition \"1-draft\";\n",
        "profile v = \"quire.value.complete/v1\";\n",
        "function unrelated using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x - y }\n",
    );
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-other.native",
        OTHER.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the unrelated source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let read = CheckedPackageV2::read(
        acquired.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("IR admits the unrelated QSL bytes: {read:?}");
    };
    *package
}

fn function(package: &CheckedPackageV2, name: &str) -> CheckedNodeId {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration
                .as_ref()
                .is_some_and(|declaration| declaration.qualified_name == [Box::<str>::from(name)])
        })
        .unwrap_or_else(|| panic!("checked function {name}"))
        .node_id
        .clone()
}

fn meter() -> Meter {
    let ceiling = u64::from(u32::MAX);
    Meter::new(ScalarLimits {
        integer_bits: ceiling,
        decimal_digits: ceiling,
        scale_expansion: ceiling,
        text_input_bytes: ceiling,
        text_scalars: ceiling,
        normalized_scalars: ceiling,
        unit_edges: ceiling,
        value_occurrences: ceiling,
        work_units: ceiling,
        result_units: ceiling,
    })
}

fn direct(
    name: &str,
    package: &CheckedPackageV2,
    arguments: Vec<Value>,
    meter: &mut Meter,
) -> quire_eval::Evaluation {
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        generated::SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("independent QSL acquisition");
    let handle = acquired.handle();
    let owner = checked_package_digest(package.package_id()).expect("admitted owner digest");
    let wire = checked_function_wire_id(&function(package, name)).expect("admitted wire key");
    let selection = handle
        .lookup_callable(owner, wire)
        .expect("QSL selects the function");
    let mut registry = handle.registry();
    let environment = ApplicationEnvironment::new(ObjectClosure::default());
    registry
        .begin_call(&selection, arguments, &environment, meter)
        .expect("direct QSL Begin")
        .run()
        .expect("interpreted body completes")
}

fn generated(
    name: &str,
    package: &CheckedPackageV2,
    arguments: Vec<Value>,
    meter: &mut Meter,
) -> quire_eval::Evaluation {
    let dependencies = DependencyInput::default();
    let source = source(&dependencies);
    let result = match name {
        "add" => generated::oracle_add(
            source,
            package,
            package,
            arguments,
            ObjectClosure::default(),
            meter,
        ),
        "eq" => generated::oracle_eq(
            source,
            package,
            package,
            arguments,
            ObjectClosure::default(),
            meter,
        ),
        "nested" => generated::oracle_nested(
            source,
            package,
            package,
            package,
            arguments,
            ObjectClosure::default(),
            meter,
        ),
        _ => panic!("unsupported corpus function"),
    };
    result
        .expect("checked owner, lookup, registration and Begin")
        .expect("generated body completes")
}

/// Trace: FR-021-AC-2, FR-021-AC-32, FR-021-AC-35, TC-031.
#[test]
fn generated_and_independent_fr485_session_match_in_outcome_and_charges() {
    let package = package();
    for (name, left, right) in [
        ("add", 3_u64, 4_u64),
        ("eq", 3_u64, 4_u64),
        ("nested", 3_u64, 4_u64),
    ] {
        let arguments = vec![
            Value::Integer(Integer::from(left)),
            Value::Integer(Integer::from(right)),
        ];
        let mut generated_meter = meter();
        let actual = generated(name, &package, arguments.clone(), &mut generated_meter);
        let mut direct_meter = meter();
        let expected = direct(name, &package, arguments, &mut direct_meter);
        assert_eq!(
            format!("{:?}", actual.outcome),
            format!("{:?}", expected.outcome),
            "{name}"
        );
        assert_eq!(actual.location, expected.location, "{name} location");
        assert_eq!(actual.losses, expected.losses, "{name} losses");
        assert_eq!(
            generated_meter.admitted_charges(),
            direct_meter.admitted_charges(),
            "{name} charge order"
        );
        for kind in LimitKind::ALL {
            assert_eq!(
                generated_meter.consumed(kind),
                direct_meter.consumed(kind),
                "{name} {kind:?}"
            );
        }
        assert!(matches!(
            actual.outcome,
            FamilyOutcome::Evaluated(Outcome::Completed(_))
        ));
    }
}

fn direct_begin_refusal(
    package: &CheckedPackageV2,
    arguments: Vec<Value>,
    meter: &mut Meter,
) -> qsl_replay::ApplicationBeginRefusal {
    let acquired = acquire_checked_closure(
        source_identity(),
        "ir595-agreement.native",
        generated::SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("independent QSL acquisition");
    let handle = acquired.handle();
    let owner = checked_package_digest(package.package_id()).expect("admitted owner digest");
    let wire = checked_function_wire_id(&function(package, "add")).expect("admitted wire key");
    let selection = handle
        .lookup_callable(owner, wire)
        .expect("QSL selects add");
    let mut registry = handle.registry();
    let environment = ApplicationEnvironment::new(ObjectClosure::default());
    match registry.begin_call(&selection, arguments, &environment, meter) {
        Ok(_) => panic!("malformed input unexpectedly began"),
        Err(refusal) => refusal,
    }
}

/// Trace: FR-021-AC-4, FR-021-AC-5, TC-031.
#[test]
fn qsl_input_refusals_precede_application_charges_and_arity_precedes_kind() {
    let package = package();
    for arguments in [
        vec![Value::Integer(Integer::from(1_u64))],
        vec![Value::Boolean(true), Value::Integer(Integer::from(1_u64))],
        vec![Value::Boolean(true)],
    ] {
        let dependencies = DependencyInput::default();
        let mut generated_meter = meter();
        let actual = generated::oracle_add(
            source(&dependencies),
            &package,
            &package,
            arguments.clone(),
            ObjectClosure::default(),
            &mut generated_meter,
        )
        .expect_err("generated application refuses invalid input");
        let CheckedFunctionError::ApplicationBeginRefused(actual) = actual else {
            panic!("expected Begin input refusal, got {actual:?}");
        };
        let mut direct_meter = meter();
        let expected = direct_begin_refusal(&package, arguments, &mut direct_meter);
        assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
        assert_eq!(generated_meter.admission_count(), 0);
        assert_eq!(direct_meter.admission_count(), 0);
        for kind in LimitKind::ALL {
            assert_eq!(generated_meter.consumed(kind), 0, "{kind:?}");
            assert_eq!(direct_meter.consumed(kind), 0, "{kind:?}");
        }
    }
}

/// Trace: FR-021-AC-9, TC-031.
#[test]
fn function_call_denial_is_incomplete_without_an_admitted_charge() {
    let package = package();
    let denial = InjectedDenial {
        point: ChargePoint::FunctionCall,
        occurrence: NonZeroU64::new(1).expect("nonzero occurrence"),
    };
    let arguments = vec![
        Value::Integer(Integer::from(3_u64)),
        Value::Integer(Integer::from(4_u64)),
    ];
    let dependencies = DependencyInput::default();
    let mut generated_meter = meter().with_injected_denial(denial);
    let actual = generated::oracle_add(
        source(&dependencies),
        &package,
        &package,
        arguments.clone(),
        ObjectClosure::default(),
        &mut generated_meter,
    )
    .expect("checked application")
    .expect("terminal incomplete");
    let mut direct_meter = meter().with_injected_denial(denial);
    let expected = direct("add", &package, arguments, &mut direct_meter);
    assert_eq!(
        format!("{:?}", actual.outcome),
        format!("{:?}", expected.outcome)
    );
    assert!(matches!(
        actual.outcome,
        FamilyOutcome::Evaluated(Outcome::Incomplete(ref incomplete))
            if incomplete.charge_point == ChargePoint::FunctionCall
    ));
    assert_eq!(generated_meter.admission_count(), 0);
    assert_eq!(direct_meter.admission_count(), 0);
}

/// Trace: FR-021-AC-36, FR-021-AC-37, TC-031.
#[test]
fn wrong_unit_or_owner_refuses_before_registration_and_charge() {
    let admitted = package();
    let unrelated = unrelated_package();
    let arguments = vec![
        Value::Integer(Integer::from(3_u64)),
        Value::Integer(Integer::from(4_u64)),
    ];
    for (unit, owner) in [(&unrelated, &admitted), (&admitted, &unrelated)] {
        let dependencies = DependencyInput::default();
        let mut meter = meter();
        let failure = generated::oracle_add(
            source(&dependencies),
            unit,
            owner,
            arguments.clone(),
            ObjectClosure::default(),
            &mut meter,
        )
        .expect_err("mismatched admitted identity must refuse");
        assert!(matches!(
            failure,
            CheckedFunctionError::OwnerIdentityMismatch
        ));
        assert_eq!(meter.admission_count(), 0);
        for kind in LimitKind::ALL {
            assert_eq!(meter.consumed(kind), 0, "{kind:?}");
        }
    }
    let mut healthy_meter = meter();
    let healthy = generated("add", &admitted, arguments, &mut healthy_meter);
    assert!(matches!(
        healthy.outcome,
        FamilyOutcome::Evaluated(Outcome::Completed(_))
    ));
    assert!(healthy_meter.admission_count() > 0);
}
