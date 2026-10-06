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

use std::{
    fmt, fs,
    num::NonZeroUsize,
    path::Path,
    time::{Duration, Instant},
};

use serde::Serialize;

use crate::kani::{
    abi::KaniSolver,
    classify::{
        classify_kani_run, classify_member, ClassifiedRun, KaniInconclusiveReason, KaniRunOutcome,
    },
    identity::ObligationKind,
    output::{
        playback::{counterexample_playback, playback_blocks},
        report::{
            members_in_request_order, KaniCheckResult, KaniHarnessReport, KaniHarnessStatus,
            KaniReportRefusal,
        },
    },
    run::{
        harness::{HarnessView, KaniExecutableHarness},
        launch::{
            run_bounded_launcher, BoundedLaunch, BoundedLaunchError, CaptureStream, LaunchOutcome,
        },
        memory::MemoryObservation,
        report_file::{fresh_report_path, remove_stale_report},
        tool::{KaniInstallation, KaniTool, KaniToolError},
    },
};

/// The stable code of a run refused because a stream carried more than its limit.
pub const OUTPUT_OVER_LIMIT_CODE: &str = "kani_output_over_limit";

/// The stable code of a run refused because a stream could not be read to its end.
pub const OUTPUT_UNREAD_CODE: &str = "kani_output_unread";

/// One execution of one harness in a crate the caller wrote.
pub struct KaniExecutionRequest<'a> {
    /// Backend to invoke.
    pub installation: &'a KaniInstallation,
    /// Explicit path to this package's helper built with this actual library artifact.
    ///
    /// Consumers deliberately build the dependency binary from their own manifest with matching
    /// target/profile/features/flags. Cargo dependency compilation alone does not deliver it.
    /// Select both consumer and dependency packages and both binaries in that build; forward
    /// dependency features through the consumer's declared features. A build from this package's
    /// own manifest can compile a different library artifact and is refused.
    pub guardian_path: &'a Path,
    /// The generated harness.
    pub harness: KaniExecutableHarness<'a>,
    /// Crate root whose `src/lib.rs` contains the harness source byte-for-byte.
    pub crate_directory: &'a Path,
    /// Cargo target directory for the run.
    pub target_directory: &'a Path,
}

/// Why a harness was not run.
#[derive(Debug)]
pub enum KaniExecutionRefusal {
    /// The installed backend could not be located or started.
    Tool(KaniToolError),
    /// Guardian setup, authenticated protocol or confirmed cleanup failed. No report is classified.
    Guardian {
        /// Stable typed failure kind.
        kind: super::launch::GuardianFailureKind,
        /// Diagnostic context; this text does not select the failure kind.
        detail: String,
    },
    /// No memory-enforcement mechanism is available; the backend was never spawned.
    MemoryMechanismUnavailable {
        /// The failed mechanism check.
        cause: std::io::Error,
    },
    /// Memory observation failed during a launch, so the backend tree was killed.
    MemoryObservationFailed {
        /// The observation failure.
        detail: String,
    },
    /// A batch exceeded its one aggregate memory ceiling. No member was classified.
    BatchMemoryExhausted {
        /// Members whose backend tree was killed.
        members: usize,
        /// The memory ceiling in bytes.
        memory_bytes: u64,
        /// Actual mechanism and its observations.
        memory: MemoryObservation,
    },
    /// The crate's `src/lib.rs` does not contain the harness source.
    HarnessNotInCrate {
        /// The generated artifact path.
        harness_path: String,
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
}

impl KaniExecutionRefusal {
    /// The stable machine-readable code of a refusal that has one: [`OUTPUT_OVER_LIMIT_CODE`] and
    /// [`OUTPUT_UNREAD_CODE`]. The other refusals are told apart by their variant.
    #[must_use]
    pub const fn code(&self) -> Option<&'static str> {
        match self {
            Self::OutputOverLimit { .. } => Some(OUTPUT_OVER_LIMIT_CODE),
            Self::OutputUnread { .. } => Some(OUTPUT_UNREAD_CODE),
            Self::Tool(_)
            | Self::Guardian { .. }
            | Self::HarnessNotInCrate { .. }
            | Self::Report(_)
            | Self::BatchTimedOut { .. }
            | Self::MemoryMechanismUnavailable { .. }
            | Self::MemoryObservationFailed { .. }
            | Self::BatchMemoryExhausted { .. }
            | Self::PlaybackForNonMember { .. } => None,
        }
    }
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(error) => write!(formatter, "{error}"),
            Self::Guardian { kind, detail } => write!(formatter, "guardian {kind:?}: {detail}"),
            Self::MemoryMechanismUnavailable { cause } => write!(
                formatter,
                "backend tree memory enforcement is unavailable: {cause}"
            ),
            Self::MemoryObservationFailed { detail } => write!(
                formatter,
                "backend tree memory observation failed: {detail}"
            ),
            Self::BatchMemoryExhausted {
                members,
                memory_bytes,
                ..
            } => write!(
                formatter,
                "the backend tree of {members} harnesses exceeded {memory_bytes} resident bytes"
            ),
            Self::HarnessNotInCrate { harness_path } => {
                write!(formatter, "the crate does not contain {harness_path}")
            }
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
    /// The identity ceilings actually enforced by this run.
    pub ceilings: crate::kani::identity::ProofCeilings,
    /// Actual backend-tree memory mechanism and observed peak.
    pub memory: MemoryObservation,
    /// Hard report-content cap, separately enforced from the identity's whole-run memory ceiling.
    pub report_cap_bytes: u64,
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
    /// plus, for a precondition harness, whose one property is its cover, the satisfied covers.
    /// Zero when the run produced no report. It is the SUCCESS-check count FR-017 defines, which
    /// the terminal maps (FR-029, FR-030) take as an explicit input.
    pub success_checks: u32,
    /// Every check Kani reported, with its class, source location and status, in report order.
    /// Empty when the run produced no report. A consumer attributes proof to source with it.
    pub checks: Vec<KaniCheckResult>,
    /// Present exactly when the harness ran in a process shared with other harnesses: then
    /// `arguments` is that process's complete argument vector, not the single-run one, and this
    /// says so and names who shared it. Absent for a run of one harness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch: Option<KaniBatchInvocation>,
}

