//! Operation-contract and frame-effect Kani obligations for one admitted `state_clause`
//! postcondition (IR-412, FR-015).
//!
//! A `postcondition` `state`/`state_clause` node of an admitted `quire.checked-package/v2`
//! package names an operation through its `operation_anchor`, and that anchor names the
//! operation's `frame`. One request yields two separate obligations over the same operation:
//!
//! - **Operation contract.** The clause's condition is read back from the graph -- never taken
//!   from a caller descriptor -- as one integer comparison between the post-state value of a
//!   field and its `pre(...)` value, both read through the clause's first parameter, `self`. The
//!   harness builds a symbolic state inside the integer range the framed object's member of that
//!   name declares, snapshots it, runs the operation subject on a copy, and asserts the comparison
//!   between the snapshot (before) and the copy (after).
//! - **Frame effects.** The frame's `modifies` field entries are the *allowed* effects. Every
//!   other caller-named state field is a *forbidden* effect: the harness asserts the operation
//!   left it unchanged. The obligation's identity is scoped to the operation, its anchor, its
//!   frame and the framed object, so a frame of another operation or another package revision is
//!   another obligation, and a subject that writes a granted field verifies while one that writes
//!   an ungranted field is falsified.
//!
//! No other value family is lowered: a condition that is not one integer comparison of pre and
//! post reads of a single field through `self`, a frame that creates, deletes or grants a
//! relationship, and a clause field whose object member declares no integer range are each
//! refused with a typed reason and produce no harness.
//!
//! The subject ABI is fixed and documented on [`StateFrameRequest`]. Nothing in this module runs
//! the prover; `execute_kani_obligation` does.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticNodeV2,
    CompleteLoweringProfileV2, CompleteLoweringRecordV2,
};
use serde::Serialize;
use serde_json::Value;

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError},
    exact_scalar::{bound_members, literal, INTEGER_RANGE_MEMBERS},
    kani::{adapter_options, i64_literal, KaniSolver},
    kani_identity::{
        StateComparison, StateFieldDomain, StateFrameHarness, StateFrameIdentity,
        StateFrameProperty, StateFrameScope,
    },
    MAX_OBLIGATION_UNWIND,
};

/// Work budget for lowering one clause and its closure.
const LOWERING_WORK_LIMIT: u64 = 65_536;

/// Every tag a postcondition clause's dependency closure holds: the clause, its anchor and frame
/// (`state`), the framed object (`model`), its reference type (`composite_type`), the field and
/// integer types (`scalar_type`, `bounded_domain`), a relationship a frame grants (`relation`), the `self` parameter (`value`) and the
/// condition (`expression`).
const STATE_FRAME_LOWERING_TAGS: [CheckedNodeTag; 8] = [
    CheckedNodeTag::State,
    CheckedNodeTag::Model,
    CheckedNodeTag::Relation,
    CheckedNodeTag::CompositeType,
    CheckedNodeTag::ScalarType,
    CheckedNodeTag::BoundedDomain,
    CheckedNodeTag::Value,
    CheckedNodeTag::Expression,
];

/// What a caller supplies to generate the obligations of one postcondition clause.
///
/// Subject ABI: `state_path` names a `Clone` struct whose fields are exactly `state_fields`, all
/// public and all `i64`, and `subject_path` names `fn(&mut State)`, the operation the clause's
/// anchor names. Every state field is symbolic in the frame obligation; only the clause's own
/// field is bounded, by the integer range the IR carries for it.
#[derive(Clone, Copy, Debug)]
pub struct StateFrameRequest<'a> {
    /// The admitted package.
    pub package: &'a CheckedPackageV2,
    /// The `state`/`state_clause` node of a `postcondition` clause.
    pub clause: &'a CheckedNodeId,
    /// Rust path of the state struct.
    pub state_path: &'a str,
    /// Every field of the state struct, in declaration order.
    pub state_fields: &'a [&'a str],
    /// Rust path of the operation subject.
    pub subject_path: &'a str,
    /// Loop unwind bound, `1..=MAX_OBLIGATION_UNWIND`.
    pub unwind: u32,
}

/// The two obligations one postcondition clause yields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateFrameObligations {
    /// The clause's operation-contract harness.
    pub postcondition: StateFrameHarness,
    /// The frame-effect harness of the clause's operation.
    pub frame: StateFrameHarness,
}

