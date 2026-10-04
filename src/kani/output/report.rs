//! The typed parse of the machine-readable report Kani writes (IR-277, IR-288).
//!
//! A proof's verdict is read from the report Kani writes under
//! `-Z unstable-options --export-json <file>`, never from the console. The report is parsed into
//! a typed [`KaniHarnessReport`]; classification reads fields of that value and never scans text
//! for Kani's wording. A report this module cannot read exactly -- not JSON, a member it needs
//! missing or mistyped, a check status outside the set Kani documents, a schema version other than
//! the one this module was written against, or a count of harness results other than the one a
//! run selects -- is refused with a typed [`KaniReportRefusal`]. It is never read as a verdict
//! and never defaulted to `Inconclusive`, so a Kani release that changes the report fails loudly
//! instead of quietly turning proofs into inconclusive runs.
//!
//! The report is read member by member rather than with `deny_unknown_fields`: Kani adds members
//! (it is an unstable interface) and this module needs only those it names. The vocabulary of
//! check statuses is closed, because a status this module does not know could be a verdict.

use std::fmt;

use serde::{Deserialize, Serialize};

/// The report schema version this module reads (`metadata.version`).
const SUPPORTED_REPORT_VERSION: &str = "1.0";

/// The check category Kani gives a `cover!` property.
const COVER_CATEGORY: &str = "cover";
/// The check category Kani gives a loop-unwinding assertion.
const UNWIND_CATEGORY: &str = "unwind";

/// Why a Kani report was not read.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KaniReportRefusal {
    /// The run exited successfully but exported no report.
    Missing,
    /// The report file could not be read.
    Unreadable {
        /// The underlying I/O error, rendered.
        detail: String,
    },
    /// The report is larger than the bound this crate reads.
    TooLarge {
        /// The bound, in bytes.
        limit: usize,
    },
    /// The report is not JSON of the shape this module reads: a member is absent or mistyped, or
    /// a check status is outside the set Kani documents.
    Malformed {
        /// The parser's description of the first mismatch.
        detail: String,
    },
    /// `metadata.version` is not the schema version this module reads.
    UnsupportedVersion {
        /// The version the report states.
        found: String,
    },
    /// The report does not hold exactly one harness result, which is what a run of one
    /// `--exact` harness yields.
    HarnessCount {
        /// How many harness results the report holds.
        found: usize,
    },
    /// The harness status says every property held, but the report lists a check that did not
    /// hold. Neither is trusted: a report that contradicts itself is not a verdict.
    Inconsistent {
        /// The first check that contradicts the harness status.
        check_id: u64,
        /// That check's status.
        status: KaniCheckStatus,
    },
    /// The report of a batch holds no result for a harness the launch asked for (FR-017-AC-23).
    HarnessMissing {
        /// The requested `module::harness` path.
        harness: String,
    },
    /// The report of a batch holds two results for one harness (FR-017-AC-23).
    HarnessDuplicated {
        /// The `module::harness` path the report names twice.
        harness: String,
    },
    /// The report of a batch holds a result for a harness the launch did not ask for
    /// (FR-017-AC-23).
    HarnessUnrequested {
        /// The `module::harness` path the report names.
        harness: String,
    },
}

impl fmt::Display for KaniReportRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => write!(formatter, "the run exported no Kani report"),
            Self::Unreadable { detail } => {
                write!(formatter, "the Kani report is unreadable: {detail}")
            }
            Self::TooLarge { limit } => {
                write!(formatter, "the Kani report is larger than {limit} bytes")
            }
            Self::Malformed { detail } => {
                write!(formatter, "the Kani report is malformed: {detail}")
            }
            Self::UnsupportedVersion { found } => write!(
                formatter,
                "the Kani report schema version {found:?} is not {SUPPORTED_REPORT_VERSION:?}"
            ),
            Self::HarnessCount { found } => write!(
                formatter,
                "the Kani report holds {found} harness results, not one"
            ),
            Self::Inconsistent { check_id, status } => write!(
                formatter,
                "the Kani report states success but check {check_id} is {status:?}"
            ),
            Self::HarnessMissing { harness } => write!(
                formatter,
                "the Kani report holds no result for the requested harness {harness}"
            ),
            Self::HarnessDuplicated { harness } => {
                write!(formatter, "the Kani report holds {harness} more than once")
            }
            Self::HarnessUnrequested { harness } => write!(
                formatter,
                "the Kani report holds a result for {harness}, which was not requested"
            ),
        }
    }
}

impl std::error::Error for KaniReportRefusal {}

/// Kani's verdict for one harness (`verification_results.results[].status`).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(crate) enum KaniHarnessStatus {
    /// Every property held.
    Success,
    /// At least one property failed.
    Failure,
}

