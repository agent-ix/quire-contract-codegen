// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authored Boolean transitions cross QSL emission and the public strict IR reader.

use std::{
    fs,
    num::NonZeroU64,
    path::PathBuf,
    process::Command,
    sync::OnceLock,
    time::Duration,
};

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
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
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
        .insert_domain_package_document("test/orders", &document)
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
    let KaniObligationOutcome::Emitted { mut state_transition_harnesses, .. } =
        generate(package, &selected(package))
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
    assert_eq!(generated, harness(&package), "identical inputs reproduce all bytes");
    let id = &generated.identity;
    let transition = package.graph().nodes.iter().find(|node| node.node_id == id.transition).unwrap();
    assert_eq!(id.occurrences, transition.occurrences);
    assert_eq!(id.source_map, package.source_map().iter().filter(|entry| entry.node_id == id.transition).cloned().collect::<Vec<_>>());
    assert!(!id.occurrences.is_empty());
    assert!(!id.source_map.is_empty());
    assert_eq!(id.field, "on");
    assert_eq!(id.parameter, "flag");
    assert_eq!(generated.rust.contents.matches("kani::any()").count(), 2);
    assert_eq!(generated.rust.contents.matches("kani::cover!(").count(), 1);
    assert!(generated.rust.contents.contains("crate::update_switch(input_current, &mut state_post)"));
    assert!(generated.rust.contents.contains("assert!(state_post,"));
    assert!(generated.rust.contents.contains("fn expected_boolean_state_transition"));
    assert!(generated.rust.contents.contains("input_current || state_pre"));
    assert!(!generated.rust.contents.contains("quire_exact"));
}

#[trace("TC-056", "FR-041-AC-10")]
#[test]
fn unsupported_boolean_preconditions_emit_no_transition_harness() {
    for pre in ["", "pre P0 using v on M::Switch::set { true }\n"] {
        let package = admitted(pre, "unsupported-boolean-transition");
        let outcome = generate(&package, &selected(&package));
        let KaniObligationOutcome::Emitted { records, state_transition_harnesses, .. } = outcome else {
            panic!("an admitted unsupported profile is accounted per item");
        };
        assert!(state_transition_harnesses.is_empty());
        assert_eq!(records.len(), 1);
        assert!(matches!(records[0].disposition, quire_contract_codegen::ObligationDisposition::Unsupported { .. }));
    }
}
