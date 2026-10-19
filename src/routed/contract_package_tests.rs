// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-039 admitted-package routing controls.

#[cfg(test)]
mod tests {
    use crate::core::contract_package::{
        routed_contract_lowering_profile, ContractDriverResultSelection, ContractRequestPosition,
        PackageItem, PackageItems, PackageRequestError,
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
    use ix_trace_rs::trace;
    use qsl_replay::{
        compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
        SuppliedLibrary,
    };
    use quire_contract_model::{
        CheckedNodeId, CheckedNodeTag, CheckedOccurrenceRole, CheckedPackageEvidence,
        CheckedPackageReadLimits, CheckedPackageRefusalCode, CheckedPackageV2,
        CheckedPackageV2ReadResult, CompleteLoweringProfileV2, ContractDependencies,
        ContractDependencyInput, ContractFeatureRecordV1, ContractOccurrenceKeyV1,
        ContractPackageReadResult, ContractPackageWireV1, ContractRootViewV1, LoweringProfileRef,
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
            .map(|node_id| ContractRequestPosition::DriverItem {
                node_id,
                result_selection: ContractDriverResultSelection::NotApplicable,
            })
            .collect()
    }

    fn source_call_arguments(
        checked: &CheckedPackageV2,
        call_id: &CheckedNodeId,
    ) -> Vec<CheckedNodeId> {
        let call = checked
            .graph()
            .nodes
            .iter()
            .find(|node| &node.node_id == call_id)
            .expect("checked source call");
        call.body
            .get("arguments")
            .and_then(serde_json::Value::as_array)
            .expect("source call arguments")
            .iter()
            .skip(1)
            .map(|argument| {
                crate::oracle::function::reference_node_id(argument, checked.graph().nodes.iter())
                    .expect("call argument reference")
                    .clone()
            })
            .collect()
    }

    fn source_function_body(checked: &CheckedPackageV2, name: &str) -> CheckedNodeId {
        let function = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "function"
                    && node.semantic_form.as_ref() == "pure_function"
                    && node.declaration.as_ref().is_some_and(|declaration| {
                        declaration.qualified_name.len() == 1
                            && declaration.qualified_name[0].as_ref() == name
                    })
            })
            .expect("named checked function");
        let body = crate::oracle::function::binding_value(&function.body, "body")
            .expect("function body binding");
        crate::oracle::function::reference_node_id(body, checked.graph().nodes.iter())
            .expect("function body reference")
            .clone()
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
            panic!("one QSL expression occurrence required for selected key");
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

