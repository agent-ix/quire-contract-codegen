//! Execution of one generated Kani obligation harness (FR-017).
//!
//! For every outcome but one, the run outcome is read from the
//! backend's own output -- read into a typed transcript by [`crate::kani_transcript`], the only
//! place Kani's prose is parsed -- and is never defaulted: a harness this module did not
//! observe verifying is not `verified`. The one exception is
//! [`KaniInconclusiveReason::TimedOut`], which is never read from output at all —
//! a timed-out run is killed before it prints one.
//!
//! The caller states a wall-clock budget on every request
//! ([`KaniExecutionRequest::timeout`]); nothing here defaults one. A run that does
//! not conclude within it is killed and classified `TimedOut` rather than left to
//! block the caller forever (agent-ix/quire-contract-codegen#58). This module's
//! own call always returns within that budget plus a small constant, regardless
//! of what the launcher forked: [`kill_process_tree`] walks `/proc` for every
//! process it can still see descended from the launcher at that instant and
//! signals each by its own positive pid (a process-group-wide signal was tried
//! first and abandoned — see [`run_launcher_with_timeout`]), and the caller is
//! never made to wait on output from anything that walk did not reach, including
//! a process the launcher forked in the instant between the walk's snapshot and
//! the kill, or one that had already reparented away from it. Such a straggler
//! is not guaranteed killed by this call — only that this call does not wait for
//! it.

use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fmt, fs, io,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde::Serialize;

use crate::{
    kani_obligations::{KaniObligationHarness, KaniScalarObligationHarness, ObligationKind},
    kani_transcript::{
        KaniBanner, KaniCoverSummary, KaniFailedCheck, KaniPlaybackTarget, KaniTranscript,
    },
    oracle::Artifact,
};
use quire_contract_ir::kani::{KaniOutcome, KaniOutcomeKind};

/// A backend component this module locates or reads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniTool {
    /// The `cargo-kani` launcher.
    Launcher,
    /// The generated crate's `src/lib.rs`, checked for the harness's generated source.
    Library,
}

/// Why the installed backend could not be located or started.
#[derive(Debug)]
pub enum KaniToolError {
    /// The component is absent at the path it was looked for.
    Missing {
        /// Component.
        tool: KaniTool,
        /// Path looked at.
        path: PathBuf,
    },
    /// The component exists but could not be read or started.
    Io {
        /// Component.
        tool: KaniTool,
        /// Path read or started.
        path: PathBuf,
        /// Underlying error.
        error: io::Error,
    },
}

impl fmt::Display for KaniToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { tool, path } => {
                write!(formatter, "{tool:?} is absent at {}", path.display())
            }
            Self::Io { tool, path, error } => {
                write!(formatter, "{tool:?} at {}: {error}", path.display())
            }
        }
    }
}

impl std::error::Error for KaniToolError {}

/// Where the Kani launcher lives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniInstallation {
    /// The `cargo-kani` executable that is invoked directly.
    pub launcher: PathBuf,
}

impl KaniInstallation {
    /// Locates the launcher the way Cargo resolves a subcommand — `$CARGO_HOME/bin`
    /// (default `$HOME/.cargo/bin`) first, then `PATH`.
    pub fn discover() -> Result<Self, KaniToolError> {
        Self::discover_from(
            env::var_os("HOME"),
            env::var_os("CARGO_HOME"),
            env::var_os("PATH"),
        )
    }

    /// `discover`'s resolution logic, taking each environment variable as an explicit argument
    /// instead of reading the process environment. `discover` is the only caller in this crate;
    /// tests call this directly so every `HOME`/`CARGO_HOME`/`PATH` combination is
    /// exercised as a pure function of its arguments, never by mutating process-wide state with
    /// `env::set_var`/`env::remove_var`, which races with any other thread reading `environ` —
    /// including a concurrently spawned child process snapshotting the environment at fork/exec.
    fn discover_from(
        home: Option<OsString>,
        cargo_home: Option<OsString>,
        path: Option<OsString>,
    ) -> Result<Self, KaniToolError> {
        let home = home.map(PathBuf::from);
        let launcher_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);
        let cargo_home = cargo_home
            .map(PathBuf::from)
            .or_else(|| home.as_ref().map(|home| home.join(".cargo")));
        let launcher = cargo_home
            .map(|directory| directory.join("bin").join(&launcher_name))
            .into_iter()
            .chain(
                path.map(|value| env::split_paths(&value).collect::<Vec<_>>())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|directory| directory.join(&launcher_name)),
            )
            .find(|candidate| candidate.is_file())
            .ok_or_else(|| KaniToolError::Missing {
                tool: KaniTool::Launcher,
                path: PathBuf::from(&launcher_name),
            })?;
        Ok(Self { launcher })
    }
}

/// A generated harness of either kind this module can run.
///
/// The two kinds carry their own identity types but share every fact execution reads, so
/// `view()` projects those facts once and the byte-for-byte source check, the launch and the classification stay one code path (FR-017).
#[derive(Clone, Copy, Debug)]
pub enum KaniExecutableHarness<'a> {
    /// A V1 frozen-clause contract harness (FR-015).
    Contract(&'a KaniObligationHarness),
    /// A V2 exact-scalar harness, as `generate_routed` returns it (FR-022).
    Scalar(&'a KaniScalarObligationHarness),
}

impl<'a> From<&'a KaniObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniObligationHarness) -> Self {
        Self::Contract(harness)
    }
}

