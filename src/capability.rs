//! Capability settlement: the one point where a requested claim is settled.
//!
//! AD-016 places capability negotiation here so that language admission stays
//! language-only and the IR stays target-neutral. The boundary holds only when
//! adding a backend kind is a compile error everywhere it must be handled, so
//! the settlement point dispatches one arm per variant of [`BackendKind`] with
//! no catch-all: a kind with no arm does not build, rather than reaching a run
//! with no settlement and nothing saying so.
//!
//! `candidates` is read, never computed. The `quire-spec-language` registry
//! computes it under FR-290's candidate-set rule; this module is the consumer
//! on the far side of the FR-331 envelope.

use std::collections::BTreeSet;

use serde::Serialize;

/// The FR-331 envelope's contract version, the only one this module reads.
pub const BACKEND_PROVIDER_CONTRACT: &str = "quire.backend-provider/v1";

/// The FR-290 vocabulary identity, the only one this module reads.
pub const CAPABILITY_VOCABULARY: &str = "quire.capability-kind/v1";

/// The ten FR-290 capability kinds.
///
/// The vocabulary is closed. A label outside it is refused with its exact
/// received bytes and never mapped onto a member.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityKind {
    /// Whether a Boolean clause holds for every assignment of its domain.
    ValueValidity,
    /// Whether an operation's contract clauses hold for every admitted pre-state.
    OperationContract,
    /// Whether one finite execution conforms step by step.
    FiniteReplay,
    /// Whether a temporal requirement holds over its profile's traces.
    TemporalSatisfaction,
    /// Finite global protocol conformance under the selected closure premises.
    GlobalConformance,
    /// Whether the selected fragment can be monitored under the declared observations.
    Monitorability,
    /// Whether one participant projection is defined under its premises.
    LocalProjection,
    /// Whether one protocol refines another under a selected relation.
    Refinement,
    /// Whether the protocol admits a causal implementation strategy.
    Realizability,
    /// Whether selected components compose under independently discharged assumptions.
    Composition,
}

impl CapabilityKind {
    /// Every member, in the order FR-290 declares them.
    ///
    /// Declaration order carries no strength and no dispatch precedence; this
    /// is a census, not a ranking.
    pub const ALL: [Self; 10] = [
        Self::ValueValidity,
        Self::OperationContract,
        Self::FiniteReplay,
        Self::TemporalSatisfaction,
        Self::GlobalConformance,
        Self::Monitorability,
        Self::LocalProjection,
        Self::Refinement,
        Self::Realizability,
        Self::Composition,
    ];

    /// The exact label this kind serializes as.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ValueValidity => "value-validity",
            Self::OperationContract => "operation-contract",
            Self::FiniteReplay => "finite-replay",
            Self::TemporalSatisfaction => "temporal-satisfaction",
            Self::GlobalConformance => "global-conformance",
            Self::Monitorability => "monitorability",
            Self::LocalProjection => "local-projection",
            Self::Refinement => "refinement",
            Self::Realizability => "realizability",
            Self::Composition => "composition",
        }
    }

    /// The member whose label is byte-equal to `label`.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.label() == label)
    }
}

/// The kind a request item carries, including the two ways it can fail to name one.
///
/// An absent kind and an unknown label settle differently and are reported with
/// different causes, so they are separate variants rather than one `Option`
/// that loses which happened.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestedKind {
    /// The item names a member of the vocabulary.
    Known(CapabilityKind),
    /// The item's kind is absent or `null`.
    Absent,
    /// The item names a label that is not byte-equal to any member.
    Unknown(String),
}

/// An advertised mode. A backend advertises a set of (kind, mode) pairs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// The backend discharges the kind over a finite bound.
    Bounded,
    /// The backend discharges the kind without one.
    Unbounded,
}

/// A claim's extent, classified by the boundedness architecture.
///
/// `quire-spec-language#222` owns the representation, the classification and
/// the finite-bound predicate; this module reads all three and computes none.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExtentClassification {
    /// The classification itself.
    pub extent: Mode,
    /// Whether a finite bound is available for an unbounded extent.
    ///
    /// It separates `requires-bound` from `unsupported` and is read from #222,
    /// never inferred from the extent.
    pub finite_bound_available: bool,
}

/// A registered backend: the pair that identifies it, and what it advertises.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendDescriptor {
    /// Backend identity, unique within a registry.
    pub identity: String,
    /// Digest of the manifest this descriptor was read from.
    pub manifest_digest: String,
    /// Advertised (kind, mode) pairs.
    pub advertised: Vec<(CapabilityKind, Mode)>,
    /// The tool identity the adapter probes after routing.
    pub pinned_tool: String,
}

