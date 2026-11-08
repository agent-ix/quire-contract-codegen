//! Driver-owned native scalar controls: no Kani proof or backend falsification credit.
use super::scalar_playback::{
    replay_fixture, replay_fixture_with_extra, transcript, ReplayFixture,
};
use ix_trace_rs::trace;
use qsl_replay::{NativeOutcome, OperatorParityResult, ReplayLimits};
use quire_contract_codegen::{
    LockedSource, OriginalScalarContext, ReplayInputs, ScalarBuildError, ScalarDriverObservation,
    ScalarReplayPlan, ScalarSentRequest, WitnessDecodeLimits, SCALAR_PLAYBACK_BYTES,
};
use quire_contract_model::CheckedPackageReadLimits;
use quire_exact::{Integer, IntegerInterval};
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

fn source(fixture: &ReplayFixture) -> LockedSource {
    LockedSource {
        authority: "test".to_owned(),
        identity: "scalar-playback.native".to_owned(),
        bytes: fixture.source.as_bytes().to_vec(),
    }
}
pub(super) fn context(fixture: &ReplayFixture) -> OriginalScalarContext {
    let source = source(fixture);
    OriginalScalarContext::new(
        fixture.checked.clone(),
        fixture.raw_package.clone(),
        &source,
        ReplayInputs {
            decode_limits: WitnessDecodeLimits {
                bytes: SCALAR_PLAYBACK_BYTES,
                values: 2,
            },
            package_name: "scalar-playback".to_owned(),
            source: source.clone(),
            dependencies: vec![],
            accounting_limits: fixture.harness.original_scalar_limits().unwrap(),
            stage_limits: BTreeMap::new(),
            declared_domains: vec![],
            replay_limits: ReplayLimits::default(),
        },
        "subject",
        CheckedPackageReadLimits::bounded(),
    )
    .expect("original source context")
}
pub(super) fn plan(fixture: &ReplayFixture, values: &[i64]) -> ScalarReplayPlan {
    context(fixture)
        .plan(
            &fixture.harness,
            &fixture.harness.identity,
            fixture
                .harness
                .scalar_replay_body()
                .expect("actual admitted positional body"),
            &fixture.artifacts,
            &transcript(&fixture.harness.identity, values),
        )
        .expect("original scalar plan")
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// This is a driver, outside CG's builder. It stages the retained bytes, compiles
// the actual staged oracle, executes it at the decoded vector and retained limits,
// then independently reads back every original staged path. No expected outcome
// is supplied to this helper, and mutation is never overwritten by regeneration.
pub(super) fn observe(
    fixture: &ReplayFixture,
    plan: &ScalarReplayPlan,
    mutate: bool,
) -> ScalarDriverObservation {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let directory = target.join(format!(
        "ir1184-native-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let scratch = Scratch(directory);
    for (path, bytes) in plan.staged_entries() {
        let destination = scratch.0.join(path);
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::write(destination, bytes).unwrap();
    }
    if mutate {
        let path = scratch.0.join("src/lib.rs");
        let original = fs::read_to_string(&path).unwrap();
        let changed = original.replace(
            "rt::IntegerArithmetic::Add",
            "rt::IntegerArithmetic::Subtract",
        );
        assert_ne!(changed, original, "kill a real generated arithmetic body");
        fs::write(path, changed).unwrap();
    }
    let deps = target.join("debug/deps");
    let exact = super::native_driver_artifacts::dependency("quire_exact");
    let arguments = plan
        .operands()
        .iter()
        .map(|value| format!("&rt::Integer::from({value}_i64)"))
        .collect::<Vec<_>>()
        .join(", ");
    let main = format!("#[path=\"src/lib.rs\"] mod generated;\nuse quire_exact as rt;\nuse rt::ScalarLimits;\nfn main() {{ let mut meter = rt::Meter::new({:?});\n let outcome = generated::{}({arguments}, &mut meter).expect(\"actual generated oracle\");\n match outcome {{ rt::Outcome::Completed(value) => println!(\"completed:{{value}}\"), rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain {{..}}) => println!(\"out_of_range\"), rt::Outcome::Undefined(_) => println!(\"undefined\"), rt::Outcome::Incomplete(_) => println!(\"incomplete\"), other => panic!(\"unexpected native outcome: {{other:?}}\") }} }}", plan.scalar_limits(), fixture.harness.identity.oracle_symbol);
    fs::write(scratch.0.join("driver.rs"), main).unwrap();
    let binary = scratch.0.join("driver");
    let mut compiler = Command::new("timeout");
    super::native_driver_artifacts::compiler(&mut compiler);
    let compile = compiler
        .args(["120", "rustc", "--edition=2021", "--cap-lints=allow"])
        .arg(scratch.0.join("driver.rs"))
        .arg("--extern")
        .arg(format!("quire_exact={}", exact.display()))
        .arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "compile the original staged artifact: {}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let execution = Command::new("timeout")
        .arg("30")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        execution.status.success(),
        "native execution: {}",
        String::from_utf8_lossy(&execution.stderr)
    );
    let output = String::from_utf8(execution.stdout).unwrap();
    let outcome = if let Some(value) = output.trim().strip_prefix("completed:") {
        NativeOutcome::Completed(value.parse::<Integer>().unwrap())
    } else {
        match output.trim() {
            "out_of_range" => NativeOutcome::RefusedOutOfRange,
            "undefined" => NativeOutcome::Undefined,
            "incomplete" => NativeOutcome::Incomplete,
            other => panic!("unknown actual driver output: {other}"),
        }
    };
    ScalarDriverObservation {
        staged_entries: plan
            .staged_entries()
            .iter()
            .map(|(path, _)| (path.clone(), fs::read(scratch.0.join(path)).unwrap()))
            .collect(),
        identity: fixture.harness.identity.clone(),
        harness: quire_contract_codegen::Artifact::new(
            fixture.harness.rust.path.clone(),
            fs::read_to_string(scratch.0.join("src/lib.rs")).unwrap(),
        ),
        record: quire_contract_codegen::Artifact::new(
            fixture.harness.record.path.clone(),
            fs::read_to_string(scratch.0.join(&fixture.harness.record.path)).unwrap(),
        ),
        operands: plan.operands().to_vec(),
        limits: plan.scalar_limits(),
        source: source(fixture),
        package: fixture.checked.clone(),
        outcome,
    }
}

#[trace(
    "TC-047",
    "FR-032-AC-15",
    "FR-032-AC-16",
    "FR-032-AC-17",
    "FR-032-AC-18"
)]
#[test]
fn original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl() {
    let fixture = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
    );
    let body = fixture.harness.scalar_replay_body().unwrap();
    let admitted = fixture.checked.resolved_graph().nodes.iter().find(|node| matches!(&node.body, quire_contract_model::CheckedTerm::Application { operation, .. } if operation.identity() == Some("quire.op.integer.add"))).unwrap();
    assert_eq!(
        body.node, admitted.node_id,
        "CG retains the actual admitted application"
    );
    assert_eq!(
        body.operands,
        fixture
            .checked
            .scalar_application_operands(&admitted.node_id, &body.occurrence)
            .unwrap(),
        "complete positional body comes independently from the IR reader"
    );
    assert_eq!(body.operands.len(), 2);
    assert_eq!(
        body.operands[0].child, body.operands[1].child,
        "two draws of the same parameter, never deduplicated"
    );
    assert!(fixture.harness.record.contents.contains("operator_parity"));
    let original = plan(&fixture, &[2, 4]);
    assert_eq!(original.operands(), [2, 4]);
    let observation = observe(&fixture, &original, false);
    assert_eq!(
        observation.outcome,
        NativeOutcome::Completed(Integer::from(6_i64))
    );
    let completed = original.complete(observation).unwrap();
    let (wire, claim, limits) = completed.request();
    let sent = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    let report = qsl_replay::replay_operator_parity(wire.clone(), claim.clone(), limits);
    assert!(
        matches!(report.result(), OperatorParityResult::Agrees { .. }),
        "real QSL result: {:?}",
        report.result()
    );
    assert_eq!(
        completed.settle(&sent, Some(&report)).unwrap(),
        report.terminal_value()
    );

    // Both are otherwise genuine membership-valid QSL claims. Structural
    // identity alone cannot authenticate the original range or opaque kind.
    let mut range = claim.clone();
    let qsl_replay::ScalarOperation::Add { left, .. } = &mut range.operation else {
        panic!("addition")
    };
    left.range = IntegerInterval::spanning(Integer::from(0_i64), Integer::from(8_i64));
    let range_report = qsl_replay::replay_operator_parity(wire.clone(), range, limits);
    assert!(matches!(
        range_report.result(),
        OperatorParityResult::Agrees { .. }
    ));
    assert!(matches!(
        completed.settle(&sent, Some(&range_report)),
        Err(ScalarBuildError::Report)
    ));
    let mut kind = claim.clone();
    kind.obligation_kind = "other_opaque_kind".to_owned();
    let kind_report = qsl_replay::replay_operator_parity(wire.clone(), kind, limits);
    assert!(matches!(
        kind_report.result(),
        OperatorParityResult::Agrees { .. }
    ));
    assert!(matches!(
        completed.settle(&sent, Some(&kind_report)),
        Err(ScalarBuildError::Report)
    ));

    let mut changed = body.clone();
    changed.operands[0].range.upper -= 1;
    assert!(matches!(
        context(&fixture).plan(
            &fixture.harness,
            &fixture.harness.identity,
            &changed,
            &fixture.artifacts,
            &transcript(&fixture.harness.identity, &[2, 4])
        ),
        Err(ScalarBuildError::BodyMismatch)
    ));
    let mutant_plan = plan(&fixture, &[4, 2]);
    let mutant = observe(&fixture, &mutant_plan, true);
    assert_ne!(
        mutant.outcome,
        NativeOutcome::Completed(Integer::from(6_i64)),
        "the actual compiled mutant changed the arithmetic"
    );
    assert!(matches!(
        mutant_plan.complete(mutant),
        Err(ScalarBuildError::Observation)
    ));
}

