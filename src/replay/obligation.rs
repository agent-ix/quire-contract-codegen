//! Structural obligation identities for function and frame replay.
//!
//! QSL supplies the node and occurrence ordinal. Kind and argument domains remain separate
//! claim facts checked before replay; they do not enter the identity.

use std::fmt;

use qsl_replay::{Identifier, NodeKey, ObligationIdentity, OccurrenceKey, OperationSite};

use crate::kani::{
    abi::KaniPrimitiveType,
    identity::{ObligationBinding, ObligationKind, StateFrameProperty},
};

/// Why a function-contract identity could not be built.
#[derive(Debug)]
pub enum ObligationIdentityError {
    /// A harness argument names no parameter of the function.
    UnboundArgument {
        /// The harness argument's identifier.
        argument: String,
    },
    /// A parameter of the function has no harness argument: O-09's `arguments` are the
    /// function's parameters, and replay refuses a parameter with no value.
    UnboundParameter {
        /// The parameter's declared identifier.
        parameter: String,
    },
    /// A harness argument declares no finite domain: an integer with no bound is
    /// `requires-bound` and is never narrowed implicitly (AD-016 arrow 5).
    UnboundedDomain {
        /// The harness argument's identifier.
        argument: String,
    },
    /// A harness argument is a Boolean that carries integer bounds.
    BoundedBoolean {
        /// The harness argument's identifier.
        argument: String,
    },
}

impl fmt::Display for ObligationIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundArgument { argument } => {
                write!(
                    f,
                    "the harness argument `{argument}` names no parameter of the function"
                )
            }
            Self::UnboundParameter { parameter } => {
                write!(f, "the parameter `{parameter}` has no harness argument")
            }
            Self::UnboundedDomain { argument } => {
                write!(f, "the harness argument `{argument}` declares no bound")
            }
            Self::BoundedBoolean { argument } => {
                write!(
                    f,
                    "the Boolean harness argument `{argument}` carries integer bounds"
                )
            }
        }
    }
}

impl std::error::Error for ObligationIdentityError {}

/// The declared domain of one argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ArgumentDomain {
    /// A Boolean: both values.
    Boolean,
    /// An inclusive integer range. The bounds are decimal strings: RFC 8785 numbers carry no
    /// integer above 2^53 exactly, and an `i64` bound may lie above it.
    IntegerRange {
        /// The inclusive minimum.
        minimum: String,
        /// The inclusive maximum.
        maximum: String,
    },
}

impl ArgumentDomain {
    /// The domain `binding` declares: its IR integer bounds, or both values of a Boolean.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError::UnboundedDomain`] for an integer with no bound and
    /// [`ObligationIdentityError::BoundedBoolean`] for a Boolean with bounds.
    pub(crate) fn of(binding: &ObligationBinding) -> Result<Self, ObligationIdentityError> {
        match (&binding.integer_bounds, binding.primitive_type) {
            (Some(bounds), KaniPrimitiveType::I64) => Ok(Self::IntegerRange {
                minimum: bounds.minimum.to_string(),
                maximum: bounds.maximum.to_string(),
            }),
            (None, KaniPrimitiveType::Boolean) => Ok(Self::Boolean),
            (None, KaniPrimitiveType::I64) => Err(ObligationIdentityError::UnboundedDomain {
                argument: binding.identifier.clone(),
            }),
            (Some(_), KaniPrimitiveType::Boolean) => Err(ObligationIdentityError::BoundedBoolean {
                argument: binding.identifier.clone(),
            }),
        }
    }
}

/// One argument of the obligation: the parameter's declared identifier, which fixes the order, its
/// node id and its declared domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ContractArgument {
    /// The parameter's declared identifier.
    pub(crate) identifier: String,
    /// The parameter's node id, from the `FunctionSite`.
    pub(crate) parameter: NodeKey,
    /// The declared domain.
    pub(crate) domain: ArgumentDomain,
}

