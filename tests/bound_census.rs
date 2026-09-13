use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    bound_strategy::{
        census::{
            compute_census, render_boundary_constants, render_edge_constants, BoundaryCensus,
            CensusEdge, CensusNames, CensusRead, CensusTag, EdgeDirection, OutOfDomainCase,
            UnrepresentableEdge,
        },
        relation::{ComparisonOperator, Domain, OperandPosition, Partner, Relation},
    },
    GenerationErrorCode, GenerationTerminalState, StrategyErrorCode,
};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("src")).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const VERSION_DOMAIN: Domain = Domain {
    minimum: 0,
    maximum: 1000,
};

const FULL_DOMAIN: Domain = Domain {
    minimum: i64::MIN,
    maximum: i64::MAX,
};

fn literal(operator: ComparisonOperator, value: i64) -> Relation {
    Relation::with_literal(operator, OperandPosition::Left, value)
}

fn two_reads(operator: ComparisonOperator) -> Relation {
    Relation::between_reads(operator)
}

/// Evaluates the relation from its textual reading, independently of `Relation::evaluate`.
fn independent_holds(relation: Relation, primary: i64, partner: Option<i64>) -> bool {
    let other = match (relation.partner(), partner) {
        (Partner::Literal(value), None) => value,
        (Partner::Read, Some(value)) => value,
        shape => panic!("valuation shape does not match the relation: {shape:?}"),
    };
    let (left, right) = match relation.primary() {
        OperandPosition::Left => (i128::from(primary), i128::from(other)),
        OperandPosition::Right => (i128::from(other), i128::from(primary)),
    };
    match relation.operator() {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => left < right,
        ComparisonOperator::LessEqual => left <= right,
        ComparisonOperator::Greater => left > right,
        ComparisonOperator::GreaterEqual => left >= right,
    }
}

fn in_domain_pairs(census: &BoundaryCensus) -> Vec<(i64, Option<i64>, CensusTag)> {
    census
        .in_domain()
        .iter()
        .map(|case| (case.primary, case.partner, case.tag))
        .collect()
}

fn out_of_domain_pairs(census: &BoundaryCensus) -> Vec<(i64, Option<i64>)> {
    census
        .out_of_domain()
        .iter()
        .map(|case| (case.primary, case.partner))
        .collect()
}

/// Trace: TC-019, FR-010-AC-1
#[test]
fn tc_019_literal_census_for_amount_less_than_seven_is_exact() {
    use CensusTag::{Holds as H, Violated as V};
    let census = compute_census(literal(ComparisonOperator::Less, 7), VERSION_DOMAIN).unwrap();
    assert_eq!(
        in_domain_pairs(&census),
        [
            (0, None, H),
            (1, None, H),
            (6, None, H),
            (7, None, V),
            (8, None, V),
            (999, None, V),
            (1000, None, V),
        ]
    );
    assert_eq!(out_of_domain_pairs(&census), [(-1, None), (1001, None)]);
    assert!(census.unrepresentable_edges().is_empty());
    assert_eq!(census.boundary_population().unwrap().len(), 7);

    let source = render_boundary_constants(
        &census,
        CensusNames {
            item_suffix: "",
            fields: &["amount"],
        },
    )
    .unwrap();
    assert!(source.contains("pub const IN_DOMAIN_CENSUS: [CensusCase; 7] = ["));
    assert!(source.contains("    CensusCase { amount: 6, tag: CensusTag::Holds },\n"));
    assert!(source.contains("    CensusCase { amount: 7, tag: CensusTag::Violated },\n"));
    assert!(source.contains("pub const OUT_OF_DOMAIN_CASES: [OutOfDomainCase; 2] = [\n    OutOfDomainCase { amount: -1 },\n    OutOfDomainCase { amount: 1001 },\n];"));
    assert!(source.contains("pub const UNREPRESENTABLE_EDGES: [UnrepresentableEdge; 0] = [];"));
    assert!(!source.contains("serde"));
}