impl<'a> From<&'a KaniScalarObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniScalarObligationHarness) -> Self {
        Self::Scalar(harness)
    }
}

/// Exactly what execution reads from a harness, whichever kind it is.
struct HarnessView<'a> {
    rust: &'a Artifact,
    kind: Option<ObligationKind>,
    unwind: u32,
    solver: &'a str,
    options: &'a [String],
}

impl<'a> KaniExecutableHarness<'a> {
    fn view(self) -> HarnessView<'a> {
        match self {
            Self::Contract(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    rust: &harness.rust,
                    kind: Some(identity.kind),
                    unwind: identity.unwind,
                    solver: &identity.solver,
                    options: &identity.options,
                }
            }
            Self::Scalar(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    rust: &harness.rust,
                    kind: None,
                    unwind: identity.unwind,
                    solver: &identity.solver,
                    options: &identity.options,
                }
            }
        }
    }
}

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
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(error) => write!(formatter, "{error}"),
            Self::HarnessNotInCrate { harness_path } => {
                write!(formatter, "the crate does not contain {harness_path}")
            }
        }
    }
}

impl std::error::Error for KaniExecutionRefusal {}

impl From<KaniToolError> for KaniExecutionRefusal {
    fn from(error: KaniToolError) -> Self {
        Self::Tool(error)
    }
}

/// Why a run proves nothing. Every variant but [`TimedOut`](Self::TimedOut) is a completed run
/// the backend printed no usable verdict for; `TimedOut` is not a completed run at all — it is
/// killed before it ever prints one.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniInconclusiveReason {
    /// Kani reported failure but printed no concrete playback.
    FailedWithoutCounterexample,
    /// Kani printed no verification verdict: a build, launcher or solver failure.
    NoVerdict,
    /// Kani reported success without a readable, non-empty cover summary, so non-vacuity was
    /// not observed.
    MissingCoverSummary,
    /// Kani's own `** <failed> of <total> failed` check summary reports zero SUCCESS checks:
    /// no check in the obligation actually ran, so a `Proved`-looking run proved nothing. This
    /// settles [`quire_contract_ir::kani::KaniOutcomeKind::Inconclusive`]'s
    /// `kani_vacuous_proof` cause for the execution path, using the check count this module
    /// itself reads from the backend's own printed output — never a generation-time value —
    /// so it is an execution outcome this module observed, not a generation-time
    /// classification reported as one (FR-017-CON-2).
    VacuousProof,
    /// A loop-unwinding check failed: the loop bound was exhausted before the property
    /// could be decided, so no failure is a counterexample.
    UnwindBoundExhausted,
    /// The run did not conclude within [`KaniExecutionRequest::timeout`]. The launcher and every
    /// process it forked were killed; no verdict, failed-check count or playback is available
    /// because none was ever printed.
    TimedOut,
}

/// The backend-reported outcome of one run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum KaniRunOutcome {
    /// Every property held and every cover was satisfied, so the harness's assumptions,
    /// requires and IR bounds are jointly satisfiable.
    Verified,
    /// A property other than a loop-unwinding check failed and Kani printed a concrete
    /// counterexample for it.
    Falsified {
        /// The concrete-playback test Kani printed for the failed check, verbatim.
        counterexample: String,
    },
    /// Vacuous: no property failed, but a cover was not satisfied within the bounds. For a
    /// precondition harness the precondition is unsatisfiable; for a contract harness the
    /// requires and IR bounds are jointly unsatisfiable, so the ensures was never checked.
    CoverUnsatisfied {
        /// Satisfied cover properties.
        satisfied: u64,
        /// Total cover properties.
        total: u64,
    },
    /// The run established nothing.
    Inconclusive {
        /// Why.
        reason: KaniInconclusiveReason,
    },
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
    pub solver: String,
    /// Process exit code, or `None` when the process was killed by a signal — including the
    /// kill this module itself sends on [`KaniInconclusiveReason::TimedOut`].
    pub exit_code: Option<i32>,
    /// Backend-reported outcome.
    pub outcome: KaniRunOutcome,
}

/// Runs the harness and reports the backend's own outcome.
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
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
    let (arguments, command) = kani_launch_command(request);
    let launch =
        run_launcher_with_timeout(command, request.timeout).map_err(|error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        })?;
    let (outcome, exit_code) = launch_evidence(launch);
    Ok(KaniExecutionEvidence {
        kind: harness.kind,
        harness_path: harness.rust.path.clone(),
        launcher_path: request.installation.launcher.display().to_string(),
        arguments,
        unwind: harness.unwind,
        solver: harness.solver.to_owned(),
        exit_code,
        outcome,
    })
}

/// Builds the exact argument vector and [`Command`] [`execute_kani_obligation`] launches for
/// `request`, without spawning it, so a caller driving [`run_launcher_with_timeout`] itself
/// launches exactly what `execute_kani_obligation` does.
pub fn kani_launch_command(request: &KaniExecutionRequest<'_>) -> (Vec<String>, Command) {
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(request.harness.view().options.iter().cloned());
    let mut command = Command::new(&request.installation.launcher);
    command
        .args(&arguments)
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory);
    (arguments, command)
}

