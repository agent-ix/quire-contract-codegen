//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules.

use ix_trace_rs::trace;
use qsl_replay::{
    compile_package, DependencyInput, DomainKey, DomainKind, FiniteBound, Integer, ProofBound,
    ReplayLimits, SourceIdentity, StageLimits, WireNodeId,
};
use quire_contract_codegen::{
    negotiate_backend_provider, routed_contract_lowering_profile, BackendDescriptor, BackendKind,
    BackendProviderEnvelope, Candidate, Candidates, CapabilityKind, Cause,
    ContractDriverResultSelection, ContractRequestPosition, Disposition, EnvelopeRefusal,
    ExtentClassification, ExtentDomain, InvalidObligationItem, ItemSettlement, KaniForm, Mode,
    ProviderOrigin, RequestItem, RequestedKind, UnsupportedObligation, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies,
    ContractOccurrenceKeyV1, ContractPackageReadResult, ContractPackageWireV1, JsonPointer,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    os::unix::fs::PermissionsExt,
    process::Command,
};

#[allow(clippy::duplicate_mod)]
#[path = "../exact_function_support/package.rs"]
mod exact_function_fixture;

fn checked_operation_identity(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation.identity()
}

fn function_body_target(body: &CheckedTerm) -> Option<&CheckedNodeId> {
    let CheckedTerm::Aggregate { members } = body else {
        return None;
    };
    members.iter().find_map(|member| match member {
        CheckedTerm::Binding { name, value } if name.as_ref() == "body" => match value.as_ref() {
            CheckedTerm::Reference { target } => Some(target),
            _ => None,
        },
        _ => None,
    })
}

fn kani(advertised: Vec<(CapabilityKind, Mode)>) -> BackendDescriptor {
    BackendDescriptor {
        identity: BackendKind::Kani.identity().to_owned(),
        advertised,
        origin: ProviderOrigin::Linked,
        domains: None,
        bounds: None,
    }
}

#[trace("TC-030", "FR-019-AC-48")]
#[test]
fn reader_refusal_projection_preserves_every_public_field() {
    let reader = ContractPackageWireV1::read(
        b"{}",
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    );
    let ContractPackageReadResult::Refused(refusal) = reader else {
        panic!("malformed contract package must be refused: {reader:?}");
    };
    let projected = quire_contract_codegen::project_contract_package_refusal(&refusal);
    let wire = serde_json::to_value(EnvelopeRefusal::ContractPackageReadRefused {
        refusal: projected.clone(),
    })
    .expect("serialize typed refusal");
    assert_eq!(wire["code"], "contract_package_read_refused");
    let fields = &wire["cause"]["refusal"];
    assert_eq!(fields["code"], serde_json::to_value(refusal.code).unwrap());
    assert_eq!(fields["path"], serde_json::to_value(refusal.path).unwrap());
    assert_eq!(
        fields["cause"],
        serde_json::to_value(refusal.cause).unwrap()
    );
    assert_eq!(
        fields["locus"],
        serde_json::to_value(refusal.locus).unwrap()
    );
    assert_eq!(
        fields["contract_version"],
        serde_json::to_value(refusal.contract_version).unwrap()
    );
    assert_eq!(
        fields["document_pointer"],
        serde_json::to_value(refusal.document_pointer).unwrap()
    );
    assert_eq!(
        fields["expected"],
        serde_json::to_value(refusal.expected).unwrap()
    );
    assert_eq!(
        fields["actual"],
        serde_json::to_value(refusal.actual).unwrap()
    );
    assert!(fields.get("expected_node_id").is_none());
}

#[trace("TC-030", "FR-019-AC-48")]
#[test]
fn named_refusal_adapter_preserves_unknown_version() {
    let reader = ContractPackageWireV1::read(
        br#"{"contract_version":"unknown"}"#,
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    );
    let ContractPackageReadResult::Refused(refusal) = reader else {
        panic!("unknown version must refuse: {reader:?}");
    };
    let projected = quire_contract_codegen::project_contract_package_refusal(&refusal);
    assert_eq!(projected.contract_version.as_deref(), Some("unknown"));
}

#[trace("TC-030", "FR-019-AC-48")]
#[test]
fn refusal_adapter_preserves_selected_document_pointer() {
    use exact_function_fixture::{source_bound_call_package, PackageBuilder};
    use quire_contract_model::{CheckedPackageRefusalCause, CheckedPackageRefusalCode};
    let digest = "ab".repeat(32);
    let selection = serde_json::json!([{
        "identity": "acme/orders",
        "digest_domain": "sha256-jcs",
        "digest": digest,
    }]);
    let mut wire = source_bound_call_package().wire();
    wire["lock"]["model_selections"] = selection.clone();
    wire["identity_preimage"]["model_selections"] = selection;
    let (_, resolved, _) = PackageBuilder::from_wire(wire).read_resolved();
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(
            digest,
            br#"{"package":{"count":9007199254740993}}"#.to_vec(),
        )
        .expect("selected document evidence");
    let bytes = serde_json::to_vec(&resolved).expect("canonical fixture wire");
    let reader = CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence);
    let CheckedPackageV2ReadResult::Refused(refusal) = reader else {
        panic!("selected document number past 2^53 must refuse: {reader:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::NoncanonicalWire);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::InexactInteger)
    );
    assert_eq!(
        refusal.document_pointer.as_ref().map(JsonPointer::as_str),
        Some("/package/count")
    );
    let projected = quire_contract_codegen::project_contract_package_refusal(&refusal);
    assert_eq!(
        projected.document_pointer.as_deref(),
        Some("/package/count")
    );
}

#[trace("TC-030", "FR-019-AC-48")]
#[test]
fn refusal_adapter_preserves_reader_node_locus() {
    use exact_function_fixture::{source_bound_call_package, PackageBuilder};
    let mut wire = source_bound_call_package().raw_wire();
    let mut functions = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("fixture nodes")
        .iter_mut()
        .filter(|node| node["node_tag"] == "function")
        .collect::<Vec<_>>();
    assert!(functions.len() >= 2);
    let first_name = functions[0]["declaration"]["qualified_name"].clone();
    functions[1]["declaration"]["qualified_name"] = first_name;
    let (reader, _, _) = PackageBuilder::from_wire(wire).read_resolved();
    let CheckedPackageV2ReadResult::Refused(refusal) = reader else {
        panic!("duplicate function name must refuse: {reader:?}");
    };
    assert!(refusal.locus.is_some(), "reader must locate the node");
    let projected = quire_contract_codegen::project_contract_package_refusal(&refusal);
    assert_eq!(projected.locus, refusal.locus);
}

#[trace("TC-030", "FR-019-AC-49")]
#[test]
fn incomplete_adapter_preserves_present_count_limit_path() {
    use exact_function_fixture::{code_id, source_bound_call_package, SOURCE_CALL};
    let (checked, ids) = source_bound_call_package().admit_resolved();
    let lowered = checked.lower(
        &[ids.resolve(&code_id(SOURCE_CALL))],
        &routed_contract_lowering_profile(),
    );
    let mut limits = CheckedPackageReadLimits::bounded();
    limits.nodes = 0;
    let reader = ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("lowered bytes"),
        limits,
        &ContractDependencies::new(),
    );
    let ContractPackageReadResult::Incomplete(incomplete) = reader else {
        panic!("node ceiling must be incomplete: {reader:?}");
    };
    assert!(incomplete.path.is_some());
    let projected = quire_contract_codegen::project_contract_package_incomplete(&incomplete);
    assert_eq!(
        projected.path.as_deref(),
        incomplete.path.as_ref().map(JsonPointer::as_str)
    );
}

#[trace("TC-030", "FR-019-AC-49")]
#[test]
fn reader_incomplete_projection_preserves_the_exhausted_limit() {
    let mut limits = CheckedPackageReadLimits::bounded();
    limits.bytes = 1;
    let reader = ContractPackageWireV1::read(b"{}", limits, &ContractDependencies::new());
    let ContractPackageReadResult::Incomplete(incomplete) = reader else {
        panic!("byte ceiling must be incomplete: {reader:?}");
    };
    let projected = quire_contract_codegen::project_contract_package_incomplete(&incomplete);
    let wire = serde_json::to_value(EnvelopeRefusal::ContractPackageReadIncomplete {
        incomplete: projected,
    })
    .expect("serialize typed incomplete result");
    assert_eq!(wire["code"], "contract_package_read_incomplete");
    let fields = &wire["cause"]["incomplete"];
    assert_eq!(
        fields["limit_kind"],
        serde_json::to_value(incomplete.limit_kind).unwrap()
    );
    assert_eq!(fields["limit"], incomplete.limit);
    assert_eq!(fields["consumed"], incomplete.consumed);
    assert_eq!(
        fields["path"],
        serde_json::to_value(incomplete.path).unwrap()
    );
}

#[trace("TC-030", "FR-019-AC-50")]
#[test]
fn admitted_function_read_keeps_supported_settlement() {
    use exact_function_fixture::{
        code_id, source_bound_call_package, SOURCE_CALL, SOURCE_FUNCTION, SOURCE_PARAM_A,
        SOURCE_PARAM_B,
    };
    let (package, ids) = source_bound_call_package().admit_resolved();
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("kani")]),
    );
    request.kani_form = KaniForm::V2FunctionApplication {
        package: Box::new(package),
        function_node_id: ids.resolve(&code_id(SOURCE_FUNCTION)),
        call_node_id: ids.resolve(&code_id(SOURCE_CALL)),
        argument_node_ids: vec![
            ids.resolve(&code_id(SOURCE_PARAM_A)),
            ids.resolve(&code_id(SOURCE_PARAM_B)),
        ],
    };
    assert_eq!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request,
        )
        .disposition,
        Disposition::Supported {
            backend: "kani".to_owned()
        }
    );
}

#[trace("TC-030", "FR-019-AC-49")]
#[test]
fn oversized_admitted_function_read_stops_the_envelope() {
    use exact_function_fixture::{
        code_id, oversized_source_bound_call_package, SOURCE_CALL, SOURCE_FUNCTION, SOURCE_PARAM_A,
        SOURCE_PARAM_B,
    };
    let (package, ids) = oversized_source_bound_call_package();
    let body_id = &package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == ids.resolve(&code_id(SOURCE_FUNCTION)))
        .expect("resolved function")
        .body;
    let body_id = function_body_target(body_id).expect("body binding").clone();
    let call_id = ids.resolve(&code_id(SOURCE_CALL));
    let lowered = package.lower(
        &[body_id, call_id.clone()],
        &routed_contract_lowering_profile(),
    );
    let bytes = lowered.package.canonical_bytes().expect("lowered bytes");
    assert!(u64::try_from(bytes.len()).unwrap() > CheckedPackageReadLimits::bounded().bytes);
    let direct = ContractPackageWireV1::read(
        bytes,
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    );
    let ContractPackageReadResult::Incomplete(expected) = direct else {
        panic!("large lowered package must exhaust bytes: {direct:?}");
    };
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("kani")]),
    );
    request.kani_form = KaniForm::V2FunctionApplication {
        package: Box::new(package),
        function_node_id: ids.resolve(&code_id(SOURCE_FUNCTION)),
        call_node_id: call_id,
        argument_node_ids: vec![
            ids.resolve(&code_id(SOURCE_PARAM_A)),
            ids.resolve(&code_id(SOURCE_PARAM_B)),
        ],
    };
    let outcome = negotiate_backend_provider(&envelope(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        vec![request],
    ));
    let Err(EnvelopeRefusal::ContractPackageReadIncomplete { incomplete }) = outcome else {
        panic!("reader limit must stop negotiation: {outcome:?}");
    };
    assert_eq!(incomplete.limit_kind, expected.limit_kind);
    assert_eq!(incomplete.limit, expected.limit);
    assert_eq!(incomplete.consumed, expected.consumed);
    assert_eq!(
        incomplete.path.as_deref(),
        expected.path.as_ref().map(JsonPointer::as_str)
    );
}

