use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    bound_strategy::{
        population::{
            render_population, side_values, Population, PopulationRequest, PopulationSide,
            RenderedPopulation, SideValues, ValueSet,
        },
        relation::{ComparisonOperator, Domain, OperandPosition, Partner, Relation},
    },
    GenerationTerminalState, StrategyErrorCode,
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

/// One valuation: primary value and, for a two-read relation, partner value.
type Valuation = (i64, Option<i64>);

const VERSION_DOMAIN: Domain = Domain {
    minimum: 0,
    maximum: 1000,
};

const FULL_DOMAIN: Domain = Domain {
    minimum: i64::MIN,
    maximum: i64::MAX,
};

const LITERAL_IDENTIFIERS: &[&str] = &["amount_current"];
const READ_IDENTIFIERS: &[&str] = &["left_current", "right_current"];
const VERSION_IDENTIFIERS: &[&str] = &["version_number_post", "version_number_pre"];

fn literal(operator: ComparisonOperator, primary: OperandPosition, value: i64) -> Relation {
    Relation::with_literal(operator, primary, value)
}

fn two_reads(operator: ComparisonOperator) -> Relation {
    Relation::between_reads(operator)
}

fn identifiers(relation: Relation) -> &'static [&'static str] {
    match relation.partner() {
        Partner::Literal(_) => LITERAL_IDENTIFIERS,
        Partner::Read => READ_IDENTIFIERS,
    }
}

fn request(
    relation: Relation,
    domain: Domain,
    population: Population,
) -> PopulationRequest<'static> {
    PopulationRequest {
        relation,
        domain,
        population,
        read_identifiers: identifiers(relation),
    }
}

/// Rust's own comparison, written out independently of the library under test.
fn compare(operator: ComparisonOperator, left: i64, right: i64) -> bool {
    match operator {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => left < right,
        ComparisonOperator::LessEqual => left <= right,
        ComparisonOperator::Greater => left > right,
        ComparisonOperator::GreaterEqual => left >= right,
    }
}

fn source_symbol(operator: ComparisonOperator) -> &'static str {
    match operator {
        ComparisonOperator::Equal => "==",
        ComparisonOperator::NotEqual => "!=",
        ComparisonOperator::Less => "<",
        ComparisonOperator::LessEqual => "<=",
        ComparisonOperator::Greater => ">",
        ComparisonOperator::GreaterEqual => ">=",
    }
}

/// Independent enumeration of one side of `relation` over a small `domain`.
fn enumerated_side(
    relation: Relation,
    domain: Domain,
    side: PopulationSide,
) -> BTreeSet<Valuation> {
    let holds = side == PopulationSide::Satisfying;
    let mut set = BTreeSet::new();
    for primary in domain.minimum..=domain.maximum {
        match relation.partner() {
            Partner::Literal(value) => {
                let result = match relation.primary() {
                    OperandPosition::Left => compare(relation.operator(), primary, value),
                    OperandPosition::Right => compare(relation.operator(), value, primary),
                };
                if result == holds {
                    set.insert((primary, None));
                }
            }
            Partner::Read => {
                for partner in domain.minimum..=domain.maximum {
                    if compare(relation.operator(), primary, partner) == holds {
                        set.insert((primary, Some(partner)));
                    }
                }
            }
        }
    }
    set
}

/// Every member of a small value set, checking its interval shape and size on the way.
fn members(set: &ValueSet) -> Vec<i64> {
    let intervals = set.intervals();
    assert!(
        (1..=2).contains(&intervals.len()),
        "{set:?} is not one or two intervals"
    );
    for pair in intervals.windows(2) {
        assert!(
            pair[0].maximum() < pair[1].minimum(),
            "{set:?} intervals overlap or are unordered"
        );
    }
    let values = intervals
        .iter()
        .flat_map(|interval| interval.minimum()..=interval.maximum())
        .collect::<Vec<_>>();
    assert_eq!(values.len() as u128, set.size(), "{set:?} size");
    assert!(values.iter().all(|value| set.contains(*value)));
    values
}