/// A backend report and its invocation metadata, without an attestation of resource enforcement.
/// Production attaches its actual observation only after the bounded launcher concludes.
#[derive(Debug)]
struct ReportedExecution {
    ceilings: crate::kani::identity::ProofCeilings,
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
}

impl ReportedExecution {
    fn with_memory(self, memory: MemoryObservation) -> KaniExecutionEvidence {
        KaniExecutionEvidence {
            memory,
            report_cap_bytes: super::REPORT_CONTENT_BYTES,
            ceilings: self.ceilings,
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
/// Bounded execution requires Linux with readable procfs task-child/RSS information and pidfd
/// APIs, plus `bwrap` supporting user/PID namespaces, PID-1 command mode, new sessions, gated
/// startup and info descriptors. Namespace creation permission is required. The explicitly
/// configured matched guardian stays INIT until lease closure; its death tears down descendants.
/// Missing resource support refuses before backend Dispatch. Guardian setup/protocol/cleanup
/// failure returns [`KaniExecutionRefusal::Guardian`] and never classifies a report.
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let deadline = Instant::now().checked_add(request.harness.view().ceilings.wall_clock);
    let stdin = super::stdin::OriginalStdin::capture_original()
        .map_err(|cause| KaniExecutionRefusal::MemoryMechanismUnavailable { cause })?;
    require_in_crate(request)?;
    run_single(request, &stdin, deadline)
}

/// Refuses a request whose crate's library source does not contain its harness's generated
/// source byte for byte.
fn require_in_crate(request: &KaniExecutionRequest<'_>) -> Result<(), KaniExecutionRefusal> {
    let harness = request.harness.view();
    let library_path = request.crate_directory.join("src").join("lib.rs");
    let library = read_file(KaniTool::Library, &library_path).map_err(|_| {
        KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        }
    })?;
    if !String::from_utf8_lossy(&library).contains(&harness.rust.contents) {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        });
    }
    Ok(())
}