#[trace("TC-030", "FR-019-AC-52")]
#[test]
fn genuine_function_backend_refusals_stay_item_level() {
    use exact_function_fixture::{
        code_id, source_bound_call_package_with_name_len,
        source_bound_call_package_with_name_segments, SOURCE_CALL, SOURCE_FUNCTION, SOURCE_PARAM_A,
        SOURCE_PARAM_B,
    };
    let request_for = |package, ids: exact_function_fixture::FixtureIds| {
        let mut request = item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate("kani")]),
        );
        request.kani_form = KaniForm::V2FunctionApplication {
            package: Box::new(package),
            function_node_id: ids.resolve(&code_id(SOURCE_FUNCTION)),
            call_node_id: ids.resolve(&code_id(SOURCE_CALL)),
            argument_node_ids: vec![
                ids.resolve(&code_id(SOURCE_PARAM_A)),
                ids.resolve(&code_id(SOURCE_PARAM_B)),
            ],
        };
        request
    };
    let structural = source_bound_call_package_with_name_segments(vec![
        "module".to_owned(),
        "function".to_owned(),
    ]);
    let too_large_to_render = source_bound_call_package_with_name_len(450_000);
    let body_id = function_body_target(
        &too_large_to_render
            .0
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == too_large_to_render.1.resolve(&code_id(SOURCE_FUNCTION)))
            .expect("resolved function")
            .body,
    )
    .expect("body id")
    .clone();
    let lowered = too_large_to_render.0.lower(
        &[
            body_id,
            too_large_to_render.1.resolve(&code_id(SOURCE_CALL)),
        ],
        &routed_contract_lowering_profile(),
    );
    assert!(
        matches!(
            ContractPackageWireV1::read(
                lowered.package.canonical_bytes().expect("lowered bytes"),
                CheckedPackageReadLimits::bounded(),
                &ContractDependencies::new(),
            ),
            ContractPackageReadResult::Admitted(_)
        ),
        "no-claim control must pass the internal reader"
    );
    for (label, (package, ids)) in [
        ("structural", structural),
        ("admitted-no-claim", too_large_to_render),
    ] {
        let settled = settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request_for(package, ids),
        );
        assert!(
            matches!(
                &settled.disposition,
                Disposition::Unsupported {
                    cause: Cause::KaniFormRefused {
                        refusal: UnsupportedObligation::OperationNotRendered {
                            operation_identity,
                            ..
                        },
                        backend,
                        ..
                    }
                } if operation_identity == "quire.op.function.call" && backend == "kani"
            ),
            "{label} should retain item-level OperationNotRendered: {settled:?}"
        );
    }
}

fn candidate(identity: &str) -> Candidate {
    Candidate {
        identity: identity.to_owned(),
    }
}

fn bounded() -> Option<ExtentClassification> {
    Some(ExtentClassification {
        extent: Mode::Bounded,
        finite_bound_available: true,
        domains: Vec::new(),
        bounds: Vec::new(),
    })
}

fn unbounded(finite_bound_available: bool) -> Option<ExtentClassification> {
    Some(ExtentClassification {
        extent: Mode::Unbounded,
        finite_bound_available,
        domains: Vec::new(),
        bounds: Vec::new(),
    })
}

fn envelope(manifest: Vec<BackendDescriptor>, items: Vec<RequestItem>) -> BackendProviderEnvelope {
    BackendProviderEnvelope {
        contract_version: BACKEND_PROVIDER_CONTRACT.to_owned(),
        capability_vocabulary: CAPABILITY_VOCABULARY.to_owned(),
        manifest,
        items,
    }
}

fn settle_one(manifest: Vec<BackendDescriptor>, item: RequestItem) -> ItemSettlement {
    let settled = negotiate_backend_provider(&envelope(manifest, vec![item]))
        .expect("a well-formed envelope is not refused");
    assert_eq!(settled.len(), 1, "one item settles once");
    settled.into_iter().next().expect("one settlement")
}

fn item(kind: RequestedKind, candidates: Candidates) -> RequestItem {
    RequestItem {
        kind,
        extent: bounded(),
        named_backend: None,
        candidates,
        kani_form: integer_scalar("quire.op.integer.add"),
    }
}

fn selected_scalar_form(package: CheckedPackageV2, node_id: CheckedNodeId) -> KaniForm {
    let application = package
        .source_map()
        .iter()
        .find(|entry| entry.node_id == node_id && entry.role == CheckedOccurrenceRole::Expression);
    let conversion = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            checked_operation_identity(&node.body) == Some("quire.op.numeric.narrow")
                && matches!(&node.body, CheckedTerm::Application { arguments, .. }
                    if matches!(arguments.first(), Some(CheckedTerm::Reference { target }) if target == &node_id))
        })
        .map(|node| node.node_id.clone());
    let selection = application.map_or(ContractDriverResultSelection::NotApplicable, |entry| {
        ContractDriverResultSelection::OwnResult {
            application: ContractOccurrenceKeyV1 {
                node_id: node_id.clone(),
                role: entry.role.clone(),
                ordinal: entry.ordinal,
            },
            conversion: conversion.as_ref().map(|node_id| {
                let source = package
                    .source_map()
                    .iter()
                    .find(|source| {
                        source.node_id == *node_id
                            && source.role == CheckedOccurrenceRole::Expression
                    })
                    .expect("checked conversion occurrence");
                ContractOccurrenceKeyV1 {
                    node_id: node_id.clone(),
                    role: source.role.clone(),
                    ordinal: source.ordinal,
                }
            }),
        }
    });
    let mut roots = vec![node_id.clone()];
    if let Some(conversion) = &conversion {
        roots.push(conversion.clone());
    }
    let lowered = package.lower(&roots, &routed_contract_lowering_profile());
    let contract = match ContractPackageWireV1::read_with_source(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
        &package,
    ) {
        Ok(ContractPackageReadResult::Admitted(contract)) => contract,
        other => panic!("scalar selection package must admit: {other:?}"),
    };
    KaniForm::V2ScalarClaim {
        package: Box::new(package),
        contract,
        positions: {
            let mut positions = vec![ContractRequestPosition::DriverItem {
                node_id: node_id.clone(),
                result_selection: selection,
            }];
            if let Some(conversion) = conversion {
                positions.push(ContractRequestPosition::ContextRoot {
                    node_id: conversion,
                });
            }
            positions
        },
        request_index: 0,
        node_id,
    }
}

fn integer_scalar(operation_identity: &str) -> KaniForm {
    if operation_identity == "quire.op.integer.add" {
        let source = "language \"ix:native\" edition \"1-draft\";\n\
             profile v = \"quire.value.complete/v1\";\n\
             function f using v(a: Int[0, 9]): Int[0, 18] pure { a + a }\n";
        let compiled = compile_package(
            qsl_replay::CompileUnit {
                source: SourceIdentity::new("test", "selected-add.native", "git", "fixture"),
                package_name: "selected-add".to_owned(),
                path: "selected-add.native",
                bytes: source.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL emits bounded integer add");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let package = match CheckedPackageV2::read(
            compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("IR admits QSL add: {other:?}"),
        };
        let node_id = package
            .graph()
            .nodes
            .iter()
            .find(|node| checked_operation_identity(&node.body) == Some(operation_identity))
            .expect("QSL add node")
            .node_id
            .clone();
        return selected_scalar_form(package, node_id);
    }
    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| checked_operation_identity(&node.body) == Some(operation_identity))
        .expect("checked scalar fixture carries the selected operation")
        .node_id
        .clone();
    selected_scalar_form(package, node_id)
}

fn outside_form() -> KaniForm {
    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.node_tag == "model")
        .expect("checked scalar fixture has a model node outside the Kani set")
        .node_id
        .clone();
    KaniForm::OutsideSet {
        package: Box::new(package),
        node_id,
    }
}

fn process(id: &str, modes: &[Mode], domains: &[DomainKind]) -> BackendDescriptor {
    BackendDescriptor {
        identity: id.to_owned(),
        advertised: modes
            .iter()
            .map(|mode| (CapabilityKind::ValueValidity, *mode))
            .collect(),
        origin: ProviderOrigin::Process,
        domains: Some(domains.iter().copied().collect::<BTreeSet<_>>()),
        bounds: None,
    }
}

fn proof_bound(kind: DomainKind, maximum: i64) -> ProofBound {
    let bound = match kind {
        DomainKind::Integer => {
            FiniteBound::integer_range(Integer::from(0_i64), Integer::from(maximum)).unwrap()
        }
        DomainKind::Collection | DomainKind::Population => {
            FiniteBound::cardinality(u64::try_from(maximum).unwrap())
        }
        _ => panic!("fixture uses integer and cardinality kinds"),
    };
    ProofBound::new(
        DomainKey::Node {
            node: WireNodeId::from_digest([1; 32]),
            path: Vec::new(),
        },
        Some(kind),
        bound,
    )
    .unwrap()
}

fn wire_node_id_from_hex(digest: &str) -> WireNodeId {
    assert_eq!(digest.len(), 64, "checked node digest is SHA-256 hex");
    let mut bytes = [0_u8; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&digest[index * 2..index * 2 + 2], 16)
            .expect("checked node digest is canonical hex");
    }
    WireNodeId::from_digest(bytes)
}

