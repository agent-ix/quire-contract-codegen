// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Admitted scalar source-selection work controls.

use super::{generate_contract_scalar_oracles, source_selection_work};
use crate::core::contract_package::{
    routed_contract_lowering_profile, ContractDriverResultSelection, ContractRequestPosition,
};
use crate::oracle::claim::ClaimDisposition;
use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, ContractDependencies, ContractOccurrenceKeyV1,
    ContractPackageReadResult, ContractPackageV1, ContractPackageWireV1,
};

fn admitted_source(source: &str) -> CheckedPackageV2 {
    let compiled = compile_package(
        SourceIdentity::new("test", "package-items.native", "git", "fixture"),
        "package-items.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL compiles genuine scalar source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("IR must admit QSL output: {other:?}"),
    }
}

fn occurrence(checked: &CheckedPackageV2, node_id: &CheckedNodeId) -> ContractOccurrenceKeyV1 {
    let sources = checked
        .source_map()
        .iter()
        .filter(|source| {
            &source.node_id == node_id && source.role == CheckedOccurrenceRole::Expression
        })
        .collect::<Vec<_>>();
    let [source] = sources.as_slice() else {
        panic!("one genuine expression occurrence required");
    };
    ContractOccurrenceKeyV1 {
        node_id: node_id.clone(),
        role: source.role.clone(),
        ordinal: source.ordinal,
    }
}

fn own_result(
    checked: &CheckedPackageV2,
    application: &CheckedNodeId,
    conversion: Option<&CheckedNodeId>,
) -> ContractDriverResultSelection {
    ContractDriverResultSelection::OwnResult {
        application: occurrence(checked, application),
        conversion: conversion.map(|node_id| occurrence(checked, node_id)),
    }
}

fn admitted_case(
    requests: usize,
    unrelated: usize,
) -> (ContractPackageV1, Vec<ContractRequestPosition>) {
    let mut source = String::from(
        "language \"ix:native\" edition \"1-draft\";\n\
             profile v = \"quire.value.complete/v1\";\n",
    );
    for index in 0..requests {
        source.push_str(&format!(
            "function f{index} using v(a: Int[0, 9]): Int[0, 100] pure {{ a + {} }}\n",
            index + 1
        ));
    }
    for index in 0..unrelated {
        source.push_str(&format!(
            "function extra{index} using v(a: Int[0, 9]): Boolean pure {{ a < {} }}\n",
            index + 20
        ));
    }
    let checked = admitted_source(&source);
    let applications = checked
        .graph()
        .nodes
        .iter()
        .filter(|node| {
            node.body
                .pointer("/operation/identity")
                .and_then(serde_json::Value::as_str)
                == Some("quire.op.integer.add")
        })
        .collect::<Vec<_>>();
    assert_eq!(applications.len(), requests);
    let mut positions = Vec::new();
    for application in &applications {
        let conversions = checked
            .graph()
            .nodes
            .iter()
            .filter(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.numeric.narrow")
                    && node.body.pointer("/arguments/0/target")
                        == Some(&serde_json::to_value(&application.node_id).expect("key"))
            })
            .collect::<Vec<_>>();
        let [conversion] = conversions.as_slice() else {
            panic!("one genuine result narrow per addition");
        };
        positions.push(ContractRequestPosition::DriverItem {
            node_id: application.node_id.clone(),
            result_selection: own_result(&checked, &application.node_id, Some(&conversion.node_id)),
        });
        positions.push(ContractRequestPosition::ContextRoot {
            node_id: conversion.node_id.clone(),
        });
    }
    for index in 0..unrelated {
        positions.push(ContractRequestPosition::ContextRoot {
            node_id: checked
                .graph()
                .nodes
                .iter()
                .find(|node| {
                    node.declaration.as_ref().is_some_and(|declaration| {
                        declaration.qualified_name.len() == 1
                            && declaration.qualified_name[0].as_ref() == format!("extra{index}")
                    })
                })
                .expect("unrelated QSL function")
                .node_id
                .clone(),
        });
    }
    let roots = positions
        .iter()
        .map(|position| position.node_id().clone())
        .collect::<Vec<_>>();
    let lowered = checked.lower(&roots, &routed_contract_lowering_profile());
    let admitted = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("genuine boundary family must admit: {other:?}"),
    };
    (*admitted, positions)
}

/// Provenance: IR-988 AC1-3; SR-16303 FND-003.
/// Counts only selected_scalar_key's source selection, not owner closure/index work.
#[test]
fn ir_988_scalar_source_selections_use_retained_occurrences() {
    let mut measured = Vec::new();
    for requests in [2, 4, 8] {
        for unrelated in [4, 8, 16] {
            let (admitted, positions) = admitted_case(requests, unrelated);
            let selected = (0..requests).collect::<Vec<_>>();
            source_selection_work::take();
            let (generated, indexes) =
                generate_contract_scalar_oracles(&admitted, &positions, &selected)
                    .expect("genuine selected scalar generation");
            let work = source_selection_work::take();
            assert_eq!(indexes.len(), requests);
            assert_eq!(generated.claim_map.items.len(), requests);
            for index in indexes {
                let ClaimDisposition::Generated(claim) = &generated.claim_map.items[index].result
                else {
                    panic!("each genuine selected conversion must generate");
                };
                assert_eq!(&claim.contract_package_identity, admitted.identity());
                assert!(!claim.checked_bounds.is_empty());
                assert!(!claim.source_map.is_empty());
                assert!(!claim.oracle_source.is_empty());
            }
            let graph_nodes = admitted.wire().semantic_graph.nodes.len();
            let source_entries = admitted.wire().source_map.nodes.len();
            println!("scalar_sources K={requests} R={unrelated} G={graph_nodes} S={source_entries} whole_map_visits={} indexed_lookups={} application_occurrences={} conversion_occurrences={}", work.whole_map_visits, work.indexed_lookups, work.application_occurrences, work.conversion_occurrences);
            if let Some(directory) = std::env::var_os("IR988_SCALAR_BOUNDARY_OUTPUT_DIR") {
                let directory = std::path::PathBuf::from(directory);
                std::fs::create_dir_all(&directory).expect("evidence directory");
                std::fs::write(
                    directory.join(format!("k{requests}-r{unrelated}.txt")),
                    format!("{generated:#?}"),
                )
                .expect("complete scalar artifact and claim output");
            }
            measured.push((requests, unrelated, work));
        }
    }
    // Capture every cell and its complete output before enforcing the resource
    // regression, so the historical algorithm produces all nine RED receipts.
    for (requests, unrelated, work) in measured {
        assert_eq!(
            work.whole_map_visits, 0,
            "K={requests} R={unrelated}: no whole-map selection scan"
        );
        assert_eq!(
            work.indexed_lookups,
            2 * requests,
            "one owner lookup per application and conversion"
        );
        assert_eq!(
            work.application_occurrences, requests,
            "one admitted application occurrence per request"
        );
        assert_eq!(
            work.conversion_occurrences, requests,
            "one admitted conversion occurrence per request"
        );
    }
}
