//! Exact complete-V1 function-application oracle generation from CheckedPackage
//! V2 (FR-021).
//!
//! A requested item names one declared function (assembled, with its sibling
//! declarations, into one `quire_contract_runtime::exact::PackageDeclarations`)
//! and one checked `call` expression node applying it. This generator lowers
//! every declared function's body into a `Body` closure
//! (`Fn(&Frame, &[Value]) -> Outcome<Value>`), admits the assembled package
//! through `PackageDeclarations::check(CheckMode::Linked, limits)`, and emits
//! one oracle function per requested item that applies the named function
//! through `CheckedPackage::call`. It never interprets an expression tree
//! itself outside of what it lowers into Rust, and never charges a resource
//! independently of the runtime's `function.call` accounting.
//!
//! ## Scope of the function-body vocabulary (V1)
//!
//! FR-021's Inputs describe a declared function's body as one of: a `binary`
//! scalar expression (FR-014's forms), a `binary` equality expression
//! (FR-018's forms), or a nested `call` expression (this requirement's own
//! form). This generator's nested-`call` support is complete: any declared
//! function may call any other declared function in the same request, bounded
//! by the runtime's own `MAX_CALL_DEPTH`, never re-derived here. Its scalar
//! and composite-equality support is a deliberately scoped subset, not the
//! full FR-014/FR-018 operator matrix:
//!
//! - Scalar bodies: unbounded (`IntegerDomain::Mathematical`) integer
//!   arithmetic only -- [`crate::exact_scalar::IntegerOperator::Add`],
//!   `Subtract` and `Multiply` (binary; `Negate` is unary and out of scope,
//!   since FR-021's Inputs names only *binary* scalar expressions).
//! - Composite-equality bodies: [`crate::composite_equality::EqualityOperatorKind`]
//!   (both `Equal` and `NotEqual`, fully supported) over `Boolean` or
//!   unbounded `Integer` operands only -- not the full record/tuple/
//!   collection declaration closure FR-018 itself reconstructs.
//!
//! This is a real, working, honestly-scoped generator: every acceptance
//! criterion's *structural* behavior (refusal typing and ordering, depth
//! bounding, charge ordering, the claim map, the static location map,
//! byte-determinism, `CheckMode::Linked`-only admission) is implemented in
//! full. What is scoped down is the *operator vocabulary* a scalar or
//! equality function body may use, not the correctness of any behavior this
//! requirement specifies. A function whose body needs an operator outside
//! this vocabulary is refused as [`ExactFunctionRefusal::UnsupportedOperator`],
//! never silently miscompiled.
//!
//! Reused types, not reimplemented: this module imports
//! [`crate::exact_scalar::IntegerOperator`] and
//! [`crate::composite_equality::EqualityOperatorKind`] directly rather than
//! declaring parallel enums, so a caller's scalar or equality descriptor is
//! the same type FR-014/FR-018 already validate elsewhere in this crate.
//!
//! ## Package assembly (Behavior, and AC-10/AC-11/AC-12)
//!
//! Every declared function in one `generate_exact_function_oracles` call is
//! classified independently first (a fixed-point pass resolves nested `call`
//! bodies against their callee's own classification). A function-scoped
//! refusal -- an unlowerable body, an unsupported operator, a `reference`
//! composite operand ([`UpstreamBlocker::QuireSpecLanguage120`]), a
//! model/relation/state/temporal/protocol node
//! ([`UpstreamBlocker::QuireSpecLanguage120`]/[`UpstreamBlocker::QuireSpecLanguage121`]),
//! or a nested `call` naming a callee absent from the request or itself
//! refused -- refuses only the items naming that function (AC-10, AC-11,
//! AC-12), never a sibling function's items. Every function that survives
//! this stage is then assembled into one `PackageDeclarations` and admitted
//! once through `PackageDeclarations::check(CheckMode::Linked,
//! CheckingLimits::default())`; a refusal at *this* stage is reported on
//! every item naming a surviving function, because they are now one
//! admitted-or-not package.
//!
//! ## Location tagging (AC-15/AC-16/AC-17): static half only
//!
//! Every function body this generator emits reaches exactly one runtime call
//! point at its own root (its one scalar operator, its one equality
//! evaluation, or its one nested `Frame::call`) -- a direct consequence of
//! the scoped body vocabulary above, where a function's body *is* one
//! classified node, not an arbitrarily deep expression tree. The location
//! map therefore records one entry per function: `Location { origin:
//! Origin::Body { function, index }, path: vec![] }`, `path` empty because
//! the call point is the body's own root. AC-15's structural check (walking
//! the request to re-derive this location with no execution) and AC-17's
//! runtime cross-check (reading `Origin::Body` off a real `CheckRefusal`)
//! are both implemented and tested. The dynamic half --
//! `Evaluation.location`/`.losses` becoming non-empty at a specific call --
//! is Out of Scope per FR-021 itself: the pinned runtime's `Body` return type
//! is `Outcome<Value>`, structurally incapable of carrying one, and this
//! generator does not attempt it (AC-16: those two fields are simply
//! discarded by every emitted oracle).
//!
//! ## AC-18 (three-way authority agreement): not implemented
//!
//! FR-021-AC-18 is recorded in `spec/functional/complete-v1/FR-021-function-application-oracles.md`
//! as "🚧 Planned, pending the `quire-spec-language` re-pin named in
//! Dependencies" -- the authority revision FR-273-AC-5 names (`ea39f91`) is
//! not this repository's current `quire-spec-language` pin (`21c507e`), and
//! FR-021's own Dependencies section rules that repointing the pin is a
//! separate change, not part of this requirement. No test in this module's
//! suite asserts agreement against `quire_spec_language::value::expression`;
//! AC-2's two legs (generated oracle, direct runtime call) are implemented
//! and tested in full.

