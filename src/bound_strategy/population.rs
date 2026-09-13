//! Constructive `Satisfying`, `Violating`, and `Broad` populations for one bound comparison.
//!
//! This consumes the IR-independent [`Relation`] and [`Domain`] of [`super::relation`]: the shape
//! FR-008 admission hands to FR-009. Extracting a relation, its domain, and its read identifiers from
//! a public IR `BoundPackage` waits on the codegen#4 bounded-integer oracle grammar and is
//! deliberately not implemented here.
//!
//! Every value set is computed directly as intervals or as "domain minus one point", so generated
//! populations construct each case on its side instead of drawing and filtering. Edge values are
//! derived with checked 128-bit arithmetic, so `i64::MIN..=i64::MAX` needs no wrapping, saturation,
//! or truncation.

// Implements: FR-009, FR-012

use std::fmt::Write as _;

use sha2::{Digest as _, Sha256};

use super::relation::{ComparisonOperator, Domain, OperandPosition, Partner, Relation};
use crate::{
    oracle::length_delimited_identity, GenerationErrorCode, StrategyDiagnostic, StrategyErrorCode,
    MAX_GENERATED_SOURCE_BYTES,
};

/// Field name of the expectation tag in every generated case type.
///
/// An injected read identifier may not use it.
pub const EXPECTATION_FIELD: &str = "expected";

/// The operator with its operands swapped: `a op b` exactly when `b mirrored(op) a`.
const fn mirrored(operator: ComparisonOperator) -> ComparisonOperator {
    match operator {
        ComparisonOperator::Equal => ComparisonOperator::Equal,
        ComparisonOperator::NotEqual => ComparisonOperator::NotEqual,
        ComparisonOperator::Less => ComparisonOperator::Greater,
        ComparisonOperator::LessEqual => ComparisonOperator::GreaterEqual,
        ComparisonOperator::Greater => ComparisonOperator::Less,
        ComparisonOperator::GreaterEqual => ComparisonOperator::LessEqual,
    }
}

/// The logical complement: `a negated(op) b` exactly when `a op b` is false.
const fn negated(operator: ComparisonOperator) -> ComparisonOperator {
    match operator {
        ComparisonOperator::Equal => ComparisonOperator::NotEqual,
        ComparisonOperator::NotEqual => ComparisonOperator::Equal,
        ComparisonOperator::Less => ComparisonOperator::GreaterEqual,
        ComparisonOperator::LessEqual => ComparisonOperator::Greater,
        ComparisonOperator::Greater => ComparisonOperator::LessEqual,
        ComparisonOperator::GreaterEqual => ComparisonOperator::Less,
    }
}

const fn symbol(operator: ComparisonOperator) -> &'static str {
    match operator {
        ComparisonOperator::Equal => "==",
        ComparisonOperator::NotEqual => "!=",
        ComparisonOperator::Less => "<",
        ComparisonOperator::LessEqual => "<=",
        ComparisonOperator::Greater => ">",
        ComparisonOperator::GreaterEqual => ">=",
    }
}

/// The operator oriented as `primary op' other`, consistent with [`Relation::evaluate`].
const fn primary_operator(relation: &Relation) -> ComparisonOperator {
    match relation.primary() {
        OperandPosition::Left => relation.operator(),
        OperandPosition::Right => mirrored(relation.operator()),
    }
}

/// Number of reads, and therefore of `i64` fields in a generated case.
const fn read_count(relation: &Relation) -> usize {
    if relation.has_partner_read() {
        2
    } else {
        1
    }
}

const fn domain_interval(domain: Domain) -> Interval {
    Interval {
        minimum: domain.minimum,
        maximum: domain.maximum,
    }
}

fn validate_domain(domain: Domain) -> Result<(), StrategyDiagnostic> {
    if domain.minimum > domain.maximum {
        return Err(diagnostic(
            StrategyErrorCode::InvalidRange,
            "domain",
            format!(
                "domain minimum {} exceeds maximum {}",
                domain.minimum, domain.maximum
            ),
        ));
    }
    Ok(())
}

