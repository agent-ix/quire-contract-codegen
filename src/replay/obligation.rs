//! The function-contract obligation identity of the function path (ADR-013 O-09, FR-016-AC-21 to
//! AC-23, AD-003 E-1).
//!
//! The identity is the SHA-256 of the RFC 8785 encoding, made by [`crate::core::canonical`], of
//! four members: the function's node id, its `declaration` occurrence key, the
//! [`ObligationKind`] of the harness replayed and the arguments, each a parameter node id with its
//! declared domain, ascending by identifier. The function node id and the occurrence key are the
//! ones `qsl_replay::call_site` returns in its `FunctionSite`; this module takes them as given and
//! derives neither, and it reads nothing else of the compiled package. The source span is not a
//! member. O-09 fixes the members and not their spelling; the member names below are CG's.

use std::fmt;

use qsl_replay::{Identifier, ObligationIdentity, OccurrenceKey, WireNodeId};
use quire_canonical::FixedShape;
use serde::Serialize;

use crate::{
    core::canonical::{content_digest, DigestError},
    kani::{
        abi::KaniPrimitiveType,
        identity::{ObligationBinding, ObligationKind},
    },
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
    /// The preimage has no RFC 8785 encoding.
    Digest(DigestError),
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
            Self::Digest(cause) => cause.fmt(f),
        }
    }
}

impl std::error::Error for ObligationIdentityError {}

/// The declared domain of one argument.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, FixedShape)]
#[serde(tag = "type", rename_all = "camelCase")]
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
                parameter: *parameter,
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

#[derive(Serialize, FixedShape)]
#[serde(rename_all = "camelCase")]
struct OccurrenceMember {
    node: String,
    role: String,
    ordinal: u64,
}

#[derive(Serialize, FixedShape)]
#[serde(rename_all = "camelCase")]
struct ArgumentMember<'a> {
    parameter: String,
    domain: &'a ArgumentDomain,
}

/// The O-09 preimage members of a function contract.
#[derive(Serialize, FixedShape)]
#[serde(rename_all = "camelCase")]
struct Preimage<'a> {
    function: String,
    declaration: OccurrenceMember,
    kind: ObligationKind,
    arguments: Vec<ArgumentMember<'a>>,
}

/// The function-contract obligation identity over `function`, its `declaration` occurrence key,
/// the `kind` of the harness replayed and `arguments`, which are put in ascending order of
/// identifier whatever order they arrive in.
///
/// # Errors
///
/// [`ObligationIdentityError::Digest`] when the preimage has no RFC 8785 encoding.
pub(crate) fn function_contract_identity(
    function: WireNodeId,
    declaration: &OccurrenceKey,
    kind: ObligationKind,
    arguments: &[ContractArgument],
) -> Result<ObligationIdentity, ObligationIdentityError> {
    let mut ascending: Vec<&ContractArgument> = arguments.iter().collect();
    ascending.sort_by(|left, right| left.identifier.cmp(&right.identifier));
    let preimage = Preimage {
        function: function.to_string(),
        declaration: OccurrenceMember {
            node: declaration.node().to_string(),
            role: declaration.origin().role().to_string(),
            ordinal: declaration.origin().ordinal(),
        },
        kind,
        arguments: ascending
            .into_iter()
            .map(|argument| ArgumentMember {
                parameter: argument.parameter.to_string(),
                domain: &argument.domain,
            })
            .collect(),
    };
    content_digest(&preimage)
        .map(ObligationIdentity::from_digest)
        .map_err(ObligationIdentityError::Digest)
}

#[cfg(test)]
mod tests {
    use qsl_replay::{Origin, Role};

    use super::*;

    fn node(byte: u8) -> WireNodeId {
        WireNodeId::from_digest([byte; 32])
    }

    fn declaration(function: WireNodeId) -> OccurrenceKey {
        OccurrenceKey::new(function, Origin::new(Role::new("declaration"), 0))
    }

    fn argument(identifier: &str, parameter: u8, domain: ArgumentDomain) -> ContractArgument {
        ContractArgument {
            identifier: identifier.to_owned(),
            parameter: node(parameter),
            domain,
        }
    }

