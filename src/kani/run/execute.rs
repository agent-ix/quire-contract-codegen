//! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
//!
//! Completed outcomes are read from the backend's own exported report. A wall-clock or
//! memory overage is observed by the bounded launcher and stops the tree before a verdict
//! can be classified. Every execution uses the ceilings recorded in the harness identity;
//! there is no independent execution budget that can weaken that identity.
//!
//! A batch ([`execute_kani_obligations`]) runs the harnesses that can share a launcher process in
//! one, and splits its one exported report and its one console back into one evidence record per
//! harness. A harness is found in the report by the `module::harness` path the launch passed to
//! `--harness`, and its playback in the console by the path its block is headed for.
//!
//! A run is two-phase (FR-017-AC-63). The proof phase launches the harness's proof vector, which
//! has no concrete playback. Only when it fails a non-unwinding property does a witness phase
//! launch the witness vector, which prints the playback; the witness phase must reproduce the
//! failed checks, or the run is inconclusive with `WitnessDisagreed`. Both phases are held to the
//! identity ceilings, and the witness phase's evidence nests the proof phase's.

use std::{
    fmt, fs,
    num::NonZeroUsize,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

use quire_contract_ir::kani::KaniCancellationSource;
use quire_exact::Cancel;
use serde::Serialize;

use crate::kani::{
    abi::KaniSolver,
    classify::{
        classify_kani_run, classify_member, ClassifiedRun, KaniInconclusiveReason, KaniRunOutcome,
    },
    identity::ObligationKind,
    output::{
        diagnostics::{KaniNoVerdictDiagnostics, KnownDirectories, ProcessOutput},
        playback::{counterexample_from_blocks, playback_blocks, select_counterexample},
        report::{
            members_in_request_order, KaniCheckClass, KaniCheckLocation, KaniCheckResult,
            KaniCheckStatus, KaniFailureCause, KaniHarnessReport, KaniHarnessStatus,
            KaniReportRefusal,
        },
    },
    run::{
        harness::{HarnessView, KaniExecutableHarness},
        launch::{
            run_bounded_launcher, BoundedLaunch, CaptureStream, CapturedStreams, LaunchOutcome,
        },
        memory::MemoryObservation,
        report_file::{fresh_report_path, read_report, remove_stale_report},
        tool::{KaniInstallation, KaniTool, KaniToolError},
    },
};

/// The stable code of a run refused because a stream carried more than its limit.
pub const OUTPUT_OVER_LIMIT_CODE: &str = "kani_output_over_limit";

/// The stable code of a run refused because a stream could not be read to its end.
pub const OUTPUT_UNREAD_CODE: &str = "kani_output_unread";

/// One execution of one harness in a crate the caller wrote.
pub struct KaniExecutionRequest<'a> {
    /// Caller-owned cancellation handle, shared only within one execution group.
    pub cancel: &'a Cancel,
    /// Backend to invoke.
    pub installation: &'a KaniInstallation,
    /// The generated harness.
    pub harness: KaniExecutableHarness<'a>,
    /// Crate root whose `src/lib.rs` contains the harness source byte-for-byte.
    pub crate_directory: &'a Path,
    /// Cargo target directory for the run.
    pub target_directory: &'a Path,
    /// The home directory of the launcher's environment, when the caller knows it. It redacts
    /// the diagnostics record of the run and keeps requests with different home
    /// directories out of one launcher process; it is never read from the process environment.
    pub home_directory: Option<&'a Path>,
}

/// One native test appended beside a generated proof solely for concrete
/// playback. The test name is the exact libtest path, including its module.
/// A `Completed { succeeded: true }` outcome attests that this declared test passed; that its
/// body replays the sealed proof function (and does not, for example, shadow it with a
/// body-local item) is the caller's responsibility.
pub struct KaniConcretePlayback<'a> {
    /// Exact libtest path of the playback function.
    pub test_name: &'a str,
    /// Complete source text of the one appended `#[test]` function.
    pub appended_test: &'a str,
}

/// The bounded native execution of one concrete Kani playback test. A successful
/// process is returned only after libtest reports that exact test passed.
#[derive(Debug)]
#[non_exhaustive]
pub enum KaniConcretePlaybackOutcome {
    /// The playback test process exited on its own.
    Completed {
        /// Whether the process exited successfully.
        succeeded: bool,
        /// Exit code, when one was available.
        exit_code: Option<i32>,
        /// Complete, bounded stdout from the playback process.
        stdout: Vec<u8>,
        /// Complete, bounded stderr from the playback process.
        stderr: Vec<u8>,
        /// Observed process-tree memory use.
        memory: MemoryObservation,
    },
    /// The process exceeded the harness's wall-clock ceiling and was torn down.
    TimedOut {
        /// Observed process-tree memory before owned teardown.
        memory: MemoryObservation,
    },
    /// Cancellation stopped the process and its owned tree.
    Cancelled {
        /// First observed cancellation source.
        source: KaniCancellationSource,
        /// Observed process-tree memory before owned teardown.
        memory: MemoryObservation,
    },
}

/// Why a harness was not run.
#[derive(Debug)]
pub enum KaniExecutionRefusal {
    /// An ordinary and a constrained harness must use their respective execution entries.
    WrongMethodExecutionRoute,
    /// A multi-harness launcher stopped on its retained cancellation source.
    BatchCancelled {
        /// The source observed before owned teardown.
        source: KaniCancellationSource,
        /// Number of members that have no execution evidence.
        members: usize,
    },
    /// The installed backend could not be located or started.
    Tool(KaniToolError),
    /// The crate's `src/lib.rs` does not contain the harness source.
    HarnessNotInCrate {
        /// The generated artifact path.
        harness_path: String,
    },
    /// A sealed generated harness's source, identity or record changed after generation.
    HarnessProvenanceMismatch {
        /// The generated artifact path.
        harness_path: String,
    },
    /// The sealed collection-precondition totality assertion or its unique source binding
    /// disagrees with the crate supplied for execution.
    BoundTotalityAssertionMismatch {
        /// The generated artifact path.
        harness_path: String,
    },
    /// Concrete playback requires a generator-sealed V2 clause harness.
    ConcretePlaybackUnsupportedHarness {
        /// The unsupported generated artifact path.
        harness_path: String,
    },
    /// The declared playback test is missing, duplicated, malformed, or does
    /// not restore the generated proof bytes in `src/lib.rs` when removed.
    ConcretePlaybackSourceMismatch {
        /// The generated artifact path whose bytes must be preserved.
        harness_path: String,
    },
    /// Libtest exited successfully without reporting the requested exact test
    /// as its one passing test. A zero-match run is not playback evidence.
    ConcretePlaybackTestNotObserved {
        /// The exact libtest path that was requested.
        test_name: String,
    },
    /// The run exported a Kani report this crate cannot read exactly. The run proved nothing
    /// and decided nothing; the refusal is never an outcome.
    Report(KaniReportRefusal),
    /// A stream carried more than its limit, so the run was stopped and its process group killed
    /// (stable code [`OUTPUT_OVER_LIMIT_CODE`], FR-017-AC-14). No text is retained and no outcome
    /// is classified: a truncated stream is evidence nobody can vouch for.
    OutputOverLimit {
        /// The stream that carried too much.
        stream: CaptureStream,
        /// The most it may carry, in bytes: 8 MiB for each harness the process ran.
        limit: usize,
        /// How many harnesses the process ran.
        harnesses: usize,
    },
    /// A stream could not be read to its end (stable code [`OUTPUT_UNREAD_CODE`],
    /// FR-017-AC-25). An unread stream is not an empty one, so nothing is classified.
    OutputUnread {
        /// The stream that was not read.
        stream: CaptureStream,
        /// What went wrong.
        detail: String,
    },
    /// A group's process was still running when its outer bound, the request timeout times the
    /// member count, elapsed. It was killed. Kani writes its report only at the end, so a killed
    /// group leaves none and no member is classified (FR-017, FR-028-AC-12).
    BatchTimedOut {
        /// How many harnesses the process ran.
        members: usize,
        /// The per-harness request timeout.
        timeout: Duration,
    },
    /// The console holds a playback block headed for a path that is not a member of the group, so
    /// no block in it can be attributed to a member (FR-017-AC-23). A member that fails several
    /// property checks has several blocks under its own path and is not this: it takes the
    /// first, as a single run does.
    PlaybackForNonMember {
        /// The path the block is headed for.
        harness: String,
    },
    /// The witness launch of a two-phase run, or its report, was refused (FR-017-AC-63). The run
    /// is never classified from the proof phase alone.
    WitnessPhaseRefused {
        /// The refusal of the witness launch, report or witness group.
        refusal: Box<KaniExecutionRefusal>,
        /// The proof-phase evidence of each held member, in member order.
        proof_evidence: Vec<KaniExecutionEvidence>,
    },
}

impl KaniExecutionRefusal {
    /// The stable machine-readable code of a refusal that has one: [`OUTPUT_OVER_LIMIT_CODE`] and
    /// [`OUTPUT_UNREAD_CODE`]. The other refusals are told apart by their variant. A refused
    /// witness phase has the code of the refusal it wraps.
    #[must_use]
    pub fn code(&self) -> Option<&'static str> {
        match self {
            Self::WitnessPhaseRefused { refusal, .. } => refusal.code(),
            Self::OutputOverLimit { .. } => Some(OUTPUT_OVER_LIMIT_CODE),
            Self::OutputUnread { .. } => Some(OUTPUT_UNREAD_CODE),
            Self::WrongMethodExecutionRoute
            | Self::BatchCancelled { .. }
            | Self::Tool(_)
            | Self::HarnessNotInCrate { .. }
            | Self::HarnessProvenanceMismatch { .. }
            | Self::BoundTotalityAssertionMismatch { .. }
            | Self::ConcretePlaybackUnsupportedHarness { .. }
            | Self::ConcretePlaybackSourceMismatch { .. }
            | Self::ConcretePlaybackTestNotObserved { .. }
            | Self::Report(_)
            | Self::BatchTimedOut { .. }
            | Self::PlaybackForNonMember { .. } => None,
        }
    }
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongMethodExecutionRoute => formatter.write_str(
                "ordinary and constrained harnesses require their respective execution entries",
            ),
            Self::BatchCancelled { source, members } => write!(
                formatter,
                "the {members}-harness group was cancelled by {source:?}"
            ),
            Self::Tool(error) => write!(formatter, "{error}"),
            Self::HarnessNotInCrate { harness_path } => {
                write!(formatter, "the crate does not contain {harness_path}")
            }
            Self::HarnessProvenanceMismatch { harness_path } => write!(
                formatter,
                "the generated source or identity for {harness_path} changed after generation"
            ),
            Self::BoundTotalityAssertionMismatch { harness_path } => write!(
                formatter,
                "the bound totality assertion for {harness_path} is inconsistent or duplicated"
            ),
            Self::ConcretePlaybackUnsupportedHarness { harness_path } => write!(
                formatter,
                "concrete playback requires a sealed V2 clause harness, got {harness_path}"
            ),
            Self::ConcretePlaybackSourceMismatch { harness_path } => write!(
                formatter,
                "the declared concrete playback test does not preserve {harness_path}"
            ),
            Self::ConcretePlaybackTestNotObserved { test_name } => write!(
                formatter,
                "concrete playback did not report {test_name} as its one passing test"
            ),
            Self::Report(refusal) => write!(formatter, "{refusal}"),
            Self::OutputOverLimit {
                stream,
                limit,
                harnesses,
            } => write!(
                formatter,
                "{OUTPUT_OVER_LIMIT_CODE}: the launcher's {stream} carried more than {limit} \
                 bytes for {harnesses} harness(es)"
            ),
            Self::OutputUnread { stream, detail } => write!(
                formatter,
                "{OUTPUT_UNREAD_CODE}: the launcher's {stream} was not read to its end: {detail}"
            ),
            Self::BatchTimedOut { members, timeout } => write!(
                formatter,
                "a group of {members} harnesses did not finish within {timeout:?} each"
            ),
            Self::PlaybackForNonMember { harness } => write!(
                formatter,
                "the console holds a playback headed for {harness}, which is not in the group"
            ),
            Self::WitnessPhaseRefused { refusal, .. } => {
                write!(formatter, "the witness phase was refused: {refusal}")
            }
        }
    }
}

impl std::error::Error for KaniExecutionRefusal {}

impl From<KaniReportRefusal> for KaniExecutionRefusal {
    fn from(refusal: KaniReportRefusal) -> Self {
        Self::Report(refusal)
    }
}

impl From<KaniToolError> for KaniExecutionRefusal {
    fn from(error: KaniToolError) -> Self {
        Self::Tool(error)
    }
}

/// What ran and the backend-reported outcome of one harness run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniExecutionEvidence {
    /// The per-harness ceilings the launcher was given for this run: the identity memory ceiling and
    /// the identity wall clock (a batch gives the launcher the product, recorded here per harness).
    pub ceilings: crate::kani::identity::ProofCeilings,
    /// Actual backend-tree memory mechanism and observed peak.
    pub memory: MemoryObservation,
    /// Every symbolic argument and its identity bounds.
    pub symbolic_arguments: Vec<crate::kani::identity::SymbolicArgumentBounds>,
    /// Contract role of a contract harness; `None` for an exact-scalar harness, whose claim
    /// has no contract role.
    pub kind: Option<ObligationKind>,
    /// Generated harness path.
    pub harness_path: String,
    /// Invoked launcher path.
    pub launcher_path: String,
    /// Complete argument vector after the launcher.
    pub arguments: Vec<String>,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Solver.
    pub solver: KaniSolver,
    /// Process exit code, or `None` when the process was killed by a signal — including the
    /// kill this module itself sends on [`KaniInconclusiveReason::TimedOut`].
    pub exit_code: Option<i32>,
    /// Backend-reported outcome.
    pub outcome: KaniRunOutcome,
    /// How many checks the report lists as holding: the non-cover checks with status success,
    /// plus, for a precondition harness, its satisfied covers. A collection precondition also needs
    /// its bound totality assertion to succeed before verification.
    /// Zero when the run produced no report. It is the SUCCESS-check count FR-017 defines, which
    /// FR-029 reads from this evidence; FR-030 takes it as an explicit input.
    pub success_checks: u32,
    /// Every check Kani reported, with its class, source location and status, in report order.
    /// Empty when the run produced no report. A consumer attributes proof to source with it.
    pub checks: Vec<KaniCheckResult>,
    /// Present exactly when the harness ran in a process shared with other harnesses: then
    /// `arguments` is that process's complete argument vector, not the single-run one, and this
    /// says so and names who shared it. Absent for a run of one harness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<KaniBatchInvocation>,
    /// The bounded, redacted record of the launcher's streams. Present on evidence whose outcome
    /// is [`KaniInconclusiveReason::NoVerdict`] (why the backend gave no verdict), and on
    /// [`KaniInconclusiveReason::FailedWithoutCounterexample`] evidence whose
    /// [`failure_cause`](Self::failure_cause) is `timeout`, `out_of_memory`, `exit_code` or
    /// `unknown` (a CBMC failure; its phase is `unknown`). Absent for every other outcome, and
    /// for a failed property. It is never an input to the outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<KaniNoVerdictDiagnostics>,
    /// Present exactly when the outcome is [`KaniInconclusiveReason::FailedWithoutCounterexample`]
    /// and the report states an exit status for the harness: why Kani gave no counterexample. It
    /// is never an input to the outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_cause: Option<KaniFailureCause>,
    /// Present exactly when the run launched a witness phase: this evidence is then the witness
    /// phase's (its `arguments` are the witness vector and its outcome is the run's), and this
    /// field is the proof phase's evidence for the same harness identity (FR-028-AC-23).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_phase: Option<Box<KaniExecutionEvidence>>,
}

/// A backend report and its invocation metadata, without an attestation of resource enforcement.
/// Production attaches its actual observation only after the bounded launcher concludes.
#[derive(Debug)]
struct ReportedExecution {
    symbolic_arguments: Vec<crate::kani::identity::SymbolicArgumentBounds>,
    kind: Option<ObligationKind>,
    harness_path: String,
    launcher_path: String,
    arguments: Vec<String>,
    unwind: u32,
    solver: KaniSolver,
    exit_code: Option<i32>,
    outcome: KaniRunOutcome,
    success_checks: u32,
    checks: Vec<KaniCheckResult>,
    batch: Option<KaniBatchInvocation>,
    diagnostics: Option<KaniNoVerdictDiagnostics>,
    failure_cause: Option<KaniFailureCause>,
}

impl ReportedExecution {
    /// The evidence with the memory observation and the per-harness ceilings the launcher was
    /// actually given, so a launch given other ceilings than the identity's records them.
    fn with_launch(
        self,
        memory: MemoryObservation,
        applied: crate::kani::identity::ProofCeilings,
    ) -> KaniExecutionEvidence {
        KaniExecutionEvidence {
            memory,
            ceilings: applied,
            symbolic_arguments: self.symbolic_arguments,
            kind: self.kind,
            harness_path: self.harness_path,
            launcher_path: self.launcher_path,
            arguments: self.arguments,
            unwind: self.unwind,
            solver: self.solver,
            exit_code: self.exit_code,
            outcome: self.outcome,
            success_checks: self.success_checks,
            checks: self.checks,
            batch: self.batch,
            diagnostics: self.diagnostics,
            failure_cause: self.failure_cause,
            proof_phase: None,
        }
    }
}

/// The statement, on a harness's evidence, that its invocation was shared (FR-017-AC-22).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniBatchInvocation {
    /// The `module::harness` path of every harness the process ran, in request order.
    pub members: Vec<String>,
    /// The per-harness request timeout T, in whole seconds rounded up, saturating. It is the
    /// value `--harness-timeout` carried unless T was too large for Kani to accept, in which case
    /// the argument vector omits it and this still names T. A member Kani's own timeout stopped
    /// was held to this many seconds.
    pub timeout_seconds: u64,
}

/// Runs the harness and reports the backend's own outcome.
///
/// The proof vector launches first. When it fails a non-unwinding property the witness vector
/// launches once under the same ceilings, and the verdict is the witness phase's with the proof
/// phase's evidence nested in `proof_phase`; a refused witness phase refuses the run with
/// [`KaniExecutionRefusal::WitnessPhaseRefused`] (FR-017-AC-63).
///
/// The backend is started as the leader of its own process group and held to the wall-clock
/// ceiling of its identity; on a timeout, an output overflow or a normal exit its whole process
/// group is killed. The runner enforces no memory cap: the caller's environment owns it
/// (FR-028-AC-21, ADR-007). A caller killed after the launch leaves the backend's process group
/// running; any bound on that orphan is the caller's environment's.
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    if matches!(request.harness, KaniExecutableHarness::Constrained(_)) {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    }
    let totality_line = require_in_crate(request)?;
    run_single(request, false, totality_line)
}

pub(in crate::kani) fn execute_constrained_raw(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    if !matches!(request.harness, KaniExecutableHarness::Constrained(_)) {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    }
    let totality_line = require_in_crate(request)?;
    run_single(request, false, totality_line)
}

/// Runs one exact concrete-playback unit test with the same process-tree bounds,
/// cancellation, output limits, and owned teardown as a proof run. The caller
/// prepares the playback test in its scratch crate after the original sealed
/// proof has run; this entry point does not classify a Kani proof verdict.
pub fn execute_kani_concrete_playback(
    request: &KaniExecutionRequest<'_>,
    playback: &KaniConcretePlayback<'_>,
) -> Result<KaniConcretePlaybackOutcome, KaniExecutionRefusal> {
    require_concrete_playback_in_crate(request, playback)?;
    let arguments = [
        "kani",
        "playback",
        "--lib",
        "-Z",
        "function-contracts",
        "-Z",
        "concrete-playback",
        "--",
        playback.test_name,
        "--exact",
        "--nocapture",
    ]
    .map(str::to_owned);
    let mut command = Command::new(&request.installation.launcher);
    command
        .args(&arguments)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory);
    let (launch, _) = start(
        request,
        command,
        request.harness.view().ceilings.wall_clock,
        NonZeroUsize::MIN,
    )?;
    let memory = launch.memory;
    match settle(launch.outcome)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            streams,
        } => {
            if exited_successfully && !playback_test_passed(&streams.stdout, playback.test_name) {
                return Err(KaniExecutionRefusal::ConcretePlaybackTestNotObserved {
                    test_name: playback.test_name.to_owned(),
                });
            }
            Ok(KaniConcretePlaybackOutcome::Completed {
                succeeded: exited_successfully,
                exit_code,
                stdout: streams.stdout,
                stderr: streams.stderr,
                memory,
            })
        }
        Concluded::TimedOut => Ok(KaniConcretePlaybackOutcome::TimedOut { memory }),
        Concluded::Cancelled { source } => {
            Ok(KaniConcretePlaybackOutcome::Cancelled { source, memory })
        }
    }
}

/// Libtest must name the exact selected test and report one pass after it.
/// Its exit status alone also describes a successful run with zero matches.
fn playback_test_passed(stdout: &[u8], test_name: &str) -> bool {
    let Ok(stdout) = std::str::from_utf8(stdout) else {
        return false;
    };
    let plain = format!("test {test_name} ... ok");
    let should_panic = format!("test {test_name} - should panic ... ok");
    let mut saw_test = false;
    let mut saw_summary = false;
    for line in stdout.lines() {
        if line.starts_with("test result:") {
            if !saw_test || saw_summary || !line.starts_with("test result: ok. 1 passed; 0 failed;")
            {
                return false;
            }
            saw_summary = true;
        } else if line.starts_with("test ") && line.contains(" ... ") {
            if saw_test || saw_summary || (line != plain && line != should_panic) {
                return false;
            }
            saw_test = true;
        }
    }
    saw_test && saw_summary
}

/// Accept only one declared unit test appended at the generated module's
/// closing brace. Removing those exact bytes must restore the sealed harness.
fn require_concrete_playback_in_crate(
    request: &KaniExecutionRequest<'_>,
    playback: &KaniConcretePlayback<'_>,
) -> Result<(), KaniExecutionRefusal> {
    let harness = request.harness.view();
    if !matches!(request.harness, KaniExecutableHarness::V2Clause(_)) {
        return Err(KaniExecutionRefusal::ConcretePlaybackUnsupportedHarness {
            harness_path: harness.rust.path.clone(),
        });
    }
    if !request.harness.has_valid_provenance() {
        return Err(KaniExecutionRefusal::HarnessProvenanceMismatch {
            harness_path: harness.rust.path.clone(),
        });
    }
    let mismatch = || KaniExecutionRefusal::ConcretePlaybackSourceMismatch {
        harness_path: harness.rust.path.clone(),
    };
    let (module, _) = harness.selection.rsplit_once("::").ok_or_else(mismatch)?;
    let (test_module, test_function) = playback.test_name.rsplit_once("::").ok_or_else(mismatch)?;
    let parsed = syn::parse_str::<syn::ItemFn>(playback.appended_test).map_err(|_| mismatch())?;
    if test_module != module
        || parsed.sig.ident != test_function
        || parsed
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident("test"))
            .count()
            != 1
    {
        return Err(mismatch());
    }
    let prefix = harness
        .rust
        .contents
        .strip_suffix("}\n")
        .ok_or_else(mismatch)?;
    let expanded = format!("{prefix}{}}}\n", playback.appended_test);
    let library_path = request.crate_directory.join("src/lib.rs");
    let library = read_file(KaniTool::Library, &library_path).map_err(|_| mismatch())?;
    let library = String::from_utf8(library).map_err(|_| mismatch())?;
    if library.matches(&expanded).count() != 1
        || library.matches(playback.appended_test).count() != 1
        || library
            .replacen(playback.appended_test, "", 1)
            .matches(&harness.rust.contents)
            .count()
            != 1
    {
        return Err(mismatch());
    }
    Ok(())
}

/// Runs a non-constrained harness over the prepared library target (`--lib`). The runner makes
/// the source tree read-only for no one (ADR-007): it is no sandbox, so the caller's environment
/// owns any read-only mount, and the staged-snapshot comparison in `scalar.rs` is the guard that
/// the run left the tree unchanged.
pub(crate) fn execute_kani_obligation_library(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    if matches!(
        request.harness,
        super::harness::KaniExecutableHarness::Constrained(_)
    ) {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    }
    let totality_line = require_in_crate(request)?;
    run_single(request, true, totality_line)
}

/// Refuses a request whose crate's library source does not contain its harness's generated
/// source byte for byte.
fn require_in_crate(
    request: &KaniExecutionRequest<'_>,
) -> Result<Option<u32>, KaniExecutionRefusal> {
    let harness = request.harness.view();
    if !request.harness.has_valid_provenance() {
        return Err(KaniExecutionRefusal::HarnessProvenanceMismatch {
            harness_path: harness.rust.path.clone(),
        });
    }
    let library_path = request.crate_directory.join("src").join("lib.rs");
    let library = read_file(KaniTool::Library, &library_path).map_err(|_| {
        KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        }
    })?;
    let library = String::from_utf8_lossy(&library);
    if !library.contains(&harness.rust.contents) {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        });
    }
    let totality_line = if let Some(marker) = request.harness.collection_precondition() {
        Some(
            bound_totality_line(
                &library,
                &harness.rust.contents,
                marker.totality_assertion_line,
            )
            .ok_or_else(|| KaniExecutionRefusal::BoundTotalityAssertionMismatch {
                harness_path: harness.rust.path.clone(),
            })?,
        )
    } else {
        None
    };
    Ok(totality_line)
}

fn bound_totality_line(library: &str, source: &str, artifact_line: u32) -> Option<u32> {
    let source_line = usize::try_from(artifact_line.checked_sub(1)?).ok()?;
    if source.lines().nth(source_line)?.trim()
        != crate::kani::identity::collection_precondition_totality_assertion()
    {
        return None;
    }
    let mut occurrences = library.match_indices(source);
    let (start, _) = occurrences.next()?;
    if occurrences.next().is_some() {
        return None;
    }
    let prefix_lines = library[..start]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count();
    u32::try_from(prefix_lines).ok()?.checked_add(artifact_line)
}

