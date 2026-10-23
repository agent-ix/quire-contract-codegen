//! Borrowed checked terms and keyed nodes shared by oracle consumers.

use quire_contract_model::{
    CheckedNodeId, CheckedSemanticNodeV2, CheckedTerm, ContractPackageV1, LiteralKind,
};
use quire_exact::Integer;

/// A view of one admitted lowered package, retaining the consumers' key-ordered walks.
/// Keyed queries use the IR owner's retained index rather than rebuilding a node map.
pub(crate) struct CheckedGraph<'a> {
    package: &'a ContractPackageV1,
    ordered: Vec<&'a CheckedSemanticNodeV2>,
}

impl<'a> CheckedGraph<'a> {
    /// Borrow the admitted package and order its node references once.
    pub(crate) fn new(package: &'a ContractPackageV1) -> Self {
        let mut ordered: Vec<_> = package.wire().semantic_graph.nodes.iter().collect();
        ordered.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
        Self { package, ordered }
    }

    /// The represented node selected by its complete admitted key.
    pub(crate) fn node(&self, key: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.package.node(key)
    }

    /// The IR owner's retained typed body of that same node.
    pub(crate) fn term(&self, key: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        self.package.term(key)
    }

    /// Iterate in the same key order as the consumers' former node maps.
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &'a CheckedSemanticNodeV2> + '_ {
        self.ordered.iter().copied()
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