/// A bounded Kani request retains its caller-supplied source bound in the borrowed request
/// while settlement reports support. The result schema has no bound field, so this checks input
/// custody and linkage without claiming a result-side serialization contract.
#[trace("FR-019-AC-26", "FR-019-AC-32")]
#[test]
fn kani_supported_bounded_request_keeps_exact_proof_bound_input() {
    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| checked_operation_identity(&node.body) == Some("quire.op.integer.add"))
        .expect("checked source carries integer addition")
        .node_id
        .clone();
    let scalar_item =
        quire_contract_codegen::derive_exact_scalar_items(&package, std::slice::from_ref(&node_id))
            .into_iter()
            .next()
            .expect("one source item")
            .expect("integer addition has a checked scalar plan");
    let generated = quire_contract_codegen::generate_exact_scalar_oracles(&package, &[scalar_item])
        .expect("checked scalar artifacts render");
    let claim = match &generated.claim_map.items[0].result {
        quire_contract_codegen::ClaimDisposition::Generated(claim) => claim,
        other => panic!("integer addition plan was not generated: {other:?}"),
    };
    let checked_bound = claim
        .checked_bounds
        .first()
        .expect("integer addition has one source-derived bound");
    let source_bound = ProofBound::new(
        DomainKey::Node {
            node: wire_node_id_from_hex(&checked_bound.digest),
            path: Vec::new(),
        },
        Some(DomainKind::Integer),
        FiniteBound::integer_range(Integer::from(-1000_i64), Integer::from(1000_i64)).unwrap(),
    )
    .unwrap();
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.extent = Some(ExtentClassification {
        extent: Mode::Bounded,
        finite_bound_available: true,
        domains: vec![ExtentDomain {
            domain: source_bound.domain().clone(),
            kind: DomainKind::Integer,
        }],
        bounds: vec![source_bound.clone()],
    });
    request.kani_form = integer_scalar("quire.op.integer.add");
    let envelope = envelope(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        vec![request],
    );
    let settlement = negotiate_backend_provider(&envelope).expect("valid envelope");
    assert_eq!(
        settlement[0].disposition,
        Disposition::Supported {
            backend: BackendKind::Kani.identity().to_owned(),
        }
    );
    assert_eq!(
        envelope.items[0].extent.as_ref().unwrap().bounds,
        [source_bound]
    );
}

/// Request-index correlation lets a caller recover each immutable request's exact bounds.
/// This asserts correlation only; it does not claim detached result serialization or settle
/// FR-019 AC-32's result-side preservation interpretation.
#[trace("FR-019-AC-2", "FR-019-AC-32")]
#[test]
fn kani_bounded_settlements_correlate_each_request_bound_by_index() {
    let make_bound = |seed: u8, kind: DomainKind, path: Vec<u32>, maximum: i64| {
        let finite = match kind {
            DomainKind::Integer => {
                FiniteBound::integer_range(Integer::from(0_i64), Integer::from(maximum)).unwrap()
            }
            DomainKind::Collection | DomainKind::Population => {
                FiniteBound::cardinality(u64::try_from(maximum).unwrap())
            }
            _ => panic!("fixture uses integer and cardinality bounds"),
        };
        ProofBound::new(
            DomainKey::Node {
                node: WireNodeId::from_digest([seed; 32]),
                path,
            },
            Some(kind),
            finite,
        )
        .unwrap()
    };
    let expected = vec![
        make_bound(1, DomainKind::Integer, vec![], 8),
        make_bound(2, DomainKind::Population, vec![1], 9),
        make_bound(3, DomainKind::Collection, vec![2, 0], 10),
    ];
    let requests = expected
        .iter()
        .map(|bound| {
            let mut request = item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            );
            request.kani_form = integer_scalar("quire.op.integer.add");
            request.extent.as_mut().unwrap().bounds = vec![bound.clone()];
            request
        })
        .collect::<Vec<_>>();
    let request_envelope = envelope(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        requests,
    );

    let settlements = negotiate_backend_provider(&request_envelope).expect("valid envelope");
    assert_eq!(settlements.len(), expected.len());
    for (expected_index, (settlement, expected_bound)) in
        settlements.iter().zip(&expected).enumerate()
    {
        assert_eq!(settlement.request_index, expected_index);
        assert_eq!(
            settlement.disposition,
            Disposition::Supported {
                backend: BackendKind::Kani.identity().to_owned()
            }
        );
        let recovered = request_envelope.items[settlement.request_index]
            .extent
            .as_ref()
            .expect("bounded request carries extent")
            .bounds
            .as_slice();
        assert_eq!(recovered, std::slice::from_ref(expected_bound));
    }
}

fn extent_domain(kind: DomainKind) -> ExtentDomain {
    ExtentDomain {
        domain: DomainKey::Node {
            node: WireNodeId::from_digest([1; 32]),
            path: Vec::new(),
        },
        kind,
    }
}

#[trace("FR-019-AC-11", "FR-019-AC-12", "FR-019-AC-20", "TC-046")]
#[test]
fn process_kind_uses_origin_manifest_modes_and_explicit_bound_kinds() {
    let backend = process("kani", &[Mode::Bounded], &[DomainKind::Integer]);
    assert_eq!(
        BackendKind::from_descriptor(&backend),
        Some(BackendKind::Process(quire_contract_codegen::BackendId(
            "kani".to_owned()
        )))
    );
    assert!(
        !BackendKind::ALL.contains(&BackendKind::Process(quire_contract_codegen::BackendId(
            "kani".to_owned()
        )))
    );
    assert_eq!(BackendKind::Kani.index(), 0);
    assert_eq!(
        BackendKind::Process(quire_contract_codegen::BackendId("kani".to_owned())).index(),
        1
    );
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("kani")]),
    );
    request.named_backend = Some("kani".to_owned());
    request.extent.as_mut().unwrap().bounds = vec![proof_bound(DomainKind::Integer, 8)];
    let supported = settle_one(vec![backend.clone()], request.clone()).disposition;
    assert_eq!(
        supported,
        Disposition::Supported {
            backend: "kani".to_owned()
        }
    );
    request.extent.as_mut().unwrap().bounds = vec![proof_bound(DomainKind::Integer, 1000)];
    let mut changed_defaults = backend.clone();
    changed_defaults.bounds = Some(BTreeMap::from([("model_check.max_depth".to_owned(), 1)]));
    assert_eq!(
        settle_one(vec![changed_defaults], request.clone()).disposition,
        supported
    );
    request.extent.as_mut().unwrap().bounds = vec![proof_bound(DomainKind::Population, 8)];
    let unsupported = settle_one(vec![backend], request.clone());
    assert_eq!(
        unsupported.disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedDomain {
                kind: CapabilityKind::ValueValidity,
                domain: DomainKind::Population,
                backend: "kani".to_owned()
            }
        }
    );
    assert!(unsupported.warning().unwrap().contains("population"));
    let wire = serde_json::to_value(&unsupported.disposition).unwrap();
    assert_eq!(wire["cause"]["cause"], "unsupported-requested-capability");
    assert_eq!(wire["cause"]["domain"], "population");
    let unbounded_only = process("solver", &[Mode::Unbounded], &[]);
    request.named_backend = Some("solver".to_owned());
    request.candidates = Candidates::Set(vec![candidate("solver")]);
    assert_eq!(
        settle_one(vec![unbounded_only], request).disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedDomain {
                kind: CapabilityKind::ValueValidity,
                domain: DomainKind::Population,
                backend: "solver".to_owned(),
            }
        }
    );
}

#[trace("FR-019-AC-42", "FR-019-AC-18", "FR-019-AC-8", "TC-046")]
#[test]
fn process_manifest_bounds_keep_absence_distinct_from_present_empty() {
    let absent = process("solver", &[Mode::Bounded], &[DomainKind::Integer]);
    let present_empty = BackendDescriptor {
        bounds: Some(BTreeMap::new()),
        ..absent.clone()
    };
    assert_eq!(absent.bounds, None);
    assert_eq!(present_empty.bounds, Some(BTreeMap::new()));
    assert_ne!(absent, present_empty);

    let valid = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("solver")]),
    );
    assert_eq!(
        settle_one(vec![absent.clone()], valid.clone()).disposition,
        settle_one(vec![present_empty.clone()], valid.clone()).disposition
    );

    let mut invalid = valid;
    invalid.kind = RequestedKind::Absent;
    invalid.candidates = Candidates::UnknownBackend("missing".to_owned());
    assert_eq!(
        settle_one(vec![absent], invalid.clone()).disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentKind,
        }
    );
    assert_eq!(
        settle_one(vec![present_empty], invalid).disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentKind,
        }
    );
}

#[trace(
    "FR-019-AC-16",
    "FR-019-AC-17",
    "FR-019-AC-21",
    "FR-019-AC-22",
    "TC-046"
)]
#[test]
fn process_unbounded_settlement_checks_only_boundable_domains() {
    let backend = process("solver", &[Mode::Bounded], &[DomainKind::Collection]);
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("solver")]),
    );
    request.extent = unbounded(true);
    request.extent.as_mut().unwrap().domains = vec![
        extent_domain(DomainKind::Collection),
        extent_domain(DomainKind::Quantity),
    ];
    assert_eq!(
        settle_one(vec![backend.clone()], request.clone()).disposition,
        Disposition::RequiresBound {
            backend: "solver".to_owned()
        }
    );
    request.extent.as_mut().unwrap().finite_bound_available = false;
    assert_eq!(
        settle_one(vec![backend.clone()], request.clone()).disposition,
        Disposition::Unsupported {
            cause: Cause::UnboundedExtent {
                kind: CapabilityKind::ValueValidity,
                backend: "solver".to_owned()
            }
        }
    );
    request
        .extent
        .as_mut()
        .unwrap()
        .domains
        .push(extent_domain(DomainKind::Integer));
    assert_eq!(
        settle_one(vec![backend], request.clone()).disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedDomain {
                kind: CapabilityKind::ValueValidity,
                domain: DomainKind::Integer,
                backend: "solver".to_owned()
            }
        }
    );
    let unbounded_backend = process("solver", &[Mode::Unbounded], &[]);
    assert_eq!(
        settle_one(vec![unbounded_backend], request).disposition,
        Disposition::Supported {
            backend: "solver".to_owned()
        }
    );
}

#[trace("FR-019-AC-19", "TC-046")]
#[test]
fn process_bounded_coverage_reads_explicit_kind_with_fixed_domain_key() {
    let backend = process("solver", &[Mode::Bounded], &[DomainKind::Collection]);
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("solver")]),
    );
    let covered = proof_bound(DomainKind::Collection, 8);
    let uncovered = proof_bound(DomainKind::Population, 8);
    assert_eq!(covered.domain(), uncovered.domain());
    assert_eq!(covered.bound(), uncovered.bound());
    request.extent.as_mut().unwrap().bounds = vec![covered];
    assert_eq!(
        settle_one(vec![backend.clone()], request.clone()).disposition,
        Disposition::Supported {
            backend: "solver".to_owned()
        }
    );
    request.extent.as_mut().unwrap().bounds = vec![uncovered];
    assert_eq!(
        settle_one(vec![backend], request).disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedDomain {
                kind: CapabilityKind::ValueValidity,
                domain: DomainKind::Population,
                backend: "solver".to_owned()
            }
        }
    );
}

