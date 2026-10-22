//! Public generation controls for the general V2 clause integer-leaf policy.

use ix_trace_rs::trace;
use quire_contract_codegen::generate_v2_clause_oracle;

use super::kani_obligations_state_frame::native_twin::Twin;

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_canonical_i64_literals_choose_the_primitive_constructor() {
    let mut failures = Vec::new();
    for spelling in ["0", "1", "17", "42", "1000", "9223372036854775807"] {
        let (package, clause) =
            Twin::with_balance_condition(&format!("self.balance >= {spelling}"))
                .emitted_package("BalanceNeverDrops");
        let generated = generate_v2_clause_oracle(&package, &clause)
            .unwrap_or_else(|error| panic!("{spelling}: {error:?}"));
        assert_eq!(generated.descriptor.node_id, clause);
        assert_eq!(
            generated,
            generate_v2_clause_oracle(&package, &clause).unwrap()
        );
        assert!(generated
            .descriptor
            .operation_identities
            .iter()
            .any(|id| id == "quire.op.integer.ge"));
        if !generated
            .source
            .contains(&format!("Ok(rt::Integer::from({spelling}_i64))"))
        {
            failures.push(spelling);
        }
    }
    assert!(
        failures.is_empty(),
        "public constructor policy missing: {failures:?}"
    );
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_large_literals_keep_arbitrary_precision_parse() {
    for spelling in [
        "9223372036854775808",
        "170141183460469231731687303715884105727",
        "170141183460469231731687303715884105728",
        "340282366920938463463374607431768211456",
    ] {
        let (package, clause) = Twin::with_balance_condition(&format!("self.balance < {spelling}"))
            .emitted_package("BalanceNeverDrops");
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        assert!(
            generated.source.contains(&format!(
                "\"{spelling}\".parse().map_err(|_| ClauseStop::InvalidConstant)"
            )),
            "large literal lost its parse fallback: {spelling}"
        );
    }
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_negated_positive_two_to_63_keeps_its_child_and_owner_call() {
    let (package, clause) = Twin::with_balance_condition("self.balance >= -9223372036854775808")
        .emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
    assert!(generated
        .source
        .contains("\"9223372036854775808\".parse().map_err(|_| ClauseStop::InvalidConstant)"));
    assert!(!generated.source.contains("Integer::from(i64::MIN)"));
    assert!(generated
        .descriptor
        .operation_identities
        .iter()
        .any(|id| id == "quire.op.integer.negate"));
    assert!(generated.source.contains("IntegerArithmetic::Negate"));
}