/// Starts `command` with the launcher's output captured for `harnesses` harnesses, and maps a
/// failure to start it to the refusal that names the launcher. Also returns the per-harness
/// ceilings the launcher was given: the identity's declared memory ceiling, and `timeout` divided among `harnesses`
/// (the identity's wall clock when that division does not fit).
fn start(
    request: &KaniExecutionRequest<'_>,
    command: Command,
    timeout: Duration,
    harnesses: NonZeroUsize,
) -> Result<(BoundedLaunch, crate::kani::identity::ProofCeilings), KaniExecutionRefusal> {
    let deadline = Instant::now().checked_add(timeout);
    if !deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        request.installation.require_executable()?;
    }
    let identity = request.harness.view().ceilings;
    let applied = crate::kani::identity::ProofCeilings {
        memory_bytes: identity.memory_bytes,
        wall_clock: u32::try_from(harnesses.get())
            .ok()
            .and_then(|members| timeout.checked_div(members))
            .filter(|_| timeout != Duration::MAX)
            .unwrap_or(identity.wall_clock),
    };
    run_bounded_launcher(
        command,
        timeout,
        harnesses,
        deadline,
        request.cancel,
        request.installation.owners(),
    )
    .map(|launch| (launch, applied))
    .map_err(|error| {
        KaniExecutionRefusal::Tool(KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        })
    })
}

/// Reads the report `launch` exported to `report_path` when it completed, then removes that file
/// whatever the read found: only this run's own file is removed.
fn take_report(
    launch: &LaunchOutcome,
    report_path: &Path,
) -> Result<Option<Vec<u8>>, KaniReportRefusal> {
    let report = match launch {
        LaunchOutcome::Completed { .. } => read_report(report_path),
        LaunchOutcome::Cancelled { .. }
        | LaunchOutcome::TimedOut
        | LaunchOutcome::OutputOverLimit { .. }
        | LaunchOutcome::OutputUnread { .. } => Ok(None),
    };
    let _ = fs::remove_file(report_path);
    report
}

/// The evidence of one harness from its `run` of a process that exited with `exit_code`.
fn evidence_of(
    request: &KaniExecutionRequest<'_>,
    harness: &HarnessView<'_>,
    arguments: Vec<String>,
    exit_code: Option<i32>,
    mut run: ClassifiedRun,
    batch: Option<KaniBatchInvocation>,
    record: Record,
    totality_line: Option<u32>,
) -> ReportedExecution {
    if matches!(run.outcome, KaniRunOutcome::Verified)
        && request.harness.collection_precondition().is_some()
        && !totality_line.is_some_and(|line| totality_check_succeeded(&run.checks, line))
    {
        run.outcome = KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::VacuousProof,
        };
    }
    ReportedExecution {
        symbolic_arguments: request.harness.symbolic_arguments(),
        kind: harness.kind,
        harness_path: harness.rust.path.clone(),
        launcher_path: request.installation.launcher.display().to_string(),
        arguments,
        unwind: harness.unwind,
        solver: harness.solver,
        exit_code,
        outcome: run.outcome,
        success_checks: run.success_checks,
        checks: run.checks,
        batch,
        diagnostics: record.diagnostics,
        failure_cause: record.failure_cause,
    }
}

fn totality_check_succeeded(checks: &[KaniCheckResult], line: u32) -> bool {
    checks.iter().any(|check| {
        matches!(&check.class, KaniCheckClass::Other(class) if class.as_str() == "assertion")
            && check.status == KaniCheckStatus::Success
            && check.location.file == "src/lib.rs"
            && check.location.line == Some(line)
    })
}

/// What the evidence retains beyond the classified run: the diagnostic record of a `NoVerdict`
/// run, or the failure cause of a `FailedWithoutCounterexample` run with the record that goes with
/// a CBMC failure. Both are decided after the outcome.
struct Record {
    diagnostics: Option<KaniNoVerdictDiagnostics>,
    failure_cause: Option<KaniFailureCause>,
}

impl Record {
    /// The record of `run`, whose report stated `exit_status` for the harness and whose launcher
    /// printed `streams`. The exit status is read only here, after the outcome is decided. `diagnose` builds the member's
    /// diagnostic record from those streams and is called only when one is retained.
    fn of(
        run: &ClassifiedRun,
        exit_status: Option<&str>,
        streams: Option<&CapturedStreams>,
        directories: &KnownDirectories<'_>,
        diagnose: impl FnOnce(&CapturedStreams) -> KaniNoVerdictDiagnostics,
    ) -> Self {
        let failure_cause = is_failed_without_counterexample(run)
            .then_some(exit_status)
            .flatten()
            .map(|status| KaniFailureCause::from_exit_status(status, directories));
        let cbmc_failed = failure_cause
            .as_ref()
            .is_some_and(KaniFailureCause::is_cbmc_failure);
        let diagnostics = streams
            .filter(|_| cbmc_failed || is_no_verdict(run))
            .map(diagnose);
        Self {
            diagnostics,
            failure_cause,
        }
    }
}

/// The request's directories to redact a run's output against.
fn directories_of<'a>(request: &'a KaniExecutionRequest<'_>) -> KnownDirectories<'a> {
    KnownDirectories {
        crate_directory: request.crate_directory,
        target_directory: request.target_directory,
        home_directory: request.home_directory,
    }
}

/// Which launch of a two-phase run (FR-017-AC-63) an execution step is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    /// The launch under the identity's proof option vector.
    Proof,
    /// The launch under the identity's witness option vector.
    Witness,
}

impl HarnessView<'_> {
    fn phase_options(&self, phase: Phase) -> &[String] {
        match phase {
            Phase::Proof => self.options,
            Phase::Witness => self.witness_options,
        }
    }
}

/// The failed non-unwinding checks of `checks`: their class and source location.
fn failed_properties(checks: &[KaniCheckResult]) -> Vec<(&KaniCheckClass, &KaniCheckLocation)> {
    checks
        .iter()
        .filter(|check| {
            matches!(check.class, KaniCheckClass::Other(_))
                && check.status == KaniCheckStatus::Failure
        })
        .map(|check| (&check.class, &check.location))
        .collect()
}

/// Whether the proof phase `proof` reported a failed non-unwinding check that only a witness
/// phase may settle. An exhausted unwind bound classifies on its own and so is never held.
fn wants_witness(proof: &KaniExecutionEvidence) -> bool {
    matches!(
        proof.outcome,
        KaniRunOutcome::Falsified { .. }
            | KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
    ) && !failed_properties(&proof.checks).is_empty()
}

/// The run's evidence once its witness phase has concluded (FR-017-AC-5, FR-017-AC-63): the
/// witness evidence, carrying the proof evidence, with the outcome the two phases agree on.
fn conclude_witness(
    proof: KaniExecutionEvidence,
    mut witness: KaniExecutionEvidence,
) -> KaniExecutionEvidence {
    let same_failures = {
        let (proved, witnessed) = (
            failed_properties(&proof.checks),
            failed_properties(&witness.checks),
        );
        proved.iter().all(|failure| witnessed.contains(failure))
            && witnessed.iter().all(|failure| proved.contains(failure))
    };
    let agrees = match &witness.outcome {
        KaniRunOutcome::Falsified { .. } => same_failures,
        // A report stating `out_of_memory` lists no checks, so it cannot reproduce the proof
        // phase's failures; the environment ended the witness, and that is the outcome
        // (FR-017-AC-63(c), FR-028-AC-25).
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample,
        } => same_failures || matches!(witness.failure_cause, Some(KaniFailureCause::OutOfMemory)),
        KaniRunOutcome::Cancelled { .. }
        | KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut | KaniInconclusiveReason::NoVerdict,
        } => true,
        KaniRunOutcome::Verified
        | KaniRunOutcome::CoverUnsatisfied { .. }
        | KaniRunOutcome::Inconclusive {
            reason:
                KaniInconclusiveReason::MissingCoverSummary
                | KaniInconclusiveReason::VacuousProof
                | KaniInconclusiveReason::UnwindBoundExhausted
                | KaniInconclusiveReason::WitnessDisagreed,
        } => false,
    };
    if !agrees {
        witness.outcome = KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::WitnessDisagreed,
        };
        witness.diagnostics = None;
        witness.failure_cause = None;
    }
    witness.proof_phase = Some(Box::new(proof));
    witness
}

/// A refused witness step: the refusal, wrapped with the proof evidence it must not drop.
fn witness_refused(
    refusal: KaniExecutionRefusal,
    proof_evidence: Vec<KaniExecutionEvidence>,
) -> KaniExecutionRefusal {
    KaniExecutionRefusal::WitnessPhaseRefused {
        refusal: Box::new(refusal),
        proof_evidence,
    }
}

/// The two-phase run of one harness whose presence in the crate has been checked.
fn run_single(
    request: &KaniExecutionRequest<'_>,
    library_only: bool,
    totality_line: Option<u32>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let proof = run_phase_single(request, Phase::Proof, library_only, totality_line)?;
    if !wants_witness(&proof) {
        return Ok(proof);
    }
    match run_phase_single(request, Phase::Witness, library_only, totality_line) {
        Ok(witness) => Ok(conclude_witness(proof, witness)),
        Err(refusal) => Err(witness_refused(refusal, vec![proof])),
    }
}

/// One launch of one harness under the options of `phase`.
fn run_phase_single(
    request: &KaniExecutionRequest<'_>,
    phase: Phase,
    library_only: bool,
    totality_line: Option<u32>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let report_path = fresh_report_path(request.target_directory);
    remove_stale_report(&report_path)?;
    let (arguments, command) = launch_command(request, phase, &report_path, library_only);
    let (launch, applied) = start(
        request,
        command,
        request.harness.view().ceilings.wall_clock,
        NonZeroUsize::MIN,
    )?;
    let report = take_report(&launch.outcome, &report_path);
    finish_single(request, arguments, launch.outcome, report?, totality_line)
        .map(|reported| reported.with_launch(launch.memory, applied))
}

/// Classify one captured launch and its own report, without asserting memory enforcement.
fn finish_single(
    request: &KaniExecutionRequest<'_>,
    arguments: Vec<String>,
    outcome: LaunchOutcome,
    report: Option<Vec<u8>>,
    totality_line: Option<u32>,
) -> Result<ReportedExecution, KaniExecutionRefusal> {
    let harness = request.harness.view();
    let (run, exit_code, streams) = launch_evidence(outcome, report.as_deref(), harness.kind)?;
    let exit_status = single_exit_status(&run, report.as_deref());
    let record = Record::of(
        &run,
        exit_status.as_deref(),
        streams.as_ref(),
        &directories_of(request),
        |streams| {
            process_output(
                request,
                streams,
                std::slice::from_ref(&harness.selection),
                &arguments,
                report.is_some(),
            )
            .analyse()
            .diagnostics_of(&harness.selection)
        },
    );
    Ok(evidence_of(
        request,
        &harness,
        arguments,
        exit_code,
        run,
        None,
        record,
        totality_line,
    ))
}

/// Whether the run's outcome is `FailedWithoutCounterexample`, the only one that keeps a cause.
fn is_failed_without_counterexample(run: &ClassifiedRun) -> bool {
    matches!(
        run.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        }
    )
}

/// The exit status a single run's report states for its harness, read only for a run already
/// classified `FailedWithoutCounterexample`. That run's report parsed when it was classified.
fn single_exit_status(run: &ClassifiedRun, report: Option<&[u8]>) -> Option<String> {
    if !is_failed_without_counterexample(run) {
        return None;
    }
    KaniHarnessReport::parse(report?).ok()?.exit_status
}

/// Whether the run's outcome is `NoVerdict`, which carries a diagnostic record.
fn is_no_verdict(run: &ClassifiedRun) -> bool {
    matches!(
        run.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict
        }
    )
}

/// What one launcher process printed, with the request's directories to redact it against.
fn process_output<'a>(
    request: &'a KaniExecutionRequest<'_>,
    streams: &'a CapturedStreams,
    selections: &'a [String],
    arguments: &'a [String],
    report_exported: bool,
) -> ProcessOutput<'a> {
    ProcessOutput {
        stdout: &streams.stdout,
        stderr: &streams.stderr,
        selections,
        arguments,
        report_exported,
        directories: directories_of(request),
    }
}

/// What one launcher process (one group) of a batch yielded: the harnesses it ran and their
/// evidence or the refusal of the whole process.
///
/// A proof-phase run lists only the members its report classified, and is absent when it
/// classified none. A witness-phase run lists the members its proof phase held; its evidence
/// nests each member's proof-phase evidence, which came from a different process.
#[derive(Debug)]
pub struct KaniGroupRun {
    /// The position in the request list of each harness the process ran, in request order.
    pub members: Vec<usize>,
    /// One evidence record per member, in the same order, or why the process's harnesses have
    /// none.
    pub evidence: Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal>,
}