impl StateComparison {
    fn from_operation(identity: &str) -> Option<Self> {
        Some(match identity {
            "quire.op.integer.lt" => Self::Lt,
            "quire.op.integer.le" => Self::Le,
            "quire.op.integer.gt" => Self::Gt,
            "quire.op.integer.ge" => Self::Ge,
            "quire.op.integer.eq" => Self::Eq,
            "quire.op.integer.ne" => Self::Ne,
            _ => return None,
        })
    }

    fn rust(self) -> &'static str {
        match self {
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
        }
    }
}

/// Which side of the operation a clause operand observes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Side {
    Pre,
    Post,
}

/// A frame effect this generator has no finite encoding for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedFrameEffect {
    /// The frame creates an object type.
    Creates,
    /// The frame deletes an object type.
    Deletes,
    /// The frame grants a relationship.
    Relationship,
    /// The frame grants a field of a type other than the clause's object.
    ForeignField,
}

/// Why a request produced no harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateFrameRefusal {
    /// The unwind bound is outside `1..=MAX_OBLIGATION_UNWIND`.
    UnwindOutOfRange {
        /// The requested bound.
        unwind: u32,
    },
    /// A path is not a Rust path.
    InvalidPath {
        /// The offending path.
        path: String,
    },
    /// A field name is not a Rust identifier, or is named twice.
    InvalidField {
        /// The offending name.
        name: String,
    },
    /// Lowering did not produce the clause: an unsupported family, a missing bound or an
    /// unknown node.
    NotLowered {
        /// IR's own record.
        record: Box<CompleteLoweringRecordV2>,
    },
    /// The node is not a `state`/`state_clause` node.
    NotAStateClause {
        /// The node's family.
        node_tag: String,
        /// The node's form.
        semantic_form: String,
    },
    /// The clause is an invariant or a precondition.
    NotAPostcondition {
        /// The clause kind the node names.
        clause: String,
    },
    /// A node does not have the shape QSpec FR-341 and FR-342 fix, or names a node the graph
    /// does not hold.
    MalformedClause {
        /// The node that failed to decode.
        at: CheckedNodeId,
    },
    /// The condition is not one integer comparison of pre and post reads of one field.
    ConditionNotSupported {
        /// The node that is outside the supported shape.
        at: CheckedNodeId,
    },
    /// The two sides of the comparison read different fields.
    ObservationsDiffer {
        /// The left operand's field.
        left: String,
        /// The right operand's field.
        right: String,
    },
    /// Both sides of the comparison observe the same side of the operation.
    ObservationsSameSide,
    /// The frame has an effect with no finite encoding.
    FrameEffectUnsupported {
        /// The frame node.
        frame: CheckedNodeId,
        /// The effect.
        effect: UnsupportedFrameEffect,
    },
    /// The framed object's member for the clause's field declares no `i64` integer range.
    BoundNotResolved {
        /// The clause's field.
        field: String,
    },
    /// A field the clause reads or the frame grants is not in `state_fields`.
    UnknownStateField {
        /// The field.
        field: String,
    },
    /// The frame grants every state field, so it forbids no effect and there is nothing to
    /// assert.
    NothingForbidden {
        /// The frame node.
        frame: CheckedNodeId,
    },
    /// The generated source exceeds [`MAX_GENERATED_SOURCE_BYTES`].
    ResourceLimitExceeded {
        /// The generated size.
        bytes: usize,
    },
    /// The generated source does not parse as Rust.
    InvalidGeneratedSyntax {
        /// The parse error.
        error: String,
    },
    /// The identity record failed to serialize.
    RecordSerialization,
}

