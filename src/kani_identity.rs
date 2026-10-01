//! The harness and identity record types of a generated Kani obligation (AD-004 step 2b).
//!
//! This module becomes `kani/identity.rs`. The generators build these records and the runner,
//! the witness join and the replay read them, so they live below both: a harness is run from its
//! identity and its source text, and no reader imports a generator.

use quire_contract_model::{CheckedNodeId, ClauseRef, DependencyIdentity, SourceSpan};
use serde::Serialize;

use crate::{
    core::artifact::Artifact,
    core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol},
    kani::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType, KaniSolver},
};

/// The contract role of one obligation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationKind {
    /// Lowered to a `kani::proof` that the precondition is total and satisfiable in bounds.
    Precondition,
    /// Lowered to a `kani::ensures` checked by `kani::proof_for_contract`.
    Postcondition,
    /// Lowered to a `kani::requires`/`kani::ensures` preservation contract.
    Invariant,
    /// A frame's effects; generated from its operation's `state_clause` by
    /// [`crate::generate_state_frame_obligations`], never by the clause renderer.
    Frame,
}

/// One primitive argument or result position of a harness's subject ABI.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObligationBinding {
    /// Generated identifier.
    pub identifier: String,
    /// Argument or result.
    pub role: KaniBindingRole,
    /// Rust primitive.
    pub primitive_type: KaniPrimitiveType,
    /// IR integer bounds; the only source of a symbolic assumption.
    pub integer_bounds: Option<KaniIntegerBounds>,
    /// Every IR dependency bound to this position.
    pub dependencies: Vec<DependencyIdentity>,
}

/// One clause oracle embedded in a harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedOracle {
    /// The clause.
    pub clause: ClauseRef,
    /// Its contract role.
    pub kind: ObligationKind,
    /// Oracle function symbol.
    pub symbol: String,
}

/// What a generated harness proves and how it is run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniObligationIdentity {
    /// Contract role.
    pub kind: ObligationKind,
    /// The obligation's clause.
    pub clause: ClauseRef,
    /// Its QSL source span.
    pub source_span: SourceSpan,
    /// The obligation's oracle followed by every assumed precondition's oracle.
    pub oracles: Vec<EmbeddedOracle>,
    /// Harness module symbol.
    pub module_symbol: ModuleSymbol,
    /// Proof function symbol.
    pub harness_symbol: HarnessSymbol,
    /// Contract function symbol, for postcondition and invariant harnesses.
    pub contract_symbol: Option<String>,
    /// Customer subject, for postcondition and invariant harnesses.
    pub subject_path: Option<String>,
    /// Symbolic arguments, ascending by identifier.
    pub arguments: Vec<ObligationBinding>,
    /// Subject results, ascending by identifier.
    pub results: Vec<ObligationBinding>,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
}

/// One generated harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniObligationHarness {
    /// Identity.
    pub identity: KaniObligationIdentity,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of the identity and the Rust source path: the persisted obligation schema a
    /// witness is decoded against.
    pub record: Artifact,
}

/// One symbolic `i64` argument of a rendered scalar harness, bounded by the IR domain the
/// lowered scalar claim carries.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationArgument {
    /// Generated identifier.
    pub identifier: String,
    /// Inclusive checked minimum.
    pub minimum: i64,
    /// Inclusive checked maximum.
    pub maximum: i64,
}

/// What a V2 scalar obligation harness proves and how it is run. Parallel to [`KaniObligationIdentity`] but for an IR-confirmed exact-scalar claim,
/// which has no QSL clause, declaration or subject signature -- a node id and its confirmed
/// operation identity stand in their place.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationIdentity {
    /// The claimed node.
    pub node_id: CheckedNodeId,
    /// The node's own catalogued operation identity, IR-confirmed at package admission.
    pub operation_identity: String,
    /// The embedded oracle's function symbol.
    pub oracle_symbol: String,
    /// Harness module symbol.
    pub module_symbol: ModuleSymbol,
    /// Proof function symbol.
    pub harness_symbol: HarnessSymbol,
    /// Symbolic arguments, in call order.
    pub arguments: Vec<ScalarObligationArgument>,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
}

/// One generated V2 exact-scalar obligation harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniScalarObligationHarness {
    /// Identity.
    pub identity: ScalarObligationIdentity,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of the identity and the Rust source path: the persisted obligation schema a
    /// witness is decoded against.
    pub record: Artifact,
}

impl KaniObligationIdentity {
    /// The `module::harness` path of this harness.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }
}

impl ScalarObligationIdentity {
    /// The `module::harness` path of this harness.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }
}

/// One generated harness with the identity it proves and its persisted record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateFrameHarness {
    /// Everything the proof is about.
    pub identity: StateFrameIdentity,
    /// The harness source, `src/generated/<module>.rs`.
    pub rust: Artifact,
    /// The persisted identity record, `kani-obligations/<module>.json`.
    pub record: Artifact,
}

/// An integer comparison operator of the closed `quire.op.integer` ordering family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComparison {
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,
    /// `==`.
    Eq,
    /// `!=`.
    Ne,
}

/// The property one harness proves.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum StateFrameProperty {
    /// `left <op> right` between the named observations of one field.
    Postcondition {
        /// The one integer field the condition reads.
        field: String,
        /// The comparison.
        comparison: StateComparison,
        /// Whether the left operand is the field's pre-state value (else its post-state value).
        left_is_pre: bool,
    },
    /// Every `checked` field is unchanged; the `granted` fields may change.
    Frame {
        /// The frame's `modifies` field entries: the allowed effects.
        granted: Vec<String>,
        /// Every other state field: the forbidden effects, each asserted unchanged.
        checked: Vec<String>,
    },
}

/// The inclusive integer range the IR carries for one state field. Every harness assumes it of
/// the symbolic pre-state, so a counterexample is a state the model admits.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateFieldDomain {
    /// The state field.
    pub field: String,
    /// Inclusive lower bound.
    pub minimum: i64,
    /// Inclusive upper bound.
    pub maximum: i64,
}

/// The operation a harness is scoped to. Two harnesses with different scopes are never the same
/// proof.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateFrameScope {
    /// The operation's name, from its anchor.
    pub operation: String,
    /// The framed object type node, the anchor's `context`.
    pub object: CheckedNodeId,
    /// The operation's `state`/`operation_anchor` node.
    pub anchor: CheckedNodeId,
    /// The operation's `state`/`frame` node.
    pub frame: CheckedNodeId,
}

/// Everything a generated harness proves, persisted with it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateFrameIdentity {
    /// The `state_clause` node both obligations of a request come from.
    pub clause: CheckedNodeId,
    /// The operation scope.
    pub scope: StateFrameScope,
    /// The property proved.
    pub property: StateFrameProperty,
    /// The IR range assumed of each state field that has one, in `state_fields` order.
    pub domains: Vec<StateFieldDomain>,
    /// Rust path of the state struct.
    pub state_path: String,
    /// Rust path of the operation subject.
    pub subject_path: String,
    /// The generated module holding the harness.
    pub module_symbol: ModuleSymbol,
    /// The `kani::proof` function.
    pub harness_symbol: HarnessSymbol,
    /// The solver, always `cadical`.
    pub solver: KaniSolver,
    /// The unwind bound.
    pub unwind: u32,
    /// The exact Kani option vector.
    pub options: Vec<String>,
}
