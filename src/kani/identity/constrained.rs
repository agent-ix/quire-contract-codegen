// SPDX-License-Identifier: AGPL-3.0-or-later
//! Method identities sit below both generation and execution.
use super::{ProofCeilings, SymbolicBounds};
use crate::core::artifact::Artifact;
use qsl_replay::SnapshotValue;
use quire_contract_model::{CheckedNodeId, ContractSourceV1};
use serde::{Serialize, Serializer};
/// The semantic obligation identity, deliberately excluding the selected method.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedObligationIdentity {
    /// Admitted package name and complete dependency/source lock.
    pub package: ContractSourceV1,
    /// Exact safety clause.
    pub clause: CheckedNodeId,
}

/// A complete state slot in canonical declaration order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedFieldIdentity {
    /// Authentic declaring object.
    pub declaring_node: CheckedNodeId,
    /// Declared field name.
    pub name: String,
    /// Complete finite domain, without a constraint-derived narrowing.
    #[serde(serialize_with = "serialize_domain")]
    pub domain: SymbolicBounds,
    /// Whether a bounded ordered collection requires uniqueness.
    pub unique: bool,
    /// Initial value retained using the owning QSL writer.
    #[serde(serialize_with = "serialize_snapshot")]
    pub initial: SnapshotValue,
}

fn serialize_snapshot<S: Serializer>(
    value: &SnapshotValue,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value
        .to_json()
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

fn serialize_limits<S: Serializer>(limits: &[u64; 10], serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeSeq;
    let mut sequence = serializer.serialize_seq(Some(limits.len()))?;
    for limit in limits {
        sequence.serialize_element(&limit.to_string())?;
    }
    sequence.end()
}

struct DomainWire<'a>(&'a SymbolicBounds);
impl Serialize for DomainWire<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let length = match self.0 {
            SymbolicBounds::Boolean => 1,
            SymbolicBounds::Integer { .. } => 3,
            SymbolicBounds::Sequence { .. } => 4,
        };
        let mut sequence = serializer.serialize_seq(Some(length))?;
        match self.0 {
            SymbolicBounds::Boolean => sequence.serialize_element("boolean")?,
            SymbolicBounds::Integer { minimum, maximum } => {
                sequence.serialize_element("integer")?;
                sequence.serialize_element(&minimum.to_string())?;
                sequence.serialize_element(&maximum.to_string())?;
            }
            SymbolicBounds::Sequence {
                minimum_length,
                maximum_length,
                element,
            } => {
                sequence.serialize_element("sequence")?;
                sequence.serialize_element(&minimum_length.to_string())?;
                sequence.serialize_element(&maximum_length.to_string())?;
                sequence.serialize_element(&DomainWire(element))?;
            }
        }
        sequence.end()
    }
}
fn serialize_domain<S: Serializer>(
    domain: &SymbolicBounds,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    DomainWire(domain).serialize(serializer)
}

/// One complete generated binder domain with its admitted collection kind.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedValueDomain {
    /// Finite bounds, retaining exact decimal endpoints in the identity wire form.
    #[serde(serialize_with = "serialize_domain")]
    pub bounds: SymbolicBounds,
    /// Distinguishes sequences from ordered sets instead of discarding uniqueness.
    pub collection_kind: Option<crate::kani::identity::CollectionKindIdentity>,
}

/// An effective operation ABI, independent of callable-body and dispatch reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedOperationIdentity {
    /// Symbolic operation selector in the canonical effective-operation order.
    pub choice: u32,
    /// Actual declaring node, never reconstructed from an implementation body.
    pub declaring_node: CheckedNodeId,
    /// Effective operation name.
    pub name: String,
    /// Selected customer implementation.
    pub subject_path: String,
    /// Parameter names and complete domains in producer order.
    pub parameters: Vec<(String, ConstrainedValueDomain)>,
    /// Complete result domain, absent only for a declared void result.
    pub result: Option<ConstrainedValueDomain>,
    /// Every admitted precondition attached to this effective operation.
    pub preconditions: Vec<CheckedNodeId>,
}

/// Exact method execution identity; the safety obligation stays separate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedMethodIdentity {
    /// Semantic obligation, unchanged when another constraint is selected.
    pub obligation: ConstrainedObligationIdentity,
    /// Separately selected constraint.
    pub constraint: Option<CheckedNodeId>,
    /// Common admitted state context.
    pub context: CheckedNodeId,
    /// Whole finite object, in canonical field declaration order.
    pub fields: Vec<ConstrainedFieldIdentity>,
    /// Complete operation census and implementation binding.
    pub operations: Vec<ConstrainedOperationIdentity>,
    /// Finite transition horizon.
    pub maximum_steps: u32,
    /// Backend ceilings.
    pub ceilings: ProofCeilings,
    /// Exact unwind.
    pub unwind: u32,
    /// Owner evaluator limits in their documented declaration order.
    #[serde(serialize_with = "serialize_limits")]
    pub evaluator_limits: [u64; 10],
    /// Owning oracle decisions, including complete native capacity or general integer policy.
    pub oracles: Vec<(
        CheckedNodeId,
        Option<crate::core::native_integer::NativeIntegerSelection>,
    )>,
    /// Canonical content digests of every complete generated owner oracle, including
    /// general fallback and operation preconditions; declaration IDs alone do not bind bodies.
    pub oracle_contents: Vec<(CheckedNodeId, [u8; 32])>,
    /// Generated module symbol.
    pub module_symbol: String,
    /// Exact backend options.
    pub options: Vec<String>,
    /// Source-bound safety assertion whose success must appear in complete evidence.
    pub safety_assertion_line: u32,
    /// Ordinary non-vacuity cover line, identified separately from the boundary cover.
    pub ordinary_cover_line: u32,
    /// Boundary reachability cover line.
    pub boundary_cover_line: Option<u32>,
}

/// A sealed generated method. Ordinary proof execution cannot consume it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniConstrainedMethodHarness {
    pub(crate) identity: ConstrainedMethodIdentity,
    pub(crate) rust: Artifact,
    pub(crate) record: Artifact,
}
impl KaniConstrainedMethodHarness {
    /// The selected method and complete domains.
    pub fn identity(&self) -> &ConstrainedMethodIdentity {
        &self.identity
    }
    /// Generated method source, retained separately from the identity record.
    pub fn rust(&self) -> &Artifact {
        &self.rust
    }
    /// Serialized identity and source path.
    pub fn record(&self) -> &Artifact {
        &self.record
    }
}
