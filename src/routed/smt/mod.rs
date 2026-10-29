//! CG-owned solver execution evidence for QSL-authored SMT queries (FR-045).
//!
//! This module classifies only the producer's two associated solver records.
//! QSL owns the query encoder, inductive method outcome and terminal record.

use std::{
    ffi::OsString,
    fmt,
    num::NonZeroU64,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use quire_exact::Cancel;

use crate::kani::run::process::{
    run_bounded_process, ProcessError, ProcessLimits, ProcessOutcome, ProcessRequest, ProcessRun,
};

/// The per-query resource limit reached by a solver invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReachedLimit {
    /// The QSL-authored query exceeds the caller's input byte ceiling.
    InputBytes,
    /// The process exceeded its wall-clock deadline.
    WallClock,
    /// The process tree exceeded its resident-memory ceiling.
    MemoryBytes,
    /// Captured stdout or stderr exceeded its byte ceiling.
    OutputBytes,
}

/// A solver invocation's observed response or typed non-success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolverStatus {
    /// The solver answered `sat`; a QSL-owned reconstruction is still required.
    Sat,
    /// The solver answered `unsat` for this query only.
    Unsat,
    /// The solver answered `unknown`.
    Unknown,
    /// The selected solver executable was unavailable.
    MissingTool,
    /// CG stopped the invocation at the named limit.
    Stopped(ReachedLimit),
    /// The process output did not contain one well-formed result.
    MalformedResponse,
    /// The process failed independently of a reached resource limit.
    ProcessFailed,
    /// Caller cancellation stopped this query before a complete response.
    Cancelled,
}

/// The exact QSL-owned selection and query bytes requested by the caller.
///
/// The caller supplies authoritative identity types for product, item,
/// instance and query kind. CG retains them without deriving or hashing an
/// identity. The selected depth is nonzero and the exact bytes are retained.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelection<Product, Item, Instance, Kind> {
    /// The selected HP-2 product identity.
    pub product: Product,
    /// The selected safety item identity.
    pub item: Item,
    /// The selected object-parameter instance identity.
    pub instance: Instance,
    /// Induction depth in joint steps.
    pub depth: std::num::NonZeroU64,
    /// QSL's base or step query kind.
    pub kind: Kind,
    /// The exact QSL-authored query bytes sent to the solver.
    pub query_bytes: Vec<u8>,
}

/// A QSL selection paired with CG's distinct process-attempt identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryAssociation<Product, Item, Instance, Kind> {
    /// The QSL-owned selection sent to the solver.
    pub selection: QuerySelection<Product, Item, Instance, Kind>,
    attempt: NonZeroU64,
}

impl<Product, Item, Instance, Kind> QueryAssociation<Product, Item, Instance, Kind> {
    /// Return the CG-assigned identity of this invocation.
    #[must_use]
    pub fn attempt(&self) -> NonZeroU64 {
        self.attempt
    }
}

/// One solver response paired with its QSL-supplied query association.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryRun<Product, Item, Instance, Kind> {
    /// Exact selection and CG-assigned attempt for this invocation.
    pub association: QueryAssociation<Product, Item, Instance, Kind>,
    /// The solver response or typed non-success.
    pub status: SolverStatus,
}

/// The selected child invocation and its associated, interpreted response.
///
/// The process record keeps the actual executable, arguments, limits, completion
/// and bounded raw streams. The association keeps the exact QSL query bytes.
#[derive(Debug)]
pub struct SolverExecution<Product, Item, Instance, Kind> {
    /// The typed query identity and non-proof or response status.
    query: QueryRun<Product, Item, Instance, Kind>,
    /// The exact owned child invocation record.
    process: ProcessRun,
}

impl<Product, Item, Instance, Kind> SolverExecution<Product, Item, Instance, Kind> {
    /// Return the CG-authenticated query identity and solver status.
    #[must_use]
    pub fn query(&self) -> &QueryRun<Product, Item, Instance, Kind> {
        &self.query
    }