/// The valuations a computed side structurally describes.
fn structural_side(values: &SideValues) -> BTreeSet<Valuation> {
    match values {
        SideValues::Literal { primary } => members(primary)
            .into_iter()
            .map(|value| (value, None))
            .collect(),
        SideValues::Correlated { primary, .. } => {
            let mut set = BTreeSet::new();
            for value in primary.minimum()..=primary.maximum() {
                let partners = values
                    .partner_values(value)
                    .unwrap_or_else(|| panic!("{values:?} draws {value} with no partner"));
                for partner in members(&partners) {
                    set.insert((value, Some(partner)));
                }
            }
            set
        }
    }
}

/// Domains of 1 to 4 members at an ordinary position and at both `i64` extremes.
fn small_domains() -> Vec<Domain> {
    let mut domains = Vec::new();
    for size in 1..=4i64 {
        for minimum in [-1, i64::MIN, i64::MAX - (size - 1)] {
            domains.push(Domain {
                minimum,
                maximum: minimum + (size - 1),
            });
        }
    }
    domains
}

/// Every operator as a two-read relation and against a literal at each domain member, on either
/// side of the comparison.
fn small_relations(domain: Domain) -> Vec<Relation> {
    let mut relations = Vec::new();
    for operator in ComparisonOperator::ALL {
        relations.push(two_reads(operator));
        for value in domain.minimum..=domain.maximum {
            relations.push(literal(operator, OperandPosition::Left, value));
            relations.push(literal(operator, OperandPosition::Right, value));
        }
    }
    relations
}

fn assert_empty_population(
    failure: &quire_contract_codegen::StrategyDiagnostic,
    side: PopulationSide,
) {
    assert_eq!(failure.code, StrategyErrorCode::EmptyPopulation);
    assert_eq!(failure.terminal_state, GenerationTerminalState::Unsupported);
    assert_eq!(failure.generation_code, None);
    let (path, name) = match side {
        PopulationSide::Satisfying => ("population.satisfying", "Satisfying"),
        PopulationSide::Violating => ("population.violating", "Violating"),
    };
    assert_eq!(failure.path, path);
    assert!(failure.message.contains(name), "{}", failure.message);
}

/// Trace: TC-018, FR-009-AC-2
#[test]
fn tc_018_value_sets_equal_independent_enumeration_and_empty_sides_refuse() {
    let mut checked_sides = 0;
    let mut refused_sides = 0;
    let mut rendered = 0;
    let mut refused_renders = 0;
    for domain in small_domains() {
        for relation in small_relations(domain) {
            let mut empty = Vec::new();
            for side in [PopulationSide::Satisfying, PopulationSide::Violating] {
                let expected = enumerated_side(relation, domain, side);
                match side_values(&relation, domain, side) {
                    Ok(values) => {
                        assert!(
                            !expected.is_empty(),
                            "{relation:?} {domain:?} {side:?} admitted an empty side"
                        );
                        assert_eq!(
                            structural_side(&values),
                            expected,
                            "{relation:?} {domain:?} {side:?}"
                        );
                        for primary in domain.minimum..=domain.maximum {
                            let partners: Vec<Option<i64>> = match relation.partner() {
                                Partner::Literal(_) => vec![None],
                                Partner::Read => {
                                    (domain.minimum..=domain.maximum).map(Some).collect()
                                }
                            };
                            for partner in partners {
                                assert_eq!(
                                    values.contains(primary, partner),
                                    expected.contains(&(primary, partner)),
                                    "{relation:?} {domain:?} {side:?} ({primary}, {partner:?})"
                                );
                            }
                        }
                        checked_sides += 1;
                    }
                    Err(failure) => {
                        assert!(
                            expected.is_empty(),
                            "{relation:?} {domain:?} {side:?} refused a non-empty side"
                        );
                        assert_empty_population(&failure, side);
                        empty.push(side);
                        refused_sides += 1;
                    }
                }
            }
            for population in Population::ALL {
                let result = render_population(&request(relation, domain, population));
                let refusal = match population {
                    Population::Satisfying => empty
                        .iter()
                        .copied()
                        .find(|side| *side == PopulationSide::Satisfying),
                    Population::Violating => empty
                        .iter()
                        .copied()
                        .find(|side| *side == PopulationSide::Violating),
                    Population::Broad => empty.first().copied(),
                };
                match (result, refusal) {
                    (Ok(generated), None) => {
                        assert!(generated.source.contains(&generated.strategy_function));
                        rendered += 1;
                    }
                    (Err(failure), Some(side)) => {
                        assert_empty_population(&failure, side);
                        refused_renders += 1;
                    }
                    (result, refusal) => panic!(
                        "{relation:?} {domain:?} {population:?}: {result:?} but expected refusal \
                         {refusal:?}"
                    ),
                }
            }
        }
    }
    // 3 domain positions x (4 sizes) x 6 operators x (1 + 2 x size) relations x 2 sides.
    assert_eq!(checked_sides + refused_sides, 3 * 6 * (3 + 5 + 7 + 9) * 2);
    assert!(refused_sides > 0);
    assert_eq!(rendered + refused_renders, 3 * 6 * (3 + 5 + 7 + 9) * 3);
    assert!(refused_renders > 0);
}

