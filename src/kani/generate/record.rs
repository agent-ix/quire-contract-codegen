//! The validated harness path and the persisted per-harness record that every family's render
//! step builds last.

use serde::Serialize;

use crate::{
    core::artifact::Artifact,
    core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError},
    kani::generate::outcome::UnsupportedObligation,
};

/// The validated `module::harness` identity of a harness, built once where the harness is
/// generated. The names are `kob_`-prefixed readable components, so a refusal is an internal
/// invariant failing and is reported as invalid generated Rust.
pub(super) fn harness_path(
    module: &str,
    harness: &str,
) -> Result<HarnessPath, UnsupportedObligation> {
    let invalid = |error: SymbolError| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    };
    Ok(HarnessPath {
        module: ModuleSymbol::try_from(module).map_err(invalid)?,
        harness: HarnessSymbol::try_from(harness).map_err(invalid)?,
    })
}

/// The persisted `kani-obligations/{module}.json` record of one harness.
pub(super) fn record<T: Serialize>(
    module: &str,
    identity: &T,
    rust: &Artifact,
) -> Result<Artifact, UnsupportedObligation> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct HarnessRecord<'a, T> {
        identity: &'a T,
        rust_path: &'a str,
    }
    let mut json = serde_json::to_string(&HarnessRecord {
        identity,
        rust_path: &rust.path,
    })
    .map_err(|_| UnsupportedObligation::RenderFailed)?;
    json.push('\n');
    Ok(artifact(format!("kani-obligations/{module}.json"), json))
}

pub(super) fn artifact(path: String, contents: String) -> Artifact {
    Artifact::new(path, contents)
}
