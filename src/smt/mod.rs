//! CG-owned solver execution evidence for QSL-authored SMT queries (FR-045).
//!
//! This module classifies only the producer's two associated solver records.
//! QSL owns the query encoder, inductive method outcome and terminal record.

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
}

/// The exact QSL-owned selection and query bytes an invocation represents.
///
/// The caller supplies authoritative identity types for product, item,
/// instance and query kind. CG retains them without deriving or hashing an
/// identity, and this type requires the exact query bytes and nonzero depth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryAssociation<Product, Item, Instance, Kind> {
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

/// One solver response paired with its QSL-supplied query association.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryRun<Product, Item, Instance, Kind> {
    /// Exact association supplied for this invocation.
    pub association: QueryAssociation<Product, Item, Instance, Kind>,
    /// The solver response or typed non-success.
    pub status: SolverStatus,
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
    base: Option<&QueryRun<Product, Item, Instance, Kind>>,
    step: Option<&QueryRun<Product, Item, Instance, Kind>>,
) -> PairEvidence {
    let (Some(base), Some(step)) = (base, step) else {
        return PairEvidence::MissingResponse;
    };
    if &base.association != expected_base || &step.association != expected_step {
        return PairEvidence::AssociationMismatch;
    }
    for status in [base.status, step.status] {
        if matches!(
            status,
            SolverStatus::MissingTool
                | SolverStatus::Stopped(_)
                | SolverStatus::MalformedResponse
                | SolverStatus::ProcessFailed
        ) {
            return PairEvidence::NonProof(status);
        }
    }
    match (base.status, step.status) {
        (SolverStatus::Unsat, SolverStatus::Unsat) => PairEvidence::InductionEligible,
        (SolverStatus::Sat, _) => PairEvidence::BaseSatCandidate,
        (SolverStatus::Unsat, SolverStatus::Sat) => PairEvidence::StepSatNotClosed,
        (other, _) if other != SolverStatus::Unsat => PairEvidence::NonProof(other),
        (_, other) => PairEvidence::NonProof(other),
    }
}

#[cfg(test)]
mod pair_tests;
