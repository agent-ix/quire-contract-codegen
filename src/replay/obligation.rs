//! Structural obligation identities for function and frame replay.
//!
//! QSL supplies the node and occurrence ordinal. Kind and argument domains remain separate
//! claim facts checked before replay; they do not enter the identity.

use std::fmt;

use qsl_replay::{Identifier, ObligationIdentity, OccurrenceKey, OperationSite, WireNodeId};

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
    /// The QSL occurrence ordinal cannot fit the structural identity.
    OrdinalTooLarge(u64),
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
            Self::OrdinalTooLarge(ordinal) => {
                write!(f, "the occurrence ordinal {ordinal} exceeds u32")
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
    pub(crate) parameter: WireNodeId,
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
    parameters: &[(Identifier, WireNodeId)],
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

/// The structural obligation identity over QSL's node and occurrence ordinal.
///
/// A frame has no parameters, so its `function` is the frame node, its `declaration` is the
/// frame's own occurrence and its `arguments` are empty ([`frame_identity`]).
///
/// The caller separately compares kind and argument domains with its retained claim record.
///
/// # Errors
///
/// [`ObligationIdentityError::OrdinalTooLarge`] for an occurrence beyond `u32`.
pub(crate) fn function_contract_identity(
    function: WireNodeId,
    declaration: &OccurrenceKey,
    _kind: ObligationKind,
    _arguments: &[ContractArgument],
) -> Result<ObligationIdentity, ObligationIdentityError> {
    let ordinal = u32::try_from(declaration.origin().ordinal())
        .map_err(|_| ObligationIdentityError::OrdinalTooLarge(declaration.origin().ordinal()))?;
    Ok(ObligationIdentity::new(function, None, ordinal))
}

/// The kind of the obligation a state-frame harness proves, read from its `property` and from
/// nothing a caller supplies: `None` for a property that is not a frame.
pub(crate) fn frame_kind(property: &StateFrameProperty) -> Option<ObligationKind> {
    match property {
        StateFrameProperty::Frame { .. } => Some(ObligationKind::Frame),
        StateFrameProperty::Postcondition { .. } => None,
    }
}

/// The identity of the frame obligation of the operation `site` names: [`function_contract_identity`]
/// with the frame node as `function`, the frame's occurrence as `declaration`, `kind` as
/// [`frame_kind`] derived it and no `arguments`. The state fields and their ranges are not
/// members: the frame node names the grants, and the harness is tied to the identity by the
/// replay's checks (AD-003 E-1).
///
/// # Errors
///
/// [`ObligationIdentityError::OrdinalTooLarge`] for an occurrence beyond `u32`.
pub(crate) fn frame_identity(
    site: &OperationSite,
    kind: ObligationKind,
) -> Result<ObligationIdentity, ObligationIdentityError> {
    function_contract_identity(site.frame.clone(), &site.frame_occurrence, kind, &[])
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use qsl_replay::{Origin, Role};
    use quire_exact::PathSegment;

    fn node(name: &str) -> WireNodeId {
        WireNodeId::new("fixture", vec![PathSegment::Text(name.to_owned())], 0)
            .expect("fixture node")
    }

    fn occurrence(node: WireNodeId, ordinal: u64) -> OccurrenceKey {
        OccurrenceKey::new(node, Origin::new(Role::new("declaration"), ordinal))
    }

    #[trace("FR-016-AC-21", "FR-016-AC-23", "TC-026")]
    #[test]
    fn function_obligation_uses_the_structural_node_and_ordinal() {
        let function = node("function");
        let declaration = occurrence(function.clone(), 2);
        let identity = function_contract_identity(
            function.clone(),
            &declaration,
            ObligationKind::Postcondition,
            &[],
        )
        .expect("valid ordinal");
        assert_eq!(identity.node(), &function);
        assert_eq!(identity.model(), None);
        assert_eq!(identity.ordinal(), 2);
        assert_ne!(
            identity,
            function_contract_identity(
                node("other"),
                &declaration,
                ObligationKind::Postcondition,
                &[],
            )
            .expect("valid ordinal")
        );
        assert_ne!(
            identity,
            function_contract_identity(
                function.clone(),
                &occurrence(function.clone(), 3),
                ObligationKind::Postcondition,
                &[],
            )
            .expect("valid ordinal")
        );
        assert_eq!(
            identity,
            function_contract_identity(function, &declaration, ObligationKind::Invariant, &[],)
                .expect("kind is a separate claim field")
        );
    }

    #[trace("FR-024-AC-20", "TC-035")]
    #[test]
    fn frame_obligation_uses_qsl_frame_and_occurrence() {
        let frame = node("frame");
        let site = OperationSite {
            frame_operation: qsl_replay::FrameOperation {
                object: qsl_replay::QualifiedName::new(vec![
                    Identifier::new("model").expect("alias"),
                    Identifier::new("Object").expect("object"),
                ])
                .expect("qualified object"),
                operation: Identifier::new("operation").expect("operation"),
            },
            anchor: node("anchor"),
            frame: frame.clone(),
            frame_occurrence: occurrence(frame.clone(), 2),
            clauses: Vec::new(),
        };
        let identity = frame_identity(&site, ObligationKind::Frame).expect("valid ordinal");
        assert_eq!(identity.node(), &frame);
        assert_eq!(identity.ordinal(), 2);
        assert_eq!(
            identity,
            function_contract_identity(frame, &site.frame_occurrence, ObligationKind::Frame, &[],)
                .expect("valid ordinal")
        );
    }

    #[trace("FR-016-AC-21", "TC-026")]
    #[test]
    fn ordinal_outside_structural_identity_range_is_refused() {
        let function = node("function");
        let declaration = occurrence(function.clone(), u64::from(u32::MAX) + 1);
        assert!(matches!(
            function_contract_identity(function, &declaration, ObligationKind::Postcondition, &[]),
            Err(ObligationIdentityError::OrdinalTooLarge(_))
        ));
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