/// One side of a relation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PopulationSide {
    /// Valuations for which the relation holds, tagged `Holds`.
    Satisfying,
    /// Valuations for which the relation is violated, tagged `Violated`.
    Violating,
}

impl PopulationSide {
    /// Both sides, satisfying first.
    pub const ALL: [Self; 2] = [Self::Satisfying, Self::Violating];

    /// Stable side name used in diagnostics.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Satisfying => "Satisfying",
            Self::Violating => "Violating",
        }
    }

    /// Expectation tag variant carried by this side's cases.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Satisfying => "Holds",
            Self::Violating => "Violated",
        }
    }

    const fn path(self) -> &'static str {
        match self {
            Self::Satisfying => "population.satisfying",
            Self::Violating => "population.violating",
        }
    }
}

/// A requested population.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Population {
    /// Only satisfying cases, each tagged `Holds`.
    Satisfying,
    /// Only violating cases, each tagged `Violated`.
    Violating,
    /// Both sides, each case keeping its own tag; requires both sides to be non-empty.
    Broad,
}

impl Population {
    /// Every population.
    pub const ALL: [Self; 3] = [Self::Satisfying, Self::Violating, Self::Broad];

    const fn name(self) -> &'static str {
        match self {
            Self::Satisfying => "Satisfying",
            Self::Violating => "Violating",
            Self::Broad => "Broad",
        }
    }

    const fn component(self) -> &'static str {
        match self {
            Self::Satisfying => "satisfying",
            Self::Violating => "violating",
            Self::Broad => "broad",
        }
    }
}

/// A non-empty inclusive interval of `i64` values.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Interval {
    minimum: i64,
    maximum: i64,
}

impl Interval {
    /// Inclusive lower bound.
    #[must_use]
    pub const fn minimum(self) -> i64 {
        self.minimum
    }

    /// Inclusive upper bound.
    #[must_use]
    pub const fn maximum(self) -> i64 {
        self.maximum
    }

    /// Whether `value` lies in the interval.
    #[must_use]
    pub const fn contains(self, value: i64) -> bool {
        self.minimum <= value && value <= self.maximum
    }

    /// Number of members, at least one and at most 2^64.
    #[must_use]
    pub fn size(self) -> u128 {
        // `abs_diff` of two `i64` is at most 2^64 - 1, so adding one stays far inside `u128`.
        u128::from(self.maximum.abs_diff(self.minimum)) + 1
    }
}

/// A non-empty set of values for one read, as at most two inclusive intervals.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValueSet {
    /// One inclusive interval.
    Interval(Interval),
    /// Every member of `domain` except `excluded`, which lies in `domain`; `domain` has at least two
    /// members. Generated code draws an index over the reduced size and maps it around the point.
    DomainExceptPoint {
        /// The interval the point is removed from.
        domain: Interval,
        /// The removed point.
        excluded: i64,
    },
}

impl ValueSet {
    /// The set as one or two disjoint ascending inclusive intervals.
    #[must_use]
    pub fn intervals(&self) -> Vec<Interval> {
        match *self {
            Self::Interval(interval) => vec![interval],
            Self::DomainExceptPoint { domain, excluded } => {
                let below = excluded
                    .checked_sub(1)
                    .filter(|maximum| *maximum >= domain.minimum)
                    .map(|maximum| Interval {
                        minimum: domain.minimum,
                        maximum,
                    });
                let above = excluded
                    .checked_add(1)
                    .filter(|minimum| *minimum <= domain.maximum)
                    .map(|minimum| Interval {
                        minimum,
                        maximum: domain.maximum,
                    });
                below.into_iter().chain(above).collect()
            }
        }
    }

