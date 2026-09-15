//! Codegen-side concrete counterexample replay boundary.

use quire_contract_ir::kani::{
    replay_counterexample, CounterexamplePacket, FiniteInput, KaniOutcome, ReplayAgreement,
};

/// Replays one retained codegen counterexample through an independently supplied native evaluator.
///
/// Packet validation, finite-population validation, and the requirement for a native false result
/// remain Contract IR-owned. Codegen cannot repair a disagreement into a proof.
pub fn replay_codegen_counterexample(
    packet: CounterexamplePacket,
    execute_native: impl FnOnce(&FiniteInput) -> KaniOutcome,
) -> Result<ReplayAgreement, KaniOutcome> {
    replay_counterexample(packet, execute_native)
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CounterexamplePacket, FiniteInput, KaniOutcome, KaniOutcomeKind, PopulationCompleteness,
        ProfileSelection, ResourceBounds,
    };

    use super::replay_codegen_counterexample;

    fn packet() -> CounterexamplePacket {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            executable_digest: "exe".to_owned(),
            options_digest: "opts".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        CounterexamplePacket {
            profile_revision: selection.revision.clone(),
            input: FiniteInput {
                model_id: "model".to_owned(),
                source_id: "source".to_owned(),
                profile: selection,
                completeness: PopulationCompleteness::Complete,
                bounds: ResourceBounds {
                    max_objects: 1,
                    max_references: 0,
                    max_input_bytes: 1,
                },
                input_bytes: 0,
                objects: vec![],
                references: vec![],
            },
            witness: "counterexample".to_owned(),
        }
    }

    /// Trace: FR-007-AC-4, TC-023.
    #[test]
    fn tc_023_replay_preserves_false_and_rejects_non_counterexample_results() {
        let agreement = replay_codegen_counterexample(packet(), |input| {
            KaniOutcome::counterexample(input.source_id.clone(), input.profile.revision.clone())
        })
        .expect("native false agrees with retained packet");
        assert_eq!(agreement.native.boolean_claim(), Some(false));
        let disagreement = replay_codegen_counterexample(packet(), |input| {
            KaniOutcome::proved(input.source_id.clone(), input.profile.revision.clone())
        })
        .expect_err("native proof cannot repair a retained counterexample");
        assert_eq!(disagreement.kind, KaniOutcomeKind::Inconclusive);
        assert_eq!(disagreement.boolean_claim(), None);
    }
}