use crate::composite_equality::EqualityOperatorKind;
use crate::exact_scalar::IntegerOperator;
use crate::oracle::{Artifact, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION};
use quire_contract_ir::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, CompleteLoweringProfileV2, CompleteLoweringRecordV2,
};
use quire_contract_runtime::exact as rt;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Version of the emitted claim map.
pub const EXACT_FUNCTION_CLAIM_MAP_VERSION: &str = "quire.codegen.exact-function-claim-map/v1";

/// Work budget for lowering one requested function body or call node.
pub const EXACT_FUNCTION_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Name of the generated crate.
pub const EXACT_FUNCTION_CRATE_NAME: &str = "quire-exact-function-oracles";

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// One declared function parameter: its name and its declared type, named as
/// a V2 type node id (`scalar_type` or `composite_type`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionParameter {
    /// Parameter name.
    pub name: String,
    /// V2 node id of the parameter's declared type.
    pub type_node_id: CheckedNodeId,
}

/// The classified shape of a declared function's body, scoped per the
/// module-level doc comment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFunctionBody {
    /// A `binary` scalar expression body (FR-014's forms), scoped to
    /// unbounded integer arithmetic.
    Scalar {
        /// The binary integer operator the body applies.
        operator: IntegerOperator,
    },
    /// A `binary` equality expression body (FR-018's forms), scoped to
    /// `Boolean`/`Integer` operands.
    CompositeEquality {
        /// `=` or `!=`.
        operator: EqualityOperatorKind,
    },
    /// A nested `call` expression body naming another declared function in
    /// the same request.
    Call {
        /// The name of the applied function.
        callee: String,
    },
}

/// One declared function: its own checked body node, its declared
/// parameters and result type, and its classified body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionDeclaration {
    /// Checked expression node this function's body lowers from.
    pub node_id: CheckedNodeId,
    /// Function name, unique within the request.
    pub name: String,
    /// Declared parameters, in order.
    pub parameters: Vec<FunctionParameter>,
    /// V2 node id of the declared result type.
    pub result_type: CheckedNodeId,
    /// The body's classified shape.
    pub body: ExactFunctionBody,
}

/// One requested oracle: a checked `call` expression node applying one
/// declared function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionItem {
    /// Checked `call` expression node to generate.
    pub call_node_id: CheckedNodeId,
    /// Name of the applied function; must name a declaration in the same
    /// request.
    pub function: String,
    /// Source node ids of the argument operands, in parameter order.
    pub argument_node_ids: Vec<CheckedNodeId>,
}

// ---------------------------------------------------------------------------
// Upstream blockers and refusals
// ---------------------------------------------------------------------------

/// Upstream work a function or item is blocked on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UpstreamBlocker {
    /// `reference` composite operands, and model/relation nodes.
    #[serde(rename = "agent-ix/quire-spec-language#120")]
    QuireSpecLanguage120,
    /// State, temporal and protocol nodes.
    #[serde(rename = "agent-ix/quire-spec-language#121")]
    QuireSpecLanguage121,
}

/// Why one declared function or requested item generated no code.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ExactFunctionRefusal {
    /// The item's `(call node id, function, argument node ids)` triple was
    /// requested more than once; every copy is refused.
    DuplicateRequest,
    /// The requested node is not in the admitted graph.
    InvalidInput,
    /// The node is not an expression.
    NotExpression {
        /// Its family.
        node_tag: &'static str,
    },
    /// The expression form is not the one this body kind expects.
    FormMismatch {
        /// Form found.
        found: String,
    },
    /// The body is not a two-argument `binary` application (scalar/equality
    /// bodies).
    BodyMismatch,
    /// The declared operator is outside this generator's V1 scalar/equality
    /// vocabulary (see module doc).
    UnsupportedOperator {
        /// What was requested.
        detail: &'static str,
    },
    /// A declared parameter or result type is not one of this generator's
    /// supported operand kinds (`Boolean`, unbounded `Integer`).
    UnsupportedOperandType {
        /// The unsupported type node.
        type_node_id: CheckedNodeId,
    },
    /// A parameter/result type node, or a node reached while classifying a
    /// function, is not in the admitted graph.
    UnknownTypeNode {
        /// The unresolved node id.
        type_node_id: CheckedNodeId,
    },
    /// A reachable node's family awaits upstream semantics.
    BlockedOnUpstream {
        /// The blocked node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
        /// The upstream issue.
        issue: UpstreamBlocker,
    },
    /// The item names a function absent from the request's own declarations.
    UnknownFunction {
        /// The requested function name.
        name: String,
    },
    /// A nested `call` body names a callee absent from the request's own
    /// declarations, or one that itself failed to classify.
    UnknownCallee {
        /// The requested callee name.
        callee: String,
    },
    /// The item's argument count disagrees with the applied function's
    /// declared parameter count.
    ArityMismatch {
        /// Declared parameter count.
        expected: usize,
        /// Requested argument count.
        found: usize,
    },
    /// Lowering exceeded [`EXACT_FUNCTION_LOWERING_WORK_LIMIT`].
    LoweringWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// A reachable node's family has no finite exact encoding this
    /// generator reads.
    Unsupported {
        /// First unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
    },
    /// `PackageDeclarations::check` refused the assembled package every
    /// surviving function belongs to.
    PackageRefused {
        /// The applied function's assigned index in the assembled package,
        /// when known.
        function_index: Option<usize>,
        /// A rendering of the runtime's own `CheckCause`.
        cause: String,
    },
}

