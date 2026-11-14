//! Symbolic reference-graph reachability claims at state-clause argument paths (FR-015-AC-98,
//! -105, -107, -108).
//!
//! The population extent comes only from `select_population` on the claim's typed population
//! selector, and the edge multiplicity only from the typed relationship-end accessor. No caller
//! bound, inferred population or reduced domain stands in for either. The harness carries every
//! count, degree and target as a native `i64`; it never builds a general-route `rt::Integer`.
use quire_contract_model::selected_population::{
    select_population, CheckedSelectedPopulation, CheckedSelectedPopulationOutcome as Population,
    CheckedSelectedPopulationSource,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedPackageV2, CheckedReachesPopulationError,
    CheckedRelationshipEnd, CheckedRelationshipEndError, CheckedTerm, ContractSourceV1,
    StateClauseKind,
};

use crate::{
    core::artifact::MAX_GENERATED_SOURCE_BYTES,
    kani::{
        abi::{adapter_options, witness_options, KaniPrimitiveType, KaniSolver},
        generate::{
            outcome::{
                GraphAccessorRefusal as Accessor, GraphPopulationRefusal as Selection,
                GraphUnsupportedReason as Reason, KaniObligationRequest, UnsupportedObligation,
            },
            record::{harness_path, record},
            render,
            spec::{
                Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement,
            },
        },
        identity::{
            GraphEdgeIdentity, GraphObligationIdentity, KaniGraphObligationHarness, ObligationKind,
            GRAPH_ASSERTION_MESSAGE,
        },
    },
};

/// Why a located claim has no harness.
pub(super) enum GraphRefusal {
    Unsupported(Reason),
    RequiresBound(CheckedNodeId),
}

impl From<Reason> for GraphRefusal {
    fn from(reason: Reason) -> Self {
        Self::Unsupported(reason)
    }
}

/// A claim whose population and edge domains are both finite and admitted.
pub(super) struct LoweredGraph {
    package: ContractSourceV1,
    clause: CheckedNodeId,
    occurrence: CheckedOccurrence,
    kind: ObligationKind,
    path: Vec<usize>,
    application: CheckedNodeId,
    population: CheckedNodeId,
    object_type: CheckedNodeId,
    maximum: u64,
    edge: GraphEdgeIdentity,
    condition: Claim,
}

/// A reachability operand: the clause's one `self` parameter, or the quantifier binder at a
/// nesting depth. An invariant has exactly one parameter, so every operand that is not a binder
/// is the object the harness started from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operand {
    Start,
    Bound(usize),
}

/// The admitted clause condition over one reachability relation: Boolean combinators,
/// reachability applications of the selected end, and quantifiers over the covering population.
#[derive(Debug, Eq, PartialEq)]
enum Claim {
    Reaches(Operand, Operand),
    Not(Box<Claim>),
    And(Box<Claim>, Box<Claim>),
    Or(Box<Claim>, Box<Claim>),
    Implies(Box<Claim>, Box<Claim>),
    Equal(Box<Claim>, Box<Claim>),
    /// `forall` (true) or `exists` (false) binder at this nesting depth over every live object.
    Quantified {
        forall: bool,
        depth: usize,
        body: Box<Claim>,
    },
}

/// The deepest quantifier nesting the oracle unrolls; each level multiplies the evaluation by
/// the population maximum.
const MAX_QUANTIFIER_DEPTH: usize = 4;

fn operation_identity(term: &CheckedTerm) -> Option<&str> {
    match term {
        CheckedTerm::Application { operation, .. } => operation.identity(),
        _ => None,
    }
}

fn is_reachability(term: &CheckedTerm) -> bool {
    matches!(
        operation_identity(term),
        Some("quire.op.model.reaches" | "quire.op.model.reaches_field")
    )
}

/// The owning clause form, independently of whether its selected path can be lowered.
pub(super) fn graph_kind(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
) -> Option<ObligationKind> {
    let CheckedTerm::Application { operation, .. } = package.term(clause)? else {
        return None;
    };
    match operation.state_clause_kind().ok().flatten()? {
        StateClauseKind::Invariant => Some(ObligationKind::Invariant),
        StateClauseKind::Precondition => Some(ObligationKind::Precondition),
        StateClauseKind::Postcondition => Some(ObligationKind::Postcondition),
        _ => None,
    }
}