/// How the launcher's run within its caller-declared budget ([`KaniExecutionRequest::timeout`])
/// concluded.
#[non_exhaustive]
pub enum LaunchOutcome {
    /// The process exited on its own within the budget.
    Completed {
        /// `ExitStatus::success()`.
        exited_successfully: bool,
        /// `ExitStatus::code()`.
        exit_code: Option<i32>,
        /// Combined stdout and stderr, newline-joined, matching `classify_kani_run`'s input shape.
        text: String,
    },
    /// The budget elapsed before the process exited. Every descendant this call could still see
    /// in `/proc` at that instant has been killed and reaped; one that forked or reparented away
    /// in the instant before the kill is not guaranteed to be — only that this call does not
    /// wait for it.
    TimedOut,
}

/// Polling interval while waiting for the launcher to exit within its budget. Short enough that
/// a tight caller-declared timeout in a test is still observed promptly, long enough not to spin.
const LAUNCHER_POLL_INTERVAL: Duration = Duration::from_millis(20);

/// Runs `command` to completion or kills what it can still find of it once `timeout` elapses,
/// whichever happens first — but always **returns** within `timeout` plus a small constant
/// either way; that bound does not depend on whether the kill actually reached everything.
///
/// Kani's launcher forks `kani-driver`, which forks CBMC, so on a timeout `child.kill()` alone
/// would leave CBMC — the actual solver, and the one most likely to be the non-terminating
/// process a budget exists to bound — orphaned and still running past the deadline it just
/// exceeded. `kill_process_tree` finds and signals every live descendant it can still see by
/// its own pid instead of relying on a process-group-wide signal: a negative-pid group kill is
/// the textbook fix, but it is deliberately not used here, because it was measured to escape its
/// own group on the sandbox this crate was developed in — killing a freshly spawned child's
/// isolated process group also killed the unrelated caller in the same run, reproduced with a
/// minimal standalone program before this function was written this way. Signalling only
/// positive, individually discovered pids cannot exhibit that failure mode.
///
/// That walk is one `/proc` snapshot, so it is inherently unable to see a process forked after
/// it, or one that reparented away from the launcher before it (a double fork, `setsid`, or
/// simply an orphan whose original parent already exited) — either keeps its own copy of the
/// inherited stdout/stderr pipe write end open, which is why this function does not wait for the
/// reader threads to see EOF on a timeout: doing so would block on that copy until whatever
/// holds it happens to exit on its own, which can be arbitrarily long and would make this
/// function's own return time unbounded — the defect this exists to remove, in a new place. See
/// the `None` arm below for that reasoning in full, and the module's own top-level doc comment
/// for the guarantee this leaves in place versus the one it does not.
///
/// Stdout and stderr are drained on their own threads as soon as the process is spawned, the same
/// way `Command::output()` drains them internally: a full pipe buffer would otherwise stall the
/// child while this function is only polling `try_wait`, turning a bounded run into a hang of its
/// own.
pub fn run_launcher_with_timeout(
    mut command: Command,
    timeout: Duration,
) -> io::Result<LaunchOutcome> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn()?;
    let pid = child.id();
    let mut stdout = child.stdout.take().expect("stdout was piped at spawn");
    let mut stderr = child.stderr.take().expect("stderr was piped at spawn");
    let stdout_reader = thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stdout.read_to_end(&mut buffer);
        buffer
    });
    let stderr_reader = thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stderr.read_to_end(&mut buffer);
        buffer
    });

    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() >= deadline {
            break None;
        }
        thread::sleep(LAUNCHER_POLL_INTERVAL);
    };

    match status {
        Some(status) => {
            let stdout_bytes = stdout_reader.join().unwrap_or_default();
            let stderr_bytes = stderr_reader.join().unwrap_or_default();
            Ok(LaunchOutcome::Completed {
                exited_successfully: status.success(),
                exit_code: status.code(),
                text: format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&stdout_bytes),
                    String::from_utf8_lossy(&stderr_bytes)
                ),
            })
        }
        None => {
            kill_process_tree(pid);
            // Belt-and-suspenders repeat targeted at the direct child alone, in case `pid` had
            // already exited between the last `try_wait` and the tree walk above and so was
            // absent from it (`kill_process_tree` reads `/proc` at one instant; it cannot see a
            // process that exited before that read).
            let _ = child.kill();
            let _ = child.wait();
            // The reader threads are deliberately NOT joined here. `kill_process_tree` only
            // reaches what its one `/proc` snapshot could still see: a process the launcher
            // forked in the instant between that snapshot and the kill, or one that reparented
            // away from the launcher before either (a double fork, `setsid`, or simply a plain
            // orphan whose parent already exited), keeps its inherited copy of the pipes' write
            // end open and is never touched by this call. Joining here would block
            // `read_to_end` on that copy until whatever holds it happens to exit on its own,
            // making this function's own return time unbounded on exactly the kind of process a
            // caller-declared budget exists to bound (agent-ix/quire-contract-codegen#58) — the
            // defect returning wearing the correct typed result. A timed-out run carries no
            // captured text at all (`LaunchOutcome::TimedOut` has none), so nothing this call
            // needs is lost by leaving the readers running in the background, unjoined, for as
            // long as whatever they are still attached to keeps them alive.
            Ok(LaunchOutcome::TimedOut)
        }
    }
}