/// The arguments of the obligation `bindings` declare, each joined by identifier to the
/// parameter of `parameters` it names. O-09's `arguments` are the function's parameters, so the
/// bindings must cover every parameter.
///
/// # Errors
///
/// [`ObligationIdentityError::UnboundArgument`] when a binding names no parameter,
/// [`ObligationIdentityError::UnboundParameter`] when a parameter has no binding, and the
/// domain refusals of [`ArgumentDomain::of`].
pub(crate) fn contract_arguments(
    parameters: &[(Identifier, NodeKey)],
    bindings: &[ObligationBinding],
) -> Result<Vec<ContractArgument>, ObligationIdentityError> {
    let arguments = bindings
        .iter()
        .map(|binding| {
            let (_, parameter) = parameters
                .iter()
                .find(|(name, _)| name.as_str() == binding.identifier)
                .ok_or_else(|| ObligationIdentityError::UnboundArgument {
                    argument: binding.identifier.clone(),
                })?;
            Ok(ContractArgument {
                identifier: binding.identifier.clone(),
                parameter: parameter.clone(),
                domain: ArgumentDomain::of(binding)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    match parameters.iter().find(|(name, _)| {
        !arguments
            .iter()
            .any(|argument| argument.identifier == name.as_str())
    }) {
        Some((name, _)) => Err(ObligationIdentityError::UnboundParameter {
            parameter: name.as_str().to_owned(),
        }),
        None => Ok(arguments),
    }
}

/// The producer-issued structural obligation identity for QSL's selected claim.
///
/// A frame has no parameters, so its `function` is the frame node, its `declaration` is the
/// frame's own occurrence and its `arguments` are empty ([`frame_identity`]).
///
/// The caller separately compares kind and argument domains with its retained claim record.
///
pub(crate) fn function_contract_identity(
    producer: &ObligationIdentity,
    _declaration: &OccurrenceKey,
    _kind: ObligationKind,
    _arguments: &[ContractArgument],
) -> Result<ObligationIdentity, ObligationIdentityError> {
    Ok(producer.clone())
}

/// The kind of the obligation a state-frame harness proves, read from its `property` and from
/// nothing a caller supplies: `None` for a property that is not a frame.
pub(crate) fn frame_kind(property: &StateFrameProperty) -> Option<ObligationKind> {
    match property {
        StateFrameProperty::Frame { .. } => Some(ObligationKind::Frame),
        StateFrameProperty::Postcondition { .. } => None,
    }
}

/// The frame obligation of the operation `site` names, issued by QSL's selected operation site.
///
pub(crate) fn frame_identity(
    site: &OperationSite,
    kind: ObligationKind,
) -> Result<ObligationIdentity, ObligationIdentityError> {
    function_contract_identity(
        site.obligation_identity(),
        &site.frame_occurrence,
        kind,
        &[],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use qsl_replay::{Origin, Role};
    use quire_exact::PathSegment;

    fn node(name: &str) -> NodeKey {
        NodeKey::new("fixture", vec![PathSegment::Text(name.to_owned())], 0).expect("fixture node")
    }

    fn occurrence(node: NodeKey, ordinal: u64) -> OccurrenceKey {
        OccurrenceKey::new(node, Origin::new(Role::new("declaration"), ordinal))
    }

    #[trace("FR-016-AC-26", "TC-026")]
    #[test]
    fn large_declaration_ordinal_preserves_producer_identity() {
        let function = node("function");
        let declaration = occurrence(function.clone(), u64::from(u32::MAX) + 1);
        let producer = ObligationIdentity::new(function, None, 0);
        assert_eq!(
            function_contract_identity(&producer, &declaration, ObligationKind::Postcondition, &[])
                .expect("the occurrence does not narrow the producer identity"),
            producer
        );
    }

    #[trace("FR-024-AC-22", "TC-035")]
    #[test]
    fn frame_kind_is_derived_from_property() {
        use crate::kani::identity::StateComparison;
        let frame = StateFrameProperty::Frame {
            granted: Vec::new(),
            checked: Vec::new(),
        };
        let postcondition = StateFrameProperty::Postcondition {
            field: "balance".to_owned(),
            comparison: StateComparison::Ge,
            left_is_pre: false,
        };
        assert_eq!(frame_kind(&frame), Some(ObligationKind::Frame));
        assert_eq!(frame_kind(&postcondition), None);
    }
}
