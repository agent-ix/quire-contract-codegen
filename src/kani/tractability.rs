//! The tractability decider and its measured table (FR-056).
//!
//! The decision sits behind one narrow interface, [`TractabilityDecider`], whose only input is
//! the [`ClauseFeatures`] value. The measured-table lookup is its one production implementation.
//! The table is data loaded from a file; no operation is named in code.

use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Deserializer, Serialize};

/// CG's generator version: the version of this crate.
pub const GENERATOR_VERSION: &str = env!("CARGO_PKG_VERSION");

/// CG's kernel version: the version of the `quire-exact` crate this build links, read from the
/// lock file at build time.
pub const KERNEL_VERSION: &str = env!("CG_QUIRE_EXACT_VERSION");

fn generator_version() -> String {
    GENERATOR_VERSION.to_owned()
}

fn kernel_version() -> String {
    KERNEL_VERSION.to_owned()
}

/// The smallest bit count `n` such that every value of `[lower, upper]` is representable in `n`
/// bits: unsigned `bits(upper)` when `lower >= 0`, else two's complement signed.
///
/// The native carrier is signed, so `[0, 2^k]` counts as `k + 1` bits.
#[must_use]
pub fn interval_bits(lower: i128, upper: i128) -> u32 {
    let unsigned_bits = |value: u128| u128::BITS - value.leading_zeros();
    if lower >= 0 {
        // A non-negative lower bound is at most `upper`; `upper` is non-negative.
        unsigned_bits(upper.unsigned_abs())
    } else {
        // `!lower` is `-lower - 1`, the magnitude two's complement needs for a negative bound.
        let negative = unsigned_bits((!lower).unsigned_abs());
        let positive = if upper >= 0 {
            unsigned_bits(upper.unsigned_abs())
        } else {
            0
        };
        negative.max(positive) + 1
    }
}

/// One operation of a clause and its width after range propagation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationWidth {
    /// The catalog operation id.
    pub operation_id: String,
    /// The largest bit width, over its operands, of the interval range propagation yields.
    pub bits: u32,
}

/// An input's inclusive bounds.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputBound {
    /// Inclusive lower bound.
    pub lower: i128,
    /// Inclusive upper bound.
    pub upper: i128,
    /// Whether the input is a Boolean.
    #[serde(default)]
    pub boolean: bool,
}

/// The only value a [`TractabilityDecider`] receives.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClauseFeatures {
    /// The ordered operations of the clause.
    pub operations: Vec<OperationWidth>,
    /// The input bounds.
    pub inputs: Vec<InputBound>,
    /// The collection bound, absent when the clause has none.
    #[serde(default)]
    pub collection_bound: Option<u64>,
    /// CG's generator version.
    #[serde(default = "generator_version")]
    pub generator_version: String,
    /// CG's kernel version.
    #[serde(default = "kernel_version")]
    pub kernel_version: String,
}

impl ClauseFeatures {
    /// Features at CG's own generator and kernel versions.
    #[must_use]
    pub fn new(
        operations: Vec<OperationWidth>,
        inputs: Vec<InputBound>,
        collection_bound: Option<u64>,
    ) -> Self {
        Self {
            operations,
            inputs,
            collection_bound,
            generator_version: generator_version(),
            kernel_version: kernel_version(),
        }
    }
}

/// A probe's measured verdict.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RowVerdict {
    /// The probe finished within the ceiling.
    Tractable,
    /// The probe did not finish within the ceiling.
    NotTractable,
}

fn required<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
}

/// One measured probe.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TractabilityRow {
    /// The catalog operation ids the probe exercised.
    pub operation_ids: Vec<String>,
    /// The largest operand bit width the probe covered.
    pub max_operand_bits: u32,
    /// The collection bound of the probe, `null` when it had none. The key is required.
    #[serde(deserialize_with = "required")]
    pub collection_bound: Option<u64>,
    /// The measured verdict.
    pub verdict: RowVerdict,
    /// Whole wall-clock seconds the probe took.
    pub wall_seconds: u64,
    /// Peak memory of the probe in bytes.
    pub peak_bytes: u64,
    /// The Kani version the probe ran under.
    pub kani_version: String,
    /// The CBMC version the probe ran under.
    pub cbmc_version: String,
    /// The solver version the probe ran under.
    pub solver_version: String,
    /// The generator version the probe ran against.
    pub generator_version: String,
    /// The kernel version the probe ran against.
    pub kernel_version: String,
}

impl TractabilityRow {
    fn is_current(&self, features: &ClauseFeatures) -> bool {
        self.generator_version == features.generator_version
            && self.kernel_version == features.kernel_version
    }
}

/// Why a clause is not run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotTractable {
    /// The refused operation id.
    pub operation_id: String,
    /// Its width in bits.
    pub width_bits: u32,
}

/// A decider's answer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Generate and execute the clause.
    Run,
    /// Refuse the clause before generation.
    Refuse {
        /// The refused operation and its width.
        reason: NotTractable,
        /// The measured row that justifies the refusal.
        evidence_row: Box<TractabilityRow>,
    },
}

/// The tractability decider: the clause features in, `Run` or `Refuse` out.
///
/// The parameter is the feature value alone, so an implementation cannot reach the clause body,
/// the package or the filesystem (FR-056-AC-1).
pub trait TractabilityDecider {
    /// Decide one clause.
    fn decide(&self, features: &ClauseFeatures) -> Decision;
}