    /// Whether `value` belongs to the set.
    #[must_use]
    pub const fn contains(&self, value: i64) -> bool {
        match *self {
            Self::Interval(interval) => interval.contains(value),
            Self::DomainExceptPoint { domain, excluded } => {
                domain.contains(value) && value != excluded
            }
        }
    }

    /// Number of members.
    #[must_use]
    pub fn size(&self) -> u128 {
        match *self {
            Self::Interval(interval) => interval.size(),
            // The invariant guarantees at least two members, so this cannot underflow.
            Self::DomainExceptPoint { domain, .. } => domain.size() - 1,
        }
    }
}

/// How a partner value set follows from the drawn primary value `p` over domain `min..=max`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PartnerRule {
    /// Exactly `p`.
    EqualToPrimary,
    /// `min..=max` without `p`.
    DomainExceptPrimary,
    /// `p + 1..=max`.
    AbovePrimary,
    /// `p..=max`.
    AtOrAbovePrimary,
    /// `min..=p - 1`.
    BelowPrimary,
    /// `min..=p`.
    AtOrBelowPrimary,
}

/// The complete value set of one side of one relation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SideValues {
    /// A primary read compared with a literal.
    Literal {
        /// Every primary value on this side.
        primary: ValueSet,
    },
    /// A primary read compared with a partner read.
    Correlated {
        /// Every primary value whose partner set on this side is non-empty.
        primary: Interval,
        /// The partner set as a function of the primary value.
        partner: PartnerRule,
        /// The shared domain partner values are drawn from.
        domain: Domain,
    },
}

impl SideValues {
    /// The non-empty partner set for `primary`, or `None` for a literal relation or a primary value
    /// outside this side's primary set.
    #[must_use]
    pub fn partner_values(&self, primary: i64) -> Option<ValueSet> {
        let Self::Correlated {
            primary: primaries,
            partner,
            domain,
        } = *self
        else {
            return None;
        };
        if !primaries.contains(primary) {
            return None;
        }
        let interval = |minimum: Option<i64>, maximum: Option<i64>| {
            let (minimum, maximum) = (minimum?, maximum?);
            (minimum <= maximum).then_some(ValueSet::Interval(Interval { minimum, maximum }))
        };
        match partner {
            PartnerRule::EqualToPrimary => interval(Some(primary), Some(primary)),
            PartnerRule::DomainExceptPrimary => {
                (domain.minimum < domain.maximum).then_some(ValueSet::DomainExceptPoint {
                    domain: domain_interval(domain),
                    excluded: primary,
                })
            }
            PartnerRule::AbovePrimary => interval(primary.checked_add(1), Some(domain.maximum)),
            PartnerRule::AtOrAbovePrimary => interval(Some(primary), Some(domain.maximum)),
            PartnerRule::BelowPrimary => interval(Some(domain.minimum), primary.checked_sub(1)),
            PartnerRule::AtOrBelowPrimary => interval(Some(domain.minimum), Some(primary)),
        }
    }

    /// Whether a valuation belongs to this side. `partner` is `None` for a literal relation.
    #[must_use]
    pub fn contains(&self, primary: i64, partner: Option<i64>) -> bool {
        match (self, partner) {
            (Self::Literal { primary: values }, None) => values.contains(primary),
            (Self::Correlated { .. }, Some(partner)) => self
                .partner_values(primary)
                .is_some_and(|values| values.contains(partner)),
            _ => false,
        }
    }
}

