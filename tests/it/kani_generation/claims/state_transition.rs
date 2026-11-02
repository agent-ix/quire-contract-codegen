// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authored Boolean transitions cross QSL emission and the public strict IR reader.

use std::{fs, num::NonZeroU64, path::PathBuf, process::Command, sync::OnceLock, time::Duration};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    negotiate_kani_obligations, KaniObligationOutcome, KaniObligationRequest,
    KaniStateTransitionHarness, ObligationItem, ProofCeilings,
};
use quire_contract_model::{
    CheckedNodeId, CheckedNodeKind, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, StateForm,
};

const PRECONDITION: &str = "pre P0 using v on M::Switch::set { flag or self.on }\n";

fn qsl_root() -> &'static PathBuf {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        // Metadata only: the serial parent test command holds the machine-wide Cargo lock.
        let output = Command::new(env!("CARGO"))
            .args(["metadata", "--offline", "--locked", "--format-version", "1"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("resolve the current locked QSL dependency");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let replay = metadata["packages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|package| package["name"] == "qsl-replay")
            .expect("the actual public QSL facade is resolved");
        let manifest = PathBuf::from(replay["manifest_path"].as_str().unwrap());
        manifest.parent().unwrap().parent().unwrap().to_owned()
    })
}

fn admitted(precondition: &str, package_name: &str) -> CheckedPackageV2 {
    let example = qsl_root().join("examples/switch-transition");
    let document = fs::read(example.join("switch.semantic-ir.json")).unwrap();
    let source = fs::read_to_string(example.join("switch.native")).unwrap() + precondition;
    let compilation = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("test", "transition.native", "git", "current"),
            package_name: package_name.to_owned(),
            path: "transition.native",
            bytes: source.as_bytes(),
        },
        [document.as_slice()],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("the authored precondition and QSL-owned source/model compile");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence.support_feature("quire.protocol.finite-global/v1");
    evidence
        .insert_domain_package_document("test/orders", document)
        .expect("retain the actual QSL-owned model evidence");
    match CheckedPackageV2::read(
        compilation.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("fresh QSL output must pass the public strict reader: {other:?}"),
    }
}

fn selected(package: &CheckedPackageV2) -> CheckedNodeId {
    let nodes = package
        .graph()
        .nodes
        .iter()
        .zip(package.node_kinds())
        .filter(|(_, kind)| **kind == CheckedNodeKind::State(StateForm::Transition))
        .map(|(node, _)| node.node_id.clone())
        .collect::<Vec<_>>();
    let [one] = nodes.as_slice() else {
        panic!("the authentic Switch source must emit exactly one transition: {nodes:?}");
    };
    one.clone()
}

fn generate(package: &CheckedPackageV2, transition: &CheckedNodeId) -> KaniObligationOutcome {
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: ProofCeilings {
            memory_bytes: NonZeroU64::new(4 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: Duration::from_secs(120),
        },
        items: &[ObligationItem::StateTransition {
            package,
            transition,
            subject_path: "crate::update_switch",
        }],
        subject_path: "crate::unused_subject",
        unwind: 3,
    })
    .expect("a well-formed transition request negotiates")
}

fn harness(package: &CheckedPackageV2) -> KaniStateTransitionHarness {
    let KaniObligationOutcome::Emitted {
        mut state_transition_harnesses,
        ..
    } = generate(package, &selected(package))
    else {
        panic!("the original Boolean transition profile must produce a harness");
    };
    assert_eq!(state_transition_harnesses.len(), 1);
    state_transition_harnesses.remove(0)
}

#[trace("TC-056", "FR-041-AC-10")]
#[test]
fn original_boolean_transition_uses_canonical_claim_and_source_provenance() {
    let package = admitted(PRECONDITION, "boolean-transition");
    let generated = harness(&package);
    assert_eq!(
        generated,
        harness(&package),
        "identical inputs reproduce all bytes"
    );
    let id = &generated.identity;
    let transition = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == id.transition)
        .unwrap();
    assert_eq!(id.occurrences, transition.occurrences);
    assert_eq!(
        id.source_map,
        package
            .source_map()
            .iter()
            .filter(|entry| entry.node_id == id.transition)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert!(!id.occurrences.is_empty());
    assert!(!id.source_map.is_empty());
    assert_eq!(id.field, "on");
    assert_eq!(id.parameter, "flag");
    assert_eq!(generated.rust.contents.matches("kani::any()").count(), 2);
    assert_eq!(generated.rust.contents.matches("kani::cover!(").count(), 1);
    assert!(generated
        .rust
        .contents
        .contains("crate::update_switch(input_current, &mut state_post)"));
    assert!(generated.rust.contents.contains("assert!(state_post,"));
    assert!(generated
        .rust
        .contents
        .contains("fn expected_boolean_state_transition"));
    assert!(generated
        .rust
        .contents
        .contains("input_current || state_pre"));
    assert!(!generated.rust.contents.contains("quire_exact"));
}