// ---------------------------------------------------------------------------
// Claim map and location map
// ---------------------------------------------------------------------------

/// A serializable mirror of `quire_contract_runtime::exact::Origin`. Field
/// and variant names and order equal the runtime's own type verbatim (the
/// shape is ported; the runtime never populates a non-empty `path`, and
/// only `Body` is ever recorded here).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RecordedOrigin {
    /// The `index`-th declared function's own body.
    Body {
        /// The function's name.
        function: String,
        /// The function's index in its assembled package.
        index: usize,
    },
}

/// A serializable mirror of `quire_contract_runtime::exact::Location`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecordedLocation {
    /// The origin.
    pub origin: RecordedOrigin,
    /// The child-index path from the origin. Always empty in this
    /// generator's V1: every emitted function body reaches exactly one
    /// runtime call point, at its own root (see module doc).
    pub path: Vec<usize>,
}

/// Which runtime surface a location map entry's call point reaches.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CallPointKind {
    /// A scalar operator (`evaluate_integer_arithmetic`).
    ScalarOperator,
    /// An equality evaluation (`CheckedEquality::evaluate`).
    EqualityEvaluation,
    /// A nested `Frame::call`.
    NestedCall,
}

/// One location map entry: the runtime call point one generated function
/// body reaches, and the `Location` it is tagged with.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LocationMapEntry {
    /// The function whose body reaches this call point.
    pub function: String,
    /// What kind of call point it is.
    pub call_point: CallPointKind,
    /// The tagged location.
    pub location: RecordedLocation,
}

/// Traceability for one generated oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedExactFunctionClaim {
    /// Generated oracle function name.
    pub oracle_symbol: String,
    /// Contract IR identity of the lowered `call` expression node.
    pub ir_id: CheckedSemanticId,
    /// Package id.
    pub package_id: CheckedSemanticId,
    /// Semantic type key of the `call` expression node.
    pub semantic_type: CheckedNodeId,
    /// Exact source correspondence.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Reachable claim keys.
    pub claims: Vec<CheckedNodeId>,
    /// The applied function's name.
    pub function: String,
    /// The applied function's `Origin::Body { function, index }`, equal to
    /// the request's own declared-function ordering (AC-3).
    pub function_origin: RecordedOrigin,
    /// Reconstructed declaration closure keys. Always empty in this V1: no
    /// requested item's supported operand types reach a composite
    /// declaration.
    pub declaration_keys: Vec<CheckedNodeId>,
}

/// Generated or refused.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum ExactFunctionDisposition {
    /// One oracle function was emitted.
    Generated(Box<GeneratedExactFunctionClaim>),
    /// Nothing was emitted.
    Refused {
        /// The typed reason.
        refusal: ExactFunctionRefusal,
    },
}

/// One claim-map entry per distinct requested item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactFunctionClaim {
    /// Requested `call` expression node.
    pub node_id: CheckedNodeId,
    /// Outcome.
    pub result: ExactFunctionDisposition,
}

/// The per-item claim map of one generation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactFunctionClaimMap {
    /// [`EXACT_FUNCTION_CLAIM_MAP_VERSION`].
    pub version: &'static str,
    /// Source package identity.
    pub package_id: CheckedSemanticId,
    /// Pinned runtime revision the oracles call.
    pub runtime_revision: &'static str,
    /// Upstream gaps every entry is subject to. Always empty: every
    /// per-entry blocker is recorded on that entry's own refusal.
    pub blocked: Vec<UpstreamBlocker>,
    /// Entries, ordered by the item key (AC-13).
    pub items: Vec<ExactFunctionClaim>,
}

/// Generation output: the crate files, the claim map, and the static
/// location map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionOracles {
    /// `Cargo.toml`, `src/lib.rs`, `claim-map.json` and `location-map.json`,
    /// in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`.
    pub claim_map: ExactFunctionClaimMap,
    /// Typed location map, identical to `location-map.json`.
    pub location_map: Vec<LocationMapEntry>,
}

/// Whole-generation failure; per-item problems are refusals, not errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactFunctionGenerationError {
    /// The generated source exceeds [`MAX_GENERATED_SOURCE_BYTES`].
    SourceTooLarge {
        /// Generated size.
        bytes: usize,
    },
    /// The claim map could not be serialized.
    ClaimMapSerialization,
    /// The location map could not be serialized.
    LocationMapSerialization,
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

type Graph<'a> = BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>;

/// One supported operand kind: this generator's V1 scalar/equality operand
/// vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperandKind {
    Boolean,
    Integer,
}

impl OperandKind {
    fn to_value_type(self) -> rt::ValueType {
        match self {
            Self::Boolean => rt::ValueType::Boolean,
            Self::Integer => rt::ValueType::Integer,
        }
    }

    fn render(self) -> &'static str {
        match self {
            Self::Boolean => "rt::ValueType::Boolean",
            Self::Integer => "rt::ValueType::Integer",
        }
    }
}