/// Computes the value set of `side` of `relation` over `domain`.
///
/// # Errors
///
/// Returns `EmptyPopulation` with terminal state `unsupported`, naming the side in its path and
/// message, when the side has no valuation over the domain.
// Implements: FR-009
pub fn side_values(
    relation: &Relation,
    domain: Domain,
    side: PopulationSide,
) -> Result<SideValues, StrategyDiagnostic> {
    validate_domain(domain)?;
    let operator = match side {
        PopulationSide::Satisfying => primary_operator(relation),
        PopulationSide::Violating => negated(primary_operator(relation)),
    };
    let values = match relation.partner() {
        Partner::Literal(literal) => literal_values(operator, literal, domain)?
            .map(|primary| SideValues::Literal { primary }),
        Partner::Read => correlated_values(operator, domain)?,
    };
    values.ok_or_else(|| {
        diagnostic(
            StrategyErrorCode::EmptyPopulation,
            side.path(),
            format!(
                "the {} side of {} has no value over {}..={}",
                side.name(),
                relation_text(relation, "primary", "partner"),
                domain.minimum,
                domain.maximum
            ),
        )
    })
}

fn literal_values(
    operator: ComparisonOperator,
    literal: i64,
    domain: Domain,
) -> Result<Option<ValueSet>, StrategyDiagnostic> {
    let exact = i128::from(literal);
    let (lower, upper) = match operator {
        ComparisonOperator::Equal => (exact, exact),
        ComparisonOperator::NotEqual => {
            return Ok(if !domain.contains(exact) {
                Some(ValueSet::Interval(domain_interval(domain)))
            } else if domain.minimum == domain.maximum {
                None
            } else {
                Some(ValueSet::DomainExceptPoint {
                    domain: domain_interval(domain),
                    excluded: literal,
                })
            });
        }
        ComparisonOperator::Less => (i128::MIN, edge(literal, -1)?),
        ComparisonOperator::LessEqual => (i128::MIN, exact),
        ComparisonOperator::Greater => (edge(literal, 1)?, i128::MAX),
        ComparisonOperator::GreaterEqual => (exact, i128::MAX),
    };
    Ok(clamped(domain, lower, upper)?.map(ValueSet::Interval))
}

fn correlated_values(
    operator: ComparisonOperator,
    domain: Domain,
) -> Result<Option<SideValues>, StrategyDiagnostic> {
    let minimum = i128::from(domain.minimum);
    let maximum = i128::from(domain.maximum);
    let (lower, upper, partner) = match operator {
        ComparisonOperator::Equal => (minimum, maximum, PartnerRule::EqualToPrimary),
        ComparisonOperator::NotEqual if domain.minimum == domain.maximum => return Ok(None),
        ComparisonOperator::NotEqual => (minimum, maximum, PartnerRule::DomainExceptPrimary),
        ComparisonOperator::Less => (
            minimum,
            edge(domain.maximum, -1)?,
            PartnerRule::AbovePrimary,
        ),
        ComparisonOperator::LessEqual => (minimum, maximum, PartnerRule::AtOrAbovePrimary),
        ComparisonOperator::Greater => {
            (edge(domain.minimum, 1)?, maximum, PartnerRule::BelowPrimary)
        }
        ComparisonOperator::GreaterEqual => (minimum, maximum, PartnerRule::AtOrBelowPrimary),
    };
    Ok(
        clamped(domain, lower, upper)?.map(|primary| SideValues::Correlated {
            primary,
            partner,
            domain,
        }),
    )
}

/// `value + delta` in checked 128-bit arithmetic.
fn edge(value: i64, delta: i128) -> Result<i128, StrategyDiagnostic> {
    i128::from(value)
        .checked_add(delta)
        .ok_or_else(arithmetic_diagnostic)
}

/// Narrows a checked 128-bit edge back to `i64`, refusing rather than truncating.
fn narrow(value: i128) -> Result<i64, StrategyDiagnostic> {
    i64::try_from(value).map_err(|_| arithmetic_diagnostic())
}

/// Intersects `lower..=upper` with `domain`; `None` when the intersection is empty.
fn clamped(
    domain: Domain,
    lower: i128,
    upper: i128,
) -> Result<Option<Interval>, StrategyDiagnostic> {
    let lower = lower.max(i128::from(domain.minimum));
    let upper = upper.min(i128::from(domain.maximum));
    if lower > upper {
        return Ok(None);
    }
    Ok(Some(Interval {
        minimum: narrow(lower)?,
        maximum: narrow(upper)?,
    }))
}

