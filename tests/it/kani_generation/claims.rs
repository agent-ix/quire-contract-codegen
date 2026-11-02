//! Current generated claims are qualified from compiled artifacts, never a saved verdict.

mod census;

use std::{collections::BTreeSet, fs, path::Path};

use crate::{kani_obligations, scratch_crate::TemporaryDirectory};
use census::{controls, Claim, Compiled, Control, Refusal};
use ix_trace_rs::trace;
use quire_contract_codegen::{
    execute_kani_obligation, Artifact, KaniCheckClass, KaniCheckStatus, KaniExecutionEvidence,
    KaniExecutionRequest, KaniInstallation, KaniRunOutcome, KaniScalarObligationHarness,
};

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction implication using v(left: Boolean, right: Boolean): Boolean pure { left implies right }\n";

fn checked_source(source: &str) -> quire_contract_model::CheckedPackageV2 {
    let compiled = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("test", "implication.native", "git", "current"),
            package_name: "current-implication".to_owned(),
            path: "implication.native",
            bytes: source.as_bytes(),
        },
        [],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("authored implications compile through QSL");
    crate::admitted_enum_equality::checked(&compiled, &[])
}

fn generated() -> (KaniScalarObligationHarness, Artifact) {
    let checked = checked_source(SOURCE);
    let node = crate::admitted_enum_equality::application(&checked, "implication");
    let (harness, manifest, _) = kani_obligations::boolean_implies_scalar_from(checked, node);
    (harness, manifest)
}

fn generated_pair() -> ([KaniScalarObligationHarness; 2], Artifact) {
    let checked = checked_source(&format!("{SOURCE}function second_implication using v(left: Boolean, right: Boolean): Boolean pure {{ right implies left }}\n"));
    let nodes = [
        crate::admitted_enum_equality::application(&checked, "implication"),
        crate::admitted_enum_equality::application(&checked, "second_implication"),
    ];
    let (harnesses, manifest, _) = kani_obligations::boolean_implies_scalars_from(checked, &nodes);
    (
        harnesses
            .try_into()
            .expect("two production claims in one request receive distinct generated identities"),
        manifest,
    )
}

fn claim(harness: &KaniScalarObligationHarness) -> Claim {
    Claim::generated(harness).expect("production generated module and named roots")
}

#[trace("TC-056", "FR-041-AC-6", "FR-041-AC-7")]
#[test]
fn current_claim_comes_from_authored_admitted_v2_implication() {
    let (harness, _) = generated();
    let declared = claim(&harness);
    assert!(harness.rust.contents.contains(
        "fn expected_boolean_implication(left: bool, right: bool) -> bool {\n!left || right"
    ));
    assert!(declared
        .module
        .contains(harness.identity.module_symbol.as_str()));
    assert!(declared.end > declared.start);
    let mut renamed = harness.clone();
    renamed.rust.contents = renamed.rust.contents.replace(&declared.module, "unclaimed");
    assert_eq!(
        Claim::generated(&renamed).unwrap_err(),
        Refusal::MissingModule(declared.module)
    );
}

#[trace("TC-056", "FR-041-AC-1", "FR-041-AC-5", "FR-041-AC-6")]
#[test]
fn a_generated_source_or_saved_verdict_without_this_build_does_not_qualify() {
    let (harness, _) = generated();
    let target = TemporaryDirectory::new("uncompiled-current-claim");
    assert!(
        matches!(Compiled::read(&target.path, &claim(&harness), Path::new("uncompiled")), Err(Refusal::MissingModule(module)) if module == harness.identity.module_symbol.as_str())
    );
}