/// Starts `command` with the launcher's output captured for `harnesses` harnesses, and maps a
/// failure to start it to the refusal that names the launcher.
fn start(
    request: &KaniExecutionRequest<'_>,
    stdin: &super::stdin::OriginalStdin,
    command: super::namespace::BackendCommand,
    timeout: Duration,
    harnesses: NonZeroUsize,
    report_path: &Path,
    deadline: Option<Instant>,
) -> Result<BoundedLaunch, KaniExecutionRefusal> {
    if !deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        // The namespace helper's successful spawn cannot establish backend executability.
        request.installation.require_executable()?;
    }
    let mut ceilings = request.harness.view().ceilings;
    ceilings.wall_clock = timeout;
    run_bounded_launcher(
        command,
        request.guardian_path,
        stdin,
        report_path,
        ceilings,
        harnesses,
        deadline,
    )
    .map_err(|error| match error {
        BoundedLaunchError::Unavailable(cause) => {
            KaniExecutionRefusal::MemoryMechanismUnavailable { cause }
        }
        BoundedLaunchError::Io(error) => KaniExecutionRefusal::Tool(KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        }),
        BoundedLaunchError::Guardian { kind, detail } => {
            KaniExecutionRefusal::Guardian { kind, detail }
        }
    })
}

/// Reads the report `launch` exported to `report_path` when it completed, then removes that file
/// whatever the read found: only this run's own file is removed.
#[cfg(all(test, not(target_os = "linux")))]
fn take_report(
    launch: &LaunchOutcome,
    report_path: &Path,
) -> Result<Option<Vec<u8>>, KaniReportRefusal> {
    let report = match launch {
        LaunchOutcome::Completed { .. } => super::report_file::read_report(report_path, None),
        LaunchOutcome::TimedOut
        | LaunchOutcome::MemoryExhausted
        | LaunchOutcome::MemoryUnobserved { .. }
        | LaunchOutcome::OutputOverLimit { .. }
        | LaunchOutcome::OutputUnread { .. } => Ok(None),
    };
    let _ = fs::remove_file(report_path);
    report
}

/// A surviving caller removes only its assigned report on every exit, including startup refusal.
struct ReportCleanup<'a>(&'a Path);

impl Drop for ReportCleanup<'_> {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.0);
    }
}

/// The evidence of one harness from its `run` of a process that exited with `exit_code`.
fn evidence_of(
    request: &KaniExecutionRequest<'_>,
    harness: &HarnessView<'_>,
    arguments: Vec<String>,
    exit_code: Option<i32>,
    run: ClassifiedRun,
    batch: Option<KaniBatchInvocation>,
) -> ReportedExecution {
    ReportedExecution {
        ceilings: harness.ceilings,
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
    }
}

/// The run of one harness whose presence in the crate has been checked.
fn run_single(
    request: &KaniExecutionRequest<'_>,
    stdin: &super::stdin::OriginalStdin,
    deadline: Option<Instant>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let report_path = fresh_report_path(request.target_directory);
    remove_stale_report(&report_path)?;
    let _cleanup = ReportCleanup(&report_path);
    let (arguments, command) = launch_command(request, &report_path);
    let launch = start(
        request,
        stdin,
        command,
        request.harness.view().ceilings.wall_clock,
        NonZeroUsize::MIN,
        &report_path,
        deadline,
    )?;
    finish_single(request, arguments, launch.outcome, launch.report?)
        .map(|reported| reported.with_memory(launch.memory))
}

/// Classify one captured launch and its own report, without asserting memory enforcement.
fn finish_single(
    request: &KaniExecutionRequest<'_>,
    arguments: Vec<String>,
    outcome: LaunchOutcome,
    report: Option<Vec<u8>>,
) -> Result<ReportedExecution, KaniExecutionRefusal> {
    let harness = request.harness.view();
    let (run, exit_code) = launch_evidence(outcome, report.as_deref(), harness.kind)?;
    Ok(evidence_of(
        request, &harness, arguments, exit_code, run, None,
    ))
}