/// Runs several harnesses, as few launcher processes as it can, and reports one evidence record
/// for each (FR-017-AC-21 to FR-017-AC-23).
///
/// Harnesses are grouped by equal option vector (their `--harness <path> --exact` selection
/// removed) and equal identity ceilings; a process also has one launcher, one crate directory and
/// one target directory and one caller-owned cancellation handle identity, so those must be
/// equal as well. A group of one is the single run of
/// [`execute_kani_obligation`], with its argument vector. A larger group is one process given one
/// `--harness <module::harness> --exact` pair per member in request order, then
/// `--harness-timeout <T>` in whole seconds rounded up (omitted when Kani 0.68 would refuse the
/// value), then the shared options. The process is bounded to the member count times T, a product
/// that does not fit never elapsing.
///
/// A harness the crate does not contain refuses the whole call before any process starts. After
/// that each process has its own result. The proof-phase processes come first, in order of their
/// first member. A member whose proof phase fails a non-unwinding property is held, and the held
/// members are regrouped by witness vector and identity ceilings into witness-phase processes,
/// which follow in order of their first held member; a witness process may therefore run members
/// of more than one proof process, and its members are not in request order across runs. Each
/// process's report is split by the
/// `module::harness` path of each entry, each member is classified from its own entry and its own
/// console playback, and a report that lacks, repeats or adds a harness, a playback that cannot be
/// attributed, an output stream over 8 MiB per member and an outer bound that elapsed each refuse
/// that process (group) and classify none of its members. The generator does not split or retry a
/// refused group.
///
/// Every group runs under the same minimal ownership as [`execute_kani_obligation`]: its own
/// process group, the wall-clock bound and no runner-side memory cap.
pub fn execute_kani_obligations(
    requests: &[KaniExecutionRequest<'_>],
) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
    if requests
        .iter()
        .any(|request| matches!(request.harness, KaniExecutableHarness::Constrained(_)))
    {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    }
    let totality_lines = requests
        .iter()
        .map(require_in_crate)
        .collect::<Result<Vec<_>, _>>()?;
    let mut runs = Vec::new();
    let mut held: Vec<(usize, &KaniExecutionRequest<'_>, KaniExecutionEvidence)> = Vec::new();
    for group in plan_groups(requests.iter().enumerate(), Phase::Proof) {
        let members: Vec<usize> = group.iter().map(|(position, _)| *position).collect();
        let evidence = match group.as_slice() {
            [(position, only)] => {
                run_phase_single(only, Phase::Proof, false, totality_lines[*position])
                    .map(|evidence| vec![evidence])
            }
            _ => run_group(&group, Phase::Proof, &totality_lines),
        };
        match evidence {
            Ok(evidence) => {
                let mut kept_members = Vec::new();
                let mut classified = Vec::new();
                for ((position, request), evidence) in group.into_iter().zip(evidence) {
                    if wants_witness(&evidence) {
                        held.push((position, request, evidence));
                    } else {
                        kept_members.push(position);
                        classified.push(evidence);
                    }
                }
                if !kept_members.is_empty() {
                    runs.push(KaniGroupRun {
                        members: kept_members,
                        evidence: Ok(classified),
                    });
                }
            }
            Err(refusal) => runs.push(KaniGroupRun {
                members,
                evidence: Err(refusal),
            }),
        }
    }
    let witness_groups = plan_groups(
        held.iter()
            .map(|(position, request, _)| (*position, *request)),
        Phase::Witness,
    );
    for group in witness_groups {
        let members: Vec<usize> = group.iter().map(|(position, _)| *position).collect();
        let witnessed = match group.as_slice() {
            [(position, only)] => {
                run_phase_single(only, Phase::Witness, false, totality_lines[*position])
                    .map(|evidence| vec![evidence])
            }
            _ => run_group(&group, Phase::Witness, &totality_lines),
        };
        let mut proofs = Vec::with_capacity(members.len());
        for position in &members {
            if let Some(at) = held.iter().position(|(held, _, _)| held == position) {
                proofs.push(held.remove(at).2);
            }
        }
        let evidence = match witnessed {
            Ok(witnessed) => Ok(proofs
                .into_iter()
                .zip(witnessed)
                .map(|(proof, witness)| conclude_witness(proof, witness))
                .collect()),
            Err(refusal) => Err(witness_refused(refusal, proofs)),
        };
        runs.push(KaniGroupRun { members, evidence });
    }
    Ok(runs)
}

/// Stable groups in first-member order, each retaining request order, of the members that can
/// share one launch of `phase`.
fn plan_groups<'a, 'b>(
    requests: impl Iterator<Item = (usize, &'a KaniExecutionRequest<'b>)>,
    phase: Phase,
) -> Vec<Vec<(usize, &'a KaniExecutionRequest<'b>)>> {
    let mut groups: Vec<Vec<(usize, &KaniExecutionRequest<'_>)>> = Vec::new();
    for member in requests {
        match groups.iter_mut().find(|group| {
            group
                .first()
                .is_some_and(|(_, first)| shares_process(first, member.1, phase))
        }) {
            Some(group) => group.push(member),
            None => groups.push(vec![member]),
        }
    }
    groups
}

/// Whether two requests can be run by one launcher process: the same option vector once the
/// harness selection is removed, the same identity ceilings, and the same launcher, crate and target
/// directory and owner dependencies, plus identical caller-owned cancellation handles.
fn shares_process(
    first: &KaniExecutionRequest<'_>,
    other: &KaniExecutionRequest<'_>,
    phase: Phase,
) -> bool {
    first.cancel == other.cancel
        && first.harness.view().ceilings == other.harness.view().ceilings
        && first.installation == other.installation
        && first.crate_directory == other.crate_directory
        && first.target_directory == other.target_directory
        && first.home_directory == other.home_directory
        && without_selection(first.harness.view().phase_options(phase))
            == without_selection(other.harness.view().phase_options(phase))
}

/// `options` with every `--harness <path> --exact` selection removed.
fn without_selection(options: &[String]) -> Vec<String> {
    let mut kept = Vec::with_capacity(options.len());
    let mut remaining = options.iter().peekable();
    while let Some(option) = remaining.next() {
        if option == "--harness" {
            remaining.next();
            remaining.next_if(|next| *next == "--exact");
        } else {
            kept.push(option.clone());
        }
    }
    kept
}

/// `timeout` in whole seconds, rounded up, saturating.
fn whole_seconds(timeout: Duration) -> u64 {
    timeout
        .as_secs()
        .saturating_add(u64::from(timeout.subsec_nanos() > 0))
}

/// The wall-clock bound of a process that runs `members` harnesses each held to `timeout`: their
/// product. A product that does not fit is [`Duration::MAX`], which never elapses
/// (FR-017-AC-15).
fn outer_bound(timeout: Duration, members: usize) -> Duration {
    u32::try_from(members)
        .ok()
        .and_then(|members| timeout.checked_mul(members))
        .unwrap_or(Duration::MAX)
}

/// The argument vector and [`Command`] recipe for one process that
/// runs every harness of `selections`.
fn batch_launch_command(
    first: &KaniExecutionRequest<'_>,
    selections: &[String],
    phase: Phase,
    report_path: &Path,
) -> (Vec<String>, Command) {
    let mut arguments = vec!["kani".to_owned()];
    for selection in selections {
        arguments.extend([
            "--harness".to_owned(),
            selection.clone(),
            "--exact".to_owned(),
        ]);
    }
    // Kani 0.68 refuses a value above `u32::MAX` (exit 2, no report); then there is no per-member
    // bound, only the process's own.
    if let Ok(seconds) = u32::try_from(whole_seconds(first.harness.view().ceilings.wall_clock)) {
        arguments.extend(["--harness-timeout".to_owned(), seconds.to_string()]);
    }
    arguments.extend(without_selection(first.harness.view().phase_options(phase)));
    arguments.extend([
        "-Z".to_owned(),
        "unstable-options".to_owned(),
        "--export-json".to_owned(),
        report_path.display().to_string(),
    ]);
    let mut command = Command::new(&first.installation.launcher);
    command
        .args(&arguments)
        .env("CARGO_TARGET_DIR", first.target_directory)
        .current_dir(first.crate_directory);
    (arguments, command)
}

/// The run of a group of more than one harness, in one process.
fn run_group(
    group: &[(usize, &KaniExecutionRequest<'_>)],
    phase: Phase,
    totality_lines: &[Option<u32>],
) -> Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal> {
    let (Some((_, first)), Some(count)) = (group.first(), NonZeroUsize::new(group.len())) else {
        return Ok(Vec::new());
    };
    let views: Vec<HarnessView<'_>> = group
        .iter()
        .map(|(_, request)| request.harness.view())
        .collect();
    let selections: Vec<String> = views.iter().map(|view| view.selection.clone()).collect();
    let report_path = fresh_report_path(first.target_directory);
    remove_stale_report(&report_path)?;
    let (arguments, command) = batch_launch_command(first, &selections, phase, &report_path);
    let (launch, applied) = start(
        first,
        command,
        outer_bound(first.harness.view().ceilings.wall_clock, group.len()),
        count,
    )?;
    let report = take_report(&launch.outcome, &report_path);
    let (exited_successfully, exit_code, streams) = match settle(launch.outcome)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            streams,
        } => (exited_successfully, exit_code, streams),
        Concluded::Cancelled { source } => {
            return Err(KaniExecutionRefusal::BatchCancelled {
                source,
                members: group.len(),
            });
        }
        Concluded::TimedOut => {
            return Err(KaniExecutionRefusal::BatchTimedOut {
                members: group.len(),
                timeout: first.harness.view().ceilings.wall_clock,
            })
        }
    };
    finish_group(
        group,
        arguments,
        exited_successfully,
        exit_code,
        &streams,
        report?,
        totality_lines,
    )
    .map(|reported| {
        reported
            .into_iter()
            .map(|run| run.with_launch(launch.memory.clone(), applied))
            .collect()
    })
}

/// Interpret only a completed group's report and console; no resource-enforcement claim.
fn finish_group(
    group: &[(usize, &KaniExecutionRequest<'_>)],
    arguments: Vec<String>,
    exited_successfully: bool,
    exit_code: Option<i32>,
    streams: &CapturedStreams,
    report: Option<Vec<u8>>,
    totality_lines: &[Option<u32>],
) -> Result<Vec<ReportedExecution>, KaniExecutionRefusal> {
    let Some((_, first)) = group.first() else {
        return Ok(Vec::new());
    };
    let views: Vec<_> = group
        .iter()
        .map(|(_, request)| request.harness.view())
        .collect();
    let selections: Vec<_> = views.iter().map(|view| view.selection.clone()).collect();
    let invocation = KaniBatchInvocation {
        members: selections.clone(),
        timeout_seconds: whole_seconds(first.harness.view().ceilings.wall_clock),
    };
    let report_exported = report.is_some();
    let output = process_output(first, streams, &selections, &arguments, report_exported);
    // Read once for the whole group, and only when some member has a record to carry.
    let shared = std::cell::OnceCell::new();
    let evidence = |run: Vec<(ClassifiedRun, Option<&str>)>| -> Vec<ReportedExecution> {
        group
            .iter()
            .zip(&views)
            .zip(run)
            .map(|(((position, request), view), (run, exit_status))| {
                let record = Record::of(
                    &run,
                    exit_status,
                    Some(streams),
                    &output.directories,
                    |_| {
                        shared
                            .get_or_init(|| output.analyse())
                            .diagnostics_of(&view.selection)
                    },
                );
                evidence_of(
                    request,
                    view,
                    arguments.clone(),
                    exit_code,
                    run,
                    Some(invocation.clone()),
                    record,
                    totality_lines.get(*position).copied().flatten(),
                )
            })
            .collect()
    };
    let Some(report) = report else {
        return if exited_successfully {
            Err(KaniReportRefusal::Missing.into())
        } else {
            // No member's entry exists to tell the members apart.
            Ok(evidence(
                group
                    .iter()
                    .map(|_| {
                        let run = ClassifiedRun {
                            outcome: KaniRunOutcome::Inconclusive {
                                reason: KaniInconclusiveReason::NoVerdict,
                            },
                            success_checks: 0,
                            checks: Vec::new(),
                        };
                        (run, None)
                    })
                    .collect(),
            ))
        };
    };
    let members = members_in_request_order(&selections, &KaniHarnessReport::parse_batch(&report)?)?;
    let text = streams.merged_text();
    let blocks = playback_blocks(&text);
    if let Some(block) = blocks
        .iter()
        .find(|block| !selections.contains(&block.harness.to_owned()))
    {
        return Err(KaniExecutionRefusal::PlaybackForNonMember {
            harness: block.harness.to_owned(),
        });
    }
    // Kani prints one counterexample block per distinct failing valuation, so a member that fails
    // two checks at different valuations has two blocks under its own path (at one valuation,
    // one block). Each member takes the first, as a single run
    // does; only a block for a path that is not a member cannot be attributed.
    // Kani exits 1 for any failed harness and for its own errors alike: a success is believed
    // beside a non-zero exit only when some entry states the failure that exit stands for.
    let process_succeeded = exited_successfully
        || members
            .iter()
            .any(|member| member.report.status == KaniHarnessStatus::Failure);
    Ok(evidence(
        members
            .iter()
            .zip(&views)
            .map(|(member, view)| {
                let run = classify_member(
                    process_succeeded,
                    member,
                    || counterexample_from_blocks(&blocks, Some(&view.selection)),
                    view.kind,
                );
                (run, member.report.exit_status.as_deref())
            })
            .collect(),
    ))
}

/// A launch that ended with output that can be read: the launcher exited, or the run's budget
/// elapsed.
enum Concluded {
    /// The launcher exited on its own.
    Completed {
        exited_successfully: bool,
        exit_code: Option<i32>,
        streams: CapturedStreams,
    },
    /// The budget elapsed and the group was killed.
    TimedOut,
    Cancelled {
        source: KaniCancellationSource,
    },
}

/// The launch as a conclusion, or the refusal of a launch whose output cannot be vouched for: a
/// stream over its limit, or one that was not read.
fn settle(launch: LaunchOutcome) -> Result<Concluded, KaniExecutionRefusal> {
    match launch {
        LaunchOutcome::Completed {
            exited_successfully,
            exit_code,
            streams,
        } => Ok(Concluded::Completed {
            exited_successfully,
            exit_code,
            streams,
        }),
        LaunchOutcome::TimedOut => Ok(Concluded::TimedOut),
        LaunchOutcome::Cancelled { source } => Ok(Concluded::Cancelled { source }),
        LaunchOutcome::OutputOverLimit {
            stream,
            limit,
            harnesses,
        } => Err(KaniExecutionRefusal::OutputOverLimit {
            stream,
            limit,
            harnesses,
        }),
        LaunchOutcome::OutputUnread { stream, detail } => {
            Err(KaniExecutionRefusal::OutputUnread { stream, detail })
        }
    }
}

/// Builds the exact argument vector and backend recipe [`execute_kani_obligation`] launches for
/// `request`, without spawning it, for owning-module argument-vector checks.
///
/// The vector is the harness identity's option vector followed by the flags that make Kani export
/// its report to a file in the target directory, which is where the verdict is read from. The
/// file's name is unique to this call (the last argument), so two runs sharing a target
/// directory never write, remove or read each other's report.
#[cfg(test)]
fn kani_launch_command(request: &KaniExecutionRequest<'_>) -> (Vec<String>, Command) {
    launch_command(
        request,
        Phase::Proof,
        &fresh_report_path(request.target_directory),
        false,
    )
}

fn launch_command(
    request: &KaniExecutionRequest<'_>,
    phase: Phase,
    report_path: &Path,
    library_only: bool,
) -> (Vec<String>, Command) {
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(request.harness.view().phase_options(phase).iter().cloned());
    if library_only {
        arguments.push("--lib".to_owned());
    }
    arguments.extend([
        "-Z".to_owned(),
        "unstable-options".to_owned(),
        "--export-json".to_owned(),
        report_path.display().to_string(),
    ]);
    let mut command = Command::new(&request.installation.launcher);
    command
        .args(&arguments)
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory);
    (arguments, command)
}

/// Maps a concluded [`LaunchOutcome`] and the report its run exported to the `(run, exit_code)`
/// pair [`KaniExecutionEvidence`] stores, exactly as `execute_kani_obligation` does. `report` is
/// the exported report's bytes, `None` when the run exported none; a timed-out run is killed
/// before it can, so its report is not consulted. `kind` is the harness's contract role (`None`
/// for an exact-scalar harness); see [`classify_kani_run`] for how it affects the zero-checks
/// rule. Kept as its own pure function so the mapping is tested directly with a value rather than
/// a real subprocess.
///
/// A launch stopped over a stream's limit or over a stream that was not read is the refusal
/// [`KaniExecutionRefusal::OutputOverLimit`] or [`KaniExecutionRefusal::OutputUnread`], never an
/// outcome.
pub(super) fn launch_evidence(
    launch: LaunchOutcome,
    report: Option<&[u8]>,
    kind: Option<ObligationKind>,
) -> Result<(ClassifiedRun, Option<i32>, Option<CapturedStreams>), KaniExecutionRefusal> {
    let stopped = |reason| {
        (
            ClassifiedRun {
                outcome: KaniRunOutcome::Inconclusive { reason },
                success_checks: 0,
                checks: Vec::new(),
            },
            None,
            None,
        )
    };
    match settle(launch)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            streams,
        } => Ok((
            classify_kani_run(
                exited_successfully,
                report,
                select_counterexample(&streams.merged_text(), None),
                kind,
            )?,
            exit_code,
            Some(streams),
        )),
        Concluded::Cancelled { source } => Ok((
            ClassifiedRun {
                outcome: KaniRunOutcome::Cancelled { source },
                success_checks: 0,
                checks: Vec::new(),
            },
            None,
            None,
        )),
        Concluded::TimedOut => Ok(stopped(KaniInconclusiveReason::TimedOut)),
    }
}

fn read_file(tool: KaniTool, path: &Path) -> Result<Vec<u8>, KaniToolError> {
    if !path.is_file() {
        return Err(KaniToolError::Missing {
            tool,
            path: path.to_path_buf(),
        });
    }
    fs::read(path).map_err(|error| KaniToolError::Io {
        tool,
        path: path.to_path_buf(),
        error,
    })
}

#[cfg(test)]
mod report_fixture {
    use super::*;

    pub(super) type FixtureExecution = KaniExecutionEvidence;

    /// Bounded public execution, grouped as the public batch entry groups it.
    pub(super) struct ReportGroup {
        pub(super) members: Vec<usize>,
        pub(super) reports: Result<Vec<FixtureExecution>, KaniExecutionRefusal>,
    }

    pub(super) fn single(
        request: &KaniExecutionRequest<'_>,
    ) -> Result<FixtureExecution, KaniExecutionRefusal> {
        execute_kani_obligation(request)
    }

    pub(super) fn groups(
        requests: &[KaniExecutionRequest<'_>],
    ) -> Result<Vec<ReportGroup>, KaniExecutionRefusal> {
        execute_kani_obligations(requests).map(|groups| {
            groups
                .into_iter()
                .map(|group| ReportGroup {
                    members: group.members,
                    reports: group.evidence,
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use std::path::PathBuf;

    use super::*;
    use crate::{
        core::test_support::discover_scratch,
        kani::{
            identity::StateFrameProperty,
            test_support::{
                report, state_frame_harness, write_launcher, COVER_NO, COVER_OK, PASSED,
            },
        },
    };

    /// FR-017-AC-61 (TC-027): only the unique bound assertion's successful check can turn a
    /// collection precondition's satisfied cover into proof evidence.
    #[trace("FR-017-AC-61", "TC-027")]
    #[test]
    fn collection_totality_binding_rejects_foreign_or_missing_checks() {
        let assertion = crate::kani::identity::collection_precondition_totality_assertion();
        let source = format!("mod proof {{\n    {assertion}\n}}\n");
        let library = format!("// prefix\n{source}");
        assert_eq!(bound_totality_line(&library, &source, 2), Some(3));
        assert_eq!(bound_totality_line(&library, &source, 1), None);
        assert_eq!(
            bound_totality_line(&format!("{library}{source}"), &source, 2),
            None
        );

        let check = KaniCheckResult {
            id: 1,
            class: KaniCheckClass::from("assertion".to_owned()),
            location: crate::kani::output::report::KaniCheckLocation {
                file: "src/lib.rs".to_owned(),
                line: Some(3),
            },
            status: KaniCheckStatus::Success,
        };
        assert!(totality_check_succeeded(std::slice::from_ref(&check), 3));
        assert!(!totality_check_succeeded(std::slice::from_ref(&check), 2));
        let mut cover = check.clone();
        cover.class = KaniCheckClass::Cover;
        assert!(!totality_check_succeeded(&[cover], 3));
        let mut foreign = check.clone();
        foreign.location.file = "src/other.rs".to_owned();
        assert!(!totality_check_succeeded(&[foreign], 3));
        let mut failed = check;
        failed.status = KaniCheckStatus::Unreachable;
        assert!(!totality_check_succeeded(&[failed], 3));
    }

    /// A parameter precondition harness has only its cover. This classifier decides it by the
    /// cover; collection preconditions additionally pass the bound totality check in `evidence_of`.
    #[trace("FR-017-AC-61", "TC-027")]
    #[test]
    fn a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule() {
        let run = |kind: Option<ObligationKind>, checks: &[(&str, &str)]| {
            launch_evidence(
                LaunchOutcome::Completed {
                    exited_successfully: true,
                    exit_code: Some(0),
                    streams: CapturedStreams::default(),
                },
                Some(&report("Success", checks)),
                kind,
            )
            .unwrap()
            .0
        };
        let satisfied = run(Some(ObligationKind::Precondition), &[COVER_OK]);
        assert_eq!(satisfied.outcome, KaniRunOutcome::Verified);
        assert_eq!(satisfied.success_checks, 1);
        for kind in [Some(ObligationKind::Postcondition), None] {
            assert_eq!(
                run(kind, &[COVER_OK]).outcome,
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::VacuousProof
                }
            );
        }
        assert_eq!(
            run(Some(ObligationKind::Precondition), &[COVER_NO]).outcome,
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            run(Some(ObligationKind::Precondition), &[]).outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary
            }
        );
    }

    /// Drives the shared command/report pipeline against a launcher stand-in that exits with `status` and,
    /// when given a report, writes it where `--export-json` names. `stale` is left in the target
    /// directory beforehand, as an earlier run would leave it.
    fn run_stand_in(
        name: &str,
        status: i32,
        exported: Option<&str>,
        stale: Option<&str>,
    ) -> Result<report_fixture::FixtureExecution, KaniExecutionRefusal> {
        run_stand_in_into(name, status, exported, stale, None)
    }

    /// [`run_stand_in`] with an optional target directory shared with other runs; a shared run
    /// lingers before exporting so that runs sharing the directory overlap.
    fn run_stand_in_into(
        name: &str,
        status: i32,
        exported: Option<&str>,
        stale: Option<&str>,
        shared_target: Option<&Path>,
    ) -> Result<report_fixture::FixtureExecution, KaniExecutionRefusal> {
        let cancel = quire_exact::Cancel::new();
        let directory = discover_scratch(name);
        let crate_directory = directory.join("crate");
        let target_directory =
            shared_target.map_or_else(|| directory.join("target"), Path::to_path_buf);
        fs::create_dir_all(crate_directory.join("src")).unwrap();
        fs::create_dir_all(&target_directory).unwrap();
        fs::write(crate_directory.join("src/lib.rs"), "").unwrap();
        if let Some(stale) = stale {
            fs::write(
                target_directory.join("quire-kani-report-other-run.json"),
                stale,
            )
            .unwrap();
        }
        if let Some(exported) = exported {
            fs::write(directory.join("exported.json"), exported).unwrap();
        }
        let launcher = directory.join("cargo-kani");
        let linger = if shared_target.is_some() {
            "sleep 1\n"
        } else {
            ""
        };
        let copy = if exported.is_some() {
            format!(
                "cp '{}' \"$last\"",
                directory.join("exported.json").display()
            )
        } else {
            ":".to_owned()
        };
        write_launcher(
            &launcher,
            &format!("for last; do :; done\n{linger}{copy}\nexit {status}\n"),
        );
        let harness = state_frame_harness(
            StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            Vec::new(),
        );
        let result = report_fixture::single(&KaniExecutionRequest {
            cancel: &cancel,
            installation: &KaniInstallation::new(launcher),
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: &crate_directory,
            target_directory: &target_directory,
            home_directory: None,
        });
        if stale.is_some() {
            assert!(
                target_directory
                    .join("quire-kani-report-other-run.json")
                    .is_file(),
                "another run's report is not this run's to remove"
            );
        }
        if shared_target.is_none() {
            let leftovers = fs::read_dir(&target_directory)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_name() != "quire-kani-report-other-run.json")
                .count();
            assert_eq!(leftovers, 0, "a run removes its own report file");
        }
        let _ = fs::remove_dir_all(directory);
        result
    }

    /// A run's report is read from the file the launch names, the previous run's file is never
    /// read in its place, and a successful exit without a report is refused.
    #[trace("FR-017-AC-18", "TC-027")]
    #[test]
    fn tc_027_execution_reads_only_the_report_its_own_run_exported() {
        let verified = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
        let evidence = run_stand_in("exported", 0, Some(&verified), None).unwrap();
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.success_checks, 1);
        assert_eq!(evidence.exit_code, Some(0));
        assert!(matches!(
            run_stand_in("missing", 0, None, None),
            Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
        ));
        assert!(
            matches!(
                run_stand_in("stale", 0, None, Some(&verified)),
                Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
            ),
            "a report left by an earlier run is not this run's verdict"
        );
        assert!(matches!(
            run_stand_in("garbage", 1, Some("not json"), None),
            Err(KaniExecutionRefusal::Report(
                KaniReportRefusal::Malformed { .. }
            ))
        ));
    }

    /// The command exports the report to the request's target directory after the harness
    /// identity's options, which are passed unchanged.
    #[trace("FR-017-AC-6", "TC-027")]
    #[test]
    fn tc_027_the_launch_exports_the_report_after_the_harness_options() {
        let cancel = quire_exact::Cancel::new();
        let harness = state_frame_harness(
            StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            vec![
                "--harness".to_owned(),
                "check".to_owned(),
                "--exact".to_owned(),
            ],
        );
        let installation = KaniInstallation::new(PathBuf::from("cargo-kani"));
        let request = KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: Path::new("/crate"),
            target_directory: Path::new("/target"),
            home_directory: None,
        };
        let (arguments, _) = kani_launch_command(&request);
        let options = request.harness.view().options;
        assert_eq!(arguments[1..=options.len()], options[..]);
        assert_eq!(
            arguments[options.len() + 1..arguments.len() - 1],
            ["-Z", "unstable-options", "--export-json"]
        );
        let report = arguments.last().unwrap();
        assert!(
            report.starts_with("/target/quire-kani-report-") && report.ends_with(".json"),
            "{report}"
        );
        // Two launches into one target directory never name the same report file.
        let (again, _) = kani_launch_command(&request);
        assert_ne!(again.last(), arguments.last());
    }

    /// Two runs sharing a target directory at the same time each read their own report: the
    /// report of a run that is still going is neither read nor removed by another.
    #[trace("TC-027", "FR-017-AC-19")]
    #[test]
    fn tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports() {
        let verified = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
        let failed = String::from_utf8(report("Failure", &[("Failure", "assertion")])).unwrap();
        let shared = discover_scratch("concurrent-target");
        let outcomes: Vec<_> = std::thread::scope(|scope| {
            let runs: Vec<_> = (0..4)
                .map(|n| {
                    let exported = if n % 2 == 0 { &verified } else { &failed };
                    let (name, shared) = (format!("concurrent-{n}"), shared.as_path());
                    scope.spawn(move || {
                        run_stand_in_into(
                            &name,
                            i32::from(n % 2 == 1),
                            Some(exported),
                            None,
                            Some(shared),
                        )
                        .unwrap()
                        .outcome
                    })
                })
                .collect();
            runs.into_iter().map(|run| run.join().unwrap()).collect()
        });
        let _ = fs::remove_dir_all(shared);
        assert_eq!(outcomes[0], KaniRunOutcome::Verified);
        assert_eq!(outcomes[2], KaniRunOutcome::Verified);
        assert!(matches!(outcomes[1], KaniRunOutcome::Inconclusive { .. }));
    }

    /// A timed-out launch maps to no exit code and `Inconclusive { reason: TimedOut }`.
    #[test]
    fn a_timed_out_launch_carries_no_exit_code_into_the_evidence() {
        // A timed-out run is killed before it writes a report, so none is consulted.
        let (run, exit_code, streams) =
            launch_evidence(LaunchOutcome::TimedOut, None, None).unwrap();
        assert!(streams.is_none());
        assert_eq!(exit_code, None);
        assert_eq!(
            run.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );
        assert_eq!(run.success_checks, 0);
    }

    /// A launch stopped over a stream or over one that was not read is the refusal that names the
    /// stream and carries its stable code, never an outcome, and launch evidence is never taken
    /// from empty text in its place.
    #[trace("FR-017-AC-14", "FR-017-AC-25", "TC-043")]
    #[test]
    fn tc_043_a_launch_stopped_over_a_stream_is_refused_with_its_stable_code() {
        let over = launch_evidence(
            LaunchOutcome::OutputOverLimit {
                stream: CaptureStream::Stderr,
                limit: 16,
                harnesses: 2,
            },
            None,
            None,
        )
        .unwrap_err();
        assert!(matches!(
            over,
            KaniExecutionRefusal::OutputOverLimit {
                stream: CaptureStream::Stderr,
                limit: 16,
                harnesses: 2
            }
        ));
        assert_eq!(over.code(), Some("kani_output_over_limit"));
        let text = over.to_string();
        assert!(
            text.contains("kani_output_over_limit")
                && text.contains("stderr")
                && text.contains("16")
                && text.contains('2'),
            "{text}"
        );
        let unread = launch_evidence(
            LaunchOutcome::OutputUnread {
                stream: CaptureStream::Stdout,
                detail: "poll failed".to_owned(),
            },
            Some(&report("Success", &[PASSED, COVER_OK])),
            None,
        )
        .unwrap_err();
        assert!(matches!(
            &unread,
            KaniExecutionRefusal::OutputUnread { stream: CaptureStream::Stdout, detail }
                if detail == "poll failed"
        ));
        assert_eq!(unread.code(), Some("kani_output_unread"));
        assert!(unread.to_string().contains("kani_output_unread"));
        assert_eq!(
            KaniExecutionRefusal::Report(KaniReportRefusal::Missing).code(),
            None
        );
    }
}

#[cfg(test)]
mod two_phase_tests;

#[cfg(test)]
pub(crate) mod batch_tests {
    use ix_trace_rs::trace;
    use quire_contract_ir::kani::KaniOutcomeKind;
    use std::path::PathBuf;

    use super::*;
    use crate::{
        core::{artifact::Artifact, test_support::discover_scratch},
        kani::{
            abi::adapter_options,
            identity::{KaniV2ClauseObligationHarness, StateFrameHarness, StateFrameProperty},
            run::launch::CAPTURE_LIMIT,
            test_support::{
                batch_report, named_state_frame_harness, write_launcher, BatchEntry, COVER_OK,
                PASSED,
            },
        },
    };

    /// The request timeout T of the tests that do not vary it.
    pub(super) const T: Duration = Duration::from_secs(30);

    /// The options every harness of these tests shares once its `--harness <path> --exact`
    /// selection is taken out, written out so the test is not the code's own vector.
    // Expected argv for the Linux full bounded-process count test only.
    const SHARED_OPTIONS: [&str; 8] = [
        "-Z",
        "function-contracts",
        "--unwind",
        "4",
        "--solver",
        "cadical",
        "--output-format",
        "regular",
    ];

    /// Waits for the process whose pid `path` records to be gone, as `kill -0` reports it.
    fn recorded_process_gone(path: &Path) {
        let pid = fs::read_to_string(path).expect("a child must have started");
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            let alive = std::process::Command::new("kill")
                .args(["-0", pid.trim()])
                .stderr(std::process::Stdio::null())
                .status()
                .is_ok_and(|status| status.success());
            if !alive {
                return;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "the backend tree child must be killed"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    pub(super) fn memory_member(module: &str, bytes: u64) -> StateFrameHarness {
        let mut harness = member(module, "check", 4);
        harness.identity.ceilings.memory_bytes = std::num::NonZeroU64::new(bytes).unwrap();
        harness
    }

    /// A named peak-memory source returning a fixed reading, for the evidence controls.
    #[derive(Debug)]
    struct FixedPeak {
        name: &'static str,
        reading: Option<crate::kani::run::memory::PeakReading>,
    }

    impl crate::kani::run::memory::PeakMemorySource for FixedPeak {
        fn name(&self) -> &str {
            self.name
        }

        fn peak(&self, _launcher: u32) -> Option<crate::kani::run::memory::PeakReading> {
            self.reading
        }
    }

    /// The identity ceilings govern execution, and the memory ceiling is a declared value the
    /// runner never enforces: a run that allocates far past a 1 MiB ceiling and exits with a
    /// verified report is `Verified`, not stopped and not refused (FR-028-AC-21, ADR-007).
    #[trace("FR-028-AC-2", "FR-028-AC-4", "FR-028-AC-21", "TC-039")]
    #[test]
    fn identity_ceilings_govern_execution_and_the_runner_enforces_no_memory_cap() {
        let mut harness = memory_member("a", 128 * 1024 * 1024);
        harness.identity.state_fields = vec!["input".to_owned()];
        harness.identity.domains = vec![crate::kani::identity::StateFieldDomain {
            field: "input".to_owned(),
            minimum: -3,
            maximum: 7,
        }];
        let verifying = StandIn::verifying("successful-memory-bound");
        let evidence = execute_kani_obligation(&verifying.request(&harness, T)).unwrap();
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.ceilings, harness.identity.ceilings);
        assert_eq!(
            evidence.symbolic_arguments,
            vec![crate::kani::identity::SymbolicArgumentBounds {
                identifier: "input".to_owned(),
                bounds: crate::kani::identity::SymbolicBounds::Integer {
                    minimum: -3,
                    maximum: 7
                },
            }]
        );
        assert_eq!(
            evidence.memory,
            crate::kani::run::memory::MemoryObservation::default(),
            "an environment that provides no peak records none"
        );
        harness.identity.ceilings.wall_clock = Duration::from_millis(200);
        let stopped = StandIn::running("identity-wall-clock-bound", "sleep 45");
        let evidence =
            execute_kani_obligation(&stopped.request(&harness, Duration::from_millis(200)))
                .unwrap();
        assert_eq!(evidence.ceilings.wall_clock, Duration::from_millis(200));
        assert_eq!(
            evidence.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );

        let mut over = memory_member("over", 1024 * 1024);
        over.identity.ceilings.wall_clock = T;
        let allocating = StandIn::running(
            "memory-ceiling-is-not-enforced",
            &format!("python3 -c 'x = bytearray(64 * 1024 * 1024)'\n{VERIFY_EVERY_HARNESS}"),
        );
        allocating.containing(&[&over]);
        let evidence = execute_kani_obligation(&allocating.request(&over, T)).unwrap();
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.ceilings.memory_bytes.get(), 1024 * 1024);
    }

    /// The peak the evidence records is the environment source's, in bytes, with its name; a
    /// source reporting KiB is converted inside the seam; a source with nothing leaves no peak
    /// and does not change the outcome (FR-028-AC-21).
    #[trace("FR-028-AC-21", "TC-039")]
    #[test]
    fn the_evidence_records_the_named_peak_the_environment_provides_in_bytes() {
        use crate::kani::run::memory::{MemoryUnit, PeakReading};
        let harness = memory_member("a", 128 * 1024 * 1024);
        for (reading, expected) in [
            (
                Some(PeakReading {
                    value: 123_456_789,
                    unit: MemoryUnit::Bytes,
                }),
                Some(123_456_789),
            ),
            (
                Some(PeakReading {
                    value: 1_048_576,
                    unit: MemoryUnit::Kibibytes,
                }),
                Some(1_073_741_824),
            ),
            (
                Some(PeakReading {
                    value: u64::MAX,
                    unit: MemoryUnit::Kibibytes,
                }),
                None,
            ),
            (None, None),
        ] {
            let mut stand_in = StandIn::verifying("named-peak-source");
            stand_in.installation =
                stand_in
                    .installation
                    .clone()
                    .with_peak_memory_source(std::sync::Arc::new(FixedPeak {
                        name: "test_peak_source",
                        reading,
                    }));
            let evidence = execute_kani_obligation(&stand_in.request(&harness, T)).unwrap();
            assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
            assert_eq!(evidence.memory.peak_resident_bytes, expected);
            assert_eq!(
                evidence.memory.source.as_deref(),
                expected.map(|_| "test_peak_source"),
                "a source is named only beside the peak it provided"
            );
        }
    }

    /// A sealed V2 clause harness whose artifact is a small module, with the given wall clock.
    fn playback_harness(wall_clock: Duration) -> KaniV2ClauseObligationHarness {
        let mut identity = runner_precondition_fixture("forall").identity;
        identity.ceilings =
            crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(wall_clock);
        identity.collection_precondition = None;
        identity.collection_operation = None;
        let module = identity.module_symbol.to_string();
        let source = format!(
            "// generated header\nmod {module} {{\nfn oracle_clause() -> bool {{ true }}\n#[cfg(kani)]\n#[kani::proof]\nfn check() {{}}\n}}\n"
        );
        KaniV2ClauseObligationHarness::generated(
            identity,
            crate::kani::test_support::fixture_obligation(),
            crate::core::artifact::Artifact::new("src/generated/playback.rs", source),
            crate::core::artifact::Artifact::new("kani-obligations/playback.json", "{}"),
        )
    }

    fn playback_request<'a>(
        stand_in: &'a StandIn,
        harness: &'a KaniV2ClauseObligationHarness,
    ) -> KaniExecutionRequest<'a> {
        KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: harness.into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        }
    }

    /// The playback entry accepts only a sealed V2 clause harness holding exactly the declared
    /// appended test, refuses every other source before any launch, reads the libtest output
    /// fail closed, and runs the launch inside the proof's wall clock, cancellation and teardown.
    #[trace("FR-017-AC-57", "FR-017-AC-58", "FR-017-AC-59", "TC-027")]
    #[cfg(target_os = "linux")]
    #[test]
    fn concrete_playback_refuses_unsealed_sources_and_bounds_the_launch() {
        let harness = playback_harness(T);
        let module = harness.identity.module_symbol.to_string();
        let test_name = format!("{module}::playback_999");
        let appended = "#[test]\nfn playback_999() {}\n";
        let playback = KaniConcretePlayback {
            test_name: &test_name,
            appended_test: appended,
        };
        let sealed = harness.rust.contents.clone();
        let source = format!("{}{appended}}}\n", sealed.strip_suffix("}\n").unwrap());
        let running = |name: &str, body: &str, library: &str| {
            let stand_in = StandIn::running(name, body);
            fs::write(stand_in.crate_directory.join("src/lib.rs"), library).unwrap();
            stand_in
        };
        let pass_output = format!(
            "printf 'test {test_name} ... ok\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\\n'"
        );

        // A non-V2 harness refuses before launch.
        let unsupported = running("playback-unsupported", "exit 0", &source);
        let frame = member("playback", "check", 4);
        assert!(matches!(
            execute_kani_concrete_playback(
                &KaniExecutionRequest {
                    cancel: &unsupported.cancel,
                    installation: &unsupported.installation,
                    harness: (&frame).into(),
                    crate_directory: &unsupported.crate_directory,
                    target_directory: &unsupported.target_directory,
                    home_directory: None,
                },
                &playback
            ),
            Err(KaniExecutionRefusal::ConcretePlaybackUnsupportedHarness { .. })
        ));
        assert!(unsupported.calls().is_empty());

        // A forged seal refuses as a provenance mismatch.
        let mut forged = harness.clone();
        forged.record.contents.push('x');
        let forged_run = running("playback-forged", "exit 0", &source);
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&forged_run, &forged), &playback),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(forged_run.calls().is_empty());

        // Each of these source cases refuses before any launch.
        let wrong_name = KaniConcretePlayback {
            test_name: &format!("{module}::other"),
            appended_test: appended,
        };
        let wrong_module = KaniConcretePlayback {
            test_name: "other::playback_999",
            appended_test: appended,
        };
        let cases: Vec<(&str, String, &KaniConcretePlayback<'_>)> = vec![
            ("absent test", sealed.clone(), &playback),
            (
                "duplicated test",
                source.replacen(appended, &appended.repeat(2), 1),
                &playback,
            ),
            ("wrong function", source.clone(), &wrong_name),
            ("wrong module", source.clone(), &wrong_module),
            (
                "altered header",
                source.replacen("// generated header", "// changed", 1),
                &playback,
            ),
            (
                "altered oracle",
                source.replacen("{ true }", "{ false }", 1),
                &playback,
            ),
            (
                "altered proof",
                source.replacen("fn check() {}", "fn check() { panic!() }", 1),
                &playback,
            ),
        ];
        for (case, library, declared) in cases {
            let stand_in = running("playback-source-case", "exit 0", &library);
            assert!(
                matches!(
                    execute_kani_concrete_playback(
                        &playback_request(&stand_in, &harness),
                        declared
                    ),
                    Err(KaniExecutionRefusal::ConcretePlaybackSourceMismatch { .. })
                ),
                "{case}"
            );
            assert!(stand_in.calls().is_empty(), "{case}: no launch");
        }
        for (case, body) in [("absent library", false), ("unreadable library", true)] {
            let stand_in = running("playback-library-case", "exit 0", &source);
            let library = stand_in.crate_directory.join("src/lib.rs");
            fs::remove_file(&library).unwrap();
            if body {
                fs::create_dir(&library).unwrap();
            }
            assert!(
                matches!(
                    execute_kani_concrete_playback(
                        &playback_request(&stand_in, &harness),
                        &playback
                    ),
                    Err(KaniExecutionRefusal::ConcretePlaybackSourceMismatch { .. })
                ),
                "{case}"
            );
            assert!(stand_in.calls().is_empty(), "{case}: no launch");
        }

        // A successful exit is playback evidence only with the exact pass line and summary.
        let zero = running(
            "playback-zero",
            "printf 'running 0 tests\\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out\\n'",
            &source,
        );
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&zero, &harness), &playback),
            Err(KaniExecutionRefusal::ConcretePlaybackTestNotObserved { .. })
        ));
        assert_eq!(zero.calls().len(), 1);
        let passed = running("playback-passed", &pass_output, &source);
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&passed, &harness), &playback),
            Ok(KaniConcretePlaybackOutcome::Completed {
                succeeded: true,
                ..
            })
        ));
        assert_eq!(
            passed.calls(),
            [vec![
                "kani".to_owned(),
                "playback".to_owned(),
                "--lib".to_owned(),
                "-Z".to_owned(),
                "function-contracts".to_owned(),
                "-Z".to_owned(),
                "concrete-playback".to_owned(),
                "--".to_owned(),
                test_name.clone(),
                "--exact".to_owned(),
                "--nocapture".to_owned(),
            ]]
        );

        // An unsuccessful exit keeps its exit code and streams and attributes nothing.
        let failed = running(
            "playback-failed",
            "printf 'error: did not compile\\n'; exit 101",
            &source,
        );
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&failed, &harness), &playback),
            Ok(KaniConcretePlaybackOutcome::Completed {
                succeeded: false,
                exit_code: Some(101),
                stdout,
                ..
            }) if stdout == b"error: did not compile\n"
        ));

        // The proof's wall clock, cancellation and owned teardown govern the launch.
        let body = "sleep 45 &\nprintf '%s' \"$!\" > \"$CALLS.child\"\nwait";
        let short = playback_harness(Duration::from_secs(2));
        let short_source = format!(
            "{}{appended}}}\n",
            short.rust.contents.strip_suffix("}\n").unwrap()
        );
        let timed = running("playback-timeout", body, &short_source);
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&timed, &short), &playback),
            Ok(KaniConcretePlaybackOutcome::TimedOut { .. })
        ));
        recorded_process_gone(&timed.directory.join("calls.child"));

        let cancelled = running("playback-cancel", body, &source);
        let result = std::thread::scope(|scope| {
            let worker = scope.spawn(|| {
                execute_kani_concrete_playback(&playback_request(&cancelled, &harness), &playback)
            });
            let ready = cancelled.directory.join("calls.child");
            let deadline = Instant::now() + Duration::from_secs(5);
            while !ready.is_file() {
                assert!(Instant::now() < deadline, "the playback process started");
                std::thread::yield_now();
            }
            cancelled.cancel.cancel(quire_exact::CancelCause::Requested);
            worker.join().unwrap().unwrap()
        });
        assert!(matches!(
            result,
            KaniConcretePlaybackOutcome::Cancelled {
                source: KaniCancellationSource::Requested,
                ..
            }
        ));
        recorded_process_gone(&cancelled.directory.join("calls.child"));

        // A stream over its limit is the existing typed refusal, not an outcome.
        let body = format!("head -c {} /dev/zero", CAPTURE_LIMIT + 1);
        let over = running("playback-over-limit", &body, &source);
        assert!(matches!(
            execute_kani_concrete_playback(&playback_request(&over, &harness), &playback),
            Err(KaniExecutionRefusal::OutputOverLimit {
                stream: CaptureStream::Stdout,
                ..
            })
        ));
    }

    /// One exact passing test must precede its one-pass summary. Libtest
    /// labels `#[should_panic]` on the test line, including for Kani playback.
    #[trace("FR-017-AC-59", "TC-027")]
    #[test]
    fn concrete_playback_parser_pairs_the_exact_test_and_summary() {
        let test_name = "playback::playback_999";
        let plain = "test playback::playback_999 ... ok";
        let should_panic = "test playback::playback_999 - should panic ... ok";
        let one = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out";
        for line in [plain, should_panic] {
            let output = format!("Finished test profile\nrunning 1 test\n{line}\n{one}\n");
            assert!(playback_test_passed(output.as_bytes(), test_name), "{line}");
        }
        let captured_panic = format!(
            "running 1 test\nthread 'playback::playback_999' panicked at lib.rs:7:7:\nunique assertion\n{should_panic}\n{one}\n"
        );
        assert!(playback_test_passed(captured_panic.as_bytes(), test_name));
        for (case, output) in [
            (
                "wrong test",
                format!("test other::playback_999 ... ok\n{one}\n"),
            ),
            (
                "zero pass",
                format!("{plain}\ntest result: ok. 0 passed; 0 failed; 1 filtered out\n"),
            ),
            (
                "two passes",
                format!("{plain}\ntest result: ok. 2 passed; 0 failed;\n"),
            ),
            (
                "failed summary",
                format!("{plain}\ntest result: FAILED. 0 passed; 1 failed;\n"),
            ),
            ("summary first", format!("{one}\n{plain}\n")),
            (
                "interleaved test result",
                format!(
                    "test playback::playback_999 - should panic ...\nunique assertion\nok\n{one}\n"
                ),
            ),
            ("no summary", format!("{plain}\n")),
            (
                "truncated tail",
                format!("{plain}\ntest result: ok. 1 pass"),
            ),
            ("duplicate test", format!("{plain}\n{plain}\n{one}\n")),
            ("duplicate summary", format!("{plain}\n{one}\n{one}\n")),
            (
                "unrelated passing test",
                format!("test other::playback_999 ... ok\n{plain}\n{one}\n"),
            ),
            (
                "incidental earlier result",
                format!("test result: ok. 0 passed; 0 failed;\n{plain}\n{one}\n"),
            ),
            (
                "incidental later result",
                format!("{plain}\n{one}\ntest result: ok. 0 passed; 0 failed;\n"),
            ),
        ] {
            assert!(
                !playback_test_passed(output.as_bytes(), test_name),
                "{case}: {output}"
            );
        }
        assert!(!playback_test_passed(b"\xff", test_name));
    }

    #[trace("FR-017-AC-21", "TC-043")]
    #[test]
    fn unequal_identity_memory_ceilings_run_in_separate_backend_processes() {
        let harnesses = [
            memory_member("a", 128 * 1024 * 1024),
            memory_member("b", 256 * 1024 * 1024),
        ];
        let stand_in = StandIn::verifying("different-memory-ceilings");
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| stand_in.request(harness, T))
            .collect();
        let groups = execute_kani_obligations(&requests).unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(stand_in.calls().len(), 2);
        for (group, harness) in groups.into_iter().zip(&harnesses) {
            let evidence = group.evidence.unwrap();
            assert_eq!(evidence.len(), 1);
            assert_eq!(evidence[0].ceilings, harness.identity.ceilings);
            assert!(evidence[0].batch.is_none());
        }
    }

    /// Unequal identity memory ceilings plan separate groups; each planned selection and argv
    /// retains its member.
    #[trace("FR-017-AC-21", "TC-043")]
    #[test]
    fn unequal_memory_ceilings_plan_separate_groups_with_their_own_selections() {
        let harnesses = [
            memory_member("a", 128 * 1024 * 1024),
            memory_member("b", 256 * 1024 * 1024),
        ];
        let stand_in = StandIn::verifying("memory-group-plan");
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| stand_in.request(harness, T))
            .collect();
        let groups = plan_groups(requests.iter().enumerate(), Phase::Proof);
        assert_eq!(
            groups
                .iter()
                .map(|group| group
                    .iter()
                    .map(|(position, _)| *position)
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [vec![0], vec![1]]
        );
        for (group, expected) in groups.iter().zip(["a::check", "b::check"]) {
            let request = group[0].1;
            let (arguments, _) =
                launch_command(request, Phase::Proof, &PathBuf::from("report.json"), false);
            assert!(arguments
                .windows(3)
                .any(|selection| selection == ["--harness", expected, "--exact"]));
        }
        assert!(
            !shares_process(&requests[0], &requests[1], Phase::Proof),
            "unequal memory ceilings cannot share a process"
        );
    }

    /// What a launcher stand-in does before anything else: it records that it ran and what it was
    /// given, and finds the report path (its last argument) and the harnesses it was asked for.
    const PROLOGUE: &str = r#"printf 'RUN\n' >> "$CALLS"