#[trace("TC-056", "FR-041-AC-2", "FR-041-AC-6", "FR-041-AC-8")]
#[test]
fn assembled_current_modules_preserve_production_items_and_mutate_only_one_subject() {
    use quote::ToTokens;
    let ([mut first, mut second], _) = generated_pair();
    assert_ne!(first.identity.module_symbol, second.identity.module_symbol);
    let original_sources = [first.rust.contents.as_str(), second.rust.contents.as_str()];
    let combined = assemble(&original_sources);
    let tokens = |source: &str| {
        syn::parse_file(source)
            .unwrap()
            .items
            .into_iter()
            .map(|item| item.to_token_stream().to_string())
            .collect::<Vec<_>>()
    };
    let combined_items = tokens(&combined);
    for source in original_sources {
        for item in tokens(source) {
            assert_eq!(
                combined_items
                    .iter()
                    .filter(|candidate| **candidate == item)
                    .count(),
                1,
                "each unchanged production item appears exactly once"
            );
        }
    }
    first.rust.contents = combined.clone();
    second.rust.contents = combined;
    let first_claim = claim(&first);
    let second_claim = claim(&second);
    assert!(first_claim.end < second_claim.start);
    let mutant = subject_mutant(&first);
    let mutant_items = tokens(&mutant.rust.contents);
    assert_eq!(combined_items.len(), mutant_items.len());
    assert_eq!(
        combined_items
            .iter()
            .zip(&mutant_items)
            .filter(|(before, after)| before != after)
            .count(),
        1,
        "only the selected module's parsed subject expression changes"
    );
    assert_eq!(claim(&mutant).harness, first_claim.harness);
    assert_eq!(claim(&second).harness, second_claim.harness);
}