/// What one launcher process (one group) of a batch yielded: the harnesses it ran and their evidence or the
/// refusal of the whole process.
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
/// one target directory, so those must be equal as well. A group of one is the single run of
/// [`execute_kani_obligation`], with its argument vector. A larger group is one process given one
/// `--harness <module::harness> --exact` pair per member in request order, then
/// `--harness-timeout <T>` in whole seconds rounded up (omitted when Kani 0.68 would refuse the
/// value), then the shared options. The process is bounded to the member count times T, a product
/// that does not fit never elapsing.
///
/// A harness the crate does not contain refuses the whole call before any process starts. After
/// that each process has its own result, in order of its first member: its report is split by the
/// `module::harness` path of each entry, each member is classified from its own entry and its own
/// console playback, and a report that lacks, repeats or adds a harness, a playback that cannot be
/// attributed, an output stream over 8 MiB per member and an outer bound that elapsed each refuse
/// that process (group) and classify none of its members. The generator does not split or retry a
/// refused group.
///
/// The same Linux, procfs/pidfd, bubblewrap and namespace-permission prerequisites as
/// [`execute_kani_obligation`] apply to every group; unsupported setup refuses before dispatch.
/// Its documented caller-death startup limitation (IR-639) applies to batches too.
pub fn execute_kani_obligations(
    requests: &[KaniExecutionRequest<'_>],
) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
    if requests.is_empty() {
        return Ok(Vec::new());
    }
    let preparation_started = Instant::now();
    let stdin = super::stdin::OriginalStdin::capture_original()
        .map_err(|cause| KaniExecutionRefusal::MemoryMechanismUnavailable { cause })?;
    for request in requests {
        require_in_crate(request)?;
    }
    let groups = plan_groups(requests);
    // Each original per-group budget includes the shared pre-spawn snapshot/validation/planning.
    // Waiting for another group is not that group's execution or a fresh setup allocation.
    let preparation = preparation_started.elapsed();
    Ok(groups
        .into_iter()
        .map(|group| {
            let started = Instant::now();
            KaniGroupRun {
                members: group.iter().map(|(position, _)| *position).collect(),
                evidence: match group.as_slice() {
                    [(_, only)] => run_single(
                        only,
                        &stdin,
                        started.checked_add(
                            only.harness
                                .view()
                                .ceilings
                                .wall_clock
                                .saturating_sub(preparation),
                        ),
                    )
                    .map(|evidence| vec![evidence]),
                    _ => run_group(&group, &stdin, preparation, started),
                },
            }
        })
        .collect())
}