    fn empty_backend_request(
        count: usize,
        package: &CheckedPackageV2,
        node_id: &CheckedNodeId,
    ) -> BackendProviderEnvelope {
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
                    kani_form: crate::routed::capability::KaniForm::OutsideSet {
                        package: Box::new(package.clone()),
                        node_id: node_id.clone(),
                    },
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
                result_selection: ContractDriverResultSelection::NotApplicable,
            },
            ContractRequestPosition::ContextRoot {
                node_id: function.clone(),
            },
            ContractRequestPosition::DriverItem {
                node_id: absent.clone(),
                result_selection: ContractDriverResultSelection::NotApplicable,
            },
            ContractRequestPosition::DriverItem {
                node_id: function.clone(),
                result_selection: ContractDriverResultSelection::NotApplicable,
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
        let settled = negotiate_contract_package(
            &admitted,
            &positions,
            &empty_backend_request(3, &source, &function),
        )
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
            bounds: None,
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
        swapped[0] = ContractRequestPosition::DriverItem {
            node_id: absent,
            result_selection: ContractDriverResultSelection::NotApplicable,
        };
        assert!(matches!(
            PackageItems::new(&admitted, &swapped),
            Err(PackageRequestError::AccountingIdentity {
                accounting_index: 0,
                ..
            })
        ));
        swapped[0] = ContractRequestPosition::DriverItem {
            node_id: function.clone(),
            result_selection: ContractDriverResultSelection::NotApplicable,
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
            let positions = drivers(std::slice::from_ref(&expression));
            let items = PackageItems::new(&admitted, &positions).expect("exact request");
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

    #[trace("FR-039-AC-1", "FR-039-AC-2", "TC-053")]
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
                bounds: None,
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
            let mut positions = drivers(&requested);
            positions[0] = ContractRequestPosition::DriverItem {
                node_id: requested[0].clone(),
                result_selection: own_result(&checked, &requested[0], None),
            };
            let routes = routes
                .into_iter()
                .enumerate()
                .map(|(index, route)| {
                    let ContractRequestPosition::DriverItem {
                        result_selection, ..
                    } = &positions[index]
                    else {
                        unreachable!("driver position")
                    };
                    route.with_scalar_selection(&admitted, result_selection.clone())
                })
                .collect::<Vec<_>>();
            let generated = generate_routed(&admitted, &positions, &routes, &contexts)
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

    /// A scalar caller must select its own checked application occurrence.
    #[trace("FR-039-AC-2", "FR-049-AC-1", "TC-053", "TC-001", "NFR-002-AC-1", "NFR-002-AC-2")]
    #[test]
    fn scalar_own_result_without_conversion_differs_from_not_applicable() {
        use crate::oracle::claim::ClaimDisposition;
        use crate::oracle::scalar::{generate_contract_scalar_oracles, ExactScalarRefusal};

        let checked = admitted_source(&SOURCE.replace("a = a", "a < 5"));
        let expression = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.integer.lt")
            })
            .expect("QSL ordering application")
            .node_id
            .clone();
        let lowered = checked.lower(
            std::slice::from_ref(&expression),
            &routed_contract_lowering_profile(),
        );
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("IR must admit ordering: {other:?}"),
        };
        let (omitted, _) = generate_contract_scalar_oracles(
            &admitted,
            &drivers(std::slice::from_ref(&expression)),
            &[0],
        )
        .expect("typed scalar refusal");
        assert!(matches!(
            &omitted.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: ExactScalarRefusal::IncompleteContext
            }
        ));
        let selected = [ContractRequestPosition::DriverItem {
            node_id: expression.clone(),
            result_selection: own_result(&checked, &expression, None),
        }];
        let (generated, _) = generate_contract_scalar_oracles(&admitted, &selected, &[0])
            .expect("selected scalar generation");
        let lib = generated.artifacts.iter().find(|artifact| artifact.path == "src/lib.rs")
            .expect("contract scalar library");
        let subject = format!("{}/{}", expression.domain, expression.digest);
        let header = lib.contents.lines().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
        assert!(header.contains(&"// SPDX-License-Identifier: MIT OR Apache-2.0"));
        assert!(header.iter().any(|line| line.starts_with("// @generated by quire-contract-codegen")));
        assert!(header.contains(&format!("// Subject: {subject}").as_str()));
        assert!(matches!(
            &generated.claim_map.items[0].result,
            ClaimDisposition::Generated(_)
        ));
    }

    /// RR-16 supplies two result contexts for one semantic add key.
    #[trace(
        "FR-039-AC-2",
        "FR-049-AC-1",
        "FR-049-AC-5",
        "FR-019-AC-43",
        "FR-019-AC-44",
        "FR-022-AC-23",
        "FR-022-AC-24",
        "TC-030",
        "TC-033",
        "TC-053"
    )]
    #[test]
    fn duplicate_expression_uses_each_selected_conversion_context() {
        use crate::oracle::claim::ClaimDisposition;
        use crate::oracle::scalar::{generate_contract_scalar_oracles, ExactScalarRefusal};

        let checked = admitted_source(
            "language \"ix:native\" edition \"1-draft\";\n\
             profile v = \"quire.value.complete/v1\";\n\
             function g using v(p: Int[0, 10]): Boolean pure { true }\n\
             function h using v(q: Int[0, 20]): Boolean pure { true }\n\
             function f using v(x: Int[0, 9]): Boolean pure { g(x + 1) and h(x + 1) }\n",
        );
        let adds = checked
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
        let [add] = adds.as_slice() else {
            panic!("RR-16 must share one addition node");
        };
        let add = add.node_id.clone();
        let applications = checked
            .source_map()
            .iter()
            .filter(|entry| entry.node_id == add && entry.role == CheckedOccurrenceRole::Expression)
            .map(|entry| ContractOccurrenceKeyV1 {
                node_id: add.clone(),
                role: entry.role.clone(),
                ordinal: entry.ordinal,
            })
            .collect::<Vec<_>>();
        let [first, second] = applications.as_slice() else {
            panic!("RR-16 must retain two source occurrences");
        };
        assert_ne!(first.ordinal, second.ordinal);
        let mut conversions = checked
            .graph()
            .nodes
            .iter()
            .filter(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.numeric.narrow")
                    && node.body.pointer("/arguments/0/target")
                        == Some(&serde_json::to_value(&add).expect("typed key"))
            })
            .map(|node| {
                let bound = checked
                    .graph()
                    .nodes
                    .iter()
                    .find(|bound| bound.node_id == node.semantic_type)
                    .expect("narrow target bound");
                let maximum = bound
                    .body
                    .pointer("/members/1/value/value")
                    .and_then(serde_json::Value::as_str)
                    .expect("integer upper endpoint");
                (
                    maximum.to_owned(),
                    node.node_id.clone(),
                    bound.node_id.clone(),
                )
            })
            .collect::<Vec<_>>();
        conversions.sort_by(|left, right| left.0.cmp(&right.0));
        let [(first_max, first_conversion, first_bound), (second_max, second_conversion, second_bound)] =
            conversions.as_slice()
        else {
            panic!("RR-16 must have two direct narrows");
        };
        assert_eq!((first_max.as_str(), second_max.as_str()), ("10", "20"));
        assert_ne!(first_bound, second_bound);
        let roots = [
            add.clone(),
            first_conversion.clone(),
            add.clone(),
            second_conversion.clone(),
        ];
        let lowered = checked.lower(&roots, &routed_contract_lowering_profile());
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("RR-16 IR package must admit: {other:?}"),
        };
        let positions = [
            ContractRequestPosition::DriverItem {
                node_id: add.clone(),
                result_selection: ContractDriverResultSelection::OwnResult {
                    application: first.clone(),
                    conversion: Some(occurrence(&checked, first_conversion)),
                },
            },
            ContractRequestPosition::ContextRoot {
                node_id: first_conversion.clone(),
            },
            ContractRequestPosition::DriverItem {
                node_id: add.clone(),
                result_selection: ContractDriverResultSelection::OwnResult {
                    application: second.clone(),
                    conversion: Some(occurrence(&checked, second_conversion)),
                },
            },
            ContractRequestPosition::ContextRoot {
                node_id: second_conversion.clone(),
            },
        ];
        let (result, _) = generate_contract_scalar_oracles(&admitted, &positions, &[0, 1])
            .expect("two selected scalar items");
        for (item, expected) in result
            .claim_map
            .items
            .iter()
            .zip([first_bound, second_bound])
        {
            let ClaimDisposition::Generated(claim) = &item.result else {
                panic!(
                    "each occurrence generates from its own context: {:?}",
                    item.result
                );
            };
            assert_eq!(claim.checked_bounds.first(), Some(expected));
        }
        // The same admitted source and node must settle each selected result
        // occurrence independently before a generation artifact exists.
        // Trace: FR-019-AC-43, FR-019-AC-44, TC-030.
        let settle = |source: &CheckedPackageV2,
                      positions: Vec<ContractRequestPosition>,
                      request_index: usize| {
            crate::routed::capability::negotiate_backend_provider(&BackendProviderEnvelope {
                contract_version: BACKEND_PROVIDER_CONTRACT.to_owned(),
                capability_vocabulary: CAPABILITY_VOCABULARY.to_owned(),
                manifest: vec![BackendDescriptor {
                    identity: "kani".to_owned(),
                    advertised: vec![(
                        crate::routed::capability::CapabilityKind::ValueValidity,
                        crate::routed::capability::Mode::Bounded,
                    )],
                    origin: ProviderOrigin::Linked,
                    domains: None,
                    bounds: None,
                }],
                items: vec![RequestItem {
                    kind: RequestedKind::Known(
                        crate::routed::capability::CapabilityKind::ValueValidity,
                    ),
                    extent: Some(crate::routed::capability::ExtentClassification {
                        extent: crate::routed::capability::Mode::Bounded,
                        finite_bound_available: true,
                        domains: Vec::new(),
                        bounds: Vec::new(),
                    }),
                    named_backend: None,
                    candidates: Candidates::Set(vec![Candidate {
                        identity: "kani".to_owned(),
                    }]),
                    kani_form: crate::routed::capability::KaniForm::V2ScalarClaim {
                        package: Box::new(source.clone()),
                        contract: admitted.clone(),
                        positions,
                        request_index,
                        node_id: add.clone(),
                    },
                }],
            })
            .expect("valid capability envelope")
            .remove(0)
            .disposition
        };
        for request_index in 0..2 {
            assert!(
                matches!(
                    settle(&checked, positions.to_vec(), request_index),
                    crate::routed::capability::Disposition::Supported { .. }
                ),
                "selected occurrence {request_index} must settle"
            );
        }
        let mut missing = positions.to_vec();
        if let ContractRequestPosition::DriverItem {
            result_selection, ..
        } = &mut missing[2]
        {
            *result_selection = ContractDriverResultSelection::NotApplicable;
        }
        assert!(matches!(
            settle(&checked, missing, 1),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal:
                        crate::kani::generate::outcome::InvalidObligationItem::ScalarResultMissing,
                    ..
                }
            }
        ));
        let mut cross_context = positions.to_vec();
        if let ContractRequestPosition::DriverItem {
            result_selection, ..
        } = &mut cross_context[2]
        {
            *result_selection = ContractDriverResultSelection::OwnResult {
                application: second.clone(),
                conversion: Some(occurrence(&checked, first_conversion)),
            };
        }
        assert!(matches!(
            settle(&checked, cross_context.clone(), 1),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal:
                        crate::kani::generate::outcome::InvalidObligationItem::ScalarContextMismatch,
                    ..
                }
            }
        ));
        assert!(matches!(
            settle(&checked, cross_context.clone(), 0),
            crate::routed::capability::Disposition::Supported { .. }
        ));
        let foreign_checked = admitted_source(SOURCE);
        assert!(matches!(
            settle(&foreign_checked, positions.to_vec(), 0),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal:
                        crate::kani::generate::outcome::InvalidObligationItem::ScalarSourceMismatch,
                    ..
                }
            }
        ));
        let mut absent_application = positions.to_vec();
        if let ContractRequestPosition::DriverItem {
            result_selection: ContractDriverResultSelection::OwnResult { application, .. },
            ..
        } = &mut absent_application[0]
        {
            application.ordinal = u64::MAX;
        }
        assert!(matches!(
            settle(&checked, absent_application, 0),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal:
                        crate::kani::generate::outcome::InvalidObligationItem::ScalarContextMismatch,
                    ..
                }
            }
        ));
        let mut foreign_conversion = positions.to_vec();
        if let ContractRequestPosition::DriverItem {
            result_selection:
                ContractDriverResultSelection::OwnResult {
                    conversion: Some(conversion),
                    ..
                },
            ..
        } = &mut foreign_conversion[0]
        {
            conversion.node_id = foreign_checked
                .graph()
                .nodes
                .iter()
                .find(|node| &*node.node_tag == "expression")
                .expect("foreign expression")
                .node_id
                .clone();
        }
        assert!(matches!(
            settle(&checked, foreign_conversion, 0),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal:
                        crate::kani::generate::outcome::InvalidObligationItem::ScalarContextMismatch,
                    ..
                }
            }
        ));
        let mut missing_root = positions.to_vec();
        missing_root.remove(1);
        assert!(matches!(
            settle(&checked, missing_root, 0),
            crate::routed::capability::Disposition::InvalidRequest {
                cause: crate::routed::capability::Cause::KaniFormInvalid {
                    refusal: crate::kani::generate::outcome::InvalidObligationItem::ScalarPositionMismatch,
                    ..
                }
            }
        ));
        let descriptor = BackendDescriptor {
            identity: "kani".to_owned(),
            advertised: Vec::new(),
            origin: ProviderOrigin::Linked,
            domains: None,
            bounds: None,
        };
        let routes = (0..2)
            .map(|request_index| {
                RoutedGenerationItem::from_descriptor(
                    request_index,
                    add.clone(),
                    Candidate {
                        identity: "kani".to_owned(),
                    },
                    &descriptor,
                )
                .expect("linked Kani route")
                .with_scalar_selection(
                    &admitted,
                    match &positions[request_index * 2] {
                        ContractRequestPosition::DriverItem {
                            result_selection, ..
                        } => result_selection.clone(),
                        ContractRequestPosition::ContextRoot { .. } => {
                            unreachable!("driver position")
                        }
                    },
                )
            })
            .collect::<Vec<_>>();
        let changed_selection = match &cross_context[2] {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => result_selection.clone(),
            ContractRequestPosition::ContextRoot { .. } => unreachable!("second driver"),
        };
        let changed_routes = [
            routes[0].clone(),
            routes[1]
                .clone()
                .with_scalar_selection(&admitted, changed_selection),
        ];
        assert!(matches!(
            generate_routed(
                &admitted,
                &cross_context,
                &changed_routes,
                &GenerationContexts {
                    kani: Some(KaniGenerationContext {
                        ceilings: crate::kani::identity::ProofCeilings {
                            memory_bytes: std::num::NonZeroU64::new(64 * 1024 * 1024)
                                .expect("nonzero"),
                            wall_clock: std::time::Duration::from_secs(10),
                        },
                        subject_path: "crate::subject",
                        unwind: 1,
                    }),
                },
            ),
            Err(
                crate::routed::generate::RoutedGenerationError::ScalarSelectionMismatch {
                    request_index: 1
                }
            )
        ));
        let routed = generate_routed(
            &admitted,
            &positions,
            &routes,
            &GenerationContexts {
                kani: Some(KaniGenerationContext {
                    ceilings: crate::kani::identity::ProofCeilings {
                        memory_bytes: std::num::NonZeroU64::new(64 * 1024 * 1024)
                            .expect("nonzero ceiling"),
                        wall_clock: std::time::Duration::from_secs(10),
                    },
                    subject_path: "crate::subject",
                    unwind: 1,
                }),
            },
        )
        .expect("both selected scalar contexts route");
        assert!(
            routed.rejected.is_empty(),
            "distinct selections are independent: {:?}",
            routed.items
        );
        assert_eq!(routed.items.len(), 2);
        let mut harness_paths = Vec::new();
        for (item, maximum) in routed.items.iter().zip([10, 20]) {
            let KindOutput::Kani { record, harness } = &item.output else {
                panic!("selected scalar routes to Kani");
            };
            assert!(
                matches!(
                    record.disposition,
                    crate::kani::generate::outcome::ObligationDisposition::Supported { .. }
                ),
                "each selected occurrence has its own supported disposition: {record:?}"
            );
            let harness = harness
                .as_ref()
                .expect("each selected occurrence has a harness");
            assert!(
                harness
                    .rust
                    .contents
                    .contains(&format!("exact <= i128::from({maximum}_i64)")),
                "each proof uses its own selected result bound: {}",
                harness.rust.contents
            );
            harness_paths.push(&harness.rust.path);
        }
        assert_ne!(harness_paths[0], harness_paths[1]);
        let mut forged = positions.clone();
        if let ContractRequestPosition::DriverItem {
            result_selection, ..
        } = &mut forged[2]
        {
            *result_selection = ContractDriverResultSelection::OwnResult {
                application: second.clone(),
                conversion: Some(occurrence(&checked, first_conversion)),
            };
        }
        let (refusal, _) = generate_contract_scalar_oracles(&admitted, &forged, &[1])
            .expect("cross-context choice is a per-item refusal");
        assert!(matches!(
            &refusal.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: ExactScalarRefusal::IncompleteContext
            }
        ));

        let repeated_roots = [
            add.clone(),
            first_conversion.clone(),
            add.clone(),
            first_conversion.clone(),
        ];
        let repeated_lowered = checked.lower(&repeated_roots, &routed_contract_lowering_profile());
        let repeated = match ContractPackageWireV1::read(
            repeated_lowered
                .package
                .canonical_bytes()
                .expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("repeated selected package must admit: {other:?}"),
        };
        let repeated_positions = [
            positions[0].clone(),
            positions[1].clone(),
            positions[0].clone(),
            positions[1].clone(),
        ];
        let (duplicate, _) =
            generate_contract_scalar_oracles(&repeated, &repeated_positions, &[0, 1])
                .expect("duplicate pair receives typed refusals");
        assert!(duplicate.claim_map.items.iter().all(|item| matches!(
            &item.result,
            ClaimDisposition::Refused {
                refusal: ExactScalarRefusal::DuplicateRequest
            }
        )));
    }

    #[trace("FR-039-AC-1", "TC-053")]
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
                        == Some(&serde_json::to_value(&expression).expect("typed key"))
            })
            .collect::<Vec<_>>();
        let [conversion] = conversions.as_slice() else {
            panic!("QSL must select one direct narrow for bounded result");
        };
        let conversion = conversion.node_id.clone();
        let roots = [expression.clone(), conversion.clone()];
        let lowered = checked.lower(&roots, &routed_contract_lowering_profile());
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().expect("FR-330 bytes"),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("IR must admit bounded expression: {other:?}"),
        };
        let positions = [
            ContractRequestPosition::DriverItem {
                node_id: expression.clone(),
                result_selection: own_result(&checked, &expression, Some(&conversion)),
            },
            ContractRequestPosition::ContextRoot {
                node_id: conversion,
            },
        ];
        let (generated, _) =
            crate::oracle::scalar::generate_contract_scalar_oracles(&admitted, &positions, &[0])
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

    #[trace("FR-039-AC-6", "TC-053")]
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
        let (generated, _) = crate::oracle::scalar::generate_contract_scalar_oracles(
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
            } if actual.as_ref() == &record && actual_occurrences == &occurrences
        ));
        assert!(!generated
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("oracle source")
            .contents
            .contains("pub fn oracle_"));
    }

    #[trace("FR-039-AC-7", "FR-039-AC-8", "TC-053")]
    #[test]
    fn genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys() {
        use crate::oracle::function::{
            generate_exact_function_oracles, ExactFunctionBody, ExactFunctionDeclaration,
            ExactFunctionItem, FunctionParameter,
        };
        use crate::oracle::operator_catalog::IntegerOperator;

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
        let call_arguments = source_call_arguments(&checked, &call);
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
            argument_node_ids: call_arguments,
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

    #[trace("FR-039-AC-2", "FR-039-AC-7", "TC-053")]
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
        use crate::oracle::operator_catalog::IntegerOperator;
        use crate::oracle::scalar::{generate_contract_scalar_oracles, ExactScalarRefusal};

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
        let addition = source_function_body(&checked, "add");
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
        let call_arguments = source_call_arguments(&checked, &call);
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
                argument_node_ids: call_arguments,
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
        let mut positions = drivers(&requested);
        positions[3] = ContractRequestPosition::DriverItem {
            node_id: unbounded.clone(),
            result_selection: own_result(&checked, &unbounded, None),
        };
        let (scalar, _) = generate_contract_scalar_oracles(&admitted, &positions, &[3])
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
            bounds: None,
        };
        let routed = [RoutedGenerationItem::from_descriptor(
            3,
            unbounded,
            Candidate {
                identity: "kani".to_owned(),
            },
            &kani,
        )
        .expect("linked Kani route")
        .with_scalar_selection(
            &admitted,
            PackageItems::new(&admitted, &positions)
                .expect("admitted positions")
                .result_selection(3)
                .expect("fourth driver selection")
                .clone(),
        )];
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
        let generated = generate_routed(&admitted, &positions, &routed, &contexts)
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
        evidence
            .insert_dependency_package("test/geometry", std::sync::Arc::new(library.clone()))
            .expect("retain imported geometry package");
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

    #[trace("FR-039-AC-3", "TC-053")]
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
        let positions = drivers(std::slice::from_ref(&call));
        let items = PackageItems::new(&admitted, &positions).expect("exact request identity");
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

        let (generated, _) = crate::oracle::scalar::generate_contract_scalar_oracles(
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
        let (extra_generated, _) = crate::oracle::scalar::generate_contract_scalar_oracles(
            &extra,
            &drivers(std::slice::from_ref(&call)),
            &[0],
        )
        .expect("extra unselected package does not redirect CG");
        assert_eq!(extra.identity(), admitted.identity());
        assert_eq!(extra_generated, generated);
    }

    #[trace("FR-039-AC-3", "TC-053")]
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
