//! Actual generated enum oracles versus direct QSV, both using real admitted identities.
#[path = "fixture.rs"]
mod fixture;
#[path = "native_values.rs"]
mod native_values;
mod generated {
    // Include the exact emitted SUT, preserving its private prepared fields for private controls.
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));

    #[ix_trace_rs::trace("FR-018-AC-35", "TC-029")]
    #[test]
    fn emitted_lookup_retains_direct_or_transitive_original_foreign_package() {
        let (compiled, libraries) = super::native_values::native_compilation();
        let prepared = environment_ordered_same(Some(compiled.context()))
            .expect("same original package bound first");
        let shades = libraries
            .iter()
            .find(|(identity, _)| identity == "test/shades")
            .expect("original shade package");
        let shade = shades
            .1
            .context()
            .enums()
            .next()
            .expect("genuine foreign enum");
        let wire = qsl_replay::WireNodeId::from_digest(*shade.key().as_bytes());
        let expected = shades.1.context().package_id();
        let mut members = prepared.enum_members.clone();
        let result = rebuild_enum(
            prepared.context,
            &mut members,
            &wire.to_string(),
            &["Shade"],
            false,
        );
        assert!(
            matches!(result, Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Foreign {
            declaration, package,
        })) if declaration == wire && package == expected)
        );
        let result = lookup_enum_case(
            prepared.context,
            wire,
            &exact::Identifier::new("DARK").expect("identifier"),
        );
        assert!(
            matches!(result, Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Foreign {
            declaration, package,
        })) if declaration == wire && package == expected)
        );
    }

    /// Private preparation components retain the real lookup fields, without forging a context.
    #[ix_trace_rs::trace("FR-018-AC-35", "TC-029")]
    #[test]
    fn emitted_lookup_components_preserve_unknown_declaration_and_case() {
        let (compiled, _) = super::native_values::native_compilation();
        let prepared =
            environment_ordered_same(Some(compiled.context())).expect("genuine prepared");
        let unknown = qsl_replay::WireNodeId::from_digest([7; 32]);
        assert!(
            matches!(prepared.context.expect("genuine retained capability").enum_declaration(unknown),
            Err(qsl_replay::AdmittedLookupRefusal::Unknown { declaration, case: None }) if declaration == unknown)
        );
        let mut members = prepared.enum_members.clone();
        let result = rebuild_enum(
            prepared.context,
            &mut members,
            &unknown.to_string(),
            &["Ordered"],
            true,
        );
        assert!(
            matches!(result, Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Unknown {
            declaration, case: None,
        })) if declaration == unknown)
        );
        let ordered = prepared
            .context
            .expect("genuine retained capability")
            .enums()
            .find(|item| {
                item.qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq(["Ordered"])
            })
            .expect("real ordered enum");
        let wire = qsl_replay::WireNodeId::from_digest(*ordered.key().as_bytes());
        let absent = exact::Identifier::new("ABSENT").expect("identifier");
        let result = lookup_enum_case(prepared.context, wire, &absent);
        assert!(
            matches!(result, Err(EnvironmentError::Lookup(qsl_replay::AdmittedLookupRefusal::Unknown {
            declaration, case: Some(case),
        })) if declaration == wire && case == absent)
        );
    }

    #[ix_trace_rs::trace("FR-018-AC-39", "FR-018-AC-42", "TC-029")]
    #[test]
    fn nominal_absence_and_private_binding_faults_never_charge() {
        let (compiled, _) = super::native_values::native_compilation();
        let ordered = compiled
            .context()
            .enums()
            .find(|item| {
                item.qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq(["Ordered"])
            })
            .expect("actual nominal declaration");
        let expected = qsl_replay::WireNodeId::from_digest(*ordered.key().as_bytes());
        assert!(matches!(environment_ordered_same(None),
            Err(EnvironmentError::MissingAdmittedContext { declaration }) if declaration == expected));
        let values = super::native_values::enum_data(compiled.context(), "Ordered");
        let other = super::fixture::compile(super::fixture::SOURCE);
        assert_ne!(
            other.context().package_id(),
            compiled.context().package_id()
        );
        for fault in ["absent", "wrong-present", "item", "package", "recipe"] {
            let mut prepared =
                environment_ordered_same(Some(compiled.context())).expect("genuine preparation");
            match fault {
                "absent" => prepared.context = None,
                "wrong-present" => prepared.context = Some(other.context()),
                "item" => prepared.binding = "wrong-item",
                "package" => prepared.expected_digest = "wrong-package",
                "recipe" => prepared.requires_context = false,
                _ => panic!("finite test fixture"),
            }
            let result = super::metered(|meter| {
                oracle_ordered_same(&prepared, &values.values[0], &values.values[0], meter)
            });
            assert_eq!(
                result.0,
                quire_exact::Outcome::Refused(quire_exact::Refusal::CheckedInvariant {
                    cause:
                        quire_exact::CheckedInvariantCause::GeneratedDescriptorReconstructionFailed,
                }),
                "{fault}"
            );
            assert!(result.1.is_empty(), "{fault} must stop before charging");
            assert!(result.2.iter().all(|used| *used == 0));
        }
    }
}
use ix_trace_rs::trace;
use quire_exact::{
    ChargePoint, CheckedInvariantCause, InjectedDenial, LimitKind, Meter, Outcome, Refusal,
    ScalarLimits,
};
use quire_semantic_value::declaration::{EqualityOperand, EqualityOperator, TypeEnvironment};

