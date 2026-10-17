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

use super::diagnostics::{redact_text, KnownDirectories};

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
    /// The `exit_status` of the one `error_details` entry that names this harness. Absent when no
    /// entry or more than one names it, when the result names no `harness_id`, and when the
    /// entry states none. Classification never reads it.
    pub(crate) exit_status: Option<String>,
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
            exit_status: None,
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

/// The exit status the one `error_details` entry naming `harness_id` states.
fn reported_exit_status(details: &[RawErrorDetail], harness_id: Option<&str>) -> Option<String> {
    let harness_id = harness_id?;
    let mut named = details
        .iter()
        .filter(|detail| detail.harness_id.as_deref() == Some(harness_id));
    match (named.next(), named.next()) {
        (Some(detail), None) => detail.exit_status.clone(),
        _ => None,
    }
}

/// The `exit_status` Kani gives a harness its own per-harness timeout stopped.
const TIMEOUT_EXIT_STATUS: &str = "timeout";

/// The most bytes an unrecognised exit status keeps.
const EXIT_STATUS_LIMIT: usize = 64;

/// Why CBMC, or a property check, ended a harness without a verdict, as the report's exit status
/// for the harness names it.
///
/// The cause serializes as `{ kind }` for `properties_failed`, `timeout` and `out_of_memory`, as
/// `{ kind: "exit_code", code }` and as `{ kind: "unknown", exitStatus }`. It is diagnostic only
/// and is never read back, so it implements no deserialization:
///
/// Trace: FR-017-AC-38, TC-027
///
/// ```compile_fail
/// let _: quire_contract_codegen::KaniFailureCause = serde_json::from_str("{}").unwrap();
/// ```
///
/// Serializing the same value compiles:
///
/// ```
/// fn serializes(cause: &quire_contract_codegen::KaniFailureCause) -> String {
///     serde_json::to_string(cause).unwrap()
/// }
/// # let _ = serializes;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum KaniFailureCause {
    /// A property check failed.
    PropertiesFailed,
    /// Kani's own per-harness timeout stopped the harness.
    Timeout,
    /// CBMC was killed by `SIGKILL`, which Kani reports as out of memory whatever sent it.
    OutOfMemory,
    /// CBMC exited with the stated status. The number is the one Kani states, not read as an
    /// exit code or a signal number.
    ExitCode {
        /// The status Kani states.
        code: i32,
    },
    /// An exit status outside the vocabulary above.
    Unknown {
        /// The status redacted and cut to 64 bytes on a character boundary.
        #[serde(rename = "exitStatus")]
        exit_status: String,
    },
}

impl KaniFailureCause {
    /// The cause the reported `exit_status` names; the first rule that applies decides.
    pub(crate) fn from_exit_status(exit_status: &str, directories: &KnownDirectories<'_>) -> Self {
        match exit_status {
            "properties_failed" => Self::PropertiesFailed,
            TIMEOUT_EXIT_STATUS => Self::Timeout,
            "out_of_memory" => Self::OutOfMemory,
            other => exit_code_of(other).map_or_else(
                || Self::Unknown {
                    exit_status: cut_to_limit(redact_text(other, directories)),
                },
                |code| Self::ExitCode { code },
            ),
        }
    }

    /// Whether the cause names a CBMC failure rather than a failed property.
    pub(crate) fn is_cbmc_failure(&self) -> bool {
        !matches!(self, Self::PropertiesFailed)
    }
}

/// The number in `exit_code_<n>`, `n` a canonical decimal that fits a signed 32-bit integer.
fn exit_code_of(exit_status: &str) -> Option<i32> {
    let digits = exit_status.strip_prefix("exit_code_")?;
    let magnitude = digits.strip_prefix('-').unwrap_or(digits);
    let canonical = magnitude == "0"
        || (magnitude.starts_with(|digit: char| ('1'..='9').contains(&digit))
            && magnitude.bytes().all(|byte| byte.is_ascii_digit()));
    if !canonical || digits == "-0" {
        return None;
    }
    digits.parse().ok()
}