fn arithmetic_diagnostic() -> StrategyDiagnostic {
    diagnostic(
        StrategyErrorCode::CorrelationOverflow,
        "relation",
        "a checked 128-bit edge value left the i64 range".to_owned(),
    )
}

/// Inputs for rendering one bound population.
#[derive(Clone, Copy, Debug)]
pub struct PopulationRequest<'a> {
    /// The admitted relation.
    pub relation: Relation,
    /// The shared inclusive domain of every read.
    pub domain: Domain,
    /// The population to render.
    pub population: Population,
    /// One Rust identifier per read, primary first then partner, used as case field names. The
    /// bound slice injects the generated oracle's dependency parameter identifiers here.
    pub read_identifiers: &'a [&'a str],
}

/// Generated Rust for one bound population and the names a consumer needs to use it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedPopulation {
    /// Complete generated Rust source.
    pub source: String,
    /// Name of the public function returning the population strategy.
    pub strategy_function: String,
    /// Name of the generated case struct.
    pub case_type: String,
    /// Name of the generated `Holds`/`Violated` expectation enum.
    pub expectation_type: String,
}

/// Renders deterministic proptest source for one bound population.
///
/// Cases are constructed on their side: a literal relation draws from its intervals or maps an
/// index around the excluded point, and a two-read relation draws the primary value from values
/// with a non-empty partner set and then derives the partner range from that primary value inside
/// `prop_flat_map`, so every shrink candidate is rebuilt from the current primary value. `Broad`
/// draws its side once through a non-shrinking choice, so shrinking never crosses sides. No
/// filter, assume, reject, or discard is emitted.
///
/// # Errors
///
/// Returns `InvalidStrategyIdentity` for a wrong number of identifiers or an identifier that is not
/// a unique Rust identifier distinct from [`EXPECTATION_FIELD`]; `EmptyPopulation` naming the first
/// empty requested side; and `InvalidGeneratedSyntax` or `ResourceLimitExceeded` for rendered
/// source that does not parse or exceeds the attested source limit.
// Implements: FR-009, FR-012
pub fn render_population(
    request: &PopulationRequest<'_>,
) -> Result<RenderedPopulation, StrategyDiagnostic> {
    validate_identifiers(request)?;
    let values = |side| side_values(&request.relation, request.domain, side);
    // Every requested side is computed, and refused when empty, before any source is rendered.
    let plan = match request.population {
        Population::Satisfying => Plan::Single(
            PopulationSide::Satisfying,
            values(PopulationSide::Satisfying)?,
        ),
        Population::Violating => Plan::Single(
            PopulationSide::Violating,
            values(PopulationSide::Violating)?,
        ),
        Population::Broad => Plan::Broad(
            values(PopulationSide::Satisfying)?,
            values(PopulationSide::Violating)?,
        ),
    };

    let relation_debug = format!("{:?}", request.relation);
    let domain_debug = format!("{:?}", request.domain);
    let population_debug = format!("{:?}", request.population);
    let identifiers_debug = format!("{:?}", request.read_identifiers);
    let identity = length_delimited_identity(&[
        "bound-population/v1",
        &relation_debug,
        &domain_debug,
        &population_debug,
        &identifiers_debug,
    ]);
    let digest = sha256(identity.as_bytes());
    let short = &digest[..16];
    let names = Names {
        case_type: format!("BoundCase{short}"),
        expectation_type: format!("BoundExpectation{short}"),
        identifiers: request.read_identifiers,
    };
    let strategy_function = format!(
        "bound_population_{}_{digest}",
        request.population.component()
    );
    let relation = relation_text(
        &request.relation,
        request.read_identifiers[0],
        request
            .read_identifiers
            .get(1)
            .copied()
            .unwrap_or("partner"),
    );
    let domain = format!("{}..={}", request.domain.minimum, request.domain.maximum);

    let body = match plan {
        Plan::Single(side, values) => indent(&side_expression(&values, side, &names)?, 4),
        Plan::Broad(satisfying, violating) => {
            let first_body = indent(
                &side_expression(&satisfying, PopulationSide::Satisfying, &names)?,
                8,
            );
            let second_body = indent(
                &side_expression(&violating, PopulationSide::Violating, &names)?,
                8,
            );
            let case_type = &names.case_type;
            format!(
                "    fn holds_cases() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
        use proptest::strategy::Strategy as _;\n\
{first_body}\n\
    }}\n\
\n\
    fn violated_cases() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
        use proptest::strategy::Strategy as _;\n\
{second_body}\n\
    }}\n\