fn run(
    harness: &KaniScalarObligationHarness,
    manifest: &Artifact,
    name: &str,
) -> (KaniExecutionEvidence, Compiled, TemporaryDirectory) {
    let scratch = kani_obligations::write_scalar_crate(name, manifest, &harness.rust.contents);
    // A new, empty compilation directory is part of qualification: a launcher returning
    // a preselected report cannot supply this build's symbols or call graph.
    let target = TemporaryDirectory::new(&format!("current-claims-{name}"));
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("installed Kani");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory: &scratch.path,
        target_directory: &target.path,
        home_directory: None,
    })
    .expect("bounded installed execution");
    let compiled = Compiled::read(
        &target.path,
        &claim(harness),
        Path::new(&evidence.launcher_path),
    )
    .expect("fresh compiled census");
    for file in ["Cargo.toml", "Cargo.lock", "src/lib.rs"] {
        fs::copy(
            scratch.path.join(file),
            target
                .path
                .join(format!("current-{}", file.replace('/', "-"))),
        )
        .unwrap();
    }

    fs::write(
        target.path.join("current-evidence.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    let attempt = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("current-claim-attempts")
        .join(name);
    fs::create_dir_all(&attempt).unwrap();
    for file in [
        "current-evidence.json",
        "current-Cargo.toml",
        "current-Cargo.lock",
        "current-src-lib.rs",
        "compiled-census.json",
        "compiled-call-graph.txt",
        "compiled-symbols.json",
    ] {
        fs::copy(target.path.join(file), attempt.join(file)).unwrap();
    }
    (evidence, compiled, target)
}

fn subject_mutant(harness: &KaniScalarObligationHarness) -> KaniScalarObligationHarness {
    use syn::spanned::Spanned;
    let parsed = syn::parse_file(&harness.rust.contents).unwrap();
    let mut calls = Vec::new();
    for item in &parsed.items {
        let syn::Item::Mod(module) = item else {
            continue;
        };
        if module.ident != harness.identity.module_symbol.as_str() {
            continue;
        }
        let (_, items) = module.content.as_ref().unwrap();
        for item in items {
            let syn::Item::Fn(function) = item else {
                continue;
            };
            if function.sig.ident != "subject_boolean_implication" {
                continue;
            }
            for statement in &function.block.stmts {
                let syn::Stmt::Local(local) = statement else {
                    continue;
                };
                if !matches!(&local.pat, syn::Pat::Ident(name) if name.ident == "subject") {
                    continue;
                }
                let syn::Expr::Call(call) = &*local.init.as_ref().unwrap().expr else {
                    panic!("subject is a direct call")
                };
                let syn::Expr::Path(path) = &*call.func else {
                    panic!("subject has a named path")
                };
                assert_eq!(
                    path.path
                        .segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect::<Vec<_>>(),
                    [
                        "quire_contract_runtime",
                        "operators",
                        "implies_short_circuit"
                    ]
                );
                assert_eq!(call.args.len(), 2);
                calls.push(call.span().byte_range());
            }
        }
    }
    let [expression] = calls.as_slice() else {
        panic!("exactly one parsed subject call: {calls:?}")
    };
    let mut mutant = harness.clone();
    mutant
        .rust
        .contents
        .replace_range(expression.clone(), "false");
    mutant
}

#[trace(
    "TC-056",
    "FR-041-AC-1",
    "FR-041-AC-3",
    "FR-041-AC-4",
    "FR-041-AC-5",
    "FR-041-AC-6",
    "FR-041-AC-7",
    "FR-041-AC-8",
    "FR-041-AC-9",
    "NFR-006-AC-19"
)]
#[test]
#[ignore = "required installed-Kani claim control in make kani-gate"]
fn current_generated_claim_has_fresh_compiled_roots_and_falsifying_control() {
    let ([mut harness, mut second_harness], manifest) = generated_pair();
    let assembled = assemble(&[&harness.rust.contents, &second_harness.rust.contents]);
    harness.rust.contents = assembled.clone();
    second_harness.rust.contents = assembled;
    let declared = claim(&harness);
    let second_claim = claim(&second_harness);
    assert!(
        declared.end < second_claim.start,
        "two generated modules have disjoint extents in the same source file"
    );
    let (healthy, compiled, healthy_target) = run(&harness, &manifest, "claim-healthy");
    assert_eq!(healthy.outcome, KaniRunOutcome::Verified, "{healthy:?}");
    compiled
        .require_positive(&declared, &healthy)
        .unwrap_or_else(|refusal| panic!("{refusal:?}; claim={declared:?}; evidence={healthy:?}"));
    let (second_healthy, second_compiled, second_target) =
        run(&second_harness, &manifest, "claim-second-healthy");
    assert_eq!(second_healthy.outcome, KaniRunOutcome::Verified);
    let mut both_checks = healthy.checks.clone();
    both_checks.extend(second_healthy.checks.iter().cloned());
    compiled
        .qualify(&[declared.clone(), second_claim.clone()], &both_checks)
        .unwrap();
    assert_eq!(
        compiled.qualify(&[declared.clone(), second_claim.clone()], &healthy.checks),
        Err(Refusal::Unreached(second_claim.module.clone()))
    );
    second_compiled
        .require_positive(&second_claim, &second_healthy)
        .unwrap();
    let shared = compiled.shared_helpers(&declared).unwrap();
    // The direct Boolean specification model calls no function. This is evidence
    // about the observed build, not a catalog substituted for graph traversal.
    let reviewed = BTreeSet::new();
    fs::write(
        healthy_target.path.join("shared-helper-evidence.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "module": declared.module, "harness": declared.harness,
            "subject_root": format!("{}::subject_boolean_implication", declared.module),
            "expectation_root": format!("{}::expected_boolean_implication", declared.module),
            "observed_helpers": shared, "reviewed_helpers": reviewed,
        }))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        shared, reviewed,
        "every observed shared helper needs its own source control"
    );
    assert!(healthy
        .checks
        .iter()
        .any(|c| c.class == KaniCheckClass::Cover && c.status == KaniCheckStatus::Satisfied));
    assert_eq!(
        compiled.qualify(&[], &healthy.checks),
        Err(Refusal::EmptyClaims)
    );
    let mut absent = declared.clone();
    absent.module.push_str("_absent");
    assert_eq!(
        compiled.qualify(&[absent.clone()], &healthy.checks),
        Err(Refusal::MissingModule(absent.module))
    );

    let mutant = subject_mutant(&harness);
    let (failed, mutant_compiled, mutant_target) = run(&mutant, &manifest, "claim-mutant");
    let mutated_claim = claim(&mutant);
    mutant_compiled
        .require_failure(&mutated_claim, &failed)
        .unwrap();
    let second_mutant = subject_mutant(&second_harness);
    let (second_failed, second_mutant_compiled, second_mutant_target) =
        run(&second_mutant, &manifest, "claim-second-mutant");
    second_mutant_compiled
        .require_failure(&claim(&second_mutant), &second_failed)
        .unwrap();
    retain_mutation(
        &second_mutant_target.path,
        &second_harness.rust.contents,
        &second_mutant.rust.contents,
    );
    let KaniRunOutcome::Falsified {
        counterexample: second_playback,
    } = &second_failed.outcome
    else {
        panic!("second module must falsify independently")
    };
    kani_obligations::assert_implication_false_left_playback(second_playback);
    assert!(second_healthy
        .checks
        .iter()
        .any(|c| c.class == KaniCheckClass::Cover && c.status == KaniCheckStatus::Satisfied));

    let second_shared = second_compiled.shared_helpers(&second_claim).unwrap();
    assert_eq!(second_shared, reviewed);
    controls(
        &[declared.clone(), second_claim.clone()],
        &shared,
        &reviewed,
        &[
            Control {
                name: &declared.module,
                before: &harness.rust.contents,
                after: &mutant.rust.contents,
                positive: &healthy,
                mutant: &failed,
            },
            Control {
                name: &second_claim.module,
                before: &second_harness.rust.contents,
                after: &second_mutant.rust.contents,
                positive: &second_healthy,
                mutant: &second_failed,
            },
        ],
    )
    .unwrap();
    assert_eq!(
        controls(
            std::slice::from_ref(&declared),
            &shared,
            &reviewed,
            &[Control {
                name: &declared.module,
                before: &harness.rust.contents,
                after: &mutant.rust.contents,
                positive: &second_healthy,
                mutant: &second_failed,
            }]
        ),
        Err(Refusal::InvalidControl(declared.module.clone())),
        "another module's executed control cannot discharge this one"
    );
    let control = Control {
        name: &declared.module,
        before: &harness.rust.contents,
        after: &mutant.rust.contents,
        positive: &healthy,
        mutant: &failed,
    };
    controls(
        std::slice::from_ref(&declared),
        &shared,
        &reviewed,
        std::slice::from_ref(&control),
    )
    .unwrap();
    assert_eq!(
        controls(std::slice::from_ref(&declared), &shared, &reviewed, &[]),
        Err(Refusal::MissingControl(declared.module.clone()))
    );
    let added = BTreeSet::from(["invented helper".to_owned()]);
    assert_eq!(
        controls(std::slice::from_ref(&declared), &shared, &added, &[]),
        Err(Refusal::HelperEvidence)
    );
    assert_eq!(
        controls(std::slice::from_ref(&declared), &added, &reviewed, &[]),
        Err(Refusal::HelperEvidence)
    );
    let mut still_verifies = control;
    still_verifies.mutant = &healthy;
    assert_eq!(
        controls(
            std::slice::from_ref(&declared),
            &shared,
            &reviewed,
            &[still_verifies]
        ),
        Err(Refusal::InvalidControl(declared.module.clone()))
    );

    for reason in [
        quire_contract_codegen::KaniInconclusiveReason::NoVerdict,
        quire_contract_codegen::KaniInconclusiveReason::TimedOut,
        quire_contract_codegen::KaniInconclusiveReason::FailedWithoutCounterexample,
    ] {
        let mut invalid = failed.clone();
        invalid.outcome = KaniRunOutcome::Inconclusive { reason };
        let rejected = Control {
            name: &declared.module,
            before: &harness.rust.contents,
            after: &mutant.rust.contents,
            positive: &healthy,
            mutant: &invalid,
        };
        assert_eq!(
            controls(
                std::slice::from_ref(&declared),
                &shared,
                &reviewed,
                &[rejected]
            ),
            Err(Refusal::InvalidControl(declared.module.clone()))
        );
    }
    let mut missing_check = failed.clone();
    missing_check.checks.clear();
    assert_eq!(
        controls(
            std::slice::from_ref(&declared),
            &shared,
            &reviewed,
            &[Control {
                name: &declared.module,
                before: &harness.rust.contents,
                after: &mutant.rust.contents,
                positive: &healthy,
                mutant: &missing_check
            }]
        ),
        Err(Refusal::InvalidControl(declared.module.clone()))
    );
    assert_eq!(
        controls(
            std::slice::from_ref(&declared),
            &shared,
            &reviewed,
            &[Control {
                name: &declared.module,
                before: &harness.rust.contents,
                after: &harness.rust.contents,
                positive: &healthy,
                mutant: &failed
            }]
        ),
        Err(Refusal::InvalidControl(declared.module.clone()))
    );
    let KaniRunOutcome::Falsified { counterexample } = &failed.outcome else {
        panic!("must execute a falsifying source mutation: {failed:?}")
    };
    kani_obligations::assert_implication_false_left_playback(counterexample);
    retain_mutation(
        &mutant_target.path,
        &harness.rust.contents,
        &mutant.rust.contents,
    );
    // Keep evidence beyond the temporary compilation directories, under this worktree's target.
    let evidence_directory =
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("qualified-current-claims");
    fs::create_dir_all(&evidence_directory).unwrap();
    for (from, to) in [
        (&healthy_target.path, "healthy"),
        (&mutant_target.path, "mutant"),
        (&second_target.path, "second-healthy"),
        (&second_mutant_target.path, "second-mutant"),
    ] {
        for file in [
            "current-evidence.json",
            "current-Cargo.toml",
            "current-Cargo.lock",
            "current-src-lib.rs",
            "compiled-census.json",
            "compiled-call-graph.txt",
            "compiled-symbols.json",
            "shared-helper-evidence.json",
            "source-mutation.diff",
            "unmodified.rs",
            "mutant.rs",
        ] {
            if from.join(file).exists() {
                fs::copy(
                    from.join(file),
                    evidence_directory.join(format!("{to}-{file}")),
                )
                .unwrap();
            }
        }
    }
}