    /// Return the selected invocation and actual bounded process result.
    #[must_use]
    pub fn process(&self) -> &ProcessRun {
        &self.process
    }
}

/// Why CG could not produce an authenticated solver execution record.
#[derive(Debug)]
pub enum SolverRunError {
    /// The process-wide attempt counter cannot issue another distinct identity.
    AttemptExhausted,
    /// The bounded child mechanism failed before a trustworthy process result.
    Process(ProcessError),
}

impl fmt::Display for SolverRunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttemptExhausted => write!(formatter, "solver attempt identities exhausted"),
            Self::Process(error) => write!(formatter, "solver process: {error}"),
        }
    }
}

impl std::error::Error for SolverRunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AttemptExhausted => None,
            Self::Process(error) => Some(error),
        }
    }
}

impl From<ProcessError> for SolverRunError {
    fn from(error: ProcessError) -> Self {
        Self::Process(error)
    }
}

// Only uniqueness is published. No process result is synchronized through this counter.
static NEXT_ATTEMPT: AtomicU64 = AtomicU64::new(1);

fn next_attempt() -> Result<NonZeroU64, SolverRunError> {
    let value = NEXT_ATTEMPT
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .map_err(|_| SolverRunError::AttemptExhausted)?;
    NonZeroU64::new(value).ok_or(SolverRunError::AttemptExhausted)
}

/// Invoke the selected solver directly for exactly one QSL-authored query.
///
/// This records a solver response; it never constructs a QSL outcome.
pub fn run_solver_query<Product, Item, Instance, Kind>(
    selection: QuerySelection<Product, Item, Instance, Kind>,
    executable: PathBuf,
    arguments: Vec<OsString>,
    limits: ProcessLimits,
    cancel: Cancel,
) -> Result<SolverExecution<Product, Item, Instance, Kind>, SolverRunError> {
    let association = QueryAssociation {
        selection,
        attempt: next_attempt()?,
    };
    let process = if association.selection.query_bytes.len() > limits.input_bytes.get() {
        ProcessRun {
            executable,
            arguments,
            limits,
            started: false,
            stdout: None,
            stderr: None,
            outcome: ProcessOutcome::InputOverLimit,
        }
    } else {
        run_bounded_process(ProcessRequest {
            executable,
            arguments,
            stdin: association.selection.query_bytes.clone(),
            limits,
            cancel,
        })?
    };
    let status = match &process.outcome {
        ProcessOutcome::MissingTool => SolverStatus::MissingTool,
        ProcessOutcome::InputOverLimit => SolverStatus::Stopped(ReachedLimit::InputBytes),
        ProcessOutcome::TimedOut => SolverStatus::Stopped(ReachedLimit::WallClock),
        ProcessOutcome::MemoryExhausted => SolverStatus::Stopped(ReachedLimit::MemoryBytes),
        ProcessOutcome::OutputOverLimit { .. } => SolverStatus::Stopped(ReachedLimit::OutputBytes),
        ProcessOutcome::Cancelled { .. } => SolverStatus::Cancelled,
        ProcessOutcome::Completed { success: true, .. } => process
            .stdout
            .as_deref()
            .map_or(SolverStatus::MalformedResponse, parse_solver_status),
        ProcessOutcome::Completed { success: false, .. }
        | ProcessOutcome::MemoryUnobserved { .. }
        | ProcessOutcome::OutputUnread { .. }
        | ProcessOutcome::InputUnread { .. } => SolverStatus::ProcessFailed,
    };
    Ok(SolverExecution {
        query: QueryRun {
            association,
            status,
        },
        process,
    })
}

fn parse_solver_status(stdout: &[u8]) -> SolverStatus {
    let Some((status, tail)) = status_and_tail(stdout) else {
        return SolverStatus::MalformedResponse;
    };
    if valid_solver_tail(tail) {
        status
    } else {
        SolverStatus::MalformedResponse
    }
}

