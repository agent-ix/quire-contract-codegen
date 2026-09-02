//! Deterministic shaped proptest strategy generation.

use std::{collections::BTreeSet, fmt::Write as _};

use quire_contract_ir::RequirementRef;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::{
    oracle::{bounded_readable_component, generated_artifact_bundle, length_delimited_identity},
    Artifact, AttestationContext, GeneratedArtifactBundle, GenerationErrorCode,
    GenerationTerminalState,
};

/// Supported integer constraint shape for one generated strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyConstraint<'a> {
    /// Every value in one inclusive range is admissible.
    InclusiveRange {
        /// Inclusive lower bound.
        minimum: i64,
        /// Inclusive upper bound.
        maximum: i64,
    },
    /// Only the listed finite values are admissible.
    Membership {
        /// Non-empty, duplicate-free admissible values.
        values: &'a [i64],
    },
    /// A related value is the primary value plus an offset from an inclusive range.
    CorrelatedOffset {
        /// Inclusive primary lower bound.
        primary_minimum: i64,
        /// Inclusive primary upper bound.
        primary_maximum: i64,
        /// Inclusive offset lower bound.
        offset_minimum: i64,
        /// Inclusive offset upper bound.
        offset_maximum: i64,
    },
    /// A bounded domain with explicitly residual excluded values.
    ResidualExclusion {
        /// Inclusive base lower bound.
        minimum: i64,
        /// Inclusive base upper bound.
        maximum: i64,
        /// Values rejected by the residual precondition.
        excluded: &'a [i64],
    },
}

/// Campaign population emitted from a supported constraint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyCampaign {
    /// Directly shaped admissible population.
    Broad,
    /// Tagged values immediately inside and outside the supported domain.
    Boundary,
    /// One explicitly pinned state value.
    StatePinned {
        /// Pinned primary value.
        value: i64,
    },
    /// A state-preserving no-event pair.
    NoEvent {
        /// State value used for both sides of the pair.
        value: i64,
    },
}

/// Inputs for one generated strategy artifact.
pub struct StrategyRequest<'a> {
    /// Requirement identity and revision retained in generated documentation.
    pub requirement: &'a RequirementRef,
    /// Stable strategy name within the requirement.
    pub strategy_id: &'a str,
    /// Constraint shape to emit.
    pub constraint: StrategyConstraint<'a>,
    /// Campaign population to emit.
    pub campaign: StrategyCampaign,
    /// Caller-owned attestation binding for the generated artifact.
    pub attestation: AttestationContext<'a>,
}

/// Campaign population for a customer enum membership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnumStrategyCampaign<'a> {
    /// Directly shaped finite membership.
    Broad,
    /// One explicitly pinned variant.
    StatePinned {
        /// Variant that must belong to the declared membership.
        variant: &'a str,
    },
    /// A state-preserving pair containing the same variant twice.
    NoEvent {
        /// Variant that must belong to the declared membership.
        variant: &'a str,
    },
}

/// Inputs for one generated customer-enum strategy artifact.
pub struct EnumStrategyRequest<'a> {
    /// Requirement identity and revision retained in generated documentation.
    pub requirement: &'a RequirementRef,
    /// Stable strategy name within the requirement.
    pub strategy_id: &'a str,
    /// Rust path of the declared customer enum.
    pub enum_type: &'a str,
    /// Non-empty, duplicate-free admissible variant names.
    pub variants: &'a [&'a str],
    /// Campaign population to emit.
    pub campaign: EnumStrategyCampaign<'a>,
    /// Caller-owned attestation binding for the generated artifact.
    pub attestation: AttestationContext<'a>,
}

/// Stable reason a strategy could not be generated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyErrorCode {
    /// A range had reversed endpoints.
    InvalidRange,
    /// A membership was empty, duplicated, or exceeded the bounded census.
    InvalidMembership,
    /// A correlated offset could overflow at a declared endpoint.
    CorrelationOverflow,
    /// The strategy identity was empty or contained control characters.
    InvalidStrategyIdentity,
    /// The customer enum type or one of its variants was not a Rust path/identifier.
    InvalidEnumIdentity,
    /// Generated source did not parse as Rust.
    InvalidGeneratedSyntax,
    /// The requested campaign cannot satisfy its required population shape.
    UnsupportedCampaignConstraint,
    /// A proof attestation could not be validated or serialized.
    AttestationGenerationFailed,
}