\n\
    // The side is drawn once and never shrinks, so no shrink step can cross sides.\n\
    proptest::bool::ANY\n\
        .no_shrink()\n\
        .prop_flat_map(|holds| if holds {{ holds_cases() }} else {{ violated_cases() }})\n\
        .boxed()"
            )
        }
    };

    let fields = fields_source(&names);
    let population_doc = match request.population {
        Population::Satisfying => "Every case is tagged `Holds`.".to_owned(),
        Population::Violating => "Every case is tagged `Violated`.".to_owned(),
        Population::Broad => "Each case keeps the tag of the side it was drawn from; the side is \
            drawn once without shrinking."
            .to_owned(),
    };
    let case_type = &names.case_type;
    let expectation_type = &names.expectation_type;
    let source = format!(
        "#![deny(missing_docs)]\n\
//! Generated bound numeric population artifact.\n\
// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {version}; DO NOT EDIT.\n\
// Relation: {relation}; Domain: {domain}; Population: {population}\n\
\n\
/// Value the relation's generated oracle returns for one case.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub enum {expectation_type} {{\n\
    /// The relation holds for the case.\n\
    Holds,\n\
    /// The relation is violated by the case.\n\
    Violated,\n\
}}\n\
\n\
/// One complete valuation of the relation's reads and its expectation tag.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct {case_type} {{\n\
{fields}\
    /// Expectation tag for this valuation.\n\
    pub {EXPECTATION_FIELD}: {expectation_type},\n\
}}\n\
\n\
impl {case_type} {{\n\
    /// Inclusive minimum of the shared read domain.\n\
    pub const DOMAIN_MINIMUM: i64 = {minimum};\n\
    /// Inclusive maximum of the shared read domain.\n\
    pub const DOMAIN_MAXIMUM: i64 = {maximum};\n\
}}\n\
\n\
/// Builds the `{population}` population of `{relation}` over `{domain}`.\n\
///\n\
/// {population_doc}\n\
pub fn {strategy_function}() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
    use proptest::strategy::Strategy as _;\n\
{body}\n\
}}\n",
        version = env!("CARGO_PKG_VERSION"),
        population = request.population.name(),
        minimum = rust_i64(request.domain.minimum),
        maximum = rust_i64(request.domain.maximum),
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(diagnostic_with_generation(
            StrategyErrorCode::ResourceLimitExceeded,
            GenerationErrorCode::ResourceLimitExceeded,
            "generated.rust",
            "the generated population exceeds the attested source-size limit",
        ));
    }
    syn::parse_file(&source).map_err(|error| {
        diagnostic_with_generation(
            StrategyErrorCode::InvalidGeneratedSyntax,
            GenerationErrorCode::InvalidGeneratedSyntax,
            "generated.rust",
            &error.to_string(),
        )
    })?;
    Ok(RenderedPopulation {
        source,
        strategy_function,
        case_type: names.case_type,
        expectation_type: names.expectation_type,
    })
}

struct Names<'a> {
    case_type: String,
    expectation_type: String,
    identifiers: &'a [&'a str],
}

enum Plan {
    Single(PopulationSide, SideValues),
    Broad(SideValues, SideValues),
}

