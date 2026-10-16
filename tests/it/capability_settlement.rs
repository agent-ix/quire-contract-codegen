//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules.

use qsl_replay::{
    compile_package, DependencyInput, DomainKey, DomainKind, FiniteBound, Integer, ProofBound,
    ReplayLimits, SourceIdentity, StageLimits, WireNodeId,
};
use quire_contract_codegen::{
    negotiate_backend_provider, routed_contract_lowering_profile, BackendDescriptor, BackendKind,
    BackendProviderEnvelope, Candidate, Candidates, CapabilityKind, Cause, ClaimDerivationRefusal,
    ContractDriverResultSelection, ContractRequestPosition, Disposition, EnvelopeRefusal,
    ExtentClassification, ExtentDomain, InvalidObligationItem, ItemSettlement, KaniForm, Mode,
    ProviderOrigin, RequestItem, RequestedKind, UnsupportedObligation, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY, QUANTITY_CONVERSION_CAPABILITY,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, ContractDependencies, ContractOccurrenceKeyV1,
    ContractPackageReadResult, ContractPackageWireV1,
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

fn kani(advertised: Vec<(CapabilityKind, Mode)>) -> BackendDescriptor {
    BackendDescriptor {
        identity: BackendKind::Kani.identity().to_owned(),
        advertised,
        origin: ProviderOrigin::Linked,
        domains: None,
        bounds: None,
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
    let package = crate::kani_obligations::bound_package(1_000);
    let clause = package
        .clauses()
        .iter()
        .find(|clause| {
            matches!(
                clause.kind(),
                quire_contract_model::ClauseKind::Precondition
                    | quire_contract_model::ClauseKind::Postcondition
                    | quire_contract_model::ClauseKind::Invariant
            )
        })
        .expect("fixture package has an admitted clause")
        .clone();
    RequestItem {
        kind,
        extent: bounded(),
        named_backend: None,
        candidates,
        kani_form: KaniForm::V1BoundClause {
            clause: Box::new(clause),
        },
    }
}

fn oversized_bound_clause() -> quire_contract_model::BoundClause {
    use serde_json::{json, Value};

    let source = json!({"document":"large-settlement-expression", "revision":1});
    let span = json!({
        "start":{"source":source,"line":1,"column":1,"byte_offset":0},
        "end":{"source":source,"line":1,"column":2,"byte_offset":1}
    });
    let owner = json!({"package":"test/large-settlement", "requirement":"FR-019", "revision":1});
    let anchor = json!({"kind":"pre", "operation":"check"});
    let long_name = "x".repeat(256);
    let identity = json!({"node":"reference", "identity":{
        "requirement":owner, "kind":"input", "observation":"current", "path":[long_name]
    }});
    let mut level = (0..4096)
        .map(|_| json!({"node":"value_reference", "name":long_name, "observation":"current", "source":span}))
        .collect::<Vec<Value>>();
    while level.len() > 1 {
        level = level
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| json!({"node":"boolean", "operator":"short_circuit_and", "left":pair[0], "right":pair[1], "source":span}))
            .collect();
    }
    let projection = json!({
        "format":quire_contract_model::EXECUTABLE_PROJECTION_FORMAT,
        "package":{"id":"test/large-settlement", "schema_version":{"major":1,"minor":1},
            "source":source, "requirements":[{"id":"FR-019","revision":1,"source":span,"clauses":[{
                "id":"large-check", "kind":"precondition", "anchor":anchor, "source":span,
                "body":identity
            }]}]},
        "bindings":[{"clause":{"requirement":owner,"clause":"large-check"}, "expression":{
            "owner":owner,"types":[],"values":[{"name":long_name,"kind":"input","value_type":{"kind":"boolean"},"source":span}],
            "functions":[],"expression":level.pop().unwrap(),"expected_type":{"kind":"boolean"},
            "execution_point":anchor,"clause_root":true
        }}]
    });
    quire_contract_model::BoundPackage::from_json_bytes(&serde_json::to_vec(&projection).unwrap())
        .expect("large, balanced typed expression remains within IR node/depth bounds")
        .clauses()[0]
        .clone()
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
            node.body
                .pointer("/operation/identity")
                .and_then(serde_json::Value::as_str)
                == Some("quire.op.numeric.narrow")
                && node.body.pointer("/arguments/0/target")
                    == Some(&serde_json::to_value(&node_id).expect("typed key"))
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
    let contract = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(contract) => contract,
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
            SourceIdentity::new("test", "selected-add.native", "git", "fixture"),
            "selected-add.native",
            source.as_bytes(),
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
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some(operation_identity)
            })
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
        .find(|node| {
            node.body
                .get("operation")
                .and_then(|op| op.get("identity"))
                .and_then(serde_json::Value::as_str)
                == Some(operation_identity)
        })
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
/// Trace: FR-019-AC-26, FR-019-AC-32
#[test]
fn kani_supported_bounded_request_keeps_exact_proof_bound_input() {
    let (package, _, _) = crate::kani_obligations::scalar_package();
    let node_id = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.body
                .get("operation")
                .and_then(|operation| operation.get("identity"))
                .and_then(serde_json::Value::as_str)
                == Some("quire.op.integer.add")
        })
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
/// Trace: FR-019-AC-2, FR-019-AC-32
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

