//! Authentic source admission helpers; no prebaked graph or minted node identities.

use quire_contract_model::{CheckedNodeId, ContractPackageV1};

pub(super) const HEALTHY: &str = include_str!("fixtures/commutative.native");
pub(super) const MUTANT: &str = include_str!("fixtures/wrong_addend.native");

pub(super) fn admit(source: &str) -> (ContractPackageV1, CheckedNodeId) {
    use qsl_replay::{
        compile_admitted, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
    };
    use quire_contract_model::{
        CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
        CheckedPackageV2ReadResult,
    };
    let admitted = compile_admitted(
        SourceIdentity::new("a", "u", "git", "1"),
        "rational-predicate",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("authentic QSL source admits");
    let wire: serde_json::Value = serde_json::from_slice(admitted.package().bytes()).unwrap();
    let mut evidence = CheckedPackageEvidence::new();
    for feature in wire["lock"]["required_features"].as_array().unwrap() {
        evidence.support_feature(feature.as_str().unwrap());
    }
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        admitted.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("strict owner reader must admit the source output");
    };
    let function = package
        .graph()
        .nodes
        .iter()
        .find(|n| n.node_tag.as_ref() == "function")
        .unwrap()
        .node_id
        .clone();
    let lowered = package.lower(
        std::slice::from_ref(&function),
        &quire_contract_codegen::routed_contract_lowering_profile(),
    );
    (
        lowered
            .package
            .wire()
            .expect("owner function closure lowers")
            .clone(),
        function,
    )
}

pub(super) fn run_native(sources: &[(&str, &str)], harness: &str) -> String {
    use crate::scratch_crate::{write_manifest, TemporaryDirectory};
    use std::{fs, process::Command};
    let directory = TemporaryDirectory::new("ir258-rational-native-body");
    let dependency = include_str!("../../../Cargo.toml")
        .lines()
        .find(|line| line.starts_with("quire-exact = "))
        .expect("root exact dependency");
    let dev_dependency = dependency.replacen(" }", ", features = [\"test-support\"] }", 1);
    let manifest=format!("[package]\nname=\"ir258-rational-native-body\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n{dependency}\n[dev-dependencies]\n{dev_dependency}\n[workspace]\n");
    write_manifest(&directory.path, &manifest);
    for (name, source) in sources {
        fs::write(directory.path.join(format!("src/{name}.rs")), source).unwrap();
    }
    let modules = sources
        .iter()
        .map(|(name, _)| format!("pub mod {name};\n"))
        .collect::<String>();
    fs::write(
        directory.path.join("src/lib.rs"),
        format!("{modules}\n#[cfg(test)] mod reference;\n"),
    )
    .unwrap();
    fs::write(directory.path.join("src/reference.rs"), harness).unwrap();
    // Adapt only the root-seeded graph offline, then prove every retained Git source/version
    // is still a member of CG's root cohort before compiling the generated native body.
    let metadata = Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .current_dir(&directory.path)
        .output()
        .unwrap();
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout).unwrap();
    let root = include_str!("../../../Cargo.lock");
    for package in metadata["packages"].as_array().unwrap() {
        let Some(source) = package["source"].as_str().filter(|s| s.starts_with("git+")) else {
            continue;
        };
        let name = package["name"].as_str().unwrap();
        let version = package["version"].as_str().unwrap();
        assert!(
            root.split("[[package]]").any(|block| block
                .lines()
                .any(|l| l == format!("name = {name:?}"))
                && block.lines().any(|l| l == format!("version = {version:?}"))
                && block.lines().any(|l| l == format!("source = {source:?}"))),
            "native cohort drift: {name}"
        );
    }
    let output = Command::new("cargo")
        .args([
            "test",
            "--locked",
            "--offline",
            "-j2",
            "--lib",
            "--",
            "--nocapture",
        ])
        .current_dir(&directory.path)
        .env("CARGO_TARGET_DIR", directory.path.join("target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    eprintln!("{stdout}");
    stdout
}

pub(super) fn source_parameters(source: &str, function: &str) -> Vec<(String, String)> {
    use qsl_replay::{call_site, DependencyInput, Identifier, QualifiedName, SourceIdentity};
    let selection = QualifiedName::new(vec![Identifier::new(function).unwrap()]).unwrap();
    let site = call_site(
        SourceIdentity::new("a", "u", "git", "1"),
        "rational-predicate",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        &selection,
    )
    .expect("independent public owner function selection");
    let mut parameters = site
        .site
        .parameters
        .iter()
        .map(|(name, node)| (name.as_str().to_owned(), node.to_string()))
        .collect::<Vec<_>>();
    parameters.sort_by(|left, right| left.1.cmp(&right.1));
    parameters
}
