//! Source-bound symbolic reference graphs at state-clause argument paths (FR-015).
//!
//! Population extent and edge multiplicity come only from admitted owner projections.
use quire_contract_model::selected_population::{
    select_population, CheckedSelectedPopulation, CheckedSelectedPopulationOutcome as Population,
    CheckedSelectedPopulationSource,
};
use quire_contract_model::{
    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedPackageV2,
    CheckedReachesPopulationError, CheckedRelationshipEndError, CheckedSemanticNodeV2,
};
use serde_json::Value;

use crate::kani::{
    abi::{adapter_options, KaniPrimitiveType, KaniSolver},
    generate::{
        outcome::{
            GraphAccessorRefusal as Accessor, GraphPopulationRefusal as Selection,
            GraphUnsupportedReason as Reason, KaniObligationRequest, UnsupportedObligation,
        },
        record::{harness_path, record},
        render,
        spec::{Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement},
    },
    identity::{
        GraphEdgeIdentity, GraphObligationIdentity, KaniGraphObligationHarness, ObligationKind,
    },
};

pub(super) enum GraphRefusal {
    Unsupported(Reason),
    RequiresBound(CheckedNodeId),
}
impl From<Reason> for GraphRefusal {
    fn from(reason: Reason) -> Self {
        Self::Unsupported(reason)
    }
}

pub(super) struct LoweredGraph {
    package_digest: String,
    clause: CheckedNodeId,
    kind: ObligationKind,
    path: Vec<usize>,
    application: CheckedNodeId,
    population: CheckedSelectedPopulation,
    edge: GraphEdgeIdentity,
    condition: String,
    same_operand: bool,
}

/// The three domain-construction mutations; the admitted input and evaluator stay intact.
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum BoundMutation {
    PopulationPlus,
    PopulationMinus,
    EdgeUpperPlus,
}

#[cfg(test)]
pub(crate) fn bound_mutant_for_test(
    request: &KaniObligationRequest<'_>,
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    path: &[usize],
    mutation: BoundMutation,
) -> KaniGraphObligationHarness {
    let mut lowered = match lower_graph(package, clause, path) {
        Ok(lowered) => lowered,
        Err(_) => panic!("the original authored graph must lower before mutation"),
    };
    match mutation {
        BoundMutation::PopulationPlus => {
            lowered.population.maximum = lowered.population.maximum.checked_add(1).unwrap();
        }
        BoundMutation::PopulationMinus => {
            lowered.population.maximum = lowered.population.maximum.checked_sub(1).unwrap();
        }
        BoundMutation::EdgeUpperPlus => {
            lowered.edge.upper = lowered.edge.upper.checked_add(1).unwrap();
        }
    }
    render_graph(request, &lowered).expect("the entire mutated domain must remain representable")
}

fn node<'a>(
    package: &'a CheckedPackageV2,
    id: &CheckedNodeId,
) -> Option<&'a CheckedSemanticNodeV2> {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == id)
}
fn target(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.get("target")?.clone()).ok()
}
fn resolve<'a>(package: &'a CheckedPackageV2, value: &'a Value) -> Option<&'a Value> {
    if value.get("term")?.as_str()? == "reference" {
        Some(&node(package, &target(value)?)?.body)
    } else {
        Some(value)
    }
}
fn operation(body: &Value) -> Option<&str> {
    body.pointer("/operation/identity").and_then(Value::as_str)
}
fn reaches(body: &Value) -> bool {
    matches!(
        operation(body),
        Some("quire.op.model.reaches" | "quire.op.model.reaches_field")
    )
}

/// The owning clause form, independently of whether its selected path can be lowered.
pub(super) fn graph_kind(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
) -> Option<ObligationKind> {
    match node(package, clause)?
        .body
        .pointer("/operation/member/clause")?
        .as_str()?
    {
        "invariant" => Some(ObligationKind::Invariant),
        "precondition" => Some(ObligationKind::Precondition),
        "postcondition" => Some(ObligationKind::Postcondition),
        _ => None,
    }
}

