//! Caller-ingress Text admission proof and real Kani mutation control (FR-035, TC-050).

use ix_trace_rs::trace;

use std::{
    fs,
    num::NonZeroU64,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    time::Duration,
};

use quire_contract_codegen::{
    execute_kani_obligation, execute_kani_obligations, generate_caller_text_kani_harness,
    CallerTextKaniRequest, KaniCallerTextObligationHarness, KaniExecutableHarness,
    KaniExecutionRefusal, KaniExecutionRequest, KaniInstallation, KaniReportRefusal,
    KaniRunOutcome, KaniSolver, KaniTool, KaniToolError, ProofCeilings, CALLER_TEXT_PAYLOAD_CLASS,
};
use quire_contract_runtime::exact::{TextProfile, TextType};
use quire_exact::Cancel;

const PROOF_CEILING: Duration = Duration::from_secs(600);

fn request(profile: TextProfile, min: u64, max: u64) -> CallerTextKaniRequest {
    CallerTextKaniRequest {
        text_type: TextType::new(min, max, profile).expect("valid bounds"),
        payload_class: CALLER_TEXT_PAYLOAD_CLASS.to_owned(),
        ceilings: ProofCeilings {
            memory_bytes: NonZeroU64::new(1_073_741_824).expect("nonzero ceiling"),
            wall_clock: PROOF_CEILING,
        },
        solver: KaniSolver::Cadical,
        unwind: 8,
    }
}

fn scratch_root(label: &str) -> crate::scratch_crate::TemporaryDirectory {
    let scratch =
        crate::scratch_crate::TemporaryDirectory::new(&format!("caller-text-admission-{label}"));
    fs::create_dir_all(scratch.path.join("crate/src")).expect("create scratch crate");
    scratch
}

fn manifest() -> String {
    format!(
        "[package]\nname = \"caller-text-admission-proof\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\n{}\n\n[workspace]\n",
        crate::scratch_crate::runtime_dependency(&["exact"])
    )
}

fn write_crate(root: &Path, harnesses: &[KaniCallerTextObligationHarness]) {
    let library = harnesses
        .iter()
        .map(|harness| harness.rust.contents.as_str())
        .collect::<String>();
    fs::write(root.join("crate/src/lib.rs"), library).expect("write harness crate");
    crate::scratch_crate::write_manifest(&root.join("crate"), &manifest());
}

struct CallerExecutorLane {
    root: PathBuf,
    _scratch: crate::scratch_crate::TemporaryDirectory,
    harness: KaniCallerTextObligationHarness,
    cancel: Cancel,
    installation: KaniInstallation,
    crate_directory: PathBuf,
    target_directory: PathBuf,
}

impl CallerExecutorLane {
    fn new(label: &str, report: Option<&str>, exit: i32) -> Self {
        let scratch =
            crate::scratch_crate::TemporaryDirectory::new(&format!("caller-text-executor-{label}"));
        let root = scratch.path.clone();
        let crate_directory = root.join("crate");
        let target_directory = root.join("target");
        fs::create_dir_all(crate_directory.join("src")).expect("create scratch crate");
        fs::create_dir_all(&target_directory).expect("create target directory");
        let harness = generate_caller_text_kani_harness(request(TextProfile::Nfc, 1, 1))
            .expect("caller harness");
        fs::write(crate_directory.join("src/lib.rs"), &harness.rust.contents)
            .expect("write caller harness source");
        let report_path = root.join("report.json");
        if let Some(report) = report {
            fs::write(&report_path, report).expect("write fake report");
        }
        let launcher = root.join("cargo-kani");
        let copy_report = if report.is_some() {
            format!("cp '{}' \"$last\"\n", report_path.display())
        } else {
            String::new()
        };
        fs::write(
            &launcher,
            format!(
                "#!/bin/sh\n[ \"$1\" = --probe ] && exit 0\nfor last; do :; done\n{copy_report}exit {exit}\n"
            ),
        )
        .expect("write launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("make launcher executable");
        Self {
            root,
            _scratch: scratch,
            harness,
            cancel: Cancel::new(),
            installation: KaniInstallation::new(launcher),
            crate_directory,
            target_directory,
        }
    }

    fn request(&self) -> KaniExecutionRequest<'_> {
        KaniExecutionRequest {
            home_directory: None,
            cancel: &self.cancel,
            installation: &self.installation,
            harness: KaniExecutableHarness::from(&self.harness),
            crate_directory: &self.crate_directory,
            target_directory: &self.target_directory,
        }
    }
}