impl StrategyErrorCode {
    const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::InvalidRange
            | Self::InvalidMembership
            | Self::CorrelationOverflow
            | Self::InvalidStrategyIdentity
            | Self::InvalidEnumIdentity => GenerationTerminalState::InvalidInput,
            Self::UnsupportedCampaignConstraint => GenerationTerminalState::Unsupported,
            Self::InvalidGeneratedSyntax | Self::AttestationGenerationFailed => {
                GenerationTerminalState::Inconclusive
            }
        }
    }
}

/// Structured strategy-generation failure.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StrategyDiagnostic {
    /// Stable diagnostic category.
    pub code: StrategyErrorCode,
    /// Interface-001 terminal state for this failure.
    pub terminal_state: GenerationTerminalState,
    /// Preserved attestation or syntax failure code, when one exists.
    pub generation_code: Option<GenerationErrorCode>,
    /// Stable input path associated with the failure.
    pub path: String,
    /// Human-readable detail not used as machine identity.
    pub message: String,
}

/// Generates a deterministic proptest strategy with no implicit filter fallback.
///
/// Range, membership, and correlated populations are shaped directly, so their shrink trees remain
/// inside the declared relation. Boundary and residual populations carry an explicit expected-domain
/// tag; a generated harness can therefore preserve rejection instead of converting it to success.
// Implements: FR-002
pub fn generate_i64_strategy(
    request: &StrategyRequest<'_>,
) -> Result<GeneratedArtifactBundle, StrategyDiagnostic> {
    validate_request(request)?;
    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let revision_text = revision.to_string();
    let constraint_text = format!("{:?}", request.constraint);
    let campaign_text = format!("{:?}", request.campaign);
    let identity = length_delimited_identity(&[
        requirement,
        &revision_text,
        request.strategy_id,
        &constraint_text,
        &campaign_text,
    ]);
    let suffix = sha256(identity.as_bytes());
    let function = format!(
        "strategy_{}_{}",
        bounded_readable_component(request.strategy_id),
        suffix
    );
    let case_type = format!("StrategyCase{}", &suffix[..16]);
    let domain_type = format!("ExpectedDomain{}", &suffix[..16]);
    let body = strategy_body(
        request.constraint,
        request.campaign,
        &case_type,
        &domain_type,
    );
    let source = format!(
        "#![deny(missing_docs)]\n\
//! Generated integer strategy artifact.\n\
// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Requirement: {requirement}@{revision}; Strategy: {}\n\
\n\
/// Expected contract-domain classification for one generated case.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub enum {domain_type} {{\n\
    /// The case is directly admissible.\n\
    Accepted,\n\
    /// The case must remain a reported precondition rejection.\n\
    Rejected,\n\
}}\n\
\n\
impl {domain_type} {{\n\
    /// Returns whether the generated harness must produce a precondition rejection.\n\
    #[must_use]\n\
    pub const fn expects_rejection(self) -> bool {{\n\
        matches!(self, Self::Rejected)\n\
    }}\n\
\n\
    /// Checks this generated domain expectation against a tri-state harness verdict.\n\
    pub fn verify(\n\
        self,\n\
        verdict: quire_contract_runtime::VerdictKind,\n\
    ) -> proptest::test_runner::TestCaseResult {{\n\
        let matches = match self {{\n\
            Self::Accepted => matches!(\n\
                verdict,\n\
                quire_contract_runtime::VerdictKind::Passed\n\
                    | quire_contract_runtime::VerdictKind::FailedPostcondition\n\
            ),\n\
            Self::Rejected => verdict == quire_contract_runtime::VerdictKind::RejectedPrecondition,\n\
        }};\n\
        if matches {{\n\
            Ok(())\n\
        }} else {{\n\
            Err(proptest::test_runner::TestCaseError::fail(format!(\n\
                \"generated expected-domain mismatch: expected={{self:?}} actual={{verdict:?}}\"\n\
            )))\n\
        }}\n\
    }}\n\
}}\n\
\n\
/// One generated primary/related value pair and its expected domain.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct {case_type} {{\n\
    /// Primary generated value.\n\
    pub primary: i64,\n\
    /// Related value for a correlated or no-event campaign.\n\
    pub related: Option<i64>,\n\
    /// Whether the harness must accept or reject this case.\n\
    pub expected: {domain_type},\n\
}}\n\
\n\
/// Builds generated strategy `{}`.\n\
pub fn {function}() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
    use proptest::strategy::Strategy as _;\n\
{body}\
}}\n",
        env!("CARGO_PKG_VERSION"),
        request.strategy_id,
        request.strategy_id,
    );
    syn::parse_file(&source).map_err(|error| {
        diagnostic_with_generation(
            StrategyErrorCode::InvalidGeneratedSyntax,
            Some(GenerationErrorCode::InvalidGeneratedSyntax),
            "generated.rust",
            &error.to_string(),
        )
    })?;
    let rust = artifact(format!("src/generated/{function}.rs"), source);
    generated_artifact_bundle(
        &request.attestation,
        request.requirement,
        "generate_i64_strategy",
        &function,
        identity.as_bytes(),
        "generated-rust-strategy",
        "quire.codegen.rust-strategy/v1",
        rust,
    )
    .map_err(attestation_diagnostic)
}

