//! Admitted source-to-comparison controls for the public V2 numeric admission.

use std::{fmt::Write as _, fs, process::Command};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    admit_v2_integer_comparison, ComparisonOperator, Domain, OperandPosition, Partner,
    V2ComparisonRefusal, V2SelectedClaimKey,
};
use quire_contract_model::{
    CheckedComparisonOperator, CheckedComparisonRefusal, CheckedOccurrence, CheckedOccurrenceRole,
    ExecutionPoint, ExecutionPointName,
};

use super::{kani_obligations_state_frame::native_twin::Twin, scratch_crate};

pub(super) fn admitted(
    condition: &str,
) -> (
    quire_contract_model::CheckedPackageV2,
    quire_contract_model::CheckedNodeId,
    CheckedOccurrence,
) {
    let twin = if condition == "self.balance >= pre(self.balance)" {
        Twin::new()
    } else {
        Twin::with_balance_condition(condition)
    };
    admit_twin(twin)
}

pub(super) fn admit_twin(
    twin: Twin,
) -> (
    quire_contract_model::CheckedPackageV2,
    quire_contract_model::CheckedNodeId,
    CheckedOccurrence,
) {
    let point = ExecutionPoint::Post {
        operation: ExecutionPointName::new("deposit").expect("point name"),
    };
    let (package, clauses) =
        twin.emitted_package_with_execution_points(&[("BalanceNeverDrops", point)]);
    let [clause] = <[quire_contract_model::CheckedNodeId; 1]>::try_from(clauses)
        .expect("one QSL-authored clause");
    let claim = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.node_id == clause)
        .expect("selected clause")
        .occurrences
        .iter()
        .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
        .expect("authentic claim")
        .clone();
    (package, clause, claim)
}

/// Six authored QSL comparisons keep their operator and one shared IR domain.
#[trace("TC-020")]
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
#[trace("TC-020")]
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
#[trace("TC-020")]
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
#[trace("TC-020")]
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