witness=no
for argument; do printf '%s\n' "$argument" >> "$CALLS"; [ "$argument" = concrete-playback ] && witness=yes; done
last=; ids=
while [ $# -gt 0 ]; do
  last="$1"
  if [ "$1" = --harness ]; then ids="$ids $2"; last="$2"; shift; fi
  shift
done
"#;

    /// A stand-in that reports every harness it is asked for as verified: one passed check and
    /// one satisfied cover.
    const VERIFY_EVERY_HARNESS: &str = r#"results=; sep=
for id in $ids; do
  results="$results$sep{\"harness_id\":\"$id\",\"status\":\"Success\",\"checks\":[{\"id\":1,\"status\":\"Success\",\"category\":\"assertion\",\"location\":{\"file\":\"src/lib.rs\",\"line\":\"1\"}},{\"id\":2,\"status\":\"Satisfied\",\"category\":\"cover\",\"location\":{\"file\":\"src/lib.rs\",\"line\":\"2\"}}]}"
  sep=,
done
printf '{"metadata":{"version":"1.0"},"verification_results":{"results":[%s]}}' "$results" > "$last"
exit 0
"#;

    pub(super) fn member(module: &str, harness: &str, unwind: u32) -> StateFrameHarness {
        let mut built = named_state_frame_harness(
            module,
            harness,
            StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            adapter_options(
                &format!("{module}::{harness}"),
                unwind,
                KaniSolver::Cadical,
                false,
            ),
        );
        built.identity.unwind = unwind;
        built
    }

    // Expected selections for the Linux full bounded-process count test only.
    fn path_of(harness: &StateFrameHarness) -> String {
        format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        )
    }

    /// A launcher stand-in in a scratch crate and target directory.
    pub(super) struct StandIn {
        pub(super) directory: PathBuf,
        pub(super) crate_directory: PathBuf,
        pub(super) target_directory: PathBuf,
        pub(super) installation: KaniInstallation,
        pub(super) cancel: Cancel,
    }

    impl StandIn {
        /// A scratch crate and target directory, and the path a launcher will be installed at.
        pub(super) fn scaffold(name: &str) -> Self {
            let directory = discover_scratch(name);
            let crate_directory = directory.join("crate");
            let target_directory = directory.join("target");
            fs::create_dir_all(crate_directory.join("src")).unwrap();
            fs::create_dir_all(&target_directory).unwrap();
            fs::write(crate_directory.join("src/lib.rs"), "").unwrap();
            let launcher = directory.join("cargo-kani");
            Self {
                directory,
                crate_directory,
                target_directory,
                installation: KaniInstallation::new(launcher),
                cancel: Cancel::new(),
            }
        }

        /// Installs the launcher: the prologue, then `body`.
        pub(super) fn install(&self, body: &str) {
            write_launcher(
                &self.installation.launcher,
                &format!(
                    "CALLS='{}'\n{PROLOGUE}{body}\n",
                    self.directory.join("calls").display()
                ),
            );
        }

        /// Persist the actual harness source that the public execution entry authenticates.
        fn containing(&self, harnesses: &[&StateFrameHarness]) {
            let source = harnesses
                .iter()
                .map(|harness| harness.rust.contents.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(self.crate_directory.join("src/lib.rs"), source).unwrap();
        }

        /// A stand-in running `body` after its prologue.
        fn running(name: &str, body: &str) -> Self {
            let stand_in = Self::scaffold(name);
            stand_in.install(body);
            stand_in
        }

        /// A stand-in that verifies every harness it is asked for.
        fn verifying(name: &str) -> Self {
            Self::running(name, VERIFY_EVERY_HARNESS)
        }

        /// A stand-in that exports `report` (when given), prints `console` and exits with the
        /// status `exit`.
        fn replaying(name: &str, report: Option<&[u8]>, console: &str, exit: i32) -> Self {
            let stand_in = Self::scaffold(name);
            let mut body = String::new();
            if let Some(report) = report {
                let exported = stand_in.directory.join("report.json");
                fs::write(&exported, report).unwrap();
                body.push_str(&format!("cp '{}' \"$last\"\n", exported.display()));
            }
            let printed = stand_in.directory.join("console");
            fs::write(&printed, console).unwrap();
            body.push_str(&format!("cat '{}'\nexit {exit}\n", printed.display()));
            stand_in.install(&body);
            stand_in
        }

        /// A stand-in that answers a launch whose vector holds `concrete-playback` (the witness
        /// phase) with `witness` and every other launch with `proof`, each a report (when given),
        /// a console and an exit status.
        fn replaying_phases(
            name: &str,
            proof: (Option<&[u8]>, &str, i32),
            witness: (Option<&[u8]>, &str, i32),
        ) -> Self {
            let stand_in = Self::scaffold(name);
            let branch = |tag: &str, (report, console, exit): (Option<&[u8]>, &str, i32)| {
                let mut body = String::new();
                if let Some(report) = report {
                    let exported = stand_in.directory.join(format!("{tag}-report.json"));
                    fs::write(&exported, report).unwrap();
                    body.push_str(&format!("cp '{}' \"$last\"\n", exported.display()));
                }
                let printed = stand_in.directory.join(format!("{tag}-console"));
                fs::write(&printed, console).unwrap();
                body.push_str(&format!("cat '{}'\nexit {exit}\n", printed.display()));
                body
            };
            let body = format!(
                "if [ \"$witness\" = yes ]; then\n{}\nelse\n{}\nfi\n",
                branch("witness", witness),
                branch("proof", proof)
            );
            stand_in.install(&body);
            stand_in
        }

        pub(super) fn request<'a>(
            &'a self,
            harness: &'a StateFrameHarness,
            timeout: Duration,
        ) -> KaniExecutionRequest<'a> {
            assert_eq!(harness.identity.ceilings.wall_clock, timeout);
            KaniExecutionRequest {
                cancel: &self.cancel,
                installation: &self.installation,
                harness: harness.into(),
                crate_directory: &self.crate_directory,
                target_directory: &self.target_directory,
                home_directory: None,
            }
        }

        /// Runs `harnesses` as a batch at `timeout`.
        pub(super) fn batch(
            &self,
            harnesses: &[StateFrameHarness],
            timeout: Duration,
        ) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
            let harnesses: Vec<_> = harnesses
                .iter()
                .cloned()
                .map(|mut harness| {
                    harness.identity.ceilings.wall_clock = timeout;
                    harness
                })
                .collect();
            let requests: Vec<_> = harnesses
                .iter()
                .map(|harness| self.request(harness, timeout))
                .collect();
            execute_kani_obligations(&requests)
        }

        /// Exercise actual bounded execution on Linux and shared POSIX report semantics on
        /// unsupported targets. Only the Linux result carries a memory-enforcement attestation.
        fn reported_batch(
            &self,
            harnesses: &[StateFrameHarness],
            timeout: Duration,
        ) -> Result<Vec<report_fixture::ReportGroup>, KaniExecutionRefusal> {
            let harnesses: Vec<_> = harnesses
                .iter()
                .cloned()
                .map(|mut harness| {
                    harness.identity.ceilings.wall_clock = timeout;
                    harness
                })
                .collect();
            let requests: Vec<_> = harnesses
                .iter()
                .map(|harness| self.request(harness, timeout))
                .collect();
            report_fixture::groups(&requests)
        }

        /// The argument vector of each process the stand-in ran, as it received them.
        pub(super) fn calls(&self) -> Vec<Vec<String>> {
            let recorded = fs::read_to_string(self.directory.join("calls")).unwrap_or_default();
            let mut calls: Vec<Vec<String>> = Vec::new();
            for line in recorded.lines() {
                if line == "RUN" {
                    calls.push(Vec::new());
                } else if let Some(call) = calls.last_mut() {
                    call.push(line.to_owned());
                }
            }
            calls
        }
    }

    impl Drop for StandIn {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.directory);
        }
    }

    /// A minimal collection-precondition harness, built by hand for the runner: a module whose
    /// one proof function holds the totality assertion on a known line, and the identity naming
    /// that line. The runner reads the identity and the source, never how they were generated.
    fn runner_precondition_fixture(quantifier: &str) -> KaniV2ClauseObligationHarness {
        use crate::{
            core::{
                artifact::Artifact,
                identity::{HarnessSymbol, ModuleSymbol},
            },
            kani::identity::{
                CollectionElementIdentity, CollectionKindIdentity, CollectionObligationIdentity,
                CollectionPreconditionIdentity, CollectionPredicateIdentity, ObligationKind,
                ProofDependencySnapshot, ScalarIntegerRepresentation, V2ClauseObligationIdentity,
            },
        };
        let module = format!("collection_{}", quantifier.rsplit('.').next().unwrap());
        let harness = "check";
        let id = |label: &str| -> quire_contract_model::CheckedNodeId {
            serde_json::from_value(serde_json::json!({
                "package": "runner-fixture", "path": [label], "ordinal": 0,
            }))
            .expect("a node id")
        };
        let package: quire_contract_model::ContractSourceV1 =
            serde_json::from_value(serde_json::json!({
                "name": "runner-fixture",
                "lock": {
                    "sources": [],
                    "edition": {"role": "edition", "definition": {"authority": "test", "identity": "edition"}},
                    "profile_selections": [], "definition_selections": [],
                    "required_features": [], "dependency_selections": []
                }
            }))
            .expect("fixture package source");
        let ceilings = crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(30),
        );
        let options = adapter_options(
            &format!("{module}::{harness}"),
            8,
            KaniSolver::Cadical,
            false,
        );
        let assertion = crate::kani::identity::collection_precondition_totality_assertion();
        let source = format!(
            "mod {module} {{\n    #[kani::proof]\n    pub fn {harness}() {{\n        {assertion}\n    }}\n}}\n"
        );
        let module_symbol = ModuleSymbol::try_from(module.as_str()).unwrap();
        let harness_symbol = HarnessSymbol::try_from(harness).unwrap();
        let collection = CollectionObligationIdentity {
            ceilings,
            package: package.clone(),
            subject_path: "crate::subject".to_owned(),
            function_id: id("function"),
            quantifier_id: id("quantifier"),
            projection_id: id("projection"),
            declaration_id: id("declaration"),
            field_name: "items".to_owned(),
            collection_kind: CollectionKindIdentity::Sequence,
            minimum_length: 0,
            maximum_length: 3,
            element: CollectionElementIdentity::Integer {
                minimum: 0,
                maximum: 9,
            },
            element_type_id: id("element-type"),
            quantifier_operation: quantifier.to_owned(),
            predicate_id: id("predicate"),
            predicate: CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold: 0 },
            solver: KaniSolver::Cadical,
            unwind: 8,
            options: options.clone(),
            witness_options: crate::kani::test_support::witness_of(&options),
            module_symbol: module_symbol.clone(),
            harness_symbol: harness_symbol.clone(),
        };
        let snapshot = ProofDependencySnapshot::default();
        let identity = V2ClauseObligationIdentity {
            dependencies: snapshot.dependencies().to_vec(),
            readiness: snapshot.readiness(),
            proof_execution_state: snapshot.proof_execution_state(),
            integer_representation: ScalarIntegerRepresentation::General,
            native_integer_selection: None,
            ceilings,
            package,
            clause: id("clause"),
            collection_operation: Some(crate::kani::identity::CollectionOperationIdentity {
                quantifier_id: collection.quantifier_id.clone(),
                projection_id: collection.projection_id.clone(),
                predicate_id: collection.predicate_id.clone(),
                predicate: collection.predicate.clone(),
                anchor_id: id("anchor"),
                fields: vec![crate::kani::identity::CollectionObjectFieldIdentity {
                    declaring_node_id: collection.declaration_id.clone(),
                    field_name: collection.field_name.clone(),
                    slot: "field_0".into(),
                    bounds: crate::kani::identity::SymbolicBounds::Sequence {
                        minimum_length: 0,
                        maximum_length: 3,
                        element: Box::new(crate::kani::identity::SymbolicBounds::Integer {
                            minimum: 0,
                            maximum: 9,
                        }),
                    },
                    collection_kind: Some(collection.collection_kind),
                }],
                selected_field: 0,
                parameters: Vec::new(),
                result: None,
                preconditions: Vec::new(),
                condition:
                    crate::kani::identity::CollectionOperationConditionIdentity::Quantifier {
                        field: 0,
                        operation: collection.quantifier_operation.clone(),
                        predicate: collection.predicate.clone(),
                    },
                precondition_bodies: Vec::new(),
            }),
            collection_precondition: Some(CollectionPreconditionIdentity {
                collection,
                totality_assertion_line: 4,
            }),
            kind: ObligationKind::Precondition,
            requires: Vec::new(),
            oracle_symbol: "oracle".to_owned(),
            subject_path: "crate::subject".to_owned(),
            arguments: Vec::new(),
            state_fields: Vec::new(),
            module_symbol,
            harness_symbol,
            solver: KaniSolver::Cadical,
            unwind: 8,
            witness_options: crate::kani::test_support::witness_of(&options),
            options,
        };
        KaniV2ClauseObligationHarness::generated(
            identity,
            crate::kani::test_support::fixture_obligation(),
            Artifact::new(format!("src/generated/{module}.rs"), source),
            Artifact::new(format!("kani-obligations/{module}.json"), "{}"),
        )
    }

    #[trace("FR-017-AC-61", "TC-027")]
    #[test]
    fn collection_totality_controls_single_readonly_and_batch_evidence() {
        use serde_json::json;

        let harness = runner_precondition_fixture("quire.op.collection.forall");
        let marker = harness.identity.collection_precondition.as_ref().unwrap();
        assert_eq!(
            harness
                .rust
                .contents
                .lines()
                .nth(usize::try_from(marker.totality_assertion_line - 1).unwrap())
                .expect("the marked assertion line exists")
                .trim(),
            crate::kani::identity::collection_precondition_totality_assertion()
        );
        assert!(!harness.record.contents.contains("sourceDigest"));
        let path = format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        );
        let check = |category: &str, file: &str, line: u32| {
            json!({
                "id": 1, "status": "Success", "category": category,
                "location": { "file": file, "line": line.to_string(), "column": "1" }
            })
        };
        let cover = || {
            json!({
                "id": 2, "status": "Satisfied", "category": "cover",
                "location": { "file": "src/lib.rs", "line": "1", "column": "1" }
            })
        };
        let report = |path: &str, checks: Vec<serde_json::Value>| {
            serde_json::to_vec(&json!({
                "metadata": { "version": "1.0" },
                "verification_results": { "results": [{
                    "harness_id": path, "status": "Success", "checks": checks
                }] }
            }))
            .unwrap()
        };
        let run = |name: &str, checks: Vec<serde_json::Value>, read_only: bool| {
            let stand_in = StandIn::replaying(name, Some(&report(&path, checks)), "", 0);
            fs::write(
                stand_in.crate_directory.join("src/lib.rs"),
                &harness.rust.contents,
            )
            .unwrap();
            let request = KaniExecutionRequest {
                cancel: &stand_in.cancel,
                installation: &stand_in.installation,
                harness: (&harness).into(),
                crate_directory: &stand_in.crate_directory,
                target_directory: &stand_in.target_directory,
                home_directory: None,
            };
            if read_only {
                execute_kani_obligation_library(&request).unwrap().outcome
            } else {
                execute_kani_obligation(&request).unwrap().outcome
            }
        };
        let vacuous = KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::VacuousProof,
        };
        let line = marker.totality_assertion_line;
        assert_eq!(
            run("collection-no-bound-check", vec![cover()], false),
            vacuous
        );
        assert_eq!(
            run(
                "collection-wrong-class",
                vec![check("other", "src/lib.rs", line), cover()],
                false
            ),
            vacuous
        );
        assert_eq!(
            run(
                "collection-wrong-file",
                vec![check("assertion", "src/other.rs", line), cover()],
                false
            ),
            vacuous
        );
        assert_eq!(
            run(
                "collection-unrelated-assertion",
                vec![check("assertion", "src/lib.rs", line + 1), cover()],
                false
            ),
            vacuous
        );
        assert_eq!(
            run(
                "collection-right-bound-check",
                vec![check("assertion", "src/lib.rs", line), cover()],
                false
            ),
            KaniRunOutcome::Verified
        );
        assert_eq!(
            run("collection-readonly-no-check", vec![cover()], true),
            vacuous
        );

        let stand_in = StandIn::scaffold("collection-duplicate-source");
        fs::write(
            stand_in.crate_directory.join("src/lib.rs"),
            format!("{}{}", harness.rust.contents, harness.rust.contents),
        )
        .unwrap();
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&harness).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));

        let mut shifted_source = harness.rust.clone();
        shifted_source.contents.insert(0, '\n');
        let bad = KaniV2ClauseObligationHarness::generated(
            harness.identity.clone(),
            harness.obligation.clone(),
            shifted_source,
            harness.record.clone(),
        );
        let stand_in = StandIn::scaffold("collection-shifted-source-valid-seal");
        fs::write(
            stand_in.crate_directory.join("src/lib.rs"),
            &bad.rust.contents,
        )
        .unwrap();
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&bad).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));

        let mut bad_identity = harness.identity.clone();
        bad_identity
            .collection_precondition
            .as_mut()
            .unwrap()
            .totality_assertion_line += 1;
        let bad = KaniV2ClauseObligationHarness::generated(
            bad_identity,
            harness.obligation.clone(),
            harness.rust.clone(),
            harness.record.clone(),
        );
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&bad).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::BoundTotalityAssertionMismatch { .. })
        ));

        let stand_in = StandIn::scaffold("collection-missing-source");
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&harness).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));

        let stand_in = StandIn::scaffold("collection-changed-crate-source");
        let assertion = crate::kani::identity::collection_precondition_totality_assertion();
        let changed = harness
            .rust
            .contents
            .replacen(&assertion, "assert!(false);", 1);
        assert_ne!(changed, harness.rust.contents);
        fs::write(stand_in.crate_directory.join("src/lib.rs"), changed).unwrap();
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&harness).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
        ));

        let mut unsealed = harness.clone();
        unsealed.rust.contents.insert(0, '\n');
        let stand_in = StandIn::scaffold("collection-unsealed-source");
        fs::write(
            stand_in.crate_directory.join("src/lib.rs"),
            &unsealed.rust.contents,
        )
        .unwrap();
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&unsealed).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));

        let other = runner_precondition_fixture("quire.op.collection.exists");
        let other_path = format!(
            "{}::{}",
            other.identity.module_symbol, other.identity.harness_symbol
        );
        let batch_report = serde_json::to_vec(&json!({
            "metadata": { "version": "1.0" },
            "error_details": [
                { "harness_id": path, "has_errors": false },
                { "harness_id": other_path, "has_errors": false }
            ],
            "verification_results": { "results": [
                { "harness_id": path, "status": "Success", "checks": [check("assertion", "src/lib.rs", line), cover()] },
                { "harness_id": other_path, "status": "Success", "checks": [cover()] }
            ] }
        })).unwrap();
        let stand_in =
            StandIn::replaying("collection-batch-bound-check", Some(&batch_report), "", 0);
        fs::write(
            stand_in.crate_directory.join("src/lib.rs"),
            format!("{}\n{}", harness.rust.contents, other.rust.contents),
        )
        .unwrap();
        let first = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&harness).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        let second = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&other).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        let runs = execute_kani_obligations(&[first, second]).unwrap();
        assert_eq!(runs.len(), 1, "both clauses share one backend process");
        let evidence = runs.into_iter().next().unwrap().evidence.unwrap();
        assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence[1].outcome, vacuous);
    }

    /// A V2 clause harness whose recorded witness vector, and only that, differs from the one its
    /// generator sealed is refused before launch by every entry point (FR-015-AC-9).
    #[trace("FR-015-AC-9", "TC-025")]
    #[test]
    fn a_tampered_witness_vector_of_a_clause_harness_is_a_provenance_mismatch() {
        let harness = runner_precondition_fixture("quire.op.collection.forall");
        assert!(KaniExecutableHarness::from(&harness).has_valid_provenance());
        let mut tampered = harness.clone();
        tampered.identity.witness_options.push("--extra".to_owned());
        assert_eq!(tampered.identity.options, harness.identity.options);
        let stand_in = StandIn::scaffold("clause-tampered-witness-vector");
        fs::write(
            stand_in.crate_directory.join("src/lib.rs"),
            &tampered.rust.contents,
        )
        .unwrap();
        let request = KaniExecutionRequest {
            cancel: &stand_in.cancel,
            installation: &stand_in.installation,
            harness: (&tampered).into(),
            crate_directory: &stand_in.crate_directory,
            target_directory: &stand_in.target_directory,
            home_directory: None,
        };
        assert!(matches!(
            execute_kani_obligation(&request),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligation_library(&request),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(matches!(
            execute_kani_obligations(&[request]),
            Err(KaniExecutionRefusal::HarnessProvenanceMismatch { .. })
        ));
        assert!(stand_in.calls().is_empty(), "nothing launched");
    }

    /// Incomplete capture refuses the real public run even when a cancellation follows its bytes.
    #[trace("FR-040-AC-3", "FR-040-AC-4")]
    #[test]
    fn output_over_limit_is_not_relabelled_as_cancellation() {
        let mut harness = member("capture_cancel", "check", 4);
        // This ceiling bounds a stuck launcher; draining the large stream can
        // take longer than the shared 30-second test ceiling under host load.
        let ceiling = Duration::from_secs(90);
        harness.identity.ceilings.wall_clock = ceiling;
        let body = format!(
            r#"printf '%s' "$$" > "$CALLS.child"
printf 'ready' > "$CALLS.ready"
head -c {} /dev/zero
printf 'written' > "$CALLS.written"
exec sleep 45
"#,
            CAPTURE_LIMIT + 1,
        );
        let stand_in = StandIn::running("capture-cancel-precedence", &body);
        stand_in.containing(&[&harness]);
        let result = std::thread::scope(|scope| {
            let worker =
                scope.spawn(|| execute_kani_obligation(&stand_in.request(&harness, ceiling)));
            let until = Instant::now() + ceiling;
            while !stand_in.directory.join("calls.written").is_file() {
                assert!(Instant::now() < until, "backend wrote over-limit output");
                std::thread::yield_now();
            }
            stand_in.cancel.cancel(quire_exact::CancelCause::Requested);
            worker.join().unwrap()
        });
        assert!(
            matches!(
                result,
                Err(KaniExecutionRefusal::OutputOverLimit {
                    stream: CaptureStream::Stdout,
                    limit: CAPTURE_LIMIT,
                    harnesses: 1,
                })
            ),
            "unexpected result: {result:?}"
        );
        recorded_process_gone(&stand_in.directory.join("calls.child"));
    }

    /// The public pre-spawn path retains the caller's source and starts no backend.
    #[trace("FR-040-AC-1", "FR-040-AC-6", "FR-040-AC-7")]
    #[test]
    fn cancellation_before_spawn_has_no_backend_or_report() {
        for (cause, source) in [
            (
                quire_exact::CancelCause::Requested,
                KaniCancellationSource::Requested,
            ),
            (
                quire_exact::CancelCause::Deadline,
                KaniCancellationSource::Deadline,
            ),
        ] {
            let harness = member("pre_cancel", "check", 4);
            let stand_in = StandIn::running("pre-cancel", VERIFY_EVERY_HARNESS);
            stand_in.containing(&[&harness]);
            stand_in.cancel.cancel(cause);
            for _ in 0..2 {
                let evidence = execute_kani_obligation(&stand_in.request(&harness, T)).unwrap();
                assert_eq!(evidence.outcome, KaniRunOutcome::Cancelled { source });
                assert!(evidence.checks.is_empty());
                assert_eq!(evidence.success_checks, 0);
                assert_eq!(evidence.exit_code, None);
                assert!(stand_in.calls().is_empty());
                let (ir, terminal) = evidence.cancellation_terminal_value().unwrap().unwrap();
                assert_eq!(ir.kind(), KaniOutcomeKind::Cancelled { source });
                assert_eq!(
                    terminal,
                    qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::Cancelled {
                        source: cause
                    })
                );
            }
        }
    }

    const CANCELLATION_BARRIER: &str = r#"printf '%s' "$$" > "$CALLS.child"
printf 'ready' > "$CALLS.ready"
exec sleep 45
"#;

    /// Run `run` while the stand-in backend is alive, record both `causes` in order from the
    /// caller's side once it reports ready, and return the run's result after the backend has
    /// been torn down.
    fn cancel_running<R: Send + std::fmt::Debug>(
        stand_in: &StandIn,
        cancel: &Cancel,
        causes: [quire_exact::CancelCause; 2],
        run: impl FnOnce() -> R + Send,
    ) -> R {
        std::thread::scope(|scope| {
            let worker = scope.spawn(run);
            let until = Instant::now() + T;
            while !stand_in.directory.join("calls.ready").is_file() {
                if Instant::now() >= until {
                    cancel.cancel(quire_exact::CancelCause::Requested);
                    panic!("the stand-in backend never reported ready");
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            cancel.cancel(causes[0]);
            cancel.cancel(causes[1]);
            assert_eq!(cancel.cause(), Some(causes[0]));
            let result = worker.join().expect("the execution worker joined");
            recorded_process_gone(&stand_in.directory.join("calls.child"));
            result
        })
    }

    /// One positively acknowledged capture operation; its production dependency remains real.
    #[derive(Debug)]
    struct OwnerGate {
        entered: std::sync::mpsc::SyncSender<()>,
        release: std::sync::Mutex<std::sync::mpsc::Receiver<()>>,
        ceiling: Duration,
        claimed: std::sync::atomic::AtomicBool,
        failed: std::sync::atomic::AtomicBool,
    }

    struct OwnerControl {
        entered: std::sync::mpsc::Receiver<()>,
        release: std::sync::mpsc::SyncSender<()>,
    }

    impl OwnerGate {
        fn new() -> (std::sync::Arc<Self>, OwnerControl) {
            let (entered, observed) = std::sync::mpsc::sync_channel(1);
            let (release, released) = std::sync::mpsc::sync_channel(1);
            (
                std::sync::Arc::new(Self {
                    entered,
                    release: std::sync::Mutex::new(released),
                    ceiling: T,
                    claimed: std::sync::atomic::AtomicBool::new(false),
                    failed: std::sync::atomic::AtomicBool::new(false),
                }),
                OwnerControl {
                    entered: observed,
                    release,
                },
            )
        }

        fn enter(&self) -> bool {
            use std::sync::atomic::Ordering;
            if self.claimed.swap(true, Ordering::AcqRel) {
                return false;
            }
            self.entered
                .send(())
                .expect("owner operation has a controller");
            self.release
                .lock()
                .unwrap()
                .recv_timeout(self.ceiling)
                .expect("owner operation released");
            true
        }

        fn refuse(&self) {
            self.failed
                .store(true, std::sync::atomic::Ordering::Release);
        }
    }

    impl OwnerControl {
        fn entered(&self) {
            self.entered
                .recv_timeout(T)
                .expect("actual owner operation reached");
        }
        fn release(&self) {
            self.release
                .send(())
                .expect("actual owner operation awaiting release");
        }
    }

    /// Fail the first capture poll once the backend is ready, after a controlled pause.
    #[derive(Debug)]
    struct ControlledPoll {
        ready: PathBuf,
        gate: std::sync::Arc<OwnerGate>,
    }

    impl super::super::owners::PollOperation for ControlledPoll {
        fn poll(
            &self,
            fds: &mut [rustix::event::PollFd<'_>],
            timeout: Option<&rustix::time::Timespec>,
        ) -> Result<usize, rustix::io::Errno> {
            let observed = rustix::event::poll(fds, timeout)?;
            if self.ready.is_file() && self.gate.enter() {
                self.gate.refuse();
                return Err(rustix::io::Errno::IO);
            }
            Ok(observed)
        }
    }

    /// Actual pipe polling failure refuses even when cancellation selected the preceding stop.
    #[trace("FR-040-AC-4")]
    #[test]
    fn unread_capture_competing_with_cancellation_refuses_public_evidence() {
        use super::super::owners::{LaunchOwners, SystemPoll};
        for cancel_before_unread in [false, true] {
            let harness = member("failed_capture", "check", 4);
            let mut stand_in = StandIn::running("cancel-unread-capture", CANCELLATION_BARRIER);
            stand_in.containing(&[&harness]);
            let (capture, pipe) = OwnerGate::new();
            let real = LaunchOwners::real();
            stand_in.installation = KaniInstallation::with_owners(
                stand_in.installation.launcher.clone(),
                std::sync::Arc::new(LaunchOwners {
                    stdout: std::sync::Arc::new(ControlledPoll {
                        ready: stand_in.directory.join("calls.ready"),
                        gate: capture.clone(),
                    }),
                    stderr: std::sync::Arc::new(SystemPoll),
                    teardown: std::sync::Arc::clone(&real.teardown),
                    peak: std::sync::Arc::clone(&real.peak),
                }),
            );
            std::thread::scope(|scope| {
                let worker =
                    scope.spawn(|| execute_kani_obligation(&stand_in.request(&harness, T)));
                pipe.entered();
                if cancel_before_unread {
                    stand_in.cancel.cancel(quire_exact::CancelCause::Requested);
                    pipe.release();
                } else {
                    pipe.release();
                    stand_in.cancel.cancel(quire_exact::CancelCause::Requested);
                }
                let refusal = worker.join().unwrap().unwrap_err();
                assert!(
                    matches!(
                        refusal,
                        KaniExecutionRefusal::OutputUnread {
                            stream: CaptureStream::Stdout,
                            ..
                        }
                    ),
                    "{refusal:?}"
                );
            });
            assert!(capture.failed.load(std::sync::atomic::Ordering::Acquire));
            assert_eq!(stand_in.calls().len(), 1);
            recorded_process_gone(&stand_in.directory.join("calls.child"));
        }
    }

    /// Reversing real caller events reverses the launch/evidence/IR/QSL cause; no report exists.
    #[trace("FR-040-AC-1", "FR-040-AC-2", "FR-040-AC-6")]
    #[test]
    fn live_cancellation_preserves_first_event_through_ir_and_qsl() {
        use quire_exact::CancelCause::{Deadline, Requested};
        for (causes, expected) in [
            ([Requested, Deadline], KaniCancellationSource::Requested),
            ([Deadline, Requested], KaniCancellationSource::Deadline),
        ] {
            let harness = member("live_cancel", "check", 4);
            let stand_in = StandIn::running("live-first-cancellation", CANCELLATION_BARRIER);
            stand_in.containing(&[&harness]);
            let caller = stand_in.cancel.clone();
            let evidence = cancel_running(&stand_in, &caller, causes, || {
                execute_kani_obligation(&stand_in.request(&harness, T))
            })
            .unwrap();
            assert_eq!(stand_in.calls().len(), 1);
            assert_eq!(
                evidence.outcome,
                KaniRunOutcome::Cancelled { source: expected }
            );
            assert_eq!(evidence.exit_code, None);
            assert_eq!(evidence.success_checks, 0);
            assert!(evidence.checks.is_empty());
            assert_eq!(evidence.memory.peak_resident_bytes, None);
            let (ir, terminal) = evidence.cancellation_terminal_value().unwrap().unwrap();
            assert_eq!(ir.kind(), KaniOutcomeKind::Cancelled { source: expected });
            assert_eq!(ir.boolean_claim(), None);
            assert_eq!(
                terminal,
                qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::Cancelled {
                    source: causes[0]
                })
            );
            caller.cancel(causes[1]);
            assert_eq!(
                evidence.cancellation_terminal_value().unwrap().unwrap(),
                (ir, terminal)
            );
        }
    }

    /// One cancelled group has no member evidence; an independent handle's group still completes.
    #[trace("FR-040-AC-5")]
    #[test]
    fn cancelled_batch_retains_first_source_and_independent_group() {
        use quire_exact::CancelCause::{Deadline, Requested};
        for (causes, expected) in [
            ([Requested, Deadline], KaniCancellationSource::Requested),
            ([Deadline, Requested], KaniCancellationSource::Deadline),
        ] {
            let harnesses = [
                member("a", "check", 4),
                member("b", "check", 4),
                member("c", "check", 4),
            ];
            let body = format!("case \"$ids\" in *a::check*)\n{CANCELLATION_BARRIER};;\nesac\n{VERIFY_EVERY_HARNESS}");
            let stand_in = StandIn::running("cancel-shared-batch", &body);
            stand_in.containing(&harnesses.iter().collect::<Vec<_>>());
            let independent = Cancel::new();
            let clone = stand_in.cancel.clone();
            let requests = [
                stand_in.request(&harnesses[0], T),
                KaniExecutionRequest {
                    cancel: &clone,
                    ..stand_in.request(&harnesses[1], T)
                },
                KaniExecutionRequest {
                    cancel: &independent,
                    ..stand_in.request(&harnesses[2], T)
                },
            ];
            let groups = cancel_running(&stand_in, &stand_in.cancel, causes, || {
                execute_kani_obligations(&requests)
            })
            .unwrap();
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0].members, [0, 1]);
            assert!(
                matches!(groups[0].evidence, Err(KaniExecutionRefusal::BatchCancelled { source, members: 2 }) if source == expected)
            );
            assert_eq!(groups[1].members, [2]);
            let evidence = groups[1].evidence.as_ref().unwrap();
            assert_eq!(evidence.len(), 1);
            assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
            assert_eq!(evidence[0].cancellation_terminal_value().unwrap(), None);
            let calls = stand_in.calls();
            assert_eq!(calls.len(), 2);
            for member in ["a::check", "b::check"] {
                assert!(calls[0]
                    .windows(3)
                    .any(|args| args == ["--harness", member, "--exact"]));
            }
        }
    }

    /// Otherwise groupable requests with distinct handles run in separate real processes.
    #[trace("FR-040-AC-5")]
    #[test]
    fn distinct_cancellation_handles_have_distinct_launchers() {
        let harnesses = [member("a", "check", 4), member("b", "check", 4)];
        let stand_in = StandIn::running("distinct-cancel-handles", VERIFY_EVERY_HARNESS);
        stand_in.containing(&[&harnesses[0], &harnesses[1]]);
        let other = Cancel::new();
        let requests = [
            stand_in.request(&harnesses[0], T),
            KaniExecutionRequest {
                cancel: &other,
                ..stand_in.request(&harnesses[1], T)
            },
        ];
        let groups = execute_kani_obligations(&requests).unwrap();
        assert_eq!(
            groups
                .iter()
                .map(|group| group.members.clone())
                .collect::<Vec<_>>(),
            [vec![0], vec![1]]
        );
        assert!(groups
            .iter()
            .all(|group| group.evidence.as_ref().unwrap()[0].outcome == KaniRunOutcome::Verified));
        assert_eq!(stand_in.calls().len(), 2);
    }

    /// A late timer event cannot relabel either a completed report or a no-event execution ceiling.
    #[trace("FR-040-AC-3", "FR-040-AC-4")]
    #[test]
    fn completed_and_elapsed_runs_do_not_acquire_late_cancellation() {
        let harness = member("late", "check", 4);
        let completed = StandIn::running("cancel-after-completion", VERIFY_EVERY_HARNESS);
        completed.containing(&[&harness]);
        let evidence = execute_kani_obligation(&completed.request(&harness, T)).unwrap();
        completed.cancel.cancel(quire_exact::CancelCause::Deadline);
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.cancellation_terminal_value().unwrap(), None);
        let mut timed_harness = harness.clone();
        timed_harness.identity.ceilings.wall_clock = Duration::ZERO;
        let timed = StandIn::running("cancel-after-ceiling", VERIFY_EVERY_HARNESS);
        timed.containing(&[&timed_harness]);
        let evidence =
            execute_kani_obligation(&timed.request(&timed_harness, Duration::ZERO)).unwrap();
        timed.cancel.cancel(quire_exact::CancelCause::Deadline);
        assert_eq!(
            evidence.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );
        assert_eq!(evidence.cancellation_terminal_value().unwrap(), None);
        assert!(timed.calls().is_empty());
    }

    /// The evidence of every member of a batch whose members share their proof process, in
    /// request order (a member held for the witness phase is in its witness group's run), or the
    /// first refusal.
    fn only_group(
        runs: Result<Vec<KaniGroupRun>, KaniExecutionRefusal>,
    ) -> Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal> {
        let runs = runs.expect("no harness is missing from the crate");
        let mut placed = Vec::new();
        for run in runs {
            let evidence = run.evidence?;
            placed.extend(run.members.into_iter().zip(evidence));
        }
        placed.sort_by_key(|(position, _)| *position);
        Ok(placed.into_iter().map(|(_, evidence)| evidence).collect())
    }

    fn outcomes(evidence: &[KaniExecutionEvidence]) -> Vec<KaniRunOutcome> {
        evidence
            .iter()
            .map(|evidence| evidence.outcome.clone())
            .collect()
    }

    fn only_report_group(
        runs: Result<Vec<report_fixture::ReportGroup>, KaniExecutionRefusal>,
    ) -> Result<Vec<report_fixture::FixtureExecution>, KaniExecutionRefusal> {
        let runs = runs.expect("no harness is missing from the crate");
        let mut placed = Vec::new();
        for run in runs {
            let reports = run.reports?;
            placed.extend(run.members.into_iter().zip(reports));
        }
        placed.sort_by_key(|(position, _)| *position);
        Ok(placed.into_iter().map(|(_, report)| report).collect())
    }

    fn reported_outcomes(reports: &[report_fixture::FixtureExecution]) -> Vec<KaniRunOutcome> {
        reports
            .iter()
            .map(|reported| reported.outcome.clone())
            .collect()
    }

    const PLAYBACK_BODY: &str = "fn from_{}() { kani::concrete_playback_run(v, h); }";

    /// A playback block headed for `path` that checks a `kind` property.
    pub(super) fn block(path: &str, kind: &str, name: &str) -> String {
        format!(
            "Concrete playback unit test for `{path}`:\n```\n/// Test generated for harness `{path}`\n///\n/// Check for `{kind}`: \"c\"\n\n#[test]\n{}\n```\n",
            PLAYBACK_BODY.replace("{}", name)
        )
    }

    pub(super) fn entry<'a>(
        harness_id: &'a str,
        status: &'a str,
        checks: &'a [(&'a str, &'a str)],
    ) -> BatchEntry<'a> {
        BatchEntry {
            harness_id,
            status,
            checks,
            exit_status: None,
        }
    }

    pub(super) const FAILED: (&str, &str) = ("Failure", "assertion");

    fn pair() -> Vec<StateFrameHarness> {
        vec![member("a", "check", 4), member("b", "check", 4)]
    }

    #[trace("FR-017-AC-21", "FR-017-AC-22", "TC-043")]
    #[test]
    fn tc_043_compatible_harnesses_start_one_process_with_one_selection_each_in_request_order() {
        for count in [1_usize, 10, 50] {
            let harnesses: Vec<_> = (0..count)
                .map(|index| member(&format!("m{index}"), "check", 4))
                .collect();

            let before = StandIn::verifying("batching-before");
            for harness in &harnesses {
                execute_kani_obligation(&before.request(harness, T)).unwrap();
            }
            assert_eq!(before.calls().len(), count, "one process each, before");

            let after = StandIn::verifying("batching-after");
            let runs = after.batch(&harnesses, T).unwrap();
            let calls = after.calls();
            assert_eq!(
                calls.len(),
                1,
                "one process for {count} compatible harnesses"
            );
            assert_eq!(runs.len(), 1);
            assert_eq!(runs[0].members, (0..count).collect::<Vec<_>>());
            let evidence = runs.into_iter().next().unwrap().evidence.unwrap();
            assert_eq!(evidence.len(), count);
            assert!(evidence
                .iter()
                .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));

            let received = &calls[0];
            let (report_flag, export) = received.split_at(received.len() - 1);
            assert_eq!(
                report_flag[report_flag.len() - 3..],
                ["-Z", "unstable-options", "--export-json"]
            );
            assert!(
                export[0].starts_with(&format!(
                    "{}/quire-kani-report-",
                    after.target_directory.display()
                )),
                "{export:?}"
            );
            let mut expected = vec!["kani".to_owned()];
            if count == 1 {
                expected.extend(harnesses[0].identity.options.iter().cloned());
            } else {
                for harness in &harnesses {
                    expected.extend([
                        "--harness".to_owned(),
                        path_of(harness),
                        "--exact".to_owned(),
                    ]);
                }
                expected.extend(["--harness-timeout".to_owned(), "30".to_owned()]);
                expected.extend(SHARED_OPTIONS.map(str::to_owned));
            }
            expected.extend(["-Z", "unstable-options", "--export-json"].map(str::to_owned));
            assert_eq!(report_flag, expected, "the vector the launcher received");
            for evidence in &evidence {
                assert_eq!(
                    evidence.arguments, *received,
                    "evidence names the real vector"
                );
                assert_eq!(evidence.batch.is_some(), count > 1);
                // Public wire claims must use actual bounded evidence, not a report fixture.
                let wire = serde_json::to_value(evidence).unwrap();
                if count == 1 {
                    assert!(wire.get("batch").is_none());
                } else {
                    let selections: Vec<_> = harnesses.iter().map(path_of).collect();
                    assert_eq!(
                        wire["batch"]["members"],
                        serde_json::to_value(selections).unwrap()
                    );
                    assert_eq!(wire["batch"]["timeoutSeconds"], 30);
                }
            }
        }
    }

    /// Harnesses with a different option vector or a different request timeout do not share a
    /// process: N harnesses in G such groups start G processes, each group's members in request
    /// order.
    #[trace("FR-017-AC-21", "TC-043")]
    #[test]
    fn tc_043_harnesses_group_by_option_vector_and_by_request_timeout() {
        // Two unwind bounds times two timeouts, two harnesses in each: eight, four groups,
        // requested interleaved so that a group is not a contiguous run of the request list.
        let cells = [(4, 30), (5, 30), (4, 60), (5, 60)];
        let harnesses: Vec<_> = (0..8)
            .map(|index| {
                let mut harness = member(&format!("m{index}"), "check", cells[index % 4].0);
                harness.identity.ceilings.wall_clock = Duration::from_secs(cells[index % 4].1);
                harness
            })
            .collect();
        let stand_in = StandIn::verifying("grouping");
        let requests: Vec<_> = harnesses
            .iter()
            .enumerate()
            .map(|(index, harness)| {
                stand_in.request(harness, Duration::from_secs(cells[index % 4].1))
            })
            .collect();
        let runs = report_fixture::groups(&requests).unwrap();
        assert_eq!(stand_in.calls().len(), 4, "eight harnesses in four groups");
        assert_eq!(
            runs.iter()
                .map(|run| run.members.clone())
                .collect::<Vec<_>>(),
            [vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]]
        );
        for run in runs {
            assert!(run
                .reports
                .unwrap()
                .iter()
                .all(|evidence| evidence.batch.is_some()));
        }
        // Equal options and timeouts again but in other crates would be two processes; with
        // one crate, one harness of each shape is two single runs, not a batch.
        let separate = StandIn::verifying("grouping-separate");
        let pair = [member("x", "check", 4), member("y", "check", 5)];
        let runs = separate.reported_batch(&pair, T).unwrap();
        assert_eq!(separate.calls().len(), 2);
        assert!(runs.iter().all(|run| run
            .reports
            .as_ref()
            .is_ok_and(|evidence| evidence.len() == 1 && evidence[0].batch.is_none())));
    }

    /// The per-member timeout is T in whole seconds rounded up; a T whose rounded value is above
    /// 4294967295 omits `--harness-timeout`, and one at the maximum carries it.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn tc_043_the_member_timeout_is_whole_seconds_rounded_up_and_omitted_above_kani_maximum() {
        let cancel = quire_exact::Cancel::new();
        let installation = KaniInstallation::new(PathBuf::from("cargo-kani"));
        let harness = member("a", "check", 4);
        let timeout_argument = |timeout: Duration| -> Option<String> {
            let mut harness = harness.clone();
            harness.identity.ceilings.wall_clock = timeout;
            let request = KaniExecutionRequest {
                cancel: &cancel,
                installation: &installation,
                harness: (&harness).into(),
                crate_directory: Path::new("/crate"),
                target_directory: Path::new("/target"),
                home_directory: None,
            };
            let (arguments, _) = batch_launch_command(
                &request,
                &["a::check".to_owned(), "b::check".to_owned()],
                Phase::Proof,
                Path::new("/target/report.json"),
            );
            let at = arguments
                .iter()
                .position(|argument| argument == "--harness-timeout");
            at.map(|at| arguments[at + 1].clone())
        };
        let max = u64::from(u32::MAX);
        for (timeout, expected) in [
            (Duration::from_secs(30), Some("30".to_owned())),
            (Duration::from_millis(1500), Some("2".to_owned())),
            (Duration::from_nanos(1), Some("1".to_owned())),
            (Duration::from_secs(max), Some(max.to_string())),
            (Duration::from_secs(max) + Duration::from_nanos(1), None),
            (Duration::from_secs(max + 1), None),
            (Duration::MAX, None),
        ] {
            assert_eq!(timeout_argument(timeout), expected, "{timeout:?}");
        }
    }

    /// A process has one launcher, one working directory and one target directory: requests that
    /// name another of any of the three are never grouped, however equal their options and
    /// timeouts. Each case runs two requests and counts the processes each launcher was started
    /// as.
    #[trace("FR-017-AC-21", "TC-043")]
    #[test]
    fn tc_043_requests_naming_another_launcher_crate_or_target_directory_are_not_grouped() {
        let cancel = quire_exact::Cancel::new();
        let (a, b) = (member("a", "check", 4), member("b", "check", 4));

        // Another launcher: each launcher runs one harness.
        let (first, second) = (StandIn::verifying("key-l1"), StandIn::verifying("key-l2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                cancel: &cancel,
                installation: &second.installation,
                ..first.request(&b, T)
            },
        ];
        let runs = report_fixture::groups(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!((first.calls().len(), second.calls().len()), (1, 1));

        // Another crate directory: two processes of the one launcher, one in each crate.
        let (first, second) = (StandIn::verifying("key-c1"), StandIn::verifying("key-c2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                cancel: &cancel,
                crate_directory: &second.crate_directory,
                ..first.request(&b, T)
            },
        ];
        let runs = report_fixture::groups(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!(first.calls().len(), 2);

        // Another target directory: the same.
        let (first, second) = (StandIn::verifying("key-t1"), StandIn::verifying("key-t2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                cancel: &cancel,
                target_directory: &second.target_directory,
                home_directory: None,
                ..first.request(&b, T)
            },
        ];
        let runs = report_fixture::groups(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!(first.calls().len(), 2);
    }

    /// A batch of N is bounded by N times T, not by T: four members at T = 20 s run for 41
    /// seconds, longer than T and than N times T halved (40 s), and inside N times T (80 s), and
    /// complete with every member verified.
    ///
    /// The stand-in's sleep never ends early, so it outlasts T and N times T halved however the
    /// scheduler treats the test; the 39 s between it and the 80 s bound tolerate host load.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t() {
        let stand_in = StandIn::running(
            "inside-n-times-t",
            &format!("sleep 41\n{VERIFY_EVERY_HARNESS}"),
        );
        let harnesses: Vec<_> = (0..4)
            .map(|index| member(&format!("m{index}"), "check", 4))
            .collect();
        let evidence = only_group(stand_in.batch(&harnesses, Duration::from_secs(20))).unwrap();
        assert_eq!(
            outcomes(&evidence),
            vec![KaniRunOutcome::Verified; 4],
            "a bound of T, not N times T, would have killed this batch"
        );
    }

    /// Two members at T = 45 s complete after 46 s: beyond (N - 1) times T (45 s), with
    /// 44 seconds left before N times T (90 s). The real batch launch must count both members.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn a_batch_finishes_between_one_fewer_members_bound_and_its_full_bound() {
        let stand_in = StandIn::running(
            "full-member-count-bound",
            &format!("sleep 46\n{VERIFY_EVERY_HARNESS}"),
        );
        let evidence = only_group(stand_in.batch(&pair(), Duration::from_secs(45)))
            .expect("both members must finish before their full 90-second batch bound");
        assert_eq!(
            outcomes(&evidence),
            vec![KaniRunOutcome::Verified; 2],
            "using one fewer member at the batch launch would kill this process before its report"
        );
        assert_eq!(stand_in.calls().len(), 1, "both members share one process");
    }

    /// The outer bound is the member count times T, and a product that does not fit never
    /// elapses.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn tc_043_the_outer_bound_is_member_count_times_timeout_and_overflow_never_elapses() {
        assert_eq!(
            outer_bound(Duration::from_secs(10), 3),
            Duration::from_secs(30)
        );
        assert_eq!(
            outer_bound(Duration::from_secs(10), 1),
            Duration::from_secs(10)
        );
        assert_eq!(outer_bound(Duration::MAX, 2), Duration::MAX);
        // A T past Kani's maximum launches without a member timeout, and its outer bound is
        // still the product where that fits.
        let past_kani = Duration::from_secs(u64::from(u32::MAX) + 1);
        assert_eq!(
            outer_bound(past_kani, 2),
            Duration::from_secs(2 * (u64::from(u32::MAX) + 1))
        );
        assert_eq!(
            outer_bound(Duration::from_secs(u64::MAX / 2 + 1), 2),
            Duration::MAX,
            "a product past the representable range is not wrapped"
        );
        assert_eq!(
            outer_bound(Duration::from_secs(1), usize::MAX),
            Duration::MAX,
            "a member count past u32 is not truncated"
        );
    }

    /// A batch still running at N times T is killed and refused as timed out, and no member is
    /// classified: the stand-in sleeps far past the bound and writes no report.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn tc_043_a_batch_still_running_at_its_outer_bound_is_refused_as_timed_out() {
        let stand_in = StandIn::running("outer-bound", "sleep 45");
        let outcome = only_group(stand_in.batch(&pair(), Duration::from_millis(200)));
        assert!(
            matches!(
                outcome,
                Err(KaniExecutionRefusal::BatchTimedOut { members: 2, timeout })
                    if timeout == Duration::from_millis(200)
            ),
            "{outcome:?}"
        );
        assert_eq!(
            fs::read_dir(&stand_in.target_directory).unwrap().count(),
            0,
            "the killed batch's report path is removed"
        );
    }

    /// A `Duration::MAX` batch launches without `--harness-timeout` and still runs (its bound
    /// never elapses); a batch at 4294967295 seconds carries it and runs.
    #[trace("FR-017-AC-21", "FR-028-AC-12", "TC-039", "TC-043")]
    #[test]
    fn tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one() {
        let without = StandIn::verifying("duration-max");
        let evidence = only_report_group(without.reported_batch(&pair(), Duration::MAX)).unwrap();
        assert!(evidence
            .iter()
            .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
        assert!(!without.calls()[0].contains(&"--harness-timeout".to_owned()));
        assert_eq!(
            evidence[0].batch.as_ref().unwrap().timeout_seconds,
            u64::MAX
        );

        let maximum = StandIn::verifying("kani-maximum");
        let seconds = u64::from(u32::MAX);
        let evidence =
            only_report_group(maximum.reported_batch(&pair(), Duration::from_secs(seconds)))
                .unwrap();
        assert!(evidence
            .iter()
            .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
        let received = &maximum.calls()[0];
        let at = received
            .iter()
            .position(|a| a == "--harness-timeout")
            .unwrap();
        assert_eq!(received[at + 1], seconds.to_string());
    }

    /// A batch that exits successfully and exported no report is refused as a single run is, and
    /// no member is classified; one that exits unsuccessfully with none leaves every member
    /// inconclusive `NoVerdict`, carrying the exit code and the batch statement.
    #[trace("FR-017-AC-21", "TC-043")]
    #[test]
    fn tc_043_a_batch_with_no_report_is_refused_after_a_clean_exit_and_no_verdict_after_a_failed_one(
    ) {
        let clean = StandIn::replaying("no-report-clean", None, "", 0);
        assert!(matches!(
            only_report_group(clean.reported_batch(&pair(), T)),
            Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
        ));
        let failed = StandIn::replaying("no-report-failed", None, "error: bad argument", 2);
        let evidence = only_report_group(failed.reported_batch(&pair(), T)).unwrap();
        assert_eq!(
            reported_outcomes(&evidence),
            vec![
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::NoVerdict
                };
                2
            ]
        );
        for evidence in &evidence {
            assert_eq!(evidence.exit_code, Some(2));
            assert_eq!(evidence.success_checks, 0);
            assert!(evidence.checks.is_empty());
            assert_eq!(
                evidence.batch.as_ref().unwrap().members,
                ["a::check", "b::check"]
            );
        }
    }

    /// Members are matched by `module::harness` path, never by the bare symbol both share, and
    /// each member's outcome, checks and SUCCESS count come from its own entry: the report lists
    /// the members in the reverse of the request order, one falsified and one verified.
    #[trace("FR-017-AC-22", "TC-043")]
    // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
    #[test]
    fn tc_043_members_are_matched_by_path_and_classified_from_their_own_entry() {
        let report = batch_report(&[
            entry("b::check", "Failure", &[FAILED, COVER_OK]),
            entry("a::check", "Success", &[PASSED, PASSED, COVER_OK]),
        ]);
        let witness = batch_report(&[entry("b::check", "Failure", &[FAILED, COVER_OK])]);
        let stand_in = StandIn::replaying_phases(
            "by-path",
            (Some(&report), "", 1),
            (
                Some(&witness),
                &block("b::check", "assertion", "falsify_b"),
                1,
            ),
        );
        let evidence = only_report_group(stand_in.reported_batch(&pair(), T)).unwrap();
        assert_eq!(evidence[0].harness_path, "src/generated/a.rs");
        assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence[0].success_checks, 2);
        assert_eq!(evidence[0].checks.len(), 3);
        assert_eq!(evidence[1].harness_path, "src/generated/b.rs");
        assert!(matches!(
            &evidence[1].outcome,
            KaniRunOutcome::Falsified { counterexample } if counterexample.contains("fn from_falsify_b")
        ));
        assert_eq!(evidence[1].success_checks, 0);
        assert_eq!(evidence[1].checks.len(), 2);
    }

    /// Each member's evidence carries its kind, harness path, launcher path, unwind bound, solver,
    /// outcome and checks beside the batch's argument vector, the member list, the statement that
    /// it was a batch and the exit code.
    #[trace("FR-017-AC-22", "TC-043")]
    #[test]
    fn tc_043_a_member_evidence_carries_its_own_fields_and_the_batch_invocation() {
        let report = batch_report(&[
            entry("a::check", "Success", &[PASSED, COVER_OK]),
            entry("b::check", "Success", &[PASSED, COVER_OK]),
        ]);
        let stand_in = StandIn::replaying("evidence-fields", Some(&report), "", 0);
        // Members share one option vector (so one process) but each evidence reads its own
        // identity's unwind bound.
        let mut second = member("b", "check", 4);
        second.identity.unwind = 7;
        let harnesses = [member("a", "check", 4), second];
        let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
        let received = stand_in.calls().remove(0);
        for (evidence, expected_unwind, module) in [(&evidence[0], 4, "a"), (&evidence[1], 7, "b")]
        {
            assert_eq!(evidence.kind, Some(ObligationKind::Frame));
            assert_eq!(evidence.harness_path, format!("src/generated/{module}.rs"));
            assert_eq!(
                evidence.launcher_path,
                stand_in.installation.launcher.display().to_string()
            );
            assert_eq!(evidence.unwind, expected_unwind);
            assert_eq!(evidence.solver, KaniSolver::Cadical);
            assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
            assert_eq!(evidence.checks.len(), 2);
            assert_eq!(evidence.arguments, received);
            assert_eq!(evidence.exit_code, Some(0));
            assert_eq!(
                evidence.batch,
                Some(KaniBatchInvocation {
                    members: vec!["a::check".to_owned(), "b::check".to_owned()],
                    timeout_seconds: 30,
                })
            );
        }
        // Only actual bounded execution yields public wire evidence. Unsupported hosts keep
        // every metadata assertion above without serializing a second wire contract.
        for (actual, module) in evidence.iter().zip(["a", "b"]) {
            let wire = serde_json::to_value(actual).unwrap();
            assert_eq!(
                wire["launcherPath"],
                stand_in.installation.launcher.display().to_string()
            );
            assert_eq!(wire["harnessPath"], format!("src/generated/{module}.rs"));
            assert_eq!(
                wire["kind"],
                serde_json::to_value(ObligationKind::Frame).unwrap()
            );
            assert_eq!(
                wire["solver"],
                serde_json::to_value(KaniSolver::Cadical).unwrap()
            );
            assert_eq!(wire["checks"].as_array().unwrap().len(), 2);
            assert_eq!(wire["batch"]["members"][1], "b::check");
            assert_eq!(wire["batch"]["timeoutSeconds"], 30);
        }
        // A single run's evidence has no `batch` member at all.
        let single = StandIn::verifying("evidence-single");
        let alone = report_fixture::single(&single.request(&harnesses[0], T)).unwrap();
        assert!(alone.batch.is_none());
        assert!(serde_json::to_value(&alone).unwrap().get("batch").is_none());
    }

    /// A member's success is believed beside a non-zero exit only when some entry states the
    /// failure that exit stands for: every member Success with exit 1 is inconclusive, and one
    /// Failure entry makes the other member's success verified.
    #[trace("FR-017-AC-22", "TC-043")]
    // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
    #[test]
    fn tc_043_a_success_beside_a_non_zero_exit_is_verified_only_when_an_entry_states_failure() {
        let no_verdict = KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict,
        };
        let all_success = batch_report(&[
            entry("a::check", "Success", &[PASSED, COVER_OK]),
            entry("b::check", "Success", &[PASSED, COVER_OK]),
        ]);
        let unexplained = StandIn::replaying("exit-unexplained", Some(&all_success), "", 1);
        assert_eq!(
            reported_outcomes(&only_report_group(unexplained.reported_batch(&pair(), T)).unwrap()),
            vec![no_verdict.clone(); 2]
        );
        let explained_report = batch_report(&[
            entry("a::check", "Success", &[PASSED, COVER_OK]),
            entry("b::check", "Failure", &[FAILED, COVER_OK]),
        ]);
        let console = block("b::check", "assertion", "falsify_b");
        let witness = batch_report(&[entry("b::check", "Failure", &[FAILED, COVER_OK])]);
        let explained = StandIn::replaying_phases(
            "exit-explained",
            (Some(&explained_report), "", 1),
            (Some(&witness), &console, 1),
        );
        let evidence = only_report_group(explained.reported_batch(&pair(), T)).unwrap();
        assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
        assert!(matches!(
            evidence[1].outcome,
            KaniRunOutcome::Falsified { .. }
        ));
        // The same two entries and a zero exit are the same results: the exit only withholds.
        let clean = StandIn::replaying("exit-zero", Some(&all_success), "", 0);
        assert_eq!(
            reported_outcomes(&only_report_group(clean.reported_batch(&pair(), T)).unwrap()),
            vec![KaniRunOutcome::Verified; 2]
        );
    }

    /// An entry that states failure with no checks and exit status `timeout` is inconclusive as
    /// timed out, naming T, while the others keep their results; one with no checks and no
    /// timeout is inconclusive with no counterexample.
    #[trace("FR-017-AC-22", "FR-028-AC-12", "TC-039", "TC-043")]
    // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
    #[test]
    fn tc_043_a_timed_out_entry_is_timed_out_naming_t_and_a_bare_failure_has_no_counterexample() {
        let report = batch_report(&[
            entry("a::check", "Success", &[PASSED, COVER_OK]),
            BatchEntry {
                harness_id: "b::check",
                status: "Failure",
                checks: &[],
                exit_status: Some("timeout"),
            },
            entry("c::check", "Failure", &[]),
            // Kani's timeout flag with checks listed: a failure that has checks is classified
            // from them, not as a cut-off.
            BatchEntry {
                harness_id: "d::check",
                status: "Failure",
                checks: &[FAILED],
                exit_status: Some("timeout"),
            },
        ]);
        let witness = batch_report(&[BatchEntry {
            harness_id: "d::check",
            status: "Failure",
            checks: &[FAILED],
            exit_status: Some("timeout"),
        }]);
        let stand_in = StandIn::replaying_phases(
            "timeout-entry",
            (Some(&report), "", 1),
            (Some(&witness), "", 1),
        );
        let harnesses = [
            member("a", "check", 4),
            member("b", "check", 4),
            member("c", "check", 4),
            member("d", "check", 4),
        ];
        // 90.5 s: Kani is told 91 whole seconds and the evidence names 91, rounded up.
        let evidence =
            only_report_group(stand_in.reported_batch(&harnesses, Duration::from_millis(90_500)))
                .unwrap();
        assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
        assert_eq!(
            evidence[1].outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );
        assert_eq!(evidence[1].batch.as_ref().unwrap().timeout_seconds, 91);
        let received = &stand_in.calls()[0];
        let at = received
            .iter()
            .position(|a| a == "--harness-timeout")
            .unwrap();
        assert_eq!(
            received[at + 1],
            "91",
            "the T the timed-out member is told it had"
        );
        assert_eq!(
            evidence[2].outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
        );
        assert_eq!(
            evidence[3].outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            },
            "a failed check and no block for d: no counterexample, not a timeout"
        );
    }

    /// A batch report that lacks a requested harness, holds one twice or holds one that was not
    /// requested refuses the whole batch with its own typed cause.
    #[trace("FR-017-AC-23", "TC-043")]
    #[test]
    fn tc_043_a_report_that_lacks_repeats_or_adds_a_harness_refuses_the_batch() {
        let verified = [PASSED, COVER_OK];
        let refusal = |name: &str, entries: &[BatchEntry<'_>]| {
            let stand_in = StandIn::replaying(name, Some(&batch_report(entries)), "", 0);
            only_report_group(stand_in.reported_batch(&pair(), T)).unwrap_err()
        };
        assert!(matches!(
            refusal("lacks", &[entry("a::check", "Success", &verified)]),
            KaniExecutionRefusal::Report(KaniReportRefusal::HarnessMissing { harness })
                if harness == "b::check"
        ));
        assert!(matches!(
            refusal(
                "repeats",
                &[
                    entry("a::check", "Success", &verified),
                    entry("a::check", "Success", &verified),
                    entry("b::check", "Success", &verified),
                ]
            ),
            KaniExecutionRefusal::Report(KaniReportRefusal::HarnessDuplicated { harness })
                if harness == "a::check"
        ));
        assert!(matches!(
            refusal(
                "adds",
                &[
                    entry("a::check", "Success", &verified),
                    entry("b::check", "Success", &verified),
                    entry("c::check", "Success", &verified),
                ]
            ),
            KaniExecutionRefusal::Report(KaniReportRefusal::HarnessUnrequested { harness })
                if harness == "c::check"
        ));
    }

    /// A member whose source is not in the crate refuses the whole batch with
    /// `HarnessNotInCrate` before any process starts, even when the other members are present.
    #[trace("FR-017-AC-23", "TC-043")]
    #[test]
    fn tc_043_a_member_missing_from_the_crate_refuses_the_batch_and_starts_nothing() {
        let stand_in = StandIn::verifying("missing-member");
        let mut harnesses = pair();
        harnesses[1].rust =
            Artifact::new("src/generated/b.rs", "pub fn absent_from_the_crate() {}");
        match stand_in.batch(&harnesses, T) {
            Err(KaniExecutionRefusal::HarnessNotInCrate { harness_path }) => {
                assert_eq!(harness_path, "src/generated/b.rs");
            }
            other => panic!("expected HarnessNotInCrate, got {other:?}"),
        }
        assert!(stand_in.calls().is_empty(), "nothing was launched");
    }

    /// A falsified member's playback is the console block headed for its own path, never an
    /// earlier member's failing block; a member whose entry names a failed property and has no
    /// block headed for it is inconclusive with no counterexample, and its neighbours keep their
    /// results.
    #[trace("FR-017-AC-23", "TC-043")]
    // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
    #[test]
    fn tc_043_a_member_is_given_the_playback_headed_for_its_own_path() {
        let report = batch_report(&[
            entry("c::check", "Failure", &[FAILED, COVER_OK]),
            entry("a::check", "Failure", &[FAILED, COVER_OK]),
            entry("b::check", "Success", &[PASSED, COVER_OK]),
            entry("d::check", "Failure", &[FAILED, COVER_OK]),
        ]);
        // `d` is falsified but Kani printed no block headed for it.
        let console = format!(
            "{}{}{}",
            block("a::check", "assertion", "falsify_a"),
            block("c::check", "cover", "cover_c"),
            block("c::check", "assertion", "falsify_c"),
        );
        let witness = batch_report(&[
            entry("c::check", "Failure", &[FAILED, COVER_OK]),
            entry("a::check", "Failure", &[FAILED, COVER_OK]),
            entry("d::check", "Failure", &[FAILED, COVER_OK]),
        ]);
        let stand_in = StandIn::replaying_phases(
            "own-playback",
            (Some(&report), "", 1),
            (Some(&witness), &console, 1),
        );
        let harnesses = [
            member("a", "check", 4),
            member("b", "check", 4),
            member("c", "check", 4),
            member("d", "check", 4),
        ];
        let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
        assert!(matches!(
            &evidence[0].outcome,
            KaniRunOutcome::Falsified { counterexample } if counterexample.contains("fn from_falsify_a")
        ));
        assert_eq!(evidence[1].outcome, KaniRunOutcome::Verified);
        assert!(
            matches!(
                &evidence[2].outcome,
                KaniRunOutcome::Falsified { counterexample }
                    if counterexample.contains("fn from_falsify_c")
                        && !counterexample.contains("falsify_a")
            ),
            "c takes its own block, not a's earlier one: {:?}",
            evidence[2].outcome
        );
        assert_eq!(
            evidence[3].outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample
            }
        );
    }

    /// The playback section of Kani 0.68's console for a batch of `two::check`, whose two
    /// assertions fail on independent paths at different valuations, and `ok::check`, verbatim
    /// from a real run: one cover block and one counterexample block per distinct failing
    /// valuation, all headed `two::check`, then
    /// `ok::check`'s cover block. (The trailing space after the harness path in the `Test
    /// generated` line is Kani's.)
    const TWO_FAILED_CHECKS_CONSOLE: &str = r#"Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `cover`: "x can be three"

