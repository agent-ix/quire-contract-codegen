// SPDX-License-Identifier: AGPL-3.0-or-later
//! Generated composite oracle versus independent QSV, with genuine source identities.
#[path = "fixture.rs"]
mod fixture;
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));

    #[ix_trace_rs::trace("FR-018-AC-8", "FR-018-AC-39", "FR-018-AC-42", "TC-029")]
    #[test]
    fn real_lookup_refusals_and_retained_metadata_remain_distinct() {
        let (compiled, libraries) = super::compilation();
        let prepared = environment_holder_same(Some(compiled.context())).expect("bound original package");
        let lookup = |wire: qsl_replay::WireNodeId, name: &[&str], form| lookup_composite_key(prepared.context, &wire.to_string(), name, form);
        let unknown = qsl_replay::WireNodeId::from_digest([7;32]);
        assert!(matches!(lookup(unknown, &["Absent"], qsl_replay::AdmittedCompositeForm::Record), Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Unknown { declaration, case: None })) if declaration == unknown));
        let marker = compiled.context().enums().next().expect("actual own enum");
        let wrong = qsl_replay::WireNodeId::from_digest(*marker.key().as_bytes());
        assert!(matches!(lookup(wrong, &["Marker"], qsl_replay::AdmittedCompositeForm::Record), Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::WrongKind { declaration })) if declaration == wrong));
        let leaf = &libraries.iter().find(|(name,_)| name == "test/composites").expect("real leaf").1;
        for declaration in leaf.context().composites() {
            let wire = qsl_replay::WireNodeId::from_digest(*declaration.node().as_bytes());
            let names = declaration.qualified_name().iter().map(|part| part.as_str()).collect::<Vec<_>>();
            assert!(matches!(lookup(wire, &names, declaration.form()), Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Foreign { declaration: actual, package })) if actual == wire && package == leaf.context().package_id()));
        }
        let pair = super::declaration(compiled.context(), "Pair");
        let wire = qsl_replay::WireNodeId::from_digest(*pair.node().as_bytes());
        assert_ne!(pair.node(), pair.key());
        assert_eq!(lookup(wire, &["Pair"], pair.form()).expect("real wire resolves kernel key"), pair.key());
        let kernel_as_wire = qsl_replay::WireNodeId::from_digest(*pair.key().as_bytes());
        assert!(matches!(lookup(kernel_as_wire, &["Pair"], pair.form()), Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Unknown { declaration, case: None })) if declaration == kernel_as_wire));
        assert!(matches!(lookup(wire, &["WrongName"], pair.form()), Err(EnvironmentError::Reconstruction(ReconstructionError::Composite))));
        assert!(matches!(lookup(wire, &["Pair"], qsl_replay::AdmittedCompositeForm::Record), Err(EnvironmentError::Reconstruction(ReconstructionError::Composite))));
        assert!(matches!(lookup_composite_key(None, &wire.to_string(), &["Pair"], pair.form()), Err(EnvironmentError::MissingAdmittedContext { declaration }) if declaration == wire));
    }

    #[ix_trace_rs::trace("FR-018-AC-8", "FR-018-AC-39", "FR-018-AC-42", "TC-029")]
    #[test]
    fn absent_wrong_or_tampered_capability_never_charges() {
        let (compiled,_) = super::compilation();
        assert!(matches!(environment_holder_same(None), Err(EnvironmentError::MissingAdmittedContext { .. })));
        let other = super::fixture::compile(super::fixture::SOURCE, &qsl_replay::DependencyInput::default());
        assert!(matches!(environment_holder_same(Some(other.context())), Err(EnvironmentError::ContextPackageMismatch { .. })));
        let (environment,_) = super::environment(compiled.context());
        let value = super::values(&environment, compiled.context(), "Holder").remove(0);
        for fault in ["absent", "wrong", "item", "package", "recipe"] {
            let mut prepared = environment_holder_same(Some(compiled.context())).expect("real preparation");
            match fault {
                "absent" => prepared.context = None,
                "wrong" => prepared.context = Some(other.context()),
                "item" => prepared.binding = "other-item",
                "package" => prepared.expected_digest = "other-package",
                "recipe" => prepared.requires_context = false,
                _ => panic!("finite private mutation"),
            }
            let actual = super::metered(|meter| oracle_holder_same(&prepared, &value, &value, meter));
            assert_eq!(actual.0, exact::Outcome::Refused(exact::Refusal::CheckedInvariant { cause: exact::CheckedInvariantCause::GeneratedDescriptorReconstructionFailed }), "{fault}");
            assert!(actual.1.is_empty(), "{fault}");
            assert!(actual.2.iter().all(|count| *count == 0), "{fault}");
        }
    }
}
use ix_trace_rs::trace;
use qsl_replay::{AdmittedCompilation, AdmittedComposite, AdmittedCompositeForm, AdmittedContext};
use quire_exact::{ChargePoint, FieldValue, InjectedDenial, LimitKind, Meter, Outcome, Presence, ScalarLimits, Value, ValueType};
use quire_semantic_value::declaration::{CompositeDeclaration, CompositeShape, EqualityOperand, EqualityOperator, FieldDeclaration, TypeEnvironment};

