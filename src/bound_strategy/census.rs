//! Domain and relation boundary census for one admitted relation (FR-010).
//!
//! The census is computed with checked 128-bit arithmetic. An edge value outside `i64` is listed as
//! an [`UnrepresentableEdge`] rather than clamped, wrapped, or dropped. Generated Rust constants are
//! the only carrier of the arrays; nothing here serializes a census.

use std::{collections::BTreeSet, fmt::Write as _};

use super::relation::{Domain, Partner, Relation};
use crate::{GenerationErrorCode, GenerationTerminalState, StrategyDiagnostic, StrategyErrorCode};

/// Value the clause's oracle returns for one in-domain census case.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CensusTag {
    /// The relation holds for the case.
    Holds,
    /// The relation is violated by the case.
    Violated,
}

/// One in-domain census case: a complete valuation of the relation's reads and its tag.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CensusCase {
    /// Primary read value.
    pub primary: i64,
    /// Partner read value, present only for a two-read relation.
    pub partner: Option<i64>,
    /// Evaluation of the relation for this valuation.
    pub tag: CensusTag,
}

/// One untagged out-of-domain edge case; at least one value lies outside the domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OutOfDomainCase {
    /// Primary read value.
    pub primary: i64,
    /// Partner read value, present only for a two-read relation.
    pub partner: Option<i64>,
}

/// Read whose edge value is unrepresentable.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CensusRead {
    /// The primary read.
    Primary,
    /// The partner read.
    Partner,
}

/// Edge an unrepresentable value is adjacent to.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CensusEdge {
    /// The domain minimum.
    Minimum,
    /// The domain maximum.
    Maximum,
    /// The relation's literal `k`.
    Literal,
}

/// Side of the edge the unrepresentable value lies on.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EdgeDirection {
    /// The edge value minus one.
    Below,
    /// The edge value plus one.
    Above,
}

/// An out-of-domain edge value that lies outside the `i64` range.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnrepresentableEdge {
    /// Read the value would have been assigned to.
    pub read: CensusRead,
    /// Edge the value is adjacent to.
    pub edge: CensusEdge,
    /// Side of the edge.
    pub direction: EdgeDirection,
}

/// The three census arrays of one admitted relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryCensus {
    relation: Relation,
    in_domain: Vec<CensusCase>,
    out_of_domain: Vec<OutOfDomainCase>,
    unrepresentable_edges: Vec<UnrepresentableEdge>,
}

impl BoundaryCensus {
    /// Relation the census was computed for.
    #[must_use]
    pub const fn relation(&self) -> Relation {
        self.relation
    }

    /// In-domain census ordered by primary then partner value, whether or not `Boundary` is
    /// admissible for it.
    #[must_use]
    pub fn in_domain(&self) -> &[CensusCase] {
        &self.in_domain
    }

    /// Untagged out-of-domain cases ordered by primary then partner value.
    #[must_use]
    pub fn out_of_domain(&self) -> &[OutOfDomainCase] {
        &self.out_of_domain
    }

    /// Unrepresentable edge values ordered by read, edge, then direction.
    #[must_use]
    pub fn unrepresentable_edges(&self) -> &[UnrepresentableEdge] {
        &self.unrepresentable_edges
    }

    /// The `Boundary` population, refused with `UnsupportedCampaignConstraint` when the in-domain
    /// census lacks a `Holds` case or a `Violated` case.
    pub fn boundary_population(&self) -> Result<&[CensusCase], StrategyDiagnostic> {
        let holds = self
            .in_domain
            .iter()
            .any(|case| case.tag == CensusTag::Holds);
        let violated = self
            .in_domain
            .iter()
            .any(|case| case.tag == CensusTag::Violated);
        if holds && violated {
            Ok(&self.in_domain)
        } else {
            Err(diagnostic(
                StrategyErrorCode::UnsupportedCampaignConstraint,
                "campaign.boundary",
                "the in-domain boundary census must contain Holds and Violated cases",
            ))
        }
    }
}