#[trace("TC-056", "FR-041-AC-10")]
#[test]
fn transition_claim_requires_its_selected_module_roots_and_postcondition_assertion() {
    use super::census::{Claim, Refusal};

    let package = admitted(PRECONDITION, "transition-claim-roots");
    let original = harness(&package);
    Claim::state_transition(&original).expect("the production claim has all named roots");

    let mut missing_module = original.clone();
    missing_module.rust.contents = missing_module.rust.contents.replace(
        original.identity.module_symbol.as_str(),
        "unclaimed_transition_module",
    );
    assert!(matches!(
        Claim::state_transition(&missing_module),
        Err(Refusal::MissingModule(_))
    ));

    let mut missing_expectation = original.clone();
    missing_expectation.rust.contents = missing_expectation
        .rust
        .contents
        .replace("expected_boolean_state_transition", "unclaimed_expectation");
    assert!(matches!(
        Claim::state_transition(&missing_expectation),
        Err(Refusal::MissingRoot(_))
    ));

    let mut missing_postcondition = original.clone();
    assert_eq!(
        missing_postcondition
            .rust
            .contents
            .matches("assert!(state_post,")
            .count(),
        1
    );
    missing_postcondition.rust.contents =
        missing_postcondition
            .rust
            .contents
            .replacen("assert!(state_post,", "assert!(true,", 1);
    assert!(matches!(
        Claim::state_transition(&missing_postcondition),
        Err(Refusal::InvalidBuild(_))
    ));
}

#[trace("TC-056", "FR-041-AC-10")]
#[test]
fn unsupported_boolean_preconditions_emit_no_transition_harness() {
    for pre in ["", "pre P0 using v on M::Switch::set { true }\n"] {
        let package = admitted(pre, "unsupported-boolean-transition");
        let outcome = generate(&package, &selected(&package));
        let KaniObligationOutcome::Emitted {
            records,
            state_transition_harnesses,
            ..
        } = outcome
        else {
            panic!("an admitted unsupported profile is accounted per item");
        };
        assert!(state_transition_harnesses.is_empty());
        assert_eq!(records.len(), 1);
        assert!(matches!(
            records[0].disposition,
            quire_contract_codegen::ObligationDisposition::Unsupported { .. }
        ));
    }
}

#[trace("TC-056", "FR-041-AC-10")]
#[test]
fn valid_transition_body_edits_and_subject_bindings_change_proof_identity() {
    let original = admitted(PRECONDITION, "transition-identity");
    let changed = admitted(
        "pre P0 using v on M::Switch::set { self.on or flag }\n",
        "transition-identity",
    );
    let before = harness(&original);
    let after = harness(&changed);
    assert_eq!(
        before.identity.transition, after.identity.transition,
        "the same structural transition key cannot substitute for its semantic body"
    );
    assert_ne!(
        before.identity.semantic_digest,
        after.identity.semantic_digest
    );
    assert_ne!(before.identity.module_symbol, after.identity.module_symbol);
    assert_ne!(before.rust.path, after.rust.path);

    let selected = selected(&original);
    let changed_subject = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: before.identity.ceilings,
        items: &[ObligationItem::StateTransition {
            package: &original,
            transition: &selected,
            subject_path: "crate::different_update",
        }],
        subject_path: "crate::unused_subject",
        unwind: 3,
    })
    .unwrap();
    let KaniObligationOutcome::Emitted {
        state_transition_harnesses,
        ..
    } = changed_subject
    else {
        panic!("another well-formed actual subject binding is generated");
    };
    let [changed_subject] = state_transition_harnesses.as_slice() else {
        panic!("one selected transition yields one harness");
    };
    assert_ne!(
        before.identity.module_symbol,
        changed_subject.identity.module_symbol
    );
    assert!(changed_subject
        .rust
        .contents
        .contains("crate::different_update(input_current, &mut state_post)"));
}