/// Locate first: even a malformed population must not hide a missing occurrence.
pub(super) fn lower_graph(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    path: &[usize],
) -> Result<LoweredGraph, GraphRefusal> {
    let missing = || GraphRefusal::Unsupported(Reason::MissingClaim);
    let clause_node = node(package, clause)
        .filter(|node| {
            node.node_tag.as_ref() == "state" && node.semantic_form.as_ref() == "state_clause"
        })
        .ok_or_else(missing)?;
    let condition = clause_node
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .and_then(|args| args.get(2))
        .ok_or_else(missing)?;
    let mut selected = condition;
    for position in path {
        selected = resolve(package, selected)
            .and_then(|body| body.get("arguments"))
            .and_then(Value::as_array)
            .and_then(|arguments| arguments.get(*position))
            .ok_or_else(missing)?;
    }
    let application = target(selected).ok_or_else(missing)?;
    let body = &node(package, &application)
        .filter(|node| reaches(&node.body))
        .ok_or_else(missing)?
        .body;
    // The current owner accessor covers relationship ends only. A field application with
    // no population link is Unavailable, rather than a guessed same-type population.
    let selector = if operation(body) == Some("quire.op.model.reaches") {
        package.reaches_population(&application).map_err(|error| {
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
        })?
    } else {
        // No owner accessor currently supplies a field-edge selection; absence is an honest
        // unavailable result. A future producer link needs its corresponding typed owner API.
        None
    };
    let source = selector.map_or(CheckedSelectedPopulationSource::Unavailable, |selector| {
        CheckedSelectedPopulationSource::Admitted { package, selector }
    });
    let population = population(select_population(source))?;
    let edge = edge(package, &application, body, &population.object_type)?;
    let args = body
        .get("arguments")
        .and_then(Value::as_array)
        .ok_or(Reason::UnsupportedClaimShape)?;
    let [left, right] = args.as_slice() else {
        return Err(Reason::UnsupportedClaimShape.into());
    };
    let same_operand = left == right;
    // Bind only reference parameters of the containing clause. A field navigation operand
    // would need a second independently drawn edge domain and is refused rather than erased.
    let parameters = clause_node
        .body
        .pointer("/arguments/0/members")
        .and_then(Value::as_array)
        .ok_or(Reason::UnsupportedClaimShape)?;
    if ![left, right]
        .iter()
        .all(|arg| parameters.iter().any(|parameter| parameter == *arg))
    {
        return Err(Reason::UnsupportedClaimShape.into());
    }
    let condition = lower_condition(package, condition, &application, Some(path), 0)?;
    Ok(LoweredGraph {
        package_digest: package.package_id().digest.to_string(),
        clause: clause.clone(),
        kind: graph_kind(package, clause).ok_or(Reason::UnsupportedClaimShape)?,
        path: path.to_vec(),
        application,
        population,
        edge,
        condition,
        same_operand,
    })
}

