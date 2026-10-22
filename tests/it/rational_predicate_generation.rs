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
