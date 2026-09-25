//! The one place this crate reads Kani's console prose to decide a verdict (codegen#59, IR-85).
//!
//! Kani 0.67.0 — the pinned version — publishes no machine-readable verdict.
//! `--output-format` offers only `regular`, `terse` and `old`, all prose;
//! `--output-into-files` writes the same prose to files; `kani list --format json` lists
//! harnesses and never a verdict. A proof's outcome is therefore recovered from the printed
//! transcript, and this module is the only code that does so. It turns the transcript into a
//! typed [`KaniTranscript`]; verdict classification reads fields of that value and never scans
//! text for Kani's wording. A falsifying playback block is passed through verbatim as the
//! counterexample; it is decoded later by the IR crate's witness parser
//! (`quire_contract_ir::kani::Witness::parse`, via `kani_witness_join`), not here.
//!
//! The wording matched here is Kani 0.67.0's, not ours. A Kani release that changes it changes
//! what this module recognises, which is why the transcripts in `tests/fixtures/kani-0.67.0/`
//! are real captures and why a scanner test keeps the wording out of every other source file.
//!
//! Parsing is total: it never refuses. Prose that is present but does not have the expected
//! shape is reported as a distinct typed value ([`KaniCoverSummary::Malformed`]) rather than
//! being dropped, so the classifier decides what an unreadable summary means.

// Kani 0.67.0's wording. These are the only copies of it in non-test source.
const SUCCESS_BANNER: &str = "VERIFICATION:- SUCCESSFUL";
const FAILURE_BANNER: &str = "VERIFICATION:- FAILED";
const FAILED_CHECK_PREFIX: &str = "Failed Checks: ";
const UNWINDING_ASSERTION: &str = "unwinding assertion";
const PLAYBACK_HEADER: &str = "Concrete playback unit test";
const PLAYBACK_FENCE: &str = "```";
const PLAYBACK_ENTRY_POINT: &str = "kani::concrete_playback_run";
const PLAYBACK_COVER_MARKER: &str = "/// Check for `cover`";
const COVER_SUMMARY_MARKER: &str = " cover properties satisfied";
const CHECKS_SUMMARY_MARKER: &str = " failed";
const SUMMARY_LINE_PREFIX: &str = "** ";

/// Which `VERIFICATION:-` verdict banners the transcript contains.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KaniBanner {
    /// Neither banner: a build, launcher or solver failure that never reached a verdict.
    Absent,
    /// Only the success banner.
    Successful,
    /// Only the failure banner.
    Failed,
    /// Both banners. Kani prints one per harness, so a multi-harness transcript can carry both.
    Both,
}

/// The kind of a check Kani listed under `Failed Checks:`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KaniFailedCheck {
    /// A loop-unwinding check: the bound was exhausted, so the failure is not a counterexample.
    UnwindingAssertion,
    /// Any other failed check.
    Property,
}

/// The word Kani puts in the parenthetical after a summary count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KaniCountQualifierKind {
    /// `(<n> unreachable)`.
    Unreachable,
    /// `(<n> undetermined)`.
    Undetermined,
}

/// The parenthetical after a summary count, e.g. `(1 unreachable)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KaniCountQualifier {
    /// The number inside the parentheses.
    pub(crate) count: u64,
    /// The word after it.
    pub(crate) kind: KaniCountQualifierKind,
}

/// The `** <failed> of <total> failed[ (<n> <word>)]` line.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KaniChecksSummary {
    /// Checks that failed.
    pub(crate) failed: u64,
    /// Checks in total.
    pub(crate) total: u64,
    /// The trailing parenthetical, when present.
    pub(crate) qualifier: Option<KaniCountQualifier>,
}

/// The `** <satisfied> of <total> cover properties satisfied[ (<n> <word>)]` line.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KaniCoverSummary {
    /// No line mentions cover properties.
    Absent,
    /// A line mentions cover properties but none has the counts shape.
    Malformed,
    /// The first line with the counts shape.
    Counts {
        /// Satisfied cover properties.
        satisfied: u64,
        /// Total cover properties.
        total: u64,
        /// The trailing parenthetical, when present.
        qualifier: Option<KaniCountQualifier>,
    },
}

