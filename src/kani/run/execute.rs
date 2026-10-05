//! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
//!
//! For every outcome but one, the run outcome is read from the
//! backend's own exported report -- read into a typed value by [`crate::kani::output::report`], the
//! only place Kani's output is read -- and is never defaulted: a harness this module did not
//! observe verifying is not `verified`, and a report it cannot read is a typed refusal, not an
//! outcome. The one exception is [`KaniInconclusiveReason::TimedOut`], which is never read from
//! output at all — a timed-out run is killed before it writes a report.
//!
//! The caller states a wall-clock budget on every request
//! ([`KaniExecutionRequest::timeout`]); nothing here defaults one. A run that does
//! not conclude within it is killed and classified `TimedOut` rather than left to
//! block the caller forever (agent-ix/quire-contract-codegen#58). This module's
//! own call always returns within that budget plus a small constant, regardless
//! of what the launcher forked: the launcher runs as the leader of its own
//! process group and a timeout kills the whole group, so CBMC and every other
//! descendant die with it. A descendant that leaves the group is not reached, and
//! the caller is never made to wait on output from it beyond a short, fixed drain.

//!
//! A batch ([`execute_kani_obligations`]) runs the harnesses that can share a launcher process in
//! one, and splits its one exported report and its one console back into one evidence record per
//! harness. A harness is found in the report by the `module::harness` path the launch passed to
//! `--harness`, and its playback in the console by the path its block is headed for.

use std::{fmt, fs, num::NonZeroUsize, path::Path, process::Command, time::Duration};

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
        launch::{run_launcher, CaptureStream, LaunchOutcome},
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
    /// Backend to invoke.
    pub installation: &'a KaniInstallation,
    /// The generated harness.
    pub harness: KaniExecutableHarness<'a>,
    /// Crate root whose `src/lib.rs` contains the harness source byte-for-byte.
    pub crate_directory: &'a Path,
    /// Cargo target directory for the run.
    pub target_directory: &'a Path,
    /// Wall-clock budget for the launcher. The caller states this explicitly on every
    /// request; there is no default that would let a run go unbounded silently. A run
    /// that has not concluded when the budget elapses is killed and reported as
    /// [`KaniRunOutcome::Inconclusive`] with [`KaniInconclusiveReason::TimedOut`].
    pub timeout: Duration,
}