type Constructor = for<'ctx> fn(
    Option<&'ctx qsl_replay::AdmittedContext>,
) -> Result<
    generated::PreparedEqualityEnvironment<'ctx>,
    generated::EnvironmentError,
>;
type Oracle = fn(
    &generated::PreparedEqualityEnvironment<'_>,
    &quire_exact::Value,
    &quire_exact::Value,
    &mut Meter,
) -> Outcome<bool>;

const LIMITS: ScalarLimits = ScalarLimits {
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

fn metered(
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

fn denials(
    run: impl Fn(&mut Meter) -> Outcome<bool>,
) -> Vec<(ChargePoint, u64, Outcome<bool>, Vec<u64>)> {
    let completed = metered(&run);
    assert!(!completed.1.is_empty(), "real enum evaluation must charge");
    let mut seen = Vec::new();
    completed
        .1
        .into_iter()
        .map(|point| {
            seen.push(point);
            let occurrence = u64::try_from(seen.iter().filter(|prior| **prior == point).count())
                .expect("fixture occurrence fits");
            let mut meter = Meter::new(LIMITS).with_injected_denial(InjectedDenial {
                point,
                occurrence: core::num::NonZeroU64::new(occurrence)
                    .expect("count includes this charge"),
            });
            let outcome = run(&mut meter);
            assert!(
                matches!(outcome, Outcome::Incomplete(_)),
                "a denied charge cannot complete a Boolean"
            );
            let counters = LimitKind::ALL
                .iter()
                .map(|kind| meter.consumed(*kind))
                .collect();
            (point, occurrence, outcome, counters)
        })
        .collect()
}

#[trace("FR-018-AC-34", "FR-018-AC-36", "TC-029")]
#[test]
fn complete_genuine_enum_pairs_match_outcomes_ordered_charges_counters_and_every_denial() {
    let (compiled, _) = native_values::native_compilation();
    assert!(!fixture::SOURCE.contains("::"));
    let cases: [(&str, Constructor, Oracle, EqualityOperator); 4] = [
        (
            "Ordered",
            generated::environment_ordered_same,
            generated::oracle_ordered_same,
            EqualityOperator::Equal,
        ),
        (
            "Ordered",
            generated::environment_ordered_different,
            generated::oracle_ordered_different,
            EqualityOperator::NotEqual,
        ),
        (
            "Unordered",
            generated::environment_unordered_same,
            generated::oracle_unordered_same,
            EqualityOperator::Equal,
        ),
        (
            "Other",
            generated::environment_other_same,
            generated::oracle_other_same,
            EqualityOperator::Equal,
        ),
    ];
    for (name, construct, oracle, operator) in cases {
        let data = native_values::enum_data(compiled.context(), name);
        assert!(data.names.iter().any(|case| case == "UNUSED"));
        let expected_names = if name == "Unordered" {
            ["DONE", "READY", "UNUSED"]
        } else {
            ["READY", "DONE", "UNUSED"]
        };
        assert_eq!(data.names, expected_names);
        let prepared = construct(Some(compiled.context())).expect("real prepared environment");
        let environment = TypeEnvironment::default();
        let checked = environment
            .check_equality(
                operator,
                EqualityOperand::typed(data.shape.clone()),
                EqualityOperand::typed(data.shape.clone()),
                &data.index,
            )
            .expect("real owner equality");
        for left in &data.values {
            for right in &data.values {
                let direct = |meter: &mut Meter| checked.evaluate(&environment, left, right, meter);
                let actual = |meter: &mut Meter| oracle(&prepared, left, right, meter);
                let observed = metered(actual);
                assert!(matches!(observed.0, Outcome::Completed(_)));
                assert!(!observed.1.is_empty());
                assert_eq!(observed, metered(direct));
                assert_eq!(denials(actual), denials(direct));
            }
        }
    }
}

#[trace("FR-018-AC-34", "FR-018-AC-36", "TC-029")]
#[test]
fn same_case_of_distinct_genuine_declarations_is_not_an_alias() {
    let (compiled, _) = native_values::native_compilation();
    let ordered = native_values::enum_data(compiled.context(), "Ordered");
    let other = native_values::enum_data(compiled.context(), "Other");
    assert_eq!(ordered.names, other.names);
    for (left, right) in ordered.members.iter().zip(&other.members) {
        assert_ne!(left.variant(), right.variant());
        let mut meter = Meter::new(LIMITS);
        let refused = quire_semantic_value::enumeration::compare_enum(
            quire_exact::ComparisonOperator::Equal,
            left,
            right,
            &mut meter,
        );
        assert!(matches!(
            refused,
            Err(quire_exact::IllTyped {
                cause: quire_exact::IllTypedCause::DistinctEnumDeclarations
            })
        ));
        assert!(meter.admitted_charges().is_empty());
    }
    let prepared =
        generated::environment_ordered_same(Some(compiled.context())).expect("ordered prepared");
    let result = metered(|meter| {
        generated::oracle_ordered_same(&prepared, &ordered.values[0], &other.values[0], meter)
    });
    assert_eq!(
        result.0,
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::EqualityOperandSourceNotAdmitted
        })
    );
    assert!(result.1.is_empty());
}

#[trace("FR-018-AC-33", "FR-018-AC-36", "TC-029")]
#[test]
fn wrong_original_package_and_wrong_prepared_item_stop_before_lookup_or_charge() {
    let (compiled, _) = native_values::native_compilation();
    let changed = fixture::compile(&format!("{}enum Extra {{ X, Y }}\n", fixture::SOURCE));
    assert_ne!(
        compiled.context().package_id(),
        changed.context().package_id()
    );
    assert!(matches!(
        generated::environment_ordered_same(Some(changed.context())),
        Err(generated::EnvironmentError::ContextPackageMismatch { .. })
    ));
    let prepared =
        generated::environment_ordered_same(Some(compiled.context())).expect("real prepared");
    let data = native_values::enum_data(compiled.context(), "Unordered");
    let result = metered(|meter| {
        generated::oracle_unordered_same(&prepared, &data.values[0], &data.values[0], meter)
    });
    assert_eq!(
        result.0,
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::GeneratedDescriptorReconstructionFailed
        })
    );
    assert!(result.1.is_empty());
}
