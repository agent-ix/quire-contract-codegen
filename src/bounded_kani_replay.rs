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
