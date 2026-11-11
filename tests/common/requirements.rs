//! Projects the QSL requirement item and record the driver would hand CG for a fixture's
//! authenticated `OwnResult` selections (FR-022, IR-304). The result bound is the semantic type of
//! the selected conversion, or of the application when there is none; the path condition is empty.

use std::collections::BTreeMap;

use quire_contract_codegen::{
    ContractDriverResultSelection, ContractRequestPosition, GenerationContexts,
    KaniGenerationContext, RequirementOccurrence, RequirementRecordInput, RequirementRecords,
    RoutedGenerationItem, RoutedRequirement,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2, ContractOccurrenceKeyV1};

/// The occurrence key of an IR occurrence.
pub fn occurrence(key: &ContractOccurrenceKeyV1) -> RequirementOccurrence {
    RequirementOccurrence {
        node_id: key.node_id.clone(),
        role: key.role.clone(),
        ordinal: key.ordinal,
    }
}

fn semantic_type(checked: &CheckedPackageV2, node: &CheckedNodeId) -> CheckedNodeId {
    checked
        .resolved_graph()
        .nodes
        .iter()
        .find(|candidate| &candidate.node_id == node)
        .expect("a selected node belongs to the source graph")
        .semantic_type
        .clone()
}

/// The requirement for one selection, or `None` for `NotApplicable`.
pub fn requirement_for(
    checked: &CheckedPackageV2,
    selection: &ContractDriverResultSelection,
) -> Option<RoutedRequirement> {
    let ContractDriverResultSelection::OwnResult {
        application,
        conversion,
    } = selection
    else {
        return None;
    };
    Some(RoutedRequirement {
        occurrence: occurrence(application),
        result_bound: semantic_type(checked, &conversion.as_ref().unwrap_or(application).node_id),
        conversion_context: conversion.as_ref().map(occurrence),
        path_condition: Vec::new(),
    })
}

/// The requirement of every bound driver position that selects an `OwnResult`, by request index.
pub fn project(
    checked: &CheckedPackageV2,
    positions: &[ContractRequestPosition],
) -> BTreeMap<usize, RoutedRequirement> {
    positions
        .iter()
        .filter(|position| !matches!(position, ContractRequestPosition::ContextRoot { .. }))
        .enumerate()
        .filter_map(|(index, position)| match position {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => requirement_for(checked, result_selection).map(|requirement| (index, requirement)),
            _ => None,
        })
        .collect()
}

/// The record QSL writes for each projected requirement.
pub fn records(requirements: &BTreeMap<usize, RoutedRequirement>) -> RequirementRecords {
    RequirementRecords::new(
        requirements
            .values()
            .map(|requirement| {
                (
                    requirement.occurrence.clone(),
                    RequirementRecordInput {
                        result_bound: requirement.result_bound.clone(),
                        conversion_context: requirement.conversion_context.clone(),
                        path_condition: requirement.path_condition.clone(),
                    },
                )
            })
            .collect(),
    )
}

/// Binds each item's projected requirement.
pub fn bind(
    routed: Vec<RoutedGenerationItem>,
    requirements: &BTreeMap<usize, RoutedRequirement>,
) -> Vec<RoutedGenerationItem> {
    routed
        .into_iter()
        .map(|item| match requirements.get(&item.request_index) {
            Some(requirement) => item.with_requirement(requirement.clone()),
            None => item,
        })
        .collect()
}

/// `contexts` with its Kani context reading `records`.
pub fn with_records<'a>(
    contexts: &GenerationContexts<'a>,
    records: &'a RequirementRecords,
) -> GenerationContexts<'a> {
    GenerationContexts {
        kani: contexts.kani.map(|kani| KaniGenerationContext {
            requirement_records: records,
            ..kani
        }),
    }
}

/// The explicit statement that a package has no requirement record.
pub static NO_RECORDS: RequirementRecords = RequirementRecords::new(BTreeMap::new());
