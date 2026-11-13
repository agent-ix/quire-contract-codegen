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
    CheckedNodeId, CheckedPackageV2, CheckedReachesPopulationError, CheckedRelationshipEndError,
    CheckedTerm, ContractSourceV1, StateClauseKind,
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
    kind: ObligationKind,
    path: Vec<usize>,
    application: CheckedNodeId,
    population: CheckedNodeId,
    object_type: CheckedNodeId,
    maximum: u64,
    edge: GraphEdgeIdentity,
    condition: String,
    same_operand: bool,
}

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
    path: &[usize],
) -> Result<LoweredGraph, GraphRefusal> {
    let missing = || GraphRefusal::Unsupported(Reason::MissingClaim);
    let is_state_clause = package.graph().nodes.iter().any(|node| {
        &node.node_id == clause
            && &*node.node_tag == "state"
            && &*node.semantic_form == "state_clause"
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
        selected = resolve(package, selected)
            .and_then(|term| match term {
                CheckedTerm::Application { arguments, .. } => arguments.get(*position),
                _ => None,
            })
            .ok_or_else(missing)?;
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
    let CheckedTerm::Application {
        arguments: operands,
        ..
    } = application_term
    else {
        return Err(missing());
    };
    let kind = graph_kind(package, clause).ok_or(Reason::UnsupportedClaimShape)?;

    // 1. The population selection. A `reaches_field` application is a reachability application
    // but names no population the owner accessor can read: it refuses as
    // `NotReachesApplication`, never a guessed same-type population.
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
    let maximum =
        population
            .maximum
            .as_str()
            .parse::<u64>()
            .map_err(|_| Reason::DomainNotRepresentable {
                population_maximum: u64::MAX,
                edge_upper: 0,
            })?;

    // 2. The destination end's finite multiplicity.
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
    let upper = end
        .upper
        .ok_or_else(|| GraphRefusal::RequiresBound(end.relationship.clone()))?;

    // 3. The claim's shape: both operands are reference parameters of the clause, and the
    // condition is a Boolean combination of this one application.
    let [left, right] = operands.as_slice() else {
        return Err(Reason::UnsupportedClaimShape.into());
    };
    let Some(CheckedTerm::Aggregate {
        members: parameters,
    }) = clause_arguments.first()
    else {
        return Err(Reason::UnsupportedClaimShape.into());
    };
    if ![left, right].iter().all(|operand| {
        parameters
            .iter()
            .any(|parameter| same_operand_term(package, parameter, operand))
    }) {
        return Err(Reason::UnsupportedClaimShape.into());
    }
    let condition = lower_condition(package, condition, application, 0)?;
    Ok(LoweredGraph {
        package: ContractSourceV1 {
            name: package.name().into(),
            lock: package.lock().clone(),
        },
        clause: clause.clone(),
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
        same_operand: same_operand_term(package, left, right),
    })
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

/// Render the clause condition as a Boolean expression over the one `reachable` variable.
fn lower_condition(
    package: &CheckedPackageV2,
    term: &CheckedTerm,
    selected: &CheckedNodeId,
    depth: usize,
) -> Result<String, GraphRefusal> {
    if depth > 256 {
        return Err(Reason::UnsupportedClaimShape.into());
    }
    if matches!(term, CheckedTerm::Reference { target } if same_node(package, target, selected)) {
        return Ok("reachable".into());
    }
    let body = resolve(package, term).ok_or(Reason::UnsupportedClaimShape)?;
    let CheckedTerm::Application {
        operation,
        arguments,
        ..
    } = body
    else {
        return Err(Reason::UnsupportedClaimShape.into());
    };
    let mut rendered = Vec::with_capacity(arguments.len());
    for argument in arguments {
        rendered.push(lower_condition(package, argument, selected, depth + 1)?);
    }
    match (operation.identity(), rendered.as_slice()) {
        (Some("quire.op.boolean.not"), [argument]) => Ok(format!("!({argument})")),
        (Some("quire.op.boolean.and"), [left, right]) => Ok(format!("({left}) && ({right})")),
        (Some("quire.op.boolean.or"), [left, right]) => Ok(format!("({left}) || ({right})")),
        (Some("quire.op.boolean.implies"), [left, right]) => Ok(format!("!({left}) || ({right})")),
        (Some("quire.op.boolean.eq"), [left, right]) => Ok(format!("({left}) == ({right})")),
        _ => Err(Reason::UnsupportedClaimShape.into()),
    }
}

pub(super) fn refusal(
    clause: &CheckedNodeId,
    path: &[usize],
    reason: Reason,
) -> UnsupportedObligation {
    UnsupportedObligation::GraphClaimNotRendered {
        clause_id: clause.clone(),
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
    let refuse = |reason| refusal(&lowered.clause, &lowered.path, reason);
    let n = lowered.maximum;
    let upper = lowered.edge.upper;
    let not_representable = || {
        refuse(Reason::DomainNotRepresentable {
            population_maximum: n,
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
    if lowered.same_operand {
        assume(&mut body, "start == target".into())?;
    }
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
            Argument::Value("target".into()),
        ],
    });
    body.push(Statement::Assert {
        condition: expression("object_count == 0 || claim")?,
        message: "the symbolic graph satisfies the admitted clause condition".into(),
    });
    // Reachability follows only the selected edge, by positive-length search: the closure starts
    // at the selected edges rather than at an identity relation, so an isolated object does not
    // reach itself and a self-loop does. Storage extents bound unrolling; the live guards keep
    // the full count and degree draws.
    let oracle = format!(
        "fn {helper}(count: i64, degrees: &[i64; {n}], edges: &[[i64; {upper}]; {n}], start: i64, target: i64) -> bool {{\n\
         let mut paths = [[false; {n}]; {n}];\n\
         for row in 0..{n} {{ for edge in 0..{upper} {{ if (row as i64) < count && (edge as i64) < degrees[row] {{ paths[row][edges[row][edge] as usize] = true; }} }} }}\n\
         for via in 0..{n} {{ for from in 0..{n} {{ for to in 0..{n} {{ paths[from][to] = paths[from][to] || (paths[from][via] && paths[via][to]); }} }} }}\n\
         let reachable = if count == 0 {{ false }} else {{ paths[start as usize][target as usize] }};\n\
         {}\n}}",
        lowered.condition
    );
    let spec = HarnessSpec::new(
        "admitted graph clause".into(),
        path.clone(),
        oracle,
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expression(format!("object_count == 0 || object_count == {n}"))?,
            message: "an inclusive population endpoint is reachable".into(),
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