/// Whether two node identities name one node. A package's terms may spell the same node as a
/// graph-local slot or as its structural key, so identities compare by structural key.
fn same_node(package: &CheckedPackageV2, left: &CheckedNodeId, right: &CheckedNodeId) -> bool {
    match (package.structural_key(left), package.structural_key(right)) {
        (Some(left), Some(right)) => left == right,
        _ => left == right,
    }
}

/// Whether two operand terms are the same: equal references to one node, or equal terms.
fn same_operand_term(package: &CheckedPackageV2, left: &CheckedTerm, right: &CheckedTerm) -> bool {
    match (left, right) {
        (CheckedTerm::Reference { target: left }, CheckedTerm::Reference { target: right }) => {
            same_node(package, left, right)
        }
        _ => left == right,
    }
}

/// A quantifier's binder is not an argument position of its own: the path continues at the term
/// it binds.
fn unbound(term: &CheckedTerm) -> &CheckedTerm {
    match term {
        CheckedTerm::Binding { value, .. } => unbound(value),
        other => other,
    }
}

/// Follow a reference to the node's own term, or stay on an inline term.
fn resolve<'a>(package: &'a CheckedPackageV2, term: &'a CheckedTerm) -> Option<&'a CheckedTerm> {
    match term {
        CheckedTerm::Reference { target } => package.term(target),
        other => Some(other),
    }
}

/// Locate first: a missing or non-reachability path is `MissingClaim` before any population or
/// edge input is read (FR-015-AC-107 precedes AC-108).
pub(super) fn lower_graph(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    occurrence: &CheckedOccurrence,
    path: &[usize],
) -> Result<LoweredGraph, GraphRefusal> {
    let missing = || GraphRefusal::Unsupported(Reason::MissingClaim);
    // The claim is keyed by the node and one of its own occurrences: an occurrence the node does
    // not carry names no claim.
    let is_state_clause = package.graph().nodes.iter().any(|node| {
        &node.node_id == clause
            && &*node.node_tag == "state"
            && &*node.semantic_form == "state_clause"
            && node.occurrences.contains(occurrence)
    });
    if !is_state_clause {
        return Err(missing());
    }
    let Some(CheckedTerm::Application {
        arguments: clause_arguments,
        ..
    }) = package.term(clause)
    else {
        return Err(missing());
    };
    let condition = clause_arguments.get(2).ok_or_else(missing)?;
    let mut selected = condition;
    for position in path {
        selected = unbound(
            resolve(package, selected)
                .and_then(|term| match term {
                    CheckedTerm::Application { arguments, .. } => arguments.get(*position),
                    _ => None,
                })
                .ok_or_else(missing)?,
        );
    }
    let CheckedTerm::Reference {
        target: application,
    } = selected
    else {
        return Err(missing());
    };
    let application_term = package
        .term(application)
        .filter(|term| is_reachability(term))
        .ok_or_else(missing)?;
    if !matches!(application_term, CheckedTerm::Application { .. }) {
        return Err(missing());
    }
    let kind = graph_kind(package, clause).ok_or(Reason::UnsupportedClaimShape)?;
    // A precondition or postcondition graph claim has no defined harness: the generated body
    // never calls the subject, assumes no sibling preconditions and has no totality or
    // satisfiability split, so a verdict over it would mean nothing for the operation.
    if kind != ObligationKind::Invariant {
        return Err(Reason::OperationClauseNotSupported.into());
    }

    // 1. The population selection and 2. the destination end of the selected application.
    let (population, end) = domain_of(package, application)?;
    let upper = end
        .upper
        .ok_or_else(|| GraphRefusal::RequiresBound(end.relationship.clone()))?;
    let maximum =
        population
            .maximum
            .as_str()
            .parse::<u64>()
            .map_err(|_| Reason::DomainNotRepresentable {
                population_maximum: population.maximum.as_str().to_owned(),
                edge_upper: upper,
            })?;

    // 3. The claim's shape: the condition is a Boolean combination of reachability applications
    // of this one end (the selected application and any other over the same population and
    // end) and quantifiers over that population, whose operands are the clause's `self` or a
    // quantifier binder.
    let Some(CheckedTerm::Aggregate {
        members: parameters,
    }) = clause_arguments.first()
    else {
        return Err(Reason::UnsupportedClaimShape.into());
    };
    if parameters.len() != 1 {
        return Err(Reason::UnsupportedClaimShape.into());
    }
    let mut shape = Shape {
        package,
        parameters,
        population: &population,
        end: &end,
        binders: Vec::new(),
    };
    let condition = shape.claim(condition, 0)?;
    Ok(LoweredGraph {
        package: ContractSourceV1 {
            name: package.name().into(),
            lock: package.lock().clone(),
        },
        clause: clause.clone(),
        occurrence: occurrence.clone(),
        kind,
        path: path.to_vec(),
        application: application.clone(),
        population: population.binding,
        object_type: population.object_type,
        maximum,
        edge: GraphEdgeIdentity {
            declaration: end.relationship,
            forward: end.forward,
            lower: end.lower,
            upper,
            ordered: end.ordered,
            unique: end.unique,
        },
        condition,
    })
}

