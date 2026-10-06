//! Resource ceilings for test fixtures; never production defaults.

use super::ProofCeilings;
use std::{num::NonZeroU64, time::Duration};

/// The explicit successful-run fixture budget.
pub fn proof_ceilings() -> ProofCeilings {
    proof_ceilings_with_wall_clock(Duration::from_secs(600))
}

/// The same fixture memory budget with the test's explicit wall allowance.
pub fn proof_ceilings_with_wall_clock(wall_clock: Duration) -> ProofCeilings {
    ProofCeilings {
        memory_bytes: NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
        wall_clock,
    }
}
