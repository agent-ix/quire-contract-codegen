//! Bind QSL composite parity reports to the exact claim sent by the driver (FR-033).
//!
//! QSL owns the result and terminal mapping. A report is readable only after its full claim,
//! including the observation, equals the identity retained at the call site.

use qsl_replay::{
    CompositeIdentity, CompositeParityReport, CompositeParityResult, TerminalValue,
    VerifiedShadowReport, VerifiedShadowResult,
};

/// A composite report could not be tied to the claim actually sent to QSL.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositeReportError {
    /// QSL returned no report for the submitted claim.
    MissingReport,
    /// The report describes a different claim or observation.
    ClaimMismatch,
}

impl std::fmt::Display for CompositeReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingReport => f.write_str("composite parity report is missing"),
            Self::ClaimMismatch => {
                f.write_str("composite parity report claim differs from sent claim")
            }
        }
    }
}

impl std::error::Error for CompositeReportError {}

/// A binding-checked falsified composite result and QSL's terminal value for it.
#[derive(Debug)]
pub struct CompositeParitySettlement<'a> {
    /// QSL's complete F-row result, including native cause or distinct incomplete stage.
    pub result: &'a CompositeParityResult,
    /// The value QSL assigned to this result, including its own refusal code.
    pub terminal_value: TerminalValue,
}

/// A binding-checked verified composite result and QSL's terminal value for it.
#[derive(Debug)]
pub struct VerifiedShadowSettlement<'a> {
    /// QSL's complete verified-shadow result.
    pub result: &'a VerifiedShadowResult,
    /// The value QSL assigned to this result.
    pub terminal_value: TerminalValue,
}

/// Read a falsified composite report only if its full identity matches the sent claim.
///
/// `sent` must be retained from the actual request before invoking QSL. It includes the complete
/// falsified observation, so equality checks every claim member without reconstructing a subset.
/// A CG pre-invocation refusal is handled by the caller before this function is reached.
///
/// # Errors
///
/// [`CompositeReportError::MissingReport`] if no report was returned, or
/// [`CompositeReportError::ClaimMismatch`] if any identity member differs. Neither has a value.
pub fn composite_parity_terminal_value<'a>(
    sent: &CompositeIdentity,
    report: Option<&'a CompositeParityReport>,
) -> Result<CompositeParitySettlement<'a>, CompositeReportError> {
    let report = report.ok_or(CompositeReportError::MissingReport)?;
    if report.claim() != sent {
        return Err(CompositeReportError::ClaimMismatch);
    }
    Ok(CompositeParitySettlement {
        result: report.result(),
        terminal_value: report.terminal_value(),
    })
}

/// Read a verified composite report only if its full identity matches the sent claim.
///
/// # Errors
///
/// [`CompositeReportError::MissingReport`] if no report was returned, or
/// [`CompositeReportError::ClaimMismatch`] if any identity member differs. Neither has a value.
pub fn verified_shadow_terminal_value<'a>(
    sent: &CompositeIdentity,
    report: Option<&'a VerifiedShadowReport>,
) -> Result<VerifiedShadowSettlement<'a>, CompositeReportError> {
    let report = report.ok_or(CompositeReportError::MissingReport)?;
    if report.claim() != sent {
        return Err(CompositeReportError::ClaimMismatch);
    }
    Ok(VerifiedShadowSettlement {
        result: report.result(),
        terminal_value: report.terminal_value(),
    })
}
