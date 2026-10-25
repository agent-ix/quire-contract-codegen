//! Manual decoding of the real helper-mutant Kani playback after qualification artifacts exist.

use ix_trace_rs::trace;

/// Decode the actual helper-mutant playback against its healthy sealed source and record.
#[trace("FR-015-AC-126", "FR-016-AC-1", "TC-026")]
#[test]
#[ignore = "run after the coordinated real Kani helper-mutant qualification produces its artifacts"]
fn native_v2_helper_mutant_playback_decodes_typed_original_witness() {
    super::kani_obligations_state_frame::decode_native_v2_helper_mutant_playback();
}
