//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules.

use qsl_replay::{DomainKey, DomainKind, FiniteBound, Integer, ProofBound, WireNodeId};
use quire_contract_codegen::{
    negotiate_backend_provider, BackendDescriptor, BackendKind, BackendProviderEnvelope, Candidate,
    Candidates, CapabilityKind, Cause, Disposition, EnvelopeRefusal, ExtentClassification,
    ExtentDomain, ItemSettlement, Mode, ProviderOrigin, RequestItem, RequestedKind,
    BACKEND_PROVIDER_CONTRACT, CAPABILITY_VOCABULARY,
};
use std::{collections::BTreeSet, fs, os::unix::fs::PermissionsExt, process::Command};

fn kani(advertised: Vec<(CapabilityKind, Mode)>) -> BackendDescriptor {
    BackendDescriptor {
        identity: BackendKind::Kani.identity().to_owned(),
        advertised,
        origin: ProviderOrigin::Linked,
        domains: None,
        bounds: Default::default(),
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
        bounds: Default::default(),
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
    changed_defaults
        .bounds
        .insert("model_check.max_depth".to_owned(), 1);
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
        bounds: Default::default(),
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
        bounds: Default::default(),
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
/// Trace: TC-030
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
            Disposition::Supported {
                backend: kani_backend.clone(),
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
