//! FR-033-AC-18 native artifact authentication. Native Boolean execution is not
//! backend proof, shadow evidence or an invented occurrence-pair count (IR-806).
use super::{admitted_enum_equality, composite_equality_agreement, composite_original_eq};
use ix_trace_rs::trace;
use qsl_replay::{EqualityOperator, ReplayLimits, WitnessValue};
use quire_contract_codegen::{
    CompositeBuildError, CompositeDriverObservation, CompositeEqualityOracles, LockedSource,
    OriginalCompositeArtifactBinding, OriginalCompositeEqContext, ReplayInputs,
    WitnessDecodeLimits,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageReadLimits,
    CheckedPackageV2,
};
use quire_exact::Outcome;
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

fn source() -> LockedSource {
    LockedSource {
        authority: "test".to_owned(),
        identity: "composite-agreement.native".to_owned(),
        namespace: "git".to_owned(),
        revision: "fixture".to_owned(),
        bytes: composite_original_eq::SOURCE.as_bytes().to_vec(),
    }
}
fn limits() -> quire_exact::ScalarLimits {
    quire_exact::ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    }
}
fn context(package: &CheckedPackageV2) -> OriginalCompositeEqContext {
    OriginalCompositeEqContext::new(
        package.clone(),
        &source(),
        ReplayInputs {
            decode_limits: WitnessDecodeLimits {
                bytes: 8 * 1024 * 1024,
                values: 2,
            },
            package_name: "composite-agreement".to_owned(),
            source: source(),
            dependencies: vec![],
            accounting_limits: limits(),
            stage_limits: BTreeMap::new(),
            declared_domains: vec![],
            replay_limits: ReplayLimits::default(),
        },
        "f",
        CheckedPackageReadLimits::bounded(),
    )
    .unwrap()
}
fn binding(
    package: &CheckedPackageV2,
    generated: &CompositeEqualityOracles,
    node: &CheckedNodeId,
    occurrence: &CheckedOccurrence,
) -> Result<OriginalCompositeArtifactBinding, CompositeBuildError> {
    context(package).retain_generated_artifact(
        generated,
        node,
        occurrence,
        EqualityOperator::Equal,
        &composite_original_eq::parameter_bounds(package, node, occurrence),
        [
            WitnessValue::Sequence(vec![WitnessValue::Integer(1), WitnessValue::Integer(2)]),
            WitnessValue::Sequence(vec![WitnessValue::Integer(1), WitnessValue::Integer(2)]),
        ],
        100_000,
    )
}
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn observe(
    package: &CheckedPackageV2,
    binding: &OriginalCompositeArtifactBinding,
    mutant: bool,
) -> CompositeDriverObservation {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let directory = target.join(format!(
        "ir1184-composite-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let scratch = Scratch(directory);
    for artifact in binding.artifacts() {
        let path = scratch.0.join(&artifact.path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, &artifact.contents).unwrap();
    }
    if mutant {
        let path = scratch.0.join("src/lib.rs");
        let original = fs::read_to_string(&path).unwrap();
        let changed = original.replace(
            "sv::EqualityOperator::Equal",
            "sv::EqualityOperator::NotEqual",
        );
        assert_ne!(
            original, changed,
            "mutate the actual emitted comparison, never regenerate over it"
        );
        fs::write(path, changed).unwrap();
    }
    fs::write(scratch.0.join("original.native"), &source().bytes).unwrap();
    let program = format!(
        r#"#[path="src/lib.rs"] mod generated;
use quire_exact as exact;
use exact::{{ScalarLimits, Integer, IntegerInterval, Value, ValueType, CollectionType, CollectionKind, CardinalityBound, Meter}};
fn main() {{
 let source=include_bytes!("original.native");
 let compiled=qsl_replay::compile_admitted(qsl_replay::CompileUnit {{ source:qsl_replay::SourceIdentity::new("test","composite-agreement.native","git","fixture"),package_name:"composite-agreement".to_owned(),path:"composite-agreement.native",bytes:source }},[],&qsl_replay::DependencyInput::default(),qsl_replay::StageLimits::default(),qsl_replay::ReplayLimits::default()).expect("same actual original context");
 let environment=generated::{}(Some(compiled.context())).expect("original generated environment");
 let limits={:?};
 let ty=CollectionType::new(CollectionKind::Sequence,ValueType::Int(IntegerInterval::new(Integer::from(0_i64),Integer::from(9_i64)).unwrap()),Some(CardinalityBound::new(0,3).unwrap()));
 let value=exact::form_collection(&ty,vec![Value::Integer(Integer::from(1_i64)),Value::Integer(Integer::from(2_i64))],&mut Meter::new(limits)).unwrap().completed().unwrap();
 let outcome=generated::{}(&environment,&value,&value,&mut Meter::new(limits));
 match outcome {{ Outcome::Completed(value)=>println!("native:{{value}}"), other=>panic!("native stopped: {{other:?}}") }}
}}"#,
        binding.environment_symbol(),
        binding.limits(),
        binding.oracle_symbol()
    );
    let program = program.replace(
        "match outcome { Outcome::",
        "match outcome { exact::Outcome::",
    );
    fs::write(scratch.0.join("driver.rs"), program).unwrap();
    let deps = target.join("debug/deps");
    let mut command = Command::new("timeout");
    command
        .args(["180", "rustc", "--edition=2021", "--cap-lints=allow"])
        .arg(scratch.0.join("driver.rs"))
        .arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg("-o")
        .arg(scratch.0.join("driver"));
    for name in [
        "quire_exact",
        "quire_semantic_value",
        "quire_contract_model",
        "qsl_replay",
        "quire_contract_runtime",
        "serde_json",
        "quire_canonical",
    ] {
        let library = fs::read_dir(&deps)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(&format!("lib{name}-"))
                    && path
                        .extension()
                        .is_some_and(|extension| extension == "rlib")
            })
            .max_by_key(|path| fs::metadata(path).unwrap().modified().unwrap())
            .unwrap_or_else(|| panic!("compiled owner dependency {name}"));
        command
            .arg("--extern")
            .arg(format!("{name}={}", library.display()));
    }
    let compiled = command.output().unwrap();
    assert!(
        compiled.status.success(),
        "compile actual emitted composite bytes: {}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let executed = Command::new("timeout")
        .arg("60")
        .arg(scratch.0.join("driver"))
        .output()
        .unwrap();
    assert!(
        executed.status.success(),
        "execute actual emitted composite oracle: {}",
        String::from_utf8_lossy(&executed.stderr)
    );
    let stdout = String::from_utf8(executed.stdout).unwrap();
    let outcome = match stdout.trim() {
        "native:true" => Outcome::Completed(true),
        "native:false" => Outcome::Completed(false),
        other => panic!("actual native result {other}"),
    };
    let mut actual_source = source();
    actual_source.bytes = fs::read(scratch.0.join("original.native")).unwrap();
    CompositeDriverObservation {
        staged_entries: binding
            .artifacts()
            .iter()
            .map(|artifact| {
                (
                    artifact.path.clone(),
                    fs::read(scratch.0.join(&artifact.path)).unwrap(),
                )
            })
            .collect(),
        oracle_symbol: binding.oracle_symbol().to_owned(),
        environment_symbol: binding.environment_symbol().to_owned(),
        operands: binding.operands().clone(),
        limits: binding.limits(),
        source: actual_source,
        package: package.clone(),
        outcome,
    }
}

#[trace("TC-048", "FR-033-AC-18")]
#[test]
fn original_composite_body_and_actual_emitted_bytes_have_independent_authority() {
    let compiled =
        composite_equality_agreement::fixture::compile_source(composite_original_eq::SOURCE);
    let package = admitted_enum_equality::checked(&compiled, &[]);
    let item = admitted_enum_equality::direct_source_item(&package, "f");
    let node = item.node_id.clone();
    let generated =
        admitted_enum_equality::generate_items(&package, Some(compiled.context()), &[item], &[])
            .unwrap();
    let node_record = package
        .graph()
        .nodes
        .iter()
        .find(|entry| entry.node_id == node)
        .unwrap();
    let mut occurrences = node_record
        .occurrences
        .iter()
        .filter(|entry| entry.role == CheckedOccurrenceRole::Expression);
    let occurrence = occurrences.next().unwrap().clone();
    assert!(occurrences.next().is_none());
    let original = binding(&package, &generated, &node, &occurrence).unwrap();
    let observed = observe(&package, &original, false);
    assert_eq!(observed.outcome, Outcome::Completed(true));
    let body = original.body().clone();
    let authenticated = original.complete(&body, observed.clone()).unwrap();
    assert_eq!(authenticated.native_outcome(), &Outcome::Completed(true));
    let (wire, claim, replay_limits) = authenticated.request();
    authenticated
        .check_sent_context(&wire, replay_limits)
        .unwrap();
    // Supplied verified evidence is independent of native execution. This real
    // facade call supplies no generated backend proof or native pair-count credit.
    let report = qsl_replay::settle_verified_shadow(
        wire.clone(),
        claim,
        qsl_replay::VerifiedShadow {
            success_checks: 1,
            refinement: qsl_replay::Refinement::NotExhausted,
        },
        replay_limits,
    );
    assert!(matches!(
        report.result(),
        qsl_replay::VerifiedShadowResult::Tested
    ));
    for member in 0..4 {
        let held = binding(&package, &generated, &node, &occurrence).unwrap();
        let mut changed = held.body().clone();
        let original_node = held.body().node.clone();
        match member {
            0 => {
                changed.node = quire_exact::NodeKey::new(
                    "other-package",
                    original_node.path(),
                    original_node.ordinal(),
                )
                .unwrap()
            }
            1 => {
                let mut path = original_node.path();
                path.push(quire_exact::PathSegment::Text("other-path".to_owned()));
                changed.node = quire_exact::NodeKey::new(
                    original_node.package(),
                    path,
                    original_node.ordinal(),
                )
                .unwrap();
            }
            2 => {
                changed.node = quire_exact::NodeKey::new(
                    original_node.package(),
                    original_node.path(),
                    original_node.ordinal() + 1,
                )
                .unwrap()
            }
            3 => changed.obligation_kind = "other_opaque_kind".to_owned(),
            _ => unreachable!(),
        }
        let original_identity = qsl_replay::parity_obligation(held.body()).unwrap();
        let changed_identity = qsl_replay::parity_obligation(&changed).unwrap();
        if member == 3 {
            assert_eq!(
                changed_identity, original_identity,
                "body-only edit leaves the admitted structural identity unchanged"
            );
        } else {
            assert_ne!(
                changed_identity, original_identity,
                "each changed structural component changes the identity"
            );
        }

        assert!(
            matches!(
                held.complete(&changed, observed.clone()),
                Err(CompositeBuildError::RetainedBodyMismatch)
            ),
            "body member {member}"
        );
    }
    let changed_bounds = binding(&package, &generated, &node, &occurrence).unwrap();
    let mut reordered_body = changed_bounds.body().clone();
    reordered_body.arguments.swap(0, 1);
    assert_eq!(
        qsl_replay::parity_obligation(&reordered_body).unwrap(),
        qsl_replay::parity_obligation(changed_bounds.body()).unwrap()
    );
    assert!(matches!(
        changed_bounds.complete(&reordered_body, observed.clone()),
        Err(CompositeBuildError::RetainedBodyMismatch)
    ));
    for member in 0..6 {
        let retained = binding(&package, &generated, &node, &occurrence).unwrap();
        let original = retained.body().clone();
        let mut changed = observed.clone();
        match member {
            0 => changed.operands.swap(0, 1),
            1 => changed.limits.work_units -= 1,
            2 => changed.source.bytes.push(b' '),
            3 => changed.oracle_symbol.push_str("_other"),
            4 => changed.environment_symbol.push_str("_other"),
            5 => changed.staged_entries[0].1.push(0),
            _ => unreachable!(),
        }
        // Equal operands have equal bytes: use a genuine distinct supplied
        // value for this adverse observation rather than pretend swapping changed it.
        if member == 0 {
            changed.operands[0] = WitnessValue::Sequence(vec![WitnessValue::Integer(2)]);
        }
        assert!(
            matches!(
                retained.complete(&original, changed),
                Err(CompositeBuildError::GeneratedArtifactChanged)
            ),
            "independent native tie {member}"
        );
    }
    let mutant_binding = binding(&package, &generated, &node, &occurrence).unwrap();
    let mutant = observe(&package, &mutant_binding, true);
    assert_eq!(
        mutant.outcome,
        Outcome::Completed(false),
        "real same-artifact mutation changed native execution"
    );
    let original_body = mutant_binding.body().clone();
    assert!(matches!(
        mutant_binding.complete(&original_body, mutant),
        Err(CompositeBuildError::GeneratedArtifactChanged)
    ));
    let mut stale = generated.clone();
    stale.artifacts[0].contents.push(' ');
    assert!(matches!(
        binding(&package, &stale, &node, &occurrence),
        Err(CompositeBuildError::GeneratedArtifactChanged)
    ));
    let mut changed_wire = wire;
    changed_wire.byte_provision[0].2.push(b' ');
    assert!(matches!(
        authenticated.check_sent_context(&changed_wire, replay_limits),
        Err(CompositeBuildError::ContextMismatch)
    ));
}
