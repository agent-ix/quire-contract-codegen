//! Original admitted positional scalar body retained by the scalar renderer (FR-032).

use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageV2,
    CheckedScalarOperand, CheckedScalarOperandError,
};

/// The CG-owned scalar-family allocation; no clause role or caller string substitutes for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarReplayKind {
    /// Bounded integer arithmetic lowering parity (FR-032).
    OperatorParity,
}

impl ScalarReplayKind {
    /// The exact FR-032 wire allocation carried opaquely by QSL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperatorParity => "operator_parity",
        }
    }
}

/// The complete original scalar body. This is a claim, not an authentication capability:
/// the replay builder compares it against the renderer's independently retained copy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalarReplayBody {
    /// Original admitted application identity, including package, path and producer ordinal.
    pub node: CheckedNodeId,
    /// Authentic expression occurrence from the admitted original application.
    pub occurrence: CheckedOccurrence,
    /// CG's typed family allocation.
    pub kind: ScalarReplayKind,
    /// One admitted operand per argument position, including repeated and inline operands.
    pub operands: Vec<CheckedScalarOperand>,
    /// The exact result interval supplied by the integer renderer.
    pub result_range: quire_contract_model::CheckedScalarOperandRange,
}

/// Why generation cannot supply replay authority. Generation itself remains available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScalarAuthorityError {
    /// No replay authority was retained by this generation path.
    Unavailable,
    /// No unique expression occurrence exists; no ordinal is invented.
    Occurrence,
    /// The reader cannot supply complete positional integer operand provenance.
    Operands(CheckedScalarOperandError),
    /// The scalar renderer's actual drawn ranges differ from the admitted projection.
    Ranges,
    /// The public harness fields changed after generation.
    GeneratedArtifactChanged,
    /// The original proving record cannot retain the complete typed body.
    Record,
}

impl ScalarReplayBody {
    pub(crate) fn record(&self) -> serde_json::Value {
        use quire_contract_model::CheckedScalarOperandChild;
        let operands = self.operands.iter().map(|operand| {
            let child = match &operand.child {
                CheckedScalarOperandChild::GraphChild(node) => serde_json::json!({"graphChild": node}),
                CheckedScalarOperandChild::InlineLiteral { application, occurrence, ordinal } => serde_json::json!({"inlineLiteral": {"application": application, "occurrence": occurrence, "ordinal": ordinal}}),
            };
            serde_json::json!({"ordinal": operand.ordinal, "child": child, "lower": operand.range.lower.to_string(), "upper": operand.range.upper.to_string()})
        }).collect::<Vec<_>>();
        serde_json::json!({"node": self.node, "occurrence": self.occurrence, "kind": self.kind.as_str(), "operands": operands, "resultLower": self.result_range.lower.to_string(), "resultUpper": self.result_range.upper.to_string()})
    }

    pub(crate) fn admitted(
        package: &CheckedPackageV2,
        node: &CheckedNodeId,
        result_range: quire_contract_model::CheckedScalarOperandRange,
    ) -> Result<Self, ScalarAuthorityError> {
        let node_record = package
            .graph()
            .nodes
            .iter()
            .find(|entry| &entry.node_id == node)
            .ok_or(ScalarAuthorityError::Occurrence)?;
        let mut occurrences = node_record
            .occurrences
            .iter()
            .filter(|occurrence| occurrence.role == CheckedOccurrenceRole::Expression);
        let occurrence = occurrences
            .next()
            .ok_or(ScalarAuthorityError::Occurrence)?
            .clone();
        if occurrences.next().is_some() {
            return Err(ScalarAuthorityError::Occurrence);
        }
        let operands = package
            .scalar_application_operands(node, &occurrence)
            .map_err(ScalarAuthorityError::Operands)?;
        Ok(Self {
            node: node.clone(),
            occurrence,
            kind: ScalarReplayKind::OperatorParity,
            operands,
            result_range,
        })
    }
}
