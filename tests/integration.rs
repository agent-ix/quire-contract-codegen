use quire_contract_codegen::{IR_CANDIDATE_REVISION, RUNTIME_REVISION};

/// TC-001.
#[test]
fn exact_upstream_revisions_match_the_dependency_declarations_and_lockfile() {
    let manifest = include_str!("../Cargo.toml");
    let lockfile = include_str!("../Cargo.lock");
    assert!(manifest.contains(&format!("rev = \"{IR_CANDIDATE_REVISION}\"")));
    assert!(manifest.contains(&format!("rev = \"{RUNTIME_REVISION}\"")));
    assert!(lockfile.contains(&format!("?rev={IR_CANDIDATE_REVISION}")));
    assert!(lockfile.contains(&format!("?rev={RUNTIME_REVISION}")));
}