#[trace("FR-019-AC-23", "TC-046")]
#[test]
fn process_bounded_maximum_does_not_change_domain_coverage() {
    let backend = process("solver", &[Mode::Bounded], &[DomainKind::Integer]);
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("solver")]),
    );
    for (kind, expected) in [
        (
            DomainKind::Integer,
            Disposition::Supported {
                backend: "solver".to_owned(),
            },
        ),
        (
            DomainKind::Population,
            Disposition::Unsupported {
                cause: Cause::UnsupportedDomain {
                    kind: CapabilityKind::ValueValidity,
                    domain: DomainKind::Population,
                    backend: "solver".to_owned(),
                },
            },
        ),
    ] {
        for maximum in [8, 1000] {
            request.extent.as_mut().unwrap().bounds = vec![proof_bound(kind, maximum)];
            assert_eq!(
                settle_one(vec![backend.clone()], request.clone()).disposition,
                expected,
                "{kind:?} maximum {maximum} must not affect coverage"
            );
        }
    }
}

#[trace("FR-019-AC-13", "FR-019-AC-18", "TC-046")]
#[test]
fn process_settlement_does_not_resolve_provider_identity_as_an_executable() {
    let identity = "/this/process/provider/does/not/exist";
    let descriptor = process(identity, &[Mode::Bounded], &[DomainKind::Integer]);
    let request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(identity)]),
    );
    assert_eq!(
        settle_one(vec![descriptor], request).disposition,
        Disposition::Supported {
            backend: identity.to_owned()
        }
    );

    let guard = crate::scratch_crate::TemporaryDirectory::new("quire-codegen-process-settlement");
    let scratch = guard.path.clone();
    let executable = scratch.join("record-start");
    let record = scratch.join("started");
    fs::write(
        &executable,
        format!("#!/bin/sh\ntouch '{}'\n", record.display()),
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
    let identity = executable.to_str().unwrap();
    let descriptor = process(identity, &[Mode::Bounded], &[DomainKind::Integer]);
    let request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(identity)]),
    );
    assert_eq!(
        settle_one(vec![descriptor], request).disposition,
        Disposition::Supported {
            backend: identity.to_owned()
        }
    );
    assert!(
        !record.exists(),
        "settlement must not start the process provider"
    );
    assert!(Command::new(&executable).status().unwrap().success());
    assert!(
        record.exists(),
        "the executable must record an actual start"
    );
}

/// Every variant of the closed backend kind reaches a dispatched arm, and each
/// one settles rather than falling through.
///
/// Provenance: codegen#86
#[trace("FR-019-AC-1", "TC-030")]
#[test]
fn tc_030_every_backend_kind_has_a_dispatched_arm() {
    for backend in BackendKind::ALL {
        let descriptor = kani(vec![(CapabilityKind::OperationContract, Mode::Bounded)]);
        let descriptor = BackendDescriptor {
            identity: backend.identity().to_owned(),
            ..descriptor
        };
        let settlement = settle_one(
            vec![descriptor],
            item(
                RequestedKind::Known(CapabilityKind::OperationContract),
                Candidates::Set(vec![candidate(backend.identity())]),
            ),
        );
        assert_eq!(
            settlement.disposition,
            Disposition::Supported {
                backend: backend.identity().to_owned(),
            },
            "{} has no arm that settles",
            backend.identity()
        );
    }
    let identity = "process-arm";
    let settlement = settle_one(
        vec![process(identity, &[Mode::Bounded], &[])],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(identity)]),
        ),
    );
    assert_eq!(
        settlement.disposition,
        Disposition::Supported {
            backend: identity.to_owned(),
        },
        "Process(id) has no arm that settles"
    );
}

/// An absent kind and an unknown label settle before the candidate table is
/// consulted, each with its own cause and the unknown label reported verbatim.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_an_absent_or_unknown_kind_settles_before_the_candidate_table() {
    // Candidates that would otherwise settle `supported`: the kind rules win.
    let candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity())]);
    let manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];

    let absent = settle_one(
        manifest.clone(),
        item(RequestedKind::Absent, candidates.clone()),
    );
    assert_eq!(
        absent.disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentKind
        }
    );

    let unknown = settle_one(
        manifest,
        item(
            RequestedKind::Unknown("value-validity-v2".to_owned()),
            candidates,
        ),
    );
    assert_eq!(
        unknown.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownKind {
                received: "value-validity-v2".to_owned()
            }
        },
        "the refused label is reported with its exact received bytes"
    );
}

/// An item carrying no extent classification settles `invalid-request`, and is
/// never defaulted to `bounded` to reach a disposition.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_an_absent_extent_classification_settles_invalid_request() {
    let settlement = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            extent: None,
            ..item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            )
        },
    );
    assert_eq!(
        settlement.disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentExtent
        }
    );
}

/// The unknown-backend mark, and a registered backend with no negotiation arm,
/// both settle `invalid-request` naming the backend.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_an_unroutable_backend_settles_invalid_request() {
    let marked = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::UnknownBackend("cvc5".to_owned()),
        ),
    );
    assert_eq!(
        marked.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: "cvc5".to_owned()
            }
        }
    );

    // Registered, advertises the kind, and still has no arm here: FR-290's
    // first candidate row refuses it rather than treating registration as
    // routability.
    let registered_without_arm = BackendDescriptor {
        identity: "cvc5".to_owned(),
        advertised: vec![(CapabilityKind::ValueValidity, Mode::Unbounded)],
        origin: ProviderOrigin::Linked,
        domains: None,
        bounds: None,
    };
    let unarmed = settle_one(
        vec![registered_without_arm],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate("cvc5")]),
        ),
    );
    assert_eq!(
        unarmed.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: "cvc5".to_owned()
            }
        }
    );
}

/// A candidate absent from the manifest, and one that does not advertise the
/// item's kind, each settle `invalid-request` naming the offender.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_inconsistent_candidates_settle_invalid_request() {
    let absent_from_manifest = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate("kani-unregistered")]),
        ),
    );
    assert_eq!(
        absent_from_manifest.disposition,
        Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate("kani-unregistered")]
            }
        },
        "a candidate that is not in the manifest is inconsistent"
    );

    let does_not_advertise = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::Refinement),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
        ),
    );
    assert_eq!(
        does_not_advertise.disposition,
        Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate(BackendKind::Kani.identity())]
            }
        }
    );
}

/// An empty candidate set settles `unsupported`, warned, naming the kind and
/// the named backend when the request names one.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_an_empty_candidate_set_settles_unsupported_with_a_warning() {
    let unnamed = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::Realizability),
            Candidates::Set(Vec::new()),
        ),
    );
    assert_eq!(
        unnamed.disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind: CapabilityKind::Realizability,
                backend: None
            }
        }
    );
    let warning = unnamed.warning().expect("an unsupported settlement warns");
    assert!(
        warning.contains("realizability"),
        "the warning names the item's kind: {warning}"
    );

    let named = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            named_backend: Some(BackendKind::Kani.identity().to_owned()),
            ..item(
                RequestedKind::Known(CapabilityKind::Realizability),
                Candidates::Set(Vec::new()),
            )
        },
    );
    let warning = named.warning().expect("an unsupported settlement warns");
    assert!(
        warning.contains("realizability") && warning.contains(BackendKind::Kani.identity()),
        "the warning names the kind and the named backend: {warning}"
    );
}

/// Two candidates with no named backend settle `invalid-request`, naming every
/// candidate in candidate order, identically under either registration order.
///
/// Provenance: codegen#86
#[trace("FR-019-AC-4", "TC-030")]
#[test]
fn tc_030_two_candidates_with_no_named_backend_settle_ambiguous() {
    let second = BackendDescriptor {
        identity: "kani-nightly".to_owned(),
        advertised: vec![(CapabilityKind::ValueValidity, Mode::Unbounded)],
        origin: ProviderOrigin::Process,
        domains: None,
        bounds: None,
    };
    let first = kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]);
    // Candidate order is bytewise by identity, and is a property of the set
    // rather than of the manifest it was read from.
    let ordered = vec![
        candidate(BackendKind::Kani.identity()),
        candidate("kani-nightly"),
    ];

    for manifest in [
        vec![first.clone(), second.clone()],
        vec![second.clone(), first.clone()],
    ] {
        // Registration order is the manifest's order; the settlement may not read it.
        let settlement = settle_one(
            manifest,
            item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(ordered.clone()),
            ),
        );
        assert_eq!(
            settlement.disposition,
            Disposition::InvalidRequest {
                cause: Cause::AmbiguousBackend {
                    candidates: ordered.clone()
                }
            },
            "registration order changed the disposition or the candidate order"
        );
        let Disposition::InvalidRequest { cause } = &settlement.disposition else {
            panic!("ambiguous candidates must be an invalid request");
        };
        assert_eq!(cause.code(), "invalid_capability");
    }
}

/// Three candidates settle ambiguous in request order under every descriptor
/// registration order, even when request order differs from bytewise order.
#[trace("FR-019-AC-4", "TC-030")]
#[test]
fn tc_030_three_candidates_settle_ambiguous_in_candidate_order() {
    let descriptors = [
        kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]),
        BackendDescriptor {
            identity: "kani-nightly".to_owned(),
            advertised: vec![(CapabilityKind::ValueValidity, Mode::Unbounded)],
            origin: ProviderOrigin::Process,
            domains: None,
            bounds: None,
        },
        BackendDescriptor {
            identity: "alpha-backend".to_owned(),
            advertised: vec![(CapabilityKind::ValueValidity, Mode::Bounded)],
            origin: ProviderOrigin::Process,
            domains: None,
            bounds: None,
        },
    ];
    let candidates = vec![
        candidate("kani-nightly"),
        candidate(BackendKind::Kani.identity()),
        candidate("alpha-backend"),
    ];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let manifest = order.map(|index| descriptors[index].clone()).to_vec();
        let settlement = settle_one(
            manifest,
            item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(candidates.clone()),
            ),
        );
        assert_eq!(
            settlement.disposition,
            Disposition::InvalidRequest {
                cause: Cause::AmbiguousBackend {
                    candidates: candidates.clone()
                }
            },
            "candidate list or cause changed for registration order {order:?}"
        );
        let Disposition::InvalidRequest { cause } = &settlement.disposition else {
            panic!("ambiguous candidates must be an invalid request");
        };
        assert_eq!(cause.code(), "invalid_capability");
    }
}

