//! IR-1244 S1: the emitted Kani function harness rebuilds its prepared function from
//! host-emitted descriptors under `cfg(kani)`, compiled and run as a real scratch crate.
//!
//! `cfg(kani)` is applied to the emitted crate alone (`cargo rustc -- --cfg kani`), so the
//! dependencies are the normally built ones. The rebuilt environment, parameters, result
//! and location are compared by `Debug` with a host re-implementation of the genuine path: the
//! test assembles the environment from the same owner views itself. It does not run the emitted
//! `PreparedFunction::new`; that real host run is the embedded differential of FR-021-AC-45 (S2).
//! The environment limits are consumed by construction and are asserted on the emitted text in
//! the `kani_harness` unit tests.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use ix_trace_rs::trace;
use qsl_replay::{
    acquire_checked_closure, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_codegen::{
    checked_composite_declarations, checked_function_declarations, generate_kani_function_harness,
    select_checked_callable, CheckedFunctionResources, CheckedFunctionSource, KaniFunctionRequest,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};

const SAME: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function same using v(left: Int[0, 0], right: Int[0, 0]): Boolean pure { left = right }\n",
);

const COMPOSITE: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "tuple Pair(Boolean, Integer);\n",
    "union Choice { Empty, Some(Pair) }\n",
    "record Holder { pair: Pair; shape: Choice; flag: Boolean?; }\n",
    "function eq using v(x: Holder, y: Holder): Boolean pure { x = y }\n",
);

/// The host has no `kani` crate; this stands in for the function the emitted rebuild helper
/// calls, with an abort where Kani reports a failed assertion.
const HOST_KANI_STUB: &str = r#"
#[cfg(kani)]
mod kani {
    pub fn assert(condition: bool, _message: &'static str) {
        if !condition { std::process::abort() }
    }
}
"#;

const PATH: &str = "function-owner.native";

fn identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "function-owner")
}

/// What the genuine host path yields for `name`, as `Debug` text: environment, parameters, result.
fn host_prepared(
    source: &str,
    name: &str,
) -> (String, CheckedNodeId, Vec<u8>, CheckedPackageEvidence) {
    let compiled = acquire_checked_closure(
        CompileUnit {
            source: identity(),
            package_name: "function-owner".to_owned(),
            path: PATH,
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("genuine checked source");
    let bytes = compiled.package().bytes().to_vec();
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let CheckedPackageV2ReadResult::Admitted(package) =
        CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence)
    else {
        panic!("Contract IR admits the QSL emission");
    };
    let function = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| {
            node.node_tag.as_ref() == "function"
                && node
                    .declaration
                    .as_ref()
                    .is_some_and(|d| d.qualified_name == [Box::<str>::from(name)])
        })
        .expect("named function")
        .node_id
        .clone();
    let selection =
        select_checked_callable(&package, &package, &function, &compiled).expect("selection");
    let owner = checked_function_declarations(&package, &compiled, &selection).expect("owner");
    let resources = CheckedFunctionResources::default();
    let environment = quire_semantic_value::declaration::TypeEnvironment::bounded_with_cancel(
        checked_composite_declarations(&owner).expect("declarations"),
        [],
        resources.environment_limits,
        &resources.cancel,
    )
    .and_then(|mut environment| {
        for view in &owner {
            if view.form() == qsl_replay::NominalDeclarationForm::Union {
                let bindings = view
                    .members()
                    .iter()
                    .filter_map(|member| match member {
                        qsl_replay::NominalDeclarationMember::UnionMember { member, .. } => {
                            Some(member.clone())
                        }
                        _ => None,
                    })
                    .collect();
                environment.seal_union_verified(
                    view.key().clone(),
                    view.node().clone(),
                    bindings,
                )?;
            }
        }
        Ok(environment)
    });
    // Both functions of the fixtures are the first in their source.
    let location = quire_semantic_value::location::ExpressionLocation::root(
        quire_semantic_value::location::ExpressionOwner::Body {
            function: name.to_owned(),
            index: 0,
        },
    );
    let text = format!(
        "{environment:?}|{:?}|{:?}|{location:?}",
        selection.signature().parameters(),
        selection.signature().result()
    );
    (text, function, bytes, evidence)
}

