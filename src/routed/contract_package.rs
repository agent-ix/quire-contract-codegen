// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-039: position-preserving access to Contract IR's admitted FR-330 package.

use std::collections::BTreeSet;

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedSourceMapEntry, CompleteLoweringProfileV2,
    ContractFeatureRecordV1, ContractPackageV1, ContractReachedClosureV1, ContractRootViewError,
    ContractRootViewV1, LoweringProfileRef,
};

/// The CG-owned lowering profile for one mixed routed request.
///
/// The driver passes this profile to Contract IR before submitting the single
/// admitted package to negotiation and generation. Its tags cover the built-in
/// scalar, equality and exact-function arms; process providers do not enlarge
/// that set. A lowered scalar with an unbounded operand still receives CG's
/// typed `RequiresBound` refusal from its admitted reached closure.
///
/// Trace: FR-039-AC-2, TC-053.
#[must_use]
pub fn routed_contract_lowering_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.routed-contract/v1".into(),
        },
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Function,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: 65_536,
    }
}

/// The role of one root in the exact sequence passed to Contract IR lowering.
///
/// Driver items retain their request order even when context-only roots are
/// interleaved. A context root has FR-330 accounting but no backend settlement
/// or generated claim of its own.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractRequestPosition {
    /// One original driver request, including repeated requested node IDs.
    DriverItem {
        /// Exact root key of this original driver request.
        node_id: CheckedNodeId,
    },
    /// A root needed to represent the meaning of a driver item.
    ContextRoot {
        /// Exact root key submitted only to represent request context.
        node_id: CheckedNodeId,
    },
}

impl ContractRequestPosition {
    /// The exact root key submitted at this lowering position.
    #[must_use]
    pub fn node_id(&self) -> &CheckedNodeId {
        match self {
            Self::DriverItem { node_id } | Self::ContextRoot { node_id } => node_id,
        }
    }
}

/// A mismatch between the driver's request and one admitted package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageRequestError {
    /// The caller supplied a different number of descriptors and request positions.
    ItemPositions {
        /// Number of descriptors.
        items: usize,
        /// Number of request positions.
        positions: usize,
    },
    /// The package has a different number of accounting records.
    Count {
        /// Number of keys the caller requested.
        requested: usize,
        /// Number of admitted accounting records.
        accounted: usize,
    },
    /// An accounting record names a different exact requested key.
    Identity {
        /// Position of the disagreement.
        request_index: usize,
        /// Exact key requested by the caller.
        requested: CheckedNodeId,
        /// Exact key in the admitted accounting record.
        accounted: CheckedNodeId,
    },
    /// A lowering root, driver or context, disagrees with IR accounting.
    AccountingIdentity {
        /// Position in the full lowering request.
        accounting_index: usize,
        /// Exact root key passed to lowering.
        requested: CheckedNodeId,
        /// Exact key in admitted FR-330 accounting.
        accounted: CheckedNodeId,
    },
    /// A requested position is outside the admitted accounting.
    Position {
        /// Position outside the admitted accounting.
        request_index: usize,
    },
    /// The admitted traversal could not reproduce a lowered item's closure.
    ReachedClosure {
        /// Position of the lowered record.
        request_index: usize,
        /// Exact key whose closure failed.
        node_id: CheckedNodeId,
        /// Reader traversal's typed failure.
        cause: ContractRootViewError,
    },
}

/// The exact admitted FR-330 decision and source occurrences at one request position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedItemAccounting {
    /// Contract IR's original typed record, including a `Lowered` disposition.
    pub record: ContractFeatureRecordV1,
    /// Exact source occurrences in the admitted request-position source map.
    pub occurrences: Vec<CheckedSourceMapEntry>,
}

/// The admitted meaning and exact available source occurrences for one position.
pub(crate) enum PackageItem<'a> {
    /// The root is represented in the package's semantic graph.
    Lowered {
        record: &'a ContractFeatureRecordV1,
        closure: ContractReachedClosureV1<'a>,
        occurrences: &'a [CheckedSourceMapEntry],
    },
    /// The original typed accounting record remains the cause of refusal.
    Refused {
        record: &'a ContractFeatureRecordV1,
        occurrences: &'a [CheckedSourceMapEntry],
    },
}

impl PackageItem<'_> {
    /// Own the record and occurrences once for a public production decision.
    pub(crate) fn accounting(&self) -> AdmittedItemAccounting {
        match self {
            Self::Lowered {
                record,
                occurrences,
                ..
            }
            | Self::Refused {
                record,
                occurrences,
            } => AdmittedItemAccounting {
                record: (**record).clone(),
                occurrences: occurrences.to_vec(),
            },
        }
    }
}

/// A checked correspondence between request positions and one admitted package.
pub(crate) struct PackageItems<'a> {
    package: &'a ContractPackageV1,
    driver_to_accounting: Vec<usize>,
}

