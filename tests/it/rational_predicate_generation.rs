//! Leaf native-source preparation only; negotiated generation/replay are not yet integrated.
use crate::rational_predicate_support::{admit, HEALTHY, MUTANT};
use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_rational_predicate_oracle, rational_predicate_bindings, FunctionPredicateItem,
    RationalComponent,
};

#[trace("TC-060", "FR-046-AC-1", "FR-046-AC-2", "FR-025-AC-10")]
#[test]
fn authentic_complete_predicate_prepares_native_source_and_grouped_arguments() {
    for source in [HEALTHY, MUTANT] {
        let (package, function) = admit(source);
        let oracle = generate_rational_predicate_oracle(FunctionPredicateItem {
            package: &package,
            function: &function,
        })
        .expect("the complete admitted healthy/mutated predicate must prepare native source");
        assert_eq!(oracle.function, function);
        assert_eq!(oracle.package, *package.identity());
        assert_eq!(oracle.parameters().len(), 2);
        assert!(!oracle.sources.is_empty());
        assert!(oracle.source.contains("evaluate_rational_arithmetic"));
        assert!(oracle.source.contains("planned_equality"));
        assert!(!oracle.source.contains("kani::assume"));
        let bindings = rational_predicate_bindings(&oracle);
        assert_eq!(bindings.len(), 4);
        assert_eq!(
            bindings
                .iter()
                .map(|binding| binding.position)
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            bindings
                .iter()
                .map(|binding| &binding.identifier)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            4
        );
        assert!(bindings
            .iter()
            .all(|binding| binding.primitive == quire_contract_codegen::KaniPrimitiveType::I64));
        for (index, param) in oracle.parameters().iter().enumerate() {
            assert_eq!(bindings[index * 2].parameter, param.node);
            assert_eq!(bindings[index * 2 + 1].parameter, param.node);
            assert_eq!(bindings[index * 2].component, RationalComponent::Numerator);
            assert_eq!(
                bindings[index * 2 + 1].component,
                RationalComponent::Denominator
            );
            assert_eq!(
                (bindings[index * 2].minimum, bindings[index * 2].maximum),
                (-9, 9)
            );
            assert_eq!(
                (
                    bindings[index * 2 + 1].minimum,
                    bindings[index * 2 + 1].maximum
                ),
                (1, 9)
            );
        }
    }
}

#[trace("TC-060", "FR-046-AC-7", "FR-046-AC-8")]
#[test]
fn original_capacity_and_operation_refusals_preserve_the_owner_input() {
    use quire_contract_codegen::RationalPredicateRefusal;
    let source = HEALTHY.replace("a + b = b + a", "a - b = b - a");
    let (package, function) = admit(&source);
    let result = generate_rational_predicate_oracle(FunctionPredicateItem {
        package: &package,
        function: &function,
    });
    assert!(
        matches!(&result, Err(RationalPredicateRefusal::UnsupportedOperation { operation, .. }) if operation == "quire.op.rational.sub"),
        "actual refusal: {result:?}"
    );
    let source = HEALTHY.replace("-9, 9;", "-9, 9223372036854775808;");
    let (package, function) = admit(&source);
    assert!(matches!(
        generate_rational_predicate_oracle(FunctionPredicateItem { package: &package, function: &function }),
        Err(RationalPredicateRefusal::UnsupportedEndpoint { component, value, .. })
            if component == "numerator_max" && value == "9223372036854775808"
    ));
}

#[trace("TC-060", "FR-046-AC-2", "FR-025-AC-10")]
#[test]
fn physical_component_admission_is_bijective_and_rejects_normalization_duplicates() {
    use quire_contract_codegen::{canonical_rational_arguments, RationalComponentRefusal};
    use quire_exact::{Integer, Rational};
    use std::collections::BTreeSet;
    let (package, function) = admit(HEALTHY);
    let oracle = generate_rational_predicate_oracle(FunctionPredicateItem {
        package: &package,
        function: &function,
    })
    .unwrap();
    assert!(matches!(
        canonical_rational_arguments(&oracle, &[[0, 2], [0, 1]]),
        Err(RationalComponentRefusal::NonCanonical { .. })
    ));
    assert!(matches!(
        canonical_rational_arguments(&oracle, &[[2, 2], [0, 1]]),
        Err(RationalComponentRefusal::NonCanonical { .. })
    ));
    let mut expected = BTreeSet::new();
    let mut accepted = Vec::new();
    let mut unrestricted = Vec::new();
    for n in -9_i64..=9 {
        for d in 1_i64..=9 {
            let mathematical = Rational::new(Integer::from(n), Integer::from(d)).unwrap();
            let key = (
                mathematical.numerator().to_string(),
                mathematical.denominator().to_string(),
            );
            expected.insert(key.clone());
            unrestricted.push(key);
            if let Ok(values) = canonical_rational_arguments(&oracle, &[[n, d], [0, 1]]) {
                accepted.push((
                    n,
                    d,
                    (
                        values[0].numerator().to_string(),
                        values[0].denominator().to_string(),
                    ),
                ));
            }
        }
    }
    let bijective = |values: Vec<(String, String)>| {
        values.len() == expected.len() && values.into_iter().collect::<BTreeSet<_>>() == expected
    };
    assert_eq!(accepted.len(), 111);
    assert!(bijective(
        accepted.iter().map(|(_, _, v)| v.clone()).collect()
    ));
    assert!(
        !bijective(unrestricted),
        "admitting all normalized pairs must fail one-to-one correspondence"
    );
    assert!(
        !bijective(
            accepted
                .iter()
                .filter(|(_, d, _)| *d == 1)
                .map(|(_, _, v)| v.clone())
                .collect()
        ),
        "integer-only draws lose declared fractions"
    );
    assert!(
        !bijective(
            accepted
                .iter()
                .filter(|(n, _, _)| *n <= 8)
                .map(|(_, _, v)| v.clone())
                .collect()
        ),
        "clipping an endpoint loses declared values"
    );
    assert_eq!(
        accepted
            .iter()
            .filter(|(n, _, _)| *n == 0)
            .map(|(_, d, _)| *d)
            .collect::<Vec<_>>(),
        vec![1]
    );
    assert!(canonical_rational_arguments(&oracle, &[[10, 1], [0, 1]]).is_err());
    assert!(canonical_rational_arguments(&oracle, &[[0, 0], [0, 1]]).is_err());
    assert!(matches!(
        canonical_rational_arguments(&oracle, &[[0, 1]]),
        Err(RationalComponentRefusal::ArgumentCount)
    ));
}

#[trace("TC-060", "FR-046-AC-1", "FR-046-AC-8")]
#[test]
fn a_foreign_real_function_selection_cannot_replace_the_admitted_closure() {
    use quire_contract_codegen::RationalPredicateRefusal;
    let (healthy, function) = admit(HEALTHY);
    let (_, mutant_function) = admit(MUTANT);
    assert_ne!(function, mutant_function);
    assert!(
        matches!(generate_rational_predicate_oracle(FunctionPredicateItem {package:&healthy,function:&mutant_function}),
        Err(RationalPredicateRefusal::SelectionUnavailable{node}) if node == mutant_function)
    );
    let oracle = generate_rational_predicate_oracle(FunctionPredicateItem {
        package: &healthy,
        function: &function,
    })
    .unwrap();
    assert!(oracle.sources.iter().any(|entry| entry.node_id == function));
    assert!(oracle
        .parameters()
        .windows(2)
        .all(|pair| pair[0].node < pair[1].node));
}