/// Differing descriptors of one identity refuse the whole envelope in either order.
#[trace("TC-030", "FR-019-AC-45")]
#[test]
fn tc_030_repeated_backend_identity_refuses_in_both_orders() {
    let kind = CapabilityKind::ValueValidity;
    let bounded_only = kani(vec![(kind, Mode::Bounded)]);
    let unbounded_advertisement = kani(vec![(kind, Mode::Unbounded)]);
    let mut request = item(
        RequestedKind::Known(kind),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.extent = unbounded(false);

    assert!(matches!(
        settle_one(vec![unbounded_advertisement.clone()], request.clone()).disposition,
        Disposition::Supported { .. }
    ));
    assert!(matches!(
        settle_one(vec![bounded_only.clone()], request.clone()).disposition,
        Disposition::Unsupported {
            cause: Cause::UnboundedExtent { .. }
        }
    ));

    for manifest in [
        vec![bounded_only.clone(), unbounded_advertisement.clone()],
        vec![unbounded_advertisement.clone(), bounded_only.clone()],
    ] {
        assert_eq!(
            negotiate_backend_provider(&envelope(manifest, vec![request.clone()])),
            Err(EnvelopeRefusal::ManifestRepeatsBackendIdentity {
                identity: BackendKind::Kani.identity().to_owned(),
            }),
            "a repeated identity cannot route in either registration order"
        );
    }
}

/// Every placement of a second descriptor refuses before any item settles.
#[trace("TC-030", "FR-019-AC-45")]
#[test]
fn tc_030_repeated_descriptors_refuse_the_whole_envelope() {
    let first = kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)]);
    let other = BackendDescriptor {
        identity: "other".to_owned(),
        ..first.clone()
    };
    let differing = kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]);
    let different_origin = BackendDescriptor {
        origin: ProviderOrigin::Process,
        ..first.clone()
    };
    let different_field = BackendDescriptor {
        bounds: Some(BTreeMap::new()),
        ..first.clone()
    };
    let request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    let manifests = [
        vec![first.clone(), first.clone()],
        vec![first.clone(), other.clone(), first.clone()],
        vec![first.clone(), differing.clone()],
        vec![differing.clone(), first.clone()],
        vec![first.clone(), other.clone(), differing.clone()],
        vec![first.clone(), differing.clone(), other.clone()],
        vec![other.clone(), first.clone(), differing.clone()],
        vec![other.clone(), differing.clone(), first.clone()],
        vec![differing.clone(), first.clone(), other.clone()],
        vec![differing.clone(), other, first.clone()],
        vec![first.clone(), different_origin.clone()],
        vec![different_origin, first.clone()],
        vec![first.clone(), different_field.clone()],
        vec![different_field, first],
    ];
    for manifest in manifests {
        assert_eq!(
            negotiate_backend_provider(&envelope(manifest, vec![request.clone()])),
            Err(EnvelopeRefusal::ManifestRepeatsBackendIdentity {
                identity: BackendKind::Kani.identity().to_owned(),
            }),
            "a repeated descriptor must return Err with no settlements"
        );
    }
}

/// A domains-only descriptor change still repeats the same backend identity.
#[trace("TC-030", "FR-019-AC-45")]
#[test]
fn tc_030_domains_only_change_refuses_without_settlements() {
    let first = kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)]);
    let different_domains = BackendDescriptor {
        domains: Some(BTreeSet::new()),
        ..first.clone()
    };
    let request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );

    for manifest in [
        vec![first.clone(), different_domains.clone()],
        vec![different_domains.clone(), first.clone()],
    ] {
        assert_eq!(
            negotiate_backend_provider(&envelope(manifest, vec![request.clone()])),
            Err(EnvelopeRefusal::ManifestRepeatsBackendIdentity {
                identity: BackendKind::Kani.identity().to_owned(),
            }),
            "a domains-only change must refuse the whole envelope"
        );
    }
}

/// Identity comparison is bytewise, including case.
#[trace("TC-030", "FR-019-AC-46")]
#[test]
fn tc_030_backend_identities_that_differ_only_in_case_are_distinct() {
    let kind = CapabilityKind::ValueValidity;
    let manifest = vec![
        BackendDescriptor {
            identity: "Kani".to_owned(),
            ..kani(vec![(kind, Mode::Unbounded)])
        },
        kani(vec![(kind, Mode::Unbounded)]),
    ];
    let mut request = item(
        RequestedKind::Known(kind),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.extent = unbounded(false);
    assert!(matches!(
        negotiate_backend_provider(&envelope(manifest, vec![request])),
        Ok(settlements) if matches!(settlements.as_slice(), [ItemSettlement {
            disposition: Disposition::Supported { backend }, ..
        }] if backend == BackendKind::Kani.identity())
    ));
}

/// Refusal selection is stable across manifest order and precedes carrier reads.
#[trace("TC-030", "FR-019-AC-46")]
#[test]
fn tc_030_repeated_identity_precedence_and_bytewise_selection() {
    let a = BackendDescriptor {
        identity: "A".to_owned(),
        ..kani(vec![])
    };
    let b = BackendDescriptor {
        identity: "B".to_owned(),
        ..kani(vec![])
    };
    let request = item(
        RequestedKind::Unknown("unread-kind".to_owned()),
        Candidates::Set(Vec::new()),
    );
    for manifest in [
        vec![b.clone(), a.clone(), b.clone(), a.clone()],
        vec![a.clone(), b.clone(), a.clone(), b.clone()],
        vec![b.clone(), b.clone(), a.clone(), a.clone()],
        vec![a.clone(), a.clone(), b.clone(), b.clone()],
    ] {
        let valid = envelope(manifest.clone(), vec![request.clone()]);
        assert_eq!(
            negotiate_backend_provider(&valid),
            Err(EnvelopeRefusal::ManifestRepeatsBackendIdentity {
                identity: "A".to_owned()
            })
        );
        for invalid in [
            BackendProviderEnvelope {
                contract_version: "bad-version".to_owned(),
                ..valid.clone()
            },
            BackendProviderEnvelope {
                capability_vocabulary: "foreign".to_owned(),
                ..valid.clone()
            },
            BackendProviderEnvelope {
                capability_vocabulary: String::new(),
                ..valid
            },
        ] {
            let (member, received) = if invalid.contract_version != BACKEND_PROVIDER_CONTRACT {
                ("contract_version", invalid.contract_version.clone())
            } else {
                (
                    "capability_vocabulary",
                    invalid.capability_vocabulary.clone(),
                )
            };
            assert_eq!(
                negotiate_backend_provider(&invalid),
                Err(EnvelopeRefusal::InvalidCapability {
                    cause: "unsupported-version",
                    member,
                    received,
                }),
                "carrier refusal must precede repeat detection and item settlement"
            );
        }
    }
}

/// Each row of the advertised-mode table settles its own disposition, and an
/// unbounded extent never settles `supported` on a bounded-only advertisement.
///
/// Provenance: codegen#86
#[trace("FR-019-AC-26", "FR-019-AC-31", "TC-030")]
#[test]
fn tc_030_the_advertised_mode_table_settles_each_row() {
    let kind = CapabilityKind::ValueValidity;
    let only = vec![candidate(BackendKind::Kani.identity())];
    let kani_backend = BackendKind::Kani.identity().to_owned();

    let rows = [
        // (advertised mode, extent, finite bound available, expected)
        (
            Mode::Bounded,
            bounded(),
            Disposition::Supported {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Unbounded,
            bounded(),
            Disposition::Unsupported {
                cause: Cause::UnsupportedRequestedCapability {
                    kind,
                    backend: Some(kani_backend.clone()),
                },
            },
        ),
        (
            Mode::Unbounded,
            unbounded(false),
            Disposition::Supported {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Bounded,
            unbounded(true),
            Disposition::RequiresBound {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Bounded,
            unbounded(false),
            Disposition::Unsupported {
                cause: Cause::UnboundedExtent {
                    kind,
                    backend: kani_backend.clone(),
                },
            },
        ),
    ];

    for (advertised, extent, expected) in rows {
        let settlement = settle_one(
            vec![kani(vec![(kind, advertised)])],
            RequestItem {
                extent: extent.clone(),
                ..item(RequestedKind::Known(kind), Candidates::Set(only.clone()))
            },
        );
        assert_eq!(
            settlement.disposition, expected,
            "advertised {advertised:?} against extent {extent:?}"
        );
    }
}

/// The Kani arm checks typed form support before extent and advertised mode.
#[trace(
    "FR-019-AC-25",
    "FR-019-AC-26",
    "FR-019-AC-27",
    "FR-019-AC-29",
    "FR-019-AC-31",
    "FR-019-AC-44"
)]
#[test]
fn kani_form_preflight_gates_support_and_preserves_extent_rules() {
    let candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity())]);
    let backend = BackendKind::Kani.identity().to_owned();
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        candidates,
    );
    request.kani_form = integer_scalar("quire.op.integer.add");
    assert_eq!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request.clone(),
        )
        .disposition,
        Disposition::Supported {
            backend: backend.clone()
        },
        "a current integer add form stays supported"
    );

    request.extent = unbounded(true);
    assert_eq!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request.clone(),
        )
        .disposition,
        Disposition::RequiresBound {
            backend: backend.clone()
        },
        "an admitted unbounded form follows the finite-bound path"
    );

    request.kani_form = integer_scalar("quire.op.integer.div");
    let refused = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)])],
        request,
    );
    assert!(matches!(
        refused.disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid {
                refusal: InvalidObligationItem::ScalarResultMissing,
                ..
            }
        }
    ));
}

/// A selected exact quantity conversion settles as OperationNotRendered for Kani.
#[trace(
    "FR-014-AC-2",
    "FR-014-AC-51",
    "FR-014-AC-55",
    "FR-019-AC-44",
    "TC-024"
)]
#[test]
fn a_selected_quantity_conversion_settles_as_operation_not_rendered() {
    let (package, _, ids) = crate::kani_obligations::routed_scalar_package();
    let backend = BackendKind::Kani.identity().to_owned();
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    let node_id = ids.resolve(&crate::kani_obligations::package::code_id(
        crate::kani_obligations::package::QUANTITY_CONVERSIONS[0],
    ));
    request.kani_form = selected_scalar_form(package, node_id.clone());
    let settled = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        request,
    );
    assert!(
        matches!(
            &settled.disposition,
            Disposition::Unsupported {
                cause: Cause::KaniFormRefused {
                    refusal: UnsupportedObligation::OperationNotRendered { operation_identity, .. },
                    backend: refusing,
                    ..
                }
            } if operation_identity == "quire.op.quantity.convert" && *refusing == backend
        ),
        "{:?}",
        settled.disposition
    );
}

/// Selected-result defects follow request/candidate validation and precede
/// the advertised-mode and extent settlement branches.
#[trace("FR-019-AC-44", "TC-030")]
#[test]
fn selected_scalar_context_refusal_keeps_validation_precedence() {
    let mut form = integer_scalar("quire.op.integer.add");
    let KaniForm::V2ScalarClaim { positions, .. } = &mut form else {
        unreachable!("bounded add is a selected scalar")
    };
    let ContractRequestPosition::DriverItem {
        result_selection, ..
    } = &mut positions[0]
    else {
        unreachable!("first position is a driver")
    };
    *result_selection = ContractDriverResultSelection::NotApplicable;
    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate("kani")]),
    );
    request.kani_form = form;
    request.extent = unbounded(false);
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request.clone()
        )
        .disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid {
                refusal: InvalidObligationItem::ScalarResultMissing,
                ..
            }
        }
    ));
    request.candidates = Candidates::Set(Vec::new());
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request.clone()
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability { .. }
        }
    ));
    request.candidates = Candidates::Set(vec![candidate("kani")]);
    request.extent = None;
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request
        )
        .disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentExtent
        }
    ));
}