/// The status of one check. These are exactly the variants of Kani's own `CheckStatus`, which the
/// report prints by its `Debug` name.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum KaniCheckStatus {
    /// The property held.
    Success,
    /// The property failed.
    Failure,
    /// A cover property was reached.
    Satisfied,
    /// A cover property cannot be reached.
    Unsatisfiable,
    /// The property's location is never reached.
    Unreachable,
    /// The solver could not decide the property.
    Undetermined,
    /// Another property failed, so this one cannot be concluded.
    Unknown,
    /// A code-coverage property was reached.
    Covered,
    /// A code-coverage property was not reached.
    Uncovered,
    /// The solver reported an error for the property.
    Error,
}

/// What kind of property a check is. Kani's class is open-ended (`assertion`, `overflow`,
/// `pointer_dereference`, ...); only the two kinds that change a verdict are named, and any
/// other class keeps Kani's own word.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(from = "String", into = "String")]
pub enum KaniCheckClass {
    /// A `kani::cover!` property.
    Cover,
    /// A loop-unwinding assertion.
    Unwind,
    /// Any other class, as Kani spelled it. It cannot hold `cover` or `unwind`: the only way to
    /// make one is `KaniCheckClass::from`, which gives those spellings their own variants.
    Other(OtherCheckClass),
}

/// The spelling of a check class that is neither a cover nor an unwinding assertion. The field
/// is private so no caller can build one that shadows a named class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherCheckClass(String);

impl OtherCheckClass {
    /// The class as Kani spelled it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for KaniCheckClass {
    fn from(class: String) -> Self {
        match class.as_str() {
            COVER_CATEGORY => Self::Cover,
            UNWIND_CATEGORY => Self::Unwind,
            _ => Self::Other(OtherCheckClass(class)),
        }
    }
}

impl From<KaniCheckClass> for String {
    fn from(class: KaniCheckClass) -> Self {
        match class {
            KaniCheckClass::Cover => COVER_CATEGORY.to_owned(),
            KaniCheckClass::Unwind => UNWIND_CATEGORY.to_owned(),
            KaniCheckClass::Other(class) => class.0,
        }
    }
}

/// Where Kani attributes a check in the source it verified.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KaniCheckLocation {
    /// The source file, as Kani printed it (`unknown` when Kani had none).
    pub file: String,
    /// The line, `None` when Kani had none.
    pub line: Option<u32>,
}

/// One check Kani reported for a harness: the per-check view a consumer attributes proof to
/// source with.
///
/// This is this crate's own view and has one wire shape, the one it serializes: `id`, `class`,
/// `location { file, line }` and `status`. It is serialize-only. Kani's report spells the class
/// `category` and the line as a string; that spelling is read by a private type and never by
/// this one, so the view cannot be deserialized into a shape it does not serialize to.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KaniCheckResult {
    /// The check's position in the harness's list, from one.
    pub id: u64,
    /// What kind of property it is.
    pub class: KaniCheckClass,
    /// Where it is attributed.
    pub location: KaniCheckLocation,
    /// Its status.
    pub status: KaniCheckStatus,
}

/// Kani prints a missing line or column as this word.
const UNKNOWN_LOCATION: &str = "unknown";

#[derive(Deserialize)]
struct RawCheck {
    id: u64,
    status: KaniCheckStatus,
    category: KaniCheckClass,
    location: RawLocation,
}

#[derive(Deserialize)]
struct RawLocation {
    file: String,
    line: String,
}

impl TryFrom<RawCheck> for KaniCheckResult {
    type Error = KaniReportRefusal;

    fn try_from(raw: RawCheck) -> Result<Self, KaniReportRefusal> {
        let line = if raw.location.line == UNKNOWN_LOCATION {
            None
        } else {
            Some(
                raw.location
                    .line
                    .parse()
                    .map_err(|_| KaniReportRefusal::Malformed {
                        detail: format!(
                            "check {}: line {:?} is not a number",
                            raw.id, raw.location.line
                        ),
                    })?,
            )
        };
        Ok(Self {
            id: raw.id,
            class: raw.category,
            location: KaniCheckLocation {
                file: raw.location.file,
                line,
            },
            status: raw.status,
        })
    }
}

/// The typed result Kani reported for one harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KaniHarnessReport {
    /// Kani's verdict.
    pub(crate) status: KaniHarnessStatus,
    /// Every check, in the order Kani listed them.
    pub(crate) checks: Vec<KaniCheckResult>,
}

#[derive(Deserialize)]
struct RawHarness {
    /// The `module::harness` path Kani echoes for the harness. A single run does not read it.
    harness_id: Option<String>,
    status: KaniHarnessStatus,
    checks: Vec<RawCheck>,
}

