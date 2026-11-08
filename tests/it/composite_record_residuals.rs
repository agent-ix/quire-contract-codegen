// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native artifact publication boundaries; no backend-proof or shadow credit.

use super::{
    admitted_enum_equality, composite_artifact_binding, composite_equality_agreement,
    composite_original_eq,
};
use ix_trace_rs::trace;
use quire_contract_codegen::{ClaimDisposition, CompositeBuildError};
use quire_contract_model::CheckedOccurrenceRole;

#[trace("TC-048")]
#[test]
fn every_published_native_file_and_claim_map_member_keeps_its_original_authority() {
    let compiled =
        composite_equality_agreement::fixture::compile_source(composite_original_eq::SOURCE);
    let package = admitted_enum_equality::checked(&compiled, &[]);
    let item = admitted_enum_equality::direct_source_item(&package, "f");
    let node = item.node_id.clone();
    let generated =
        admitted_enum_equality::generate_items(&package, Some(compiled.context()), &[item], &[])
            .unwrap();
    let original = package
        .graph()
        .nodes
        .iter()
        .find(|entry| entry.node_id == node)
        .unwrap();
    let mut occurrences = original
        .occurrences
        .iter()
        .filter(|entry| entry.role == CheckedOccurrenceRole::Expression);
    let occurrence = occurrences.next().unwrap();
    assert!(occurrences.next().is_none());
    assert!(composite_artifact_binding::binding(
        &package, &compiled, &generated, &node, occurrence
    )
    .is_ok());
    assert!(!generated.artifacts.is_empty());
    for index in 0..generated.artifacts.len() {
        let mut changed = generated.clone();
        changed.artifacts[index].contents.push(' ');
        assert!(
            matches!(
                composite_artifact_binding::binding(
                    &package, &compiled, &changed, &node, occurrence
                ),
                Err(CompositeBuildError::GeneratedArtifactChanged)
            ),
            "all published artifact bytes: {}",
            generated.artifacts[index].path
        );
    }
    for member in 0..3 {
        let mut changed = generated.clone();
        let ClaimDisposition::Generated(claim) = &mut changed.claim_map.items[0].result else {
            panic!("genuine generated equality claim");
        };
        let retained_node = claim.node_id.clone();
        match member {
            0 => claim.environment_symbol.push_str("_changed"),
            1 => {
                let source = claim
                    .source_map
                    .first_mut()
                    .expect("actual source correspondence");
                source
                    .regions
                    .first_mut()
                    .expect("actual source span")
                    .start += 1;
            }
            2 => {
                claim.descriptor.left_conversion_target =
                    if claim.descriptor.left_conversion_target.is_some() {
                        None
                    } else {
                        Some(claim.descriptor.left_source_type.clone())
                    };
            }
            _ => unreachable!(),
        }
        assert_eq!(
            claim.node_id, retained_node,
            "metadata is outside structural node identity"
        );
        assert_ne!(
            changed.claim_map, generated.claim_map,
            "the mutation must change the record"
        );
        assert!(
            matches!(
                composite_artifact_binding::binding(
                    &package, &compiled, &changed, &node, occurrence
                ),
                Err(CompositeBuildError::GeneratedArtifactChanged)
            ),
            "complete native claim-map authority {member}"
        );
    }
}