/// Trace: TC-018, FR-009-AC-4
#[test]
fn tc_018_amount_below_zero_and_at_or_above_zero_refuse_the_empty_side() {
    let below = literal(ComparisonOperator::Less, OperandPosition::Left, 0);
    for population in [Population::Satisfying, Population::Broad] {
        let failure = render_population(&request(below, VERSION_DOMAIN, population)).unwrap_err();
        assert_empty_population(&failure, PopulationSide::Satisfying);
    }
    let violating =
        render_population(&request(below, VERSION_DOMAIN, Population::Violating)).unwrap();
    assert!(violating.source.contains("(0i64..=1000i64)"));

    let at_or_above = literal(ComparisonOperator::GreaterEqual, OperandPosition::Left, 0);
    for population in [Population::Violating, Population::Broad] {
        let failure =
            render_population(&request(at_or_above, VERSION_DOMAIN, population)).unwrap_err();
        assert_empty_population(&failure, PopulationSide::Violating);
    }
    let satisfying = render_population(&request(
        at_or_above,
        VERSION_DOMAIN,
        Population::Satisfying,
    ))
    .unwrap();
    assert!(satisfying.source.contains("(0i64..=1000i64)"));
}

/// Trace: TC-018, FR-009-AC-5
#[test]
fn tc_018_full_i64_domain_value_sets_use_checked_edges_without_overflow() {
    let full = u128::from(u64::MAX) + 1;

    let not_equal = side_values(
        &two_reads(ComparisonOperator::NotEqual),
        FULL_DOMAIN,
        PopulationSide::Satisfying,
    )
    .unwrap();
    for (primary, expected) in [
        (i64::MIN, vec![(i64::MIN + 1, i64::MAX)]),
        (i64::MAX, vec![(i64::MIN, i64::MAX - 1)]),
        (0, vec![(i64::MIN, -1), (1, i64::MAX)]),
    ] {
        let partners = not_equal.partner_values(primary).unwrap();
        assert_eq!(partners.size(), full - 1);
        let intervals = partners
            .intervals()
            .iter()
            .map(|interval| (interval.minimum(), interval.maximum()))
            .collect::<Vec<_>>();
        assert_eq!(intervals, expected);
    }
    let equal_violating = side_values(
        &two_reads(ComparisonOperator::NotEqual),
        FULL_DOMAIN,
        PopulationSide::Violating,
    )
    .unwrap();
    assert!(equal_violating.contains(i64::MIN, Some(i64::MIN)));
    assert!(!equal_violating.contains(i64::MIN, Some(i64::MAX)));

    let less = side_values(
        &two_reads(ComparisonOperator::Less),
        FULL_DOMAIN,
        PopulationSide::Satisfying,
    )
    .unwrap();
    let SideValues::Correlated { primary, .. } = less else {
        panic!("two reads must be correlated: {less:?}");
    };
    assert_eq!(
        (primary.minimum(), primary.maximum(), primary.size()),
        (i64::MIN, i64::MAX - 1, full - 1)
    );
    assert_eq!(
        less.partner_values(i64::MIN).unwrap().size(),
        full - 1,
        "i64::MIN < every other member"
    );
    assert_eq!(
        less.partner_values(i64::MAX - 1).unwrap().intervals()[0].minimum(),
        i64::MAX
    );
    assert_eq!(less.partner_values(i64::MAX), None);

    let below_minimum = literal(ComparisonOperator::Less, OperandPosition::Left, i64::MIN);
    assert_empty_population(
        &side_values(&below_minimum, FULL_DOMAIN, PopulationSide::Satisfying).unwrap_err(),
        PopulationSide::Satisfying,
    );
    let above_maximum = literal(ComparisonOperator::Less, OperandPosition::Right, i64::MAX);
    assert_empty_population(
        &side_values(&above_maximum, FULL_DOMAIN, PopulationSide::Satisfying).unwrap_err(),
        PopulationSide::Satisfying,
    );
    let everything_else = side_values(
        &literal(
            ComparisonOperator::NotEqual,
            OperandPosition::Left,
            i64::MIN,
        ),
        FULL_DOMAIN,
        PopulationSide::Satisfying,
    )
    .unwrap();
    let SideValues::Literal { primary } = everything_else else {
        panic!("a literal relation must be a literal side: {everything_else:?}");
    };
    assert_eq!(primary.size(), full - 1);
    assert_eq!(primary.intervals().len(), 1);

    for operator in [
        ComparisonOperator::NotEqual,
        ComparisonOperator::Less,
        ComparisonOperator::Equal,
    ] {
        for relation in [
            two_reads(operator),
            literal(operator, OperandPosition::Left, 0),
        ] {
            for population in Population::ALL {
                let generated =
                    render_population(&request(relation, FULL_DOMAIN, population)).unwrap();
                assert!(generated.source.contains("i64::MIN"));
                assert!(generated.source.contains("i64::MAX"));
            }
        }
    }
}