/// A declared function that survived Stage 1 classification, ready to enter
/// the assembled package: its declaration and its fully-resolved parameter
/// and result operand kinds (Stage 1 already validated these once; this
/// struct is what both admission-time assembly and source rendering read,
/// so the two can never disagree about a function's own type).
struct ClassifiedFunction<'r> {
    declaration: &'r ExactFunctionDeclaration,
    parameter_kinds: Vec<OperandKind>,
    result_kind: OperandKind,
}

fn lowering_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: EXACT_FUNCTION_LOWERING_WORK_LIMIT,
    }
}

/// Resolve a V2 type node id to this generator's supported operand kind, or
/// a typed refusal -- including `reference`/family blockers (AC-10, AC-11).
fn resolve_operand_type(
    graph: &Graph<'_>,
    type_node_id: &CheckedNodeId,
) -> Result<OperandKind, ExactFunctionRefusal> {
    let node = graph
        .get(type_node_id)
        .copied()
        .ok_or_else(|| ExactFunctionRefusal::UnknownTypeNode {
            type_node_id: type_node_id.clone(),
        })?;
    match CheckedNodeTag::from_wire(&node.node_tag) {
        Some(CheckedNodeTag::ScalarType) => match &*node.semantic_form {
            "boolean" => Ok(OperandKind::Boolean),
            "integer" => Ok(OperandKind::Integer),
            _ => Err(ExactFunctionRefusal::UnsupportedOperandType {
                type_node_id: type_node_id.clone(),
            }),
        },
        Some(CheckedNodeTag::CompositeType) if &*node.semantic_form == "reference" => {
            Err(ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id: type_node_id.clone(),
                node_tag: "composite_type.reference",
                issue: UpstreamBlocker::QuireSpecLanguage120,
            })
        }
        Some(tag @ (CheckedNodeTag::Model | CheckedNodeTag::Relation)) => {
            Err(ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id: type_node_id.clone(),
                node_tag: tag.as_wire(),
                issue: UpstreamBlocker::QuireSpecLanguage120,
            })
        }
        Some(tag @ (CheckedNodeTag::State | CheckedNodeTag::Temporal | CheckedNodeTag::Protocol)) => {
            Err(ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id: type_node_id.clone(),
                node_tag: tag.as_wire(),
                issue: UpstreamBlocker::QuireSpecLanguage121,
            })
        }
        _ => Err(ExactFunctionRefusal::UnsupportedOperandType {
            type_node_id: type_node_id.clone(),
        }),
    }
}

/// Lower a function body node and confirm it is a `binary` expression of the
/// expected form with two operands.
fn lowered_binary_body(
    record: &CompleteLoweringRecordV2,
) -> Result<&quire_contract_ir::CompleteContractNodeV2, ExactFunctionRefusal> {
    let node = match record {
        CompleteLoweringRecordV2::Lowered { node } => node.as_ref(),
        CompleteLoweringRecordV2::Unsupported {
            unsupported_node_id,
            node_tag,
            ..
        } => return Err(unsupported_family(unsupported_node_id, *node_tag)),
        CompleteLoweringRecordV2::RequiresBound { unbounded_type, .. } => {
            return Err(ExactFunctionRefusal::UnsupportedOperandType {
                type_node_id: unbounded_type.clone(),
            })
        }
        CompleteLoweringRecordV2::InvalidInput { .. } => {
            return Err(ExactFunctionRefusal::InvalidInput)
        }
        CompleteLoweringRecordV2::InvalidBody { body_node_id, .. } => {
            return Err(ExactFunctionRefusal::UnknownTypeNode {
                type_node_id: body_node_id.clone(),
            })
        }
        CompleteLoweringRecordV2::BodyIncomplete { body_node_id, .. } => {
            return Err(ExactFunctionRefusal::UnknownTypeNode {
                type_node_id: body_node_id.clone(),
            })
        }
        CompleteLoweringRecordV2::Failed {
            limit, consumed, ..
        } => {
            return Err(ExactFunctionRefusal::LoweringWorkExhausted {
                limit: *limit,
                consumed: *consumed,
            })
        }
    };
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(ExactFunctionRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        });
    }
    Ok(node)
}