impl BackendDescriptor {
    /// The modes this backend advertises for `kind`, ascending.
    fn modes_for(&self, kind: CapabilityKind) -> BTreeSet<Mode> {
        self.advertised
            .iter()
            .filter(|(advertised, _)| *advertised == kind)
            .map(|(_, mode)| *mode)
            .collect()
    }

    fn advertises(&self, kind: CapabilityKind) -> bool {
        !self.modes_for(kind).is_empty()
    }
}

/// A candidate: the pair (backend identity, manifest digest) of a registered backend.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Candidate {
    /// Backend identity.
    pub identity: String,
    /// Manifest digest.
    pub manifest_digest: String,
}

/// An item's `candidates` value.
///
/// FR-331 admits exactly two shapes, and the unknown-backend mark carries the
/// identity the request named so the refusal can name it back.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Candidates {
    /// The candidate set the registry computed, ordered by identity then digest.
    Set(Vec<Candidate>),
    /// The request named a backend that is not registered.
    UnknownBackend(String),
}

/// One request item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestItem {
    /// The one FR-290 kind this item requires.
    pub kind: RequestedKind,
    /// The item's extent classification, absent when the request carries none.
    pub extent: Option<ExtentClassification>,
    /// The backend the caller names, when it names one.
    pub named_backend: Option<String>,
    /// The candidates the registry computed for this item.
    pub candidates: Candidates,
}

/// The FR-331 backend provider envelope, as far as settlement reads it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendProviderEnvelope {
    /// Exactly [`BACKEND_PROVIDER_CONTRACT`].
    pub contract_version: String,
    /// Exactly [`CAPABILITY_VOCABULARY`].
    pub capability_vocabulary: String,
    /// One descriptor per registered backend in the registry snapshot.
    pub manifest: Vec<BackendDescriptor>,
    /// The request's items, in request order.
    pub items: Vec<RequestItem>,
}

/// A refusal of the whole carrier, before any kind is read.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "cause")]
pub enum EnvelopeRefusal {
    /// The contract version or the capability vocabulary is absent or another identity.
    ///
    /// FR-290 refuses the carrier rather than the item, and none of its labels
    /// are read: a vocabulary that is not the one selected does not merely
    /// rename members, it makes every label in the carrier unreadable.
    InvalidCapability {
        /// Always `unsupported-version`; the field keeps the cause printable
        /// beside the item-level causes rather than implied by the variant.
        cause: &'static str,
        /// The identity the carrier supplied, or the empty string when absent.
        received: String,
    },
}

/// Why an item settled as something other than `supported`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "cause")]
pub enum Cause {
    /// The item's kind is absent or `null`.
    AbsentKind,
    /// The item's kind is a label outside the vocabulary, reported verbatim.
    UnknownKind {
        /// The exact received label.
        received: String,
    },
    /// The item carries no extent classification.
    AbsentExtent,
    /// The request named an unregistered backend, or one with no negotiation arm.
    UnknownBackend {
        /// The backend identity the request named.
        backend: String,
    },
    /// A candidate is absent from the manifest, does not advertise the kind, or
    /// disagrees with the named backend.
    InconsistentCandidates {
        /// Every offending candidate, in candidate order.
        candidates: Vec<Candidate>,
    },
    /// More than one candidate, and the request names no backend.
    AmbiguousBackend {
        /// Every candidate, in candidate order.
        candidates: Vec<Candidate>,
    },
    /// No registered backend advertises the item's kind.
    UnsupportedRequestedCapability {
        /// The kind no candidate advertises.
        kind: CapabilityKind,
        /// The backend the request named, when it named one.
        backend: Option<String>,
    },
    /// An unbounded extent against a bounded-only advertisement, with no finite
    /// bound available.
    UnboundedExtent {
        /// The kind whose extent could not be bounded.
        kind: CapabilityKind,
        /// The single candidate that advertises `bounded` only.
        backend: String,
    },
}

impl Cause {
    /// The FR-272 code this cause is typed under.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AbsentKind
            | Self::UnknownKind { .. }
            | Self::AbsentExtent
            | Self::UnknownBackend { .. }
            | Self::InconsistentCandidates { .. }
            | Self::AmbiguousBackend { .. } => "invalid_capability",
            Self::UnsupportedRequestedCapability { .. } | Self::UnboundedExtent { .. } => {
                "unsupported_projection"
            }
        }
    }
}

