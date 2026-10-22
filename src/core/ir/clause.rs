//! Admitted checked-package state-clause body access.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    CheckedMemberType, CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2,
};
use serde_json::Value;

// The clause controls load the existing state-frame fixture here too so they can exercise the
// crate-private graph view; its other fixture APIs are unused in this module.
#[cfg(test)]
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../../tests/state_frame_support/model.rs"]
mod model;
// The QSL fixture is also loaded by state-frame tests; these controls need its admitted package.
#[cfg(test)]
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../../tests/state_frame_support/native_twin.rs"]
mod native_twin;

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

fn node_id(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.clone()).ok()
}

fn reference(value: &Value) -> Option<CheckedNodeId> {
    (value.get("term")?.as_str()? == "reference")
        .then(|| node_id(value.get("target")?))
        .flatten()
}

fn arguments<'a>(value: &'a Value, operator: &str, identity: &str) -> Option<&'a Vec<Value>> {
    (value.get("term")?.as_str()? == "application"
        && value.get("operator")?.as_str()? == operator
        && value.get("operation")?.get("identity")?.as_str()? == identity)
        .then(|| value.get("arguments")?.as_array())
        .flatten()
}

struct ClauseNodeIndex<'a> {
    nodes: BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>,
}

impl<'a> ClauseNodeIndex<'a> {
    fn from_nodes(graph_nodes: impl IntoIterator<Item = &'a CheckedSemanticNodeV2>) -> Self {
        let mut nodes = BTreeMap::new();
        for node in graph_nodes {
            nodes.entry(&node.node_id).or_insert(node);
        }
        Self { nodes }
    }

    fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.nodes.get(id).copied()
    }
}

/// One admitted package and its node lookup, shared by clause planning and body reading.
pub(crate) struct ClauseGraphView<'a> {
    package: &'a CheckedPackageV2,
    index: ClauseNodeIndex<'a>,
}

impl<'a> ClauseGraphView<'a> {
    pub(crate) fn new(package: &'a CheckedPackageV2) -> Self {
        Self {
            package,
            index: ClauseNodeIndex::from_nodes(package.graph().nodes.iter()),
        }
    }

