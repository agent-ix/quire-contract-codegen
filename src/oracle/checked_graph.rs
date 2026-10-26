//! Borrowed checked terms and keyed nodes shared by oracle consumers.

use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2, CheckedTerm,
    ContractPackageV1,
};
use serde_json::Value;

pub(crate) trait GraphBody: Sized {
    type Owner;

    fn nodes(owner: &Self::Owner) -> &[CheckedSemanticNodeV2<Self>];
    fn node<'a>(
        owner: &'a Self::Owner,
        ordered: &[&'a CheckedSemanticNodeV2<Self>],
        key: &CheckedNodeId,
    ) -> Option<&'a CheckedSemanticNodeV2<Self>>;
    fn term<'a>(owner: &'a Self::Owner, key: &CheckedNodeId) -> Option<&'a CheckedTerm>;
}

impl GraphBody for Value {
    type Owner = ContractPackageV1;

    fn nodes(owner: &Self::Owner) -> &[CheckedSemanticNodeV2<Self>] {
        &owner.wire().semantic_graph.nodes
    }

    fn node<'a>(
        owner: &'a Self::Owner,
        _: &[&'a CheckedSemanticNodeV2<Self>],
        key: &CheckedNodeId,
    ) -> Option<&'a CheckedSemanticNodeV2<Self>> {
        owner.node(key)
    }

    fn term<'a>(owner: &'a Self::Owner, key: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        owner.term(key)
    }
}

impl GraphBody for CheckedTerm {
    type Owner = CheckedPackageV2;

    fn nodes(owner: &Self::Owner) -> &[CheckedSemanticNodeV2<Self>] {
        &owner.graph().nodes
    }

    fn node<'a>(
        _: &'a Self::Owner,
        ordered: &[&'a CheckedSemanticNodeV2<Self>],
        key: &CheckedNodeId,
    ) -> Option<&'a CheckedSemanticNodeV2<Self>> {
        ordered
            .binary_search_by(|node| node.node_id.cmp(key))
            .ok()
            .and_then(|position| ordered.get(position).copied())
    }

    fn term<'a>(owner: &'a Self::Owner, key: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        owner.term(key)
    }
}

/// A view of one admitted lowered package, retaining the consumers' key-ordered walks.
/// Keyed queries use the IR owner's retained index rather than rebuilding a node map.
pub(crate) struct CheckedGraph<'a, Body: GraphBody = Value> {
    package: Package<'a, Body>,
    ordered: Vec<&'a CheckedSemanticNodeV2<Body>>,
}

enum Package<'a, Body: GraphBody> {
    Owner(&'a Body::Owner),
    #[cfg(test)]
    Fixture(Vec<(&'a CheckedNodeId, &'a fixture::Node<Body>)>),
}

impl<'a> CheckedGraph<'a, Value> {
    /// Borrow the admitted package and order its node references once.
    pub(crate) fn new(package: &'a ContractPackageV1) -> Self {
        let mut ordered: Vec<_> = Value::nodes(package).iter().collect();
        ordered.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
        Self {
            package: Package::Owner(package),
            ordered,
        }
    }

    /// Borrow an original admitted source for generators that inspect its full graph.
    /// The source exposes keyed terms but no public keyed-node accessor, so the
    /// same ordered references also support binary node lookup in this case.
    pub(crate) fn from_checked(package: &'a CheckedPackageV2) -> CheckedGraph<'a, CheckedTerm> {
        Self::from_checked_with_node_visit(package, || {})
    }

    /// Keep a caller's test observer attached to each actual source-node visit.
    pub(crate) fn from_checked_with_node_visit(
        package: &'a CheckedPackageV2,
        mut visit: impl FnMut(),
    ) -> CheckedGraph<'a, CheckedTerm> {
        let mut ordered: Vec<_> = CheckedTerm::nodes(package)
            .iter()
            .inspect(|_| visit())
            .collect();
        ordered.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
        CheckedGraph {
            package: Package::Owner(package),
            ordered,
        }
    }
}

impl<'a, Body: GraphBody> CheckedGraph<'a, Body> {
    /// The represented node selected by its complete admitted key.
    pub(crate) fn node(&self, key: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2<Body>> {
        match self.package {
            Package::Owner(package) => Body::node(package, &self.ordered, key),
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
            Package::Owner(package) => Body::term(package, key),
            #[cfg(test)]
            Package::Fixture(ref nodes) => {
                let position = nodes
                    .binary_search_by(|(candidate, _)| (*candidate).cmp(key))
                    .ok()?;
                let node: &'a fixture::Node<Body> = nodes.get(position)?.1;
                node.body.as_ref()
            }
        }
    }

    /// Iterate in the same key order as the consumers' former node maps.
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &'a CheckedSemanticNodeV2<Body>> + '_ {
        self.ordered.iter().copied()
    }

    /// The represented node count, including nodes without a body.
    pub(crate) fn len(&self) -> usize {
        self.ordered.len()
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

/// Private damaged-graph controls construct typed terms explicitly. They do not
/// decode JSON or manufacture an admitted package, and carry no admission credit.
#[cfg(test)]
pub(crate) mod fixture {
    use super::*;

    #[derive(Clone)]
    pub(crate) struct Node<Body: GraphBody = CheckedTerm> {
        pub(crate) metadata: CheckedSemanticNodeV2<Body>,
        pub(crate) body: Option<CheckedTerm>,
    }

    impl Node<CheckedTerm> {
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
            if let Some(body) = &self.body {
                self.metadata.body = body.clone();
            }
        }
    }

    impl<Body: GraphBody> std::ops::Deref for Node<Body> {
        type Target = CheckedSemanticNodeV2<Body>;

        fn deref(&self) -> &Self::Target {
            &self.metadata
        }
    }

    impl<Body: GraphBody> std::ops::DerefMut for Node<Body> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.metadata
        }
    }

    impl<'a> FromIterator<(&'a CheckedNodeId, &'a Node)> for CheckedGraph<'a, CheckedTerm> {
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