/// Why a harness was not run.
#[derive(Debug)]
pub enum KaniExecutionRefusal {
    /// The installed backend could not be located or started.
    Tool(KaniToolError),
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
            | Self::HarnessNotInCrate { .. }
            | Self::Report(_)
            | Self::BatchTimedOut { .. }
            | Self::PlaybackForNonMember { .. } => None,
        }
    }
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(error) => write!(formatter, "{error}"),
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
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    require_in_crate(request)?;
    run_single(request)
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
    command: Command,
    timeout: Duration,
    harnesses: NonZeroUsize,
) -> Result<LaunchOutcome, KaniExecutionRefusal> {
    run_launcher(command, timeout, harnesses).map_err(|error| {
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
        LaunchOutcome::TimedOut
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
    run: ClassifiedRun,
    batch: Option<KaniBatchInvocation>,
) -> KaniExecutionEvidence {
    KaniExecutionEvidence {
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
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let harness = request.harness.view();
    let report_path = fresh_report_path(request.target_directory);
    remove_stale_report(&report_path)?;
    let (arguments, command) = launch_command(request, &report_path);
    let launch = start(request, command, request.timeout, NonZeroUsize::MIN)?;
    let report = take_report(&launch, &report_path);
    let (run, exit_code) = launch_evidence(launch, report?.as_deref(), harness.kind)?;
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
/// removed) and equal request timeout; a process also has one launcher, one crate directory and
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
pub fn execute_kani_obligations(
    requests: &[KaniExecutionRequest<'_>],
) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
    for request in requests {
        require_in_crate(request)?;
    }
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
    Ok(groups
        .into_iter()
        .map(|group| KaniGroupRun {
            members: group.iter().map(|(position, _)| *position).collect(),
            evidence: match group.as_slice() {
                [(_, only)] => run_single(only).map(|evidence| vec![evidence]),
                _ => run_group(&group),
            },
        })
        .collect())
}

/// Whether two requests can be run by one launcher process: the same option vector once the
/// harness selection is removed, the same timeout, and the same launcher, crate and target
/// directory.
fn shares_process(first: &KaniExecutionRequest<'_>, other: &KaniExecutionRequest<'_>) -> bool {
    first.timeout == other.timeout
        && first.installation.launcher == other.installation.launcher
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

/// The argument vector and [`Command`] of one process that runs every harness of `selections`.
fn batch_launch_command(
    first: &KaniExecutionRequest<'_>,
    selections: &[String],
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
    if let Ok(seconds) = u32::try_from(whole_seconds(first.timeout)) {
        arguments.extend(["--harness-timeout".to_owned(), seconds.to_string()]);
    }
    arguments.extend(without_selection(first.harness.view().options));
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
    let (arguments, command) = batch_launch_command(first, &selections, &report_path);
    let launch = start(
        first,
        command,
        outer_bound(first.timeout, group.len()),
        count,
    )?;
    let report = take_report(&launch, &report_path);
    let (exited_successfully, exit_code, text) = match settle(launch)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        } => (exited_successfully, exit_code, text),
        Concluded::TimedOut => {
            return Err(KaniExecutionRefusal::BatchTimedOut {
                members: group.len(),
                timeout: first.timeout,
            })
        }
    };
    let invocation = KaniBatchInvocation {
        members: selections.clone(),
        timeout_seconds: whole_seconds(first.timeout),
    };
    let evidence = |run: Vec<ClassifiedRun>| -> Vec<KaniExecutionEvidence> {
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
    let Some(report) = report? else {
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
                classify_member(
                    process_succeeded,
                    member,
                    || counterexample_playback(&text, Some(&view.selection)),
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

/// Builds the exact argument vector and [`Command`] [`execute_kani_obligation`] launches for
/// `request`, without spawning it, so a caller driving [`crate::run_launcher_with_timeout`] itself
/// launches exactly what `execute_kani_obligation` does.
///
/// The vector is the harness identity's option vector followed by the flags that make Kani export
/// its report to a file in the target directory, which is where the verdict is read from. The
/// file's name is unique to this call (the last argument), so two runs sharing a target
/// directory never write, remove or read each other's report.
pub fn kani_launch_command(request: &KaniExecutionRequest<'_>) -> (Vec<String>, Command) {
    launch_command(request, &fresh_report_path(request.target_directory))
}

fn launch_command(
    request: &KaniExecutionRequest<'_>,
    report_path: &Path,
) -> (Vec<String>, Command) {
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(request.harness.view().options.iter().cloned());
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
pub fn launch_evidence(
    launch: LaunchOutcome,
    report: Option<&[u8]>,
    kind: Option<ObligationKind>,
) -> Result<(ClassifiedRun, Option<i32>), KaniExecutionRefusal> {
    match settle(launch)? {
        Concluded::Completed {
            exited_successfully,
            exit_code,
            text,
        } => Ok((
            classify_kani_run(exited_successfully, report, &text, kind)?,
            exit_code,
        )),
        Concluded::TimedOut => Ok((
            ClassifiedRun {
                outcome: KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::TimedOut,
                },
                success_checks: 0,
                checks: Vec::new(),
            },
            None,
        )),
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
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::kani::{
        identity::StateFrameProperty,
        test_support::{
            discover_scratch, report, state_frame_harness, write_launcher, COVER_NO, COVER_OK,
            PASSED,
        },
    };

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

    /// Drives `execute_kani_obligation` against a launcher stand-in that exits with `status` and,
    /// when given a report, writes it where `--export-json` names. `stale` is left in the target
    /// directory beforehand, as an earlier run would leave it.
    fn run_stand_in(
        name: &str,
        status: i32,
        exported: Option<&str>,
        stale: Option<&str>,
    ) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
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
    ) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
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
        let result = execute_kani_obligation(&KaniExecutionRequest {
            installation: &KaniInstallation { launcher },
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: &crate_directory,
            target_directory: &target_directory,
            timeout: Duration::from_secs(30),
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
    ///
    /// Trace: FR-017-AC-18, FR-017-AC-19, TC-027
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
    ///
    /// Trace: FR-017-AC-6, FR-017-AC-19, TC-027
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
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: Path::new("/crate"),
            target_directory: Path::new("/target"),
            timeout: Duration::from_secs(1),
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
    ///
    /// Trace: FR-017-AC-19, TC-027
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

#[cfg(test)]
mod batch_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        core::artifact::Artifact,
        kani::{
            abi::adapter_options,
            identity::{StateFrameHarness, StateFrameProperty},
            run::launch::CAPTURE_LIMIT,
            test_support::{
                batch_report, discover_scratch, named_state_frame_harness, write_launcher,
                BatchEntry, COVER_OK, PASSED,
            },
        },
    };

    /// The request timeout T of the tests that do not vary it.
    const T: Duration = Duration::from_secs(30);

    /// The options every harness of these tests shares once its `--harness <path> --exact`
    /// selection is taken out, written out so the test is not the code's own vector.
    const SHARED_OPTIONS: [&str; 12] = [
        "-Z",
        "function-contracts",
        "-Z",
        "concrete-playback",
        "--unwind",
        "4",
        "--solver",
        "cadical",
        "--output-format",
        "regular",
        "--concrete-playback",
        "print",
    ];

    /// What a launcher stand-in does before anything else: it records that it ran and what it was
    /// given, and finds the report path (its last argument) and the harnesses it was asked for.
    const PROLOGUE: &str = r#"printf 'RUN\n' >> "$CALLS"
for argument; do printf '%s\n' "$argument" >> "$CALLS"; done
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

    fn path_of(harness: &StateFrameHarness) -> String {
        format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        )
    }

    /// A launcher stand-in in a scratch crate and target directory.
    struct StandIn {
        directory: PathBuf,
        crate_directory: PathBuf,
        target_directory: PathBuf,
        installation: KaniInstallation,
    }

    impl StandIn {
        /// A scratch crate and target directory, and the path a launcher will be installed at.
        fn scaffold(name: &str) -> Self {
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
                installation: KaniInstallation { launcher },
            }
        }

        /// Installs the launcher: the prologue, then `body`.
        fn install(&self, body: &str) {
            write_launcher(
                &self.installation.launcher,
                &format!(
                    "CALLS='{}'\n{PROLOGUE}{body}\n",
                    self.directory.join("calls").display()
                ),
            );
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

        fn request<'a>(
            &'a self,
            harness: &'a StateFrameHarness,
            timeout: Duration,
        ) -> KaniExecutionRequest<'a> {
            KaniExecutionRequest {
                installation: &self.installation,
                harness: harness.into(),
                crate_directory: &self.crate_directory,
                target_directory: &self.target_directory,
                timeout,
            }
        }

        /// Runs `harnesses` as a batch at `timeout`.
        fn batch(
            &self,
            harnesses: &[StateFrameHarness],
            timeout: Duration,
        ) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
            let requests: Vec<_> = harnesses
                .iter()
                .map(|harness| self.request(harness, timeout))
                .collect();
            execute_kani_obligations(&requests)
        }

        /// The argument vector of each process the stand-in ran, as it received them.
        fn calls(&self) -> Vec<Vec<String>> {
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

    /// The evidence of the one process of a batch that ran as one group, or its refusal.
    fn only_group(
        runs: Result<Vec<KaniGroupRun>, KaniExecutionRefusal>,
    ) -> Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal> {
        let mut runs = runs.expect("no harness is missing from the crate");
        assert_eq!(runs.len(), 1, "the harnesses share one process");
        runs.remove(0).evidence
    }

    fn outcomes(evidence: &[KaniExecutionEvidence]) -> Vec<KaniRunOutcome> {
        evidence
            .iter()
            .map(|evidence| evidence.outcome.clone())
            .collect()
    }

    const PLAYBACK_BODY: &str = "fn from_{}() { kani::concrete_playback_run(v, h); }";

    /// A playback block headed for `path` that checks a `kind` property.
    fn block(path: &str, kind: &str, name: &str) -> String {
        format!(
            "Concrete playback unit test for `{path}`:\n```\n/// Test generated for harness `{path}`\n///\n/// Check for `{kind}`: \"c\"\n\n#[test]\n{}\n```\n",
            PLAYBACK_BODY.replace("{}", name)
        )
    }

    fn entry<'a>(
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

    const FAILED: (&str, &str) = ("Failure", "assertion");

    fn pair() -> Vec<StateFrameHarness> {
        vec![member("a", "check", 4), member("b", "check", 4)]
    }

    /// N harnesses with equal options and timeout start one launcher process, whose arguments hold
    /// one `--harness <module::harness> --exact` pair per member in request order, then
    /// `--harness-timeout` T, then the shared options; one harness starts one process with the
    /// single-run vector. Counting the processes before (one run each) and after batching gives
    /// N and 1 for N = 1, 10 and 50.
    ///
    /// Trace: FR-017-AC-21, TC-043
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
            }
        }
    }

    /// Harnesses with a different option vector or a different request timeout do not share a
    /// process: N harnesses in G such groups start G processes, each group's members in request
    /// order.
    ///
    /// Trace: FR-017-AC-21, TC-043
    #[test]
    fn tc_043_harnesses_group_by_option_vector_and_by_request_timeout() {
        // Two unwind bounds times two timeouts, two harnesses in each: eight, four groups,
        // requested interleaved so that a group is not a contiguous run of the request list.
        let cells = [(4, 30), (5, 30), (4, 60), (5, 60)];
        let harnesses: Vec<_> = (0..8)
            .map(|index| member(&format!("m{index}"), "check", cells[index % 4].0))
            .collect();
        let stand_in = StandIn::verifying("grouping");
        let requests: Vec<_> = harnesses
            .iter()
            .enumerate()
            .map(|(index, harness)| {
                stand_in.request(harness, Duration::from_secs(cells[index % 4].1))
            })
            .collect();
        let runs = execute_kani_obligations(&requests).unwrap();
        assert_eq!(stand_in.calls().len(), 4, "eight harnesses in four groups");
        assert_eq!(
            runs.iter()
                .map(|run| run.members.clone())
                .collect::<Vec<_>>(),
            [vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]]
        );
        for run in runs {
            assert!(run
                .evidence
                .unwrap()
                .iter()
                .all(|evidence| evidence.batch.is_some()));
        }
        // Equal options and timeouts again but in other crates would be two processes; with
        // one crate, one harness of each shape is two single runs, not a batch.
        let separate = StandIn::verifying("grouping-separate");
        let pair = [member("x", "check", 4), member("y", "check", 5)];
        let runs = separate.batch(&pair, T).unwrap();
        assert_eq!(separate.calls().len(), 2);
        assert!(runs.iter().all(|run| run
            .evidence
            .as_ref()
            .is_ok_and(|evidence| evidence.len() == 1 && evidence[0].batch.is_none())));
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
            let request = KaniExecutionRequest {
                installation: &installation,
                harness: (&harness).into(),
                crate_directory: Path::new("/crate"),
                target_directory: Path::new("/target"),
                timeout,
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

    /// A process has one launcher, one working directory and one target directory: requests that
    /// name another of any of the three are never grouped, however equal their options and
    /// timeouts. Each case runs two requests and counts the processes each launcher was started
    /// as.
    ///
    /// Trace: FR-017-AC-21, TC-043
    #[test]
    fn tc_043_requests_naming_another_launcher_crate_or_target_directory_are_not_grouped() {
        let (a, b) = (member("a", "check", 4), member("b", "check", 4));

        // Another launcher: each launcher runs one harness.
        let (first, second) = (StandIn::verifying("key-l1"), StandIn::verifying("key-l2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                installation: &second.installation,
                ..first.request(&b, T)
            },
        ];
        let runs = execute_kani_obligations(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!((first.calls().len(), second.calls().len()), (1, 1));

        // Another crate directory: two processes of the one launcher, one in each crate.
        let (first, second) = (StandIn::verifying("key-c1"), StandIn::verifying("key-c2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                crate_directory: &second.crate_directory,
                ..first.request(&b, T)
            },
        ];
        let runs = execute_kani_obligations(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!(first.calls().len(), 2);

        // Another target directory: the same.
        let (first, second) = (StandIn::verifying("key-t1"), StandIn::verifying("key-t2"));
        let requests = [
            first.request(&a, T),
            KaniExecutionRequest {
                target_directory: &second.target_directory,
                ..first.request(&b, T)
            },
        ];
        let runs = execute_kani_obligations(&requests).unwrap();
        assert_eq!(runs.len(), 2);
        assert_eq!(first.calls().len(), 2);
    }

    /// A batch of N is bounded by N times T, not by T: four members at T = 3 s run for eight
    /// seconds, longer than T and than N times T halved (6 s), and inside N times T (12 s), and
    /// complete with every member verified.
    ///
    /// The stand-in's sleep never ends early, so it outlasts T and N times T halved however the
    /// scheduler treats the test; only the 4 s between it and the 12 s bound can be eaten by a
    /// loaded host (an earlier 1 s / 2 s / 4 s version left 2 s and flaked on a loaded runner).
    ///
    /// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
    #[test]
    fn tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t() {
        let stand_in = StandIn::running(
            "inside-n-times-t",
            &format!("sleep 8\n{VERIFY_EVERY_HARNESS}"),
        );
        let harnesses: Vec<_> = (0..4)
            .map(|index| member(&format!("m{index}"), "check", 4))
            .collect();
        let evidence = only_group(stand_in.batch(&harnesses, Duration::from_secs(3))).unwrap();
        assert_eq!(
            outcomes(&evidence),
            vec![KaniRunOutcome::Verified; 4],
            "a bound of T, not N times T, would have killed this batch"
        );
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

    /// A batch still running at N times T is killed and refused as timed out, and no member is
    /// classified: the stand-in sleeps far past the bound and writes no report.
    ///
    /// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
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
    ///
    /// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
    #[test]
    fn tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one() {
        let without = StandIn::verifying("duration-max");
        let evidence = only_group(without.batch(&pair(), Duration::MAX)).unwrap();
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
        let evidence = only_group(maximum.batch(&pair(), Duration::from_secs(seconds))).unwrap();
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
    ///
    /// Trace: FR-017-AC-21, TC-043
    #[test]
    fn tc_043_a_batch_with_no_report_is_refused_after_a_clean_exit_and_no_verdict_after_a_failed_one(
    ) {
        let clean = StandIn::replaying("no-report-clean", None, "", 0);
        assert!(matches!(
            only_group(clean.batch(&pair(), T)),
            Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
        ));
        let failed = StandIn::replaying("no-report-failed", None, "error: bad argument", 2);
        let evidence = only_group(failed.batch(&pair(), T)).unwrap();
        assert_eq!(
            outcomes(&evidence),
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
    ///
    /// Trace: FR-017-AC-22, TC-043
    #[test]
    fn tc_043_members_are_matched_by_path_and_classified_from_their_own_entry() {
        let report = batch_report(&[
            entry("b::check", "Failure", &[FAILED, COVER_OK]),
            entry("a::check", "Success", &[PASSED, PASSED, COVER_OK]),
        ]);
        let console = format!(
            "{}{}{}",
            block("a::check", "cover", "cover_a"),
            block("b::check", "cover", "cover_b"),
            block("b::check", "assertion", "falsify_b")
        );
        let stand_in = StandIn::replaying("by-path", Some(&report), &console, 1);
        let evidence = only_group(stand_in.batch(&pair(), T)).unwrap();
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
    ///
    /// Trace: FR-017-AC-22, TC-043
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
        let evidence = only_group(stand_in.batch(&harnesses, T)).unwrap();
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
        let wire = serde_json::to_value(&evidence[0]).unwrap();
        assert_eq!(wire["batch"]["members"][1], "b::check");
        assert_eq!(wire["batch"]["timeoutSeconds"], 30);
        // A single run's evidence has no `batch` member at all.
        let single = StandIn::verifying("evidence-single");
        let alone = execute_kani_obligation(&single.request(&harnesses[0], T)).unwrap();
        assert!(serde_json::to_value(&alone).unwrap().get("batch").is_none());
    }

    /// A member's success is believed beside a non-zero exit only when some entry states the
    /// failure that exit stands for: every member Success with exit 1 is inconclusive, and one
    /// Failure entry makes the other member's success verified.
    ///
    /// Trace: FR-017-AC-22, TC-043
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
            outcomes(&only_group(unexplained.batch(&pair(), T)).unwrap()),
            vec![no_verdict.clone(); 2]
        );
        let explained_report = batch_report(&[
            entry("a::check", "Success", &[PASSED, COVER_OK]),
            entry("b::check", "Failure", &[FAILED, COVER_OK]),
        ]);
        let console = block("b::check", "assertion", "falsify_b");
        let explained = StandIn::replaying("exit-explained", Some(&explained_report), &console, 1);
        let evidence = only_group(explained.batch(&pair(), T)).unwrap();
        assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
        assert!(matches!(
            evidence[1].outcome,
            KaniRunOutcome::Falsified { .. }
        ));
        // The same two entries and a zero exit are the same results: the exit only withholds.
        let clean = StandIn::replaying("exit-zero", Some(&all_success), "", 0);
        assert_eq!(
            outcomes(&only_group(clean.batch(&pair(), T)).unwrap()),
            vec![KaniRunOutcome::Verified; 2]
        );
    }

    /// An entry that states failure with no checks and exit status `timeout` is inconclusive as
    /// timed out, naming T, while the others keep their results; one with no checks and no
    /// timeout is inconclusive with no counterexample.
    ///
    /// Trace: FR-017-AC-22, FR-028-AC-12, TC-039, TC-043
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
        let stand_in = StandIn::replaying("timeout-entry", Some(&report), "", 1);
        let harnesses = [
            member("a", "check", 4),
            member("b", "check", 4),
            member("c", "check", 4),
            member("d", "check", 4),
        ];
        // 90.5 s: Kani is told 91 whole seconds and the evidence names 91, rounded up.
        let evidence =
            only_group(stand_in.batch(&harnesses, Duration::from_millis(90_500))).unwrap();
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
    ///
    /// Trace: FR-017-AC-23, TC-043
    #[test]
    fn tc_043_a_report_that_lacks_repeats_or_adds_a_harness_refuses_the_batch() {
        let verified = [PASSED, COVER_OK];
        let refusal = |name: &str, entries: &[BatchEntry<'_>]| {
            let stand_in = StandIn::replaying(name, Some(&batch_report(entries)), "", 0);
            only_group(stand_in.batch(&pair(), T)).unwrap_err()
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
    ///
    /// Trace: FR-017-AC-23, TC-043
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
    ///
    /// Trace: FR-017-AC-23, TC-043
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
            "{}{}{}{}",
            block("a::check", "assertion", "falsify_a"),
            block("b::check", "cover", "cover_b"),
            block("c::check", "cover", "cover_c"),
            block("c::check", "assertion", "falsify_c"),
        );
        let stand_in = StandIn::replaying("own-playback", Some(&report), &console, 1);
        let harnesses = [
            member("a", "check", 4),
            member("b", "check", 4),
            member("c", "check", 4),
            member("d", "check", 4),
        ];
        let evidence = only_group(stand_in.batch(&harnesses, T)).unwrap();
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
    ///
    /// Trace: FR-017-AC-22, FR-017-AC-23, TC-043
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
        let stand_in = StandIn::replaying(
            "two-failed-checks",
            Some(&report),
            TWO_FAILED_CHECKS_CONSOLE,
            1,
        );
        let harnesses = [member("two", "check", 4), member("ok", "check", 4)];
        let evidence = only_group(stand_in.batch(&harnesses, T)).unwrap();
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
            TWO_FAILED_CHECKS_CONSOLE,
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
    ///
    /// Trace: FR-017-AC-23, TC-043
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
            only_group(stand_in.batch(&pair(), T)),
            Err(KaniExecutionRefusal::PlaybackForNonMember { harness }) if harness == "z::check"
        ));
    }

    /// A run over the limit for its harness count is refused with the stable code naming the
    /// stream, the limit and the count: one harness at 8 MiB, a batch of two at 16 MiB; a batch
    /// at exactly its limit completes with its real exit status.
    ///
    /// Trace: FR-017-AC-14, TC-043
    #[test]
    fn tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count() {
        let flood = |bytes: usize, exit: i32| format!("head -c {bytes} /dev/zero; exit {exit}");
        let one = StandIn::running("flood-one", &flood(CAPTURE_LIMIT + 1, 0));
        let harness = member("a", "check", 4);
        let single = execute_kani_obligation(&one.request(&harness, T)).unwrap_err();
        assert!(matches!(
            single,
            KaniExecutionRefusal::OutputOverLimit {
                stream: CaptureStream::Stdout,
                limit,
                harnesses: 1
            } if limit == CAPTURE_LIMIT
        ));
        assert_eq!(single.code(), Some("kani_output_over_limit"));

        let two = StandIn::running("flood-two", &flood(2 * CAPTURE_LIMIT + 1, 0));
        let batch = only_group(two.batch(&pair(), T)).unwrap_err();
        assert!(matches!(
            batch,
            KaniExecutionRefusal::OutputOverLimit {
                harnesses: 2,
                limit,
                ..
            } if limit == 2 * CAPTURE_LIMIT
        ));

        let exact = StandIn::running("flood-exact", &flood(2 * CAPTURE_LIMIT, 1));
        let evidence = only_group(exact.batch(&pair(), T)).unwrap();
        assert!(evidence
            .iter()
            .all(|evidence| evidence.exit_code == Some(1)));
        assert_eq!(
            outcomes(&evidence),
            vec![
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::NoVerdict
                };
                2
            ]
        );
    }
}
