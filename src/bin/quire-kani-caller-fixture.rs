//! Packaged TC-049 caller: normal public execution, or the sole explicitly enabled fixture item.

use quire_contract_codegen::{
    execute_kani_obligation, Artifact, KaniExecutableHarness, KaniExecutionRequest,
    KaniInstallation, StateFrameHarness, StateFrameIdentity,
};
use serde::Deserialize;
use std::{
    fs,
    io::{self, Read},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceArtifact {
    path: String,
    content: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicRun {
    identity: StateFrameIdentity,
    rust: SourceArtifact,
    record: SourceArtifact,
    launcher: PathBuf,
    crate_directory: PathBuf,
    target_directory: PathBuf,
}

#[cfg(feature = "guardian-test-support")]
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum PrivateScenario {
    ExactDeath {
        prefix: quire_contract_codegen::GuardianFixturePrefix,
        death: quire_contract_codegen::GuardianFixtureDeath,
    },
    PendingDispatch {
        backend_marker: PathBuf,
    },
    Dispatched {
        backend_marker: PathBuf,
        worker_acknowledgement: PathBuf,
    },
}

#[cfg(feature = "guardian-test-support")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateRun {
    program: std::ffi::OsString,
    arguments: Vec<std::ffi::OsString>,
    directory: PathBuf,
    environment: Vec<(std::ffi::OsString, std::ffi::OsString)>,
    report_path: PathBuf,
    ceilings: quire_contract_codegen::ProofCeilings,
    scenario: PrivateScenario,
}

#[derive(Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum Invocation {
    Public {
        guardian_path: PathBuf,
        run: PublicRun,
    },
    #[cfg(feature = "guardian-test-support")]
    Private {
        guardian_path: PathBuf,
        run: PrivateRun,
    },
}

fn read_invocation(path: &Path) -> Result<Invocation, io::Error> {
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(
            i32::try_from((rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::NONBLOCK).bits())
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "fixture open flags exceed range",
                    )
                })?,
        )
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "fixture invocation must be regular",
        ));
    }
    let mut bytes = Vec::new();
    file.take(1_048_577).read_to_end(&mut bytes)?;
    if bytes.len() > 1_048_576 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "fixture invocation exceeds bound",
        ));
    }
    serde_json::from_slice(&bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn run() -> Result<(), String> {
    let started = Instant::now();
    // This packaged caller exercises ordinary closed-at-exec stdin through actual descriptor
    // flags. The public execution API captures it internally; no request can override stdin.
    rustix::io::fcntl_setfd(std::io::stdin(), rustix::io::FdFlags::CLOEXEC)
        .map_err(|error| error.to_string())?;
    // This process owns its dedicated session/group before any monitor can exist.
    #[cfg(target_os = "linux")]
    rustix::process::setsid().map_err(|error| error.to_string())?;
    let mut arguments = std::env::args_os().skip(1);
    let invocation = arguments.next().ok_or("expected one invocation document")?;
    if arguments.next().is_some() {
        return Err("expected one invocation document".to_owned());
    }
    let invocation = read_invocation(Path::new(&invocation)).map_err(|error| error.to_string())?;
    match invocation {
        Invocation::Public { guardian_path, run } => {
            let harness = StateFrameHarness {
                identity: run.identity,
                rust: Artifact::new(run.rust.path, run.rust.content),
                record: Artifact::new(run.record.path, run.record.content),
            };
            execute_kani_obligation(&KaniExecutionRequest {
                installation: &KaniInstallation {
                    launcher: run.launcher,
                },
                guardian_path: &guardian_path,
                harness: KaniExecutableHarness::from(&harness),
                crate_directory: &run.crate_directory,
                target_directory: &run.target_directory,
            })
            .map_err(|error| error.to_string())?;
        }
        #[cfg(feature = "guardian-test-support")]
        Invocation::Private { guardian_path, run } => {
            use quire_contract_codegen::{
                observe_guardian_fixture, GuardianFixtureRequest, GuardianFixtureScenario,
            };
            let deadline = started
                .checked_add(run.ceilings.wall_clock)
                .ok_or("fixture deadline exceeds monotonic range")?;
            let scenario = match run.scenario {
                PrivateScenario::ExactDeath { prefix, death } => {
                    GuardianFixtureScenario::ExactDeath { prefix, death }
                }
                PrivateScenario::PendingDispatch { backend_marker } => {
                    GuardianFixtureScenario::PendingDispatch { backend_marker }
                }
                PrivateScenario::Dispatched {
                    backend_marker,
                    worker_acknowledgement,
                } => GuardianFixtureScenario::Dispatched {
                    backend_marker,
                    worker_acknowledgement,
                },
            };
            let observation = observe_guardian_fixture(GuardianFixtureRequest {
                guardian_path: &guardian_path,
                program: &run.program,
                arguments: &run.arguments,
                directory: &run.directory,
                environment: &run.environment,
                report_path: &run.report_path,
                deadline,
                ceilings: run.ceilings,
                harnesses: std::num::NonZeroUsize::MIN,
                scenario,
            })
            .map_err(|error| error.to_string())?;
            // Reporting belongs only to the packaged caller; the library returns raw facts and
            // has already performed unchanged cleanup. The harness evaluates every predicate.
            serde_json::to_writer(std::io::stdout(), &observation)
                .map_err(|error| error.to_string())?;
        }
    }
    #[cfg(not(feature = "guardian-test-support"))]
    let _ = started;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("caller fixture refused: {error}");
            ExitCode::FAILURE
        }
    }
}