/// Computes the FR-010 census for `relation` over `domain`.
///
/// Fails only for a reversed domain. A single-tagged in-domain census is still returned, so the
/// out-of-domain and unrepresentable-edge arrays remain available; the refusal is reported by
/// [`BoundaryCensus::boundary_population`].
// Implements: FR-010
pub fn compute_census(
    relation: Relation,
    domain: Domain,
) -> Result<BoundaryCensus, StrategyDiagnostic> {
    if domain.minimum > domain.maximum {
        return Err(diagnostic(
            StrategyErrorCode::InvalidRange,
            "domain",
            "inclusive domain minimum exceeds maximum",
        ));
    }
    let minimum = i128::from(domain.minimum);
    let maximum = i128::from(domain.maximum);
    let mut edge_candidates = vec![minimum, minimum + 1, maximum - 1, maximum];
    if let Partner::Literal(literal) = relation.partner() {
        let literal = i128::from(literal);
        edge_candidates.extend([literal - 1, literal, literal + 1]);
    }
    let edges = edge_candidates
        .into_iter()
        .filter(|value| domain.contains(*value))
        .filter_map(|value| i64::try_from(value).ok())
        .collect::<BTreeSet<_>>();

    let mut in_domain = BTreeSet::new();
    let mut out_of_domain = BTreeSet::new();
    let mut unrepresentable = BTreeSet::new();
    let below_minimum = (minimum - 1, CensusEdge::Minimum, EdgeDirection::Below);
    let above_maximum = (maximum + 1, CensusEdge::Maximum, EdgeDirection::Above);

    match relation.partner() {
        Partner::Literal(literal) => {
            for primary in &edges {
                in_domain.insert(tagged(relation, *primary, None));
            }
            let literal = i128::from(literal);
            let mut candidates = vec![below_minimum, above_maximum];
            for (value, direction) in [
                (literal - 1, EdgeDirection::Below),
                (literal + 1, EdgeDirection::Above),
            ] {
                if !domain.contains(value) {
                    candidates.push((value, CensusEdge::Literal, direction));
                }
            }
            for (value, edge, direction) in candidates {
                match i64::try_from(value) {
                    Ok(primary) => {
                        out_of_domain.insert(OutOfDomainCase {
                            primary,
                            partner: None,
                        });
                    }
                    Err(_) => {
                        unrepresentable.insert(UnrepresentableEdge {
                            read: CensusRead::Primary,
                            edge,
                            direction,
                        });
                    }
                }
            }
        }
        Partner::Read => {
            for primary in &edges {
                let value = i128::from(*primary);
                for partner in [value - 1, value, value + 1] {
                    if domain.contains(partner) {
                        if let Ok(partner) = i64::try_from(partner) {
                            in_domain.insert(tagged(relation, *primary, Some(partner)));
                        }
                    }
                }
            }
            for (value, edge, direction) in [below_minimum, above_maximum] {
                match i64::try_from(value) {
                    Ok(partner) => {
                        for primary in &edges {
                            out_of_domain.insert(OutOfDomainCase {
                                primary: *primary,
                                partner: Some(partner),
                            });
                        }
                    }
                    Err(_) => {
                        unrepresentable.insert(UnrepresentableEdge {
                            read: CensusRead::Partner,
                            edge,
                            direction,
                        });
                    }
                }
            }
            for ((value, edge, direction), partner) in [
                (below_minimum, domain.minimum),
                (above_maximum, domain.maximum),
            ] {
                match i64::try_from(value) {
                    Ok(primary) => {
                        out_of_domain.insert(OutOfDomainCase {
                            primary,
                            partner: Some(partner),
                        });
                    }
                    Err(_) => {
                        unrepresentable.insert(UnrepresentableEdge {
                            read: CensusRead::Primary,
                            edge,
                            direction,
                        });
                    }
                }
            }
        }
    }

    Ok(BoundaryCensus {
        relation,
        in_domain: in_domain.into_iter().collect(),
        out_of_domain: out_of_domain.into_iter().collect(),
        unrepresentable_edges: unrepresentable.into_iter().collect(),
    })
}

fn tagged(relation: Relation, primary: i64, partner: Option<i64>) -> CensusCase {
    let holds = relation.evaluate(primary, partner) == Some(true);
    CensusCase {
        primary,
        partner,
        tag: if holds {
            CensusTag::Holds
        } else {
            CensusTag::Violated
        },
    }
}

/// Generated item names for one rendered census.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CensusNames<'a> {
    /// ASCII-alphanumeric suffix appended to every generated item name; may be empty.
    pub item_suffix: &'a str,
    /// Field identifier of each read: the primary read, then the partner read when present.
    pub fields: &'a [&'a str],
}

