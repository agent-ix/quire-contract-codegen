//! The generated harness of any kind this crate can run, and exactly what execution reads from
//! it (FR-017).

use crate::{
    core::artifact::Artifact,
    kani::abi::KaniSolver,
    kani::identity::{
        CollectionElementIdentity, KaniCollectionObligationHarness, KaniGraphObligationHarness,
        KaniObligationHarness, KaniScalarObligationHarness, ObligationKind, StateFrameHarness,
        StateFrameProperty, SymbolicArgumentBounds, SymbolicBounds,
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
    /// A V2 admitted symbolic collection claim.
    Collection(&'a KaniCollectionObligationHarness),
    /// An admitted symbolic graph clause occurrence.
    Graph(&'a KaniGraphObligationHarness),
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

impl<'a> From<&'a KaniCollectionObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniCollectionObligationHarness) -> Self {
        Self::Collection(harness)
    }
}

impl<'a> From<&'a KaniGraphObligationHarness> for KaniExecutableHarness<'a> {
    fn from(harness: &'a KaniGraphObligationHarness) -> Self {
        Self::Graph(harness)
    }
}

/// Exactly what execution reads from a harness, whichever kind it is.
pub(super) struct HarnessView<'a> {
    pub(super) ceilings: crate::kani::identity::ProofCeilings,
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
    pub(super) fn has_valid_provenance(self) -> bool {
        match self {
            Self::Collection(harness) => harness.has_valid_provenance(),
            Self::Graph(harness) => harness.has_valid_provenance(),
            Self::Contract(_) | Self::Scalar(_) | Self::StateFrame(_) => true,
        }
    }

    /// Record argument bounds only when producing execution evidence.
    pub(super) fn symbolic_arguments(self) -> Vec<SymbolicArgumentBounds> {
        match self {
            Self::Contract(harness) => {
                let identity = &harness.identity;
                identity
                    .arguments
                    .iter()
                    .map(|argument| SymbolicArgumentBounds {
                        identifier: argument.identifier.clone(),
                        bounds: match (&argument.primitive_type, &argument.integer_bounds) {
                            (crate::kani::abi::KaniPrimitiveType::Boolean, _) => {
                                SymbolicBounds::Boolean
                            }
                            (crate::kani::abi::KaniPrimitiveType::I64, Some(bounds)) => {
                                SymbolicBounds::Integer {
                                    minimum: bounds.minimum,
                                    maximum: bounds.maximum,
                                }
                            }
                            (crate::kani::abi::KaniPrimitiveType::I64, None) => {
                                SymbolicBounds::Integer {
                                    minimum: i64::MIN,
                                    maximum: i64::MAX,
                                }
                            }
                        },
                    })
                    .collect()
            }
            Self::Scalar(harness) => {
                let identity = &harness.identity;
                let boolean =
                    identity.operation_identity == crate::kani::identity::BOOLEAN_IMPLIES_IDENTITY;
                identity
                    .arguments
                    .iter()
                    .map(|argument| SymbolicArgumentBounds {
                        identifier: argument.identifier.clone(),
                        bounds: if boolean {
                            SymbolicBounds::Boolean
                        } else {
                            SymbolicBounds::Integer {
                                minimum: argument.minimum,
                                maximum: argument.maximum,
                            }
                        },
                    })
                    .collect()
            }
            Self::StateFrame(harness) => {
                let identity = &harness.identity;
                identity
                    .state_fields
                    .iter()
                    .map(|field| SymbolicArgumentBounds {
                        identifier: field.clone(),
                        bounds: identity
                            .domains
                            .iter()
                            .find(|domain| &domain.field == field)
                            .map_or(
                                SymbolicBounds::Integer {
                                    minimum: i64::MIN,
                                    maximum: i64::MAX,
                                },
                                |domain| SymbolicBounds::Integer {
                                    minimum: domain.minimum,
                                    maximum: domain.maximum,
                                },
                            ),
                    })
                    .collect()
            }
            Self::Graph(harness) => {
                let identity = &harness.identity;
                let mut slots = vec![SymbolicArgumentBounds {
                    identifier: "object_count".into(),
                    bounds: SymbolicBounds::Integer {
                        minimum: 0,
                        maximum: i64::try_from(identity.maximum_population).unwrap_or(i64::MAX),
                    },
                }];
                for name in ["start", "target"] {
                    slots.push(SymbolicArgumentBounds {
                        identifier: name.into(),
                        bounds: SymbolicBounds::Integer {
                            minimum: 0,
                            maximum: i64::try_from(identity.maximum_population.saturating_sub(1))
                                .unwrap_or(i64::MAX),
                        },
                    });
                }
                for row in 0..identity.maximum_population {
                    slots.push(SymbolicArgumentBounds {
                        identifier: format!("degree_{row}"),
                        bounds: SymbolicBounds::Integer {
                            minimum: 0,
                            maximum: i64::try_from(identity.edge.upper).unwrap_or(i64::MAX),
                        },
                    });
                    for column in 0..identity.edge.upper {
                        slots.push(SymbolicArgumentBounds {
                            identifier: format!("edge_{row}_{column}"),
                            bounds: SymbolicBounds::Integer {
                                minimum: 0,
                                maximum: i64::try_from(
                                    identity.maximum_population.saturating_sub(1),
                                )
                                .unwrap_or(i64::MAX),
                            },
                        });
                    }
                }
                slots
            }
            Self::Collection(harness) => {
                let identity = &harness.identity;
                let element = match identity.element {
                    CollectionElementIdentity::Boolean => SymbolicBounds::Boolean,
                    CollectionElementIdentity::Integer { minimum, maximum } => {
                        SymbolicBounds::Integer { minimum, maximum }
                    }
                };
                vec![SymbolicArgumentBounds {
                    identifier: format!(
                        "{}::{}::{}",
                        identity.declaration_id.digest,
                        identity.function_id.digest,
                        identity.field_name
                    ),
                    bounds: SymbolicBounds::Sequence {
                        minimum_length: identity.minimum_length,
                        maximum_length: identity.maximum_length,
                        element: Box::new(element),
                    },
                }]
            }
        }
    }

    pub(super) fn view(self) -> HarnessView<'a> {
        match self {
            Self::Contract(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    ceilings: identity.ceilings,
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
                    ceilings: identity.ceilings,
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
                    ceilings: identity.ceilings,
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
            Self::Graph(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    ceilings: identity.ceilings,
                    rust: &harness.rust,
                    selection: format!("{}::{}", identity.module_symbol, identity.harness_symbol),
                    kind: Some(identity.kind),
                    unwind: identity.unwind,
                    solver: identity.solver,
                    options: &identity.options,
                }
            }
            Self::Collection(harness) => {
                let identity = &harness.identity;
                HarnessView {
                    ceilings: identity.ceilings,
                    rust: &harness.rust,
                    selection: format!("{}::{}", identity.module_symbol, identity.harness_symbol),
                    kind: None,
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
    use ix_trace_rs::trace;

    /// A state-clause harness reports the contract role of what it proves, so execution
    /// evidence tells an operation-contract proof from a frame-effect proof.
    #[trace("TC-027")]
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
    #[trace("FR-028-AC-4")]
    #[test]
    fn unranged_state_draws_record_the_full_i64_domain() {
        let mut harness = state_frame_harness(
            StateFrameProperty::Frame {
                granted: vec!["input".to_owned()],
                checked: vec!["audit".to_owned()],
            },
            Vec::new(),
        );
        harness.identity.state_fields = vec!["input".to_owned(), "audit".to_owned()];
        harness.identity.domains = vec![crate::kani::identity::StateFieldDomain {
            field: "input".to_owned(),
            minimum: -3,
            maximum: 7,
        }];
        assert_eq!(
            KaniExecutableHarness::from(&harness).symbolic_arguments(),
            vec![
                SymbolicArgumentBounds {
                    identifier: "input".to_owned(),
                    bounds: SymbolicBounds::Integer {
                        minimum: -3,
                        maximum: 7
                    }
                },
                SymbolicArgumentBounds {
                    identifier: "audit".to_owned(),
                    bounds: SymbolicBounds::Integer {
                        minimum: i64::MIN,
                        maximum: i64::MAX
                    }
                },
            ]
        );
    }
}