/// The population and destination end one reachability application names. A `reaches_field`
/// application is a reachability application but names no population the owner accessor can
/// read: it refuses as `NotReachesApplication`, never a guessed same-type population.
fn domain_of(
    package: &CheckedPackageV2,
    application: &CheckedNodeId,
) -> Result<(CheckedSelectedPopulation, CheckedRelationshipEnd), GraphRefusal> {
    let selector = package.reaches_population(application).map_err(|error| {
        Reason::PopulationAccessorRefused {
            refusal: match error {
                CheckedReachesPopulationError::UnknownNode => Accessor::UnknownNode,
                CheckedReachesPopulationError::NotReachesApplication => {
                    Accessor::NotReachesApplication
                }
                CheckedReachesPopulationError::UnresolvedPopulation => {
                    Accessor::UnresolvedPopulation
                }
            },
        }
    })?;
    let source = selector.map_or(CheckedSelectedPopulationSource::Unavailable, |selector| {
        CheckedSelectedPopulationSource::Admitted { package, selector }
    });
    let population = finite_population(select_population(source))?;
    let end =
        package
            .relationship_end(application)
            .map_err(|error| Reason::RelationshipEndRefused {
                refusal: match error {
                    CheckedRelationshipEndError::UnknownNode => Accessor::UnknownNode,
                    CheckedRelationshipEndError::NotReachesApplication => {
                        Accessor::NotReachesApplication
                    }
                    CheckedRelationshipEndError::UnresolvedEnd => Accessor::UnresolvedEnd,
                },
            })?;
    if !same_node(package, &end.destination_type, &population.object_type) {
        return Err(Reason::PopulationTypeMismatch.into());
    }
    Ok((population, end))
}

/// The named `name`/`level` literal of a parameter node's aggregate body.
fn parameter_member<'a>(term: &'a CheckedTerm, member: &str) -> Option<&'a str> {
    let CheckedTerm::Aggregate { members } = term else {
        return None;
    };
    members.iter().find_map(|candidate| match candidate {
        CheckedTerm::Binding { name, value } if name.as_ref() == member => match &**value {
            CheckedTerm::Literal { value, .. } => value.as_str(),
            _ => None,
        },
        _ => None,
    })
}

/// What the condition may mention: the one population and end of the selected application, the
/// clause's parameter and the quantifier binders in scope (name and level).
struct Shape<'a> {
    package: &'a CheckedPackageV2,
    parameters: &'a [CheckedTerm],
    population: &'a CheckedSelectedPopulation,
    end: &'a CheckedRelationshipEnd,
    binders: Vec<(String, u64)>,
}