fn assemble(sources: &[&str]) -> String {
    use quote::ToTokens;
    use syn::spanned::Spanned;
    let mut seen = BTreeSet::new();
    let mut attributes = String::new();
    let mut combined = String::new();
    for source in sources {
        let parsed = syn::parse_file(source).unwrap();
        for attribute in parsed.attrs {
            if seen.insert(format!("attribute:{}", attribute.to_token_stream())) {
                attributes.push_str(&source[attribute.span().byte_range()]);
                attributes.push('\n');
            }
        }
        for item in parsed.items {
            if seen.insert(item.to_token_stream().to_string()) {
                // Keep every production item's source bytes; only identical shared
                // top-level definitions are coalesced when assembling one crate.
                combined.push_str(&source[item.span().byte_range()]);
                combined.push('\n');
            }
        }
    }
    attributes.push_str(&combined);
    syn::parse_file(&attributes).expect("assembled production items remain valid Rust");
    attributes
}

fn retain_mutation(directory: &Path, before: &str, after: &str) {
    fs::write(directory.join("unmodified.rs"), before).unwrap();
    fs::write(directory.join("mutant.rs"), after).unwrap();
    let output = std::process::Command::new("diff")
        .args([
            "--unified",
            "--label",
            "unmodified/src/lib.rs",
            "--label",
            "mutant/src/lib.rs",
        ])
        .arg(directory.join("unmodified.rs"))
        .arg(directory.join("mutant.rs"))
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "the source mutation must differ"
    );
    fs::write(directory.join("source-mutation.diff"), output.stdout).unwrap();
}