/// How one item settled.
///
/// The four dispositions are FR-331's. `RequiresBound` is a negotiation
/// disposition and never a verification result; nothing in this module converts
/// one into the other.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "disposition")]
pub enum Disposition {
    /// The single candidate's arm discharges the claim as requested.
    Supported {
        /// The backend that settled it.
        backend: String,
    },
    /// The claim needs a finite bound before the single candidate can discharge it.
    RequiresBound {
        /// The backend that settled it.
        backend: String,
    },
    /// No candidate discharges the claim; the warning names what was requested.
    Unsupported {
        /// Why, typed under `unsupported_projection`.
        cause: Cause,
    },
    /// The request itself is wrong, and the cause names what is wrong with it.
    InvalidRequest {
        /// Why, typed under `invalid_capability`.
        cause: Cause,
    },
}

/// One item's settlement, in request order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ItemSettlement {
    /// The item's index in the request.
    pub request_index: usize,
    /// The disposition.
    pub disposition: Disposition,
}

impl ItemSettlement {
    /// The warning an `unsupported` settlement carries, if any.
    ///
    /// FR-290 requires the warning to name the item's kind, and the named
    /// backend when the request names one. It is derived from the cause rather
    /// than stored beside it, so the two cannot disagree.
    #[must_use]
    pub fn warning(&self) -> Option<String> {
        match &self.disposition {
            Disposition::Unsupported { cause } => Some(match cause {
                Cause::UnsupportedRequestedCapability { kind, backend } => match backend {
                    Some(backend) => format!(
                        "no registered backend advertises {} for named backend {backend}",
                        kind.label()
                    ),
                    None => format!("no registered backend advertises {}", kind.label()),
                },
                Cause::UnboundedExtent { kind, backend } => format!(
                    "{backend} advertises {} bounded only and no finite bound is available",
                    kind.label()
                ),
                other => format!("unsupported: {}", other.code()),
            }),
            _ => None,
        }
    }
}

/// The closed set of backend kinds this generator settles for.
///
/// One arm per variant, no catch-all. Adding a variant without an arm is a
/// compile error at [`negotiate_arm`], which is the whole point of the kind
/// being closed: an open set of hand-written negotiate functions gives the same
/// behaviour at run time and none of the enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKind {
    /// The Kani backend descriptor, implemented here and in `quire-contract-ir`.
    Kani,
}

impl BackendKind {
    /// Every variant, for the census the settlement gate reads.
    pub const ALL: [Self; 1] = [Self::Kani];

    /// The backend identity this kind registers under.
    #[must_use]
    pub const fn identity(self) -> &'static str {
        match self {
            Self::Kani => "kani",
        }
    }

    /// The kind a registered backend identity names, when one has an arm here.
    ///
    /// A registered backend with no arm answers `None` and settles
    /// `invalid-request`/`unknown-backend`, which is FR-290's first candidate
    /// row: a backend nothing here can settle for is not silently treated as
    /// one that can.
    #[must_use]
    pub fn from_identity(identity: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.identity() == identity)
    }
}

/// Settle every item of an envelope.
///
/// Trace: TC-030
// Implements: FR-019
pub fn negotiate_backend_provider(
    envelope: &BackendProviderEnvelope,
) -> Result<Vec<ItemSettlement>, EnvelopeRefusal> {
    if envelope.contract_version != BACKEND_PROVIDER_CONTRACT {
        return Err(EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            received: envelope.contract_version.clone(),
        });
    }
    if envelope.capability_vocabulary != CAPABILITY_VOCABULARY {
        return Err(EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            received: envelope.capability_vocabulary.clone(),
        });
    }
    Ok(envelope
        .items
        .iter()
        .enumerate()
        .map(|(request_index, item)| ItemSettlement {
            request_index,
            disposition: negotiate_item(&envelope.manifest, item),
        })
        .collect())
}