type CallerCheck<'a> = (&'a str, &'a str);
type CallerReportEntry<'a> = (&'a str, &'a str, &'a [CallerCheck<'a>]);

fn caller_report(status: &str, checks: &[CallerCheck<'_>]) -> String {
    caller_batch_report(&[(
        "caller_text_nfc_1_1::caller_text_admission_agrees",
        status,
        checks,
    )])
}

fn caller_batch_report(entries: &[CallerReportEntry<'_>]) -> String {
    let results: Vec<_> = entries
        .iter()
        .map(|(harness_id, status, checks)| {
            let checks: Vec<_> = checks
                .iter()
                .enumerate()
                .map(|(index, (status, category))| {
                    serde_json::json!({
                        "id": index + 1,
                        "status": status,
                        "category": category,
                        "location": { "file": "src/lib.rs", "line": "10", "column": "5" }
                    })
                })
                .collect();
            serde_json::json!({
                "harness_id": harness_id,
                "status": status,
                "checks": checks
            })
        })
        .collect();
    serde_json::json!({
        "metadata": { "version": "1.0" },
        "verification_results": { "results": results }
    })
    .to_string()
}

#[trace("FR-035-AC-4", "FR-035-AC-5", "FR-017-AC-26", "TC-050")]
#[test]
#[ignore = "kani lane: 12 independent caller proofs plus a real falsifying mutation"]
fn tc_050_real_kani_proves_all_caller_text_profiles_and_falsifies_mutation() {
    let mut harnesses = Vec::new();
    for profile in TextProfile::ALL {
        for (min, max) in [(0, 8), (1, 1)] {
            harnesses.push(
                generate_caller_text_kani_harness(request(profile, min, max))
                    .expect("supported caller proof"),
            );
        }
    }
    assert_eq!(harnesses.len(), 12);

    let scratch = scratch_root("all-profiles");
    let root = &scratch.path;
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let cancel = Cancel::new();
    let crate_directory = root.join("crate");
    let target_directory = root.join("target");
    // Run one obligation at a time, serially, reusing this target directory. The generated
    // crate contains only the selected harness for each run, so compiler/backend work does not
    // scale with all twelve caller claims being present together.
    for harness in &harnesses {
        write_crate(&root, std::slice::from_ref(harness));
        let request = KaniExecutionRequest {
            home_directory: None,
            cancel: &cancel,
            installation: &installation,
            harness: KaniExecutableHarness::from(harness),
            crate_directory: &crate_directory,
            target_directory: &target_directory,
        };
        let evidence = execute_kani_obligation(&request).expect("caller proof executes");
        let selected_path = harness.identity.harness_path().to_string();
        assert_eq!(
            evidence.outcome,
            KaniRunOutcome::Verified,
            "profile {:?} bounds {}..={} observed memory {:?}",
            harness.identity.caller_claim.profile,
            harness.identity.caller_claim.min,
            harness.identity.caller_claim.max,
            evidence.memory
        );
        assert_eq!(evidence.kind, None);
        assert_eq!(
            evidence.caller_text_claim,
            Some(harness.identity.caller_claim.clone())
        );
        assert_eq!(
            evidence.selected_harness_path.as_deref(),
            Some(selected_path.as_str())
        );
        assert!(evidence.batch.is_none(), "each proof runs independently");
        assert!(evidence
            .arguments
            .windows(2)
            .any(|pair| pair[0] == "--harness" && pair[1] == selected_path));
        assert!(evidence.success_checks >= 2, "agreement and cover hold");
    }

    let selected = harnesses
        .iter()
        .find(|harness| {
            harness.identity.caller_claim.profile == quire_contract_codegen::CallerTextProfile::Nfc
                && harness.identity.caller_claim.min == 1
                && harness.identity.caller_claim.max == 1
        })
        .expect("NFC restrictive harness");
    let mut mutated = selected.clone();
    let original_call = "rt::admit_text(operand, text_type, meter)";
    let changed_call = "let widened = rt::TextType::new(0, 8, text_type.profile()).expect(\"mutation\");\n    rt::admit_text(operand, &widened, meter)";
    assert_eq!(mutated.rust.contents.matches(original_call).count(), 1);
    mutated.rust.contents = mutated.rust.contents.replace(original_call, changed_call);
    write_crate(&root, std::slice::from_ref(&mutated));
    let mutation = execute_kani_obligation(&KaniExecutionRequest {
        home_directory: None,
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::from(&mutated),
        crate_directory: &crate_directory,
        target_directory: &root.join("mutation-target"),
    })
    .unwrap_or_else(|refusal: KaniExecutionRefusal| {
        panic!("mutated caller proof refused: {refusal}")
    });
    assert!(matches!(
        mutation.outcome,
        KaniRunOutcome::Falsified { ref counterexample } if !counterexample.is_empty()
    ));
    assert_eq!(mutation.kind, None);
    assert_eq!(
        mutation.caller_text_claim,
        Some(selected.identity.caller_claim.clone())
    );
    assert_eq!(
        mutation.selected_harness_path.as_deref(),
        Some(selected.identity.harness_path().to_string().as_str())
    );

    write_crate(&root, std::slice::from_ref(selected));
    let restored = execute_kani_obligation(&KaniExecutionRequest {
        home_directory: None,
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::from(selected),
        crate_directory: &crate_directory,
        target_directory: &root.join("restored-target"),
    })
    .expect("restored caller proof executes");
    assert_eq!(restored.outcome, KaniRunOutcome::Verified);
    assert_eq!(
        restored.caller_text_claim,
        Some(selected.identity.caller_claim.clone())
    );
    assert!(restored
        .arguments
        .windows(selected.identity.options.len())
        .any(|options| options == selected.identity.options));
}

/// Caller Text identity remains attached to cover and inconclusive outcomes, while the same
/// production execution path refuses a source mismatch, unavailable launcher, or malformed Kani
/// report without producing evidence.
///
#[trace("FR-017-AC-26", "TC-050")]
#[test]
fn tc_050_caller_executor_preserves_outcomes_and_refuses_untrusted_execution() {
    let report = caller_report(
        "Success",
        &[("Success", "assertion"), ("Unsatisfiable", "cover")],
    );
    let cover = CallerExecutorLane::new("cover-unsatisfied", Some(&report), 0);
    let evidence = execute_kani_obligation(&cover.request()).expect("classified caller report");
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );
    assert_eq!(evidence.kind, None);
    assert_eq!(
        evidence.caller_text_claim,
        Some(cover.harness.identity.caller_claim.clone())
    );
    assert_eq!(
        evidence.selected_harness_path.as_deref(),
        Some("caller_text_nfc_1_1::caller_text_admission_agrees")
    );

    let malformed = CallerExecutorLane::new("malformed-report", Some("not json"), 1);
    assert!(matches!(
        execute_kani_obligation(&malformed.request()),
        Err(KaniExecutionRefusal::Report(
            KaniReportRefusal::Malformed { .. }
        ))
    ));

    let failed = caller_report("Failure", &[("Failure", "assertion")]);
    let inconclusive = CallerExecutorLane::new("no-playback", Some(&failed), 1);
    let evidence = execute_kani_obligation(&inconclusive.request())
        .expect("failure without playback is an inconclusive execution result");
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: quire_contract_codegen::KaniInconclusiveReason::FailedWithoutCounterexample
        }
    );
    assert_eq!(
        evidence.caller_text_claim,
        Some(inconclusive.harness.identity.caller_claim.clone())
    );
    assert_eq!(
        evidence.selected_harness_path.as_deref(),
        Some("caller_text_nfc_1_1::caller_text_admission_agrees")
    );

    let mismatch = CallerExecutorLane::new("source-mismatch", Some(&report), 0);
    fs::write(
        mismatch.crate_directory.join("src/lib.rs"),
        "pub fn unrelated() {}\n",
    )
    .expect("replace source with mismatched content");
    assert!(matches!(
        execute_kani_obligation(&mismatch.request()),
        Err(KaniExecutionRefusal::HarnessNotInCrate { .. })
    ));

    let missing = CallerExecutorLane::new("missing-launcher", Some(&report), 0);
    fs::remove_file(&missing.installation.launcher).expect("remove launcher");
    assert!(matches!(
        execute_kani_obligation(&missing.request()),
        Err(KaniExecutionRefusal::Tool(KaniToolError::Io {
            tool: KaniTool::Launcher,
            ..
        }))
    ));
}

