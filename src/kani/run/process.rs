//! Shared bounded child execution. Backend-specific interpretation stays with the caller.

use std::{
    ffi::OsString,
    fmt, io,
    num::{NonZeroU64, NonZeroUsize},
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::kani::{
    identity::ProofCeilings,
    run::launch::{
        failure_phase, run_bounded_process_internal, BoundedLaunchError, CaptureStream,
        LaunchFailurePhase, LaunchOutcome,
    },
};
use quire_contract_ir::kani::KaniCancellationSource;
use quire_exact::{Cancel, CancelCause};

/// Finite limits for one direct child invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessLimits {
    /// Maximum query bytes supplied on stdin.
    pub input_bytes: NonZeroUsize,
    /// Maximum elapsed execution time.
    pub wall_clock: Duration,
    /// Maximum observed resident memory of the owned process tree.
    pub memory_bytes: NonZeroU64,
    /// Maximum bytes retained from each of stdout and stderr.
    pub output_bytes: NonZeroUsize,
}

/// A direct child invocation, with no shell or command-string interpretation.
#[derive(Debug)]
pub struct ProcessRequest {
    /// Absolute selected executable path.
    pub executable: PathBuf,
    /// Arguments passed unchanged to the executable.
    pub arguments: Vec<OsString>,
    /// Exact input bytes passed on stdin.
    pub stdin: Vec<u8>,
    /// Caller-selected per-invocation limits.
    pub limits: ProcessLimits,
    /// Caller-owned cancellation handle, shared with the invocation.
    pub cancel: Cancel,
}

/// Why a bounded child finished or refused; output is never truncated into a success.
#[derive(Debug)]
pub enum ProcessOutcome {
    /// The selected executable was absent before spawn.
    MissingTool,
    /// Input exceeded the caller's limit before spawn.
    InputOverLimit,
    /// Child exited after consuming the complete supplied input.
    Completed {
        /// Whether the child reported a successful exit.
        success: bool,
        /// Numeric exit status when the operating system supplies one.
        exit_code: Option<i32>,
    },
    /// The deadline was reached.
    TimedOut,
    /// The process tree crossed the memory limit.
    MemoryExhausted,
    /// The process tree could not be observed or its cleanup confirmed.
    MemoryUnobserved {
        /// Why observation or owned cleanup could not be confirmed.
        detail: String,
    },
    /// One output stream exceeded its limit.
    OutputOverLimit {
        /// The stream that exceeded its limit.
        stream: CaptureStream,
    },
    /// One output stream could not be read whole.
    OutputUnread {
        /// The unread stream.
        stream: CaptureStream,
        /// Why the stream could not be read whole.
        detail: String,
    },
    /// Exact stdin bytes could not be written whole.
    InputUnread {
        /// Why stdin could not be written whole.
        detail: String,
    },
    /// A caller-owned cancellation stopped the child.
    Cancelled {
        /// The caller's retained first cancellation cause.
        source: CancelCause,
    },
}

/// Selected invocation and actual outcome, including whether a child started.
#[derive(Debug)]
pub struct ProcessRun {
    /// Selected executable path.
    pub executable: PathBuf,
    /// Selected argument vector.
    pub arguments: Vec<OsString>,
    /// Limits actually applied to this invocation.
    pub limits: ProcessLimits,
    /// Whether a child was spawned before the outcome.
    pub started: bool,
    /// Bounded raw stdout, including a prefix on a stopped or over-limit run; absent if unread.
    pub stdout: Option<Vec<u8>>,
    /// Bounded raw stderr, including a prefix on a stopped or over-limit run; absent if unread.
    pub stderr: Option<Vec<u8>>,
    /// Typed completion or refusal.
    pub outcome: ProcessOutcome,
}

/// The operation that failed before a trustworthy child outcome was available.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessFailurePhase {
    /// The caller selected an invalid executable path.
    Selection,
    /// Direct child spawn failed.
    Spawn,
    /// The owned child could not be attached to its supervisor.
    Ownership,
    /// Captured pipes could not be acquired.
    CaptureSetup,
    /// Owned namespace startup failed.
    Startup,
    /// Child completion or reaping could not be confirmed.
    Wait,
    /// A process operation failed outside a named phase.
    Other,
}

