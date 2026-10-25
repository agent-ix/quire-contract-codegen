// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Shared bound readers over admitted checked terms.

use super::checked_term::{bound_members, literal, literal_count, literal_integer};
use quire_contract_model::{CheckedTerm, LiteralKind};
use quire_exact::{
    DecimalType, IntegerInterval, RationalDomain, RoundingMode, TextProfile, TextType,
};

fn literal_interval(lower: &CheckedTerm, upper: &CheckedTerm) -> Option<IntegerInterval> {
    IntegerInterval::new(literal_integer(lower)?, literal_integer(upper)?).ok()
}

/// The `integer_range` member names (QSpec `positive-operation-identities.json`).
pub(crate) const INTEGER_RANGE_MEMBERS: [&str; 2] = ["min", "max"];
/// The `rational_range` member names.
pub(crate) const RATIONAL_RANGE_MEMBERS: [&str; 4] = [
    "numerator_min",
    "numerator_max",
    "denominator_min",
    "denominator_max",
];
/// The `decimal_range` member names.
pub(crate) const DECIMAL_RANGE_MEMBERS: [&str; 5] = [
    "coefficient_min",
    "coefficient_max",
    "scale_min",
    "scale_max",
    "rounding",
];
/// The `float_rounding` member names.
pub(crate) const FLOAT_ROUNDING_MEMBERS: [&str; 1] = ["rounding"];
/// The `text_bounds` member names.
pub(crate) const TEXT_BOUNDS_MEMBERS: [&str; 3] = ["min", "max", "text_profile"];
/// The `collection_bounds` member names.
pub(crate) const COLLECTION_BOUNDS_MEMBERS: [&str; 2] = ["min", "max"];

pub(crate) fn read_integer_range(members: &[CheckedTerm]) -> Option<IntegerInterval> {
    let [lower, upper] = bound_members(members, INTEGER_RANGE_MEMBERS)?;
    literal_interval(lower, upper)
}

pub(crate) fn read_rational_range(members: &[CheckedTerm]) -> Option<RationalDomain> {
    let [numerator_lower, numerator_upper, denominator_lower, denominator_upper] =
        bound_members(members, RATIONAL_RANGE_MEMBERS)?;
    RationalDomain::new(
        literal_interval(numerator_lower, numerator_upper)?,
        literal_interval(denominator_lower, denominator_upper)?,
    )
    .ok()
}

pub(crate) fn read_decimal_range(members: &[CheckedTerm]) -> Option<DecimalType> {
    let [lower, upper, min_scale, max_scale, rounding] =
        bound_members(members, DECIMAL_RANGE_MEMBERS)?;
    DecimalType::new(
        literal_integer(lower)?,
        literal_integer(upper)?,
        literal_count(min_scale)?,
        literal_count(max_scale)?,
        RoundingMode::from_code(literal(rounding, LiteralKind::Text)?)?,
    )
    .ok()
}

pub(crate) fn read_float_rounding(members: &[CheckedTerm]) -> Option<RoundingMode> {
    let [rounding] = bound_members(members, FLOAT_ROUNDING_MEMBERS)?;
    RoundingMode::from_code(literal(rounding, LiteralKind::Text)?)
}

pub(crate) fn read_text_bounds(members: &[CheckedTerm]) -> Option<TextType> {
    let [min, max, profile] = bound_members(members, TEXT_BOUNDS_MEMBERS)?;
    TextType::new(
        literal_count(min)?,
        literal_count(max)?,
        TextProfile::from_code(literal(profile, LiteralKind::Text)?)?,
    )
    .ok()
}