/// A checked integer function call settles only when the FR-021 lowering plan
/// succeeds and the requested callee and ordered arguments match the source.
#[trace("FR-019-AC-25", "FR-019-AC-26", "FR-019-AC-29")]
#[test]
fn kani_source_call_preflight_binds_callee_and_ordered_arguments() {
    use exact_function_fixture::{
        code_id, source_bound_call_package, SOURCE_CALL, SOURCE_FUNCTION, SOURCE_OTHER_FUNCTION,
        SOURCE_PARAM_A, SOURCE_PARAM_B,
    };
    let (package, ids) = source_bound_call_package().admit_resolved();
    let function = ids.resolve(&code_id(SOURCE_FUNCTION));
    let other_function = ids.resolve(&code_id(SOURCE_OTHER_FUNCTION));
    let call = ids.resolve(&code_id(SOURCE_CALL));
    let argument_a = ids.resolve(&code_id(SOURCE_PARAM_A));
    let argument_b = ids.resolve(&code_id(SOURCE_PARAM_B));
    let kani_manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];
    let form = |function_node_id, argument_node_ids| KaniForm::V2FunctionApplication {
        package: Box::new(package.clone()),
        function_node_id,
        call_node_id: call.clone(),
        argument_node_ids,
    };

    let mut request = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.kani_form = form(
        function.clone(),
        vec![argument_a.clone(), argument_b.clone()],
    );
    assert_eq!(
        settle_one(kani_manifest.clone(), request.clone()).disposition,
        Disposition::Supported {
            backend: BackendKind::Kani.identity().to_owned(),
        }
    );

    assert_eq!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)])],
            request.clone(),
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind: CapabilityKind::ValueValidity,
                backend: Some(BackendKind::Kani.identity().to_owned()),
            }
        },
        "a supported bounded function application still requires the advertised bounded mode"
    );

    request.kani_form = form(other_function, vec![argument_a.clone(), argument_b.clone()]);
    let wrong_callee = settle_one(kani_manifest.clone(), request.clone());
    match wrong_callee.disposition {
        Disposition::InvalidRequest {
            cause:
                Cause::KaniFormInvalid {
                    form,
                    refusal: InvalidObligationItem::UnknownNode,
                    ..
                },
        } => {
            assert_eq!(
                form.form,
                quire_contract_codegen::KaniFormName::V2FunctionApplication
            );
            assert_eq!(
                form.family,
                quire_contract_codegen::KaniFormFamily::FunctionApplication
            );
            assert_eq!(
                serde_json::to_value(form).unwrap()["form"],
                "v2_function_application"
            );
        }
        other => panic!("wrong callee should be a source-bound V2 refusal: {other:?}"),
    }

    request.kani_form = form(function, vec![argument_b, argument_a]);
    assert!(matches!(
        settle_one(kani_manifest, request).disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid {
                refusal: InvalidObligationItem::UnknownNode,
                ..
            }
        }
    ));
}

fn source_derived_division_refusal() -> UnsupportedObligation {
    use quire_contract_codegen::{
        negotiate_kani_obligations, KaniObligationOutcome, KaniObligationRequest,
        ObligationDisposition, ObligationItem,
    };
    use std::time::Duration;

    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| checked_operation_identity(&node.body) == Some("quire.op.integer.div"))
        .expect("checked scalar fixture carries integer division")
        .node_id
        .clone();
    let item =
        quire_contract_codegen::derive_exact_scalar_items(&package, std::slice::from_ref(&node_id))
            .into_iter()
            .next()
            .expect("one requested node is derived")
            .expect("integer division has a source-derived descriptor");
    let claim_map = quire_contract_codegen::generate_exact_scalar_oracles(&package, &[item])
        .expect("source-derived division plan renders")
        .claim_map;
    let generated = match &claim_map.items[0].result {
        quire_contract_codegen::ClaimDisposition::Generated(generated) => generated,
        other => panic!("derived integer division must generate its upstream oracle: {other:?}"),
    };
    let checked_bound = generated
        .checked_bounds
        .first()
        .expect("division plan identifies its exact checked bound");
    let contract = exact_function_fixture::admitted_function_contract(
        &package,
        std::slice::from_ref(&node_id),
    );
    let result_selection = quire_contract_codegen::ContractDriverResultSelection::NotApplicable;
    let items = [ObligationItem::ScalarClaim {
        package: &contract,
        claim_map: &claim_map,
        node_id: &node_id,
        claim_index: 0,
        result_selection: &result_selection,
    }];
    let request = KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(30),
        ),
        items: &items,
        subject_path: "crate::subject",
        unwind: 4,
    };
    let records = match negotiate_kani_obligations(&request).expect("valid request") {
        KaniObligationOutcome::Emitted { records, .. }
        | KaniObligationOutcome::Rejected { records } => records,
    };
    match records
        .into_iter()
        .next()
        .expect("one refusal record")
        .disposition
    {
        ObligationDisposition::Unsupported { reason } => {
            match &reason {
                UnsupportedObligation::OperationNotRendered {
                    derived_domains, ..
                } => {
                    assert!(
                        derived_domains.iter().any(|domain| matches!(
                            domain,
                            quire_contract_codegen::DerivedDomain::IntegerRange {
                                bound,
                                lower,
                                upper,
                            } if bound == checked_bound && lower == "-1000" && upper == "1000"
                        )),
                        "typed domain must preserve the exact checked bound key, kind, and limits"
                    );
                }
                other => panic!("unexpected integer division refusal: {other:?}"),
            }
            reason
        }
        other => panic!("integer division must be unsupported by Kani: {other:?}"),
    }
}

/// Direct FR-015 classification remains available for checked-source callers.
#[trace("TC-025")]
#[test]
fn direct_fr015_division_keeps_renderer_refusal() {
    assert!(matches!(
        source_derived_division_refusal(),
        UnsupportedObligation::OperationNotRendered { .. }
    ));
}

/// A source-mapped, admitted integer division reaches form classification, rather than
/// disappearing behind the missing-result guard. The FR-015 refusal wins before mode settlement.
#[trace(
    "FR-015-AC-15",
    "FR-019-AC-25",
    "FR-019-AC-29",
    "FR-019-AC-44",
    "FR-014-AC-55",
    "FR-022-AC-30",
    "TC-030"
)]
#[test]
fn kani_selected_division_preserves_exact_renderer_refusal() {
    use crate::kani_obligations::package;

    const DIVISION: u32 = 9_301_101;
    const DIVISION_NARROW: u32 = 9_301_102;
    let mut builder = package::qsl_shaped_package();
    let integer = package::key(package::T_INTEGER);
    let division_law =
        package::integer_division_definition(quire_exact::DivisionProfile::Truncating);
    builder.select_definition(division_law.clone());
    builder.application_bounded(
        DIVISION,
        "expression",
        "binary",
        &integer,
        package::application(
            "binary",
            package::op_full(
                "quire.op.integer.div",
                vec![package::law("integer_division", division_law)],
                None,
                None,
            ),
            &integer,
            vec![
                package::reference(&package::key(package::QSL_PARAM_X)),
                package::reference(&package::key(package::QSL_LITERAL_TWO)),
            ],
        ),
        &[],
    );
    let division_bound = builder.bound(&package::Bound::Integer(0, 10));
    builder.narrow(
        DIVISION_NARROW,
        &package::code_id(DIVISION).digest,
        &division_bound,
    );
    builder.expression_occurrences();
    let (checked, ids) = builder.admit_resolved();
    let plain_source = checked.clone();
    let form = selected_scalar_form(checked, ids.resolve(&package::code_id(DIVISION)));
    let KaniForm::V2ScalarClaim {
        contract,
        positions,
        node_id,
        ..
    } = &form
    else {
        unreachable!("selected scalar helper constructs the scalar form")
    };
    let ContractRequestPosition::DriverItem {
        result_selection, ..
    } = &positions[0]
    else {
        panic!("first position must be the selected division")
    };
    assert!(matches!(
        result_selection,
        ContractDriverResultSelection::OwnResult {
            conversion: Some(_),
            ..
        }
    ));
    let mut request = RequestItem {
        kind: RequestedKind::Known(CapabilityKind::ValueValidity),
        extent: unbounded(false),
        named_backend: None,
        candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
        kani_form: form.clone(),
    };
    request.candidates = Candidates::Set(Vec::new());
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)])],
            request.clone(),
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability { .. }
        }
    ));
    request.candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity())]);
    let settled = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)])],
        request,
    );
    let Disposition::Unsupported {
        cause:
            Cause::KaniFormRefused {
                form: identity,
                refusal,
                backend,
            },
    } = settled.disposition
    else {
        panic!("selected integer division must keep its typed FR-015 refusal: {settled:?}")
    };
    assert_eq!(
        identity.family,
        quire_contract_codegen::KaniFormFamily::Integer
    );
    assert_eq!(backend, BackendKind::Kani.identity());
    assert!(
        matches!(
            &refusal,
            UnsupportedObligation::OperationNotRendered {
                operation_identity,
                derived_domains,
            } if operation_identity == "quire.op.integer.div"
                && derived_domains.iter().any(|domain| matches!(
                    domain,
                    quire_contract_codegen::DerivedDomain::IntegerRange {
                        lower,
                        upper,
                        ..
                    } if lower == "0" && upper == "10"
                ))
        ),
        "wrong division refusal: {refusal:?}"
    );

    let routed = quire_contract_codegen::RoutedGenerationItem::from_descriptor(
        0,
        node_id.clone(),
        candidate(BackendKind::Kani.identity()),
        &kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]),
    )
    .expect("linked Kani route")
    .with_scalar_selection(contract, result_selection.clone());
    let plain_route = routed.clone();
    let generated = quire_contract_codegen::generate_routed(
        contract,
        positions,
        &[routed],
        &quire_contract_codegen::GenerationContexts {
            kani: Some(quire_contract_codegen::KaniGenerationContext {
                ceilings: crate::common::proof_ceilings::proof_ceilings(),
                subject_path: "crate::subject",
                unwind: 4,
            }),
        },
    )
    .expect("admitted route keeps a per-item refusal");
    assert!(generated.rejected.is_empty());
    let claim = generated
        .claim_map
        .as_ref()
        .expect("division claim map")
        .items
        .iter()
        .find(|claim| claim.node_id == *node_id)
        .expect("selected division claim");
    assert_eq!(
        claim.operation.provenance,
        quire_contract_codegen::OperationProvenance::IrConfirmed
    );
    let quire_contract_codegen::ClaimDisposition::Generated(oracle) = &claim.result else {
        panic!("source-backed division must generate its oracle: {claim:?}");
    };
    assert!(oracle
        .oracle_source
        .contains("rt::DivisionProfile::Truncating"));
    let quire_contract_codegen::KindOutput::Kani { record, harness } = &generated.items[0].output
    else {
        panic!("selected division must reach the Kani arm")
    };
    assert_eq!(
        record.disposition,
        quire_contract_codegen::ObligationDisposition::Unsupported { reason: refusal }
    );
    assert!(harness.is_none(), "refused division emits no Kani harness");

    let roots = positions
        .iter()
        .map(|position| position.node_id().clone())
        .collect::<Vec<_>>();
    let lowered = plain_source.lower(&roots, &routed_contract_lowering_profile());
    let plain = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("plain division package must admit: {other:?}"),
    };
    let plain_route = plain_route.with_scalar_selection(&plain, result_selection.clone());
    assert!(matches!(
        quire_contract_codegen::generate_routed(
            &plain,
            positions,
            &[plain_route],
            &quire_contract_codegen::GenerationContexts {
                kani: Some(quire_contract_codegen::KaniGenerationContext {
                    ceilings: crate::common::proof_ceilings::proof_ceilings(),
                    subject_path: "crate::subject",
                    unwind: 4,
                }),
            },
        ),
        Err(quire_contract_codegen::RoutedGenerationError::UnauthenticatedOperationView {
            request_index: 0,
            node_id: refused,
        }) if refused == *node_id
    ));
}