impl std::fmt::Display for StateFrameRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnwindOutOfRange { unwind } => {
                write!(
                    formatter,
                    "unwind bound {unwind} is outside 1..={MAX_OBLIGATION_UNWIND}"
                )
            }
            Self::InvalidPath { path } => write!(formatter, "`{path}` is not a Rust path"),
            Self::InvalidField { name } => {
                write!(
                    formatter,
                    "`{name}` is not a distinct Rust field identifier"
                )
            }
            Self::NotLowered { record } => {
                write!(formatter, "the clause did not lower: {record:?}")
            }
            Self::NotAStateClause {
                node_tag,
                semantic_form,
            } => {
                write!(
                    formatter,
                    "node is `{node_tag}`/`{semantic_form}`, not state/state_clause"
                )
            }
            Self::NotAPostcondition { clause } => {
                write!(formatter, "clause kind `{clause}` is not a postcondition")
            }
            Self::MalformedClause { at } => write!(formatter, "malformed clause at {}", at.digest),
            Self::ConditionNotSupported { at } => {
                write!(
                    formatter,
                    "condition is outside the supported shape at {}",
                    at.digest
                )
            }
            Self::ObservationsDiffer { left, right } => {
                write!(
                    formatter,
                    "the condition reads `{left}` and `{right}`, not one field"
                )
            }
            Self::ObservationsSameSide => {
                formatter.write_str("the condition compares two reads of the same side")
            }
            Self::FrameEffectUnsupported { frame, effect } => {
                write!(
                    formatter,
                    "frame {} has unsupported effect {effect:?}",
                    frame.digest
                )
            }
            Self::BoundNotResolved { field } => {
                write!(
                    formatter,
                    "the object declares no i64 integer range for `{field}`"
                )
            }
            Self::UnknownStateField { field } => {
                write!(formatter, "`{field}` is not a declared state field")
            }
            Self::NothingForbidden { frame } => {
                write!(formatter, "frame {} grants every state field", frame.digest)
            }
            Self::ResourceLimitExceeded { bytes } => {
                write!(
                    formatter,
                    "generated source of {bytes} bytes exceeds the ceiling"
                )
            }
            Self::InvalidGeneratedSyntax { error } => {
                write!(formatter, "generated source does not parse: {error}")
            }
            Self::RecordSerialization => {
                formatter.write_str("the identity record did not serialize")
            }
        }
    }
}

impl std::error::Error for StateFrameRefusal {}

/// Generates the operation-contract and frame-effect harnesses of one postcondition clause.
///
/// # Errors
///
/// A [`StateFrameRefusal`] naming why no harness can be produced; nothing is partially returned.
pub fn generate_state_frame_obligations(
    request: &StateFrameRequest<'_>,
) -> Result<StateFrameObligations, StateFrameRefusal> {
    validate_request(request)?;
    let graph = Graph::of(request.package);
    lower_clause(request)?;
    let shape = ClauseShape::read(&graph, request.clause)?;
    let granted = shape.frame_grants(&graph)?;
    let condition = shape.condition(&graph, request.clause)?;
    let known = request
        .state_fields
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for field in granted
        .iter()
        .map(String::as_str)
        .chain([condition.field.as_str()])
    {
        if !known.contains(field) {
            return Err(StateFrameRefusal::UnknownStateField {
                field: field.to_owned(),
            });
        }
    }
    let checked = request
        .state_fields
        .iter()
        .filter(|field| !granted.contains(**field))
        .map(|field| (*field).to_owned())
        .collect::<Vec<_>>();
    if checked.is_empty() {
        return Err(StateFrameRefusal::NothingForbidden {
            frame: shape.scope.frame.clone(),
        });
    }
    let domains = state_domains(&graph, &shape.scope.object, request);
    if !domains.iter().any(|domain| domain.field == condition.field) {
        return Err(StateFrameRefusal::BoundNotResolved {
            field: condition.field,
        });
    }
    let postcondition = render(
        request,
        &shape.scope,
        &domains,
        StateFrameProperty::Postcondition {
            field: condition.field,
            comparison: condition.comparison,
            left_is_pre: condition.left == Side::Pre,
        },
        &format!("post_{}", short(&request.clause.digest)),
    )?;
    let frame = render(
        request,
        &shape.scope,
        &domains,
        StateFrameProperty::Frame {
            granted: granted.into_iter().collect(),
            checked,
        },
        &format!(
            "frame_{}_{}",
            short(&shape.scope.anchor.digest),
            short(&request.clause.digest)
        ),
    )?;
    Ok(StateFrameObligations {
        postcondition,
        frame,
    })
}

