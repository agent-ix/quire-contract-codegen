//! Admitted source-to-comparison controls for the public V2 numeric admission.

use std::{fmt::Write as _, fs, process::Command};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    admit_v2_integer_comparison, ComparisonOperator, Domain, OperandPosition, Partner,
    V2ComparisonRefusal, V2SelectedClaimKey,
};
use quire_contract_model::{
    CheckedComparisonOperator, CheckedComparisonRefusal, CheckedOccurrence, CheckedOccurrenceRole,
};

use super::{kani_obligations_state_frame::native_twin::Twin, scratch_crate};

fn admitted(
    condition: &str,
) -> (
    quire_contract_model::CheckedPackageV2,
    quire_contract_model::CheckedNodeId,
    CheckedOccurrence,
) {
    let twin = Twin::with_balance_condition(condition);
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let claim = package
        .node(&clause)
        .expect("selected clause")
        .occurrences
        .iter()
        .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .expect("authentic claim")
        .clone();
    (package, clause, claim)
}

/// Six authored QSL comparisons keep their operator and one shared IR domain.
#[trace("TC-020", "FR-008-AC-1", "FR-008-AC-3", "FR-008-AC-9")]
#[test]
fn selected_v2_comparisons_follow_typed_ir_projection() {
    for (symbol, expected) in [
        ("=", ComparisonOperator::Equal),
        ("!=", ComparisonOperator::NotEqual),
        ("<", ComparisonOperator::Less),
        ("<=", ComparisonOperator::LessEqual),
        (">", ComparisonOperator::Greater),
        (">=", ComparisonOperator::GreaterEqual),
    ] {
        let condition = format!("self.balance {symbol} pre(self.balance)");
        let (package, clause, claim) = admitted(&condition);
        let key = V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &clause,
            occurrence: &claim,
        };
        let direct = package
            .direct_integer_comparison(&clause, &claim)
            .expect("IR projects this exact clause");
        let admitted = admit_v2_integer_comparison(&package, key).expect("typed V2 admission");
        assert_eq!(admitted.projection, direct);
        assert_eq!(admitted.relation.operator(), expected);
        assert_eq!(
            admitted.domain,
            Domain {
                minimum: 0,
                maximum: 1000
            }
        );
        assert_eq!(admitted.clause, clause);
        assert_eq!(admitted.claim_owner, clause);
        assert_eq!(admitted.occurrence, claim);
        assert!(admitted.oracle.source.contains("pub fn oracle_numeric("));
        let closed = match direct.operator {
            CheckedComparisonOperator::Eq => ComparisonOperator::Equal,
            CheckedComparisonOperator::Ne => ComparisonOperator::NotEqual,
            CheckedComparisonOperator::Lt => ComparisonOperator::Less,
            CheckedComparisonOperator::Le => ComparisonOperator::LessEqual,
            CheckedComparisonOperator::Gt => ComparisonOperator::Greater,
            CheckedComparisonOperator::Ge => ComparisonOperator::GreaterEqual,
        };
        assert_eq!(admitted.relation.operator(), closed);
    }
}

/// Literal placement changes the authored operand order without changing read provenance.
#[trace("TC-020", "FR-008-AC-2")]
#[test]
fn selected_v2_literal_first_comparison_keeps_order() {
    for (condition, position) in [
        ("self.balance < 7", OperandPosition::Left),
        ("7 > self.balance", OperandPosition::Right),
    ] {
        let (package, clause, claim) = admitted(condition);
        let selected = admit_v2_integer_comparison(
            &package,
            V2SelectedClaimKey {
                clause: &clause,
                claim_owner: &clause,
                occurrence: &claim,
            },
        )
        .expect("literal comparison admitted");
        assert_eq!(selected.relation.primary(), position);
        assert_eq!(selected.relation.partner(), Partner::Literal(7));
        assert_eq!(
            selected.domain,
            Domain {
                minimum: 0,
                maximum: 1000
            }
        );
        assert_eq!(selected.relation.evaluate(6, None), Some(true));
        assert_eq!(selected.relation.evaluate(7, None), Some(false));
    }
}

