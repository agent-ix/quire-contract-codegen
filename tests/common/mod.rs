//! Shared integration-test fixtures.

/// The byte-ceiling fixtures' measurement of a lowered package's length.
pub mod byte_ceiling;

/// The panic-token scan shared by the generator-source and emitted-source scans.
pub mod panic_scan;

/// A matching V2 clause claim for tests that do not exercise FR-015-AC-48.
pub mod v2_claim;

use quire_contract_codegen::ProofCeilings;

/// Shared explicit proof-resource fixture budgets.
pub mod proof_ceilings;

/// The driver's projection of QSL requirement records for fixtures that select an `OwnResult`.
pub mod requirements;