#[test]
fn kani_concrete_playback_check_1077496887511954657() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 3
        vec![3],
    ];
    kani::concrete_playback_run(concrete_vals, check);
}
```
Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `assertion`: ""first""

#[test]
fn kani_concrete_playback_check_1550118722174123344() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 8
        vec![8],
        // 1
        vec![1],
    ];
    kani::concrete_playback_run(concrete_vals, check);
}
```
Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `assertion`: ""second""

#[test]
fn kani_concrete_playback_check_13421931990910725816() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 7
        vec![7],
        // 0
        vec![0],
    ];
    kani::concrete_playback_run(concrete_vals, check);
}
```
INFO: To automatically add the concrete playback unit test(s) to the src code, run Kani with `--concrete-playback=inplace`.
Concrete playback unit test for `ok::check`:
```
/// Test generated for harness `ok::check`
///
/// Check for `cover`: "x can be three"

#[test]
fn kani_concrete_playback_check_1077496887511954657() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 3
        vec![3],
    ];
    kani::concrete_playback_run(concrete_vals, check);
}
```
"#;

    /// A member that fails two property checks has two counterexample blocks under its own path in
    /// real Kani's console. It takes the first, as the same harness run alone does, and neither
    /// it nor its verified neighbour loses its evidence: the group is not refused.
    #[trace("FR-017-AC-22", "FR-017-AC-23", "TC-043")]
    // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
    #[test]
    fn tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour() {
        let report = batch_report(&[
            entry(
                "two::check",
                "Failure",
                &[("Satisfied", "cover"), FAILED, FAILED],
            ),
            entry("ok::check", "Success", &[PASSED, COVER_OK]),
        ]);
        let witness = batch_report_single_two();
        let stand_in = StandIn::replaying_phases(
            "two-failed-checks",
            (Some(&report), "", 1),
            (Some(&witness), TWO_FAILED_CHECKS_CONSOLE, 1),
        );
        let harnesses = [member("two", "check", 4), member("ok", "check", 4)];
        let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
        let KaniRunOutcome::Falsified { counterexample } = &evidence[0].outcome else {
            panic!("two::check is falsified: {:?}", evidence[0].outcome);
        };
        assert!(counterexample.contains("\"\"first\"\""), "{counterexample}");
        assert!(!counterexample.contains("second"), "{counterexample}");
        assert_eq!(evidence[0].success_checks, 0);
        assert_eq!(evidence[1].outcome, KaniRunOutcome::Verified);
        // The same harness alone, with the same console, is falsified with that same block.
        let alone = classify_kani_run(
            false,
            Some(&batch_report_single_two()),
            select_counterexample(TWO_FAILED_CHECKS_CONSOLE, None),
            None,
        )
        .unwrap();
        assert_eq!(alone.outcome, evidence[0].outcome);
    }

    /// The report of `two::check` alone, as a single run exports it.
    fn batch_report_single_two() -> Vec<u8> {
        batch_report(&[entry(
            "two::check",
            "Failure",
            &[("Satisfied", "cover"), FAILED, FAILED],
        )])
    }

    /// A console block headed for a path that is not a member refuses the whole group and
    /// classifies no member.
    #[trace("FR-017-AC-23", "TC-043")]
    #[test]
    fn tc_043_an_unattributable_playback_refuses_the_whole_group() {
        let report = batch_report(&[
            entry("a::check", "Failure", &[FAILED, COVER_OK]),
            entry("b::check", "Success", &[PASSED, COVER_OK]),
        ]);
        let stranger = format!(
            "{}{}",
            block("a::check", "assertion", "falsify_a"),
            block("z::check", "cover", "cover_z")
        );
        let stand_in = StandIn::replaying("playback-stranger", Some(&report), &stranger, 1);
        assert!(matches!(
            only_report_group(stand_in.reported_batch(&pair(), T)),
            Err(KaniExecutionRefusal::PlaybackForNonMember { harness }) if harness == "z::check"
        ));
    }

    /// A run over the limit for its harness count is refused with the stable code naming the
    /// stream, the limit and the count: one harness at 8 MiB, a batch of two at 16 MiB; a batch
    /// at exactly its limit completes with its real exit status.
    #[trace("FR-017-AC-14", "TC-043")]
    #[test]
    fn tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count() {
        let flood = |bytes: usize, exit: i32| {
            // An observable descendant remains alive until the group is killed; over-limit and
            // completed conclusions must both kill it.
            let descendant = r#"sleep 45 &
printf '%s' "$!" > "$CALLS.child"
"#;
            format!("{descendant}head -c {bytes} /dev/zero; exit {exit}")
        };
        let one = StandIn::running("flood-one", &flood(CAPTURE_LIMIT + 1, 0));
        let harness = member("a", "check", 4);
        let single = report_fixture::single(&one.request(&harness, T)).unwrap_err();
        assert!(matches!(
            single,
            KaniExecutionRefusal::OutputOverLimit {
                stream: CaptureStream::Stdout,
                limit,
                harnesses: 1
            } if limit == CAPTURE_LIMIT
        ));
        assert_eq!(single.code(), Some("kani_output_over_limit"));
        recorded_process_gone(&one.directory.join("calls.child"));

        let two = StandIn::running("flood-two", &flood(2 * CAPTURE_LIMIT + 1, 0));
        let batch = only_report_group(two.reported_batch(&pair(), T)).unwrap_err();
        assert!(matches!(
            batch,
            KaniExecutionRefusal::OutputOverLimit {
                harnesses: 2,
                limit,
                ..
            } if limit == 2 * CAPTURE_LIMIT
        ));
        recorded_process_gone(&two.directory.join("calls.child"));

        let exact = StandIn::running("flood-exact", &flood(2 * CAPTURE_LIMIT, 1));
        let evidence = only_report_group(exact.reported_batch(&pair(), T)).unwrap();
        recorded_process_gone(&exact.directory.join("calls.child"));
        assert!(evidence
            .iter()
            .all(|evidence| evidence.exit_code == Some(1)));
        assert_eq!(
            reported_outcomes(&evidence),
            vec![
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::NoVerdict
                };
                2
            ]
        );
    }

    /// The diagnostic record of a run Kani gave no verdict for, or whose CBMC failed, over launcher stand-ins
    /// (FR-017-AC-32 to FR-017-AC-36).
    pub(crate) mod no_verdict_diagnostics {
        use super::*;
        use crate::kani::{
            output::diagnostics::KaniFailurePhase,
            test_support::{report, COVER_NO},
        };

        const BANNER: &str = "Kani Rust Verifier 0.68.0 (cargo plugin)\n";
        const SOLVING: &str = "Solving with CaDiCaL 3.0.0\n";
        const NO_VERDICT: KaniRunOutcome = KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::NoVerdict,
        };

        /// A stand-in that exports `report` (when given), prints `stdout` and `stderr` apart and
        /// exits with `exit`.
        fn printing(
            name: &str,
            report: Option<&[u8]>,
            stdout: &str,
            stderr: &str,
            exit: i32,
        ) -> StandIn {
            let stand_in = StandIn::scaffold(name);
            stand_in.install(&print_body(&stand_in, report, exit));
            stand_in.set_streams(stdout, stderr);
            stand_in
        }

        fn print_body(stand_in: &StandIn, report: Option<&[u8]>, exit: i32) -> String {
            let mut body = String::new();
            if let Some(report) = report {
                let exported = stand_in.directory.join("report.json");
                fs::write(&exported, report).unwrap();
                body.push_str(&format!("cp '{}' \"$last\"\n", exported.display()));
            }
            body.push_str(&format!(
                "cat '{0}/stdout'\ncat '{0}/stderr' 1>&2\nexit {exit}\n",
                stand_in.directory.display()
            ));
            body
        }

        impl StandIn {
            /// What the next run of a `printing` stand-in writes to each stream.
            fn set_streams(&self, stdout: &str, stderr: &str) {
                fs::write(self.directory.join("stdout"), stdout).unwrap();
                fs::write(self.directory.join("stderr"), stderr).unwrap();
            }
        }

        fn proofs() -> StateFrameHarness {
            member("proofs", "check", 4)
        }

        fn run(stand_in: &StandIn, harness: &StateFrameHarness) -> KaniExecutionEvidence {
            execute_kani_obligation(&stand_in.request(harness, T)).unwrap()
        }

        fn diagnostics(evidence: &KaniExecutionEvidence) -> &KaniNoVerdictDiagnostics {
            evidence
                .diagnostics
                .as_ref()
                .unwrap_or_else(|| panic!("no diagnostic record on {:?}", evidence.outcome))
        }

        fn wire_has_no_diagnostics(evidence: &KaniExecutionEvidence) -> bool {
            serde_json::to_value(evidence)
                .unwrap()
                .as_object()
                .unwrap()
                .get("diagnostics")
                .is_none()
        }

        /// A launcher that exits 1, exports no report and prints nothing is `NoVerdict` with
        /// phase `launcher`, two empty excerpts and the exit code.
        ///
        #[trace("FR-017-AC-32", "TC-027")]
        #[test]
        fn tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher() {
            let stand_in = printing("diag-silent", None, "", "", 1);
            let evidence = run(&stand_in, &proofs());
            assert_eq!(evidence.outcome, NO_VERDICT);
            assert_eq!(evidence.exit_code, Some(1));
            let record = diagnostics(&evidence);
            assert_eq!(record.phase, KaniFailurePhase::Launcher);
            for excerpt in [&record.stdout, &record.stderr] {
                assert_eq!((excerpt.text.as_str(), excerpt.omitted_bytes), ("", 0));
            }
            assert_eq!(
                serde_json::to_value(&evidence).unwrap()["diagnostics"],
                serde_json::json!({
                    "phase": "launcher",
                    "stdout": { "text": "", "omittedBytes": 0 },
                    "stderr": { "text": "", "omittedBytes": 0 },
                })
            );
        }

        /// One launcher stand-in of the marker table: what it prints, whether it exports a report,
        /// the extra options of its harness and the phase expected.
        struct Case<'a> {
            label: &'a str,
            stdout: String,
            stderr: &'a str,
            exported: Option<&'a [u8]>,
            extra: &'a [&'a str],
            expected: KaniFailurePhase,
        }

        fn case<'a>(
            label: &'a str,
            stdout: String,
            stderr: &'a str,
            exported: Option<&'a [u8]>,
            extra: &'a [&'a str],
            expected: KaniFailurePhase,
        ) -> Case<'a> {
            Case {
                label,
                stdout,
                stderr,
                exported,
                extra,
                expected,
            }
        }

        /// Single-run stand-ins that exit 1: each marker set gives its phase, and the marker
        /// need not be in the excerpt.
        ///
        #[trace("FR-017-AC-33", "TC-027")]
        #[test]
        fn tc_027_marker_stand_ins_decide_the_failure_phase() {
            let checking = "Checking harness proofs::check...\n";
            let cargo_failed =
                "error: Failed to execute cargo (exit status: 101). Found 1 compilation errors.\n";
            let filler = "filler line\n".repeat(2000);
            let solver_stdout = format!("{BANNER}{checking}{SOLVING}");
            let long_after = format!("{solver_stdout}{filler}");
            let success = report("Success", &[PASSED, COVER_OK]);
            let cases = [
                case(
                    "solver",
                    solver_stdout.clone(),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Solver,
                ),
                case(
                    "backend",
                    format!("{BANNER}{checking}"),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Backend,
                ),
                case(
                    "compile-stdout",
                    format!("{BANNER}{cargo_failed}"),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Compile,
                ),
                case(
                    "compile-stderr",
                    BANNER.to_owned(),
                    "error: could not compile `x` (lib) due to 1 previous error\n",
                    None,
                    &[],
                    KaniFailurePhase::Compile,
                ),
                case(
                    "launcher",
                    String::new(),
                    "error: unexpected argument\n",
                    None,
                    &[],
                    KaniFailurePhase::Launcher,
                ),
                case(
                    "banner",
                    BANNER.to_owned(),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Unknown,
                ),
                case(
                    "other-wording",
                    format!("{BANNER}{checking}Running the SAT solver CaDiCaL\n"),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Backend,
                ),
                case(
                    "foreign-path",
                    format!("{BANNER}Checking harness other::path...\n{SOLVING}"),
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Unknown,
                ),
                case(
                    "parallel",
                    solver_stdout.clone(),
                    "",
                    None,
                    &["-j", "2"],
                    KaniFailurePhase::Unknown,
                ),
                case(
                    "report",
                    solver_stdout,
                    "",
                    Some(&success),
                    &[],
                    KaniFailurePhase::Unknown,
                ),
                case(
                    "before-window",
                    long_after,
                    "",
                    None,
                    &[],
                    KaniFailurePhase::Solver,
                ),
            ];
            for Case {
                label,
                stdout,
                stderr,
                exported,
                extra,
                expected,
            } in cases
            {
                let stand_in = printing(&format!("diag-{label}"), exported, &stdout, stderr, 1);
                let mut harness = proofs();
                harness
                    .identity
                    .options
                    .extend(extra.iter().map(|o| (*o).to_owned()));
                let evidence = run(&stand_in, &harness);
                assert_eq!(evidence.outcome, NO_VERDICT, "{label}");
                assert_eq!(diagnostics(&evidence).phase, expected, "{label}");
                if label == "before-window" {
                    let kept = &diagnostics(&evidence).stdout;
                    assert!(kept.omitted_bytes > 0 && kept.text.len() <= 8192);
                    assert!(!kept.text.contains("Solving with"));
                }
            }
        }

        /// The redaction of the streams of a real run uses the request's directories: the crate,
        /// target and home directory it was given, and nothing from the process environment.
        ///
        #[trace("FR-017-AC-35", "TC-027")]
        #[test]
        fn tc_027_a_runs_streams_are_redacted_against_the_requests_directories() {
            let stand_in = StandIn::scaffold("diag-redact");
            stand_in.install(&print_body(&stand_in, None, 1));
            let (crate_directory, target, home) = (
                stand_in.crate_directory.display().to_string(),
                stand_in.target_directory.display().to_string(),
                stand_in.directory.display().to_string(),
            );
            let stdout = format!(
                "{BANNER}({crate_directory})\n{crate_directory}/src/lib.rs:6:21\n{target}/debug/x.out\n{home}/.kani/bin\n/opt/tool/bin\nhttps://host/x/y\n2026-10-16T22:27:04.275975036Z\nRuntime Solver: 1.385e-05s\ncheck_ttl_30s\nthread 'main' (4242) panicked\n"
            );
            stand_in.set_streams(&stdout, "pid 4242\n");
            let harness = proofs();
            let home_directory = PathBuf::from(&home);
            let with_home = execute_kani_obligation(&KaniExecutionRequest {
                home_directory: Some(&home_directory),
                ..stand_in.request(&harness, T)
            })
            .unwrap();
            let record = diagnostics(&with_home);
            assert_eq!(
                record.stdout.text,
                format!(
                    "{BANNER}(<CRATE_DIR>)\n<CRATE_DIR>/src/lib.rs:6:21\n<TARGET_DIR>/debug/x.out\n<HOME>/.kani/bin\n<PATH>\nhttps://host/x/y\n<TIMESTAMP>\nRuntime Solver: <DURATION>\ncheck_ttl_30s\nthread 'main' (<PID>) panicked\n"
                )
            );
            assert_eq!(record.stderr.text, "pid <PID>\n");

            let without_home = run(&stand_in, &harness);
            let text = &diagnostics(&without_home).stdout.text;
            assert!(
                !text.contains("<HOME>") && text.contains("<PATH>\n<PATH>\n"),
                "{text}"
            );
        }

        /// Two runs whose streams differ only in redacted values serialize byte-identical records.
        ///
        #[trace("FR-017-AC-35", "TC-027")]
        #[test]
        fn tc_027_runs_differing_only_in_redacted_values_serialize_identical_records() {
            let stand_in = StandIn::scaffold("diag-identical");
            stand_in.install(&print_body(&stand_in, None, 1));
            let dir = stand_in.crate_directory.display().to_string();
            let harness = proofs();
            let mut records = Vec::new();
            for (stamp, seconds, pid) in [
                ("2026-10-16T22:27:04.275975036Z", "0.00331318s", 4242),
                ("2025-01-02T03:04:05+01:00", "1.385e-05s", 17),
            ] {
                stand_in.set_streams(
                    &format!("{BANNER}{dir}/src/lib.rs:6:21\nat {stamp} in {seconds}\n"),
                    &format!("process {pid} aborted\n"),
                );
                records
                    .push(serde_json::to_string(diagnostics(&run(&stand_in, &harness))).unwrap());
            }
            assert_eq!(records[0], records[1]);
        }

        /// The process `HOME` is never read: the same run under two `HOME` values gives the same
        /// record. The probe below runs in a child test process that carries the `HOME`.
        ///
        #[trace("FR-017-AC-35", "TC-027")]
        #[test]
        fn tc_027_the_same_run_under_two_process_homes_gives_the_same_record() {
            let exe = std::env::current_exe().unwrap();
            let scratch = discover_scratch("diag-home");
            let mut records = Vec::new();
            for home in ["/ir809-home-a", "/ir809-home-b"] {
                let out = scratch.join(home.trim_start_matches('/'));
                let status = std::process::Command::new(&exe)
                    .args(["--exact", PROBE, "--test-threads=1"])
                    .env("HOME", home)
                    .env(PROBE_OUT, &out)
                    .status()
                    .unwrap();
                assert!(status.success());
                records.push(fs::read_to_string(&out).expect("the probe wrote its record"));
            }
            let _ = fs::remove_dir_all(scratch);
            assert!(records[0].contains("<PATH>"), "{}", records[0]);
            assert_eq!(records[0], records[1]);
        }

        const PROBE: &str = "kani::run::execute::batch_tests::no_verdict_diagnostics::home_probe";
        const PROBE_OUT: &str = "IR809_HOME_PROBE_OUT";

        /// Run by the test above in a child process; a no-op elsewhere.
        #[test]
        fn home_probe() {
            let Ok(out) = std::env::var(PROBE_OUT) else {
                return;
            };
            let home = std::env::var("HOME").unwrap();
            let stand_in = StandIn::scaffold("diag-home-probe");
            stand_in.install(&print_body(&stand_in, None, 1));
            stand_in.set_streams(&format!("{BANNER}{home}/.kani/bin\n"), "");
            let evidence = run(&stand_in, &proofs());
            fs::write(out, serde_json::to_string(diagnostics(&evidence)).unwrap()).unwrap();
        }

        /// Apart from `NoVerdict` evidence and the `FailedWithoutCounterexample` evidence of a CBMC
        /// failure, no evidence has a `diagnostics` member on the wire: verified,
        /// cover-unsatisfied, falsified, timed-out, memory-exhausted and other inconclusive runs
        /// have none.
        ///
        #[trace("FR-017-AC-32", "TC-027")]
        #[test]
        fn tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member() {
            let harness = proofs();
            let noisy = format!("{BANNER}error: could not compile\n{SOLVING}");
            let falsifying = block("proofs::check", "assertion", "falsify");
            let runs: [(&str, Option<Vec<u8>>, String, i32); 4] = [
                (
                    "verified",
                    Some(report("Success", &[PASSED, COVER_OK])),
                    noisy.clone(),
                    0,
                ),
                (
                    "cover",
                    Some(report("Success", &[PASSED, COVER_NO])),
                    noisy.clone(),
                    0,
                ),
                (
                    "falsified",
                    Some(report("Failure", &[FAILED, COVER_OK])),
                    falsifying,
                    1,
                ),
                (
                    "failed-bare",
                    Some(report("Failure", &[FAILED, COVER_OK])),
                    noisy,
                    1,
                ),
            ];
            let mut seen = Vec::new();
            for (label, exported, stdout, exit) in runs {
                let stand_in = printing(
                    &format!("diag-other-{label}"),
                    exported.as_deref(),
                    &stdout,
                    "",
                    exit,
                );
                let evidence = run(&stand_in, &harness);
                assert!(
                    wire_has_no_diagnostics(&evidence),
                    "{label}: {:?}",
                    evidence.outcome
                );
                assert!(evidence.diagnostics.is_none());
                seen.push(evidence.outcome);
            }
            assert!(matches!(seen[0], KaniRunOutcome::Verified));
            assert!(matches!(seen[1], KaniRunOutcome::CoverUnsatisfied { .. }));
            assert!(matches!(seen[2], KaniRunOutcome::Falsified { .. }));
            assert!(matches!(
                seen[3],
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::FailedWithoutCounterexample
                }
            ));

            let mut short = proofs();
            short.identity.ceilings.wall_clock = Duration::from_secs(1);
            let slow = StandIn::running("diag-timeout", "exec sleep 30");
            let timed_out =
                execute_kani_obligation(&slow.request(&short, Duration::from_secs(1))).unwrap();
            assert_eq!(
                timed_out.outcome,
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::TimedOut
                }
            );
            assert!(wire_has_no_diagnostics(&timed_out));
        }

        /// A refusal returns no evidence and so no record: an over-limit stream by its code, an
        /// unread one by its code.
        ///
        #[trace("FR-017-AC-32", "FR-017-AC-36", "TC-043")]
        #[test]
        fn tc_043_over_limit_and_unread_streams_stay_refused_with_no_record() {
            let flood = StandIn::running(
                "diag-flood",
                &format!("head -c {} /dev/zero\nexit 1\n", CAPTURE_LIMIT + 1),
            );
            let refusal = execute_kani_obligation(&flood.request(&proofs(), T)).unwrap_err();
            assert_eq!(refusal.code(), Some("kani_output_over_limit"));

            let members = StandIn::running(
                "diag-flood-group",
                &format!("head -c {} /dev/zero\nexit 1\n", 2 * CAPTURE_LIMIT + 1),
            );
            let group = only_group(members.batch(&pair(), T)).unwrap_err();
            assert_eq!(group.code(), Some("kani_output_over_limit"));
            assert!(matches!(
                group,
                KaniExecutionRefusal::OutputOverLimit { harnesses: 2, .. }
            ));

            let refusal = launch_evidence(
                LaunchOutcome::OutputUnread {
                    stream: CaptureStream::Stderr,
                    detail: "read failed".to_owned(),
                },
                None,
                None,
            )
            .unwrap_err();
            assert_eq!(refusal.code(), Some("kani_output_unread"));
        }

        /// The record never feeds classification: the same report and exit with other stream text
        /// classify alike, and two no-report runs differing only in their streams have equal
        /// evidence outside `diagnostics` and `memory`.
        ///
        #[trace("FR-017-AC-36", "TC-043")]
        #[test]
        fn tc_043_stream_text_never_changes_the_classification() {
            let harness = proofs();
            let exported = report("Failure", &[FAILED, COVER_OK]);
            let stand_in = printing(
                "diag-independent",
                Some(&exported),
                &block("proofs::check", "assertion", "falsify"),
                "",
                1,
            );
            let plain = run(&stand_in, &harness);
            stand_in.set_streams(
                &format!(
                    "{BANNER}{SOLVING}{}",
                    block("proofs::check", "assertion", "falsify")
                ),
                "error: could not compile `x`\nerror: Failed to execute cargo (exit status: 1)\n",
            );
            let noisy = run(&stand_in, &harness);
            assert!(matches!(plain.outcome, KaniRunOutcome::Falsified { .. }));
            assert_eq!(
                (&plain.outcome, &plain.checks, plain.success_checks),
                (&noisy.outcome, &noisy.checks, noisy.success_checks)
            );

            let silent = printing("diag-no-report", None, "first run\n", "", 1);
            let first = run(&silent, &harness);
            silent.set_streams(&format!("{BANNER}{SOLVING}"), "something else\n");
            let second = run(&silent, &harness);
            assert_ne!(first.diagnostics, second.diagnostics);

            // The one argument that may differ is the run's own report path, the last one,
            // which each run gave the launcher.
            let received = silent.calls();
            assert_eq!(received.len(), 2);
            let mut paths = Vec::new();
            let mut stripped = Vec::new();
            for (evidence, argv) in [first, second].into_iter().zip(received) {
                assert_eq!(
                    evidence.arguments, argv,
                    "the evidence names the launch's vector"
                );
                let mut evidence = evidence;
                paths.push(evidence.arguments.pop().expect("a report path"));
                evidence.diagnostics = None;
                evidence.memory = stripped.first().map_or_else(
                    || evidence.memory.clone(),
                    |first: &KaniExecutionEvidence| first.memory.clone(),
                );
                stripped.push(evidence);
            }
            assert!(paths.iter().all(|path| path.contains("quire-kani-report-")));
            assert_ne!(
                paths[0], paths[1],
                "each run exports to its own report path"
            );
            assert_eq!(stripped[0], stripped[1]);
        }

        fn three() -> Vec<StateFrameHarness> {
            ["a", "b", "c"]
                .map(|module| member(module, "check", 4))
                .to_vec()
        }

        fn group_evidence(stand_in: &StandIn) -> Vec<KaniExecutionEvidence> {
            only_group(stand_in.batch(&three(), T)).unwrap()
        }

        /// Each member of a no-report group has its own phase and the group's excerpts; a compile
        /// failure with no segment gives every member `compile`.
        ///
        #[trace("FR-017-AC-36", "TC-043")]
        #[test]
        fn tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts() {
            let stdout = format!(
                "{BANNER}Checking harness a::check...\n{SOLVING}Checking harness b::check...\n"
            );
            let stand_in = printing("diag-group", None, &stdout, "warning: x\n", 1);
            let evidence = group_evidence(&stand_in);
            assert!(evidence.iter().all(|e| e.outcome == NO_VERDICT));
            let phases: Vec<_> = evidence.iter().map(|e| diagnostics(e).phase).collect();
            assert_eq!(
                phases,
                [
                    KaniFailurePhase::Solver,
                    KaniFailurePhase::Backend,
                    KaniFailurePhase::Unknown
                ]
            );
            for member in &evidence {
                assert_eq!(diagnostics(member).stdout.text, stdout);
                assert_eq!(diagnostics(member).stderr.text, "warning: x\n");
            }

            let failed = format!("{BANNER}error: Failed to execute cargo (exit status: 101).\n");
            let evidence = group_evidence(&printing("diag-group-compile", None, &failed, "", 1));
            assert!(evidence
                .iter()
                .all(|e| e.outcome == NO_VERDICT
                    && diagnostics(e).phase == KaniFailurePhase::Compile));
        }

        /// A group whose report states success for every member beside an unsuccessful exit is
        /// `NoVerdict` with phase `unknown` for every member; one whose report holds a failure has
        /// no `NoVerdict` member and no record.
        ///
        #[trace("FR-017-AC-36", "TC-043")]
        // Linux only: these expect the witness phase, which the report-only fixture of other targets does not run.
        #[test]
        fn tc_043_a_group_report_decides_between_unknown_and_no_record() {
            let all_success = batch_report(&[
                entry("a::check", "Success", &[PASSED, COVER_OK]),
                entry("b::check", "Success", &[PASSED, COVER_OK]),
                entry("c::check", "Success", &[PASSED, COVER_OK]),
            ]);
            let stdout = format!("{BANNER}Checking harness a::check...\n{SOLVING}");
            let stand_in = printing("diag-group-success", Some(&all_success), &stdout, "", 1);
            let evidence = group_evidence(&stand_in);
            assert!(evidence.iter().all(|e| e.outcome == NO_VERDICT));
            assert!(evidence
                .iter()
                .all(|e| diagnostics(e).phase == KaniFailurePhase::Unknown));
            assert!(evidence
                .iter()
                .all(|e| diagnostics(e).stdout.text == stdout));

            let with_failure = batch_report(&[
                entry("a::check", "Success", &[PASSED, COVER_OK]),
                entry("b::check", "Success", &[PASSED, COVER_OK]),
                entry("c::check", "Failure", &[FAILED, COVER_OK]),
            ]);
            let witness = batch_report(&[entry("c::check", "Failure", &[FAILED, COVER_OK])]);
            let stand_in = StandIn::replaying_phases(
                "diag-group-failure",
                (Some(&with_failure), &stdout, 1),
                (Some(&witness), "", 1),
            );
            let evidence = group_evidence(&stand_in);
            assert!(evidence.iter().all(|e| e.outcome != NO_VERDICT));
            assert!(evidence.iter().all(wire_has_no_diagnostics));
        }

        /// Requests whose `home_directory` differs start separate processes; equal ones share
        /// one, and its members' excerpts of the shared output are byte-identical.
        ///
        #[trace("FR-017-AC-36", "TC-043")]
        #[test]
        fn tc_043_requests_with_different_home_directories_are_not_grouped() {
            let (u, v) = (PathBuf::from("/h/u"), PathBuf::from("/h/v"));
            let pair = pair();
            let processes = |first: Option<&Path>, second: Option<&Path>| {
                let stand_in = printing("diag-homes", None, &format!("{BANNER}/h/u/x\n"), "", 1);
                let requests = [
                    KaniExecutionRequest {
                        home_directory: first,
                        ..stand_in.request(&pair[0], T)
                    },
                    KaniExecutionRequest {
                        home_directory: second,
                        ..stand_in.request(&pair[1], T)
                    },
                ];
                let runs = execute_kani_obligations(&requests).unwrap();
                let evidence: Vec<_> = runs
                    .into_iter()
                    .flat_map(|run| run.evidence.unwrap())
                    .collect();
                (stand_in.calls().len(), evidence)
            };
            assert_eq!(processes(Some(&u), Some(&v)).0, 2);
            assert_eq!(processes(Some(&u), None).0, 2);
            let (count, evidence) = processes(Some(&u), Some(&u));
            assert_eq!(count, 1);
            assert_eq!(
                diagnostics(&evidence[0]).stdout,
                diagnostics(&evidence[1]).stdout
            );
            assert_eq!(
                diagnostics(&evidence[0]).stdout.text,
                format!("{BANNER}<HOME>/x\n")
            );
            assert_eq!(processes(None, None).0, 1);
        }

        /// The CBMC failure cause kept on `FailedWithoutCounterexample` evidence.
        pub(crate) mod failure_cause {
            use serde_json::{json, Value};

            use super::*;

            const NO_COUNTEREXAMPLE: KaniRunOutcome = KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample,
            };
            const CHECKING: &str = "Checking harness proofs::check...\n";
            const HARNESS: &str = "proofs::check";

            /// A run that is not `FailedWithoutCounterexample`: its label, report, console, exit
            /// code and the test of its outcome.
            type OtherOutcome<'a> = (&'a str, Vec<u8>, &'a str, i32, fn(&KaniRunOutcome) -> bool);

            /// A one-harness report: the result's `harness_id`, its status and checks, and the
            /// `error_details` member as given.
            fn document(
                harness_id: Option<&str>,
                status: &str,
                checks: &[(&str, &str)],
                details: Option<Value>,
            ) -> Vec<u8> {
                let mut document: Value = serde_json::from_slice(&report(status, checks)).unwrap();
                match harness_id {
                    Some(id) => {
                        document["verification_results"]["results"][0]["harness_id"] = json!(id)
                    }
                    None => {
                        document["verification_results"]["results"][0]
                            .as_object_mut()
                            .unwrap()
                            .remove("harness_id");
                    }
                }
                if let Some(details) = details {
                    document["error_details"] = details;
                }
                serde_json::to_vec(&document).unwrap()
            }

            fn entry_for(harness: &str, exit_status: Value) -> Value {
                json!({ "harness_id": harness, "has_errors": true, "exit_status": exit_status })
            }

            /// A failure with no checks and the one entry for the harness stating `exit_status`.
            fn failing_with(exit_status: &str) -> Vec<u8> {
                document(
                    Some(HARNESS),
                    "Failure",
                    &[],
                    Some(json!([entry_for(HARNESS, json!(exit_status))])),
                )
            }

            fn wire(evidence: &KaniExecutionEvidence) -> Value {
                serde_json::to_value(evidence).unwrap()
            }

            fn has_member(evidence: &KaniExecutionEvidence, name: &str) -> bool {
                wire(evidence).as_object().unwrap().contains_key(name)
            }

            /// The streams a CBMC failure leaves: Kani's line and CBMC's own message.
            fn cbmc_failed(name: &str, report: &[u8]) -> StandIn {
                printing(
                    name,
                    Some(report),
                    &format!("{BANNER}{CHECKING}CBMC failed with status 64\n"),
                    "Unknown option: --no-such-option\n",
                    1,
                )
            }

            /// An environment memory kill is classified from the backend's report alone: a report
            /// stating `out_of_memory` is `FailedWithoutCounterexample` with that cause, and a
            /// launcher killed by SIGKILL that exports no report is `NoVerdict`; FR-029's mapping
            /// of each is asserted in `tests/it/terminal_map.rs`. The runner measured nothing itself (FR-028-AC-25, ADR-007).
            #[trace("FR-028-AC-25", "TC-039")]
            #[test]
            fn an_environment_memory_kill_is_classified_from_the_report_only() {
                let harness = proofs();
                let stand_in = cbmc_failed("env-oom-report", &failing_with("out_of_memory"));
                let evidence = run(&stand_in, &harness);
                assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE);
                assert_eq!(
                    wire(&evidence)["failureCause"],
                    json!({ "kind": "out_of_memory" })
                );
                assert_eq!(evidence.memory.peak_resident_bytes, None);

                let killed = StandIn::running("env-oom-sigkill", "kill -9 $$");
                let evidence = run(&killed, &harness);
                assert_eq!(
                    evidence.outcome,
                    KaniRunOutcome::Inconclusive {
                        reason: KaniInconclusiveReason::NoVerdict
                    }
                );
                assert_eq!(evidence.exit_code, None);
            }

            /// A `FailedWithoutCounterexample` run keeps the cause the report states, and a
            /// cause that names a CBMC failure brings the diagnostic record.
            ///
            pub(crate) fn single_failure_reports(
                mut observe: impl FnMut(&str, &KaniExecutionEvidence),
            ) {
                let stand_in = cbmc_failed("cause-vocabulary", &failing_with("exit_code_64"));
                let harness = proofs();
                let evidence = run(&stand_in, &harness);
                assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE);
                assert_eq!(
                    wire(&evidence)["failureCause"],
                    json!({ "kind": "exit_code", "code": 64 })
                );
                observe("exit_code_64", &evidence);
                let record = diagnostics(&evidence);
                assert_eq!(record.phase, KaniFailurePhase::Unknown);
                assert!(record
                    .stderr
                    .text
                    .contains("Unknown option: --no-such-option"));
                assert!(record.stdout.text.contains("CBMC failed with status 64"));

                let crate_path = format!("{}/x", stand_in.crate_directory.display());
                let long = "a".repeat(100);
                let straddling = format!("{}\u{1f600}tail", "a".repeat(62));
                let cases: Vec<(String, Value)> = vec![
                    ("exit_code_15".into(), json!({ "kind": "exit_code", "code": 15 })),
                    ("exit_code_0".into(), json!({ "kind": "exit_code", "code": 0 })),
                    ("exit_code_-1".into(), json!({ "kind": "exit_code", "code": -1 })),
                    (
                        "exit_code_2147483647".into(),
                        json!({ "kind": "exit_code", "code": 2_147_483_647 }),
                    ),
                    (
                        "exit_code_-2147483648".into(),
                        json!({ "kind": "exit_code", "code": -2_147_483_648_i64 }),
                    ),
                    ("out_of_memory".into(), json!({ "kind": "out_of_memory" })),
                    ("timeout".into(), json!({ "kind": "timeout" })),
                    (
                        "cbmc_exploded".into(),
                        json!({ "kind": "unknown", "exitStatus": "cbmc_exploded" }),
                    ),
                    ("a".repeat(64), json!({ "kind": "unknown", "exitStatus": "a".repeat(64) })),
                    ("a".repeat(65), json!({ "kind": "unknown", "exitStatus": "a".repeat(64) })),
                    (long, json!({ "kind": "unknown", "exitStatus": "a".repeat(64) })),
                    (straddling, json!({ "kind": "unknown", "exitStatus": "a".repeat(62) })),
                    (crate_path, json!({ "kind": "unknown", "exitStatus": "<CRATE_DIR>/x" })),
                    (
                        "x\u{1b}[31mred".into(),
                        json!({ "kind": "unknown", "exitStatus": "xred" }),
                    ),
                    (
                        "killed pid 4242 after 3.5s".into(),
                        json!({ "kind": "unknown", "exitStatus": "killed pid <PID> after <DURATION>" }),
                    ),
                ]
                .into_iter()
                .chain(
                    [
                        "exit_code_",
                        "exit_code_064",
                        "exit_code_+64",
                        "exit_code_-0",
                        "exit_code_2147483648",
                        "exit_code_64x",
                        "EXIT_CODE_64",
                    ]
                    .map(|status| (status.to_owned(), json!({ "kind": "unknown", "exitStatus": status }))),
                )
                .collect();
                for (status, expected) in cases {
                    fs::write(
                        stand_in.directory.join("report.json"),
                        failing_with(&status),
                    )
                    .unwrap();
                    let evidence = run(&stand_in, &harness);
                    assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE, "{status}");
                    assert_eq!(wire(&evidence)["failureCause"], expected, "{status}");
                    assert!(evidence.diagnostics.is_some(), "{status}");
                    observe(&status, &evidence);
                }
                for (label, details) in [
                    (
                        "properties_failed",
                        Some(json!([entry_for(HARNESS, json!("properties_failed"))])),
                    ),
                    ("absent", None),
                    ("null", Some(json!([entry_for(HARNESS, Value::Null)]))),
                ] {
                    fs::write(
                        stand_in.directory.join("report.json"),
                        document(Some(HARNESS), "Failure", &[], details),
                    )
                    .unwrap();
                    let evidence = run(&stand_in, &harness);
                    assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE, "{label}");
                    if label == "properties_failed" {
                        assert_eq!(
                            evidence.failure_cause,
                            Some(KaniFailureCause::PropertiesFailed)
                        );
                    } else {
                        assert_eq!(evidence.failure_cause, None);
                    }
                    assert!(evidence.diagnostics.is_none());
                    observe(label, &evidence);
                }
            }

            /// A failed property keeps its cause and no record; a report naming no single entry
            /// for the harness keeps neither.
            ///
            pub(crate) fn unattributed_failure_reports(
                mut observe: impl FnMut(&str, &KaniExecutionEvidence),
            ) {
                let harness = proofs();
                let failing = |details: Option<Value>, id: Option<&str>| {
                    document(id, "Failure", &[FAILED], details)
                };
                let stand_in = printing("cause-property", Some(&[]), CHECKING, "", 1);
                let run_with = |report: Vec<u8>| {
                    fs::write(stand_in.directory.join("report.json"), report).unwrap();
                    run(&stand_in, &harness)
                };
                let evidence = run_with(failing(
                    Some(json!([entry_for(HARNESS, json!("properties_failed"))])),
                    Some(HARNESS),
                ));
                assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE);
                assert_eq!(
                    wire(&evidence)["failureCause"],
                    json!({ "kind": "properties_failed" })
                );
                assert!(!has_member(&evidence, "diagnostics"));

                observe("properties_failed", &evidence);
                let exit_64 = entry_for(HARNESS, json!("exit_code_64"));
                let silent: [(&str, Vec<u8>); 5] = [
                    ("no error_details", failing(None, Some(HARNESS))),
                    (
                        "another harness only",
                        failing(
                            Some(json!([entry_for("other::check", json!("timeout"))])),
                            Some(HARNESS),
                        ),
                    ),
                    (
                        "two entries",
                        failing(Some(json!([exit_64.clone(), exit_64])), Some(HARNESS)),
                    ),
                    (
                        "null exit_status",
                        failing(
                            Some(json!([entry_for(HARNESS, Value::Null)])),
                            Some(HARNESS),
                        ),
                    ),
                    (
                        "no harness_id",
                        failing(
                            Some(json!([entry_for(HARNESS, json!("exit_code_64"))])),
                            None,
                        ),
                    ),
                ];
                for (label, report) in silent {
                    let evidence = run_with(report);
                    assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE, "{label}");
                    assert!(!has_member(&evidence, "failureCause"), "{label}");
                    assert!(!has_member(&evidence, "diagnostics"), "{label}");
                    observe(label, &evidence);
                }
            }

            /// An entry with `exit_code_64` beside any other outcome leaves that outcome as it
            /// was and adds no cause: the cause is read only for `FailedWithoutCounterexample`.
            ///
            #[trace("FR-017-AC-38", "TC-027")]
            #[test]
            fn tc_027_the_cause_is_kept_for_no_other_outcome() {
                let harness = proofs();
                let with_entry = |status: &str, checks: &[(&str, &str)]| {
                    document(
                        Some(HARNESS),
                        status,
                        checks,
                        Some(json!([entry_for(HARNESS, json!("exit_code_64"))])),
                    )
                };
                let falsifying = block(HARNESS, "assertion", "falsify");
                let cases: [OtherOutcome<'_>; 6] = [
                    (
                        "verified",
                        with_entry("Success", &[PASSED, COVER_OK]),
                        "",
                        0,
                        |outcome| *outcome == KaniRunOutcome::Verified,
                    ),
                    (
                        "cover-unsatisfied",
                        with_entry("Success", &[PASSED, COVER_NO]),
                        "",
                        0,
                        |outcome| matches!(outcome, KaniRunOutcome::CoverUnsatisfied { .. }),
                    ),
                    (
                        "falsified",
                        with_entry("Failure", &[FAILED, COVER_OK]),
                        &falsifying,
                        1,
                        |outcome| matches!(outcome, KaniRunOutcome::Falsified { .. }),
                    ),
                    (
                        "unwind bound exhausted",
                        with_entry("Failure", &[("Failure", "unwind")]),
                        "",
                        1,
                        |outcome| {
                            *outcome
                                == KaniRunOutcome::Inconclusive {
                                    reason: KaniInconclusiveReason::UnwindBoundExhausted,
                                }
                        },
                    ),
                    (
                        "no verdict: success beside exit 1",
                        with_entry("Success", &[PASSED, COVER_OK]),
                        "",
                        1,
                        |outcome| *outcome == NO_VERDICT,
                    ),
                    ("no verdict: no report", Vec::new(), "", 1, |outcome| {
                        *outcome == NO_VERDICT
                    }),
                ];
                for (label, report, console, exit, expected) in cases {
                    let exported = (!report.is_empty()).then_some(report.as_slice());
                    let stand_in = printing("cause-other-outcome", exported, console, "", exit);
                    let evidence = run(&stand_in, &harness);
                    assert!(
                        expected(&evidence.outcome),
                        "{label}: {:?}",
                        evidence.outcome
                    );
                    assert!(!has_member(&evidence, "failureCause"), "{label}");
                }

                // A run stopped by the timeout reads no report.
                let stand_in = StandIn::scaffold("cause-stopped");
                let reported = finish_single(
                    &stand_in.request(&harness, T),
                    Vec::new(),
                    LaunchOutcome::TimedOut,
                    Some(failing_with("exit_code_64")),
                    None,
                )
                .unwrap();
                assert_eq!(
                    reported.outcome,
                    KaniRunOutcome::Inconclusive {
                        reason: KaniInconclusiveReason::TimedOut
                    }
                );
                assert_eq!((reported.failure_cause, reported.diagnostics), (None, None));
            }

            /// A number and a lone surrogate escape as the exit status are still refused as
            /// malformed, with no evidence; a string no vocabulary names never is.
            ///
            #[trace("FR-017-AC-38", "TC-027")]
            #[test]
            fn tc_027_a_malformed_exit_status_still_refuses_the_report() {
                let harness = proofs();
                let stand_in = printing("cause-malformed", Some(&[]), CHECKING, "", 1);
                for literal in ["64", r#""\ud800""#] {
                    let text = String::from_utf8(failing_with("timeout")).unwrap();
                    let malformed = text.replace(r#""timeout""#, literal);
                    assert_ne!(text, malformed);
                    fs::write(stand_in.directory.join("report.json"), malformed).unwrap();
                    let refusal =
                        execute_kani_obligation(&stand_in.request(&harness, T)).unwrap_err();
                    assert!(
                        matches!(
                            refusal,
                            KaniExecutionRefusal::Report(KaniReportRefusal::Malformed { .. })
                        ),
                        "{literal}: {refusal:?}"
                    );
                }
            }

            /// A batch member keeps the cause beside the batch timeout rule.
            ///
            pub(crate) fn batch_failure_reports(mut observe: impl FnMut(&[KaniExecutionEvidence])) {
                let members = ["a", "b", "c", "d", "e"].map(|module| member(module, "check", 4));
                let entry_with = |id: &'static str, status, checks, exit_status| BatchEntry {
                    harness_id: id,
                    status,
                    checks,
                    exit_status,
                };
                let report = batch_report(&[
                    entry_with("a::check", "Success", &[PASSED, COVER_OK], None),
                    entry_with("b::check", "Failure", &[], Some("exit_code_64")),
                    entry_with("c::check", "Failure", &[], Some("timeout")),
                    entry_with("d::check", "Failure", &[FAILED], Some("timeout")),
                    entry_with("e::check", "Failure", &[FAILED], Some("properties_failed")),
                ]);
                let stdout = format!("{BANNER}CBMC failed with status 64\n");
                // The failed members d and e are held for a witness group that reports its own two
                // entries.
                let witness = batch_report(&[
                    entry_with("d::check", "Failure", &[FAILED], Some("timeout")),
                    entry_with("e::check", "Failure", &[FAILED], Some("properties_failed")),
                ]);
                let stand_in = StandIn::scaffold("cause-batch");
                let proof_body = print_body(&stand_in, Some(&report), 1);
                let exported = stand_in.directory.join("witness-report.json");
                fs::write(&exported, &witness).unwrap();
                let witness_body = print_body(&stand_in, None, 1).replacen(
                    "cat",
                    &format!("cp '{}' \"$last\"\ncat", exported.display()),
                    1,
                );
                stand_in.install(&format!(
                    "if [ \"$witness\" = yes ]; then\n{witness_body}\nelse\n{proof_body}\nfi\n"
                ));
                stand_in.set_streams(&stdout, "Unknown option\n");
                let evidence = only_group(stand_in.batch(&members, T)).unwrap();
                observe(&evidence);
                let causes: Vec<_> = evidence
                    .iter()
                    .map(|member| wire(member).get("failureCause").cloned())
                    .collect();
                assert_eq!(
                    causes,
                    [
                        None,
                        Some(json!({ "kind": "exit_code", "code": 64 })),
                        None,
                        Some(json!({ "kind": "timeout" })),
                        Some(json!({ "kind": "properties_failed" })),
                    ]
                );
                assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
                assert_eq!(evidence[1].outcome, NO_COUNTEREXAMPLE);
                assert_eq!(
                    evidence[2].outcome,
                    KaniRunOutcome::Inconclusive {
                        reason: KaniInconclusiveReason::TimedOut
                    }
                );
                assert_eq!(evidence[3].outcome, NO_COUNTEREXAMPLE);
                assert_eq!(evidence[4].outcome, NO_COUNTEREXAMPLE);
                assert_eq!(evidence[2].batch.as_ref().unwrap().timeout_seconds, 30);
                for index in [1, 3] {
                    let record = diagnostics(&evidence[index]);
                    assert_eq!(record.phase, KaniFailurePhase::Unknown);
                    assert_eq!(record.stdout.text, stdout);
                    assert_eq!(record.stderr.text, "Unknown option\n");
                }
                for index in [0, 2, 4] {
                    assert!(
                        !has_member(&evidence[index], "diagnostics"),
                        "member {index}"
                    );
                }
            }

            /// Two entries for `b::check`, one `timeout` and one `exit_code_64`: the timeout rule
            /// reads the entries as before and the member is timed out with no cause.
            ///
            #[trace("FR-017-AC-39", "TC-043")]
            #[test]
            fn tc_043_two_entries_for_a_member_leave_the_timeout_rule_unchanged() {
                let members = ["a", "b"].map(|module| member(module, "check", 4));
                let mut document: Value = serde_json::from_slice(&batch_report(&[
                    BatchEntry {
                        harness_id: "a::check",
                        status: "Success",
                        checks: &[PASSED, COVER_OK],
                        exit_status: None,
                    },
                    BatchEntry {
                        harness_id: "b::check",
                        status: "Failure",
                        checks: &[],
                        exit_status: Some("timeout"),
                    },
                ]))
                .unwrap();
                document["error_details"]
                    .as_array_mut()
                    .unwrap()
                    .push(entry_for("b::check", json!("exit_code_64")));
                let report = serde_json::to_vec(&document).unwrap();
                let stand_in = printing("cause-batch-two", Some(&report), BANNER, "", 1);
                let evidence = only_group(stand_in.batch(&members, T)).unwrap();
                assert_eq!(
                    evidence[1].outcome,
                    KaniRunOutcome::Inconclusive {
                        reason: KaniInconclusiveReason::TimedOut
                    }
                );
                assert!(!has_member(&evidence[1], "failureCause"));
                assert!(!has_member(&evidence[1], "diagnostics"));
            }

            /// Three runs with the same failure report and different CBMC causes and streams
            /// classify alike and have equal evidence outside the cause, the record, the memory
            /// observation and the report path.
            ///
            pub(crate) fn cause_independent_classification(
                mut observe: impl FnMut(&str, &KaniExecutionEvidence),
            ) {
                let harness = proofs();
                let mut stripped = Vec::new();
                let stand_in = printing("cause-independent", Some(&[]), "", "", 1);
                for (index, (status, stdout)) in [
                    ("exit_code_64", "CBMC failed with status 64\n"),
                    ("out_of_memory", "Killed\n"),
                    ("cbmc_exploded", "something else entirely\n"),
                ]
                .into_iter()
                .enumerate()
                {
                    fs::write(stand_in.directory.join("report.json"), failing_with(status))
                        .unwrap();
                    stand_in.set_streams(stdout, &format!("stderr {index}\n"));
                    let mut evidence = run(&stand_in, &harness);
                    assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE, "{status}");
                    assert!(evidence.failure_cause.is_some() && evidence.diagnostics.is_some());
                    observe(status, &evidence);
                    evidence.failure_cause = None;
                    evidence.diagnostics = None;
                    evidence.arguments.pop().expect("a report path");
                    evidence.memory = stripped.first().map_or_else(
                        || evidence.memory.clone(),
                        |first: &KaniExecutionEvidence| first.memory.clone(),
                    );
                    stripped.push(evidence);
                }
                assert_eq!(stripped[0], stripped[1]);
                assert_eq!(stripped[1], stripped[2]);
            }
        }
    }
    #[trace("FR-017-AC-52", "TC-027")]
    #[test]
    fn sealed_scalar_launch_selects_the_library_without_changing_proof_options() {
        let (harness, _) =
            crate::kani::test_support::scalar_source_fixture(Duration::from_secs(600));
        let cancel = quire_exact::Cancel::new();
        let installation = KaniInstallation::new("cargo-kani");
        let request = KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: KaniExecutableHarness::Scalar(&harness),
            crate_directory: Path::new("/crate"),
            target_directory: Path::new("/target"),
            home_directory: None,
        };
        let (arguments, _) = launch_command(
            &request,
            Phase::Proof,
            Path::new("/target/report.json"),
            true,
        );
        let options = request.harness.view().options;
        assert_eq!(&arguments[1..=options.len()], options);
        assert_eq!(arguments[options.len() + 1], "--lib");
        assert_eq!(
            &arguments[options.len() + 2..],
            [
                "-Z",
                "unstable-options",
                "--export-json",
                "/target/report.json"
            ]
        );
    }
}