    pub(crate) fn package(&self) -> &'a CheckedPackageV2 {
        self.package
    }

    pub(crate) fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.index.node(id)
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

    fn term(&mut self, value: &Value, side: ClauseInputSide) -> Option<ClauseTerm> {
        self.visits = self.visits.checked_add(1)?;
        if self.visits > MAX_CONDITION_TERMS {
            return None;
        }
        match value.get("term")?.as_str()? {
            "reference" => {
                let id = reference(value)?;
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
                let body = node.body.clone();
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
        let members = node.body.get("members")?.as_array()?;
        let number = |name| -> Option<i128> {
            members
                .iter()
                .find(|entry| entry.get("name").and_then(Value::as_str) == Some(name))?
                .get("value")?
                .get("value")?
                .as_str()?
                .parse()
                .ok()
        };
        Some((number("min")?, number("max")?))
    }

    fn term_with_bound(
        &mut self,
        value: &Value,
        side: ClauseInputSide,
        result_bound: Option<(i128, i128)>,
    ) -> Option<ClauseTerm> {
        match value.get("term")?.as_str()? {
            "literal" => match value.get("value_kind")?.as_str()? {
                "boolean" => match value.get("value")?.as_str()? {
                    "true" => Some(ClauseTerm::Boolean(true)),
                    "false" => Some(ClauseTerm::Boolean(false)),
                    _ => None,
                },
                "integer" => Some(ClauseTerm::Integer(
                    value.get("value")?.as_str()?.to_owned(),
                )),
                _ => None,
            },
            "application" => {
                let identity = value.get("operation")?.get("identity")?.as_str()?;
                let args = value.get("arguments")?.as_array()?;
                match identity {
                    "quire.op.state.pre" if args.len() == 1 => {
                        let read = self.term_body(&args[0])?;
                        if read.get("term")?.as_str()? != "application"
                            || read.get("operation")?.get("identity")?.as_str()?
                                != "quire.op.record.project"
                        {
                            return None;
                        }
                        let [deref] = read.get("arguments")?.as_array()?.as_slice() else {
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
        project: &Value,
        deref: &Value,
        side: ClauseInputSide,
    ) -> Option<ClauseTerm> {
        let declaration = node_id(
            project
                .get("operation")?
                .get("member")?
                .get("declaration")?,
        )?;
        let name = project
            .get("operation")?
            .get("member")?
            .get("name")?
            .as_str()?;
        if project
            .get("operation")?
            .get("member")?
            .get("kind")?
            .as_str()?
            != "field"
            || !matches!(side, ClauseInputSide::Current | ClauseInputSide::Pre)
        {
            return None;
        }
        let deref = self.term_body(deref)?;
        if node_id(deref.get("result_type")?)? != declaration {
            return None;
        }
        let [subject] = arguments(&deref, "deref", "quire.op.model.deref")?.as_slice() else {
            return None;
        };
        if reference(subject)? != self.self_id {
            return None;
        }
        let fields = self.package.model_object_fields(&declaration).ok()?;
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

    fn term_body(&self, value: &Value) -> Option<Value> {
        match reference(value) {
            Some(id) => Some(self.node(&id)?.body.clone()),
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
    let [parameters, _anchor, condition] =
        arguments(&clause.body, "state_clause", "quire.op.state.clause")?.as_slice()
    else {
        return None;
    };
    let declared = parameters.get("members")?.as_array()?;
    let self_id = declared.first().and_then(reference)?;
    if package.parameter_declaration(&self_id).ok()?.name.as_ref() != "self" {
        return None;
    }
    let allowed_parameters = declared
        .iter()
        .skip(1)
        .map(reference)
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
        condition_node_id: reference(condition),
    })
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use ix_trace_rs::trace;
    use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};

    use super::{native_twin::Twin, ClauseGraphView, ClauseNodeIndex};
    use crate::oracle::clause::{
        derive_v2_clause_descriptor, derive_v2_clause_descriptor_with_view,
        generate_v2_clause_oracle, generate_v2_clause_oracle_with_view, V2ClauseRefusal,
    };

    fn id(number: u8) -> CheckedNodeId {
        CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: format!("{number:064x}").into(),
        }
    }

    fn node(number: u8, label: &str) -> CheckedSemanticNodeV2 {
        CheckedSemanticNodeV2 {
            node_id: id(number),
            schema_version: "quire.checked-semantic-graph/v2".into(),
            node_tag: "expression".into(),
            semantic_form: "literal".into(),
            semantic_type: id(0),
            dependencies: Vec::new(),
            occurrences: Vec::new(),
            recursion_group: None,
            nominal_identity_preimage: None,
            declaration: None,
            owner: None,
            body: serde_json::json!({"label": label}),
        }
    }

    #[trace("FR-014-AC-38", "TC-024")]
    #[test]
    fn clause_node_index_preserves_first_node_and_reuses_one_walk() {
        let graph = [node(1, "first"), node(2, "other"), node(1, "later")];
        let visits = Cell::new(0);
        let index = ClauseNodeIndex::from_nodes(graph.iter().inspect(|_| {
            visits.set(visits.get() + 1);
        }));
        for _ in 0..3 {
            assert_eq!(index.node(&id(1)).unwrap().body["label"], "first");
            assert_eq!(index.node(&id(2)).unwrap().body["label"], "other");
            assert!(index.node(&id(3)).is_none());
        }
        assert_eq!(visits.get(), graph.len());

        let (package, clause) = Twin::new().emitted_package("BalanceNeverDrops");
        let view = ClauseGraphView::new(&package);
        let generated = generate_v2_clause_oracle_with_view(&clause, &view).unwrap();
        assert_eq!(generated.descriptor.node_id, clause);
        assert_eq!(
            generated,
            generate_v2_clause_oracle(&package, &clause).unwrap()
        );
    }

    #[trace("FR-014-AC-38", "TC-024")]
    #[test]
    fn clause_planning_and_reading_share_one_graph_walk() {
        // A checked graph is a Vec, so the number of whole-graph accesses is not
        // observable through the public package API. Guard the two call sites.
        let reader = include_str!("clause.rs");
        let production_reader = reader.split("\n#[cfg(test)]\nmod tests").next().unwrap();
        let source = format!(
            "{}{}",
            include_str!("../../oracle/clause.rs"),
            production_reader
        );
        assert_eq!(
            source.matches(concat!(".gra", "ph()")).count(),
            1,
            "clause planning and reading must build one reusable graph view"
        );

        let (package, clause) = Twin::new().emitted_package("BalanceNeverDrops");
        let view = ClauseGraphView::new(&package);
        let descriptor = derive_v2_clause_descriptor_with_view(&clause, &view).unwrap();
        assert_eq!(descriptor.node_id, clause);
        assert_eq!(
            descriptor,
            derive_v2_clause_descriptor(&package, &clause).unwrap()
        );
        let non_clause = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_tag.as_ref() == "expression")
            .unwrap();
        assert_eq!(
            generate_v2_clause_oracle_with_view(&non_clause.node_id, &view),
            Err(V2ClauseRefusal::NotClause)
        );
    }
}
