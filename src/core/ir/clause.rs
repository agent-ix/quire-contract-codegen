//! Admitted checked-package state-clause body access.

use std::collections::{BTreeMap, BTreeSet};

use super::checked_term::{application, reference};
use quire_contract_model::{
    ApplicationOperator, CheckedMemberType, CheckedNodeId, CheckedNodeKind, CheckedPackageV2,
    CheckedSemanticNodeV2, CheckedTerm, LiteralKind,
};

const MAX_CONDITION_TERMS: usize = 256;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ClauseInputSide {
    Current,
    Pre,
    Parameter,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ClauseInput {
    pub(crate) name: String,
    pub(crate) side: ClauseInputSide,
    pub(crate) lower: i128,
    pub(crate) upper: i128,
}

#[derive(Clone, Debug)]
pub(crate) enum ClauseTerm {
    Boolean(bool),
    Integer(String),
    Input(ClauseInput),
    Operation {
        identity: String,
        arguments: Vec<ClauseTerm>,
        result_bound: Option<(i128, i128)>,
    },
}

pub(crate) struct ClauseCondition {
    pub(crate) term: ClauseTerm,
    pub(crate) inputs: Vec<ClauseInput>,
    pub(crate) condition_node_id: Option<CheckedNodeId>,
}

/// First occurrence of each checked graph node, retained for repeated clause lookups.
pub(crate) struct ClauseNodeIndex<'a> {
    nodes: BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>,
}

impl<'a> ClauseNodeIndex<'a> {
    pub(crate) fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Keep the first node for a duplicate identity.
    pub(crate) fn insert(&mut self, node: &'a CheckedSemanticNodeV2) {
        self.nodes.entry(&node.node_id).or_insert(node);
    }

    /// Look up an admitted node without scanning the graph again.
    pub(crate) fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.nodes.get(id).copied()
    }
}

/// One admitted package and its node lookup, shared by clause planning and body reading.
pub(crate) struct ClauseGraphView<'a> {
    package: &'a CheckedPackageV2,
    index: ClauseNodeIndex<'a>,
    kinds: BTreeMap<&'a CheckedNodeId, CheckedNodeKind>,
}

impl<'a> ClauseGraphView<'a> {
    pub(crate) fn new(package: &'a CheckedPackageV2) -> Self {
        let mut index = ClauseNodeIndex::new();
        let mut kinds = BTreeMap::new();
        for (node, kind) in package.graph().nodes.iter().zip(package.node_kinds()) {
            index.insert(node);
            kinds.entry(&node.node_id).or_insert(*kind);
        }
        Self {
            package,
            index,
            kinds,
        }
    }

    pub(crate) fn package(&self) -> &'a CheckedPackageV2 {
        self.package
    }

    pub(crate) fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.index.node(id)
    }

    pub(crate) fn kind(&self, id: &CheckedNodeId) -> Option<CheckedNodeKind> {
        self.kinds.get(id).copied()
    }
}

struct Reader<'a, 'view> {
    package: &'a CheckedPackageV2,
    nodes: &'view ClauseGraphView<'a>,
    self_id: CheckedNodeId,
    allowed_parameters: BTreeSet<CheckedNodeId>,
    inputs: BTreeSet<ClauseInput>,
    visiting: BTreeSet<CheckedNodeId>,
    visits: usize,
}

impl Reader<'_, '_> {
    fn node(&self, id: &CheckedNodeId) -> Option<&CheckedSemanticNodeV2> {
        self.nodes.node(id)
    }

    fn term(&mut self, value: &CheckedTerm, side: ClauseInputSide) -> Option<ClauseTerm> {
        self.visits = self.visits.checked_add(1)?;
        if self.visits > MAX_CONDITION_TERMS {
            return None;
        }
        match value {
            CheckedTerm::Reference { target } => {
                let id = target.clone();
                if !self.visiting.insert(id.clone()) {
                    return None;
                }
                let node = self.node(&id)?;
                if &*node.node_tag == "value" && &*node.semantic_form == "parameter" {
                    let bound = self.bound(&node.semantic_type)?;
                    let declaration = self.package.parameter_declaration(&id).ok()?;
                    if side != ClauseInputSide::Current || !self.allowed_parameters.contains(&id) {
                        return None;
                    }
                    let input = ClauseInput {
                        name: declaration.name.to_string(),
                        side: ClauseInputSide::Parameter,
                        lower: bound.0,
                        upper: bound.1,
                    };
                    self.inputs.insert(input.clone());
                    self.visiting.remove(&id);
                    return Some(ClauseTerm::Input(input));
                }
                let body = self.package.term(&id)?.clone();
                let result_bound = self.bound(&node.semantic_type);
                let result = self.term_with_bound(&body, side, result_bound);
                self.visiting.remove(&id);
                result
            }
            _ => self.term_with_bound(value, side, None),
        }
    }