fn validate_identifiers(request: &PopulationRequest<'_>) -> Result<(), StrategyDiagnostic> {
    let identifiers = request.read_identifiers;
    if identifiers.len() != read_count(&request.relation) {
        return Err(diagnostic(
            StrategyErrorCode::InvalidStrategyIdentity,
            "read_identifiers",
            format!(
                "the relation has {} read(s) but {} identifier(s) were supplied",
                read_count(&request.relation),
                identifiers.len()
            ),
        ));
    }
    for (index, identifier) in identifiers.iter().enumerate() {
        if syn::parse_str::<syn::Ident>(identifier).is_err()
            || *identifier == EXPECTATION_FIELD
            || identifiers[..index].contains(identifier)
        {
            return Err(diagnostic(
                StrategyErrorCode::InvalidStrategyIdentity,
                "read_identifiers",
                format!(
                    "read identifier {identifier:?} must be a unique Rust identifier other than \
                     {EXPECTATION_FIELD:?}"
                ),
            ));
        }
    }
    Ok(())
}

fn fields_source(names: &Names<'_>) -> String {
    let roles = ["primary", "partner"];
    names
        .identifiers
        .iter()
        .zip(roles)
        .map(|(identifier, role)| {
            format!("    /// Value of the {role} read.\n    pub {identifier}: i64,\n")
        })
        .collect()
}

/// Renders one side's strategy expression, unindented, ending in `.boxed()`.
fn side_expression(
    values: &SideValues,
    side: PopulationSide,
    names: &Names<'_>,
) -> Result<String, StrategyDiagnostic> {
    let case_type = &names.case_type;
    let tag = format!("{}::{}", names.expectation_type, side.tag());
    let primary = names.identifiers[0];
    match *values {
        SideValues::Literal {
            primary: ValueSet::Interval(interval),
        } => Ok(format!(
            "{}\n    .prop_map(|primary_value| {case_type} {{\n        {primary}: primary_value,\n        {EXPECTATION_FIELD}: {tag},\n    }})\n    .boxed()",
            fixed_range(interval.minimum, interval.maximum),
        )),
        SideValues::Literal {
            primary: ValueSet::DomainExceptPoint { domain, excluded },
        } => Ok(format!(
            "{}\n    .prop_map(|primary_index| {{\n        let primary_value = if primary_index >= {excluded} {{\n            primary_index + 1\n        }} else {{\n            primary_index\n        }};\n        {case_type} {{\n            {primary}: primary_value,\n            {EXPECTATION_FIELD}: {tag},\n        }}\n    }})\n    .boxed()",
            fixed_range(domain.minimum, narrow(edge(domain.maximum, -1)?)?),
            excluded = rust_i64(excluded),
        )),
        SideValues::Correlated {
            primary: primaries,
            partner,
            domain,
        } => {
            let partner_identifier = names.identifiers[1];
            let primary_range = fixed_range(primaries.minimum, primaries.maximum);
            let construct = |partner_value: &str| {
                format!(
                    "{case_type} {{\n            {primary}: primary_value,\n            {partner_identifier}: {partner_value},\n            {EXPECTATION_FIELD}: {tag},\n        }}"
                )
            };
            let minimum = rust_i64(domain.minimum);
            let maximum = rust_i64(domain.maximum);
            let partner_draw = match partner {
                PartnerRule::EqualToPrimary => {
                    return Ok(format!(
                        "{primary_range}\n    .prop_map(|primary_value| {case_type} {{\n        {primary}: primary_value,\n        {partner_identifier}: primary_value,\n        {EXPECTATION_FIELD}: {tag},\n    }})\n    .boxed()"
                    ));
                }
                PartnerRule::DomainExceptPrimary => format!(
                    "{}.prop_map(move |partner_index| {{\n        let partner_value = if partner_index >= primary_value {{\n            partner_index + 1\n        }} else {{\n            partner_index\n        }};\n        {}\n    }})",
                    fixed_range(domain.minimum, narrow(edge(domain.maximum, -1)?)?),
                    construct("partner_value"),
                ),
                PartnerRule::AbovePrimary => partner_range(
                    "primary_value + 1",
                    &maximum,
                    domain.maximum == i64::MIN,
                    &construct("partner_value"),
                ),
                PartnerRule::AtOrAbovePrimary => partner_range(
                    "primary_value",
                    &maximum,
                    domain.maximum == i64::MIN,
                    &construct("partner_value"),
                ),
                PartnerRule::BelowPrimary => partner_range(
                    &minimum,
                    "primary_value - 1",
                    domain.minimum == i64::MIN,
                    &construct("partner_value"),
                ),
                PartnerRule::AtOrBelowPrimary => partner_range(
                    &minimum,
                    "primary_value",
                    domain.minimum == i64::MIN,
                    &construct("partner_value"),
                ),
            };
            Ok(format!(
                "{primary_range}\n    // The partner range is rebuilt from the current primary value at every shrink step.\n    .prop_flat_map(|primary_value| {{\n        {partner_draw}\n    }})\n    .boxed()"
            ))
        }
    }
}

