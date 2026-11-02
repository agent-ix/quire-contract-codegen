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
    /// Catalog operation identity read from the admitted application itself.
    pub operation_identity: String,
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
    /// The original application has no matching authenticated catalog operation.
    Operation,
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
        serde_json::json!({"node": self.node, "operation": self.operation_identity, "occurrence": self.occurrence, "kind": self.kind.as_str(), "operands": operands, "resultLower": self.result_range.lower.to_string(), "resultUpper": self.result_range.upper.to_string()})
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
        let operation_identity = match package.term(node) {
            Some(quire_contract_model::CheckedTerm::Application { operation, .. }) => operation
                .identity()
                .ok_or(ScalarAuthorityError::Operation)?
                .to_owned(),
            _ => return Err(ScalarAuthorityError::Operation),
        };
        let occurrence = unique_expression_occurrence(&node_record.occurrences)?;
        let operands = package
            .scalar_application_operands(node, &occurrence)
            .map_err(ScalarAuthorityError::Operands)?;
        Ok(Self {
            node: node.clone(),
            operation_identity,
            occurrence,
            kind: ScalarReplayKind::OperatorParity,
            operands,
            result_range,
        })
    }
}

fn unique_expression_occurrence(
    occurrences: &[CheckedOccurrence],
) -> Result<CheckedOccurrence, ScalarAuthorityError> {
    let mut expressions = occurrences
        .iter()
        .filter(|occurrence| occurrence.role == CheckedOccurrenceRole::Expression);
    let selected = expressions.next().ok_or(ScalarAuthorityError::Occurrence)?;
    if expressions.next().is_some() {
        return Err(ScalarAuthorityError::Occurrence);
    }
    Ok(selected.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    #[trace("TC-047", "FR-032-AC-18")]
    #[test]
    fn absent_or_ambiguous_scalar_authority_never_invents_an_occurrence() {
        let occurrence = CheckedOccurrence {
            role: CheckedOccurrenceRole::Expression,
            ordinal: 17,
        };
        assert_eq!(
            unique_expression_occurrence(&[]),
            Err(ScalarAuthorityError::Occurrence)
        );
        assert_eq!(
            unique_expression_occurrence(&[
                occurrence.clone(),
                CheckedOccurrence {
                    ordinal: 23,
                    ..occurrence.clone()
                }
            ]),
            Err(ScalarAuthorityError::Occurrence)
        );
        assert_eq!(
            unique_expression_occurrence(std::slice::from_ref(&occurrence)).unwrap(),
            occurrence
        );
        let (fixture, _) =
            crate::kani::test_support::scalar_source_fixture(std::time::Duration::from_secs(1));
        assert_eq!(
            fixture.scalar_replay_body(),
            Err(ScalarAuthorityError::Unavailable)
        );
    }
}