    fn range(minimum: &str, maximum: &str) -> ArgumentDomain {
        ArgumentDomain::IntegerRange {
            minimum: minimum.to_owned(),
            maximum: maximum.to_owned(),
        }
    }

    fn arguments() -> Vec<ContractArgument> {
        vec![
            argument("a", 1, range("0", "9")),
            argument("b", 2, ArgumentDomain::Boolean),
        ]
    }

    fn identity(
        function: u8,
        declaration_node: u8,
        kind: ObligationKind,
        arguments: &[ContractArgument],
    ) -> ObligationIdentity {
        function_contract_identity(
            node(function),
            &declaration(node(declaration_node)),
            kind,
            arguments,
        )
        .expect("the preimage encodes")
    }

    /// The identity depends on the function, the declaration key, the kind and the arguments
    /// only: arguments given in any order give one identity, and each member changes it.
    ///
    /// Trace: FR-016-AC-23, TC-026
    #[test]
    fn tc_026_only_the_four_members_decide_the_identity() {
        let kind = ObligationKind::Postcondition;
        let base = identity(7, 7, kind, &arguments());

        let mut reversed = arguments();
        reversed.reverse();
        assert_eq!(base, identity(7, 7, kind, &reversed));

        assert_ne!(base, identity(8, 7, kind, &arguments()), "function");
        assert_ne!(base, identity(7, 8, kind, &arguments()), "declaration");
        assert_ne!(
            base,
            identity(7, 7, ObligationKind::Precondition, &arguments()),
            "kind"
        );
        let mut rebound = arguments();
        rebound[0].parameter = node(3);
        assert_ne!(base, identity(7, 7, kind, &rebound), "parameter node id");
        let mut widened = arguments();
        widened[0].domain = range("0", "10");
        assert_ne!(base, identity(7, 7, kind, &widened), "domain");
    }

    /// An `i64` bound above 2^53 is a decimal string in the preimage, so the encoder accepts it
    /// and the identity tells the extreme domain from a near one.
    ///
    /// Trace: FR-016-AC-21, TC-026
    #[test]
    fn tc_026_an_i64_extreme_domain_is_encodable() {
        let kind = ObligationKind::Invariant;
        let extreme = [argument(
            "a",
            1,
            range(&i64::MIN.to_string(), &i64::MAX.to_string()),
        )];
        let near = [argument(
            "a",
            1,
            range(&i64::MIN.to_string(), &(i64::MAX - 1).to_string()),
        )];
        assert_ne!(identity(7, 7, kind, &extreme), identity(7, 7, kind, &near));
    }

    /// Ascending by identifier is not ascending by node id: with `a` bound to a node id above
    /// `b`'s, the digest equals the hand-written text that lists `a` first. A sort by node id
    /// would list `b` first and fail.
    ///
    /// Trace: FR-016-AC-21, TC-026
    #[test]
    fn tc_026_arguments_are_ordered_by_identifier_not_by_node_id() {
        use sha2::{Digest, Sha256};
        let hex = |byte: u8| format!("{byte:02x}").repeat(32);
        let arguments = [
            argument("b", 1, ArgumentDomain::Boolean),
            argument("a", 9, range("0", "9")),
        ];
        let text = format!(
            "{{\"arguments\":[{{\"domain\":{{\"maximum\":\"9\",\"minimum\":\"0\",\
             \"type\":\"integerRange\"}},\"parameter\":\"{}\"}},\
             {{\"domain\":{{\"type\":\"boolean\"}},\"parameter\":\"{}\"}}],\
             \"declaration\":{{\"node\":\"{}\",\"ordinal\":0,\"role\":\"declaration\"}},\
             \"function\":\"{}\",\"kind\":\"frame\"}}",
            hex(9),
            hex(1),
            hex(7),
            hex(7),
        );
        let expected: [u8; 32] = Sha256::digest(text.as_bytes()).into();
        let built = identity(7, 7, ObligationKind::Frame, &arguments);
        assert_eq!(*built.as_bytes(), expected);
    }
}