/// Missing cover evidence stays inconclusive, and a compatible batch binds each caller's report
/// outcome and playback to that member's own typed identity and selected harness path.
///
#[trace("FR-017-AC-26", "TC-050")]
#[test]
fn tc_050_caller_executor_preserves_no_cover_and_mixed_batch_member_identity() {
    let no_cover_report = caller_report("Success", &[("Success", "assertion")]);
    let no_cover = CallerExecutorLane::new("no-cover-summary", Some(&no_cover_report), 0);
    let evidence = execute_kani_obligation(&no_cover.request()).expect("inconclusive caller run");
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: quire_contract_codegen::KaniInconclusiveReason::MissingCoverSummary
        }
    );
    assert_eq!(
        evidence.caller_text_claim,
        Some(no_cover.harness.identity.caller_claim.clone())
    );
    assert_eq!(
        evidence.selected_harness_path.as_deref(),
        Some("caller_text_nfc_1_1::caller_text_admission_agrees")
    );

    let first = no_cover.harness.clone();
    let second = generate_caller_text_kani_harness(request(TextProfile::Nfd, 1, 1))
        .expect("second caller profile");
    let first_path = first.identity.harness_path().to_string();
    let second_path = second.identity.harness_path().to_string();
    let batch_report = caller_batch_report(&[
        (
            &first_path,
            "Success",
            &[("Success", "assertion"), ("Satisfied", "cover")],
        ),
        (
            &second_path,
            "Failure",
            &[("Failure", "assertion"), ("Satisfied", "cover")],
        ),
    ]);
    let batch = CallerExecutorLane::new("mixed-batch", Some(&batch_report), 1);
    write_crate(&batch.root, &[first.clone(), second.clone()]);
    let playback = format!(
        "Concrete playback unit test for `{second_path}`:\n```\n/// Test generated for harness `{second_path}`\n///\n/// Check for `assertion`: \"caller admission agreement\"\n\n#[test]\nfn caller_text_mutation() {{\n    kani::concrete_playback_run(vec![], caller_text_admission_agrees);\n}}\n```\n"
    );
    fs::write(batch.root.join("playback.txt"), &playback).expect("write playback fixture");
    fs::write(
        &batch.installation.launcher,
        format!(
            "#!/bin/sh\n[ \"$1\" = --probe ] && exit 0\nfor last; do :; done\ncp '{}' \"$last\"\ncat '{}'\nexit 1\n",
            batch.root.join("report.json").display(),
            batch.root.join("playback.txt").display()
        ),
    )
    .expect("write batch launcher");
    fs::set_permissions(
        &batch.installation.launcher,
        fs::Permissions::from_mode(0o755),
    )
    .expect("make batch launcher executable");

    let requests = [
        KaniExecutionRequest {
            home_directory: None,
            cancel: &batch.cancel,
            installation: &batch.installation,
            harness: (&first).into(),
            crate_directory: &batch.crate_directory,
            target_directory: &batch.target_directory,
        },
        KaniExecutionRequest {
            home_directory: None,
            cancel: &batch.cancel,
            installation: &batch.installation,
            harness: (&second).into(),
            crate_directory: &batch.crate_directory,
            target_directory: &batch.target_directory,
        },
    ];
    let mut groups = execute_kani_obligations(&requests).expect("compatible caller batch");
    assert_eq!(groups.len(), 1);
    let group = groups.pop().expect("one caller batch");
    assert_eq!(group.members, [0, 1]);
    let evidence = group.evidence.expect("classified batch members");
    assert_eq!(evidence.len(), 2);
    assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence[0].kind, None);
    assert_eq!(
        evidence[0].caller_text_claim,
        Some(first.identity.caller_claim.clone())
    );
    assert_eq!(
        evidence[0].selected_harness_path.as_deref(),
        Some(first_path.as_str())
    );
    assert_eq!(evidence[1].kind, None);
    assert_eq!(
        evidence[1].caller_text_claim,
        Some(second.identity.caller_claim.clone())
    );
    assert_eq!(
        evidence[1].selected_harness_path.as_deref(),
        Some(second_path.as_str())
    );
    assert!(matches!(
        &evidence[1].outcome,
        KaniRunOutcome::Falsified { counterexample }
            if counterexample.contains(&second_path)
                && counterexample.contains("kani::concrete_playback_run")
                && !counterexample.contains(&first_path)
    ));
    assert!(evidence.iter().all(|item| item.batch.is_some()));
}