fn validate_request(request: &StateFrameRequest<'_>) -> Result<(), StateFrameRefusal> {
    if !(1..=MAX_OBLIGATION_UNWIND).contains(&request.unwind) {
        return Err(StateFrameRefusal::UnwindOutOfRange {
            unwind: request.unwind,
        });
    }
    for path in [request.state_path, request.subject_path] {
        if syn::parse_str::<syn::Path>(path).is_err() {
            return Err(StateFrameRefusal::InvalidPath {
                path: path.to_owned(),
            });
        }
    }
    let mut seen = BTreeSet::new();
    for field in request.state_fields {
        if !is_identifier(field) || !seen.insert(*field) {
            return Err(StateFrameRefusal::InvalidField {
                name: (*field).to_owned(),
            });
        }
    }
    Ok(())
}

fn is_identifier(name: &str) -> bool {
    syn::parse_str::<syn::Ident>(name).is_ok()
}

fn short(digest: &str) -> &str {
    digest.get(..12).unwrap_or(digest)
}

fn lower_clause(request: &StateFrameRequest<'_>) -> Result<(), StateFrameRefusal> {
    let profile = CompleteLoweringProfileV2 {
        supported_tags: BTreeSet::from(STATE_FRAME_LOWERING_TAGS),
        require_bounds: true,
        work_limit: LOWERING_WORK_LIMIT,
    };
    let result = request
        .package
        .lower(std::slice::from_ref(request.clause), &profile);
    match result.records.into_iter().next() {
        Some(CompleteLoweringRecordV2::Lowered { .. }) => Ok(()),
        Some(record) => Err(StateFrameRefusal::NotLowered {
            record: Box::new(record),
        }),
        None => Err(StateFrameRefusal::MalformedClause {
            at: request.clause.clone(),
        }),
    }
}

/// The admitted graph by node id.
struct Graph<'g> {
    nodes: BTreeMap<&'g CheckedNodeId, &'g CheckedSemanticNodeV2>,
}

impl<'g> Graph<'g> {
    fn of(package: &'g CheckedPackageV2) -> Self {
        Self {
            nodes: package
                .graph()
                .nodes
                .iter()
                .map(|node| (&node.node_id, node))
                .collect(),
        }
    }

    /// `term` itself when inline, else the node a `reference` term names.
    fn follow(&self, term: &'g Value, parent: &'g CheckedNodeId) -> Option<Located<'g>> {
        if term.get("term")?.as_str()? != "reference" {
            return Some(Located {
                id: parent,
                node: None,
                body: term,
            });
        }
        let node = *self.nodes.get(&node_id(term.get("target")?)?)?;
        Some(Located {
            id: &node.node_id,
            node: Some(node),
            body: &node.body,
        })
    }
}

/// A term resolved to the node holding it.
struct Located<'g> {
    id: &'g CheckedNodeId,
    node: Option<&'g CheckedSemanticNodeV2>,
    body: &'g Value,
}

fn node_id(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.clone()).ok()
}

/// The arguments of an application of `operator`/`identity`.
fn application<'v>(body: &'v Value, operator: &str, identity: &str) -> Option<&'v [Value]> {
    let is_application = body.get("term")?.as_str()? == "application"
        && body.get("operator")?.as_str()? == operator
        && body.get("operation")?.get("identity")?.as_str()? == identity;
    if !is_application {
        return None;
    }
    body.get("arguments")?.as_array().map(Vec::as_slice)
}

struct ClauseShape {
    /// The clause's parameters: `self` first, then the result and every operation parameter.
    /// Only `self` is a read of the framed state.
    parameters: Vec<CheckedNodeId>,
    scope: StateFrameScope,
    condition: Value,
}

struct Condition {
    field: String,
    comparison: StateComparison,
    left: Side,
}

