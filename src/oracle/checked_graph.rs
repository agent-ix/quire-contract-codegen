//! Borrowed checked terms and keyed nodes shared by oracle consumers.

use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2, CheckedTerm,
    ContractPackageV1, LiteralKind,
};
use quire_exact::Integer;

/// A view of one admitted lowered package, retaining the consumers' key-ordered walks.
/// Keyed queries use the IR owner's retained index rather than rebuilding a node map.
pub(crate) struct CheckedGraph<'a> {
    package: Package<'a>,
    ordered: Vec<&'a CheckedSemanticNodeV2>,
}

enum Package<'a> {
    Lowered(&'a ContractPackageV1),
    Checked(&'a CheckedPackageV2),
    #[cfg(test)]
    Fixture(Vec<(&'a CheckedNodeId, &'a fixture::Node)>),
}

impl<'a> CheckedGraph<'a> {
    /// Borrow the admitted package and order its node references once.
    pub(crate) fn new(package: &'a ContractPackageV1) -> Self {
        let mut ordered: Vec<_> = package.wire().semantic_graph.nodes.iter().collect();
        ordered.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
        Self {
            package: Package::Lowered(package),
            ordered,
        }
    }

    /// Borrow an original admitted source for generators that inspect its full graph.
    /// The source exposes keyed terms but no public keyed-node accessor, so the
    /// same ordered references also support binary node lookup in this case.
    pub(crate) fn from_checked(package: &'a CheckedPackageV2) -> Self {
        Self::from_checked_with_node_visit(package, || {})
    }

    /// Keep a caller's test observer attached to each actual source-node visit.
    pub(crate) fn from_checked_with_node_visit(
        package: &'a CheckedPackageV2,
        mut visit: impl FnMut(),
    ) -> Self {
        let mut ordered: Vec<_> = package.graph().nodes.iter().inspect(|_| visit()).collect();
        ordered.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
        Self {
            package: Package::Checked(package),
            ordered,
        }
    }

    /// The represented node selected by its complete admitted key.
    pub(crate) fn node(&self, key: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        match self.package {
            Package::Lowered(package) => package.node(key),
            Package::Checked(_) => self
                .ordered
                .binary_search_by(|node| node.node_id.cmp(key))
                .ok()
                .and_then(|position| self.ordered.get(position).copied()),
            #[cfg(test)]
            Package::Fixture(ref nodes) => nodes
                .binary_search_by(|(candidate, _)| (*candidate).cmp(key))
                .ok()
                .and_then(|position| nodes.get(position).map(|(_, node)| &node.metadata)),
        }
    }

    /// The IR owner's retained typed body of that same node.
    pub(crate) fn term(&self, key: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        match self.package {
            Package::Lowered(package) => package.term(key),
            Package::Checked(package) => package.term(key),
            #[cfg(test)]
            Package::Fixture(ref nodes) => {
                let position = nodes
                    .binary_search_by(|(candidate, _)| (*candidate).cmp(key))
                    .ok()?;
                let node: &'a fixture::Node = nodes.get(position)?.1;
                node.body.as_ref()
            }
        }
    }

    /// Iterate in the same key order as the consumers' former node maps.
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &'a CheckedSemanticNodeV2> + '_ {
        self.ordered.iter().copied()
    }

    /// The represented node count, including nodes without a body.
    pub(crate) fn len(&self) -> usize {
        self.ordered.len()
    }

    /// The unchanged catalog payload of an admitted application.
    pub(crate) fn operation(&self, key: &CheckedNodeId) -> Option<&'a serde_json::Value> {
        match self.term(key)? {
            CheckedTerm::Application { operation, .. } => Some(operation),
            _ => None,
        }
    }

    /// Arguments in the admitted application's original source order.
    pub(crate) fn arguments(&self, key: &CheckedNodeId) -> Option<&'a [CheckedTerm]> {
        match self.term(key)? {
            CheckedTerm::Application { arguments, .. } => Some(arguments),
            _ => None,
        }
    }

    /// The owner's closed application class, without a wire-string decode.
    pub(crate) fn operator(&self, key: &CheckedNodeId) -> Option<ApplicationOperator> {
        match self.term(key)? {
            CheckedTerm::Application { operator, .. } => Some(*operator),
            _ => None,
        }
    }
}

