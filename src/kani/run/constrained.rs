// SPDX-License-Identifier: AGPL-3.0-or-later
//! Dedicated constrained execution and the one typed owner-observation conversion.

use crate::kani::identity::constrained::*;
use crate::kani::{
    classify::{KaniInconclusiveReason, KaniRunOutcome},
    output::report::KaniCheckResult,
    run::{
        execute::{
            execute_constrained_raw, KaniExecutionEvidence, KaniExecutionRefusal,
            KaniExecutionRequest,
        },
        harness::KaniExecutableHarness,
    },
};
use serde::Serialize;

/// Backend evidence sealed to its originating constrained method and source bindings.
#[derive(Debug, Serialize)]
pub struct ConstrainedBackendEvidence {
    pub(in crate::kani) method: ConstrainedMethodIdentity,
    pub(in crate::kani) backend: KaniExecutionEvidence,
    pub(in crate::kani) safety_assertion_line: u32,
    pub(in crate::kani) ordinary_cover_line: u32,
    pub(in crate::kani) boundary_cover_line: Option<u32>,
}
impl ConstrainedBackendEvidence {
    /// The exact method that produced this evidence.
    pub fn method(&self) -> &ConstrainedMethodIdentity {
        &self.method
    }
    /// Original typed backend checks, without an unconditional proof conversion.
    pub fn checks(&self) -> &[KaniCheckResult] {
        &self.backend.checks
    }
    /// Original falsifying playback, when the backend actually retained one.
    pub fn playback(&self) -> Option<&str> {
        if let KaniRunOutcome::Falsified { counterexample } = &self.backend.outcome {
            Some(counterexample)
        } else {
            None
        }
    }
    /// Original cancellation source, without exposing an ordinary proof outcome.
    pub fn cancellation_source(&self) -> Option<quire_contract_ir::kani::KaniCancellationSource> {
        if let KaniRunOutcome::Cancelled { source } = self.backend.outcome {
            Some(source)
        } else {
            None
        }
    }
    /// Original typed backend failure cause, when the report retained one.
    pub fn failure_cause(&self) -> Option<&crate::kani::output::report::KaniFailureCause> {
        self.backend.failure_cause.as_ref()
    }
    /// Original bounded/redacted backend diagnostics, when retained.
    pub fn diagnostics(
        &self,
    ) -> Option<&crate::kani::output::diagnostics::KaniNoVerdictDiagnostics> {
        self.backend.diagnostics.as_ref()
    }
    /// Actual backend-tree memory observation.
    pub fn memory(&self) -> &super::memory::MemoryObservation {
        &self.backend.memory
    }
    /// The original typed non-proof reason, retained without inventing a cover result.
    pub fn nonproof_reason(&self) -> Option<KaniInconclusiveReason> {
        if let KaniRunOutcome::Inconclusive { reason } = self.backend.outcome {
            Some(reason)
        } else {
            None
        }
    }
}

/// Run only a sealed constrained harness through the existing bounded launcher.
///
/// # Errors
/// Refuses an ordinary harness, a nonunique source binding or any existing launch/report error.
pub fn execute_constrained_method(
    request: &KaniExecutionRequest<'_>,
) -> Result<ConstrainedBackendEvidence, KaniExecutionRefusal> {
    let KaniExecutableHarness::Constrained(harness) = request.harness else {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    };
    let library =
        std::fs::read_to_string(request.crate_directory.join("src/lib.rs")).map_err(|_| {
            KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: harness.rust.path.clone(),
            }
        })?;
    let mut matches = library.match_indices(&harness.rust.contents);
    let (Some((start, _)), None) = (matches.next(), matches.next()) else {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        });
    };
    let offset = u32::try_from(
        library[..start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count(),
    )
    .map_err(|_| KaniExecutionRefusal::HarnessNotInCrate {
        harness_path: harness.rust.path.clone(),
    })?;
    let line = |value: u32| {
        offset
            .checked_add(value)
            .ok_or_else(|| KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: harness.rust.path.clone(),
            })
    };
    let safety_assertion_line = line(harness.identity.safety_assertion_line)?;
    let ordinary_cover_line = line(harness.identity.ordinary_cover_line)?;
    let boundary_cover_line = harness.identity.boundary_cover_line.map(line).transpose()?;
    let backend = execute_constrained_raw(request)?;
    Ok(ConstrainedBackendEvidence {
        method: harness.identity.clone(),
        backend,
        safety_assertion_line,
        ordinary_cover_line,
        boundary_cover_line,
    })
}