/// Settle one item, evaluating FR-290's rules in order; the first match settles.
fn negotiate_item(manifest: &[BackendDescriptor], item: &RequestItem) -> Disposition {
    let kind = match &item.kind {
        RequestedKind::Absent => {
            return Disposition::InvalidRequest {
                cause: Cause::AbsentKind,
            }
        }
        RequestedKind::Unknown(received) => {
            return Disposition::InvalidRequest {
                cause: Cause::UnknownKind {
                    received: received.clone(),
                },
            }
        }
        RequestedKind::Known(kind) => *kind,
    };
    let Some(extent) = item.extent else {
        return Disposition::InvalidRequest {
            cause: Cause::AbsentExtent,
        };
    };
    let candidates = match &item.candidates {
        Candidates::UnknownBackend(backend) => {
            return Disposition::InvalidRequest {
                cause: Cause::UnknownBackend {
                    backend: backend.clone(),
                },
            }
        }
        Candidates::Set(candidates) => candidates.as_slice(),
    };
    if let Some(backend) = unroutable_named_backend(item.named_backend.as_deref()) {
        return Disposition::InvalidRequest {
            cause: Cause::UnknownBackend { backend },
        };
    }
    let offending = inconsistent_candidates(manifest, item, kind, candidates);
    if !offending.is_empty() {
        return Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: offending,
            },
        };
    }
    match candidates {
        [] => Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind,
                backend: item.named_backend.clone(),
            },
        },
        [single] => negotiate_single_candidate(manifest, single, kind, extent),
        many if item.named_backend.is_none() => Disposition::InvalidRequest {
            cause: Cause::AmbiguousBackend {
                candidates: many.to_vec(),
            },
        },
        many => Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: many.to_vec(),
            },
        },
    }
}

/// The named backend, when it names no negotiation arm.
///
/// FR-290's first candidate row pairs "a registered backend with no negotiation
/// arm" with the unknown-backend mark, and that mark carries the identity the
/// request *named*. Both halves of the row are therefore read as being about the
/// named backend, not about any member of the set: the alternative reading makes
/// the row swallow the ambiguous-backend row below it, because an unarmed
/// backend anywhere in a multi-candidate set would settle before the ambiguity
/// the caller actually has to resolve. A candidate that reaches routing with no
/// arm is still refused with this cause, at the single-candidate arm.
fn unroutable_named_backend(named: Option<&str>) -> Option<String> {
    named
        .filter(|named| BackendKind::from_identity(named).is_none())
        .map(str::to_owned)
}

/// Every candidate absent from the manifest, not advertising the kind, or
/// disagreeing with the named backend, in candidate order.
fn inconsistent_candidates(
    manifest: &[BackendDescriptor],
    item: &RequestItem,
    kind: CapabilityKind,
    candidates: &[Candidate],
) -> Vec<Candidate> {
    candidates
        .iter()
        .filter(|candidate| {
            let registered = descriptor(manifest, candidate);
            let disagrees = item
                .named_backend
                .as_ref()
                .is_some_and(|named| *named != candidate.identity);
            disagrees || registered.is_none_or(|backend| !backend.advertises(kind))
        })
        .cloned()
        .collect()
}

fn descriptor<'a>(
    manifest: &'a [BackendDescriptor],
    candidate: &Candidate,
) -> Option<&'a BackendDescriptor> {
    manifest.iter().find(|backend| {
        backend.identity == candidate.identity
            && backend.manifest_digest == candidate.manifest_digest
    })
}

/// Route the single candidate to its arm.
///
/// Every function that constructs a [`Disposition`] is named `negotiate_*`, and
/// TC-030's source gate is what holds that true as the module grows.
fn negotiate_single_candidate(
    manifest: &[BackendDescriptor],
    candidate: &Candidate,
    kind: CapabilityKind,
    extent: ExtentClassification,
) -> Disposition {
    let Some(backend) = descriptor(manifest, candidate) else {
        return Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate.clone()],
            },
        };
    };
    let Some(routed) = BackendKind::from_identity(&backend.identity) else {
        return Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: backend.identity.clone(),
            },
        };
    };
    negotiate_arm(routed, backend, kind, extent)
}

/// Dispatch one arm per backend kind.
///
/// Trace: TC-030
// Implements: FR-019
fn negotiate_arm(
    routed: BackendKind,
    backend: &BackendDescriptor,
    kind: CapabilityKind,
    extent: ExtentClassification,
) -> Disposition {
    match routed {
        BackendKind::Kani => negotiate_kani(backend, kind, extent),
    }
}

/// The Kani arm.
///
/// Kani discharges what its manifest advertises, so the advertised-mode
/// comparison is the whole of its settlement today. It compares the claim's
/// extent against the advertised modes and never narrows the extent to reach a
/// disposition.
fn negotiate_kani(
    backend: &BackendDescriptor,
    kind: CapabilityKind,
    extent: ExtentClassification,
) -> Disposition {
    let modes = backend.modes_for(kind);
    match extent.extent {
        Mode::Bounded => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded if modes.contains(&Mode::Unbounded) => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded if extent.finite_bound_available => Disposition::RequiresBound {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded => Disposition::Unsupported {
            cause: Cause::UnboundedExtent {
                kind,
                backend: backend.identity.clone(),
            },
        },
    }
}
