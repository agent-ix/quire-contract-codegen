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

fn generated() -> (KaniScalarObligationHarness, Artifact) {
    generated_named("implication")
}

fn generated_named(function: &str) -> (KaniScalarObligationHarness, Artifact) {
    let source = SOURCE.replace("function implication", &format!("function {function}"));
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
    .expect("authored implication compiles through QSL");
    let checked = crate::admitted_enum_equality::checked(&compiled, &[]);
    let node = crate::admitted_enum_equality::application(&checked, function);
    let (harness, manifest, _) = kani_obligations::boolean_implies_scalar_from(checked, node);
    (harness, manifest)
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
    assert!(matches!(Compiled::read(&target.path, &claim(&harness)), Err(Refusal::MissingModule(module)) if module == harness.identity.module_symbol.as_str()));
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
    let compiled = Compiled::read(&target.path, &claim(harness)).expect("fresh compiled census");
    fs::write(
        target.path.join("current-evidence.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    (evidence, compiled, target)
}

fn subject_mutant(harness: &KaniScalarObligationHarness) -> KaniScalarObligationHarness {
    let mut mutant = harness.clone();
    let expression = "quire_contract_runtime::operators::implies_short_circuit(left, || right)";
    let parsed = syn::parse_file(&harness.rust.contents).unwrap();
    let calls = parsed.items.iter().filter_map(|item| match item { syn::Item::Mod(m) if m.ident == harness.identity.module_symbol.as_str() => m.content.as_ref(), _ => None })
        .flat_map(|(_, items)| items).filter_map(|item| match item { syn::Item::Fn(f) if f.sig.ident == "subject_boolean_implication" => Some(f), _ => None })
        .flat_map(|f| &f.block.stmts).filter(|stmt| matches!(stmt, syn::Stmt::Local(local) if matches!(&local.pat, syn::Pat::Ident(id) if id.ident == "subject") && local.init.as_ref().is_some_and(|init| matches!(&*init.expr, syn::Expr::Call(_))))).count();
    assert_eq!(
        calls, 1,
        "one parsed subject call inside the claimed module"
    );
    let module = claim(harness);
    let start = harness
        .rust
        .contents
        .lines()
        .take(usize::try_from(module.start - 1).unwrap())
        .map(|line| line.len() + 1)
        .sum::<usize>();
    let end = harness
        .rust
        .contents
        .lines()
        .take(usize::try_from(module.end).unwrap())
        .map(|line| line.len() + 1)
        .sum::<usize>()
        .min(harness.rust.contents.len());
    let selected = &harness.rust.contents[start..end];
    assert_eq!(selected.matches(expression).count(), 1);
    mutant.rust.contents = format!(
        "{}{}{}",
        &harness.rust.contents[..start],
        selected.replacen(expression, "false", 1),
        &harness.rust.contents[end..]
    );
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
    let (mut harness, manifest) = generated();
    let (mut second_harness, _) = generated_named("second_implication");
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
        .qualify(std::slice::from_ref(&declared), &healthy.checks)
        .unwrap();
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
        .qualify(std::slice::from_ref(&second_claim), &second_healthy.checks)
        .unwrap();
    let shared = compiled.shared_helpers(&declared).unwrap();
    // The direct Boolean specification model calls no function. This is evidence
    // about the observed build, not a catalog substituted for graph traversal.
    let reviewed = BTreeSet::new();
    fs::write(healthy_target.path.join("shared-helper-evidence.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "module": declared.module, "harness": declared.harness,
        "subject_root": format!("{}::subject_boolean_implication", declared.module),
        "expectation_root": format!("{}::expected_boolean_implication", declared.module),
        "observed_helpers": shared, "reviewed_helpers": reviewed,
    })).unwrap()).unwrap();
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
    retain_mutation(&second_mutant_target.path, &second_harness.rust.contents, &second_mutant.rust.contents);
    let KaniRunOutcome::Falsified { counterexample: second_playback } = &second_failed.outcome else { panic!("second module must falsify independently") };
    kani_obligations::assert_implication_false_left_playback(second_playback);
    assert!(second_healthy.checks.iter().any(|c| c.class == KaniCheckClass::Cover && c.status == KaniCheckStatus::Satisfied));

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


    for reason in [quire_contract_codegen::KaniInconclusiveReason::NoVerdict, quire_contract_codegen::KaniInconclusiveReason::TimedOut, quire_contract_codegen::KaniInconclusiveReason::FailedWithoutCounterexample] {
        let mut invalid = failed.clone(); invalid.outcome = KaniRunOutcome::Inconclusive { reason };
        let rejected = Control { name: &declared.module, before: &harness.rust.contents, after: &mutant.rust.contents, positive: &healthy, mutant: &invalid };
        assert_eq!(controls(std::slice::from_ref(&declared), &shared, &reviewed, &[rejected]), Err(Refusal::InvalidControl(declared.module.clone())));
    }
    let mut missing_check = failed.clone(); missing_check.checks.clear();
    assert_eq!(controls(std::slice::from_ref(&declared), &shared, &reviewed, &[Control { name: &declared.module, before: &harness.rust.contents, after: &mutant.rust.contents, positive: &healthy, mutant: &missing_check }]), Err(Refusal::InvalidControl(declared.module.clone())));
    assert_eq!(controls(std::slice::from_ref(&declared), &shared, &reviewed, &[Control { name: &declared.module, before: &harness.rust.contents, after: &harness.rust.contents, positive: &healthy, mutant: &failed }]), Err(Refusal::InvalidControl(declared.module.clone())));
    let KaniRunOutcome::Falsified { counterexample } = &failed.outcome else {
        panic!("must execute a falsifying source mutation: {failed:?}")
    };
    kani_obligations::assert_implication_false_left_playback(counterexample);
    retain_mutation(&mutant_target.path, &harness.rust.contents, &mutant.rust.contents);
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
    let mut seen = BTreeSet::new();
    let mut combined = String::new();
    for source in sources {
        for item in syn::parse_file(source).unwrap().items {
            let text = item.to_token_stream().to_string();
            if seen.insert(text.clone()) {
                combined.push_str(&text);
                combined.push('\n');
            }
        }
    }
    // Formatting supplies exact distinct module extents for Kani's line attribution.
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(combined.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn retain_mutation(directory: &Path, before: &str, after: &str) {
    fs::write(directory.join("unmodified.rs"), before).unwrap();
    fs::write(directory.join("mutant.rs"), after).unwrap();
    let output = std::process::Command::new("diff").args(["--unified", "--label", "unmodified/src/lib.rs", "--label", "mutant/src/lib.rs"])
        .arg(directory.join("unmodified.rs")).arg(directory.join("mutant.rs")).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "the source mutation must differ");
    fs::write(directory.join("source-mutation.diff"), output.stdout).unwrap();
}
