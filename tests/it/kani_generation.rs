//! Surviving admitted V2 generation controls in the real Kani lane.

use ix_trace_rs::trace;

pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    crate::kani_obligations::guard_sources()
}

/// The lane filter selects a production V2 scalar harness with one final cover.
#[trace("NFR-006-AC-4", "FR-015-AC-58", "TC-045")]
#[test]
fn kani_generation_selects_an_admitted_v2_scalar_harness() {
    let sources = guard_sources();
    assert_eq!(sources.len(), 1);
    let (family, source) = &sources[0];
    assert_eq!(*family, "scalar");
    assert_eq!(source.matches("#[kani::proof]").count(), 1);
    assert_eq!(source.matches("kani::cover!(").count(), 1);
}

/// The installed backend verifies a routed V2 scalar obligation and refuses a missing harness.
#[trace("NFR-006-AC-4", "FR-017-AC-6", "FR-017-AC-7", "FR-017-AC-11", "TC-045")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn kani_generation_real_v2_scalar_verifies() {
    crate::kani_obligations::verify_routed_scalar_harness_real_kani();
}