/// Kills `root` and every process descended from it, discovered by walking `/proc`'s live
/// parent/child relationships at one instant and signalling each by its own positive pid. See
/// [`run_launcher_with_timeout`] for why this walks the tree instead of sending one signal to a
/// process group.
fn kill_process_tree(root: u32) {
    for pid in descendants_including_self(root) {
        let _ = Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status();
    }
}

/// `root` followed by every live process transitively parented by it, in discovery order.
/// Built from one snapshot of `/proc`, so a process forked after the snapshot is not included —
/// the same inherent limitation any tree-walking killer has, standard practice for this problem.
fn descendants_including_self(root: u32) -> Vec<u32> {
    let mut children_of: HashMap<u32, Vec<u32>> = HashMap::new();
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let Some(pid) = entry
                .file_name()
                .to_str()
                .and_then(|name| name.parse::<u32>().ok())
            else {
                continue;
            };
            if let Some(ppid) = parent_pid(pid) {
                children_of.entry(ppid).or_default().push(pid);
            }
        }
    }
    let mut order = vec![root];
    let mut frontier = vec![root];
    while let Some(pid) = frontier.pop() {
        if let Some(children) = children_of.get(&pid) {
            for &child in children {
                order.push(child);
                frontier.push(child);
            }
        }
    }
    order
}

/// The parent pid recorded in `/proc/<pid>/stat`'s fourth field, or `None` when the process is
/// gone or the field cannot be read. The executable name in the second field is
/// parenthesized and may itself contain spaces or parentheses, so the parse splits on the last
/// `)` in the line rather than on whitespace from the start.
fn parent_pid(pid: u32) -> Option<u32> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let after_comm = stat.rsplit_once(')')?.1;
    after_comm.split_whitespace().nth(1)?.parse().ok()
}

/// Maps a concluded [`LaunchOutcome`] to the `(outcome, exit_code)` pair
/// [`KaniExecutionEvidence`] stores, exactly as `execute_kani_obligation` does. Kept as its own
/// pure function so the mapping is tested directly with a value rather than a real subprocess.
pub fn launch_evidence(launch: LaunchOutcome) -> (KaniRunOutcome, Option<i32>) {
    match launch {
        LaunchOutcome::Completed {
            exited_successfully,
            exit_code,
            text,
        } => (classify_kani_run(exited_successfully, &text), exit_code),
        LaunchOutcome::TimedOut => (
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut,
            },
            None,
        ),
    }
}

/// Classifies one run. Every generated harness, of every kind, carries exactly the covers that
/// witness its assumptions are satisfiable, so success without every cover satisfied is vacuous
/// and never `Verified`.
///
/// Before consulting the cover summary at all, a `** <failed> of <total> failed` line, when
/// present, is reduced to a SUCCESS-check count (`total - failed`) and routed through
/// [`KaniOutcome::proved_from_checks`] — the one implementation of "a proof backed by zero
/// checks proved nothing" that this module and `quire-contract-ir` both had before this shared
/// call, disagreeing (agent-ix/quire-contract-codegen#99). A verdict of `Inconclusive` under
/// [`KaniOutcomeKind::Inconclusive`]'s `kani_vacuous_proof` cause is not reported as-is — that
/// would violate FR-017-CON-2, which forbids reporting a generation-time classification as an
/// execution outcome — it is mapped into this module's own [`KaniInconclusiveReason::VacuousProof`].
/// The count it classifies is read from this run's own transcript, never from generation time, so
/// the mapped result is still this module's own observation of what the backend printed, not a
/// borrowed verdict. A transcript with no such line at all (older or differently shaped output)
/// falls through unchanged to the cover-only classification below.
///
/// `pub` so a test asserting "this transcript proves falsification" can route through the same
/// classifier production uses (IR-220), instead of re-implementing banner parsing that misreads
/// an inconclusive run — CBMC out-of-memory among them — as a decided failure.
pub fn classify_kani_run(exited_successfully: bool, text: &str) -> KaniRunOutcome {
    classify_transcript(exited_successfully, &KaniTranscript::parse(text))
}