fn status_and_tail(stdout: &[u8]) -> Option<(SolverStatus, &[u8])> {
    let mut offset = 0;
    for line in stdout.split_inclusive(|byte| *byte == b'\n') {
        offset += line.len();
        let line = line.trim_ascii();
        if line.is_empty() || line.starts_with(b";") || line == b"success" {
            continue;
        }
        let status = match line {
            b"sat" => SolverStatus::Sat,
            b"unsat" => SolverStatus::Unsat,
            b"unknown" => SolverStatus::Unknown,
            _ => return None,
        };
        return Some((status, &stdout[offset..]));
    }
    None
}

// Retain the exact tail of the same completed solver stream. A proof is a
// nonempty sequence of complete top-level Alethe commands; QSL checks their
// meaning after the CG handoff.
pub(super) fn captured_alethe_proof(stdout: &[u8]) -> Option<&[u8]> {
    let (SolverStatus::Unsat, tail) = status_and_tail(stdout)? else {
        return None;
    };
    let mut index = 0;
    let mut commands = 0usize;
    while index < tail.len() {
        match tail[index] {
            byte if byte.is_ascii_whitespace() => index += 1,
            b';' => {
                while index < tail.len() && tail[index] != b'\n' {
                    index += 1;
                }
            }
            b'(' => {
                if solver_error_head(&tail[index..]) {
                    return None;
                }
                index = sexpr_end(tail, index)?;
                commands += 1;
            }
            _ => return None,
        }
    }
    (commands > 0).then_some(tail)
}

// Model and proof output is retained raw. Only its outer SMT-LIB response
// framing is checked: comments and balanced S-expressions after the status.
fn valid_solver_tail(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            byte if byte.is_ascii_whitespace() => index += 1,
            b';' => {
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'(' => {
                if solver_error_head(&bytes[index..]) {
                    return false;
                }
                let Some(end) = sexpr_end(bytes, index) else {
                    return false;
                };
                index = end;
            }
            _ => return false,
        }
    }
    true
}

fn solver_error_head(bytes: &[u8]) -> bool {
    let after_open = bytes[1..].trim_ascii_start();
    after_open.starts_with(b"error")
        && after_open
            .get(5)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b')')
}

fn sexpr_end(bytes: &[u8], start: usize) -> Option<usize> {
    enum Quote {
        Plain,
        String,
        Symbol,
        Comment,
    }
    let mut depth = 0usize;
    let mut quote = Quote::Plain;
    let mut index = start;
    while index < bytes.len() {
        let byte = bytes[index];
        match quote {
            Quote::Plain => match byte {
                b'(' => depth = depth.checked_add(1)?,
                b')' => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        return Some(index + 1);
                    }
                }
                b'"' => quote = Quote::String,
                b'|' => quote = Quote::Symbol,
                b';' => quote = Quote::Comment,
                _ => {}
            },
            Quote::String if byte == b'"' => {
                if bytes.get(index + 1) == Some(&b'"') {
                    index += 1;
                } else {
                    quote = Quote::Plain;
                }
            }
            Quote::Symbol if byte == b'\\' => index += 1,
            Quote::Symbol if byte == b'|' => quote = Quote::Plain,
            Quote::Comment if byte == b'\n' => quote = Quote::Plain,
            _ => {}
        }
        index += 1;
    }
    None
}

/// CG's assessment of a base/step pair as run evidence only.
///
/// This is not a QSL method outcome or terminal settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PairEvidence {
    /// Both correctly associated solver responses were `unsat`.
    InductionEligible,
    /// A base `sat` can only be a candidate for QSL reconstruction and replay.
    BaseSatCandidate,
    /// A `sat` step does not close induction after an `unsat` base.
    StepSatNotClosed,
    /// A response came from another query, item, product, instance or depth.
    AssociationMismatch,
    /// A required response is absent.
    MissingResponse,
    /// The pair has the retained non-proof solver disposition.
    NonProof(SolverStatus),
}