/// Trace: TC-018, FR-009-AC-3, FR-009-AC-6
#[test]
fn tc_018_generated_sources_are_constructive_and_byte_identical() {
    let mut scanned = 0;
    let mut domains = small_domains();
    domains.push(VERSION_DOMAIN);
    domains.push(FULL_DOMAIN);
    for domain in domains {
        let relations = if domain.maximum.abs_diff(domain.minimum) < 4 {
            small_relations(domain)
        } else {
            ComparisonOperator::ALL
                .iter()
                .flat_map(|operator| {
                    [
                        two_reads(*operator),
                        literal(*operator, OperandPosition::Left, 7),
                        literal(*operator, OperandPosition::Right, 7),
                    ]
                })
                .collect()
        };
        for relation in relations {
            for population in Population::ALL {
                let Ok(first) = render_population(&request(relation, domain, population)) else {
                    continue;
                };
                let second = render_population(&request(relation, domain, population)).unwrap();
                assert_eq!(first, second);
                for forbidden in [
                    "prop_filter",
                    "prop_filter_map",
                    "prop_assume",
                    "reject",
                    "discard",
                    "Discard",
                    "prop_oneof",
                    "Union",
                ] {
                    assert!(
                        !first.source.contains(forbidden),
                        "{relation:?} {domain:?} {population:?} contains {forbidden}"
                    );
                }
                scanned += 1;
            }
        }
    }
    assert!(scanned > 1000, "only {scanned} sources scanned");

    let relation = two_reads(ComparisonOperator::Equal);
    let names = Population::ALL
        .iter()
        .map(|population| {
            let generated = render_population(&PopulationRequest {
                relation,
                domain: VERSION_DOMAIN,
                population: *population,
                read_identifiers: VERSION_IDENTIFIERS,
            })
            .unwrap();
            (generated.strategy_function, generated.case_type)
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        names.len(),
        3,
        "each population has distinct generated names"
    );
}

/// Trace: TC-018
#[test]
fn tc_018_malformed_requests_refuse_before_rendering() {
    let relation = two_reads(ComparisonOperator::Less);
    for (read_identifiers, reason) in [
        (&["only_one"][..], "one identifier for two reads"),
        (&["left", "left"][..], "duplicate identifiers"),
        (&["left", "expected"][..], "the reserved tag field"),
        (&["left", "not an identifier"][..], "a non-identifier"),
        (&["left", "fn"][..], "a keyword"),
    ] {
        let failure = render_population(&PopulationRequest {
            relation,
            domain: VERSION_DOMAIN,
            population: Population::Broad,
            read_identifiers,
        })
        .unwrap_err();
        assert_eq!(
            failure.code,
            StrategyErrorCode::InvalidStrategyIdentity,
            "{reason}"
        );
        assert_eq!(failure.path, "read_identifiers", "{reason}");
    }

    let reversed = render_population(&request(
        relation,
        Domain {
            minimum: 2,
            maximum: 1,
        },
        Population::Satisfying,
    ))
    .unwrap_err();
    assert_eq!(reversed.code, StrategyErrorCode::InvalidRange);
    assert_eq!(
        reversed.terminal_state,
        GenerationTerminalState::InvalidInput
    );
    assert_eq!(reversed.path, "domain");
}

/// Rendered source of one population as a module of the generated check crate.
struct Module {
    name: String,
    generated: RenderedPopulation,
}

fn projection(module: &Module, relation: Relation) -> String {
    let Module { name, generated } = module;
    let case_type = &generated.case_type;
    let expectation_type = &generated.expectation_type;
    let identifiers = identifiers_for(relation, name);
    let partner = identifiers
        .get(1)
        .map(|identifier| format!("Some(case.{identifier})"))
        .unwrap_or_else(|| "None".to_owned());
    format!(
        "|case: &crate::{name}::{case_type}| (case.{}, {partner}, case.expected == crate::{name}::{expectation_type}::Holds)",
        identifiers[0]
    )
}

fn identifiers_for(relation: Relation, name: &str) -> &'static [&'static str] {
    if name.starts_with("version_") {
        VERSION_IDENTIFIERS
    } else {
        identifiers(relation)
    }
}

