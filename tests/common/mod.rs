//! Shared integration-test fixtures.

/// The panic-token scan shared by the generator-source and emitted-source scans.
pub mod panic_scan;

/// The `withdraw` obligation fixture, shared with whatever test negotiates it as a real bound
/// package.
pub mod withdraw_fixture;