/// Trace: TC-019, FR-010-AC-1
#[test]
fn tc_019_literal_edges_coinciding_with_domain_edges_are_deduplicated() {
    use CensusTag::{Holds as H, Violated as V};
    let census = compute_census(literal(ComparisonOperator::Less, 1), VERSION_DOMAIN).unwrap();
    assert_eq!(
        in_domain_pairs(&census),
        [
            (0, None, H),
            (1, None, V),
            (2, None, V),
            (999, None, V),
            (1000, None, V),
        ]
    );
    assert_eq!(out_of_domain_pairs(&census), [(-1, None), (1001, None)]);
    assert!(census.unrepresentable_edges().is_empty());
    let source = render_boundary_constants(
        &census,
        CensusNames {
            item_suffix: "",
            fields: &["amount"],
        },
    )
    .unwrap();
    assert!(source.contains("pub const IN_DOMAIN_CENSUS: [CensusCase; 5] = ["));
    assert_eq!(source.matches("CensusCase { amount: 1, ").count(), 1);
}

/// Trace: TC-019, FR-010-AC-2
#[test]
fn tc_019_two_read_census_for_version_unchanged_is_exact() {
    use CensusTag::{Holds as H, Violated as V};
    let census = compute_census(two_reads(ComparisonOperator::Equal), VERSION_DOMAIN).unwrap();
    assert_eq!(
        in_domain_pairs(&census),
        [
            (0, Some(0), H),
            (0, Some(1), V),
            (1, Some(0), V),
            (1, Some(1), H),
            (1, Some(2), V),
            (999, Some(998), V),
            (999, Some(999), H),
            (999, Some(1000), V),
            (1000, Some(999), V),
            (1000, Some(1000), H),
        ]
    );
    assert_eq!(
        out_of_domain_pairs(&census),
        [
            (-1, Some(0)),
            (0, Some(-1)),
            (0, Some(1001)),
            (1, Some(-1)),
            (1, Some(1001)),
            (999, Some(-1)),
            (999, Some(1001)),
            (1000, Some(-1)),
            (1000, Some(1001)),
            (1001, Some(1000)),
        ]
    );
    assert!(census.unrepresentable_edges().is_empty());

    let source = render_boundary_constants(
        &census,
        CensusNames {
            item_suffix: "",
            fields: &["post_version_number", "pre_version_number"],
        },
    )
    .unwrap();
    assert!(source.contains(
        "    CensusCase { post_version_number: 999, pre_version_number: 1000, tag: CensusTag::Violated },\n"
    ));
    assert!(source.contains(
        "    OutOfDomainCase { post_version_number: 1001, pre_version_number: 1000 },\n"
    ));
}

/// Trace: TC-019, FR-010-AC-3
#[test]
fn tc_019_full_i64_domain_lists_outer_edges_as_unrepresentable() {
    let amount = compute_census(literal(ComparisonOperator::Less, 7), FULL_DOMAIN).unwrap();
    assert_eq!(
        amount
            .in_domain()
            .iter()
            .map(|case| case.primary)
            .collect::<Vec<_>>(),
        [i64::MIN, i64::MIN + 1, 6, 7, 8, i64::MAX - 1, i64::MAX]
    );
    assert!(amount.out_of_domain().is_empty());
    assert_eq!(
        amount.unrepresentable_edges(),
        [
            UnrepresentableEdge {
                read: CensusRead::Primary,
                edge: CensusEdge::Minimum,
                direction: EdgeDirection::Below,
            },
            UnrepresentableEdge {
                read: CensusRead::Primary,
                edge: CensusEdge::Maximum,
                direction: EdgeDirection::Above,
            },
        ]
    );

    let equal = compute_census(two_reads(ComparisonOperator::Equal), FULL_DOMAIN).unwrap();
    assert_eq!(equal.in_domain().len(), 10);
    assert!(equal.out_of_domain().is_empty());
    assert_eq!(
        equal.unrepresentable_edges(),
        [
            UnrepresentableEdge {
                read: CensusRead::Primary,
                edge: CensusEdge::Minimum,
                direction: EdgeDirection::Below,
            },
            UnrepresentableEdge {
                read: CensusRead::Primary,
                edge: CensusEdge::Maximum,
                direction: EdgeDirection::Above,
            },
            UnrepresentableEdge {
                read: CensusRead::Partner,
                edge: CensusEdge::Minimum,
                direction: EdgeDirection::Below,
            },
            UnrepresentableEdge {
                read: CensusRead::Partner,
                edge: CensusEdge::Maximum,
                direction: EdgeDirection::Above,
            },
        ]
    );

    // A literal at the i64 edge is itself an unrepresentable literal edge, never wrapped.
    let at_maximum =
        compute_census(literal(ComparisonOperator::Less, i64::MAX), FULL_DOMAIN).unwrap();
    assert!(at_maximum.out_of_domain().is_empty());
    assert!(at_maximum
        .unrepresentable_edges()
        .contains(&UnrepresentableEdge {
            read: CensusRead::Primary,
            edge: CensusEdge::Literal,
            direction: EdgeDirection::Above,
        }));
}