/// Stable groups in first-member order, each retaining request order.
fn plan_groups<'a, 'b>(
    requests: &'a [KaniExecutionRequest<'b>],
) -> Vec<Vec<(usize, &'a KaniExecutionRequest<'b>)>> {
    let mut groups: Vec<Vec<(usize, &KaniExecutionRequest<'_>)>> = Vec::new();
    for member in requests.iter().enumerate() {
        match groups.iter_mut().find(|group| {
            group
                .first()
                .is_some_and(|(_, first)| shares_process(first, member.1))
        }) {
            Some(group) => group.push(member),
            None => groups.push(vec![member]),
        }
    }
    groups
}

/// Whether two requests can be run by one launcher process: the same option vector once the
/// harness selection is removed, the same identity ceilings, and the same launcher, crate and target
/// directory.
fn shares_process(first: &KaniExecutionRequest<'_>, other: &KaniExecutionRequest<'_>) -> bool {
    first.harness.view().ceilings == other.harness.view().ceilings
        && first.installation.launcher == other.installation.launcher
        && first.guardian_path == other.guardian_path
        && first.crate_directory == other.crate_directory
        && first.target_directory == other.target_directory
        && without_selection(first.harness.view().options)
            == without_selection(other.harness.view().options)
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

/// The argument vector and [`super::namespace::BackendCommand`] recipe for one process that
/// runs every harness of `selections`.
fn batch_launch_command(
    first: &KaniExecutionRequest<'_>,
    selections: &[String],
    report_path: &Path,
) -> (Vec<String>, super::namespace::BackendCommand) {
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
    arguments.extend(without_selection(first.harness.view().options));
    arguments.extend([
        "-Z".to_owned(),
        "unstable-options".to_owned(),
        "--export-json".to_owned(),
    ]);
    let mut command = super::namespace::BackendCommand::new(&first.installation.launcher);
    command
        .args(&arguments)
        .arg(report_path)
        .env("CARGO_TARGET_DIR", first.target_directory)
        .current_dir(first.crate_directory);
    arguments.push(report_path.display().to_string());
    (arguments, command)
}

/// The run of a group of more than one harness, in one process.
fn run_group(
    group: &[(usize, &KaniExecutionRequest<'_>)],
    stdin: &super::stdin::OriginalStdin,
    preparation: Duration,
    started: Instant,
) -> Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal> {
    let (Some((_, first)), Some(count)) = (group.first(), NonZeroUsize::new(group.len())) else {
        return Ok(Vec::new());
    };
    let timeout = outer_bound(first.harness.view().ceilings.wall_clock, group.len());
    let deadline = started.checked_add(timeout.saturating_sub(preparation));
    let views: Vec<HarnessView<'_>> = group
        .iter()
        .map(|(_, request)| request.harness.view())
        .collect();
    let selections: Vec<String> = views.iter().map(|view| view.selection.clone()).collect();
    let report_path = fresh_report_path(first.target_directory);
    remove_stale_report(&report_path)?;
    let _cleanup = ReportCleanup(&report_path);
    let (arguments, command) = batch_launch_command(first, &selections, &report_path);
    let launch = start(
        first,
        stdin,
        command,
        timeout,
        count,
        &report_path,
        deadline,
    )?;
    let report = launch.report;
    let (exited_successfully, exit_code, text) = match settle(launch.outcome)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        } => (exited_successfully, exit_code, text),
        Concluded::MemoryExhausted => {
            return Err(KaniExecutionRefusal::BatchMemoryExhausted {
                members: group.len(),
                memory_bytes: first.harness.view().ceilings.memory_bytes.get(),
                memory: launch.memory,
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
        &text,
        report?,
    )
    .map(|reported| {
        reported
            .into_iter()
            .map(|run| run.with_memory(launch.memory.clone()))
            .collect()
    })
}

/// Interpret only a completed group's report and console; no resource-enforcement claim.
fn finish_group(
    group: &[(usize, &KaniExecutionRequest<'_>)],
    arguments: Vec<String>,
    exited_successfully: bool,
    exit_code: Option<i32>,
    text: &str,
    report: Option<Vec<u8>>,
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
    let evidence = |run: Vec<ClassifiedRun>| -> Vec<ReportedExecution> {
        group
            .iter()
            .zip(&views)
            .zip(run)
            .map(|(((_, request), view), run)| {
                evidence_of(
                    request,
                    view,
                    arguments.clone(),
                    exit_code,
                    run,
                    Some(invocation.clone()),
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
                    .map(|_| ClassifiedRun {
                        outcome: KaniRunOutcome::Inconclusive {
                            reason: KaniInconclusiveReason::NoVerdict,
                        },
                        success_checks: 0,
                        checks: Vec::new(),
                    })
                    .collect(),
            ))
        };
    };
    let members = members_in_request_order(&selections, &KaniHarnessReport::parse_batch(&report)?)?;
    let blocks = playback_blocks(text);
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
                classify_member(
                    process_succeeded,
                    member,
                    || counterexample_playback(text, Some(&view.selection)),
                    view.kind,
                )
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
        text: String,
    },
    /// The budget elapsed and the group was killed.
    TimedOut,
    MemoryExhausted,
}

/// The launch as a conclusion, or the refusal of a launch whose output cannot be vouched for: a
/// stream over its limit, or one that was not read.
fn settle(launch: LaunchOutcome) -> Result<Concluded, KaniExecutionRefusal> {
    match launch {
        LaunchOutcome::Completed {
            exited_successfully,
            exit_code,
            text,
        } => Ok(Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        }),
        LaunchOutcome::TimedOut => Ok(Concluded::TimedOut),
        LaunchOutcome::MemoryExhausted => Ok(Concluded::MemoryExhausted),
        LaunchOutcome::MemoryUnobserved { detail } => {
            Err(KaniExecutionRefusal::MemoryObservationFailed { detail })
        }
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
fn kani_launch_command(
    request: &KaniExecutionRequest<'_>,
) -> (Vec<String>, super::namespace::BackendCommand) {
    launch_command(request, &fresh_report_path(request.target_directory))
}

fn launch_command(
    request: &KaniExecutionRequest<'_>,
    report_path: &Path,
) -> (Vec<String>, super::namespace::BackendCommand) {
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(request.harness.view().options.iter().cloned());
    arguments.extend([
        "-Z".to_owned(),
        "unstable-options".to_owned(),
        "--export-json".to_owned(),
    ]);
    let mut command = super::namespace::BackendCommand::new(&request.installation.launcher);
    command
        .args(&arguments)
        .arg(report_path)
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory);
    arguments.push(report_path.display().to_string());
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
) -> Result<(ClassifiedRun, Option<i32>), KaniExecutionRefusal> {
    let stopped = |reason| {
        (
            ClassifiedRun {
                outcome: KaniRunOutcome::Inconclusive { reason },
                success_checks: 0,
                checks: Vec::new(),
            },
            None,
        )
    };
    match settle(launch)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        } => Ok((
            classify_kani_run(exited_successfully, report, &text, kind)?,
            exit_code,
        )),
        Concluded::TimedOut => Ok(stopped(KaniInconclusiveReason::TimedOut)),
        Concluded::MemoryExhausted => Ok(stopped(KaniInconclusiveReason::MemoryExhausted)),
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

#[cfg(all(test, not(target_os = "linux")))]
mod report_fixture {
    use super::*;
    #[cfg(not(target_os = "linux"))]
    use crate::kani::run::launch::run_launcher;

    #[cfg(not(target_os = "linux"))]
    pub(super) type FixtureExecution = ReportedExecution;

    /// Linux exercises bounded public execution; unsupported targets exercise report semantics
    /// without inventing resource-enforcement evidence.
    pub(super) struct ReportGroup {
        pub(super) members: Vec<usize>,
        pub(super) reports: Result<Vec<FixtureExecution>, KaniExecutionRefusal>,
    }

    #[cfg(not(target_os = "linux"))]
    pub(super) fn single(
        request: &KaniExecutionRequest<'_>,
    ) -> Result<FixtureExecution, KaniExecutionRefusal> {
        require_in_crate(request)?;
        let report_path = fresh_report_path(request.target_directory);
        remove_stale_report(&report_path)?;
        let (arguments, recipe) = launch_command(request, &report_path);
        let outcome = run_launcher(
            recipe.report_test_command(),
            request.harness.view().ceilings.wall_clock,
            NonZeroUsize::MIN,
        )
        .map_err(|error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        })?;
        let report = take_report(&outcome, &report_path);
        finish_single(request, arguments, outcome, report?)
    }

    #[cfg(not(target_os = "linux"))]
    pub(super) fn groups(
        requests: &[KaniExecutionRequest<'_>],
    ) -> Result<Vec<ReportGroup>, KaniExecutionRefusal> {
        for request in requests {
            require_in_crate(request)?;
        }
        Ok(plan_groups(requests)
            .into_iter()
            .map(|group| ReportGroup {
                members: group.iter().map(|(position, _)| *position).collect(),
                reports: match group.as_slice() {
                    [(_, only)] => single(only).map(|report| vec![report]),
                    _ => completed_group(&group),
                },
            })
            .collect())
    }

    #[cfg(not(target_os = "linux"))]
    fn completed_group(
        group: &[(usize, &KaniExecutionRequest<'_>)],
    ) -> Result<Vec<FixtureExecution>, KaniExecutionRefusal> {
        let first = group[0].1;
        let selections: Vec<_> = group
            .iter()
            .map(|(_, request)| request.harness.view().selection.clone())
            .collect();
        let report_path = fresh_report_path(first.target_directory);
        remove_stale_report(&report_path)?;
        let (arguments, recipe) = batch_launch_command(first, &selections, &report_path);
        let outcome = run_launcher(
            recipe.report_test_command(),
            outer_bound(first.harness.view().ceilings.wall_clock, group.len()),
            NonZeroUsize::new(group.len()).unwrap(),
        )
        .map_err(|error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: first.installation.launcher.clone(),
            error,
        })?;
        let report = take_report(&outcome, &report_path);
        let Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        } = settle(outcome)?
        else {
            panic!("report fixture must complete; bounded lifecycle is tested separately");
        };
        finish_group(
            group,
            arguments,
            exited_successfully,
            exit_code,
            &text,
            report?,
        )
    }
}

#[cfg(all(test, not(target_os = "linux")))]
#[path = "../../../tests/common/kani_execution_single.rs"]
mod report_tests;

#[cfg(test)]
mod classification_tests {
    use super::*;
    use crate::kani::{
        identity::StateFrameProperty,
        test_support::{report, state_frame_harness, COVER_NO, COVER_OK, PASSED},
    };
    use std::path::PathBuf;

    /// A precondition harness asserts nothing; its only property is its cover. The same report
    /// that is a vacuous proof for any other harness therefore decides by the cover alone for a
    /// precondition harness, whose satisfied cover is its one successful check.
    ///
    /// Trace: FR-017-AC-13, TC-027
    #[test]
    fn a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule() {
        let run = |kind: Option<ObligationKind>, checks: &[(&str, &str)]| {
            launch_evidence(
                LaunchOutcome::Completed {
                    exited_successfully: true,
                    exit_code: Some(0),
                    text: String::new(),
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

    /// The command exports the report to the request's target directory after the harness
    /// identity's options, which are passed unchanged.
    ///
    /// Trace: FR-017-AC-6, TC-027
    #[test]
    fn tc_027_the_launch_exports_the_report_after_the_harness_options() {
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
        let installation = KaniInstallation {
            launcher: PathBuf::from("cargo-kani"),
        };
        let request = KaniExecutionRequest {
            installation: &installation,
            guardian_path: Path::new("/unused-helper"),
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: Path::new("/crate"),
            target_directory: Path::new("/target"),
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

    /// A timed-out launch maps to no exit code and `Inconclusive { reason: TimedOut }`.
    #[test]
    fn a_timed_out_launch_carries_no_exit_code_into_the_evidence() {
        // A timed-out run is killed before it writes a report, so none is consulted.
        let (run, exit_code) = launch_evidence(LaunchOutcome::TimedOut, None, None).unwrap();
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
    ///
    /// Trace: FR-017-AC-14, FR-017-AC-25, TC-043
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

#[cfg(all(test, not(target_os = "linux")))]
#[path = "../../../tests/common/kani_execution_batch.rs"]
mod batch_tests;

#[cfg(test)]
mod planning_tests {
    use super::*;
    use crate::kani::{
        abi::adapter_options,
        identity::{StateFrameHarness, StateFrameProperty},
        test_support::named_state_frame_harness,
    };
    use std::path::PathBuf;
    const T: Duration = Duration::from_secs(30);

    fn member(module: &str, harness: &str, unwind: u32) -> StateFrameHarness {
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
    fn memory_member(module: &str, bytes: u64) -> StateFrameHarness {
        let mut harness = member(module, "check", 4);
        harness.identity.ceilings.memory_bytes = std::num::NonZeroU64::new(bytes).unwrap();
        harness
    }

    /// Unequal identity memory ceilings plan separate groups; each planned selection and argv
    /// retains its member, without executing an unsupported platform's bounded runner.
    /// Trace: FR-028-AC-21, FR-017-AC-21.
    #[test]
    fn unequal_memory_ceilings_plan_separate_groups_with_their_own_selections() {
        let harnesses = [
            memory_member("a", 128 * 1024 * 1024),
            memory_member("b", 256 * 1024 * 1024),
        ];
        let installation = KaniInstallation {
            launcher: PathBuf::from("/unused-launcher"),
        };
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| KaniExecutionRequest {
                installation: &installation,
                guardian_path: Path::new("/unused-helper"),
                harness: harness.into(),
                crate_directory: Path::new("/unused-crate"),
                target_directory: Path::new("/unused-target"),
            })
            .collect();
        let groups = plan_groups(&requests);
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
            let (arguments, _) = launch_command(request, &PathBuf::from("report.json"));
            assert!(arguments
                .windows(3)
                .any(|selection| selection == ["--harness", expected, "--exact"]));
        }
        assert!(
            !shares_process(&requests[0], &requests[1]),
            "unequal memory ceilings cannot share a process"
        );
    }

    /// The per-member timeout is T in whole seconds rounded up; a T whose rounded value is above
    /// 4294967295 omits `--harness-timeout`, and one at the maximum carries it.
    ///
    /// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
    #[test]
    fn tc_043_the_member_timeout_is_whole_seconds_rounded_up_and_omitted_above_kani_maximum() {
        let installation = KaniInstallation {
            launcher: PathBuf::from("cargo-kani"),
        };
        let harness = member("a", "check", 4);
        let timeout_argument = |timeout: Duration| -> Option<String> {
            let mut harness = harness.clone();
            harness.identity.ceilings.wall_clock = timeout;
            let request = KaniExecutionRequest {
                installation: &installation,
                guardian_path: Path::new("/unused-helper"),
                harness: (&harness).into(),
                crate_directory: Path::new("/crate"),
                target_directory: Path::new("/target"),
            };
            let (arguments, _) = batch_launch_command(
                &request,
                &["a::check".to_owned(), "b::check".to_owned()],
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

    /// The outer bound is the member count times T, and a product that does not fit never
    /// elapses.
    ///
    /// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
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
}
