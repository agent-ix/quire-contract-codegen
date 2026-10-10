// Bounded collection query admission.
pub(crate) mod bounded_collections;
// The bounded-Kani profile and its classification.
pub(crate) mod bounded_kani_profile;
// Checked arithmetic admission.
pub(crate) mod definedness_arithmetic;
// Finite reference graph reachability admission.
pub(crate) mod finite_reference_graphs;

use quire_contract_ir::kani::{
    CapabilityDisposition, DispatchIndex, KaniOutcome, KaniOutcomeError, KaniOutcomeKind,
    KaniProfile, SemanticFamily,
};
use quire_contract_model::{std001_code, Std001Code};

/// A family-lowering refusal, preserving IR's outcome-construction failure separately.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FamilyLoweringError {
    /// A typed non-Boolean IR outcome.
    Outcome(KaniOutcome),
    /// IR refused construction of the requested outcome.
    OutcomeConstruction(KaniOutcomeError),
}

impl FamilyLoweringError {
    /// Returns the IR outcome when its construction succeeded.
    #[must_use]
    pub const fn outcome(&self) -> Option<&KaniOutcome> {
        match self {
            Self::Outcome(outcome) => Some(outcome),
            Self::OutcomeConstruction(_) => None,
        }
    }

    /// The stable code of the outcome or of IR's construction refusal.
    #[must_use]
    pub const fn code(&self) -> Std001Code {
        match self {
            Self::Outcome(outcome) => outcome.code,
            Self::OutcomeConstruction(error) => error.code(),
        }
    }
}

impl std::fmt::Display for FamilyLoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Outcome(outcome) => write!(f, "{}: {}", outcome.code, outcome.source_id),
            Self::OutcomeConstruction(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for FamilyLoweringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Outcome(_) => None,
            Self::OutcomeConstruction(error) => Some(error),
        }
    }
}

impl From<KaniOutcome> for FamilyLoweringError {
    fn from(outcome: KaniOutcome) -> Self {
        Self::Outcome(outcome)
    }
}

impl From<KaniOutcomeError> for FamilyLoweringError {
    fn from(error: KaniOutcomeError) -> Self {
        Self::OutcomeConstruction(error)
    }
}

fn refusal(
    kind: KaniOutcomeKind,
    code: Std001Code,
    source_id: &str,
    profile: &KaniProfile,
) -> FamilyLoweringError {
    match KaniOutcome::non_success(kind, code, source_id, profile.selection().revision.clone()) {
        Ok(outcome) => FamilyLoweringError::Outcome(outcome),
        Err(error) => FamilyLoweringError::OutcomeConstruction(error),
    }
}

/// Admit one construct through the shared profile and dispatch authorities.
fn admit_family(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    construct: &str,
    family: SemanticFamily,
    source_id: &str,
) -> Result<(), FamilyLoweringError> {
    let entries = profile.classify(&[construct.to_owned()], source_id)?;
    // A successful single-construct classification contains exactly that entry.
    let Some(entry) = entries.first() else {
        return Err(refusal(
            KaniOutcomeKind::Refused,
            Std001Code::KANI_CAPABILITY_MISSING,
            source_id,
            profile,
        ));
    };
    match &entry.disposition {
        CapabilityDisposition::Supported { module } => {
            let descriptor = dispatch.resolve(construct).map_err(|_| {
                refusal(
                    KaniOutcomeKind::Refused,
                    std001_code!("kani_dispatch_unowned"),
                    source_id,
                    profile,
                )
            })?;
            if descriptor.family != family || descriptor.module_id != *module {
                return Err(refusal(
                    KaniOutcomeKind::Refused,
                    std001_code!("kani_dispatch_profile_mismatch"),
                    source_id,
                    profile,
                ));
            }
            Ok(())
        }
        CapabilityDisposition::Refused { code } => {
            Err(refusal(KaniOutcomeKind::Refused, *code, source_id, profile))
        }
        CapabilityDisposition::Inconclusive { code } => Err(refusal(
            KaniOutcomeKind::Inconclusive,
            *code,
            source_id,
            profile,
        )),
    }
}
