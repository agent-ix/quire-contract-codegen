//! The one place this crate reads what Kani produced for a run (IR-277, IR-288).
//!
//! A proof's verdict is read from the machine-readable report Kani writes under
//! `-Z unstable-options --export-json <file>`, never from the console. The report is parsed into
//! a typed [`KaniHarnessReport`]; classification reads fields of that value and never scans text
//! for Kani's wording. A report this module cannot read exactly -- not JSON, a member it needs
//! missing or mistyped, a check status outside the set Kani documents, a schema version other than
//! the one this module was written against, or a count of harness results other than the one a
//! run selects -- is refused with a typed [`KaniReportRefusal`]. It is never read as a verdict
//! and never defaulted to `Inconclusive`, so a Kani release that changes the report fails loudly
//! instead of quietly turning proofs into inconclusive runs.
//!
//! Kani's report carries no concrete playback. The falsifying playback is a Rust unit test Kani
//! prints on stdout, and [`counterexample_playback`] extracts that one fenced block from the
//! console text. It reads a payload to hand on verbatim, never a verdict: whether a run
//! falsified is decided by the report alone, and the block is looked up only after the report
//! names a failed property check.
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

const PLAYBACK_HEADER: &str = "Concrete playback unit test";
const PLAYBACK_FENCE: &str = "```";
const PLAYBACK_ENTRY_POINT: &str = "kani::concrete_playback_run";
const PLAYBACK_COVER_MARKER: &str = "/// Check for `cover`";

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
    /// Reads the single harness result of an exported Kani report.
    pub(crate) fn parse(report: &[u8]) -> Result<Self, KaniReportRefusal> {
        let raw: RawReport =
            serde_json::from_slice(report).map_err(|error| KaniReportRefusal::Malformed {
                detail: error.to_string(),
            })?;
        if raw.metadata.version != SUPPORTED_REPORT_VERSION {
            return Err(KaniReportRefusal::UnsupportedVersion {
                found: raw.metadata.version,
            });
        }
        let mut results = raw.verification_results.results;
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

/// The concrete-playback unit test Kani printed for a failed property check, verbatim, or `None`
/// when it printed none. A playback printed for a satisfied cover witnesses reachability and is
/// never returned. A fence with no closing fence ends the scan.
pub(crate) fn counterexample_playback(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(start) = rest.find(PLAYBACK_HEADER) {
        let tail = &rest[start..];
        let fence = tail.find(PLAYBACK_FENCE)?;
        let body = &tail[fence + PLAYBACK_FENCE.len()..];
        let end = body.find(PLAYBACK_FENCE)?;
        let test = body[..end].trim();
        if test.contains(PLAYBACK_ENTRY_POINT)
            && !test
                .lines()
                .any(|line| line.starts_with(PLAYBACK_COVER_MARKER))
        {
            return Some(test.to_owned());
        }
        rest = &body[end + PLAYBACK_FENCE.len()..];
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani_execution::{classify_kani_run, KaniInconclusiveReason, KaniRunOutcome};
    use crate::kani_identity::ObligationKind;

    /// A real Kani capture of one `--exact` harness: the exported report, its stdout, and
    /// whether it exited successfully.
    struct Capture {
        report: &'static str,
        stdout: &'static str,
        exited_successfully: bool,
    }

    macro_rules! capture {
        ($name:literal) => {
            Capture {
                report: include_str!(concat!("../tests/fixtures/kani-report/", $name, ".json")),
                stdout: include_str!(concat!("../tests/fixtures/kani-report/", $name, ".stdout")),
                exited_successfully: include_str!(concat!(
                    "../tests/fixtures/kani-report/",
                    $name,
                    ".exit"
                ))
                .trim()
                    == "0",
            }
        };
    }

    fn classified(capture: &Capture) -> KaniRunOutcome {
        classify_kani_run(
            capture.exited_successfully,
            Some(capture.report.as_bytes()),
            capture.stdout,
            None,
        )
        .expect("a real Kani report is readable")
        .outcome
    }

    fn report(capture: &Capture) -> KaniHarnessReport {
        KaniHarnessReport::parse(capture.report.as_bytes()).expect("a real Kani report is readable")
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_success_with_a_satisfied_cover_is_verified() {
        let capture = capture!("verified-satisfied-cover");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Success);
        assert_eq!(parsed.property_successes(), 1);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 1));
        assert!(!parsed.failed_property() && !parsed.failed_unwinding());
        assert_eq!(classified(&capture), KaniRunOutcome::Verified);
    }

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_failure_carries_the_assertion_playback_not_the_cover_one() {
        let capture = capture!("falsified-with-playback");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Failure);
        assert!(parsed.failed_property() && !parsed.failed_unwinding());
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 1));
        assert!(matches!(
            classified(&capture),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`")
                    && !counterexample.contains("Check for `cover`")
                    && counterexample.contains("assertion failed: x < 5")
        ));
    }

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_unwinding_failure_is_inconclusive_not_falsified() {
        let capture = capture!("unwind-exhausted");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Failure);
        assert!(parsed.failed_unwinding());
        assert!(
            !parsed.failed_property(),
            "the undetermined checks are not failures"
        );
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
    }

    /// Kani reports a harness whose every check is unreachable as a success with zero successful
    /// checks, so it is vacuous before its cover is consulted.
    ///
    /// Trace: FR-017-AC-12, FR-017-AC-13, TC-027
    #[test]
    fn tc_027_real_kani_a_run_with_no_successful_check_is_a_vacuous_proof() {
        let capture = capture!("vacuous-cover");
        let parsed = report(&capture);
        assert_eq!(parsed.status, KaniHarnessStatus::Success);
        assert_eq!(parsed.property_successes(), 0);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (0, 1));
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            }
        );
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_partly_satisfied_covers_are_cover_unsatisfied() {
        let capture = capture!("partial-cover");
        let parsed = report(&capture);
        assert_eq!((parsed.covers_satisfied(), parsed.covers_total()), (1, 2));
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
    }

    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_real_kani_success_without_a_cover_is_inconclusive() {
        let capture = capture!("no-cover");
        assert_eq!(report(&capture).covers_total(), 0);
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary
            }
        );
    }

    /// A run's verdict is the report's, whatever the console says: the success banner printed
    /// beside a failing report decides nothing.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_the_console_banner_never_decides_the_verdict() {
        let capture = capture!("falsified-with-playback");
        let outcome = classify_kani_run(
            false,
            Some(capture.report.as_bytes()),
            "VERIFICATION:- SUCCESSFUL",
            None,
        )
        .expect("readable")
        .outcome;
        assert_eq!(
            outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
        );
        let verified = capture!("verified-satisfied-cover");
        let outcome = classify_kani_run(
            true,
            Some(verified.report.as_bytes()),
            "VERIFICATION:- FAILED",
            None,
        )
        .expect("readable")
        .outcome;
        assert_eq!(outcome, KaniRunOutcome::Verified);
    }

    fn mutated(edit: impl FnOnce(&mut serde_json::Value)) -> Vec<u8> {
        let mut report: serde_json::Value =
            serde_json::from_str(capture!("verified-satisfied-cover").report).unwrap();
        edit(&mut report);
        serde_json::to_vec(&report).unwrap()
    }

    /// A report that differs from the schema this module reads is refused with its own typed
    /// cause and never classified.
    ///
    /// Trace: FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_report_that_changed_shape_is_refused_not_classified() {
        let version = mutated(|report| report["metadata"]["version"] = "2.0".into());
        assert_eq!(
            KaniHarnessReport::parse(&version),
            Err(KaniReportRefusal::UnsupportedVersion {
                found: "2.0".to_owned()
            })
        );
        let status = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["status"] = "Proved".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&status),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let harness_status = mutated(|report| {
            report["verification_results"]["results"][0]["status"] = "Timeout".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&harness_status),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let renamed = mutated(|report| {
            let results = report["verification_results"].take();
            report["verification"] = results;
            report
                .as_object_mut()
                .unwrap()
                .remove("verification_results");
        });
        assert!(matches!(
            KaniHarnessReport::parse(&renamed),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        assert!(matches!(
            KaniHarnessReport::parse(b"VERIFICATION:- SUCCESSFUL"),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        for refusal in [&version, &status, &harness_status, &renamed] {
            let refused = classify_kani_run(true, Some(refusal), "VERIFICATION:- SUCCESSFUL", None);
            assert!(refused.is_err(), "a refused report is never a verdict");
        }
    }

    /// A report holding any number of harness results but one is refused: a run selects one
    /// `--exact` harness, so another count means the selection or the report is not what was
    /// asked for.
    ///
    /// Trace: FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_report_without_exactly_one_harness_is_refused() {
        let none = mutated(|report| {
            report["verification_results"]["results"] = serde_json::json!([]);
        });
        assert_eq!(
            KaniHarnessReport::parse(&none),
            Err(KaniReportRefusal::HarnessCount { found: 0 })
        );
        let two = mutated(|report| {
            let first = report["verification_results"]["results"][0].clone();
            report["verification_results"]["results"]
                .as_array_mut()
                .unwrap()
                .push(first);
        });
        assert_eq!(
            KaniHarnessReport::parse(&two),
            Err(KaniReportRefusal::HarnessCount { found: 2 })
        );
    }

    /// A report whose harness says success but which lists a check that failed, errored, was
    /// undetermined or unknown, or an unwinding assertion that failed, contradicts itself and is
    /// refused with its own cause, for every obligation kind. It is never `Verified`.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-18, TC-027
    #[test]
    fn tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified() {
        for status in ["Failure", "Error", "Undetermined", "Unknown"] {
            for category in ["assertion", "unwind", "cover"] {
                let report = mutated(|report| {
                    let checks = &mut report["verification_results"]["results"][0]["checks"];
                    let last = checks.as_array().unwrap().len() - 1;
                    checks[last]["status"] = status.into();
                    checks[last]["category"] = category.into();
                });
                let found = KaniHarnessReport::parse(&report);
                assert!(
                    matches!(found, Err(KaniReportRefusal::Inconsistent { .. })),
                    "{category} {status}: {found:?}"
                );
                for kind in [None, Some(ObligationKind::Precondition)] {
                    let run = classify_kani_run(true, Some(&report), "", kind);
                    assert!(
                        matches!(run, Err(KaniReportRefusal::Inconsistent { .. })),
                        "{category} {status} {kind:?}: {run:?}"
                    );
                }
            }
        }
        let held = mutated(|_| {});
        assert!(KaniHarnessReport::parse(&held).is_ok());
    }

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

    /// The per-check view of a real run names every check with its id, class, source location and
    /// status, so a consumer can attribute each successful check to a source line.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status() {
        let capture = capture!("falsified-with-playback");
        let run = classify_kani_run(
            capture.exited_successfully,
            Some(capture.report.as_bytes()),
            capture.stdout,
            None,
        )
        .unwrap();
        let at = |line| KaniCheckLocation {
            file: "src/lib.rs".to_owned(),
            line: Some(line),
        };
        assert_eq!(
            run.checks,
            [
                KaniCheckResult {
                    id: 1,
                    class: KaniCheckClass::Cover,
                    location: at(15),
                    status: KaniCheckStatus::Satisfied,
                },
                KaniCheckResult {
                    id: 2,
                    class: KaniCheckClass::from("assertion".to_owned()),
                    location: at(16),
                    status: KaniCheckStatus::Failure,
                },
            ]
        );
        let unwound = capture!("unwind-exhausted");
        let run = classify_kani_run(
            unwound.exited_successfully,
            Some(unwound.report.as_bytes()),
            unwound.stdout,
            None,
        )
        .unwrap();
        assert!(run
            .checks
            .iter()
            .any(|check| check.class == KaniCheckClass::Unwind
                && check.status == KaniCheckStatus::Failure
                && check.location.line == Some(24)));
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

    /// A location Kani leaves unknown has no line; a line that is neither a number nor Kani's
    /// word for none is a malformed report, not a dropped location.
    ///
    /// Trace: FR-017-AC-20, TC-027
    #[test]
    fn tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused() {
        let unknown = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["location"]["line"] =
                "unknown".into();
        });
        let parsed = KaniHarnessReport::parse(&unknown).unwrap();
        assert_eq!(parsed.checks[0].location.line, None);
        let bad = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["location"]["line"] =
                "12a".into();
        });
        assert!(matches!(
            KaniHarnessReport::parse(&bad),
            Err(KaniReportRefusal::Malformed { .. })
        ));
        let no_location = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]
                .as_object_mut()
                .unwrap()
                .remove("location");
        });
        assert!(matches!(
            KaniHarnessReport::parse(&no_location),
            Err(KaniReportRefusal::Malformed { .. })
        ));
    }

    /// A check category outside the two this module names is a property: an unknown category must
    /// still count against a proof, never be dropped.
    ///
    /// Trace: FR-017-AC-12, TC-027
    #[test]
    fn tc_027_an_unnamed_check_category_is_a_property() {
        let report = mutated(|report| {
            report["verification_results"]["results"][0]["checks"][0]["category"] =
                "pointer_dereference".into();
            report["verification_results"]["results"][0]["checks"][0]["status"] = "Failure".into();
            report["verification_results"]["results"][0]["status"] = "Failure".into();
        });
        let parsed = KaniHarnessReport::parse(&report).unwrap();
        assert!(parsed.failed_property());
        assert_eq!(
            parsed.checks[0].class,
            KaniCheckClass::from("pointer_dereference".to_owned())
        );
    }

    /// Trace: FR-017-AC-5, FR-017-AC-12, TC-027
    #[test]
    fn tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence() {
        let cover = "Concrete playback unit test for `h`:\n```\n/// Check for `cover`: \"c\"\nfn t() { kani::concrete_playback_run(v, h); }\n```\n";
        let property = "Concrete playback unit test for `h`:\n```\n/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }\n```\n";
        let no_entry = "Concrete playback unit test for `h`:\n```\nfn b() {}\n```\n";
        let unterminated =
            "Concrete playback unit test for `h`:\n```\nfn c() { kani::concrete_playback_run(v, h); }";
        assert_eq!(
            counterexample_playback(&format!("{cover}{no_entry}{property}")).as_deref(),
            Some("/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }")
        );
        assert_eq!(counterexample_playback(&format!("{cover}{no_entry}")), None);
        assert_eq!(
            counterexample_playback(&format!("{cover}{unterminated}")),
            None
        );
        assert_eq!(counterexample_playback(""), None);
    }
}