/// Classify one base/step pair without manufacturing QSL-owned outcomes.
///
/// The expected associations include the exact bytes and query kind. The
/// QSL-facing caller supplies each identity type. A response from another
/// run cannot complete this pair, even if its solver status is `unsat`.
#[must_use]
pub fn classify_pair<Product: Eq, Item: Eq, Instance: Eq, Kind: Eq>(
    expected_base: &QueryAssociation<Product, Item, Instance, Kind>,
    expected_step: &QueryAssociation<Product, Item, Instance, Kind>,
    base: Option<&SolverExecution<Product, Item, Instance, Kind>>,
    step: Option<&SolverExecution<Product, Item, Instance, Kind>>,
) -> PairEvidence {
    let (Some(base), Some(step)) = (base, step) else {
        return PairEvidence::MissingResponse;
    };
    if expected_base.attempt == expected_step.attempt
        || expected_base.selection.kind == expected_step.selection.kind
        || &base.query.association != expected_base
        || &step.query.association != expected_step
    {
        return PairEvidence::AssociationMismatch;
    }
    for execution in [base, step] {
        if matches!(
            execution.query.status,
            SolverStatus::Sat | SolverStatus::Unsat | SolverStatus::Unknown
        ) && (!execution.process.started
            || !matches!(
                execution.process.outcome,
                ProcessOutcome::Completed { success: true, .. }
            ))
        {
            return PairEvidence::NonProof(SolverStatus::ProcessFailed);
        }
    }
    for status in [base.query.status, step.query.status] {
        if matches!(
            status,
            SolverStatus::Unknown
                | SolverStatus::MissingTool
                | SolverStatus::Stopped(_)
                | SolverStatus::MalformedResponse
                | SolverStatus::ProcessFailed
                | SolverStatus::Cancelled
        ) {
            return PairEvidence::NonProof(status);
        }
    }
    match (base.query.status, step.query.status) {
        (SolverStatus::Unsat, SolverStatus::Unsat) => PairEvidence::InductionEligible,
        (SolverStatus::Sat, _) => PairEvidence::BaseSatCandidate,
        (SolverStatus::Unsat, SolverStatus::Sat) => PairEvidence::StepSatNotClosed,
        (other, _) if other != SolverStatus::Unsat => PairEvidence::NonProof(other),
        (_, other) => PairEvidence::NonProof(other),
    }
}

#[cfg(test)]
mod pair_tests;

#[cfg(test)]
mod proof_tests {
    use ix_trace_rs::trace;

    use super::{captured_alethe_proof, parse_solver_status, SolverStatus};

    #[trace("TC-059", "FR-045-AC-5", "FR-045-AC-7")]
    #[test]
    fn completed_unsat_stdout_keeps_exact_proof_tail() {
        let response = b"success\nunsat\n(assume a0 true)\n(step s0 (cl) :rule resolution)\n";
        assert_eq!(parse_solver_status(response), SolverStatus::Unsat);
        assert_eq!(
            captured_alethe_proof(response),
            Some(b"(assume a0 true)\n(step s0 (cl) :rule resolution)\n".as_slice())
        );
    }

    #[trace("TC-059", "FR-045-AC-7")]
    #[test]
    fn absent_or_ambiguous_proof_tail_is_not_proof_evidence() {
        for response in [
            b"unsat\n".as_slice(),
            b"unsat\n; only a comment\n",
            b"unsat\nsat\n",
            b"unsat\n(error \"failure\")\n",
            b"unsat\n(step s0 (cl) :rule resolution",
            b"unsat\n(step s0 (cl) :rule resolution) trailing",
            b"unsat\nsuccess\n",
            b"unknown\n(step s0 (cl) :rule resolution)",
        ] {
            assert_eq!(captured_alethe_proof(response), None);
        }
        for response in [
            b"unsat\nsat\n".as_slice(),
            b"unsat\n(error \"failure\")\n",
            b"unsat\n(step s0 (cl) :rule resolution",
            b"unsat\n(step s0 (cl) :rule resolution) trailing",
            b"unsat\nsuccess\n",
        ] {
            assert_eq!(
                parse_solver_status(response),
                SolverStatus::MalformedResponse
            );
        }
    }
}

pub mod refinement;
