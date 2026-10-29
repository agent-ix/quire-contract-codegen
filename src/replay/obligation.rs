//! The obligation identity of the function path and of the frame path (ADR-013 O-09,
//! FR-016-AC-21 to AC-23, FR-024-AC-20, AD-003 E-1).
//!
//! The identity is the SHA-256 of the RFC 8785 encoding, made by [`crate::core::canonical`], of
//! four members: the function's node id, its `declaration` occurrence key, the
//! [`ObligationKind`] of the harness replayed and the arguments, each a parameter node id with its
//! declared domain, ascending by identifier. The function node id and the occurrence key are the
//! ones `qsl_replay::call_site` returns in its `FunctionSite`; this module takes them as given and
//! derives neither, and it reads nothing else of the compiled package. The source span is not a
//! member. O-09 fixes the members and not their spelling; the member names below are CG's.
//! A frame obligation uses the same four members with the frame node as `function`, the frame's
//! occurrence as `declaration` and no arguments ([`frame_identity`]).

use std::fmt;

use qsl_replay::{Identifier, ObligationIdentity, OccurrenceKey, OperationSite, WireNodeId};
use quire_canonical::FixedShape;
use serde::Serialize;

use crate::{
    core::canonical::{content_digest, DigestError},
    kani::{
        abi::KaniPrimitiveType,
        identity::{ObligationBinding, ObligationKind, StateFrameProperty},
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

/// The obligation identity over `function`, its `declaration` occurrence key, the `kind` of the
/// harness replayed and `arguments`, which are put in ascending order of identifier whatever
/// order they arrive in. It is the one function that mints an identity, for the function path
/// and for the frame path (FR-024-AC-20): nothing else encodes or digests an obligation's
/// members, and the digest is [`content_digest`]'s.
///
/// A frame has no parameters, so its `function` is the frame node, its `declaration` is the
/// frame's own occurrence and its `arguments` are empty ([`frame_identity`]).
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
    let preimage = obligation_preimage(function, declaration, kind, arguments);
    content_digest(&preimage)
        .map(ObligationIdentity::from_digest)
        .map_err(ObligationIdentityError::Digest)
}

fn obligation_preimage<'a>(
    function: WireNodeId,
    declaration: &OccurrenceKey,
    kind: ObligationKind,
    arguments: &'a [ContractArgument],
) -> Preimage<'a> {
    let mut ascending: Vec<&ContractArgument> = arguments.iter().collect();
    ascending.sort_by(|left, right| left.identifier.cmp(&right.identifier));
    Preimage {
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
    }
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
/// [`ObligationIdentityError::Digest`] when the preimage has no RFC 8785 encoding.
pub(crate) fn frame_identity(
    site: &OperationSite,
    kind: ObligationKind,
) -> Result<ObligationIdentity, ObligationIdentityError> {
    function_contract_identity(site.frame, &site.frame_occurrence, kind, &[])
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use qsl_replay::{call_site, DependencyInput, Origin, QualifiedName, Role, SourceIdentity};

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
    #[trace("FR-016-AC-23", "TC-026")]
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
    #[trace("FR-016-AC-21", "TC-026")]
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

    /// QSL admits the function and reports parameters in declared `z, a` order. The object
    /// passed to the sole encoder carries its exact site members and sorts arguments by name.
    #[trace("FR-016-AC-21", "FR-024-AC-20", "TC-026", "TC-035")]
    #[test]
    fn tc_026_admitted_function_site_has_exact_four_preencoding_members() {
        let source = "language \"ix:native\" edition \"1-draft\";\n\
            profile v = \"quire.value.complete/v1\";\n\
            function ordered using v(z: Boolean, a: Int[0, 9]): Boolean pure { z or a <= 9 }\n";
        let selection = QualifiedName::new(vec![Identifier::new("ordered").unwrap()]).unwrap();
        let site = call_site(
            SourceIdentity::new("agent-ix", "test:obligation-seam", "git", "r1"),
            "test:obligation-seam",
            source.as_bytes(),
            [],
            &DependencyInput::default(),
            &selection,
        )
        .expect("QSL admits the fixture")
        .site;
        assert_eq!(
            site.parameters
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>(),
            ["z", "a"]
        );
        let arguments = site
            .parameters
            .iter()
            .map(|(name, parameter)| ContractArgument {
                identifier: name.as_str().to_owned(),
                parameter: *parameter,
                domain: match name.as_str() {
                    "z" => ArgumentDomain::Boolean,
                    "a" => range("0", "9"),
                    other => panic!("unexpected parameter {other}"),
                },
            })
            .collect::<Vec<_>>();
        let preimage = obligation_preimage(
            site.function,
            &site.declaration,
            ObligationKind::Postcondition,
            &arguments,
        );
        assert_eq!(
            serde_json::to_value(&preimage).unwrap(),
            serde_json::json!({
                "function": site.function.to_string(),
                "declaration": {
                    "node": site.declaration.node().to_string(),
                    "role": site.declaration.origin().role().to_string(),
                    "ordinal": site.declaration.origin().ordinal(),
                },
                "kind": "postcondition",
                "arguments": [
                    {"domain": {"type": "integerRange", "minimum": "0", "maximum": "9"}, "parameter": site.parameters[1].1.to_string()},
                    {"domain": {"type": "boolean"}, "parameter": site.parameters[0].1.to_string()},
                ],
            })
        );
    }

    fn frame_site(frame: u8, role: &str, ordinal: u64) -> OperationSite {
        let frame = node(frame);
        OperationSite {
            frame_operation: qsl_replay::FrameOperation {
                object: qsl_replay::QualifiedName::new(vec![
                    Identifier::new("model").expect("model alias"),
                    Identifier::new("Object").expect("object name"),
                ])
                .expect("qualified object name"),
                operation: Identifier::new("operation").expect("operation name"),
            },
            anchor: node(0xa0),
            frame,
            frame_occurrence: OccurrenceKey::new(frame, Origin::new(Role::new(role), ordinal)),
            clauses: Vec::new(),
        }
    }

    /// The frame identity is the function path's one function over the frame node, the frame's
    /// own occurrence and no arguments: it equals that call, and changes when the kind, the
    /// frame node, or the occurrence's node, role or ordinal changes alone. The frame node is
    /// the only place a node enters: the anchor is not a member.
    #[trace("FR-024-AC-20", "FR-024-AC-21", "FR-024-AC-22", "TC-035")]
    #[test]
    fn tc_035_a_frame_identity_is_the_one_function_over_the_frame_and_its_occurrence() {
        let site = frame_site(7, "generated", 2);
        let base = frame_identity(&site, ObligationKind::Frame).expect("encodes");
        assert_eq!(
            base,
            function_contract_identity(
                site.frame,
                &site.frame_occurrence,
                ObligationKind::Frame,
                &[]
            )
            .expect("encodes")
        );

        let mint = |site: &OperationSite, kind| frame_identity(site, kind).expect("encodes");
        assert_ne!(base, mint(&site, ObligationKind::Postcondition), "kind");
        assert_ne!(
            base,
            mint(&frame_site(8, "generated", 2), ObligationKind::Frame),
            "function node"
        );
        assert_ne!(
            base,
            mint(&frame_site(7, "generated", 3), ObligationKind::Frame),
            "occurrence ordinal"
        );
        assert_ne!(
            base,
            mint(&frame_site(7, "claim", 2), ObligationKind::Frame),
            "occurrence role"
        );
        let mut moved = site.clone();
        moved.frame_occurrence =
            OccurrenceKey::new(node(9), site.frame_occurrence.origin().clone());
        assert_ne!(base, mint(&moved, ObligationKind::Frame), "occurrence node");
        let mut other_anchor = site.clone();
        other_anchor.anchor = node(0xb0);
        other_anchor.clauses = Vec::new();
        assert_eq!(base, mint(&other_anchor, ObligationKind::Frame), "anchor");
    }

    /// The kind is read from the harness's property: a frame is the `frame` kind and a
    /// postcondition has none.
    #[trace("FR-024-AC-22", "TC-035")]
    #[test]
    fn tc_035_the_frame_kind_is_derived_from_the_property() {
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

    /// The one minting function encodes and digests through `core::canonical` only: the
    /// non-test source names no encoder function of `quire_canonical`, no `sha2` and no
    /// `ByteDigest::of`, and `sha2` is not in `[dependencies]`.
    #[trace("FR-024-AC-20", "TC-035")]
    #[test]
    fn tc_035_the_identity_is_encoded_and_digested_only_through_core_canonical() {
        let source = include_str!("obligation.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("the source has a production part")
            .replace("use quire_canonical::FixedShape;", "");
        for banned in [
            "quire_canonical",
            "sha2",
            "Sha256",
            "sha256",
            "ByteDigest::of",
            "to_vec",
            "Writer",
            "encode(",
        ] {
            assert!(
                !production.contains(banned),
                "the production source names `{banned}`"
            );
        }
        let manifest = include_str!("../../Cargo.toml");
        let dependencies = manifest
            .split("[dependencies]")
            .nth(1)
            .and_then(|rest| rest.split("\n[").next())
            .expect("the manifest has a dependencies table");
        assert!(!dependencies.contains("sha2"));
    }
}
