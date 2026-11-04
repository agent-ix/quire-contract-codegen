// SPDX-License-Identifier: AGPL-3.0-or-later

//! Production FR-315 preparation and direct execution of one selected refinement.

use std::{collections::TryReserveError, ffi::OsString, num::NonZeroU64, path::Path};

use qsl_replay::{
    encode_refinement_query, NodeKey, RefinementQueryKind, RefinementQueryRequest,
    SmtEncodingRefusal,
};
use quire_exact::{Cancel, CancelCause};

use super::{
    refinement::{
        refinement_handoff, RefinementExpected, RefinementHandoffError, RefinementRole,
        RefinementRun,
    },
    run_solver_query, QuerySelection, SolverRunError,
};
use crate::kani::run::process::ProcessLimits;

/// Association with the same immutable request and its ordered retained subjects.
///
/// Equality means the very same request, rather than equal display names or a
/// caller's reconstruction of the request's private subject data.
#[derive(Clone, Copy)]
pub struct RefinementSubjects<'request, 'input> {
    request: &'request RefinementQueryRequest<'input>,
}

impl std::fmt::Debug for RefinementSubjects<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefinementSubjects")
            .field("refinement", self.request.refinement_key())
            .finish_non_exhaustive()
    }
}

impl PartialEq for RefinementSubjects<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.request, other.request)
    }
}

impl Eq for RefinementSubjects<'_, '_> {}

impl<'request, 'input> RefinementSubjects<'request, 'input> {
    /// Return the actual request retaining both subjects in their selected order.
    #[must_use]
    pub fn request(&self) -> &'request RefinementQueryRequest<'input> {
        self.request
    }
}

/// Why preparation failed before launching any process.
#[derive(Debug)]
pub enum RefinementPreparationError {
    /// Original QSL refusal; `None` means selecting the retained refinement failed.
    Encoding {
        /// Query selected when encoding failed.
        role: Option<RefinementRole>,
        /// Unmodified QSL refusal or internal fault.
        refusal: SmtEncodingRefusal,
    },
    /// The caller cancelled between encoding operations.
    Cancelled(CancelCause),
    /// Native reservation failed, without a partial prepared collection.
    Allocation(TryReserveError),
    /// A row or total record count is not representable.
    CountOverflow,
}

/// All authoritative query bytes, prepared before any solver process is launched.
pub struct PreparedRefinement<'request, 'input> {
    subjects: RefinementSubjects<'request, 'input>,
    step_count: usize,
    queries: Vec<(RefinementRole, Vec<u8>)>,
}

/// Prepare the Base and declaration-order Step queries from one QSL request.
///
/// QSL owns all query encoding. Refusal or cancellation produces no executable
/// collection and therefore cannot be confused with a solver response.
pub fn prepare_refinement<'request, 'input>(
    request: &'request RefinementQueryRequest<'input>,
    cancel: &Cancel,
) -> Result<PreparedRefinement<'request, 'input>, RefinementPreparationError> {
    check_cancel(cancel)?;
    let step_count =
        request
            .step_count()
            .map_err(|refusal| RefinementPreparationError::Encoding {
                role: None,
                refusal,
            })?;
    let count = step_count
        .checked_add(1)
        .ok_or(RefinementPreparationError::CountOverflow)?;
    u64::try_from(step_count).map_err(|_| RefinementPreparationError::CountOverflow)?;
    let mut queries = Vec::new();
    queries
        .try_reserve_exact(count)
        .map_err(RefinementPreparationError::Allocation)?;
    for role in std::iter::once(RefinementRole::Base)
        .chain((0..step_count).map(|row| RefinementRole::Step { row }))
    {
        check_cancel(cancel)?;
        let kind = match role {
            RefinementRole::Base => RefinementQueryKind::Initial,
            RefinementRole::Step { row } => RefinementQueryKind::Step {
                row: u64::try_from(row).map_err(|_| RefinementPreparationError::CountOverflow)?,
            },
        };
        let query = encode_refinement_query(request, kind).map_err(|refusal| {
            RefinementPreparationError::Encoding {
                role: Some(role),
                refusal,
            }
        })?;
        queries.push((role, query));
    }
    check_cancel(cancel)?;
    Ok(PreparedRefinement {
        subjects: RefinementSubjects { request },
        step_count,
        queries,
    })
}