/// Which check a concrete-playback unit test was generated for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KaniPlaybackTarget {
    /// A satisfied cover: it witnesses reachability and is not a counterexample.
    Cover,
    /// Any other check: the test reproduces the failure.
    Property,
}

/// One fenced concrete-playback unit test that calls `kani::concrete_playback_run`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KaniPlayback {
    /// Which check it was generated for.
    pub(crate) target: KaniPlaybackTarget,
    /// The fenced body, trimmed, verbatim.
    pub(crate) test: String,
}

/// Everything read from one Kani run's printed output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct KaniTranscript {
    /// The verdict banners.
    pub(crate) banner: KaniBanner,
    /// Every `Failed Checks:` entry, in order.
    pub(crate) failed_checks: Vec<KaniFailedCheck>,
    /// The first well-formed checks summary line, when there is one.
    pub(crate) checks_summary: Option<KaniChecksSummary>,
    /// The cover summary.
    pub(crate) cover_summary: KaniCoverSummary,
    /// The playback tests, in order, up to the first unterminated fence.
    pub(crate) playbacks: Vec<KaniPlayback>,
}

impl KaniTranscript {
    /// The only reader of Kani's prose. `text` is stdout and stderr, newline-joined.
    pub(crate) fn parse(text: &str) -> Self {
        let has_success = text.contains(SUCCESS_BANNER);
        let has_failure = text.contains(FAILURE_BANNER);
        Self {
            banner: match (has_success, has_failure) {
                (false, false) => KaniBanner::Absent,
                (true, false) => KaniBanner::Successful,
                (false, true) => KaniBanner::Failed,
                (true, true) => KaniBanner::Both,
            },
            failed_checks: failed_checks(text),
            checks_summary: checks_summary(text),
            cover_summary: cover_summary(text),
            playbacks: playbacks(text),
        }
    }
}

fn failed_checks(text: &str) -> Vec<KaniFailedCheck> {
    text.lines()
        .filter_map(|line| line.trim().strip_prefix(FAILED_CHECK_PREFIX))
        .map(|check| {
            if check.starts_with(UNWINDING_ASSERTION) {
                KaniFailedCheck::UnwindingAssertion
            } else {
                KaniFailedCheck::Property
            }
        })
        .collect()
}

/// The parenthetical Kani appends to a summary line, e.g. `(38 undetermined)`. Both words are
/// attested in Kani 0.67.0 output: `unreachable` when a check's location is never hit,
/// `undetermined` when the solver could not decide. Either may follow either summary line.
fn qualifier(tail: &str) -> Option<KaniCountQualifier> {
    let inner = tail.strip_prefix(" (")?.strip_suffix(')')?;
    let (count, word) = inner.split_once(' ')?;
    let kind = match word {
        "unreachable" => KaniCountQualifierKind::Unreachable,
        "undetermined" => KaniCountQualifierKind::Undetermined,
        _ => return None,
    };
    Some(KaniCountQualifier {
        count: count.parse().ok()?,
        kind,
    })
}

/// One `** <a> of <b><marker>[ (<n> <word>)]` line as `(a, b, qualifier)`.
fn counts_line(line: &str, marker: &str) -> Option<(u64, u64, Option<KaniCountQualifier>)> {
    let rest = line.trim().strip_prefix(SUMMARY_LINE_PREFIX)?;
    let (counts, tail) = rest.split_once(marker)?;
    let qualifier = if tail.is_empty() {
        None
    } else {
        Some(qualifier(tail)?)
    };
    let (first, second) = counts.split_once(" of ")?;
    Some((first.parse().ok()?, second.parse().ok()?, qualifier))
}

fn checks_summary(text: &str) -> Option<KaniChecksSummary> {
    text.lines().find_map(|line| {
        let (failed, total, qualifier) = counts_line(line, CHECKS_SUMMARY_MARKER)?;
        Some(KaniChecksSummary {
            failed,
            total,
            qualifier,
        })
    })
}