impl ClauseShape {
    /// Decodes the clause, its anchor and its frame by QSpec FR-341 and FR-342.
    fn read<'g>(graph: &Graph<'g>, clause: &'g CheckedNodeId) -> Result<Self, StateFrameRefusal> {
        let malformed = || StateFrameRefusal::MalformedClause { at: clause.clone() };
        let node = *graph.nodes.get(clause).ok_or_else(malformed)?;
        if &*node.node_tag != "state" || &*node.semantic_form != "state_clause" {
            return Err(StateFrameRefusal::NotAStateClause {
                node_tag: node.node_tag.to_string(),
                semantic_form: node.semantic_form.to_string(),
            });
        }
        let body = &node.body;
        let [parameters, anchor, condition] =
            application(body, "state_clause", "quire.op.state.clause")
                .and_then(|arguments| <&[Value; 3]>::try_from(arguments).ok())
                .ok_or_else(malformed)?;
        let kind = body
            .get("operation")
            .and_then(|operation| operation.get("member"))
            .and_then(|member| member.get("clause"))
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        if kind != "postcondition" {
            return Err(StateFrameRefusal::NotAPostcondition {
                clause: kind.to_owned(),
            });
        }
        let parameters = parameters
            .get("members")
            .and_then(Value::as_array)
            .ok_or_else(malformed)?
            .iter()
            .map(|member| node_id(member.get("target")?))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(malformed)?;
        let anchor = graph
            .follow(anchor, clause)
            .and_then(|anchor| anchor.node)
            .ok_or_else(malformed)?;
        let condition = graph.follow(condition, clause).ok_or_else(malformed)?;
        Ok(Self {
            parameters,
            scope: read_scope(graph, anchor).ok_or_else(malformed)?,
            condition: condition.body.clone(),
        })
    }

    /// The frame's granted field names; refuses every effect with no finite encoding.
    fn frame_grants(&self, graph: &Graph<'_>) -> Result<BTreeSet<String>, StateFrameRefusal> {
        let frame = &self.scope.frame;
        let malformed = || StateFrameRefusal::MalformedClause { at: frame.clone() };
        let body = &graph.nodes.get(frame).ok_or_else(malformed)?.body;
        let unsupported = |effect| StateFrameRefusal::FrameEffectUnsupported {
            frame: frame.clone(),
            effect,
        };
        for (member, effect) in [
            ("creates", UnsupportedFrameEffect::Creates),
            ("deletes", UnsupportedFrameEffect::Deletes),
        ] {
            let entries = body
                .get(member)
                .and_then(Value::as_array)
                .ok_or_else(malformed)?;
            if !entries.is_empty() {
                return Err(unsupported(effect));
            }
        }
        let mut granted = BTreeSet::new();
        for entry in body
            .get("modifies")
            .and_then(Value::as_array)
            .ok_or_else(malformed)?
        {
            match entry.get("kind").and_then(Value::as_str) {
                Some("field") => {}
                Some(_) => return Err(unsupported(UnsupportedFrameEffect::Relationship)),
                None => return Err(malformed()),
            }
            if entry.get("declaration").and_then(node_id).as_ref() != Some(&self.scope.object) {
                return Err(unsupported(UnsupportedFrameEffect::ForeignField));
            }
            let name = entry
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            if !is_identifier(name) {
                return Err(StateFrameRefusal::InvalidField {
                    name: name.to_owned(),
                });
            }
            granted.insert(name.to_owned());
        }
        Ok(granted)
    }

    /// The condition as one integer comparison of the pre and post reads of one field.
    fn condition<'g>(
        &'g self,
        graph: &Graph<'g>,
        clause: &'g CheckedNodeId,
    ) -> Result<Condition, StateFrameRefusal> {
        let unsupported = || StateFrameRefusal::ConditionNotSupported { at: clause.clone() };
        let identity = self
            .condition
            .get("operation")
            .and_then(|operation| operation.get("identity"))
            .and_then(Value::as_str)
            .ok_or_else(unsupported)?;
        let comparison = StateComparison::from_operation(identity).ok_or_else(unsupported)?;
        let [left, right] = application(&self.condition, "binary", identity)
            .and_then(|arguments| <&[Value; 2]>::try_from(arguments).ok())
            .ok_or_else(unsupported)?;
        let left = self.observation(graph, clause, left)?;
        let right = self.observation(graph, clause, right)?;
        if left.field != right.field {
            return Err(StateFrameRefusal::ObservationsDiffer {
                left: left.field,
                right: right.field,
            });
        }
        if left.side == right.side {
            return Err(StateFrameRefusal::ObservationsSameSide);
        }
        Ok(Condition {
            field: left.field,
            comparison,
            left: left.side,
        })
    }

    /// `pre(<read>)` or `<read>`, where a read is `project(deref(self), field)`.
    fn observation<'g>(
        &self,
        graph: &Graph<'g>,
        clause: &'g CheckedNodeId,
        term: &'g Value,
    ) -> Result<Observation, StateFrameRefusal> {
        let unsupported =
            |at: &CheckedNodeId| StateFrameRefusal::ConditionNotSupported { at: at.clone() };
        let operand = graph
            .follow(term, clause)
            .ok_or_else(|| unsupported(clause))?;
        if let Some([inner]) = application(operand.body, "pre", "quire.op.state.pre")
            .and_then(|arguments| <&[Value; 1]>::try_from(arguments).ok())
        {
            let read = graph
                .follow(inner, operand.id)
                .ok_or_else(|| unsupported(operand.id))?;
            let field = self.field_read(graph, &read)?;
            return Ok(Observation {
                field,
                side: Side::Pre,
            });
        }
        Ok(Observation {
            field: self.field_read(graph, &operand)?,
            side: Side::Post,
        })
    }

    /// The field a `record.project` of `deref(self)` reads.
    fn field_read<'g>(
        &self,
        graph: &Graph<'g>,
        read: &Located<'g>,
    ) -> Result<String, StateFrameRefusal> {
        let unsupported = || StateFrameRefusal::ConditionNotSupported {
            at: read.id.clone(),
        };
        let [dereference] = application(read.body, "query", "quire.op.record.project")
            .and_then(|arguments| <&[Value; 1]>::try_from(arguments).ok())
            .ok_or_else(unsupported)?;
        let member = read
            .body
            .get("operation")
            .and_then(|operation| operation.get("member"))
            .ok_or_else(unsupported)?;
        let is_own_field = member.get("kind").and_then(Value::as_str) == Some("field")
            && member.get("declaration").and_then(node_id).as_ref() == Some(&self.scope.object);
        let name = member
            .get("name")
            .and_then(Value::as_str)
            .filter(|_| is_own_field)
            .ok_or_else(unsupported)?;
        if !is_identifier(name) {
            return Err(StateFrameRefusal::InvalidField {
                name: name.to_owned(),
            });
        }
        let dereference = graph.follow(dereference, read.id).ok_or_else(unsupported)?;
        let [subject] = application(dereference.body, "deref", "quire.op.model.deref")
            .and_then(|arguments| <&[Value; 1]>::try_from(arguments).ok())
            .ok_or_else(unsupported)?;
        let subject = graph
            .follow(subject, dereference.id)
            .and_then(|subject| subject.node)
            .ok_or_else(unsupported)?;
        let is_self = &*subject.node_tag == "value"
            && &*subject.semantic_form == "parameter"
            && self.parameters.first() == Some(&subject.node_id);
        if !is_self {
            return Err(unsupported());
        }
        Ok(name.to_owned())
    }
}