impl Shape<'_> {
    fn operand(&self, term: &CheckedTerm) -> Result<Operand, GraphRefusal> {
        let unsupported = || GraphRefusal::from(Reason::UnsupportedClaimShape);
        let CheckedTerm::Reference { target } = term else {
            return Err(unsupported());
        };
        if self
            .parameters
            .iter()
            .any(|parameter| same_operand_term(self.package, parameter, term))
        {
            return Ok(Operand::Start);
        }
        let node = self
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| same_node(self.package, &node.node_id, target))
            .filter(|node| &*node.semantic_form == "parameter")
            .filter(|node| {
                same_node(
                    self.package,
                    &node.semantic_type,
                    &self.population.object_type,
                )
            })
            .ok_or_else(unsupported)?;
        let body = self.package.term(&node.node_id).ok_or_else(unsupported)?;
        let name = parameter_member(body, "name").ok_or_else(unsupported)?;
        let level = parameter_member(body, "level")
            .and_then(|level| level.parse::<u64>().ok())
            .ok_or_else(unsupported)?;
        self.binders
            .iter()
            .position(|(bound, at)| bound == name && *at == level)
            .map(Operand::Bound)
            .ok_or_else(unsupported)
    }

    /// A reachability application over the selected population and end, or the shape refusal.
    fn reaches(&self, id: &CheckedNodeId, operands: &[CheckedTerm]) -> Result<Claim, GraphRefusal> {
        let unsupported = || GraphRefusal::from(Reason::UnsupportedClaimShape);
        let (population, end) = domain_of(self.package, id).map_err(|_| unsupported())?;
        if !same_node(self.package, &population.binding, &self.population.binding)
            || !same_node(self.package, &end.relationship, &self.end.relationship)
            || end.forward != self.end.forward
        {
            return Err(unsupported());
        }
        let [left, right] = operands else {
            return Err(unsupported());
        };
        Ok(Claim::Reaches(self.operand(left)?, self.operand(right)?))
    }

    /// `forall`/`exists` over every instance of the population's object type.
    fn quantifier(
        &mut self,
        forall: bool,
        arguments: &[CheckedTerm],
        depth: usize,
    ) -> Result<Claim, GraphRefusal> {
        let unsupported = || GraphRefusal::from(Reason::UnsupportedClaimShape);
        let [CheckedTerm::Reference { target: collection }, CheckedTerm::Binding { name, value }] =
            arguments
        else {
            return Err(unsupported());
        };
        let Some(CheckedTerm::Application { operation, .. }) = self.package.term(collection) else {
            return Err(unsupported());
        };
        if operation.identity() != Some("quire.op.model.all_instances")
            || !operation
                .member_declaration()
                .ok()
                .flatten()
                .is_some_and(|declaration| {
                    same_node(self.package, declaration, &self.population.object_type)
                })
        {
            return Err(unsupported());
        }
        let nesting = self.binders.len();
        if nesting >= MAX_QUANTIFIER_DEPTH {
            return Err(unsupported());
        }
        self.binders.push((
            name.as_ref().to_owned(),
            (self.parameters.len() + nesting) as u64,
        ));
        let body = self.claim(value, depth + 1);
        self.binders.pop();
        Ok(Claim::Quantified {
            forall,
            depth: nesting,
            body: Box::new(body?),
        })
    }

    fn claim(&mut self, term: &CheckedTerm, depth: usize) -> Result<Claim, GraphRefusal> {
        let unsupported = || GraphRefusal::from(Reason::UnsupportedClaimShape);
        if depth > 256 {
            return Err(unsupported());
        }
        let (id, body) = match term {
            CheckedTerm::Reference { target } => (
                Some(target),
                self.package.term(target).ok_or_else(unsupported)?,
            ),
            other => (None, other),
        };
        let CheckedTerm::Application {
            operation,
            arguments,
            ..
        } = body
        else {
            return Err(unsupported());
        };
        if is_reachability(body) {
            return self.reaches(id.ok_or_else(unsupported)?, arguments);
        }
        match operation.identity() {
            Some("quire.op.collection.forall") => return self.quantifier(true, arguments, depth),
            Some("quire.op.collection.exists") => return self.quantifier(false, arguments, depth),
            _ => {}
        }
        let mut rendered = Vec::with_capacity(arguments.len());
        for argument in arguments {
            rendered.push(self.claim(argument, depth + 1)?);
        }
        let mut operands = rendered.into_iter();
        match (
            operation.identity(),
            operands.next(),
            operands.next(),
            operands.next(),
        ) {
            (Some("quire.op.boolean.not"), Some(argument), None, None) => {
                Ok(Claim::Not(Box::new(argument)))
            }
            (Some("quire.op.boolean.and"), Some(left), Some(right), None) => {
                Ok(Claim::And(Box::new(left), Box::new(right)))
            }
            (Some("quire.op.boolean.or"), Some(left), Some(right), None) => {
                Ok(Claim::Or(Box::new(left), Box::new(right)))
            }
            (Some("quire.op.boolean.implies"), Some(left), Some(right), None) => {
                Ok(Claim::Implies(Box::new(left), Box::new(right)))
            }
            (Some("quire.op.boolean.eq"), Some(left), Some(right), None) => {
                Ok(Claim::Equal(Box::new(left), Box::new(right)))
            }
            _ => Err(unsupported()),
        }
    }
}