/// Trace: FR-019-AC-11, FR-019-AC-12, FR-019-AC-20, TC-046.
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

/// Trace: FR-019-AC-42, FR-019-AC-18, FR-019-AC-8, TC-046
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

/// Trace: FR-019-AC-16, FR-019-AC-17, FR-019-AC-21, FR-019-AC-22, TC-046.
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

/// Trace: FR-019-AC-19, TC-046.
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

/// Trace: FR-019-AC-23, TC-046.
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

/// Trace: FR-019-AC-13, FR-019-AC-18, TC-046.
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

    let scratch = std::env::temp_dir().join(format!(
        "quire-codegen-process-settlement-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&scratch).unwrap();
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
    fs::remove_dir_all(scratch).unwrap();
}

/// Every variant of the closed backend kind reaches a dispatched arm, and each
/// one settles rather than falling through.
///
/// Trace: FR-019-AC-1, TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
    }
}

/// Each row of the advertised-mode table settles its own disposition, and an
/// unbounded extent never settles `supported` on a bounded-only advertisement.
///
/// Trace: FR-019-AC-26, FR-019-AC-31
/// Provenance: codegen#86
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
///
/// Trace: FR-019-AC-25, FR-019-AC-26, FR-019-AC-27, FR-019-AC-29, FR-019-AC-31, FR-019-AC-44.
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

/// A selected quantity conversion settles as unsupported with the named capability: the Kani
/// arm cannot accept an operation its generator refuses.
///
/// Trace: FR-014-AC-2, FR-019-AC-44, TC-024.
#[test]
fn a_selected_quantity_conversion_settles_as_unsupported_with_its_named_capability() {
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
                    refusal: UnsupportedObligation::NoDerivableClaim {
                        node_id: refused,
                        reason: ClaimDerivationRefusal::QuantityConversionUnsupported {
                            capability,
                            ..
                        },
                    },
                    backend: refusing,
                    ..
                }
            } if refused == &node_id
                && *capability == QUANTITY_CONVERSION_CAPABILITY
                && *refusing == backend
        ),
        "{:?}",
        settled.disposition
    );
}

/// Selected-result defects follow request/candidate validation and precede
/// the advertised-mode and extent settlement branches.
/// Trace: FR-019-AC-44, TC-030.
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
/// Trace: FR-019-AC-25, FR-019-AC-26, FR-019-AC-29
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
        .find(|node| {
            node.body
                .get("operation")
                .and_then(|operation| operation.get("identity"))
                .and_then(serde_json::Value::as_str)
                == Some("quire.op.integer.div")
        })
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
/// Trace: TC-025.
#[test]
fn direct_fr015_division_keeps_renderer_refusal() {
    assert!(matches!(
        source_derived_division_refusal(),
        UnsupportedObligation::OperationNotRendered { .. }
    ));
}

/// A source-mapped, admitted integer division reaches form classification, rather than
/// disappearing behind the missing-result guard. The FR-015 refusal wins before mode settlement.
/// Trace: FR-015-AC-15, FR-019-AC-25, FR-019-AC-29, FR-019-AC-44, TC-030.
#[test]
fn kani_selected_division_preserves_exact_renderer_refusal() {
    use crate::kani_obligations::package;

    const DIVISION: u32 = 9_301_101;
    const DIVISION_NARROW: u32 = 9_301_102;
    let mut builder = package::qsl_shaped_package();
    let integer = package::key(package::T_INTEGER);
    let division_law = package::integer_division_definition(
        quire_contract_runtime::exact::DivisionProfile::Truncating,
    );
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
    let quire_contract_codegen::KindOutput::Kani { record, harness } = &generated.items[0].output
    else {
        panic!("selected division must reach the Kani arm")
    };
    assert_eq!(
        record.disposition,
        quire_contract_codegen::ObligationDisposition::Unsupported { reason: refusal }
    );
    assert!(harness.is_none(), "refused division emits no Kani harness");
}