    fn bound(&self, id: &CheckedNodeId) -> Option<(i128, i128)> {
        let node = self.node(id)?;
        if &*node.node_tag != "bounded_domain" || &*node.semantic_form != "integer_range" {
            return None;
        }
        let CheckedTerm::Aggregate { members } = self.package.term(id)? else {
            return None;
        };
        let number = |name| -> Option<i128> {
            let value = members.iter().find_map(|entry| match entry {
                CheckedTerm::Binding {
                    name: actual,
                    value,
                } if actual.as_ref() == name => Some(value.as_ref()),
                _ => None,
            })?;
            let CheckedTerm::Literal { value, .. } = value else {
                return None;
            };
            value.as_str()?.parse().ok()
        };
        Some((number("min")?, number("max")?))
    }

    fn term_with_bound(
        &mut self,
        value: &CheckedTerm,
        side: ClauseInputSide,
        result_bound: Option<(i128, i128)>,
    ) -> Option<ClauseTerm> {
        match value {
            CheckedTerm::Literal {
                value_kind, value, ..
            } => match value_kind {
                LiteralKind::Boolean => match value.as_str()? {
                    "true" => Some(ClauseTerm::Boolean(true)),
                    "false" => Some(ClauseTerm::Boolean(false)),
                    _ => None,
                },
                LiteralKind::Integer => Some(ClauseTerm::Integer(value.as_str()?.to_owned())),
                _ => None,
            },
            CheckedTerm::Application {
                operation,
                arguments: args,
                ..
            } => {
                let identity = operation.identity()?;
                match identity {
                    "quire.op.state.pre" if args.len() == 1 => {
                        let read = self.term_body(&args[0])?;
                        let CheckedTerm::Application {
                            operation,
                            arguments,
                            ..
                        } = &read
                        else {
                            return None;
                        };
                        if operation.identity()? != "quire.op.record.project" {
                            return None;
                        }
                        let [deref] = arguments.as_slice() else {
                            return None;
                        };
                        self.field(&read, deref, ClauseInputSide::Pre)
                    }
                    "quire.op.record.project" if args.len() == 1 => {
                        self.field(value, &args[0], side)
                    }
                    _ => {
                        let arguments = args
                            .iter()
                            .map(|arg| self.term(arg, side))
                            .collect::<Option<Vec<_>>>()?;
                        Some(ClauseTerm::Operation {
                            identity: identity.to_owned(),
                            arguments,
                            result_bound,
                        })
                    }
                }
            }
            _ => None,
        }
    }

    fn field(
        &mut self,
        project: &CheckedTerm,
        deref: &CheckedTerm,
        side: ClauseInputSide,
    ) -> Option<ClauseTerm> {
        let CheckedTerm::Application { operation, .. } = project else {
            return None;
        };
        let (declaration, name) = operation.projection_field().ok()??;
        if !matches!(side, ClauseInputSide::Current | ClauseInputSide::Pre) {
            return None;
        }
        let deref = self.term_body(deref)?;
        let CheckedTerm::Application { result_type, .. } = &deref else {
            return None;
        };
        if result_type != declaration {
            return None;
        }
        let [subject] = application(&deref, ApplicationOperator::Deref, "quire.op.model.deref")?
        else {
            return None;
        };
        if reference(subject)? != &self.self_id {
            return None;
        }
        let fields = self.package.model_object_fields(declaration).ok()?;
        let field = fields.field(name)?;
        let Some(CheckedMemberType::IntRange { lower, upper }) = field.member_type() else {
            return None;
        };
        let input = ClauseInput {
            name: name.to_owned(),
            side,
            lower: *lower,
            upper: *upper,
        };
        self.inputs.insert(input.clone());
        Some(ClauseTerm::Input(input))
    }

    fn term_body(&self, value: &CheckedTerm) -> Option<CheckedTerm> {
        match reference(value) {
            Some(id) => Some(self.package.term(id)?.clone()),
            None => Some(value.clone()),
        }
    }
}

pub(crate) fn read_clause(
    id: &CheckedNodeId,
    nodes: &ClauseGraphView<'_>,
) -> Option<ClauseCondition> {
    let package = nodes.package();
    let clause = nodes.node(id)?;
    if &*clause.node_tag != "state" || &*clause.semantic_form != "state_clause" {
        return None;
    }
    let [parameters, _anchor, condition] = application(
        package.term(id)?,
        ApplicationOperator::StateClause,
        "quire.op.state.clause",
    )?
    else {
        return None;
    };
    let CheckedTerm::Aggregate { members: declared } = parameters else {
        return None;
    };
    let self_id = declared.first().and_then(reference)?.clone();
    if package.parameter_declaration(&self_id).ok()?.name.as_ref() != "self" {
        return None;
    }
    let allowed_parameters = declared
        .iter()
        .skip(1)
        .map(|term| reference(term).cloned())
        .collect::<Option<_>>()?;
    let mut reader = Reader {
        package,
        nodes,
        self_id,
        allowed_parameters,
        inputs: BTreeSet::new(),
        visiting: BTreeSet::new(),
        visits: 0,
    };
    let term = reader.term(condition, ClauseInputSide::Current)?;
    Some(ClauseCondition {
        term,
        inputs: reader.inputs.into_iter().collect(),
        condition_node_id: reference(condition).cloned(),
    })
}
