// SPDX-License-Identifier: AGPL-3.0-or-later
//! Reader evidence held from published QSpec definition artifacts, independently of wire claims.

use std::{fs, path::Path};

use quire_contract_model::{
    CheckedArtifactRef, CheckedExtensionDefinition, CheckedPackageEvidence,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedDefinitions {
    definitions: Vec<PublishedDefinition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedDefinition {
    reference: CheckedArtifactRef,
    definition: CheckedExtensionDefinition,
}

/// Hold only published typed extension definitions whose full refs the fixture selected.
/// A wire selection alone supplies neither a definition nor a supported capability.
pub(crate) fn retain_selected_extensions(
    qspec: &Path,
    wire: &Value,
    evidence: &mut CheckedPackageEvidence,
) {
    let selected = wire["lock"]["definition_selections"]
        .as_array()
        .expect("fixture definition selections")
        .iter()
        .map(|reference| {
            serde_json::from_value::<CheckedArtifactRef>(reference.clone())
                .expect("fixture definition reference")
        })
        .collect::<Vec<_>>();
    let definitions: PublishedDefinitions = serde_json::from_slice(
        &fs::read(qspec.join("proposals/checked-package-v2/extension-definitions.json"))
            .expect("read authoritative QSpec extension definitions"),
    )
    .expect("published typed extension definitions");
    for published in definitions.definitions {
        if selected.contains(&published.reference) {
            evidence
                .insert_extension_definition(published.reference, published.definition)
                .expect("retain exact published extension definition");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::{
        CheckedPackageReadLimits, CheckedPackageRefusalCode, CheckedPackageV2,
        CheckedPackageV2ReadResult,
    };
    use std::path::PathBuf;

    fn all_families() -> (PathBuf, Value, Vec<u8>) {
        let root = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"));
        let wire: Value = serde_json::from_slice(
            &fs::read(
                root.join("proposals/checked-package-v2/fixtures/positive-all-families.json"),
            )
            .expect("read published all-families fixture"),
        )
        .expect("published fixture JSON");
        let bytes = serde_json::to_vec(&wire).expect("canonical fixture bytes");
        (root, wire, bytes)
    }

    fn model_evidence(root: &Path, wire: &Value) -> CheckedPackageEvidence {
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        for selected in wire["lock"]["model_selections"]
            .as_array()
            .expect("model selections")
        {
            assert_eq!(selected["identity"], "acme/orders");
            evidence
                .insert_domain_package_document(
                    selected["digest"].as_str().expect("selected model digest"),
                    fs::read(
                        root.join("proposals/checked-package-v2/domain-package-acme-orders.json"),
                    )
                    .expect("published selected model document"),
                )
                .expect("retain selected model document");
        }
        evidence
    }

    fn assert_unheld_retention(bytes: &[u8], evidence: &CheckedPackageEvidence) {
        let CheckedPackageV2ReadResult::Refused(refusal) =
            CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), evidence)
        else {
            panic!("unheld extension must remain a strict reader refusal");
        };
        assert_eq!(
            refusal.code,
            CheckedPackageRefusalCode::UnknownRequiredCapability
        );
        assert_eq!(
            refusal.path.as_ref().map(|path| path.as_str()),
            Some("/semantic_graph/nodes/34/body/definition")
        );
        assert_eq!(refusal.document_pointer, refusal.path);
    }

    /// Provenance: IR-975 / CG #546 fixture-evidence repair for the IR120 reader.
    #[trace("FR-014-AC-44", "TC-024")]
    #[test]
    fn published_extension_evidence_is_required_for_strict_all_families_admission() {
        let (root, wire, bytes) = all_families();
        let mut evidence = model_evidence(&root, &wire);
        assert_unheld_retention(&bytes, &evidence);
        retain_selected_extensions(&root, &wire, &mut evidence);
        let CheckedPackageV2ReadResult::Admitted(package) =
            CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence)
        else {
            panic!("the held exact published retention definition must admit the real fixture");
        };
        assert_eq!(
            package.graph().nodes.len(),
            wire["semantic_graph"]["nodes"].as_array().unwrap().len()
        );
        assert_eq!(package.graph().nodes[34].node_tag.as_ref(), "extension");
        assert_eq!(
            package.graph().nodes[34].body["definition"],
            wire["semantic_graph"]["nodes"][34]["body"]["definition"]
        );
    }

    #[trace("FR-014-AC-44", "TC-024")]
    #[test]
    fn mismatched_selected_extension_refs_do_not_manufacture_reader_evidence() {
        let (root, wire, bytes) = all_families();
        for (member, value) in [("authority", "other"), ("identity", "unpublished")] {
            let mut selections = wire.clone();
            selections["lock"]["definition_selections"][0][member] = Value::from(value);
            let mut evidence = model_evidence(&root, &wire);
            retain_selected_extensions(&root, &selections, &mut evidence);
            // Read the SAME unchanged genuine wire: supplying a different selection to the
            // evidence loader cannot stand in for its required example/retention artifact.
            assert_unheld_retention(&bytes, &evidence);
        }
    }
}