/// Compile the emitted harness crate with `cfg(kani)` and return what its probe prints.
fn run_probe(label: &str, source: &str, name: &str, probe: &str) -> (String, String) {
    let (expected, function, bytes, evidence) = host_prepared(source, name);
    let dependencies = DependencyInput::default();
    let harness = generate_kani_function_harness(&KaniFunctionRequest {
        package_bytes: &bytes,
        read_limits: CheckedPackageReadLimits::bounded(),
        evidence: &evidence,
        source: CheckedFunctionSource {
            source: identity(),
            path: PATH,
            bytes: source.as_bytes(),
            packages: &[],
            dependencies: &dependencies,
            limits: StageLimits::default(),
            replay_limits: ReplayLimits::default(),
            body_resources: CheckedFunctionResources::default(),
        },
        function,
    })
    .expect("the harness generates");
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(label);
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("src")).expect("scratch directory");
    for artifact in &harness.artifacts {
        let contents = if artifact.path == "Cargo.toml" {
            // One package name per label, so a shared target never reuses a sibling's artifact.
            let manifest = artifact.contents.replacen(
                &format!(
                    "name = \"{}\"",
                    quire_contract_codegen::EXACT_FUNCTION_CRATE_NAME
                ),
                &format!("name = \"{label}\""),
                1,
            );
            manifest.replace(
                "quire-contract-codegen = { git = \"https://github.com/agent-ix/quire-contract-codegen\", branch = \"main\" }",
                &format!(
                    "quire-contract-codegen = {{ path = {:?} }}",
                    env!("CARGO_MANIFEST_DIR")
                ),
            )
        } else {
            format!("{}\n{HOST_KANI_STUB}\n{probe}\n", artifact.contents)
        };
        fs::write(root.join(&artifact.path), contents).expect("write the scratch file");
    }
    super::scratch_crate::resolve_seeded_lock(&root);
    let build = super::scratch_crate::lock_agreement_build();
    let compiled = Command::new(env!("CARGO"))
        .args([
            "rustc",
            "--lib",
            "--profile",
            "test",
            "--locked",
            "--offline",
            "--message-format=json",
            "--",
            "--cfg",
            "kani",
            "--check-cfg",
            "cfg(kani)",
        ])
        .env("CARGO_TARGET_DIR", super::scratch_crate::agreement_target())
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&root)
        .output()
        .expect("spawn cargo");
    assert!(
        compiled.status.success(),
        "the emitted harness must compile under cfg(kani):\n{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let executable = String::from_utf8_lossy(&compiled.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|message| message["reason"] == "compiler-artifact")
        .filter(|message| message["profile"]["test"] == true)
        .find_map(|message| message["executable"].as_str().map(str::to_owned))
        .expect("the test executable");
    let run = Command::new(executable)
        .args(["--nocapture", "--test-threads=1", "kani_probe"])
        .output()
        .expect("run the probe");
    drop(build);
    assert!(
        run.status.success(),
        "probe failed:\n{}{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout).into_owned();
    let observed = stdout
        .lines()
        .find_map(|line| line.split_once("PROBE:").map(|(_, rest)| rest))
        .unwrap_or_else(|| panic!("the probe prints its observation:\n{stdout}"))
        .to_owned();
    (expected, observed)
}

/// FR-021-AC-44: the `cfg(kani)` items compile, rebuild the environment, parameters and result
/// the genuine path yields for a scalar equality case, and the rebuilt body runs.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn rebuilt_scalar_equality_matches_the_genuine_path_and_runs() {
    let probe = r#"
#[cfg(kani)]
#[test]
fn kani_probe() {
    use quire_exact::{Integer, Meter, ScalarLimits};
    let factory = kani_factory_same();
    let prepared = factory.0.clone();
    assert!(prepared.ready().expect("ready").is_none());
    println!("PROBE:{:?}|{:?}|{:?}|{:?}", prepared.environment, prepared.parameters, prepared.result, prepared.location);
    let large = u64::from(u32::MAX);
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: large, decimal_digits: large, scale_expansion: large, text_input_bytes: large,
        text_scalars: large, normalized_scalars: large, unit_edges: large, value_occurrences: large,
        work_units: large, result_units: large,
    });
    let transition = factory.begin()
        .resume(Resume::Start(vec![Value::Integer(Integer::from(0_i64)), Value::Integer(Integer::from(0_i64))]), &mut meter)
        .expect("the rebuilt body runs");
    let Transition::Done(evaluation) = transition else { panic!("one step") };
    assert!(matches!(evaluation.outcome, FamilyOutcome::Evaluated(Outcome::Completed(Value::Boolean(true)))));
}
"#;
    let (expected, observed) = run_probe("kani-harness-same", SAME, "same", probe);
    assert_eq!(observed, expected);
    assert!(
        expected.contains("Int("),
        "the case has bounded parameters: {expected}"
    );
}

/// FR-021-AC-44: record, tuple and union declarations, with presence and the union seal, rebuild
/// to the environment the genuine path yields.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn rebuilt_composite_environment_matches_the_genuine_path() {
    let probe = r#"
#[cfg(kani)]
#[test]
fn kani_probe() {
    let factory = kani_factory_eq();
    let prepared = factory.0.clone();
    assert!(prepared.ready().expect("ready").is_none());
    println!("PROBE:{:?}|{:?}|{:?}|{:?}", prepared.environment, prepared.parameters, prepared.result, prepared.location);
}
"#;
    let (expected, observed) = run_probe("kani-harness-composite", COMPOSITE, "eq", probe);
    assert_eq!(observed, expected);
    assert!(
        expected.contains("Holder"),
        "the environment holds the declarations: {expected}"
    );
}