/// Emitted V2 source executes all six predicates over independently expected valuations.
#[trace("TC-020", "FR-008-AC-3", "FR-008-AC-7")]
#[test]
fn selected_v2_comparison_oracles_execute_author_order() {
    let scratch = scratch_crate::TemporaryDirectory::new("v2-numeric-comparison-oracles");
    let mut source = String::new();
    for (index, (symbol, expected)) in [
        ("=", [true, false, false]),
        ("!=", [false, true, true]),
        ("<", [false, true, false]),
        ("<=", [true, true, false]),
        (">", [false, false, true]),
        (">=", [true, false, true]),
    ]
    .into_iter()
    .enumerate()
    {
        let condition = format!("self.balance {symbol} pre(self.balance)");
        let (package, clause, claim) = admitted(&condition);
        let selected = admit_v2_integer_comparison(
            &package,
            V2SelectedClaimKey {
                clause: &clause,
                claim_owner: &clause,
                occurrence: &claim,
            },
        )
        .expect("source-authored direct comparison admitted");
        writeln!(
            source,
            "mod comparison_{index} {{\n{}\n}}",
            selected.oracle.source
        )
        .unwrap();
        writeln!(
            source,
            "#[test] fn comparison_{index}_runs() {{\n\
             assert_eq!(comparison_{index}::oracle_numeric(0, Some(0)), Some({}));\n\
             assert_eq!(comparison_{index}::oracle_numeric(0, Some(1)), Some({}));\n\
             assert_eq!(comparison_{index}::oracle_numeric(1, Some(0)), Some({}));\n\
             assert_eq!(comparison_{index}::oracle_numeric(0, None), None);\n\
             }}",
            expected[0], expected[1], expected[2],
        )
        .unwrap();
    }
    let manifest = "[package]\nname = \"v2-numeric-comparison-oracles\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[workspace]\n";
    scratch_crate::write_manifest(&scratch.path, manifest);
    fs::write(scratch.path.join("src/lib.rs"), source).expect("write generated oracle controls");
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .args(["test", "--locked", "--offline", "-j2", "--lib"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .current_dir(&scratch.path)
        .output()
        .expect("run generated V2 comparison oracles");
    assert!(
        output.status.success(),
        "generated comparison control failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// IR's structural refusal wins before CG's owner and domain checks.
#[trace("TC-020", "FR-008-AC-4", "FR-008-AC-5")]
#[test]
fn v2_comparison_refusals_preserve_selected_key_and_ir_cause() {
    let (package, clause, claim) = admitted("self.balance >= pre(self.balance)");
    let wrong_role = CheckedOccurrence {
        role: CheckedOccurrenceRole::Expression,
        ordinal: claim.ordinal,
    };
    let wrong = admit_v2_integer_comparison(
        &package,
        V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &clause,
            occurrence: &wrong_role,
        },
    )
    .expect_err("wrong role refuses");
    assert_eq!(wrong.clause, clause);
    assert_eq!(wrong.claim_owner, clause);
    assert_eq!(wrong.occurrence, wrong_role);
    assert!(matches!(
        wrong.cause,
        V2ComparisonRefusal::Ir(ref error)
            if error.kind == CheckedComparisonRefusal::MissingClaimOccurrence
    ));

    let foreign = admitted("self.balance > pre(self.balance)").1;
    let wrong_owner = admit_v2_integer_comparison(
        &package,
        V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &foreign,
            occurrence: &claim,
        },
    )
    .expect_err("foreign owner refuses");
    assert_eq!(wrong_owner.cause, V2ComparisonRefusal::WrongClaimOwner);
    assert_eq!(wrong_owner.claim_owner, foreign);
}