/// Trace: TC-019, FR-010-AC-1, FR-010-AC-2, FR-010-AC-3
#[test]
fn tc_019_rendered_census_constants_compile_with_denied_warnings() {
    let fixtures = [
        (
            "Amount",
            literal(ComparisonOperator::Less, 7),
            VERSION_DOMAIN,
            &["amount"][..],
        ),
        (
            "Version",
            two_reads(ComparisonOperator::Equal),
            VERSION_DOMAIN,
            &["post", "pre"][..],
        ),
        (
            "AmountFull",
            literal(ComparisonOperator::Less, 7),
            FULL_DOMAIN,
            &["amount"][..],
        ),
        (
            "EqualFull",
            two_reads(ComparisonOperator::Equal),
            FULL_DOMAIN,
            &["x", "y"][..],
        ),
    ];
    let mut source =
        String::from("#![deny(missing_docs)]\n//! Generated bound census fixture crate.\n\n");
    for (suffix, relation, domain, fields) in fixtures {
        let census = compute_census(relation, domain).unwrap();
        source.push_str(
            &render_boundary_constants(
                &census,
                CensusNames {
                    item_suffix: suffix,
                    fields,
                },
            )
            .unwrap(),
        );
        source.push('\n');
    }
    source.push_str(
        r#"
#[cfg(test)]
mod generated_tests {
    use super::*;

    #[test]
    fn amount_arrays_are_exact() {
        let census: Vec<(i64, bool)> = IN_DOMAIN_CENSUS_AMOUNT
            .iter()
            .map(|case| (case.amount, case.tag == CensusTagAmount::Holds))
            .collect();
        assert_eq!(
            census,
            [(0, true), (1, true), (6, true), (7, false), (8, false), (999, false), (1000, false)]
        );
        assert_eq!(
            OUT_OF_DOMAIN_CASES_AMOUNT,
            [OutOfDomainCaseAmount { amount: -1 }, OutOfDomainCaseAmount { amount: 1001 }]
        );
        assert_eq!(UNREPRESENTABLE_EDGES_AMOUNT.len(), 0);
    }

    #[test]
    fn version_arrays_are_exact() {
        assert_eq!(IN_DOMAIN_CENSUS_VERSION.len(), 10);
        assert_eq!(
            IN_DOMAIN_CENSUS_VERSION[4],
            CensusCaseVersion { post: 1, pre: 2, tag: CensusTagVersion::Violated }
        );
        assert_eq!(OUT_OF_DOMAIN_CASES_VERSION.len(), 10);
        assert_eq!(OUT_OF_DOMAIN_CASES_VERSION[0], OutOfDomainCaseVersion { post: -1, pre: 0 });
        assert_eq!(OUT_OF_DOMAIN_CASES_VERSION[9], OutOfDomainCaseVersion { post: 1001, pre: 1000 });
    }

    #[test]
    fn full_domain_arrays_list_both_outer_edges() {
        assert_eq!(IN_DOMAIN_CENSUS_AMOUNTFULL[0].amount, i64::MIN);
        assert_eq!(IN_DOMAIN_CENSUS_AMOUNTFULL[6].amount, i64::MAX);
        assert_eq!(OUT_OF_DOMAIN_CASES_AMOUNTFULL.len(), 0);
        assert_eq!(
            UNREPRESENTABLE_EDGES_AMOUNTFULL,
            [
                UnrepresentableEdgeAmountFull {
                    read: "amount",
                    edge: CensusEdgeAmountFull::Minimum,
                    direction: EdgeDirectionAmountFull::Below,
                },
                UnrepresentableEdgeAmountFull {
                    read: "amount",
                    edge: CensusEdgeAmountFull::Maximum,
                    direction: EdgeDirectionAmountFull::Above,
                },
            ]
        );
        assert_eq!(OUT_OF_DOMAIN_CASES_EQUALFULL.len(), 0);
        assert_eq!(UNREPRESENTABLE_EDGES_EQUALFULL.len(), 4);
        assert_eq!(UNREPRESENTABLE_EDGES_EQUALFULL[3].read, "y");
        assert_eq!(
            IN_DOMAIN_CENSUS_EQUALFULL[9],
            CensusCaseEqualFull { x: i64::MAX, y: i64::MAX, tag: CensusTagEqualFull::Holds }
        );
    }
}
"#,
    );

    let temporary = TemporaryDirectory::new("quire-generated-bound-census");
    fs::write(
        temporary.0.join("Cargo.toml"),
        "[package]\nname = \"generated-bound-census\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    )
    .unwrap();
    fs::write(temporary.0.join("src/lib.rs"), source).unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated census constants did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: TC-019, FR-010-AC-4
#[test]
fn tc_019_single_tagged_census_refuses_boundary_but_keeps_edge_arrays() {
    let census =
        compute_census(literal(ComparisonOperator::LessEqual, 1000), VERSION_DOMAIN).unwrap();
    assert!(census
        .in_domain()
        .iter()
        .all(|case| case.tag == CensusTag::Holds));
    let refusal = census.boundary_population().unwrap_err();
    assert_eq!(
        refusal.code,
        StrategyErrorCode::UnsupportedCampaignConstraint
    );
    assert_eq!(refusal.terminal_state, GenerationTerminalState::Unsupported);
    assert_eq!(refusal.generation_code, None);
    assert_eq!(refusal.path, "campaign.boundary");

    let names = CensusNames {
        item_suffix: "",
        fields: &["amount"],
    };
    assert_eq!(
        render_boundary_constants(&census, names).unwrap_err(),
        refusal
    );
    assert_eq!(
        census.out_of_domain(),
        [
            OutOfDomainCase {
                primary: -1,
                partner: None,
            },
            OutOfDomainCase {
                primary: 1001,
                partner: None,
            },
        ]
    );
    let edges = render_edge_constants(&census, names).unwrap();
    assert!(edges.contains("pub const OUT_OF_DOMAIN_CASES: [OutOfDomainCase; 2] = [\n    OutOfDomainCase { amount: -1 },\n    OutOfDomainCase { amount: 1001 },\n];"));
    assert!(edges.contains("pub const UNREPRESENTABLE_EDGES"));
    assert!(!edges.contains("IN_DOMAIN_CENSUS"));
    assert!(!edges.contains("CensusTag"));
}

/// Trace: TC-019
#[test]
fn tc_019_invalid_domain_and_field_names_are_structured_refusals() {
    let reversed = compute_census(
        literal(ComparisonOperator::Less, 7),
        Domain {
            minimum: 1,
            maximum: 0,
        },
    )
    .unwrap_err();
    assert_eq!(reversed.code, StrategyErrorCode::InvalidRange);
    assert_eq!(
        reversed.terminal_state,
        GenerationTerminalState::InvalidInput
    );

    let census = compute_census(two_reads(ComparisonOperator::Equal), VERSION_DOMAIN).unwrap();
    for (suffix, fields) in [
        ("", &["post"][..]),
        ("", &["post", "post"][..]),
        ("", &["post", "type"][..]),
        ("", &["post", "tag"][..]),
        ("", &["Post", "pre"][..]),
        ("", &["post", "pre", "extra"][..]),
        ("a_b", &["post", "pre"][..]),
    ] {
        let refusal = render_edge_constants(
            &census,
            CensusNames {
                item_suffix: suffix,
                fields,
            },
        )
        .unwrap_err();
        assert_eq!(refusal.code, StrategyErrorCode::InvalidStrategyIdentity);
        assert_eq!(
            refusal.terminal_state,
            GenerationTerminalState::InvalidInput
        );
    }

    let oversized = "A".repeat(100_000);
    let refusal = render_edge_constants(
        &census,
        CensusNames {
            item_suffix: &oversized,
            fields: &["post", "pre"],
        },
    )
    .unwrap_err();
    assert_eq!(refusal.code, StrategyErrorCode::ResourceLimitExceeded);
    assert_eq!(
        refusal.generation_code,
        Some(GenerationErrorCode::ResourceLimitExceeded)
    );
    assert_eq!(refusal.terminal_state, GenerationTerminalState::Unsupported);
}

fn sweep_domains() -> Vec<Domain> {
    let mut domains = Vec::new();
    for minimum in -3..=3 {
        for width in 0..=6 {
            domains.push(Domain {
                minimum,
                maximum: minimum + width,
            });
        }
    }
    for width in 0..=3 {
        domains.push(Domain {
            minimum: i64::MIN,
            maximum: i64::MIN + width,
        });
        domains.push(Domain {
            minimum: i64::MAX - width,
            maximum: i64::MAX,
        });
    }
    domains.push(FULL_DOMAIN);
    domains.push(VERSION_DOMAIN);
    domains
}

fn sweep_relations(domain: Domain) -> Vec<Relation> {
    let mut literals = vec![i64::MIN, i64::MIN + 1, 0, 7, 1000, i64::MAX - 1, i64::MAX];
    let low = i128::from(domain.minimum) - 3;
    let high = i128::from(domain.maximum) + 3;
    literals.extend(
        [low, low + 1, low + 2]
            .into_iter()
            .chain(i128::from(domain.minimum)..=i128::from(domain.minimum).saturating_add(8))
            .chain([high - 2, high - 1, high])
            .filter_map(|value| i64::try_from(value).ok()),
    );
    literals.sort_unstable();
    literals.dedup();
    let mut relations = Vec::new();
    for operator in ComparisonOperator::ALL {
        relations.push(two_reads(operator));
        for value in &literals {
            for primary in [OperandPosition::Left, OperandPosition::Right] {
                relations.push(Relation::with_literal(operator, primary, *value));
            }
        }
    }
    relations
}

/// Trace: TC-019, FR-010-AC-5, NFR-004-AC-2
#[test]
fn tc_019_exhaustive_sweep_tags_domains_order_size_and_determinism() {
    let mut largest = 0;
    let mut admitted = 0;
    let mut checked = 0;
    for domain in sweep_domains() {
        let inside = |value: i64| (domain.minimum..=domain.maximum).contains(&value);
        for relation in sweep_relations(domain) {
            let census = compute_census(relation, domain).unwrap();
            checked += 1;

            for case in census.in_domain() {
                assert!(inside(case.primary), "{relation:?} {domain:?} {case:?}");
                assert_eq!(case.partner.is_some(), relation.has_partner_read());
                assert!(case.partner.map_or(true, inside));
                let expected = if independent_holds(relation, case.primary, case.partner) {
                    CensusTag::Holds
                } else {
                    CensusTag::Violated
                };
                assert_eq!(case.tag, expected, "{relation:?} {domain:?} {case:?}");
            }
            for case in census.out_of_domain() {
                assert_eq!(case.partner.is_some(), relation.has_partner_read());
                assert!(
                    !inside(case.primary) || case.partner.is_some_and(|value| !inside(value)),
                    "{relation:?} {domain:?} {case:?} has no out-of-domain value"
                );
            }
            assert!(in_domain_pairs(&census)
                .windows(2)
                .all(|pair| (pair[0].0, pair[0].1) < (pair[1].0, pair[1].1)));
            assert!(out_of_domain_pairs(&census)
                .windows(2)
                .all(|pair| pair[0] < pair[1]));
            assert!(census
                .unrepresentable_edges()
                .windows(2)
                .all(|pair| pair[0] < pair[1]));

            let size = census.in_domain().len() + census.out_of_domain().len();
            assert!(size <= 20, "{relation:?} {domain:?} holds {size} cases");
            largest = largest.max(size);

            let again = compute_census(relation, domain).unwrap();
            assert_eq!(census, again);
            let fields: &[&str] = if relation.has_partner_read() {
                &["post", "pre"]
            } else {
                &["amount"]
            };
            let names = CensusNames {
                item_suffix: "Sweep",
                fields,
            };
            assert_eq!(
                render_edge_constants(&census, names).unwrap().as_bytes(),
                render_edge_constants(&again, names).unwrap().as_bytes()
            );
            if census.boundary_population().is_ok() {
                admitted += 1;
                assert_eq!(
                    render_boundary_constants(&census, names)
                        .unwrap()
                        .as_bytes(),
                    render_boundary_constants(&again, names).unwrap().as_bytes()
                );
            }
        }
    }
    // The sweep reaches the largest census the FR-010 rules produce: 10 in-domain pairs (the
    // `min` and `max` primary edges each lose one out-of-domain partner) plus 10 out-of-domain
    // pairs, exactly reaching NFR-004's bound of 20.
    assert_eq!(largest, 20, "the sweep must reach the worst-case census");
    assert!(admitted > 0 && admitted < checked);
}