/// A table that fails to load.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TractabilityLoadError {
    /// The file is missing or unreadable.
    Unreadable {
        /// The requested path.
        path: PathBuf,
        /// The operating system's message.
        message: String,
    },
    /// The file is not a JSON array.
    NotATable {
        /// The requested path.
        path: PathBuf,
        /// The parser's message.
        message: String,
    },
    /// One row is malformed.
    MalformedRow {
        /// The requested path.
        path: PathBuf,
        /// Zero-based row index.
        row: usize,
        /// The parser's message.
        message: String,
    },
}

impl fmt::Display for TractabilityLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unreadable { path, message } => {
                write!(
                    formatter,
                    "tractability table {}: {message}",
                    path.display()
                )
            }
            Self::NotATable { path, message } => write!(
                formatter,
                "tractability table {} is not a JSON array of rows: {message}",
                path.display()
            ),
            Self::MalformedRow { path, row, message } => write!(
                formatter,
                "tractability table {} row {row}: {message}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for TractabilityLoadError {}

/// The measured table: one row per measured probe, in file order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TractabilityTable {
    rows: Vec<TractabilityRow>,
}

impl TractabilityTable {
    /// The empty table: every clause runs.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// A table of `rows`.
    #[must_use]
    pub fn from_rows(rows: Vec<TractabilityRow>) -> Self {
        Self { rows }
    }

    /// The rows, in file order.
    #[must_use]
    pub fn rows(&self) -> &[TractabilityRow] {
        &self.rows
    }

    /// Load the table at `path`: a JSON array of rows. A missing file, an unreadable file or any
    /// malformed row is an error naming the path (and the row); nothing is skipped.
    ///
    /// # Errors
    ///
    /// [`TractabilityLoadError`] as described above.
    pub fn load(path: &Path) -> Result<Self, TractabilityLoadError> {
        let bytes = std::fs::read(path).map_err(|error| TractabilityLoadError::Unreadable {
            path: path.to_owned(),
            message: error.to_string(),
        })?;
        Self::parse(path, &bytes)
    }

    fn parse(path: &Path, bytes: &[u8]) -> Result<Self, TractabilityLoadError> {
        let values: Vec<serde_json::Value> =
            serde_json::from_slice(bytes).map_err(|error| TractabilityLoadError::NotATable {
                path: path.to_owned(),
                message: error.to_string(),
            })?;
        let rows = values
            .into_iter()
            .enumerate()
            .map(|(row, value)| {
                serde_json::from_value(value).map_err(|error| TractabilityLoadError::MalformedRow {
                    path: path.to_owned(),
                    row,
                    message: error.to_string(),
                })
            })
            .collect::<Result<Vec<TractabilityRow>, _>>()?;
        Ok(Self { rows })
    }
}

impl TractabilityDecider for TractabilityTable {
    /// Refuse when some operation of the clause appears in a current `not_tractable` row whose
    /// `max_operand_bits` is at least that operation's width. Of several qualifying rows the
    /// smallest `max_operand_bits` wins, then file order, then the clause's operation order.
    fn decide(&self, features: &ClauseFeatures) -> Decision {
        let mut best: Option<(&TractabilityRow, &OperationWidth)> = None;
        for row in &self.rows {
            if row.verdict != RowVerdict::NotTractable || !row.is_current(features) {
                continue;
            }
            let Some(operation) = features.operations.iter().find(|operation| {
                row.max_operand_bits >= operation.bits
                    && row.operation_ids.contains(&operation.operation_id)
            }) else {
                continue;
            };
            // A later row replaces the best only with strictly fewer bits, so file order breaks
            // ties.
            if best.is_none_or(|(held, _)| row.max_operand_bits < held.max_operand_bits) {
                best = Some((row, operation));
            }
        }
        best.map_or(Decision::Run, |(row, operation)| Decision::Refuse {
            reason: NotTractable {
                operation_id: operation.operation_id.clone(),
                width_bits: operation.bits,
            },
            evidence_row: Box::new(row.clone()),
        })
    }
}

/// The options of a generation request that FR-056 reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GenerationOptions {
    /// Path of the measured table. Absent: no table is loaded and every clause runs.
    pub tractability_table: Option<PathBuf>,
}

impl GenerationOptions {
    /// Load the table the options name, or the empty table when none is named. A named path that
    /// fails to load is an error, never an empty table.
    ///
    /// # Errors
    ///
    /// [`TractabilityLoadError`] from [`TractabilityTable::load`].
    pub fn load_table(&self) -> Result<TractabilityTable, TractabilityLoadError> {
        self.tractability_table
            .as_deref()
            .map_or_else(|| Ok(TractabilityTable::empty()), TractabilityTable::load)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_bits_follow_the_spec_definition() {
        assert_eq!(interval_bits(0, (1_i128 << 32) - 1), 32);
        assert_eq!(interval_bits(0, 1_i128 << 32), 33);
        assert_eq!(interval_bits(0, (1_i128 << 64) - 1), 64);
        assert_eq!(interval_bits(0, 1_i128 << 64), 65);
        assert_eq!(interval_bits(0, (1_i128 << 63) - 1), 63);
        assert_eq!(interval_bits(-128, 127), 8);
        assert_eq!(interval_bits(-129, 0), 9);
        assert_eq!(interval_bits(-1, 0), 1);
        assert_eq!(interval_bits(i128::MIN, i128::MAX), 128);
    }
}