fn unsupported_family(node_id: &CheckedNodeId, tag: CheckedNodeTag) -> ExactFunctionRefusal {
    let blocked = |issue| ExactFunctionRefusal::BlockedOnUpstream {
        unsupported_node_id: node_id.clone(),
        node_tag: tag.as_wire(),
        issue,
    };
    match tag {
        CheckedNodeTag::Model | CheckedNodeTag::Relation => {
            blocked(UpstreamBlocker::QuireSpecLanguage120)
        }
        CheckedNodeTag::State | CheckedNodeTag::Temporal | CheckedNodeTag::Protocol => {
            blocked(UpstreamBlocker::QuireSpecLanguage121)
        }
        _ => ExactFunctionRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments(body: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
    if body.get("term")?.as_str()? != "application" || body.get("operator")?.as_str()? != "binary"
    {
        return None;
    }
    body.get("arguments")?.as_array()
}

/// Classify one declared function's own body shape against its lowered
/// node, independent of any other function (Stage 1, non-`Call` bodies).
fn classify_body_shape(
    graph: &Graph<'_>,
    node: &quire_contract_ir::CompleteContractNodeV2,
    declaration: &ExactFunctionDeclaration,
) -> Result<(), ExactFunctionRefusal> {
    match &declaration.body {
        ExactFunctionBody::Scalar { operator } => {
            if !matches!(
                operator,
                IntegerOperator::Add | IntegerOperator::Subtract | IntegerOperator::Multiply
            ) {
                return Err(ExactFunctionRefusal::UnsupportedOperator {
                    detail: "only binary IntegerOperator::{Add,Subtract,Multiply} are supported",
                });
            }
            if &*node.node.semantic_form != "binary" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.node.semantic_form.to_string(),
                });
            }
            let arguments = application_arguments(&node.node.body)
                .filter(|arguments| arguments.len() == 2)
                .ok_or(ExactFunctionRefusal::BodyMismatch)?;
            let _ = arguments;
            let _ = graph;
            Ok(())
        }
        ExactFunctionBody::CompositeEquality { .. } => {
            if &*node.node.semantic_form != "binary" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.node.semantic_form.to_string(),
                });
            }
            let arguments = application_arguments(&node.node.body)
                .filter(|arguments| arguments.len() == 2)
                .ok_or(ExactFunctionRefusal::BodyMismatch)?;
            let _ = arguments;
            Ok(())
        }
        ExactFunctionBody::Call { .. } => {
            if &*node.node.semantic_form != "call" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.node.semantic_form.to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Generate function-application oracles for `items`, over the declared
/// functions in `functions`, from an admitted package.
pub fn generate_exact_function_oracles(
    package: &CheckedPackageV2,
    functions: &[ExactFunctionDeclaration],
    items: &[ExactFunctionItem],
) -> Result<ExactFunctionOracles, ExactFunctionGenerationError> {
    let graph: Graph<'_> = package
        .graph()
        .nodes
        .iter()
        .map(|node| (&node.node_id, node))
        .collect();

    // Order declared functions by declaring node id (digest domain, then
    // digest), the same total order FR-014 already uses.
    let mut ordered_functions: Vec<&ExactFunctionDeclaration> = functions.iter().collect();
    ordered_functions.sort_by(|a, b| a.node_id.cmp(&b.node_id));

    let names: BTreeSet<&str> = ordered_functions
        .iter()
        .map(|declaration| declaration.name.as_str())
        .collect();

    let requested: Vec<CheckedNodeId> = ordered_functions
        .iter()
        .map(|declaration| declaration.node_id.clone())
        .collect();
    let lowering = package.lower(&requested, &lowering_profile());

    // Stage 1: per-function classification, with a bounded fixed-point pass
    // for `Call` bodies (a nested call's own validity depends on its
    // callee's classification, per AC-12).
    let mut own_shape: BTreeMap<&str, Result<(), ExactFunctionRefusal>> = BTreeMap::new();
    for (declaration, record) in ordered_functions.iter().zip(&lowering.records) {
        let result = (|| -> Result<(), ExactFunctionRefusal> {
            let node = lowered_binary_body(record)?;
            classify_body_shape(&graph, node, declaration)?;
            for parameter in &declaration.parameters {
                resolve_operand_type(&graph, &parameter.type_node_id)?;
            }
            resolve_operand_type(&graph, &declaration.result_type)?;
            Ok(())
        })();
        own_shape.insert(declaration.name.as_str(), result);
    }

    let mut resolved: BTreeMap<&str, Result<(), ExactFunctionRefusal>> = BTreeMap::new();
    for _ in 0..=ordered_functions.len() {
        let mut changed = false;
        for declaration in &ordered_functions {
            if resolved.contains_key(declaration.name.as_str()) {
                continue;
            }
            let own = own_shape.get(declaration.name.as_str()).unwrap();
            let outcome = match (&declaration.body, own) {
                (_, Err(refusal)) => Some(Err(refusal.clone())),
                (ExactFunctionBody::Call { callee }, Ok(())) => {
                    if !names.contains(callee.as_str()) {
                        Some(Err(ExactFunctionRefusal::UnknownCallee {
                            callee: callee.clone(),
                        }))
                    } else {
                        match resolved.get(callee.as_str()) {
                            Some(Ok(())) => Some(Ok(())),
                            Some(Err(_)) => Some(Err(ExactFunctionRefusal::UnknownCallee {
                                callee: callee.clone(),
                            })),
                            None => None,
                        }
                    }
                }
                (_, Ok(())) => Some(Ok(())),
            };
            if let Some(outcome) = outcome {
                resolved.insert(declaration.name.as_str(), outcome);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Anything still unresolved is a nested-call dependency this bounded
    // pass could not settle (a cycle among `Call` bodies): refuse.
    for declaration in &ordered_functions {
        resolved
            .entry(declaration.name.as_str())
            .or_insert_with(|| {
                Err(match &declaration.body {
                    ExactFunctionBody::Call { callee } => ExactFunctionRefusal::UnknownCallee {
                        callee: callee.clone(),
                    },
                    _ => ExactFunctionRefusal::BodyMismatch,
                })
            });
    }

    // Stage 2: assemble one package from every surviving function, in
    // order, and admit it once.
    let mut survivors: Vec<&ExactFunctionDeclaration> = Vec::new();
    for declaration in &ordered_functions {
        if matches!(resolved.get(declaration.name.as_str()), Some(Ok(()))) {
            survivors.push(declaration);
        }
    }

    let mut classified: Vec<ClassifiedFunction<'_>> = Vec::with_capacity(survivors.len());
    for declaration in &survivors {
        let parameter_kinds: Vec<OperandKind> = declaration
            .parameters
            .iter()
            .map(|parameter| resolve_operand_type(&graph, &parameter.type_node_id))
            .collect::<Result<_, _>>()
            .expect("Stage 1 already validated every survivor's parameter types");
        let result_kind = resolve_operand_type(&graph, &declaration.result_type)
            .expect("Stage 1 already validated every survivor's result type");
        classified.push(ClassifiedFunction {
            declaration,
            parameter_kinds,
            result_kind,
        });
    }

    let declared_functions: Vec<rt::FunctionDeclaration> = classified
        .iter()
        .map(|function| rt::FunctionDeclaration {
            name: function.declaration.name.clone(),
            parameters: function
                .declaration
                .parameters
                .iter()
                .zip(&function.parameter_kinds)
                .map(|(parameter, kind)| (parameter.name.clone(), kind.to_value_type()))
                .collect(),
            result: function.result_kind.to_value_type(),
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, _args| rt::Outcome::Refused(rt::Refusal::CheckedInvariant)),
        })
        .collect();

    let function_index: BTreeMap<&str, usize> = classified
        .iter()
        .enumerate()
        .map(|(index, function)| (function.declaration.name.as_str(), index))
        .collect();

    let package_check = rt::PackageDeclarations {
        types: rt::TypeEnvironment::new(Vec::new(), core::iter::empty()).expect(
            "an empty composite/enum declaration set always admits: generation-time invariant",
        ),
        functions: declared_functions,
    }
    .check(rt::CheckMode::Linked, rt::CheckingLimits::default());

    let package_refusal: Option<String> = match &package_check {
        Ok(_) => None,
        Err(refusals) => Some(
            refusals
                .iter()
                .map(|refusal| format!("{:?}", refusal.cause))
                .collect::<Vec<_>>()
                .join("; "),
        ),
    };

    // Location map: one entry per surviving function, whether or not the
    // package itself later admits -- this is a static, generation-time fact
    // about the request, independent of admission (AC-15).
    let mut location_map = Vec::with_capacity(survivors.len());
    for declaration in &survivors {
        let index = function_index[declaration.name.as_str()];
        let call_point = match &declaration.body {
            ExactFunctionBody::Scalar { .. } => CallPointKind::ScalarOperator,
            ExactFunctionBody::CompositeEquality { .. } => CallPointKind::EqualityEvaluation,
            ExactFunctionBody::Call { .. } => CallPointKind::NestedCall,
        };
        location_map.push(LocationMapEntry {
            function: declaration.name.clone(),
            call_point,
            location: RecordedLocation {
                origin: RecordedOrigin::Body {
                    function: declaration.name.clone(),
                    index,
                },
                path: Vec::new(),
            },
        });
    }

    // Stage 3: per requested item.
    let mut counts: BTreeMap<ItemKey, u32> = BTreeMap::new();
    let mut by_key: BTreeMap<ItemKey, &ExactFunctionItem> = BTreeMap::new();
    let function_node_id_by_name: BTreeMap<&str, &CheckedNodeId> = functions
        .iter()
        .map(|declaration| (declaration.name.as_str(), &declaration.node_id))
        .collect();
    for item in items {
        let key = ItemKey::of(item, &function_node_id_by_name);
        *counts.entry(key.clone()).or_insert(0) += 1;
        by_key.entry(key).or_insert(item);
    }

    let call_requested: Vec<CheckedNodeId> =
        by_key.values().map(|item| item.call_node_id.clone()).collect();
    let call_lowering = package.lower(&call_requested, &lowering_profile());

    let mut source = SourceBuilder::default();
    let mut claims = Vec::with_capacity(by_key.len());
    for ((key, item), record) in by_key.into_iter().zip(&call_lowering.records) {
        let duplicate = counts.get(&key).copied().unwrap_or(0) > 1;
        let result = if duplicate {
            ExactFunctionDisposition::Refused {
                refusal: ExactFunctionRefusal::DuplicateRequest,
            }
        } else {
            match item_disposition(
                &survivors,
                &function_index,
                &package_refusal,
                &lowering.package_id,
                item,
                record,
            ) {
                Ok((symbol, claim)) => {
                    let declaration = survivors
                        .iter()
                        .find(|declaration| declaration.name == item.function)
                        .expect("item_disposition only returns Ok for a surviving function");
                    source.item(&symbol, item, declaration);
                    ExactFunctionDisposition::Generated(Box::new(claim))
                }
                Err(refusal) => ExactFunctionDisposition::Refused { refusal },
            }
        };
        claims.push(ExactFunctionClaim {
            node_id: item.call_node_id.clone(),
            result,
        });
    }

    let claim_map = ExactFunctionClaimMap {
        version: EXACT_FUNCTION_CLAIM_MAP_VERSION,
        package_id: lowering.package_id.clone(),
        runtime_revision: RUNTIME_REVISION,
        blocked: Vec::new(),
        items: claims,
    };
    let lib = source.finish(&classified, &lowering.package_id);
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(ExactFunctionGenerationError::SourceTooLarge { bytes: lib.len() });
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| ExactFunctionGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text = String::from_utf8(map_bytes)
        .map_err(|_| ExactFunctionGenerationError::ClaimMapSerialization)?;

    let mut location_bytes = serde_json::to_vec_pretty(&location_map)
        .map_err(|_| ExactFunctionGenerationError::LocationMapSerialization)?;
    location_bytes.push(b'\n');
    let location_text = String::from_utf8(location_bytes)
        .map_err(|_| ExactFunctionGenerationError::LocationMapSerialization)?;

    Ok(ExactFunctionOracles {
        artifacts: vec![
            artifact("Cargo.toml", manifest()),
            artifact("src/lib.rs", lib),
            artifact("claim-map.json", map_text),
            artifact("location-map.json", location_text),
        ],
        claim_map,
        location_map,
    })
}

#[allow(clippy::too_many_arguments)]
fn item_disposition(
    survivors: &[&ExactFunctionDeclaration],
    function_index: &BTreeMap<&str, usize>,
    package_refusal: &Option<String>,
    package_id: &CheckedSemanticId,
    item: &ExactFunctionItem,
    record: &CompleteLoweringRecordV2,
) -> Result<(String, GeneratedExactFunctionClaim), ExactFunctionRefusal> {
    let node = lowered_binary_body(record)?;
    if &*node.node.semantic_form != "call" {
        return Err(ExactFunctionRefusal::FormMismatch {
            found: node.node.semantic_form.to_string(),
        });
    }
    let Some(declaration) = survivors
        .iter()
        .find(|declaration| declaration.name == item.function)
    else {
        return Err(ExactFunctionRefusal::UnknownFunction {
            name: item.function.clone(),
        });
    };
    if item.argument_node_ids.len() != declaration.parameters.len() {
        return Err(ExactFunctionRefusal::ArityMismatch {
            expected: declaration.parameters.len(),
            found: item.argument_node_ids.len(),
        });
    }
    let index = function_index[declaration.name.as_str()];
    if let Some(cause) = package_refusal {
        return Err(ExactFunctionRefusal::PackageRefused {
            function_index: Some(index),
            cause: cause.clone(),
        });
    }
    let symbol = format!("call_{}", node.node.node_id.digest);
    Ok((
        symbol.clone(),
        GeneratedExactFunctionClaim {
            oracle_symbol: format!("oracle_{symbol}"),
            ir_id: node.ir_id.clone(),
            package_id: package_id.clone(),
            semantic_type: node.semantic_type.clone(),
            source_map: node.source_map.clone(),
            claims: node.claims.clone(),
            function: declaration.name.clone(),
            function_origin: RecordedOrigin::Body {
                function: declaration.name.clone(),
                index,
            },
            declaration_keys: Vec::new(),
        },
    ))
}

// ---------------------------------------------------------------------------
// Item ordering key (AC-13)
// ---------------------------------------------------------------------------

/// The total order and duplicate-detection key: the `call` expression
/// node's id, then the applied function's declaring node id (absent ranks
/// before present, for an item naming an unknown function), then each
/// argument operand's source node id, every node id compared by digest
/// domain then digest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ItemKey {
    call_node_id: CheckedNodeId,
    function_node_id: Option<CheckedNodeId>,
    argument_node_ids: Vec<CheckedNodeId>,
}

impl ItemKey {
    fn of(item: &ExactFunctionItem, function_node_id_by_name: &BTreeMap<&str, &CheckedNodeId>) -> Self {
        Self {
            call_node_id: item.call_node_id.clone(),
            function_node_id: function_node_id_by_name
                .get(item.function.as_str())
                .map(|id| (*id).clone()),
            argument_node_ids: item.argument_node_ids.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

const SOURCE_HEADER: &str = "\
// @generated by quire-contract-codegen function-application oracles. Do not edit.
//
// `checked_package()` assembles this package's `PackageDeclarations` and
// admits it through `PackageDeclarations::check(CheckMode::Linked, ..)`,
// returning `Result<CheckedPackage, Vec<CheckRefusal>>` unchanged -- the
// same admission this generator already ran once at generation time.
// Every oracle function below calls `CheckedPackage::call` with an
// already-checked package and returns its `Outcome<Value>` unchanged; it
// never reads `Evaluation.location` or `.losses`, since the pinned runtime
// never populates either field. No charge amount and no literal `Outcome`/
// `Value` constant standing in for a runtime result appears in this source:
// every charge and every result comes from the runtime.

use quire_contract_runtime::exact as rt;
";

#[derive(Default)]
struct SourceBuilder {
    functions: String,
    rendered_bodies: BTreeSet<String>,
}

impl SourceBuilder {
    fn item(&mut self, symbol: &str, item: &ExactFunctionItem, declaration: &ExactFunctionDeclaration) {
        self.functions.push_str(&format!(
            "\n/// Node `{}`: applies `{}`.\n\
             pub fn oracle_{symbol}(\n    \
             package: &rt::CheckedPackage,\n    \
             arguments: Vec<rt::Value>,\n    \
             objects: &rt::ObjectEnvironment,\n    \
             meter: &mut rt::Meter,\n\
             ) -> Result<rt::Outcome<rt::Value>, rt::InputRefusal> {{\n    \
             package\n        \
             .call({:?}, arguments, objects, meter)\n        \
             .map(|evaluation| evaluation.outcome)\n}}\n",
            item.call_node_id.digest, declaration.name, declaration.name
        ));
    }

    /// Render `checked_package()` from every surviving function's own
    /// classified shape (declaration plus resolved operand kinds), in
    /// assembled order.
    fn finish(mut self, classified: &[ClassifiedFunction<'_>], package_id: &CheckedSemanticId) -> String {
        let mut declarations = String::new();
        for function in classified {
            declarations.push_str(&render_function_declaration(function));
            self.rendered_bodies.insert(function.declaration.name.clone());
        }
        let mut source = format!("// Source package: {}\n", package_id.digest);
        source.push_str(SOURCE_HEADER);
        source.push_str(&format!(
            "\npub fn checked_package() -> Result<rt::CheckedPackage, Vec<rt::CheckRefusal>> {{\n    \
             let types = rt::TypeEnvironment::new(Vec::new(), core::iter::empty::<rt::ObjectTypeDeclaration>())\n        \
             .expect(\"an empty composite/enum declaration set always admits: generation-time invariant\");\n    \
             let functions = vec![{declarations}];\n    \
             rt::PackageDeclarations {{ types, functions }}.check(rt::CheckMode::Linked, rt::CheckingLimits::default())\n}}\n"
        ));
        source.push_str(&self.functions);
        source
    }
}

/// Render one `rt::FunctionDeclaration` literal, using the classified
/// function's own resolved parameter/result kinds -- never a placeholder,
/// so a `Boolean` and an `Integer` parameter can never render identically.
fn render_function_declaration(function: &ClassifiedFunction<'_>) -> String {
    let parameters: String = function
        .declaration
        .parameters
        .iter()
        .zip(&function.parameter_kinds)
        .map(|(parameter, kind)| format!("({:?}.to_owned(), {}), ", parameter.name, kind.render()))
        .collect();
    let body = render_body(function.declaration);
    format!(
        "rt::FunctionDeclaration {{\n        \
         name: {:?}.to_owned(),\n        \
         parameters: vec![{parameters}],\n        \
         result: {},\n        \
         ieee_requirements: Vec::new(),\n        \
         integer_division_consumers: Vec::new(),\n        \
         measure_discharged: true,\n        \
         body: Box::new({body}),\n    \
         }},\n    ",
        function.declaration.name,
        function.result_kind.render(),
    )
}

fn render_body(declaration: &ExactFunctionDeclaration) -> String {
    match &declaration.body {
        ExactFunctionBody::Scalar { operator } => {
            let variant = match operator {
                IntegerOperator::Add => "Add",
                IntegerOperator::Subtract => "Subtract",
                IntegerOperator::Multiply => "Multiply",
                IntegerOperator::Negate => unreachable!("Negate refused at generation time"),
            };
            format!(
                "|frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {{\n        \
                 let [rt::Value::Integer(left), rt::Value::Integer(right)] = args else {{\n            \
                 return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n        }};\n        \
                 let evaluated = frame.meter(|meter| rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::{variant}(left, right), None, meter));\n        \
                 match evaluated {{\n            \
                 Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Integer(value)),\n            \
                 Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),\n            \
                 Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),\n            \
                 Ok(rt::Outcome::Incomplete(incomplete)) => rt::Outcome::Incomplete(incomplete),\n            \
                 Err(refusal) => rt::Outcome::Refused(refusal),\n        \
                 }}\n    }}"
            )
        }
        ExactFunctionBody::CompositeEquality { operator } => {
            let path = match operator {
                EqualityOperatorKind::Equal => "rt::EqualityOperator::Equal",
                EqualityOperatorKind::NotEqual => "rt::EqualityOperator::NotEqual",
            };
            format!(
                "|frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {{\n        \
                 let [left, right] = args else {{\n            \
                 return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n        }};\n        \
                 let operand_type = match left {{\n            \
                 rt::Value::Boolean(_) => rt::ValueType::Boolean,\n            \
                 rt::Value::Integer(_) => rt::ValueType::Integer,\n            \
                 _ => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n        \
                 }};\n        \
                 let environment = match rt::TypeEnvironment::new(Vec::new(), core::iter::empty::<rt::ObjectTypeDeclaration>()) {{\n            \
                 Ok(environment) => environment,\n            \
                 Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n        \
                 }};\n        \
                 let checked = match environment.check_equality({path}, rt::EqualityOperand::typed(operand_type.clone()), rt::EqualityOperand::typed(operand_type)) {{\n            \
                 Ok(checked) => checked,\n            \
                 Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n        \
                 }};\n        \
                 match frame.meter(|meter| checked.evaluate(left, right, meter)) {{\n            \
                 Ok(rt::Outcome::Completed(value)) => rt::Outcome::Completed(rt::Value::Boolean(value)),\n            \
                 Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),\n            \
                 Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),\n            \
                 Ok(rt::Outcome::Incomplete(incomplete)) => rt::Outcome::Incomplete(incomplete),\n            \
                 Err(refusal) => rt::Outcome::Refused(refusal),\n        \
                 }}\n    }}"
            )
        }
        ExactFunctionBody::Call { callee } => {
            format!(
                "|frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {{\n        \
                 frame.call({callee:?}, args)\n    }}"
            )
        }
    }
}

fn manifest() -> String {
    format!(
        "[package]\nname = \"{EXACT_FUNCTION_CRATE_NAME}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\", features = [\"exact\"] }}\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n[workspace]\n"
    )
}

fn artifact(path: &str, contents: String) -> Artifact {
    let digest = Sha256::digest(contents.as_bytes());
    let sha256 = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    Artifact {
        path: path.to_owned(),
        contents,
        sha256,
    }
}