type Constructor = for<'ctx> fn(Option<&'ctx AdmittedContext>) -> Result<generated::PreparedEqualityEnvironment<'ctx>, generated::EnvironmentError>;
type Oracle = fn(&generated::PreparedEqualityEnvironment<'_>, &Value, &Value, &mut Meter) -> Outcome<bool>;
const LIMITS: ScalarLimits = ScalarLimits { integer_bits: u64::MAX, decimal_digits: u64::MAX, scale_expansion: u64::MAX, text_input_bytes: u64::MAX, text_scalars: u64::MAX, normalized_scalars: u64::MAX, unit_edges: u64::MAX, value_occurrences: u64::MAX, work_units: u64::MAX, result_units: u64::MAX };

fn compilation() -> (AdmittedCompilation, Vec<(String, AdmittedCompilation)>) {
    let mode = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/context-mode")).expect("retained source choice");
    match mode.as_str() { "direct" => fixture::imported(false), "transitive" => fixture::imported(true), other => panic!("unknown source choice {other}") }
}
fn declaration<'a>(context: &'a AdmittedContext, name: &str) -> &'a AdmittedComposite {
    context.composites().find(|item| item.qualified_name().iter().map(|part| part.as_str()).eq([name])).expect("actual declaration")
}
fn environment(context: &AdmittedContext) -> (TypeEnvironment, quire_semantic_value::enumeration::EnumMemberIndex) {
    let pair = declaration(context,"Pair");
    let declarations = ["Pair", "OtherPair", "Holder", "OtherHolder"].into_iter().map(|name| {
        let actual = declaration(context,name);
        let shape = match actual.form() {
            AdmittedCompositeForm::Union => panic!("the independent record/tuple fixture cannot contain a Union"),
            AdmittedCompositeForm::Tuple => CompositeShape::Tuple(vec![ValueType::Boolean, ValueType::Boolean]),
            AdmittedCompositeForm::Record => CompositeShape::Record(vec![
                FieldDeclaration::new("flag", ValueType::Boolean, Presence::Required),
                FieldDeclaration::new("pair", ValueType::Composite(pair.key()), Presence::Required),
            ]),
        };
        CompositeDeclaration::new(actual.key(), name, shape)
    });
    (TypeEnvironment::new(declarations, []).expect("independent real closed environment"), Default::default())
}
fn values(environment: &TypeEnvironment, context: &AdmittedContext, name: &str) -> Vec<Value> {
    let actual = declaration(context,name);
    let pairs = [false,true].into_iter().flat_map(|a| [false,true].into_iter().map(move |b| (a,b))).collect::<Vec<_>>();
    match actual.form() {
        AdmittedCompositeForm::Union => panic!("the independent record/tuple value fixture cannot contain a Union"),
        AdmittedCompositeForm::Tuple => pairs.into_iter().map(|(a,b)| environment.tuple(actual.key(), vec![Value::Boolean(a),Value::Boolean(b)]).expect("genuine tuple value")).collect(),
        AdmittedCompositeForm::Record => pairs.into_iter().flat_map(|(a,b)| [false,true].into_iter().map(move |flag| (a,b,flag))).map(|(a,b,flag)| {
            let pair = environment.tuple(declaration(context,"Pair").key(), vec![Value::Boolean(a),Value::Boolean(b)]).expect("nested tuple");
            environment.record(actual.key(), vec![("flag", FieldValue::Present(Value::Boolean(flag))), ("pair", FieldValue::Present(pair))]).expect("genuine record value")
        }).collect(),
    }
}
fn metered(run: impl FnOnce(&mut Meter) -> Outcome<bool>) -> (Outcome<bool>,Vec<ChargePoint>,Vec<u64>) {
    let mut meter = Meter::new(LIMITS);
    let outcome = run(&mut meter);
    (outcome, meter.admitted_charges().iter().map(|charge| charge.point).collect(), LimitKind::ALL.iter().map(|kind| meter.consumed(*kind)).collect())
}
fn denials(run: impl Fn(&mut Meter) -> Outcome<bool>) -> Vec<(ChargePoint,u64,Outcome<bool>,Vec<u64>)> {
    let healthy = metered(&run);
    assert!(!healthy.1.is_empty(), "actual composite evaluation charges");
    let mut seen = Vec::new();
    healthy.1.into_iter().map(|point| {
        seen.push(point);
        let occurrence = u64::try_from(seen.iter().filter(|prior| **prior == point).count()).expect("finite charge count");
        let mut meter = Meter::new(LIMITS).with_injected_denial(InjectedDenial { point, occurrence: core::num::NonZeroU64::new(occurrence).expect("nonzero occurrence") });
        let outcome = run(&mut meter);
        assert!(matches!(outcome, Outcome::Incomplete(_)), "denied charge stops without Boolean");
        (point,occurrence,outcome,LimitKind::ALL.iter().map(|kind| meter.consumed(*kind)).collect())
    }).collect()
}