/// Diagnostic spelling of the owner's closed variant, never a dispatch input.
pub(crate) fn term_name(term: &CheckedTerm) -> &'static str {
    match term {
        CheckedTerm::Literal { .. } => "literal",
        CheckedTerm::Reference { .. } => "reference",
        CheckedTerm::Application { .. } => "application",
        CheckedTerm::Aggregate { .. } => "aggregate",
        CheckedTerm::Binding { .. } => "binding",
        CheckedTerm::DependencyReference { .. } => "dependency_reference",
        CheckedTerm::Frame { .. } => "frame",
        CheckedTerm::AbstractionRelation { .. } => "abstraction_relation",
        CheckedTerm::ExtensionForm { .. } => "extension_form",
    }
}

/// The members of an admitted aggregate, without interpreting its wire representation.
pub(crate) fn aggregate_members<'a>(
    term: impl Into<Option<&'a CheckedTerm>>,
) -> Option<&'a [CheckedTerm]> {
    match term.into()? {
        CheckedTerm::Aggregate { members } => Some(members),
        _ => None,
    }
}

/// Select the complete, unique set of named bound members in the requested order.
pub(crate) fn bound_members<'a, const N: usize>(
    members: &'a [CheckedTerm],
    names: [&str; N],
) -> Option<[&'a CheckedTerm; N]> {
    if members.len() != N {
        return None;
    }
    let mut found = [None; N];
    for member in members {
        let CheckedTerm::Binding { name, value } = member else {
            return None;
        };
        let slot = &mut found[names
            .iter()
            .position(|expected| *expected == name.as_ref())?];
        if slot.replace(value.as_ref()).is_some() {
            return None;
        }
    }
    found
        .into_iter()
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()
}

/// The unchanged string payload of a literal of the selected closed kind.
pub(crate) fn literal(term: &CheckedTerm, kind: LiteralKind) -> Option<&str> {
    match term {
        CheckedTerm::Literal {
            value_kind, value, ..
        } if *value_kind == kind => value.as_str(),
        _ => None,
    }
}

/// The same canonical arbitrary-precision integer accepted by the scalar bound reader.
pub(crate) fn literal_integer(term: &CheckedTerm) -> Option<Integer> {
    let spelling = literal(term, LiteralKind::Integer)?;
    let value: Integer = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

/// A canonical integer literal whose value fits the cardinality counter.
pub(crate) fn literal_count(term: &CheckedTerm) -> Option<u64> {
    let spelling = literal(term, LiteralKind::Integer)?;
    let value: u64 = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

/// Private damaged-graph controls construct typed terms explicitly. They do not
/// decode JSON or manufacture an admitted package, and carry no admission credit.
#[cfg(test)]
pub(crate) mod fixture {
    use super::*;

    #[derive(Clone)]
    pub(crate) struct Node {
        pub(crate) metadata: CheckedSemanticNodeV2,
        pub(crate) body: Option<CheckedTerm>,
    }

    impl Node {
        pub(crate) fn new(metadata: CheckedSemanticNodeV2, body: Option<CheckedTerm>) -> Self {
            let mut node = Self { metadata, body };
            node.sync_body();
            node
        }

        pub(crate) fn from_admitted(
            metadata: &CheckedSemanticNodeV2,
            body: Option<&CheckedTerm>,
        ) -> Self {
            Self {
                metadata: metadata.clone(),
                body: body.cloned(),
            }
        }

        pub(crate) fn sync_body(&mut self) {
            self.metadata.body = self
                .body
                .as_ref()
                .map(|term| serde_json::to_value(term).expect("typed fixture serializes"))
                .unwrap_or(serde_json::Value::Null);
        }
    }

    impl std::ops::Deref for Node {
        type Target = CheckedSemanticNodeV2;

        fn deref(&self) -> &Self::Target {
            &self.metadata
        }
    }

    impl std::ops::DerefMut for Node {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.metadata
        }
    }

    impl<'a> FromIterator<(&'a CheckedNodeId, &'a Node)> for CheckedGraph<'a> {
        fn from_iter<T: IntoIterator<Item = (&'a CheckedNodeId, &'a Node)>>(input: T) -> Self {
            let unique: std::collections::BTreeMap<_, _> = input.into_iter().collect();
            let nodes: Vec<_> = unique.into_iter().collect();
            let ordered = nodes.iter().map(|(_, node)| &node.metadata).collect();
            Self {
                package: Package::Fixture(nodes),
                ordered,
            }
        }
    }
}

#[cfg(test)]
mod tests;
