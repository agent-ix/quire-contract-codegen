//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules.

use quire_contract_codegen::{
    negotiate_backend_provider, BackendDescriptor, BackendKind, BackendProviderEnvelope, Candidate,
    Candidates, CapabilityKind, Cause, Disposition, EnvelopeRefusal, ExtentClassification,
    ItemSettlement, Mode, ProviderOrigin, RequestItem, RequestedKind, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY,
};

fn kani(advertised: Vec<(CapabilityKind, Mode)>) -> BackendDescriptor {
    BackendDescriptor {
        identity: BackendKind::Kani.identity().to_owned(),
        advertised,
        origin: ProviderOrigin::Linked,
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
    })
}

fn unbounded(finite_bound_available: bool) -> Option<ExtentClassification> {
    Some(ExtentClassification {
        extent: Mode::Unbounded,
        finite_bound_available,
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

/// Every variant of the closed backend kind reaches a dispatched arm, and each
/// one settles rather than falling through.
///
/// Trace: TC-030
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
        origin: ProviderOrigin::Process,
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
                extent,
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