#[trace("FR-018-AC-2", "FR-018-AC-8", "FR-018-AC-9", "TC-029")]
#[test]
fn genuine_nested_composites_match_outcomes_charges_counters_and_each_denial() {
    let (compiled,_) = compilation();
    let (environment,index) = environment(compiled.context());
    let cases: [(&str,Constructor,Oracle,EqualityOperator);6] = [
        ("Pair",generated::environment_pair_same,generated::oracle_pair_same,EqualityOperator::Equal),
        ("Pair",generated::environment_pair_different,generated::oracle_pair_different,EqualityOperator::NotEqual),
        ("Holder",generated::environment_holder_same,generated::oracle_holder_same,EqualityOperator::Equal),
        ("Holder",generated::environment_holder_different,generated::oracle_holder_different,EqualityOperator::NotEqual),
        ("OtherPair",generated::environment_other_pair_same,generated::oracle_other_pair_same,EqualityOperator::Equal),
        ("OtherHolder",generated::environment_other_holder_same,generated::oracle_other_holder_same,EqualityOperator::Equal),
    ];
    for (name,constructor,oracle,operator) in cases {
        let prepared = constructor(Some(compiled.context())).expect("actual emitted preparation");
        let shape = ValueType::Composite(declaration(compiled.context(),name).key());
        environment.check_type(&shape).expect("independent type check");
        let wire_node = declaration(compiled.context(),name).node();
        assert_ne!(wire_node, declaration(compiled.context(),name).key());
        let wrong_identity = if name.ends_with("Pair") {
            environment.tuple(wire_node, vec![Value::Boolean(false), Value::Boolean(false)])
        } else {
            environment.record(wire_node, Vec::new())
        }.expect_err("a genuine wire node is not a kernel declaration key");
        assert_eq!(wrong_identity.cause, quire_semantic_value::declaration::ConstructionCause::UnknownDeclaration);
        let checked = environment.check_equality(operator, EqualityOperand::typed(shape.clone()), EqualityOperand::typed(shape), &index).expect("independent equality check");
        for left in values(&environment,compiled.context(),name) {
            for right in values(&environment,compiled.context(),name) {
                let direct = |meter: &mut Meter| checked.evaluate(&environment,&left,&right,meter);
                let generated = |meter: &mut Meter| oracle(&prepared,&left,&right,meter);
                assert_eq!(metered(&generated),metered(&direct),"{name}: {left:?}/{right:?}");
                assert_eq!(denials(&generated),denials(&direct),"{name}: {left:?}/{right:?}");
            }
        }
        let foreign_name = if name.ends_with("Pair") { if name == "Pair" { "OtherPair" } else { "Pair" } } else if name == "Holder" { "OtherHolder" } else { "Holder" };
        let left = values(&environment,compiled.context(),name).remove(0);
        let wrong = values(&environment,compiled.context(),foreign_name).remove(0);
        let actual = metered(|meter| oracle(&prepared,&left,&wrong,meter));
        let direct = metered(|meter| checked.evaluate(&environment,&left,&wrong,meter));
        assert_eq!(actual,direct,"same shape is not nominal identity");
        assert!(matches!(actual.0,Outcome::Refused(_)));
        assert!(actual.1.is_empty());
        assert!(actual.2.iter().all(|count| *count == 0));
    }
}