impl TryFrom<RawHarness> for KaniHarnessReport {
    type Error = KaniReportRefusal;

    fn try_from(raw: RawHarness) -> Result<Self, KaniReportRefusal> {
        Ok(Self {
            status: raw.status,
            checks: raw
                .checks
                .into_iter()
                .map(KaniCheckResult::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Deserialize)]
struct RawReport {
    metadata: RawMetadata,
    verification_results: RawResults,
    /// Kani lists an entry for every harness; one that errored carries an `exit_status`.
    #[serde(default)]
    error_details: Vec<RawErrorDetail>,
}

#[derive(Deserialize)]
struct RawErrorDetail {
    harness_id: Option<String>,
    exit_status: Option<String>,
}

/// The `exit_status` Kani gives a harness its own per-harness timeout stopped.
const TIMEOUT_EXIT_STATUS: &str = "timeout";

/// One harness result of a batch report, with the path Kani names it by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KaniMemberReport {
    /// The `module::harness` path the launch passed to `--harness`, as Kani echoes it.
    pub(crate) harness_id: String,
    /// The typed result.
    pub(crate) report: KaniHarnessReport,
    /// Whether Kani's own per-harness timeout stopped this harness (its `error_details` entry
    /// gives the exit status `timeout`).
    pub(crate) timed_out: bool,
}

/// The harness results of `members` in the order `requested` names them, one each.
///
/// A result for a harness not requested, two for one harness, and a requested harness with none
/// each refuse the whole report: which member a result belongs to is then not known, and a batch
/// whose members cannot be told apart has no member evidence.
pub(crate) fn members_in_request_order(
    requested: &[String],
    members: &[KaniMemberReport],
) -> Result<Vec<KaniMemberReport>, KaniReportRefusal> {
    for member in members {
        if !requested.contains(&member.harness_id) {
            return Err(KaniReportRefusal::HarnessUnrequested {
                harness: member.harness_id.clone(),
            });
        }
        if members
            .iter()
            .filter(|other| other.harness_id == member.harness_id)
            .count()
            > 1
        {
            return Err(KaniReportRefusal::HarnessDuplicated {
                harness: member.harness_id.clone(),
            });
        }
    }
    requested
        .iter()
        .map(|harness| {
            members
                .iter()
                .find(|member| &member.harness_id == harness)
                .cloned()
                .ok_or_else(|| KaniReportRefusal::HarnessMissing {
                    harness: harness.clone(),
                })
        })
        .collect()
}

#[derive(Deserialize)]
struct RawMetadata {
    version: String,
}

#[derive(Deserialize)]
struct RawResults {
    results: Vec<RawHarness>,
}

impl KaniHarnessReport {
    /// The report document, at the one schema version this module reads.
    fn read(report: &[u8]) -> Result<RawReport, KaniReportRefusal> {
        let raw: RawReport =
            serde_json::from_slice(report).map_err(|error| KaniReportRefusal::Malformed {
                detail: error.to_string(),
            })?;
        if raw.metadata.version != SUPPORTED_REPORT_VERSION {
            return Err(KaniReportRefusal::UnsupportedVersion {
                found: raw.metadata.version,
            });
        }
        Ok(raw)
    }

    /// Reads the single harness result of an exported Kani report.
    pub(crate) fn parse(report: &[u8]) -> Result<Self, KaniReportRefusal> {
        let mut results = Self::read(report)?.verification_results.results;
        match (results.pop(), results.is_empty()) {
            (Some(harness), true) => {
                let report = Self::try_from(harness)?;
                report.refuse_contradiction()?;
                Ok(report)
            }
            (Some(_), false) => Err(KaniReportRefusal::HarnessCount {
                found: results.len() + 1,
            }),
            (None, _) => Err(KaniReportRefusal::HarnessCount { found: 0 }),
        }
    }

    /// Reads every harness result of an exported Kani report, in the order Kani listed them (which
    /// is not the order the launch asked for them in). Each result is read exactly as a single
    /// run's is, and must name its harness.
    pub(crate) fn parse_batch(report: &[u8]) -> Result<Vec<KaniMemberReport>, KaniReportRefusal> {
        let raw = Self::read(report)?;
        let details = raw.error_details;
        raw.verification_results
            .results
            .into_iter()
            .map(|harness| {
                let harness_id =
                    harness
                        .harness_id
                        .clone()
                        .ok_or_else(|| KaniReportRefusal::Malformed {
                            detail: "a harness result names no harness_id".to_owned(),
                        })?;
                let report = Self::try_from(harness)?;
                report.refuse_contradiction()?;
                let timed_out = details.iter().any(|detail| {
                    detail.harness_id.as_deref() == Some(harness_id.as_str())
                        && detail.exit_status.as_deref() == Some(TIMEOUT_EXIT_STATUS)
                });
                Ok(KaniMemberReport {
                    harness_id,
                    report,
                    timed_out,
                })
            })
            .collect()
    }

    /// A harness that states success must list no check that failed, errored or was left
    /// undecided or unknown. Kani derives the harness status from its checks, so a report where
    /// they disagree has changed shape or been altered, and reading either side would invent a
    /// verdict.
    fn refuse_contradiction(&self) -> Result<(), KaniReportRefusal> {
        if self.status != KaniHarnessStatus::Success {
            return Ok(());
        }
        match self.checks.iter().find(|check| {
            matches!(
                check.status,
                KaniCheckStatus::Failure
                    | KaniCheckStatus::Error
                    | KaniCheckStatus::Undetermined
                    | KaniCheckStatus::Unknown
            )
        }) {
            Some(check) => Err(KaniReportRefusal::Inconsistent {
                check_id: check.id,
                status: check.status,
            }),
            None => Ok(()),
        }
    }

    /// Checks that are not covers and held.
    pub(crate) fn property_successes(&self) -> u32 {
        self.count(|check| {
            check.class != KaniCheckClass::Cover && check.status == KaniCheckStatus::Success
        })
    }

    /// Cover properties that were satisfied.
    pub(crate) fn covers_satisfied(&self) -> u32 {
        self.count(|check| {
            check.class == KaniCheckClass::Cover && check.status == KaniCheckStatus::Satisfied
        })
    }

    /// Cover properties in total.
    pub(crate) fn covers_total(&self) -> u32 {
        self.count(|check| check.class == KaniCheckClass::Cover)
    }

    /// Whether a loop-unwinding assertion failed.
    pub(crate) fn failed_unwinding(&self) -> bool {
        self.count(|check| {
            check.class == KaniCheckClass::Unwind && check.status == KaniCheckStatus::Failure
        }) > 0
    }

    /// Whether a property other than a cover or an unwinding assertion failed.
    pub(crate) fn failed_property(&self) -> bool {
        self.count(|check| {
            matches!(check.class, KaniCheckClass::Other(_))
                && check.status == KaniCheckStatus::Failure
        }) > 0
    }

    fn count(&self, matching: impl Fn(&KaniCheckResult) -> bool) -> u32 {
        let count = self.checks.iter().filter(|check| matching(check)).count();
        u32::try_from(count).unwrap_or(u32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Other` cannot shadow the named classes: a spelling of `cover` or `unwind` is always that
    /// class, so its serialized form, its equality and its counting agree.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_a_class_spelled_cover_or_unwind_is_never_other() {
        assert_eq!(
            KaniCheckClass::from("cover".to_owned()),
            KaniCheckClass::Cover
        );
        assert_eq!(
            KaniCheckClass::from("unwind".to_owned()),
            KaniCheckClass::Unwind
        );
        let other = KaniCheckClass::from("assertion".to_owned());
        assert!(matches!(&other, KaniCheckClass::Other(name) if name.as_str() == "assertion"));
        assert_eq!(String::from(other), "assertion");
        let decoded: KaniCheckClass = serde_json::from_str("\"cover\"").unwrap();
        assert_eq!(decoded, KaniCheckClass::Cover);
    }

    /// The per-check view has one wire shape: it serializes as `id`, `class`, `location { file,
    /// line }` and `status`, with the line a number (or null), and it is not read back from any
    /// other spelling. Kani's own `category` key is not part of it.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_the_per_check_view_has_one_serialized_wire_shape() {
        let checks = [
            KaniCheckResult {
                id: 1,
                class: KaniCheckClass::Cover,
                location: KaniCheckLocation {
                    file: "src/lib.rs".to_owned(),
                    line: Some(15),
                },
                status: KaniCheckStatus::Satisfied,
            },
            KaniCheckResult {
                id: 2,
                class: KaniCheckClass::from("assertion".to_owned()),
                location: KaniCheckLocation {
                    file: "unknown".to_owned(),
                    line: None,
                },
                status: KaniCheckStatus::Failure,
            },
        ];
        assert_eq!(
            serde_json::to_value(checks).unwrap(),
            serde_json::json!([
                {
                    "id": 1,
                    "class": "cover",
                    "location": { "file": "src/lib.rs", "line": 15 },
                    "status": "Satisfied"
                },
                {
                    "id": 2,
                    "class": "assertion",
                    "location": { "file": "unknown", "line": null },
                    "status": "Failure"
                },
            ])
        );
    }
}