/// A range strategy over the fixed `minimum..=maximum`.
///
/// proptest 1.5.0 builds every integer range tree with `BinarySearch::new_clamped`, which computes
/// `end - 1` and overflows when the inclusive end is `i64::MIN`. The only such non-empty range is the
/// single point `i64::MIN`, which is rendered as `Just` instead.
fn fixed_range(minimum: i64, maximum: i64) -> String {
    if maximum == i64::MIN {
        "proptest::strategy::Just(i64::MIN)".to_owned()
    } else {
        format!("({}..={})", rust_i64(minimum), rust_i64(maximum))
    }
}

/// A partner range whose bounds follow the current primary value.
///
/// When the upper end can reach `i64::MIN` (see [`fixed_range`]), the rendered code selects `Just`
/// for that single point at run time.
fn partner_range(lower: &str, upper: &str, may_end_at_minimum: bool, construct: &str) -> String {
    let draw = if may_end_at_minimum {
        format!(
            "{{\n        let partner_maximum = {upper};\n        if partner_maximum == i64::MIN {{\n            proptest::strategy::Just(i64::MIN).boxed()\n        }} else {{\n            ({lower}..=partner_maximum).boxed()\n        }}\n    }}"
        )
    } else {
        format!("({lower}..={upper})")
    };
    format!("{draw}.prop_map(move |partner_value| {{\n        {construct}\n    }})")
}

fn indent(text: &str, width: usize) -> String {
    let padding = " ".repeat(width);
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{padding}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn relation_text(relation: &Relation, primary: &str, partner: &str) -> String {
    let symbol = symbol(relation.operator());
    let other = match relation.partner() {
        Partner::Literal(literal) => literal.to_string(),
        Partner::Read => partner.to_owned(),
    };
    match relation.primary() {
        OperandPosition::Left => format!("{primary} {symbol} {other}"),
        OperandPosition::Right => format!("{other} {symbol} {primary}"),
    }
}

fn rust_i64(value: i64) -> String {
    match value {
        i64::MIN => "i64::MIN".to_owned(),
        i64::MAX => "i64::MAX".to_owned(),
        value => format!("{value}i64"),
    }
}

fn diagnostic(code: StrategyErrorCode, path: &str, message: String) -> StrategyDiagnostic {
    StrategyDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        generation_code: None,
        path: path.to_owned(),
        message,
    }
}

fn diagnostic_with_generation(
    code: StrategyErrorCode,
    generation_code: GenerationErrorCode,
    path: &str,
    message: &str,
) -> StrategyDiagnostic {
    StrategyDiagnostic {
        code,
        terminal_state: generation_code.terminal_state(),
        generation_code: Some(generation_code),
        path: path.to_owned(),
        message: message.to_owned(),
    }
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}