/// The classifier measures the same rendered source lines as generation and refuses before
/// Supported when an admitted typed expression would exceed the generated-source ceiling.
#[test]
fn kani_form_preflight_refuses_admitted_expression_over_source_limit() {
    let mut request = item(
        RequestedKind::Known(CapabilityKind::OperationContract),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.kani_form = KaniForm::V1BoundClause {
        clause: Box::new(oversized_bound_clause()),
    };
    let settled = settle_one(
        vec![kani(vec![(
            CapabilityKind::OperationContract,
            Mode::Bounded,
        )])],
        request,
    );
    assert!(
        matches!(
            settled.disposition,
            Disposition::Unsupported {
                cause: Cause::KaniFormRefused {
                    refusal: UnsupportedObligation::ClauseLowering {
                        generation_code:
                            quire_contract_codegen::GenerationErrorCode::ResourceLimitExceeded
                    },
                    ..
                }
            }
        ),
        "large admitted expression unexpectedly settled as {:?}",
        settled.disposition
    );
}

/// V1 roles with zero definedness obligations remain healthy controls, and a source clause with
/// one such obligation preserves that exact count.
/// Trace: FR-019-AC-25, FR-019-AC-26, FR-019-AC-33
#[test]
fn kani_v1_planned_clause_selector_uses_the_exact_lowering_refusal() {
    let package = crate::kani_obligations::bound_package(1_000);
    for kind in [
        quire_contract_model::ClauseKind::Precondition,
        quire_contract_model::ClauseKind::Postcondition,
        quire_contract_model::ClauseKind::Invariant,
    ] {
        let clause = package
            .clauses()
            .iter()
            .find(|clause| clause.kind() == kind)
            .expect("fixture has each executable clause role")
            .clone();
        let settled = settle_one(
            vec![kani(vec![(
                CapabilityKind::OperationContract,
                Mode::Bounded,
            )])],
            RequestItem {
                kind: RequestedKind::Known(CapabilityKind::OperationContract),
                extent: bounded(),
                named_backend: None,
                candidates: Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
                kani_form: KaniForm::V1BoundClause {
                    clause: Box::new(clause),
                },
            },
        );
        assert!(
            matches!(settled.disposition, Disposition::Supported { .. }),
            "typed V1 {kind:?} clause was refused as {:?}",
            settled.disposition
        );
    }

    let mut request = item(
        RequestedKind::Known(CapabilityKind::OperationContract),
        Candidates::Set(vec![candidate(BackendKind::Kani.identity())]),
    );
    request.kani_form = KaniForm::V1BoundClause {
        clause: Box::new(crate::kani_obligations::ir295_definedness_clause()),
    };
    let disposition = settle_one(
        vec![kani(vec![(
            CapabilityKind::OperationContract,
            Mode::Bounded,
        )])],
        request,
    )
    .disposition;
    assert!(
        matches!(
            disposition,
            Disposition::Unsupported {
                cause: Cause::KaniFormRefused {
                    refusal: UnsupportedObligation::DefinednessNotEncoded { obligations },
                    ..
                }
            } if obligations == 2
        ),
        "actual V1 definedness disposition: {disposition:?}"
    );
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
/// Trace: FR-019-AC-25, FR-019-AC-26, FR-019-AC-33
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
/// Trace: FR-019-AC-25, FR-019-AC-27, FR-019-AC-30, FR-019-AC-33.
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
            .and_then(|node| node.body.get("operation")?.get("identity")?.as_str())
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
///
/// Trace: FR-019-AC-44, TC-030.
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
/// Trace: FR-019-AC-44, TC-030.
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

/// Trace: FR-019-AC-44, TC-030.
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
/// Trace: FR-019-AC-44, TC-030.
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
        .and_then(|node| node.body.get("operation")?.get("identity")?.as_str())
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
///
/// Trace: FR-019-AC-27, FR-019-AC-28, FR-019-AC-29.
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
///
/// Trace: FR-019-AC-30.
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
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_a_foreign_capability_vocabulary_refuses_the_carrier() {
    for supplied in ["", "quire.capability-kind/v2"] {
        let refusal = negotiate_backend_provider(&BackendProviderEnvelope {
            capability_vocabulary: supplied.to_owned(),
            ..envelope(
                vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
                vec![item(
                    RequestedKind::Known(CapabilityKind::ValueValidity),
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
/// Trace: TC-030
/// Provenance: codegen#86
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
