//! Proposed genuine-source regression. No source conversion node is claimed.
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));

    #[ix_trace_rs::trace("FR-018-AC-43", "TC-029")]
    #[test]
    fn prepared_numeric_descriptors_preserve_exact_source_domains() {
        let compiled = super::compile_source();
        let prepared = environment_numeric(None).expect("actual context-free package preparation");
        assert!(prepared.context.is_none());
        assert!(!prepared.requires_context);
        assert_eq!(prepared.left_source, super::bounded_integer());
        assert_eq!(prepared.left_target, Some(exact::ValueType::Integer));
        assert_eq!(prepared.right_source, exact::ValueType::Integer);
        assert_eq!(prepared.right_target, None);
    }
}
use ix_trace_rs::trace;
use quire_exact::{
    ChargePoint, Integer, IntegerInterval, LimitKind, Meter, Outcome, ScalarLimits, Value,
    ValueType,
};
use quire_semantic_value::declaration::{EqualityOperand, EqualityOperator, TypeEnvironment};

fn compile_source() -> qsl_replay::AdmittedCompilation {
    let source = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction f using v(a: Int[0, 9],b: Integer): Boolean pure { convert<Integer>(a) = b }\n";
    qsl_replay::compile_admitted(
        qsl_replay::SourceIdentity::new("test", "admitted-enums.native", "git", "fixture"),
        "admitted-enums.native",
        source.as_bytes(),
        [],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("independently compile exactly the retained original source")
}
fn bounded_integer() -> ValueType {
    ValueType::Int(
        IntegerInterval::new(Integer::from(0_i64), Integer::from(9_i64))
            .expect("valid source interval"),
    )
}
const LIMITS: ScalarLimits = ScalarLimits {
    integer_bits: 1000,
    decimal_digits: 1000,
    scale_expansion: 1000,
    text_input_bytes: 1000,
    text_scalars: 1000,
    normalized_scalars: 1000,
    unit_edges: 1000,
    value_occurrences: 1000,
    work_units: 1000,
    result_units: 1000,
};
fn observed(
    run: impl FnOnce(&mut Meter) -> Outcome<bool>,
) -> (Outcome<bool>, Vec<ChargePoint>, Vec<u64>) {
    let mut meter = Meter::new(LIMITS);
    let outcome = run(&mut meter);
    (
        outcome,
        meter
            .admitted_charges()
            .iter()
            .map(|charge| charge.point)
            .collect(),
        LimitKind::ALL
            .iter()
            .map(|kind| meter.consumed(*kind))
            .collect(),
    )
}

#[trace("FR-018-AC-43", "TC-029")]
#[test]
fn unbounded_right_values_outside_sibling_interval_still_complete() {
    let compiled = compile_source();
    let prepared =
        generated::environment_numeric(Some(compiled.context())).expect("same original context");
    let no_context = generated::environment_numeric(None).expect("context-free preparation");
    let environment = TypeEnvironment::default();
    let checked = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::converted(bounded_integer(), ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
            &quire_semantic_value::enumeration::EnumMemberIndex::default(),
        )
        .expect("QSpec admits Int[0,9] to unbounded Integer");
    for left in [0_i64, 9] {
        for right in [-100_i64, -1, 0, 9, 10, 42, 100] {
            let l = Value::Integer(Integer::from(left));
            let r = Value::Integer(Integer::from(right));
            let direct = observed(|meter| checked.evaluate(&environment, &l, &r, meter));
            assert_eq!(direct.0, Outcome::Completed(left == right));
            assert!(
                !direct.1.is_empty(),
                "actual successful integer equality must charge"
            );
            let actual = observed(|meter| generated::oracle_numeric(&prepared, &l, &r, meter));
            assert_eq!(
                actual, direct,
                "right source Integer is not Int[0,9]: ({left},{right})"
            );
            assert_eq!(
                observed(|meter| generated::oracle_numeric(&no_context, &l, &r, meter)),
                direct
            );
        }
    }
    for left in [-1_i64, 10] {
        let l = Value::Integer(Integer::from(left));
        let r = Value::Integer(Integer::from(42_i64));
        let direct = observed(|meter| checked.evaluate(&environment, &l, &r, meter));
        assert_eq!(
            direct.0,
            Outcome::Refused(quire_exact::Refusal::CheckedInvariant {
                cause: quire_exact::CheckedInvariantCause::EqualityOperandSourceNotAdmitted,
            }),
            "left source bound is preserved"
        );
        assert!(direct.1.is_empty());
        let actual = observed(|meter| generated::oracle_numeric(&prepared, &l, &r, meter));
        assert_eq!(actual, direct, "fix must not widen left Int source");
    }
}

#[trace("FR-018-AC-40", "FR-018-AC-42", "TC-029")]
#[test]
fn context_free_constructor_rejects_explicit_wrong_context() {
    let other = qsl_replay::compile_admitted(
        qsl_replay::SourceIdentity::new("test", "other.native", "git", "fixture"),
        "other.native", b"language \"ix:native\" edition \"1-draft\"; profile v = \"quire.value.complete/v1\"; function other using v(): Boolean pure { true }", [],
        &qsl_replay::DependencyInput::default(), qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    ).expect("another actual source package");
    assert!(matches!(
        generated::environment_numeric(Some(other.context())),
        Err(generated::EnvironmentError::ContextPackageMismatch { .. })
    ));
}