/// `text` cut to its longest prefix of at most [`EXIT_STATUS_LIMIT`] bytes ending on a character
/// boundary.
fn cut_to_limit(mut text: String) -> String {
    let end = (0..=EXIT_STATUS_LIMIT.min(text.len()))
        .rev()
        .find(|end| text.is_char_boundary(*end))
        .unwrap_or(0);
    text.truncate(end);
    text
}

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
        let raw = Self::read(report)?;
        let details = raw.error_details;
        let mut results = raw.verification_results.results;
        match (results.pop(), results.is_empty()) {
            (Some(harness), true) => {
                let exit_status = reported_exit_status(&details, harness.harness_id.as_deref());
                let mut report = Self::try_from(harness)?;
                report.exit_status = exit_status;
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
                let mut report = Self::try_from(harness)?;
                report.exit_status = reported_exit_status(&details, Some(harness_id.as_str()));
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
    use std::path::Path;

    use serde_json::json;

    use super::*;
    use ix_trace_rs::trace;
    use crate::kani::test_support::implements_deserialize;

    const CRATE_DIR: &str = "/h/u/ws/crate";

    fn cause_of(exit_status: &str) -> serde_json::Value {
        let directories = KnownDirectories {
            crate_directory: Path::new(CRATE_DIR),
            target_directory: Path::new("/h/u/ws/crate/target"),
            home_directory: Some(Path::new("/h/u")),
        };
        serde_json::to_value(KaniFailureCause::from_exit_status(
            exit_status,
            &directories,
        ))
        .unwrap()
    }

    fn unknown(exit_status: &str) -> serde_json::Value {
        json!({ "kind": "unknown", "exitStatus": exit_status })
    }

    /// A single-run report whose one result is `harness_id` and whose `error_details` is `details`.
    fn report_with(harness_id: Option<&str>, details: Option<serde_json::Value>) -> Vec<u8> {
        let mut result = json!({ "status": "Failure", "checks": [] });
        if let Some(harness_id) = harness_id {
            result["harness_id"] = json!(harness_id);
        }
        let mut document = json!({
            "metadata": { "version": "1.0" },
            "verification_results": { "results": [result] },
        });
        if let Some(details) = details {
            document["error_details"] = details;
        }
        serde_json::to_vec(&document).unwrap()
    }

    fn exit_status_of(report: &[u8]) -> Option<String> {
        KaniHarnessReport::parse(report).unwrap().exit_status
    }

    /// The vocabulary: each reserved status names its cause, `exit_code_<n>` keeps a canonical
    /// signed 32-bit decimal as stated, and any other value is `unknown`.
    ///
    /// Trace: FR-017-AC-37, TC-027
    #[test]
    fn tc_027_the_exit_status_vocabulary_maps_to_its_causes() {
        assert_eq!(
            cause_of("properties_failed"),
            json!({ "kind": "properties_failed" })
        );
        assert_eq!(cause_of("timeout"), json!({ "kind": "timeout" }));
        assert_eq!(
            cause_of("out_of_memory"),
            json!({ "kind": "out_of_memory" })
        );
        for (status, code) in [
            ("exit_code_64", 64_i64),
            ("exit_code_15", 15),
            ("exit_code_0", 0),
            ("exit_code_-1", -1),
            ("exit_code_2147483647", 2_147_483_647),
            ("exit_code_-2147483648", -2_147_483_648),
        ] {
            assert_eq!(
                cause_of(status),
                json!({ "kind": "exit_code", "code": code }),
                "{status}"
            );
        }
        assert_eq!(cause_of("cbmc_exploded"), unknown("cbmc_exploded"));
        for status in [
            "exit_code_",
            "exit_code_064",
            "exit_code_+64",
            "exit_code_-0",
            "exit_code_2147483648",
            "exit_code_-2147483649",
            "exit_code_64x",
            "exit_code_-",
            "EXIT_CODE_64",
        ] {
            assert_eq!(cause_of(status), unknown(status), "{status}");
        }
    }

    /// An unknown status is redacted and then cut to 64 bytes on a character boundary.
    ///
    /// Trace: FR-017-AC-37, TC-027
    #[test]
    fn tc_027_an_unknown_exit_status_is_redacted_and_cut_to_64_bytes_on_a_character() {
        for (length, kept) in [(64, 64), (65, 64), (100, 64), (10, 10)] {
            let status = "a".repeat(length);
            assert_eq!(
                cause_of(&status),
                unknown(&"a".repeat(kept)),
                "{length} bytes"
            );
        }
        // The 64th byte falls inside a 4-byte character: the cut is before the character.
        let straddling = format!("{}\u{1f600}tail", "a".repeat(62));
        assert_eq!(cause_of(&straddling), unknown(&"a".repeat(62)));
        let fitting = format!("{}\u{1f600}tail", "a".repeat(60));
        assert_eq!(
            cause_of(&fitting),
            unknown(&format!("{}\u{1f600}", "a".repeat(60)))
        );
        assert_eq!(cause_of("/h/u/ws/crate/x"), unknown("<CRATE_DIR>/x"));
        assert_eq!(cause_of("x\u{1b}[31mred\u{1b}[0m"), unknown("xred"));
        assert_eq!(
            cause_of("killed pid 4242 after 3.5s"),
            unknown("killed pid <PID> after <DURATION>")
        );
    }

    /// The reported exit status is the one of the single entry that names the result's harness.
    ///
    /// Trace: FR-017-AC-38, TC-027
    #[test]
    fn tc_027_the_exit_status_is_read_only_from_the_one_entry_naming_the_harness() {
        let entry = |harness: &str, status: serde_json::Value| json!({ "harness_id": harness, "exit_status": status });
        let status = |details: Option<serde_json::Value>, harness: Option<&str>| {
            exit_status_of(&report_with(harness, details))
        };
        let named = Some("m::h");
        assert_eq!(
            status(Some(json!([entry("m::h", json!("exit_code_64"))])), named).as_deref(),
            Some("exit_code_64")
        );
        assert_eq!(status(None, named), None, "no error_details member");
        assert_eq!(status(Some(json!([])), named), None, "no entry");
        assert_eq!(
            status(Some(json!([entry("n::g", json!("timeout"))])), named),
            None,
            "an entry for another harness"
        );
        assert_eq!(
            status(
                Some(json!([
                    entry("m::h", json!("timeout")),
                    entry("m::h", json!("exit_code_64"))
                ])),
                named
            ),
            None,
            "two entries"
        );
        assert_eq!(
            status(Some(json!([entry("m::h", json!(null))])), named),
            None
        );
        assert_eq!(
            status(
                Some(json!([{ "harness_id": "m::h", "has_errors": false }])),
                named
            ),
            None,
            "an entry with no exit status"
        );
        assert_eq!(
            status(Some(json!([entry("m::h", json!("timeout"))])), None),
            None,
            "a result with no harness_id"
        );
    }

    /// A number and a lone surrogate escape are refused as malformed; any other string is read.
    ///
    /// Trace: FR-017-AC-38, TC-027
    #[test]
    fn tc_027_a_number_or_lone_surrogate_exit_status_is_still_malformed() {
        let with = |literal: &str| {
            format!(
                r#"{{"metadata":{{"version":"1.0"}},"error_details":[{{"harness_id":"m::h","exit_status":{literal}}}],"verification_results":{{"results":[{{"harness_id":"m::h","status":"Failure","checks":[]}}]}}}}"#
            )
        };
        for literal in ["64", r#""\ud800""#] {
            assert!(
                matches!(
                    KaniHarnessReport::parse(with(literal).as_bytes()),
                    Err(KaniReportRefusal::Malformed { .. })
                ),
                "{literal}"
            );
        }
        let read = KaniHarnessReport::parse(with(r#""\u00e9""#).as_bytes()).unwrap();
        assert_eq!(read.exit_status.as_deref(), Some("\u{e9}"));
    }

    /// The cause has the stated wire shape and implements no deserialization.
    ///
    /// Trace: FR-017-AC-38, TC-027
    #[test]
    fn tc_027_the_cause_implements_no_deserialize() {
        assert!(
            implements_deserialize!(String),
            "the probe sees a Deserialize impl"
        );
        assert!(!implements_deserialize!(KaniFailureCause));
    }

    /// `Other` cannot shadow the named classes: a spelling of `cover` or `unwind` is always that
    /// class, so its serialized form, its equality and its counting agree.
    #[trace("FR-017-AC-20", "TC-027")]
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
    #[trace("FR-017-AC-20", "TC-027")]
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