fn rust_literal(value: i64) -> String {
    match value {
        i64::MIN => "i64::MIN".to_owned(),
        i64::MAX => "i64::MAX".to_owned(),
        value => format!("{value}i64"),
    }
}

/// An independent `fn(i64, Option<i64>) -> bool` evaluating `relation` as written.
fn relation_closure(relation: Relation) -> String {
    let symbol = source_symbol(relation.operator());
    match (relation.partner(), relation.primary()) {
        (Partner::Read, _) => {
            format!("|primary: i64, partner: Option<i64>| primary {symbol} partner.unwrap()")
        }
        (Partner::Literal(value), OperandPosition::Left) => {
            format!(
                "|primary: i64, _: Option<i64>| primary {symbol} {}",
                rust_literal(value)
            )
        }
        (Partner::Literal(value), OperandPosition::Right) => {
            format!(
                "|primary: i64, _: Option<i64>| {} {symbol} primary",
                rust_literal(value)
            )
        }
    }
}

const GENERATED_SUPPORT: &str = r#"use std::{cell::Cell, collections::BTreeSet, fmt::Debug};

use proptest::{
    strategy::{BoxedStrategy, Strategy as _, ValueTree as _},
    test_runner::{Config, RngAlgorithm, TestCaseError, TestError, TestRng, TestRunner},
};

type Valuation = (i64, Option<i64>, bool);
type Projection<T> = fn(&T) -> Valuation;
type RelationFn = fn(i64, Option<i64>) -> bool;

const WALK_SEEDS: u32 = 48;
const WALK_REGENERATIONS: u32 = 4;
const WALK_NODE_BUDGET: usize = 200_000;

fn seeded_runner(seed: u32, config: Config) -> TestRunner {
    let mut bytes = [0u8; 32];
    bytes[..4].copy_from_slice(&seed.to_le_bytes());
    TestRunner::new_with_rng(config, TestRng::from_seed(RngAlgorithm::ChaCha, &bytes))
}

fn strict_config(cases: u32) -> Config {
    Config {
        cases,
        max_local_rejects: 0,
        max_global_rejects: 0,
        failure_persistence: None,
        ..Config::default()
    }
}

