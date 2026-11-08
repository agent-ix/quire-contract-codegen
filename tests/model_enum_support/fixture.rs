// SPDX-License-Identifier: AGPL-3.0-or-later
//! Genuine model enum values reconstructed from native-admitted equality witnesses.

use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use quire_semantic_value::enumeration::{EnumDeclaration, EnumValue};
use std::collections::BTreeMap;

pub fn compile(
    source: &str,
    document: &str,
) -> (qsl_replay::AdmittedCompilation, CheckedPackageV2) {
    let compiled = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("agent-ix", "enum-comparison"),
            package_name: "agent-ix/enum-comparison".to_owned(),
            path: "enum-comparison.native",
            bytes: source.as_bytes(),
        },
        [document.as_bytes()],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("the original QSpec color source and model compile");
    let wire: serde_json::Value =
        serde_json::from_slice(compiled.package().bytes()).expect("original emission JSON");
    let mut evidence = CheckedPackageEvidence::new();
    for feature in wire["lock"]["required_features"]
        .as_array()
        .expect("features")
    {
        evidence.support_feature(feature.as_str().expect("feature"));
    }
    evidence
        .insert_domain_package_document("acme/colors", document.as_bytes())
        .expect("the original selected document is valid evidence");
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        compiled.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("the original emitted model enum package must admit");
    };
    (compiled, *package)
}

pub fn colors(
    compiled: &qsl_replay::AdmittedCompilation,
    package: &CheckedPackageV2,
) -> BTreeMap<String, EnumValue> {
    let term_data = package
        .checked_term_data()
        .expect("the admitted model enum package retains validated equality term data");
    let mut witnesses = BTreeMap::new();
    for decision in &term_data.equality_decisions {
        for member in &decision.enum_members {
            assert!(!member.ordered, "model variants are unordered");
            match witnesses.insert(member.case.to_string(), member) {
                None => {}
                Some(previous) => assert_eq!(previous, member, "one original identity per case"),
            }
        }
    }
    assert_eq!(
        witnesses.keys().map(String::as_str).collect::<Vec<_>>(),
        ["Green", "Red"]
    );
    let first = witnesses["Red"];
    let declared = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.node_id == first.enum_type)
        .expect("original enum declaration");
    assert_eq!(
        (&*declared.node_tag, &*declared.semantic_form),
        ("model", "variant_type")
    );
    let graph = compiled.context().wire_graph();
    let declaration = EnumDeclaration::new(
        graph
            .key(&declared.node_id)
            .expect("producer's original model declaration key"),
        false,
        witnesses.keys().cloned().collect(),
    )
    .expect("the admitted selected enum declaration reconstructs");
    witnesses
        .into_iter()
        .map(|(case, witness)| {
            assert_eq!(witness.enum_type, first.enum_type);
            let member = package
                .resolved_graph()
                .nodes
                .iter()
                .find(|node| node.node_id == witness.member)
                .expect("selected enum member witness");
            assert_eq!(
                (&*member.node_tag, &*member.semantic_form),
                ("value", "enum_value")
            );
            assert_eq!(member.semantic_type, declared.node_id);
            let value = declaration
                .member(
                    &case,
                    graph
                        .key(&witness.member)
                        .expect("producer's selected member key"),
                )
                .expect("the original admitted member belongs to the selected declaration");
            (case, value)
        })
        .collect()
}

pub fn equality_application(package: &CheckedPackageV2, function: &str) -> CheckedNodeId {
    let function = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration.as_ref().is_some_and(|declaration| {
                declaration.qualified_name == [Box::<str>::from(function)]
            })
        })
        .expect("the authored function declaration");
    let regions = package
        .source_map()
        .iter()
        .filter(|entry| entry.node_id == function.node_id)
        .flat_map(|entry| &entry.regions)
        .collect::<Vec<_>>();
    package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "binary"
                && package
                    .source_map()
                    .iter()
                    .filter(|entry| entry.node_id == node.node_id)
                    .flat_map(|entry| &entry.regions)
                    .any(|region| {
                        regions.iter().any(|outer| {
                            outer.source == region.source
                                && outer.start <= region.start
                                && region.end <= outer.end
                        })
                    })
        })
        .expect("the function's original equality application")
        .node_id
        .clone()
}