/// Renders the untagged out-of-domain and unrepresentable-edge constants.
///
/// Emitted in every bundle for a non-refused population request, including a `Satisfying`,
/// `Violating`, or `Broad` request for a relation whose `Boundary` population is refused. A refused
/// request yields a diagnostic and no rendered source.
// Implements: FR-010
pub fn render_edge_constants(
    census: &BoundaryCensus,
    names: CensusNames<'_>,
) -> Result<String, StrategyDiagnostic> {
    render(census, names, None)
}

/// Renders the tagged in-domain census together with the edge constants.
///
/// Refuses with `UnsupportedCampaignConstraint` when the in-domain census is single-tagged.
// Implements: FR-010
pub fn render_boundary_constants(
    census: &BoundaryCensus,
    names: CensusNames<'_>,
) -> Result<String, StrategyDiagnostic> {
    let population = census.boundary_population()?;
    render(census, names, Some(population))
}

fn render(
    census: &BoundaryCensus,
    names: CensusNames<'_>,
    population: Option<&[CensusCase]>,
) -> Result<String, StrategyDiagnostic> {
    validate_names(census.relation, names)?;
    let suffix = names.item_suffix;
    let constant_suffix = if suffix.is_empty() {
        String::new()
    } else {
        format!("_{}", suffix.to_ascii_uppercase())
    };
    let primary = names.fields[0];
    let partner = names.fields.get(1).copied();
    let mut source = String::new();
    let _ = write!(
        source,
        "/// Domain edge an unrepresentable edge value is adjacent to.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub enum CensusEdge{suffix} {{\n\
    /// The domain minimum.\n\
    Minimum,\n\
    /// The domain maximum.\n\
    Maximum,\n\
    /// The relation's literal.\n\
    Literal,\n\
}}\n\
\n\
/// Side of the edge an unrepresentable edge value lies on.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub enum EdgeDirection{suffix} {{\n\
    /// The edge value minus one.\n\
    Below,\n\
    /// The edge value plus one.\n\
    Above,\n\
}}\n\
\n\
/// An out-of-domain edge value outside the `i64` range, listed instead of clamped or wrapped.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct UnrepresentableEdge{suffix} {{\n\
    /// Field identifier of the read the value belongs to.\n\
    pub read: &'static str,\n\
    /// Edge the value is adjacent to.\n\
    pub edge: CensusEdge{suffix},\n\
    /// Side of the edge.\n\
    pub direction: EdgeDirection{suffix},\n\
}}\n\
\n\
/// One untagged out-of-domain edge case; at least one value lies outside the domain.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct OutOfDomainCase{suffix} {{\n\
{fields}\
}}\n",
        fields = field_declarations(primary, partner),
    );
    if let Some(population) = population {
        let _ = write!(
            source,
            "\n\
/// Value the clause's oracle returns for one in-domain census case.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub enum CensusTag{suffix} {{\n\
    /// The relation holds.\n\
    Holds,\n\
    /// The relation is violated.\n\
    Violated,\n\
}}\n\
\n\
/// One in-domain boundary census case and its oracle tag.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct CensusCase{suffix} {{\n\
{fields}\
    /// Value the clause's oracle returns for this case.\n\
    pub tag: CensusTag{suffix},\n\
}}\n\
\n\
/// In-domain boundary census, ordered by primary then partner value.\n\
pub const IN_DOMAIN_CENSUS{constant_suffix}: [CensusCase{suffix}; {length}] = [{entries}];\n",
            fields = field_declarations(primary, partner),
            length = population.len(),
            entries = population
                .iter()
                .map(|case| {
                    let tag = match case.tag {
                        CensusTag::Holds => "Holds",
                        CensusTag::Violated => "Violated",
                    };
                    format!(
                        "\n    CensusCase{suffix} {{ {}, tag: CensusTag{suffix}::{tag} }},",
                        field_values(primary, partner, case.primary, case.partner)
                    )
                })
                .chain(trailing_newline(population.len()))
                .collect::<String>(),
        );
    }
    let out_of_domain = census.out_of_domain();
    let unrepresentable = census.unrepresentable_edges();
    let _ = write!(
        source,
        "\n\
/// Out-of-domain edge cases for consumer domain-admission tests, ordered by primary then partner.\n\
pub const OUT_OF_DOMAIN_CASES{constant_suffix}: [OutOfDomainCase{suffix}; {ood_length}] = [{ood_entries}];\n\
\n\
/// Edge values outside the `i64` range, ordered by read, edge, then direction.\n\
pub const UNREPRESENTABLE_EDGES{constant_suffix}: [UnrepresentableEdge{suffix}; {edge_length}] = [{edge_entries}];\n",
        ood_length = out_of_domain.len(),
        ood_entries = out_of_domain
            .iter()
            .map(|case| format!(
                "\n    OutOfDomainCase{suffix} {{ {} }},",
                field_values(primary, partner, case.primary, case.partner)
            ))
            .chain(trailing_newline(out_of_domain.len()))
            .collect::<String>(),
        edge_length = unrepresentable.len(),
        edge_entries = unrepresentable
            .iter()
            .map(|edge| {
                let read = match edge.read {
                    CensusRead::Primary => primary,
                    CensusRead::Partner => partner.unwrap_or(primary),
                };
                let kind = match edge.edge {
                    CensusEdge::Minimum => "Minimum",
                    CensusEdge::Maximum => "Maximum",
                    CensusEdge::Literal => "Literal",
                };
                let direction = match edge.direction {
                    EdgeDirection::Below => "Below",
                    EdgeDirection::Above => "Above",
                };
                format!(
                    "\n    UnrepresentableEdge{suffix} {{ read: \"{read}\", edge: CensusEdge{suffix}::{kind}, direction: EdgeDirection{suffix}::{direction} }},"
                )
            })
            .chain(trailing_newline(unrepresentable.len()))
            .collect::<String>(),
    );
    syn::parse_file(&source).map_err(|error| {
        diagnostic_with_generation(
            StrategyErrorCode::InvalidGeneratedSyntax,
            Some(GenerationErrorCode::InvalidGeneratedSyntax),
            "generated.rust",
            &error.to_string(),
        )
    })?;
    Ok(source)
}