struct Observation {
    field: String,
    side: Side,
}

/// The scope of an `operation_anchor` node (QSpec FR-342): its `context`, `operation` and
/// `frame` bindings, read by name.
fn read_scope(graph: &Graph<'_>, anchor: &CheckedSemanticNodeV2) -> Option<StateFrameScope> {
    if &*anchor.node_tag != "state" || &*anchor.semantic_form != "operation_anchor" {
        return None;
    }
    let members = anchor.body.get("members")?.as_array()?;
    let [context, operation, frame] = bound_members(members, ["context", "operation", "frame"])?;
    let target = |term: &Value| {
        (term.get("term")?.as_str()? == "reference").then_some(())?;
        node_id(term.get("target")?)
    };
    let (object, frame) = (target(context)?, target(frame)?);
    let is_frame = graph
        .nodes
        .get(&frame)
        .is_some_and(|node| &*node.node_tag == "state" && &*node.semantic_form == "frame");
    if !is_frame {
        return None;
    }
    Some(StateFrameScope {
        operation: literal(operation, "text")?.to_owned(),
        object,
        anchor: anchor.node_id.clone(),
        frame,
    })
}

fn integer_range(body: &Value) -> Option<(i64, i64)> {
    let members = body.get("members")?.as_array()?;
    let [minimum, maximum] = bound_members(members, INTEGER_RANGE_MEMBERS)?;
    Some((
        literal(minimum, "integer")?.parse().ok()?,
        literal(maximum, "integer")?.parse().ok()?,
    ))
}