impl Claim {
    /// The claim as a Rust Boolean expression over `paths`, `count` and `start`. A quantifier
    /// ranges over the live objects `0..count` of the `maximum`-sized storage.
    fn rust(&self, maximum: u64) -> String {
        let operand = |operand: &Operand| match operand {
            Operand::Start => "(start as usize)".to_owned(),
            Operand::Bound(depth) => format!("q{depth}"),
        };
        match self {
            Self::Reaches(from, to) => format!("paths[{}][{}]", operand(from), operand(to)),
            Self::Not(argument) => format!("!({})", argument.rust(maximum)),
            Self::And(left, right) => {
                format!("({}) && ({})", left.rust(maximum), right.rust(maximum))
            }
            Self::Or(left, right) => {
                format!("({}) || ({})", left.rust(maximum), right.rust(maximum))
            }
            Self::Implies(left, right) => {
                format!("!({}) || ({})", left.rust(maximum), right.rust(maximum))
            }
            Self::Equal(left, right) => {
                format!("({}) == ({})", left.rust(maximum), right.rust(maximum))
            }
            Self::Quantified {
                forall,
                depth,
                body,
            } => {
                let (name, start, join) = if *forall {
                    ("all", "true", "&&")
                } else {
                    ("any", "false", "||")
                };
                format!(
                    "{{ let mut {name}{depth} = {start}; for q{depth} in 0..{maximum} {{ if (q{depth} as i64) < count {{ {name}{depth} = {name}{depth} {join} ({}); }} }} {name}{depth} }}",
                    body.rust(maximum)
                )
            }
        }
    }
}

fn finite_population(outcome: Population) -> Result<CheckedSelectedPopulation, GraphRefusal> {
    let refusal = match outcome {
        Population::Finite(population) => return Ok(population),
        Population::Unbounded { population, .. }
        | Population::UnknownClosureOpenExtent { population, .. } => {
            return Err(GraphRefusal::RequiresBound(population))
        }
        Population::AdmittedUnbounded(unbounded) => {
            return Err(GraphRefusal::RequiresBound(unbounded.binding))
        }
        Population::Unavailable => return Err(Reason::PopulationUnavailable.into()),
        Population::ForeignPackage(_) => Selection::ForeignPackage,
        Population::StaleBinding(_) => Selection::StaleBinding,
        Population::NotPopulation(_) => Selection::NotPopulation,
        Population::MissingOccurrence(_) => Selection::MissingOccurrence,
        Population::UnknownClosureUnresolvedObjectType { .. } => {
            Selection::UnknownClosureUnresolvedObjectType
        }
        Population::UnknownClosureUnresolvedSubtypeClosure { .. } => {
            Selection::UnknownClosureUnresolvedSubtypeClosure
        }
    };
    Err(Reason::PopulationSelectionRefused { refusal }.into())
}

pub(super) fn refusal(
    clause: &CheckedNodeId,
    occurrence: &CheckedOccurrence,
    path: &[usize],
    reason: Reason,
) -> UnsupportedObligation {
    UnsupportedObligation::GraphClaimNotRendered {
        clause_id: clause.clone(),
        occurrence: occurrence.clone(),
        argument_path: path.to_vec(),
        reason,
    }
}

fn input(name: String, minimum: u64, maximum: u64) -> Statement {
    Statement::Input {
        name,
        ty: KaniPrimitiveType::I64,
        domain: (i128::from(minimum), i128::from(maximum)),
        then: vec![],
    }
}