/// A launch mechanism failure that prevents a trustworthy child outcome.
#[derive(Debug)]
pub enum ProcessError {
    /// A required namespace or memory mechanism is unavailable.
    Unavailable(io::Error),
    /// Another process operation failed before a trustworthy outcome was available.
    Io {
        /// The operation that failed.
        phase: ProcessFailurePhase,
        /// The original operating-system error.
        source: io::Error,
    },
}

impl fmt::Display for ProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(source) => {
                write!(formatter, "bounded process mechanism unavailable: {source}")
            }
            Self::Io { phase, source } => {
                write!(formatter, "bounded process {phase:?} failed: {source}")
            }
        }
    }
}

impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unavailable(source) | Self::Io { source, .. } => Some(source),
        }
    }
}

/// Execute one bounded child with exact stdin and separate output streams.
pub fn run_bounded_process(request: ProcessRequest) -> Result<ProcessRun, ProcessError> {
    let ProcessRequest {
        executable,
        arguments,
        stdin,
        limits,
        cancel,
    } = request;
    let finish = |started, outcome, stdout, stderr| ProcessRun {
        executable: executable.clone(),
        arguments: arguments.clone(),
        limits,
        started,
        stdout,
        stderr,
        outcome,
    };
    if stdin.len() > limits.input_bytes.get() {
        return Ok(finish(false, ProcessOutcome::InputOverLimit, None, None));
    }
    if !executable.is_absolute() {
        return Err(ProcessError::Io {
            phase: ProcessFailurePhase::Selection,
            source: io::Error::new(
                io::ErrorKind::InvalidInput,
                "selected executable must have an absolute path",
            ),
        });
    }
    match std::fs::metadata(&executable) {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => {
            return Err(ProcessError::Io {
                phase: ProcessFailurePhase::Selection,
                source: io::Error::other("selected executable is not a file"),
            })
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(finish(false, ProcessOutcome::MissingTool, None, None));
        }
        Err(error) => {
            return Err(ProcessError::Io {
                phase: ProcessFailurePhase::Selection,
                source: error,
            })
        }
    }
    let deadline = Instant::now()
        .checked_add(limits.wall_clock)
        .ok_or_else(|| ProcessError::Io {
            phase: ProcessFailurePhase::Selection,
            source: io::Error::new(
                io::ErrorKind::InvalidInput,
                "wall-clock limit cannot be represented",
            ),
        })?;
    let run = run_bounded_process_internal(
        &executable,
        &arguments,
        stdin,
        ProofCeilings {
            memory_bytes: limits.memory_bytes,
            wall_clock: limits.wall_clock,
        },
        limits.output_bytes.get(),
        deadline,
        &cancel,
    )
    .map_err(|error| match error {
        BoundedLaunchError::Unavailable(error) => ProcessError::Unavailable(error),
        BoundedLaunchError::Io(error) => ProcessError::Io {
            phase: match failure_phase(&error) {
                Some(LaunchFailurePhase::Spawn) => ProcessFailurePhase::Spawn,
                Some(LaunchFailurePhase::Ownership) => ProcessFailurePhase::Ownership,
                Some(LaunchFailurePhase::CaptureSetup) => ProcessFailurePhase::CaptureSetup,
                Some(LaunchFailurePhase::Startup) => ProcessFailurePhase::Startup,
                Some(LaunchFailurePhase::Wait) => ProcessFailurePhase::Wait,
                None => ProcessFailurePhase::Other,
            },
            source: error,
        },
    })?;
    let (outcome, completed_streams) = match (run.outcome, run.input_failure) {
        (
            LaunchOutcome::Completed {
                exited_successfully,
                exit_code,
                streams,
            },
            input_failure,
        ) => (
            match input_failure {
                Some(detail) => ProcessOutcome::InputUnread { detail },
                None => ProcessOutcome::Completed {
                    success: exited_successfully,
                    exit_code,
                },
            },
            Some(streams),
        ),
        (LaunchOutcome::MemoryExhausted, _) => (ProcessOutcome::MemoryExhausted, None),
        (LaunchOutcome::MemoryUnobserved { detail }, _) => {
            (ProcessOutcome::MemoryUnobserved { detail }, None)
        }
        (LaunchOutcome::OutputOverLimit { stream, .. }, _) => {
            (ProcessOutcome::OutputOverLimit { stream }, None)
        }
        (LaunchOutcome::OutputUnread { stream, detail }, _) => {
            (ProcessOutcome::OutputUnread { stream, detail }, None)
        }
        (LaunchOutcome::Cancelled { source }, _) => (
            ProcessOutcome::Cancelled {
                source: match source {
                    KaniCancellationSource::Requested => CancelCause::Requested,
                    KaniCancellationSource::Deadline => CancelCause::Deadline,
                },
            },
            None,
        ),
        (LaunchOutcome::TimedOut, Some(detail)) => (ProcessOutcome::InputUnread { detail }, None),
        (LaunchOutcome::TimedOut, None) => (ProcessOutcome::TimedOut, None),
    };
    let (stdout, stderr) = match completed_streams {
        Some(streams) => (Some(streams.stdout), Some(streams.stderr)),
        None => (run.partial_stdout, run.partial_stderr),
    };
    Ok(finish(run.started, outcome, stdout, stderr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    fn limits(input_bytes: usize, output_bytes: usize) -> ProcessLimits {
        ProcessLimits {
            input_bytes: NonZeroUsize::new(input_bytes).unwrap(),
            wall_clock: Duration::from_secs(5),
            memory_bytes: NonZeroU64::new(128 * 1024 * 1024).unwrap(),
            output_bytes: NonZeroUsize::new(output_bytes).unwrap(),
        }
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn exact_input_reaches_one_direct_child_and_returns_whole_output() {
        let bytes = b"(check-sat)\n(with spaces)\0\n".repeat(8_000);
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/cat".into(),
            arguments: vec!["-".into()],
            stdin: bytes.clone(),
            limits: limits(256 * 1024, 256 * 1024),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert_eq!(run.executable, PathBuf::from("/usr/bin/cat"));
        assert_eq!(run.arguments, vec![OsString::from("-")]);
        assert_eq!(run.limits, limits(256 * 1024, 256 * 1024));
        assert_eq!(run.stdout.as_deref(), Some(bytes.as_slice()));
        assert_eq!(run.stderr.as_deref(), Some(b"".as_slice()));
        match run.outcome {
            ProcessOutcome::Completed { success, exit_code } => {
                assert!(success);
                assert_eq!(exit_code, Some(0));
            }
            other => panic!("expected complete exact input, got {other:?}"),
        }
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn input_excess_refuses_before_selected_program_can_start() {
        let marker = std::env::temp_dir().join(format!("cg-ir974-input-{}", std::process::id()));
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/touch".into(),
            arguments: vec![marker.as_os_str().to_owned()],
            stdin: vec![b'x'; 5],
            limits: limits(4, 1024),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(!run.started);
        assert!(matches!(run.outcome, ProcessOutcome::InputOverLimit));
        assert!(!marker.exists());
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn missing_selected_executable_refuses_without_starting_child() {
        let run = run_bounded_process(ProcessRequest {
            executable: "/nonexistent/cg-ir974-solver".into(),
            arguments: vec![],
            stdin: b"(check-sat)".to_vec(),
            limits: limits(1024, 1024),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(!run.started);
        assert!(matches!(run.outcome, ProcessOutcome::MissingTool));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn relative_executable_is_a_typed_selection_error() {
        let error = run_bounded_process(ProcessRequest {
            executable: "solver".into(),
            arguments: vec![],
            stdin: vec![],
            limits: limits(1, 64),
            cancel: Cancel::new(),
        })
        .unwrap_err();
        assert!(
            matches!(error, ProcessError::Io { phase: ProcessFailurePhase::Selection, source }
            if source.kind() == io::ErrorKind::InvalidInput)
        );
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn output_excess_stops_with_only_a_bounded_raw_prefix() {
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/yes".into(),
            arguments: vec![],
            stdin: vec![],
            limits: limits(1, 64),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert!(matches!(
            run.outcome,
            ProcessOutcome::OutputOverLimit {
                stream: CaptureStream::Stdout
            }
        ));
        assert_eq!(run.stdout.as_ref().map(Vec::len), Some(64));
        assert!(run
            .stdout
            .as_ref()
            .is_some_and(|bytes| bytes.starts_with(b"y\n")));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn caller_cancellation_refuses_before_selected_program_starts() {
        let cancel = Cancel::new();
        cancel.cancel(CancelCause::Requested);
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/touch".into(),
            arguments: vec![],
            stdin: vec![],
            limits: limits(1, 64),
            cancel,
        })
        .unwrap();
        assert!(!run.started);
        assert!(matches!(
            run.outcome,
            ProcessOutcome::Cancelled {
                source: CancelCause::Requested
            }
        ));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn nonzero_exit_keeps_status_and_whole_captured_streams() {
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/false".into(),
            arguments: vec![],
            stdin: vec![],
            limits: limits(1, 64),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert!(matches!(
            run.outcome,
            ProcessOutcome::Completed {
                success: false,
                exit_code: Some(1),
                ..
            }
        ));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn child_exit_before_consuming_input_refuses_as_unread_stdin() {
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/false".into(),
            arguments: vec![],
            stdin: vec![b'x'; 256 * 1024],
            limits: limits(256 * 1024, 64),
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert!(matches!(run.outcome, ProcessOutcome::InputUnread { .. }));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn running_child_stops_at_its_wall_clock_limit() {
        let mut bounds = limits(1, 64);
        bounds.wall_clock = Duration::from_secs(1);
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/sleep".into(),
            arguments: vec!["30".into()],
            stdin: vec![],
            limits: bounds,
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert!(matches!(run.outcome, ProcessOutcome::TimedOut));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn overflowing_wall_clock_limit_refuses_before_spawn() {
        let marker = std::env::temp_dir().join(format!("cg-ir974-overflow-{}", std::process::id()));
        let mut bounds = limits(1, 64);
        bounds.wall_clock = Duration::MAX;
        let result = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/touch".into(),
            arguments: vec![marker.as_os_str().to_owned()],
            stdin: vec![],
            limits: bounds,
            cancel: Cancel::new(),
        });
        assert!(matches!(
            result,
            Err(ProcessError::Io { phase: ProcessFailurePhase::Selection, source })
                if source.kind() == io::ErrorKind::InvalidInput
        ));
        assert!(!marker.exists());
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn timed_out_child_retains_bounded_raw_output_without_a_success_status() {
        let mut bounds = limits(1, 64);
        bounds.wall_clock = Duration::from_secs(1);
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/python3".into(),
            arguments: vec![
                "-c".into(),
                "import sys,time; sys.stdout.buffer.write(b'partial'); sys.stdout.flush(); time.sleep(30)".into(),
            ],
            stdin: vec![],
            limits: bounds,
            cancel: Cancel::new(),
        }).unwrap();
        assert!(run.started);
        assert!(matches!(run.outcome, ProcessOutcome::TimedOut));
        assert_eq!(run.stdout.as_deref(), Some(b"partial".as_slice()));
        assert_eq!(run.stderr.as_deref(), Some(b"".as_slice()));
    }

    #[trace("FR-045-AC-3")]
    #[test]
    fn owned_child_tree_stops_when_memory_limit_is_reached() {
        let mut bounds = limits(1, 64);
        bounds.memory_bytes = NonZeroU64::new(32 * 1024 * 1024).unwrap();
        let run = run_bounded_process(ProcessRequest {
            executable: "/usr/bin/python3".into(),
            arguments: vec![
                "-c".into(),
                "import time; payload = bytearray(96 * 1024 * 1024); time.sleep(10)".into(),
            ],
            stdin: vec![],
            limits: bounds,
            cancel: Cancel::new(),
        })
        .unwrap();
        assert!(run.started);
        assert!(matches!(run.outcome, ProcessOutcome::MemoryExhausted));
    }
}