fn trailing_newline(length: usize) -> Option<String> {
    (length > 0).then(|| "\n".to_owned())
}

fn field_declarations(primary: &str, partner: Option<&str>) -> String {
    let mut fields = format!("    /// Primary read value.\n    pub {primary}: i64,\n");
    if let Some(partner) = partner {
        let _ = write!(
            fields,
            "    /// Partner read value.\n    pub {partner}: i64,\n"
        );
    }
    fields
}

fn field_values(
    primary: &str,
    partner: Option<&str>,
    primary_value: i64,
    partner_value: Option<i64>,
) -> String {
    let mut values = format!("{primary}: {}", integer(primary_value));
    if let (Some(partner), Some(value)) = (partner, partner_value) {
        let _ = write!(values, ", {partner}: {}", integer(value));
    }
    values
}

fn integer(value: i64) -> String {
    match value {
        i64::MIN => "i64::MIN".to_owned(),
        i64::MAX => "i64::MAX".to_owned(),
        value => value.to_string(),
    }
}

fn validate_names(relation: Relation, names: CensusNames<'_>) -> Result<(), StrategyDiagnostic> {
    if !names
        .item_suffix
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        return Err(diagnostic(
            StrategyErrorCode::InvalidStrategyIdentity,
            "names.item_suffix",
            "the generated item suffix must be ASCII alphanumeric",
        ));
    }
    let expected = if relation.has_partner_read() { 2 } else { 1 };
    let unique = names.fields.iter().collect::<BTreeSet<_>>();
    let valid_field = |field: &&str| {
        field
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_lowercase() || first == '_')
            && field.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
            })
            && *field != "_"
            && *field != "tag"
            && syn::parse_str::<syn::Ident>(field).is_ok()
    };
    if names.fields.len() != expected
        || unique.len() != expected
        || !names.fields.iter().all(valid_field)
    {
        return Err(diagnostic(
            StrategyErrorCode::InvalidStrategyIdentity,
            "names.fields",
            "one distinct snake-case Rust identifier other than `tag` is required per read",
        ));
    }
    Ok(())
}

fn diagnostic(code: StrategyErrorCode, path: &str, message: &str) -> StrategyDiagnostic {
    diagnostic_with_generation(code, None, path, message)
}

fn diagnostic_with_generation(
    code: StrategyErrorCode,
    generation_code: Option<GenerationErrorCode>,
    path: &str,
    message: &str,
) -> StrategyDiagnostic {
    let terminal_state = match (generation_code, code) {
        (Some(generation), _) => generation.terminal_state(),
        (None, StrategyErrorCode::InvalidRange | StrategyErrorCode::InvalidStrategyIdentity) => {
            GenerationTerminalState::InvalidInput
        }
        (None, _) => GenerationTerminalState::Unsupported,
    };
    StrategyDiagnostic {
        code,
        terminal_state,
        generation_code,
        path: path.to_owned(),
        message: message.to_owned(),
    }
}