/// Visits the root and every candidate on every protocol-valid simplify/complicate path of one
/// seeded tree.
fn walk_every_path<T: Debug>(strategy: &BoxedStrategy<T>, seed: u32, mut visit: impl FnMut(&T)) {
    let replay = |path: &[bool]| {
        let mut runner = seeded_runner(seed, strict_config(WALK_REGENERATIONS));
        let mut tree = strategy.new_tree(&mut runner).expect("constructive trees never reject");
        for simplify in path {
            let moved = if *simplify { tree.simplify() } else { tree.complicate() };
            assert!(moved, "replaying a recorded shrink path diverged");
        }
        tree
    };
    visit(&replay(&[]).current());
    let mut pending = vec![Vec::new()];
    let mut nodes = 0;
    while let Some(path) = pending.pop() {
        nodes += 1;
        assert!(nodes <= WALK_NODE_BUDGET, "shrink walk exceeded {WALK_NODE_BUDGET} nodes");
        for simplify in [true, false] {
            if !simplify && path.is_empty() {
                // `ValueTree::complicate` need not handle being called before any `simplify`.
                continue;
            }
            let mut tree = replay(&path);
            let moved = if simplify { tree.simplify() } else { tree.complicate() };
            if moved {
                visit(&tree.current());
                let mut next = path.clone();
                next.push(simplify);
                pending.push(next);
            }
        }
    }
}

/// FR-009-AC-2 and FR-012-AC-1 for one small population.
pub fn check_small<T: Debug>(
    label: &str,
    strategy: BoxedStrategy<T>,
    project: Projection<T>,
    expected: &[Valuation],
    minimum: i64,
    maximum: i64,
) {
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    let mut generated = BTreeSet::new();
    for seed in 0..WALK_SEEDS {
        let mut root_tag = None;
        walk_every_path(&strategy, seed, |case| {
            let valuation = project(case);
            let (primary, partner, holds) = valuation;
            assert!((minimum..=maximum).contains(&primary), "{label}: {case:?} left the domain");
            if let Some(partner) = partner {
                assert!((minimum..=maximum).contains(&partner), "{label}: {case:?} left the domain");
            }
            assert!(expected.contains(&valuation), "{label}: {case:?} is not on its side");
            let original = *root_tag.get_or_insert(holds);
            assert_eq!(original, holds, "{label}: seed {seed} changed tag or side at {case:?}");
            generated.insert(valuation);
        });
    }
    assert_eq!(generated, expected, "{label}: generatable set differs from the enumeration");
}