/// The classification rule (codegen#55), over the typed transcript only. Kani's prose is read in
/// [`crate::kani_transcript`] and nowhere else.
fn classify_transcript(exited_successfully: bool, transcript: &KaniTranscript) -> KaniRunOutcome {
    if exited_successfully && transcript.banner == KaniBanner::Successful {
        if let Some(summary) = transcript.checks_summary {
            let success_checks =
                usize::try_from(summary.total.saturating_sub(summary.failed)).unwrap_or(usize::MAX);
            let checks_outcome = KaniOutcome::proved_from_checks(
                success_checks,
                "kani_execution::classify_kani_run",
                "checks_summary",
            );
            if checks_outcome.kind == KaniOutcomeKind::Inconclusive
                && checks_outcome.code == "kani_vacuous_proof"
            {
                return KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::VacuousProof,
                };
            }
        }
        return match transcript.cover_summary {
            KaniCoverSummary::Counts {
                satisfied, total, ..
            } if total > 0 && satisfied == total => KaniRunOutcome::Verified,
            KaniCoverSummary::Counts {
                satisfied, total, ..
            } if total > 0 => KaniRunOutcome::CoverUnsatisfied { satisfied, total },
            KaniCoverSummary::Counts { .. }
            | KaniCoverSummary::Malformed
            | KaniCoverSummary::Absent => KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary,
            },
        };
    }
    if matches!(transcript.banner, KaniBanner::Failed | KaniBanner::Both) {
        if transcript
            .failed_checks
            .contains(&KaniFailedCheck::UnwindingAssertion)
        {
            return KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted,
            };
        }
        let counterexample = transcript
            .playbacks
            .iter()
            .find(|playback| playback.target == KaniPlaybackTarget::Property);
        return match counterexample {
            Some(playback) => KaniRunOutcome::Falsified {
                counterexample: playback.test.clone(),
            },
            None => KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample,
            },
        };
    }
    KaniRunOutcome::Inconclusive {
        reason: KaniInconclusiveReason::NoVerdict,
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
    use super::*;

    const COVER_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `cover`: \"contract assumptions are jointly satisfiable\"\n\n#[test]\nfn kani_concrete_playback_h_1() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";
    const ASSERTION_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `assertion`: \"|post_state: &i64| *post_state <= 5\"\n\n#[test]\nfn kani_concrete_playback_h_2() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![8, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";

    /// Success is `Verified` only with every cover satisfied, for every obligation kind; a
    /// vacuous run is `CoverUnsatisfied`. Summaries are Kani 0.67.0's own output. The parametrized
    /// table's last case is the exact transcript measured running the IR-217 scalar harness for
    /// node 1001 under `cargo kani --unwind 16` capped at 12 GB (IR-220): CBMC's own
    /// out-of-memory abort prints the identical `VERIFICATION:- FAILED` banner a real
    /// counterexample does, so a test asserting on that banner directly -- as
    /// `bounded_kani_corpus.rs` did before IR-220 -- would report success on a run that decided
    /// zero properties. Fed to `classify_kani_run` this transcript must not be `Falsified`.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-5, TC-027
    #[test]
    fn tc_027_run_classification_never_defaults_to_verified() {
        let verified = format!(
            "SUMMARY:\n ** 0 of 43 failed\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n{COVER_PLAYBACK}"
        );
        assert_eq!(classify_kani_run(true, &verified), KaniRunOutcome::Verified);
        // Jointly unsatisfiable requires: every check succeeds, the ensures is unreachable, and
        // the cover after the contract call is unreachable. Reproduced under Kani 0.67.0.
        let vacuous = "SUMMARY:\n ** 0 of 49 failed (1 unreachable)\n\n ** 0 of 1 cover properties satisfied (1 unreachable)\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, vacuous),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            classify_kani_run(
                true,
                " ** 1 of 2 cover properties satisfied\nVERIFICATION:- SUCCESSFUL"
            ),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
        // The failure's counterexample is the assertion playback, not the cover playback.
        let falsified = format!(
            "SUMMARY:\n ** 1 of 43 failed\nFailed Checks: |post_state: &i64| *post_state <= 5\n\n ** 1 of 1 cover properties satisfied\n\nVERIFICATION:- FAILED\n{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"
        );
        assert!(matches!(
            classify_kani_run(false, &falsified),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`") && !counterexample.contains("Check for `cover`")
        ));
        for (success, text, expected) in [
            (
                false,
                format!("VERIFICATION:- FAILED\n{COVER_PLAYBACK}"),
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
            (
                false,
                "error[E0308]: mismatched types".to_owned(),
                KaniInconclusiveReason::NoVerdict,
            ),
            // Success text from a process that exited unsuccessfully is not success.
            (false, verified.clone(), KaniInconclusiveReason::NoVerdict),
            (
                true,
                "VERIFICATION:- SUCCESSFUL".to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            (
                true,
                " ** 0 of 0 cover properties satisfied\nVERIFICATION:- SUCCESSFUL".to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            (
                true,
                " ** 1 of 1 cover properties satisfied (garbage)\nVERIFICATION:- SUCCESSFUL"
                    .to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            // CBMC's own out-of-memory abort prints the identical VERIFICATION:- FAILED banner
            // a real counterexample does, with zero properties ever decided. Exact transcript
            // measured running the IR-217 scalar harness for node 1001 under `cargo kani
            // --unwind 16` capped at 12 GB (IR-220): fed to the raw-banner check this repo's
            // own bounded_kani_corpus.rs test used before this change, this transcript passes;
            // fed to classify_kani_run it must not be Falsified.
            (
                false,
                "Runtime Symex: 294.218s\n\
                 size of program expression: 980453 steps\n\
                 Generated 41018 VCC(s), 12121 remaining after simplification\n\
                 Runtime Convert SSA: 15.3149s\n\
                 Running propositional reduction\n\
                 Post-processing\n\
                 Out of memory\n\
                 \n\
                 CBMC failed with status 6\n\
                 VERIFICATION:- FAILED\n\
                 \n\
                 Manual Harness Summary:\n\
                 Verification failed for - kob_n1001::proof\n\
                 Complete - 0 successfully verified harnesses, 1 failures, 1 total.\n"
                    .to_owned(),
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
        ] {
            assert_eq!(
                classify_kani_run(success, &text),
                KaniRunOutcome::Inconclusive { reason: expected },
                "{text}"
            );
        }
    }

    /// A transcript reporting zero total checks (`** 0 of 0 failed`) alongside a satisfied 1-of-1
    /// cover and `VERIFICATION:- SUCCESSFUL` is `Inconclusive` under the `VacuousProof` reason,
    /// never `Verified`.
    ///
    /// Trace: FR-017-AC-13, TC-027
    #[test]
    fn a_zero_total_checks_summary_is_inconclusive_not_verified_even_with_every_cover_satisfied() {
        let vacuous_by_checks = "SUMMARY:\n ** 0 of 0 failed\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, vacuous_by_checks),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            },
            "zero total checks must not be Verified merely because covers were satisfied"
        );

        // A nonzero, fully-successful checks line does not trip the new gate: the existing
        // cover-based classification still governs, unchanged.
        let genuinely_verified = "SUMMARY:\n ** 0 of 5 failed\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, genuinely_verified),
            KaniRunOutcome::Verified
        );

        // A transcript with no checks-failed line at all (older or differently shaped output) is
        // unaffected: the cover-only classification still applies exactly as before.
        let no_checks_line = " ** 1 of 1 cover properties satisfied\nVERIFICATION:- SUCCESSFUL";
        assert_eq!(
            classify_kani_run(true, no_checks_line),
            KaniRunOutcome::Verified
        );
    }

    /// The checks-failed line's parenthetical is not always `unreachable`: this repository's own
    /// `tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified` fixture below carries a
    /// real `(38 undetermined)` suffix on a *different* (failed) transcript. Before this test,
    /// `checks_summary` only recognised `unreachable` — a zero-success `SUCCESS` transcript whose
    /// parenthetical instead said `undetermined` would fail to parse, silently fall through past
    /// the vacuity gate entirely, and be classified purely from the (here, satisfied) cover line
    /// as `Verified`, the exact defect agent-ix/quire-contract-codegen#99 exists to close.
    #[test]
    fn a_checks_summary_with_an_undetermined_parenthetical_still_routes_through_the_vacuity_gate() {
        let vacuous_with_undetermined_suffix = "SUMMARY:\n ** 0 of 0 failed (0 undetermined)\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, vacuous_with_undetermined_suffix),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::VacuousProof
            },
            "an `undetermined` parenthetical must not be treated as an unparseable line"
        );

        // Nonzero success checks with an `undetermined` parenthetical still passes through to
        // `Verified`, unaffected — the parenthetical is metadata, not itself a check outcome.
        let genuinely_verified_with_undetermined_suffix = "SUMMARY:\n ** 0 of 5 failed (2 undetermined)\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, genuinely_verified_with_undetermined_suffix),
            KaniRunOutcome::Verified
        );

        // A word this module does not recognise still falls through to the cover-only path,
        // exactly like having no parenthetical at all — parsing stays fail-closed rather than
        // guessing at Kani's full vocabulary.
        let unrecognised_word = "SUMMARY:\n ** 0 of 0 failed (0 somethingelse)\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_kani_run(true, unrecognised_word),
            KaniRunOutcome::Verified
        );
    }

    /// An exhausted unwind bound is inconclusive even when Kani prints a playback. The output
    /// is Kani 0.67.0's for a loop past `--unwind 4`.
    ///
    /// Trace: FR-017-AC-5, TC-027
    #[test]
    fn tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified() {
        let unwound = format!(
            "VERIFICATION RESULT:\n ** 1 of 39 failed (38 undetermined)\n\n ** 1 of 1 cover properties satisfied\n\nFailed Checks: unwinding assertion loop 0\n File: \"src/lib.rs\", line 10, in looping\n\nVERIFICATION:- FAILED\n[Kani] info: Verification output shows one or more unwinding failures.\n{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"
        );
        assert_eq!(
            classify_kani_run(false, &unwound),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
        // A succeeded unwinding check in the results listing is not a failure.
        let listed = format!(
            "Check 1: f.unwind.1\n\t - Status: SUCCESS\n\t - Description: \"unwinding assertion loop 0\"\n ** 1 of 1 cover properties satisfied\nVERIFICATION:- SUCCESSFUL\n{COVER_PLAYBACK}"
        );
        assert_eq!(classify_kani_run(true, &listed), KaniRunOutcome::Verified);
    }

    /// A scratch directory unique to this process and this call, so parallel tests never
    /// collide and nothing here touches a real `$HOME` or `$CARGO_HOME`.
    fn discover_scratch(name: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "quire-kani-discover-{name}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    /// `discover_from` is a pure function of its arguments, so every `HOME`/`CARGO_HOME`/`PATH`
    /// combination is exercised directly here with no process environment mutation.
    ///
    /// Trace: FR-017-AC-2, TC-027
    #[test]
    fn tc_027_the_launcher_resolves_through_cargo_home_then_path() {
        let directory = discover_scratch("launcher");
        let cargo_home = directory.join("cargo-home");
        let on_path = directory.join("on-path");
        fs::create_dir_all(cargo_home.join("bin")).unwrap();
        fs::create_dir_all(&on_path).unwrap();
        let launcher_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);

        // Nothing to find: refused as a missing launcher.
        let error = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.clone().into_os_string()),
        )
        .unwrap_err();
        assert!(
            matches!(
                error,
                KaniToolError::Missing {
                    tool: KaniTool::Launcher,
                    ..
                }
            ),
            "got {error}"
        );

        // Only on PATH: found there, and HOME need not be set.
        fs::write(on_path.join(&launcher_name), b"").unwrap();
        let installation = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.clone().into_os_string()),
        )
        .unwrap();
        assert_eq!(installation.launcher, on_path.join(&launcher_name));

        // `$CARGO_HOME/bin` wins over PATH.
        fs::write(cargo_home.join("bin").join(&launcher_name), b"").unwrap();
        let installation = KaniInstallation::discover_from(
            None,
            Some(cargo_home.clone().into_os_string()),
            Some(on_path.into_os_string()),
        )
        .unwrap();
        assert_eq!(
            installation.launcher,
            cargo_home.join("bin").join(&launcher_name)
        );

        let _ = fs::remove_dir_all(directory);
    }

    /// The launcher's descendants are killed when the budget elapses, not only the immediate
    /// child, reproducing the defect against a real process tree with a genuine grandchild —
    /// not the direct child under another name. `sh -c '(sh -c "echo $$ > pidfile ; exec sleep
    /// 30") & wait'` spawns an outer `sh` (the pid `run_launcher_with_timeout` itself sees) that
    /// forks a real grandchild shell in the background — a distinct pid, never `exec`'d into the
    /// outer shell — which records its own pid and then execs into `sleep 30`; the outer shell
    /// only `wait`s, so it stays alive (and stays the parent `kill_process_tree` must walk
    /// through) for the whole run. This is deliberately not `sh -c 'echo $$ > pidfile ; exec
    /// sleep 30'` run directly: there, `exec` replaces the shell in place, so the recorded pid
    /// would be the direct child's own — the one plain `child.kill()` already handles without
    /// walking `/proc` at all, which is why that version of this test stayed green when
    /// `kill_process_tree` was disabled outright (confirmed by disabling it: this version goes
    /// red, that one did not) and is not evidence the tree-walking kill does anything.
    ///
    /// The budget below is not one fixed guess: [`GRANDCHILD_KILL_TIMEOUT_LADDER`] is tried in
    /// increasing order until the grandchild is confirmed both timed out and killed. A single
    /// fixed budget cannot be correct here because `kill_process_tree` only ever reaches what
    /// its one `/proc` snapshot, taken the instant the budget elapses, can still see — the
    /// module doc says outright that a straggler which has not yet forked by then "is not
    /// guaranteed killed by this call". A short budget races that snapshot against however long
    /// the OS takes, under whatever load this test happens to run under, to actually schedule,
    /// fork and exec the grandchild, so a single fixed budget can lose that race under load
    /// without `kill_process_tree` having done anything wrong. Only every rung up to the
    /// ladder's top losing the race is a real finding about `kill_process_tree`; any earlier
    /// rung losing it is exactly the "not guaranteed" case the module doc already describes.
    ///
    /// A rung can also "lose" not because the grandchild is still running but because it died
    /// before it finished writing its own pidfile — a race at the *other* end of the same
    /// window. That is read as at least as strong evidence of a kill as finding the pidfile and
    /// then finding `/proc/<pid>` gone, not weaker, and is treated the same way: retry on an
    /// earlier rung, accept on the last.
    #[test]
    fn a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child() {
        let mut last_surviving_grandchild = None;
        for &budget in GRANDCHILD_KILL_TIMEOUT_LADDER.iter() {
            let directory = discover_scratch("launcher-timeout-grandchild");
            let pidfile = directory.join("pid");
            let mut command = Command::new("sh");
            // `sleep 30` (not the ladder's own scale) so the grandchild is still alive at
            // every rung's deadline: this test's timeout classification must come from the
            // budget elapsing, never from the grandchild finishing on its own.
            command.arg("-c").arg(format!(
                "(sh -c 'echo $$ > {} ; exec sleep 30') & wait",
                pidfile.display()
            ));
            let launch = run_launcher_with_timeout(command, budget).unwrap();
            assert!(
                matches!(launch, LaunchOutcome::TimedOut),
                "a run past its budget must classify as timed out"
            );

            // A missing, empty, or unparseable pidfile means the grandchild was killed before
            // it finished recording its own pid — informationally at least as strong as
            // confirming its pid is gone from `/proc`, not a test failure. Treat it as "not
            // surviving" and move on rather than panicking, which would escape the retry
            // ladder entirely and report a fast, successful kill as a test crash.
            let grandchild_pid: Option<i32> = fs::read_to_string(&pidfile)
                .ok()
                .and_then(|contents| contents.trim().parse().ok());
            let grandchild_survived = grandchild_pid.is_some_and(|pid| !process_gone_within(pid));

            if grandchild_survived {
                let pid = grandchild_pid.expect("survived implies a parsed pid");
                // This rung lost the race: the grandchild is still alive past the budget.
                // Kill it directly so a losing rung never leaves an orphaned `sleep 30`
                // behind, whether this is a retry or the last rung about to fail.
                let _ = Command::new("kill")
                    .args(["-KILL", &pid.to_string()])
                    .status();
                last_surviving_grandchild = Some(pid);
            } else {
                last_surviving_grandchild = None;
            }
            let _ = fs::remove_dir_all(directory);

            if !grandchild_survived {
                return;
            }
            // Survived this rung: indistinguishable, from here, between "genuinely not
            // killed" and "forked after this rung's snapshot" — retry at the next, larger
            // budget rather than assert either reading.
        }

        panic!(
            "the timed-out run's grandchild ({last_surviving_grandchild:?}) was still present \
             in `/proc` even at the largest budget in GRANDCHILD_KILL_TIMEOUT_LADDER \
             ({GRANDCHILD_KILL_TIMEOUT_LADDER:?}); kill_process_tree must be reaching every \
             descendant its own `/proc` snapshot could see, not merely losing a race against \
             scheduling delay"
        );
    }

    /// Whether `pid` is dead, waiting up to [`GRANDCHILD_REAP_WAIT`] for it to become so.
    ///
    /// Two facts about a SIGKILLed process make a bare `/proc/<pid>` existence check wrong.
    /// The kill is asynchronous: `kill_process_tree` has returned once the signal is sent, not
    /// once the target has stopped running, so the process can still read as running for a few
    /// milliseconds. And the grandchild is reparented to init when its parent dies, which
    /// reaps it in its own time (measured here: PID 1 left killed grandchildren as zombies
    /// for seconds), so `/proc/<pid>` outlives a process that is already dead. Dead means
    /// absent or in state `Z`/`X`; anything else after the wait is a survivor.
    fn process_gone_within(pid: i32) -> bool {
        let deadline = Instant::now() + GRANDCHILD_REAP_WAIT;
        loop {
            let state = fs::read_to_string(format!("/proc/{pid}/stat"))
                .ok()
                .and_then(|stat| {
                    stat.rsplit_once(')')
                        .and_then(|(_, rest)| rest.trim_start().chars().next())
                });
            if matches!(state, None | Some('Z') | Some('X')) {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// How long a delivered SIGKILL is given to take effect before its target counts as a
    /// survivor. Far longer than signal delivery takes; a process still running after it was
    /// not killed.
    const GRANDCHILD_REAP_WAIT: Duration = Duration::from_secs(2);

    /// Successive budgets [`a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`]
    /// tries, smallest first: the common case (an unloaded or lightly loaded run) settles on
    /// the first, cheap rung, and only a run actually contending for CPU climbs further. The
    /// top rung is a generous safety margin chosen as a judgment call, not a number backed by
    /// direct local measurement — this fix's own local validation environment could not
    /// reproduce the original flake at all — sized so that exhausting it is a real finding
    /// about `kill_process_tree`, not an unlucky scheduling instant.
    const GRANDCHILD_KILL_TIMEOUT_LADDER: [Duration; 5] = [
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(5),
    ];

    /// A process the launcher forks (or that reparents to it) in the instant before this call's
    /// `/proc` snapshot-and-kill sweep survives that sweep and keeps its inherited copy of the
    /// pipes' write end open — reproduced directly with the reviewer's own case: `( sleep 45 &
    /// )` backgrounds and immediately orphans a `sleep 45` (its parent, the subshell, exits at
    /// once), while `exec sleep 45` replaces the outer shell — this call's own direct child —
    /// with a second `sleep 45` that `kill_process_tree` does reach and kill. If this function
    /// waited for the stdout/stderr reader threads to see EOF on every timeout, it would block
    /// on the orphan's still-open copy of the pipe until that `sleep 45` finished on its own —
    /// unbounded, and exactly the defect #58 exists to remove, back again behind the correct
    /// typed result. This must return in about 200ms, nowhere near the orphan's own 45s.
    #[test]
    fn a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("( sleep 45 & ) ; exec sleep 45");
        let started = Instant::now();
        let outcome = run_launcher_with_timeout(command, Duration::from_millis(200)).unwrap();
        assert!(
            matches!(outcome, LaunchOutcome::TimedOut),
            "a run past its budget must classify as timed out"
        );
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "a 200ms budget must not take anywhere near the orphaned process's own 45s sleep: \
             took {:?}",
            started.elapsed()
        );
    }

    /// A timed-out launch maps to no exit code and `Inconclusive { reason: TimedOut }`.
    #[test]
    fn a_timed_out_launch_carries_no_exit_code_into_the_evidence() {
        let (outcome, exit_code) = launch_evidence(LaunchOutcome::TimedOut);
        assert_eq!(exit_code, None);
        assert_eq!(
            outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );
    }

    /// Regression coverage for the polling/draining plumbing `run_launcher_with_timeout` added:
    /// a process that exits within its budget still reports its real exit status and combined
    /// output, unchanged from what `Command::output()` used to hand back directly. No criterion
    /// traces this specifically — the Kani lane's real `cargo-kani` runs
    /// (`tests/kani_obligations.rs`) already exercise this completed path end to end; this is
    /// only faster, hermetic coverage of the same plumbing.
    #[test]
    fn a_run_finishing_within_its_budget_reports_its_own_exit_status_and_output() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("printf out; printf err 1>&2; exit 3");
        let outcome = run_launcher_with_timeout(command, Duration::from_secs(5)).unwrap();
        match outcome {
            LaunchOutcome::Completed {
                exited_successfully,
                exit_code,
                text,
            } => {
                assert!(!exited_successfully);
                assert_eq!(exit_code, Some(3));
                assert!(text.contains("out"));
                assert!(text.contains("err"));
            }
            LaunchOutcome::TimedOut => panic!("a fast process must not be reported as timed out"),
        }
    }
}