impl<'a> PackageItems<'a> {
    /// Match every position and exact key before any item can be consumed.
    pub(crate) fn new(
        package: &'a ContractPackageV1,
        positions: &[ContractRequestPosition],
    ) -> Result<Self, PackageRequestError> {
        let accounted = &package.wire().feature_accounting;
        if positions.len() != accounted.len() {
            return Err(PackageRequestError::Count {
                requested: positions.len(),
                accounted: accounted.len(),
            });
        }
        let mut driver_to_accounting = Vec::new();
        for (accounting_index, (position, record)) in positions.iter().zip(accounted).enumerate() {
            if position.node_id() != record.node_id() {
                return Err(PackageRequestError::AccountingIdentity {
                    accounting_index,
                    requested: position.node_id().clone(),
                    accounted: record.node_id().clone(),
                });
            }
            if matches!(position, ContractRequestPosition::DriverItem { .. }) {
                driver_to_accounting.push(accounting_index);
            }
        }
        Ok(Self {
            package,
            driver_to_accounting,
        })
    }

    /// Number of original driver requests, excluding context-only roots.
    pub(crate) fn driver_count(&self) -> usize {
        self.driver_to_accounting.len()
    }

    /// Exact key of one driver request, independently of context-root order.
    pub(crate) fn driver_node(&self, request_index: usize) -> Option<&'a CheckedNodeId> {
        let accounting_index = *self.driver_to_accounting.get(request_index)?;
        self.package
            .wire()
            .feature_accounting
            .get(accounting_index)
            .map(ContractFeatureRecordV1::node_id)
    }

    /// Return the exact admitted outcome at one request position.
    pub(crate) fn at(
        &self,
        request_index: usize,
        node_id: &CheckedNodeId,
    ) -> Result<PackageItem<'a>, PackageRequestError> {
        let accounting_index = *self
            .driver_to_accounting
            .get(request_index)
            .ok_or(PackageRequestError::Position { request_index })?;
        let wire = self.package.wire();
        let record = wire
            .feature_accounting
            .get(accounting_index)
            .ok_or(PackageRequestError::Position { request_index })?;
        if record.node_id() != node_id {
            return Err(PackageRequestError::Identity {
                request_index,
                requested: node_id.clone(),
                accounted: record.node_id().clone(),
            });
        }
        // The sole IR reader guarantees source-map alignment with accounting.
        let occurrences = wire.source_map.requested_items[accounting_index]
            .occurrences
            .as_slice();
        match record {
            ContractFeatureRecordV1::Lowered { node_id } => {
                let view = self.package.reached_closure(node_id).map_err(|cause| {
                    PackageRequestError::ReachedClosure {
                        request_index,
                        node_id: node_id.clone(),
                        cause,
                    }
                })?;
                let ContractRootViewV1::Lowered(closure) = view else {
                    return Err(PackageRequestError::ReachedClosure {
                        request_index,
                        node_id: node_id.clone(),
                        cause: ContractRootViewError::Inconsistent,
                    });
                };
                Ok(PackageItem::Lowered {
                    record,
                    closure,
                    occurrences,
                })
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
    use super::{
        routed_contract_lowering_profile, ContractRequestPosition, PackageItem, PackageItems,
        PackageRequestError,
    };
    use crate::routed::capability::{
        negotiate_contract_package, BackendDescriptor, BackendProviderEnvelope, Candidate,
        Candidates, ContractItemSettlement, ProviderOrigin, RequestItem, RequestedKind,
        BACKEND_PROVIDER_CONTRACT, CAPABILITY_VOCABULARY,
    };
    use crate::routed::generate::{
        generate_routed, GenerationContexts, KaniGenerationContext, KindOutput,
        RoutedGenerationItem,
    };
    use qsl_replay::{
        compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
        SuppliedLibrary,
    };
    use quire_contract_model::{
        CheckedNodeId, CheckedNodeTag, CheckedPackageEvidence, CheckedPackageReadLimits,
        CheckedPackageRefusalCode, CheckedPackageV2, CheckedPackageV2ReadResult,
        CompleteLoweringProfileV2, ContractDependencies, ContractDependencyInput,
        ContractFeatureRecordV1, ContractPackageReadResult, ContractPackageWireV1,
        ContractRootViewV1, LoweringProfileRef,
    };

    const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
        profile v = \"quire.value.complete/v1\";\n\
        function f using v(a: Int[0, 9]): Boolean pure { a = a }\n";

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

    fn drivers(node_ids: &[CheckedNodeId]) -> Vec<ContractRequestPosition> {
        node_ids
            .iter()
            .cloned()
            .map(|node_id| ContractRequestPosition::DriverItem { node_id })
            .collect()
    }

    fn empty_backend_request(count: usize) -> BackendProviderEnvelope {
        BackendProviderEnvelope {
            contract_version: BACKEND_PROVIDER_CONTRACT.to_owned(),
            capability_vocabulary: CAPABILITY_VOCABULARY.to_owned(),
            manifest: Vec::new(),
            items: (0..count)
                .map(|_| RequestItem {
                    kind: RequestedKind::Absent,
                    extent: None,
                    named_backend: None,
                    candidates: Candidates::Set(Vec::new()),
                })
                .collect(),
        }
    }

    fn replace_exact_digest(value: &mut serde_json::Value, old: &str, new: &str) {
        match value {
            serde_json::Value::String(text) if text == old => *text = new.to_owned(),
            serde_json::Value::Array(items) => {
                for item in items {
                    replace_exact_digest(item, old, new);
                }
            }
            serde_json::Value::Object(members) => {
                for member in members.values_mut() {
                    replace_exact_digest(member, old, new);
                }
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }

    /// Controls exact positions and source occurrences for FR-039-AC-2.
    #[test]
    fn admitted_qsl_positions_keep_repeated_keys_and_absent_sibling_sources() {
        let source = admitted_source(SOURCE);
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
        let requested = vec![
            function.clone(),
            function.clone(),
            absent.clone(),
            function.clone(),
        ];
        let positions = vec![
            ContractRequestPosition::DriverItem {
                node_id: function.clone(),
            },
            ContractRequestPosition::ContextRoot {
                node_id: function.clone(),
            },
            ContractRequestPosition::DriverItem {
                node_id: absent.clone(),
            },
            ContractRequestPosition::DriverItem {
                node_id: function.clone(),
            },
        ];
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
        let items = PackageItems::new(&admitted, &positions).expect("positions and keys match");
        let settled = negotiate_contract_package(&admitted, &positions, &empty_backend_request(3))
            .expect("production settlement consumes the admitted package");
        assert_eq!(settled.len(), 3);
        for index in [0, 2] {
            let ContractItemSettlement { backend, .. } = &settled[index];
            assert!(
                matches!(
                    backend,
                    Some(crate::routed::capability::ItemSettlement {
                        disposition: crate::routed::capability::Disposition::InvalidRequest {
                            cause: crate::routed::capability::Cause::AbsentKind,
                        },
                        ..
                    })
                ),
                "lowered sibling retains its own FR-019 settlement"
            );
        }
        assert!(settled[1].backend.is_none());
        for (index, settlement) in settled.iter().enumerate() {
            let accounting_index = [0, 2, 3][index];
            assert_eq!(settlement.request_index, index);
            assert_eq!(settlement.node_id, requested[accounting_index]);
            assert_eq!(
                settlement.admitted.record,
                admitted.wire().feature_accounting[accounting_index]
            );
            assert_eq!(
                settlement.admitted.occurrences,
                admitted.wire().source_map.requested_items[accounting_index].occurrences
            );
        }
        assert_eq!(items.driver_count(), 3);
        assert_eq!(items.driver_node(2), Some(&function));
        assert!(matches!(
            &settled[1].admitted.record,
            ContractFeatureRecordV1::InvalidInput { node_id } if *node_id == absent
        ));
        assert!(settled[1].admitted.occurrences.is_empty());
        let process = BackendDescriptor {
            identity: "context-test-process".to_owned(),
            advertised: Vec::new(),
            origin: ProviderOrigin::Process,
            domains: None,
            bounds: Default::default(),
        };
        let routes = [function.clone(), absent.clone(), function.clone()]
            .into_iter()
            .enumerate()
            .map(|(request_index, node_id)| {
                RoutedGenerationItem::from_descriptor(
                    request_index,
                    node_id,
                    Candidate {
                        identity: process.identity.clone(),
                    },
                    &process,
                )
                .expect("process route")
            })
            .collect::<Vec<_>>();
        let generated = generate_routed(
            &admitted,
            &positions,
            &routes,
            &GenerationContexts::default(),
        )
        .expect("context root does not shift routed driver positions");
        assert_eq!(generated.items.len(), 3);
        for (request_index, output) in generated.items.iter().enumerate() {
            let accounting_index = [0, 2, 3][request_index];
            assert_eq!(output.request_index, request_index);
            assert_eq!(
                output.admitted.record,
                admitted.wire().feature_accounting[accounting_index]
            );
            assert_eq!(
                output.admitted.occurrences,
                admitted.wire().source_map.requested_items[accounting_index].occurrences
            );
        }
        assert_eq!(generated.items[0].output, KindOutput::Process);
        assert_eq!(generated.items[1].output, KindOutput::PackageRefused);
        assert_eq!(generated.items[2].output, KindOutput::Process);
        for index in [0, 2] {
            match items.at(index, &function).expect("lowered position") {
                PackageItem::Lowered {
                    closure,
                    occurrences,
                    ..
                } => {
                    assert_eq!(closure.root.node_id, function);
                    assert!(closure
                        .nodes
                        .iter()
                        .any(|entry| entry.node.node_id == function));
                    assert!(
                        closure.nodes.len() > 1,
                        "the admitted function reaches its typed body nodes"
                    );
                    assert!(
                        !occurrences.is_empty(),
                        "QSL function has a source occurrence"
                    );
                }
                PackageItem::Refused { .. } => panic!("function must lower"),
            }
        }
        match items.at(1, &absent).expect("absent sibling position") {
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
        let mut swapped = positions;
        swapped[0] = ContractRequestPosition::DriverItem { node_id: absent };
        assert!(matches!(
            PackageItems::new(&admitted, &swapped),
            Err(PackageRequestError::AccountingIdentity {
                accounting_index: 0,
                ..
            })
        ));
        swapped[0] = ContractRequestPosition::DriverItem {
            node_id: function.clone(),
        };
        swapped[1] = ContractRequestPosition::ContextRoot {
            node_id: requested[2].clone(),
        };
        assert!(matches!(
            PackageItems::new(&admitted, &swapped),
            Err(PackageRequestError::AccountingIdentity {
                accounting_index: 1,
                ..
            })
        ));
        assert!(matches!(
            PackageItems::new(&admitted, &drivers(&requested[..3])),
            Err(PackageRequestError::Count {
                requested: 3,
                accounted: 4,
            })
        ));
    }

    /// Controls the independently admitted operation input for FR-039-AC-1.
    #[test]
    fn independently_admitted_qsl_operations_reach_distinct_package_nodes() {
        let not_equal = SOURCE.replace("a = a", "a != a");
        let variants = [
            (SOURCE, "quire.op.integer.eq"),
            (&not_equal, "quire.op.integer.ne"),
        ];
        let mut identities = Vec::new();
        for (source_text, operation) in variants {
            let source = admitted_source(source_text);
            let expression = source
                .graph()
                .nodes
                .iter()
                .find(|node| {
                    node.node_tag.as_ref() == "expression"
                        && node.semantic_form.as_ref() == "binary"
                })
                .expect("QSL comparison expression")
                .node_id
                .clone();
            let lowered = source.lower(std::slice::from_ref(&expression), &profile());
            let admitted = match ContractPackageWireV1::read(
                lowered.package.canonical_bytes().expect("FR-330 bytes"),
                CheckedPackageReadLimits::bounded(),
                &ContractDependencies::new(),
            ) {
                ContractPackageReadResult::Admitted(package) => package,
                other => panic!("IR must admit its own FR-330 output: {other:?}"),
            };
            let items = PackageItems::new(&admitted, &drivers(std::slice::from_ref(&expression)))
                .expect("exact request");
            match items.at(0, &expression).expect("first position") {
                PackageItem::Lowered { closure, .. } => {
                    assert_eq!(
                        closure
                            .root
                            .body
                            .pointer("/operation/identity")
                            .and_then(serde_json::Value::as_str),
                        Some(operation)
                    );
                }
                PackageItem::Refused { record, .. } => panic!("comparison must lower: {record:?}"),
            }
            identities.push(admitted.identity().clone());
        }
        assert_ne!(
            identities[0], identities[1],
            "changed admitted operation changes package identity"
        );
    }

    /// Trace: FR-039-AC-1, FR-039-AC-2, TC-053.
    #[test]
    fn admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling() {
        let mut emitted = Vec::new();
        for (symbol, operation, rendered) in [
            ("<", "quire.op.integer.lt", "rt::OrderingOperator::Less"),
            (">", "quire.op.integer.gt", "rt::OrderingOperator::Greater"),
        ] {
            let source = SOURCE.replace("a = a", &format!("a {symbol} a"));
            let checked = admitted_source(&source);
            let expression = checked
                .graph()
                .nodes
                .iter()
                .find(|node| {
                    node.body
                        .pointer("/operation/identity")
                        .and_then(serde_json::Value::as_str)
                        == Some(operation)
                })
                .expect("QSL integer ordering node")
                .node_id
                .clone();
            let absent = CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: "0".repeat(64).into_boxed_str(),
            };
            let requested = vec![expression.clone(), absent.clone()];
            let lowered = checked.lower(&requested, &routed_contract_lowering_profile());
            let admitted = match ContractPackageWireV1::read(
                lowered.package.canonical_bytes().expect("FR-330 bytes"),
                CheckedPackageReadLimits::bounded(),
                &ContractDependencies::new(),
            ) {
                ContractPackageReadResult::Admitted(package) => package,
                other => panic!("IR must admit the ordering package: {other:?}"),
            };
            assert!(matches!(
                &admitted.wire().feature_accounting[0],
                ContractFeatureRecordV1::Lowered { node_id } if *node_id == requested[0]
            ));
            let descriptor = BackendDescriptor {
                identity: "kani".to_owned(),
                advertised: Vec::new(),
                origin: ProviderOrigin::Linked,
                domains: None,
                bounds: Default::default(),
            };
            let routes = [expression, absent]
                .into_iter()
                .enumerate()
                .map(|(index, node_id)| {
                    RoutedGenerationItem::from_descriptor(
                        index,
                        node_id,
                        Candidate {
                            identity: "kani".to_owned(),
                        },
                        &descriptor,
                    )
                    .expect("linked Kani route")
                })
                .collect::<Vec<_>>();
            let contexts = GenerationContexts {
                kani: Some(KaniGenerationContext {
                    ceilings: crate::kani::identity::ProofCeilings {
                        memory_bytes: std::num::NonZeroU64::new(64 * 1024 * 1024)
                            .expect("nonzero ceiling"),
                        wall_clock: std::time::Duration::from_secs(10),
                    },
                    subject_path: "crate::subject",
                    unwind: 1,
                }),
            };
            let generated = generate_routed(&admitted, &drivers(&requested), &routes, &contexts)
                .expect("routed generation consumes admitted package");
            assert_eq!(generated.items[0].request_index, 0);
            assert_eq!(generated.items[1].request_index, 1);
            for (index, output) in generated.items.iter().enumerate() {
                assert_eq!(
                    output.admitted.record,
                    admitted.wire().feature_accounting[index]
                );
                assert_eq!(
                    output.admitted.occurrences,
                    admitted.wire().source_map.requested_items[index].occurrences
                );
            }
            assert_eq!(generated.items[1].output, KindOutput::PackageRefused);
            assert!(matches!(
                &generated.items[1].admitted.record,
                ContractFeatureRecordV1::InvalidInput { node_id } if *node_id == requested[1]
            ));
            assert!(generated.items[1].admitted.occurrences.is_empty());
            let source = generated
                .oracle_artifacts
                .expect("one admitted ordering oracle")
                .into_iter()
                .find(|artifact| artifact.path == "src/lib.rs")
                .expect("oracle source");
            assert!(
                source.contents.contains(rendered),
                "the admitted operation is rendered"
            );
            emitted.push((admitted.identity().clone(), source.contents));
        }
        assert_ne!(emitted[0].0, emitted[1].0, "the source packages differ");
        assert_ne!(
            emitted[0].1, emitted[1].1,
            "the admitted operation drives bytes"
        );
    }

    /// Trace: FR-039-AC-1, TC-053.
    #[test]
    fn admitted_bounded_integer_operation_generates_from_reached_bounds() {
        let source = SOURCE.replace("Boolean pure { a = a }", "Int[0, 18] pure { a + a }");
        let checked = admitted_source(&source);
        let expression = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.add")
            })
            .expect("QSL integer addition")
            .node_id
            .clone();
        let lowered = checked.lower(std::slice::from_ref(&expression), &profile());
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("IR must admit bounded expression: {other:?}"),
        };
        let generated = crate::oracle::scalar::generate_contract_scalar_oracles(
            &admitted,
            &drivers(std::slice::from_ref(&expression)),
            &[0],
        )
        .expect("admitted bounded operation generates");
        let claim = &generated.claim_map.items[0];
        let crate::oracle::claim::ClaimDisposition::Generated(generated_claim) = &claim.result
        else {
            panic!("bounded operation must generate: {:?}", claim.result);
        };
        assert!(
            !generated_claim.checked_bounds.is_empty(),
            "the admitted closure names checked result and operand bounds"
        );
        assert_eq!(
            generated_claim.contract_package_identity,
            *admitted.identity()
        );
        assert!(generated
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("oracle source")
            .contents
            .contains("rt::IntegerArithmetic::Add"));
    }

    /// Trace: FR-039-AC-6, TC-053.
    #[test]
    fn admitted_failed_accounting_refuses_without_an_oracle() {
        let checked = admitted_source(SOURCE);
        let expression = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.eq")
            })
            .expect("QSL equality expression")
            .node_id
            .clone();
        let mut limited = profile();
        limited.work_limit = 0;
        let lowered = checked.lower(std::slice::from_ref(&expression), &limited);
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("IR reader must admit typed failed accounting: {other:?}"),
        };
        let record = admitted.wire().feature_accounting[0].clone();
        assert!(matches!(&record, ContractFeatureRecordV1::Failed { .. }));
        let occurrences = admitted.wire().source_map.requested_items[0]
            .occurrences
            .clone();
        let generated = crate::oracle::scalar::generate_contract_scalar_oracles(
            &admitted,
            &drivers(std::slice::from_ref(&expression)),
            &[0],
        )
        .expect("typed failed item is a per-item refusal");
        assert!(matches!(
            &generated.claim_map.items[0].result,
            crate::oracle::claim::ClaimDisposition::Refused {
                refusal: crate::oracle::scalar::ExactScalarRefusal::PackageAccounting {
                    record: actual,
                    occurrences: actual_occurrences,
                },
            } if actual == &record && actual_occurrences == &occurrences
        ));
        assert!(!generated
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("oracle source")
            .contents
            .contains("pub fn oracle_"));
    }

    /// Trace: FR-039-AC-7, FR-039-AC-8, TC-053.
    #[test]
    fn genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys() {
        use crate::oracle::function::{
            generate_exact_function_oracles, ExactFunctionBody, ExactFunctionDeclaration,
            ExactFunctionItem, FunctionParameter,
        };
        use crate::oracle::scalar::IntegerOperator;

        let source = "language \"ix:native\" edition \"1-draft\";\n\
            profile v = \"quire.value.complete/v1\";\n\
            function add using v(a: Integer, b: Integer): Integer pure { a + b }\n\
            function caller using v(a: Integer, b: Integer): Integer pure { add(a, b) }\n";
        let checked = admitted_source(source);
        let addition = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.add")
            })
            .expect("QSL addition body")
            .node_id
            .clone();
        let call = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "expression" && node.semantic_form.as_ref() == "call"
            })
            .expect("QSL function call")
            .node_id
            .clone();
        let integer = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "scalar_type" && node.semantic_form.as_ref() == "integer"
            })
            .expect("QSL Integer type")
            .node_id
            .clone();
        let requested = vec![addition.clone(), call.clone()];
        let lowered = checked.lower(&requested, &routed_contract_lowering_profile());
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("function package must admit: {other:?}"),
        };
        let function = ExactFunctionDeclaration {
            node_id: addition,
            name: "add".to_owned(),
            parameters: ["a", "b"]
                .into_iter()
                .map(|name| FunctionParameter {
                    name: name.to_owned(),
                    type_node_id: integer.clone(),
                })
                .collect(),
            result_type: integer,
            body: ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        };
        let call_item = ExactFunctionItem {
            call_node_id: call.clone(),
            function: "add".to_owned(),
            argument_node_ids: vec![call.clone(), call.clone()],
        };
        let generated = generate_exact_function_oracles(
            &admitted,
            &drivers(&requested),
            &[0],
            &[function],
            &[1],
            &[call_item],
        )
        .expect("admitted function generation");
        let crate::oracle::claim::ClaimDisposition::Generated(claim) =
            &generated.claim_map.items[0].result
        else {
            panic!(
                "genuine call should generate: {:?}",
                generated.claim_map.items[0].result
            );
        };
        assert_eq!(claim.contract_package_identity, *admitted.identity());
        assert_eq!(claim.node_id, requested[1]);
        let ContractRootViewV1::Lowered(closure) = admitted
            .reached_closure(&requested[1])
            .expect("authoritative call closure")
        else {
            panic!("requested call must be lowered");
        };
        assert_eq!(claim.claims, closure.claims);
        assert_eq!(claim.bounds, closure.bounds);
        assert_eq!(
            claim.source_map,
            admitted.wire().source_map.requested_items[1].occurrences
        );
        assert!(generated
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("generated source")
            .contents
            .contains("pub fn oracle_"));
    }

    /// Trace: FR-039-AC-2, FR-039-AC-7, TC-053.
    #[test]
    fn one_routed_profile_admits_function_and_equality_beside_scalar_bound_refusal() {
        use crate::oracle::claim::ClaimDisposition;
        use crate::oracle::equality::{
            generate_composite_equality_oracles, CompositeEqualityItem, EqualityOperandDescriptor,
            EqualityOperatorKind,
        };
        use crate::oracle::function::{
            generate_exact_function_oracles, ExactFunctionBody, ExactFunctionDeclaration,
            ExactFunctionItem, FunctionParameter,
        };
        use crate::oracle::scalar::{
            generate_contract_scalar_oracles, ExactScalarRefusal, IntegerOperator,
        };

        let source = "language \"ix:native\" edition \"1-draft\";\n\
            profile v = \"quire.value.complete/v1\";\n\
            function add using v(a: Integer, b: Integer): Integer pure { a + b }\n\
            function caller using v(a: Integer, b: Integer): Integer pure { add(a, b) }\n\
            function same using v(a: Integer): Boolean pure { a = a }\n\
            function mixed using v(a: Int[0, 9], p: Integer): Integer pure { a + p }\n";
        let checked = admitted_source(source);
        let integer = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "scalar_type" && node.semantic_form.as_ref() == "integer"
            })
            .expect("Integer scalar type")
            .node_id
            .clone();
        let addition = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.add")
                    && node.semantic_type == integer
            })
            .expect("plain function addition")
            .node_id
            .clone();
        let unbounded = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.add")
                    && node
                        .body
                        .pointer("/arguments/1/target")
                        .and_then(|target| {
                            serde_json::from_value::<CheckedNodeId>(target.clone()).ok()
                        })
                        .and_then(|target| {
                            checked
                                .graph()
                                .nodes
                                .iter()
                                .find(|operand| operand.node_id == target)
                        })
                        .and_then(|operand| {
                            operand
                                .body
                                .pointer("/members/0/value/value")
                                .and_then(serde_json::Value::as_str)
                        })
                        == Some("p")
            })
            .expect("mixed addition with plain parameter p")
            .node_id
            .clone();
        let call = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "expression" && node.semantic_form.as_ref() == "call"
            })
            .expect("function call")
            .node_id
            .clone();
        let equality = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.eq")
            })
            .expect("QSL integer equality")
            .node_id
            .clone();
        let requested = vec![
            addition.clone(),
            call.clone(),
            equality.clone(),
            unbounded.clone(),
        ];
        let lowered = checked.lower(&requested, &routed_contract_lowering_profile());
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("one mixed package must admit: {other:?}"),
        };
        assert!(admitted
            .wire()
            .feature_accounting
            .iter()
            .all(|record| matches!(record, ContractFeatureRecordV1::Lowered { .. })));
        let function = ExactFunctionDeclaration {
            node_id: addition,
            name: "add".to_owned(),
            parameters: ["a", "b"]
                .into_iter()
                .map(|name| FunctionParameter {
                    name: name.to_owned(),
                    type_node_id: integer.clone(),
                })
                .collect(),
            result_type: integer.clone(),
            body: ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        };
        let functions = generate_exact_function_oracles(
            &admitted,
            &drivers(&requested),
            &[0],
            &[function],
            &[1],
            &[ExactFunctionItem {
                call_node_id: call,
                function: "add".to_owned(),
                argument_node_ids: vec![requested[1].clone(), requested[1].clone()],
            }],
        )
        .expect("function generation");
        assert!(matches!(
            functions.claim_map.items[0].result,
            ClaimDisposition::Generated(_)
        ));
        let equality = generate_composite_equality_oracles(
            &admitted,
            &drivers(&requested),
            &[2],
            &[CompositeEqualityItem {
                node_id: equality,
                operator: EqualityOperatorKind::Equal,
                left: EqualityOperandDescriptor::typed(integer.clone()),
                right: EqualityOperandDescriptor::typed(integer.clone()),
            }],
            Default::default(),
        )
        .expect("equality generation");
        assert!(matches!(
            equality.claim_map.items[0].result,
            ClaimDisposition::Generated(_)
        ));
        let scalar = generate_contract_scalar_oracles(&admitted, &drivers(&requested), &[3])
            .expect("scalar settlement");
        assert!(
            matches!(
                &scalar.claim_map.items[0].result,
                ClaimDisposition::Refused {
                    refusal: ExactScalarRefusal::RequiresBound { unbounded_type }
                } if *unbounded_type == integer
            ),
            "actual scalar decision: {:?}",
            scalar.claim_map.items[0].result
        );
        assert_eq!(scalar.claim_map.items[0].node_id, unbounded);
        assert!(!scalar.artifacts[1].contents.contains("pub fn oracle_"));
        let kani = BackendDescriptor {
            identity: "kani".to_owned(),
            advertised: Vec::new(),
            origin: ProviderOrigin::Linked,
            domains: None,
            bounds: Default::default(),
        };
        let routed = [RoutedGenerationItem::from_descriptor(
            3,
            unbounded,
            Candidate {
                identity: "kani".to_owned(),
            },
            &kani,
        )
        .expect("linked Kani route")];
        let contexts = GenerationContexts {
            kani: Some(KaniGenerationContext {
                ceilings: crate::kani::identity::ProofCeilings {
                    memory_bytes: std::num::NonZeroU64::new(64 * 1024 * 1024)
                        .expect("nonzero ceiling"),
                    wall_clock: std::time::Duration::from_secs(10),
                },
                subject_path: "crate::subject",
                unwind: 1,
            }),
        };
        let generated = generate_routed(&admitted, &drivers(&requested), &routed, &contexts)
            .expect("Kani produces a typed backend refusal");
        let [output] = generated.items.as_slice() else {
            panic!("one routed driver item");
        };
        assert_eq!(output.request_index, 3);
        assert_eq!(
            output.admitted.record,
            admitted.wire().feature_accounting[3]
        );
        assert_eq!(
            output.admitted.occurrences,
            admitted.wire().source_map.requested_items[3].occurrences
        );
        assert!(matches!(
            &output.output,
            KindOutput::Kani { record, harness }
                if matches!(record.disposition,
                    crate::kani::generate::outcome::ObligationDisposition::RequiresBound { .. })
                    && harness.is_none()
        ));
    }

    struct SelectedFixture {
        checked_bytes: Vec<u8>,
        library: CheckedPackageV2,
        checked: Box<CheckedPackageV2>,
        target: CheckedNodeId,
        second_target: CheckedNodeId,
        call: CheckedNodeId,
        evidence: CheckedPackageEvidence,
    }

    fn selected_fixture() -> SelectedFixture {
        let library_source = format!(
            "{}function h using v(a: Int[0, 9]): Boolean pure {{ a > 1 }}\n",
            SOURCE.replace("a = a", "a < 5")
        );
        let library_compiled = compile_package(
            SourceIdentity::new("test", "geometry", "git", "fixture"),
            "geometry.native",
            library_source.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("library compiles with its own identity");
        let mut library_evidence = CheckedPackageEvidence::new();
        library_evidence.support_feature("quire.value.complete/v1");
        let library = match CheckedPackageV2::read(
            library_compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &library_evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("library source must admit: {other:?}"),
        };
        let target = library
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == "f"
                })
            })
            .expect("library function")
            .node_id
            .clone();
        let second_target = library
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == "h"
                })
            })
            .expect("second library function")
            .node_id
            .clone();
        let import = "import \"test/geometry\" as g;\n";
        let program_source = format!(
            "language \"ix:native\" edition \"1-draft\";\n\
             profile v = \"quire.value.complete/v1\";\n\
             {import}\
             function f using v(a: Int[0, 9]): Boolean pure {{ a < 5 }}\n\
             function p using v(a: Int[0, 9]): Boolean pure {{ g::f(a) and g::h(a) }}\n"
        );
        let dependencies = DependencyInput::new([SuppliedLibrary {
            identity: "test/geometry".to_owned(),
            source: SourceIdentity::new("test", "geometry", "git", "fixture"),
            path: "geometry.native".to_owned(),
            bytes: library_source.into_bytes(),
        }])
        .expect("one supplied library");
        let compiled = compile_package(
            SourceIdentity::new("test", "program", "git", "fixture"),
            "program.native",
            program_source.as_bytes(),
            [],
            &dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("imported call compiles");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        evidence.insert_dependency_package("test/geometry", std::sync::Arc::new(library.clone()));
        let checked = match CheckedPackageV2::read(
            compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => package,
            other => panic!("selected call must admit: {other:?}"),
        };
        assert!(checked
            .graph()
            .nodes
            .iter()
            .any(|node| node.declaration.as_ref().is_some_and(|declaration| {
                declaration.qualified_name.len() == 1
                    && declaration.qualified_name[0].as_ref() == "f"
            })));
        let call = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == "p"
                })
            })
            .expect("function reaching both imported calls")
            .node_id
            .clone();
        SelectedFixture {
            checked_bytes: compiled.bytes().to_vec(),
            library,
            checked,
            target,
            second_target,
            call,
            evidence,
        }
    }

    /// Trace: FR-039-AC-3, TC-053.
    #[test]
    fn selected_function_targets_keep_reader_identity_and_order() {
        let SelectedFixture {
            checked_bytes: _,
            library,
            checked,
            target,
            second_target,
            call,
            evidence: _,
        } = selected_fixture();
        let lowered = checked.lower(std::slice::from_ref(&call), &profile());
        let bytes = lowered.package.canonical_bytes().expect("FR-330 bytes");
        assert!(matches!(
            ContractPackageWireV1::read(
                bytes,
                CheckedPackageReadLimits::bounded(),
                &ContractDependencies::new(),
            ),
            ContractPackageReadResult::Refused(refusal)
                if refusal.code == CheckedPackageRefusalCode::MissingImport
        ));
        let mut selected_contract = ContractDependencies::new();
        selected_contract.insert(
            "test/geometry",
            ContractDependencyInput {
                identity: "test/geometry",
                package: &library,
            },
        );
        let admitted = match ContractPackageWireV1::read(
            bytes,
            CheckedPackageReadLimits::bounded(),
            &selected_contract,
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("FR-330 reader must admit selected call: {other:?}"),
        };
        let items = PackageItems::new(&admitted, &drivers(std::slice::from_ref(&call)))
            .expect("exact request identity");
        let PackageItem::Lowered { closure, .. } = items.at(0, &call).expect("root view") else {
            panic!("selected call must lower");
        };
        assert_eq!(closure.selected_references.len(), 2);
        assert!(closure.selected_references.iter().any(|selected| {
            selected.package == *library.package_id() && selected.node == target
        }));
        assert!(closure.selected_references.iter().any(|selected| {
            selected.package == *library.package_id() && selected.node == second_target
        }));
        assert!(closure
            .nodes
            .iter()
            .all(|entry| entry.node.node_id != target));
        let selected = closure
            .selected_references
            .into_iter()
            .map(|reference| (reference.package, reference.node))
            .collect::<Vec<_>>();

        let generated = crate::oracle::scalar::generate_contract_scalar_oracles(
            &admitted,
            &drivers(std::slice::from_ref(&call)),
            &[0],
        )
        .expect("backend returns a typed result");
        assert!(matches!(
            &generated.claim_map.items[0].result,
            crate::oracle::claim::ClaimDisposition::Refused {
                refusal: crate::oracle::scalar::ExactScalarRefusal::SelectedDependencyUnsupported {
                    references,
                },
            } if references == &selected
        ));

        selected_contract.insert(
            "test/unselected",
            ContractDependencyInput {
                identity: "test/unselected",
                package: &library,
            },
        );
        let extra = match ContractPackageWireV1::read(
            bytes,
            CheckedPackageReadLimits::bounded(),
            &selected_contract,
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("unselected evidence must not replace selected targets: {other:?}"),
        };
        let extra_generated = crate::oracle::scalar::generate_contract_scalar_oracles(
            &extra,
            &drivers(std::slice::from_ref(&call)),
            &[0],
        )
        .expect("extra unselected package does not redirect CG");
        assert_eq!(extra.identity(), admitted.identity());
        assert_eq!(extra_generated, generated);
    }

    /// Trace: FR-039-AC-3, TC-053.
    #[test]
    fn checked_reader_refuses_forged_same_key_local_function() {
        let SelectedFixture {
            checked_bytes,
            library: _,
            checked,
            target,
            second_target: _,
            call: _,
            evidence,
        } = selected_fixture();
        // The source compiler gives independent local and library declarations
        // different keys. Rebinding the local key in canonical checked wire
        // must be refused by Contract IR before the forged pair can reach CG.
        let local = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name.len() == 1
                        && declaration.qualified_name[0].as_ref() == "f"
                })
            })
            .expect("local function")
            .node_id
            .clone();
        assert_ne!(local, target);
        let mut collision: serde_json::Value =
            serde_json::from_slice(&checked_bytes).expect("QSL checked wire");
        replace_exact_digest(&mut collision, &local.digest, &target.digest);
        for pointer in ["/semantic_graph/nodes", "/source_map"] {
            collision
                .pointer_mut(pointer)
                .and_then(serde_json::Value::as_array_mut)
                .expect("checked wire array")
                .sort_by(|left, right| {
                    left["node_id"]["digest"]
                        .as_str()
                        .cmp(&right["node_id"]["digest"].as_str())
                });
        }
        let projection = collision["semantic_graph"]["nodes"]
            .as_array()
            .expect("graph nodes")
            .iter()
            .cloned()
            .map(|mut node| {
                node.as_object_mut()
                    .expect("graph node")
                    .remove("occurrences");
                node
            })
            .collect();
        collision["identity_preimage"]["identity_projection"] =
            serde_json::Value::Array(projection);
        collision["package_id"]["digest"] = serde_json::Value::String(
            quire_canonical::sha256(
                &collision["identity_preimage"],
                quire_canonical::Limits::new(1 << 24),
            )
            .expect("identity preimage canonicalizes")
            .to_string(),
        );
        let collision_bytes =
            quire_canonical::to_vec(&collision, quire_canonical::Limits::new(1 << 24))
                .expect("checked wire canonicalizes");
        let refusal = match CheckedPackageV2::read(
            &collision_bytes,
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Refused(refusal) => refusal,
            other => panic!("forged same-key wire must refuse before CG: {other:?}"),
        };
        assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
        assert_eq!(
            refusal.cause,
            Some(quire_contract_model::CheckedPackageRefusalCause::StaleNodeKey)
        );
        assert_eq!(refusal.locus, Some(target));
        assert_eq!(refusal.expected_node_id(), Some(&local));
        assert_eq!(
            refusal.path.as_ref().map(|path| path.as_str()),
            Some("/semantic_graph/nodes/11/node_id")
        );
    }
}