/// Public IR projections remain exact across every comparison, read provenance and observation.
#[trace("FR-008-AC-9", "TC-017")]
#[test]
fn v2_accessor_population_and_refusal_fields_agree() {
    use quire_contract_codegen::{prepare_v2_numeric_population, CensusTag, Population};
    use quire_contract_model::{
        CheckedComparisonObservation as Observation, CheckedComparisonOperandKind as Kind,
        CheckedComparisonRead as Read,
    };

    let mut observations = [false; 3];
    let mut provenances = [false; 2];
    for symbol in ["=", "!=", "<", "<=", ">", ">="] {
        for (twin, name) in [
            (
                if symbol == ">=" {
                    Twin::new()
                } else {
                    Twin::with_balance_condition(&format!(
                        "self.balance {symbol} pre(self.balance)"
                    ))
                },
                "BalanceNeverDrops",
            ),
            (
                Twin::with_balance_invariant_condition(&format!(
                    "self.balance {symbol} self.audit"
                )),
                "BalanceNonnegative",
            ),
            (
                Twin::with_bounded_other_precondition_condition(&format!(
                    "self.balance {symbol} other"
                )),
                "OtherBelowLimit",
            ),
            (Twin::with_bounded_other_condition(), "BalanceNeverDrops"),
        ] {
            let (package, clause) = twin.emitted_package(name);
            let claim = package
                .resolved_graph()
                .nodes
                .iter()
                .find(|node| node.node_id == clause)
                .expect("authored clause")
                .occurrences
                .iter()
                .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
                .expect("authentic claim");
            let direct = package
                .direct_integer_comparison(&clause, claim)
                .expect("public clause-context projection");
            let selected = admit_v2_integer_comparison(
                &package,
                V2SelectedClaimKey {
                    clause: &clause,
                    claim_owner: &clause,
                    occurrence: claim,
                },
            )
            .expect("generator admits the projected comparison");
            assert_eq!(selected.projection, direct);
            for (ordinal, operand) in direct.operands.iter().enumerate() {
                assert_eq!(
                    operand.ordinal,
                    u64::try_from(ordinal).expect("two operands")
                );
                if let Kind::Read {
                    provenance,
                    observation,
                } = &operand.kind
                {
                    match provenance {
                        Read::StateField { .. } => provenances[0] = true,
                        Read::OperationInput { ordinal, .. } => {
                            assert_eq!(*ordinal, 0);
                            provenances[1] = true;
                        }
                    }
                    observations[match observation {
                        Observation::Current => 0,
                        Observation::Pre => 1,
                        Observation::Post => 2,
                    }] = true;
                    assert_eq!(i128::from(selected.domain.minimum), operand.range.lower);
                    assert_eq!(i128::from(selected.domain.maximum), operand.range.upper);
                }
            }
            let generated = prepare_v2_numeric_population(&selected, Population::Broad)
                .expect("both comparison sides are nonempty");
            assert_eq!(generated.census.relation(), selected.relation);
            assert!(!generated.census.in_domain().is_empty());
            for case in generated.census.in_domain() {
                let primary = i128::from(case.primary);
                let partner = match selected.relation.partner() {
                    Partner::Read => i128::from(case.partner.expect("second read")),
                    Partner::Literal(value) => {
                        assert_eq!(case.partner, None);
                        i128::from(value)
                    }
                };
                let (left, right) = match selected.relation.primary() {
                    OperandPosition::Left => (primary, partner),
                    OperandPosition::Right => (partner, primary),
                };
                for (value, operand) in [left, right].into_iter().zip(&direct.operands) {
                    assert!((operand.range.lower..=operand.range.upper).contains(&value));
                }
                let holds = match direct.operator {
                    CheckedComparisonOperator::Eq => left == right,
                    CheckedComparisonOperator::Ne => left != right,
                    CheckedComparisonOperator::Lt => left < right,
                    CheckedComparisonOperator::Le => left <= right,
                    CheckedComparisonOperator::Gt => left > right,
                    CheckedComparisonOperator::Ge => left >= right,
                };
                assert_eq!(
                    case.tag,
                    if holds {
                        CensusTag::Holds
                    } else {
                        CensusTag::Violated
                    }
                );
            }
        }
    }
    assert_eq!(observations, [true; 3]);
    assert_eq!(provenances, [true; 2]);

    for (twin, name) in [
        (Twin::with_negated_balance_clause(), "BalanceNeverDrops"),
        (Twin::without_balance_range(), "BalanceNeverDrops"),
        (
            Twin::with_balance_condition("self.balance + 1 >= pre(self.balance)"),
            "BalanceNeverDrops",
        ),
        (
            Twin::with_balance_invariant_condition("self.balance = 500"),
            "BalanceNonnegative",
        ),
    ] {
        let (package, clause) = twin.emitted_package(name);
        let claim = package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| node.node_id == clause)
            .expect("authored clause")
            .occurrences
            .iter()
            .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
            .expect("authentic claim");
        let direct = package
            .direct_integer_comparison(&clause, claim)
            .expect_err("IR refuses shape or range");
        let refused = admit_v2_integer_comparison(
            &package,
            V2SelectedClaimKey {
                clause: &clause,
                claim_owner: &clause,
                occurrence: claim,
            },
        )
        .expect_err("generator preserves IR refusal");
        assert_eq!(refused.clause, clause);
        assert_eq!(&refused.occurrence, claim);
        assert_eq!(refused.claim_owner, clause);
        assert_eq!(refused.path, direct.path);
        assert_eq!(refused.child, direct.child);
        assert_eq!(refused.cause, V2ComparisonRefusal::Ir(direct));
        let generation = quire_contract_codegen::generate_v2_numeric_strategy(
            &quire_contract_codegen::V2NumericRequest {
                package: &package,
                key: V2SelectedClaimKey {
                    clause: &clause,
                    claim_owner: &clause,
                    occurrence: claim,
                },
                population: Population::Broad,
                policy: quire_contract_codegen::V2NumericPolicy {
                    minimum_accepted_cases: 0,
                    minimum_rejected_cases: 0,
                    maximum_discarded_cases: 0,
                },
                process_source: None,
            },
        )
        .expect_err("refused comparison produces no artifact bundle");
        assert_eq!(generation.comparison.as_ref(), Some(refused.as_ref()));
    }
}
