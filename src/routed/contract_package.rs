// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-039: position-preserving access to Contract IR's admitted FR-330 package.

use quire_contract_model::{
    CheckedNodeId, CheckedSemanticNodeV2, CheckedSourceMapEntry, ContractFeatureRecordV1,
    ContractPackageV1,
};

/// A mismatch between the driver's request and one admitted package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PackageRequestError {
    /// The package has a different number of accounting records.
    Count { requested: usize, accounted: usize },
    /// An accounting record names a different exact requested key.
    Identity {
        request_index: usize,
        requested: CheckedNodeId,
        accounted: CheckedNodeId,
    },
    /// A requested position is outside the admitted accounting.
    Position { request_index: usize },
    /// A lowered record's root is absent from the admitted graph.
    MissingRoot {
        request_index: usize,
        node_id: CheckedNodeId,
    },
}

/// The admitted meaning and exact available source occurrences for one position.
pub(crate) enum PackageItem<'a> {
    /// The root is represented in the package's semantic graph.
    Lowered {
        node: &'a CheckedSemanticNodeV2,
        occurrences: &'a [CheckedSourceMapEntry],
    },
    /// The original typed accounting record remains the cause of refusal.
    Refused {
        record: &'a ContractFeatureRecordV1,
        occurrences: &'a [CheckedSourceMapEntry],
    },
}

/// A checked correspondence between request positions and one admitted package.
pub(crate) struct PackageItems<'a> {
    package: &'a ContractPackageV1,
}

impl<'a> PackageItems<'a> {
    /// Match every position and exact key before any item can be consumed.
    pub(crate) fn new(
        package: &'a ContractPackageV1,
        requested: &[CheckedNodeId],
    ) -> Result<Self, PackageRequestError> {
        let accounted = &package.wire().feature_accounting;
        if requested.len() != accounted.len() {
            return Err(PackageRequestError::Count {
                requested: requested.len(),
                accounted: accounted.len(),
            });
        }
        for (request_index, (node_id, record)) in requested.iter().zip(accounted).enumerate() {
            if node_id != record.node_id() {
                return Err(PackageRequestError::Identity {
                    request_index,
                    requested: node_id.clone(),
                    accounted: record.node_id().clone(),
                });
            }
        }
        Ok(Self { package })
    }

    /// Return the exact admitted outcome at one request position.
    pub(crate) fn at(&self, request_index: usize) -> Result<PackageItem<'a>, PackageRequestError> {
        let wire = self.package.wire();
        let record = wire
            .feature_accounting
            .get(request_index)
            .ok_or(PackageRequestError::Position { request_index })?;
        // The sole IR reader guarantees source-map alignment with accounting.
        let occurrences = wire.source_map.requested_items[request_index]
            .occurrences
            .as_slice();
        match record {
            ContractFeatureRecordV1::Lowered { node_id } => {
                let node = wire
                    .semantic_graph
                    .nodes
                    .binary_search_by(|node| node.node_id.cmp(node_id))
                    .ok()
                    .and_then(|index| wire.semantic_graph.nodes.get(index))
                    .ok_or_else(|| PackageRequestError::MissingRoot {
                        request_index,
                        node_id: node_id.clone(),
                    })?;
                Ok(PackageItem::Lowered { node, occurrences })
            }
            ContractFeatureRecordV1::Unsupported { .. }
            | ContractFeatureRecordV1::RequiresBound { .. }
            | ContractFeatureRecordV1::InvalidInput { .. }
            | ContractFeatureRecordV1::Failed { .. }
            | ContractFeatureRecordV1::InvalidBody { .. }
            | ContractFeatureRecordV1::BodyIncomplete { .. } => Ok(PackageItem::Refused {
                record,
                occurrences,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PackageItem, PackageItems, PackageRequestError};
    use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
    use quire_contract_model::{
        CheckedNodeId, CheckedNodeTag, CheckedPackageEvidence, CheckedPackageReadLimits,
        CheckedPackageV2, CheckedPackageV2ReadResult, CompleteLoweringProfileV2,
        ContractDependencies, ContractFeatureRecordV1, ContractPackageReadResult,
        ContractPackageWireV1, LoweringProfileRef,
    };

    const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
        profile v = \"quire.value.complete/v1\";\n\
        function f using v(a: Int[0, 9]): Boolean pure { a = a }\n";

    fn admitted_source() -> CheckedPackageV2 {
        let compiled = compile_package(
            SourceIdentity::new("test", "package-items.native", "git", "fixture"),
            "package-items.native",
            SOURCE.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL compiles the source");
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

    fn profile() -> CompleteLoweringProfileV2 {
        CompleteLoweringProfileV2 {
            reference: LoweringProfileRef {
                authority: "quire".into(),
                identity: "complete-v1".into(),
            },
            supported_tags: [
                CheckedNodeTag::ScalarType,
                CheckedNodeTag::CompositeType,
                CheckedNodeTag::BoundedDomain,
                CheckedNodeTag::Value,
                CheckedNodeTag::Expression,
                CheckedNodeTag::Function,
                CheckedNodeTag::Model,
                CheckedNodeTag::Relation,
                CheckedNodeTag::State,
                CheckedNodeTag::Temporal,
                CheckedNodeTag::Protocol,
                CheckedNodeTag::Claim,
                CheckedNodeTag::Correspondence,
            ]
            .into(),
            require_bounds: false,
            work_limit: u64::MAX,
        }
    }

    /// Trace: FR-039-AC-2, FR-039-AC-5, TC-053.
    #[test]
    fn admitted_qsl_positions_keep_repeated_keys_and_absent_sibling_sources() {
        let source = admitted_source();
        let function = source
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == "f"
                })
            })
            .expect("QSL function node")
            .node_id
            .clone();
        let absent = CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: "0".repeat(64).into_boxed_str(),
        };
        let requested = vec![function.clone(), absent.clone(), function.clone()];
        let lowered = source.lower(&requested, &profile());
        let bytes = lowered.package.canonical_bytes().expect("FR-330 bytes");
        let admitted = match ContractPackageWireV1::read(
            bytes,
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("FR-330 reader must admit IR output: {other:?}"),
        };
        let items = PackageItems::new(&admitted, &requested).expect("positions and keys match");
        for index in [0, 2] {
            match items.at(index).expect("lowered position") {
                PackageItem::Lowered { node, occurrences } => {
                    assert_eq!(node.node_id, function);
                    assert!(
                        !occurrences.is_empty(),
                        "QSL function has a source occurrence"
                    );
                }
                PackageItem::Refused { .. } => panic!("function must lower"),
            }
        }
        match items.at(1).expect("absent sibling position") {
            PackageItem::Refused {
                record,
                occurrences,
            } => {
                assert!(
                    matches!(record, ContractFeatureRecordV1::InvalidInput { node_id } if *node_id == absent)
                );
                assert!(occurrences.is_empty(), "absent key has no invented region");
            }
            PackageItem::Lowered { .. } => panic!("absent sibling cannot lower"),
        }
        let swapped = vec![absent, function.clone(), function];
        assert!(matches!(
            PackageItems::new(&admitted, &swapped),
            Err(PackageRequestError::Identity {
                request_index: 0,
                ..
            })
        ));
    }
}