/// Generates a deterministic customer-enum membership strategy.
///
/// Finite enum memberships use `proptest::sample::select` directly and therefore never rely on
/// statistically improbable filtering. Pinned and no-event campaigns validate their selected
/// variant against the same declared membership.
// Implements: FR-002
pub fn generate_enum_strategy(
    request: &EnumStrategyRequest<'_>,
) -> Result<GeneratedArtifactBundle, StrategyDiagnostic> {
    validate_strategy_identity(request.strategy_id)?;
    if syn::parse_str::<syn::Path>(request.enum_type).is_err() {
        return Err(diagnostic(
            StrategyErrorCode::InvalidEnumIdentity,
            "enum_type",
            "customer enum type must be a valid Rust path",
        ));
    }
    let variants = request
        .variants
        .iter()
        .map(|variant| {
            syn::parse_str::<syn::Ident>(variant)
                .map(|_| (*variant).to_owned())
                .map_err(|_| {
                    diagnostic(
                        StrategyErrorCode::InvalidEnumIdentity,
                        "variants",
                        "every customer enum variant must be a Rust identifier",
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let unique = variants.iter().cloned().collect::<BTreeSet<_>>();
    if variants.is_empty() || variants.len() > 256 || unique.len() != variants.len() {
        return Err(diagnostic(
            StrategyErrorCode::InvalidMembership,
            "variants",
            "enum membership must contain 1 through 256 unique variants",
        ));
    }
    let selected = match request.campaign {
        EnumStrategyCampaign::Broad => None,
        EnumStrategyCampaign::StatePinned { variant }
        | EnumStrategyCampaign::NoEvent { variant } => Some(variant),
    };
    if selected.is_some_and(|variant| !request.variants.contains(&variant)) {
        return Err(diagnostic(
            StrategyErrorCode::InvalidMembership,
            "campaign.variant",
            "pinned enum variant is outside the declared membership",
        ));
    }

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let revision_text = revision.to_string();
    let variants_text = format!("{variants:?}");
    let campaign_text = format!("{:?}", request.campaign);
    let identity = length_delimited_identity(&[
        requirement,
        &revision_text,
        request.strategy_id,
        request.enum_type,
        &variants_text,
        &campaign_text,
    ]);
    let suffix = sha256(identity.as_bytes());
    let function = format!(
        "strategy_{}_{}",
        bounded_readable_component(request.strategy_id),
        suffix
    );
    let case_type = format!("EnumStrategyCase{}", &suffix[..16]);
    let members = variants
        .iter()
        .map(|variant| format!("{}::{variant}", request.enum_type))
        .collect::<Vec<_>>()
        .join(", ");
    let body = match request.campaign {
        EnumStrategyCampaign::Broad => format!(
            "    proptest::sample::select(vec![{members}])\n        .prop_map(|current| {case_type} {{ current, related: None }})\n        .boxed()"
        ),
        EnumStrategyCampaign::StatePinned { variant } => format!(
            "    proptest::strategy::Just({case_type} {{ current: {}::{variant}, related: None }}).boxed()",
            request.enum_type
        ),
        EnumStrategyCampaign::NoEvent { variant } => format!(
            "    proptest::strategy::Just({case_type} {{ current: {}::{variant}, related: Some({}::{variant}) }}).boxed()",
            request.enum_type, request.enum_type
        ),
    };
    let source = format!(
        "#![deny(missing_docs)]\n\
//! Generated customer-enum strategy artifact.\n\
// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Requirement: {requirement}@{revision}; Enum strategy: {}\n\
\n\
/// One generated customer-enum value and optional no-event partner.\n\
#[derive(Clone, Debug)]\n\
pub struct {case_type} {{\n\
    /// Primary generated enum value.\n\
    pub current: {},\n\
    /// Matching no-event value when requested.\n\
    pub related: Option<{}>,\n\
}}\n\
\n\
/// Builds generated enum strategy `{}`.\n\
pub fn {function}() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n\
    use proptest::strategy::Strategy as _;\n\
{body}\
}}\n",
        env!("CARGO_PKG_VERSION"),
        request.strategy_id,
        request.enum_type,
        request.enum_type,
        request.strategy_id,
    );
    syn::parse_file(&source).map_err(|error| {
        diagnostic_with_generation(
            StrategyErrorCode::InvalidGeneratedSyntax,
            Some(GenerationErrorCode::InvalidGeneratedSyntax),
            "generated.rust",
            &error.to_string(),
        )
    })?;
    let rust = artifact(format!("src/generated/{function}.rs"), source);
    generated_artifact_bundle(
        &request.attestation,
        request.requirement,
        "generate_enum_strategy",
        &function,
        identity.as_bytes(),
        "generated-rust-strategy",
        "quire.codegen.rust-strategy/v1",
        rust,
    )
    .map_err(attestation_diagnostic)
}

fn validate_request(request: &StrategyRequest<'_>) -> Result<(), StrategyDiagnostic> {
    validate_strategy_identity(request.strategy_id)?;
    if matches!(
        (request.constraint, request.campaign),
        (
            StrategyConstraint::CorrelatedOffset { .. },
            StrategyCampaign::StatePinned { .. }
        )
    ) {
        return Err(diagnostic(
            StrategyErrorCode::UnsupportedCampaignConstraint,
            "campaign",
            "state-pinned correlated campaigns require a related value and are unsupported",
        ));
    }
    if matches!(request.campaign, StrategyCampaign::Boundary)
        && matches!(
            request.constraint,
            StrategyConstraint::InclusiveRange {
                minimum: i64::MIN,
                maximum: i64::MAX,
            } | StrategyConstraint::ResidualExclusion {
                minimum: i64::MIN,
                maximum: i64::MAX,
                ..
            }
        )
    {
        return Err(diagnostic(
            StrategyErrorCode::UnsupportedCampaignConstraint,
            "campaign.boundary",
            "a full-width i64 range has no representable outside boundary cases",
        ));
    }
    match request.constraint {
        StrategyConstraint::InclusiveRange { minimum, maximum }
        | StrategyConstraint::ResidualExclusion {
            minimum, maximum, ..
        } if minimum > maximum => Err(diagnostic(
            StrategyErrorCode::InvalidRange,
            "constraint.range",
            "inclusive range minimum exceeds maximum",
        )),
        StrategyConstraint::Membership { values } => {
            let unique = values.iter().copied().collect::<BTreeSet<_>>();
            if values.is_empty() || values.len() > 256 || unique.len() != values.len() {
                Err(diagnostic(
                    StrategyErrorCode::InvalidMembership,
                    "constraint.values",
                    "membership must contain 1 through 256 unique values",
                ))
            } else {
                Ok(())
            }
        }
        StrategyConstraint::CorrelatedOffset {
            primary_minimum,
            primary_maximum,
            offset_minimum,
            offset_maximum,
        } => {
            if primary_minimum > primary_maximum || offset_minimum > offset_maximum {
                return Err(diagnostic(
                    StrategyErrorCode::InvalidRange,
                    "constraint.correlation",
                    "correlated primary or offset range is reversed",
                ));
            }
            if primary_minimum.checked_add(offset_minimum).is_none()
                || primary_minimum.checked_add(offset_maximum).is_none()
                || primary_maximum.checked_add(offset_minimum).is_none()
                || primary_maximum.checked_add(offset_maximum).is_none()
            {
                Err(diagnostic(
                    StrategyErrorCode::CorrelationOverflow,
                    "constraint.correlation",
                    "a correlated endpoint would overflow i64",
                ))
            } else {
                Ok(())
            }
        }
        StrategyConstraint::ResidualExclusion {
            minimum,
            maximum,
            excluded,
        } => {
            let unique = excluded.iter().copied().collect::<BTreeSet<_>>();
            if unique.len() != excluded.len()
                || excluded
                    .iter()
                    .any(|value| *value < minimum || *value > maximum)
            {
                Err(diagnostic(
                    StrategyErrorCode::InvalidMembership,
                    "constraint.excluded",
                    "residual exclusions must be unique members of the base range",
                ))
            } else {
                Ok(())
            }
        }
        StrategyConstraint::InclusiveRange { .. } => Ok(()),
    }
}

fn validate_strategy_identity(value: &str) -> Result<(), StrategyDiagnostic> {
    if value.is_empty() || value.chars().any(|character| character.is_control()) {
        Err(diagnostic(
            StrategyErrorCode::InvalidStrategyIdentity,
            "strategy_id",
            "strategy identity must be non-empty and contain no control characters",
        ))
    } else {
        Ok(())
    }
}

fn strategy_body(
    constraint: StrategyConstraint<'_>,
    campaign: StrategyCampaign,
    case_type: &str,
    domain_type: &str,
) -> String {
    match campaign {
        StrategyCampaign::StatePinned { value } => {
            let expected = expected_domain(constraint, value, None, domain_type);
            format!(
                "    proptest::strategy::Just({case_type} {{ primary: {value}, related: None, expected: {expected} }}).boxed()"
            )
        }
        StrategyCampaign::NoEvent { value } => {
            let expected = expected_domain(constraint, value, Some(value), domain_type);
            format!(
                "    proptest::strategy::Just({case_type} {{ primary: {value}, related: Some({value}), expected: {expected} }}).boxed()"
            )
        }
        StrategyCampaign::Broad => broad_body(constraint, case_type, domain_type),
        StrategyCampaign::Boundary => boundary_body(constraint, case_type, domain_type),
    }
}

fn broad_body(constraint: StrategyConstraint<'_>, case_type: &str, domain_type: &str) -> String {
    match constraint {
        StrategyConstraint::InclusiveRange { minimum, maximum } => format!(
            "    ({minimum}i64..={maximum}i64)\n        .prop_map(|primary| {case_type} {{ primary, related: None, expected: {domain_type}::Accepted }})\n        .boxed()"
        ),
        StrategyConstraint::Membership { values } => format!(
            "    proptest::sample::select(vec!{})\n        .prop_map(|primary| {case_type} {{ primary, related: None, expected: {domain_type}::Accepted }})\n        .boxed()",
            integer_list(values)
        ),
        StrategyConstraint::CorrelatedOffset {
            primary_minimum,
            primary_maximum,
            offset_minimum,
            offset_maximum,
        } => format!(
            "    ({primary_minimum}i64..={primary_maximum}i64, {offset_minimum}i64..={offset_maximum}i64)\n        .prop_map(|(primary, offset)| {case_type} {{ primary, related: Some(primary + offset), expected: {domain_type}::Accepted }})\n        .boxed()"
        ),
        StrategyConstraint::ResidualExclusion {
            minimum,
            maximum,
            excluded,
        } => {
            let excluded = integer_list(excluded);
            format!(
                "    ({minimum}i64..={maximum}i64)\n        .prop_map(|primary| {{\n            let expected = if {excluded}.contains(&primary) {{ {domain_type}::Rejected }} else {{ {domain_type}::Accepted }};\n            {case_type} {{ primary, related: None, expected }}\n        }})\n        .boxed()"
            )
        }
    }
}

fn boundary_body(constraint: StrategyConstraint<'_>, case_type: &str, domain_type: &str) -> String {
    if let StrategyConstraint::CorrelatedOffset {
        primary_minimum,
        primary_maximum,
        offset_minimum,
        offset_maximum,
    } = constraint
    {
        let mut cases = BTreeSet::new();
        for primary in [primary_minimum, primary_maximum] {
            for offset in [offset_minimum, offset_maximum] {
                if let Some(related) = primary.checked_add(offset) {
                    cases.insert((primary, related));
                }
            }
        }
        if let Some(primary) = primary_minimum.checked_sub(1) {
            if let Some(related) = primary.checked_add(offset_minimum) {
                cases.insert((primary, related));
            }
        }
        if let Some(primary) = primary_maximum.checked_add(1) {
            if let Some(related) = primary.checked_add(offset_maximum) {
                cases.insert((primary, related));
            }
        }
        if let Some(related) = primary_minimum
            .checked_add(offset_minimum)
            .and_then(|value| value.checked_sub(1))
        {
            cases.insert((primary_minimum, related));
        }
        if let Some(related) = primary_maximum
            .checked_add(offset_maximum)
            .and_then(|value| value.checked_add(1))
        {
            cases.insert((primary_maximum, related));
        }
        let entries = cases
            .into_iter()
            .map(|(primary, related)| {
                let expected = expected_domain(constraint, primary, Some(related), domain_type);
                format!(
                    "{case_type} {{ primary: {primary}, related: Some({related}), expected: {expected} }}"
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        return format!("    proptest::sample::select(vec![{entries}]).boxed()");
    }

    let cases = match constraint {
        StrategyConstraint::InclusiveRange { minimum, maximum } => {
            let mut cases = BTreeSet::new();
            cases.insert(minimum);
            cases.insert(maximum);
            if let Some(value) = minimum.checked_sub(1) {
                cases.insert(value);
            }
            if let Some(value) = maximum.checked_add(1) {
                cases.insert(value);
            }
            cases
        }
        StrategyConstraint::ResidualExclusion {
            minimum,
            maximum,
            excluded,
        } => {
            let mut cases = BTreeSet::new();
            cases.insert(minimum);
            cases.insert(maximum);
            if let Some(value) = minimum.checked_sub(1) {
                cases.insert(value);
            }
            if let Some(value) = maximum.checked_add(1) {
                cases.insert(value);
            }
            for value in excluded {
                cases.insert(*value);
                if let Some(adjacent) = value.checked_sub(1) {
                    cases.insert(adjacent);
                }
                if let Some(adjacent) = value.checked_add(1) {
                    cases.insert(adjacent);
                }
            }
            cases
        }
        StrategyConstraint::Membership { values } => {
            let mut cases = BTreeSet::new();
            cases.extend(values.iter().copied());
            for value in values {
                if let Some(adjacent) = value.checked_sub(1) {
                    cases.insert(adjacent);
                }
                if let Some(adjacent) = value.checked_add(1) {
                    cases.insert(adjacent);
                }
            }
            cases
        }
        StrategyConstraint::CorrelatedOffset { .. } => BTreeSet::new(),
    };
    let entries = cases
        .into_iter()
        .map(|primary| {
            let related = None;
            let expected = expected_domain(constraint, primary, related, domain_type);
            let related = related
                .map(|value| format!("Some({value})"))
                .unwrap_or_else(|| "None".to_owned());
            format!(
                "{case_type} {{ primary: {primary}, related: {related}, expected: {expected} }}"
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("    proptest::sample::select(vec![{entries}]).boxed()")
}

fn expected_domain(
    constraint: StrategyConstraint<'_>,
    primary: i64,
    related: Option<i64>,
    domain_type: &str,
) -> String {
    let accepted = match constraint {
        StrategyConstraint::InclusiveRange { minimum, maximum } => {
            (minimum..=maximum).contains(&primary)
        }
        StrategyConstraint::Membership { values } => values.contains(&primary),
        StrategyConstraint::CorrelatedOffset {
            primary_minimum,
            primary_maximum,
            offset_minimum,
            offset_maximum,
        } => related.is_some_and(|related| {
            (primary_minimum..=primary_maximum).contains(&primary)
                && primary.checked_add(offset_minimum).is_some_and(|minimum| {
                    primary
                        .checked_add(offset_maximum)
                        .is_some_and(|maximum| (minimum..=maximum).contains(&related))
                })
        }),
        StrategyConstraint::ResidualExclusion {
            minimum,
            maximum,
            excluded,
        } => (minimum..=maximum).contains(&primary) && !excluded.contains(&primary),
    };
    format!(
        "{domain_type}::{}",
        if accepted { "Accepted" } else { "Rejected" }
    )
}

fn integer_list(values: &[i64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
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
    StrategyDiagnostic {
        code,
        terminal_state: generation_code
            .map(GenerationErrorCode::terminal_state)
            .unwrap_or_else(|| code.terminal_state()),
        generation_code,
        path: path.to_owned(),
        message: message.to_owned(),
    }
}

fn attestation_diagnostic(code: GenerationErrorCode) -> StrategyDiagnostic {
    diagnostic_with_generation(
        StrategyErrorCode::AttestationGenerationFailed,
        Some(code),
        "generated.attestation",
        "the strategy proof attestation could not be emitted",
    )
}

fn artifact(path: String, contents: String) -> Artifact {
    let sha256 = sha256(contents.as_bytes());
    Artifact {
        path,
        contents,
        sha256,
    }
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}
