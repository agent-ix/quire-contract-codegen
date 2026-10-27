//! Shared integration-test fixtures.

/// The byte-ceiling fixtures' measurement of a lowered package's length.
pub mod byte_ceiling;

/// The panic-token scan shared by the generator-source and emitted-source scans.
pub mod panic_scan;

use quire_contract_codegen::ProofCeilings;

/// Shared explicit proof-resource fixture budgets.
pub mod proof_ceilings;
