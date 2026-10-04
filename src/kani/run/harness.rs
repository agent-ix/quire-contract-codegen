//! The generated harness of any kind this crate can run, and exactly what execution reads from
//! it (FR-017).

use crate::{
    core::artifact::Artifact,
    kani::abi::KaniSolver,
    kani::identity::{
        KaniObligationHarness, KaniScalarObligationHarness, ObligationKind, StateFrameHarness,
        StateFrameProperty,
    },
};

/// A generated harness of either kind this module can run.
///
/// The two kinds carry their own identity types but share every fact execution reads, so
/// `view()` projects those facts once and the byte-for-byte source check, the launch and the classification stay one code path (FR-017).
#[derive(Clone, Copy, Debug)]
pub enum KaniExecutableHarness<'a> {
    /// A V1 frozen-clause contract harness (FR-015).
    Contract(&'a KaniObligationHarness),
    /// A V2 exact-scalar harness, as `generate_routed` returns it (FR-022).
    Scalar(&'a KaniScalarObligationHarness),
    /// A V2 state-clause operation-contract or frame-effect harness (IR-412).
    StateFrame(&'a StateFrameHarness),
}

impl<'a> From<&'a KaniObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniObligationHarness) -> Self {
        Self::Contract(harness)
    }
}

impl<'a> From<&'a KaniScalarObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniScalarObligationHarness) -> Self {
        Self::Scalar(harness)
    }
}

impl<'a> From<&'a StateFrameHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a StateFrameHarness) -> Self {
        Self::StateFrame(harness)
    }
}

/// Exactly what execution reads from a harness, whichever kind it is.
pub(super) struct HarnessView<'a> {
    pub(super) rust: &'a Artifact,
    /// The `module::harness` path Kani names the harness by, which a batch passes to `--harness`
    /// and finds again as the `harness_id` of the report entry.
    pub(super) selection: String,
    pub(super) kind: Option<ObligationKind>,
    pub(super) unwind: u32,
    pub(super) solver: KaniSolver,
    pub(super) options: &'a [String],
}

impl<'a> KaniExecutableHarness<'a> {
    pub(super) fn view(self) -> HarnessView<'a> {
        match self {
            Self::Contract(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    rust: &harness.rust,
                    selection: identity.harness_path().to_string(),
                    kind: Some(identity.kind),
                    unwind: identity.unwind,
                    solver: identity.solver,
                    options: &identity.options,
                }
            }
            Self::Scalar(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    rust: &harness.rust,
                    selection: identity.harness_path().to_string(),
                    kind: None,
                    unwind: identity.unwind,
                    solver: identity.solver,
                    options: &identity.options,
                }
            }
            Self::StateFrame(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    rust: &harness.rust,
                    selection: format!("{}::{}", identity.module_symbol, identity.harness_symbol),
                    kind: Some(match identity.property {
                        StateFrameProperty::Postcondition { .. } => ObligationKind::Postcondition,
                        StateFrameProperty::Frame { .. } => ObligationKind::Frame,
                    }),
                    unwind: identity.unwind,
                    solver: identity.solver,
                    options: &identity.options,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::{identity::StateComparison, test_support::state_frame_harness};

    /// A state-clause harness reports the contract role of what it proves, so execution
    /// evidence tells an operation-contract proof from a frame-effect proof.
    ///
    /// Trace: TC-027
    #[test]
    fn tc_027_a_state_frame_harness_reports_the_kind_of_what_it_proves() {
        let harness = |property| state_frame_harness(property, Vec::new());
        let contract = harness(StateFrameProperty::Postcondition {
            field: "balance".to_owned(),
            comparison: StateComparison::Ge,
            left_is_pre: false,
        });
        let frame = harness(StateFrameProperty::Frame {
            granted: Vec::new(),
            checked: vec!["audit".to_owned()],
        });
        assert_eq!(
            KaniExecutableHarness::from(&contract).view().kind,
            Some(ObligationKind::Postcondition)
        );
        assert_eq!(
            KaniExecutableHarness::from(&frame).view().kind,
            Some(ObligationKind::Frame)
        );
    }
}
