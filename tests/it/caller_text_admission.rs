//! Caller-ingress Text admission proof and real Kani mutation control (FR-035, TC-050).

use std::{
    env, fs,
    num::NonZeroU64,
    path::{Path, PathBuf},
    time::Duration,
};

use quire_contract_codegen::{
    execute_kani_obligation, execute_kani_obligations, generate_caller_text_kani_harness,
    CallerTextKaniRequest, KaniCallerTextObligationHarness, KaniExecutableHarness,
    KaniExecutionRefusal, KaniExecutionRequest, KaniInstallation, KaniRunOutcome, KaniSolver,
    ProofCeilings, CALLER_TEXT_PAYLOAD_CLASS,
};
use quire_contract_runtime::exact::{TextProfile, TextType};

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

fn scratch_root(label: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("caller-text-admission")
        .join(format!("{label}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("crate/src")).expect("create scratch crate");
    root
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

/// Trace: FR-035-AC-4, FR-035-AC-5, FR-017-AC-26, TC-050.
#[test]
#[ignore = "kani lane: 12 caller proofs plus a real falsifying mutation; coordinate slot before running"]
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

    let root = scratch_root("all-profiles");
    write_crate(&root, &harnesses);
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let crate_directory = root.join("crate");
    let target_directory = root.join("target");
    let requests: Vec<_> = harnesses
        .iter()
        .map(|harness| KaniExecutionRequest {
            installation: &installation,
            harness: KaniExecutableHarness::from(harness),
            crate_directory: &crate_directory,
            target_directory: &target_directory,
        })
        .collect();
    let groups = execute_kani_obligations(&requests).expect("caller batch executes");
    let evidence = groups
        .into_iter()
        .flat_map(|group| group.evidence.expect("batch report classifies"))
        .collect::<Vec<_>>();
    assert_eq!(evidence.len(), 12);
    for (harness, evidence) in harnesses.iter().zip(evidence) {
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.kind, None);
        assert_eq!(
            evidence.caller_text_claim,
            Some(harness.identity.caller_claim.clone())
        );
        assert_eq!(
            evidence.selected_harness_path.as_deref(),
            Some(harness.identity.harness_path().to_string().as_str())
        );
        assert!(evidence.batch.as_ref().is_some_and(|batch| batch
            .members
            .contains(&harness.identity.harness_path().to_string())));
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
    let mut mutated_group = harnesses.clone();
    let selected_position = mutated_group
        .iter()
        .position(|harness| harness.identity == selected.identity)
        .expect("selected harness is in the batch");
    mutated_group[selected_position] = mutated.clone();
    write_crate(&root, &mutated_group);
    let mutation = execute_kani_obligation(&KaniExecutionRequest {
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

    write_crate(&root, &harnesses);
    let restored = execute_kani_obligation(&KaniExecutionRequest {
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