fn population(outcome: Population) -> Result<CheckedSelectedPopulation, GraphRefusal> {
    let reason = match outcome {
        Population::Finite(population) => return Ok(population),
        Population::Unbounded { population, .. }
        | Population::UnknownClosureOpenExtent { population, .. } => {
            return Err(GraphRefusal::RequiresBound(population))
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
    Err(Reason::PopulationSelectionRefused { refusal: reason }.into())
}

fn edge(
    package: &CheckedPackageV2,
    application: &CheckedNodeId,
    body: &Value,
    object: &CheckedNodeId,
) -> Result<GraphEdgeIdentity, GraphRefusal> {
    let operation = operation(body)
        .ok_or(Reason::UnsupportedClaimShape)?
        .to_owned();
    let member = body
        .pointer("/operation/member")
        .ok_or(Reason::UnsupportedClaimShape)?;
    let name = member
        .get("name")
        .and_then(Value::as_str)
        .ok_or(Reason::UnsupportedClaimShape)?
        .to_owned();
    if operation == "quire.op.model.reaches" {
        let end = package.relationship_end(application).map_err(|error| {
            Reason::RelationshipEndRefused {
                refusal: match error {
                    CheckedRelationshipEndError::UnknownNode => Accessor::UnknownNode,
                    CheckedRelationshipEndError::NotReachesApplication => {
                        Accessor::NotReachesApplication
                    }
                    CheckedRelationshipEndError::UnresolvedEnd => Accessor::UnresolvedEnd,
                },
            }
        })?;
        if &end.destination_type != object {
            return Err(Reason::PopulationTypeMismatch.into());
        }
        let upper = end
            .upper
            .ok_or_else(|| GraphRefusal::RequiresBound(end.relationship.clone()))?;
        return Ok(GraphEdgeIdentity {
            operation,
            declaration: end.relationship,
            name,
            lower: end.lower,
            upper,
            ordered: end.ordered,
            unique: end.unique,
        });
    }
    let declaration: CheckedNodeId = serde_json::from_value(
        member
            .get("declaration")
            .ok_or(Reason::FieldNotAdmitted)?
            .clone(),
    )
    .map_err(|_| Reason::FieldNotAdmitted)?;
    let fields = package
        .model_object_fields(&declaration)
        .map_err(|_| Reason::FieldNotAdmitted)?;
    let field = fields.field(&name).ok_or(Reason::FieldNotAdmitted)?;
    let (destination, lower, upper, ordered, unique) =
        match field.member_type().ok_or(Reason::UnsupportedFieldType)? {
            CheckedMemberType::Reference(destination) => (destination, 1, 1, false, true),
            CheckedMemberType::Option(element) => match element.as_ref() {
                CheckedMemberType::Reference(destination) => (destination, 0, 1, false, true),
                _ => return Err(Reason::UnsupportedFieldType.into()),
            },
            CheckedMemberType::Collection {
                kind,
                element,
                bounds,
            } => {
                let (lower, upper) =
                    bounds.ok_or_else(|| GraphRefusal::RequiresBound(declaration.clone()))?;
                let CheckedMemberType::Reference(destination) = element.as_ref() else {
                    return Err(Reason::UnsupportedFieldType.into());
                };
                let (ordered, unique) = match kind {
                    CheckedCollectionKind::Set => (false, true),
                    CheckedCollectionKind::Bag => (false, false),
                    CheckedCollectionKind::Sequence => (true, false),
                    CheckedCollectionKind::OrderedSet => (true, true),
                };
                (destination, lower, upper, ordered, unique)
            }
            _ => return Err(Reason::UnsupportedFieldType.into()),
        };
    if destination != object {
        return Err(Reason::PopulationTypeMismatch.into());
    }
    Ok(GraphEdgeIdentity {
        operation,
        declaration,
        name,
        lower,
        upper,
        ordered,
        unique,
    })
}

fn lower_condition(
    package: &CheckedPackageV2,
    term: &Value,
    selected: &CheckedNodeId,
    selection_path: Option<&[usize]>,
    depth: usize,
) -> Result<String, GraphRefusal> {
    if depth > 256 {
        return Err(Reason::UnsupportedClaimShape.into());
    }
    if target(term).as_ref() == Some(selected) {
        return Ok(if selection_path == Some(&[][..]) {
            "selected_reachable"
        } else {
            "reachable"
        }
        .into());
    }
    let body = resolve(package, term).ok_or(Reason::UnsupportedClaimShape)?;
    let args = body
        .get("arguments")
        .and_then(Value::as_array)
        .ok_or(Reason::UnsupportedClaimShape)?;
    match (operation(body), args.as_slice()) {
        (Some("quire.op.boolean.not"), [arg]) => Ok(format!(
            "!({})",
            lower_condition(
                package,
                arg,
                selected,
                selection_path.and_then(|path| path.strip_prefix(&[0])),
                depth + 1
            )?
        )),
        (
            Some(
                identity @ ("quire.op.boolean.and"
                | "quire.op.boolean.or"
                | "quire.op.boolean.implies"
                | "quire.op.boolean.eq"),
            ),
            [left, right],
        ) => {
            let left = lower_condition(
                package,
                left,
                selected,
                selection_path.and_then(|path| path.strip_prefix(&[0])),
                depth + 1,
            )?;
            let right = lower_condition(
                package,
                right,
                selected,
                selection_path.and_then(|path| path.strip_prefix(&[1])),
                depth + 1,
            )?;
            Ok(match identity {
                "quire.op.boolean.and" => format!("({left}) && ({right})"),
                "quire.op.boolean.or" => format!("({left}) || ({right})"),
                "quire.op.boolean.implies" => format!("!({left}) || ({right})"),
                "quire.op.boolean.eq" => format!("({left}) == ({right})"),
                _ => return Err(Reason::UnsupportedClaimShape.into()),
            })
        }
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
fn binding(name: &str, value: String) -> Result<Statement, UnsupportedObligation> {
    Ok(Statement::Let(LetBinding {
        name: name.into(),
        ty: None,
        mutable: false,
        init: Init::Expression(expression(value)?),
    }))
}
fn assume(body: &mut Vec<Statement>, condition: String) -> Result<(), UnsupportedObligation> {
    body.push(Statement::Assume {
        condition: expression(condition)?,
    });
    Ok(())
}

/// Render all admitted slots; representation/resource refusal never shrinks a domain.
pub(super) fn render_graph(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredGraph,
) -> Result<KaniGraphObligationHarness, UnsupportedObligation> {
    let refuse = |reason| refusal(&lowered.clause, &lowered.path, reason);
    let n = lowered.population.maximum;
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
    let bytes = n
        .checked_mul(upper)
        .and_then(|slots| slots.checked_add(n))
        .and_then(|slots| slots.checked_mul(8))
        .and_then(|bytes| bytes.checked_add(n.checked_mul(n)?))
        .and_then(|bytes| bytes.checked_add(24))
        .ok_or_else(not_representable)?;
    if bytes > request.ceilings.memory_bytes.get() {
        return Err(refuse(Reason::DomainExceedsMemoryCeiling {
            required: bytes,
            supplied: request.ceilings.memory_bytes.get(),
        }));
    }
    // Each unrolled symbolic slot has a bounded source cost, independently of backend memory.
    let slots = n
        .checked_mul(upper)
        .and_then(|slots| slots.checked_add(n))
        .ok_or_else(not_representable)?;
    let uniqueness_checks = if lowered.edge.unique {
        n.checked_mul(upper)
            .and_then(|pairs| pairs.checked_mul(upper.saturating_sub(1)))
            .map(|pairs| pairs / 2)
    } else {
        Some(0)
    };
    let source_units = uniqueness_checks.and_then(|checks| checks.checked_add(slots));
    if n > i64::MAX as u64
        || upper > i64::MAX as u64
        || source_units.is_none_or(|units| {
            units > (crate::core::artifact::MAX_GENERATED_SOURCE_BYTES / 256) as u64
        })
    {
        return Err(not_representable());
    }
    let material = crate::core::canonical::preimage_value(&(
        &lowered.package_digest,
        &lowered.clause,
        lowered.kind,
        &lowered.path,
        &lowered.application,
        &lowered.population.binding,
        &lowered.population.object_type,
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
        package_digest: lowered.package_digest.clone(),
        clause_id: lowered.clause.clone(),
        kind: lowered.kind,
        argument_path: lowered.path.clone(),
        application_id: lowered.application.clone(),
        population_id: lowered.population.binding.clone(),
        object_type: lowered.population.object_type.clone(),
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
        let mut entries = Vec::new();
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
    body.push(binding("degrees", format!("[{}]", degrees.join(",")))?);
    body.push(binding("edges", format!("[{}]", rows.join(",")))?);
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
    // Storage extents bound solver unrolling; live guards preserve the full count/degree draws.
    // Transitive closure starts at the selected edges, not at an identity relation. Thus an
    // isolated vertex does not reach itself, while a selected self edge does.
    let oracle = format!("fn {helper}(count: i64, degrees: &[i64; {n}], edges: &[[i64; {upper}]; {n}], start: i64, target: i64) -> bool {{\nlet mut paths = [[false; {n}]; {n}];\nfor row in 0..{n} {{ for edge in 0..{upper} {{ if row < count as usize && edge < degrees[row] as usize {{ paths[row][edges[row][edge] as usize] = true; }} }} }}\nfor via in 0..{n} {{ for from in 0..{n} {{ for to in 0..{n} {{ paths[from][to] = paths[from][to] || (paths[from][via] && paths[via][to]); }} }} }}\nlet reachable = if count == 0 {{ false }} else {{ paths[start as usize][target as usize] }};\nlet selected_reachable = reachable;\n{}\n}}",lowered.condition);
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
    let mut rust = render::render(&spec)?;
    rust.contents = crate::core::provenance::with_rust_header(
        rust.contents,
        [
            &lowered.clause,
            &lowered.application,
            &lowered.population.binding,
            &lowered.edge.declaration,
        ]
        .iter()
        .map(|id| crate::core::provenance::node_subject(id))
        .collect::<Vec<_>>()
        .iter()
        .map(String::as_str),
    );
    let record = record(path.module.as_str(), &identity, &rust)?;
    Ok(KaniGraphObligationHarness::generated(
        identity, rust, record,
    ))
}