#[trace("TC-047", "FR-032-AC-6", "FR-032-AC-17", "FR-032-AC-18")]
#[test]
fn scalar_actual_sent_context_and_every_driver_tie_are_independently_checked() {
    let fixture = replay_fixture_with_extra(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
        "function elsewhere using v(): Int[0, 18] pure { 0 }\n",
    );
    let original = plan(&fixture, &[2, 4]);
    let observation = observe(&fixture, &original, false);
    let completed = original.complete(observation.clone()).unwrap();
    let (wire, claim, limits) = completed.request();
    let report = qsl_replay::replay_operator_parity(wire.clone(), claim.clone(), limits);
    assert!(matches!(
        report.result(),
        OperatorParityResult::Agrees { .. }
    ));
    let sent = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    assert!(
        completed.settle(&sent, None).is_err(),
        "absent report supplies no settlement"
    );
    let mut other_wire = wire.clone();
    other_wire.replay_target = qsl_replay::RequestTargetWire::function(
        qsl_replay::QualifiedName::new(vec![qsl_replay::Identifier::new("elsewhere").unwrap()])
            .unwrap(),
        qsl_replay::OccurrenceKey::new(claim.node.clone(), claim.occurrence.clone()),
    );
    let other_sent = ScalarSentRequest {
        wire: other_wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    let other_report = qsl_replay::replay_operator_parity(other_wire, claim.clone(), limits);
    assert_eq!(other_report.claim(), report.claim(), "the actual equal-claim/different-function report exposes why claim equality is insufficient");
    assert!(matches!(
        completed.settle(&other_sent, Some(&other_report)),
        Err(ScalarBuildError::SentContext)
    ));
    let smaller_limits = limits.with_input_bytes(limits.input_bytes - 1);
    let changed_budget_report =
        qsl_replay::replay_operator_parity(wire.clone(), claim.clone(), smaller_limits);
    let changed_budget = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: smaller_limits,
    };
    assert_eq!(changed_budget_report.claim(), report.claim());
    assert!(matches!(
        completed.settle(&changed_budget, Some(&changed_budget_report)),
        Err(ScalarBuildError::SentContext)
    ));
    let mut changed_bytes = wire.clone();
    changed_bytes.byte_provision[0].2.push(b' ');
    let changed_bytes_report =
        qsl_replay::replay_operator_parity(changed_bytes.clone(), claim.clone(), limits);
    assert_eq!(changed_bytes_report.claim(), report.claim());
    assert!(matches!(
        completed.settle(
            &ScalarSentRequest {
                wire: changed_bytes,
                claim: claim.clone(),
                replay_limits: limits
            },
            Some(&changed_bytes_report)
        ),
        Err(ScalarBuildError::SentContext)
    ));

    // Genuine facade reports over counterfactual claims exercise every outcome
    // class. These mutations are binding negatives, never fresh native executions.
    for member in 0..7 {
        let mut changed = claim.clone();
        match member {
            0 => changed.generated = NativeOutcome::Completed(Integer::from(7_i64)),
            1 => changed.generated = NativeOutcome::Incomplete,
            2 => changed.generated = NativeOutcome::ExecutionFault,
            3 => {
                let qsl_replay::ScalarOperation::Add { left, .. } = &mut changed.operation else {
                    panic!("addition")
                };
                left.value = 10;
            }
            4 => changed.limits.work_units = 0,
            5 => {
                changed.occurrence = qsl_replay::Origin::new(
                    qsl_replay::Role::new("expression"),
                    claim.occurrence.ordinal() + 1,
                )
            }
            6 => {
                changed.result_range =
                    IntegerInterval::spanning(Integer::from(0_i64), Integer::from(5_i64))
            }
            _ => unreachable!(),
        }
        let actual = ScalarSentRequest {
            wire: wire.clone(),
            claim: changed.clone(),
            replay_limits: limits,
        };
        let changed_report = qsl_replay::replay_operator_parity(wire.clone(), changed, limits);
        match member {
            0 => assert!(matches!(
                changed_report.result(),
                OperatorParityResult::Diverged { .. }
            )),
            1 | 2 => assert!(matches!(
                changed_report.result(),
                OperatorParityResult::GeneratedFault(_)
            )),
            3 => assert!(matches!(
                changed_report.result(),
                OperatorParityResult::RefusedInput(_)
            )),
            4 => assert!(matches!(
                changed_report.result(),
                OperatorParityResult::Incomplete(_)
            )),
            5 => assert!(matches!(
                changed_report.result(),
                OperatorParityResult::Refused(_)
            )),
            _ => {}
        }
        assert!(
            matches!(
                completed.settle(&actual, Some(&changed_report)),
                Err(ScalarBuildError::Report)
            ),
            "full actual-sent/report member {member}"
        );
    }

    // Mutate independently retained actual driver records after a real run.
    // These are adverse assertions, never claimed as fresh executions.
    for changed in 0..8 {
        let mut bad = observation.clone();
        match changed {
            0 => bad.staged_entries[0].1.push(0),
            1 => bad.identity.oracle_symbol.push_str("_other"),
            2 => bad.harness.contents.push(' '),
            3 => bad.record.contents.push(' '),
            4 => bad.operands.swap(0, 1),
            5 => bad.limits.work_units -= 1,
            6 => bad.source.bytes.push(b' '),
            7 => bad.source.identity.push_str("-other"),
            _ => unreachable!(),
        }
        assert!(
            matches!(
                plan(&fixture, &[2, 4]).complete(bad),
                Err(ScalarBuildError::Observation)
            ),
            "driver tie {changed}"
        );
    }
    for member in 0..3 {
        let mut identity = fixture.harness.identity.clone();
        match member {
            0 => identity.node_id.package = "other-package".into(),
            1 => identity
                .node_id
                .path
                .push(quire_contract_model::CheckedNodePathSegment::Name(
                    "other-path".into(),
                )),
            2 => identity.node_id.ordinal += 1,
            _ => unreachable!(),
        }
        assert!(
            matches!(
                context(&fixture).plan(
                    &fixture.harness,
                    &identity,
                    fixture.harness.scalar_replay_body().unwrap(),
                    &fixture.artifacts,
                    &transcript(&fixture.harness.identity, &[2, 4])
                ),
                Err(ScalarBuildError::BodyMismatch)
            ),
            "structural member {member}"
        );
    }
    let mut changed_artifacts = fixture.artifacts.clone();
    changed_artifacts[0].contents.push(' ');
    assert!(matches!(
        context(&fixture).plan(
            &fixture.harness,
            &fixture.harness.identity,
            fixture.harness.scalar_replay_body().unwrap(),
            &changed_artifacts,
            &transcript(&fixture.harness.identity, &[2, 4])
        ),
        Err(ScalarBuildError::Artifacts)
    ));
    assert_eq!(
        completed.settle(&sent, Some(&report)).unwrap(),
        report.terminal_value()
    );
}
