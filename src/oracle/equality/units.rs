//! Attach the original admitted owner's unit metadata to a checked environment.

use qsl_replay::AdmittedContext;
use quire_semantic_value::declaration::TypeEnvironment;

/// Call after package/context validation and nominal closure reconstruction.
/// A missing context preserves the caller's existing context-free environment;
/// it does not authorize a nominal item or enable Quantity oracle generation.
pub(super) fn attach_owner_units(
    environment: TypeEnvironment,
    context: Option<&AdmittedContext>,
) -> TypeEnvironment {
    match context {
        Some(context) => environment.with_units(context.unit_table().clone()),
        None => environment,
    }
}

#[cfg(test)]
mod tests {
    use super::attach_owner_units;
    use ix_trace_rs::trace;
    use qsl_replay::{
        compile_admitted, DependencyInput, ReplayLimits, SourceIdentity, StageLimits, WireNodeId,
    };
    use quire_exact::{Integer, Quantity, Rational, ValueType};
    use quire_semantic_value::declaration::TypeEnvironment;

    #[trace("TC-029", "FR-018-AC-45")]
    #[test]
    fn preparation_preserves_the_complete_owner_table_and_returned_unit_ids() {
        let compiled = compile_admitted(
            SourceIdentity::new("agent-ix", "unit-environment", "git", "1"),
            "unit-environment.native",
            br#"language "ix:native" edition "1-draft";
profile v = "quire.value.complete/v1";
dimension Length;
unit m : Length = rational(1, 1);
unit km : Length = rational(1000, 1) * m;
function keep using v(a: Quantity<m>): Boolean pure { true }
"#,
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine owner quantity declarations compile");
        let context = compiled.context();
        let original = TypeEnvironment::new([], []).expect("an empty environment");
        let m = context
            .units()
            .find(|unit| unit.qualified_name().last().unwrap().as_str() == "m")
            .expect("the declared metre unit");
        let km = context
            .units()
            .find(|unit| unit.qualified_name().last().unwrap().as_str() == "km")
            .expect("the declared but unused kilometre unit");
        assert_ne!(m.id(), km.id());
        assert!(m.converts_to(km));
        assert!(original.units().get(m.id()).is_none());
        let metre = Quantity::new(Rational::from_integer(Integer::from(1_i64)), m.id());
        assert!(original.units().resolve(&metre).is_none());

        let context_free = super::super::admit_environment(None, Vec::new(), &Default::default())
            .expect("context-free production preparation");
        assert_eq!(context_free.units(), original.units());
        let prepared =
            super::super::admit_environment(Some(context), Vec::new(), &Default::default())
                .expect("production preparation retains the admitted owner's units");
        assert_eq!(prepared.units(), context.unit_table());
        for unit in [m, km] {
            let looked_up = context
                .unit_declaration(WireNodeId::from_digest(*unit.node().as_bytes()))
                .expect("lookup uses the owner's node, not a fabricated unit id");
            assert_eq!(looked_up.id(), unit.id());
            assert_eq!(prepared.units().get(unit.id()), Some(unit.quantity_unit()));
            let quantity = Quantity::new(Rational::from_integer(Integer::from(1_i64)), unit.id());
            let resolved = prepared
                .units()
                .resolve(&quantity)
                .expect("owner metadata resolves its quantity");
            assert_eq!(resolved.quantity(), &quantity);
            assert_eq!(resolved.unit(), unit.quantity_unit());
            prepared
                .check_type(&ValueType::Quantity(unit.id()))
                .expect("quantity remains a recognized type");
        }
        let preserved = attach_owner_units(prepared, None);
        assert_eq!(preserved.units(), context.unit_table());
    }
}