pub(super) fn qualify() {
    use super::census::{controls, Claim, Compiled, Control};
    use quire_contract_codegen::{
        execute_kani_obligation, Artifact, KaniExecutionRequest, KaniInstallation, KaniRunOutcome,
    };
    use std::collections::BTreeSet;

    let package = admitted(PRECONDITION, "qualified-boolean-transition");
    let mut healthy = harness(&package);
    healthy.rust.contents.insert_str(0,
        "#[inline(never)]\nfn update_switch(input_current: bool, state: &mut bool) { *state = input_current || *state; }\n");
    let declared = Claim::state_transition(&healthy).unwrap();
    let manifest = Artifact::new("Cargo.toml", "[package]\nname = \"boolean-transition-control\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[workspace]\n");
    let run = |harness: &KaniStateTransitionHarness, name: &str| {
        let scratch =
            crate::kani_obligations::write_scalar_crate(name, &manifest, &harness.rust.contents);
        crate::scratch_crate::resolve_seeded_lock(&scratch.path);
        let target = crate::scratch_crate::TemporaryDirectory::new(&format!("transition-{name}"));
        let installation = KaniInstallation::discover().expect("installed Kani");
        let cancel = quire_exact::Cancel::new();
        let evidence = execute_kani_obligation(&KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: harness.into(),
            crate_directory: &scratch.path,
            target_directory: &target.path,
            home_directory: None,
        })
        .expect("actual bounded transition proof execution");
        let claim = Claim::state_transition(harness).unwrap();
        let compiled = Compiled::read(
            &target.path,
            &claim,
            std::path::Path::new(&evidence.launcher_path),
        )
        .expect("fresh compiled transition module and named roots");
        let retained = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/current-transition-attempts")
            .join(name);
        fs::create_dir_all(&retained).unwrap();
        for file in [
            "compiled-census.json",
            "compiled-call-graph.txt",
            "compiled-symbols.json",
        ] {
            fs::copy(target.path.join(file), retained.join(file)).unwrap();
        }
        for file in ["Cargo.toml", "Cargo.lock", "src/lib.rs"] {
            fs::copy(
                scratch.path.join(file),
                retained.join(file.replace('/', "-")),
            )
            .unwrap();
        }
        fs::write(
            retained.join("execution.json"),
            serde_json::to_vec_pretty(&evidence).unwrap(),
        )
        .unwrap();
        (evidence, compiled, retained)
    };
    let (positive, compiled, retained) = run(&healthy, "healthy");
    compiled.require_positive(&declared, &positive).unwrap();
    let shared = compiled.shared_helpers(&declared).unwrap();
    let reviewed = BTreeSet::new();
    assert_eq!(
        shared, reviewed,
        "every computed shared helper needs its own live controls"
    );
    fs::write(
        retained.join("shared-helper-evidence.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "observed_helpers": shared, "reviewed_helpers": reviewed,
            "module": declared.module, "harness": declared.harness,
        }))
        .unwrap(),
    )
    .unwrap();
    for (name, before, after) in [
        ("false-post", "state_post\n}", "false\n}"),
        (
            "suppressed-update",
            "crate::update_switch(input_current, &mut state_post);",
            "let _ = input_current;",
        ),
        (
            "missing-precondition",
            "kani::assume(input_current || state_pre);",
            "let _ = (input_current, state_pre);",
        ),
    ] {
        assert_eq!(
            healthy.rust.contents.matches(before).count(),
            1,
            "one exact mutation site"
        );
        let mut mutant = healthy.clone();
        mutant.rust.contents = mutant.rust.contents.replacen(before, after, 1);
        let (failed, mutant_compiled, retained) = run(&mutant, name);
        let mutant_claim = Claim::state_transition(&mutant).unwrap();
        mutant_compiled
            .require_failure(&mutant_claim, &failed)
            .unwrap();
        super::retain_mutation(&retained, &healthy.rust.contents, &mutant.rust.contents);
        let KaniRunOutcome::Falsified { counterexample } = &failed.outcome else {
            panic!("each control must execute and falsify its own postcondition");
        };
        let values = crate::kani_obligations::boolean_playback_inputs(counterexample.as_str());
        match name {
            "suppressed-update" => assert_eq!(values, [true, false]),
            "missing-precondition" => assert_eq!(values, [false, false]),
            "false-post" => assert!(values[0] || values[1], "the actual precondition holds"),
            _ => unreachable!(),
        }
        controls(
            std::slice::from_ref(&declared),
            &shared,
            &reviewed,
            &[Control {
                name: &declared.module,
                before: &healthy.rust.contents,
                after: &mutant.rust.contents,
                positive: &positive,
                mutant: &failed,
            }],
        )
        .unwrap();
    }
}