/// FR-009-AC-1/3/5 draws and FR-012-AC-2 greedy shrink paths for one wide population.
pub fn check_campaign<T: Debug>(
    label: &str,
    strategy: BoxedStrategy<T>,
    project: Projection<T>,
    relation: RelationFn,
    minimum: i64,
    maximum: i64,
    trees: u32,
) {
    let check = |case: &T| -> Result<bool, String> {
        let (primary, partner, holds) = project(case);
        let in_domain = (minimum..=maximum).contains(&primary)
            && partner.map_or(true, |partner| (minimum..=maximum).contains(&partner));
        if !in_domain {
            return Err(format!("{label}: {case:?} left the domain"));
        }
        if relation(primary, partner) != holds {
            return Err(format!("{label}: {case:?} is not on its tagged side"));
        }
        Ok(holds)
    };

    let evaluations = Cell::new(0u32);
    let holds_seen = Cell::new(0u32);
    let mut runner = TestRunner::new_with_rng(
        strict_config(10_000),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    runner
        .run(&strategy, |case| {
            evaluations.set(evaluations.get() + 1);
            let holds = check(&case).map_err(TestCaseError::fail)?;
            holds_seen.set(holds_seen.get() + u32::from(holds));
            Ok(())
        })
        .unwrap_or_else(|error| panic!("{label}: {error}"));
    // Every case was evaluated exactly once: no local reject, global reject, or discard retry.
    assert_eq!(evaluations.get(), 10_000, "{label}");
    if label.ends_with("Broad") {
        assert!(holds_seen.get() > 0 && holds_seen.get() < 10_000, "{label}: one-sided Broad");
    }

    for seed in 0..trees {
        let mut runner = seeded_runner(seed, strict_config(256));
        let mut tree = strategy.new_tree(&mut runner).unwrap();
        let original = check(&tree.current()).unwrap();
        let mut steps = 0;
        while tree.simplify() {
            steps += 1;
            assert!(steps < 100_000, "{label}: greedy shrink did not terminate");
            let (primary, partner, _) = project(&tree.current());
            assert_eq!(check(&tree.current()).unwrap(), original, "{label}: tag changed");
            // A synthetic failure predicate, so the path both keeps and rejects candidates.
            let fails = (primary ^ partner.unwrap_or(0)) & 1 == 0;
            if !fails && tree.complicate() {
                assert_eq!(check(&tree.current()).unwrap(), original, "{label}: tag changed");
            }
        }
    }
}

/// FR-012-AC-3: a Violating campaign whose oracle is replaced by `post == pre || post == pre + 1`.
pub fn check_replaced_oracle_counterexample<T: Debug>(
    strategy: BoxedStrategy<T>,
    project: Projection<T>,
) {
    let mut runner = TestRunner::new_with_rng(
        strict_config(1_000_000),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let result = runner.run(&strategy, |case| {
        let (post, pre, holds) = project(&case);
        let pre = pre.unwrap();
        let oracle = post == pre || post == pre + 1;
        if oracle == holds {
            Ok(())
        } else {
            Err(TestCaseError::fail("conformance mismatch"))
        }
    });
    match result {
        Err(TestError::Fail(_, minimal)) => {
            let (post, pre, holds) = project(&minimal);
            let pre = pre.unwrap();
            assert_eq!(post, pre + 1, "{minimal:?}");
            assert!((0..=1000).contains(&post) && (0..=1000).contains(&pre), "{minimal:?}");
            assert!(!holds, "{minimal:?} must stay Violated");
        }
        other => panic!("expected a minimal counterexample, got {other:?}"),
    }
}
"#;

/// Trace: TC-018, TC-021, FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-5, FR-012-AC-1, FR-012-AC-2, FR-012-AC-3
#[test]
fn tc_018_tc_021_generated_populations_compile_draw_and_shrink_inside_their_sides() {
    let mut modules = Vec::new();
    let mut tests = String::from("use super::support::*;\n");

    let mut index = 0;
    let mut walk_domains = Vec::new();
    for size in 1..=4i64 {
        // Spanning zero, wholly negative, wholly positive, and at both i64 extremes, because
        // proptest's integer shrinking targets zero clamped to the range.
        for minimum in [-1, -9, 5, i64::MIN, i64::MAX - (size - 1)] {
            walk_domains.push(Domain {
                minimum,
                maximum: minimum + (size - 1),
            });
        }
    }
    for domain in walk_domains {
        let mut relations = vec![];
        for operator in ComparisonOperator::ALL {
            relations.push(two_reads(operator));
            for value in domain.minimum..=domain.maximum {
                relations.push(literal(operator, OperandPosition::Left, value));
            }
        }
        for relation in relations {
            let satisfying = enumerated_side(relation, domain, PopulationSide::Satisfying);
            let violating = enumerated_side(relation, domain, PopulationSide::Violating);
            for population in Population::ALL {
                let Ok(generated) = render_population(&request(relation, domain, population))
                else {
                    continue;
                };
                let name = format!("small_{index}");
                index += 1;
                let mut expected = String::new();
                let tagged = match population {
                    Population::Satisfying => vec![(&satisfying, true)],
                    Population::Violating => vec![(&violating, false)],
                    Population::Broad => vec![(&satisfying, true), (&violating, false)],
                };
                for (set, holds) in tagged {
                    for (primary, partner) in set {
                        let partner = partner
                            .map(|value| format!("Some({})", rust_literal(value)))
                            .unwrap_or_else(|| "None".to_owned());
                        let _ = write!(
                            expected,
                            "({}, {partner}, {holds}), ",
                            rust_literal(*primary)
                        );
                    }
                }
                let module = Module { name, generated };
                let _ = writeln!(
                    tests,
                    "#[test]\nfn {name}() {{\n    check_small(\"{relation:?} over {label_min}..={label_max} {population:?}\", crate::{name}::{function}(), {projection}, &[{expected}], {min}, {max});\n}}\n",
                    name = module.name,
                    function = module.generated.strategy_function,
                    projection = projection(&module, relation),
                    label_min = domain.minimum,
                    label_max = domain.maximum,
                    min = rust_literal(domain.minimum),
                    max = rust_literal(domain.maximum),
                );
                modules.push(module);
            }
        }
    }
    // 5 positions x 6 operators x (3 + 4 + 5 + 6) relations x 3 populations, minus refusals.
    assert!(index > 1000, "only {index} small populations generated");

    let version = two_reads(ComparisonOperator::Equal);
    for population in Population::ALL {
        let generated = render_population(&PopulationRequest {
            relation: version,
            domain: VERSION_DOMAIN,
            population,
            read_identifiers: VERSION_IDENTIFIERS,
        })
        .unwrap();
        let module = Module {
            name: format!("version_{population:?}").to_lowercase(),
            generated,
        };
        let _ = writeln!(
            tests,
            "#[test]\nfn {name}() {{\n    check_campaign(\"VersionUnchanged {population:?}\", crate::{name}::{function}(), {projection}, {relation}, 0, 1000, 1000);\n}}\n",
            name = module.name,
            function = module.generated.strategy_function,
            projection = projection(&module, version),
            relation = relation_closure(version),
        );
        if population == Population::Violating {
            let _ = writeln!(
                tests,
                "#[test]\nfn version_violating_replaced_oracle() {{\n    check_replaced_oracle_counterexample(crate::{name}::{function}(), {projection});\n}}\n",
                name = module.name,
                function = module.generated.strategy_function,
                projection = projection(&module, version),
            );
        }
        modules.push(module);
    }

    for operator in [
        ComparisonOperator::NotEqual,
        ComparisonOperator::Less,
        ComparisonOperator::Equal,
    ] {
        for relation in [
            two_reads(operator),
            literal(operator, OperandPosition::Left, 0),
        ] {
            for population in Population::ALL {
                let generated =
                    render_population(&request(relation, FULL_DOMAIN, population)).unwrap();
                let module = Module {
                    name: format!(
                        "full_{operator:?}_{}_{population:?}",
                        if relation.has_partner_read() {
                            "reads"
                        } else {
                            "literal"
                        }
                    )
                    .to_lowercase(),
                    generated,
                };
                let _ = writeln!(
                    tests,
                    "#[test]\nfn {name}() {{\n    check_campaign(\"{relation:?} full {population:?}\", crate::{name}::{function}(), {projection}, {closure}, i64::MIN, i64::MAX, 100);\n}}\n",
                    name = module.name,
                    function = module.generated.strategy_function,
                    projection = projection(&module, relation),
                    closure = relation_closure(relation),
                );
                modules.push(module);
            }
        }
    }

    let temporary = TemporaryDirectory::new("quire-generated-bound-populations");
    fs::write(
        temporary.0.join("Cargo.toml"),
        "[package]\nname = \"generated-bound-populations\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\n",
    )
    .unwrap();
    let mut root = String::from(
        "#![deny(missing_docs)]\n//! Generated bound population check crate.\n\n#[cfg(test)]\nmod support;\n#[cfg(test)]\nmod populations;\n\n",
    );
    for module in &modules {
        fs::write(
            temporary.0.join(format!("src/{}.rs", module.name)),
            &module.generated.source,
        )
        .unwrap();
        let _ = writeln!(root, "pub mod {};", module.name);
    }
    fs::write(temporary.0.join("src/lib.rs"), root).unwrap();
    fs::write(temporary.0.join("src/support.rs"), GENERATED_SUPPORT).unwrap();
    fs::write(temporary.0.join("src/populations.rs"), tests).unwrap();

    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "generated populations did not compile and execute:\nstdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected_tests = modules.len() + 1;
    assert!(
        stdout.contains(&format!("{expected_tests} passed")),
        "expected {expected_tests} generated tests to pass:\n{stdout}"
    );
}