fn check_cancel(cancel: &Cancel) -> Result<(), RefinementPreparationError> {
    match cancel.cause() {
        Some(cause) => Err(RefinementPreparationError::Cancelled(cause)),
        None => Ok(()),
    }
}

/// Typed records still tied to the request that supplied their complete query set.
#[derive(Debug)]
pub struct RefinementExecution<'request, 'input> {
    step_count: usize,
    expected: Vec<RefinementExpected<NodeKey, RefinementSubjects<'request, 'input>>>,
    records: Vec<RefinementRun<NodeKey, RefinementSubjects<'request, 'input>>>,
}

/// Why execution could not produce the complete run collection.
#[derive(Debug)]
pub enum RefinementExecutionError<'request, 'input> {
    /// Native reservation failed before any solver process.
    Allocation(TryReserveError),
    /// A process mechanism failed; earlier genuine records remain available.
    Process {
        /// Query whose execution mechanism failed.
        role: RefinementRole,
        /// Original typed process mechanism error.
        error: SolverRunError,
        /// Completed invocations before the failure.
        completed: RefinementExecution<'request, 'input>,
    },
}

impl<'request, 'input> PreparedRefinement<'request, 'input> {
    /// Execute each prepared query unchanged, directly, under its own limits.
    ///
    /// Non-proof statuses remain typed records; no QSL outcome is constructed.
    pub fn execute(
        self,
        executable: &Path,
        arguments: &[OsString],
        limits: ProcessLimits,
        cancel: &Cancel,
    ) -> Result<RefinementExecution<'request, 'input>, RefinementExecutionError<'request, 'input>>
    {
        let mut output = RefinementExecution {
            step_count: self.step_count,
            expected: Vec::new(),
            records: Vec::new(),
        };
        output
            .expected
            .try_reserve_exact(self.queries.len())
            .map_err(RefinementExecutionError::Allocation)?;
        output
            .records
            .try_reserve_exact(self.queries.len())
            .map_err(RefinementExecutionError::Allocation)?;
        for (role, query_bytes) in self.queries {
            let expected_query = query_bytes.clone();
            let execution = run_solver_query(
                QuerySelection {
                    product: (),
                    item: self.subjects.request.refinement_key().clone(),
                    instance: (),
                    depth: NonZeroU64::MIN,
                    kind: role,
                    query_bytes,
                },
                executable.to_owned(),
                arguments.to_vec(),
                limits,
                cancel.clone(),
            );
            let execution = match execution {
                Ok(execution) => execution,
                Err(error) => {
                    return Err(RefinementExecutionError::Process {
                        role,
                        error,
                        completed: output,
                    })
                }
            };
            let association = &execution.query().association;
            output.expected.push(RefinementExpected {
                item: self.subjects.request.refinement_key().clone(),
                subjects: self.subjects,
                role,
                attempt: association.attempt(),
                query_bytes: expected_query,
            });
            output
                .records
                .push(RefinementRun::from_execution(self.subjects, execution));
        }
        Ok(output)
    }
}

impl<'request, 'input> RefinementExecution<'request, 'input> {
    /// Return the original expected invocation associations, in declaration order.
    #[must_use]
    pub fn expected(&self) -> &[RefinementExpected<NodeKey, RefinementSubjects<'request, 'input>>] {
        &self.expected
    }

    /// Return all genuine process records, including typed non-proof outcomes.
    #[must_use]
    pub fn records(&self) -> &[RefinementRun<NodeKey, RefinementSubjects<'request, 'input>>] {
        &self.records
    }

    /// Validate the complete evidence and serialize only a proof-bearing collection.
    pub fn handoff(&self) -> Result<Vec<u8>, RefinementHandoffError> {
        refinement_handoff(self.step_count, &self.expected, &self.records)
    }
}
