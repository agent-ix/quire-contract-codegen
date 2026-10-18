//! Private libtest orchestration of a generated unsatisfiable-assumption control.

use crate::kani::{
    classify::KaniRunOutcome,
    generate::unsatisfiable_harness_for_test,
    run::{
        execute::{execute_kani_obligation, KaniExecutionRequest},
        tool::KaniInstallation,
    },
    test_support::discover_scratch,
};
use ix_trace_rs::trace;

#[trace("FR-015-AC-117", "TC-025")]
#[test]
#[ignore = "installed Kani backend: run this private lib control explicitly and serially"]
fn unsatisfiable_assume_is_cover_unsatisfied_under_real_kani() {
    let harness = unsatisfiable_harness_for_test();
    let directory = discover_scratch("render-unsat");
    std::fs::create_dir_all(directory.join("src")).unwrap();
    std::fs::write(directory.join("Cargo.toml"), "[package]\nname = \"render-unsat-control\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
    std::fs::write(directory.join("src/lib.rs"), &harness.rust.contents).unwrap();
    std::fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let installation = KaniInstallation::discover().expect("installed cargo-kani");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &quire_exact::Cancel::new(),
        installation: &installation,
        harness: (&harness).into(),
        crate_directory: &directory,
        target_directory: &directory.join("target"),
        home_directory: None,
    })
    .unwrap();
    assert!(
        evidence.success_checks > 0,
        "a reachable pre-assumption property must succeed: {evidence:?}"
    );
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );
    std::fs::remove_dir_all(directory).unwrap();
}