fn v2_state_frame_form(
    fixture: crate::kani_obligations_state_frame::Fixture,
    role: quire_contract_codegen::KaniObligationRole,
) -> KaniForm {
    KaniForm::V2StateFrame {
        package: Box::new(fixture.package),
        clause: fixture.clause,
        role,
        state_path: "crate::subject::Account".to_owned(),
        state_fields: crate::kani_obligations_state_frame::ir295_state_fields()
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        subject_path: "crate::subject::deposit".to_owned(),
    }
}

/// The V2 state-frame selectors derive both role and refusal from the admitted graph.
#[trace("FR-019-AC-25", "FR-019-AC-26", "FR-019-AC-33")]
#[test]
fn kani_state_frame_planned_selectors_keep_the_exact_fr015_refusal() {
    use quire_contract_codegen::{KaniObligationRole, StateFrameRefusal};

    let candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity())]);
    let backend = BackendKind::Kani.identity().to_owned();
    for role in [KaniObligationRole::Postcondition, KaniObligationRole::Frame] {
        let healthy = v2_state_frame_form(
            crate::kani_obligations_state_frame::ir295_healthy_fixture(),
            role,
        );
        let healthy = settle_one(
            vec![kani(vec![(
                CapabilityKind::OperationContract,
                Mode::Bounded,
            )])],
            RequestItem {
                kind: RequestedKind::Known(CapabilityKind::OperationContract),
                extent: bounded(),
                named_backend: None,
                candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
                kani_form: healthy,
            },
        );
        assert!(
            matches!(healthy.disposition, Disposition::Supported { .. }),
            "healthy state-frame role {role:?} refused as {:?}",
            healthy.disposition
        );
    }

    let mismatch_fixture = crate::kani_obligations_state_frame::ir295_healthy_fixture();
    let mut mismatch = item(
        RequestedKind::Known(CapabilityKind::OperationContract),
        candidates.clone(),
    );
    mismatch.kani_form = v2_state_frame_form(mismatch_fixture, KaniObligationRole::Precondition);
    assert!(matches!(
        settle_one(
            vec![kani(vec![(
                CapabilityKind::OperationContract,
                Mode::Bounded
            )])],
            mismatch,
        )
        .disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid { .. }
        }
    ));

    let mut contract = item(
        RequestedKind::Known(CapabilityKind::OperationContract),
        candidates.clone(),
    );
    contract.kani_form = v2_state_frame_form(
        crate::kani_obligations_state_frame::ir295_selector_fixture(true, false),
        KaniObligationRole::Postcondition,
    );
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::OperationContract, Mode::Bounded)])],
            contract,
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::KaniStateFrameRefused {
                refusal: StateFrameRefusal::ConditionNotSupported { .. },
                backend: received,
                ..
            }
        } if received == backend
    ));

    let mut frame = item(
        RequestedKind::Known(CapabilityKind::OperationContract),
        candidates,
    );
    frame.kani_form = v2_state_frame_form(
        crate::kani_obligations_state_frame::ir295_selector_fixture(false, true),
        KaniObligationRole::Frame,
    );
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::OperationContract, Mode::Bounded)])],
            frame,
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::KaniStateFrameRefused {
                refusal: StateFrameRefusal::NothingForbidden { .. },
                backend: received,
                ..
            }
        } if received == backend
    ));
}

/// State-frame refusals retain the checked operation identity and selected harness role.
#[trace("FR-019-AC-25", "FR-019-AC-27", "FR-019-AC-30", "FR-019-AC-33")]
#[test]
fn kani_state_frame_refusal_identity_keeps_operation_and_contract_frame_role() {
    use quire_contract_codegen::{KaniObligationRole, KaniStateFrameRole, StateFrameRefusal};

    let backend = BackendKind::Kani.identity().to_owned();
    let candidates = Candidates::Set(vec![candidate(&backend)]);
    for (fixture, obligation_role, expected_frame_role) in [
        (
            crate::kani_obligations_state_frame::ir295_selector_fixture(true, false),
            KaniObligationRole::Postcondition,
            KaniStateFrameRole::Contract,
        ),
        (
            crate::kani_obligations_state_frame::ir295_selector_fixture(false, true),
            KaniObligationRole::Frame,
            KaniStateFrameRole::Frame,
        ),
    ] {
        let expected_operation = fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == fixture.clause)
            .and_then(|node| checked_operation_identity(&node.body))
            .expect("checked state clause has an operation identity")
            .to_owned();
        let settlement = settle_one(
            vec![kani(vec![(
                CapabilityKind::OperationContract,
                Mode::Bounded,
            )])],
            RequestItem {
                kind: RequestedKind::Known(CapabilityKind::OperationContract),
                extent: bounded(),
                named_backend: None,
                candidates: candidates.clone(),
                kani_form: v2_state_frame_form(fixture, obligation_role),
            },
        );
        let Disposition::Unsupported {
            cause:
                Cause::KaniStateFrameRefused {
                    form,
                    refusal: actual_refusal,
                    ..
                },
            ..
        } = settlement.disposition
        else {
            panic!("state-frame selector did not retain its typed refusal: {settlement:?}");
        };
        match expected_frame_role {
            KaniStateFrameRole::Contract => assert!(matches!(
                actual_refusal,
                StateFrameRefusal::ConditionNotSupported { .. }
            )),
            KaniStateFrameRole::Frame => assert!(matches!(
                actual_refusal,
                StateFrameRefusal::NothingForbidden { .. }
            )),
        }
        assert_eq!(
            form.operation_identity.as_deref(),
            Some(expected_operation.as_str())
        );
        assert_eq!(form.state_frame_role, Some(expected_frame_role));
    }
}

/// Checked scalar nodes without source occurrences are invalid at the routed preflight seam,
/// even when the direct FR-015 classifier has a result for the same nodes.
#[trace("FR-019-AC-44", "TC-030")]
#[test]
fn kani_routed_preflight_requires_selected_result_occurrences() {
    use quire_contract_codegen::{
        negotiate_kani_obligations, KaniObligationOutcome, KaniObligationRequest, ObligationItem,
    };
    use std::time::Duration;

    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_ids = package
        .graph()
        .nodes
        .iter()
        .filter(|node| &*node.node_tag == "expression")
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    let derived = quire_contract_codegen::derive_exact_scalar_items(&package, &node_ids);
    let derivable = derived
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let claim_map = quire_contract_codegen::generate_exact_scalar_oracles(&package, &derivable)
        .expect("source-derived scalar plan renders")
        .claim_map;
    let contract = exact_function_fixture::admitted_function_contract(&package, &node_ids);
    let result_selection = quire_contract_codegen::ContractDriverResultSelection::NotApplicable;
    let kani_items = claim_map
        .items
        .iter()
        .enumerate()
        .map(|(claim_index, claim)| ObligationItem::ScalarClaim {
            package: &contract,
            claim_map: &claim_map,
            node_id: &claim.node_id,
            claim_index,
            result_selection: &result_selection,
        })
        .collect::<Vec<_>>();
    let kani_request = KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(30),
        ),
        items: &kani_items,
        subject_path: "crate::subject",
        unwind: 4,
    };
    let records = match negotiate_kani_obligations(&kani_request).expect("valid request") {
        KaniObligationOutcome::Emitted { records, .. }
        | KaniObligationOutcome::Rejected { records } => records,
    };
    assert_eq!(
        records.len(),
        claim_map.items.len(),
        "one FR-015 classification per typed claim"
    );

    for claim in &claim_map.items {
        let mut request = item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
        );
        request.kani_form = selected_scalar_form(package.clone(), claim.node_id.clone());
        let settlement = settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            request,
        );
        assert!(
            matches!(
                settlement.disposition,
                Disposition::InvalidRequest {
                    cause: Cause::KaniFormInvalid {
                        refusal: InvalidObligationItem::ScalarResultMissing,
                        ..
                    }
                }
            ),
            "{} has no checked source occurrence",
            claim.operation.identity
        );
    }
}

/// Every scalar family retains its typed identity when a source occurrence is missing.
#[trace("FR-019-AC-44", "TC-030")]
#[test]
fn kani_scalar_family_selectors_require_selected_result() {
    use quire_contract_codegen::KaniFormFamily;

    let (package, claim_map, _) = crate::kani_obligations::scalar_package();
    let selectors = [
        ("quire.op.integer.div", KaniFormFamily::Integer, false),
        ("quire.op.rational.add", KaniFormFamily::Rational, false),
        ("quire.op.decimal.add", KaniFormFamily::Decimal, false),
        (
            "quire.op.ieee.float32.add",
            KaniFormFamily::IeeeFloat,
            false,
        ),
        ("quire.op.text.eq", KaniFormFamily::Text, false),
        ("quire.op.enum.eq", KaniFormFamily::Enum, false),
        ("quire.op.quantity.add", KaniFormFamily::Quantity, false),
    ];
    for (operation, expected_family, _) in selectors {
        let source_claim = claim_map
            .items
            .iter()
            .find(|claim| claim.operation.identity == operation)
            .unwrap_or_else(|| panic!("scalar fixture lacks {operation}"));
        let form = selected_scalar_form(package.clone(), source_claim.node_id.clone());
        let settled = settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            RequestItem {
                kind: RequestedKind::Known(CapabilityKind::ValueValidity),
                extent: bounded(),
                named_backend: None,
                candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
                kani_form: form,
            },
        );
        match settled.disposition {
            Disposition::InvalidRequest {
                cause:
                    Cause::KaniFormInvalid {
                        form,
                        refusal: InvalidObligationItem::ScalarResultMissing,
                        ..
                    },
            } => assert_eq!(form.family, expected_family, "{operation}"),
            other => panic!("{operation} selector settled unexpectedly: {other:?}"),
        }
    }
}

