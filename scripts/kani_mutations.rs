//! `kani-mutations`: inject each declared defect into a scratch copy of the working directory and
//! require the owning proof to reject it (FR-055).

use std::{
    env, fs,
    fs::File,
    io,
    os::unix::{fs::symlink, process::CommandExt},
    path::{Component, Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

use serde::Deserialize;

/// Fewer declared mutants than this are refused before any run.
const FLOOR: usize = 3;
const REJECTION_MARKER: &str = "VERIFICATION:- FAILED";
const POLL: Duration = Duration::from_millis(50);
/// Set by SIGINT, SIGTERM and SIGHUP: the run stops, kills the child's process group and removes
/// its scratch directory instead of dying with the scratch on disk and Kani still running.
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_signal: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

fn install_signal_handlers() {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        // SAFETY: `on_signal` only stores to an atomic, which is async-signal-safe, and has the
        // `extern "C" fn(c_int)` signature `signal(2)` expects.
        unsafe {
            libc::signal(
                signal,
                on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t,
            );
        }
    }
}

/// Directories that are build output or version control, never part of the candidate.
const SKIPPED: &[&str] = &[".git", ".worktrees", "node_modules"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Mutant {
    file: String,
    anchor: String,
    replacement: String,
    proof: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Pass,
    Fail,
    Malformed,
    Unavailable,
}

impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Malformed => "malformed",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Why the target did not finish a run: exit status 2 is a refusal, 1 any other failure.
pub enum Stop {
    Refused(String),
    Failed(String),
}

struct Config {
    declaration: PathBuf,
    mutant_timeout: Duration,
    budget: Duration,
    command: Vec<String>,
}

fn parse_args(args: &[String]) -> Result<Config, Stop> {
    let usage = || {
        Stop::Failed("usage: cg_gates kani-mutations DECLARATION MUTANT_TIMEOUT_SECS BUDGET_SECS -- CMD [ARG...]".into())
    };
    let [declaration, timeout, budget, separator, command @ ..] = args else {
        return Err(usage());
    };
    if separator != "--" || command.is_empty() {
        return Err(usage());
    }
    let seconds = |text: &String| {
        text.parse::<u64>()
            .map(Duration::from_secs)
            .map_err(|_| usage())
    };
    Ok(Config {
        declaration: PathBuf::from(declaration),
        mutant_timeout: seconds(timeout)?,
        budget: seconds(budget)?,
        command: command.to_vec(),
    })
}

pub fn run(args: &[String]) -> Result<(), Stop> {
    let config = parse_args(args)?;
    let text = fs::read_to_string(&config.declaration)
        .map_err(|error| Stop::Failed(format!("{}: {error}", config.declaration.display())))?;
    let mutants: Vec<Mutant> = serde_json::from_str(&text)
        .map_err(|error| Stop::Failed(format!("{}: {error}", config.declaration.display())))?;
    if mutants.len() < FLOOR {
        return Err(Stop::Refused(format!(
            "kani-mutations: refused: {} declared mutants, the floor is {FLOOR}",
            mutants.len()
        )));
    }
    install_signal_handlers();
    // One scratch root per run: every mutant's tree is rebuilt at `tree`, and they all share the
    // build cache in `target`, so only the first mutant pays for the dependencies.
    let scratch =
        Scratch::new().map_err(|error| Stop::Failed(format!("scratch directory: {error}")))?;
    let available = kani_on_path();
    let start = Instant::now();
    let mut outcomes = Vec::with_capacity(mutants.len());
    for mutant in &mutants {
        let (outcome, detail) = if available {
            evaluate(mutant, &config, start, &scratch)
        } else {
            (Outcome::Unavailable, "cargo-kani is not on PATH".into())
        };
        println!(
            "kani-mutations: harness={} mutant={}:{:?} outcome={}",
            mutant.proof,
            mutant.file,
            mutant.anchor.lines().next().unwrap_or_default().trim(),
            outcome.name()
        );
        if outcome != Outcome::Pass {
            eprintln!("kani-mutations: {}: {detail}", mutant.proof);
        }
        outcomes.push(outcome);
    }
    let count = |wanted| {
        outcomes
            .iter()
            .filter(|outcome| **outcome == wanted)
            .count()
    };
    println!(
        "kani-mutations: total={} pass={} fail={} malformed={} unavailable={}",
        outcomes.len(),
        count(Outcome::Pass),
        count(Outcome::Fail),
        count(Outcome::Malformed),
        count(Outcome::Unavailable)
    );
    if INTERRUPTED.load(Ordering::SeqCst) {
        return Err(Stop::Failed("kani-mutations: interrupted".into()));
    }
    if count(Outcome::Pass) == outcomes.len() {
        Ok(())
    } else {
        Err(Stop::Failed(
            "kani-mutations: not every mutant was rejected".into(),
        ))
    }
}

fn kani_on_path() -> bool {
    use std::os::unix::fs::PermissionsExt;
    env::var_os("PATH").is_some_and(|path| {
        env::split_paths(&path).any(|dir| {
            fs::metadata(dir.join("cargo-kani"))
                .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
        })
    })
}

fn evaluate(
    mutant: &Mutant,
    config: &Config,
    start: Instant,
    scratch: &Scratch,
) -> (Outcome, String) {
    let source = Path::new(&mutant.file);
    if source.is_absolute()
        || source
            .components()
            .any(|c| matches!(c, Component::ParentDir))
    {
        return (
            Outcome::Malformed,
            format!("{} is not a path inside the repository", mutant.file),
        );
    }
    if has_symlink_component(source) {
        return (
            Outcome::Malformed,
            format!("{} passes through a symbolic link", mutant.file),
        );
    }
    let Ok(text) = fs::read_to_string(source) else {
        return (
            Outcome::Malformed,
            format!("{} is not readable", mutant.file),
        );
    };
    let occurrences = if mutant.anchor.is_empty() {
        0
    } else {
        text.matches(&mutant.anchor).count()
    };
    if occurrences != 1 {
        return (
            Outcome::Malformed,
            format!(
                "anchor occurs {occurrences} times in {}, not once",
                mutant.file
            ),
        );
    }
    if INTERRUPTED.load(Ordering::SeqCst) {
        return (Outcome::Fail, "interrupted before this mutant ran".into());
    }
    let Some(remaining) = config
        .budget
        .checked_sub(start.elapsed())
        .filter(|d| !d.is_zero())
    else {
        return (
            Outcome::Fail,
            "total wall-clock budget exhausted before this mutant ran".into(),
        );
    };
    match inject_and_run(
        mutant,
        &text,
        config,
        scratch,
        config.mutant_timeout.min(remaining),
    ) {
        Ok(Run::Exited { status, output }) => {
            if status.success() {
                (Outcome::Fail, explain_failure(&status, &output))
            } else if output.contains(REJECTION_MARKER) {
                (Outcome::Pass, String::new())
            } else {
                (Outcome::Fail, explain_failure(&status, &output))
            }
        }
        Ok(Run::TimedOut) => (Outcome::Fail, "the run timed out".into()),
        Ok(Run::Interrupted) => (Outcome::Fail, "the run was interrupted".into()),
        Err(error) => (Outcome::Fail, format!("could not run the mutant: {error}")),
    }
}

enum Run {
    Exited { status: ExitStatus, output: String },
    TimedOut,
    Interrupted,
}

/// Lines of the owning test's captured output a `fail` row reports.
const TAIL_LINES: usize = 40;

/// Why a mutant run that exited was not a rejection: which of the four causes it was, the exit
/// status and the last lines of the captured output.
fn explain_failure(status: &ExitStatus, output: &str) -> String {
    let cause = if status.success() {
        if output.contains("running 0 tests") {
            "the test filter matched no test, so nothing ran"
        } else {
            "the proof verified the injected defect (a surviving mutant)"
        }
    } else if output.contains("could not compile") || output.contains("error[E") {
        "the mutant did not compile"
    } else if output.contains("panicked at") || output.contains("test result: FAILED") {
        "the test ran and failed without Kani's rejection verdict"
    } else {
        "the run failed before the owning test ran, or without any test output"
    };
    let lines: Vec<&str> = output.lines().collect();
    let tail = lines[lines.len().saturating_sub(TAIL_LINES)..].join("\n");
    format!(
        "{cause}; the run exited {status} without {REJECTION_MARKER}; last {TAIL_LINES} lines of its output:\n{tail}"
    )
}

fn has_symlink_component(path: &Path) -> bool {
    let mut walked = PathBuf::new();
    path.components().any(|component| {
        walked.push(component);
        fs::symlink_metadata(&walked).is_ok_and(|meta| meta.file_type().is_symlink())
    })
}

fn inject_and_run(
    mutant: &Mutant,
    text: &str,
    config: &Config,
    scratch: &Scratch,
    limit: Duration,
) -> io::Result<Run> {
    let tree = scratch.path().join("tree");
    let _ = fs::remove_dir_all(&tree);
    fs::create_dir(&tree)?;
    copy_tree(Path::new("."), &tree, scratch.path())?;
    // A symlinked source would write through to the working tree; replace the link itself.
    let target = tree.join(&mutant.file);
    fs::remove_file(&target)?;
    fs::write(
        &target,
        text.replacen(&mutant.anchor, &mutant.replacement, 1),
    )?;

    let log_path = scratch.path().join("run.log");
    let log = File::create(&log_path)?;
    let mut command = Command::new(&config.command[0]);
    command
        .args(&config.command[1..])
        .arg(&mutant.proof)
        .current_dir(&tree)
        .env("CARGO_TARGET_DIR", scratch.path().join("target"))
        .stdin(Stdio::null())
        .stdout(log.try_clone()?)
        .stderr(log)
        .process_group(0);
    let mut child = command.spawn()?;
    let result = wait_for(&mut child, &log_path, Instant::now() + limit);
    if !matches!(result, Ok(Run::Exited { .. })) {
        // Every way out other than a normal exit (timeout, signal, I/O error) ends the whole group.
        kill_group(&child);
        let _ = child.wait();
    }
    result
}

fn wait_for(
    child: &mut std::process::Child,
    log_path: &Path,
    deadline: Instant,
) -> io::Result<Run> {
    loop {
        if let Some(status) = child.try_wait()? {
            let output = String::from_utf8_lossy(&fs::read(log_path)?).into_owned();
            return Ok(Run::Exited { status, output });
        }
        if INTERRUPTED.load(Ordering::SeqCst) {
            return Ok(Run::Interrupted);
        }
        if Instant::now() >= deadline {
            return Ok(Run::TimedOut);
        }
        thread::sleep(POLL);
    }
}

fn kill_group(child: &std::process::Child) {
    let Some(pid) = i32::try_from(child.id())
        .ok()
        .and_then(rustix::process::Pid::from_raw)
    else {
        return;
    };
    // The child leads its own group, so this reaches cargo, rustc and Kani together.
    let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
}

fn copy_tree(from: &Path, to: &Path, skip: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_text = name.to_string_lossy();
        if fs::canonicalize(entry.path()).is_ok_and(|path| path == skip) {
            continue;
        }
        let is_build_dir = name_text.starts_with("target") || name_text.ends_with("-target");
        if SKIPPED.contains(&name_text.as_ref()) || (is_build_dir && entry.path().is_dir()) {
            continue;
        }
        let destination = to.join(&name);
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            symlink(fs::read_link(entry.path())?, &destination)?;
        } else if kind.is_dir() {
            fs::create_dir(&destination)?;
            copy_tree(&entry.path(), &destination, skip)?;
        } else {
            fs::copy(entry.path(), &destination)?;
        }
    }
    Ok(())
}

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> io::Result<Self> {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!("cg-kani-mutations-{}-{id}", std::process::id()));
        fs::create_dir(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