/// The range of each state field: the `integer_range` the field's own member of the framed
/// object's body references, for each field whose member declares one.
fn state_domains(
    graph: &Graph<'_>,
    object: &CheckedNodeId,
    request: &StateFrameRequest<'_>,
) -> Vec<StateFieldDomain> {
    let members = graph
        .nodes
        .get(object)
        .and_then(|node| node.body.get("members")?.as_array())
        .map(Vec::as_slice)
        .unwrap_or_default();
    let declared = |field: &str| {
        let member = members
            .iter()
            .find(|member| member.get("name").and_then(Value::as_str) == Some(field))?;
        let target = node_id(member.get("value")?.get("target")?)?;
        let bound = graph.nodes.get(&target)?;
        (&*bound.semantic_form == "integer_range")
            .then(|| integer_range(&bound.body))
            .flatten()
    };
    request
        .state_fields
        .iter()
        .filter_map(|field| {
            declared(field).map(|(minimum, maximum)| StateFieldDomain {
                field: (*field).to_owned(),
                minimum,
                maximum,
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn render(
    request: &StateFrameRequest<'_>,
    scope: &StateFrameScope,
    domains: &[StateFieldDomain],
    property: StateFrameProperty,
    module: &str,
) -> Result<StateFrameHarness, StateFrameRefusal> {
    let invalid_symbol = |error: SymbolError| StateFrameRefusal::InvalidGeneratedSyntax {
        error: error.to_string(),
    };
    let path = HarnessPath {
        module: ModuleSymbol::try_from(module).map_err(invalid_symbol)?,
        harness: HarnessSymbol::try_from(HARNESS).map_err(invalid_symbol)?,
    };
    let options = adapter_options(
        &path.to_string(),
        request.unwind,
        KaniSolver::Cadical,
        false,
    );
    let abi = Abi {
        state_path: request.state_path,
        subject_path: request.subject_path,
        state_fields: request.state_fields,
    };
    let body = match &property {
        StateFrameProperty::Postcondition {
            field,
            comparison,
            left_is_pre,
        } => postcondition_body(
            &abi,
            scope,
            domains,
            &Postcondition {
                field,
                comparison: *comparison,
                left_is_pre: *left_is_pre,
            },
            module,
        ),
        StateFrameProperty::Frame { checked, .. } => {
            frame_body(&abi, scope, domains, checked, module)
        }
    };
    let identity = StateFrameIdentity {
        clause: request.clause.clone(),
        scope: scope.clone(),
        property,
        domains: domains.to_vec(),
        state_path: request.state_path.to_owned(),
        subject_path: request.subject_path.to_owned(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
    };
    let source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n// Obligation: state-clause {} of operation {:?}\n\n{body}",
        match identity.property {
            StateFrameProperty::Postcondition { .. } => "postcondition",
            StateFrameProperty::Frame { .. } => "frame",
        },
        scope.operation,
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(StateFrameRefusal::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    if let Err(error) = syn::parse_file(&source) {
        return Err(StateFrameRefusal::InvalidGeneratedSyntax {
            error: error.to_string(),
        });
    }
    let mut record = serde_json::to_string(&RecordView {
        identity: &identity,
        rust_path: &format!("src/generated/{module}.rs"),
    })
    .map_err(|_| StateFrameRefusal::RecordSerialization)?;
    record.push('\n');
    Ok(StateFrameHarness {
        rust: Artifact::new(format!("src/generated/{module}.rs"), source),
        record: Artifact::new(format!("kani-obligations/{module}.json"), record),
        identity,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordView<'a> {
    identity: &'a StateFrameIdentity,
    rust_path: &'a str,
}

/// What the generated harness bodies need of the request: the subject ABI.
struct Abi<'a> {
    state_path: &'a str,
    subject_path: &'a str,
    state_fields: &'a [&'a str],
}

/// `let pre: S = S { a: kani::any(), ... };` and an assumption of each field's IR range.
fn symbolic_state(abi: &Abi<'_>, domains: &[StateFieldDomain]) -> String {
    let fields = abi
        .state_fields
        .iter()
        .map(|field| format!("{field}: kani::any()"))
        .collect::<Vec<_>>()
        .join(", ");
    let assumptions = domains
        .iter()
        .map(
            |StateFieldDomain {
                 field,
                 minimum,
                 maximum,
             }| {
                format!(
                    "        kani::assume(pre.{field} >= {} && pre.{field} <= {});\n",
                    i64_literal(*minimum),
                    i64_literal(*maximum)
                )
            },
        )
        .collect::<String>();
    format!(
        "        let pre: {state} = {state} {{ {fields} }};\n{assumptions}",
        state = abi.state_path
    )
}

/// `text` as a Rust string literal. It is passed to `assert!` as a format argument, never as the
/// format string, so braces in an operation name cannot break the generated source.
fn assertion_message(text: &str) -> String {
    format!("{text:?}")
}

/// The `kani::proof` function every generated module holds.
const HARNESS: &str = "check";

struct Postcondition<'a> {
    field: &'a str,
    comparison: StateComparison,
    left_is_pre: bool,
}

fn postcondition_body(
    abi: &Abi<'_>,
    scope: &StateFrameScope,
    domains: &[StateFieldDomain],
    condition: &Postcondition<'_>,
    module: &str,
) -> String {
    let Postcondition {
        field,
        comparison,
        left_is_pre,
    } = *condition;
    let (left, right) = if left_is_pre {
        ("pre", "post")
    } else {
        ("post", "pre")
    };
    let operator = comparison.rust();
    let message = assertion_message(&format!(
        "operation `{}`: postcondition `{left}.{field} {operator} {right}.{field}` failed",
        scope.operation
    ));
    format!(
        "#[cfg(kani)]\nmod {module} {{\n    use super::*;\n\n    #[kani::proof]\n    fn {HARNESS}() {{\n{state}        let mut post = pre.clone();\n        {subject}(&mut post);\n        kani::cover!(true, \"state bounds hold and the operation returns\");\n        assert!(\n            {left}.{field} {operator} {right}.{field},\n            \"{{}}\",\n            {message}\n        );\n    }}\n}}\n",
        state = symbolic_state(abi, domains),
        subject = abi.subject_path,
    )
}

fn frame_body(
    abi: &Abi<'_>,
    scope: &StateFrameScope,
    domains: &[StateFieldDomain],
    checked: &[String],
    module: &str,
) -> String {
    let assertions = checked
        .iter()
        .map(|field| {
            let message = assertion_message(&format!(
                "operation `{}` changed `{field}`, which its frame does not modify",
                scope.operation
            ));
            format!("        assert!(post.{field} == pre.{field}, \"{{}}\", {message});\n")
        })
        .collect::<String>();
    format!(
        "#[cfg(kani)]\nmod {module} {{\n    use super::*;\n\n    #[kani::proof]\n    fn {HARNESS}() {{\n{state}        let mut post = pre.clone();\n        {subject}(&mut post);\n        kani::cover!(true, \"the operation returns\");\n{assertions}    }}\n}}\n",
        state = symbolic_state(abi, domains),
        subject = abi.subject_path,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope(operation: &str) -> StateFrameScope {
        let id = |digest: &str| -> CheckedNodeId {
            serde_json::from_value(serde_json::json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": digest,
            }))
            .expect("a node id")
        };
        StateFrameScope {
            operation: operation.to_owned(),
            object: id(&"1".repeat(64)),
            anchor: id(&"2".repeat(64)),
            frame: id(&"3".repeat(64)),
        }
    }

    /// An operation name that would break a format string reaches the generated assertions as a
    /// format argument, never as the format string, in both generators.
    ///
    /// Trace: FR-015-AC-26, TC-025.
    #[test]
    fn tc_025_an_operation_name_with_braces_cannot_break_an_assertion() {
        let abi = Abi {
            state_path: "crate::State",
            subject_path: "crate::operate",
            state_fields: &["balance", "audit"],
        };
        let scope = scope("dep{osit}{}");
        let domains = [];
        let frame = frame_body(&abi, &scope, &domains, &["audit".to_owned()], "m");
        let postcondition = postcondition_body(
            &abi,
            &scope,
            &domains,
            &Postcondition {
                field: "balance",
                comparison: StateComparison::Ge,
                left_is_pre: false,
            },
            "m",
        );
        syn::parse_file(&frame).expect("the frame harness parses");
        syn::parse_file(&postcondition).expect("the postcondition harness parses");
        // `"{}"` is the format string and the operation-bearing literal its argument.
        assert!(frame.contains("pre.audit, \"{}\", \"operation `dep{osit}{}` changed"));
        assert!(postcondition.contains("\"{}\",\n            \"operation `dep{osit}{}`:"));
    }
}