#[trace("FR-019-AC-44", "TC-030")]
#[test]
fn kani_boolean_operator_selector_requires_selected_result() {
    let (package, node_id) = crate::kani_obligations::boolean_operation_package();
    let settled = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            kind: RequestedKind::Known(CapabilityKind::ValueValidity),
            extent: bounded(),
            named_backend: None,
            candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            kani_form: selected_scalar_form(package, node_id),
        },
    );
    assert!(matches!(
        settled.disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid {
                form: quire_contract_codegen::KaniFormIdentity {
                    family: quire_contract_codegen::KaniFormFamily::Boolean,
                    ..
                },
                refusal: InvalidObligationItem::ScalarResultMissing,
                ..
            }
        }
    ));
}

/// Nonexpression checked nodes cannot furnish a selected scalar result.
#[trace("FR-019-AC-44", "TC-030")]
#[test]
fn kani_scalar_path_requires_selected_result_for_nonderivable_nodes() {
    let (package, _, _) = crate::kani_obligations::scalar_package();
    let value = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.node_tag == "value")
        .expect("scalar fixture has an admitted value node");
    let value_settlement = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            kind: RequestedKind::Known(CapabilityKind::ValueValidity),
            extent: bounded(),
            named_backend: None,
            candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            kani_form: selected_scalar_form(package.clone(), value.node_id.clone()),
        },
    );
    assert!(matches!(
        value_settlement.disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid {
                refusal: InvalidObligationItem::ScalarResultMissing,
                ..
            }
        }
    ));

    let state = crate::kani_obligations_state_frame::ir295_healthy_fixture();
    let expected_operation = state
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == state.clause)
        .and_then(|node| checked_operation_identity(&node.body))
        .expect("checked state clause has an operation identity")
        .to_owned();
    let state_settlement = settle_one(
        vec![kani(vec![(
            CapabilityKind::OperationContract,
            Mode::Bounded,
        )])],
        RequestItem {
            kind: RequestedKind::Known(CapabilityKind::OperationContract),
            extent: bounded(),
            named_backend: None,
            candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            kani_form: selected_scalar_form(state.package, state.clause),
        },
    );
    let Disposition::InvalidRequest {
        cause: Cause::KaniFormInvalid { form, refusal, .. },
    } = state_settlement.disposition
    else {
        panic!("state clause did not retain its scalar-path refusal: {state_settlement:?}");
    };
    assert_eq!(refusal, InvalidObligationItem::ScalarResultMissing);
    assert_eq!(
        form.operation_identity.as_deref(),
        Some(expected_operation.as_str())
    );
    assert_eq!(
        form.role,
        Some(quire_contract_codegen::KaniObligationRole::Postcondition)
    );
}

/// Form refusal follows candidate validation and precedes mode settlement.
#[trace("FR-019-AC-27", "FR-019-AC-28", "FR-019-AC-29")]
#[test]
fn kani_form_refusal_keeps_candidate_and_mode_precedence() {
    let outside = outside_form();
    let mut one = item(
        RequestedKind::Known(CapabilityKind::ValueValidity),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    one.kani_form = outside.clone();
    one.extent = unbounded(false);
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            one.clone(),
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedKaniForm { .. }
        }
    ));

    let in_set = integer_scalar("quire.op.integer.add");
    let KaniForm::V2ScalarClaim {
        package, node_id, ..
    } = in_set
    else {
        unreachable!("integer scalar helper returns a V2 scalar form")
    };
    one.kani_form = KaniForm::OutsideSet { package, node_id };
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            one.clone(),
        )
        .disposition,
        Disposition::InvalidRequest {
            cause: Cause::KaniFormInvalid { .. }
        }
    ));
    one.kani_form = outside.clone();

    one.candidates = Candidates::Set(Vec::new());
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            one.clone(),
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability { .. }
        }
    ));

    let process_candidate = BackendDescriptor {
        identity: "kani-nightly".to_owned(),
        advertised: vec![(CapabilityKind::ValueValidity, Mode::Bounded)],
        origin: ProviderOrigin::Process,
        domains: None,
        bounds: None,
    };
    one.candidates = Candidates::Set(vec![
        candidate(BackendKind::Kani.identity()),
        candidate("kani-nightly"),
    ]);
    assert!(matches!(
        settle_one(
            vec![
                kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]),
                process_candidate,
            ],
            one.clone(),
        )
        .disposition,
        Disposition::InvalidRequest {
            cause: Cause::AmbiguousBackend { .. }
        }
    ));

    one.candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity())]);
    assert!(matches!(
        settle_one(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Unbounded)])],
            one,
        )
        .disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedKaniForm { .. }
        }
    ));
}

/// Human-facing labels do not participate in typed Kani form identity.
#[trace("FR-019-AC-30")]
#[test]
fn kani_form_identity_does_not_depend_on_display_or_debug_text() {
    for form in [
        integer_scalar("quire.op.integer.add"),
        integer_scalar("quire.op.rational.add"),
    ] {
        let renderings = [
            ("display label A", "debug label A"),
            ("display label B", "debug label B"),
        ];
        let dispositions = renderings.map(|(_display, _debug)| {
            let mut request = item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            );
            request.kani_form = form.clone();
            settle_one(
                vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
                request,
            )
            .disposition
        });
        assert_eq!(dispositions[0], dispositions[1]);
    }
}

/// An envelope whose capability vocabulary is absent or another identity is
/// refused whole, and no kind of it is read.
///
/// Provenance: codegen#86
#[trace("FR-019-AC-7", "TC-030")]
#[test]
fn tc_030_a_foreign_capability_vocabulary_refuses_the_carrier() {
    for supplied in [
        "",
        "quire.capability-kind/v2",
        "quire.capability-kind/v1 ",
        "QUIRE.CAPABILITY-KIND/V1",
        "quire.capability-kind/v10",
        "quire.capability-kind/v1.0",
        "quire.capability-kind/",
    ] {
        let refusal = negotiate_backend_provider(&BackendProviderEnvelope {
            capability_vocabulary: supplied.to_owned(),
            ..envelope(
                vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
                vec![item(
                    RequestedKind::Unknown("foreign-kind".to_owned()),
                    Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
                )],
            )
        })
        .expect_err("a foreign vocabulary refuses the carrier");
        assert_eq!(
            refusal,
            EnvelopeRefusal::InvalidCapability {
                cause: "unsupported-version",
                member: "capability_vocabulary",
                received: supplied.to_owned()
            },
            "no settlement is returned, so no label of the carrier was read"
        );
        assert_eq!(
            serde_json::to_value(&refusal).expect("typed refusal serializes")["code"],
            "invalid_capability"
        );
    }
}

// ---------------------------------------------------------------------------
// The emitted forms, and the censuses they are read against
// ---------------------------------------------------------------------------

/// Every kind and cause serialises in the spelling FR-290 writes.
///
/// Nothing else in this suite reads a serialised form, so without this the eight
/// `Serialize` derives are a wire contract with no test: a consumer matching
/// FR-290's `unknown-backend` never matched the `unknown_backend` an earlier
/// `rename_all` produced, and every assertion here still passed.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_kinds_and_causes_serialise_in_the_spelling_the_spec_writes() {
    for kind in CapabilityKind::ALL {
        assert_eq!(
            serde_json::to_value(kind).expect("a kind serialises"),
            serde_json::Value::String(kind.label().to_owned()),
            "a kind serialises as its own label, not as a second spelling of it"
        );
    }

    let causes = [
        (Cause::AbsentKind, "absent-kind", "invalid_capability"),
        (
            Cause::UnknownKind {
                received: "nope".to_owned(),
            },
            "unknown-kind",
            "invalid_capability",
        ),
        (Cause::AbsentExtent, "absent-extent", "invalid_capability"),
        (
            Cause::UnknownBackend {
                backend: "cvc5".to_owned(),
            },
            "unknown-backend",
            "invalid_capability",
        ),
        (
            Cause::InconsistentCandidates {
                candidates: Vec::new(),
            },
            "inconsistent-candidates",
            "invalid_capability",
        ),
        (
            Cause::AmbiguousBackend {
                candidates: Vec::new(),
            },
            "ambiguous-backend",
            "invalid_capability",
        ),
        (
            Cause::UnsupportedRequestedCapability {
                kind: CapabilityKind::Refinement,
                backend: None,
            },
            "unsupported-requested-capability",
            "unsupported_projection",
        ),
        (
            Cause::UnboundedExtent {
                kind: CapabilityKind::Refinement,
                backend: "kani".to_owned(),
            },
            "unbounded-extent",
            "unsupported_projection",
        ),
    ];
    for (cause, spelling, code) in causes {
        assert_eq!(
            cause.code(),
            code,
            "{spelling} is typed under the wrong code"
        );
        let emitted = serde_json::to_value(&cause).expect("a cause serialises");
        assert_eq!(
            emitted.get("cause").and_then(serde_json::Value::as_str),
            Some(spelling),
            "the emitted cause is not the spelling FR-290 writes: {emitted}"
        );
    }
}

/// Each census array holds every variant of its kind, in index order.
///
/// `index` is an exhaustive match, so a new variant cannot compile without one;
/// reading every index back out of the array is what makes forgetting to extend
/// the array fail too. Without it `ALL` is a hand-kept list whose length happens
/// to be right, and `tc_030_every_backend_kind_has_a_dispatched_arm` — which
/// iterates it — silently checks fewer variants than exist.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_each_census_holds_every_variant_of_its_kind() {
    for (position, kind) in CapabilityKind::ALL.into_iter().enumerate() {
        assert_eq!(kind.index(), position, "{kind:?} is not at its own index");
        assert_eq!(
            CapabilityKind::from_label(kind.label()),
            Some(kind),
            "a member's own label does not read back as that member"
        );
    }
    for (position, backend) in BackendKind::ALL.into_iter().enumerate() {
        assert_eq!(
            backend.index(),
            position,
            "{backend:?} is not at its own index"
        );
    }
    assert_eq!(
        CapabilityKind::from_label("value-validity-v2"),
        None,
        "a label outside the vocabulary is refused, never mapped onto a member"
    );
}

/// A foreign contract version refuses the carrier, and the refusal says which
/// member was wrong.
///
/// Provenance: codegen#86
#[trace("TC-030")]
#[test]
fn tc_030_a_foreign_contract_version_refuses_the_carrier_and_names_the_member() {
    let refusal = negotiate_backend_provider(&BackendProviderEnvelope {
        contract_version: "quire.backend-provider/v2".to_owned(),
        capability_vocabulary: String::new(),
        ..envelope(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            vec![item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
            )],
        )
    })
    .expect_err("a foreign contract version refuses the carrier");
    assert_eq!(
        refusal,
        EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            member: "contract_version",
            received: "quire.backend-provider/v2".to_owned(),
        },
        "both members are wrong here, and the refusal names the one it read"
    );
}