fn expression(text: impl Into<String>) -> Result<ExprText, UnsupportedObligation> {
    Ok(ExprText::new(text)?)
}

fn assume(body: &mut Vec<Statement>, condition: String) -> Result<(), UnsupportedObligation> {
    body.push(Statement::Assume {
        condition: expression(condition)?,
    });
    Ok(())
}

/// Render every admitted slot. A representation or resource refusal never shrinks a domain.
pub(super) fn render_graph(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredGraph,
) -> Result<KaniGraphObligationHarness, UnsupportedObligation> {
    let refuse = |reason| refusal(&lowered.clause, &lowered.occurrence, &lowered.path, reason);
    let n = lowered.maximum;
    let upper = lowered.edge.upper;
    let not_representable = || {
        refuse(Reason::DomainNotRepresentable {
            population_maximum: n.to_string(),
            edge_upper: upper,
        })
    };
    let required = n.max(upper).checked_add(1).ok_or_else(not_representable)?;
    if u64::from(request.unwind) < required {
        return Err(refuse(Reason::UnwindBelowDomain {
            required,
            supplied: request.unwind,
        }));
    }
    let slots = n
        .checked_mul(upper)
        .and_then(|edges| edges.checked_add(n))
        .ok_or_else(not_representable)?;
    let bytes = slots
        .checked_mul(8)
        .and_then(|bytes| bytes.checked_add(n.checked_mul(n)?))
        .and_then(|bytes| bytes.checked_add(24))
        .ok_or_else(not_representable)?;
    if bytes > request.ceilings.memory_bytes.get() {
        return Err(refuse(Reason::DomainExceedsMemoryCeiling {
            required: bytes,
            supplied: request.ceilings.memory_bytes.get(),
        }));
    }
    let uniqueness = if lowered.edge.unique {
        n.checked_mul(upper)
            .and_then(|pairs| pairs.checked_mul(upper.saturating_sub(1)))
            .map(|pairs| pairs / 2)
    } else {
        Some(0)
    };
    if n > i64::MAX as u64
        || upper > i64::MAX as u64
        || uniqueness
            .and_then(|checks| checks.checked_add(slots))
            .is_none_or(|units| units > (MAX_GENERATED_SOURCE_BYTES / 256) as u64)
    {
        return Err(not_representable());
    }
    let material = crate::core::canonical::preimage_value(&(
        &lowered.package,
        &lowered.clause,
        &lowered.occurrence,
        lowered.kind,
        &lowered.path,
        &lowered.application,
        &lowered.population,
        &lowered.object_type,
        n,
        &lowered.edge,
        request.subject_path,
        request.unwind,
        request.ceilings,
    ))
    .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let digest = crate::core::canonical::content_digest(&material)
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let suffix: String = digest
        .iter()
        .take(12)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let path = harness_path(
        &format!("kob_graph_{suffix}"),
        &format!("prove_graph_{suffix}"),
    )?;
    let helper = format!("graph_condition_{suffix}");
    let identity = GraphObligationIdentity {
        ceilings: request.ceilings,
        package: lowered.package.clone(),
        clause_id: lowered.clause.clone(),
        occurrence: lowered.occurrence.clone(),
        kind: lowered.kind,
        argument_path: lowered.path.clone(),
        application_id: lowered.application.clone(),
        population_id: lowered.population.clone(),
        object_type: lowered.object_type.clone(),
        maximum_population: n,
        edge: lowered.edge.clone(),
        subject_path: request.subject_path.into(),
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options: adapter_options(
            &path.to_string(),
            request.unwind,
            KaniSolver::Cadical,
            false,
        ),
        witness_options: witness_options(
            &path.to_string(),
            request.unwind,
            KaniSolver::Cadical,
            false,
        ),
        module_symbol: path.module.clone(),
        harness_symbol: path.harness.clone(),
    };
    let mut body = vec![
        input("object_count".into(), 0, n),
        input("start".into(), 0, n.saturating_sub(1)),
        input("target".into(), 0, n.saturating_sub(1)),
    ];
    assume(
        &mut body,
        "object_count == 0 || (start < object_count && target < object_count)".into(),
    )?;
    // An invariant has one parameter, so the harness draws one object for `self`; `target` stays
    // a drawn slot of the layout (the replay decodes it) and equals `start`.
    assume(&mut body, "start == target".into())?;
    let mut degrees = Vec::new();
    let mut rows = Vec::new();
    for row in 0..n {
        let degree = format!("degree_{row}");
        body.push(input(degree.clone(), 0, upper));
        assume(
            &mut body,
            format!(
                "object_count <= {row} || {degree} >= {}",
                lowered.edge.lower
            ),
        )?;
        degrees.push(degree.clone());
        let mut entries: Vec<String> = Vec::new();
        for column in 0..upper {
            let name = format!("edge_{row}_{column}");
            body.push(input(name.clone(), 0, n.saturating_sub(1)));
            assume(
                &mut body,
                format!("object_count <= {row} || {degree} <= {column} || {name} < object_count"),
            )?;
            if lowered.edge.unique {
                for previous in &entries {
                    assume(
                        &mut body,
                        format!(
                            "object_count <= {row} || {degree} <= {column} || {name} != {previous}"
                        ),
                    )?;
                }
            }
            entries.push(name);
        }
        rows.push(format!("[{}]", entries.join(",")));
    }
    body.push(Statement::Let(LetBinding {
        name: "degrees".into(),
        ty: None,
        mutable: false,
        init: Init::Expression(expression(format!("[{}]", degrees.join(",")))?),
    }));
    body.push(Statement::Let(LetBinding {
        name: "edges".into(),
        ty: None,
        mutable: false,
        init: Init::Expression(expression(format!("[{}]", rows.join(",")))?),
    }));
    body.push(Statement::Call {
        result: "claim".into(),
        callee: helper.clone(),
        args: vec![
            Argument::Value("object_count".into()),
            Argument::Borrow("degrees".into()),
            Argument::Borrow("edges".into()),
            Argument::Value("start".into()),
        ],
    });
    body.push(Statement::Assert {
        condition: expression("object_count == 0 || claim")?,
        message: GRAPH_ASSERTION_MESSAGE.into(),
    });
    // Reachability follows only the selected edge, by positive-length search: the closure starts
    // at the selected edges rather than at an identity relation, so an isolated object does not
    // reach itself and a self-loop does. Storage extents bound unrolling; the live guards keep
    // the full count and degree draws.
    let oracle = format!(
        "fn {helper}(count: i64, degrees: &[i64; {n}], edges: &[[i64; {upper}]; {n}], start: i64) -> bool {{\n\
         let _ = start;\n\
         if count == 0 {{ return true; }}\n\
         let mut paths = [[false; {n}]; {n}];\n\
         for row in 0..{n} {{ for edge in 0..{upper} {{ if (row as i64) < count && (edge as i64) < degrees[row] {{ paths[row][edges[row][edge] as usize] = true; }} }} }}\n\
         for via in 0..{n} {{ for from in 0..{n} {{ for to in 0..{n} {{ paths[from][to] = paths[from][to] || (paths[from][via] && paths[via][to]); }} }} }}\n\
         {}\n}}",
        lowered.condition.rust(n)
    );
    let spec = HarnessSpec::new(
        "admitted graph clause".into(),
        path.clone(),
        oracle,
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expression(format!("object_count == {n}"))?,
            message: "the maximum-size population is drawn".into(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rendered = render::render(&spec)?;
    let source = crate::core::provenance::with_rust_header(
        rendered.contents,
        [
            &lowered.clause,
            &lowered.application,
            &lowered.population,
            &lowered.edge.declaration,
        ]
        .into_iter()
        .map(crate::core::provenance::node_subject)
        .collect::<Vec<_>>()
        .iter()
        .map(String::as_str),
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    let rust = crate::core::artifact::Artifact::new(rendered.path, source);
    let record = record(path.module.as_str(), &identity, &rust)?;
    let harness = KaniGraphObligationHarness::generated(identity, rust, record);
    if !harness.has_valid_provenance() {
        return Err(UnsupportedObligation::RenderFailed);
    }
    Ok(harness)
}
