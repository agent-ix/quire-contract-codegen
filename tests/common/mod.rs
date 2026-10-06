//! Shared integration-test fixtures.

/// The byte-ceiling fixtures' measurement of a lowered package's length.
pub mod byte_ceiling;

/// The panic-token scan shared by the generator-source and emitted-source scans.
pub mod panic_scan;

/// The `withdraw` obligation fixture, shared with whatever test negotiates it as a real bound
/// package.
pub mod withdraw_fixture;

use quire_contract_codegen::ProofCeilings;

/// Shared explicit proof-resource fixture budgets.
pub mod proof_ceilings;

/// Actual packaged helper built by Cargo alongside the integration test's normal library.
pub fn guardian_path() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_BIN_EXE_quire-kani-guardian"))
}

/// Capture original stdin once, before any integration request creates guardian controls.
pub fn original_stdin() -> &'static quire_contract_codegen::OriginalStdin {
    use std::{os::fd::AsFd, sync::OnceLock};
    static ORIGINAL: OnceLock<quire_contract_codegen::OriginalStdin> = OnceLock::new();
    ORIGINAL.get_or_init(|| {
        quire_contract_codegen::OriginalStdin::capture(std::io::stdin().as_fd())
            .expect("capture the integration caller's known-valid original stdin")
    })
}

/// Shared Kani command/report fixture builders, also used by library unit tests.
pub(crate) mod kani_run;