fn cover_summary(text: &str) -> KaniCoverSummary {
    if let Some((satisfied, total, qualifier)) = text
        .lines()
        .find_map(|line| counts_line(line, COVER_SUMMARY_MARKER))
    {
        return KaniCoverSummary::Counts {
            satisfied,
            total,
            qualifier,
        };
    }
    if text.contains(COVER_SUMMARY_MARKER.trim_start()) {
        KaniCoverSummary::Malformed
    } else {
        KaniCoverSummary::Absent
    }
}

/// Fenced concrete-playback unit tests. A block with no closing fence ends the scan.
fn playbacks(text: &str) -> Vec<KaniPlayback> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find(PLAYBACK_HEADER) {
        let tail = &rest[start..];
        let Some(fence) = tail.find(PLAYBACK_FENCE) else {
            break;
        };
        let body = &tail[fence + PLAYBACK_FENCE.len()..];
        let Some(end) = body.find(PLAYBACK_FENCE) else {
            break;
        };
        let test = body[..end].trim();
        if test.contains(PLAYBACK_ENTRY_POINT) {
            let is_cover = test
                .lines()
                .any(|line| line.starts_with(PLAYBACK_COVER_MARKER));
            found.push(KaniPlayback {
                target: if is_cover {
                    KaniPlaybackTarget::Cover
                } else {
                    KaniPlaybackTarget::Property
                },
                test: test.to_owned(),
            });
        }
        rest = &body[end + PLAYBACK_FENCE.len()..];
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani_execution::{classify_kani_run, KaniInconclusiveReason, KaniRunOutcome};

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_a_typed_transcript_reads_each_prose_element() {
        let text = "SUMMARY:\n ** 1 of 39 failed (38 undetermined)\n\n ** 0 of 1 cover properties satisfied (1 unreachable)\n\nFailed Checks: unwinding assertion loop 0\n File: \"a.rs\"\nFailed Checks: assertion failed: x < 5\n\nVERIFICATION:- FAILED\nConcrete playback unit test for `h`:\n```\n/// Check for `cover`: \"c\"\nfn t() { kani::concrete_playback_run(v, h); }\n```\nConcrete playback unit test for `h`:\n```\n/// Check for `assertion`: \"a\"\nfn u() { kani::concrete_playback_run(v, h); }\n```\n";
        let transcript = KaniTranscript::parse(text);
        assert_eq!(transcript.banner, KaniBanner::Failed);
        assert_eq!(
            transcript.failed_checks,
            [
                KaniFailedCheck::UnwindingAssertion,
                KaniFailedCheck::Property
            ]
        );
        assert_eq!(
            transcript.checks_summary,
            Some(KaniChecksSummary {
                failed: 1,
                total: 39,
                qualifier: Some(KaniCountQualifier {
                    count: 38,
                    kind: KaniCountQualifierKind::Undetermined
                })
            })
        );
        assert_eq!(
            transcript.cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 0,
                total: 1,
                qualifier: Some(KaniCountQualifier {
                    count: 1,
                    kind: KaniCountQualifierKind::Unreachable
                })
            }
        );
        assert_eq!(
            transcript
                .playbacks
                .iter()
                .map(|playback| playback.target)
                .collect::<Vec<_>>(),
            [KaniPlaybackTarget::Cover, KaniPlaybackTarget::Property]
        );
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_banners_and_absent_prose_are_distinguished() {
        assert_eq!(KaniTranscript::parse("").banner, KaniBanner::Absent);
        assert_eq!(
            KaniTranscript::parse("VERIFICATION:- SUCCESSFUL").banner,
            KaniBanner::Successful
        );
        assert_eq!(
            KaniTranscript::parse("VERIFICATION:- SUCCESSFUL\nVERIFICATION:- FAILED").banner,
            KaniBanner::Both
        );
        let empty = KaniTranscript::parse("error[E0308]: mismatched types");
        assert_eq!(empty.cover_summary, KaniCoverSummary::Absent);
        assert_eq!(empty.checks_summary, None);
        assert!(empty.failed_checks.is_empty());
        assert!(empty.playbacks.is_empty());
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_a_cover_line_without_the_counts_shape_is_malformed_not_absent() {
        for text in [
            " ** 1 of 1 cover properties satisfied (garbage)",
            " ** x of 1 cover properties satisfied",
            "cover properties satisfied",
        ] {
            assert_eq!(
                KaniTranscript::parse(text).cover_summary,
                KaniCoverSummary::Malformed,
                "{text}"
            );
        }
        // A later well-formed line wins over an earlier malformed one.
        assert_eq!(
            KaniTranscript::parse(
                " ** 1 of 1 cover properties satisfied (garbage)\n ** 1 of 2 cover properties satisfied"
            )
            .cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 1,
                total: 2,
                qualifier: None
            }
        );
    }

    /// An unterminated playback fence ends the scan and keeps what was read before it; a block
    /// that never calls the playback entry point is not a playback.
    ///
    /// Trace: FR-017-AC-5, FR-017-AC-10, TC-027
    #[test]
    fn tc_027_playback_scanning_stops_at_an_unterminated_fence() {
        let text = "Concrete playback unit test for `h`:\n```\nfn a() { kani::concrete_playback_run(v, h); }\n```\nConcrete playback unit test for `h`:\n```\nfn b() {}\n```\nConcrete playback unit test for `h`:\n```\nfn c() { kani::concrete_playback_run(v, h); }";
        let playbacks = KaniTranscript::parse(text).playbacks;
        assert_eq!(playbacks.len(), 1);
        assert_eq!(
            playbacks[0].test,
            "fn a() { kani::concrete_playback_run(v, h); }"
        );
    }

    /// A real Kani 0.67.0 capture: its stdout and stderr newline-joined as the launcher joins
    /// them, and whether it exited successfully. See `tests/fixtures/kani-0.67.0/MANIFEST.tsv`
    /// for the exact command each was captured with.
    struct Capture {
        text: String,
        exited_successfully: bool,
    }

    macro_rules! capture {
        ($name:literal) => {
            Capture {
                text: format!(
                    "{}\n{}",
                    include_str!(concat!("../tests/fixtures/kani-0.67.0/", $name, ".stdout")),
                    include_str!(concat!("../tests/fixtures/kani-0.67.0/", $name, ".stderr")),
                ),
                exited_successfully: include_str!(concat!(
                    "../tests/fixtures/kani-0.67.0/",
                    $name,
                    ".exit"
                ))
                .trim()
                    == "0",
            }
        };
    }

    fn classified(capture: &Capture) -> KaniRunOutcome {
        classify_kani_run(capture.exited_successfully, &capture.text)
    }

    fn unreachable(count: u64) -> Option<KaniCountQualifier> {
        Some(KaniCountQualifier {
            count,
            kind: KaniCountQualifierKind::Unreachable,
        })
    }

    fn targets(transcript: &KaniTranscript) -> Vec<KaniPlaybackTarget> {
        transcript.playbacks.iter().map(|p| p.target).collect()
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_success_with_a_satisfied_cover_is_verified() {
        let capture = capture!("verified-satisfied-cover");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Successful);
        assert!(transcript.failed_checks.is_empty());
        assert_eq!(
            transcript.checks_summary,
            Some(KaniChecksSummary {
                failed: 0,
                total: 1,
                qualifier: None
            })
        );
        assert_eq!(
            transcript.cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 1,
                total: 1,
                qualifier: None
            }
        );
        assert_eq!(targets(&transcript), [KaniPlaybackTarget::Cover]);
        assert_eq!(classified(&capture), KaniRunOutcome::Verified);
    }

    /// Trace: FR-017-AC-5, FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_failure_carries_the_assertion_playback_not_the_cover_one() {
        let capture = capture!("falsified-with-playback");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Failed);
        assert_eq!(transcript.failed_checks, [KaniFailedCheck::Property]);
        assert_eq!(
            transcript.cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 1,
                total: 1,
                qualifier: None
            }
        );
        assert_eq!(
            targets(&transcript),
            [KaniPlaybackTarget::Cover, KaniPlaybackTarget::Property]
        );
        assert!(matches!(
            classified(&capture),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`")
                    && !counterexample.contains("Check for `cover`")
                    && counterexample.contains("vec![9]")
        ));
    }

    /// Trace: FR-017-AC-5, FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_unwinding_failure_is_inconclusive_not_falsified() {
        let capture = capture!("unwind-exhausted");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Failed);
        assert_eq!(
            transcript.failed_checks,
            [KaniFailedCheck::UnwindingAssertion]
        );
        assert_eq!(
            transcript.checks_summary,
            Some(KaniChecksSummary {
                failed: 1,
                total: 3,
                qualifier: Some(KaniCountQualifier {
                    count: 2,
                    kind: KaniCountQualifierKind::Undetermined
                })
            })
        );
        assert_eq!(targets(&transcript), [KaniPlaybackTarget::Cover]);
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_unreachable_cover_is_cover_unsatisfied() {
        let capture = capture!("vacuous-cover");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Successful);
        assert_eq!(
            transcript.checks_summary,
            Some(KaniChecksSummary {
                failed: 0,
                total: 1,
                qualifier: unreachable(1)
            })
        );
        assert_eq!(
            transcript.cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 0,
                total: 1,
                qualifier: unreachable(1)
            }
        );
        assert!(transcript.playbacks.is_empty());
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_partly_satisfied_covers_are_cover_unsatisfied() {
        let capture = capture!("partial-cover");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Successful);
        assert_eq!(
            transcript.cover_summary,
            KaniCoverSummary::Counts {
                satisfied: 1,
                total: 2,
                qualifier: None
            }
        );
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
    }

    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_real_kani_0_67_0_success_without_a_cover_summary_is_inconclusive() {
        let capture = capture!("no-cover");
        let transcript = KaniTranscript::parse(&capture.text);
        assert_eq!(transcript.banner, KaniBanner::Successful);
        assert_eq!(transcript.cover_summary, KaniCoverSummary::Absent);
        assert!(transcript.playbacks.is_empty());
        assert_eq!(
            classified(&capture),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary
            }
        );
    }

    /// Every `.rs` file under `dir`, recursively.
    fn rust_sources(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("source directory is readable") {
            let path = entry.expect("source entry is readable").path();
            if path.is_dir() {
                rust_sources(&path, found);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                found.push(path);
            }
        }
    }

    /// The part of a source file that is not its `#[cfg(test)] mod tests` block. The block is
    /// excluded because tests construct transcripts as input. Its span ends at the first line
    /// that is exactly `}` (rustfmt puts nothing else at column 0 inside a module), and nothing
    /// but whitespace may follow, so production code placed after the test module fails this
    /// instead of escaping the scan.
    fn production_part(text: &str) -> Result<&str, String> {
        let Some((before, module)) = text.split_once("#[cfg(test)]\nmod tests") else {
            return Ok(text);
        };
        let Some((_, after)) = module.split_once("\n}\n") else {
            return Err("the test module has no closing `}` at column 0".to_owned());
        };
        if after.trim().is_empty() {
            Ok(before)
        } else {
            Err(format!("code follows the test module: {:?}", after.trim()))
        }
    }

    /// Kani's prose is read, to decide a verdict, in this module and nowhere else: no other
    /// non-test source file under `src/` may contain the wording.
    ///
    /// Trace: FR-017-AC-10, TC-027
    #[test]
    fn tc_027_no_other_source_file_contains_kani_prose_literals() {
        const LITERALS: [&str; 7] = [
            "VERIFICATION:-",
            "Failed Checks",
            "cover properties satisfied",
            "unwinding assertion",
            "Concrete playback unit test",
            "kani::concrete_playback_run",
            "Check for `cover`",
        ];
        let mut sources = Vec::new();
        rust_sources(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut sources,
        );
        let mut scanned = 0;
        for path in sources {
            if path
                .file_name()
                .is_some_and(|name| name == "kani_transcript.rs")
            {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("source file is readable");
            let production = production_part(&text)
                .unwrap_or_else(|reason| panic!("{}: {reason}", path.display()));
            scanned += 1;
            for literal in LITERALS {
                assert!(
                    !production.contains(literal),
                    "{} contains Kani prose {literal:?}; read it in kani_transcript.rs",
                    path.display()
                );
            }
        }
        assert!(scanned > 20, "the scan found only {scanned} source files");
    }
}
