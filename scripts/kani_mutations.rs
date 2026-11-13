//! `kani-mutations`: inject each declared defect into a scratch copy of the working directory and
//! require the owning proof to reject it (FR-055).

use std::{
    env, fs,
    fs::File,
    io,
    os::unix::{fs::symlink, process::CommandExt},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde::Deserialize;

/// Fewer declared mutants than this are refused before any run.
const FLOOR: usize = 3;
const REJECTION_MARKER: &str = "VERIFICATION:- FAILED";
const POLL: Duration = Duration::from_millis(50);
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
    let available = kani_on_path();
    let start = Instant::now();
    let mut outcomes = Vec::with_capacity(mutants.len());
    for mutant in &mutants {
        let (outcome, detail) = if available {
            evaluate(mutant, &config, start)
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

fn evaluate(mutant: &Mutant, config: &Config, start: Instant) -> (Outcome, String) {
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
    match inject_and_run(mutant, &text, config, config.mutant_timeout.min(remaining)) {
        Ok(Run::Exited { success, output }) => {
            if success {
                (
                    Outcome::Fail,
                    "the proof verified the injected defect".into(),
                )
            } else if output.contains(REJECTION_MARKER) {
                (Outcome::Pass, String::new())
            } else {
                (
                    Outcome::Fail,
                    format!("the run failed without {REJECTION_MARKER}"),
                )
            }
        }
        Ok(Run::TimedOut) => (Outcome::Fail, "the run timed out".into()),
        Err(error) => (Outcome::Fail, format!("could not run the mutant: {error}")),
    }
}

enum Run {
    Exited { success: bool, output: String },
    TimedOut,
}

fn inject_and_run(
    mutant: &Mutant,
    text: &str,
    config: &Config,
    limit: Duration,
) -> io::Result<Run> {
    let scratch = Scratch::new()?;
    copy_tree(Path::new("."), scratch.path())?;
    // A symlinked source would write through to the working tree; replace the link itself.
    let target = scratch.path().join(&mutant.file);
    fs::remove_file(&target)?;
    fs::write(
        &target,
        text.replacen(&mutant.anchor, &mutant.replacement, 1),
    )?;

    let log_path = scratch.path().join(".kani-mutations.log");
    let log = File::create(&log_path)?;
    let mut command = Command::new(&config.command[0]);
    command
        .args(&config.command[1..])
        .arg(&mutant.proof)
        .current_dir(scratch.path())
        .stdin(Stdio::null())
        .stdout(log.try_clone()?)
        .stderr(log)
        .process_group(0);
    let mut child = command.spawn()?;
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = child.try_wait()? {
            let output = String::from_utf8_lossy(&fs::read(&log_path)?).into_owned();
            return Ok(Run::Exited {
                success: status.success(),
                output,
            });
        }
        if Instant::now() >= deadline {
            kill_group(&child);
            child.wait()?;
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

fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_text = name.to_string_lossy();
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
            copy_tree(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), &destination)?;
        }
    }
    Ok(())
}

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> io::Result<Self> {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
